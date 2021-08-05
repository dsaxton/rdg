```
rd 0.1
Generate random data at the command line

USAGE:
    rd [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --count <count>    Number of values in output, default 1
    -d, --delim <delim>    Delimiter to use between values, default \n

SUBCOMMANDS:
    float     Random floats, default support [0, 1)
    int       Random integers, default support {0, 1}
    string    Random strings, default length 8
    word      Random words, requires a wordlist
```
