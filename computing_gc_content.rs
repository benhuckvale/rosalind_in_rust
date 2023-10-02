use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut label_and_percentages: Vec<(String, f64)> = Vec::new();

    let mut current_label = String::new();
    let mut current_sequence = String::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with('>') {
            if !current_label.is_empty() {
                label_and_percentages.push((
                    current_label.clone(),
                    calculate_gc_percentage(&current_sequence)
                ));
            }
            current_sequence.clear();
            current_label = line[1..].trim().to_string();

        } else {
            current_sequence.push_str(&line.trim());
        }
    }
    if !current_label.is_empty() {
        label_and_percentages.push((
            current_label.clone(),
            calculate_gc_percentage(&current_sequence)
        ));
    }

    let mut sorted: Vec<_> = label_and_percentages.iter().collect();
    sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    for (label, percentage) in sorted {
        println!("{}\n{:.3}", label, percentage);
    }

    Ok(())
}

fn calculate_gc_percentage(sequence: &str) -> f64 {
    let char_set: &[char] = &['G', 'C'];
    let gc_count = count_characters(&sequence, char_set) ;
    let gc_percentage = (gc_count as f64)/(sequence.len() as f64)*100.0;
    gc_percentage
}

fn count_characters(sequence: &str, char_set: &[char]) -> usize {
    sequence.chars().filter(|&c| char_set.contains(&c)).count()
}
