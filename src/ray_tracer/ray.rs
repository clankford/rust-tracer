use crate::ray_tracer::tuple::Tuple;
use crate::ray_tracer::sphere::Sphere;
use crate::ray_tracer::intersection::Intersection;

#[cfg(test)]
use crate::ray_tracer::traits::object::Object;



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

impl <'a> Ray {

    // Returns an Option so that None can be returned if the ray does not intersect with the sphere
    // Returns None if the ray does not intersect with the object and returns a vector of intersections
    // if there is an intersection with the object.
    pub fn intersect(r: Ray, s: &'a Sphere) -> Option<Vec<Intersection<'a, Sphere>>> { 
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
            Some(vec![Intersection::new(t1, s), Intersection::new(t2, s)])
        }
    }

    // Returns the closest positive intersection to the Ray's origin.
    pub fn hit<'b>(is: &'b Vec<Intersection<'a, Sphere>>) -> Option<&'b Intersection<'a, Sphere>> {
        let mut closest = &is[0];
        for i in 1..is.len() {
            if closest.t < 0.0 && is[i].t >= 0.0 {
                closest = &is[i];
            } else if is[i].t < closest.t && is[i].t >= 0.0 {
                closest = &is[i];
            }
        }
        if closest.t < 0.0 {
            None
        } else {
            Some(closest)
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
        )
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
        )
    }

    #[test]
    fn compute_ray_sphere_intersect() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let result = Ray::intersect(r, &s).unwrap();
        let expected = vec![Intersection::new(4.0, &s), Intersection::new(6.0, &s)];
        assert!(
            (result[0].t == expected[0].t) && (result[1].t == expected[1].t),
            "The t values of the intersection were not calculated correctly!"
        )
    }

    #[test]
    fn compute_ray_sphere_tangent_intersect() {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let result = Ray::intersect(r, &s).unwrap();
        let expected = vec![Intersection::new(5.0, &s), Intersection::new(5.0, &s)];
        assert!(
            (result[0].t == expected[0].t) && (result[1].t == expected[1].t),
            "The t values of the intersection were not calculated correctly!"
        )
    }

    #[test]
    fn compute_no_ray_sphere_intersect() {
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let result = Ray::intersect(r, &s);
        assert!(
            result.is_none(),
            "The result was not None, meaning there was an intersection found when there isn't supposed to be an intersection."
        )
    }

    #[test]
    fn compute_ray_originating_in_sphere_interesect() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let result = Ray::intersect(r, &s).unwrap();
        let expected = vec![Intersection::new(-1.0, &s), Intersection::new(1.0, &s)];
        assert!(
            (result[0].t == expected[0].t) && (result[1].t == expected[1].t),
            "The t values of the intersection were not calculated correctly!"
        )
    }

    #[test]
    fn compute_sphere_behind_ray_intersect() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let result = Ray::intersect(r, &s).unwrap();
        let expected = vec![Intersection::new(-6.0, &s), Intersection::new(-4.0, &s)];
        assert!(
            (result[0].t == expected[0].t) && (result[1].t == expected[1].t),
            "The t values of the intersection were not calculated correctly!"
        )
    }

    #[test]
    fn intersect_sets_object_on_intersection() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let result = Ray::intersect(r, &s).unwrap();
        let expected = &s;
        assert!(
            result[0].object == expected,
            "The object was not set correctly in the intersect function."
        )
    }

    #[test]
    fn find_hit_all_positive() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let is = vec![i1, i2];
        let result = Ray::hit(&is).unwrap();
        let expected = Intersection::new(1.0, &s);
        assert!(
            result.t == expected.t,
            "The expected intersection was not returned."
        )
    }

    #[test]
    fn find_hit_one_negative_one_positive() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let is = vec![i1, i2];
        let result = Ray::hit(&is).unwrap();
        let expected = Intersection::new(1.0, &s);
        assert!(
            result.t == expected.t,
            "The expected intersection was not returned."
        )
    }

    #[test]
    fn find_hit_all_negative() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let is = vec![i1, i2];
        let result = Ray::hit(&is);
        assert!(
            result.is_none(),
            "The expected intersection was not returned."
        )
    }

    #[test]
    fn find_hit_many_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let is = vec![i1, i2, i3, i4];
        let result = Ray::hit(&is).unwrap();
        let expected = Intersection::new(2.0, &s);
        assert!(
            result.t == expected.t,
            "The expected intersection was not returned."
        )
    }
}