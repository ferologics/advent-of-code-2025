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
            Rotation::Left(bounded_u100) => self - bounded_u100,
            Rotation::Right(bounded_u100) => self + bounded_u100,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rotation {
    Left(BoundedU100),
    Right(BoundedU100),
}

impl Rotation {
    fn new(value: &str) -> Option<Self> {
        let (dir, val) = value.split_at(1);
        let val = val.parse::<u32>().ok()?;
        let val = BoundedU100::new(val);
        match dir {
            "L" => Some(Self::Left(val)),
            "R" => Some(Self::Right(val)),
            _ => None,
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
        // println!("current_position: {current_position:?}, rotation: {rotation:?}");
        current_position = current_position.perform(rotation);
        if current_position.0 == 0 {
            zero_counter += 1
        };
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
    println!("password: {pwd}"); // 1081
}

#[cfg(test)]
mod tests {
    use crate::crack_the_code;

    #[test]
    fn test_example() {
        let input = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let pwd = crack_the_code(input);
        println!("password: {pwd}");
        assert_eq!(pwd, 3);
    }
}
