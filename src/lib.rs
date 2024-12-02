pub mod solutions;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Day not yet implemented: {}.", .0)]
    DayNotImplemented(String)
}

pub fn run_day(day: &str) -> Result<(), Error> {
    match day {
        "1a" => {
            solutions::day1a::main();
            Ok(())
        }
        "1b" => {
            solutions::day1b::main();
            Ok(())
        }
        "2a" => {
            solutions::day2a::main();
            Ok(())
        }
        "2b" => {
            solutions::day2b::main();
            Ok(())
        }

        // <-- INSERT NEW DAY HERE -->
        _ => Err(Error::DayNotImplemented(day.to_string())),
    }
}
