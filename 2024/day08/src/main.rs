use std::collections::{HashMap, HashSet};

type Antennas = HashMap<char, Vec<Coord>>;
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Coord {
    x: i64,
    y: i64,
}

fn main() {
    for filename in ["example", "input"] {
        let input = std::fs::read_to_string(filename).unwrap();
        let boundary = parse_boundary(&input);
        let antennas = parse_antennas(&input);
        let antinodes = calc_antinodes(&antennas, &boundary);
        let antinodes_with_harmonics = calc_antinodes_with_harmonics(&antennas, &boundary);

        println!("number of antinodes using {filename}: {}", antinodes.len());
        println!(
            "number of antinodes (with harmonics) using {filename}: {}",
            antinodes_with_harmonics.len()
        );
    }
}

fn calc_antinodes(antennas: &Antennas, boundary: &Coord) -> HashSet<Coord> {
    antennas
        .values()
        .map(|ants_freq| {
            let cartesian = ants_freq
                .iter()
                .flat_map(|y| ants_freq.iter().map(move |x| (x, y)));
            cartesian
                .filter(|(ant1, ant2)| ant1 != ant2)
                .map(antpair_antinodes)
                .flatten()
                .filter(|ant| coord_within_bound(ant, boundary))
        })
        .flatten()
        .collect()
}

fn calc_antinodes_with_harmonics(antennas: &Antennas, boundary: &Coord) -> HashSet<Coord> {
    antennas
        .values()
        .map(|ants_freq| {
            let cartesian = ants_freq
                .iter()
                .flat_map(|y| ants_freq.iter().map(move |x| (x, y)));
            cartesian
                .filter(|(ant1, ant2)| ant1 != ant2)
                .map(|ants| antpair_antinodes_harmonics(ants, boundary))
                .flatten()
                .filter(|ant| coord_within_bound(ant, boundary))
        })
        .flatten()
        .collect()
}

/// Boundary is non-inclusive
fn coord_within_bound(coord: &Coord, boundary: &Coord) -> bool {
    coord.x >= 0 && coord.x < boundary.x && coord.y >= 0 && coord.y < boundary.y
}

fn antpair_antinodes((ant1, ant2): (&Coord, &Coord)) -> Vec<Coord> {
    let anti1 = Coord {
        x: ant1.x + 2 * (ant2.x - ant1.x),
        y: ant1.y + 2 * (ant2.y - ant1.y),
    };
    let anti2 = Coord {
        x: ant2.x + 2 * (ant1.x - ant2.x),
        y: ant2.y + 2 * (ant1.y - ant2.y),
    };
    vec![anti1, anti2]
}

fn antpair_antinodes_harmonics((ant1, ant2): (&Coord, &Coord), boundary: &Coord) -> Vec<Coord> {
    let forwards = (0..)
        .map(|i| Coord {
            x: ant1.x + 2 * i * (ant2.x - ant1.x),
            y: ant1.y + 2 * i * (ant2.y - ant1.y),
        })
        .take_while(|coord| coord_within_bound(coord, boundary));
    let backwards = (0..)
        .map(|i| Coord {
            x: ant1.x - 2 * i * (ant2.x - ant1.x),
            y: ant1.y - 2 * i * (ant2.y - ant1.y),
        })
        .take_while(|coord| coord_within_bound(coord, boundary));

    forwards.chain(backwards).collect()
}

/// Boundary is non-inclusive
fn parse_boundary(input: &str) -> Coord {
    Coord {
        x: input.lines().last().unwrap().len() as i64,
        y: input.lines().count() as i64,
    }
}

fn parse_cell(cell: char, x: usize, y: usize, map: &mut Antennas) {
    if cell != '.' {
        let coord = Coord {
            x: x as i64,
            y: y as i64,
        };
        match map.get_mut(&cell) {
            Some(list) => {
                list.push(coord);
            }
            None => {
                map.insert(cell, vec![coord]);
            }
        };
    }
}

fn parse_antennas(input: &str) -> Antennas {
    let mut map: Antennas = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, cell) in line.char_indices() {
            parse_cell(cell, x, y, &mut map);
        }
    }
    map
}
