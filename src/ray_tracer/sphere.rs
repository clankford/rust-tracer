use crate::ray_tracer::tuple::Tuple;
use crate::ray_tracer::traits::object::Object;
use crate::ray_tracer::matrix::Matrix;
use crate::ray_tracer::material::Material;

#[cfg(test)]
use crate::ray_tracer::matrix::RotationAxis;

#[derive(PartialEq)]
pub struct Sphere {
    pub origin: Tuple,
    pub transform: Matrix,
    pub material: Material
}

impl Object for Sphere {
    fn new() -> Self {
        Default::default()
    }

    // Find the normal vector at a given point on the object. This is the perpendicular vector from
    // that point on the surface.
    fn normal_at(&self, world_point: Tuple) -> Tuple {
        let transform_inverse = self.transform.inverse();
        let object_point = &transform_inverse * &world_point;
        let object_normal = &object_point - &Tuple::point(0.0, 0.0, 0.0);
        let mut world_normal = &transform_inverse.transpose() * &object_normal;
        world_normal.w = Some(0);
        
        world_normal.norm()
    }
}

// There are instances where I want to instantiate a sphere with some but not all default values.
impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            origin: Tuple::point(0.0, 0.0, 0.0),
            transform: Matrix::identity(),
            material: Material::default()
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

    #[test]
    fn normal_at_point_on_x() {
        let s = Sphere::new();
        let result = s.normal_at(Tuple::point(1.0, 0.0, 0.0));
        assert!(
            result == Tuple::vector(1.0, 0.0, 0.0),
            "The normal_at function did not compute the right normal vector."
        )
    }

    #[test]
    fn normal_at_point_on_y() {
        let s = Sphere::new();
        let result = s.normal_at(Tuple::point(0.0, 1.0, 0.0));
        assert!(
            result == Tuple::vector(0.0, 1.0, 0.0),
            "The normal_at function did not compute the right normal vector."
        )
    }

    #[test]
    fn normal_at_point_on_z() {
        let s = Sphere::new();
        let result = s.normal_at(Tuple::point(0.0, 0.0, 1.0));
        assert!(
            result == Tuple::vector(0.0, 0.0, 1.0),
            "The normal_at function did not compute the right normal vector."
        )
    }

    #[test]
    fn normal_at_nonaxial_point() {
        let s = Sphere::new();
        let result = s.normal_at(Tuple::point(3.0_f32.sqrt() / 3.0, 3.0_f32.sqrt() / 3.0, 3.0_f32.sqrt() / 3.0));
        assert!(
            result == Tuple::vector(3.0_f32.sqrt() / 3.0, 3.0_f32.sqrt() / 3.0, 3.0_f32.sqrt() / 3.0),
            "The normal_at function did not compute the right normal vector."
        )
    }

    #[test]
    fn normal_is_normalized() {
        let s = Sphere::new();
        let result = s.normal_at(Tuple::point(3.0_f32.sqrt() / 3.0, 3.0_f32.sqrt() / 3.0, 3.0_f32.sqrt() / 3.0));
        assert!(
            result == Tuple::vector(3.0_f32.sqrt() / 3.0, 3.0_f32.sqrt() / 3.0, 3.0_f32.sqrt() / 3.0).norm(),
            "The normal_at function did not compute the right normal vector."
        )
    }

    #[test]
    fn normal_of_translated_sphere() {
        let s = Sphere { transform: Matrix::translation(0.0, 1.0, 0.0), ..Default::default() };
        let result = s.normal_at(Tuple::point(0.0, 1.70711, -0.70711));
        assert!(
            result == Tuple::vector(0.0, 0.70711, -0.70711),
            "The normal_at function did not compute the right normal vector."

        )
    }

    #[test]
    fn normal_of_transformed_sphere() {
        let s = Sphere { transform: &Matrix::scaling(1.0, 0.5, 1.0) * 
                                    &Matrix::rotation(std::f32::consts::PI/5.0, RotationAxis::Z),
                                    ..Default::default() };
        let result = s.normal_at(Tuple::point(0.0, 2.0_f32.sqrt() / 2.0, -2.0_f32.sqrt() / 2.0));
        assert!(
            result == Tuple::vector(0.0, 0.97014, -0.24254),
            "The normal_at function did not compute the right normal vector."
        )
    }

    #[test]
    fn assign_material_to_sphere() {
        let mut s = Sphere::new();
        s.material.ambient = 1.0;
        assert!(
            s.material.ambient == 1.0,
            "The value in the material in the sphere was not set correctly"
        )
    }
}