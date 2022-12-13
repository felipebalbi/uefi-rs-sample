#![no_main]
#![no_std]
#![feature(abi_efiapi)]

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use log::info;
use micromath::{
    vector::{F32x2, Vector},
    F32Ext,
};
use uefi::{
    prelude::*,
    proto::console::gop::{BltOp, BltPixel, BltRegion, GraphicsOutput},
    table::boot::BootServices,
    Result,
};

struct Buffer {
    width: usize,
    height: usize,
    pixels: Vec<BltPixel>,
}

impl Buffer {
    fn new(width: usize, height: usize) -> Self {
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

    fn blit(&self, gop: &mut GraphicsOutput) -> Result {
        gop.blt(BltOp::BufferToVideo {
            buffer: &self.pixels,
            src: BltRegion::Full,
            dest: (0, 0),
            dims: (self.width, self.height),
        })
    }
}

trait Widget {
    fn render(&self, buffer: &mut Buffer);
}

pub struct Square {
    size: f32,
    position: F32x2,
    color: BltPixel,
}

impl Square {
    pub fn new(size: f32, position: F32x2, color: BltPixel) -> Self {
        Self {
            size,
            position,
            color,
        }
    }
}

impl Widget for Square {
    fn render(&self, buffer: &mut Buffer) {
        let begin_x = self.position.x as usize;
        let end_x = (self.position.x + self.size) as usize;

        for x in begin_x..end_x {
            let begin_y = self.position.y as usize;
            let end_y = (self.position.y + self.size) as usize;

            for y in begin_y..end_y {
                buffer.set_pixel(x, y, self.color);
            }
        }
    }
}

pub struct Rectangle {
    width: f32,
    height: f32,
    position: F32x2,
    color: BltPixel,
}

impl Rectangle {
    pub fn new(width: f32, height: f32, position: F32x2, color: BltPixel) -> Self {
        Self {
            width,
            height,
            position,
            color,
        }
    }
}

impl Widget for Rectangle {
    fn render(&self, buffer: &mut Buffer) {
        let begin_x = self.position.x as usize;
        let end_x = (self.position.x + self.width) as usize;

        for x in begin_x..end_x {
            let begin_y = self.position.y as usize;
            let end_y = (self.position.y + self.height) as usize;

            for y in begin_y..end_y {
                buffer.set_pixel(x, y, self.color);
            }
        }
    }
}

pub struct Circle {
    radius: f32,
    position: F32x2,
    color: BltPixel,
}

impl Circle {
    pub fn new(radius: f32, position: F32x2, color: BltPixel) -> Self {
        Self {
            radius,
            position,
            color,
        }
    }
}

impl Widget for Circle {
    fn render(&self, buffer: &mut Buffer) {
        let begin_x = (self.position.x - self.radius) as usize;
        let end_x = (self.position.x + self.radius) as usize;

        for x in begin_x..end_x {
            let begin_y = (self.position.y - self.radius) as usize;
            let end_y = (self.position.y + self.radius) as usize;

            for y in begin_y..end_y {
                let pixel_position = F32x2 {
                    x: x as f32,
                    y: y as f32,
                };

                if self.position.distance(pixel_position).abs() <= self.radius {
                    buffer.set_pixel(x, y, self.color);
                }
            }
        }
    }
}

fn draw(bt: &BootServices) -> Result {
    // Open graphics output protocol.
    let gop_handle = bt.get_handle_for_protocol::<GraphicsOutput>()?;
    let mut gop = bt.open_protocol_exclusive::<GraphicsOutput>(gop_handle)?;

    // Get screen resolution
    let (width, height) = gop.current_mode_info().resolution();

    let mut current_buffer = 0;
    let mut buf1 = Buffer::new(width, height);
    let mut buf2 = Buffer::new(width, height);

    let background_rect = Rectangle::new(
        width as f32,
        height as f32,
        F32x2::default(),
        BltPixel::new(215, 215, 215),
    );
    background_rect.render(&mut buf1);
    background_rect.render(&mut buf2);

    loop {
        if current_buffer == 0 {
            render(height, &mut buf1, &mut gop)?;
            current_buffer = 1;
        } else {
            render(height, &mut buf1, &mut gop)?;
            current_buffer = 0;
        }
    }
}

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    info!("Hello World!");

    let bt = system_table.boot_services();
    draw(bt).unwrap();

    Status::SUCCESS
}

fn render(height: usize, buf: &mut Buffer, gop: &mut GraphicsOutput) -> Result {
    let rect = Rectangle::new(
        50.0,
        100.0,
        F32x2 {
            x: 100.0,
            y: height as f32 / 2.0,
        },
        BltPixel::new(255, 0, 0),
    );
    rect.render(buf);

    let square = Square::new(
        50.0,
        F32x2 {
            x: 200.0,
            y: height as f32 / 2.0,
        },
        BltPixel::new(0, 255, 0),
    );
    square.render(buf);

    let circle = Circle::new(
        50.0,
        F32x2 {
            x: 400.0,
            y: height as f32 / 2.0,
        },
        BltPixel::new(0, 0, 255),
    );
    circle.render(buf);

    buf.blit(gop)
}
