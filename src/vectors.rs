use std::ops::{Add, Sub, Div, Mul};
//use std::cmp::PartialEq;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ThreeVector(pub f64, pub f64, pub f64);

impl Add for ThreeVector {
    type Output = ThreeVector;

    fn add(self, other: ThreeVector) -> ThreeVector {
        ThreeVector(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2
        )
    }
}

impl Sub<ThreeVector> for ThreeVector {
    type Output = ThreeVector;

    fn sub(self, other: ThreeVector) -> ThreeVector {
        ThreeVector(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
        )
    }
}

impl Sub<f64> for ThreeVector {
    type Output = ThreeVector;

    fn sub(self, other: f64) -> ThreeVector {
        ThreeVector(
            self.0 - other,
            self.1 - other,
            self.2 - other,
        )
    }
}

impl Div<f64> for ThreeVector {
    type Output = ThreeVector;

    fn div(self, other: f64) -> ThreeVector {
        ThreeVector(
        self.0 / other,
        self.1 / other,
        self.2 / other,
        )
    }
}

impl Mul<f64> for ThreeVector {
    type Output = ThreeVector;

    fn mul(self, other: f64) -> ThreeVector {
        ThreeVector(
            self.0 * other,
            self.1 * other,
            self.2 * other,
        )
    }
}

impl ThreeVector {
    pub fn dot_product(&self, &other: &ThreeVector) -> f64 {
        self.0 * other.0 +
        self.1 * other.1 +
        self.2 * other.2
    }

    pub fn cross_product(&self, &other: &ThreeVector) -> ThreeVector {
        ThreeVector(
            self.1 * other.2 - self.2 * other.1,
            self.0 * other.2 - self.2 * other.0,
            self.0 * other.1 - self.1 * other.0
        )
    }

    pub fn magnitude(&self) -> f64 {
        let square_sum = self.0.powi(2) + self.1.powi(2) + self.2.powi(2);
        square_sum.sqrt()
    }

    pub fn as_unit_vector(&self) -> ThreeVector {
        let length = self.magnitude();
        ThreeVector(self.0/length,
                    self.1/length,
                    self.2/length)
    }
}




