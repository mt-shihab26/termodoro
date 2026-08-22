use std::{
    fs,
    io::{self, Error, ErrorKind, Read, Result, Write},
};

use flate2::{Compression, read::GzDecoder, write::GzEncoder};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use time::OffsetDateTime;

use crate::utils::{
    date::now,
    db::rt,
    drive::{auth, client},
    path::{backup_state_path, db_path},
};

/// Persisted backup state: the Drive file id and hash of the last successful upload, so
/// repeated backups overwrite the same file and skip uploading unchanged data.
#[derive(Debug, Default, Deserialize, Serialize)]
struct BackupState {
    drive_file_id: Option<String>,
    last_backup_hash: Option<String>,
    last_backup_at: Option<OffsetDateTime>,
}

impl BackupState {
    /// Loads the backup state from disk, returning the default if the file does not exist.
    fn load() -> Self {
        let path = backup_state_path();
        fs::read_to_string(path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    /// Saves the backup state to disk.
    fn save(&self) {
        let path = backup_state_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string(self) {
            let _ = fs::write(path, json);
        }
    }
}

/// Runs the Google OAuth browser sign-in flow (prints/opens an authorize URL, then waits
/// for the local redirect) and stores the resulting refresh token in the OS keyring, so
/// later `backup`/`restore` runs authenticate silently. If a refresh token is already
/// stored, this just confirms it's still valid instead of prompting again.
pub fn run_login() -> Result<()> {
    rt().block_on(async {
        auth::access_token().await?;
        println!("signed in to Google Drive");
        Ok(())
    })
}

/// Prints whether orivo currently has a stored Google Drive refresh token.
pub fn run_status() -> Result<()> {
    if auth::is_signed_in()? {
        println!("signed in to Google Drive");
    } else {
        println!("not signed in — run `orivo login` to sign in");
    }
    Ok(())
}

/// Removes the stored Google refresh token, signing orivo out of Google Drive.
pub fn run_logout() -> Result<()> {
    auth::sign_out()?;
    println!("signed out of Google Drive");
    Ok(())
}

/// Gzip-compresses, hashes, and (unless unchanged) uploads the local database to the
/// user's Google Drive app-data folder, overwriting the previous backup in place.
pub fn run_backup() -> Result<()> {
    let mut state = BackupState::load();

    let raw = fs::read(db_path())?;
    let gzipped = gzip(&raw)?;
    let hash = hash(&gzipped);

    if state.last_backup_hash.as_deref() == Some(hash.as_str()) {
        println!("nothing changed, skipping backup");
        return Ok(());
    }

    rt().block_on(async {
        let file_id = match &state.drive_file_id {
            Some(id) => Some(id.clone()),
            None => client::find_backup_file().await?,
        };

        let file_id = client::upload_or_update(file_id.as_deref(), &gzipped).await?;

        state.drive_file_id = Some(file_id);
        state.last_backup_hash = Some(hash);
        state.last_backup_at = Some(now());
        state.save();

        println!("backup uploaded to Google Drive");
        Ok(())
    })
}

/// Downloads the Google Drive backup and overwrites the local database with it, after an
/// interactive confirmation prompt since this permanently discards local data.
pub fn run_restore() -> Result<()> {
    println!("This will overwrite your local todos and pomodoro history with the Drive backup.");
    print!("Continue? [y/N] ");
    io::stdout().flush()?;

    let mut answer = String::new();
    io::stdin().read_line(&mut answer)?;
    if !answer.trim().eq_ignore_ascii_case("y") {
        println!("restore cancelled");
        return Ok(());
    }

    let state = BackupState::load();

    rt().block_on(async {
        let file_id = match state.drive_file_id {
            Some(id) => id,
            None => client::find_backup_file().await?.ok_or_else(|| {
                Error::new(ErrorKind::NotFound, "no backup found in Google Drive")
            })?,
        };

        let gzipped = client::download(&file_id).await?;
        let raw = gunzip(&gzipped)?;

        let path = db_path();
        let tmp_path = path.with_extension("sqlite.tmp");
        fs::write(&tmp_path, raw)?;
        fs::rename(&tmp_path, &path)?;

        println!("restore complete");
        Ok(())
    })
}

/// Compresses `data` with gzip.
fn gzip(data: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    encoder.finish()
}

/// Decompresses gzip-compressed `data`.
fn gunzip(data: &[u8]) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    GzDecoder::new(data).read_to_end(&mut out)?;
    Ok(out)
}

/// Returns the lowercase hex-encoded SHA-256 hash of `data`.
fn hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher
        .finalize()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
