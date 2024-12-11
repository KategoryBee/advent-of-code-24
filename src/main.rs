use std::io;

use num::Integer;

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 55312, "test input failed");
    println!("Test passed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(input_path: &str) -> usize {
    let input = read_input(input_path);

    let mut total = 0;
    for stone in input {
        total += stones_after_iters(&stone, 25);
    }

    total
}

fn stones_after_iters(stone: &str, iterations_left: i32) -> usize {
    if iterations_left == 0 {
        return 1;
    }

    let iters_remain = iterations_left - 1;

    if stone == "0" {
        stones_after_iters("1", iters_remain)
    } else if stone.len().is_even() {
        let mid = stone.len() / 2;
        let (l, r) = stone.split_at(mid);

        let r = r.trim_start_matches('0');
        let r = if r.is_empty() { "0" } else { r };

        stones_after_iters(l, iters_remain) + stones_after_iters(r, iters_remain)
    } else {
        let mulleed: usize = stone.parse::<usize>().unwrap() * 2024;

        stones_after_iters(&mulleed.to_string(), iters_remain)
    }
}

fn read_input(input_path: &str) -> Vec<String> {
    let line = read_lines(input_path).unwrap().next().unwrap().unwrap();
    line.split_ascii_whitespace()
        .map(|x| x.to_owned())
        .collect()
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}
