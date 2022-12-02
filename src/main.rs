#![no_main]
#![no_std]
#![feature(abi_efiapi)]

extern crate alloc;

use core::mem;
use log::info;
use uefi::prelude::*;
use uefi::proto::console::gop::{BltOp, BltPixel, GraphicsOutput};
use uefi::proto::rng::Rng;
use uefi::table::boot::BootServices;
use uefi::Result;

fn get_random_usize(rng: &mut Rng) -> usize {
    let mut buf = [0; mem::size_of::<usize>()];
    rng.get_rng(None, &mut buf).expect("get_rng failed");
    usize::from_le_bytes(buf)
}

fn draw_random_rectangles(bt: &BootServices) -> Result {
    // Open graphics output protocol.
    let gop_handle = bt.get_handle_for_protocol::<GraphicsOutput>()?;
    let mut gop = bt.open_protocol_exclusive::<GraphicsOutput>(gop_handle)?;

    // Open random number generator protocol.
    let rng_handle = bt.get_handle_for_protocol::<Rng>()?;
    let mut rng = bt.open_protocol_exclusive::<Rng>(rng_handle)?;

    // Get screen resolution
    let (width, height) = gop.current_mode_info().resolution();

    // Draw randomly colored squares to random locations on the screen
    loop {
        // First corner of a rectangle
        let mut x = get_random_usize(&mut rng) % width;
        let mut y = get_random_usize(&mut rng) % height;
        let mut size = get_random_usize(&mut rng) % 200;

        if size == 0 {
            size = 1;
        }

        if x + size > width {
            x = x - size;
        }

        if y + size > height {
            y = y - size;
        }

        let red = get_random_usize(&mut rng) % 255;
        let green = get_random_usize(&mut rng) % 255;
        let blue = get_random_usize(&mut rng) % 255;

        let color = BltPixel::new(red as u8, green as u8, blue as u8);
        let dest = (x, y);
        let dims = (size, size);

        gop.blt(BltOp::VideoFill { color, dest, dims })?;
    }
}

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    info!("Hello World!");

    let bt = system_table.boot_services();
    draw_random_rectangles(bt).unwrap();

    Status::SUCCESS
}
