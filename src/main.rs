use std::io;

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 11, "test input failed");
    println!("Test passed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(input_path: &str) -> i64 {
    let mut input = Input::read(input_path);

    input.left.sort();
    input.right.sort();

    let mut total = 0;

    for (l, r) in input.left.into_iter().zip(input.right) {
        total += l.abs_diff(r) as i64;
    }

    total
}

struct Input {
    left: Vec<i64>,
    right: Vec<i64>,
}

impl Input {
    fn read(filename: &str) -> Input {
        let mut res = Input {
            left: vec![],
            right: vec![],
        };

        for line in read_lines(filename).unwrap() {
            let line = line.unwrap();

            let parts: Vec<&str> = line.split_ascii_whitespace().collect();

            res.left.push(parts[0].parse().unwrap());
            res.right.push(parts[1].parse().unwrap());
        }

        res
    }
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}
