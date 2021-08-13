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
    // TODO: make this correct
    fn parse(string: &str) -> Option<Pattern> {
        if Pattern::is_valid_literal_type(string) {
            return Some(Pattern {
                value: String::from(string),
                kind: PatternKind::Literal,
                repetitions: 1,
            });
        }
        if Pattern::is_valid_parentheses_type(string) {
            return Some(Pattern {
                value: String::from(string),
                kind: PatternKind::Parentheses,
                repetitions: 1,
            });
        }
        None
    }

    fn is_valid_literal_type(string: &str) -> bool {
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
            if Pattern::is_special_char(c) {
                return false;
            }
            escaped = false;
        }
        true
    }

    fn is_valid_parentheses_type(string: &str) -> bool {
        if string.is_empty() {
            return false;
        }

        if !string.starts_with('(') || !string.ends_with(')') {
            return false;
        }

        true // FIXME: this is wrong
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
}

#[cfg(test)]
mod tests {
    use crate::pattern::{Pattern, PatternKind};

    #[test]
    fn parse_valid_literal_pattern() {
        let mut result: Pattern;
        let mut expected: Pattern;
        for value in ["abc", "a2c", "abc\\(", "\\(xyz", "012"].iter() {
            result = Pattern::parse(value).unwrap();
            expected = Pattern {
                value: String::from(*value),
                kind: PatternKind::Literal,
                repetitions: 1,
            };
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn parse_invalid_pattern() {
        let mut result: Option<Pattern>;
        for value in [")abc", ")(", "["].iter() {
            result = Pattern::parse(value);
            assert!(result.is_none());
        }
    }

    #[test]
    fn parens_are_special() {
        assert!(Pattern::is_special_char('('));
        assert!(Pattern::is_special_char(')'));
    }

    #[test]
    fn brackets_are_special() {
        assert!(Pattern::is_special_char('['));
        assert!(Pattern::is_special_char(']'));
    }

    #[test]
    fn alphanumerics_are_not_special() {
        assert!(!Pattern::is_special_char('a'));
        assert!(!Pattern::is_special_char('z'));
        assert!(!Pattern::is_special_char('A'));
        assert!(!Pattern::is_special_char('Z'));
        assert!(!Pattern::is_special_char('0'));
        assert!(!Pattern::is_special_char('9'));
    }

    #[test]
    fn star_is_special() {
        assert!(Pattern::is_special_char('*'));
    }

    #[test]
    fn slash_is_special() {
        assert!(Pattern::is_special_char('\\'));
    }
}
