use std::io;

use regex::Regex;

fn main() {
    let test_result = solve("test.txt", Vec2(11, 7));
    assert_eq!(test_result, 12, "test input failed");
    println!("Test passed");

    let result = solve("input.txt", Vec2(101, 103));
    println!("result: {result}");
}

fn solve(input_path: &str, field_size: Vec2) -> i64 {
    let mut robots = read_input(input_path);

    let ticks = 100;

    for r in robots.iter_mut() {
        r.pos += r.vel * ticks;

        r.pos.0 %= field_size.0;
        if r.pos.0 < 0 {
            r.pos.0 += field_size.0;
        }
        r.pos.1 %= field_size.1;
        if r.pos.1 < 0 {
            r.pos.1 += field_size.1;
        }
    }

    // collect in to buckets
    let midpoint_x = field_size.0 / 2;
    let midpoint_y = field_size.1 / 2;

    let mut total_tl = 0;
    let mut total_tr = 0;
    let mut total_bl = 0;
    let mut total_br = 0;

    for r in robots {
        if r.pos.0 == midpoint_x || r.pos.1 == midpoint_y {
            // on centre line. doing it this way to make the match easier
            continue;
        }

        match (r.pos.0 < midpoint_x, r.pos.1 < midpoint_y) {
            (true, true) => total_tl += 1,
            (true, false) => total_bl += 1,
            (false, true) => total_tr += 1,
            (false, false) => total_br += 1,
        }
    }

    total_tl * total_tr * total_bl * total_br
}

#[derive(Clone, Copy, PartialEq, Eq)]
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
