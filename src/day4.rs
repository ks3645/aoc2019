use crate::utils;
use crate::utils::Part;

pub fn solve(part: Part) -> i32 {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 4).unwrap();

    let range: Vec<i32> = input
        .split("-")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    count_valid_passwords(range[0], range[1], part)
}

fn count_valid_passwords(min: i32, max: i32, part: Part) -> i32 {
    let mut count = 0;

    for password in min..max {
        if is_valid_password(password.to_string(), part) {
            count += 1;
        }
    }

    count
}

fn is_valid_password(password: String, part: Part) -> bool {
    let chars: Vec<char> = password.chars().collect();

    if chars.len() != 6 {
        return false;
    }

    let mut prev = None;
    let mut found_double = false;
    let mut is_increasing = true;
    let mut repeat_count = 0;
    for c in chars {
        let digit = c.to_digit(10).unwrap();

        if let Some(prev_digit) = prev {
            if digit == prev_digit {
                repeat_count += 1;
            } else if repeat_count > 0 {
                match part {
                    Part::One => {
                        found_double = true;
                    }
                    Part::Two => {
                        if repeat_count == 1 {
                            found_double = true;
                        }
                    }
                }
                repeat_count = 0;
            }
            if digit < prev_digit {
                is_increasing = false;
            }
        }

        prev = Some(digit);
    }

    if repeat_count > 0 {
        match part {
            Part::One => {
                found_double = true;
            }
            Part::Two => {
                if repeat_count == 1 {
                    found_double = true;
                }
            }
        }
    }

    found_double && is_increasing
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_password_part_one() {
        assert!(is_valid_password(111111.to_string(), Part::One));
        assert_eq!(is_valid_password(223450.to_string(), Part::One), false);
        assert_eq!(is_valid_password(123789.to_string(), Part::One), false);
        assert_eq!(is_valid_password(129338.to_string(), Part::One), false);
        assert_eq!(is_valid_password(11111.to_string(), Part::One), false);
    }

    #[test]
    fn test_valid_password_part_two() {
        assert!(is_valid_password(112233.to_string(), Part::Two));
        assert_eq!(is_valid_password(123444.to_string(), Part::Two), false);
        assert!(is_valid_password(111122.to_string(), Part::Two), false);
        assert!(is_valid_password(337777.to_string(), Part::Two), false);
    }
}
