## `rdg`

```
rdg 0.1
Generate random data at the command line

USAGE:
    rdg [OPTIONS] [SUBCOMMAND]

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

## `rdg string`

```
rdg-string
Random strings, default pattern [A-Za-z0-9]{10}

USAGE:
    rdg string [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --pattern <string>    Pattern from which to sample, default [A-Za-z0-9]{10}
```

## `rdg word`

```
rdg-word
Random words, requires a wordlist

USAGE:
    rdg word --file <path>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <path>    Wordlist used for sampling
```

## `rdg float`

```
rdg-float
Random floating point numbers, default support [0, 1)

USAGE:
    rdg float [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --lower <integer>    Lower bound (inclusive), default 0
    -u, --upper <integer>    Upper bound (exclusive), default 1
```

## `rdg int`

```
rdg-int
Random integers, default support {0, 1}

USAGE:
    rdg int [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --lower <integer>    Lower bound (inclusive), default 0
    -u, --upper <integer>    Upper bound (exclusive), default 2
```

# Examples

```shell
$ rdg string --pattern "[A-Za-z0-9]{30}"
7XVzCeHizkRmoRUVC5ye5FYuBGMgm5
```

```shell
$ rdg --count 5 string --pattern "(bob|alice)[0-9]{3}@example.com"
alice929@example.com
bob431@example.com
alice974@example.com
alice391@example.com
alice545@example.com
```

```shell
$ rdg --count 5 word --file /usr/share/dict/american-english
gels
defended
shorts
forename
strengthen
```
