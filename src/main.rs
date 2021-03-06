use std::process;

mod app;
mod pattern;
mod sample;

use pattern::Pattern;

const EXIT_SUCCESS: i32 = 0;
const EXIT_ERROR: i32 = 1;

fn main() {
    let app_matches = app::create_app().get_matches();
    let count = app_matches
        .value_of("count")
        .unwrap_or("1")
        .parse::<u64>()
        .unwrap_or_else(|err| {
            eprintln!("Error parsing count: {}", err);
            process::exit(EXIT_ERROR);
        });

    match app_matches.subcommand() {
        Some(("word", word_matches)) => {
            let file = word_matches.value_of("file").unwrap();
            for _ in 0..count {
                match sample::from_wordlist(file) {
                    Ok(value) => println!("{}", value),
                    Err(err) => {
                        eprintln!("Error reading file: {}", err);
                        process::exit(EXIT_ERROR);
                    }
                }
            }
        }
        Some(("string", string_matches)) => {
            let pattern = string_matches
                .value_of("pattern")
                .unwrap_or("[A-Za-z0-9]{10}");
            let sampler = match Pattern::parse(pattern) {
                Ok(s) => s.to_string_sampler(),
                Err(_) => {
                    eprintln!("Unable to parse pattern: {}", pattern);
                    process::exit(EXIT_ERROR);
                }
            };
            for _ in 0..count {
                println!("{}", sampler.sample());
            }
        }
        Some(("int", int_matches)) => {
            let lower = int_matches
                .value_of("lower")
                .unwrap_or("0")
                .parse::<u64>()
                .unwrap_or_else(|err| {
                    eprintln!("Error parsing lower: {}", err);
                    process::exit(EXIT_ERROR);
                });
            let upper = int_matches
                .value_of("upper")
                .unwrap_or("2")
                .parse::<u64>()
                .unwrap_or_else(|err| {
                    eprintln!("Error parsing upper: {}", err);
                    process::exit(EXIT_ERROR);
                });

            if lower >= upper {
                eprintln!("Error: lower must be strictly less than upper");
                process::exit(EXIT_ERROR);
            }

            for _ in 0..count {
                println!("{}", sample::integer_given_bounds(lower, upper));
            }
        }
        Some(("float", float_matches)) => {
            let lower = float_matches
                .value_of("lower")
                .unwrap_or("0")
                .parse::<f64>()
                .unwrap_or_else(|err| {
                    eprintln!("Error parsing lower: {}", err);
                    process::exit(EXIT_ERROR);
                });
            let upper = float_matches
                .value_of("upper")
                .unwrap_or("1")
                .parse::<f64>()
                .unwrap_or_else(|err| {
                    eprintln!("Error parsing upper: {}", err);
                    process::exit(EXIT_ERROR);
                });

            if lower >= upper {
                eprintln!("Error: lower must be strictly less than upper");
                process::exit(EXIT_ERROR);
            }

            for _ in 0..count {
                println!("{}", sample::float_given_bounds(lower, upper));
            }
        }
        _ => {
            process::exit(EXIT_ERROR);
        }
    }
    process::exit(EXIT_SUCCESS);
}
