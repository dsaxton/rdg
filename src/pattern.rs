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
impl Pattern {
    fn parse(string: &str) -> Pattern {
        Pattern {
            value: String::from(string),
            kind: PatternKind::Literal,
            repetitions: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pattern::{Pattern, PatternKind};

    #[test]
    fn parse_literal_pattern() {
        let result = Pattern::parse("abc");
        let expected = Pattern {
            value: String::from("abc"),
            kind: PatternKind::Literal,
            repetitions: 1,
        };
        assert_eq!(result, expected);
    }
}
