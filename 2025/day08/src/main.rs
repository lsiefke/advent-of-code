use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: u64,
    y: u64,
    z: u64,
}

impl Coord {
    fn dist_squared(&self, other: &Coord) -> u64 {
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}

fn main() {
    for path in ["input/example", "input/input"] {
        let input = std::fs::read_to_string(path).unwrap();
        let boxes = parse(&input);

        let nconnections = 10;
        let solution1 = part1(&boxes, nconnections, 3);
        println!("{path} - part1 with {nconnections} connections: {solution1}");

        let nconnections = 1000;
        let solution1 = part1(&boxes, nconnections, 3);
        println!("{path} - part1 with {nconnections} connections: {solution1}");
        println!("{path} - part2: {}", part2(&boxes));
    }
}

fn part1(boxes: &[Coord], nconnections: usize, nlargest: usize) -> usize {
    let combinations = sortedcombinations(boxes);

    let mut circuit_sets = connect_boxes(&combinations, nconnections);

    // sort sets by their size in descending order
    circuit_sets.sort_by_key(|set| std::cmp::Reverse(set.len()));
    // circuit_sets.sort_by(|a, b| b.len().cmp(&a.len()));

    circuit_sets
        .iter()
        .take(nlargest)
        .map(|circuits| circuits.len())
        .product()
}

fn part2(_boxes: &[Coord]) -> u64 {
    todo!()
}

fn sortedcombinations(boxes: &[Coord]) -> Vec<(Coord, Coord)> {
    let mut combinations = vec![];
    for i in 0..(boxes.len() - 1) {
        for j in (i + 1)..boxes.len() {
            combinations.push((boxes[i], boxes[j]));
        }
    }

    // sort combinations by distances of the pairs
    combinations.sort_by(|(left_coord1, left_coord2), (right_coord1, right_coord2)| {
        left_coord1
            .dist_squared(left_coord2)
            .cmp(&right_coord1.dist_squared(right_coord2))
    });

    combinations
}

fn connect_boxes(combinations: &[(Coord, Coord)], nconnections: usize) -> Vec<HashSet<Coord>> {
    let mut circuit_sets = vec![];
    for (coord1, coord2) in combinations.iter().take(nconnections) {
        merge_two_sets(&mut circuit_sets, coord1, coord2);
        let suitable_circuit = circuit_sets
            .iter_mut()
            .find(|circuit| circuit.contains(coord1) || circuit.contains(coord2));

        match suitable_circuit {
            Some(circuit) => {
                circuit.insert(*coord1);
                circuit.insert(*coord2);
            }
            _ => {
                circuit_sets.push(HashSet::from([*coord1, *coord2]));
            }
        };
    }

    circuit_sets
}

/// Merges two sets if coord1 and coord2 appear in two different sets.
fn merge_two_sets(circuit_sets: &mut Vec<HashSet<Coord>>, coord1: &Coord, coord2: &Coord) {
    let suitable_circuit1 = circuit_sets
        .iter()
        .position(|circuit| circuit.contains(coord1));
    let suitable_circuit2 = circuit_sets
        .iter()
        .position(|circuit| circuit.contains(coord2));

    if let (Some(circuit1), Some(circuit2)) = (suitable_circuit1, suitable_circuit2)
        && circuit1 != circuit2
    {
        circuit_sets[circuit1] = circuit_sets[circuit1]
            .union(&circuit_sets[circuit2])
            .copied()
            .collect();
        circuit_sets.remove(circuit2);
    }
}

fn parse(input: &str) -> Vec<Coord> {
    input
        .lines()
        .map(|line| {
            let mut num_iter = line.split(',');
            Coord {
                x: num_iter.next().unwrap().parse().unwrap(),
                y: num_iter.next().unwrap().parse().unwrap(),
                z: num_iter.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}
