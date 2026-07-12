use notify_rust::Notification;
use rodio::Source;
use rodio::source::SineWave;
use std::thread;
use std::time::Duration;

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

pub fn notify_sound() {
    thread::spawn(|| {
        let result = (|| -> Result<(), Box<dyn std::error::Error>> {
            let stream_handle = rodio::DeviceSinkBuilder::open_default_sink()?;
            let mixer = stream_handle.mixer();
            let wave = SineWave::new(740.0).amplify(0.2).take_duration(Duration::from_secs(3));
            mixer.add(wave);
            thread::sleep(Duration::from_millis(1500));
            Ok(())
        })();
        if let Err(e) = result {
            log_error!("failed to play notification sound: {e}");
        }
    });
}
