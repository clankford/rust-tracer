use crate::ray_tracer::traits::object::Object;

#[cfg(test)]
use crate::ray_tracer::sphere::Sphere;

pub struct Intersection<'a> {
    pub t: f32,
    pub object: Box<dyn Object + 'a>
}

impl<'a> Intersection<'a> {
    pub fn new(t: f32, object: Box<dyn Object + 'a>) -> Intersection<'a> {
        Intersection {
            t,
            object
        }
    }

    fn get_t(&self) -> f32 {
        self.t
    }
    
    fn get_object(&self) -> &Box<dyn Object + 'a> {
        &self.object
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_intersection() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, Box::new(&s));
        assert!(
            i.t == 3.5 &&
            i.object.get_material() == s.get_material() &&
            i.object.get_object_type() == s.get_object_type() &&
            i.object.get_transform() == s.get_transform() &&
            i.object.get_origin() == s.get_origin(),
            "The creation of the ray was not valid."
        );
    }
}