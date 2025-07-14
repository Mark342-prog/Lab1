use raylib::prelude::*;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    color_buffer: Image,
    background_color: Color,
    current_color: Color,
    pixels: Vec<Vec<Color>>,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, background_color);
        let pixels = vec![vec![background_color; width as usize]; height as usize];
        Framebuffer {
            width: width as usize,
            height: height as usize,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
            pixels,
        }
    }

    pub fn clear(&mut self) {

        self.color_buffer = Image::gen_image_color(self.width as i32, self.height as i32, self.background_color);
        self.pixels = vec![vec![self.background_color; self.width]; self.height];
    }

    pub fn set_pixel_current(&mut self, x: u32, y: u32) {
        if x < self.width as u32 && y < self.height as u32 {
            self.color_buffer.draw_pixel(x as i32, y as i32, self.current_color);
            self.pixels[y as usize][x as usize] = self.current_color;
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.color_buffer.draw_pixel(x as i32, y as i32, color);
            self.pixels[y][x] = color;
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        if x < self.width && y < self.height {
            self.pixels[y][x]
        } else {
            Color::BLACK 
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
        self.clear();
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn render_to_file(&mut self, file_path: &str) {
        self.color_buffer.export_image(file_path);
    }
}
