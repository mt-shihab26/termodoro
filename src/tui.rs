use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Offset, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::symbols;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Tabs, Widget};

use crate::tabs::{timer::TimerTab, todos::TodosTab};

pub struct Tui {
    pub selected_tab: usize,
}

impl Widget for Tui {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
        let [top, main] = area.layout(&layout);

        Line::from_iter([Span::from("Orivo").bold()])
            .centered()
            .render(top, buf);

        Tabs::new(vec!["Todos", "Timer"])
            .style(Color::White)
            .highlight_style(Style::default().magenta().on_black().bold())
            .select(self.selected_tab)
            .divider(symbols::DOT)
            .padding(" ", " ")
            .render(main + Offset::new(1, 0), buf);

        match self.selected_tab {
            0 => TodosTab.render(main, buf),
            1 => TimerTab.render(main, buf),
            _ => unreachable!(),
        }
    }
}
