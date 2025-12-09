use std::cmp::{max, min};

use geo::Covers;
use geo_types::{LineString, Polygon, Rect, coord};

#[derive(Debug, Copy, Clone)]
struct Coord {
    x: u64,
    y: u64,
}

fn main() {
    for path in ["input/example", "input/input"] {
        let input = std::fs::read_to_string(path).unwrap();
        let redtiles = parse(&input);
        let solution1 = part1(&redtiles);
        println!("{path} - part1: {solution1}");
        let solution2 = part2(&redtiles);
        println!("{path} - part2: {solution2}");
    }
}

fn part1(redtiles: &[Coord]) -> u64 {
    allrects(redtiles)
        .iter()
        .map(|(coord1, coord2)| rectsize(coord1, coord2))
        .max()
        .unwrap_or(0)
}

fn part2(redtiles: &[Coord]) -> u64 {
    let poly: Polygon<f32> = Polygon::new(
        LineString::from(
            redtiles
                .iter()
                .map(|coord| (coord.x as f32, coord.y as f32))
                .collect::<Vec<_>>(),
        ),
        vec![],
    );

    allrects(redtiles)
        .into_iter()
        .filter(|(coord1, coord2)| overlaps(coord1, coord2, &poly))
        .map(|(coord1, coord2)| rectsize(&coord1, &coord2))
        .max()
        .unwrap_or(0)
}

fn allrects(redtiles: &[Coord]) -> Vec<(Coord, Coord)> {
    let mut rectangles = vec![];
    for i in 0..(redtiles.len() - 1) {
        for j in (i + 1)..redtiles.len() {
            rectangles.push((redtiles[i], redtiles[j]));
        }
    }
    rectangles
}

/// Checks whether a point lies within or on the boundary of a polygon.
fn overlaps(coord1: &Coord, coord2: &Coord, polygon: &Polygon<f32>) -> bool {
    let rect = Rect::new(
        coord! {x: coord1.x as f32, y: coord1.y as f32},
        coord! {x: coord2.x as f32, y: coord2.y as f32},
    );
    polygon.covers(&rect)
}

fn rectsize(coord1: &Coord, coord2: &Coord) -> u64 {
    let lenx = max(coord1.x, coord2.x) + 1 - min(coord1.x, coord2.x);
    let leny = max(coord1.y, coord2.y) + 1 - min(coord1.y, coord2.y);
    lenx * leny
}

fn parse(input: &str) -> Vec<Coord> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut numiter = line.split(',');
            Coord {
                x: numiter.next().unwrap().parse().unwrap(),
                y: numiter.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}
