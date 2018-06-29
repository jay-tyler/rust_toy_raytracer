extern crate ray_tracer;
extern crate rand;
use ray_tracer::vectors::ThreeVector;
use ray_tracer::sphere::Sphere;
use ray_tracer::hitable::{HitRecord, Hitable, HitableList};
use std::f64;
use ray_tracer::camera::Camera;
use ray_tracer::rays::Ray;

fn color(r: &ray_tracer::rays::Ray, world: &HitableList) -> ray_tracer::vectors::ThreeVector {
    let mut rec =  HitRecord{
        t: f64::MAX,
        p: ThreeVector(f64::NAN,f64::NAN,f64::NAN),
        normal: ThreeVector(f64::NAN,f64::NAN,f64::NAN),
    };
    if world.hit(&r, 0.0, f64::MAX, &mut rec) {
        return ThreeVector(
            rec.normal.0 + 1.,
            rec.normal.1 + 1.,
            rec.normal.2 + 1.,
        ) * 0.5;
    } else {
        let unit_direction = r.direction.as_unit_vector();
        let t: f64 = (unit_direction.1 + 1_f64) * 0.5;
        return ThreeVector(1.0, 1.0, 1.0) * (1.0 - t) +
               ThreeVector(0.5, 0.7, 1.0) * t;
    }
}

fn get_hundred_antialias_samples() -> [(f64, f64); 100] {
    let mut array: [(f64, f64); 100] = [(0., 0.); 100];
    for i in 0..100 {
        array[i].0 = rand::random::<f64>();
        array[i].1 = rand::random::<f64>();
    }
    array
}

fn main() {
    let nx = 200;
    let ny = 100;
    let alias_samples = get_hundred_antialias_samples();
    println!("P3\n{} {}\n255", nx, ny);

    let lower_left_corner = ray_tracer::vectors::ThreeVector(-2.,-1., -1.);
    let horizontal = ray_tracer::vectors::ThreeVector(4., 0., 0.);
    let vertical = ray_tracer::vectors::ThreeVector(0.,2., 0.);
    let origin = ray_tracer::vectors::ThreeVector(0.,0.,0.);

    let world = HitableList{
        list: vec!(
            Sphere{
                center: ThreeVector(0., 0., -1.),
                radius: 0.5,},
            Sphere{
                center: ThreeVector(0., -100.5, -1.),
                radius: 100.,})};
    let camera = Camera {
        lower_left_corner: ThreeVector(-2., -1., -1.),
        horizontal: ThreeVector(4., 0., 0.),
        vertical: ThreeVector(0., 2., 0.),
        origin: ThreeVector(0., 0., 0.),
    };
    for j in (0..ny-1).rev() {
        for i in 0..nx {
            let mut col = ThreeVector(0., 0., 0.);
            for (mut u, mut v) in alias_samples.iter() {
                    u = (i as f64 + u) / nx as f64;
                    v = (j as f64 + v) / ny as f64;
                    let r = camera.get_ray(u, v);
                    let p = r.point_at_parameter(2.0);
                    col = col + color(&r, &world);
                }
                col = col / 100_f64;  // TODO: remove connascense with samples?
                col = col * 255_f64;
                println!("{} {} {}", col.0 as u8, col.1 as u8, col.2 as u8);
        }
    }
}
