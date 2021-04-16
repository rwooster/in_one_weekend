use super::util;
use std::fmt;
use std::ops;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3(x, y, z)
    }

    pub fn x(&self) -> f32 {
        self.0
    }
    pub fn y(&self) -> f32 {
        self.1
    }
    pub fn z(&self) -> f32 {
        self.2
    }

    pub fn norm_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn norm(self) -> f32 {
        self.norm_squared().sqrt()
    }

    pub fn dot(self, rhs: Self) -> f32 {
        (self.0 * rhs.0) + (self.1 * rhs.1) + (self.2 * rhs.2)
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    pub fn unit_vector(self) -> Self {
        self / self.norm()
    }

    pub fn random() -> Self {
        Self::new(
            util::random_float(),
            util::random_float(),
            util::random_float(),
        )
    }

    pub fn random_range(min: f32, max: f32) -> Self {
        Self::new(
            util::random_float_bounds(min, max),
            util::random_float_bounds(min, max),
            util::random_float_bounds(min, max),
        )
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.0.abs() < s && self.0.abs() < s && self.2.abs() < s
    }

}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.norm_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit_vector()
}

pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(*normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - (*n * v.dot(*n) * 2.0)
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = (-(*uv).dot(*n)).min(1.0);
    let r_out_perp = (*uv + *n * cos_theta) * etai_over_etat;
    let r_out_parallel = *n * (1.0 - r_out_perp.norm_squared()).abs().sqrt();
    r_out_perp + r_out_parallel
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        self * (1.0 / rhs)
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0, -self.1, -self.2)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

// Type aliases
pub use Vec3 as Point3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operators() {
        let v1 = Vec3(1.0, 1.0, 1.0);
        let v2 = Vec3(2.0, 2.0, 2.0);

        assert!(v1 + v2 == Vec3(3.0, 3.0, 3.0));
        assert!(v2 - v1 == Vec3(1.0, 1.0, 1.0));
        assert!(v2 * v2 == Vec3(4.0, 4.0, 4.0));
        assert!(v1 * 2.5 == Vec3(2.5, 2.5, 2.5));
        assert!(v2 / 2.0 == Vec3(1.0, 1.0, 1.0));
        assert!(-v1 == Vec3(-1.0, -1.0, -1.0));
    }

    #[test]
    fn test_functions() {
        let v1 = Vec3(1.0, 1.0, 1.0);
        let v2 = Vec3(2.0, 2.0, 2.0);

        assert!(v1.dot(v2) == 6.0);
        assert!(v2.norm_squared() == 12.0);
        assert!(v2.norm() == (12.0 as f32).sqrt());
        assert!(v1.cross(v2) == Vec3(0.0, 0.0, 0.0));
        assert!(Vec3(1.0, 0.0, 0.0).unit_vector() == Vec3(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_near_zero() {
        let v1 = Vec3(1.0, 1.0, 1.0);
        let v2 = Vec3(0.0000000001, 0.0, 0.0);

        assert!(!v1.near_zero());
        assert!(v2.near_zero());
    }
}
