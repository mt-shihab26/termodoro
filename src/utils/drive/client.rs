use std::io::{Error, ErrorKind, Result};

use serde::Deserialize;

use super::auth;

/// Fixed name of orivo's single backup file inside Drive's hidden app-data folder.
const BACKUP_FILE_NAME: &str = "orivo-backup.sqlite.gz";
/// Boundary marker for the hand-rolled `multipart/related` upload body. Long and specific
/// enough that it will never occur inside gzip-compressed sqlite bytes.
const UPLOAD_BOUNDARY: &str = "orivo-drive-backup-boundary-3f7a1c9e";

const FILES_URL: &str = "https://www.googleapis.com/drive/v3/files";
const UPLOAD_URL: &str = "https://www.googleapis.com/upload/drive/v3/files";

#[derive(Deserialize)]
struct FileListResponse {
    files: Vec<DriveFile>,
}

#[derive(Deserialize)]
struct DriveFile {
    id: String,
}

/// Looks up the backup file's Drive file id by its fixed name in the app-data folder.
/// Used to recover a lost local `fileId`, e.g. after reinstalling on a new machine.
pub async fn find_backup_file() -> Result<Option<String>> {
    let token = auth::access_token().await?;

    let resp = reqwest::Client::new()
        .get(FILES_URL)
        .bearer_auth(token)
        .query(&[
            ("spaces", "appDataFolder"),
            ("q", &format!("name = '{BACKUP_FILE_NAME}'")),
            ("fields", "files(id)"),
        ])
        .send()
        .await
        .map_err(io_err)?
        .error_for_status()
        .map_err(io_err)?;

    let body: FileListResponse = resp.json().await.map_err(io_err)?;
    Ok(body.files.into_iter().next().map(|f| f.id))
}

/// Creates the backup file if `file_id` is `None`, otherwise overwrites it in place.
/// Returns the Drive file id, so the caller can persist it for next time.
pub async fn upload_or_update(file_id: Option<&str>, bytes: &[u8]) -> Result<String> {
    let token = auth::access_token().await?;

    let metadata = match file_id {
        Some(_) => "{}".to_string(),
        None => format!(r#"{{"name":"{BACKUP_FILE_NAME}","parents":["appDataFolder"]}}"#),
    };

    let mut body = Vec::with_capacity(bytes.len() + metadata.len() + 256);
    body.extend_from_slice(
        format!(
            "--{UPLOAD_BOUNDARY}\r\nContent-Type: application/json; charset=UTF-8\r\n\r\n{metadata}\r\n"
        )
        .as_bytes(),
    );
    body.extend_from_slice(
        format!("--{UPLOAD_BOUNDARY}\r\nContent-Type: application/gzip\r\n\r\n").as_bytes(),
    );
    body.extend_from_slice(bytes);
    body.extend_from_slice(format!("\r\n--{UPLOAD_BOUNDARY}--").as_bytes());

    let http = reqwest::Client::new();
    let request = match file_id {
        Some(id) => http.patch(format!("{UPLOAD_URL}/{id}?uploadType=multipart")),
        None => http.post(format!("{UPLOAD_URL}?uploadType=multipart")),
    };

    let resp = request
        .bearer_auth(token)
        .header(
            "Content-Type",
            format!("multipart/related; boundary={UPLOAD_BOUNDARY}"),
        )
        .body(body)
        .send()
        .await
        .map_err(io_err)?
        .error_for_status()
        .map_err(io_err)?;

    let file: DriveFile = resp.json().await.map_err(io_err)?;
    Ok(file.id)
}

/// Downloads the raw (gzip-compressed) bytes of the backup file with the given Drive file id.
pub async fn download(file_id: &str) -> Result<Vec<u8>> {
    let token = auth::access_token().await?;

    let resp = reqwest::Client::new()
        .get(format!("{FILES_URL}/{file_id}?alt=media"))
        .bearer_auth(token)
        .send()
        .await
        .map_err(io_err)?
        .error_for_status()
        .map_err(io_err)?;

    Ok(resp.bytes().await.map_err(io_err)?.to_vec())
}

fn io_err(e: impl std::fmt::Display) -> Error {
    Error::new(ErrorKind::Other, e.to_string())
}
