use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines();
    let first_line = lines.next();
    let second_line = lines.next();
    match (first_line, second_line) {
        (Some(Ok(first)), Some(Ok(second))) => {
            let identical_count = first
                .chars()
                .zip(second.chars())
                .filter(|(a, b)| a == b)
                .count();
            println!("{}", first.len() - identical_count);
        },
        _ => {
            eprintln!("Invalid input: Expected exactly two valid lines in file");
        }
    }

    Ok(())
}
