use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Populations {
    counts: HashMap<String, u32>,
}

impl Populations {
    fn new(population: HashMap<String, u32>) -> Self {
        Populations { counts: population }
    }

    fn popped_with_probability(&self, genotype: &str) -> (Populations, f32) {
        match self.counts.get(genotype) {
            Some(&count) if count > 0 => {
                let total_counts: u32 = self.counts.values().sum();
                let probability = count as f32 / total_counts as f32;

                let mut new_counts = self.counts.clone();
                new_counts.insert(genotype.to_string(), count - 1);
                (
                    Populations { counts: new_counts },
                    probability,
                )
            }
            _ => {
                // If the genotype is not found or the count is already 0, return the original population and probability 0
                (self.clone(), 0.0)
            }
        }
    }
}

fn generate_pairs<T: Clone>(list: &[T]) -> impl Iterator<Item = (T, T)> + '_ {
    list.iter()
        .cloned()
        .flat_map(move |a| list.iter().cloned().map(move |b| (a.clone(), b)))
}

fn prepare_permutation_set(input: &[&str]) -> HashSet<String> {
    let mut permutations = HashSet::new();

    for &string in input {
        let chars: Vec<char> = string.chars().collect();
        let reversed: String = chars.iter().rev().collect();
        permutations.insert(string.to_string());
        permutations.insert(reversed);
    }

    permutations
}

fn calculate_fraction(pair: (&str, &str), target_set: &HashSet<String>) -> f32 {
    let pair_0_chars: Vec<char> = pair.0.chars().collect();
    let pair_1_chars: Vec<char> = pair.1.chars().collect();

    let combinations = [
        (0, 0),
        (0, 1),
        (1, 0),
        (1, 1),
    ];

    let count = combinations.iter().filter(|&&(index1, index2)| {
        target_set.contains(&(pair_0_chars[index1].to_string() + &pair_1_chars[index2].to_string()))
    }).count();

    count as f32 / 4.0
}

fn main() {
    let mut populations_map = HashMap::new();
    populations_map.insert("YY".to_string(), 2);
    populations_map.insert("Yy".to_string(), 2);
    populations_map.insert("yy".to_string(), 2);

    // Create a Populations instance from populations_map
    let populations = Populations::new(populations_map.clone());

    // We are just interested in organisms with the dominant phenotype
    let target_set = prepare_permutation_set(&["YY", "Yy"]);

    // Calculate the probability
    let keys: Vec<&String> = populations_map.keys().collect();
    let probabilities: Vec<(&String, &String, f32, f32, f32, f32)> = generate_pairs(&keys)
        .map(|(a, b)| {
            let (populations2, prob1) = populations.popped_with_probability(a);
            let (_, prob2) = populations2.popped_with_probability(b);
            let fraction = calculate_fraction((a, b), &target_set);
            (a, b, prob1, prob2, fraction, prob1 * prob2 * fraction)
        })
        .collect();
    println!("Potential pairing, p(1st), p(2nd|1st), p(Descendent in target set|Pairing), Total probability for this pairing");
    for (a, b, p1, p2, f, total) in probabilities.iter() {
        println!("          {} x {}, {:6.3}, {:10.3}, {:35.3}, {:34.5}", a, b, p1, p2, f, total);
    }
    let total_probability: f32 = probabilities.iter().map(|t|t.5).sum::<f32>();
    println!("Total probability: {:.5}", total_probability);
}
