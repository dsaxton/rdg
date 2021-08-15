use clap::{App, AppSettings, Arg};

pub fn create_app() -> App<'static> {
    App::new("rd")
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
                .about("Random strings, default pattern \"[A-Za-z0-9]{10}\"")
                .setting(AppSettings::ColoredHelp)
                .arg(
                    Arg::new("pattern")
                        .short('p')
                        .long("pattern")
                        .value_name("string")
                        .about("Pattern from which to sample, default \"[A-Za-z0-9]{10}\"")
                        .takes_value(true),
                ),
        )
}
