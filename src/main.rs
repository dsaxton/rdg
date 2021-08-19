use std::fs::File;
use std::process;

mod app;
mod pattern;
mod sample;

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
    let delimiter = app_matches.value_of("delimiter").unwrap_or("\n");

    match app_matches.subcommand() {
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
                .unwrap_or("1")
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
                print!(
                    "{}{}",
                    sample::integer_given_bounds(lower, upper),
                    delimiter
                );
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
                eprintln!("lower must be strictly less than upper");
                process::exit(EXIT_ERROR);
            }

            for _ in 0..count {
                print!("{}{}", sample::float_given_bounds(lower, upper), delimiter);
            }
        }
        Some(("word", word_matches)) => {
            let wordlist = word_matches.value_of("wordlist").unwrap();
            let file = match File::open(wordlist) {
                Ok(f) => f,
                Err(err) => {
                    eprintln!("Error reading file: {}", err);
                    process::exit(EXIT_ERROR);
                }
            };
            for _ in 0..count {
                print!("{}{}", sample::from_wordlist(&file), delimiter);
            }
        }
        Some(("string", string_matches)) => {
            // TODO: remove this debugging
            let pat = string_matches.value_of("pattern").expect("invalid pattern");
            println!(
                "pattern: {} can_parse_as_literal_kind: {}",
                pat,
                pattern::can_parse_as_literal_kind(pat)
            );
            println!(
                "pattern: {} can_parse_as_parentheses_kind: {}",
                pat,
                pattern::can_parse_as_parentheses_kind(pat)
            );
        }
        _ => {
            process::exit(EXIT_ERROR);
        }
    }
    process::exit(EXIT_SUCCESS);
}
