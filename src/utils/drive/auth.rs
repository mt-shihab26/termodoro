use std::io::{Error, ErrorKind, Result};

use oauth2::{
    AuthType, AuthUrl, ClientId, ClientSecret, DeviceAuthorizationUrl, EndpointNotSet, EndpointSet,
    RefreshToken, Scope, StandardDeviceAuthorizationResponse, TokenResponse, TokenUrl,
    basic::BasicClient,
};

/// Public OAuth2 client id for orivo's "TVs and Limited Input devices" client, registered
/// once in Google Cloud Console (see README's "For maintainers" section). Not confidential
/// for this client type, but injected via `ORIVO_GOOGLE_CLIENT_ID` at build time rather than
/// hardcoded, so the real value never lands in git history and doesn't trip secret scanners.
const CLIENT_ID: Option<&str> = option_env!("ORIVO_GOOGLE_CLIENT_ID");
/// Accompanying public client "secret" for the installed-app OAuth client type, injected via
/// `ORIVO_GOOGLE_CLIENT_SECRET` at build time for the same reason.
const CLIENT_SECRET: Option<&str> = option_env!("ORIVO_GOOGLE_CLIENT_SECRET");
/// Limited scope: only the app-data folder, not the user's full Drive.
const SCOPE: &str = "https://www.googleapis.com/auth/drive.appdata";

const KEYRING_SERVICE: &str = "orivo";
const KEYRING_USERNAME: &str = "google-drive-refresh-token";

type OrivoClient =
    BasicClient<EndpointSet, EndpointSet, EndpointNotSet, EndpointNotSet, EndpointSet>;

/// Builds the Google OAuth2 client configured for the device authorization grant.
fn build_client() -> Result<OrivoClient> {
    let client_id = CLIENT_ID.ok_or_else(|| {
        io_err("orivo was built without ORIVO_GOOGLE_CLIENT_ID set; Google Drive backup is unavailable")
    })?;
    let client_secret = CLIENT_SECRET.ok_or_else(|| {
        io_err(
            "orivo was built without ORIVO_GOOGLE_CLIENT_SECRET set; Google Drive backup is unavailable",
        )
    })?;

    let client_id = ClientId::new(client_id.to_string());
    let client_secret = ClientSecret::new(client_secret.to_string());
    let auth_url =
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).map_err(io_err)?;
    let token_url =
        TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string()).map_err(io_err)?;
    let device_auth_url =
        DeviceAuthorizationUrl::new("https://oauth2.googleapis.com/device/code".to_string())
            .map_err(io_err)?;

    Ok(BasicClient::new(client_id)
        .set_client_secret(client_secret)
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_device_authorization_url(device_auth_url)
        .set_auth_type(AuthType::RequestBody))
}

/// Returns the keyring entry used to store the Google refresh token.
fn entry() -> Result<keyring::Entry> {
    keyring::Entry::new(KEYRING_SERVICE, KEYRING_USERNAME).map_err(io_err)
}

/// Runs the OAuth2 device-code flow: prints a verification URL and code for the user to
/// approve in a browser, polls Google until access is granted, then stores the returned
/// refresh token in the OS keyring.
async fn device_login() -> Result<RefreshToken> {
    let client = build_client()?;
    let http = reqwest::Client::new();

    let details: StandardDeviceAuthorizationResponse = client
        .exchange_device_code()
        .add_scope(Scope::new(SCOPE.to_string()))
        .request_async(&http)
        .await
        .map_err(io_err)?;

    println!(
        "To back up orivo's data to Google Drive, open:\n  {}\nand enter the code: {}",
        details.verification_uri(),
        details.user_code().secret()
    );

    let token = client
        .exchange_device_access_token(&details)
        .request_async(&http, tokio::time::sleep, None)
        .await
        .map_err(io_err)?;

    let refresh_token = token.refresh_token().cloned().ok_or_else(|| {
        Error::new(
            ErrorKind::Other,
            "Google did not return a refresh token for this login",
        )
    })?;

    entry()?
        .set_password(refresh_token.secret())
        .map_err(io_err)?;

    Ok(refresh_token)
}

/// Returns a fresh short-lived access token for calling the Drive API, triggering the
/// device-code login flow if no refresh token is stored yet (or it has been revoked).
pub async fn access_token() -> Result<String> {
    let refresh_token = match entry()?.get_password() {
        Ok(secret) => RefreshToken::new(secret),
        Err(keyring::Error::NoEntry) => device_login().await?,
        Err(e) => return Err(io_err(e)),
    };

    let client = build_client()?;
    let http = reqwest::Client::new();

    let token = match client
        .exchange_refresh_token(&refresh_token)
        .request_async(&http)
        .await
    {
        Ok(token) => token,
        Err(_) => {
            // The stored refresh token was rejected (revoked/expired) — log in again.
            let refresh_token = device_login().await?;
            client
                .exchange_refresh_token(&refresh_token)
                .request_async(&http)
                .await
                .map_err(io_err)?
        }
    };

    if let Some(rotated) = token.refresh_token() {
        let _ = entry()?.set_password(rotated.secret());
    }

    Ok(token.access_token().secret().to_string())
}

fn io_err(e: impl std::fmt::Display) -> Error {
    Error::new(ErrorKind::Other, e.to_string())
}
