#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct Composite {
    value: String,
    repetitions: usize,
}
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct Literal {
    value: String,
    repetitions: usize,
}
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct Paren {
    value: String,
    repetitions: usize,
}
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct Bracket {
    value: String,
    repetitions: usize,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fails() {
        assert!(false);
    }
}
