use std::{collections::HashMap, io};

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 81, "test input failed");
    println!("Test passed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(input_path: &str) -> usize {
    let input = read_input(input_path);

    let starting_tiles = input.iter().filter(|p| *p.1 == 0);

    let mut total = 0;
    for (&start, _) in starting_tiles {
        total += trails_at(start, 0, &input);
    }

    total
}

fn trails_at(pos: (i8, i8), height: u8, input: &HashMap<(i8, i8), u8>) -> usize {
    if height == 9 {
        return 1;
    }

    let left = (pos.0 - 1, pos.1);
    let right = (pos.0 + 1, pos.1);
    let up = (pos.0, pos.1 - 1);
    let down = (pos.0, pos.1 + 1);

    let next_height = height + 1;

    let mut found = 0;

    if input.get(&left) == Some(&next_height) {
        found += trails_at(left, next_height, input);
    }

    if input.get(&right) == Some(&next_height) {
        found += trails_at(right, next_height, input);
    }

    if input.get(&up) == Some(&next_height) {
        found += trails_at(up, next_height, input);
    }

    if input.get(&down) == Some(&next_height) {
        found += trails_at(down, next_height, input);
    }

    found
}

fn read_input(input_path: &str) -> HashMap<(i8, i8), u8> {
    let mut res = HashMap::new();

    for (y, line) in read_lines(input_path).unwrap().enumerate() {
        let line = line.unwrap();
        for (x, c) in line.bytes().enumerate() {
            res.insert((x as _, y as _), c - b'0');
        }
    }

    res
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}
