use std::{collections::VecDeque, io};

use num::Integer;

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 1928, "test input failed");
    println!("Test passed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(input_path: &str) -> usize {
    let mut input = read_input(input_path);

    let mut total = 0;
    let mut position = 0;
    while !input.is_empty() {
        // We consume the front 1 'len' at a time, each iteration.
        let front = input.front_mut().unwrap();
        let front_id = front.id;

        assert!(front.len > 0);
        front.len -= 1;
        if front.len == 0 {
            input.pop_front();
        }

        if let Some(file_id) = front_id {
            total += position * file_id;
            position += 1;
            continue;
        }

        // If we get here, there's empty space on the front that we need to fill by reading from
        // the back. We _might_ actually already be at the end too, or perhaps the front and back
        // blocks are the same block. so take care of that.
        //
        // The while here _is important_. We MUST read something from the back since we already
        // 'consumed' from the front.
        while let Some(back) = input.back_mut() {
            if back.id.is_none() {
                input.pop_back();
                continue;
            }

            // Otherwise there's a file we pretend to defrag by moving to the front
            let file_id = back.id.unwrap();
            assert!(back.len > 0);
            back.len -= 1;
            if back.len == 0 {
                input.pop_back();
            }

            total += position * file_id;
            position += 1;
            break;
        }
    }

    total
}

#[derive(Debug)]
struct InputBlock {
    id: Option<usize>, // None if empty space
    len: usize,
}

fn read_input(input_path: &str) -> VecDeque<InputBlock> {
    let line = read_lines(input_path).unwrap().next().unwrap().unwrap();
    let as_bytes = line.as_bytes();

    let mut result = VecDeque::new();

    for (i, &b) in as_bytes.iter().enumerate() {
        let is_file = i.is_even();
        let len = (b - b'0') as usize;
        let id = if is_file { Some(i / 2) } else { None };

        if len > 0 {
            result.push_back(InputBlock { id, len });
        }
    }

    result
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}
