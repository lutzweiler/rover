use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use std::ops::{Add, Mul, Sub};
use Vec3 as Color;

pub trait ToTriangle {
    fn to_triangles(&self) -> Vec<Triangle<Vec3>>;
}

#[derive(Debug)]
pub struct Triangle<T>
where
    T: Copy + Add<T, Output = T> + Sub<T, Output = T> + Mul<f32, Output = T>,
{
    points: [T; 3],
    colors: [Color; 3],
    normals: [T; 3],
}

impl<T> Triangle<T>
where
    T: Copy + Add<T, Output = T> + Sub<T, Output = T> + Mul<f32, Output = T>,
{
    pub fn new_with_normals(p: [T; 3], c: [Color; 3], n: [T; 3]) -> Self {
        Triangle {
            points: p,
            colors: c,
            normals: n,
        }
    }
}

impl Triangle<Vec3> {
    pub fn new(p: [Vec3; 3], c: [Color; 3]) -> Self {
        let n0 = (p[1] - p[0]).cross(p[2] - p[0]).normalize_or_zero();
        let n1 = (p[2] - p[1]).cross(p[0] - p[1]).normalize_or_zero();
        let n2 = (p[0] - p[2]).cross(p[1] - p[2]).normalize_or_zero();
        Triangle {
            points: p,
            colors: c,
            normals: [n0, n1, n2],
        }
    }
}

impl Triangle<Vec3> {
    pub fn from_string(lines: [&String; 3], default_color: Color) -> Result<Self, String> {
        let mut has_color = true;
        for pos in lines {
            if pos.split_whitespace().count() < 6 {
                has_color = false;
                break;
            }
        }
        match has_color {
            true => Triangle::from_string_with_color(lines),
            false => Triangle::from_string_without_color(
                lines,
                [default_color, default_color, default_color]
            ),
        }
    }

    pub fn from_string_without_color(lines: [&String; 3], colors: [Color; 3]) -> Result<Self, String> {
        let mut positions = [Vec3::new(0., 0., 0.); 3];
        let mut i = 0;
        for string in lines.into_iter() {
            let mut value = Vec3::new(0., 0., 0.);
            let mut line_iter = string.split_whitespace();
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
            positions[i] = value;
            i += 1;
        }
        Ok(Triangle::new(positions, colors))
    }

    pub fn from_string_with_color(values: [&String; 3]) -> Result<Self, String> {
        let mut colors = [Color::new(0., 0., 0.); 3];
        let mut i = 0;
        for string in values {
            let mut value = Vec3::new(0., 0., 0.);
            let mut line_iter = string.split_whitespace().skip(3); //skip the positions only handle the colors now
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
            colors[i] = value;
            i += 1;
        }
        Self::from_string_without_color(values, colors)
    }
}

impl Triangle<Vec3> {
    pub fn triangle_list_to_mesh(triangles: Vec<Triangle<Vec3>>) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let mut indices = Vec::<u32>::new();
        let mut positions = Vec::<[f32; 3]>::new();
        let mut normals = Vec::<[f32; 3]>::new();
        let mut colors = Vec::<[f32; 4]>::new();
        let mut uvs = Vec::<[f32; 2]>::new();

        fn vec_to_array(v: Vec3) -> [f32; 3] {
            [v.x, v.y, v.z]
        }

        let mut index = 0;
        for t in &triangles {
            for i in 0..3 {
                indices.push(index);
                positions.push(vec_to_array(t.points[i]));
                normals.push(vec_to_array(t.normals[i]));
                colors.push([t.colors[i].x, t.colors[i].y, t.colors[i].z, 1.]);
                uvs.push([0., 0.]); //no texturing
                index += 1;
            }
        }

        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_triangle_with_colors() {
        let a0 = format!("1.0 3.0 .4 .2 .3 .9");
        let a1 = format!("0 3.0 -.4 .2 0 .9");
        let a2 = format!("-2 3.0 .4 .2 1 .9");
        let t = Triangle::from_string([&a0, &a1, &a2], Color::new(1., 1., 1.)).unwrap();
        assert_eq!(t.points[0], Vec3::new(1., 3., 0.4));
        assert_eq!(t.points[1], Vec3::new(0., 3., -0.4));
        assert_eq!(t.points[2], Vec3::new(-2., 3., 0.4));
        assert_eq!(t.colors[0], Vec3::new(0.2, 0.3, 0.9));
        assert_eq!(t.colors[1], Vec3::new(0.2, 0.0, 0.9));
        assert_eq!(t.colors[2], Vec3::new(0.2, 1., 0.9));
    }

    #[test]
    fn parse_triangle_without_colors() {
        let a0 = format!("1.0 3.0 .4");
        let a1 = format!("0 3.0 -.4");
        let a2 = format!("-2 3.0 .4");
        let t = Triangle::from_string([&a0, &a1, &a2], Color::new(0.8, 0.8, 0.8)).unwrap();
        assert_eq!(t.points[0], Vec3::new(1., 3., 0.4));
        assert_eq!(t.points[1], Vec3::new(0., 3., -0.4));
        assert_eq!(t.points[2], Vec3::new(-2., 3., 0.4));
        assert_eq!(t.colors[0], Vec3::new(0.8, 0.8, 0.8));
        assert_eq!(t.colors[1], Vec3::new(0.8, 0.8, 0.8));
        assert_eq!(t.colors[2], Vec3::new(0.8, 0.8, 0.8));
    }
}
