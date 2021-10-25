fn main() {
    let input_string: &'static str = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
    let alphabet = "AGCT".chars();

    let mut counts = alphabet.map(|c| { input_string.matches(c).count() });
    if let Some(count) = counts.next() {
        print!("{}", count);
        for count in counts {
            print!(" {}", count);
        }
    }
    println!();
}
