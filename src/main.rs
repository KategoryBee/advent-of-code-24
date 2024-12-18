use std::{
    collections::{HashMap, HashSet},
    io,
};

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 7036, "test input failed");
    println!("Test passed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(input_path: &str) -> i32 {
    let input = read_input(input_path);

    let cost_move = 1;
    let cost_rotate = 1000;

    // We start facing east.
    let initial = (input.start, Direction::Right);
    let mut to_check = vec![initial];

    let mut costs = HashMap::new();
    costs.insert(initial, 0);

    while let Some(current) = to_check.pop() {
        // Doing a full search / mapping out of costs, rather that breadth first, since i'm too lazy
        // to implement it and i'm expecting a kick in part 2 because of the last few puzzles.
        let current_cost = *costs.get(&current).unwrap();

        let mut test = |pos, dir, cost| {
            let c = *costs.get(&(pos, dir)).unwrap_or(&i32::MAX);
            if cost < c {
                costs.insert((pos, dir), cost);
                to_check.push((pos, dir));
            }
        };

        let (current_pos, current_dir) = current;

        // Moving in current direction _may_ not be a valid move. need to check positions.
        let moved_pos = current_pos + current_dir.to_vec();
        if input.positions.contains(&moved_pos) {
            test(moved_pos, current_dir, current_cost + cost_move);
        }

        // figure out all adjacent verticies. check cost of them, and enqueue if we found cheaper
        let rotated = [
            (current_dir.rotate_cw(), current_cost + cost_rotate),
            (current_dir.rotate_ccw(), current_cost + cost_rotate),
            (current_dir.flip(), current_cost + cost_rotate * 2),
        ];

        for (dir, cost) in rotated {
            // rotated are always valid verticies
            test(current_pos, dir, cost);
        }
    }

    // at end, lowest of (end_node (all directions))
    let endings = [
        (input.end, Direction::Up),
        (input.end, Direction::Right),
        (input.end, Direction::Down),
        (input.end, Direction::Left),
    ];

    endings
        .iter()
        .map(|n| *costs.get(n).unwrap())
        .min()
        .unwrap()
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn to_vec(self) -> Vec2 {
        match self {
            Direction::Up => Vec2(0, -1),
            Direction::Right => Vec2(1, 0),
            Direction::Down => Vec2(0, 1),
            Direction::Left => Vec2(-1, 0),
        }
    }

    fn rotate_cw(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn rotate_ccw(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    fn flip(self) -> Self {
        self.rotate_cw().rotate_cw()
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            other => panic!("Unknown movement {other}"),
        }
    }
}

#[derive(Default)]
struct Input {
    positions: HashSet<Vec2>,
    start: Vec2,
    end: Vec2,
}

fn read_input(input_path: &str) -> Input {
    let mut res = Input::default();

    for (y, line) in read_lines(input_path).unwrap().enumerate() {
        let line = line.unwrap();

        for (x, c) in line.chars().enumerate() {
            let pos = Vec2(x as _, y as _);

            match c {
                '#' => (), // wall, no op
                '.' => {
                    res.positions.insert(pos);
                }
                'S' => {
                    res.positions.insert(pos);
                    res.start = pos;
                }
                'E' => {
                    res.positions.insert(pos);
                    res.end = pos;
                }
                other => panic!("expected map character {other}"),
            }
        }
    }

    res
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}
