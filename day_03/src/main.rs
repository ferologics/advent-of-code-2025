// find the largest possible joltage each bank can produce
// In 987654321111111, you can make the largest joltage possible, 98, by turning on the first two batteries.
fn find_max_joltage(bank: &str) -> u8 {
    let (mut a, mut b) = (0, 0);
    let chars: Vec<char> = bank.chars().collect();
    fn val(a: u8, b: u8) -> u8 {
        a * 10 + b
    }
    for (idx, i) in chars.iter().enumerate() {
        let n = i.to_string().parse::<u8>().ok().unwrap();
        if n > a && idx != (chars.len() - 1) {
            a = n;
            b = 0;
        } else if n > b {
            b = n;
        }
    }
    val(a, b)
}

fn crack_the_code(input: &str) -> u32 {
    let banks = input.lines();
    banks.map(find_max_joltage).map(|n| n as u32).sum()
}

fn find_max_joltage_v2(bank: &str, k: usize) -> u64 {
    let chars: Vec<char> = bank.chars().collect();
    let n = chars.len();
    let mut result = String::with_capacity(k);
    let mut start = 0;

    for remaining in (1..=k).rev() {
        // Pick from start..=(n - remaining)
        let end = n - remaining;
        let slice = &chars[start..=end];
        let max_char = *slice.iter().max().unwrap();
        // Find FIRST occurrence of max (leftmost gives most options for remaining picks)
        let max_idx = slice.iter().position(|&c| c == max_char).unwrap();
        result.push(chars[start + max_idx]);
        start = start + max_idx + 1;
    }

    result.parse().unwrap()
}

fn crack_the_code_v2(input: &str) -> u64 {
    input.lines().map(|bank| find_max_joltage_v2(bank, 12)).sum()
}

fn main() {
    let input = utils::read_puzzle_input(3);
    let pt1 = crack_the_code(&input);
    println!("part 1: {pt1}");
    let pt2 = crack_the_code_v2(&input);
    println!("part 2: {pt2}");
}

#[cfg(test)]
mod tests {
    use crate::{crack_the_code, crack_the_code_v2};

    const EXAMPLE: &str = r"987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part_1() {
        assert_eq!(crack_the_code(EXAMPLE), 357);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(crack_the_code_v2(EXAMPLE), 3121910778619);
    }
}
