use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout},
    widgets::Block,
    Frame,
};

pub fn draw_terminal() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}

fn run(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| draw_ui(frame))?;
        if handle_events()? {
            break Ok(());
        }
    }
}

fn draw_ui(frame: &mut Frame) {
    use Constraint::{Fill, Length, Min};

    let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
    let [title_area, main_area, status_area] = vertical.areas(frame.area());
    let horizontal = Layout::horizontal([Fill(1); 2]);
    let [left_area, right_area] = horizontal.areas(main_area);

    frame.render_widget(Block::bordered().title("Timed Rust"), title_area);
    frame.render_widget(Block::bordered().title("Bottom"), status_area);
    frame.render_widget(Block::bordered().title("Clock"), left_area);
    frame.render_widget(Block::bordered().title("Stopwatch"), right_area);
}

fn handle_events() -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Char('r') => return Ok(false),
            _ => {}
        },
        _ => {}
    }
    Ok(false)
}
