use bevy::math::f64::DVec3 as Vec3;
use std::ops::{Add, Mul, Sub};
use Vec3 as Color;

use crate::math;
use crate::triangle::Triangle;

pub trait FromString {
    fn from_string(lines: &str) -> Result<Self, String>
    where
        Self: Sized;
}

pub struct BezierCurve<T, const N: usize>
where
    T: Copy + Add<T> + Sub<T> + Mul,
    [(); N + 1]:,
{
    points: [T; N + 1],
}

pub struct BezierRectangle<T, const N: usize, const M: usize>
where
    T: Copy + Add<T> + Sub<T> + Mul,
    [(); (N + 1) * (M + 1)]:,
{
    points: [T; (N + 1) * (M + 1)],
    colors: [Color; 4],
}

pub struct BezierTriangle<T, const N: usize>
where
    T: Copy + Add<T> + Sub<T> + Mul,
    [(); math::triangular_number(N + 1)]:,
{
    points: [T; math::triangular_number(N + 1)],
    colors: [Color; 3],
}

impl<T, const N: usize> BezierCurve<T, N>
where
    T: Copy + Add<T> + Sub<T> + Mul,
    [(); N + 1]:,
{
    pub fn new(points: [T; N + 1]) -> Self {
        BezierCurve { points: points }
    }
    fn evaluate(&self, t: f64) -> T {
        unimplemented!()
    }
    fn subdivide(&self, t: f64) -> (Self, Self) {
        unimplemented!()
    }
}

impl<T, const N: usize, const M: usize> BezierRectangle<T, N, M>
where
    T: Copy + Add<T> + Sub<T> + Mul,
    [(); (N + 1) * (M + 1)]:,
{
    pub fn new(points: [T; (N + 1) * (M + 1)], colors: [Color; 4]) -> Self {
        BezierRectangle {
            points: points,
            colors: colors,
        }
    }
    fn evaluate(&self, u: f64, v: f64) -> T {
        unimplemented!()
    }
    fn subdivide(&self, axis: math::Axis2D, t: f64) -> (Self, Self) {
        unimplemented!()
    }
    fn triangulate(&self, max_curveature: f64, max_triangles: u32) -> Vec<Triangle<T>> {
        unimplemented!()
    }
}

impl<T, const N: usize> BezierTriangle<T, N>
where
    T: Copy + Add<T> + Sub<T> + Mul,
    [(); math::triangular_number(N + 1)]:,
{
    pub fn new(points: [T; math::triangular_number(N + 1)], colors: [Color; 3]) -> Self {
        BezierTriangle {
            points: points,
            colors: colors,
        }
    }
    fn evaluate(&self, u: f64, v: f64) -> T {
        unimplemented!()
    }
    fn subdivide(&self, u: f64, v: f64) -> (Self, Self) {
        unimplemented!()
    }
    fn triangulate(&self, max_curveature: f64, max_triangles: u32) -> Vec<Triangle<T>> {
        unimplemented!()
    }
}

impl<const N: usize, const M: usize> FromString for BezierRectangle<Vec3, N, M>
where
    [(); (N + 1) * (M + 1)]:,
{
    fn from_string(lines: &str) -> Result<BezierRectangle<Vec3, N, M>, String> {
        let num_ctrl_pts: usize = (N + 1) * (M + 1);
        let mut points = [Vec3::new(0., 0., 0.); (N + 1) * (M + 1)];
        let mut colors = [Color::new(0., 0., 0.); 4];
        let mut index = 0;
        for line in lines.lines() {
            let mut value = Vec3::new(0., 0., 0.);
            let mut line_iter = line.split_whitespace();
            if let Some(x) = line_iter.next() {
                value.x = x.parse::<f64>().unwrap();
            } else {
                return Err(format!("missing vector element"));
            }
            if let Some(y) = line_iter.next() {
                value.y = y.parse::<f64>().unwrap();
            } else {
                return Err(format!("missing vector element"));
            }
            if let Some(z) = line_iter.next() {
                value.z = z.parse::<f64>().unwrap();
            } else {
                return Err(format!("missing vector element"));
            }
            if index < num_ctrl_pts {
                points[index] = value;
                println!("point {}", value);
            } else if index < num_ctrl_pts + 4 {
                colors[index - num_ctrl_pts] = value;
                println!("color {}", value);
            }
            index += 1;
        }
        if index != (num_ctrl_pts + 4) {
            let err_message = format!("{} coordinates given, {} expected", index, num_ctrl_pts);
            return Err(err_message);
        }
        Ok(BezierRectangle::<Vec3, N, M>::new(points, colors))
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn initialization() {
        let three_colors = [Color::new(1., 0., 0.), Color::new(0., 1., 0.), Color::new(0., 0., 1.)];
        let four_colors = [
            Color::new(1., 0., 0.),
            Color::new(0., 1., 0.),
            Color::new(0., 0., 1.),
            Color::new(0., 1., 1.),
        ];

        let pts1 = [1, 2, 3, 4];
        let pts2 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let pts3 = [1, 2, 3, 4, 5, 6];
        let b = BezierCurve::<_, 3>::new(pts1);
        let r = BezierRectangle::<_, 2, 3>::new(pts2, four_colors);
        let t = BezierTriangle::<_, 2>::new(pts3, three_colors);

        let v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(1., 4., 3.);
        let v3 = Vec3::new(3., 2., 3.);
        let pts4 = [v1, v2, v3];
        let b2 = BezierCurve::<_, 2>::new(pts4);
        let b3 = BezierCurve::<_, 0>::new([3]);

        assert!(true);
    }

    #[test]
    fn parse_off_cbez_3d() {
        let example_cbez123 = "0. 0. 0.
            1. 2. 3.
            1. 3. -2.
            2. .1 -.3
            1. 2. 3.
            1. 2. 3.
            1. 1. 0.
            0. 1. 1.
            1. 0. 1.
            1. 0. 1.";

        let example_cbez333 = "0. 0. 0.
            1. 2. 3.
            1. 3. -2.
            2. .1 -.3
            1. 2. 3.
            1. 2. 3.
            1. 2. 3.
            1. 2 3.
            1. 2. 3.
            1. 2. 3.
            1. 2. 3.
            1. 2. 3.
            1. 2. 3.
            1. 2. 3.
            1. 2. 3.
            1. 2. 3.
            1. 1. 0.
            0. 1. 1.
            1. 0. 1.
            1. 0. 1.";

        match BezierRectangle::<Vec3, 1, 2>::from_string(example_cbez123) {
            Ok(b) => {
                assert_eq!(b.points[0], Vec3::new(0., 0., 0.));
                assert_eq!(b.points[5], Vec3::new(1., 2., 3.));
                assert_eq!(b.colors[0], Vec3::new(1., 1., 0.));
                assert_eq!(b.colors[3], Vec3::new(1., 0., 1.));
            },
            Err(e) => {
                println!("{}", e);
                assert!(false);
            } 
        }

        match BezierRectangle::<Vec3, 3, 3>::from_string(example_cbez333) {
            Ok(b) => {
                assert_eq!(b.points[0], Vec3::new(0., 0., 0.));
                assert_eq!(b.points[15], Vec3::new(1., 2., 3.));
                assert_eq!(b.colors[0], Vec3::new(1., 1., 0.));
                assert_eq!(b.colors[3], Vec3::new(1., 0., 1.));
            },
            Err(e) => {
                println!("{}", e);
                assert!(false);
            } 
        }
    }
}
