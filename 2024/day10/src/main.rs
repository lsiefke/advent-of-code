use std::{collections::HashSet, fmt::Display};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    for path in ["example", "example2", "example3", "example4", "input"] {
        let input = std::fs::read_to_string(path).unwrap();
        let field = parse(&input);

        println!("{path}");
        println!("part1 score: {}", traverse_all_heads(&field, false));
        println!("part2 score: {}", traverse_all_heads(&field, true));
    }
}

fn trailheads(field: &[Vec<u8>]) -> Vec<Coord> {
    field
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_x, cell)| **cell == 0)
                .map(move |(x, _)| Coord { y, x })
        })
        .collect()
}

fn traverse_all_heads(field: &[Vec<u8>], part2: bool) -> usize {
    // println!("---num trailheads: {} ---", trailheads(field).len());
    trailheads(field)
        .iter()
        .map(|head| {
            let hikes = traverse(field, &vec![], &[], head);
            // println!("{head:?} - unique hikes: {}", unique_hikes(&hikes));
            match part2 {
                false => unique_hikes(&hikes),
                true => hikes.len(),
            }
        })
        .sum()
}

fn unique_hikes(hikes: &[Vec<Coord>]) -> usize {
    let destinations = hikes.iter().map(|hike| hike.last().unwrap());
    let uniques: HashSet<&Coord> = HashSet::from_iter(destinations);
    uniques.len()
}

fn traverse(
    field: &[Vec<u8>],
    hikes: &Vec<Vec<Coord>>,
    visited: &[Coord],
    pos_now: &Coord,
) -> Vec<Vec<Coord>> {
    let mut visited = visited.to_owned();
    visited.push(*pos_now);

    let height_here = at(field, pos_now).unwrap();
    if height_here == 9 {
        let mut hikes = hikes.clone();
        hikes.push(visited.to_vec());
        return hikes.to_vec();
    }

    let valid_nexts: Vec<_> = neighbours(field, pos_now)
        .into_iter()
        .filter(|neighbour| {
            let height_next = at(field, neighbour).unwrap();
            height_next == height_here + 1
        })
        .filter(|neighbour| !visited.contains(neighbour))
        .collect();

    if valid_nexts.len() == 1 {
        return traverse(field, hikes, &visited, &valid_nexts[0]);
    }

    let mut nexts = valid_nexts
        .iter()
        .flat_map(|neighbour| {
            let mut hikes = hikes.clone();
            hikes.push(visited.to_vec());
            traverse(field, &hikes, &visited, neighbour)
        })
        .filter(|hike| at(field, hike.last().unwrap()) == Some(9))
        .collect::<Vec<_>>();

    let mut hikes = hikes.clone();
    hikes.append(&mut nexts);
    hikes.to_vec()
}

fn neighbours(field: &[Vec<u8>], coord: &Coord) -> Vec<Coord> {
    let lenx = field.first().unwrap().len();
    let leny = field.len();
    let above = (coord.x as i64, coord.y as i64 - 1);
    let below = (coord.x as i64, coord.y as i64 + 1);
    let left = (coord.x as i64 - 1, coord.y as i64);
    let right = (coord.x as i64 + 1, coord.y as i64);
    [above, below, left, right]
        .iter()
        .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < lenx as i64 && *y < leny as i64)
        .map(|(x, y)| Coord {
            x: *x as usize,
            y: *y as usize,
        })
        .collect()
}

fn at(field: &[Vec<u8>], coord: &Coord) -> Option<u8> {
    field.get(coord.y)?.get(coord.x).copied()
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c.to_digit(10) {
                    Some(num) => num as u8,
                    None => u8::MAX,
                })
                .collect()
        })
        .collect()
}
