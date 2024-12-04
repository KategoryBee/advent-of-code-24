use std::io;

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 18, "test input failed");
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

    let offsets = &[
        (1, -1),
        (1, 0),
        (1, 1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
    ];

    for x in 0..width {
        for y in 0..height {
            if val_at(x, y) != b'X' {
                continue;
            }

            for &(off_x, off_y) in offsets.iter() {
                let m = val_at(x + off_x * 1, y + off_y * 1);
                let a = val_at(x + off_x * 2, y + off_y * 2);
                let s = val_at(x + off_x * 3, y + off_y * 3);

                if m == b'M' && a == b'A' && s == b'S' {
                    total += 1;
                }
            }
        }
    }

    total
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}
