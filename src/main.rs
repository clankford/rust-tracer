use std::ops::Add;
use std::ops::Sub;
use std::cmp::Eq;
use std::ops::Neg;
use std::ops::Mul;
use std::ops::Div;

fn main() {
    let a = Tuple::point(4.3, -4.2, 3.1);
    println!("Point: {:#?},", a);
    let b = Tuple::vector(4.0, -4.0, 3.0);
    println!("Vector: {:#?}", b);
    println!("{}", a == b);
    println!("{}", f_equal(a.x, b.x));
    // The overloaded + operator for the Tuple type requires references to avoid copying the Tuple.
    let c = &a + &b;
    println!("{:#?}", c);
    let d = &a - &b;
    println!("{:#?}", d);
    println!("Negate a Tuple.");
    println!("{:#?}", -&d);
    println!("Multiple a vector by a scalar.");
    println!("{:#?}", &d * 1.22);
    println!("Divide a vector by a scalar.");
    println!("{:#?}", &d / 0.0373);
}

fn f_equal(a: f32, b: f32) -> bool {
    const EPSILON: f32 = 0.00001;
    let diff: f32 = a - b;
    if diff.abs() < EPSILON {
        true
    } else {
        false
    }
}

#[derive(Debug)]
struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    // When w = 1 the tuple is a point, when w = 0 the tuple is a vector.
    w: u8
}

impl Tuple {
    // Using field init shorthand because the function parameter names are the same
    // as the struct's field names.
    fn point(x: f32, y: f32, z: f32) -> Tuple {
        Tuple {
            x,
            y,
            z,
            w: 1
        }
    }

    // Using field init shorthand because the function parameter names are the same
    // as the struct's field names.
    fn vector(x: f32, y: f32, z: f32) -> Tuple {
        Tuple {
            x,
            y,
            z,
            w: 0
        }
    }
}

// To avoid copying/clone the Tuple type everytime a + operator is used, we are implementing
// the Add trait on the &Tuple (reference) type. 
impl Add for &Tuple {
    type Output = Tuple;

    // TODO: Return a Result so that an error can be returned if two points are 
    // attempted to be added together. Or Panic.
    fn add(self, other: &Tuple) -> Tuple {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w
        }
    }
}

// To avoid copying/clone the Tuple type everytime a - operator is used, we are implementing
// the Sub trait on the &Tuple (reference) type. 
impl Sub for &Tuple {
    type Output = Tuple;

    // TODO: Return a Result so that an error can be returned if two points are 
    // attempted to be added together. Or Panic.
    fn sub(self, other: &Tuple) -> Tuple {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w
        }
    }
}

// Adding <f32> allows us to dictate the type of RHS. In this case, it allows us to multiple a Tuple
// by an f32. 
// https://doc.rust-lang.org/book/ch19-03-advanced-traits.html?highlight=overload#default-generic-type-parameters-and-operator-overloading
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

// Adding <f32> allows us to dictate the type of RHS. In this case, it allows us to divide a Tuple
// by an f32. 
// https://doc.rust-lang.org/book/ch19-03-advanced-traits.html?highlight=overload#default-generic-type-parameters-and-operator-overloading
impl Div<f32> for &Tuple {
    type Output = Tuple;

    fn div(self, other: f32) -> Tuple {
        Tuple {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w
        }
    }
}

// Must overload PartialEq instead of leveraging Derive PartialEq on the Tuple struct. This is
// because we have a custom implementation for comparing floating point numbers f_equal.
impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        if f_equal(self.x, other.x) & f_equal(self.y, other.y) &
            f_equal(self.z, other.z) & (self.w == other.w) {
                true
            } else {
                false
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
            0, v.w,
            "The 'w' field of a vector should be 0, value was {}", v.w
        );
    }

    #[test]
    fn tuple_is_a_point() {
        let p = Tuple::point(1.0, 2.0, 3.0);
        assert_eq!(
            1, p.w,
            "The 'w' field of a point should be 1, value was {}", p.w
        );
    }

    #[test]
    fn values_are_equal() {
        let a: f32 = 1.222225;
        let b: f32 = 1.222226;
        let x: bool = f_equal(a, b);
        assert_eq!(
            true, x,
            "The values {} and {} should be equal = true, value was {}", a, b, x
        )
    }

    #[test]
    fn values_are_not_equal() {
        let a: f32 = 0.00001;
        let b: f32 = 0.000021;
        let x: bool = f_equal(a, b);
        assert_eq!(
            false, x,
            "The values {} and {} should be equal = false, value was {}", a, b, x
        )
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
    fn subtract_two_points() {
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
}