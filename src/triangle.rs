use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use std::ops::{Add, Mul};
use Vec3 as Color;

pub trait ToTriangle {
    fn to_triangles(&self) -> Vec<Triangle<Vec3>>;
}

#[derive(Debug)]
pub struct Triangle<T>
where
    T: Copy + Add<T, Output = T> + Mul<f32, Output = T>,
{
    points: [T; 3],
    colors: [Color; 3],
    normals: [T; 3],
}

impl<T> Triangle<T>
where
    T: Copy + Add<T, Output = T> + Mul<f32, Output = T>,
{
    pub fn new(p: [T; 3], c: [Color; 3], n: [T; 3]) -> Self {
        Triangle {
            points: p,
            colors: c,
            normals: n,
        }
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
