use crate::file_util::read_file_to_string_by_new_line;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

fn is_doc_valid(valid_fields_len: usize, valid_fields_present: usize, has_cid: usize) -> bool {
    return valid_fields_present == valid_fields_len ||
        (valid_fields_present == valid_fields_len - 1 && has_cid == 0);
}

fn is_field_valid(field_name: &str, field_value: &str) -> bool {
    //part-1 - no validation - 206
    //return true;

    //part-2 - 123
    //byr (Birth Year) - four digits; at least 1920 and at most 2002.
    if field_name == "byr" {
        if !is_number_valid(&field_value, 1920, 2002) {
            return false;
        }
    }
    //iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    if field_name == "iyr" {
        if !is_number_valid(&field_value, 2010, 2020) {
            return false;
        }
    }
    //eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    if field_name == "eyr" {
        if !is_number_valid(&field_value, 2020, 2030) {
            return false;
        }
    }

    /* hgt (Height) - a number followed by either cm or in:
    If cm, the number must be at least 150 and at most 193.
    If in, the number must be at least 59 and at most 76.
    **/
    if field_name == "hgt" {
        if Regex::new("^(\\d+)(cm|in)$").unwrap().is_match(field_value) {
            let number = &field_value[..field_value.len()-2];
            let unit = &field_value[field_value.len()-2..field_value.len()];
            if (unit == "cm" && !is_number_valid(number, 150, 193))
                || (unit == "in" && !is_number_valid(number, 59, 76)) {
                return false
            }
        } else {
            return false;
        }
    }
    //hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    if field_name == "hcl" {
        if !Regex::new("^#[0-9a-f]{6}$").unwrap().is_match(field_value) {
            return false;
        }
    }
    //ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    if field_name == "ecl" {
        if !Regex::new("^amb|blu|brn|gry|grn|hzl|oth$").unwrap().is_match(field_value) {
            return false;
        }
    }
    //pid (Passport ID) - a nine-digit number, including leading zeroes.
    if field_name == "pid" {
        if !Regex::new("^\\d{9}$").unwrap().is_match(field_value) {
            return false;
        }
    }
    return true;
}

fn is_number_valid(number_str: &str, min: usize, max: usize) -> bool {
    let number = String::from(number_str).parse::<usize>();
    match number {
        Ok(val) => return val >= min && val <= max,
        Err(_e) => return false,
    }
}

fn validate_line(pure_line: &String) -> bool {
    let valid_fields: HashSet<&str> = vec![ "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid" ].into_iter().collect();
    let mut doc_fields = HashMap::new();
    let mut has_cid = 0;

    let field_iter = pure_line.split_whitespace();

    for field in field_iter {
        let mut iter = field.split(":");

        let field_name = iter.next().unwrap();
        let field_value = iter.next().unwrap();

        if valid_fields.contains(field_name) && !doc_fields.contains_key(field_name) {
            if is_field_valid(field_name, field_value) {
                doc_fields.insert(field_name, field_value);
            }
        }

        if field_name == "cid" {
            has_cid += 1;
        }
    }

    return is_doc_valid(valid_fields.len(), doc_fields.len(), has_cid);
}

fn valid_doc(doc_input: &Vec<String>) -> usize {
    return doc_input
        .iter()
        .filter(|line| validate_line(line))
        .count();
}

pub fn solution() -> usize {
    let doc_input = read_file_to_string_by_new_line("input/day04" , " ");
    return valid_doc(&doc_input);
}