use std::env;

pub fn read_puzzle_input(day: u8) -> String {
    let mut path = env::current_dir().unwrap();
    path.push(format!("day_{:02}", day));
    path.push("src");
    path.push("puzzle_input.txt");
    std::fs::read_to_string(path).unwrap()
}
