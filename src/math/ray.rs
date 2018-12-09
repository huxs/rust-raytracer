use crate::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {

    pub fn new(_origin: Vec3, _direction: Vec3) -> Ray {
        Ray{origin:_origin, direction:_direction}
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}
