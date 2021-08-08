```
rd 0.1
Generate random data at the command line

USAGE:
    rd [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --count <integer>       Number of values to generate, default 1
    -d, --delimiter <string>    Delimiter to use between values, default \n
    -t, --threads <integer>     Number of threads, default 1

SUBCOMMANDS:
    float     Random floating point numbers, default support [0, 1)
    int       Random integers, default support {0, 1}
    string    Random strings, default length 10
    word      Random words, requires a wordlist
```
