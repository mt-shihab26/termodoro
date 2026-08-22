use std::{
    io::{BufRead, BufReader, Error, ErrorKind, Result, Write},
    net::TcpListener,
    process::{Command, Stdio},
};

use oauth2::{
    AuthType, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointNotSet,
    EndpointSet, PkceCodeChallenge, RedirectUrl, RefreshToken, Scope, TokenResponse, TokenUrl,
    basic::BasicClient, url::Url,
};

/// Limited scope: only the app-data folder, not the user's full Drive. Google's device-code
/// flow rejects Drive scopes outright despite its own docs listing this one as supported, so
/// login instead uses the standard Authorization Code + PKCE flow with a local loopback
/// redirect — the same approach `rclone`/`gdrive` use for Drive access from a CLI.
const SCOPE: &str = "https://www.googleapis.com/auth/drive.appdata";

const KEYRING_SERVICE: &str = "orivo";
const KEYRING_USERNAME: &str = "google-drive-refresh-token";

type OrivoClient =
    BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>;

/// Reads the Google OAuth client id at runtime: a `GOOGLE_CLIENT_ID` env var (set directly,
/// or via `.env` — see `main()`'s `dotenvy::dotenv()` call) overrides the value baked into
/// the binary at compile time for official release builds (see README's "For maintainers"
/// section). Not confidential for this "Desktop app" client type, but kept out of source so
/// the real value never lands in git history and doesn't trip secret scanners.
fn client_id() -> Option<String> {
    std::env::var("GOOGLE_CLIENT_ID")
        .ok()
        .or_else(|| option_env!("GOOGLE_CLIENT_ID").map(str::to_string))
}

/// Reads the accompanying public client "secret" the same way, via `GOOGLE_CLIENT_SECRET`.
fn client_secret() -> Option<String> {
    std::env::var("GOOGLE_CLIENT_SECRET")
        .ok()
        .or_else(|| option_env!("GOOGLE_CLIENT_SECRET").map(str::to_string))
}

/// Builds the Google OAuth2 client. No redirect URI is set here since it depends on which
/// local port `browser_login` happens to bind to; `exchange_refresh_token` doesn't need one.
fn build_client() -> Result<OrivoClient> {
    let client_id = client_id()
        .ok_or_else(|| io_err("GOOGLE_CLIENT_ID is not set; Google Drive backup is unavailable"))?;
    let client_secret = client_secret().ok_or_else(|| {
        io_err("GOOGLE_CLIENT_SECRET is not set; Google Drive backup is unavailable")
    })?;

    let auth_url =
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).map_err(io_err)?;
    let token_url =
        TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string()).map_err(io_err)?;

    Ok(BasicClient::new(ClientId::new(client_id))
        .set_client_secret(ClientSecret::new(client_secret))
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_auth_type(AuthType::RequestBody))
}

/// Returns the keyring entry used to store the Google refresh token.
fn entry() -> Result<keyring::Entry> {
    keyring::Entry::new(KEYRING_SERVICE, KEYRING_USERNAME).map_err(io_err)
}

/// Best-effort attempt to open `url` in the system's default browser via `xdg-open`. Failures
/// are silently ignored — the URL is always printed too, so this is a convenience, not a
/// requirement (e.g. it does nothing useful over a headless SSH session).
fn open_in_browser(url: &str) {
    let _ = Command::new("xdg-open")
        .arg(url)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
}

/// Runs the OAuth2 authorization-code flow with a local loopback redirect: prints a Google
/// sign-in URL, waits for the browser to redirect back after approval, exchanges the code for
/// tokens, then stores the returned refresh token in the OS keyring. Requires a browser on
/// this same machine — unlike the device-code flow, the redirect can't be completed elsewhere.
async fn browser_login() -> Result<RefreshToken> {
    // Bind port 0 so the OS picks a free one; Google's "Desktop app" client type allows any
    // loopback port without pre-registering it.
    let listener = TcpListener::bind("127.0.0.1:0").map_err(io_err)?;
    let port = listener.local_addr().map_err(io_err)?.port();
    let redirect_url = RedirectUrl::new(format!("http://127.0.0.1:{port}")).map_err(io_err)?;

    let client = build_client()?.set_redirect_uri(redirect_url);

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(SCOPE.to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    println!(
        "To back up orivo's data to Google Drive, open this URL in a browser on this machine and approve access:\n  {authorize_url}"
    );
    open_in_browser(authorize_url.as_str());

    let (code, state) = wait_for_redirect(&listener)?;
    if state.secret() != csrf_state.secret() {
        return Err(io_err("OAuth redirect state mismatch; aborting login"));
    }

    let http = reqwest::Client::new();
    let token = client
        .exchange_code(code)
        .set_pkce_verifier(pkce_verifier)
        .request_async(&http)
        .await
        .map_err(io_err)?;

    let refresh_token = token
        .refresh_token()
        .cloned()
        .ok_or_else(|| io_err("Google did not return a refresh token for this login"))?;

    entry()?
        .set_password(refresh_token.secret())
        .map_err(io_err)?;

    Ok(refresh_token)
}

/// Blocks on one incoming connection to the loopback listener, parses the OAuth redirect's
/// `code`/`state` query params, and replies with a small confirmation page.
fn wait_for_redirect(listener: &TcpListener) -> Result<(AuthorizationCode, CsrfToken)> {
    let (mut stream, _) = listener.accept().map_err(io_err)?;

    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();
    reader.read_line(&mut request_line).map_err(io_err)?;

    let path = request_line
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| io_err("malformed OAuth redirect request"))?;
    let url = Url::parse(&format!("http://127.0.0.1{path}")).map_err(io_err)?;

    let code = url
        .query_pairs()
        .find(|(key, _)| key == "code")
        .map(|(_, value)| AuthorizationCode::new(value.into_owned()))
        .ok_or_else(|| io_err("OAuth redirect missing `code` parameter"))?;
    let state = url
        .query_pairs()
        .find(|(key, _)| key == "state")
        .map(|(_, value)| CsrfToken::new(value.into_owned()))
        .ok_or_else(|| io_err("OAuth redirect missing `state` parameter"))?;

    let body = "<html><body>Signed in to orivo \u{2014} you can close this tab.</body></html>";
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    );
    let _ = stream.write_all(response.as_bytes());

    Ok((code, state))
}

/// Returns a fresh short-lived access token for calling the Drive API, triggering the
/// browser sign-in flow if no refresh token is stored yet (or it has been revoked).
pub async fn access_token() -> Result<String> {
    let refresh_token = match entry()?.get_password() {
        Ok(secret) => RefreshToken::new(secret),
        Err(keyring::Error::NoEntry) => browser_login().await?,
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
            let refresh_token = browser_login().await?;
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
