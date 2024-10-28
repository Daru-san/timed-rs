mod commands;

use commands::{clock, stopwatch, timer};

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
    /// Clock
    Clock {
        /// Time formatting
        #[clap(short, long, default_value = "%X")]
        format: String,
    },

    Stopwatch {},

    /// Timer
    Timer {
        /// Time to run the timer
        time: f32,
    },
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Clock { format }) => {
            clock::run(format);
        }
        Some(Commands::Timer { time }) => {
            timer::run(*time);
        }
        Some(Commands::Stopwatch {}) => {
            stopwatch::run();
        }
        None => {
            clock::run("%X");
        }
    }
}
