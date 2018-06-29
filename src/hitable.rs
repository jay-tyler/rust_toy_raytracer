use vectors::ThreeVector;
use rays::Ray;
use std::vec;
use sphere::Sphere;
use std::f64;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: ThreeVector,
    pub normal: ThreeVector,
}

pub trait Hitable {
    fn hit(&self, r: &Ray,
           t_min: f64, t_max: f64,
           rec: &mut HitRecord) -> bool;
}

pub struct HitableList {
    pub list: vec::Vec<Sphere>
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray,
           t_min: f64, t_max: f64,
           rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord{
            t: f64::MAX,
            p: ThreeVector(f64::NAN,f64::NAN,f64::NAN), //TODO: OK?
            normal: ThreeVector(f64::NAN,f64::NAN,f64::NAN), //TODO: OK?
        };
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for sphere in &self.list {
            if sphere.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        //let temp_rec = HitRecord{}  //TODO how to initialize?
        return hit_anything;
    }
}