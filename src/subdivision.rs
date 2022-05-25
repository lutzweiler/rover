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
        let limit = 5000;
        while self.elements.len() < limit {
            let mut new_elements = Vec::<T>::new();
            for e in &self.elements {
                if self.elements.len() < limit { break; }
                new_elements.append(&mut e.subdivide());
            }
            self.elements = new_elements;
        }
   }
}