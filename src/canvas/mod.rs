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
}