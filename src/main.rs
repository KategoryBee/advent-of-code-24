use std::{collections::HashMap, io};

use num::Integer;

fn main() {
    // let test_result = solve("test.txt");
    // assert_eq!(test_result, 55312, "test input failed");
    // println!("Test passed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(input_path: &str) -> usize {
    let input = read_input(input_path);

    let mut memoized = HashMap::new();
    let mut total = 0;
    for stone in input.iter() {
        total += stones_after_iters(&stone, 75, &mut memoized);
    }

    total
}

fn stones_after_iters<'a>(
    stone: &'a str,
    iterations_left: i32,
    memoized: &mut HashMap<(&'a str, i32), usize>,
) -> usize {
    if let Some(&r) = memoized.get(&(stone, iterations_left)) {
        return r;
    }

    if iterations_left == 0 {
        return 1;
    }

    let iters_remain = iterations_left - 1;

    let res = if stone == "0" {
        stones_after_iters("1", iters_remain, memoized)
    } else if stone.len().is_even() {
        let mid = stone.len() / 2;
        let (l, r) = stone.split_at(mid);

        let r = r.trim_start_matches('0');
        let r = if r.is_empty() { "0" } else { r };

        stones_after_iters(l, iters_remain, memoized)
            + stones_after_iters(r, iters_remain, memoized)
    } else {
        let mulleed: usize = stone.parse::<usize>().unwrap() * 2024;

        // Eh just leak it rather than appease the borrow checker. We're dynamic programming anyway
        // and have a short runtime.
        stones_after_iters(mulleed.to_string().leak(), iters_remain, memoized)
    };

    memoized.insert((stone, iterations_left), res);
    res
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
