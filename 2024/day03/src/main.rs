// https://adventofcode.com/2024/day/3
fn main() {
    // part 1
    for filename in ["example_part1", "input"] {
        let input = std::fs::read_to_string(filename).unwrap();
        println!("{filename}, computed: {}", parse_part1(&input));
    }

    // part 2
    for filename in ["example_part2", "input"] {
        let input = std::fs::read_to_string(filename).unwrap();
        println!("{filename}, computed: {}", parse_part2(&input));
    }
}

fn parse_part1(input: &str) -> i64 {
    let mut summed = 0;
    let len_mul = 4; // length of the string "mul("
    let mut input = input.to_string();
    loop {
        match input.find("mul(") {
            Some(mul) => {
                // remove every char from until (and including) "mul("
                input.replace_range(..(mul + len_mul), "");

                // verify that there is a closing parenthesis before next valid "mul("
                let next_mul = input.find("mul(");
                let closing = input.find(")");
                if closing.is_none() || next_mul.is_some() && next_mul.unwrap() < closing.unwrap() {
                    continue;
                }
                let closing = closing.unwrap();

                // separate factors from rest by closing parenthesis
                let (factors_unparsed, _) = input.split_at(closing);

                match multiply(factors_unparsed) {
                    Some(product) => summed += product,
                    None => continue,
                }
            }
            None => break,
        }
    }
    summed
}

fn parse_part2(input: &str) -> i64 {
    let mut summed = 0;
    let input = input.to_string();
    for do_calc in input.split("do()") {
        match do_calc.find("don't()") {
            Some(index) => {
                let without_dont = do_calc.split_at(index).0.to_string();
                summed += parse_part1(&without_dont);
            }
            None => summed += parse_part1(&do_calc),
        };
    }
    summed
}

fn multiply(factors_unparsed: &str) -> Option<i64> {
    // separate factors by ","
    let comma = factors_unparsed.find(",")?;

    let (factor1, factor2) = factors_unparsed.split_at(comma);
    let mut factor2 = factor2.to_string();
    factor2.remove(0); // remove comma at beginning

    // parse factors into numeric primitives
    let factor1 = factor1.parse::<i64>();
    let factor2 = factor2.parse::<i64>();

    // multiply or give up
    match (factor1, factor2) {
        (Ok(num1), Ok(num2)) => Some(num1 * num2),
        _ => None,
    }
}
