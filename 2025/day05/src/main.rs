use std::ops::RangeInclusive;

fn main() {
    for path in ["input/example", "input/input"] {
        let input = std::fs::read_to_string(path).unwrap();
        let mut freshranges = parse_fresh(&input);
        // println!("{freshranges:?}");
        let availables = parse_availables(&input);
        // println!("{availables:?}");
        let solution_part1 = available_and_fresh(&availables, &freshranges);
        println!("{solution_part1}");
        let solution_part2 = num_fresh_ids(&mut freshranges);
        println!("{solution_part2}");
    }
}

/// Iterate and merge ranges. Ranges are merged when one range contains the start or end of another range.
fn num_fresh_ids(freshranges: &mut Vec<RangeInclusive<u64>>) -> u64 {
    dedup_ranges(freshranges);

    // Sum up lengths of all ranges.
    freshranges
        .iter()
        .map(|range| {
            // +1 because the end of the range is inclusive.
            range.end() + 1 - range.start()
        })
        .sum()
}

/// Iterate and merge ranges. Ranges are merged when one range contains the start or end of another range.
fn dedup_ranges(freshranges: &mut Vec<RangeInclusive<u64>>) {
    let mut i = 0;
    while i < freshranges.len() {
        let range_current = &freshranges[i];
        match freshranges.iter().take(i).position(|range_prev| {
            range_prev.contains(range_current.start())
                || range_current.contains(range_prev.start())
                || range_prev.contains(range_current.end())
                || range_current.contains(range_prev.end())
        }) {
            Some(j) => {
                // merge range[i] into range[j]
                freshranges[j] = merge_ranges(&freshranges[j], &freshranges[i]);
                freshranges.remove(i);
                // reset loop iteration, as the change might enables new merges of ranges below i
                i = j + 1
            }
            None => i += 1,
        }
    }
}

fn merge_ranges(range1: &RangeInclusive<u64>, range2: &RangeInclusive<u64>) -> RangeInclusive<u64> {
    use std::cmp::{max, min};
    let start = min(range1.start(), range2.start());
    let end = max(range1.end(), range2.end());
    *start..=*end
}

fn available_and_fresh(availables: &[u64], freshranges: &[RangeInclusive<u64>]) -> usize {
    availables
        .iter()
        .map(|available| {
            freshranges
                .iter()
                .any(|freshrange| freshrange.contains(available))
        })
        .filter(|available_and_fresh| *available_and_fresh)
        .count()
}

fn parse_fresh(input: &str) -> Vec<RangeInclusive<u64>> {
    let mut sep = input.split("\n\n");
    let fresh_ranges = sep.next().unwrap();

    fresh_ranges
        .lines()
        .map(|line| {
            let mut rangesep = line.split("-");
            let start = rangesep.next().unwrap().parse().unwrap();
            let end = rangesep.next().unwrap().parse().unwrap();
            start..=end
        })
        .collect()
}

fn parse_availables(input: &str) -> Vec<u64> {
    let mut sep = input.split("\n\n");
    sep.next();
    let fresh_ranges = sep.next().unwrap();

    fresh_ranges
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}
