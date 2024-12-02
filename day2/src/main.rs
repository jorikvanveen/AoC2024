fn get_input() -> Vec<Vec<i32>> {
    std::fs::read_to_string("input")
        .unwrap()
        .split("\n")
        .filter(|report| report != &"")
        .map(|report| {
            report
                .trim()
                .split(" ")
                .map(|level| level.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    let is_increasing = (0..report.len()-1).all(|idx| { report[idx] < report[idx+1] });
    let is_decreasing = (0..report.len()-1).all(|idx| { report[idx] > report[idx+1] });
    let deltas: Vec<_> = (0..report.len()-1).map(|idx| (report[idx] - report[idx+1]).abs()).collect();
    let max_delta = deltas.iter().max().unwrap();
    let min_delta = deltas.iter().min().unwrap();

    (is_increasing || is_decreasing) && min_delta >= &1 && max_delta <= &3
}

fn part1() {
    let reports = get_input();
    let safe_reports: Vec<_> = reports.into_iter().filter(is_report_safe).collect();
    println!("{}", safe_reports.len());
}

fn part2() {
    let reports = get_input();
    let safe_reports: Vec<_> = reports.into_iter().filter(|report| {
        (0..report.len()).map(|idx| {
            let mut cloned_report = report.clone();
            cloned_report.remove(idx);
            cloned_report
        }).any(|report| is_report_safe(&report))
    }).collect();

    println!("{}", safe_reports.len());
}

fn main() {
    part1();
    part2()
}
