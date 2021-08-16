use rand::{thread_rng, Rng};

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
        if Pattern::can_parse_as_literal_type(string) {
            return Ok(Pattern {
                value: String::from(string),
                kind: PatternKind::Literal,
                repetitions: 1,
            });
        }
        if Pattern::can_parse_as_parentheses_type(string) {
            return Ok(Pattern {
                value: String::from(string),
                kind: PatternKind::Parentheses,
                repetitions: 1,
            });
        }
        Err(ParseError)
    }

    // TODO: implement this
    fn to_string_sampler(&self) -> StringSampler {
        StringSampler {
            support: vec![self.value.clone()],
            repetitions: self.repetitions,
        }
    }

    fn can_parse_as_literal_type(string: &str) -> bool {
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
            if Pattern::is_special_character(c) {
                return false;
            }
            escaped = false;
        }
        true
    }

    fn can_parse_as_parentheses_type(string: &str) -> bool {
        // need to pop quantifier off the end
        if !string.starts_with('(') {
            return false;
        }
        // what about pipes?
        if string.ends_with(')') {
            if Pattern::can_parse_as_literal_type(&string[1..(string.len() - 1)]) {
                return true;
            }
            return false;
        }
        // split on every unescaped pipe, remove surrounding parens and ensure each pattern is literal
        false
    }

    fn is_special_character(character: char) -> bool {
        "()[]{}*\\"
            .chars()
            .map(|c| c == character)
            .reduce(|a, b| a || b)
            .unwrap()
    }

    fn pop_quantifier(string: &str) -> (&str, Option<u8>) {
        if !string.ends_with('}') {
            return (string, None);
        }
        let mut opening_brace_found = false;
        // walk the string from right to left and look for an opening brace
        // once we know it's unescaped try to parse the string in between and
        // if successful return this value along with the truncated input string
        for (reflected_idx, c) in string.chars().rev().enumerate() {
            let idx = string.len() - reflected_idx - 1;
            if opening_brace_found && c != '\\' {
                let parsed_quantifier = string[(idx + 2)..(string.len() - 1)].parse::<u8>();
                match parsed_quantifier {
                    Ok(value) => return (&string[..(idx + 1)], Some(value)),
                    _ => return (string, None),
                };
            }
            opening_brace_found = false;
            if c == '{' && idx > 0 {
                opening_brace_found = true;
            }
        }
        (string, None)
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct StringSampler {
    support: Vec<String>,
    repetitions: u8,
}

#[allow(dead_code)]
impl StringSampler {
    fn sample(&self) -> String {
        let mut result = String::from("");
        let mut idx: usize;
        for _ in 0..self.repetitions {
            idx = (thread_rng().gen::<f64>() * (self.support.len() as f64)).floor() as usize;
            result.push_str(&self.support[idx])
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_sample() {
        let mut sampler: StringSampler;
        let mut result: String;

        sampler = StringSampler {
            support: vec![String::from("abc")],
            repetitions: 1,
        };
        result = sampler.sample();
        assert_eq!(result, String::from("abc"));

        sampler = StringSampler {
            support: vec![String::from("abc")],
            repetitions: 3,
        };
        result = sampler.sample();
        assert_eq!(result, String::from("abcabcabc"));

        sampler = StringSampler {
            support: vec![String::from("a"), String::from("z")],
            repetitions: 2,
        };
        result = sampler.sample();
        assert!(result == *"aa" || result == *"zz" || result == *"az" || result == *"za");
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
        for c in "()[]{}*\\".chars() {
            assert!(Pattern::is_special_character(c));
        }
    }

    #[test]
    fn check_non_special_characters() {
        for c in "AZaz09!@#".chars() {
            assert!(!Pattern::is_special_character(c));
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
            ("[abc]\\{}", ("[abc]\\{}", None)),
            ("[abc]\\{25\\}", ("[abc]\\{25\\}", None)),
            ("[abc]", ("[abc]", None)),
        ]
        .iter()
        {
            result = Pattern::pop_quantifier(input);
            assert_eq!(result, *expected);
        }
    }
}
