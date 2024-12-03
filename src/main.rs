use std::io;

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 161, "test input failed");
    println!("Test passed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(input_path: &str) -> i64 {
    let rx = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut total = 0;

    for line in read_lines(input_path).unwrap() {
        let line = line.unwrap();

        for (_, [a, b]) in rx.captures_iter(&line).map(|c| c.extract()) {
            let a: i64 = a.parse().unwrap();
            let b: i64 = b.parse().unwrap();
            total += a * b;
        }
    }

    total
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}
