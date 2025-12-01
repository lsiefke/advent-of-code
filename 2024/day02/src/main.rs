// https://adventofcode.com/2024/day/2
fn main() {
    for filename in ["example", "input"] {
        let reports = parse(filename);
        let verdicts = reports.iter().map(|report| safety_check(report, false));
        println!(
            "{} reports are safe without problem dampener",
            verdicts.filter(|verdict| *verdict).count()
        );
        let use_problem_dampener = true;
        let verdicts = reports
            .iter()
            .map(|report| safety_check(report, use_problem_dampener));
        println!(
            "{} reports are safe with problem dampener",
            verdicts.filter(|verdict| *verdict).count()
        );
        // for report in reports {
        //     let verdict = safety_check_with_dampener(&report, problem_dampener);
        //     println!("report: {report:?}, safe (with problem dampener): {verdict}");
        // }
    }
}

fn parse(filename: &str) -> Vec<Vec<i32>> {
    let input = std::fs::read_to_string(filename).unwrap();

    input
        .lines()
        .map(|line| line.split(" "))
        .map(|levels| levels.map(|level| level.parse::<i32>().unwrap()).collect())
        .collect()
}

/// Rule 1: numbers are all increasing or all decreasing
/// Rule 2: adjacent numbers differ by either 1, 2, or 3
fn safety_check(report: &[i32], use_problem_dampener: bool) -> bool {
    let decreasing = report.is_sorted_by(|a, b| a > b);
    let increasing = report.is_sorted_by(|a, b| a < b);
    let rule1 = decreasing || increasing;

    let rule2 = report.windows(2).fold(true, |acc, neighbours| {
        let diff = neighbours[0].abs_diff(neighbours[1]);
        let within_range = (1..=3).contains(&diff);
        acc && within_range
    });

    let problem_resolved = if !(rule1 && rule2) && use_problem_dampener {
        problem_dampener(report)
    } else {
        false
    };

    (rule1 && rule2) || problem_resolved
}

/// build all possible reports with each one missing one level.
/// if at least one of them passes the safety check, then the problem dampener succeeded.
fn problem_dampener(report: &[i32]) -> bool {
    let safes_with_dampening = (0..report.len())
        .map(|i| {
            let mut dampened_report = report.to_vec();
            dampened_report.remove(i);
            dampened_report
        })
        .map(|new_hope| safety_check(&new_hope, false))
        .filter(|verdict| *verdict)
        .count();
    return safes_with_dampening > 0;
}
