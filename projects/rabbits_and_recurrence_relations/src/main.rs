extern crate rabbits_and_recurrence_relations;
use rabbits_and_recurrence_relations::calculate_term;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: usize = args[1].parse().expect("Failed to parse 'n'");
    let k: u64 = args[2].parse().expect("Failed to parse 'k'");
    let result: u64 = calculate_term(n, k);
    println!("{}", result);
}
