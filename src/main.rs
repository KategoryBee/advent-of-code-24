use std::io;

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 3749, "test input failed");
    println!("Test passed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(input_path: &str) -> i64 {
    let mut total = 0;

    for line in read_lines(input_path).unwrap() {
        let line = line.unwrap();

        let mut split = line.split(':');
        let test_value: i64 = split.next().unwrap().parse().unwrap();
        let nums: Vec<i64> = split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        if report_ok(test_value, &nums) {
            total += test_value
        }
    }

    total
}

fn report_ok(test_value: i64, nums: &[i64]) -> bool {
    // There's probably some fancy rust crate to generate all combinations of something like
    // [Op; OpCount] as an iterator, but i can't find it. we only have 3 choices, so i'll just
    // use bitbashing.
    let op_count = nums.len() - 1;
    let op_max = 1 << op_count;

    for i in 0..op_max {
        let mut o = i;
        let mut n = nums.iter();
        let mut running_total = *n.next().unwrap();

        for &next_num in n {
            match o & 1 {
                0 => running_total += next_num,
                1 => running_total *= next_num,
                _ => panic!(),
            }
            o >>= 1;

            if running_total == test_value {
                return true;
            }
        }
    }

    false
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}
