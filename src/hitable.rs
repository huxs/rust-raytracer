use crate::math::Vec3;
use crate::math::Ray;

use crate::material::Material;
use crate::material::MaterialKind;

use crate::sphere::Sphere;

pub struct HitRecord<'a> {
    pub t: f32,
    pub pos: Vec3,
    pub normal: Vec3,
    pub material: &'a MaterialKind,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f32, pos: Vec3, normal: Vec3, material: &'a MaterialKind) -> HitRecord<'a> {
        HitRecord{t, pos, normal, material}
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HitableList<'a> {
    pub hitables : Vec<Box<Hitable + 'a>>
}

impl<'a> Hitable for HitableList<'a> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_rec : Option<HitRecord> = None;
        let mut closest_t = t_max;
        for hitable in &self.hitables {
            match hitable.hit(&ray, t_min, closest_t) {
                Some(rec) => {
                    closest_t = rec.t;
                    closest_rec = Some(rec);
                }
                None => {}
            }
        }
        return closest_rec
    }
}

pub enum HitableKind {
    Sphere(Sphere)
}

impl HitableKind {
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self {
            HitableKind::Sphere(sphere) => { return Sphere::hit(sphere, ray, t_min, t_max) }
            _ => { return None; }
        }
    }
}