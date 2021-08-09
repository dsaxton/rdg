use std::process;

use clap::{App, AppSettings, Arg};

mod sample;

const EXIT_SUCCESS: i32 = 0;
const EXIT_ERROR: i32 = 1;

fn main() {
    let app_matches = App::new("rd")
        .version("0.1")
        .about("Generate random data at the command line")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::DisableHelpSubcommand)
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .value_name("integer")
                .about("Number of values to generate, default 1")
                .takes_value(true),
        )
        .arg(
            Arg::new("delimiter")
                .short('d')
                .long("delimiter")
                .value_name("string")
                .about("Delimiter to use between values, default \\n")
                .takes_value(true),
        )
        // TODO: implement this
        .arg(
            Arg::new("threads")
                .short('t')
                .long("threads")
                .value_name("integer")
                .about("Number of threads, default 1")
                .takes_value(true),
        )
        .subcommand(
            App::new("int")
                .about("Random integers, default support {0, 1}")
                .setting(AppSettings::ColoredHelp)
                .arg(
                    Arg::new("lower")
                        .short('l')
                        .long("lower")
                        .value_name("integer")
                        .about("Lower bound (inclusive), default 0")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("upper")
                        .short('u')
                        .long("upper")
                        .value_name("integer")
                        .about("Upper bound (exclusive), default 2")
                        .takes_value(true),
                ),
        )
        .subcommand(
            App::new("float")
                .about("Random floating point numbers, default support [0, 1)")
                .setting(AppSettings::ColoredHelp)
                .arg(
                    Arg::new("lower")
                        .short('l')
                        .long("lower")
                        .value_name("integer")
                        .about("Lower bound (inclusive), default 0")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("upper")
                        .short('u')
                        .long("upper")
                        .value_name("integer")
                        .about("Upper bound (exclusive), default 1")
                        .takes_value(true),
                ),
        )
        .subcommand(
            App::new("word")
                .about("Random words, requires a wordlist")
                .setting(AppSettings::ColoredHelp)
                .arg(
                    Arg::new("wordlist")
                        .short('w')
                        .long("wordlist")
                        .value_name("path")
                        .about("Wordlist used for sampling")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            App::new("string")
                .about("Random strings, default length 10")
                .setting(AppSettings::ColoredHelp)
                .arg(
                    Arg::new("length")
                        .short('l')
                        .long("length")
                        .value_name("integer")
                        .about("Length of string, default 10")
                        .takes_value(true),
                ),
        )
        .get_matches();

    let count = if let Ok(c) = app_matches.value_of("count").unwrap_or("1").parse::<u64>() {
        c
    } else {
        println!("{}", "invalid count argument");
        process::exit(EXIT_ERROR);
    };
    let delimiter = app_matches.value_of("delimiter").unwrap_or("\n");

    match app_matches.subcommand() {
        Some(("int", int_matches)) => {
            let lower = if let Ok(l) = int_matches.value_of("lower").unwrap_or("0").parse::<u64>() {
                l
            } else {
                // TODO: exit in a better way
                println!("{}", "invalid lower argument");
                process::exit(EXIT_ERROR);
            };
            let upper = if let Ok(u) = int_matches.value_of("upper").unwrap_or("2").parse::<u64>() {
                u
            } else {
                println!("{}", "invalid upper argument");
                process::exit(EXIT_ERROR);
            };

            if lower >= upper {
                println!("{}", "lower must be strictly less than upper");
                process::exit(EXIT_ERROR);
            }

            for _ in 0..count {
                print!(
                    "{}{}",
                    sample::integer_given_bounds(lower, upper),
                    delimiter
                );
            }
            process::exit(EXIT_SUCCESS);
        }
        Some(("float", float_matches)) => {
            let lower = if let Ok(l) = float_matches
                .value_of("lower")
                .unwrap_or("0")
                .parse::<f64>()
            {
                l
            } else {
                // TODO: exit in a better way
                println!("{}", "invalid lower argument");
                process::exit(EXIT_ERROR);
            };
            let upper = if let Ok(u) = float_matches
                .value_of("upper")
                .unwrap_or("2")
                .parse::<f64>()
            {
                u
            } else {
                println!("{}", "invalid upper argument");
                process::exit(EXIT_ERROR);
            };

            if lower >= upper {
                println!("{}", "lower must be strictly less than upper");
                process::exit(EXIT_ERROR);
            }

            for _ in 0..count {
                print!("{}{}", sample::float_given_bounds(lower, upper), delimiter);
            }
            process::exit(EXIT_SUCCESS);
        }
        Some(("word", word_matches)) => {
            let wordlist = word_matches.value_of("wordlist").unwrap();
            for _ in 0..count {
                print!("{}{}", sample::from_wordlist(wordlist), delimiter);
            }
            process::exit(EXIT_SUCCESS);
        }
        Some(("string", string_matches)) => {
            let length = string_matches
                .value_of("length")
                .unwrap_or("10")
                .parse::<usize>()
                .expect("length must be a non-negative integer");
            for _ in 0..count {
                print!("{}{}", sample::string_from_alphanumeric(length), delimiter);
            }
            process::exit(EXIT_SUCCESS);
        }
        _ => {
            process::exit(EXIT_ERROR);
        }
    }
}
