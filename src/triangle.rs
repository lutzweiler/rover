use std::ops::{Add, Mul};

pub struct Triangle<T>
where
    T: Copy + Add<T, Output = T> + Mul<f64, Output = T>,
{
    v1: T,
    v2: T,
    v3: T,
}
