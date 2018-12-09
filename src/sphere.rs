use crate::math::Vec3;
use crate::math::Ray;

use crate::hitable::Hitable;
use crate::hitable::HitRecord;

use crate::material::Material;

pub struct Sphere<'a> {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<Material + 'a>
}

impl<'a> Sphere<'a> {
    pub fn new(_center: Vec3, _radius: f32, _material: Box<Material + 'a>) -> Sphere {
        Sphere{center:_center, radius:_radius, material:_material}
    }
}

/*
    x*x + y*y + z*z = R*R
    (x-cx)*(x-cx) + (y-cy)*(y-cy) + (z-cz)*(z-cz) = R*R
    p = (x, y, z)
    c = (cx, cy, cz)
    
    dot((p-c), (p-c)) = R*R
    dot((t*d+o- c)(t*d+o-c)) = R*R

    t^2*dot(d,d) + 2*t*dot(d,o-c) + dot(o-c,o-c) - R*R = 0

    a = dot(d,d)
    b = 2*dot(d, o-c)
    c = dot(o-c, o-c) - R*R

    t^2 * a + t * b + c
    t = (-b +- sqrt(b^2 - 4*a*c)) / 2*a

    b^2 - 4*a*c = 0 : 1 solution
    b^2 - 4*a*c > 0 : 2 solutions
    b^2 - 4*a*c < 0 : 0 solutions
*/

impl<'a> Hitable for Sphere<'a> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction); 
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius*self.radius;
        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let pos = ray.point_at_parameter(t);
                return Some(HitRecord::new(t, pos, (pos - self.center) / self.radius, self.material.as_ref()));
            } 
            let t = (-b + discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let pos = ray.point_at_parameter(t);
                return Some(HitRecord::new(t, pos, (pos - self.center) / self.radius, self.material.as_ref()));
            }          
        }
        return None;
    }
}