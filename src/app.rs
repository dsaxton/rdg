use clap::{App, AppSettings, Arg};

pub fn create_app() -> App<'static> {
    App::new("rdg")
        .version("0.1.1")
        .about("Generate random data at the command line")
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
        .subcommand(
            App::new("word")
                .about("Random words, requires a wordlist")
                .arg(
                    Arg::new("file")
                        .short('f')
                        .long("file")
                        .value_name("path")
                        .about("Wordlist used for sampling")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            App::new("string")
                .about("Random strings, default pattern [A-Za-z0-9]{10}")
                .arg(
                    Arg::new("pattern")
                        .short('p')
                        .long("pattern")
                        .value_name("string")
                        .about("Pattern from which to sample, default [A-Za-z0-9]{10}")
                        .takes_value(true),
                ),
        )
        .subcommand(
            App::new("int")
                .about("Random integers, default support {0, 1}")
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
}
