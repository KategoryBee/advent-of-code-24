use std::io;

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 9, "test input failed");
    println!("Test passed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(input_path: &str) -> i64 {
    let mut total = 0;

    let field: Vec<Vec<u8>> = read_lines(input_path)
        .unwrap()
        .map(|e| e.unwrap().into_bytes())
        .collect();

    let val_at = |x: i64, y: i64| {
        if x < 0 || y < 0 {
            return 0u8;
        }

        if let Some(row) = field.get(y as usize) {
            *row.get(x as usize).unwrap_or(&0u8)
        } else {
            0u8
        }
    };

    let width = field[0].len() as i64;
    let height = field.len() as i64;

    for x in 0..width {
        for y in 0..height {
            if val_at(x, y) != b'A' {
                continue;
            }

            let lu = val_at(x - 1, y - 1);
            let ld = val_at(x - 1, y + 1);
            let ru = val_at(x + 1, y - 1);
            let rd = val_at(x + 1, y + 1);

            let has_diag_1 = (lu == b'M' && rd == b'S') || (lu == b'S' && rd == b'M');
            let has_diag_2 = (ru == b'M' && ld == b'S') || (ru == b'S' && ld == b'M');

            if has_diag_1 && has_diag_2 {
                total += 1;
            }
        }
    }

    total
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}
