use std::ops::RangeInclusive;

fn main() {
    for path in ["input/example", "input/input"] {
        let input = std::fs::read_to_string(path).unwrap();
        let ranges = parse(&input);
        let invalids_part1 = ranges.iter().flat_map(invalid_ids_part1);
        let sum = invalids_part1.sum::<u64>();
        println!("Part1: {sum:?}");
        let invalids_part2 = ranges.iter().flat_map(invalid_ids_part2);
        let sum = invalids_part2.sum::<u64>();
        println!("Part2: {sum:?}");
    }
}

fn invalid_ids_part1(range: &RangeInclusive<u64>) -> Vec<u64> {
    range
        .clone()
        .filter(|num| repeats_twice(*num))
        .filter(|num| range.contains(num))
        .collect()
}

fn invalid_ids_part2(range: &RangeInclusive<u64>) -> Vec<u64> {
    range
        .clone()
        .filter(|num| repeats_atleast_twice(*num))
        .filter(|num| range.contains(num))
        .collect()
}

fn repeats_twice(num: u64) -> bool {
    let num_digits = num.ilog10() + 1;

    match num_digits.is_multiple_of(2) {
        true => {
            let higher_half = num / 10_u64.pow(num_digits / 2);
            let lower_half = num % 10_u64.pow(num_digits / 2);
            higher_half == lower_half
        }
        false => false,
    }
}

fn repeats_atleast_twice(num: u64) -> bool {
    let num_digits = num.ilog10() + 1;
    // Possible are repetitions of the numbers in the higher 'half' of the number.
    let possible_reps = (0..(num_digits / 2)).map(|i| (i, num / 10_u64.pow(num_digits - i - 1)));
    // concatenations are the repetitions of the numbers
    let mut concatenations = possible_reps.map(|(i, num)| repeat_num(num, num_digits / (i + 1)));
    concatenations.any(|concat| concat == num)
}

// concats num n times
fn repeat_num(num: u64, n: u32) -> u64 {
    let subnum_digits = num.ilog10() + 1;
    let mut ret = 0;
    for i in 0..n {
        ret += num * 10_u64.pow(i * subnum_digits);
    }
    ret
}

fn parse(input: &str) -> Vec<RangeInclusive<u64>> {
    let input = input.replace("\n", "");
    input
        .split(',')
        .map(|range| {
            let mut start_end = range.split("-");
            let start = start_end.next().unwrap().parse().unwrap();
            let end = start_end.next().unwrap().parse().unwrap();
            start..=end
        })
        .collect()
}
