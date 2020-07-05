extern crate rust_tracer;
use crate::rust_tracer::ray_tracer::canvas::*;
use crate::rust_tracer::ray_tracer::tuple::*;
use crate::rust_tracer::ray_tracer::ray::*;
use crate::rust_tracer::ray_tracer::sphere::*;
use crate::rust_tracer::ray_tracer::traits::object::*;
use crate::rust_tracer::ray_tracer::matrix::*;
use crate::rust_tracer::ray_tracer::material::*;
use crate::rust_tracer::ray_tracer::light::*;



use std::io;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    sphere_shadow_test();
    projectile_test();
}

fn sphere_shadow_test() {
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100;
    // Size of a single pixel in world space units.
    let pixel_size = wall_size / canvas_pixels as f32;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    //let mut shape = Sphere::new();
    let mut shape = Sphere { transform: &Matrix::shearing(0.25, 0.0, 0.0, 0.0, 0.0, 0.0) * 
                                    &Matrix::scaling(0.5, 1.0, 1.0), 
                                    ..Default::default() };
    
    // Create and apply a material to the shape.
    shape.material = Material::new();
    shape.material.color = Tuple::color(1.0, 0.5, 0.75);
    
    // Create a light source for the scene
    let light_position = Tuple::point(10.0, 10.0, -10.0);
    let light_color = Tuple::color(1.0, 1.0, 1.0);
    let light = Light::new(light_color, light_position);

    // For each row of pixels in the canvas
    for y in 0..canvas_pixels - 1 {
        // Compute the world y coordinate (top = +half, bottom = -half)
        let world_y = half - pixel_size * y as f32;

        for x in 0..canvas_pixels - 1 {
            // Compute the world x coordinate (left = -half, right = half)
            let world_x = -half + pixel_size * x as f32;
            // Describe the point on the wall that the ray will target
            let position = Tuple::point(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, (&position - &ray_origin).norm());
            let xs = r.intersect(&shape);

            match xs {
                None => continue,
                Some(i) => {
                    match Ray::hit(&i) {
                        None => continue,
                        Some(j) => {
                            let point = r.position(j.t);
                            let normal = shape.normal_at(point);
                            let eye = -&r.direction;
                            let color = light.lighting(&shape.material, point, eye, normal);
                            canvas.write_pixel(x, y, color)
                        }
                    }
                }
            }
        }
    }

    let ppm = canvas.canvas_to_ppm();
    create_ppm_file(ppm, "Sphere_Shadow").expect("Failed to write image to file.");
}

fn projectile_test() {
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
    create_ppm_file(ppm, "Projectile").expect("Failed to write image to file.");
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

fn create_ppm_file(ppm: String, file_name: &str) -> io::Result<()> {
    let name = format!("images/{}.ppm", file_name);
    let mut file = File::create(name)?;
    write!(file, "{}", ppm)?;
    Ok(())
}