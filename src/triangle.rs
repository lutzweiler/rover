use bevy::math::f64::DVec3 as Vec3;
use std::ops::{Add, Mul};
use Vec3 as Color;

pub struct Triangle<T>
where
    T: Copy + Add<T, Output = T> + Mul<f64, Output = T>,
{
    points: [T; 3],
    colors: [Color; 3],
    normals: [T; 3],
}

impl<T> Triangle<T>
where
    T: Copy + Add<T, Output = T> + Mul<f64, Output = T>,
{
    pub fn new(p: [T; 3], c: [Color; 3], n: [T; 3]) -> Self {
        Triangle {
            points: p,
            colors: c,
            normals: n,
        }
    }
}
