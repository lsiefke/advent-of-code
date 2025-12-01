use std::collections::{HashMap, HashSet};

type Rules = HashMap<u32, HashSet<u32>>;
type Update = Vec<u32>;

fn main() {
    for filename in ["example", "input"] {
        let input = std::fs::read_to_string(filename).unwrap();
        // println!("{:#?}", rules(&input));
        // println!("{:#?}", updates(input));
        let rules = rules(&input);
        let updates = updates(&input);

        // for update in &updates {
        //     let verdict = check_updates(&rules, &update);
        //     println!("{:?} - {verdict}", update);
        // }

        println!("sum of middle-pages: {}", middlepage_sums(&rules, &updates));
        let corrections: Vec<Update> = updates
            .iter()
            .map(|update| (update, check_updates(&rules, update)))
            .filter(|(_update, verdict)| !*verdict)
            .map(|(update, _verdict)| correct_update(&rules, update))
            .collect();
        println!(
            "corrections - sum of middle-pages: {}",
            middlepage_sums(&rules, &corrections)
        );
    }
}

fn middlepage_sums(rules: &Rules, updates: &[Update]) -> u32 {
    updates
        .iter()
        .map(|update| (update, check_updates(&rules, update)))
        .filter(|(_update, verdict)| *verdict)
        .map(|(update, _verdict)| update[(update.len() - 1) / 2])
        .sum::<u32>()
}

fn correct_update(rules: &Rules, update: &Update) -> Update {
    // Each page is mapped to a value (priority), which tells how often it is found in rules from other pages.
    // Then, they are sorted with respect to their priority.
    let mut priorities: Vec<(u32, usize)> = update
        .iter()
        .map(|page| {
            let priority = update
                .iter()
                .map(|p| {
                    rules
                        .get(p)
                        .and_then(|p| Some(p.contains(page)))
                        .unwrap_or(false) // Happens as pages have no priority rules for themselfes
                })
                .filter(|contains| *contains)
                .count();
            (*page, priority)
        })
        .collect();

    priorities.sort_by_key(|(_page, priority)| *priority);
    let corrected = priorities.iter().map(|(page, _priority)| *page).collect();

    corrected
}

fn check_updates(rules: &Rules, update: &Update) -> bool {
    for i in 0..update.len() {
        let page = update[i];
        let following_pages = update.iter().skip(i + 1);
        if !check_following(rules, page, following_pages) {
            return false;
        }
    }
    true
}

fn check_following<'a>(
    rules: &Rules,
    page: u32,
    following_pages: impl Iterator<Item = &'a u32>,
) -> bool {
    let page_valid = following_pages.fold(true, |all_valid, following| {
        let following_valids = rules.get(&page);
        if following_valids.is_none() {
            return false;
        }
        let following_valids = following_valids.unwrap();
        all_valid && following_valids.contains(following)
    });

    page_valid
}

fn updates(input: &str) -> Vec<Update> {
    let mut updates = vec![];

    for line in input.lines() {
        let comma = line.find(",");
        if comma.is_none() {
            continue;
        }
        let mut pages = vec![];

        // comma separated numbers
        for num in line.split(",") {
            pages.push(num.parse::<u32>().unwrap());
        }

        updates.push(pages);
    }
    updates
}

fn rules(input: &str) -> Rules {
    let mut map = HashMap::new();

    for line in input.lines() {
        let mid = line.find("|");
        if mid.is_none() {
            break;
        }

        let (x, y) = line.split_at(mid.unwrap());
        let mut y = y.to_string();
        y.remove(0); // remove "|"
        let x = x.to_string().parse::<u32>().unwrap();
        let y = y.to_string().parse::<u32>().unwrap();
        match map.contains_key(&x) {
            true => {
                let set: &mut HashSet<u32> = map.get_mut(&x).unwrap();
                set.insert(y);
            }
            false => {
                let mut set = HashSet::new();
                set.insert(y);
                map.insert(x, set);
            }
        };
    }

    map
}
