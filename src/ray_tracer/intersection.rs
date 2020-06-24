use crate::ray_tracer::traits::object::Object;

#[cfg(test)]
use crate::ray_tracer::sphere::Sphere;

pub struct Intersection<'a, T: Object> {
    pub t: f32,
    pub object: &'a T
}

impl<'a, T: Object> Intersection<'a, T> {
    pub fn new(t: f32, object: &T) -> Intersection<T> {
        Intersection {
            t,
            object
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_intersection() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);
        assert!(
            i.t == 3.5 && i.object == &s,
            "The creation of the ray was not valid."
        );
    }
}