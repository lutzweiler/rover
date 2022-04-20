#![feature(generic_const_exprs)]
#![allow(dead_code)]
#![allow(unused_variables)]


use bevy::math::f64::DVec3 as Vec3;
use std::ops::{Add, Mul, Sub};

/*
    this project uses const generics to create indiviudal structs for any degree of bezier curve/surface.
    since bezier curves use one more control point than their degree, and surfaces need a product / triangular number of ctrl points,
    it is necessary to compute the array sizes at compile time from the generic parameter.
    evaluating generic const expressions is a feature only available in the nightly version of rust and needs to be enabled manually.
    as such it is possible that some syntax needs to be changed in the future in order for this to work correctly.
    further, some trait bounds need to be explicitly stated:
    for a bezier curve of degree N, an array of size N+1 is needed. in the (extremely unrealistic case) of N=usize::max,
    such an array can not be allocated, therefore the trait bound
        [(); N+1]:
    is needed.
    note that the compiler cannot infer the degree of a bezier object only from an array size. it can however infer the type of control point.
    initialize a bezier curve of eg degree 2 like this:
        let b = BezierCurve::<_, 2>::new([1,2,3]);
*/

use Vec3 as Color;

enum Axis2D {
    X,
    Y,
}

enum Axis3D {
    X,
    Y,
    Z,
}

trait FromString {
    fn from_string(lines: &str) -> Result<Self, String> where Self: Sized;
}


const fn triangular_number(n: usize) -> usize {
    n * (n + 1) / 2
}

struct Triangle<T>
where
    T: Copy + Add<T> + Sub<T> + Mul,
{
    v1: T,
    v2: T,
    v3: T,
}

struct BezierCurve<T, const N: usize>
where
    T: Copy + Add<T> + Sub<T> + Mul,
    [(); N + 1]:,
{
    points: [T; N + 1],
}

struct BezierRectangle<T, const N: usize, const M: usize>
where
    T: Copy + Add<T> + Sub<T> + Mul,
    [(); (N + 1) * (M + 1)]:,
{
    points: [T; (N + 1) * (M + 1)],
    colors: [Color; 4],
}

struct BezierTriangle<T, const N: usize>
where
    T: Copy + Add<T> + Sub<T> + Mul,
    [(); triangular_number(N + 1)]:,
{
    points: [T; triangular_number(N + 1)],
    colors: [Color; 3],
}

impl<T, const N: usize> BezierCurve<T, N>
where
    T: Copy + Add<T> + Sub<T> + Mul,
    [(); N + 1]:,
{
    fn new(points: [T; N + 1]) -> Self {
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
    fn new(points: [T; (N + 1) * (M + 1)], colors: [Color; 4]) -> Self {
        BezierRectangle { points: points, colors: colors }
    }
    fn evaluate(&self, u: f64, v: f64) -> T {
        unimplemented!()
    }
    fn subdivide(&self, axis: Axis2D, t: f64) -> (Self, Self) {
        unimplemented!()
    }
    fn triangulate(&self, max_curveature: f64, max_triangles: u32) -> Vec<Triangle<T>> {
        unimplemented!()
    }
}

impl<T, const N: usize> BezierTriangle<T, N>
where
    T: Copy + Add<T> + Sub<T> + Mul,
    [(); triangular_number(N + 1)]:,
{
    fn new(points: [T; triangular_number(N + 1)], colors: [Color; 3]) -> Self {
        BezierTriangle { points: points, colors: colors}
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

impl<const N: usize, const M: usize> FromString for BezierRectangle<Vec3, N, M> where
    [(); (N + 1) * (M + 1)]:,
{
    fn from_string(lines: &str) -> Result<BezierRectangle<Vec3,N,M>, String> {
        let num_ctrl_pts: usize = (N+1)*(M+1);
        let mut points = [Vec3::new(0., 0., 0.); (N+1)*(M+1)];
        let mut colors = [Color::new(0., 0., 0.); 4];
        let mut index = 0;
        for line in lines.lines() {
            let mut value = Vec3::new(0., 0., 0.);
            let mut line_iter = line.split_whitespace();
            if let Some(x) = line_iter.next() {
                value.x = x.parse::<f64>().unwrap();
            } else {
                return Err(format!("missing vector element"))
            }
            if let Some(y) = line_iter.next() {
                value.y = y.parse::<f64>().unwrap();
            } else {
                return Err(format!("missing vector element"))
            }
            if let Some(z) = line_iter.next() {
                value.z = z.parse::<f64>().unwrap();
            } else {
                return Err(format!("missing vector element"))
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
            return Err(err_message)
        }
        Ok(BezierRectangle::<Vec3,N,M>::new(points, colors))
    }
}

fn main() {
    let arg = std::env::args().nth(1);
    if let Some(num) = arg {
        let x: usize = num.parse().unwrap();
    }

    let x = Color::new(1., 2., 3.);
    let four_colors = [Color::new(1.,0.,0.), Color::new(0.,1.,0.), Color::new(0.,0.,1.), Color::new(0.,1.,1.)];
    let three_colors = [Color::new(1.,0.,0.), Color::new(0.,1.,0.), Color::new(0.,0.,1.)];

    let pts = [1, 2, 3, 4];
    let pts2 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let pts3 = [1, 2, 3, 4, 5, 6];
    let b = BezierCurve::<_, 3>::new(pts);
    let r = BezierRectangle::<_, 2, 3>::new(pts2, four_colors);
    let t = BezierTriangle::<_, 2>::new(pts3, three_colors);

    let v1 = Vec3::new(1., 2., 3.);
    let v2 = Vec3::new(1., 4., 3.);
    let v3 = Vec3::new(3., 2., 3.);
    let pts4 = [v1, v2, v3];
    let b2 = BezierCurve::<_, 2>::new(pts4);

    let b3 = BezierCurve::<_, 0>::new([3]);

    let test = false;

    const N: usize = 3;
    let p = [1; N + 1];
    let b4 = BezierCurve::<_, N>::new(p);

    let example_cbez333 = 
        "0 0 0
        1. 2. 3.
        1 3. -2
        2. .1 -.3
        1. 2. 3.
        1. 2. 3.
        1. 2. 3.
        1. 2. 3.
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

    let b5 = BezierRectangle::<Vec3,3,3>::from_string(example_cbez333);
}
