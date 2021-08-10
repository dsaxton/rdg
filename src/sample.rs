use std::fs::File;
use std::io::{BufRead, BufReader};

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn integer_given_bounds(lower: u64, upper: u64) -> u64 {
    lower + (thread_rng().gen::<f64>() * ((upper - lower) as f64)).floor() as u64
}

pub fn float_given_bounds(lower: f64, upper: f64) -> f64 {
    lower + thread_rng().gen::<f64>() * (upper - lower)
}

pub fn from_wordlist(wordlist: &str) -> String {
    let file = File::open(wordlist).expect("file does not exist");
    let reader = BufReader::new(file);
    let mut selected_word = vec![String::from("")]; // FIXME: this feels like a hack

    for (idx, line) in reader.lines().enumerate() {
        if thread_rng().gen::<f64>() < 1.0 / ((idx + 1) as f64) {
            selected_word.pop();
            selected_word.push(line.unwrap())
        }
    }
    selected_word[0].clone()
}

pub fn string_from_alphanumeric(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::sample;

    #[test]
    fn integer_sampling_respects_bounds() {
        let lower: u64 = 10;
        let upper: u64 = 20;
        let mut result: u64;
        for _ in 0..100 {
            result = sample::integer_given_bounds(lower, upper);
            assert!(result >= lower);
            assert!(result < upper);
        }
    }

    #[test]
    fn float_sampling_respects_bounds() {
        let lower: f64 = 10.0;
        let upper: f64 = 20.0;
        let mut result: f64;
        for _ in 0..100 {
            result = sample::float_given_bounds(lower, upper);
            assert!(result >= lower);
            assert!(result < upper);
        }
    }

    #[test]
    fn string_sampling_respects_length() {
        let length: usize = 20;
        let mut result: String;
        for _ in 0..100 {
            result = sample::string_from_alphanumeric(length);
            assert_eq!(result.len(), length);
        }
    }
}
