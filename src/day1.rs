use crate::utils;
use crate::utils::Part;

pub fn solve(part: Part) -> i32 {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 1).unwrap();

    input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .map(|mass| calculate_fuel(mass, &part))
        .sum::<i32>()
}

fn calculate_fuel(mass: i32, part: &Part) -> i32 {
    if mass <= 0 {
        return 0;
    }
    let fuel = (mass / 3) - 2;
    if fuel <= 0 {
        return 0;
    }
    match part {
        Part::One => fuel,
        Part::Two => fuel + calculate_fuel(fuel, part),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(calculate_fuel(12, &utils::Part::One), 2);
        assert_eq!(calculate_fuel(14, &utils::Part::One), 2);
        assert_eq!(calculate_fuel(1969, &utils::Part::One), 654);
        assert_eq!(calculate_fuel(100756, &utils::Part::One), 33583);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(calculate_fuel(14, &utils::Part::Two), 2);
        assert_eq!(calculate_fuel(1969, &utils::Part::Two), 966);
        assert_eq!(calculate_fuel(100756, &utils::Part::Two), 50346);
    }
}
