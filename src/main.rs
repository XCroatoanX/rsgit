use std::env::args;
use std::time::{Duration, Instant};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{Clear, ClearType};
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::{Block, RatatuiLogo, RatatuiLogoSize};
use ratatui::{DefaultTerminal, Frame, TerminalOptions, Viewport};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Fullscreen,
    });
    let size = match args().nth(1).as_deref() {
        Some("small") => RatatuiLogoSize::Small,
        Some("tiny") => RatatuiLogoSize::Tiny,
        _ => RatatuiLogoSize::default(),
    };
    let result = run(terminal, size);
    ratatui::restore();
    crossterm::execute!(
        std::io::stdout(),
        Clear(ClearType::All),
        crossterm::cursor::MoveTo(0, 0)
    )?;
    println!();
    result
}

fn run(mut terminal: DefaultTerminal, size: RatatuiLogoSize) -> Result<()> {
    let startup_duration = Duration::from_secs(2);

    terminal.clear()?;

    let startup_started = Instant::now();
    while startup_started.elapsed() < startup_duration {
        terminal.draw(|frame| render_startup(frame, size))?;
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if matches!(key.code, KeyCode::Char('q') | KeyCode::Esc) {
                    terminal.clear()?;
                    return Ok(());
                }
            }
        }
    }

    let result = loop {
        terminal.draw(render_layout)?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if matches!(key.code, KeyCode::Char('q') | KeyCode::Esc) {
                    break Ok(());
                }
            }
        }
    };

    terminal.clear()?;
    result
}

fn render_startup(frame: &mut Frame, size: RatatuiLogoSize) {
    let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
    let [top, bottom] = frame.area().layout(&layout);

    frame.render_widget("Powered by", top);
    frame.render_widget(RatatuiLogo::new(size), bottom);
}

fn render_layout(frame: &mut Frame) {
    let vertical = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Fill(2),
        Constraint::Fill(1),
    ]);
    let [top, middle, bottom] = vertical.areas(frame.area());
    let horizontal = Layout::horizontal([Constraint::Fill(1); 2]);
    let [left, right] = horizontal.areas(middle);

    frame.render_widget(Block::bordered().title("Top"), top);
    frame.render_widget(Block::bordered().title("Left"), left);
    frame.render_widget(Block::bordered().title("Right"), right);
    frame.render_widget(Block::bordered().title("Bottom"), bottom);
}
