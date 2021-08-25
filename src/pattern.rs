use crate::sample;

#[derive(Debug, PartialEq)]
pub struct Pattern {
    pub subpatterns: Vec<SubPattern>,
}

#[derive(Debug, PartialEq)]
pub struct SubPattern {
    value: String,
    kind: SubPatternKind,
    quantifier: u8,
}

#[derive(Debug, PartialEq, Clone)]
enum SubPatternKind {
    Literal,
    Brackets,
    Parentheses { pipe_positions: Option<Vec<usize>> },
}

#[derive(Debug)]
pub struct ParseError;

impl Pattern {
    pub fn parse(string: &str) -> Result<Pattern, ParseError> {
        let mut subpatterns: Vec<SubPattern> = vec![];
        let mut idx: usize = 0;
        while idx < string.len() {
            match pop_subpattern(&string[idx..]) {
                Some((p, i)) => {
                    idx += i + 1;
                    subpatterns.push(p);
                }
                None => return Err(ParseError),
            }
        }
        Ok(Pattern { subpatterns })
    }

    pub fn to_string_sampler(&self) -> sample::StringSampler {
        let mut support: Vec<Vec<String>> = vec![];
        let mut repetitions: Vec<u8> = vec![];
        for p in &self.subpatterns {
            repetitions.push(p.quantifier);
            match &p.kind {
                SubPatternKind::Literal => {
                    support.push(vec![unescape(&p.value)]);
                }
                SubPatternKind::Brackets => {
                    support.push(unescape(&p.value).chars().map(|c| c.to_string()).collect());
                }
                SubPatternKind::Parentheses { pipe_positions } => match pipe_positions {
                    Some(pipes) => {
                        let split_string = split_at_positions(&p.value, pipes);
                        support.push(split_string.iter().map(|s| unescape(s)).collect());
                    }
                    None => {
                        support.push(vec![unescape(&p.value)]);
                    }
                },
            }
        }
        sample::StringSampler {
            support,
            repetitions,
        }
    }
}

pub fn parse_as_literal_kind(string: &str) -> Result<SubPattern, ParseError> {
    let mut escaped = false;
    for (i, c) in string.chars().enumerate() {
        if escaped {
            escaped = false;
            continue;
        }
        if is_escape_character(c) {
            if i < string.len() - 1 {
                escaped = true;
                continue;
            }
            return Err(ParseError);
        }
        if is_special_character(c) {
            return Err(ParseError);
        }
        escaped = false;
    }
    Ok(SubPattern {
        kind: SubPatternKind::Literal,
        value: String::from(string),
        quantifier: 1,
    })
}

pub fn parse_as_brackets_kind(string: &str) -> Result<SubPattern, ParseError> {
    let (string, q) = pop_quantifier(string);
    let q = q.unwrap_or(1);
    if !string.starts_with('[') || !string.ends_with(']') {
        return Err(ParseError);
    }
    if parse_as_literal_kind(&string[1..(string.len() - 1)]).is_ok() {
        return Ok(SubPattern {
            value: expand_ranges(&string[1..(string.len() - 1)]),
            kind: SubPatternKind::Brackets,
            quantifier: q,
        });
    }
    Err(ParseError)
}

pub fn parse_as_parentheses_kind(string: &str) -> Result<SubPattern, ParseError> {
    let (string, q) = pop_quantifier(string);
    let q = q.unwrap_or(1);
    let indexes = match find_parentheses_boundaries(string) {
        Ok(vec) => vec,
        Err(_) => return Err(ParseError),
    };
    if indexes.len() == 2 && parse_as_literal_kind(&string[(indexes[0] + 1)..indexes[1]]).is_ok() {
        return Ok(SubPattern {
            value: String::from(&string[1..(string.len() - 1)]),
            kind: SubPatternKind::Parentheses {
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
    Ok(SubPattern {
        value: String::from(&string[1..(string.len() - 1)]),
        kind: SubPatternKind::Parentheses {
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

pub fn seek_to_unescaped(string: &str, cs: Vec<char>) -> usize {
    let mut escaped = false;
    for (i, a) in string.chars().enumerate() {
        if escaped {
            escaped = false;
            continue;
        }
        if is_escape_character(a) {
            escaped = true;
            continue;
        }
        if cs.iter().any(|b| a == *b) {
            return i;
        }
    }
    string.len()
}

pub fn find_parentheses_boundaries(string: &str) -> Result<Vec<usize>, ParseError> {
    if !string.starts_with('(') || !string.ends_with(')') {
        return Err(ParseError);
    }
    let mut indexes: Vec<usize> = vec![0];
    let mut escaped_ = false;
    for (i, c) in string.chars().enumerate() {
        if escaped_ {
            escaped_ = false;
            continue;
        }
        if is_escape_character(c) {
            escaped_ = true;
            continue;
        }
        if c == '|' {
            indexes.push(i);
        }
        escaped_ = false;
    }
    indexes.push(string.len() - 1);
    Ok(indexes)
}

pub fn pop_subpattern(string: &str) -> Option<(SubPattern, usize)> {
    if string.is_empty() {
        return None;
    }
    let chars = string.chars().collect::<Vec<_>>();
    let parse_function = |s| match chars[0] {
        '(' => parse_as_parentheses_kind(s),
        '[' => parse_as_brackets_kind(s),
        _ => parse_as_literal_kind(s),
    };
    let closing_char = match chars[0] {
        '(' => ')',
        '[' => ']',
        _ => '_',
    };
    match chars[0] {
        '(' | '[' => {
            let end_idx = seek_to_unescaped(string, vec![closing_char]);
            if end_idx == string.len() {
                return None;
            }
            if end_idx == string.len() - 1
                || (end_idx < string.len() - 1 && chars[end_idx + 1] != '{')
            {
                match parse_function(&string[..(end_idx + 1)]) {
                    Ok(pattern) => return Some((pattern, end_idx)),
                    Err(_) => return None,
                }
            }
            let closing_brace_idx =
                seek_to_unescaped(&string[(end_idx + 1)..], vec!['}']) + end_idx + 1;
            if closing_brace_idx == string.len() {
                return None;
            }
            match parse_function(&string[..(closing_brace_idx + 1)]) {
                Ok(pattern) => Some((pattern, closing_brace_idx)),
                Err(_) => None,
            }
        }
        _ => {
            let next_idx = seek_to_unescaped(string, vec!['(', '[']);
            match parse_as_literal_kind(&string[..next_idx]) {
                Ok(pattern) => Some((pattern, next_idx - 1)),
                Err(_) => None,
            }
        }
    }
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
        return vec![String::from(string)];
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
            assert_eq!(
                parse_as_literal_kind(s).unwrap().kind,
                SubPatternKind::Literal
            );
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
                SubPatternKind::Parentheses { .. },
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
            assert!(parse_as_brackets_kind(s).is_ok())
        }
    }

    #[test]
    fn can_parse_as_brackets_invalid() {
        for s in [
            "[abc\\]", "\\[abc]", "[()]", "[[]]", "[(]", "[)]", "[[]", "[]]", "[abc|]", "[|]",
            "[abc}]", "[{abc]",
        ] {
            assert!(parse_as_brackets_kind(s).is_err())
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
                subpatterns: vec![SubPattern {
                    value: String::from(*value),
                    kind: SubPatternKind::Literal,
                    quantifier: 1,
                }],
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
                subpatterns: vec![SubPattern {
                    value: String::from(&value[1..(value.len() - 1)]),
                    kind: SubPatternKind::Parentheses {
                        pipe_positions: Some(vec![1, 3]),
                    },
                    quantifier: 1,
                }],
            };
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn parse_valid_parentheses_pattern_with_quantifier() {
        let actual = Pattern::parse("(a|b|c){5}").unwrap();
        let expected = Pattern {
            subpatterns: vec![SubPattern {
                value: String::from("a|b|c"),
                kind: SubPatternKind::Parentheses {
                    pipe_positions: Some(vec![1, 3]),
                },
                quantifier: 5,
            }],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_valid_parentheses_pattern_with_escape() {
        let actual = Pattern::parse("(a\\)bc){23}").unwrap();
        let expected = Pattern {
            subpatterns: vec![SubPattern {
                value: String::from("a\\)bc"),
                kind: SubPatternKind::Parentheses {
                    pipe_positions: None,
                },
                quantifier: 23,
            }],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_valid_compound_pattern() {
        let actual = Pattern::parse("abc[123]").unwrap();
        let expected = Pattern {
            subpatterns: vec![
                SubPattern {
                    value: String::from("abc"),
                    kind: SubPatternKind::Literal,
                    quantifier: 1,
                },
                SubPattern {
                    value: String::from("123"),
                    kind: SubPatternKind::Brackets,
                    quantifier: 1,
                },
            ],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_invalid_pattern() {
        for value in [")abc", ")(", "["].iter() {
            assert!(Pattern::parse(value).is_err());
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
            assert_eq!(pop_quantifier(input), *expected);
        }
    }

    #[test]
    fn check_find_parentheses_boundaries_valid() {
        for (input, expected) in [
            ("(abc)", vec![0, 4]),
            ("(a|b|c)", vec![0, 2, 4, 6]),
            ("(a|bbb|c)", vec![0, 2, 6, 8]),
        ] {
            assert_eq!(find_parentheses_boundaries(input).unwrap(), expected);
        }
    }

    #[test]
    fn check_find_parentheses_boundaries_invalid() {
        for invalid in ["abc", "(abc", "abc)", "[a|b|c]"] {
            assert!(find_parentheses_boundaries(invalid).is_err())
        }
    }

    #[test]
    fn check_expand_ranges() {
        let mut actual: String;
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
            ("-AZ", "-AZ"),
            ("AZ-", "AZ-"),
            ("Z-A", "Z-A"),
            ("az-", "az-"),
            ("-az", "-az"),
            ("z-a", "z-a"),
            ("09-", "09-"),
            ("-09", "-09"),
            ("9-0", "9-0"),
        ] {
            actual = expand_ranges(input);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn check_unescape() {
        for (input, expected) in [
            ("abc\\)", "abc)"),
            ("\\|", "|"),
            ("\\(abc\\)", "(abc)"),
            ("\\(abc", "(abc"),
            ("\\(abc\\]", "(abc]"),
            ("\\[abc\\]", "[abc]"),
            ("\\\\", "\\"),
        ] {
            assert_eq!(unescape(input), expected);
        }
    }

    #[test]
    fn check_seek_to_unescaped() {
        for (input, target, expected) in [
            ("(abc)", vec![')'], 4),
            ("[abc]", vec![']'], 4),
            ("abc\\[[", vec!['['], 5),
            ("abc", vec!['['], 3),
            ("12{", vec!['{'], 2),
            ("abc(|", vec!['|'], 4),
            ("abc(|", vec!['|', '('], 3),
        ] {
            assert_eq!(seek_to_unescaped(input, target), expected);
        }
    }

    #[test]
    fn check_split_at_positions() {
        let mut actual: Vec<String>;
        let mut expected: Vec<String>;
        actual = split_at_positions("abc", &[1]);
        expected = vec![String::from("a"), String::from("c")];
        assert_eq!(actual, expected);

        actual = split_at_positions("abc", &[]);
        expected = vec![String::from("abc")];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_pop_subpattern_parentheses() {
        let actual = pop_subpattern("(abc)").unwrap();
        let expected = (
            SubPattern {
                value: String::from("abc"),
                kind: SubPatternKind::Parentheses {
                    pipe_positions: None,
                },
                quantifier: 1,
            },
            4,
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_pop_subpattern_parentheses_quantifier() {
        let actual = pop_subpattern("(abc){5}").unwrap();
        let expected = (
            SubPattern {
                value: String::from("abc"),
                kind: SubPatternKind::Parentheses {
                    pipe_positions: None,
                },
                quantifier: 5,
            },
            7,
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_pop_subpattern_parentheses_additional_pattern() {
        let actual = pop_subpattern("(abc)[xyz]").unwrap();
        let expected = (
            SubPattern {
                value: String::from("abc"),
                kind: SubPatternKind::Parentheses {
                    pipe_positions: None,
                },
                quantifier: 1,
            },
            4,
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_pop_subpattern_invalid() {
        for input in ["(abc){5", "(abc){z}", "abc]"] {
            assert!(pop_subpattern(input).is_none());
        }
    }

    #[test]
    fn check_pop_subpattern_brackets() {
        let actual = pop_subpattern("[abc]").unwrap();
        let expected = (
            SubPattern {
                value: String::from("abc"),
                kind: SubPatternKind::Brackets,
                quantifier: 1,
            },
            4,
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_pop_subpattern_brackets_with_quantifier() {
        let actual = pop_subpattern("[abc]{15}").unwrap();
        let expected = (
            SubPattern {
                value: String::from("abc"),
                kind: SubPatternKind::Brackets,
                quantifier: 15,
            },
            8,
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_pop_subpattern_brackets_with_quantifier_extra_pattern() {
        let actual = pop_subpattern("[abc]{15}xxx").unwrap();
        let expected = (
            SubPattern {
                value: String::from("abc"),
                kind: SubPatternKind::Brackets,
                quantifier: 15,
            },
            8,
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_pop_subpattern_literal() {
        let actual = pop_subpattern("abc123").unwrap();
        let expected = (
            SubPattern {
                value: String::from("abc123"),
                kind: SubPatternKind::Literal,
                quantifier: 1,
            },
            5,
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_pop_subpattern_literal_with_another() {
        let actual = pop_subpattern("abc123(a|b|c)").unwrap();
        let expected = (
            SubPattern {
                value: String::from("abc123"),
                kind: SubPatternKind::Literal,
                quantifier: 1,
            },
            5,
        );
        assert_eq!(actual, expected);
    }
}
