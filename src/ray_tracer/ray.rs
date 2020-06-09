use crate::ray_tracer::tuple::Tuple;

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
}