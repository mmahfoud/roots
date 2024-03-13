fn power(x: f64, n: usize) -> f64 {
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

const EPSILON: f64 = 1e-20;
const MAX_ITERATIONS: usize = 1_000_000;

fn next(x: f64, n: usize, p: f64) -> f64 {
    p - ((power(p, n) - x) / ((n as f64) * power(p, n - 1)))
}

fn root(x: f64, n: usize) -> f64 {
    if x.abs() <= EPSILON {
        return 0.0;
    }
    let mut p = if x >= 0.0 { 1.0 } else { -1.0 };
    let mut u = next(x, n, p);
    let mut i: usize = 1;
    while (((u - p) / p).abs() >= EPSILON) && (i <= MAX_ITERATIONS) {
        //println!("{}: {} - {} = {}", i, u, p, (u - p).abs());
        p = u;
        u = next(x, n, p);
        i += 1;
    }
    //println!("---------");
    u
}

struct TestCase {
    values: Vec<f64>,
    expected_root: f64,
}

fn main() {
    let mut test_cases: Vec<TestCase> = (0..=1000)
        .map(|x| TestCase {
            expected_root: x as f64,
            values: (2..=10).map(|y| power(x as f64, y)).collect(),
        })
        .collect();

    test_cases.push(TestCase {
        expected_root: 2.82842712474619,
        values: vec![8.0],
    });

    for test in test_cases {
        let roots: Vec<(f64, usize, f64, f64, bool)> = test
            .values
            .iter()
            .enumerate()
            .map(|(i, v)| {
                let r = root(*v, i + 2);
                (
                    test.expected_root,
                    i + 2,
                    *v,
                    r,
                    (r - test.expected_root).abs() < EPSILON,
                )
            })
            .collect();

        for (x, n, v, r, b) in roots {
            println!("{}: {}v{} = {} => {}", x, n, v, r, b);
        }
    }
}
