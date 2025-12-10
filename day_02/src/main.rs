#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(value: &str) -> Self {
        let parts: Vec<&str> = value.split("-").collect();
        let start = parts[0].parse::<u64>().ok().unwrap();
        let end = parts[1].parse::<u64>().ok().unwrap();
        Self { start, end }
    }
}

fn is_even_digits(x: &u64) -> bool {
    // checked_ilog10 returns the power of 10 (e.g. 100 -> 2).
    // Add 1 to get the digit count.
    match x.checked_ilog10() {
        Some(log) => (log + 1) % 2 == 0,
        None => false, // Handles 0, which has 1 digit (odd)
    }
}

fn find_invalid_ids(range: Range) -> Vec<u64> {
    let mut ids = vec![];
    // find any numbers with some sequence of digits repeated twice (55, 6464, 123123, ...)
    let even_digit_numbers = (range.start..=range.end).filter(is_even_digits);
    for i in even_digit_numbers {
        let i_string = i.to_string();
        let (start, end) = i_string.split_at(i_string.len() / 2);
        if start == end {
            ids.push(i);
        }
    }
    ids
}

fn find_invalid_ids_v2(range: Range) -> Vec<u64> {
    let mut ids: Vec<u64> = vec![];
    // find any numbers with some sequence of digits repeated AT LEAST twice (55, 6464, 123123, 111, 123123123, ...)
    for i in range.start..=range.end {
        let i_string = i.to_string();
        // find any repeating pattern up to len() / 2
        for j in 1..=i_string.len() / 2 {
            let pattern = &i_string[..j];
            if i_string.split(pattern).all(|p| p.is_empty()) {
                ids.push(i);
                break;
            }
        }
    }
    ids
}

fn crack_the_code(input: &str) -> u64 {
    let id_ranges_split: Vec<&str> = input.split(",").collect();
    let ranges = id_ranges_split.into_iter().map(Range::new);
    let invalid_ids: Vec<u64> = ranges.flat_map(find_invalid_ids).collect();
    invalid_ids.iter().sum()
}

fn crack_the_code_v2(input: &str) -> u64 {
    let id_ranges_split: Vec<&str> = input.split(",").collect();
    let ranges = id_ranges_split.into_iter().map(Range::new);
    let invalid_ids: Vec<u64> = ranges.flat_map(find_invalid_ids_v2).collect();
    invalid_ids.iter().sum()
}

/// Generate all repeated-pattern numbers up to max_val
fn generate_all_repeated_patterns(max_val: u64) -> Vec<u64> {
    use std::collections::HashSet;

    let max_digits = max_val.checked_ilog10().unwrap_or(0) as usize + 1;
    let mut results = HashSet::new();

    // Pattern lengths from 1 up to half of max_digits
    for pattern_len in 1..=max_digits / 2 {
        // Generate all patterns of this length (no leading zeros)
        let pattern_start = if pattern_len == 1 { 1 } else { 10_u64.pow((pattern_len - 1) as u32) };
        let pattern_end = 10_u64.pow(pattern_len as u32);

        for pattern_num in pattern_start..pattern_end {
            let pattern = pattern_num.to_string();
            let mut repeated = pattern.clone();

            // Keep repeating until we exceed max_val
            loop {
                repeated.push_str(&pattern);
                if let Ok(n) = repeated.parse::<u64>() {
                    if n > max_val {
                        break;
                    }
                    results.insert(n);
                } else {
                    break; // overflow
                }
            }
        }
    }

    results.into_iter().collect()
}

fn find_invalid_ids_v3(range: Range) -> Vec<u64> {
    generate_all_repeated_patterns(range.end)
        .into_iter()
        .filter(|&n| n >= range.start && n <= range.end)
        .collect()
}

fn crack_the_code_v3(input: &str) -> u64 {
    let id_ranges_split: Vec<&str> = input.split(",").collect();
    let ranges = id_ranges_split.into_iter().map(Range::new);
    let invalid_ids: Vec<u64> = ranges.flat_map(find_invalid_ids_v3).collect();
    invalid_ids.iter().sum()
}

fn main() {
    let id_ranges_raw = "2157315-2351307,9277418835-9277548385,4316210399-4316270469,5108-10166,872858020-872881548,537939-575851,712-1001,326613-416466,53866-90153,907856-1011878,145-267,806649-874324,6161532344-6161720341,1-19,543444404-543597493,35316486-35418695,20-38,84775309-84908167,197736-309460,112892-187377,336-552,4789179-4964962,726183-793532,595834-656619,1838-3473,3529-5102,48-84,92914229-92940627,65847714-65945664,64090783-64286175,419838-474093,85-113,34939-52753,14849-30381";

    let pt1_answer = crack_the_code(id_ranges_raw);
    println!("pt 1: {pt1_answer}"); // 29818212493

    use std::time::Instant;

    let start = Instant::now();
    let pt2_v2 = crack_the_code_v2(id_ranges_raw);
    let v2_time = start.elapsed();
    println!("pt 2 (check each): {pt2_v2} in {:?}", v2_time);

    let start = Instant::now();
    let pt2_v3 = crack_the_code_v3(id_ranges_raw);
    let v3_time = start.elapsed();
    println!("pt 2 (generator):  {pt2_v3} in {:?}", v3_time);
}

#[cfg(test)]
mod tests {
    use crate::{crack_the_code, crack_the_code_v2, crack_the_code_v3};

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part_1() {
        assert_eq!(crack_the_code(EXAMPLE), 1227775554);
    }

    #[test]
    fn test_part_2_check_each() {
        assert_eq!(crack_the_code_v2(EXAMPLE), 4174379265);
    }

    #[test]
    fn test_part_2_generator() {
        assert_eq!(crack_the_code_v3(EXAMPLE), 4174379265);
    }
}
