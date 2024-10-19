use std::io;

use clap::Parser;
use core::f64;
use std::process::exit;

use ratatui::{backend::CrosstermBackend, Terminal};

use crate::{
    app::{App, AppResult},
    commands::{clock, stopwatch, timer},
    event::{Event, EventHandler},
    handler::handle_key_events,
    tui::Tui,
};

pub mod app;
pub mod commands;
pub mod event;
pub mod handler;
pub mod tui;
pub mod ui;

#[derive(Parser, Debug)]
#[clap(author = "Daru", version, about)]
/// Application configuration
struct Args {
    /// whether to be verbose
    #[arg(short = 'v')]
    verbose: bool,

    /// Use the timer
    #[arg(short, long, default_value = "0.0")]
    time: f64,

    /// Use the stopwatch
    #[arg(short, long)]
    stopwatch: bool,
}

#[tokio::main]
async fn main() -> AppResult<()> {
    let args = Args::parse();
    let enable_timer = !(args.time == 0.0);
    let enable_stopwatch = args.stopwatch;
    let enable_verbosity = args.verbose;

    if enable_verbosity {
        println!("DEBUG {args:?}");
    }

    if enable_timer && enable_stopwatch {
        println!("Please select one item");
        exit(0);
    }

    if enable_timer {
        timer::run(args.time);
    }

    if enable_stopwatch {
        stopwatch::run();
    }

    if !enable_timer && !enable_stopwatch && !enable_verbosity {
        clock::run();
    }
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
