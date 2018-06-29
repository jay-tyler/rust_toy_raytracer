use vectors::ThreeVector;
use rays::Ray;

pub struct Camera {
    pub lower_left_corner: ThreeVector,
    pub horizontal: ThreeVector,
    pub vertical: ThreeVector,
    pub origin: ThreeVector,
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        return Ray{
            origin: self.origin,
            direction: self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        };
    }
}