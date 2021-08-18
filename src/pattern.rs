use crate::sample;

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

#[derive(Debug)]
struct ParseError;

#[allow(dead_code)]
impl Pattern {
    // TODO: make this correct
    fn parse(string: &str) -> Result<Pattern, ParseError> {
        if can_parse_as_literal_kind(string) {
            return Ok(Pattern {
                value: String::from(string),
                kind: PatternKind::Literal,
                repetitions: 1,
            });
        }
        if can_parse_as_parentheses_kind(string) {
            return Ok(Pattern {
                value: String::from(string),
                kind: PatternKind::Parentheses,
                repetitions: 1,
            });
        }
        Err(ParseError)
    }

    // TODO: implement this
    fn to_string_sampler(&self) -> sample::StringSampler {
        sample::StringSampler {
            support: vec![self.value.clone()],
            repetitions: self.repetitions,
        }
    }
}

fn can_parse_as_literal_kind(string: &str) -> bool {
    let mut escaped_by_previous = false;
    for c in string.chars() {
        if escaped_by_previous {
            escaped_by_previous = false;
            continue;
        }
        if is_escape_character(c) {
            escaped_by_previous = true;
            continue;
        }
        if is_special_character(c) {
            return false;
        }
        escaped_by_previous = false;
    }
    true
}

fn can_parse_as_parentheses_kind(string: &str) -> bool {
    let (string, _) = pop_quantifier(string);
    if !string.starts_with('(') || !string.ends_with(')') || string.len() < 3 {
        return false;
    }
    let mut terminal_indexes: Vec<usize> = vec![0];
    let mut escaped_by_previous = false;
    for (i, c) in string.chars().enumerate() {
        if escaped_by_previous {
            escaped_by_previous = false;
            continue;
        }
        if is_escape_character(c) {
            escaped_by_previous = true;
            continue;
        }
        if c == '|' {
            terminal_indexes.push(i);
        }
        escaped_by_previous = false;
    }
    terminal_indexes.push(string.len() - 1);
    if terminal_indexes.len() == 2 {
        return can_parse_as_literal_kind(&string[terminal_indexes[0]..terminal_indexes[1]]);
    }
    let mut all_patterns_valid = true;
    for (i, p) in terminal_indexes.iter().enumerate() {
        if i == 0 {
            continue;
        }
        all_patterns_valid = all_patterns_valid
            && can_parse_as_literal_kind(&string[(terminal_indexes[i - 1] + 1)..*p]);
    }
    all_patterns_valid
}

fn is_special_character(character: char) -> bool {
    "()[]{}*\\|".chars().any(|c| c == character)
}

fn is_escape_character(character: char) -> bool {
    character == '\\'
}

#[allow(dead_code)]
fn pop_quantifier(string: &str) -> (&str, Option<u8>) {
    if !string.ends_with('}') {
        return (string, None);
    }
    let mut previous_was_open_brace = false;
    for (reflected_idx, c) in string.chars().rev().enumerate() {
        let idx = string.len() - reflected_idx - 1;
        if previous_was_open_brace && !is_escape_character(c) {
            let parsed_quantifier = string[(idx + 2)..(string.len() - 1)].parse::<u8>();
            match parsed_quantifier {
                Ok(value) => return (&string[..(idx + 1)], Some(value)),
                _ => return (string, None),
            };
        }
        previous_was_open_brace = c == '{';
    }
    (string, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_as_literal_valid() {
        let mut result: bool;
        for s in ["abc", "abc\\(", "\\(abc\\)", "123", "\\*\\{"] {
            result = can_parse_as_literal_kind(s);
            assert!(result)
        }
    }

    #[test]
    fn can_parse_as_literal_invalid() {
        let mut result: bool;
        for s in ["(abc)", "[123]", "\\[123]", "abc(1|2|3)"] {
            result = can_parse_as_literal_kind(s);
            assert!(!result)
        }
    }

    #[test]
    fn can_parse_as_parentheses_invalid() {
        let mut result: bool;
        for s in ["abc", "[abc]", "(abc", "abc)", "(abc)a"] {
            result = can_parse_as_parentheses_kind(s);
            assert!(!result)
        }
    }

    #[test]
    fn can_parse_as_parentheses_valid() {
        let mut result: bool;
        for s in ["(abc)", "(123)", "(abc){5}", "(a|b|c)"] {
            result = can_parse_as_parentheses_kind(s);
            assert!(result)
        }
    }

    #[test]
    fn parse_valid_literal_pattern() {
        let mut result: Pattern;
        let mut expected: Pattern;
        for value in [
            "abc",
            "a2c",
            "abc\\(",
            "\\(xyz",
            "012",
            "\\[abc\\]",
            "\\(abc\\)",
        ]
        .iter()
        {
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
    fn parse_valid_parentheses_pattern() {
        let mut result: Pattern;
        let mut expected: Pattern;
        for value in ["(abc)", "(abc\\*)", "(a|b|c)"].iter() {
            result = Pattern::parse(value).unwrap();
            expected = Pattern {
                value: String::from(*value),
                kind: PatternKind::Parentheses,
                repetitions: 1,
            };
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn parse_invalid_pattern() {
        let mut result: Result<Pattern, ParseError>;
        for value in [")abc", ")(", "["].iter() {
            result = Pattern::parse(value);
            assert!(result.is_err());
        }
    }

    #[test]
    fn check_special_characters() {
        for c in "()[]{}*\\|".chars() {
            assert!(is_special_character(c));
        }
    }

    #[test]
    fn check_non_special_characters() {
        for c in "AZaz09!@#".chars() {
            assert!(!is_special_character(c));
        }
    }

    #[test]
    fn check_pop_quantifier() {
        let mut result: (&str, Option<u8>);
        for (input, expected) in [
            ("(abc){5}", ("(abc)", Some(5))),
            ("[abc]{25}", ("[abc]", Some(25))),
            ("[123]{25}", ("[123]", Some(25))),
            ("[123]{00025}", ("[123]", Some(25))),
            ("[abc]\\{123}", ("[abc]\\{123}", None)),
            ("[abc]{123\\}", ("[abc]{123\\}", None)),
            ("[abc]\\{123\\}", ("[abc]\\{123\\}", None)),
            ("[abc]\\{}", ("[abc]\\{}", None)),
            ("[abc]{\\}", ("[abc]{\\}", None)),
            ("[abc]", ("[abc]", None)),
        ]
        .iter()
        {
            result = pop_quantifier(input);
            assert_eq!(result, *expected);
        }
    }
}
