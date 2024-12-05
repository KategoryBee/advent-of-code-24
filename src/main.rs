use std::{collections::HashSet, io};

use itertools::Itertools;

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 123, "test input failed");
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

    for mut update in updates {
        let mut update_changed = false;

        // If we swap elements, all combinations need to be checked again.
        let mut needs_retry = true;
        while needs_retry {
            needs_retry = false;

            for (i, j) in (0..update.len()).tuple_combinations() {
                let a = update[i];
                let b = update[j];

                if orderings.contains(&(b, a)) {
                    update_changed = true;
                    needs_retry = true;

                    update.swap(i, j);
                }
            }
        }

        if update_changed {
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
