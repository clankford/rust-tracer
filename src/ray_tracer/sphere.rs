use crate::ray_tracer::tuple::Tuple;
use crate::ray_tracer::traits::object::Object;

#[derive(PartialEq)]
pub struct Sphere {
    pub origin: Tuple
}

impl Object for Sphere {
    fn new() -> Self {
        Sphere {
            origin: Tuple::point(0.0, 0.0, 0.0)
        }
    }
}