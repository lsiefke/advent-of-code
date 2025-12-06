/// Could be prettier but I am short on time today.
///
#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

impl Op {
    fn from_str(s: &str) -> Op {
        match s {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => panic!(),
        }
    }
}

fn main() {
    for path in ["input/example", "input/input"] {
        let input = std::fs::read_to_string(path).unwrap();
        let (numbers_part1, ops) = parse_part1(&input);
        let solution1 = solve_part1(&numbers_part1, &ops);
        println!("{path} - part1: {solution1}");
        let numbers_part2 = parse_part2(&input);
        let solution2 = solve_part2(&numbers_part2, &ops);
        println!("{path} - part2: {solution2}");
    }
}

fn solve_part1(numbers_rows: &[Vec<u64>], ops: &[Op]) -> u64 {
    let ncols = ops.len();

    (0..ncols)
        .map(|col| {
            let init = match ops[col] {
                Op::Add => 0,
                Op::Mul => 1,
            };
            numbers_rows.iter().fold(init, |acc, row| match ops[col] {
                Op::Add => acc + row[col],
                Op::Mul => acc * row[col],
            })
        })
        .sum()
}

fn solve_part2(numbers_all: &[Vec<u64>], ops: &[Op]) -> u64 {
    numbers_all
        .iter()
        .zip(ops)
        .map(|(numbers, op)| {
            let init = match op {
                Op::Add => 0,
                Op::Mul => 1,
            };
            numbers.iter().fold(init, |acc, row| match op {
                Op::Add => acc + row,
                Op::Mul => acc * row,
            })
        })
        .sum()
}

fn parse_part2(input: &str) -> Vec<Vec<u64>> {
    let input = input.trim();
    let ncols = input.lines().next().unwrap().len();

    let mut numbers = vec![];
    let mut numbers_same_op = vec![];
    for col in 0..ncols {
        if input
            .lines()
            .all(|line| line.chars().nth(col).unwrap().is_whitespace())
        {
            // new operation
            numbers.push(numbers_same_op.clone());
            numbers_same_op.clear();
            continue;
        }

        let mut digit = 0;
        let mut num = 0;
        for rowstr in input.lines().rev().skip(1) {
            match rowstr.chars().nth(col).unwrap().to_digit(10) {
                Some(n) => {
                    num += n as u64 * 10_u64.pow(digit as u32);
                    digit += 1;
                }
                None => continue,
            }
        }

        numbers_same_op.push(num);
    }
    numbers.push(numbers_same_op.clone());
    numbers
}

fn parse_part1(input: &str) -> (Vec<Vec<u64>>, Vec<Op>) {
    let input = input.trim();
    let idx_line_ops = input
        .lines()
        .take_while(|line| line_holds_numbers(line))
        .count();

    let numbers = input.lines().take(idx_line_ops).map(|line| {
        line.split_whitespace()
            .map(|numstr| numstr.parse().unwrap())
            .collect()
    });

    let ops = input
        .lines()
        .nth(idx_line_ops)
        .unwrap()
        .split_whitespace()
        .map(Op::from_str);

    (numbers.collect(), ops.collect())
}

fn line_holds_numbers(line: &str) -> bool {
    line.split_whitespace()
        .next()
        .unwrap()
        .parse::<u64>()
        .is_ok()
}
