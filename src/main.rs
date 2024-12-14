use std::io;

use itertools::Itertools;
use regex::Regex;

fn main() {
    // No test data available for part 2
    // let test_result = solve("test.txt");
    // assert_eq!(test_result, 480, "test input failed");
    // println!("Test passed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(input_path: &str) -> i64 {
    let input = read_input(input_path);

    let mut total = 0;

    for p in input {
        total += solve_puzzle(p);
    }

    total
}

// solving analytically:

// Puzzle:
// Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400

// X = 94*a + 22*b
// Y = 34*a + 67*b

// substituting in target X and Y:
// 8400 = 94*a + 22*b
// 5400 = 34*a + 67*b

// 94a = 8400 - 22b
// a = (8400 - 22b)/94
// a = (8400/94) - (22/94)b

// 5400 = 34*a + 67*b
// 5400 = 34*(8400 - 22b)/94 + 67*b
// 5400 = 34*(8400/94) - 34(22/94)b + 67*b

// 67*b - 34(22/94)b = 5400 - 34*(8400/94)
// b(67 - 34(22/94)) = 5400 - 34*(8400/94)
// b = (5400 - 34*(8400/94))/(67 - 34(22/94))
// b = 40

// So with placeholders:
// b = (p_y - a_y*(p_x/a_x))/(b_y - a_y(b_x/a_x))
// But we need integer solutions only. By the looks of the input data, there's no values for
// either linear equation that give us anything but 1 solution (aka they're never parallel). So
// the easy thing is to solve as floating point, round to nearest, and check if the solution works
// with those integers.
fn solve_puzzle(p: Puzzle) -> i64 {
    let a_x = p.a_x as f64;
    let a_y = p.a_y as f64;
    let b_x = p.b_x as f64;
    let b_y = p.b_y as f64;
    let p_x = p.prize_x as f64;
    let p_y = p.prize_y as f64;

    let b = (p_y - a_y * (p_x / a_x)) / (b_y - a_y * (b_x / a_x));
    let a = (p_x - b_x * b) / a_x;

    let a = a.round() as i64;
    let b = b.round() as i64;

    // Can't have negative button presses. We get these for some invalid solutions. So no
    // tokens spent.
    if a < 0 || b < 0 {
        return 0;
    }

    // Check found solutions against input. Due to fractions, and our rounding, these may not
    // actually be valid results. I'm hoping we have enough precision that this check will work in
    // all cases.
    let found_x = p.a_x * a + p.b_x * b;
    let found_y = p.a_y * a + p.b_y * b;

    if found_x != p.prize_x || found_y != p.prize_y {
        return 0;
    }

    // Tokens spent:
    a * 3 + b
}

struct Puzzle {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    prize_x: i64,
    prize_y: i64,
}

fn read_input(input_path: &str) -> Vec<Puzzle> {
    let mut res = Vec::new();

    let input = read_lines(input_path).unwrap().filter_map(|f| {
        let line = f.unwrap();
        if line.is_empty() {
            None
        } else {
            Some(line)
        }
    });

    let xy_pattern = Regex::new(r"X[=+](\d+), Y[=+](\d+)").unwrap();

    for (a, b, prize) in input.tuples() {
        let (_, [a_x, a_y]) = xy_pattern.captures(&a).unwrap().extract();
        let (_, [b_x, b_y]) = xy_pattern.captures(&b).unwrap().extract();
        let (_, [prize_x, prize_y]) = xy_pattern.captures(&prize).unwrap().extract();

        res.push(Puzzle {
            a_x: a_x.parse().unwrap(),
            a_y: a_y.parse().unwrap(),
            b_x: b_x.parse().unwrap(),
            b_y: b_y.parse().unwrap(),
            prize_x: prize_x.parse::<i64>().unwrap() + 10000000000000,
            prize_y: prize_y.parse::<i64>().unwrap() + 10000000000000,
        });
    }

    res
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}
