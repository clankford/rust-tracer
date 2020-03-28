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
        let i = self.get_index(x, y);
        self.pixels[i]
    }

    // TODO: Add error handling for out of bounds
    pub fn write_pixel(&mut self, x: usize, y: usize, color: Tuple) {
        let i = self.get_index(x, y);
        self.pixels[i] = color
    }

    pub fn canvas_to_ppm(&self) -> String {
        let header = create_ppm_header(self.width, self.height);
        let body = create_ppm_body(&self.pixels, self.width);

        header
    }

    fn get_index(&self, width: usize, height: usize) ->  usize {
        width * self.height + height
    }
}

fn create_ppm_header(width: usize, height: usize) -> String {
    format!("P3\n{} {}\n255\n", width, height)
}

fn create_ppm_body(pixels: &Vec<Tuple>, width: usize) -> String {
    
    let color_scale: f32 = 255.0;

    let mut flat = Vec::new();
    for pixel in pixels {
        flat.push(pixel.x);
        flat.push(pixel.y);
        flat.push(pixel.z);
    }

    let trans: Vec<i32> = flat.iter()
        .map(|i| {
            (i * color_scale).round() as i32
        })
        .map(|i| {
            if i > 255 {
                255
            }
            else if i < 0 {
                0
            }
            else {
                i
            }
        }).collect();

    let mut body = String::new();
    let mut i = 1;
    for e in trans {
        let s;
        if i % (width * 3) == 0 {
            s = format!("{}\n", e);
        }
        else {
            s = format!("{} ", e);
        }
        body.push_str(&s);
        i = i + 1;
    }
    body.to_string()
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
        c.write_pixel(9, 19, new_pixel);
        let p = c.pixel_at(9, 19);
        assert!(p == Tuple::color(1.0, 1.0, 1.0),
            "Result: R = {}, G = {}, B = {};
            Expected: R = 1.0, G = 1.0, B = 1.0", p.x, p.y, p.z)
    }

    #[test]
    fn construct_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = c.canvas_to_ppm();
        assert!(ppm == "P3\n5 3\n255\n", "Result: PPM = \n{}Expected: PPM = \nP3\n5 3\n255", ppm);
    }

    #[test]
    fn costruct_ppm_body() {
        let mut c = Canvas::new(5,3);
        let c1 = Tuple::color(1.5, 0.0, 0.0);
        let c2 = Tuple::color(0.0, 0.5, 0.0);
        let c3 = Tuple::color(-0.5, 0.0, 1.0);
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);
        let ppm = create_ppm_body(&c.pixels, c.width);
        let row1 = String::from("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n");
        let row2 = String::from("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n");
        let row3 = String::from("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n");
        let exp = row1 + &row2 + &row3;
        assert!(ppm == exp,
                "\nResult: PPM = \n{:?}\nExpected: PPM = \n{:?}", ppm, exp);
    }
}