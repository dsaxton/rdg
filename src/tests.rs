mod sample;

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
