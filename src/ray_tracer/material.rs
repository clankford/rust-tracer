use crate::ray_tracer::tuple::Tuple;
use crate::ray_tracer::common::f_equal;

pub struct Material {
    pub color: Tuple,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32
}

impl Material {
    // TODO: This is not consistent with other contstructors with default values (see: Sphere) make
    // these consistent
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            color: Tuple::color(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0
        }
    }
}

// Must overload PartialEq instead of leveraging Derive PartialEq on the Material struct. This is
// because we have a custom implementation for comparing floating point numbers f_equal.
impl PartialEq for Material {
    fn eq(&self, other: &Material) -> bool {
        if f_equal(self.ambient, other.ambient) && self.color == other.color &&
            f_equal(self.diffuse, other.diffuse) && f_equal(self.shininess, other.shininess) &&
            f_equal(self.specular, other.specular) {
                true
            }
        else {
            false
        }
    }
}
impl Eq for Material {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_material() {
        let m = Material::default();
        assert!(
            m.color == Tuple::color(1.0, 1.0, 1.0) &&
            m.ambient == 0.1 &&
            m.diffuse == 0.9 &&
            m.specular == 0.9 &&
            m.shininess == 200.0,
            "The creation of the material struct did not happen correctly."
        )
    }

    #[test]
    fn material_equality() {
        let m1 = Material::default();
        let m2 = Material::default();
        assert!(
            m1 == m2,
            "The materials were not equal but they should be"
        )
    }
}