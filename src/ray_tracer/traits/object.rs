use crate::ray_tracer::tuple::Tuple;


pub trait Object {
    fn new() -> Self;
    fn normal_at(&self, point: Tuple) -> Tuple;
}