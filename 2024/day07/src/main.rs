#[derive(Debug)]
struct Line {
    test_value: u64,
    numbers: Vec<u64>,
}

impl Line {
    fn from_str(s: &str) -> Line {
        let (test_value, numbers) = s.split_at(s.find(":").unwrap());
        let numbers = &numbers[1..]; // remove ':' at beginning
        let numbers = numbers.split_whitespace().map(|num| num.parse().unwrap());

        Line {
            test_value: test_value.parse().unwrap(),
            numbers: numbers.collect(),
        }
    }
}

fn main() {
    for filename in ["example", "input"] {
        let input = std::fs::read_to_string(filename).unwrap();
        let lines = parse_input(&input);

        let with_concat = false;
        let part1 = calibrated_sum(&lines, with_concat);
        let with_concat = true;
        let part2 = calibrated_sum(&lines, with_concat);

        println!("calibration result for {filename}. part1: {part1}; part2: {part2}");
    }
}

fn parse_input(input: &str) -> Vec<Line> {
    input.lines().map(Line::from_str).collect()
}

fn calibrated_sum(lines: &[Line], with_concat: bool) -> u64 {
    lines
        .iter()
        .map(|line| (line.test_value, solvable(line, with_concat)))
        .filter(|(_, solved)| *solved)
        .map(|(test_value, _)| test_value)
        .sum()
}

/// Create all possible combinations of operations, calculate, and compare to test value.
fn solvable(line: &Line, with_concat: bool) -> bool {
    let num_ops: u64 = if !with_concat { 2 } else { 3 };
    // There are a number of (2 or 3)^n possible calculations ber line with
    // 2 or 3: either + or * (or ||),
    // n: length of line.numbers - 1 (-1 because the operation uses two successive numbers).
    let ncalculations = num_ops.pow(line.numbers.len() as u32 - 1);

    // number of bits needed to represent the greatest number: log_numops(n+1)
    let bin_width = (ncalculations + 1).ilog(num_ops) as usize;

    // create a list of numbers with 0: add; 1: mul, 2: concat
    let list_ops = (0..ncalculations).map(|i| convert_base(i, num_ops, bin_width));

    let mut results = list_ops.map(|ops| {
        ops.iter()
            .enumerate()
            .fold(line.numbers[0], |acc, (i, op)| match op {
                0 => acc + line.numbers[i + 1],
                1 => acc * line.numbers[i + 1],
                2 => concat_decimals(acc, line.numbers[i + 1]),
                _ => panic!(),
            })
    });

    results.any(|res| res == line.test_value)
}

fn convert_base(num: u64, base: u64, width: usize) -> Vec<u8> {
    let mut converted = vec![0; width];
    let mut num = num;
    let mut quotient = 1;
    let mut remainder;
    let mut digit = 0;
    while quotient > 0 {
        quotient = num / base;
        remainder = num % base;
        converted[width - 1 - digit] = remainder as u8;
        digit += 1;
        num = quotient;
    }
    converted
}

fn concat_decimals(num1: u64, num2: u64) -> u64 {
    let exp = num2.ilog10() + 1;
    num1 * 10_u64.pow(exp) + num2
}
