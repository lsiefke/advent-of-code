use cached::proc_macro::cached;

struct Coord {
    col: usize,
    row: usize,
}

fn main() {
    for path in ["input/example", "input/input"] {
        let input = std::fs::read_to_string(path).unwrap();

        // part1
        let field_with_beams = process_beam_part1(&input);
        let nsplittings = count_splittings(&field_with_beams);
        print_beams(&field_with_beams);
        println!("{path} - part1: {nsplittings}");

        // part2
        let field = parse(&input);
        let start = field[0].iter().position(|cell| *cell == 'S').unwrap();
        let ntimelines = count_timelines(&field, Coord { row: 0, col: start }, 0);
        println!("{path} - part2: {ntimelines}");
    }
}

// Memoisation with cached gave some troubles with the lifetime of field, so a custom key is used here.
#[cached(
    key = "String",
    convert = r##"{ format!("{}:{}:{}:{}", field.len(), position.row, position.col, counter) }"##
)]
fn count_timelines(field: &[Vec<char>], position: Coord, counter: usize) -> usize {
    if position.row == field.len() - 1 {
        counter + 1
    } else {
        let cell_below = field[position.row + 1][position.col];
        match cell_below {
            '^' => {
                let next1 = Coord {
                    col: position.col - 1,
                    row: position.row + 1,
                };
                let next2 = Coord {
                    col: position.col + 1,
                    row: position.row + 1,
                };
                count_timelines(field, next1, counter) + count_timelines(field, next2, counter)
            }
            _ => {
                let next = Coord {
                    row: position.row + 1,
                    ..position
                };
                count_timelines(field, next, counter)
            }
        }
    }
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn process_beam_part1(input: &str) -> Vec<Vec<char>> {
    let field_with_beams: Vec<Vec<_>> = input.lines().fold(
        vec![],
        |mut previous_lines, line_now| match previous_lines.last() {
            None => {
                previous_lines.push(line_now.chars().collect());
                previous_lines
            }
            Some(line_above) => {
                previous_lines.push(process_line(line_above, line_now));
                previous_lines
            }
        },
    );
    field_with_beams
}

fn process_line(line_above: &[char], line_now: &str) -> Vec<char> {
    line_now
        .char_indices()
        .map(|(i, cell)| {
            if cell == '^' {
                return '^';
            }
            let above = line_above.get(i).unwrap();
            if *above == 'S' || *above == '|' {
                return '|';
            }
            let topleft = if i > 0 { line_above.get(i - 1) } else { None };
            let topright = line_above.get(i + 1);

            match (topleft, topright) {
                (Some('^'), _) | (_, Some('^')) => '|',
                _ => '.',
            }
        })
        .collect()
}

fn count_splittings(field_with_beams: &[Vec<char>]) -> usize {
    let neighbouring_lines = field_with_beams.iter().zip(field_with_beams.iter().skip(1));
    neighbouring_lines
        .flat_map(|(line_prev, line_now)| {
            line_now
                .iter()
                .enumerate()
                .map(|(i, cell)| {
                    let above = line_prev[i];
                    match (cell, above) {
                        ('^', '|') => 1,
                        _ => 0,
                    }
                })
                .collect::<Vec<_>>()
        })
        .sum()
}

fn print_beams(field_with_beams: &[Vec<char>]) {
    for line in field_with_beams {
        for cell in line {
            print!("{cell}");
        }
        println!();
    }
}
