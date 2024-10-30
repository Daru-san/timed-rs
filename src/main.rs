mod commands;
mod time;
mod tui;

use commands::clock::Clock;
use commands::stopwatch::Stopwatch;
use commands::timer::Timer;

use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Display a clock with formatting
    Clock {
        /// Time formatting
        #[clap(short, long, default_value = "%X")]
        format: String,
    },

    /// Run the stopwatch
    Stopwatch {},

    /// Run a timer
    Timer {
        /// Time to run the timer, must be a positive integer
        time: u32,
    },

    /// Show the TUI interface
    Tui {},
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Clock { format }) => {
            let clock = Clock::new(format.clone());
            clock.run();
        }
        Some(Commands::Timer { time }) => {
            let timer = Timer::new(*time);
            timer.run();
        }
        Some(Commands::Stopwatch {}) => {
            let stopwatch = Stopwatch::new();
            stopwatch.run();
        }

        Some(Commands::Tui {}) => {
            tui::tui::draw_terminal();
        }

        // Run clock when no parameters are selected
        None => {
            let clock = Clock::new("%X".to_string());
            clock.run();
        }
    }
}
