fn main() {
    let a = Tuple {
        x: 4.3,
        y: -4.2,
        z: 3.1,
        w: 1
    };
    println!("{}, {}, {}, {}", a.x, a.y, a.z, a.w);
}

struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: u8
}