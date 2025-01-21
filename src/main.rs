use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, List, ListDirection, ListState, StatefulWidget},
    DefaultTerminal, Frame,
};
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App {
        exit: false,
        selected: 0,
    };
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}

pub struct App {
    exit: bool,
    selected: usize,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut list_state = ListState::default();
        list_state.select_first();
        while !self.exit {
            terminal.draw(|frame| self.draw(frame, &mut list_state))?;
            self.handle_events(&mut list_state)?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame, list_state: &mut ListState) {
        frame.render_stateful_widget(self, frame.area(), list_state);
    }

    fn handle_events(&mut self, list_state: &mut ListState) -> io::Result<()> {
        match crossterm::event::read()? {
            Event::Key(key_event) => self.handle_key_event(key_event, list_state)?,
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        list_state: &mut ListState,
    ) -> io::Result<()> {
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('q') {
            self.exit = true;
        }

        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Down
            || key_event.code == KeyCode::Char('j')
        {
            list_state.select_next()
        }

        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Up
            || key_event.code == KeyCode::Char('k')
        {
            list_state.select_previous()
        }

        Ok(())
    }
}

#[derive(Clone)]
struct Todo {
    desc: String,
    done: bool,
}

fn get_items(items: Vec<Todo>) -> Vec<String> {
    items.iter().map(|item| format!("{}", item.desc)).collect()
}

impl StatefulWidget for &App {
    type State = ListState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut ListState) {
        let title = Line::from(" Term_Todo ").centered().bold();
        let instructions = Line::from(" Press 'q' to exit ").centered();
        let items = [
            Todo {
                desc: "Item 1".to_string(),
                done: false,
            },
            Todo {
                desc: "Item 2".to_string(),
                done: true,
            },
        ];

        List::new(get_items(items.to_vec()))
            .block(
                Block::bordered()
                    .title(title)
                    .title_bottom(instructions)
                    .border_set(border::THICK),
            )
            .style(Style::new().white())
            .highlight_style(Style::new().black().bg(Color::Blue))
            .highlight_symbol(">> ")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom)
            .render(area, buf, state);
    }
}
