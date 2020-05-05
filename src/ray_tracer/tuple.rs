use std::ops::{Add,Sub,Neg,Mul,Div};
use std::cmp::Eq;
use std::f32;
use crate::ray_tracer::common::f_equal;



// TODO: Is there a way to create Structs for Vector & Point and refactor this module to handle the
// operators against those concrete types without duplicating functionality?
// Add can add any two tuples together. Dot can only be done on two vectors.
// Trait Tuple that has an add function with default functionality that adds two tuples together
// --Currently no way to do the above within a default implementation.
// Vector implements Tuple, inherits default add functionality.
// Point implements Tuple, inherits default add functionality.
// Vector has method dot, can only dot two Vectors
// Point does not have method dot.

#[derive(Copy, Clone, Debug)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    // When w = 1 the tuple is a point, when w = 0 the tuple is a vector.
    pub w: Option<u8>
}

// TODO: Should implement methods for each operator in addition to overloading. 
// This will be for completion. I can't find an elegant solution for this without duplicating
// the implementation of (for instance) the add method in the method itself and the overloaded +
// operator.
impl Tuple {
    
    // Using field init shorthand because the function parameter names are the same
    // as the struct's field names.
    pub fn point(x: f32, y: f32, z: f32) -> Tuple {
        Tuple {
            x,
            y,
            z,
            w: Some(1)
        }
    }

    // Using field init shorthand because the function parameter names are the same
    // as the struct's field names.
    pub fn vector(x: f32, y: f32, z: f32) -> Tuple {
        Tuple {
            x,
            y,
            z,
            w: Some(0)
        }
    }

    pub fn color(x: f32, y: f32, z: f32) -> Tuple {
        Tuple {
            x,
            y,
            z,
            w: None
        }
    }

    // Magnitude of a point is 0.
    pub fn mag(&self) -> f32 {
        match self.w {
            Some(0) => (self.x.powi(2) + self.y.powi(2) +  self.z.powi(2)).sqrt(),
            Some(1) => 0.0,
            _ => panic!("Cannot take the magnitude of a color"),
        }
    }

    // Can only normalize a vector.
    pub fn norm(&self) -> Tuple {
        match self.w {
            Some(0) => {
                let m: f32 = self.mag();
                Tuple {
                    x: self.x / m,
                    y: self.y / m,
                    z: self.z / m,
                    w: Some(0)
                }
            },
            Some(1) => panic!("Cannot normalize a point, w = Some(1)."),
            _ => panic!("Cannot normalize a color, w = None"),
        }
    }

    // Can only take a cross product with two vectors.
    pub fn cross(&self, other: Tuple) -> Tuple {
        match (self.w, other.w) {
            (Some(0), Some(0)) => {
                Tuple {
                    x: self.y * other.z - self.z * other.y,
                    y: self.z * other.x - self.x * other.z,
                    z: self.x * other.y - self.y * other.x,
                    w: Some(0)
                }
            },
            _ => panic!("Can only take the cross product of two vectors."),
        }
    }

    // Can only take the Hadamard Product of two of the same type of Tuple
    pub fn hadamard_product(&self, other: Tuple) -> Tuple {
        match (self.w, other.w) {
            (Some(0), Some(0)) |
            (Some(1), Some(1)) |
            (None, None) => {
                Tuple {
                    x: self.x * other.x,
                    y: self.y * other.y,
                    z: self.z * other.z,
                    w: self.w
                }
            },
            _ => panic!("Can only take the Hadamard Product of two of the same type of Tuple.")
        }
    }
}


// To avoid copying/cloning the Tuple type everytime a + operator is used, we are implementing
// the Add trait on the &Tuple (reference) type. 
impl Add for &Tuple {
    type Output = Tuple;

    // Only allow for vector + vector or point + vector or color + color.
    fn add(self, other: &Tuple) -> Tuple {
        match (self.w, other.w) {
            (Some(0), Some(0)) |
            (Some(1), Some(0)) |
            (Some(0), Some(1)) => {
                Tuple {
                    x: self.x + other.x,
                    y: self.y + other.y,
                    z: self.z + other.z,
                    // Safe to use unwrap here because we know that self.w or other.w is
                    // never going to be None, due to the match expression above.
                    w: Some(self.w.unwrap() + other.w.unwrap())
                }
            },
            (None, None) => {
                Tuple {
                    x: self.x + other.x,
                    y: self.y + other.y,
                    z: self.z + other.z,
                    w: None
                }
            },
            _ => panic!("Only points and vectors can be added to vectors, or color added to color.")
        }
    }
}

// To avoid copying/clone the Tuple type everytime a - operator is used, we are implementing
// the Sub trait on the &Tuple (reference) type. 
impl Sub for &Tuple {
    type Output = Tuple;

    // Only allow for vector - vector, point - vector, point - point, color - color, 
    // but not vector - point.
    fn sub(self, other: &Tuple) -> Tuple {
        match (self.w, other.w) {
            (Some(0), Some(0)) |
            (Some(1), Some(0)) |
            (Some(1), Some(1)) => {
                Tuple {
                    x: self.x - other.x,
                    y: self.y - other.y,
                    z: self.z - other.z,
                    // Safe to use unwrap here because we know that self.w or other.w is
                    // never going to be None, due to the match expression above.
                    w: Some(self.w.unwrap() - other.w.unwrap())
                }
            },
            (None, None) => {
                Tuple {
                    x: self.x - other.x,
                    y: self.y - other.y,
                    z: self.z - other.z,
                    w: None
                }
            },
            _ => panic!("Can only subtract two vectors, two points, two colors, or a point from a vector.")
        }
    }
}

// Adding <f32> allows us to dictate the type of RHS. In this case, it allows us to multiply a Tuple
// by an f32. 
// https://doc.rust-lang.org/book/ch19-03-advanced-traits.html?highlight=overload#default-generic-type-parameters-and-operator-overloading
// Tuple * Scalar
impl Mul<f32> for &Tuple {
    type Output = Tuple;

    fn mul(self, other: f32) -> Tuple {
        Tuple {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w
        }
    }
}

// Acts as the dot product for vectors.
impl Mul for &Tuple {
    type Output = f32;

    fn mul(self, other: &Tuple) -> f32 {
        match (self.w, other.w) {
            (Some(0), Some(0)) |
            (Some(1), Some(0)) |
            (Some(0), Some(1)) => {
                self.x * other.x + self.y * other.y + self.z * other.z
            },
            _ => panic!("Can't take the dot product of two points or with colors.")
        }
    }
}

// Adding <f32> allows us to dictate the type of RHS. In this case, it allows us to divide a Tuple
// by an f32. 
// https://doc.rust-lang.org/book/ch19-03-advanced-traits.html?highlight=overload#default-generic-type-parameters-and-operator-overloading
// Tuple / scalar
impl Div<f32> for &Tuple {
    type Output = Tuple;

    fn div(self, other: f32) -> Tuple {
        match self.w {
            None => panic!("Cannot divide using colors."),
            _ => {
                Tuple {
                    x: self.x / other,
                    y: self.y / other,
                    z: self.z / other,
                    w: self.w
                }
            }   
        }
    }
}

// Must overload PartialEq instead of leveraging Derive PartialEq on the Tuple struct. This is
// because we have a custom implementation for comparing floating point numbers f_equal.
impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        match (self.w, other.w) {
            (None, None) | (Some(0), Some(0)) | (Some(1), Some(1)) => {
                if f_equal(self.x, other.x) & f_equal(self.y, other.y) &
                    f_equal(self.z, other.z) {
                        true
                    } else {
                        false
                    }  
            }
        _ => false         
        }   
    }
}
impl Eq for Tuple {}

// To avoid copying/clone the Tuple type everytime a - operator is used, we are implementing
// the Neg trait on the &Tuple (reference) type. 
impl Neg for &Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_is_a_vector() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(
            0, v.w.unwrap(),
            "The 'w' field of a vector should be 0, value was {}", v.w.unwrap()
        );
    }

    #[test]
    fn tuple_is_a_point() {
        let p = Tuple::point(1.0, 2.0, 3.0);
        assert_eq!(
            1, p.w.unwrap(),
            "The 'w' field of a point should be 1, value was {}", p.w.unwrap()
        );
    }

    // TODO: This is a weak test for color. Check to see if w = None
    #[test]
    fn tuple_is_a_color() {
        let c = Tuple::color(-0.5, 0.4, 1.7);
        let red = -0.5;
        let green = 0.4;
        let blue = 1.7;
        assert!(red == c.x && green == c.y && blue == c.z, "r = {}, g = {}, b = {}", c.x, c.y, c.z);
    }

    #[test]
    fn vectors_are_equal() {
        let a = Tuple::vector(1.000001, 2.0, 3.0);
        let b = Tuple::vector(1.0, 2.0, 3.0);
        let x: bool = a == b;
        assert_eq!(
            true, x,
            "The vectors a and b should be equal = true, value was {}", x
        )
    }

    #[test]
    fn colors_are_equal() {
        let c1 = Tuple::color(0.5, 0.2, 0.3);
        let c2 = Tuple::color(0.5, 0.2, 0.3);
        let x: bool = c1 == c2;
        assert_eq!(
            true, x,
            "The colors a and b should be equal = true, value was {}", x
        )
    }

    #[test]
    fn add_vector_and_point() {
        let p = Tuple::point(3.0, -2.0, 5.0);
        let v = Tuple::vector(-2.0, 3.0, 1.0);
        let y: Tuple = &p + &v;
        let x: bool = Tuple::point(1.0, 1.0, 6.0) == y;
        assert_eq!(
            true, x,
            "The sum of the point and vector should equal (1, 1, 6, 1), value was {:#?}", y
        )
    }

    #[test]
    fn add_two_colors() {
        let c1 = Tuple::color(0.5, 0.2, 0.3);
        let c2 = Tuple::color(0.5, 0.2, 0.3);
        let c3 = &c1 + &c2;
        let x = Tuple::color(1.0, 0.4, 0.6) == c3;
        assert_eq!(
            true, x,
            "The sum of the colors equal (1.0, 0.4, 0.6), value was {:#?}", x
        )
    }

    #[test]
    fn subtract_two_points() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0, 6.0, 7.0);
        let y: Tuple = &p1 - &p2;
        let expected = Tuple::vector(-2.0, -4.0, -6.0);
        let x: bool = expected == y;
        assert_eq!(
            true, x,
            "The difference between the two points should equal {:#?}, value was {:#?}", expected, y
        )
    }

    #[test]
    fn subtract_two_colors() {
        let c1 = Tuple::color(0.5, 0.2, 0.3);
        let c2 = Tuple::color(0.3, 0.1, 0.1);
        let c3 = &c1 - &c2;
        let x = Tuple::color(0.2, 0.1, 0.2) == c3;
        assert_eq!(
            true, x,
            "The difference of the colors equal (1.0, 0.4, 0.6), value was {:#?}", x
        )
    }

    #[test]
    fn subtract_vector_from_point() {
        let p = Tuple::point(3.0, 2.0, 1.0);
        let v = Tuple::vector(5.0, 6.0, 7.0);
        let y: Tuple = &p - &v;
        let expected: Tuple = Tuple::point(-2.0, -4.0, -6.0);
        let x: bool = expected == y;
        assert_eq!(
            true, x,
            "The difference between the two points should equal {:#?}, value was {:#?}", expected, y
        )
    }

    #[test]
    fn subtract_two_vectors() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0, 6.0, 7.0);
        let y: Tuple = &p1 - &p2;
        let expected: Tuple = Tuple::vector(-2.0, -4.0, -6.0);
        let x: bool = expected == y;
        assert_eq!(
            true, x,
            "The difference between the two points should equal {:#?}, value was {:#?}", expected, y
        )
    }

    #[test]
    fn negate_tuple() {
        let p = Tuple::point(3.0, -2.0, 1.0);
        let expected = Tuple::point(-3.0, 2.0, -1.0);
        let y: Tuple = -&p;
        let x: bool = expected == y;
        assert_eq!(
            true, x,
            "The negation of the tuple should equal {:#?}, value was {:#?}", expected, y
        )
    }

    #[test]
    fn negate_tuple_no_moving() {
        let p = Tuple::point(3.0, -2.0, 1.0);
        let expected = Tuple::point(-3.0, 2.0, -1.0);
        let y: Tuple = -&p;
        let x: bool = expected == y;
        assert_eq!(
            true, x,
            "The negation of the tuple should equal {:#?}, value was {:#?}", expected, y
        )
    }

    #[test]
    fn multiply_tuple_by_scalar() {
        let p = Tuple::point(3.0, -2.0, 1.0);
        let expected = Tuple::point(10.5, -7.0, 3.5);
        let output: Tuple = &p * 3.5;
        let r: bool = expected == output;
        assert_eq!(
            true, r,
            "The multiplication of the tuple and scalar should equal {:#?}, value was {:#?}", expected, output
        )
    }

    #[test]
    fn multiply_tuple_by_fraction() {
        let p = Tuple::point(3.0, -2.0, 1.0);
        let expected = Tuple::point(1.5, -1.0, 0.5);
        let output: Tuple = &p * 0.5;
        let r: bool = expected == output;
        assert_eq!(
            true, r,
            "The multiplication of the tuple and fraction should equal {:#?}, value was {:#?}", expected, output
        )
    }

    #[test]
    fn multiply_color_by_scalar() {
        let c = Tuple::color(0.7, 0.4, 0.3);
        let expected = Tuple::color(1.4, 0.8, 0.6);
        let output: Tuple = &c * 2.0;
        let r: bool = expected == output;
        assert_eq!(
            true, r,
            "The multiplication of the color and scalar should equal {:#?}, value was {:#?}", expected, output
        )
    }

    #[test]
    fn divide_tuple_by_scalar() {
        let p = Tuple::point(3.0, -2.0, 1.0);
        let expected = Tuple::point(6.0, -4.0, 2.0);
        let output: Tuple = &p / 0.5;
        let r: bool = expected == output;
        assert_eq!(
            true, r,
            "The division of the tuple and scalar should equal {:#?}, value was {:#?}", expected, output
        )
    }

    #[test]
    fn magnitude_of_1_dim_vec() {
        let v = Tuple::vector(1.0, 0.0, 0.0);
        let expected = 1.0;
        let output: f32 = v.mag();
        let r: bool =  f_equal(expected, output);
        assert_eq!(
            true, r,
            "The magnitude of the vector should equal {:#?}, value was {:#?}", expected, output
        )
    }

    #[test]
    fn magnitude_of_vector() {
        let v = Tuple::vector(1.0, -2.0, 3.0);
        let expected = 3.74165;
        let output: f32 = v.mag();
        let r: bool = f_equal(expected, output);
        assert_eq!(
            true, r,
            "The magnitude of the vector should equal {:#?}, value was {:#?}", expected, output
        )
    }

    #[test]
    fn normalize_1_dim_vec() {
        let v = Tuple::vector(4.0, 0.0, 0.0);
        let expected = Tuple::vector(1.0, 0.0, 0.0);
        let output: Tuple = v.norm();
        let r: bool = expected == output;
        assert_eq!(
            true, r,
            "The normalized vector should equal {:#?}, value was {:#?}", expected, output
        )
    }

    #[test]
    fn normalize_vec() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        let expected = Tuple::vector(0.26726, 0.53452, 0.80178);
        let output: Tuple = v.norm();
        let r: bool = expected == output;
        assert_eq!(
            true, r,
            "The normalized vector should equal {:#?}, value was {:#?}", expected, output
        )
    }

    #[test]
    fn mag_of_normalized_vector() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        let expected = 1.0;
        let nv = v.norm();
        let output: f32 = nv.mag();
        let r: bool = f_equal(expected, output);
        assert_eq!(
            true, r,
            "The magnitude of the vector should equal {:#?}, value was {:#?}", expected, output
        )
    }

    #[test]
    fn dot_product_of_vector() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.0, 4.0);
        let expected = 20.0;
        let output = &v1 * &v2;
        let r: bool = expected == output;
        assert_eq!(
            true, r,
            "The dot product of the vectors should equal {:#?}, value was {:#?}", expected, output
        )
    }

    #[test]
    fn hadamard_product_of_two_colors() {
        let c1 = Tuple::color(1.0, 2.0, 3.0);
        let c2 = Tuple::color(2.0, 3.0, 4.0);
        let expected = Tuple::color(2.0, 6.0, 12.0);
        let output = c1.hadamard_product(c2);
        let r: bool = expected == output;
        assert_eq!(
            true, r,
            "The dot product of the vectors should equal {:#?}, value was {:#?}", expected, output
        )
    }

    #[test]
    fn cross_product_of_vector() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.0, 4.0);
        let expected = Tuple::vector(-1.0, 2.0, -1.0);
        let output = v1.cross(v2);
        let r: bool = expected == output;
        assert_eq!(
            true, r,
            "The cross product of the vectors should equal {:#?}, value was {:#?}", expected, output
        )
    }

    #[test]
    fn cross_product_of_vector_reverse() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.0, 4.0);
        let expected = Tuple::vector(1.0, -2.0, 1.0);
        let output = v2.cross(v1);
        let r: bool = expected == output;
        assert_eq!(
            true, r,
            "The cross product of the vectors should equal {:#?}, value was {:#?}", expected, output
        )
    }
}