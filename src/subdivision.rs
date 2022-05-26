use crate::triangle::{ToTriangle, Triangle};
use crate::bezier::rectangle::BezierRectangle;
use bevy::prelude::Vec3;
use crate::math;

pub trait Subdivide {
    fn subdivide(&self) -> Vec<Self> where Self: Sized;
}

pub struct SubdivisionSet<T>
    where T: Subdivide
{
    pub elements: Vec<T>,
}

impl<T> SubdivisionSet<T>
    where T: Subdivide 
{
    pub fn new() -> Self {
        SubdivisionSet {
            elements: Vec::<T>::new(),
        }
    }

    pub fn subdivide(&mut self) {
        let limit = 1000; //finishes one subdivision run after limit is reached, possibly much larger
        while self.elements.len() < limit {
            let mut new_elements = Vec::<T>::new();
            for e in &self.elements {
                let subdivided = &mut e.subdivide();
                new_elements.append(subdivided);
            }
            self.elements = new_elements;
        }
   }
}

//what we really want to do is implement ToTriangle for a type that has
//a vector part Vec3, but that would require additional generic parameter for these types
//this means this block needs to be copied for BezierTriangles and BezierCurves
impl<const N: usize, const M:usize> ToTriangle for SubdivisionSet<BezierRectangle<Vec3,N,M>>
where
    [(); (N + 1) * (M + 1)]:,
        [(); math::triangular_number(N + 1)]:,
        [(); math::triangular_number(M + 1)]:,
{
    fn to_triangles(&self) -> Vec<Triangle<Vec3>> {
        let mut triangles = Vec::<Triangle<Vec3>>::new();
        for e in &self.elements {
            triangles.append(&mut e.to_triangles());
        }
        triangles
    }
}