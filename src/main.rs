use clap::{App, Arg};

fn main() {
    let matches = App::new("rd")
        .version("0.1")
        .author("Daniel Saxton")
        .about("Generates random strings")
        .subcommand(
            // TODO: remove versions for subcommands
            App::new("float")
                .about("Generates random floats")
                .arg(Arg::new("min").short('m').long("min").about("Minimum value"))
                .arg(Arg::new("max").short('M').long("max").about("Maximum value")),
        )
        .subcommand(
            App::new("int")
                .about("Generates random ints")
                .arg(Arg::new("min").short('m').long("min").about("Minimum value"))
                .arg(Arg::new("max").short('M').long("max").about("Maximum value")),
        )
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("int") {
        println!("int subcommand");
        if matches.is_present("min") {
            println!("min was provided");
        }
        if matches.is_present("max") {
            println!("max was provided");
        }
    }

    if let Some(ref matches) = matches.subcommand_matches("float") {
        println!("float subcommand");
        if matches.is_present("min") {
            println!("min was provided");
        }
        if matches.is_present("max") {
            println!("max was provided");
        }
    }
}
