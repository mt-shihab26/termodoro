use notify_rust::Notification;

use crate::log_error;

/// Sends a desktop notification with the app name prepended to the summary.
pub fn notify(summary: &str, body: &str) {
    let summary = format!("{} — {summary}", env!("CARGO_PKG_NAME"));
    if let Err(e) = Notification::new()
        .summary(&summary)
        .body(body)
        .sound_name("message-new-instant")
        .show()
    {
        log_error!("failed to send notification: {e}");
    }
}
