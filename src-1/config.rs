use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{action::Action, app::Mode};

#[derive(Clone, Debug, Default)]
pub struct KeyBindings(pub HashMap<Mode, HashMap<Vec<KeyEvent>, Action>>);

#[derive(Clone, Debug, Default)]
pub struct Config {
    pub keybindings: KeyBindings,
}

impl Config {
    pub fn new() -> Self {
        let mut home_bindings: HashMap<Vec<KeyEvent>, Action> = HashMap::new();

        home_bindings.insert(
            vec![KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE)],
            Action::Quit,
        );
        home_bindings.insert(
            vec![KeyEvent::new(KeyCode::Char('d'), KeyModifiers::CONTROL)],
            Action::Quit,
        );
        home_bindings.insert(
            vec![KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL)],
            Action::Quit,
        );
        home_bindings.insert(
            vec![KeyEvent::new(KeyCode::Char('z'), KeyModifiers::CONTROL)],
            Action::Suspend,
        );

        let mut bindings = HashMap::new();
        bindings.insert(Mode::Home, home_bindings);

        Self {
            keybindings: KeyBindings(bindings),
        }
    }
}
