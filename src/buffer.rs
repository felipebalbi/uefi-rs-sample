use alloc::vec;
use alloc::vec::Vec;
use uefi::proto::console::gop::BltPixel;

pub struct Buffer {
    pub width: usize,
    pub height: usize,
    pixels: Vec<BltPixel>,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![BltPixel::new(0, 0, 0); width * height],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: BltPixel) {
        if let Some(pixel) = self.pixels.get_mut(y * self.width + x) {
            pixel.red = color.red;
            pixel.green = color.green;
            pixel.blue = color.blue;
        }
    }

    pub fn get(&self) -> &Vec<BltPixel> {
        &self.pixels
    }
}
