use clap::{App, Arg};
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let matches = App::new("rd")
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

    let lines = if let Some(lines_matches) = matches.value_of("lines") {
        lines_matches
    } else {
        "1"
    };
    let lines = lines
        .parse::<u32>()
        .expect("lines must be a non-negative integer");

    if let Some(int_matches) = matches.subcommand_matches("int") {
        let lower = if let Some(l) = int_matches.value_of("lower") {
            l
        } else {
            "0"
        };
        let lower = lower
            .parse::<u32>()
            .expect("lower must be a non-negative integer");

        let upper = if let Some(u) = int_matches.value_of("upper") {
            u
        } else {
            "10"
        };
        let upper = upper
            .parse::<u32>()
            .expect("upper must be a non-negative integer");

        if lower > upper {
            panic!("lower cannot be greater than upper");
        }

        let mut result: u32;
        for _ in 0..lines {
            result = rng.gen_range(lower..upper);
            println!("{}", result);
        }
    }
}
