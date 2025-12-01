// https://adventofcode.com/2024/day/4
fn main() {
    for filename in ["example", "input"] {
        let input = std::fs::read_to_string(filename).unwrap();
        println!(
            "XMAS was counted {} times in {filename}.",
            xmax_counter(&input)
        );
        println!("X-MAS was counted {} times in {filename}.", part2(&input));
    }
}

fn part2(input: &str) -> usize {
    let mut input = input.to_string();
    if input.ends_with('\n') {
        input.pop();
    }
    let lines = horizontals(&input);
    let mut sum = 0;
    for y in 1..(lines.len() - 1) {
        for x in 1..(lines[0].len() - 1) {
            if char_at(&lines, y, x) == 'A' {
                if (char_at(&lines, y - 1, x - 1) == 'M'
                    && char_at(&lines, y - 1, x + 1) == 'S'
                    && char_at(&lines, y + 1, x - 1) == 'M'
                    && char_at(&lines, y + 1, x + 1) == 'S')
                    || (char_at(&lines, y - 1, x - 1) == 'S'
                        && char_at(&lines, y - 1, x + 1) == 'S'
                        && char_at(&lines, y + 1, x - 1) == 'M'
                        && char_at(&lines, y + 1, x + 1) == 'M')
                    || (char_at(&lines, y - 1, x - 1) == 'M'
                        && char_at(&lines, y - 1, x + 1) == 'M'
                        && char_at(&lines, y + 1, x - 1) == 'S'
                        && char_at(&lines, y + 1, x + 1) == 'S')
                    || (char_at(&lines, y - 1, x - 1) == 'S'
                        && char_at(&lines, y - 1, x + 1) == 'M'
                        && char_at(&lines, y + 1, x - 1) == 'S'
                        && char_at(&lines, y + 1, x + 1) == 'M')
                {
                    {
                        sum += 1;
                    }
                }
            }
        }
    }

    sum
}

fn char_at(lines: &[String], y: usize, x: usize) -> char {
    lines[y].char_indices().nth(x).unwrap().1
}

fn xmax_counter(input: &str) -> usize {
    // input is assumed to have same length for every row. No trailing new line
    let mut input = input.to_string();
    if input.ends_with('\n') {
        input.pop();
    }
    let mut strings = vec![];
    strings.append(&mut horizontals(&input));
    strings.append(&mut verticals(&input));
    strings.append(&mut diagonals(&input));

    let xmas_sum = strings
        .iter()
        .map(|s: &String| s.matches("XMAS").count())
        .sum::<usize>();
    let xmas_reverted_sum = strings
        .iter()
        .map(|s: &String| s.matches("SAMX").count())
        .sum::<usize>();
    xmas_sum + xmas_reverted_sum
}

fn horizontals(input: &str) -> Vec<String> {
    let mut ret = vec![];
    for s in input.split('\n') {
        ret.push(s.to_string());
    }
    ret
}

fn verticals(input: &str) -> Vec<String> {
    let mut ret = vec![];
    let nrows = input.find("\n").unwrap_or(input.len());
    let ncols = input.len() / nrows;
    assert_eq!(nrows, ncols);
    let input = input.replace("\n", "");

    for j in 0..ncols {
        let mut col = String::with_capacity(nrows);
        for i in 0..nrows {
            col.push(input.chars().nth(i * nrows + j).unwrap());
        }
        ret.push(col);
    }
    ret
}

fn diagonals(input: &str) -> Vec<String> {
    let mut ret = vec![];
    let nrows = input.find("\n").unwrap_or(input.len());
    let ncols = input.len() / nrows;
    let input = input.replace("\n", "");

    // direction: top-left to bottom-right, lower half
    for i in 0..nrows {
        let mut diag = String::with_capacity(nrows);
        for j in 0..ncols {
            let index = (i + j) * nrows + j;
            match input.chars().nth(index) {
                Some(character) => diag.push(character),
                None => continue,
            };
        }
        ret.push(diag);
    }

    // direction: top-left to bottom-right, upper half
    for i in 1..(nrows) {
        let mut diag = String::with_capacity(nrows);
        for j in 0..ncols {
            let index = (i + j) * nrows + j;
            match input.chars().nth_back(index) {
                Some(character) => diag.push(character),
                None => continue,
            };
        }
        ret.push(diag);
    }

    // direction: top-right to bottom-left, lower half
    for i in 0..ncols {
        let mut diag = String::with_capacity(nrows);
        for j in 0..nrows {
            let index = (ncols - 1 - i + nrows - 1 - j) * nrows + j;
            match input.chars().nth(index) {
                Some(character) => diag.push(character),
                None => continue,
            };
        }
        ret.push(diag);
    }

    // direction: top-right to bottom-left, upper half
    for i in 0..(ncols - 1) {
        let mut diag = String::with_capacity(nrows);
        for j in 0..nrows {
            let index = (ncols - 1 - i + nrows - 1 - j) * nrows + j;
            match input.chars().nth_back(index) {
                Some(character) => diag.push(character),
                None => continue,
            };
        }
        ret.push(diag);
    }
    ret
}
