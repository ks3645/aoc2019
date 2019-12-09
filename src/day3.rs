use crate::utils;
use crate::utils::Part;

use std::collections::HashMap;

pub fn solve(part: Part) -> i32 {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 3).unwrap();

    let mut lines = input.lines();

    let wire_one: Vec<&str> = lines.next().unwrap().split(",").collect();
    let wire_two: Vec<&str> = lines.next().unwrap().split(",").collect();

    find_min_intersection(&wire_one, &wire_two, part)
}

fn find_min_intersection(wire_one: &Vec<&str>, wire_two: &Vec<&str>, part: Part) -> i32 {
    let mut wire_one_ports = HashMap::new();
    let mut intersections = HashMap::new();

    let mut pos = Port::center();
    let mut steps_taken: i32 = 0;
    for step in wire_one {
        let dir: Direction = step[0..1].into();
        let step_count = step[1..].parse::<u32>().unwrap();

        for _ in 0..step_count {
            pos = pos.neighbor(&dir);
            steps_taken += 1;

            wire_one_ports.insert(pos.clone(), steps_taken);
        }
    }

    pos = Port::center();
    steps_taken = 0;
    for step in wire_two {
        let dir: Direction = step[0..1].into();
        let step_count = step[1..].parse::<u32>().unwrap();

        for _ in 0..step_count {
            pos = pos.neighbor(&dir);
            steps_taken += 1;

            if wire_one_ports.contains_key(&pos) {
                let wire_one_steps = wire_one_ports.get(&pos).unwrap();
                intersections.insert(pos.clone(), wire_one_steps + steps_taken);
            }
        }
    }

    match part {
        Part::One => intersections
            .keys()
            .map(|port| port.x.abs() + port.y.abs())
            .min()
            .unwrap(),
        Part::Two => intersections.values().min().unwrap().clone(),
    }
}

// uses Cartesian coords for now
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Port {
    x: i32,
    y: i32,
}

impl Port {
    fn center() -> Self {
        Port { x: 0, y: 0 }
    }

    fn neighbor(&self, dir: &Direction) -> Self {
        match dir {
            Direction::Up => Port {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Port {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Down => Port {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Right => Port {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl std::convert::From<&str> for Direction {
    fn from(c: &str) -> Self {
        match c {
            "U" => Direction::Up,
            "L" => Direction::Left,
            "D" => Direction::Down,
            "R" => Direction::Right,
            _ => panic!("not a valid direction"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_str = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";

        let mut lines = test_str.lines();

        let wire_one: Vec<&str> = lines.next().unwrap().split(",").collect();
        let wire_two: Vec<&str> = lines.next().unwrap().split(",").collect();

        assert_eq!(find_min_intersection(&wire_one, &wire_two, Part::One), 159);
    }

    #[test]
    fn test_part_two() {
        let test_str = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";

        let mut lines = test_str.lines();

        let wire_one: Vec<&str> = lines.next().unwrap().split(",").collect();
        let wire_two: Vec<&str> = lines.next().unwrap().split(",").collect();

        assert_eq!(find_min_intersection(&wire_one, &wire_two, Part::Two), 610);
    }
}
