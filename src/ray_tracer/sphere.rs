use crate::ray_tracer::tuple::Tuple;

pub struct Sphere {
    pub origin: Tuple
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            origin: Tuple::point(0.0, 0.0, 0.0)
        }
    }
}