use bevy::prelude::Vec3;
use std::ops::{Add, Mul};
use Vec3 as Color;

use crate::math;
use crate::triangle::Triangle;

pub struct BezierTriangle<T, const N: usize>
where
    T: Copy + Add<T, Output = T> + Mul<f32, Output = T>,
    [(); math::triangular_number(N + 1)]:,
{
    points: [T; math::triangular_number(N + 1)],
    colors: [Color; 3],
}

impl<T, const N: usize> BezierTriangle<T, N>
where
    T: Copy + Add<T, Output = T> + Mul<f32, Output = T>,
    [(); math::triangular_number(N + 1)]:,
{
    pub fn new(points: [T; math::triangular_number(N + 1)], colors: [Color; 3]) -> Self {
        BezierTriangle {
            points: points,
            colors: colors,
        }
    }
    fn evaluate(&self, u: f32, v: f32) -> T {
        unimplemented!()
    }
    pub fn subdivide(&self, u: f32, v: f32) -> (Self, Self) {
        unimplemented!()
    }
    fn triangulate(&self, max_curveature: f32, max_triangles: u32) -> Vec<Triangle<T>> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialization() {
        let three_colors = [Color::new(1., 0., 0.), Color::new(0., 1., 0.), Color::new(0., 0., 1.)];

        let pts = [1., 2., 3., 4., 5., 6.];
        let t = BezierTriangle::<_, 2>::new(pts, three_colors);
    }
}
