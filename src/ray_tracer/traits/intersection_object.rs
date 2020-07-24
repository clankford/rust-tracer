use crate::ray_tracer::traits::object::Object;

pub trait IntersectionObject {
    fn get_t(&self) -> f32;
    fn get_object(&self) -> Box<dyn Object>;
}