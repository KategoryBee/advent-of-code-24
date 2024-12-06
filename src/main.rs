use std::{collections::HashSet, io};

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 41, "test input failed");
    println!("Test passed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(input_path: &str) -> i64 {
    let mut obstacles = HashSet::new();
    let mut visited = HashSet::new();
    let mut position = (0, 0);
    let mut dir = '^';
    let mut width = 0;
    let mut height = 0;

    for (y, line) in read_lines(input_path).unwrap().enumerate() {
        let line = line.unwrap();

        height += 1;
        width = line.as_bytes().len() as i32;

        for (x, &c) in line.as_bytes().iter().enumerate() {
            let pos = (x as i32, y as i32);
            if c == b'#' {
                obstacles.insert(pos);
            }

            if c == b'^' {
                position = pos;
            }
        }
    }

    while position.0 >= 0 && position.1 >= 0 && position.0 < width && position.1 < height {
        visited.insert(position);

        let movement = match dir {
            '^' => (0, -1),
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            _ => panic!("unknown direction {dir}"),
        };

        let next = (position.0 + movement.0, position.1 + movement.1);

        if obstacles.contains(&next) {
            dir = match dir {
                '^' => '>',
                '>' => 'v',
                'v' => '<',
                '<' => '^',
                _ => panic!("unknown direction {dir}"),
            }
        } else {
            position = next;
        }
    }

    for y in 0..height + 1 {
        for x in 0..width + 1 {
            let p = (x, y);
            if obstacles.contains(&p) {
                print!("#")
            } else if visited.contains(&p) {
                print!("X")
            } else {
                print!(".")
            }
        }
        println!()
    }

    visited.len() as i64
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}
