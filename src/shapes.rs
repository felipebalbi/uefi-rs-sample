use crate::buffer::Buffer;
use micromath::{
    vector::{F32x2, Vector},
    F32Ext,
};
use uefi::proto::console::gop::BltPixel;

pub trait Shape {
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

impl Shape for Square {
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

impl Shape for Rectangle {
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

impl Shape for Circle {
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
