use std::{
    collections::{HashMap, HashSet},
    io,
};

use itertools::Itertools;

fn main() {
    let test_result = solve("test.txt", (12, 12));
    assert_eq!(test_result, 14, "test input failed");
    println!("Test passed");

    let result = solve("input.txt", (50, 50));
    println!("result: {result}");
}

fn solve(input_path: &str, map_size: (i64, i64)) -> usize {
    let antennas = parse_antennas(input_path);
    let nodes = collect_nodes(&antennas);

    for y in 0..map_size.1 {
        for x in 0..map_size.0 {
            if nodes.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!()
    }

    nodes.iter().filter(|&&p| within_map(p, map_size)).count()
}

fn parse_antennas(input_path: &str) -> HashMap<char, Vec<(i64, i64)>> {
    let mut result: HashMap<char, Vec<(i64, i64)>> = HashMap::new();

    for (y, input) in read_lines(input_path).unwrap().enumerate() {
        let input = input.unwrap();
        for (x, c) in input.chars().enumerate() {
            if c == '.' {
                continue;
            }

            result.entry(c).or_default().push((x as _, y as _));
        }
    }

    result
}

fn collect_nodes(antennas: &HashMap<char, Vec<(i64, i64)>>) -> HashSet<(i64, i64)> {
    let mut result = HashSet::new();

    for a in antennas.values() {
        for (l, r) in a.iter().tuple_combinations() {
            let dist = (l.0 - r.0, l.1 - r.1);

            let node1 = (l.0 + dist.0, l.1 + dist.1);
            let node2 = (r.0 - dist.0, r.1 - dist.1);

            result.insert(node1);
            result.insert(node2);
        }
    }

    result
}

fn within_map(p: (i64, i64), map_size: (i64, i64)) -> bool {
    p.0 >= 0 && p.1 >= 0 && p.0 < map_size.0 && p.1 < map_size.1
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}
