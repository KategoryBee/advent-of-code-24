use itertools::Itertools;
use std::{collections::HashSet, io};

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 143, "test input failed");
    println!("Test passed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(input_path: &str) -> i64 {
    let mut total = 0;

    let mut orderings = HashSet::<(i64, i64)>::new();
    let mut updates = vec![];

    for line in read_lines(input_path).unwrap() {
        let line = line.unwrap();

        if line.contains('|') {
            let mut pages = line.split('|');

            let page: i64 = pages.next().unwrap().parse().unwrap();
            let is_before: i64 = pages.next().unwrap().parse().unwrap();

            orderings.insert((page, is_before));
        }

        if line.contains(',') {
            let update: Vec<i64> = line.split(',').map(|e| e.parse().unwrap()).collect();
            updates.push(update);
        }
    }

    for update in updates {
        let mut in_correct_order = true;
        for pages in update.iter().combinations(2) {
            // combinations always yields the same order for individual elements as the source,
            // so 'a' is printed before 'b'. We only need to check our orderings to make sure
            // there's no restriction on b being printed before a.
            let a = *pages[0];
            let b = *pages[1];

            if orderings.contains(&(b, a)) {
                in_correct_order = false;
            }
        }

        if in_correct_order {
            let middle = update[update.len() / 2];
            total += middle;
        }
    }

    total
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}
