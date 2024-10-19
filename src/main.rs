mod commands;

use commands::{clock, stopwatch, timer};

use clap::Parser;
use core::f64;
use std::process::exit;

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

fn main() {
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
}
