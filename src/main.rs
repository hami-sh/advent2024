use advent2024::run_day;
use clap::{Parser, Subcommand};
use std::time::Instant;


/// CLI for Advent of Code 2024 solutions
#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run puzzle solutions
    Run {
        /// Specific day to execute
        #[arg(short, long)]
        day: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run { day} => {
            let start = Instant::now();

            match day {
                Some(day) => {
                    println!("Running puzzle {}...", day);
                    if let Err(e) = run_day(day) {
                        eprintln!("Error running puzzle: {}", e);
                        return;
                    }
                }
                None => {
                    println!("No command given, exiting...")
                }
            }

            let duration = start.elapsed();
            println!("Done! 🎉 -- Elapsed time: {:?}", duration);
        }
    }
}