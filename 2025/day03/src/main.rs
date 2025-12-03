fn main() {
    for path in ["input/example", "input/input"] {
        let input = std::fs::read_to_string(path).unwrap();
        let banks = parse(&input);
        let total_joltage_part1 = banks.iter().map(|bank| maxjoltage_part1(bank)).sum::<u64>();
        let total_joltage_part1_n2 = banks
            .iter()
            .map(|bank| maxjoltage_part2(bank, 2))
            .sum::<u64>();
        let total_joltage_part2 = banks
            .iter()
            .map(|bank| maxjoltage_part2(bank, 12))
            .sum::<u64>();
        println!("{path}");
        println!("Part1 total output joltaged: {total_joltage_part1}");
        println!("Part1 total output joltaged calculated with n=2: {total_joltage_part1_n2}");
        println!("Part2 total output joltaged: {total_joltage_part2}");
    }
}

/// n: number of batteries used (i.e., number of digits within bank).
fn maxjoltage_part2(bank: &[u8], n: u64) -> u64 {
    let mut skip = 0;
    let mut maxjoltage = 0;
    for i in 0..n {
        let digit = n - i - 1;
        let take = bank.len() - digit as usize - skip;
        let (maxpos_new, maxval) = findmax(bank, skip, take);
        skip = maxpos_new + 1;
        maxjoltage += 10_u64.pow(digit as u32) * maxval;
    }
    maxjoltage
}

fn findmax(bank: &[u8], skip: usize, take: usize) -> (usize, u64) {
    let (max_pos, max_val) = bank
        .iter()
        .enumerate()
        .skip(skip)
        .take(take) // do not use the last elements, as it could be needed later
        .rev() // max() and max_by_key() return the last max element
        .max_by_key(|(_i, val)| **val)
        .unwrap();
    (max_pos, *max_val as u64)
}

fn maxjoltage_part1(bank: &[u8]) -> u64 {
    let (max_pos, max_val) = bank
        .iter()
        .take(bank.len() - 1) // do not use the last element, as it could be needed later
        .rev() // max() and max_by_key() return the last max element
        .enumerate()
        .max_by_key(|(_i, val)| **val)
        .unwrap();
    // handle the reverse order search
    let max_pos = bank.len() - 1 - max_pos;

    let (_second_max_pos, second_max_val) = bank
        .iter()
        .skip(max_pos)
        .take(bank.len())
        .rev()
        .enumerate()
        .max_by_key(|(_i, val)| **val)
        .unwrap();

    let joltage = max_val * 10 + second_max_val;
    joltage as u64
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .take_while(|line| *line != "\n")
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}
