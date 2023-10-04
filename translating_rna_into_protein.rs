use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_sequence = match fs::read_to_string(&args[1]) {
        Ok(contents) => contents.trim().to_string(),
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            std::process::exit(1);
        }
    };

    let mut codon_table = HashMap::new();
    let codon_table_data = "
        UUU F      CUU L      AUU I      GUU V
        UUC F      CUC L      AUC I      GUC V
        UUA L      CUA L      AUA I      GUA V
        UUG L      CUG L      AUG M      GUG V
        UCU S      CCU P      ACU T      GCU A
        UCC S      CCC P      ACC T      GCC A
        UCA S      CCA P      ACA T      GCA A
        UCG S      CCG P      ACG T      GCG A
        UAU Y      CAU H      AAU N      GAU D
        UAC Y      CAC H      AAC N      GAC D
        UAA Stop   CAA Q      AAA K      GAA E
        UAG Stop   CAG Q      AAG K      GAG E
        UGU C      CGU R      AGU S      GGU G
        UGC C      CGC R      AGC S      GGC G
        UGA Stop   CGA R      AGA R      GGA G
        UGG W      CGG R      AGG R      GGG G
    ";
    codon_table_data
        .lines()
        .flat_map(|line| line.split_whitespace())
        .collect::<Vec<_>>()
        .chunks(2)
        .for_each(|parts| {
            let codon: String = parts[0].to_string();
            let acid: String = parts[1].to_string();
            codon_table.insert(codon, acid);
        });

    let unknown: String = String::from("?");
    let amino_acids: String = input_sequence
        .chars()
        .collect::<Vec<char>>()
        .chunks(3)
        .map(|chunk| {
            let codon: String = chunk.iter().collect();
            match codon_table.get(&codon).unwrap_or(&unknown).as_str() {
                "Stop" => "\n".to_string(),
                other => other.to_string(),
            }
        })
        .collect();

    print!("{}", amino_acids);
}
