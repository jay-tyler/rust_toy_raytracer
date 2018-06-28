extern crate ray_tracer;
use ray_tracer::vectors::ThreeVector;
use ray_tracer::rays::Ray;

fn hit_sphere(center: &ThreeVector,
              radius: f64,
              r: &ray_tracer::rays::Ray) -> f64 {
    let oc = r.origin - *center;
    let a = r.direction.dot_product(&r.direction);
    let mut b = oc.dot_product(&r.direction);
    b = b * 2.;
    let c = oc.dot_product(&oc) - radius * radius;
    let discriminant = b * b - a * c * 4_f64;
    if (discriminant < 0_f64) {
        return -1.;
    } else {
        return (-b - discriminant.sqrt()) / (a * 2.0_f64);
    }
}


fn color(r: &ray_tracer::rays::Ray) -> ray_tracer::vectors::ThreeVector {
    let t = hit_sphere(&ThreeVector(0., 0., -1.), 0.5, &r);
    if t > 0.0_f64 {
        let n = r.point_at_parameter(t) - ThreeVector(0., 0., -1.);
        return ThreeVector(n.0 + 1.,
                           n.1 + 1.,
                           n.2 + 1.) * 0.5_f64;
    }
    let unit_vector = ray_tracer::vectors::ThreeVector(1., 1., 1.);
    let unit_direction = r.direction.as_unit_vector();
    let v_whatever = ray_tracer::vectors::ThreeVector(0.5, 0.7, 1.0);
    let t = 0.5 * unit_direction.1 + 1.0;

    unit_vector * (1.0 - t) + v_whatever * t
}



fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);

    let lower_left_corner = ray_tracer::vectors::ThreeVector(-2.,-1., -1.);
    let horizontal = ray_tracer::vectors::ThreeVector(4., 0., 0.);
    let vertical = ray_tracer::vectors::ThreeVector(0.,2., 0.);
    let origin = ray_tracer::vectors::ThreeVector(0.,0.,0.);

    for j in (0..ny-1).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;
            let r = ray_tracer::rays::Ray {
                origin: origin,
                direction: lower_left_corner +
                           horizontal * u +
                           vertical * v
            };
            let mut col = color(&r);
            col = col * 255_f64;
            println!("{} {} {}", col.0 as u8, col.1 as u8, col.2 as u8);
        }
    }
}
