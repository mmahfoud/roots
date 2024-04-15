use crate::polynomials::Polynom;

pub mod polynomials;

/**
calculates the result of raising an f64 number to an integer u32 power.

# Examples

```
let x = 2.0f64;
let n = 4u32;
let answer = crate::power(x, n);

assert_eq!(16.0f64, answer);
```
*/
fn power(x: f64, n: u32) -> f64 {
    let mut result = 1f64;
    let mut y = x;
    let mut count = n;
    while count > 0 {
        if count % 2 == 1 {
            result *= y;
        }
        y *= y;
        count /= 2;
    }
    result
}

/// Approximation order
const EPSILON: f64 = 1e-20;

/// Maximum number of iterations
const MAX_ITERATIONS: u32 = 1_000_000;

/**
calculates the next term of Newton method serie (for Polynom P(X) = X^n - v

Description here
and here.

* `v` the v in P(X) = X^n - v
* `n` the Polynom degree n in P(X) = X^n - v
* `p` the previous term
*/
fn basic_next(v: f64, n: u32, p: f64) -> f64 {
    // Un+1 = Un - (Un^n - x)/(n.Un^(n-1))
    p - ((power(p, n) - v) / ((n as f64) * power(p, n - 1)))
}

/**
calculates an approximate value of the n-th root of an f64 number.

# Examples

```
let x: f64 = 16.0;
let n: u32 = 4;
let answer = crate::root(x, n);

assert!((answer - 2.0).abs() >= crate::EPSILON);
```
*/
fn root<F>(x: f64, n: u32, next: F) -> f64
where
    F: Fn((f64, u32, f64)) -> f64,
{
    // return 0 for values of x too small
    if x.abs() <= EPSILON {
        return 0.0;
    }

    // start from +1 or -1 (based on the sign of x)
    let mut p = if x >= 0.0 { 1.0 } else { -1.0 };

    // calculate next value
    let mut u = next((x, n, p));

    let mut i: u32 = 1;
    while (((u - p) / p).abs() >= EPSILON) && (i <= MAX_ITERATIONS) {
        p = u;
        u = next((x, n, p));
        i += 1;
    }
    u
}

fn main() {
    let sqrt_4 = root(4.0, 2, |(a, b, c)| basic_next(a, b, c));
    println!("4^(1/2) = {}", sqrt_4);

    println!(
        "(81.0)^(1/4) = {}",
        root(81.0, 4, |(a, b, c)| (((b as f64) - 1.0) * power(c, b) + a)
            / ((b as f64) * power(c, b - 1)))
    );

    println!(
        "X^2+1 * X-1 = {}",
        Polynom::initialize(vec![1.0, 0.0, 1.0]) * Polynom::initialize(vec![-1.0, 1.0])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Represents a test case to be checked. it contains a number (expected_root) and a vector of its powers starting from its square.
    struct TestCase {
        values: Vec<f64>,
        expected_root: f64,
    }

    #[test]
    fn nth_square_for_x_to_the_power_n_should_be_x() {
        // initialize test case with numbers between 0 and 1000 with their powers from 2 to 10.
        let mut test_cases: Vec<TestCase> = (0..=10)
            .map(|x| TestCase {
                expected_root: x as f64,
                values: (2..=10).map(|y| power(x as f64, y)).collect(),
            })
            .collect();

        // add a test case for square root of 8
        test_cases.push(TestCase {
            expected_root: 2.82842712474619,
            values: vec![8.0],
        });

        // add a test case for square root of 8
        test_cases.push(TestCase {
            expected_root: 4.9999999999999999, // Should be 5 but 4.9999999999999999 is close enough (<= EPSILON 1e-20)
            values: vec![25.0],
        });

        // perform calculations on each test case
        for test in test_cases {
            let roots: Vec<(f64, u32, f64, f64, bool)> = test
                .values
                .into_iter()
                .enumerate()
                .map(|(i, v)| {
                    let r = root(v, (i as u32) + 2, |(a, b, c)| basic_next(a, b, c));
                    (
                        test.expected_root,
                        (i as u32) + 2,
                        v,
                        r,
                        (r - test.expected_root).abs() < EPSILON,
                    )
                })
                .collect();

            // print test results for this test case
            for (x, n, v, r, b) in roots {
                println!("{}th root of {} expected={} found={}! => {}", n, v, x, r, b);
                assert!(b, "{}th root of {} expected={} found={}!", n, v, x, r);
            }
        }
    }
}
