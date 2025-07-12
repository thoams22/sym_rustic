use std::collections::HashMap;

use crate::ast::{numeral, Expr, Expression};

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a == 0 {
        return b;
    }

    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

pub fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

const PRIMES_25: [u64; 25] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
];
const PRIMES_FACTORS_UPPER_LIMIT: u64 = 1000000;
/// Returns an `Option` of a `HashMap` prime factors and their counts.
/// Up to 1000000
///
/// First term is the prime factor and the second is the count.
///
/// # Examples
/// ```
/// use std::collections::HashMap;
/// use sym_rustic::utils::prime_factors;
///
/// let factors = prime_factors(12);
/// assert_eq!(factors, Some(HashMap::from([(2, 2), (3, 1)])));
/// ```
pub fn prime_factors(mut n: u64) -> Option<HashMap<u64, u64>> {
    if n == 0 || n == 1 {
        return None;
    }

    let mut factors = HashMap::new();
    let mut i = 0;

    while i < PRIMES_25.len() && n > 1 {
        if n % PRIMES_25[i] == 0 {
            factors.insert(PRIMES_25[i], factors.get(&PRIMES_25[i]).unwrap_or(&0) + 1);
            n /= PRIMES_25[i];
        } else {
            i += 1;
        }
    }

    if n > 1 {
        let mut j = 97;
        while j * j <= n && j < PRIMES_FACTORS_UPPER_LIMIT {
            while n % j == 0 {
                factors.insert(j, factors.get(&j).unwrap_or(&0) + 1);
                n /= j;
            }
            j += 1;
        }
    }

    Some(factors)
}

pub fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

fn multinomial_coefficient(n: u64, k: &[u64]) -> u64 {
    factorial(n) / k.iter().map(|k| factorial(*k)).product::<u64>()
}

/// Expand a multinomial expression
pub fn multinomial_expansion(terms: &[Expression], n: u64) -> Expression {
    let mut result: Vec<Expression> = vec![];
    let m = terms.len();
    let exponent_permutations: Vec<Vec<u64>> = find_permutations_with_sum(m, n);
    let mut already_calc_coef: Vec<(Vec<u64>, u64)> = Vec::new();

    for exponent_permutation in exponent_permutations {
        let mut mult: Vec<Expression> = Vec::new();

        // Because the coefficient are the n_th layer of a Pascal's m-simplex
        // These are repetitive so we store them to not recalculate them
        let mut sorted_exponent_permutations = exponent_permutation.clone();

        // Sort the permutation because in permutation [3, 1, 0, 0] != [1, 3, 0, 0] but the associated coefficient is the same
        sorted_exponent_permutations.sort();
        let mut coeff: u64 = 0;

        // Check if we already calculate the coefficient
        for (term_exponents, coefficient) in &already_calc_coef {
            if *term_exponents == sorted_exponent_permutations {
                coeff = *coefficient;
                break;
            }
        }
        // If not we add it to the list
        if coeff == 0 {
            coeff = multinomial_coefficient(n, &sorted_exponent_permutations);
            already_calc_coef.push((sorted_exponent_permutations, coeff));
        }

        mult.push(Expression::integer(coeff));

        // Make a multiplication of all a_m to the power stored in exponent_permutation
        for (j, _) in exponent_permutation.iter().enumerate().take(m) {
            mult.push(Expression::exponentiation(
                terms[j].clone(),
                Expression::integer(exponent_permutation[j]),
            ));
        }

        result.push(Expression::multiplication(mult));
    }

    Expression::addition(result)
}

/// Find all permutations of m element that sum to n
fn find_permutations_with_sum(m: usize, n: u64) -> Vec<Vec<u64>> {
    let mut result = Vec::new();
    let mut current_permutation = Vec::new();

    // Helper function to backtrack and find permutations
    fn backtrack(
        m: usize,
        n: u64,
        current_sum: u64,
        current_permutation: &mut Vec<u64>,
        result: &mut Vec<Vec<u64>>,
    ) {
        if current_sum == n && current_permutation.len() == m {
            result.push(current_permutation.clone());
            return;
        }

        if current_sum > n || current_permutation.len() >= m {
            return;
        }

        for i in 0..=n {
            current_permutation.push(i);
            backtrack(m, n, current_sum + i, current_permutation, result);
            current_permutation.pop();
        }
    }

    // Start the backtracking algorithm
    backtrack(m, n, 0, &mut current_permutation, &mut result);

    result
}

/// Output the k! permutations of `array` in which the first `k` elements are permuted in all ways.
/// 
/// To get all permutations of array, use `k` = array.len().
/// 
/// If `k` is out of bounds return `None` 
/// 
/// Implemeted from https://en.wikipedia.org/wiki/Heap%27s_algorithm 
pub fn permutations<T>(k : usize, array: &mut Vec<T>) -> Option<Vec<Vec<T>>>
where T: Clone
{
    if k == 0 || k > array.len() {
        return None;
    }
    if k == 1 {
        return Some(vec![array.to_vec()]);
    }
    else{
        let mut perm: Vec<Vec<T>> = vec![];

        let result = permutations(k - 1, array);
        if let Some(mut result_perm) = result {
            perm.append(&mut result_perm);
        }

        for i in 0..(k-1) {
            if k % 2 == 0{
                array.swap(i, k-1)
            }else{
                array.swap(0, k-1)
            }
            let result = permutations(k - 1, array);
            if let Some(mut result_perm) = result {
                perm.append(&mut result_perm);
            }
        }
        Some(perm)
    }
}

/// Transform a `Vec<Expression>` representing a `Multiplication` into
/// a tuple that represent the terms with the sign and the coefficient separate.
///
/// The form is (negative, coefficient, terms)
pub fn transform_multiplication(terms: Vec<Expression>) -> (bool, u64, Vec<Expression>) {
    let mut negative = false;
    let mut coeff = 1;
    let mut striped_terms = vec![];

    terms.iter().for_each(|term| match term {
        Expression::Negation(inner) => {
            if let Expression::Number(numeral::Numeral::Integer(a)) = inner.term {
                coeff *= a;
                negative = !negative;
            } else {
                striped_terms.push(term.clone());
            }
        }
        Expression::Number(numeral::Numeral::Integer(a)) => {
            coeff *= a;
        }
        _ => {
            striped_terms.push(term.clone());
        }
    });

    (negative, coeff, striped_terms)
}

// Isolate a variable in an expression
// Reduce the occurence of the variable
// Substitute an expression by another expression

pub struct Polynomial {
    /// Represent the coefficient of the polynomial.
    /// They are in a descending order
    ///
    /// #Exemple
    /// ```
    /// // for a polynome P = x^2 + 8x
    /// // the coefficents sould be [1, 8, 0]
    /// assert_eq!(P.coefficients, vec![Expression::integer(1), Expression::integer(8), Expression::integer(0)])
    /// ```
    coefficients: Vec<Expression>,
    /// Represent the indeterminate in respect to wich are the coefficents
    /// 
    /// #Exemple
    /// ```
    /// // for a polynome P = x^2 + 8x
    /// // the indeterminate sould be x
    /// assert_eq!(P.indeterminate, Expression::variable("x"))
    /// ```
    indeterminate: Expression,
}

impl Polynomial {

    pub fn new(coefficients: Vec<Expression>, indeterminate: Expression) -> Self {
        Self { coefficients, indeterminate }
    }

    pub fn get_coefficients(&self) -> &[Expression] {
        &self.coefficients
    }

    pub fn get_indeterminate(&self) -> &Expression {
        &self.indeterminate
    }
}

/// Convert an `Expression` into an `Option<Polynomial>`
///
/// #Exemple
/// ```
/// // x^2 + 8x
/// let expr = Expression::addition(vec![
///     Expression::exponentiation(
///         Expression::variable("x"),        
///         Expression::integer(2),        
///     ),
///     Expression::multiplication(vec![
///         Expression::integer(8),         
///         Expression::variable("x"),        
///     ])
/// ])
///
/// let polynomial: Polynomial = to_polynomial(expr, Expression::variable("x"));
///
/// assert_eq!(polynomial.get_coefficients(), &[Expression::integer(1), Expression::integer(8), Expression::integer(0)])
/// assert_eq!(polynomial.get_indeterminate(), &Expression::ariable("x"))
/// ```
pub fn to_polynomial(polynomial: Expression, indeterminate: Expression) -> Option<Polynomial> {
    Some(match polynomial {
        Expression::Number(_) => {
            Polynomial::new(vec![polynomial], indeterminate)
        },
        Expression::Constant(_) => {
            Polynomial::new(vec![polynomial], indeterminate)
        },
        Expression::Variable(ref variable) => {
            if let Expression::Variable(ref var) = indeterminate {
                if variable.is_equal(&var) {
                    return Some(Polynomial::new(vec![Expression::integer(1), Expression::integer(0)], indeterminate))
                }
            }
            Polynomial::new(vec![polynomial], indeterminate)
        },
        Expression::Multiplication(multiplication) => {
            todo!()
        },
        Expression::Exponentiation(exponentiation) => todo!(),
        Expression::Subtraction(subtraction) => todo!(),
        Expression::Division(division) => todo!(),
        Expression::Addition(addition) => todo!(),
        Expression::Negation(negation) => todo!(),
        Expression::Equality(_) |
        Expression::Complex(_) |
        Expression::Function(_) |
        Expression::Derivative(_) => return None,
    })
}