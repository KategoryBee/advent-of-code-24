use std::io;

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 48, "test input failed");
    println!("Test passed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(input_path: &str) -> i64 {
    let rx = regex::Regex::new(r"don't|do|mul\((\d+),(\d+)\)").unwrap();

    let mut total = 0;
    let mut mul_enabled = true;

    for line in read_lines(input_path).unwrap() {
        let line = line.unwrap();

        for caps in rx.captures_iter(&line) {
            if &caps[0] == "do" {
                mul_enabled = true;
            } else if &caps[0] == "don't" {
                mul_enabled = false;
            } else {
                assert!(caps[0].starts_with("mul"));
                let a: i64 = caps[1].parse().unwrap();
                let b: i64 = caps[2].parse().unwrap();

                if mul_enabled {
                    total += a * b;
                }
            }
        }
    }

    total
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}
