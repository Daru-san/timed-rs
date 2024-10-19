mod commands;

use commands::{clock, stopwatch, timer};
use clap::Parser;

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
    if args.verbose {
        println!("DEBUG {args:?}");
    }
    println!(
        "Hello {} (from timed-rs)!",
        args.name.unwrap_or("world".to_string())
    );
}
