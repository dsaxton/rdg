#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct Pattern {
    value: String,
    kind: PatternKind,
    repetitions: u8,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum PatternKind {
    Literal,
    Parentheses,
    Brackets,
    Compound,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct ParseError {
    message: String,
}

#[allow(dead_code)]
impl Pattern {
    fn parse(string: &str) -> Result<Pattern, ParseError> {
        if string.is_empty() {
            Ok(Pattern {
                value: String::from(""),
                kind: PatternKind::Literal,
                repetitions: 1,
            })
        } else {
            Err(ParseError {
                message: String::from("Failed to parse string"),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pattern::{Pattern, PatternKind};

    #[test]
    fn parse_literal_pattern() {
        let result = Pattern::parse("abc").unwrap();
        let expected = Pattern {
            value: String::from("abc"),
            kind: PatternKind::Literal,
            repetitions: 1,
        };
        assert_eq!(result, expected);
    }
}
