use clap::{App, Arg};
use rand::prelude::*;

fn main() {
    let app = App::new("rd")
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
        // TODO: add help for each subcommand
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
                        .about("Upper bound (exclusive), default 10")
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
                        .about("Lower bound (inclusive), default 0"),
                )
                .arg(
                    Arg::new("upper")
                        .short('u')
                        .long("upper")
                        .about("Upper bound (exclusive), default 1"),
                ),
        )
        .get_matches();

    let lines = match app.value_of("lines") {
        Some(l) => l
            .parse::<u64>()
            .expect("lines must be a non-negative integer"),
        None => 1,
    };
    let mut rng = thread_rng();
    let mut seed: f64;
    let subcommand_name = app
        .subcommand_name()
        .expect("please enter a subcommand such as help");

    match subcommand_name {
        "int" => {
            let lower = app
                .subcommand_matches("int")
                .expect("invalid subcommand")
                .value_of("lower")
                .unwrap_or("0")
                .parse::<u64>()
                .expect("lower must be a non-negative integer");
            let upper = app
                .subcommand_matches("int")
                .expect("invalid subcommand")
                .value_of("upper")
                .unwrap_or("1")
                .parse::<u64>()
                .expect("upper must be a non-negative integer");
            if lower >= upper {
                panic!("lower must be strictly less than upper")
            }
            let mut delta: u64;
            for _ in 0..lines {
                seed = rng.gen();
                delta = (seed * ((upper - lower + 1) as f64)).floor() as u64;
                println!("{}", lower + delta);
            }
        }
        _ => panic!("invalid subcommand"),
    }
}
