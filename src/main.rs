use clap::{App, Arg};
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let matches = App::new("rd")
        .version("0.1")
        .author("Daniel Saxton")
        .about("Generate random strings")
        .subcommand(
            // TODO: remove versions for subcommands
            App::new("int")
                .about("Generate random ints")
                // TODO: how to enforce data types?
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

    if let Some(ref matches) = matches.subcommand_matches("int") {
        let lower = if let Some(l) = matches.value_of("lower") {
            l
        } else {
            "0"
        };
        // TODO: deal with panic when cannot parse
        let lower = lower.parse::<u8>().unwrap();

        let upper = if let Some(u) = matches.value_of("upper") {
            u
        } else {
            "10"
        };
        // TODO: deal with panic when cannot parse
        let upper = upper.parse::<u8>().unwrap();

        // TODO: validate that lower < upper

        let result: u8 = rng.gen_range(lower..upper);
        println!("{}", result);
    }
}
