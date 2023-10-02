extern crate itertools;
use itertools::iterate;

/// Calculate the n-th term of a recurrence relation.
///
/// Given values `n` and `k`, this function calculates the `n`-th term `F(n)` of the recurrence relation:
///
/// `F(n) = k * F(n-1) + F(n-2)`
///
/// with initial conditions:
///
/// - `F(1) = 1`
/// - `F(2) = 1`
///
/// # Arguments
///
/// - `n`: The term to calculate.
/// - `k`: The coefficient `k` in the recurrence relation.
///
/// # Returns
///
/// The calculated `n`-th term `F(n)`.
///
/// # Examples
///
/// ```
/// let n = 5;
/// let k = 3;
/// let result = rabbits_and_recurrence_relations::calculate_term(n, k);
/// assert_eq!(result, 19);
/// ```
pub fn calculate_term(n: usize, k: u64) -> u64 {
    let result = iterate((1, 1), |&(a, b)| (b, k * a + b))
        .nth(n-2_usize)
        .unwrap()
        .1;
    return result;
}
