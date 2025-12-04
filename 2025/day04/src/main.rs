use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Coord {
    x: i64,
    y: i64,
}

fn main() {
    for path in ["input/example", "input/input"] {
        let input = std::fs::read_to_string(path).unwrap();
        let rolls = parse_rolls(&input);
        let solution_part1 = accessible_rolls_part1(&rolls);
        let solution_part2 = accessible_rolls_part2(&rolls, 0);
        println!("part1: {solution_part1}. part2: {solution_part2}");
    }
}

fn accessible_rolls_part1(rolls: &HashSet<Coord>) -> usize {
    rolls.iter().fold(0, |naccessibles, coord| {
        match num_neighbours(rolls, coord) {
            ..4 => naccessibles + 1,
            _ => naccessibles,
        }
    })
}

fn accessible_rolls_part2(rolls: &HashSet<Coord>, previously_removed: usize) -> usize {
    let toremove: HashSet<_> = rolls
        .iter()
        .flat_map(|coord| match num_neighbours(rolls, coord) {
            ..4 => Some(*coord),
            _ => None,
        })
        .collect();

    // Call recursively
    match toremove.len() {
        0 => previously_removed,
        _ => {
            let trimmed = rolls.difference(&toremove).copied().collect();
            accessible_rolls_part2(&trimmed, previously_removed + toremove.len())
        }
    }
}

fn num_neighbours(rolls: &HashSet<Coord>, coord: &Coord) -> usize {
    let above = (coord.x, coord.y - 1);
    let below = (coord.x, coord.y + 1);
    let left = (coord.x - 1, coord.y);
    let right = (coord.x + 1, coord.y);
    let topleft = (coord.x - 1, coord.y - 1);
    let topright = (coord.x + 1, coord.y - 1);
    let bottomleft = (coord.x - 1, coord.y + 1);
    let bottomright = (coord.x + 1, coord.y + 1);
    [
        above,
        below,
        left,
        right,
        topleft,
        topright,
        bottomleft,
        bottomright,
    ]
    .iter()
    .filter(|(x, y)| *x >= 0 && *y >= 0)
    .filter(|(x, y)| {
        rolls.iter().any(|roll| {
            let coord = Coord { x: *x, y: *y };
            *roll == coord
        })
    })
    .count()
}

/// Returns coordinates of paper rolls (@)
fn parse_rolls(input: &str) -> HashSet<Coord> {
    input
        .lines()
        .take_while(|line| *line != "/n")
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices().map(move |(x, cell)| match cell {
                '@' => Some(Coord {
                    x: x as i64,
                    y: y as i64,
                }),
                _ => None,
            })
        })
        .flatten()
        .collect()
}
