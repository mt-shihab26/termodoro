use notify_rust::Notification;
use rodio::Source;
use rodio::source::SineWave;
use std::thread;
use std::time::Duration;

use crate::kinds::phase::Phase;
use crate::log_error;

/// Sends a desktop notification with the app name prepended to the summary.
pub fn notify(summary: &str, body: &str, phase: &Phase) {
    let summary = format!("{} — {summary}", env!("CARGO_PKG_NAME"));
    if let Err(e) = Notification::new()
        .summary(&summary)
        .body(body)
        .sound_name("message-new-instant")
        .show()
    {
        log_error!("failed to send notification: {e}");
    }
    sound(phase);
}

fn sound(phase: &Phase) {
    let (freq, duration_ms) = match phase {
        // Work done → warm, satisfying tone
        Phase::Work => (660.0_f32, 150_u64),
        // Short break done → bright, alerting tone
        Phase::Break => (880.0_f32, 100_u64),
        // Long break done → softer, lower tone
        Phase::LongBreak => (523.0_f32, 200_u64),
    };

    thread::spawn(move || {
        let result = (|| -> Result<(), Box<dyn std::error::Error>> {
            let mut stream_handle = rodio::DeviceSinkBuilder::open_default_sink()?;
            stream_handle.log_on_drop(false);
            let mixer = stream_handle.mixer();
            let wave = SineWave::new(freq).amplify(0.15).take_duration(Duration::from_millis(duration_ms));
            mixer.add(wave);
            thread::sleep(Duration::from_millis(duration_ms + 30));
            Ok(())
        })();
        if let Err(e) = result {
            log_error!("failed to play notification sound: {e}");
        }
    });
}
