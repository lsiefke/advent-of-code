use std::collections::VecDeque;

use cached::proc_macro::cached;

fn main() {
    for path in ["example", "input"] {
        let input = std::fs::read_to_string(path).unwrap();
        let mut stones = parse(&input);
        println!("Initial stones with {path}: {:?}", stones);

        for i in 1..6 {
            stones = blink(&stones);
            println!("After {i} blinks: {:?}", stones);
        }
        let mut stones = parse_deque(&input);
        for i in 1..6 {
            blink_mut(&mut stones);
            println!("After {i} blinks: {:?}", stones);
        }

        let stones = parse(&input);
        let start = std::time::Instant::now();
        let i = 75;
        let nstones = blink_stone_counter(&stones, i);
        let end = std::time::Instant::now();
        println!(
            "There are {} stones after blinking {i} times (computed within {:?})",
            nstones,
            end - start
        );
    }
}

fn blink(stones: &[u64]) -> Vec<u64> {
    stones.iter().copied().flat_map(blink_stone).collect()
}

fn blink_stone_counter(stones: &[u64], depth: usize) -> usize {
    stones
        .iter()
        .map(|stone| nstones_depth(*stone, depth))
        .sum()
}

#[cached]
fn nstones_depth(stone: u64, depth: usize) -> usize {
    let mut nstones = 0;
    if depth == 1 {
        let substones = blink_stone(stone);
        return substones.len();
    }

    for substone in blink_stone(stone) {
        nstones += nstones_depth(substone, depth - 1);
    }

    nstones
}

fn blink_stone(stone: u64) -> Vec<u64> {
    match stone {
        0 => vec![1],
        _ => {
            if numdigits_even(stone) {
                vec![stone * 2024]
            } else {
                // cut number in halfes
                vec![
                    stone / 10_u64.pow(numdigits(stone) / 2 + 1),
                    stone % 10_u64.pow(numdigits(stone) / 2 + 1),
                ]
            }
        }
    }
}

fn blink_mut(stones: &mut VecDeque<u64>) {
    let mut nconcats = 0;
    for i in 0..stones.len() {
        let stone = stones[i + nconcats];
        match stone {
            0 => stones[i + nconcats] = 1,
            _ => {
                if numdigits_even(stone) {
                    stones[i + nconcats] *= 2024;
                } else {
                    // cut number in halfes
                    stones[i + nconcats] = stone / 10_u64.pow(numdigits(stone) / 2 + 1);
                    stones.insert(
                        i + 1 + nconcats,
                        stone % 10_u64.pow(numdigits(stone) / 2 + 1),
                    );
                    nconcats += 1;
                }
            }
        }
    }
}

fn parse(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}
fn parse_deque(input: &str) -> VecDeque<u64> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

#[inline]
fn numdigits_even(x: u64) -> bool {
    x.ilog10().is_multiple_of(2)
}

#[inline]
fn numdigits(x: u64) -> u32 {
    x.ilog10()
}
