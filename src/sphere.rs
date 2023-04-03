use cgmath::{Vector3, InnerSpace};

use crate::{Ray, hittable::Hittable, convex::Convex};

#[derive(Clone)]
pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<(f64, Ray)> {
        let pc = self.center - ray.origin;
        let pc2 = pc.dot(ray.direction);

        let descriminant = self.radius * self.radius - pc.magnitude2() + pc2 * pc2;

        if descriminant < 0.0 {
            return None
        }

        let t = pc2 - descriminant;

        if t < 0.0 {
            return None
        }

        let intersection = ray.at(t);
        let normal = Ray::new(intersection, intersection - self.center);

        Some((t, normal))
    }
}

impl Convex for Sphere {
    fn support(&self, direction: Vector3<f64>) -> f64 {
        self.center.dot(direction.normalize()) + self.radius
    }
}
