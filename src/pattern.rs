use crate::sample;

#[derive(Debug, PartialEq)]
pub struct Pattern {
    value: String,
    kind: PatternKind,
    quantifier: u8,
}

#[derive(Debug, PartialEq)]
enum PatternKind {
    Literal,
    Parentheses {
        pipe_positions: Option<Vec<usize>>,
    },
    Brackets,
    #[allow(dead_code)]
    Compound,
}

#[derive(Debug)]
pub struct ParseError;

impl Pattern {
    pub fn parse(string: &str) -> Result<Pattern, ParseError> {
        if let Ok(pattern) = parse_as_literal_kind(string) {
            return Ok(pattern);
        }
        if let Ok(pattern) = parse_as_parentheses_kind(string) {
            return Ok(pattern);
        }
        if let Ok(pattern) = parse_as_brackets_kind(string) {
            return Ok(pattern);
        }
        parse_as_compound_kind(string)
    }

    pub fn to_string_sampler(&self) -> sample::StringSampler {
        match &self.kind {
            PatternKind::Literal => sample::StringSampler {
                support: vec![unescape(&self.value)],
                repetitions: self.quantifier,
            },
            PatternKind::Parentheses { pipe_positions } => match pipe_positions {
                Some(pipes) => {
                    let split_string = split_at_positions(&self.value, pipes);
                    sample::StringSampler {
                        support: split_string,
                        repetitions: self.quantifier,
                    }
                }
                None => sample::StringSampler {
                    support: vec![self.value.clone()],
                    repetitions: self.quantifier,
                },
            },
            PatternKind::Brackets => sample::StringSampler {
                support: unescape(&self.value)
                    .chars()
                    .map(|c| c.to_string())
                    .collect(),
                repetitions: self.quantifier,
            },
            _ => sample::StringSampler {
                support: vec![String::from("...")],
                repetitions: self.quantifier,
            },
        }
    }
}

pub fn parse_as_literal_kind(string: &str) -> Result<Pattern, ParseError> {
    let mut escaped_by_previous = false;
    for (i, c) in string.chars().enumerate() {
        if escaped_by_previous {
            escaped_by_previous = false;
            continue;
        }
        if is_escape_character(c) {
            if i < string.len() - 1 {
                escaped_by_previous = true;
                continue;
            }
            return Err(ParseError);
        }
        if is_special_character(c) {
            return Err(ParseError);
        }
        escaped_by_previous = false;
    }
    Ok(Pattern {
        kind: PatternKind::Literal,
        value: String::from(string),
        quantifier: 1,
    })
}

pub fn parse_as_parentheses_kind(string: &str) -> Result<Pattern, ParseError> {
    let (string, q) = pop_quantifier(string);
    let q = q.unwrap_or(1);
    let indexes = match find_parentheses_boundaries(string) {
        Ok(vec) => vec,
        Err(_) => return Err(ParseError),
    };
    if indexes.len() == 2 && parse_as_literal_kind(&string[(indexes[0] + 1)..indexes[1]]).is_ok() {
        return Ok(Pattern {
            value: String::from(&string[1..(string.len() - 1)]),
            kind: PatternKind::Parentheses {
                pipe_positions: None,
            },
            quantifier: q,
        });
    }
    for (i, p) in indexes.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if parse_as_literal_kind(&string[(indexes[i - 1] + 1)..*p]).is_err() {
            return Err(ParseError);
        }
    }
    Ok(Pattern {
        value: String::from(&string[1..(string.len() - 1)]),
        kind: PatternKind::Parentheses {
            pipe_positions: Some(
                indexes[1..(indexes.len() - 1)]
                    .iter()
                    .map(|i| i - 1)
                    .collect(),
            ),
        },
        quantifier: q,
    })
}

pub fn parse_as_compound_kind(string: &str) -> Result<Pattern, ParseError> {
    if string.is_empty() {
        return Ok(Pattern {
            value: String::from(string),
            kind: PatternKind::Compound,
            quantifier: 1,
        });
    }
    Err(ParseError)
}

pub fn parse_as_brackets_kind(string: &str) -> Result<Pattern, ParseError> {
    let (string, q) = pop_quantifier(string);
    let q = q.unwrap_or(1);
    if !string.starts_with('[') || !string.ends_with(']') {
        return Err(ParseError);
    }
    if parse_as_literal_kind(&string[1..(string.len() - 1)]).is_ok() {
        return Ok(Pattern {
            value: expand_ranges(&string[1..(string.len() - 1)]),
            kind: PatternKind::Brackets,
            quantifier: q,
        });
    }
    Err(ParseError)
}

pub fn find_parentheses_boundaries(string: &str) -> Result<Vec<usize>, ParseError> {
    if !string.starts_with('(') || !string.ends_with(')') {
        return Err(ParseError);
    }
    let mut indexes: Vec<usize> = vec![0];
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
            indexes.push(i);
        }
        escaped_by_previous = false;
    }
    indexes.push(string.len() - 1);
    Ok(indexes)
}

pub fn pop_quantifier(string: &str) -> (&str, Option<u8>) {
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

fn expand_ranges(string: &str) -> String {
    string
        .replace("A-Z", "ABCDEFGHIJKLMNOPQRSTUVWXYZ")
        .replace("0-9", "0123456789")
        .replace("a-z", "abcdefghijklmnopqrstuvwxyz")
}

fn unescape(string: &str) -> String {
    let mut result = String::from("");
    let mut escaped = false;
    for c in string.chars() {
        if !escaped && is_escape_character(c) {
            escaped = true;
            continue;
        }
        escaped = false;
        result.push(c);
    }
    result
}

fn split_at_positions(string: &str, positions: &[usize]) -> Vec<String> {
    if positions.is_empty() {
        return vec![String::from("")];
    }
    let mut split_string = Vec::new();
    for (i, d) in positions.iter().enumerate() {
        if i == 0 {
            split_string.push(String::from(&string[0..*d]));
        } else {
            split_string.push(String::from(&string[(positions[i - 1] + 1)..*d]));
        }
        if i == positions.len() - 1 {
            split_string.push(String::from(&string[(*d + 1)..]));
        }
    }
    split_string
}

fn is_special_character(character: char) -> bool {
    "()[]{}\\|".chars().any(|c| c == character)
}

fn is_escape_character(character: char) -> bool {
    character == '\\'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_as_literal_valid() {
        for s in [
            "abc",
            "abc\\(",
            "\\(abc",
            "\\(abc\\)",
            "abc\\]",
            "\\[abc",
            "\\[abc\\]",
            "123",
            "*\\{",
            "&#$",
            "ab-*",
            "...",
            "$^",
            "a2z#@",
            "",
            "\\\\",
        ] {
            assert_eq!(parse_as_literal_kind(s).unwrap().kind, PatternKind::Literal);
        }
    }

    #[test]
    fn can_parse_as_literal_invalid() {
        for s in [
            "(abc)", "\\(abc)", "(abc\\)", "abc)", "(abc", "[123]", "\\[123]", "[123\\]",
            "abc(123)", "(123)abc", ")(", "\\",
        ] {
            assert!(parse_as_literal_kind(s).is_err());
        }
    }

    #[test]
    fn can_parse_as_parentheses_valid() {
        for s in [
            "(abc)",
            "(123)",
            "(alice|bob)",
            "(alice|bob){2}",
            "(abc){5}",
            "(a|bc){5}",
            "(ab|c){5}",
            "(1&$#@){5}",
            "(1&|$#@){5}",
            "(a|b|c)",
            "(a|\\|)",
            "(a|2|$|%|^)",
            "(a|2|$|%|^){100}",
            "(12|a|-)",
            "(12|a|-){100}",
        ] {
            assert!(matches!(
                parse_as_parentheses_kind(s).unwrap().kind,
                PatternKind::Parentheses { .. },
            ))
        }
    }

    #[test]
    fn can_parse_as_parentheses_invalid() {
        for s in [
            "abc",
            "[abc]",
            "(abc",
            "abc)",
            "(abc))",
            "((abc)",
            "((abc))",
            "|abc)",
            "(abc|",
            "(abc)a",
            "a(abc)",
            "(abc){1}}",
            "{1}(abc)",
            "(abc){{1}",
        ] {
            assert!(parse_as_parentheses_kind(s).is_err())
        }
    }

    #[test]
    fn can_parse_as_brackets_valid() {
        let mut actual: Result<Pattern, ParseError>;
        for s in [
            "[abc]",
            "[abc]{10}",
            "[A-Z]",
            "[a-z]",
            "[0-9]",
            "[A-Za-z0-9]{3}",
            "[a&^#]",
            "[\\|]",
        ] {
            actual = parse_as_brackets_kind(s);
            assert!(actual.is_ok())
        }
    }

    #[test]
    fn can_parse_as_brackets_invalid() {
        let mut actual: Result<Pattern, ParseError>;
        for s in [
            "[abc\\]", "\\[abc]", "[()]", "[[]]", "[(]", "[)]", "[[]", "[]]", "[abc|]", "[|]",
            "[abc}]", "[{abc]",
        ] {
            actual = parse_as_brackets_kind(s);
            assert!(actual.is_err())
        }
    }

    #[test]
    fn parse_valid_literal_pattern() {
        let mut actual: Pattern;
        let mut expected: Pattern;
        for value in [
            "abc",
            "a2c",
            "ABC",
            "...",
            "$^",
            "#$@#",
            "abc\\(",
            "\\(xyz",
            "012",
            "\\[abc\\]",
            "\\(abc\\)",
        ]
        .iter()
        {
            actual = Pattern::parse(value).unwrap();
            expected = Pattern {
                value: String::from(*value),
                kind: PatternKind::Literal,
                quantifier: 1,
            };
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn parse_valid_parentheses_pattern() {
        let mut actual: Pattern;
        let mut expected: Pattern;
        for value in ["(a|b|c)", "(1|2|3)"] {
            actual = Pattern::parse(value).unwrap();
            expected = Pattern {
                value: String::from(&value[1..(value.len() - 1)]),
                kind: PatternKind::Parentheses {
                    pipe_positions: Some(vec![1, 3]),
                },
                quantifier: 1,
            };
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn parse_valid_parentheses_pattern_quantifier() {
        let actual = Pattern::parse("(a|b|c){5}").unwrap();
        let expected = Pattern {
            value: String::from("a|b|c"),
            kind: PatternKind::Parentheses {
                pipe_positions: Some(vec![1, 3]),
            },
            quantifier: 5,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_valid_parentheses_pattern_with_escape() {
        let actual = Pattern::parse("(a\\)bc){23}").unwrap();
        let expected = Pattern {
            value: String::from("a\\)bc"),
            kind: PatternKind::Parentheses {
                pipe_positions: None,
            },
            quantifier: 23,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_invalid_pattern() {
        let mut actual: Result<Pattern, ParseError>;
        for value in [")abc", ")(", "["].iter() {
            actual = Pattern::parse(value);
            assert!(actual.is_err());
        }
    }

    #[test]
    fn check_special_characters() {
        for c in "()[]{}\\|".chars() {
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
        let mut actual: (&str, Option<u8>);
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
            actual = pop_quantifier(input);
            assert_eq!(actual, *expected);
        }
    }

    #[test]
    fn check_find_parentheses_boundaries_valid() {
        let mut actual: Vec<usize>;
        for (input, expected) in [
            ("(abc)", vec![0, 4]),
            ("(a|b|c)", vec![0, 2, 4, 6]),
            ("(a|bbb|c)", vec![0, 2, 6, 8]),
        ] {
            actual = find_parentheses_boundaries(input).unwrap();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn check_find_parentheses_boundaries_invalid() {
        let mut actual: Result<Vec<usize>, ParseError>;
        for invalid in ["abc", "(abc", "abc)", "[a|b|c]"] {
            actual = find_parentheses_boundaries(invalid);
            assert!(actual.is_err())
        }
    }

    #[test]
    fn check_expand_ranges() {
        let mut result: String;
        for (input, expected) in [
            ("A-Z", "ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
            ("a-z", "abcdefghijklmnopqrstuvwxyz"),
            ("0-9", "0123456789"),
            ("a-z0-9", "abcdefghijklmnopqrstuvwxyz0123456789"),
            ("A-Z0-9", "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"),
            ("0-9abc", "0123456789abc"),
            ("abc0-9", "abc0123456789"),
            ("A-Z123", "ABCDEFGHIJKLMNOPQRSTUVWXYZ123"),
            ("123A-Z", "123ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
        ] {
            result = expand_ranges(input);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn check_unescape() {
        let mut result: String;
        for (input, expected) in [
            ("abc\\)", "abc)"),
            ("\\|", "|"),
            ("\\(abc\\)", "(abc)"),
            ("\\(abc", "(abc"),
            ("\\(abc\\]", "(abc]"),
            ("\\[abc\\]", "[abc]"),
        ] {
            result = unescape(input);
            assert_eq!(result, expected);
        }
    }
}
