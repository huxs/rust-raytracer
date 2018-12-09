use rand::Rng;
use crate::math::Vec3;

pub fn random_direction(rng: &mut rand::ThreadRng) -> Vec3 {
    loop {
        let v = Vec3{x:rng.gen::<f32>() * 2.0 - 1.0, y:rng.gen::<f32>() * 2.0 - 1.0, z:rng.gen::<f32>() * 2.0 - 1.0};
        if v.squared_length() < 1.0 {
            return v;
        }
    } 
}

pub fn random_direction_on_disc(rng: &mut rand::ThreadRng) -> Vec3 {
    loop {
        let v = Vec3::new(rng.gen::<f32>() * 2.0 - 1.0, rng.gen::<f32>() * 2.0 - 1.0, 0.0);
        if v.dot(v) < 1.0 {
            return v;
        }
    }
}