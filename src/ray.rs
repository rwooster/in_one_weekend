use super::vec3;

// A ray is represnted by P(t) = A + tB
// P is a position along a 3D line
// A is the origin of the ray
// B is the direction of the ray
pub struct Ray {
    pub origin: vec3::Point3,
    pub direction: vec3::Vec3,
}

impl Ray {
    pub fn new(origin: vec3::Vec3, direction: vec3::Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f32) -> vec3::Point3 {
        self.origin + (self.direction * t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray() {
        let r1 = Ray::new(vec3::Vec3(1.0, 1.0, 1.0), vec3::Vec3(2.0, 0.0, 0.0));

        assert!(r1.at(5.0) == vec3::Vec3(11.0, 1.0, 1.0));
    }
}
