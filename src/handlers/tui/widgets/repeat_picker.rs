use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::{Buffer, Rect, Widget};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

use crate::domains::todos::Repeat;

pub enum RepeatAction {
    Confirm(Option<Repeat>),
    Cancel,
    None,
}

#[derive(Clone)]
pub struct RepeatPicker {
    cursor: usize,
}

impl RepeatPicker {
    pub fn new(selected: Option<Repeat>) -> Self {
        let cursor = selected
            .and_then(|r| Repeat::ALL.iter().position(|v| v == &r).map(|i| i + 1))
            .unwrap_or(0);

        Self { cursor }
    }

    pub fn handle(&mut self, key: KeyEvent) -> RepeatAction {
        let max = Repeat::ALL.len();
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                self.cursor = (self.cursor + 1).min(max);
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.cursor = self.cursor.saturating_sub(1);
            }
            KeyCode::Enter => {
                let repeat = if self.cursor == 0 {
                    None
                } else {
                    Some(Repeat::ALL[self.cursor - 1])
                };
                return RepeatAction::Confirm(repeat);
            }
            KeyCode::Esc => return RepeatAction::Cancel,
            _ => {}
        }
        RepeatAction::None
    }
}

impl Widget for &RepeatPicker {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [list_area, hint_area] = Layout::vertical([Constraint::Fill(1), Constraint::Length(3)]).areas(area);

        let options = std::iter::once("None").chain(Repeat::ALL.iter().map(|r| r.label()));

        let items: Vec<ListItem> = options
            .enumerate()
            .map(|(i, label)| {
                let active = i == self.cursor;
                let style = if active {
                    Style::default().fg(Color::Cyan).bold()
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(format!("{} {}", if active { ">" } else { " " }, label)).style(style)
            })
            .collect();

        List::new(items).render(list_area, buf);

        Paragraph::new(
            "[j/k]Navigate\n\
            [Enter]Confirm\n\
            [Esc]Back",
        )
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .fg(Color::DarkGray)
        .render(hint_area, buf);
    }
}
