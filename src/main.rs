use std::{
    collections::{HashMap, HashSet},
    io,
};

use itertools::Itertools;

type Pos = (i16, i16);

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 368, "test input failed");
    println!("Test passed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(input_path: &str) -> usize {
    let input = read_input(input_path);

    let regions = balkanize(input);

    let mut total = 0;
    for (plant, region) in regions {
        let fences = sides_of(&region);
        total += fences * region.len();

        println!(
            "Region {} has area {}, and {} fences",
            plant as char,
            region.len(),
            fences
        );
    }

    total
}

fn sides_of(region: &HashSet<Pos>) -> usize {
    // Since the fence is on a boundary, mark it on the lower of the 2 cells (left for x, up for y)
    let mut sides_left = Vec::new();
    let mut sides_right = Vec::new();
    let mut sides_up = Vec::new();
    let mut sides_down = Vec::new();

    // weh this doesn't work, gaps in the horizontal/vertical line need to add 1 to the length!
    for &pos in region {
        let left = (pos.0 - 1, pos.1);
        let right = (pos.0 + 1, pos.1);
        let up = (pos.0, pos.1 - 1);
        let down = (pos.0, pos.1 + 1);

        if !region.contains(&left) {
            sides_left.push(left);
        }

        if !region.contains(&right) {
            sides_right.push(right);
        }

        if !region.contains(&up) {
            sides_up.push(up);
        }

        if !region.contains(&down) {
            sides_down.push(down);
        }
    }

    // Sort by y, then x. Then we look for any gaps in the fencing. The number of total fencing
    // segments is then 1 + the number of gaps.
    //
    // We don't need to de-duplicate in a seperate pass, since a fence can never be added twice due
    // to the way the algo works.
    sides_left.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    let left_count = sides_left
        .iter()
        .tuple_windows()
        .filter(|(a, b)| a.0 != b.0 || (b.1 - a.1) > 1)
        .count()
        + 1;

    sides_right.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    let right_count = sides_right
        .iter()
        .tuple_windows()
        .filter(|(a, b)| a.0 != b.0 || (b.1 - a.1) > 1)
        .count()
        + 1;

    // Same as the horizontal fences, but in Y first order instead
    sides_up.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
    let up_count = sides_up
        .iter()
        .tuple_windows()
        .filter(|(a, b)| a.1 != b.1 || (b.0 - a.0) > 1)
        .count()
        + 1;

    sides_down.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
    let down_count = sides_down
        .iter()
        .tuple_windows()
        .filter(|(a, b)| a.1 != b.1 || (b.0 - a.0) > 1)
        .count()
        + 1;

    // The above is kind of a horror. Originally It was only vert/horizontal, but the mobius example
    // breaks that, so i needed to split out left/right boundaries. It's a cool test case, but the
    // easiest thing to do then was duplicate existing code.
    left_count + right_count + up_count + down_count
}

// Turn the input in to a list of regions
fn balkanize(mut input: HashMap<Pos, u8>) -> Vec<(u8, HashSet<Pos>)> {
    let mut result = Vec::new();

    while let Some((&start, &plant)) = input.iter().next() {
        let mut region = HashSet::new();
        extract_region(start, plant, &mut input, &mut region);
        result.push((plant, region));
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
