fn main() {
    for path in ["example", "input"] {
        let input = std::fs::read_to_string(path).unwrap();
        let mut decoded = decode(&input);
        let mut filesystem = decoded.clone();

        compact(&mut decoded);
        compact_keepfiles(&mut filesystem);

        println!("checksum: {}", checksum(&decoded));
        println!("checksum part2: {}", checksum(&filesystem));
    }
}

fn checksum(filesystem: &[Option<u64>]) -> u64 {
    filesystem
        .iter()
        .enumerate()
        .map(|(i, block)| match block {
            None => 0,
            Some(block) => i as u64 * block,
        })
        .sum()
}

fn compact_keepfiles(filesystem: &mut [Option<u64>]) {
    let mut file_start = filesystem.len() - 1;
    let mut file_end = filesystem.len() - 1;
    let mut file_id = filesystem.last().unwrap().unwrap_or(0);
    for i in 0..filesystem.len() {
        let irev = filesystem.len() - 1 - i;

        if let Some(block) = filesystem[irev] {
            if block != file_id {
                swap_emptyspace(filesystem, file_start, file_end);
                file_start = irev;
                file_end = irev;
                file_id = block;
            } else {
                file_start = irev;
            }
        }
    }
}

fn swap_emptyspace(filesystem: &mut [Option<u64>], start: usize, end: usize) {
    let idx_space = filesystem
        .iter()
        .enumerate()
        .take_while(|(i, _)| *i < start)
        .position(|(i, _)| {
            filesystem
                .iter()
                .skip(i)
                .take(end - start + 1)
                .all(|b| b.is_none())
        });
    if let Some(i) = idx_space {
        // swap empty space with block
        for j in start..=end {
            filesystem.swap(i + j - start, j);
        }
    }
}

fn compact(filesystem: &mut [Option<u64>]) {
    for i in 0..filesystem.len() {
        if filesystem[i].is_none() {
            let last = filesystem.len()
                - 1
                - filesystem
                    .iter()
                    .rev()
                    .position(|block| block.is_some())
                    .unwrap();
            if last <= i {
                break;
            }
            // switch empty space with block
            filesystem[i] = filesystem[last];
            filesystem[last] = None;
        }
    }
}

fn decode(input: &str) -> Vec<Option<u64>> {
    let base = 10;
    input
        .char_indices()
        .take_while(|(_i, char)| char.is_digit(base))
        .flat_map(|(i, char)| {
            let num = char.to_digit(base).unwrap();
            match i % 2 {
                0 => vec![Some((i / 2) as u64); num as usize],
                1 => vec![None; num as usize],
                _ => panic!(),
            }
        })
        .collect()
}
