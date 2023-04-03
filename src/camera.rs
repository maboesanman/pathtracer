use cgmath::{InnerSpace, Vector3};
use rand::{Rng, distributions::Uniform, SeedableRng, rngs::SmallRng};

use crate::Ray;

pub struct Camera {
    pub ray: Ray,
    pub image_width: u32,
    pub image_height: u32,
    pub field_of_view: f64,
    pub focal_length: f64,
    pub aperture: f64,
}

impl Camera {
    pub fn aspect_ratio(&self) -> f64 {
        self.image_width as f64 / self.image_height as f64
    }

    pub fn viewport_height(&self) -> f64 {
        self.viewport_width() / self.aspect_ratio()
    }

    pub fn viewport_width(&self) -> f64 {
        (self.field_of_view * 0.5).tan() * 2.0 * self.focal_length
    }
}

pub struct CameraRayGen<'a> {
    pub camera: &'a Camera,
    pub rng: SmallRng,
    unit_distribution: Uniform<f64>,
    x_direction: Vector3<f64>,
    y_direction: Vector3<f64>,
    x_aperture: Vector3<f64>,
    y_aperture: Vector3<f64>,
    upper_left: Vector3<f64>,
}

impl<'a> CameraRayGen<'a> {
    pub fn new(camera: &'a Camera) -> Self {
        let center_direction = camera.ray.direction.normalize_to(camera.focal_length);
        let screen_center = camera.ray.origin + center_direction;
        let x_direction = center_direction.cross(Vector3 { x: 0.0, y: 1.0, z: 0.0 }).normalize_to(camera.viewport_width());
        let y_direction = center_direction.cross(x_direction).normalize_to(camera.viewport_height());
        let upper_left = screen_center - x_direction * 0.5 - y_direction * 0.5;
        let rng = SmallRng::from_entropy();
        let unit_distribution = Uniform::new(0.0, 1.0);
        let x_aperture = x_direction.normalize_to(camera.aperture * 0.5);
        let y_aperture = y_direction.normalize_to(camera.aperture * 0.5);
        // println!("center_direction: {center_direction:?}");
        // println!("screen_center: {screen_center:?}");
        // println!("x_direction: {x_direction:?}");
        // println!("y_direction: {y_direction:?}");
        // println!("upper_left: {upper_left:?}");

        Self {
            camera,
            rng,
            unit_distribution,
            x_direction,
            y_direction,
            x_aperture,
            y_aperture,
            upper_left,
        }
    }

    pub fn gen_ray(&mut self, x: u32, y: u32) -> Ray {

        //   --------0,1--------
        //   |        |        |
        //   |        |        |
        // -1,0------0,0------1,0
        //   |        |        |
        //   |        |        |
        //   --------0,-1-------
        // let x = x as f64 / self.camera.image_width as f64;
        // let y = y as f64 / self.camera.image_width as f64;
        // let x = x 

        let point = self.upper_left
                  + self.x_direction * ((x as f64 + self.rng.sample(self.unit_distribution)) / self.camera.image_width as f64) 
                  + self.y_direction * ((y as f64 + self.rng.sample(self.unit_distribution)) / self.camera.image_height as f64);

        let base = self.camera.ray.origin
                 + self.x_aperture * self.rng.sample(self.unit_distribution)
                 + self.y_aperture * self.rng.sample(self.unit_distribution);

        Ray::new(base, point - base)
    }
}