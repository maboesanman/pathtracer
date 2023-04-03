use crate::Ray;

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<(f64, Ray)>;
}
