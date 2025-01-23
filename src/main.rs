use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, List, ListDirection, ListState, Paragraph, StatefulWidget},
    DefaultTerminal, Frame,
};
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::{fs::OpenOptions, io, io::Write};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

enum InputMode {
    Normal,
    Insert,
}

#[derive(Clone, Serialize, Deserialize)]
struct Todo {
    desc: String,
    done: bool,
}

pub struct App {
    exit: bool,
    file: std::fs::File,
    todos: Vec<Todo>,
    input: Input,
    input_mode: InputMode,
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .read(true)
        .open("todos.json")
        .expect("Could not open or write to file.");

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let file_json: Vec<Todo> = if contents.is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&contents).expect("Could not process file.")
    };

    let mut app = App {
        exit: false,
        file,
        todos: file_json.to_vec(),
        input: Input::default(),
        input_mode: InputMode::Normal,
    };
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
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
        // create the main layout
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(60),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ])
            .split(frame.area());
        // set the cursor position based on the input mode
        let mut border_style = Style::new().fg(Color::White);
        match self.input_mode {
            InputMode::Insert => {
                border_style = Style::new().fg(Color::Green);
                frame.set_cursor_position(Position {
                    x: layout[1].x + self.input.visual_cursor() as u16 + 1,
                    y: layout[1].y + 1,
                });
            }
            InputMode::Normal => {}
        }

        // render the "stateful" List widget
        frame.render_stateful_widget(self, layout[0], list_state);

        // create a paragraph widget for the input
        let paragraph = Paragraph::new(self.input.value())
            .block(Block::bordered().border_set(border::THICK).title("Input"))
            .style(border_style);

        // render the paragraph widget
        frame.render_widget(paragraph, layout[1]);

        let footer_text = vec![
            Line::from("Press 'q' to quit,"),
            Line::from("'i' to enter insert mode,"),
            Line::from("'Enter' to toggle todo items,"),
            Line::from("'j' and 'k' to navigate."),
            Line::from("Use 'Esc' to switch back to normal mode from insert mode."),
        ];
        let footer = Paragraph::new(footer_text)
            .block(Block::bordered().border_set(border::THICK).title("Help"));

        frame.render_widget(footer, layout[2]);
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
        match self.input_mode {
            InputMode::Insert => match key_event.code {
                KeyCode::Enter => {
                    self.todos.push(Todo {
                        desc: self.input.value().to_string(),
                        done: false,
                    });

                    self.write_file()?;
                    self.input.reset();
                }
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                }
                _ => {
                    self.input
                        .handle_event(&crossterm::event::Event::Key(key_event));
                }
            },
            InputMode::Normal => match key_event.code {
                KeyCode::Char('q') => self.exit = true,
                KeyCode::Char('i') => self.input_mode = InputMode::Insert,
                KeyCode::Char('j') | KeyCode::Down => list_state.select_next(),
                KeyCode::Char('k') | KeyCode::Up => list_state.select_previous(),
                KeyCode::Enter => {
                    let selected = list_state.selected().unwrap();
                    self.todos[selected].done = !self.todos[selected].done;
                    self.write_file()?;
                }
                _ => {}
            },
        }
        Ok(())
    }

    fn write_file(&mut self) -> io::Result<()> {
        let updated_contents =
            serde_json::to_string(&self.todos).expect("Could not serialize todos.");
        self.file.set_len(0)?;
        self.file.write_all(updated_contents.as_bytes())?;
        Ok(())
    }
}

fn get_items(items: Vec<Todo>) -> Vec<String> {
    items
        .iter()
        .map(|item| {
            let done = if item.done { "✓" } else { " " };
            format!("{} {}", item.desc, done)
        })
        .collect()
}

// https://docs.rs/ratatui/latest/ratatui/widgets/trait.StatefulWidget.html
impl StatefulWidget for &App {
    type State = ListState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut ListState) {
        let title = Line::from(" Term_Todo ").centered().bold();
        let items = &self.todos;

        List::new(get_items(items.to_vec()))
            .block(Block::bordered().title(title).border_set(border::THICK))
            .style(Style::new().white())
            .highlight_style(Style::new().black().bg(Color::Blue))
            .highlight_symbol(">> ")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom)
            .render(area, buf, state);
    }
}
