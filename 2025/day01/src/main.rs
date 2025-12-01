struct Rot {
    dir: char,
    dist: i32,
}

fn main() {
    for path in ["test", "example", "input"] {
        let input = std::fs::read_to_string(path).unwrap();
        let rotations = parse(&input);
        let dial = 50;
        let passwd1 = password_part1(dial, &rotations);
        let passwd2 = password_part2(dial, &rotations);

        println!("Password Part1: {passwd1}, Password Part2: {passwd2}");
    }
}

fn password_part1(init: i32, rotations: &[Rot]) -> u32 {
    let (_dial, passwd) = rotations.iter().fold((init, 0), |(dial, passwd), rot| {
        let newstate = match rot.dir {
            'L' => dial - rot.dist,
            _ => dial + rot.dist,
        };

        let newdial = match newstate % 100 {
            100 => 0,
            0.. => newstate % 100,               // positive newdial
            ..0 => 100 - (newstate.abs() % 100), // negative newdial
        };

        let passwd = if newdial == 0 { passwd + 1 } else { passwd };
        (newdial, passwd)
    });
    passwd
}

fn password_part2(init: i32, rotations: &[Rot]) -> u32 {
    let (_dial, passwd) = rotations.iter().fold((init, 0), |(dial, passwd), rot| {
        let newstate = match rot.dir {
            'L' => dial - rot.dist,
            _ => dial + rot.dist,
        };

        let clicks0 = match rot.dir {
            'L' => {
                if dial == 0 {
                    // if current dial is 0, it was counted with the last rotation already. skip this one
                    (dial + rot.dist) as u32 / 100
                } else {
                    ((100 - dial) + rot.dist) as u32 / 100
                }
            }
            _ => newstate as u32 / 100,
        };

        let newdial = match newstate % 100 {
            100 => 0,
            0.. => newstate % 100,               // positive newdial
            ..0 => 100 - (newstate.abs() % 100), // negative newdial
        };

        (newdial, passwd + clicks0)
    });
    passwd
}

fn parse(input: &str) -> Vec<Rot> {
    input
        .lines()
        .map(|line| {
            let (dir, dist) = line.split_at(1);
            Rot {
                dir: dir.chars().nth(0).unwrap(),
                dist: dist.parse().unwrap(),
            }
        })
        .collect()
}
