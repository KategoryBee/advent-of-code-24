use std::{collections::HashSet, io};

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 6, "test input failed");
    println!("Test passed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(input_path: &str) -> i64 {
    let mut obstacles = HashSet::new();
    let mut position = (0, 0);
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

    let mut total = 0;
    for y in 0..height {
        for x in 0..width {
            let mut with_extra_blocker = obstacles.clone();
            with_extra_blocker.insert((x, y));

            if contains_cycle(position, &with_extra_blocker, width, height) {
                total += 1;
            }
        }
    }

    total
}

fn contains_cycle(
    start: (i32, i32),
    obstacles: &HashSet<(i32, i32)>,
    field_width: i32,
    field_height: i32,
) -> bool {
    let mut position = start;
    let mut dir = '^';

    let mut visited = HashSet::new();

    while position.0 >= 0
        && position.1 >= 0
        && position.0 < field_width
        && position.1 < field_height
    {
        if visited.contains(&(position, dir)) {
            return true;
        }

        visited.insert((position, dir));

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

    false
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}
