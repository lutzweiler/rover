use bevy::prelude::Vec3;
use std::ops::{Add, Mul};
use Vec3 as Color;

use crate::math;
use crate::subdivision::Subdivide;
use crate::triangle::{ToTriangle, Triangle};

pub trait FromString {
    fn from_string(lines: &str) -> Result<Self, String>
    where
        Self: Sized;
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
    T: Copy + Add<T, Output = T> + Mul<f32, Output = T>,
    [(); (N + 1) * (M + 1)]:,
{
    points: [T; (N + 1) * (M + 1)],
    colors: [Color; 4],
}

impl<T, const N: usize, const M: usize> BezierRectangle<T, N, M>
where
    T: Copy + Add<T, Output = T> + Mul<f32, Output = T> + std::fmt::Debug,
    [(); (N + 1) * (M + 1)]:,
{
    pub fn new(points: [T; (N + 1) * (M + 1)], colors: [Color; 4]) -> Self {
        BezierRectangle {
            points: points,
            colors: colors,
        }
    }

    fn evaluate(&self, u: f32, v: f32) -> T {
        unimplemented!()
    }

    pub fn subdivide_cross(&self) -> Vec<Self>
    where
        [(); math::triangular_number(N + 1)]:,
        [(); math::triangular_number(M + 1)]:,
    {
        let (l, r) = self.subdivide(math::Axis2D::U, 0.5);
        let (tl, bl) = l.subdivide(math::Axis2D::V, 0.5);
        let (tr, br) = r.subdivide(math::Axis2D::V, 0.5);

        vec![tl, bl, tr, br]
    }

    pub fn subdivide(&self, axis: math::Axis2D, t: f32) -> (Self, Self)
    where
        [(); math::triangular_number(N + 1)]:,
        [(); math::triangular_number(M + 1)]:,
    {
        match axis {
            math::Axis2D::U => self.subdivide_u(t),
            math::Axis2D::V => self.subdivide_v(t),
        }
    }

    fn triangulate(&self, max_curveature: f32, max_triangles: u32) -> Vec<Triangle<T>> {
        unimplemented!()
    }

    fn subdivide_u(&self, t: f32) -> (Self, Self)
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
        let new_color_top = math::lerp(self.colors[0], self.colors[2], t);
        let new_color_bot = math::lerp(self.colors[1], self.colors[3], t);
        colors_left[0] = self.colors[0];
        colors_left[1] = self.colors[1];
        colors_left[2] = new_color_top;
        colors_left[3] = new_color_bot;
        colors_right[0] = new_color_top;
        colors_right[1] = new_color_bot;
        colors_right[2] = self.colors[2];
        colors_right[3] = self.colors[3];

        //assemble everything into new patches
        (
            BezierRectangle::<T, N, M>::new(left, colors_left),
            BezierRectangle::<T, N, M>::new(right, colors_right),
        )
    }

    fn subdivide_v(&self, t: f32) -> (Self, Self)
    where
        [(); math::triangular_number(M + 1)]:,
    {
        //control points for the new surfaces
        let mut top = [self.points[0]; (N + 1) * (M + 1)];
        let mut bot = [self.points[0]; (N + 1) * (M + 1)];

        //calculate the control points
        for i in 0..N + 1 {
            //take each column seperately
            let mut col = [self.points[0]; M + 1];
            for j in 0..M + 1 {
                col[j] = self.points[j * (N + 1) + i];
            }
            let col = col;

            //compute new values
            let res = math::compute_triangular_scheme::<T, { M + 1 }>(&col, t);

            //put the new values in the right place
            let mut row_offset = 0;
            let mut row_len = M + 1;
            for j in 0..M + 1 {
                top[j * (N + 1) + i] = res[row_offset + 0]; //first element of the row put in left in forward order
                bot[(M - j) * (N + 1) + i] = res[row_offset + row_len - 1]; //last element of the row put in right in reverse order
                row_offset += row_len;
                row_len -= 1;
            }
        }

        //calculate the new colors
        let mut colors_top = [Color::new(0., 0., 0.); 4];
        let mut colors_bot = [Color::new(0., 0., 0.); 4];
        let new_color_left = math::lerp(self.colors[0], self.colors[1], t);
        let new_color_right = math::lerp(self.colors[2], self.colors[3], t);
        colors_top[0] = self.colors[0];
        colors_top[1] = new_color_left;
        colors_top[2] = self.colors[2];
        colors_top[3] = new_color_right;
        colors_bot[0] = new_color_left;
        colors_bot[1] = self.colors[1];
        colors_bot[2] = new_color_right;
        colors_bot[3] = self.colors[3];

        //assemble everything into new patches
        (
            BezierRectangle::<T, N, M>::new(top, colors_top),
            BezierRectangle::<T, N, M>::new(bot, colors_bot),
        )
    }
}

impl<const N: usize, const M: usize> BezierRectangle<Vec3, N, M>
where
    [(); (N + 1) * (M + 1)]:,
{
    #[allow(unused_parens)]
    fn corner_normals(&self) -> (Vec3, Vec3, Vec3, Vec3) {
        let b00u = (self.points[0 * (N + 1) + 1] - self.points[0 * (N + 1) + 0]);
        let b00v = (self.points[1 * (N + 1) + 0] - self.points[0 * (N + 1) + 0]);
        let b10u = -(self.points[0 * (N + 1) + N - 1] - self.points[0 * (N + 1) + N]);
        let b10v = (self.points[1 * (N + 1) + N] - self.points[0 * (N + 1) + N]);
        let b01u = (self.points[M * (N + 1) + 1] - self.points[M * (N + 1) + 0]);
        let b01v = -(self.points[(M - 1) * (N + 1) + 0] - self.points[M * (N + 1) + 0]);
        let b11u = -(self.points[M * (N + 1) + N - 1] - self.points[M * (N + 1) + N]);
        let b11v = -(self.points[(M - 1) * (N + 1) + N] - self.points[M * (N + 1) + N]);
        let n00 = b00u.cross(b00v).normalize_or_zero();
        let n10 = b10u.cross(b10v).normalize_or_zero();
        let n01 = b01u.cross(b01v).normalize_or_zero();
        let n11 = b11u.cross(b11v).normalize_or_zero();
        (n00, n10, n01, n11)
    }
}

impl<const N: usize, const M: usize> ToTriangle for BezierRectangle<Vec3, N, M>
where
    [(); (N + 1) * (M + 1)]:,
{
    fn to_triangles(&self) -> Vec<Triangle<Vec3>> {
        let v00 = self.points[0];
        let v01 = self.points[N];
        let v10 = self.points[M * (N + 1)];
        let v11 = self.points[M * (N + 1) + M];
        let (n0, n1, n2, n3) = self.corner_normals();
        let t1 = Triangle::new(
            [v00, v10, v01],
            [self.colors[0], self.colors[1], self.colors[2]],
            [n0, n2, n1],
        );
        let t2 = Triangle::new(
            [v10, v11, v01],
            [self.colors[1], self.colors[3], self.colors[2]],
            [n2, n3, n1],
        );
        vec![t1, t2]
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
                value.x = x.parse::<f32>().unwrap();
            } else {
                return Err(format!("missing vector element"));
            }
            if let Some(y) = line_iter.next() {
                value.y = y.parse::<f32>().unwrap();
            } else {
                return Err(format!("missing vector element"));
            }
            if let Some(z) = line_iter.next() {
                value.z = z.parse::<f32>().unwrap();
            } else {
                return Err(format!("missing vector element"));
            }
            if index < num_ctrl_pts {
                points[index] = value;
                //println!("point {}", value);
            } else if index < num_ctrl_pts + 4 {
                colors[index - num_ctrl_pts] = value;
                //println!("color {}", value);
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

impl<T, const N: usize, const M: usize> Subdivide for BezierRectangle<T, N, M>
where
    T: Copy + Add<T, Output = T> + Mul<f32, Output = T> + std::fmt::Debug,
    [(); (N + 1) * (M + 1)]:,
    [(); math::triangular_number(N + 1)]:,
    [(); math::triangular_number(M + 1)]:,
{
    fn subdivide(&self) -> Vec<Self> {
        self.subdivide_cross()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    fn example_bezier_rectangle() -> BezierRectangle<f32, 3, 2> {
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

    fn example_bezier_rectangle_2() -> BezierRectangle<f32, 3, 3> {
        let pts = [4., 0., 4., 0., 4., 0., 4., 4., 4., 0., 0., 4., 2., 2., 0., 0.];
        let four_colors = [
            Color::new(1., 0., 0.),
            Color::new(0., 1., 0.),
            Color::new(0., 0., 1.),
            Color::new(1., 1., 1.),
        ];
        let r = BezierRectangle::<_, 3, 3>::new(pts, four_colors);
        r
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
    fn bezier_rectangle_subdivide_u() {
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
        assert_eq!(l.colors[3], Color::new(0.5, 0.5, 1.));
        assert_eq!(r.colors[0], Color::new(0.5, 0.5, 0.));
        assert_eq!(r.colors[2], Color::new(0.5, 0.5, 1.));

        //TODO: make a test for evaluating l(.5, .5) == surf(.25, .5) once evaluation is implemented
    }

    #[test]
    fn bezier_rectangle_subdivide_v() {
        let surf = example_bezier_rectangle();
        let (t, b) = surf.subdivide(math::Axis2D::V, 0.5);
        println!("top {:?}", t);
        println!("bottom {:?}", b);

        //points at the split line match
        assert_eq!(t.points[8 + 0], b.points[0 + 0]);
        assert_eq!(t.points[8 + 1], b.points[0 + 1]);
        assert_eq!(t.points[8 + 2], b.points[0 + 2]);
        assert_eq!(t.points[8 + 3], b.points[0 + 3]);

        //points on top and bottom border match with original
        assert_eq!(t.points[0 + 0], surf.points[0 + 0]);
        assert_eq!(t.points[0 + 1], surf.points[0 + 1]);
        assert_eq!(t.points[0 + 2], surf.points[0 + 2]);
        assert_eq!(t.points[0 + 3], surf.points[0 + 3]);
        assert_eq!(b.points[8 + 0], surf.points[8 + 0]);
        assert_eq!(b.points[8 + 1], surf.points[8 + 1]);
        assert_eq!(b.points[8 + 2], surf.points[8 + 2]);
        assert_eq!(b.points[8 + 3], surf.points[8 + 3]);

        //test colors
        assert_eq!(t.colors[2], Color::new(0.5, 0.0, 0.5));
        assert_eq!(t.colors[3], Color::new(0.5, 1.0, 0.5));
        assert_eq!(b.colors[0], Color::new(0.5, 0.0, 0.5));
        assert_eq!(b.colors[1], Color::new(0.5, 1.0, 0.5));

        //TODO: make a test for evaluating t(.5, .5) == surf(.5, .25) once evaluation is implemented
    }

    #[test]
    fn bezier_rectangle_subdivide() {
        //take a surface and divide it into left and right
        //rotate the original surface and divide it into top and bottom
        //rotate top and bottom back
        //now left == top and right == bottom
        let surf = example_bezier_rectangle_2();
        let (l, r) = surf.subdivide(math::Axis2D::U, 0.5);
        let mut transposed_points = [0.; 16];
        for i in 0..4 {
            for j in 0..4 {
                transposed_points[i * 4 + j] = surf.points[j * 4 + i];
            }
        }
        let transposed_points = transposed_points;
        let transposed = BezierRectangle::<f32, 3, 3>::new(transposed_points, surf.colors);
        let (t, b) = transposed.subdivide(math::Axis2D::V, 0.5);
        let mut top_retransposed = [0.; 16];
        let mut bot_retransposed = [0.; 16];
        for i in 0..4 {
            for j in 0..4 {
                top_retransposed[i * 4 + j] = t.points[j * 4 + i];
                bot_retransposed[i * 4 + j] = b.points[j * 4 + i];
            }
        }
        assert_eq!(l.points, top_retransposed);
        assert_eq!(r.points, bot_retransposed);
    }

    #[test]
    fn bezier_rectangle_double_subdivide() {
        //the order of subsequent U- and V-subdivisions does not matter
        let surf = example_bezier_rectangle_2();
        let (al, ar) = surf.subdivide(math::Axis2D::U, 0.5);
        let (atl, abl) = al.subdivide(math::Axis2D::V, 0.5);
        let (atr, abr) = ar.subdivide(math::Axis2D::V, 0.5);

        let (bt, bb) = surf.subdivide(math::Axis2D::V, 0.5);
        let (btl, btr) = bt.subdivide(math::Axis2D::U, 0.5);
        let (bbl, bbr) = bb.subdivide(math::Axis2D::U, 0.5);

        assert_eq!(atl.points, btl.points);
        assert_eq!(atr.points, btr.points);
        assert_eq!(abl.points, bbl.points);
        assert_eq!(abr.points, bbr.points);
        assert_eq!(atl.colors, btl.colors);
        assert_eq!(atr.colors, btr.colors);
        assert_eq!(abl.colors, bbl.colors);
        assert_eq!(abr.colors, bbr.colors);
    }

    #[test]
    //use cargo test -- --nocapture to see performance numbers
    fn bezier_rectangle_subdivide_performance() {
        let cbez333 = "0. 0. 0.
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
        let surf = BezierRectangle::<Vec3, 3, 3>::from_string(cbez333).unwrap();
        let mut old_vec = vec![surf];
        let mut new_vec = Vec::<BezierRectangle<Vec3, 3, 3>>::new();
        for i in 0..9 {
            let now = Instant::now();
            for s in &old_vec {
                let (l, r) = s.subdivide(math::Axis2D::U, 0.5);
                let (tl, bl) = l.subdivide(math::Axis2D::V, 0.5);
                let (tr, br) = l.subdivide(math::Axis2D::V, 0.5);
                new_vec.push(tl);
                new_vec.push(tr);
                new_vec.push(bl);
                new_vec.push(br);
            }
            let len = new_vec.len();
            let nanos = now.elapsed().as_nanos();
            let secs = nanos as f32 / 1_000_000_000f64;
            let per_surf = nanos / len as u128;
            println!(
                "Computing {} surfaces took {} seconds, that is {} ns per surface",
                len, secs, per_surf
            );
            old_vec.clear();
            (new_vec, old_vec) = (old_vec, new_vec);
        }
    }
}
