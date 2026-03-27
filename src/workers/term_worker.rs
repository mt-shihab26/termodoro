use std::sync::mpsc::Sender;
use std::thread;

use ratatui::crossterm::event::{self, Event as TerminalEvent, KeyEventKind};

use crate::event::Event;

pub fn spawn(sender: Sender<Event>) {
    thread::spawn(move || {
        loop {
            if let Ok(TerminalEvent::Key(key)) = event::read() {
                if key.kind == KeyEventKind::Press {
                    if sender.send(Event::Key(key)).is_err() {
                        break;
                    }
                }
            }
        }
    });
}
