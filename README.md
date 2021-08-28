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

SUBCOMMANDS:
    float     Random floating point numbers, default support [0, 1)
    int       Random integers, default support {0, 1}
    string    Random strings, default pattern [A-Za-z0-9]{10}
    word      Random words, requires a wordlist
```

## `rd string`

```
rd-string
Random strings, default pattern [A-Za-z0-9]{10}

USAGE:
    rd string [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --pattern <string>    Pattern from which to sample, default [A-Za-z0-9]{10}
```

## `rd word`

```
rd-word
Random words, requires a wordlist

USAGE:
    rd word --file <path>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <path>    Wordlist used for sampling
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

## Examples

```shell
$ rd string --pattern "[A-Za-z0-9]{30}"
7XVzCeHizkRmoRUVC5ye5FYuBGMgm5
```

```shell
$ rd --count 10 string --pattern "(bob|alice)[0-9]{3}@example.com"
alice929@example.com
bob431@example.com
alice974@example.com
alice391@example.com
alice545@example.com
```

```shell
$ rd --count 5 word --file /usr/share/dict/american-english
gels
defended
shorts
forename
strengthen
```
