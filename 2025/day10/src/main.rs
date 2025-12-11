use std::{fmt::Debug, str::FromStr};

use rand::prelude::*;
use rayon::prelude::*;

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    multibuttons: Vec<Vec<u8>>,
    joltages: Vec<u32>,
}

fn main() {
    for path in ["input/example", "input/input"] {
        let input = std::fs::read_to_string(path).unwrap();
        let parsed = parse(&input);
        let solution1 = part1(&parsed);
        println!("{solution1}");
    }
}

fn part1(input: &[Machine]) -> usize {
    input
        .par_iter()
        .map(|machine| {
            let mut least_trys = usize::MAX;
            for _ in 0..machine.multibuttons.len() * 1000 {
                let random_run_trys = random_run_part1(machine);
                if random_run_trys < least_trys {
                    least_trys = random_run_trys;
                }
            }
            // println!("{least_trys}");
            least_trys
        })
        .sum()
}

fn random_run_part1(input: &Machine) -> usize {
    let mut rng = rand::rng();
    let limit = input.multibuttons.len() * 10;
    let mut lights_init = vec![false; input.lights.len()];
    let mut lastbutton = usize::MAX;

    let mut i = 0;
    while i < limit {
        i += 1;
        let mut nextbutton = rng.random_range(0..input.multibuttons.len());
        while nextbutton == lastbutton {
            // never press a button twice
            nextbutton = rng.random_range(0..input.multibuttons.len());
        }
        lastbutton = nextbutton;

        // order.push(nextbutton);
        pushbuttons_part1(&input.multibuttons[nextbutton], &mut lights_init);
        if input.lights == lights_init {
            break;
        }
    }
    i
}

fn pushbuttons_part1(buttons: &[u8], lights: &mut [bool]) {
    for button in buttons {
        lights[*button as usize] = !lights[*button as usize];
    }
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| {
            let mut parsed = Machine {
                lights: vec![],
                multibuttons: vec![],
                joltages: vec![],
            };
            for word in line.split_whitespace() {
                match word.chars().next() {
                    Some('[') => parsed.lights = parse_lights(word),
                    Some('(') => parsed.multibuttons.push(parse_numbers(word)),
                    Some('{') => parsed.joltages = parse_numbers(word),
                    _ => break,
                }
            }
            parsed
        })
        .collect()
}

fn parse_lights(s: &str) -> Vec<bool> {
    s.chars()
        .filter(|c| *c == '.' || *c == '#')
        .map(|c| match c {
            '.' => false,
            '#' => true,
            _ => panic!(),
        })
        .collect()
}

/// Parses buttons and joltages.
fn parse_numbers<T: FromStr>(s: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    s[1..(s.len() - 1)] // remove first and last bracket
        .split(',')
        .map(|num| num.parse::<T>().unwrap())
        .collect()
}
