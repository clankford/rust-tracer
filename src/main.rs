mod tuple;

fn main() {
    let a = tuple::Tuple::point(4.3, -4.2, 3.1);
    println!("Point: {:#?},", a);
    let b = tuple::Tuple::vector(4.0, -4.0, 3.0);
    println!("Vector: {:#?}", b);
    println!("{}", a == b);
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
    println!("Magnitude a vector");
    println!("{:#?}", b.mag());
    println!("Normalized vector");
    println!("{:#?}", b.norm());
    println!("Dot product of vectors");
    println!("{:#?}", &b * &-&b);
    println!("Cross product of vectors");
    println!("{:#?}", b.cross(-&b));
}