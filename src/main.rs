#![feature(generic_const_exprs)]
#![allow(dead_code)]
#![allow(unused_variables)]

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

mod bezier;
mod math;
mod triangle;

//use crate::bezier::*;
//use bevy::math::f64::DVec3 as Vec3;
//use Vec3 as Color;

fn main() {
}

#[cfg(test)]
mod tests{
}