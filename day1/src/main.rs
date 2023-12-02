fn find_digits(line: &str) -> (u32, u32) {
    let mut first_digit = 0;
    let mut last_digit = 0;
    for char in line.chars() {
        if !char.is_ascii_digit() { continue; };
        let digit = char.to_digit(10).unwrap();

        last_digit = digit;

        if first_digit == 0 { first_digit = digit; }
    }
    (first_digit, last_digit)
}

fn main() {
    let calibration_values = include_str!("../assets/input.txt");
    let mut calibration_sum = 0;

    for line in calibration_values.lines() {
        let digits = find_digits(line);
        let number = (digits.0.to_string() + &*digits.1.to_string()).parse::<u32>().unwrap();
        calibration_sum += number;
    }

    println!("{}", &calibration_sum);
}
