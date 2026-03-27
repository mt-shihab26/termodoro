use std::sync::mpsc::Sender;
use std::thread;

use ratatui::crossterm::event::{self, Event, KeyEventKind};

use crate::event::AppEvent;

pub fn spawn(sender: Sender<AppEvent>) {
    thread::spawn(move || {
        loop {
            if let Ok(Event::Key(key)) = event::read() {
                if key.kind == KeyEventKind::Press {
                    if sender.send(AppEvent::Key(key)).is_err() {
                        break;
                    }
                }
            }
        }
    });
}
