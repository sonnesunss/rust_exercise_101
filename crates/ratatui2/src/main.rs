use color_eyre;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    widgets::{Block, Borders},
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // 使用默认的init创建一个运行实例
    let terminal = ratatui::init();
    let result = run(terminal);

    result
}

// 渲染UI
fn draw(frame: &mut Frame) {
    let block = Block::new()
        .borders(Borders::ALL)
        .title("Hello block. press q to quit app")
        .title_position(ratatui::widgets::block::Position::Top);

    frame.render_widget(block, frame.area());
}

// run app runloop
// 处理用户事件、更新app状态等
fn run(mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
    let mut should_exit = false;

    loop {
        terminal.draw(|frame| draw(frame))?;
        should_exit = handle_events()?;
        if should_exit {
            break;
        }
    }

    Ok(())
}

// 处理用户事件，主要使用crossterm提供的事件机制支持
fn handle_events() -> std::io::Result<bool> {
    if let Event::Key(key) = event::read()? {
        if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
            return Ok(true);
        }
    }

    Ok(false)
}
