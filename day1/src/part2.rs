use std::collections::HashMap;
use std::ops::AddAssign;

fn find_digits(line: &str) -> (u32, u32) {
    let numbers_in_text = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut first_digit = 0;
    let mut last_digit = 0;
    let mut substring = String::new();
    let mut found_digit: u32 = 0;

    for char in line.chars() {
        if char.is_ascii_digit() {
            found_digit = char.to_digit(10).unwrap();
            substring = String::new();
        } else {
            substring.add_assign(String::from(char).as_str());
            for (number, digit) in &numbers_in_text {
                if substring.contains(number) {
                    found_digit = *digit;
                    substring = String::from(&substring[(substring.len() - 1)..]);
                    break;
                }
            }
        }

        last_digit = found_digit;

        if first_digit == 0 { first_digit = found_digit; }
    }
    (first_digit, last_digit)
}

pub fn main() {
    let calibration_values = include_str!("../assets/input.txt");
    let mut calibration_sum = 0;

    for line in calibration_values.lines() {
        let digits = find_digits(line);
        let number = (digits.0.to_string() + &*digits.1.to_string()).parse::<u32>().unwrap();
        calibration_sum += number;
    }

    println!("{}", &calibration_sum);
}
