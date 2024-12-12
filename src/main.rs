use std::{
    collections::{HashMap, HashSet},
    io,
};

type Pos = (i16, i16);

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 1930, "test input failed");
    println!("Test passed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(input_path: &str) -> usize {
    let input = read_input(input_path);

    let regions = balkanize(input);

    let mut total = 0;
    for r in regions {
        let perim = perimiter_of(&r);
        total += perim * r.len();
    }

    total
}

fn perimiter_of(region: &HashSet<Pos>) -> usize {
    let mut total = 0;

    for &pos in region {
        let neighbours = [
            (pos.0 - 1, pos.1),
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 - 1),
            (pos.0, pos.1 + 1),
        ];

        for adjacent in neighbours {
            if !region.contains(&adjacent) {
                total += 1;
            }
        }
    }

    total
}

// Turn the input in to a list of regions
fn balkanize(mut input: HashMap<Pos, u8>) -> Vec<HashSet<Pos>> {
    let mut result = Vec::new();

    while let Some((&start, &plant)) = input.iter().next() {
        let mut region = HashSet::new();
        extract_region(start, plant, &mut input, &mut region);
        result.push(region);
    }

    result
}

fn extract_region(start: Pos, plant: u8, input: &mut HashMap<Pos, u8>, output: &mut HashSet<Pos>) {
    let mut to_check = vec![start];

    while let Some(pos) = to_check.pop() {
        if input.get(&pos) != Some(&plant) {
            continue;
        }

        input.remove(&pos);
        output.insert(pos);

        let neighbours = [
            (pos.0 - 1, pos.1),
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 - 1),
            (pos.0, pos.1 + 1),
        ];

        for adjacent in neighbours {
            to_check.push(adjacent);
        }
    }
}

fn read_input(input_path: &str) -> HashMap<Pos, u8> {
    let mut res = HashMap::new();
    for (y, line) in read_lines(input_path).unwrap().enumerate() {
        for (x, c) in line.unwrap().bytes().enumerate() {
            res.insert((x as _, y as _), c);
        }
    }
    res
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}
