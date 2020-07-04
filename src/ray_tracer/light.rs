use crate::ray_tracer::tuple::Tuple;
use crate::ray_tracer::material::Material;


pub struct Light {
    pub intensity: Tuple,
    pub position: Tuple
}

impl Light {
    pub fn new(intensity: Tuple, position: Tuple) -> Self {
        Light {
            intensity,
            position 
        }
    }

    fn lighting(&self, material: Material, position: Tuple, eyev: Tuple, normalv: Tuple) -> Tuple {
        // Combine the surface color with the light's color/intensity
        let effective_color = material.color.hadamard_product(self.intensity);
        // Find the direction to the light source
        let lightv = (&self.position - &position).norm();
        // Compute the ambient contribution
        let ambient = &effective_color * material.ambient;
        // light_dot_normal represents the cosine of the angle between the light vector and the
        // normal vector. A negative number means the light is on the other side of the surface.
        let light_dot_normal = &lightv * &normalv;
        // Initalize to black
        let mut diffuse = Tuple::color(0.0, 0.0, 0.0);
        // Initialize to black
        let mut specular = Tuple::color(0.0, 0.0, 0.0);
        if light_dot_normal >= 0.0 {
            // Compute the diffuse contribution
            diffuse = &(&effective_color * material.diffuse) * light_dot_normal;
            // reflect_dot_eye represents the cosine of the angle between the reflection vector
            // and the eye vector. A negative number means the light reflects away from the eye.
            let reflectv = -&lightv.reflect(normalv);
            let reflect_dot_eye = &reflectv * &eyev;
            if reflect_dot_eye > 0.0 {
                // Compute the specular contribution
                let factor = reflect_dot_eye.powf(material.shininess);
                specular = &(&self.intensity * material.specular) * factor;
            }
        }

        &(&ambient + &diffuse) + &specular
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_light() {
        let l = Light::new(Tuple::color(1.0, 1.0, 1.0), Tuple::point(0.0, 0.0, 0.0));
        assert!(
            l.intensity == Tuple::color(1.0, 1.0, 1.0) && l.position == Tuple::point(0.0, 0.0, 0.0),
            "The creation of the light struct did not happen correctly."
        )
    }

    // TODO: How do I use setup functions in unit tests?
    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(Tuple::color(1.0, 1.0, 1.0), Tuple::point(0.0, 0.0, -10.0));
        let result = light.lighting(m, position, eyev, normalv);
        let expected = Tuple::color(1.9, 1.9, 1.9);
        assert!(
            result == expected,
            "The resultant lighting was not calculated correctly."
        )
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_eye_offset_45() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 2.0_f32.sqrt() / 2.0, -2.0_f32.sqrt() / 2.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(Tuple::color(1.0, 1.0, 1.0), Tuple::point(0.0, 0.0, -10.0));
        let result = light.lighting(m, position, eyev, normalv);
        let expected = Tuple::color(1.0, 1.0, 1.0);
        assert!(
            result == expected,
            "The resultant lighting was not calculated correctly."
        )
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(Tuple::color(1.0, 1.0, 1.0), Tuple::point(0.0, 10.0, -10.0));
        let result = light.lighting(m, position, eyev, normalv);
        let expected = Tuple::color(0.7364, 0.7364, 0.7364);
        assert!(
            result == expected,
            "The resultant lighting was not calculated correctly."
        )
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection_vector() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, -2.0_f32.sqrt() / 2.0, -2.0_f32.sqrt() / 2.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(Tuple::color(1.0, 1.0, 1.0), Tuple::point(0.0, 10.0, -10.0));
        let result = light.lighting(m, position, eyev, normalv);
        let expected = Tuple::color(1.63638, 1.63638, 1.63638);
        assert!(
            result == expected,
            "The resultant lighting was not calculated correctly. The result was: {:#?}", result
        )
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(Tuple::color(1.0, 1.0, 1.0), Tuple::point(0.0, 0.0, 10.0));
        let result = light.lighting(m, position, eyev, normalv);
        let expected = Tuple::color(0.1, 0.1, 0.1);
        assert!(
            result == expected,
            "The resultant lighting was not calculated correctly."
        )
    }
}