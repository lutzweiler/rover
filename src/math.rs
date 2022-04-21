use std::ops::{Add, Mul};

pub enum Axis2D {
    X,
    Y,
}

pub enum Axis3D {
    X,
    Y,
    Z,
}

pub const fn triangular_number(n: usize) -> usize {
    n * (n + 1) / 2
}

pub fn lerp<T>(a: T, b: T, t: f64) -> T where
    T: Add<T, Output = T> + Mul<f64, Output = T>,
{
    a * (1f64 - t) + b * t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangular_numbers() {
        assert_eq!(triangular_number(0), 0);
        assert_eq!(triangular_number(1), 1);
        for i in 1..10 {
            let a = triangular_number(i-1);
            let b = triangular_number(i);
            assert_eq!(a + i, b);
        }
    }

    #[test]
    fn linear_interpolation() {
        let examples = vec!(
            (0., 0., 0., 0.),
            (0., 2., 0., 0.),
            (0., 2., 1., 2.),
            (0., 2., 0.5, 1.),
            (-1., 1., 0.5, 0.),
            (0., 1., -1., -1.),
            (0., 1., 2., 2.),
        );
        for (a, b, t, r) in examples {
            assert_eq!(lerp(a, b, t), r);
        }
    }

}
