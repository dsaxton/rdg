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
fn is_special_char(character: char) -> bool {
    for c in ['(', ')', '[', ']', '*', '\\'].iter() {
        if character == *c {
            return true;
        }
    }
    false
}

#[allow(dead_code)]
fn is_valid_literal(string: &str) -> bool {
    let mut escaped = false;
    for (i, c) in string.chars().enumerate() {
        if escaped {
            escaped = false;
            continue;
        }
        if c == '\\' && i < string.len() - 1 {
            escaped = true;
            continue;
        }
        if is_special_char(c) {
            return false;
        }
        escaped = false;
    }
    true
}

#[allow(dead_code)]
impl Pattern {
    // TODO: make this correct
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
    use crate::pattern;
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

    #[test]
    fn parens_are_special() {
        assert!(pattern::is_special_char('('));
        assert!(pattern::is_special_char(')'));
    }

    #[test]
    fn brackets_are_special() {
        assert!(pattern::is_special_char('['));
        assert!(pattern::is_special_char(']'));
    }

    #[test]
    fn alphanumerics_are_not_special() {
        assert!(!pattern::is_special_char('a'));
        assert!(!pattern::is_special_char('1'));
    }

    #[test]
    fn star_is_special() {
        assert!(pattern::is_special_char('*'));
    }

    #[test]
    fn slash_is_special() {
        assert!(pattern::is_special_char('\\'));
    }
}
