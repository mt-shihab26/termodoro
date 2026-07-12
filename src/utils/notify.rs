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
    sound();
}

fn sound() {
    thread::spawn(|| {
        let result = (|| -> Result<(), Box<dyn std::error::Error>> {
            let mut stream_handle = rodio::DeviceSinkBuilder::open_default_sink()?;
            stream_handle.log_on_drop(false);
            let mixer = stream_handle.mixer();
            let wave = SineWave::new(880.0).amplify(0.15).take_duration(Duration::from_millis(120));
            mixer.add(wave);
            thread::sleep(Duration::from_millis(150));
            Ok(())
        })();
        if let Err(e) = result {
            log_error!("failed to play notification sound: {e}");
        }
    });
}
