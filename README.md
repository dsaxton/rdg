## `rd`

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

## `rd string`

```
rd-string
Random strings, default length 10

USAGE:
    rd string [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --length <integer>    Length of string, default 10
```

## `rd word`

```
rd-word
Random words, requires a wordlist

USAGE:
    rd word --wordlist <path>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -w, --wordlist <path>    Wordlist used for sampling
```

## `rd float`

```
rd-float
Random floating point numbers, default support [0, 1)

USAGE:
    rd float [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --lower <integer>    Lower bound (inclusive), default 0
    -u, --upper <integer>    Upper bound (exclusive), default 1
```

## `rd int`

```
rd-int
Random integers, default support {0, 1}

USAGE:
    rd int [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --lower <integer>    Lower bound (inclusive), default 0
    -u, --upper <integer>    Upper bound (exclusive), default 2
```
