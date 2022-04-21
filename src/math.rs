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

pub fn lerp<T>(a: T, b: T, t: f64) -> T
where
    T: Add<T, Output = T> + Mul<f64, Output = T>,
{
    a * (1f64 - t) + b * t
}

/*
    Computes a triangular array of values out from a starting row
    Each consecutive row is computed from its previous by applying a linear interpolation of adjacent elements
    For starting elements a0, a1, a2,... the computation is as follows:
        a0  a1  a2  a3
         \  /\  /\  /
         b0  b1  b2
          \  /\  /
           c0  c1
           \  /
            d0
    where the configuration
           x  y
           \ /
            z
    computes z by linear interpolation between x and y at parameter t
*/
pub fn compute_triangular_scheme<T, const N: usize>(elements: &[T; N], t: f64) -> [T; triangular_number(N)]
where
    T: Copy + Add<T, Output = T> + Mul<f64, Output = T>,
    [(); triangular_number(N)]:,
{
    let mut row_len = N;
    let mut row_offset_prev = 0;
    let mut row_offset_curr = 0;
    let mut triangle = [elements[0]; triangular_number(N)];

    //copy first row
    for i in 0..N {
        triangle[i] = elements[i];
    }
    row_offset_curr += row_len;
    row_len -= 1;

    //apply scheme recursively starting from the copied row
    while row_len > 0 {
        for i in 0..row_len {
            let a = triangle[row_offset_prev + i];
            let b = triangle[row_offset_prev + i + 1];
            triangle[row_offset_curr + i] = lerp(a, b, t);
        }

        row_offset_prev = row_offset_curr;
        row_offset_curr += row_len;
        row_len -= 1;
    }
    triangle
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangular_numbers() {
        assert_eq!(triangular_number(0), 0);
        assert_eq!(triangular_number(1), 1);
        for i in 1..10 {
            let a = triangular_number(i - 1);
            let b = triangular_number(i);
            assert_eq!(a + i, b);
        }
    }

    #[test]
    fn linear_interpolation() {
        let examples = vec![
            (0., 0., 0., 0.),
            (0., 2., 0., 0.),
            (0., 2., 1., 2.),
            (0., 2., 0.5, 1.),
            (-1., 1., 0.5, 0.),
            (0., 1., -1., -1.),
            (0., 1., 2., 2.),
        ];
        for (a, b, t, r) in examples {
            assert_eq!(lerp(a, b, t), r);
        }
    }

    #[test]
    fn triangular_scheme() {
        let row = [0., 4., 6., 9.];
        let res = compute_triangular_scheme(&row, 0.5);
        let expected = [0., 4., 6., 9., 2., 5., 7.5, 3.5, 6.25, 4.875];
        assert_eq!(res, expected);
    }
}
