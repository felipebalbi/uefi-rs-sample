#![no_main]
#![no_std]
#![feature(abi_efiapi)]

extern crate alloc;

use log::info;
use micromath::vector::F32x2;
use uefi::{
    prelude::*,
    proto::console::gop::{BltOp, BltPixel, BltRegion, GraphicsOutput},
    table::boot::BootServices,
    Result,
};
use uefi_rs_sample::{
    buffer::Buffer,
    shapes::{Circle, Rectangle, Shape, Square},
};

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
            render(&mut buf1, &mut gop)?;
            current_buffer = 1;
        } else {
            render(&mut buf1, &mut gop)?;
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

fn render(buf: &mut Buffer, gop: &mut GraphicsOutput) -> Result {
    let rect = Rectangle::new(
        50.0,
        100.0,
        F32x2 {
            x: 100.0,
            y: buf.height as f32 / 2.0,
        },
        BltPixel::new(255, 0, 0),
    );
    rect.render(buf);

    let square = Square::new(
        50.0,
        F32x2 {
            x: 200.0,
            y: buf.height as f32 / 2.0,
        },
        BltPixel::new(0, 255, 0),
    );
    square.render(buf);

    let circle = Circle::new(
        50.0,
        F32x2 {
            x: 400.0,
            y: buf.height as f32 / 2.0,
        },
        BltPixel::new(0, 0, 255),
    );
    circle.render(buf);

    gop.blt(BltOp::BufferToVideo {
        buffer: buf.get(),
        src: BltRegion::Full,
        dest: (0, 0),
        dims: (buf.width, buf.height),
    })
}
