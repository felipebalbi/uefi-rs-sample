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

#[cfg(test)]
mod test {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn buffer_defaults_to_black() {
        let buffer = Buffer::new(100, 200);

        assert_eq!(buffer.width, 100);
        assert_eq!(buffer.height, 200);
        assert_eq!(size_of::<Buffer>(), 100 * 200 * size_of::<BltPixel>());

        let pixels = buffer.pixels;

        for pixel in pixels.into_iter() {
            assert_eq!(pixel.red, 0);
            assert_eq!(pixel.green, 0);
            assert_eq!(pixel.blue, 0);
        }
    }
}
