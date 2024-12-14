use std::{collections::HashSet, io};

use regex::Regex;

fn main() {
    // let test_result = solve("test.txt", Vec2(11, 7));
    // assert_eq!(test_result, 12, "test input failed");
    // println!("Test passed");

    let result = solve("input.txt", Vec2(101, 103));
    println!("result: {result}");
}

fn solve(input_path: &str, field_size: Vec2) -> i64 {
    let mut robots = read_input(input_path);

    for tick in 1..1000000 {
        // Move robots and collect current positions.
        let mut positions = HashSet::new();
        for r in robots.iter_mut() {
            r.pos += r.vel;

            r.pos.0 %= field_size.0;
            if r.pos.0 < 0 {
                r.pos.0 += field_size.0;
            }
            r.pos.1 %= field_size.1;
            if r.pos.1 < 0 {
                r.pos.1 += field_size.1;
            }

            positions.insert(r.pos);
        }

        if looks_like_an_xmas_tree(&positions) {
            println!("Iteration {tick}");
            print_field(&positions, field_size);
            return tick;
        }
    }

    panic!("Couldn't find tree");
}

// Assuming it's a typical xmas tree, it probably has a long uninterrupted vertical bit?
fn looks_like_an_xmas_tree(positions: &HashSet<Vec2>) -> bool {
    let expected_len = 10;
    for p in positions {
        let mut trunk_len = 0;

        for y in 0..expected_len {
            if positions.contains(&(*p + Vec2(y, 0))) {
                trunk_len += 1;
            }
        }

        if trunk_len == expected_len {
            return true;
        }
    }

    false
}

fn print_field(positions: &HashSet<Vec2>, field_size: Vec2) {
    for y in 0..field_size.1 {
        for x in 0..field_size.0 {
            let c = if positions.contains(&Vec2(x, y)) {
                '#'
            } else {
                ' '
            };
            print!("{c}");
        }
        println!();
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

struct Robot {
    pos: Vec2,
    vel: Vec2,
}

fn read_input(input_path: &str) -> Vec<Robot> {
    let mut res = Vec::new();

    let pattern = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    for line in read_lines(input_path).unwrap() {
        let line = line.unwrap();

        let (_, [px, py, vx, vy]) = pattern.captures(&line).unwrap().extract();

        res.push(Robot {
            pos: Vec2(px.parse().unwrap(), py.parse().unwrap()),
            vel: Vec2(vx.parse().unwrap(), vy.parse().unwrap()),
        });
    }

    res
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}
