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
fn is_valid_parentheses_type(string: &str) -> bool {
    if string.is_empty() {
        return false;
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
        assert!(!pattern::is_special_char('z'));
        assert!(!pattern::is_special_char('A'));
        assert!(!pattern::is_special_char('Z'));
        assert!(!pattern::is_special_char('0'));
        assert!(!pattern::is_special_char('9'));
    }

    #[test]
    fn star_is_special() {
        assert!(pattern::is_special_char('*'));
    }

    #[test]
    fn slash_is_special() {
        assert!(pattern::is_special_char('\\'));
    }

    #[test]
    fn alphanumerics_are_literal() {
        let mut result: bool;
        result = pattern::is_valid_literal("abc");
        assert!(result);

        result = pattern::is_valid_literal("123");
        assert!(result);
    }

    #[test]
    fn trailing_escape_is_not_literal() {
        assert!(!pattern::is_valid_literal("abc\\"));
    }

    #[test]
    fn literal_is_not_parentheses() {
        assert!(!pattern::is_valid_parentheses_type(""));
    }
}
