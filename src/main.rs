use clap::{App, Arg};
use rand::prelude::*;

fn main() {
    // TODO: add tests
    let app_matches = App::new("rd")
        .version("0.1")
        .author("Daniel Saxton")
        .about("Generate random strings")
        .arg(
            Arg::new("lines")
                .short('n')
                .long("lines")
                .about("Number of lines of output, default 1")
                .takes_value(true),
        )
        .arg(
            Arg::new("delim")
                .short('d')
                .long("delim")
                .about("Delimeter to use between values, default \\n")
                .takes_value(true),
        )
        .subcommand(
            App::new("int")
                .about("Generate random ints")
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
                .about("Generate random floats")
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
        .get_matches();

    let lines = app_matches.value_of("lines").unwrap_or("1").parse::<u64>().expect("lines must be a non-negative integer");
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
                .expect("lower must be a non-negative integer");
            let upper = app_matches
                .subcommand_matches("float")
                .unwrap()
                .value_of("upper")
                .unwrap_or("1")
                .parse::<f64>()
                .expect("upper must be a non-negative integer");
            if lower >= upper {
                panic!("lower must be strictly less than upper")
            }
            for _ in 0..lines {
                seed = rng.gen();
                print!("{}", seed * (upper - lower));
                print!("{}", delim);
            }
        }
        _ => panic!("invalid subcommand"),
    }
}
