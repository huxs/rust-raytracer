use crate::math::Vec3;
use crate::math::Ray;

use crate::random::random_direction_on_disc;

use std::f32::consts::PI;

pub struct Camera {
    pub origin: Vec3,
    pub upper_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lens_radius: f32,
    pub u: Vec3,
    pub v: Vec3,
}

impl Camera {
    pub fn new(origin: Vec3, target: Vec3, up: Vec3, fov: f32, aspect: f32, aperature: f32, focus_dist: f32) -> Camera {

        let w = (origin - target).normalize();
        let u = up.cross(w);
        let v = w.cross(u);

        let theta = fov*PI/180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;

        Camera{
           origin,
           upper_left_corner: -u * half_width * focus_dist + v * half_height * focus_dist - w * focus_dist,
           horizontal: u * half_width * 2.0 * focus_dist,
           vertical: v * -half_height * 2.0 * focus_dist,
           lens_radius: aperature * 0.5,
           u,
           v,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32, rng: &mut rand::ThreadRng) -> Ray {
        let rd = random_direction_on_disc(rng) * self.lens_radius;
        let offset = self.u*rd.x + self.v*rd.y;
        Ray::new(self.origin + offset, self.upper_left_corner + self.horizontal * u + self.vertical * v - offset)
    }
}       