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

struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    // When w = 1 the tuple is a point, when w = 0 the tuple is a vector.
    w: u8
}