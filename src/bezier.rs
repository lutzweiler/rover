use bevy::math::f64::DVec3 as Vec3;
use std::ops::{Add, Mul};
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
    T: Copy + Add<T, Output = T> + Mul<f64, Output = T>,
    [(); N + 1]:,
{
    points: [T; N + 1],
}

/*
    A rectangular bezier patch of degree N in u direction and degree M in v direction
    control point b_ij is stored at points[j*(N+1) + i]
    the colors and parameters are matched as such:
        at (u,v) == (0,0) interpolates point b_00 and has color colors[0]
        at (u,v) == (1,0) interpolates point b_N0 and has color colors[1]
        at (u,v) == (0,1) interpolates point b_0M and has color colors[2]
        at (u,v) == (1,1) interpolates point b_NM and has color colors[3]
    In general it is easiest to imagine the following configuration:

        (0,0) -- u -- (1,0)               b_00 b_10 b_20 --- b_N0
          |             |                 b_01                |                    c[0] - c[1]
          v             |      maps to    b_02                |      with colors    |      |
          |             |                  |                  |                    c[2] - c[3]
        (0,1) ------- (1,1)               b_0M b_1M b_2M --- b_NM
*/
#[derive(Debug)]
pub struct BezierRectangle<T, const N: usize, const M: usize>
where
    T: Copy + Add<T, Output = T> + Mul<f64, Output = T>,
    [(); (N + 1) * (M + 1)]:,
{
    points: [T; (N + 1) * (M + 1)],
    colors: [Color; 4],
}

pub struct BezierTriangle<T, const N: usize>
where
    T: Copy + Add<T, Output = T> + Mul<f64, Output = T>,
    [(); math::triangular_number(N + 1)]:,
{
    points: [T; math::triangular_number(N + 1)],
    colors: [Color; 3],
}

impl<T, const N: usize> BezierCurve<T, N>
where
    T: Copy + Add<T, Output = T> + Mul<f64, Output = T>,
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
    T: Copy + Add<T, Output = T> + Mul<f64, Output = T> + std::fmt::Debug,
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

    fn subdivide(&self, axis: math::Axis2D, t: f64) -> (Self, Self)
    where
        [(); math::triangular_number(N + 1)]:,
        [(); math::triangular_number(M + 1)]:,
    {
        match axis {
            math::Axis2D::U => self.subdivide_u(t),
            math::Axis2D::V => self.subdivide_v(t),
        }
    }

    fn triangulate(&self, max_curveature: f64, max_triangles: u32) -> Vec<Triangle<T>> {
        unimplemented!()
    }

    fn subdivide_u(&self, t: f64) -> (Self, Self)
    where
        [(); math::triangular_number(N + 1)]:,
    {
        //control points for the new surfaces
        let mut left = [self.points[0]; (N + 1) * (M + 1)];
        let mut right = [self.points[0]; (N + 1) * (M + 1)];

        //calculate the control points
        for j in 0..M + 1 {
            //take each row seperately
            let mut row = [self.points[0]; N + 1];
            row[..].copy_from_slice(&self.points[j * (N + 1)..(j + 1) * (N + 1)]);
            let row = row;
            println!("{:?}", row);

            //compute new values
            let res = math::compute_triangular_scheme::<T, { N + 1 }>(&row, t);

            //put the new values in the right place
            let mut row_offset = 0;
            let mut row_len = N + 1;
            for i in 0..N + 1 {
                left[j * (N + 1) + i] = res[row_offset + 0]; //first element of the row put in left in forward order
                right[(j + 1) * (N + 1) - (i + 1)] = res[row_offset + row_len - 1]; //last element of the row put in right in reverse order
                row_offset += row_len;
                row_len -= 1;
            }
        }

        //calculate the new colors
        let mut colors_left = [Color::new(0., 0., 0.); 4];
        let mut colors_right = [Color::new(0., 0., 0.); 4];
        colors_left[0] = self.colors[0];
        colors_right[1] = self.colors[1];
        colors_right[2] = self.colors[2];
        colors_left[3] = self.colors[3];
        let new_color_top = math::lerp(self.colors[0], self.colors[1], t);
        let new_color_bot = math::lerp(self.colors[3], self.colors[2], t);
        colors_left[1] = new_color_top;
        colors_left[2] = new_color_bot;
        colors_right[0] = new_color_top;
        colors_right[3] = new_color_bot;

        //assemble everything into new patches
        (
            BezierRectangle::<T, N, M>::new(left, colors_left),
            BezierRectangle::<T, N, M>::new(right, colors_right),
        )
    }

    fn subdivide_v(&self, t: f64) -> (Self, Self) {
        unimplemented!()
    }
}

impl<T, const N: usize> BezierTriangle<T, N>
where
    T: Copy + Add<T, Output = T> + Mul<f64, Output = T>,
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
    pub fn subdivide(&self, u: f64, v: f64) -> (Self, Self) {
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
mod tests {
    use super::*;

    fn example_bezier_rectangle() -> BezierRectangle<f64, 3, 2> {
        let pts = [4., 0., 4., 0., 4., 0., 4., 4., 4., 0., 0., 4.];
        let four_colors = [
            Color::new(1., 0., 0.),
            Color::new(0., 1., 0.),
            Color::new(0., 0., 1.),
            Color::new(1., 1., 1.),
        ];
        let r = BezierRectangle::<_, 3, 2>::new(pts, four_colors);
        r
    }

    #[test]
    fn initialization() {
        let three_colors = [Color::new(1., 0., 0.), Color::new(0., 1., 0.), Color::new(0., 0., 1.)];

        let pts1 = [1., 2., 3., 4.];
        let pts3 = [1., 2., 3., 4., 5., 6.];
        let b = BezierCurve::<_, 3>::new(pts1);
        let t = BezierTriangle::<_, 2>::new(pts3, three_colors);

        let v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(1., 4., 3.);
        let v3 = Vec3::new(3., 2., 3.);
        let pts4 = [v1, v2, v3];
        let b2 = BezierCurve::<_, 2>::new(pts4);
        let b3 = BezierCurve::<_, 0>::new([3.]);
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
            }
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
            }
            Err(e) => {
                println!("{}", e);
                assert!(false);
            }
        }
    }

    #[test]
    fn rectangular_bezier_surface_subdivide_u() {
        let surf = example_bezier_rectangle();
        let (l, r) = surf.subdivide(math::Axis2D::U, 0.5);
        println!("left {:?}", l);
        println!("right {:?}", r);

        //points at the split line match
        assert_eq!(l.points[0 + 3], r.points[0 + 0]);
        assert_eq!(l.points[4 + 3], r.points[4 + 0]);
        assert_eq!(l.points[8 + 3], r.points[8 + 0]);

        //points on left and right border match with original
        assert_eq!(l.points[0 + 0], surf.points[0 + 0]);
        assert_eq!(l.points[4 + 0], surf.points[4 + 0]);
        assert_eq!(l.points[8 + 0], surf.points[8 + 0]);
        assert_eq!(r.points[0 + 3], surf.points[0 + 3]);
        assert_eq!(r.points[4 + 3], surf.points[4 + 3]);
        assert_eq!(r.points[8 + 3], surf.points[8 + 3]);

        //test colors
        assert_eq!(l.colors[1], Color::new(0.5, 0.5, 0.));
        assert_eq!(l.colors[2], Color::new(0.5, 0.5, 1.));
        assert_eq!(r.colors[0], Color::new(0.5, 0.5, 0.));
        assert_eq!(r.colors[3], Color::new(0.5, 0.5, 1.));

        //TODO: make a test for evaluating l(.5, .5) == surf(.25, .5) once evaluation is implemented
    }

    //#[test]
    fn bezier_rectangle_subdivide() {
        //subdiv u B = subdiv v B transposed
    }
}
