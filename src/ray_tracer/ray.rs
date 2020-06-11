use crate::ray_tracer::tuple::Tuple;
use crate::ray_tracer::sphere::Sphere;

pub struct Ray {
    //TODO: This doesn't feel safe becasue an origin HAS to be a point, not a vector. How can I have more safety here?
    pub origin: Tuple,
    pub direction: Tuple
}

impl Ray {
    //TODO: If I can't get stronger types on origin and direction then I should add validation here.
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray {
            origin,
            direction
        }
    }

    // Returns the position of along a ray at time t.
    pub fn position(&self, t: f32) -> Tuple {
        &self.origin + &(&self.direction * t)
    }


    // Returns an Option so that None can be returned if the ray does not intersect with the sphere
    pub fn intersect(r: Ray, s: Sphere) -> Option<[f32; 2]> {
        
        // Yields the vector from the sphere's origin to the ray's origin
        let sphere_to_ray = &r.origin - &s.origin;
        // Dot product of the ray direction on itself
        let a = &r.direction * &r.direction;
        let b = 2.0 * (&r.direction * &sphere_to_ray);
        let c = (&sphere_to_ray * &sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;

        // Ray doesn't intersect
        if discriminant < 0.0 {
            None
        }
        // Ray does intersect
        else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            Some([t1, t2])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_ray() {
        let p = Tuple::point(1.0, 2.0, 3.0);
        let v = Tuple::vector(4.0, 5.0, 6.0);
        let r = Ray::new(p, v);
        assert!(
            r.direction == v && r.origin == p,
            "The creation of the ray was not valid."
        );
    }

    #[test]
    fn compute_point_from_distance() {
        let p = Tuple::point(2.0, 3.0, 4.0);
        let v = Tuple::vector(1.0, 0.0, 0.0);
        let r = Ray::new(p, v);
        let result1 = r.position(0.0);
        let result2 = r.position(1.0);
        let result3 = r.position(-1.0);
        let result4 = r.position(2.5);
        let expected1 = Tuple::point(2.0, 3.0, 4.0);
        let expected2 = Tuple::point(3.0, 3.0, 4.0);
        let expected3 = Tuple::point(1.0, 3.0, 4.0);
        let expected4 = Tuple::point(4.5, 3.0, 4.0);
        assert!(
            result1 ==  expected1 && result2 == expected2 && result3 == expected3 && result4 == expected4,
            "The position was not calculated correctly."
        );
    }

    #[test]
    fn compute_ray_sphere_intersect() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let result = Ray::intersect(r, s).unwrap_or_default();
        let expected = [4.0, 6.0];
        assert!(
            result == expected,
            "The intersect was expected at point {:#?} but the result was {:#?}", expected, result
        );
    }

    #[test]
    fn compute_ray_sphere_tangent_intersect() {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let result = Ray::intersect(r, s).unwrap_or_default();
        let expected = [5.0, 5.0];
        assert!(
            result == expected,
            "The intersect was expected at point {:#?} but the result was {:#?}, both values in the array should be equal.", expected, result
        )
    }

    #[test]
    fn compute_no_ray_sphere_intersect() {
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let result = Ray::intersect(r, s);
        let expected: Option<[f32; 2]> = None;
        assert!(
            result == expected,
            "The intersect was expected at point {:#?} but the result was {:#?}, both values in the array should be equal.", expected, result
        )
    }

    #[test]
    fn compute_ray_originating_in_sphere_interesect() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let result = Ray::intersect(r, s).unwrap_or_default();
        let expected = [-1.0, 1.0];
        assert!(
            result == expected,
            "The intersect was expected at point {:#?} but the result was {:#?}, both values in the array should be equal.", expected, result
        )
    }

    #[test]
    fn compute_sphere_behind_ray_intersect() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let result = Ray::intersect(r, s).unwrap_or_default();
        let expected = [-6.0, -4.0];
        assert!(
            result == expected,
            "The intersect was expected at point {:#?} but the result was {:#?}", expected, result
        );
    }
}