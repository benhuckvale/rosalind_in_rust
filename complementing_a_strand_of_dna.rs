use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_string = match fs::read_to_string(&args[1]) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            std::process::exit(1); // Exit with an error code
        }
    };
    let result: String = input_string.trim().chars().rev()
        .map(|x| match x {
            'A' => 'T',
            'C' => 'G',
            'G' => 'C',
            'T' => 'A',
            _ => x
        }).collect();
    println!("{}", result);
}
