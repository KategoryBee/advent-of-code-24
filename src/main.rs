use std::io;

use itertools::Itertools;
use regex::Regex;

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 480, "test input failed");
    println!("Test passed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(input_path: &str) -> i32 {
    let input = read_input(input_path);

    let mut total = 0;

    for p in input {
        total += solve_puzzle(p);
    }

    total
}

fn solve_puzzle(p: Puzzle) -> i32 {
    // per spec, the buttons need to be pressed at most 100 times. this is only a space of 10,000
    // so brute force

    let mut lowest: Option<i32> = None;

    for (a, b) in itertools::iproduct!(0..=100, 0..=100) {
        let pos_x = p.a_x * a + p.b_x * b;
        let pos_y = p.a_y * a + p.b_y * b;

        if pos_x == p.prize_x && pos_y == p.prize_y {
            let tokens_spent = a * 3 + b;
            lowest = Some(match lowest {
                None => tokens_spent,
                Some(current_min) => current_min.min(tokens_spent),
            });
        }
    }

    lowest.unwrap_or(0)
}

struct Puzzle {
    a_x: i32,
    a_y: i32,
    b_x: i32,
    b_y: i32,
    prize_x: i32,
    prize_y: i32,
}

fn read_input(input_path: &str) -> Vec<Puzzle> {
    let mut res = Vec::new();

    let input = read_lines(input_path).unwrap().filter_map(|f| {
        let line = f.unwrap();
        if line.is_empty() {
            None
        } else {
            Some(line)
        }
    });

    let xy_pattern = Regex::new(r"X[=+](\d+), Y[=+](\d+)").unwrap();

    for (a, b, prize) in input.tuples() {
        let (_, [a_x, a_y]) = xy_pattern.captures(&a).unwrap().extract();
        let (_, [b_x, b_y]) = xy_pattern.captures(&b).unwrap().extract();
        let (_, [prize_x, prize_y]) = xy_pattern.captures(&prize).unwrap().extract();

        res.push(Puzzle {
            a_x: a_x.parse().unwrap(),
            a_y: a_y.parse().unwrap(),
            b_x: b_x.parse().unwrap(),
            b_y: b_y.parse().unwrap(),
            prize_x: prize_x.parse().unwrap(),
            prize_y: prize_y.parse().unwrap(),
        });
    }

    res
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}
