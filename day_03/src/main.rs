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

fn main() {
    let input = utils::read_puzzle_input(3);
    let answer = crack_the_code(&input);
    println!("answer: {answer}");
}

#[cfg(test)]
mod tests {
    use crate::crack_the_code;

    const EXAMPLE: &str = r"987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part_1() {
        assert_eq!(crack_the_code(EXAMPLE), 357);
    }
}
