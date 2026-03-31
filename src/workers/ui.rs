use std::sync::mpsc::Sender;
use std::{thread, time::Duration};

use crate::{kinds::event::Event, log_error};

const UI_TICK_MS: u64 = 16;

pub fn spawn(sender: Sender<Event>) {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(UI_TICK_MS));

        if sender.send(Event::TodosTick).is_err() {
            log_error!("ui worker: event channel closed, stopping");
            break;
        }
    });
}
