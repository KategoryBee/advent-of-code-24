use std::{collections::HashMap, io};

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 31, "test input failed");
    println!("Test passed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(input_path: &str) -> i64 {
    let input = Input::read(input_path);

    let mut total = 0;
    for (i, count) in input.left {
        let other_count = *input.right.get(&i).unwrap_or(&0);
        total += i * count * other_count;
    }

    total
}

struct Input {
    // lists of integers and the number of times they've appeared
    left: HashMap<i64, i64>,
    right: HashMap<i64, i64>,
}

impl Input {
    fn read(filename: &str) -> Input {
        let mut res = Input {
            left: HashMap::new(),
            right: HashMap::new(),
        };

        for line in read_lines(filename).unwrap() {
            let line = line.unwrap();

            let parts: Vec<&str> = line.split_ascii_whitespace().collect();

            let l: i64 = parts[0].parse().unwrap();
            let r: i64 = parts[1].parse().unwrap();

            Self::add_to_list(l, &mut res.left);
            Self::add_to_list(r, &mut res.right);
        }

        res
    }

    fn add_to_list(v: i64, list: &mut HashMap<i64, i64>) {
        *list.entry(v).or_default() += 1;
    }
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}
