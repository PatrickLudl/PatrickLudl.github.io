use std::ops::{Add};

#[derive(Debug, PartialEq, Clone, Copy)]
struct ComplexNumber {
    re: f64,
    im: f64,
}

impl ComplexNumber {
    fn new(re: f64, im: f64) -> Self {
        ComplexNumber { re, im }
    }

    fn add(&self, other: &ComplexNumber) -> ComplexNumber {
        ComplexNumber {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }

    fn subtract(&self, other: &ComplexNumber) -> ComplexNumber {
        ComplexNumber {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }

    fn multiply(&self, other: &ComplexNumber) -> ComplexNumber {
        ComplexNumber {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }

    fn absolute_value(&self) -> f64 {
        (self.re.powi(2) + self.im.powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests_complex {
    use super::*;
    const TOLERANCE: f64 = 1e-12;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < TOLERANCE
    }

    #[test]
    fn test_add() {
        assert!(
            approx_eq(
                ComplexNumber::new(3.0, 4.0).add(&ComplexNumber::new(-8.0, 13.0)).re,
                ComplexNumber::new(-5.0, 17.0).re
            ) && approx_eq(
                ComplexNumber::new(3.0, 4.0).add(&ComplexNumber::new(-8.0, 13.0)).im,
                ComplexNumber::new(-5.0, 17.0).im
            )
        );
        assert!(
            approx_eq(
                ComplexNumber::new(0.0, -1.0).add(&ComplexNumber::new(0.0, 1.0)).re,
                ComplexNumber::new(0.0, 0.0).re
            ) && approx_eq(
                ComplexNumber::new(0.0, -1.0).add(&ComplexNumber::new(0.0, 1.0)).im,
                ComplexNumber::new(0.0, 0.0).im
            )
        );
        assert!(
            approx_eq(
                ComplexNumber::new(2f64.sqrt(), 1.5).add(&ComplexNumber::new(1.0, -0.5)).re,
                ComplexNumber::new(1.0 + 2f64.sqrt(), 1.0).re
            ) && approx_eq(
                ComplexNumber::new(2f64.sqrt(), 1.5).add(&ComplexNumber::new(1.0, -0.5)).im,
                ComplexNumber::new(1.0 + 2f64.sqrt(), 1.0).im
            )
        );
    }

    #[test]
    fn test_multiply() {
        assert!(
            approx_eq(
                ComplexNumber::new(3.0, 4.0).multiply(&ComplexNumber::new(0.0, 0.0)).re,
                ComplexNumber::new(0.0, 0.0).re
            ) && approx_eq(
                ComplexNumber::new(3.0, 4.0).multiply(&ComplexNumber::new(0.0, 0.0)).im,
                ComplexNumber::new(0.0, 0.0).im
            )
        );
        assert!(
            approx_eq(
                ComplexNumber::new(-1.0, 8.0).multiply(&ComplexNumber::new(2.0, -1.0)).re,
                ComplexNumber::new(6.0, 17.0).re
            ) && approx_eq(
                ComplexNumber::new(-1.0, 8.0).multiply(&ComplexNumber::new(2.0, -1.0)).im,
                ComplexNumber::new(6.0, 17.0).im
            )
        );
        let result = ComplexNumber::new(2f64.sqrt(), 1.0).multiply(&ComplexNumber::new(3f64.sqrt(), 2f64.sqrt()));
        let expected = ComplexNumber::new(6f64.sqrt() - 2f64.sqrt(), 3f64.sqrt() + 2.0);
        assert!(approx_eq(result.re, expected.re) && approx_eq(result.im, expected.im));
    }

    #[test]
    fn test_absolute_value() {
        assert!(approx_eq(ComplexNumber::new(1.0, 1.0).absolute_value(), 2f64.sqrt()));
        assert!(approx_eq(ComplexNumber::new(0.0, -1.0).absolute_value(), 1.0));
        assert!(approx_eq(ComplexNumber::new(3.0, -4.0).absolute_value(), 5.0));
        assert!(approx_eq(
            ComplexNumber::new(0.5, -3f64.sqrt() / 2.0).absolute_value(),
            1.0
        ));
    }
}


#[derive(Debug, Clone)]
struct SquareMatrix {
    size: usize,
    elements: Vec<ComplexNumber>,
}

impl SquareMatrix {
    fn new(size: usize) -> Self {
        SquareMatrix {
            size,
            elements: vec![ComplexNumber::new(0.0, 0.0); size * size],
        }
    }

    fn get_element(&self, i: usize, j: usize) -> &ComplexNumber {
        &self.elements[i * self.size + j]
    }

    fn set_element(&mut self, i: usize, j: usize, value: ComplexNumber) {
        self.elements[i * self.size + j] = value;
    }

    fn add(&self, other: &SquareMatrix) -> SquareMatrix {
        let mut result = SquareMatrix::new(self.size);
        for i in 0..self.size {
            for j in 0..self.size {
                result.set_element(i, j, self.get_element(i, j).add(other.get_element(i, j)));
            }
        }
        result
    }

    fn multiply(&self, other: &SquareMatrix) -> SquareMatrix {
        let mut result = SquareMatrix::new(self.size);
        for i in 0..self.size {
            for j in 0..self.size {
                let mut sum = ComplexNumber::new(0.0, 0.0);
                for k in 0..self.size {
                    sum = sum.add(&self.get_element(i, k).multiply(other.get_element(k, j)));
                }
                result.set_element(i, j, sum);
            }
        }
        result
    }

    fn trace(&self) -> ComplexNumber {
        let mut sum = ComplexNumber::new(0.0, 0.0);
        for i in 0..self.size {
            sum = sum.add(self.get_element(i, i));
        }
        sum
    }
}

fn are_numerically_equal(a: &SquareMatrix, b: &SquareMatrix, tolerance: f64) -> bool {
    for i in 0..a.size {
        for j in 0..a.size {
            if a.get_element(i, j).subtract(b.get_element(i, j)).absolute_value() >= tolerance {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_numerically_equal() {
        let mut a = SquareMatrix::new(2);
        a.set_element(0, 0, ComplexNumber::new(1.0, 1.0));
        a.set_element(0, 1, ComplexNumber::new(-8.0, 0.0));
        a.set_element(1, 0, ComplexNumber::new(0.0, 0.0));
        a.set_element(1, 1, ComplexNumber::new(2.0, 3.0));

        let mut b = SquareMatrix::new(2);
        b.set_element(0, 0, ComplexNumber::new(0.0, -1.0));
        b.set_element(0, 1, ComplexNumber::new(0.0, 0.0));
        b.set_element(1, 0, ComplexNumber::new(1.0, -5.0));
        b.set_element(1, 1, ComplexNumber::new(2.0, 3.0));

        let mut c = SquareMatrix::new(2);
        c.set_element(0, 0, ComplexNumber::new(1.0, 0.0));
        c.set_element(0, 1, ComplexNumber::new(-8.0, 0.0));
        c.set_element(1, 0, ComplexNumber::new(1.0, -5.0));
        c.set_element(1, 1, ComplexNumber::new(4.0, 6.0));

        let mut d = SquareMatrix::new(2);
        d.set_element(0, 0, ComplexNumber::new(-7.0, 39.0));
        d.set_element(0, 1, ComplexNumber::new(-16.0, -24.0));
        d.set_element(1, 0, ComplexNumber::new(17.0, -7.0));
        d.set_element(1, 1, ComplexNumber::new(-5.0, 12.0));

        assert!(are_numerically_equal(&a, &a, 1e-9));
        assert!(are_numerically_equal(&b, &b, 1e-9));
        assert!(are_numerically_equal(&c, &c, 1e-9));
        assert!(are_numerically_equal(&d, &d, 1e-9));
        assert!(!are_numerically_equal(&a, &b, 1e-9));
        assert!(!are_numerically_equal(&a, &c, 1e-9));

        let a_plus_b = a.add(&b);
        assert!(are_numerically_equal(&a_plus_b, &c, 1e-9));
        assert!(!are_numerically_equal(&a_plus_b, &d, 1e-9));

        let a_times_b = a.multiply(&b);
        assert!(!are_numerically_equal(&a_times_b, &c, 1e-9));
        assert!(are_numerically_equal(&a_times_b, &d, 1e-9));

        assert_eq!(a.trace(), ComplexNumber::new(3.0, 4.0));
        assert_eq!(b.trace(), ComplexNumber::new(2.0, 2.0));
        assert_eq!(c.trace(), ComplexNumber::new(5.0, 6.0));
        assert_eq!(d.trace(), ComplexNumber::new(-12.0, 51.0));
    }
}


use std::f64::consts::PI;

fn is_in(matrix: &SquareMatrix, list: &[SquareMatrix]) -> bool {
    list.iter().any(|m| are_numerically_equal(matrix, m, 1e-9))
}

fn generate_group_elements(initial_set: &[SquareMatrix]) -> Vec<SquareMatrix> {
    let mut group_elements = initial_set.to_vec();
    let mut i = 0;
    while i < group_elements.len() {
        let mut j = 0;
        while j < group_elements.len() {
            let product = group_elements[i].multiply(&group_elements[j]);
            if !is_in(&product, &group_elements) {
                group_elements.push(product);
            }
            j += 1;
        }
        i += 1;
    }
    group_elements
}

fn is_irreducible(group_elements: &[SquareMatrix]) -> bool {
    let order = group_elements.len() as f64;
    let trace_sum_squared: f64 = group_elements.iter()
        .map(|m| m.trace().absolute_value().powi(2))
        .sum();
    (trace_sum_squared - order).abs() <= 1e-9
}


#[cfg(test)]
mod test_groups {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_group_properties_omega() {
        let omega = ComplexNumber::new(-0.5, (3.0f64.sqrt() / 2.0));
        let omega2 = omega.multiply(&omega);

        let mut g1 = SquareMatrix::new(3);
        g1.set_element(0, 1, ComplexNumber::new(1.0, 0.0));
        g1.set_element(1, 2, ComplexNumber::new(1.0, 0.0));
        g1.set_element(2, 0, ComplexNumber::new(1.0, 0.0));

        let mut g2 = SquareMatrix::new(3);
        g2.set_element(0, 0, ComplexNumber::new(1.0, 0.0));
        g2.set_element(1, 1, omega);
        g2.set_element(2, 2, omega2);

        let initial_set = vec![g1, g2];
        let group_elements = generate_group_elements(&initial_set);
        let order = group_elements.len();
        let irreducible = is_irreducible(&group_elements);

        assert_eq!(order, 27);
        assert!(irreducible);
    }

    #[test]
    fn test_group_properties_eta() {
        let eta = ComplexNumber::new(0.0, 1.0); // exp(2pi i / 4) = i
        let eta2 = eta.multiply(&eta);
        let eta3 = eta2.multiply(&eta);

        let mut g3 = SquareMatrix::new(3);
        g3.set_element(0, 2, ComplexNumber::new(1.0, 0.0));
        g3.set_element(1, 1, eta2);
        g3.set_element(2, 0, eta);

        let mut g4 = SquareMatrix::new(3);
        g4.set_element(0, 1, eta3);
        g4.set_element(1, 0, ComplexNumber::new(1.0, 0.0));
        g4.set_element(2, 2, eta2);

        let initial_set_eta = vec![g3, g4];
        let group_elements_eta = generate_group_elements(&initial_set_eta);
        let order_eta = group_elements_eta.len();
        let irreducible_eta = is_irreducible(&group_elements_eta);

        assert_eq!(order_eta, 192);
        assert!(irreducible_eta);
    }

    #[test]
    fn test_group_properties_simple() {
        let minus_one = ComplexNumber::new(-1.0, 0.0);

        let mut g5 = SquareMatrix::new(2);
        g5.set_element(0, 0, ComplexNumber::new(1.0, 0.0));
        g5.set_element(1, 1, minus_one);

        let mut g6 = SquareMatrix::new(2);
        g6.set_element(0, 0, minus_one);
        g6.set_element(1, 1, ComplexNumber::new(1.0, 0.0));

        let initial_set_simple = vec![g5, g6];
        let group_elements_simple = generate_group_elements(&initial_set_simple);
        let order_simple = group_elements_simple.len();
        let irreducible_simple = is_irreducible(&group_elements_simple);

        assert_eq!(order_simple, 4);
        assert!(!irreducible_simple);
    }
}


use std::time::Instant;

fn main() {
    let omega = ComplexNumber::new(-0.5, 3.0f64.sqrt() / 2.0);
    let omega2 = omega.multiply(&omega);

    let alpha = ComplexNumber::new(0.0, -1.0 / 3.0f64.sqrt());
    let epsilon = ComplexNumber::new((4.0 * PI / 9.0).cos(), (4.0 * PI / 9.0).sin());
    let epsilon_omega = epsilon.multiply(&omega);

    let mut g1 = SquareMatrix::new(3);
    g1.set_element(0, 1, ComplexNumber::new(1.0, 0.0));
    g1.set_element(1, 2, ComplexNumber::new(1.0, 0.0));
    g1.set_element(2, 0, ComplexNumber::new(1.0, 0.0));

    let mut g2 = SquareMatrix::new(3);
    g2.set_element(0, 0, ComplexNumber::new(1.0, 0.0));
    g2.set_element(1, 1, omega);
    g2.set_element(2, 2, omega2);

    let mut g7 = SquareMatrix::new(3);
    g7.set_element(0, 0, alpha);
    g7.set_element(0, 1, alpha);
    g7.set_element(0, 2, alpha);
    g7.set_element(1, 0, alpha);
    g7.set_element(1, 1, alpha.multiply(&omega));
    g7.set_element(1, 2, alpha.multiply(&omega2));
    g7.set_element(2, 0, alpha);
    g7.set_element(2, 1, alpha.multiply(&omega2));
    g7.set_element(2, 2, alpha.multiply(&omega));

    let mut g8 = SquareMatrix::new(3);
    g8.set_element(0, 0, epsilon);
    g8.set_element(1, 1, epsilon);
    g8.set_element(2, 2, epsilon_omega);

    let initial_set = vec![g1, g2, g7, g8];

    let start = Instant::now();
    let group_elements = generate_group_elements(&initial_set);
    let order = group_elements.len();
    let irreducible = is_irreducible(&group_elements);
    let duration = start.elapsed();

    println!("The order of the group is {}", order);
    println!("The representation is irreducible: {}", irreducible);
    println!("Time needed is: {:?}", duration);
    /*
    The order of the group is 648
    The representation is irreducible: true
    Time needed is: 13.5238412s
    */
}