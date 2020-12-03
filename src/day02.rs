use lazy_static::lazy_static;
use regex::Regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

lazy_static! {
    static ref PASSWORD_POLICY_REGEX: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w*)").unwrap();
}

/// Password Policy
pub struct PasswordPolicy {
    char_bounds: RangeInclusive<usize>,
    char_value: char,
    password: String,
}

fn parse_line_to_policy(line: &str) -> Option<PasswordPolicy> {
    // ? is an easier way to handle errors, see https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/the-question-mark-operator-for-easier-error-handling.html
    let results = PASSWORD_POLICY_REGEX.captures(line)?;

    // the result is like below:
    // Captures({0: Some("2-4 q: qxql"), 1: Some("2"), 2: Some("4"), 3: Some("q"), 4: Some("qxql")})
    // And this is the reasons why the index starts from 1 instead of 0

    let lower_bound = results.get(1)?.as_str().parse().ok()?;
    let upper_bound = results.get(2)?.as_str().parse().ok()?;

    Some(PasswordPolicy {
        char_bounds: (lower_bound..=upper_bound),
        char_value: results.get(3)?.as_str().chars().next()?,
        password: results.get(4)?.as_str().to_string(),
    })
}

fn check_1(password_policy: PasswordPolicy) -> bool {
    let char_occurrence = &(password_policy.password)
        .chars()
        .filter(|c| c == &password_policy.char_value)
        .count();

    return password_policy.char_bounds.contains(char_occurrence);
}

fn check_2(password_policy: PasswordPolicy) -> bool {

    let (lower_position, upper_position) = password_policy.char_bounds.clone().into_inner();
    let lower_value = password_policy.password.chars().nth(lower_position - 1).unwrap();
    let upper_value = password_policy.password.chars().nth(upper_position - 1).unwrap();

    return (password_policy.char_value == lower_value) ^ (password_policy.char_value == upper_value);
}

pub fn solution() -> i32 {
    let filename = "input/day02";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut counter = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.

        let password_policy = parse_line_to_policy(&line).unwrap();

        if check_2(password_policy) {
            counter += 1;
        }
    }

    return counter;
}