fn main() {
    let input_string: &'static str = "GATGGAACTTGACTACGTAAATT";
    let result = input_string.replace("T", "U");
    println!("{}", result);
}
