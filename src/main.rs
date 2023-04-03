use std::path::Path;

use camera::{Camera, CameraRayGen};
use half_plane::HalfPlane;
use hittable::Hittable;
use image::{RgbImage, Rgb};
use cgmath::{Vector3, InnerSpace};
use sphere::Sphere;
use rand::{Rng, thread_rng, distributions::Uniform, RngCore, rngs::SmallRng};
use rayon::prelude::*;

pub mod hittable;
pub mod sphere;
pub mod camera;
pub mod convex;
pub mod half_plane;

const IMG_WIDTH: u32 = 800;
const IMG_HEIGHT: u32 = 450;
const IMG_SAMPLES: u32 = 100;

pub struct World {
    pub stuff: Vec<Box<dyn Hittable + Send + Sync>>
}

pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Self {
        Ray {
            origin,
            direction: direction.normalize()
        }
    }
    
    pub fn at(&self, t: f64) -> Vector3<f64> {
        self.origin + t * self.direction
    }

    pub fn rand_diffuse(&self, rng: &mut SmallRng) -> Self {
        let dist = Uniform::new_inclusive(-1.0, 1.0);
        let v = loop {
            let x = rng.sample(dist);
            let y = rng.sample(dist);
            let z = rng.sample(dist);

            let v = Vector3 { x, y, z };
            if v.magnitude2() <= 1.0 {
                // break v
                break v.normalize()
            }
        };

        Ray {
            origin: self.origin,
            direction: self.direction + v
        }
    }

    pub fn sunward_ray(&self) -> Option<Self> {
        let sun_dir = - Vector3 { x: 1.0, y: 5.0, z: 0.5 };
        if sun_dir.dot(self.direction) < 0.0 {
            return None
        }

        Some(Ray {
            origin: self.origin,
            direction: sun_dir
        })
    }

    pub fn rand_ray(&self, prob: f64, rng: &mut SmallRng) -> Self {
        if rng.gen_bool(prob) {
            self.sunward_ray().unwrap_or(self.rand_diffuse(rng))
        } else {
            self.rand_diffuse(rng)
        }
    }
}

fn trace_path(ray: Ray, world: &World, depth: usize, rng: &mut SmallRng) -> Rgb<f64> {
    if depth == 0 {
        return Rgb([0.0, 0.0, 0.0])
    }
    let mut hit = None;
    for item in &world.stuff {
        if let Some((t, norm)) = item.hit(&ray) {
            hit = match hit {
                Some((t_new, norm_new)) => if t_new < t {
                    Some((t_new, norm_new))
                } else {
                    Some((t, norm))
                },
                None => Some((t, norm)),
            };
        }
    }

    if let Some((_, norm)) = hit {
        // let p = 10.0 - depth as f64;
        // let p = p * p / 100.0;
        // let p = p.clamp(0.0, 1.0);
        let new_ray = norm.rand_diffuse(rng);
        let next = trace_path(new_ray, world, depth - 1, rng);
        return Rgb([next.0[0] * 0.4, next.0[1] * 0.4, next.0[2] * 0.4])
    }
    
    let unit = ray.direction.normalize();
    let t = 0.5 * (unit.y + 1.0);
    let ti = 1.0 - t;

    Rgb([
        ti + t * 0.5,
        ti + t * 0.7,
        1.0
    ])
}

fn main() {

    let world = World {
        stuff: vec![
            Box::new(Sphere {
                center: Vector3 { x: 0.0, y: 0.0, z: -1.0 },
                radius: 0.5,
            }),
            Box::new(Sphere {
                center: Vector3 { x: 1.5, y: 0.0, z: -3.0 },
                radius: 0.5,
            }),
            Box::new(HalfPlane {
                normal: Vector3 { x: 0.0, y: 1.0, z: 0.0 },
                offset: -0.5,
            }),
        ]
    };

    let camera = Camera {
        ray: Ray { 
            origin: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            direction: Vector3 { x: 0.0, y: 0.0, z: -1.0 },
        },
        image_width: IMG_WIDTH,
        image_height: IMG_HEIGHT,
        focal_length: 0.87,
        aperture: 0.03,
        field_of_view: 2.0,
    };

    let pixels: Vec<_> = (0..IMG_HEIGHT).into_par_iter().map(|y| {
        let mut camera_ray_gen = CameraRayGen::new(&camera);
        let mut pixels = Vec::new();
        for x in 0..IMG_WIDTH {
            let mut r: f64 = 0.0;
            let mut g: f64 = 0.0;
            let mut b: f64 = 0.0;

            for _ in 0..IMG_SAMPLES {
                let ray = camera_ray_gen.gen_ray(x, y);
                let color = trace_path(ray, &world, 20, &mut camera_ray_gen.rng);

                r += color.0[0];
                g += color.0[1];
                b += color.0[2];
            }

            let scale = 1.0 / IMG_SAMPLES as f64;

            let r = (r * scale).sqrt();
            let g = (g * scale).sqrt();
            let b = (b * scale).sqrt();

            let r: u8 = (r * 256.0).clamp(0.0, 256.0 - f64::EPSILON) as u8;
            let g: u8 = (g * 256.0).clamp(0.0, 256.0 - f64::EPSILON) as u8;
            let b: u8 = (b * 256.0).clamp(0.0, 256.0 - f64::EPSILON) as u8;

            pixels.push(Rgb([r, g, b]));
        }
        pixels
    }).collect();

    let mut image = RgbImage::new(IMG_WIDTH, IMG_HEIGHT);
    for (y, line) in pixels.into_iter().enumerate() {
        for (x, color) in line.into_iter().enumerate() {
            image.put_pixel(x as u32, y as u32, color)
        }
    }

    image.save(Path::new("./my-test-image.png")).unwrap();
}
