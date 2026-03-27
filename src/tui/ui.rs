use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::symbols;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Tabs, Widget};

use super::tabs::timer::Timer;
use super::tabs::todos::Todos;

pub struct Ui {
    pub selected_tab: usize,
}

impl Widget for Ui {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([Constraint::Length(1), Constraint::Length(1), Constraint::Fill(1)]);

        let [top, tabs_area, main] = area.layout(&layout);

        Line::from_iter([Span::from("Orivo").bold()])
            .centered()
            .render(top, buf);

        Tabs::new(vec!["Todos", "Timer"])
            .style(Color::White)
            .highlight_style(Style::default().magenta().on_black().bold())
            .select(self.selected_tab)
            .divider(symbols::DOT)
            .padding(" ", " ")
            .render(tabs_area, buf);

        match self.selected_tab {
            0 => Todos.render(main, buf),
            1 => Timer.render(main, buf),
            _ => unreachable!(),
        }
    }
}
