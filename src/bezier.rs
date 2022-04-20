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
        if index != num_ctrl_pts {
            let err_message = format!("{} coordinates given, {} expected", index, num_ctrl_pts);
            return Err(err_message);
        }
        Ok(BezierRectangle::<Vec3, N, M>::new(points, colors))
    }
}
