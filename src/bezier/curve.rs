use std::ops::{Add, Mul};

pub struct BezierCurve<T, const N: usize>
where
    T: Copy + Add<T, Output = T> + Mul<f32, Output = T>,
    [(); N + 1]:,
{
    points: [T; N + 1],
}

impl<T, const N: usize> BezierCurve<T, N>
where
    T: Copy + Add<T, Output = T> + Mul<f32, Output = T>,
    [(); N + 1]:,
{
    pub fn new(points: [T; N + 1]) -> Self {
        BezierCurve { points: points }
    }
    fn evaluate(&self, t: f32) -> T {
        unimplemented!()
    }
    fn subdivide(&self, t: f32) -> (Self, Self) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::Vec3;

    #[test]
    fn initialization() {
        let pts1 = [1., 2., 3., 4.];
        let v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(1., 4., 3.);
        let v3 = Vec3::new(3., 2., 3.);
        let pts2 = [v1, v2, v3];
        let b1 = BezierCurve::<_, 3>::new(pts1);
        let b2 = BezierCurve::<_, 2>::new(pts2);
        let b3 = BezierCurve::<_, 0>::new([3.]);
    }
}
