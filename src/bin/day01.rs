use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("inputs/01.txt").expect("Should have been able to read the file");
    let trimmed = contents.trim();
    println!("part 1: {}", sum_of_calibrations(trimmed));
    println!("part 2: {}", sum_of_calibrations_spelled_out(trimmed));
}

fn sum_of_calibrations(content: &str) -> u64 {
    content.split('\n').map(calibration_value).sum()
}

fn calibration_value(line: &str) -> u64 {
    let digits = line.chars().filter(|c| c.is_ascii_digit()).collect::<Vec<_>>();
    let first = digits.first().expect("must have a first digit");
    let last = digits.last().expect("must have a last digit");
    format!("{first}{last}")
        .parse()
        .expect("should be a number")
}

fn sum_of_calibrations_spelled_out(content: &str) -> u64 {
    content.split('\n').map(calibration_value_spelled_out).sum()
}

fn calibration_value_spelled_out(line: &str) -> u64 {
    let values = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut first_digit = 0;
    let mut first_digit_idx = usize::MAX;
    let mut last_digit = 0;
    let mut last_digit_idx = 0;

    for (i, value) in values.iter().enumerate() {
        let num_val = format!("{i}");
        let search_strings = [num_val.as_str(), value];
        for val in search_strings {
            if let Some(idx) = line.find(val) {
                if idx <= first_digit_idx {
                    first_digit = i;
                    first_digit_idx = idx;
                }
            }
            if let Some(idx) = line.rfind(val) {
                if idx >= last_digit_idx {
                    last_digit = i;
                    last_digit_idx = idx;
                }
            }
        }
    }
    format!("{first_digit}{last_digit}")
        .parse()
        .expect("should be a number")
}
