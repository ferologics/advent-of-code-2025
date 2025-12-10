use std::{
    env,
    ops::{Add, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoundedU100(u8);

impl BoundedU100 {
    pub fn new(value: u32) -> Self {
        Self((value % 100) as u8)
    }
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl Add for BoundedU100 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let sum = (self.0 as u16 + rhs.0 as u16) % 100;
        Self(sum as u8)
    }
}

impl Sub for BoundedU100 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let diff = (self.0 as u16 + 100 - rhs.0 as u16) % 100;
        Self(diff as u8)
    }
}

impl BoundedU100 {
    fn perform(self, rotation: Rotation) -> Self {
        match rotation {
            Rotation::Left(n) => self - BoundedU100::new(n),
            Rotation::Right(n) => self + BoundedU100::new(n),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rotation {
    Left(u32),
    Right(u32),
}

impl Rotation {
    fn new(value: &str) -> Option<Self> {
        let (dir, val) = value.split_at(1);
        let val = val.parse::<u32>().ok()?;
        match dir {
            "L" => Some(Self::Left(val)),
            "R" => Some(Self::Right(val)),
            _ => None,
        }
    }

    fn count_zeros(&self, start: u8) -> u32 {
        match *self {
            Rotation::Right(n) => (start as u32 + n) / 100,
            Rotation::Left(n) => {
                if start == 0 {
                    n / 100
                } else if n >= start as u32 {
                    (n - start as u32) / 100 + 1
                } else {
                    0
                }
            }
        }
    }
}

fn parse_rotations(input: &str) -> Vec<Rotation> {
    input.lines().flat_map(Rotation::new).collect()
}

fn crack_the_code(input: &str) -> u32 {
    let starting_position = BoundedU100(50);
    let rotations = parse_rotations(input);
    let mut current_position = starting_position;
    let mut zero_counter = 0u32;
    for rotation in rotations {
        current_position = current_position.perform(rotation);
        if current_position.0 == 0 {
            zero_counter += 1
        };
    }
    zero_counter
}

fn crack_the_code_v2(input: &str) -> u32 {
    let starting_position = BoundedU100(50);
    let rotations = parse_rotations(input);
    let mut current_position = starting_position;
    let mut zero_counter = 0u32;
    for rotation in rotations {
        zero_counter += rotation.count_zeros(current_position.0);
        current_position = current_position.perform(rotation);
    }
    zero_counter
}

fn read_document() -> String {
    let mut file_path = env::current_dir().ok().unwrap();
    file_path.push("day_01");
    file_path.push("src");
    file_path.push("document.txt");
    let result = std::fs::read_to_string(file_path);
    result.ok().unwrap()
}

fn main() {
    let input = read_document();
    let pwd = crack_the_code(&input);
    println!("part 1: {pwd}"); // 1081
    let pwd2 = crack_the_code_v2(&input);
    println!("part 2: {pwd2}");
}

#[cfg(test)]
mod tests {
    use crate::{crack_the_code, crack_the_code_v2};

    const EXAMPLE: &str = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1() {
        assert_eq!(crack_the_code(EXAMPLE), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(crack_the_code_v2(EXAMPLE), 6);
    }
}
