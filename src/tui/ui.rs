use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::symbols;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Tabs, Widget};

use super::tabs::timer::{self, Timer, TimerState};
use super::tabs::todos::{self, Todos};

pub struct Ui<'a> {
    pub selected_tab: usize,
    pub timer: &'a TimerState,
}

impl<'a> Widget for Ui<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([Constraint::Length(1), Constraint::Length(1), Constraint::Fill(1)]);
        let [top, tabs_area, main] = area.layout(&layout);

        Line::from_iter([Span::from("Orivo").bold().fg(Color::Green)])
            .centered()
            .render(top, buf);

        let highlight_color = match self.selected_tab {
            0 => todos::COLOR,
            _ => timer::COLOR,
        };

        Tabs::new(vec!["Todos", "Timer"])
            .style(Color::White)
            .highlight_style(Style::default().fg(highlight_color).on_black().bold())
            .select(self.selected_tab)
            .divider(symbols::DOT)
            .padding(" ", " ")
            .render(tabs_area, buf);

        match self.selected_tab {
            0 => Todos.render(main, buf),
            1 => Timer::new(self.timer).render(main, buf),
            _ => unreachable!(),
        }
    }
}
