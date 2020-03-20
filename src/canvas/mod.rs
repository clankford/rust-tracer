use crate::Tuple;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Tuple>
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            // Create vector with right length then set everything to black
            pixels: vec![Tuple::color(0.0, 0.0, 0.0); width * height]
        }
    }

    // TODO: Add error handling for out of bounds
    pub fn pixel_at(&self, x: usize, y: usize) -> Tuple {
        self.pixels[(x * y) - 1]
    }

    // TODO: Add error handling for out of bounds
    pub fn write_pixel(&mut self, x: usize, y: usize, color: Tuple) {
        self.pixels[(x * y) - 1] = color
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_canvas() {
        let c = Canvas::new(10, 20);
        let length = c.pixels.len();
        assert!(c.width == 10 && c.height == 20 && length == 200, 
            "Result: width = {}, height = {}, length = {}; 
            Expected: width = 10, height = 20, length = 200", c.width, c.height, length);
    }

    #[test]
    fn update_pixel_color() {
        let mut c = Canvas::new(10, 20);
        let new_pixel = Tuple::color(1.0, 1.0, 1.0);
        c.write_pixel(10, 20, new_pixel);
        let p = c.pixel_at(10, 20);
        assert!(p == Tuple::color(1.0, 1.0, 1.0),
            "Result: R = {}, G = {}, B = {};
            Expected: R = 1.0, G = 1.0, B = 1.0", p.x, p.y, p.z)
    }
}