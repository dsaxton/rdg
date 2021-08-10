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
    let mut selected_word = vec![String::from("")];

    for (idx, line) in reader.lines().enumerate() {
        if thread_rng().gen::<f64>() < 1.0 / ((idx + 1) as f64) {
            selected_word.pop();
            selected_word.push(line.unwrap())
        }
    }
    selected_word.pop().unwrap()
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
    use rand::{thread_rng, Rng};

    use crate::sample;

    #[test]
    fn integer_sampling_respects_bounds() {
        let mut lower: u64;
        let mut upper: u64;
        let mut result: u64;
        for _ in 0..100 {
            lower = thread_rng().gen_range(0..100);
            upper = lower + thread_rng().gen_range(1..100);
            result = sample::integer_given_bounds(lower, upper);
            assert!(result >= lower);
            assert!(result < upper);
        }
    }

    #[test]
    fn float_sampling_respects_bounds() {
        let mut lower: f64;
        let mut upper: f64;
        let mut result: f64;
        for _ in 0..100 {
            lower = 100.0 * thread_rng().gen::<f64>();
            upper = lower + 100.0 * thread_rng().gen::<f64>();
            result = sample::float_given_bounds(lower, upper);
            assert!(result >= lower);
            assert!(result < upper);
        }
    }

    #[test]
    fn string_sampling_respects_length() {
        let mut length: usize;
        let mut result: String;
        for _ in 0..100 {
            length = thread_rng().gen_range(1..100);
            result = sample::string_from_alphanumeric(length);
            assert_eq!(result.len(), length);
        }
    }
}
