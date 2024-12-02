use std::io;

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 4, "test input failed");
    println!("Test passed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(input_path: &str) -> i64 {
    let input = Input::read(input_path);

    let mut safe = 0;
    for report in input.reports {
        if any_report_permutation_ok(&report) {
            safe += 1;
        }
    }

    safe
}

fn any_report_permutation_ok(report: &[i64]) -> bool {
    // check if removal of any 1 particular value is ok.
    for to_ignore in 0..report.len() {
        let mut reports = report.to_vec();
        reports.remove(to_ignore);
        if report_ok(&reports) {
            println!("{reports:?} is safe");
            return true;
        }
    }

    println!("{report:?} is not safe");
    false
}

fn report_ok(report: &[i64]) -> bool {
    let all_increasing = report.is_sorted_by(|a, b| a < b);
    let all_decreasing = report.is_sorted_by(|a, b| a > b);

    if !(all_increasing || all_decreasing) {
        return false;
    }

    for v in report.windows(2) {
        let a = v[0];
        let b = v[1];

        let is_ok = [-1, -2, -3, 1, 2, 3].contains(&(a - b));
        if !is_ok {
            return false;
        }
    }

    true
}

struct Input {
    // each report is a list of numbers called 'levels'
    reports: Vec<Vec<i64>>,
}

impl Input {
    fn read(filename: &str) -> Input {
        let mut reports = Vec::<Vec<i64>>::new();

        for line in read_lines(filename).unwrap() {
            let line = line.unwrap();

            let parts: Vec<i64> = line
                .split_ascii_whitespace()
                .map(|e: &str| e.parse().unwrap())
                .collect();

            reports.push(parts);
        }

        Input { reports }
    }
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}
