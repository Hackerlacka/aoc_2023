use std::fs::read_to_string;

fn read_calibration_val_no_chars(line: String) -> i32 {
    let first_digit = line.chars().find(|&x| x.is_digit(10)).unwrap();
    let last_digit = line.chars().rev().find(|&x| x.is_digit(10)).unwrap();

    return format!("{}{}", first_digit, last_digit).parse::<i32>().unwrap();
}

fn read_calibration_val(line: &str, interpret_chars: bool) -> i32 {
    let char_digits = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut modified_line;

    if interpret_chars {
        modified_line = String::new();
        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                modified_line.push(c);
            } else {
                for (j, char_digit) in char_digits.iter().enumerate() {
                    if line.len() - i < char_digit.len() {
                        continue;
                    }

                    let part_line: &str = &line[i..i + char_digit.len()];

                    if char_digit.eq(&part_line) {
                        modified_line.push_str(&(j + 1).to_string());
                        break;
                    }
                }
            }
        }
    } else {
        modified_line = line.to_owned();
    }

    return read_calibration_val_no_chars(modified_line);
}

pub fn read_calibration_vals(file: &str, interpret_chars: bool) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    for line in read_to_string(file).unwrap().lines() {
        result.push(read_calibration_val(line, interpret_chars));
    }

    return result;
}