use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::{App, AppSettings, Arg};
use rand::prelude::*;

fn main() {
    // TODO: add tests
    let app_matches = App::new("rd")
        .version("0.1")
        .author("Daniel Saxton")
        .about("Generate random strings")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::ArgRequiredElseHelp)
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
        .subcommand(
            App::new("int")
                .about("Random integers")
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
                .about("Random floating point numbers")
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
                .about("Random words")
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
        .get_matches();

    let lines = app_matches
        .value_of("count")
        .unwrap_or("1")
        .parse::<u64>()
        .expect("lines must be a non-negative integer");
    let delim = app_matches.value_of("delim").unwrap_or("\n");
    let mut rng = thread_rng();
    let mut seed: f64;
    let subcommand_name = app_matches
        .subcommand_name()
        .expect("please enter a subcommand such as help");

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
            let mut delta: u64;
            for _ in 0..lines {
                seed = rng.gen();
                delta = (seed * ((upper - lower) as f64)).floor() as u64;
                print!("{}", lower + delta);
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
            for _ in 0..lines {
                seed = rng.gen();
                print!("{}", seed * (upper - lower));
                print!("{}", delim);
            }
        }
        "word" => {
            let wordlist = app_matches
                .subcommand_matches("word")
                .unwrap()
                .value_of("wordlist")
                .unwrap();
            for _ in 0..lines {
                let file = File::open(wordlist).expect("file does not exist");
                let reader = BufReader::new(file);

                for (idx, line) in reader.lines().enumerate() {
                    seed = rng.gen();
                    if seed < 1.0 / ((idx + 1) as f64) {
                        print!("{}", line.unwrap());
                        print!("{}", delim);
                    }
                }
            }
        }
        _ => panic!("invalid subcommand"),
    }
}
