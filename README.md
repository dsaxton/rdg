```
rd 0.1
Generate random strings at the command line

USAGE:
    rd [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --count <count>    Number of values in output, default 1
    -d, --delim <delim>    Delimiter to use between values, default \n

SUBCOMMANDS:
    float    Random floats, default support [0, 1)
    help     Prints this message or the help of the given subcommand(s)
    int      Random integers, default support {0, 1}
    word     Random words, requires a wordlist
```
