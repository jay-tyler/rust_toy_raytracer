pub mod rays;
pub mod vectors;
pub mod hitable;
pub mod sphere;

#[cfg(test)]
mod test_vectors {
//    crate ray_tracer;
    use vectors;
    #[test]
    fn test_vector_fields() {
        // Stupid sanity test
        let v1 = vectors::ThreeVector(1.,2.,3.);
        assert_eq!(v1.0, 1.);
    }

    #[test]
    fn test_add_vectors() {
        let v1 = vectors::ThreeVector(1., 2.,3.);
        let v2 = vectors::ThreeVector(5., 7., 8.);
        let vsum = v1 + v2;
        assert_eq!(vsum.0, 6.);
        assert_eq!(vsum.1, 9.);
        assert_eq!(vsum.2, 11.);
        println!("{:?}{:?}", v1, v2);
    }

    #[test]
    fn test_sub_vectors_vector() {
        let v1 = vectors::ThreeVector(1., 2.,3.);
        let v2 = vectors::ThreeVector(5., 7., 8.);
        let vdiff = v2 - v1;
        assert_eq!(vdiff.0, 4.);
        assert_eq!(vdiff.1, 5.);
        assert_eq!(vdiff.2, 5.);
    }

    #[test]
    fn test_sub_vectors_f64() {
        let v1 = vectors::ThreeVector(1., 2.,3.);
        let one_and_half = 1.5_f64;
        let vdiff = v1 - one_and_half;
//        let expected = vectors::ThreeVector(-0.5, 0.5, 1.5);
        assert_eq!(vdiff.0, -0.5);
        assert_eq!(vdiff.1, 0.5);
        assert_eq!(vdiff.2, 1.5);
    }

    #[test]
    fn test_dot_vectors() {
        let v1 = vectors::ThreeVector(1., 2.,3.);
        let v2 = vectors::ThreeVector(5., 7., 8.);
        let vdot = v1.dot_product(&v2);
        // TODO: figure out why &v1.dot_product(&v2) gives back a reference

        assert_eq!(vdot, 43. as f64);
    }

    #[test]
    fn test_cross_vectors() {
        let v1 = vectors::ThreeVector(2., 3., 4.);
        let v2 = vectors::ThreeVector(5., 6., 7.);
        let vcross = v1.cross_product(&v2);
        let expected = vectors::ThreeVector(-3., -6., -3.);

        assert_eq!(vcross, expected);
    }

    #[test]
    fn test_magnitude_vectors() {
        let v = vectors::ThreeVector(4., -2., 3.);
        let expected= 29_f64.sqrt();

        assert_eq!(v.magnitude(), expected);
    }

    #[test]
    fn test_multiply_vectors() {
        let v1 = vectors::ThreeVector(2., 3., 4.);
        let v2 = v1 * 3_f64;
        let expected = vectors::ThreeVector(6., 9., 12.);

        assert_eq!(v2, expected);
    }

    #[test]
    fn test_divide_vectors() {
        let v1 = vectors::ThreeVector(2., 3., 4.);
        let v2 = v1 / 2_f64;
        let expected = vectors::ThreeVector(1., 1.5, 2.);

        assert_eq!(v2, expected);
    }

    #[test]
    fn test_unit_vector() {
        let v = vectors::ThreeVector(4., -2., 3.);
        let vunit = v.as_unit_vector();
        let expected = vectors::ThreeVector(4., -2., 3.) * (1_f64/29_f64.sqrt());

        assert_eq!(vunit, expected);
    }
}


#[cfg(test)]
mod test_rays {
    use rays;
    use vectors;

    #[test]
    fn test_ray_fields() {
        let ray = rays::Ray{
            origin: vectors::ThreeVector(1., 1., 1.),
            direction: vectors::ThreeVector(3., 2., 1.)
        };

        let expected_origin = vectors::ThreeVector(1., 1., 1.);
        let expected_direction = vectors::ThreeVector(3., 2., 1.);

        assert_eq!(ray.origin, expected_origin);
        assert_eq!(ray.direction, expected_direction);
    }

    #[test]
    fn test_point_at_parameter() {
        let ray = rays::Ray{
            origin: vectors::ThreeVector(1., 1., 1.),
            direction: vectors::ThreeVector(3., 2., 1.)
        };

        let zero_travel = ray.point_at_parameter(0.);
        let zero_expected = vectors::ThreeVector(1., 1., 1.);

        let one_travel = ray.point_at_parameter(1.);
        let one_expected = vectors::ThreeVector(4., 3., 2.);

        let three_travel = ray.point_at_parameter(3.);
        let three_expected = vectors::ThreeVector(10., 7., 4.);

        assert_eq!(zero_travel, zero_expected);
        assert_eq!(one_travel, one_expected);
        assert_eq!(three_travel, three_expected);
    }
}

#[cfg(test)]
mod test_hitable {
    use hitable;
    use vectors;

    #[test]
    fn test_fields() {
        let h = hitable::HitRecord{
            t: 4.5_f64,
            p: vectors::ThreeVector(4., 5., 6. ),
            normal: vectors::ThreeVector(7., 8., 9.)
        };
        let p_expected = vectors::ThreeVector(4., 5., 6.);
        let n_expected = vectors::ThreeVector(7., 8., 9.);
        assert_eq!(h.t, 4.5_f64);
        assert_eq!(h.p, p_expected);
        assert_eq!(h.normal, n_expected);
    }
}

