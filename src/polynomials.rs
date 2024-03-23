use std::{fmt, ops};

#[derive(Debug, PartialEq)]
pub struct Polynom {
    coeficients: Vec<f64>,
}

impl Polynom {
    pub fn zero() -> Self {
        Polynom {
            coeficients: vec![0.0],
        }
    }

    pub fn single(n: usize) -> Self {
        Polynom {
            coeficients: (0..=n).map(|i| if i < n { 0.0 } else { 1.0 }).collect(),
        }
    }

    pub fn initialize(coefs: Vec<f64>) -> Self {
        assert!(coefs.len() > 0);
        assert_ne!(*coefs.last().unwrap(), 0.0);
        Polynom { coeficients: coefs }
    }

    pub fn degree(&self) -> usize {
        self.coeficients.len() - 1
    }

    pub fn at(&self, i: usize) -> f64 {
        self.coeficients[i]
    }

    pub fn set_at(mut self, i: usize, v: f64) -> Self {
        // assert!(i < self.degree() || v != 0.0);
        self.coeficients[i] = v;
        self.trim()
    }

    pub fn plus(mut self, x: f64) -> Self {
        for i in 0..self.coeficients.len() {
            self.coeficients[i] += x;
        }
        self.trim()
    }

    pub fn by(mut self, x: f64) -> Self {
        for i in 0..self.coeficients.len() {
            self.coeficients[i] *= x;
        }
        self
    }

    fn trim(mut self) -> Self {
        while self.coeficients.len() > 1 && *self.coeficients.last().unwrap() == 0.0 {
            self.coeficients.pop();
        }
        self
    }
}

impl fmt::Display for Polynom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        let d = self.degree();
        for i in (0..=d).rev() {
            let x = self.coeficients[i];
            if x != 0.0 {
                if i < d && x > 0.0 {
                    res.push('+');
                } else if x < 0.0 {
                    res.push('-');
                }
                if x.abs() != 1.0 || i == 0 {
                    res.push_str(x.abs().to_string().as_str());
                }
                if i > 0 {
                    res.push('X');
                }
                if i > 1 {
                    res.push('^');
                    res.push_str(i.to_string().as_str());
                }
            }
        }
        if res.len() == 0 {
            res.push('0');
        }
        write!(f, "{}", res)
    }
}

impl ops::Add<Polynom> for Polynom {
    type Output = Polynom;

    fn add(self, rhs: Polynom) -> Self::Output {
        let (min, max) = if self.degree() >= rhs.degree() {
            (rhs, self)
        } else {
            (self, rhs)
        };
        Polynom {
            coeficients: (0..=max.degree())
                .map(|i| {
                    if i <= min.degree() {
                        min.coeficients[i] + max.coeficients[i]
                    } else {
                        max.coeficients[i]
                    }
                })
                .collect(),
        }
    }
}

impl ops::Mul<Polynom> for Polynom {
    type Output = Polynom;

    fn mul(self, rhs: Polynom) -> Self::Output {
        let d = self.degree() + rhs.degree();
        let mut result = Polynom::single(d);
        result.coeficients[d] = 0.0;
        for i in 0..=self.degree() {
            for j in 0..=rhs.degree() {
                result.coeficients[i + j] += self.coeficients[i] * rhs.coeficients[j];
            }
        }
        result.trim()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_zero_to_zero_should_return_zero() {
        let z1 = Polynom::zero();
        let z2 = Polynom::zero();
        let r = z1 + z2;
        assert_eq!(r, Polynom::zero());
    }

    #[test]
    fn add_zero_to_single_n_should_return_single_n() {
        let p1 = Polynom::zero();
        let p2 = Polynom::single(3);
        let r = p1 + p2;
        assert_eq!(r, Polynom::single(3));
    }

    #[test]
    fn add_single_n_to_single_n_should_return_2_single_n() {
        let p1 = Polynom::single(3);
        let p2 = Polynom::single(3);
        let r = p1 + p2;
        let expexted = Polynom::single(3).by(2.0);
        assert_eq!(r, expexted);
    }

    #[test]
    fn multiply_simple_cases() {
        assert_eq!(Polynom::zero() * Polynom::single(1), Polynom::zero());

        assert_eq!(Polynom::single(2) * Polynom::single(3), Polynom::single(5));

        assert_eq!(
            Polynom::single(2).plus(1.0) * Polynom::zero().plus(5.0),
            Polynom::single(2).plus(1.0).by(5.0)
        );
    }

    #[test]
    fn valid_polynomes() {
        assert_eq!(Polynom::zero().plus(0.0), Polynom::zero());

        assert_eq!(Polynom::single(1).plus(-1.0), Polynom::zero().plus(-1.0));

        assert_eq!(
            Polynom::initialize(vec![0.0, -1.0]).plus(1.0),
            Polynom::initialize(vec![1.0])
        );

        let mut p = Polynom::single(10);
        for i in 0..p.degree() {
            p = p.set_at(i, 1.0);
        }
        assert_eq!(p, Polynom::initialize(vec![1.0; 11]));
    }

    #[test]
    fn test_format() {
        assert_eq!(format!("{}", Polynom::initialize(vec![-1.0, -2.0, 0.0, 0.0, 3.0, -4.0])), "-4X^5+3X^4-2X-1");

        assert_eq!(format!("{}", Polynom::zero()), "0");

        assert_eq!(format!("{}", Polynom::single(2)), "X^2");

        assert_eq!(format!("{}", Polynom::single(2).by(2.0).plus(-1.0).by(-0.5)), "-0.5X^2+0.5X+0.5");
    }
}
