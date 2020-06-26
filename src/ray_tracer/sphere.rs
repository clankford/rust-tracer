use crate::ray_tracer::tuple::Tuple;
use crate::ray_tracer::traits::object::Object;
use crate::ray_tracer::matrix::Matrix;

#[derive(PartialEq)]
pub struct Sphere {
    pub origin: Tuple,
    pub transform: Matrix
}

impl Object for Sphere {
    fn new() -> Self {
        Sphere {
            origin: Tuple::point(0.0, 0.0, 0.0),
            transform: Matrix::identity()
        }
    }
}

// There are instances where I want to instantiate a sphere with some but not all default values.
impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            origin: Tuple::point(0.0, 0.0, 0.0),
            transform: Matrix::identity() 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_transformation() {
        let s = Sphere::new();
        assert!(
            s.transform == Matrix::identity(),
            "The default transform for the sphere was not set correctly on construction."
        )
    }

    #[test]
    fn change_transformation() {
        let mut s = Sphere::new();
        s.transform = Matrix::translation(2.0, 3.0, 4.0);
        assert!(
            s.transform == Matrix::translation(2.0, 3.0, 4.0),
            "The default transform for the sphere was not set correctly on construction."
        )
    }
}