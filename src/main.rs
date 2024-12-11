use std::io;

use num::Integer;

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 2858, "test input failed");
    println!("Test passed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(input_path: &str) -> usize {
    let mut input = read_input(input_path);

    let mut moved_files = Vec::new();

    for mut file in input.files.into_iter().rev() {
        // Since we only move backward through files, and only touch them once, we don't need to
        // insert spaces as we 'free' them up, since no file could possibly move in to that space.

        let move_to = input
            .spaces
            .iter_mut()
            .find(|s| s.len >= file.pos.len && s.start < file.pos.start);

        if let Some(dest) = move_to {
            file.pos.start = dest.start;

            // Empty out the space. We _could_ deallocate the found space here and that might speed
            // things up, but I'm too lazy to import a list datastructure and test if it ends up
            // faster
            dest.start += file.pos.len;
            dest.len -= file.pos.len;
            moved_files.push(file);
        } else {
            // No space available. Stay in place
            moved_files.push(file);
        }
    }

    let mut total = 0;

    for f in moved_files {
        for pos in f.pos.to_range() {
            total += pos * f.id;
        }
    }

    total
}

#[derive(Debug)]
struct Input {
    files: Vec<File>,
    spaces: Vec<Span>,
}

#[derive(Debug)]
struct File {
    id: usize,
    pos: Span,
}

#[derive(Debug)]
struct Span {
    start: usize,
    len: usize,
}

impl Span {
    fn to_range(&self) -> std::ops::Range<usize> {
        self.start..(self.start + self.len)
    }
}

fn read_input(input_path: &str) -> Input {
    let line = read_lines(input_path).unwrap().next().unwrap().unwrap();
    let as_bytes = line.as_bytes();

    let mut files = Vec::new();
    let mut spaces = Vec::new();
    let mut offset = 0;

    for (i, &b) in as_bytes.iter().enumerate() {
        let is_file = i.is_even();
        let len = (b - b'0') as usize;

        let pos = Span { start: offset, len };
        offset += len;

        if is_file {
            assert_ne!(len, 0);
            files.push(File { id: i / 2, pos });
        } else if len > 0 {
            spaces.push(pos);
        };
    }

    Input { files, spaces }
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}
