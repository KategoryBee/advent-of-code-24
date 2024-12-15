use std::{collections::HashSet, io};

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 10092, "test input failed");
    println!("Test passed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(input_path: &str) -> i64 {
    let mut input = read_input(input_path);

    for m in input.instructions {
        let dir = m.to_vec();

        // Try to move one dir. Scan in that direction, from the robot, until we hit either an Empty
        // space, or a wall, or a box. If it's a box, continue scanning.
        //
        // If it's a wall, no-op.
        // Otherwise move robot one cell in wanted direction.
        // If there's a box there, move that box to the empty space we located to simulate pushing
        // the entire stock
        let scan = find_space(input.robot, dir, &input.boxes, &input.walls);
        match scan {
            None => (), // found a wall. no op
            Some(empty_space) => {
                input.robot += dir;
                if input.boxes.take(&input.robot).is_some() {
                    input.boxes.insert(empty_space);
                }
            }
        }
    }

    gps_sum(&input.boxes)
}

fn find_space(
    start: Vec2,
    dir: Vec2,
    boxes: &HashSet<Vec2>,
    walls: &HashSet<Vec2>,
) -> Option<Vec2> {
    let mut pos = start;
    for _ in 0..1000 {
        pos += dir;
        if boxes.contains(&pos) {
            // Continue scanning
            continue;
        }
        if walls.contains(&pos) {
            return None;
        }

        // Otherwise, empty space
        return Some(pos);
    }

    panic!("Could not find empty space or wall after too many iterations");
}

fn gps_sum(boxes: &HashSet<Vec2>) -> i64 {
    boxes.iter().map(|b| (100 * b.1 + b.0) as i64).sum()
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2(i32, i32);

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        *self = *self + rhs;
    }
}

impl std::ops::Mul<i32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: i32) -> Self::Output {
        Vec2(self.0 * rhs, self.1 * rhs)
    }
}

enum Move {
    Up,
    Right,
    Down,
    Left,
}

impl Move {
    fn to_vec(&self) -> Vec2 {
        match self {
            Move::Up => Vec2(0, -1),
            Move::Right => Vec2(1, 0),
            Move::Down => Vec2(0, 1),
            Move::Left => Vec2(-1, 0),
        }
    }
}

impl From<char> for Move {
    fn from(value: char) -> Self {
        match value {
            '^' => Move::Up,
            '>' => Move::Right,
            'v' => Move::Down,
            '<' => Move::Left,
            other => panic!("Unknown movement {other}"),
        }
    }
}

#[derive(Default)]
struct Input {
    robot: Vec2,
    walls: HashSet<Vec2>,
    boxes: HashSet<Vec2>,
    instructions: Vec<Move>,
}

fn read_input(input_path: &str) -> Input {
    let mut res = Input::default();

    let mut y = 0;

    for line in read_lines(input_path).unwrap() {
        let line = line.unwrap();

        if line.starts_with('#') {
            for (x, c) in line.chars().enumerate() {
                let pos = Vec2(x as _, y);
                match c {
                    '#' => {
                        res.walls.insert(pos);
                    }
                    'O' => {
                        res.boxes.insert(pos);
                    }
                    '@' => {
                        res.robot = pos;
                    }
                    '.' => (), // empty space
                    other => panic!("expected map character {other}"),
                }
            }

            y += 1;
        } else if line.is_empty() {
            // No-op
        } else {
            // list of instructions
            for c in line.chars() {
                res.instructions.push(c.into());
            }
        }
    }

    res
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}
