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
    let mut i_width = 1;
    let mut i_chars = 0;
    let mut inserted_whitespace = false;
    for e in trans {
        let mut insert_val: String = "".to_owned();
        let val = format!("{}", e);
        println!("i_chars + val.len() = {}", i_chars + val.len());
        // val.len() + 1 - the +1 is to account for the white space that would need to be added
        if i_chars + val.len() + 1 > 70 {
            println!("End of chars");
            insert_val.push_str("\n");
            insert_val.push_str(&val);
            i_chars = 0;
            i_chars = i_chars + val.len();
        }
        else if i_width % (width * 3) == 0 {
            println!("End of row");
            insert_val.push_str(" ");
            insert_val.push_str(&val);
            insert_val.push_str("\n");
            inserted_whitespace = true;
            i_chars = 0;
        }
        else {
            if inserted_whitespace || i_width == 1 {
                insert_val.push_str(&val);
                inserted_whitespace = false;
            }
            else {
                insert_val.push_str(" ");
                insert_val.push_str(&val);
            }
            i_chars = i_chars + insert_val.len();
        }
        body.push_str(&insert_val);
        i_width = i_width + 1;
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
        let mut c = Canvas::new(5, 3);
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

    #[test]
    fn construct_ppm_body_line_break() {
        let mut c = Canvas::new(10, 2);
        let color = Tuple::color(1.0, 0.8, 0.6);
        c.pixels = vec![color; c.width * c.height];
        let ppm = create_ppm_body(&c.pixels, c.width);
        let row1 = String::from("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n");
        let row2 = String::from("153 255 204 153 255 204 153 255 204 153 255 204 153\n");
        let row3 = String::from("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n");
        let row4 = String::from("153 255 204 153 255 204 153 255 204 153 255 204 153\n");
        let exp = row1 + &row2 + &row3 +&row4;
        assert!(ppm == exp,
            "\nResult: PPM = \n{:?}\nExpected: PPM = \n{:?}", ppm, exp);
    }
}