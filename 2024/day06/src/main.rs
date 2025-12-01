// https://adventofcode.com/2024/day/6
#[derive(Clone, Eq, Hash, PartialEq, Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn below(&self) -> Coordinate {
        Coordinate {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn above(&self) -> Coordinate {
        Coordinate {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn left(&self) -> Coordinate {
        Coordinate {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn right(&self) -> Coordinate {
        Coordinate {
            x: self.x + 1,
            y: self.y,
        }
    }
}

type Obstacles = Vec<Coordinate>;

#[derive(Debug)]
struct Guard {
    coord: Coordinate,
    dir: Dir,
}

#[derive(Debug)]
enum NextPose {
    Ongoing(Guard),
    Final(Guard),
}

#[derive(Debug, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
use std::collections::{HashSet, VecDeque};

use Dir::{Down, Left, Right, Up};

impl Dir {
    fn from_char(char: &char) -> Dir {
        match char {
            '^' => Up,
            'v' => Down,
            '<' => Left,
            '>' => Right,
            _ => todo!(),
        }
    }

    fn next_dir(&self) -> Dir {
        match self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }
}

fn main() {
    for filename in ["example", "input"] {
        let input = std::fs::read_to_string(filename).unwrap();
        let (obstacles, guard) = parse_field(&input);
        let border = Coordinate {
            x: input.find('\n').unwrap_or(input.len()),
            y: input.lines().count(),
        };

        let mut visited = HashSet::new();
        let visited = count_visited(&guard, &obstacles, &border, &mut visited);
        println!("The guard visited {} cells.", visited.len());

        let trap_options = variants(&guard, &obstacles, &border)
            .map(|obstacles| {
                let mut visited = HashSet::new();
                let mut guards_last_coords = VecDeque::new();
                detect_loop(
                    &guard,
                    &obstacles,
                    &border,
                    &mut visited,
                    &mut guards_last_coords,
                )
            })
            .filter(|loop_detected| *loop_detected)
            .count();
        println!("There are {} options to trap the guard.", trap_options);
    }
}

fn variants(
    guard: &Guard,
    obstacles: &Obstacles,
    border: &Coordinate,
) -> impl Iterator<Item = Obstacles> {
    // all_cells is Cartesian product of xs and ys
    let all_cells = (0..border.x)
        .flat_map(|x| (0..border.y).map(move |y| (x, y)))
        .map(|(x, y)| Coordinate { x, y });
    let new_obstacles =
        all_cells.filter(|coord| !obstacles.contains(coord) && guard.coord != *coord);
    new_obstacles.map(|coord| {
        let mut tmp = obstacles.clone();
        tmp.push(coord);
        tmp
    })
}

fn detect_loop(
    guard: &Guard,
    obstacles: &Obstacles,
    border: &Coordinate,
    visited: &mut HashSet<Coordinate>,
    guards_last_coords: &mut VecDeque<Coordinate>,
) -> bool {
    let next_guard = next_pose(obstacles, border, guard);
    update_visited_cells(guard, &next_guard, visited);

    // Either recursively evaluate the next pose or stop when the guard leaves the field.
    match next_guard {
        NextPose::Ongoing(next) => {
            if guards_last_coords
                .iter()
                .skip(4) // a loop has at least four turns
                .any(|coord| *coord == next.coord)
            {
                true
            } else {
                guards_last_coords.push_front(next.coord.clone());
                detect_loop(&next, obstacles, border, visited, guards_last_coords)
            }
        }
        NextPose::Final(_) => false,
    }
}

fn update_visited_cells(guard: &Guard, next_guard: &NextPose, visited: &mut HashSet<Coordinate>) {
    match &next_guard {
        NextPose::Ongoing(next_guard) | NextPose::Final(next_guard) => {
            match guard.dir {
                Up => (next_guard.coord.y..=guard.coord.y).for_each(|y| {
                    visited.insert(Coordinate {
                        x: next_guard.coord.x,
                        y,
                    });
                }),
                Down => (guard.coord.y..=next_guard.coord.y).for_each(|y| {
                    visited.insert(Coordinate {
                        x: next_guard.coord.x,
                        y,
                    });
                }),
                Left => (next_guard.coord.x..=guard.coord.x).for_each(|x| {
                    visited.insert(Coordinate {
                        x,
                        y: next_guard.coord.y,
                    });
                }),
                Right => (guard.coord.x..=next_guard.coord.x).for_each(|x| {
                    visited.insert(Coordinate {
                        x,
                        y: next_guard.coord.y,
                    });
                }),
            };
        }
    };
}

fn count_visited(
    guard: &Guard,
    obstacles: &Obstacles,
    border: &Coordinate,
    visited: &mut HashSet<Coordinate>,
) -> HashSet<Coordinate> {
    let next_guard = next_pose(obstacles, border, guard);
    update_visited_cells(guard, &next_guard, visited);

    // Either recursively evaluate the next pose or stop when the guard leaves the field.
    match next_guard {
        NextPose::Ongoing(next) => count_visited(&next, obstacles, border, visited),
        NextPose::Final(_) => visited.clone(),
    }
}

fn dist_to_guard(guard: &Guard, coord: &Coordinate) -> usize {
    guard.coord.x.abs_diff(coord.x) + guard.coord.y.abs_diff(coord.y)
}

/// Next pose is either an in front of an obstacle or at the border as the guard leaves the field.
/// Guard goes into direction it points and rotates afterwards.
fn next_pose(obstacles: &Obstacles, border: &Coordinate, guard: &Guard) -> NextPose {
    let next_obstacle = obstacles
        .iter()
        .filter(|ob| match guard.dir {
            Up => ob.y < guard.coord.y && ob.x == guard.coord.x,
            Down => ob.y > guard.coord.y && ob.x == guard.coord.x,
            Left => ob.x < guard.coord.x && ob.y == guard.coord.y,
            Right => ob.x > guard.coord.x && ob.y == guard.coord.y,
        })
        .map(|ob| (ob, dist_to_guard(guard, ob)))
        .min_by_key(|(_ob, dist)| *dist);

    match next_obstacle {
        Some((obstacle, _dist)) => {
            // guard moved to an obstacle
            let coord = match guard.dir {
                Up => obstacle.below(),
                Down => obstacle.above(),
                Left => obstacle.right(),
                Right => obstacle.left(),
            };
            NextPose::Ongoing(Guard {
                coord,
                dir: guard.dir.next_dir(),
            })
        }
        None => {
            // guard reaches border and leaves the field
            let coord = match guard.dir {
                Up => Coordinate {
                    x: guard.coord.x,
                    y: 0,
                },
                Down => Coordinate {
                    x: guard.coord.x,
                    y: border.y - 1,
                },
                Left => Coordinate {
                    x: 0,
                    y: guard.coord.y,
                },
                Right => Coordinate {
                    x: border.x - 1,
                    y: guard.coord.y,
                },
            };
            NextPose::Final(Guard {
                coord,
                dir: guard.dir,
            })
        }
    }
}

/// Returns the location of the obstacles and the pose of the guard.
fn parse_field(input: &str) -> (Obstacles, Guard) {
    let mut obstacles = vec![];
    let mut player = Guard {
        coord: Coordinate { x: 0, y: 0 },
        dir: Up,
    };
    for (y, line) in input.lines().enumerate() {
        for (x, cell) in line.char_indices() {
            match cell {
                '#' => obstacles.push(Coordinate { x, y }),
                '^' | 'v' | '<' | '>' => {
                    player = Guard {
                        coord: Coordinate { x, y },
                        dir: Dir::from_char(&cell),
                    }
                }
                _ => continue,
            }
        }
    }

    (obstacles, player)
}
