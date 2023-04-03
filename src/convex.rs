use cgmath::Vector3;

pub trait Convex {
    fn support(&self, direction: Vector3<f64>) -> f64;
}

pub trait ConvexHull {
    fn hull_support(&self, direction: Vector3<f64>) -> f64;
}

impl<T> ConvexHull for T
where T: Convex
{
    fn hull_support(&self, direction: Vector3<f64>) -> f64 {
        self.support(direction)
    }
}