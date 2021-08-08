use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::{App, AppSettings, Arg};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

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

    let count = app_matches
        .value_of("count")
        .unwrap_or("1")
        .parse::<u64>()
        .expect("lines must be a non-negative integer");
    let delimiter = app_matches.value_of("delimiter").unwrap_or("\n");
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
                print!("{}{}", sample_integer_given_bounds(lower, upper), delimiter);
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
                print!("{}{}", sample_float_given_bounds(lower, upper), delimiter);
            }
        }
        "word" => {
            let wordlist = app_matches
                .subcommand_matches("word")
                .unwrap()
                .value_of("wordlist")
                .unwrap();
            for _ in 0..count {
                print!("{}{}", sample_from_wordlist(wordlist), delimiter);
            }
        }
        "string" => {
            let length = app_matches
                .subcommand_matches("string")
                .unwrap()
                .value_of("length")
                .unwrap_or("10")
                .parse::<usize>()
                .expect("length must be a non-negative integer");
            for _ in 0..count {
                print!("{}{}", sample_string_from_alphanumeric(length), delimiter);
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
    let delta = thread_rng().gen::<f64>() * (upper - lower);
    lower + delta
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

fn sample_string_from_alphanumeric(length: usize) -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    rand_string
}
