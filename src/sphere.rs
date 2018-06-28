use vectors::ThreeVector;
use hitable::{Hitable, HitRecord};
use rays::Ray;

pub struct Sphere {
    pub center: ThreeVector,
    pub radius: f64,
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray,
           t_min: f64, t_max: f64,
           rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.dot_product(&r.direction);
        let b = oc.dot_product(&r.direction);
        let c = oc.dot_product(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0_f64 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                // TODO: way to prevent code duplication?
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
        }
        false
    }
}

