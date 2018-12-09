use crate::math::Vec3;
use crate::math::Ray;

use crate::hitable::HitRecord;

use rand::Rng;
use crate::random::random_direction;

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, rng: &mut rand::ThreadRng) -> (bool, Ray, Vec3);
}

pub struct Lambertian {
    pub albedo: Vec3
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord, rng: &mut rand::ThreadRng) -> (bool, Ray, Vec3) {
        (true, Ray::new(rec.pos, rec.normal + random_direction(rng)), self.albedo)
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, rng: &mut rand::ThreadRng) -> (bool, Ray, Vec3) {
        let reflect = Ray::new(rec.pos, ray.direction.normalize().reflect(rec.normal) + (random_direction(rng) * self.fuzz));

        (reflect.direction.dot(rec.normal) > 0.0, reflect, self.albedo)
    }
}

/*
    Snell's law
    n1 * sin(theta1) = n2 * sin(theta2)    
    n: refract index (air=1, glass=1.3-1.7, diamond=2.4)

    i: incident
    n: normal
    t: reflected

    t = (n_perp * sin(theta2)) - n*cos(theta2)
    sin(theta2) = (n1/n2)*sin(theta1)

    t = (n_perp * sin(theta2)) - n*cos(theta2)
    cos(theta2) = sqrt(1 - sin(theta2)^2)

    cos(theta2) = sqrt(1 - ((n1/n2)*sin(theta1))^2)
    cos(theta2) = sqrt(1 - (n1/n2)^2*(1 - cos(theta1))^2)

    n_perp = ((i - n * cos(theta1)) * sin(theta2) / sin(theta2))
    n_perp = ((i - n * cos(theta1))) * (n1/n2)
*/

fn refract(ray_dir: Vec3, normal: Vec3, ni_over_nr: f32) -> (bool, Vec3) {
    let incident = ray_dir.normalize();
    let cos_theta1 = incident.dot(normal);
    
    let discriminant = 1.0 - (ni_over_nr*ni_over_nr)*(1.0 - cos_theta1*cos_theta1);
    if discriminant > 0.0 {
        return (true, ((incident - normal * cos_theta1)*ni_over_nr) - (normal * discriminant.sqrt()) );
    } 
    (false, Vec3::ZERO)
}

/* Schlick approximation */

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
    r0 = r0*r0;
    r0 + (1.0-r0) * (1.0-cosine).powf(5.0)
}

pub struct Dielectric {
    pub ref_idx : f32
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, rng: &mut rand::ThreadRng) -> (bool, Ray, Vec3) {
        let in_direction = ray.direction.normalize();

        let (outward_normal, ni_over_nr, cosine) = if in_direction.dot(rec.normal) > 0.0 {
            (-rec.normal, self.ref_idx, self.ref_idx * in_direction.dot(rec.normal) / in_direction.length())
        } else {
            (rec.normal, 1.0 / self.ref_idx, -(self.ref_idx * in_direction.dot(rec.normal) / in_direction.length()))
        };

        let (result, refracted) = refract(ray.direction, outward_normal, ni_over_nr);
        let reflect_prob = if result == true {
            schlick(cosine, self.ref_idx)
        } else {
            1.0
        };

        let new_ray = if rng.gen::<f32>() < reflect_prob {
            Ray::new(rec.pos, in_direction.reflect(rec.normal))
        } else {
            Ray::new(rec.pos, refracted)
        };

        (true, new_ray, Vec3::ONE)
    }
}

enum MaterialKind {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric)
}