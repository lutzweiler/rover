pub enum Axis2D {
    X,
    Y,
}

pub enum Axis3D {
    X,
    Y,
    Z,
}

pub const fn triangular_number(n: usize) -> usize {
    n * (n + 1) / 2
}
