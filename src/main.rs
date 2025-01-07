use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::Stylize,
    text::Line,
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
struct App {
    hours: u8,
    exit: bool,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), io::Error> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    fn draw(&self, frame: &mut Frame) {
        let [top, bottom] =
            Layout::vertical([Constraint::Length(3), Constraint::Min(5)]).areas(frame.area());
        let [bottom_left, bottom_right] =
            Layout::horizontal([Constraint::Fill(1); 2]).areas(bottom);

        let title = Paragraph::new("tamatar")
            .centered()
            .bold()
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(title, top);
        frame.render_widget(
            Block::bordered().title(Line::from("Time").bold().centered()),
            bottom_left,
        );
        frame.render_widget(
            Block::bordered().title(Line::from("Controls").bold().centered()),
            bottom_right,
        );
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('e') => self.exit(),
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = App::default().run(&mut terminal);
    ratatui::restore();
    result
}
