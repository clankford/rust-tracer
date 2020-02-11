mod tuple;
use tuple::Tuple;

fn main() {
    
    let mut p = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.0, 0.0).norm()
    };

    let e = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0)
    };

    let mut t = 1;
    let mut done = false;
    while !done {
        if p.position.y <= 0.0 {
            done = true;
        }
        else {
            p = tick(&e, p);
            t = t + 1;
            println!("{:#?}", p.position);
        }
    }
}

fn tick(env: &Environment, proj: Projectile) -> Projectile {
    let position = &proj.position + &proj.velocity;
    let velocity = &(&proj.velocity + &env.gravity) + &env.wind;
    
    Projectile {
        position: position,
        velocity: velocity
    }
}

struct Projectile {
    position: Tuple,
    velocity: Tuple
}

struct Environment {
    gravity: Tuple,
    wind: Tuple
}