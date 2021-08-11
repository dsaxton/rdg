#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Pattern {
    Composite { value: String, repetitions: usize },
    Bracket { value: String, repetitions: usize },
    Literal { value: String, repetitions: usize },
    Paren { value: String, repetitions: usize },
}

impl Pattern {
    #[allow(dead_code)]
    fn to_paren(&self) -> Option<Pattern> {
        match self {
            Pattern::Literal {
                value: _,
                repetitions: _,
            } => None,
            Pattern::Paren {
                value: _,
                repetitions: _,
            } => None,
            Pattern::Bracket {
                value: _,
                repetitions: _,
            } => None,
            Pattern::Composite {
                value: _,
                repetitions: _,
            } => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expression;

    #[test]
    fn literal_to_paren() {
        let result = expression::Pattern::Literal {
            value: String::from("abc"),
            repetitions: 1,
        };
        let expected = expression::Pattern::Paren {
            value: String::from("(a|b|c)"),
            repetitions: 1,
        };
        assert_eq!(result, expected);
    }
}
