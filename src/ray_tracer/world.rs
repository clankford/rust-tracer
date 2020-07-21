use crate::ray_tracer::light::Light;
use crate::ray_tracer::tuple::Tuple;
use crate::ray_tracer::sphere::Sphere;
use crate::ray_tracer::material::Material;
use crate::ray_tracer::matrix::Matrix;
use crate::ray_tracer::traits::object::Object;

pub struct World {
    light: Light,
    objects: Vec<Box<dyn Object>>
}

impl World {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for World {
    fn default() -> Self {
        let mut m1 = Material::new();
        m1.color = Tuple::color(0.8, 1.0, 0.6);
        m1.diffuse = 0.7;
        m1.specular = 0.2;
        let s1 = Sphere { material: m1, ..Default::default() };
        let s2 = Sphere { transform: Matrix::scaling(0.5, 0.5, 0.5), ..Default::default() };
        World {
            light: Light::new(Tuple::color(1.0, 1.0, 1.0), Tuple::point(-10.0, 10.0, -10.0)),
            // s1 and s2 are treated as Trait Objects, as required by the type of objects in the Struct
            objects: vec![Box::new(s1), Box::new(s2)]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_world() {
        let light = Light::new(Tuple::color(1.0, 1.0, 1.0), Tuple::point(-10.0, 10.0, -10.0));
        let mut m1 = Material::new();
        m1.color = Tuple::color(0.8, 1.0, 0.6);
        m1.diffuse = 0.7;
        m1.specular = 0.2;
        let s1 = Sphere { material: m1, ..Default::default() };
        let s2 = Sphere { transform: Matrix::scaling(0.5, 0.5, 0.5), ..Default::default() };
        let w = World::new();
        assert!(
            w.light == light &&
            w.objects[0].get_material() == s1.get_material() &&
            w.objects[0].get_object_type() == s1.get_object_type() &&
            w.objects[0].get_transform() == s1.get_transform() &&
            w.objects[0].get_origin() == s1.get_origin() &&
            w.objects[1].get_material() == s2.get_material() &&
            w.objects[1].get_object_type() == s2.get_object_type() &&
            w.objects[1].get_transform() == s2.get_transform() &&
            w.objects[1].get_origin() == s2.get_origin(),            
            "The default world was not created correctly."
        )
    }
}