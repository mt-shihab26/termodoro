use notify_rust::Notification;

use crate::log_error;

pub fn notify(summary: &str, body: &str) {
    let summary = format!("{} — {summary}", env!("CARGO_PKG_NAME"));
    if let Err(e) = Notification::new().summary(&summary).body(body).show() {
        log_error!("failed to send notification: {e}");
    }
}
