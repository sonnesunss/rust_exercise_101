use crossterm::event::KeyCode;
use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;
use ratatui::Frame;
use ratatui::widgets::{Block, Borders};

fn main() -> color_eyre::Result<()> {
    println!("Hello, world!");
    color_eyre::install()?;

    let terminal = ratatui::init();

    let r = App::default().run(terminal);

    ratatui::restore();

    r
}

#[allow(dead_code)]
#[derive(Default)]
struct App {
    should_exit: bool,
}

#[allow(dead_code)]
impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            self.should_exit = self.handle_events()?;

            if self.should_exit {
                break;
            }
        }

        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) {
        let block = Block::new()
            .borders(Borders::ALL)
            .title("Hello Block widget");

        frame.render_widget(block, frame.area());
    }

    pub fn handle_events(&self) -> std::io::Result<bool> {
        if let Event::Key(key) = event::read()? {
            return Ok(KeyCode::Char('q') == key.code);
        }
        Ok(false)
    }
}
