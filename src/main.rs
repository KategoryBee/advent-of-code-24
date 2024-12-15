use std::{collections::HashSet, io};

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 9021, "test input failed");
    println!("Test passed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(input_path: &str) -> i64 {
    let mut input = read_input(input_path);

    for m in input.instructions {
        #[cfg(debug_assertions)]
        print_field(input.robot, &input.boxes, &input.walls);
        // Need to recursively move everything at pos X, in dir. includes checcking to left of pos
        // for a box. Then bail out of _all_ moves if any would hit a wall.
        let robot_dest = input.robot + m.to_vec();
        let mut new_boxes = input.boxes.clone();

        if try_move(robot_dest, m, &input.walls, &mut new_boxes) {
            input.robot = robot_dest;
            input.boxes = new_boxes;
        }
    }

    print_field(input.robot, &input.boxes, &input.walls);
    gps_sum(&input.boxes)
}

fn print_field(robot: Vec2, boxes: &HashSet<Vec2>, walls: &HashSet<Vec2>) {
    for y in 0..50 {
        for x in 0..100 {
            let pos = Vec2(x, y);
            let left = Vec2(x - 1, y);

            let c = if pos == robot {
                '@'
            } else if walls.contains(&pos) {
                '#'
            } else if boxes.contains(&pos) {
                '['
            } else if boxes.contains(&left) {
                // current pos is to the right of a box
                ']'
            } else {
                '.'
            };
            print!("{c}");
        }
        println!();
    }
}

// `position` is the space where something is trying to move in to.
fn try_move(position: Vec2, m: Move, walls: &HashSet<Vec2>, boxes: &mut HashSet<Vec2>) -> bool {
    if walls.contains(&position) {
        return false;
    }

    if let Some(box_left) = boxes.take(&position) {
        // Hit the left side of a box
        let box_right = box_left + Vec2(1, 0);

        let left_ok = try_move(box_left + m.to_vec(), m, walls, boxes);
        let right_ok = try_move(box_right + m.to_vec(), m, walls, boxes);

        if left_ok && right_ok {
            boxes.insert(box_left + m.to_vec());
        } else {
            boxes.insert(box_left);
            return false;
        }
    }

    if let Some(box_left) = boxes.take(&(position + Vec2(-1, 0))) {
        // Hit the right side of a box
        let box_right = box_left + Vec2(1, 0);

        let left_ok = try_move(box_left + m.to_vec(), m, walls, boxes);
        let right_ok = try_move(box_right + m.to_vec(), m, walls, boxes);

        if left_ok && right_ok {
            boxes.insert(box_left + m.to_vec());
        } else {
            boxes.insert(box_left);
            return false;
        }
    }

    // empty space, can move in to it ok.
    true
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

#[derive(Clone, Copy)]
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
                let pos_l = Vec2((x * 2) as _, y);
                let pos_r = Vec2((x * 2 + 1) as _, y);

                match c {
                    '#' => {
                        res.walls.insert(pos_l);
                        res.walls.insert(pos_r);
                    }
                    'O' => {
                        // We track the 'left' side of the box as its origin, and the rest of the
                        // code assumes it extends to the right.
                        res.boxes.insert(pos_l);
                    }
                    '@' => {
                        res.robot = pos_l;
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
