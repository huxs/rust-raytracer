use crate::math::Vec3;
use crate::math::Ray;

use crate::material::Material;

pub struct HitRecord<'a> {
    pub t: f32,
    pub pos: Vec3,
    pub normal: Vec3,
    pub material: &'a Material
}

impl<'a> HitRecord<'a> {
    pub fn new(_t: f32, _pos: Vec3, _normal: Vec3, _material: &'a Material) -> HitRecord {
        HitRecord{t:_t, pos:_pos, normal:_normal, material:_material}
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
