use std::env;

// Implementation of itertools::iterate from the crate itertools
struct Iterate<T, F> {
    current: T,
    func: F,
}

impl<T, F> Iterate<T, F>
where
    F: Fn(&T) -> T,
    T: Clone,
{
    fn new(initial: T, func: F) -> Self {
        Iterate {
            current: initial,
            func,
        }
    }
}

impl<T, F> Iterator for Iterate<T, F>
where
    F: Fn(&T) -> T,
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let next_value = (self.func)(&self.current);
        self.current = next_value.clone();
        Some(next_value)
    }
}

fn iterate<T, F>(initial: T, func: F) -> Iterate<T, F>
where
    F: Fn(&T) -> T,
    T: Clone,
{
    Iterate::new(initial, func)
}


// Function to calculate term n for recurrence relation with coefficient k
pub fn calculate_term(n: usize, k: u64) -> u64 {
    let result = iterate((1, 1), |&(a, b)| (b, k * a + b))
        .nth(n-3_usize)
        .unwrap()
        .1;
    return result;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: usize = args[1].parse().expect("Failed to parse 'n'");
    let k: u64 = args[2].parse().expect("Failed to parse 'k'");
    let result: u64 = calculate_term(n, k);
    println!("{}", result);
}
