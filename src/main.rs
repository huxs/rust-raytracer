#![feature(duration_as_u128)]

use std::time::Instant;

use rand::Rng;

use sdl2::pixels::Color;
use sdl2::rect::Point;

mod math;
use crate::math::Vec3;
use crate::math::Ray;

mod hitable;
use crate::hitable::Hitable;
use crate::hitable::HitableList;
use crate::hitable::HitableKind;

mod sphere;
use crate::sphere::Sphere;

mod camera;
use crate::camera::Camera;

mod material;
use crate::material::Lambertian;
use crate::material::Metal;
use crate::material::Dielectric;

use crate::material::MaterialKind;

mod random;

fn main() {

    let window_width = 800;
    let window_height = 600;
    let samples_per_pixel = 100;
    let realtime = false;

    let sdl_context = sdl2::init()
        .expect("Failed to init SDL");

    let video_subsystem = sdl_context.video()
        .expect("Failed to init video subsystem");

    let mut title : String;

    let window = video_subsystem.window("Rust Raytracer", window_width, window_height)
        .position_centered()
        .build()
        .expect("Failed to create window");

    let (width, height) = window.size();

    let mut canvas = window.into_canvas()
        .build()
        .expect("Failed to create canvas");

    let mut event_pump = sdl_context.event_pump()
            .expect("Failed to get event pump");

    let mut render = true;

    'running: loop {
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                Event::Quit {..} => { break 'running; },
                _ => ()
            }
        }

        if render == true { 

            canvas.set_draw_color(Color::RGB(128,0,0));
            canvas.clear();

            let aspect_ratio = width as f32 / height as f32;
            //let camera_pos = Vec3::new(90.0, 1.5, 2.0);
            let camera_pos = Vec3::new(0.0, 0.0, 2.0);
            let camera_target= Vec3::new(0.0, 0.0, -1.0);
            let _focus_distance = (camera_pos-camera_target).length();
            let camera = Camera::new(camera_pos, camera_target, Vec3::new(0.0, 1.0, 0.0), 45.0, aspect_ratio, 0.05, 5.0);

        //    let mut list = HitableList{ hitables: Vec::new() };
        //    list.hitables.push(Box::new(Sphere::new(Vec3{x:0.0, y:0.0, z:-1.0}, 0.5, MaterialKind::Lambertian(Lambertian{albedo:Vec3::new(0.1, 0.2, 0.5)}))));
        //     list.hitables.push(Box::new(Sphere::new(Vec3{x:0.0, y:-100.5, z:-1.0}, 100.0, MaterialKind::Lambertian(Lambertian{albedo:Vec3::new(0.8, 0.8, 0.0)}))));
        //     list.hitables.push(Box::new(Sphere::new(Vec3{x:1.0, y:0.0, z:-1.0}, 0.5, MaterialKind::Metal(Metal{albedo:Vec3{x:0.8, y:0.6, z:0.2}, fuzz:0.0}))));
        //     list.hitables.push(Box::new(Sphere::new(Vec3{x:-1.0, y:0.0, z:-1.0}, 0.5, MaterialKind::Dielectric(Dielectric{ref_idx:1.5}))));
        //     list.hitables.push(Box::new(Sphere::new(Vec3{x:-1.0, y:0.0, z:-1.0}, -0.45, MaterialKind::Dielectric(Dielectric{ref_idx:1.5}))));

            let hitables : Vec<HitableKind> = vec![
                HitableKind::Sphere(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, MaterialKind::Lambertian(Lambertian{albedo:Vec3::new(0.1, 0.2, 0.5)}))),
                HitableKind::Sphere(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, MaterialKind::Lambertian(Lambertian{albedo:Vec3::new(0.8, 0.8, 0.0)}))),
                HitableKind::Sphere(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, MaterialKind::Metal(Metal{albedo:Vec3{x:0.8, y:0.6, z:0.2}, fuzz:0.0}))),
                HitableKind::Sphere(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, MaterialKind::Dielectric(Dielectric{ref_idx:1.5}))),
                HitableKind::Sphere(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, MaterialKind::Dielectric(Dielectric{ref_idx:1.5})))
            ];

            let _rng_scene = rand::thread_rng();
            let mut rng = rand::thread_rng();

            //let list = random_scene(&mut rng_scene);

            let now = Instant::now();

            for row in 0..height {
                for column in 0..width {

                    let mut color = Vec3{x:0.0, y:0.0, z:0.0};
                    for _sample in 0..samples_per_pixel {
                        let u = (column as f32 + rng.gen::<f32>()) / width as f32;
                        let v = (row as f32 + rng.gen::<f32>()) / height as f32;
                        let ray = camera.get_ray(u, v, &mut rng);
                        color += trace(&ray, &hitables, &mut rng, 0);
                    }
                    color /= samples_per_pixel as f32;
                    canvas.set_draw_color(Color::RGB((color.x * 255.99) as u8, (color.y * 255.99) as u8, (color.z * 255.99) as u8));
                    canvas.draw_point(Point::new(column as i32, row as i32 ))
                        .expect("Failed to draw point");
                }
            }

            let elapesed = now.elapsed().as_millis();
            let ray_count = window_width * window_height * samples_per_pixel;

            title = format!("Rust Raytracer ms: {} ray count: {}", elapesed.to_string(), ray_count );

            canvas.window_mut().set_title(&title)
                .expect("Failed to set title");

            if realtime == false {
                render = false;
            }
        }

        canvas.present();
    }
}

// fn random_scene(rng: &mut rand::ThreadRng) -> HitableList {
//     let mut list = HitableList{ hitables: Vec::new() };
//     list.hitables.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, -1.0), 1000.0, Box::new(Lambertian{albedo:Vec3::new(0.5, 0.5, 0.5)}))));
//     for a in -11..10 {
//         for b in -11..10 {
//             let rand_mat = rng.gen::<f32>();
//             let center = Vec3::new(a as f32 + 0.9 * rng.gen::<f32>(), 0.2, b as f32 + 0.9 * rng.gen::<f32>());
//             if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
//                 if rand_mat < 0.8 {
//                     let albedo = Vec3::new(rng.gen::<f32>()*rng.gen::<f32>(), rng.gen::<f32>()*rng.gen::<f32>(), rng.gen::<f32>()*rng.gen::<f32>());
//                     list.hitables.push(Box::new(Sphere::new(center, 0.2, Box::new(Lambertian{albedo:albedo}))));
//                 } else if rand_mat < 0.95 {
//                     let albedo = Vec3::new((1.0 + rng.gen::<f32>())*0.5, (1.0 + rng.gen::<f32>())*0.5, (1.0 + rng.gen::<f32>())*0.5);
//                     list.hitables.push(Box::new(Sphere::new(center, 0.2, Box::new(Metal{albedo:albedo, fuzz:rng.gen::<f32>()*0.5}))));
//                 } else {
//                     list.hitables.push(Box::new(Sphere::new(center, 0.2, Box::new(Dielectric{ref_idx:1.5}))));
//                 }
//             }
//         }
//     }

//     list.hitables.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Box::new(Dielectric{ref_idx:1.5}))));
//     list.hitables.push(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Box::new(Lambertian{albedo:Vec3::new(0.4, 0.2, 0.1)}))));
//     list.hitables.push(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Box::new(Metal{albedo:Vec3::new(0.7, 0.6, 0.5), fuzz:0.0}))));

//     list
// }

fn trace(ray: &Ray, hitables: &Vec<HitableKind>, rng: &mut rand::ThreadRng, depth: i32) -> Vec3 {

    let mut closest_rec : Option<hitable::HitRecord> = None;
    let mut closest_t = 100.0;
    for hitable in hitables {
        match hitable.hit(&ray, 0.001, closest_t) {
            Some(rec) => {
                closest_t = rec.t;
                closest_rec = Some(rec);
            }
            None => {}
        }
    }

    match closest_rec {
        Some(rec) => {           
            let (result, ray, attenuation) = rec.material.scatter(ray, &rec, rng);
            if depth < 50 && result == true {
                return attenuation * trace(&ray, hitables, rng, depth + 1);
            } else {
                return Vec3::ZERO;
            }
        }
        None => {}
    }
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    return Vec3::ONE * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

