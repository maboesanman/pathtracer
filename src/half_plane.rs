use cgmath::{Vector3, InnerSpace};

use crate::{hittable::Hittable, Ray};



pub struct HalfPlane {
    pub normal: Vector3<f64>,
    pub offset: f64,
}

impl Hittable for HalfPlane {
    fn hit(&self, ray: &crate::Ray) -> Option<(f64, crate::Ray)> {
        let proj = ray.origin.project_on(self.normal);
        let t = (self.offset - proj.magnitude()) / (ray.direction.dot(self.normal));
        let intersect = ray.at(t);

        if t < 0.0 {
            return None
        }

        Some((t, Ray::new(intersect, self.normal)))
    }
}
