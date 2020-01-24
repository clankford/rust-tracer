fn main() {
    let a = point(4.3, -4.2, 3.1);
    println!("Point: {}, {}, {}, {}", a.x, a.y, a.z, a.w);
    let b = vector(4.0, -4.0, 3.0);
    println!("Vector: {}, {}, {}, {}", b.x, b.y, b.z, b.w);
}

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

fn equal(a: f32, b: f32) -> bool {
    const EPSILON: f32 = 0.00001;
    let diff: f32 = a - b;
    if diff.abs() < EPSILON {
        true
    } else {
        false
    }
}

struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    // When w = 1 the tuple is a point, when w = 0 the tuple is a vector.
    w: u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vector() {
        let v = vector(1.0, 2.0, 3.0);
        assert_eq!(
            0, v.w,
            "The 'w' field of a vector should be 0, value was {}", v.w
        );
    }

    #[test]
    fn is_point() {
        let p = point(1.0, 2.0, 3.0);
        assert_eq!(
            1, p.w,
            "The 'w' field of a point should be 1, value was {}", p.w
        );
    }

    #[test]
    fn values_are_equal() {
        let a: f32 = 1.222225;
        let b: f32 = 1.222226;
        assert_eq!(
            true, equal(a, b),
            "The values {} and {} should be equal = true, value was {}", a, b, equal(a,b)
        )
    }

    #[test]
    fn values_are_not_equal() {
        let a: f32 = 0.00001;
        let b: f32 = 0.000021;
        assert_eq!(
            true, !equal(a, b),
            "The values {} and {} should be equal = false, value was {}", a, b, equal(a,b)
        )
    }
}