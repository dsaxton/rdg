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
        if string.is_empty() {
            return false;
        }
        if !string.starts_with('(') {
            return false;
        }
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
        for c in ['(', ')', '[', ']', '{', '}', '*', '\\'].iter() {
            if character == *c {
                return true;
            }
        }
        false
    }

    fn parse_quantifier(string: &str) -> Result<u8, ParseError> {
        if !string.ends_with('}') {
            return Err(ParseError);
        }
        Ok(1)
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
        for c in ['(', ')', '[', ']', '{', '}', '*', '\\'].iter() {
            assert!(Pattern::is_special_character(*c));
        }
    }

    #[test]
    fn check_non_special_characters() {
        for c in ['A', 'Z', 'a', 'z', '0', '9', '!', '@', '#'].iter() {
            assert!(!Pattern::is_special_character(*c));
        }
    }

    #[test]
    fn parse_quantifier_invalid() {
        let mut result: Result<u8, ParseError>;
        for s in ["(abc)", "(abc)}", "(a|b|c){}}", "[abc]{", "(abc){x}"].iter() {
            result = Pattern::parse_quantifier(s);
            assert!(result.is_err());
        }
    }

    #[test]
    fn parse_quantifier_valid() {
        let mut result: u8;
        for (input, expected) in [("(abc){1}", 1), ("(abc){23}", 23), ("[A-Z]{3}", 3)].iter() {
            result = Pattern::parse_quantifier(input).unwrap();
            assert_eq!(result, *expected);
        }
    }
}
