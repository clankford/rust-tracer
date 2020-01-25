fn main() {
    let a = Tuple::point(4.3, -4.2, 3.1);
    println!("Point: {}, {}, {}, {}", a.x, a.y, a.z, a.w);
    let b = Tuple::vector(4.0, -4.0, 3.0);
    println!("Vector: {}, {}, {}, {}", b.x, b.y, b.z, b.w);
    println!("{}", a.equal(&b));
    println!("{}", f_equal(a.x, b.x));
    let c = a.add(&b);
    println!("{}, {}, {}, {}", c.x, c.y, c.z, c.w);
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

    // Takes a reference to a Tuple object since it is not operating on the values
    // passed to it. This allows the Tuple objects to stay in scope for other operations.
    fn equal(&self, a: &Tuple) -> bool {
        if f_equal(a.x, self.x) & f_equal(a.y, self.y) &
            f_equal(a.z, self.z) & (a.w == self.w) {
                true
            } else {
                false
            }
    }

    // TODO: Return a Result so that an error can be returned if two points are 
    // attempted to be added together.
    fn add(&self, a: &Tuple) -> Tuple {
        Tuple {
            x: a.x + self.x, 
            y: a.y + self.y, 
            z: a.z + self.z, 
            w: a.w + self.w
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vector() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(
            0, v.w,
            "The 'w' field of a vector should be 0, value was {}", v.w
        );
    }

    #[test]
    fn is_point() {
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
        let x: bool = a.equal(&b);
        assert_eq!(
            true, x,
            "The vectors a and b should be equal = true, value was {}", x
        )
    }

    #[test]
    fn add_vector_and_point() {
        let p = Tuple::point(3.0, -2.0, 5.0);
        let v = Tuple::vector(-2.0, 3.0, 1.0);
        let y: Tuple = p.add(&v);
        let x: bool = (&Tuple::point(1.0, 1.0, 6.0)).equal(&y);
        assert_eq!(
            true, x,
            "The sum of the point and vector should equal (1, 1, 6, 1), value was {:#?}", y
        )
    }
}