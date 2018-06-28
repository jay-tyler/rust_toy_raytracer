use vectors;

#[derive(Debug)]
pub struct Ray {
    pub origin: vectors::ThreeVector,
    pub direction: vectors::ThreeVector,
}

impl Ray {
    pub fn point_at_parameter(&self, t: f64) -> vectors::ThreeVector {
        self.origin + (self.direction * t)
    }
}