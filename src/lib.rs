#![no_main]
#![no_std]
#![feature(abi_efiapi)]

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use uefi::{
    proto::console::gop::{BltOp, BltPixel, BltRegion, GraphicsOutput},
    Result,
};

pub mod shapes;

pub struct Buffer {
    width: usize,
    height: usize,
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

    fn set_pixel(&mut self, x: usize, y: usize, color: BltPixel) {
        if let Some(pixel) = self.pixels.get_mut(y * self.width + x) {
            pixel.red = color.red;
            pixel.green = color.green;
            pixel.blue = color.blue;
        }
    }

    pub fn blit(&self, gop: &mut GraphicsOutput) -> Result {
        gop.blt(BltOp::BufferToVideo {
            buffer: &self.pixels,
            src: BltRegion::Full,
            dest: (0, 0),
            dims: (self.width, self.height),
        })
    }
}
