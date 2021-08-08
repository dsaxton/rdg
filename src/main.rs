use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::{App, AppSettings, Arg};
use rand::prelude::*;

fn main() {
    // TODO: add tests
    let app_matches = App::new("rd")
        .version("0.1")
        .about("Generate random data at the command line")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::DisableHelpSubcommand)
        .arg(
            Arg::new("count")
                .short('n')
                .long("count")
                .about("Number of values in output, default 1")
                .takes_value(true),
        )
        .arg(
            Arg::new("delim")
                .short('d')
                .long("delim")
                .about("Delimiter to use between values, default \\n")
                .takes_value(true),
        )
        // TODO: implement this
        .arg(
            Arg::new("concurrency")
                .short('c')
                .long("concurrency")
                .about("Concurrency level, default 1")
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
                        .about("Lower bound (inclusive), default 0")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("upper")
                        .short('u')
                        .long("upper")
                        .about("Upper bound (exclusive), default 2")
                        .takes_value(true),
                ),
        )
        .subcommand(
            App::new("float")
                .about("Random floats, default support [0, 1)")
                .setting(AppSettings::ColoredHelp)
                .arg(
                    Arg::new("lower")
                        .short('l')
                        .long("lower")
                        .about("Lower bound (inclusive), default 0")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("upper")
                        .short('u')
                        .long("upper")
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
                        .about("Wordlist used for sampling")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            App::new("string")
                .about("Random strings, default length 8")
                .setting(AppSettings::ColoredHelp)
                .arg(
                    Arg::new("length")
                        .short('l')
                        .long("length")
                        .about("Length of string, default 8")
                        .takes_value(true),
                ),
        )
        .get_matches();

    let count = app_matches
        .value_of("count")
        .unwrap_or("1")
        .parse::<u64>()
        .expect("lines must be a non-negative integer");
    let delim = app_matches.value_of("delim").unwrap_or("\n");
    let subcommand_name = app_matches.subcommand_name().unwrap();

    match subcommand_name {
        // TODO: possible to match on the subcommand itself?
        "int" => {
            let lower = app_matches
                .subcommand_matches("int")
                .unwrap()
                .value_of("lower")
                .unwrap_or("0")
                .parse::<u64>()
                .expect("lower must be a non-negative integer");
            let upper = app_matches
                .subcommand_matches("int")
                .unwrap()
                .value_of("upper")
                .unwrap_or("2")
                .parse::<u64>()
                .expect("upper must be a non-negative integer");
            if lower >= upper {
                panic!("lower must be strictly less than upper")
            }
            for _ in 0..count {
                print!("{}", sample_integer_given_bounds(lower, upper));
                print!("{}", delim);
            }
        }
        "float" => {
            let lower = app_matches
                .subcommand_matches("float")
                .unwrap()
                .value_of("lower")
                .unwrap_or("0")
                .parse::<f64>()
                .expect("lower must be a non-negative float");
            let upper = app_matches
                .subcommand_matches("float")
                .unwrap()
                .value_of("upper")
                .unwrap_or("1")
                .parse::<f64>()
                .expect("upper must be a non-negative float");
            if lower >= upper {
                panic!("lower must be strictly less than upper")
            }
            for _ in 0..count {
                print!("{}", sample_float_given_bounds(lower, upper));
                print!("{}", delim);
            }
        }
        "word" => {
            let wordlist = app_matches
                .subcommand_matches("word")
                .unwrap()
                .value_of("wordlist")
                .unwrap();
            // TODO: consider allowing sampling without replacement
            for _ in 0..count {
                print!("{}", sample_from_wordlist(wordlist));
                print!("{}", delim);
            }
        }
        _ => (),
    }
}

fn sample_integer_given_bounds(lower: u64, upper: u64) -> u64 {
    let delta = (thread_rng().gen::<f64>() * ((upper - lower) as f64)).floor() as u64;
    lower + delta
}

fn sample_float_given_bounds(lower: f64, upper: f64) -> f64 {
    thread_rng().gen::<f64>() * (upper - lower)
}

fn sample_from_wordlist(wordlist: &str) -> String {
    let file = File::open(wordlist).expect("file does not exist");
    let reader = BufReader::new(file);
    let mut selected_word = vec![String::from("")]; // FIXME: this feels like a hack

    // TODO: consider using for_byte_line as in pattern.rs from ripgrep
    for (idx, line) in reader.lines().enumerate() {
        if thread_rng().gen::<f64>() < 1.0 / ((idx + 1) as f64) {
            selected_word.pop();
            selected_word.push(line.unwrap())
        }
    }
    selected_word[0].clone()
}
