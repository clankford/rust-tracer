extern crate rust_tracer;
use crate::rust_tracer::ray_tracer::canvas::*;
use crate::rust_tracer::ray_tracer::tuple::*;


use std::io;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    
    let start = Tuple::point(0.0, 1.0, 0.0);
    let velocity = &Tuple::vector(1.0, 1.8, 0.0).norm() * 11.25;

    let mut p = Projectile {
        position: start,
        velocity: velocity
    };

    let gravity = Tuple::vector(0.0, -0.1, 0.0);
    let wind = Tuple::vector(-0.01, 0.0, 0.0);

    let e = Environment {
        gravity: gravity,
        wind: wind
    };

    let mut canvas = Canvas::new(900, 550);
    let plot_color = Tuple::color(255.0, 0.0, 0.0);
    
    let mut t = 0;

    println!("At t: {}, the position of the projectile is: {}, {}, {}.", t, p.position.x, p.position.y, p.position.z);

    let mut done = false;
    while !done {
        if p.position.y <= 0.0 {
            done = true;
        }
        else {
            p= tick(&e, p);
            t = t + 1;
            if p.position.x <= (canvas.width - 1) as f32 && p.position.y >= 0.0 {
                canvas.write_pixel(p.position.x as usize, canvas.height - p.position.y as usize, plot_color);
            }
            println!("At t: {}, the position of the projectile is: {}, {}, {}.", t, p.position.x, p.position.y, p.position.z);
        }
    }

    let ppm = canvas.canvas_to_ppm();
    create_ppm_file(ppm).expect("Failed to write image to file.");
}

// Takes a reference to Environment, as envronment is not modified during the tick function, it is
// likely to be used again.
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

fn create_ppm_file(ppm: String) -> io::Result<()> {
    let mut file = File::create("image.ppm")?;
    write!(file, "{}", ppm)?;
    Ok(())
}