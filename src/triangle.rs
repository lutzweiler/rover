use std::ops::{Add, Mul, Sub};

pub struct Triangle<T>
where
    T: Copy + Add<T> + Sub<T> + Mul,
{
    v1: T,
    v2: T,
    v3: T,
}
