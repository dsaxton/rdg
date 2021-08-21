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
    Parentheses,
    Brackets,
    #[allow(dead_code)]
    Compound,
}

#[derive(Debug)]
pub struct ParseError;

impl Pattern {
    // TODO: make this correct
    pub fn parse(string: &str) -> Result<Pattern, ParseError> {
        let (string, q) = pop_quantifier(string);
        let q = q.unwrap_or(1);
        if can_parse_as_literal_kind(string) {
            return Ok(Pattern {
                value: String::from(string),
                kind: PatternKind::Literal,
                quantifier: q,
            });
        }
        if can_parse_as_parentheses_kind(string) {
            return Ok(Pattern {
                value: String::from(&string[1..(string.len() - 1)]),
                kind: PatternKind::Parentheses,
                quantifier: q,
            });
        }
        if can_parse_as_brackets_kind(string) {
            return Ok(Pattern {
                value: expand_ranges(&string[1..(string.len() - 1)]),
                kind: PatternKind::Brackets,
                quantifier: q,
            });
        }
        Err(ParseError)
    }

    // TODO: implement for composite
    pub fn to_string_sampler(&self) -> sample::StringSampler {
        match &self {
            Pattern {
                value,
                kind: PatternKind::Literal,
                quantifier,
            } => sample::StringSampler {
                support: vec![unescape(value)],
                repetitions: *quantifier,
            },
            Pattern {
                value,
                kind: PatternKind::Brackets,
                quantifier,
            } => sample::StringSampler {
                support: unescape(value).chars().map(|c| c.to_string()).collect(),
                repetitions: *quantifier,
            },
            _ => sample::StringSampler {
                support: vec![String::from("...")],
                repetitions: 1,
            },
        }
    }
}

pub fn can_parse_as_literal_kind(string: &str) -> bool {
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
            return false;
        }
        if is_special_character(c) {
            return false;
        }
        escaped_by_previous = false;
    }
    true
}

pub fn can_parse_as_parentheses_kind(string: &str) -> bool {
    let (string, _) = pop_quantifier(string);
    // TODO: pull this outside the function so indexes can be reused?
    let indexes = match find_parentheses_boundaries(string) {
        Ok(vec) => vec,
        Err(_) => return false,
    };
    if indexes.len() == 2 {
        return can_parse_as_literal_kind(&string[(indexes[0] + 1)..indexes[1]]);
    }
    for (i, p) in indexes.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if !can_parse_as_literal_kind(&string[(indexes[i - 1] + 1)..*p]) {
            return false;
        }
    }
    true
}

pub fn can_parse_as_brackets_kind(string: &str) -> bool {
    let (string, _) = pop_quantifier(string);
    if !string.starts_with('[') || !string.ends_with(']') {
        return false;
    }
    can_parse_as_literal_kind(&string[1..(string.len() - 1)])
}

#[allow(dead_code)]
fn can_parse_as_compound_kind(string: &str) -> bool {
    // TODO: fix this
    !string.is_empty()
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
        let mut result: bool;
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
            result = can_parse_as_literal_kind(s);
            assert!(result)
        }
    }

    #[test]
    fn can_parse_as_literal_invalid() {
        let mut result: bool;
        for s in [
            "(abc)", "\\(abc)", "(abc\\)", "abc)", "(abc", "[123]", "\\[123]", "[123\\]",
            "abc(123)", "(123)abc", ")(", "\\",
        ] {
            result = can_parse_as_literal_kind(s);
            assert!(!result)
        }
    }

    #[test]
    fn can_parse_as_parentheses_valid() {
        let mut result: bool;
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
            result = can_parse_as_parentheses_kind(s);
            assert!(result)
        }
    }

    #[test]
    fn can_parse_as_parentheses_invalid() {
        let mut result: bool;
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
            result = can_parse_as_parentheses_kind(s);
            assert!(!result)
        }
    }

    #[test]
    fn can_parse_as_brackets_valid() {
        let mut result: bool;
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
            result = can_parse_as_brackets_kind(s);
            assert!(result)
        }
    }

    #[test]
    fn can_parse_as_brackets_invalid() {
        let mut result: bool;
        for s in [
            "[abc\\]", "\\[abc]", "[()]", "[[]]", "[(]", "[)]", "[[]", "[]]", "[abc|]", "[|]",
            "[abc}]", "[{abc]",
        ] {
            result = can_parse_as_brackets_kind(s);
            assert!(!result)
        }
    }

    #[test]
    fn can_parse_as_compound_valid() {
        let mut result: bool;
        for s in ["abc[0-9]{10}"] {
            result = can_parse_as_compound_kind(s);
            assert!(result)
        }
    }

    #[test]
    fn can_parse_as_compound_invalid() {
        let mut result: bool;
        for s in ["abc[[0-9]{10}"] {
            result = can_parse_as_compound_kind(s);
            assert!(!result)
        }
    }

    #[test]
    fn parse_valid_literal_pattern() {
        let mut result: Pattern;
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
            result = Pattern::parse(value).unwrap();
            expected = Pattern {
                value: String::from(*value),
                kind: PatternKind::Literal,
                quantifier: 1,
            };
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn parse_valid_parentheses_pattern() {
        let mut result: Pattern;
        let mut expected: Pattern;
        for value in ["(abc)", "(abc\\*)", "(a|b|c)"] {
            result = Pattern::parse(value).unwrap();
            expected = Pattern {
                value: String::from(&value[1..(value.len() - 1)]),
                kind: PatternKind::Parentheses,
                quantifier: 1,
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

    #[test]
    fn check_find_parentheses_boundaries() {
        let mut valid_result: Vec<usize>;
        for (input, expected) in [
            ("(abc)", vec![0, 4]),
            ("(a|b|c)", vec![0, 2, 4, 6]),
            ("(a|bbb|c)", vec![0, 2, 6, 8]),
        ] {
            valid_result = find_parentheses_boundaries(input).unwrap();
            assert_eq!(valid_result, expected);
        }

        let mut invalid_result: Result<Vec<usize>, ParseError>;
        for invalid in ["abc", "(abc", "abc)", "[a|b|c]"] {
            invalid_result = find_parentheses_boundaries(invalid);
            assert!(invalid_result.is_err())
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
