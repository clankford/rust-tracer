use crate::ray_tracer::tuple::Tuple;
use crate::ray_tracer::material::Material;
use crate::ray_tracer::matrix::Matrix;
use crate::ray_tracer::enums::object_types::ObjectTypes;

pub trait Object {
    // Getters for when an object gets boxed as part of world.objects Train Object
    // These properties should be present for every type of object.
    fn get_origin(&self) -> &Tuple;
    fn get_transform(&self) -> &Matrix;
    fn get_material(&self) -> &Material;
    fn get_object_type(&self) -> ObjectTypes;
    fn normal_at(&self, point: Tuple) -> Tuple;
}