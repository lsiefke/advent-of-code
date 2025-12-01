use std::cmp::{max, min};

// https://adventofcode.com/2024/day/1
fn main() {
    for input in ["example", "input"] {
        let (list1, list2) = parse(input);

        println!(
            "total distance: {}, similarity: {}",
            total_distance(&list1, &list2),
            similarity(&list1, &list2)
        );
    }
}

fn parse(filename: &str) -> (Vec<usize>, Vec<usize>) {
    let input = std::fs::read_to_string(filename).unwrap();

    input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(left, right)| (left.replace(" ", ""), right.replace(" ", "")))
        .map(|(left, right)| {
            (
                left.parse::<usize>().unwrap(),
                right.parse::<usize>().unwrap(),
            )
        })
        .unzip()
}

fn total_distance(list1: &[usize], list2: &[usize]) -> usize {
    let mut sorted1 = Vec::from(list1);
    sorted1.sort();
    let mut sorted2 = Vec::from(list2);
    sorted2.sort();

    let dists = sorted1
        .iter()
        .zip(sorted2)
        .map(|(id1, id2)| max(id1, &id2) - min(id1, &id2));

    dists.sum()
}

fn similarity(list1: &[usize], list2: &[usize]) -> usize {
    list1
        .iter()
        .map(|id1| id1 * list2.iter().filter(|id2| id1 == *id2).count())
        .sum()
}
