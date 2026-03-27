use std::sync::mpsc::Sender;
use std::thread;

use ratatui::crossterm::event::{self};

use crate::tui::event::AppEvent;

pub fn spawn(sender: Sender<AppEvent>) {
    thread::spawn(move || {
        loop {
            if let Ok(event) = event::read() {
                if sender.send(AppEvent::Term(event)).is_err() {
                    break;
                }
            }
        }
    });
}
