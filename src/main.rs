#![no_main]
#![no_std]
#![feature(abi_efiapi)]

extern crate alloc;

use alloc::vec::Vec;
use core::mem;
use log::info;
use uefi::prelude::*;
use uefi::proto::console::gop::{BltOp, BltPixel, BltRegion, GraphicsOutput};
use uefi::proto::rng::Rng;
use uefi::table::boot::BootServices;
use uefi::Result;

struct Buffer<'a> {
    dims: (usize, usize),
    dest: (usize, usize),
    pixels: &'a [BltPixel],
}

impl<'a> Buffer<'a> {
    /// Create a new `Buffer`.
    fn new(dims: (usize, usize), dest: (usize, usize), pixels: &'a [BltPixel]) -> Self {
        Buffer { dims, dest, pixels }
    }

    /// Blit the buffer to the framebuffer.
    fn blit(&self, gop: &mut GraphicsOutput) -> Result {
        gop.blt(BltOp::BufferToVideo {
            buffer: self.pixels,
            src: BltRegion::Full,
            dest: self.dest,
            dims: self.dims,
        })
    }
}

fn get_random_usize(rng: &mut Rng) -> usize {
    let mut buf = [0; mem::size_of::<usize>()];
    rng.get_rng(None, &mut buf).expect("get_rng failed");
    usize::from_le_bytes(buf)
}

fn rick(bt: &BootServices) -> Result {
    let data = include_bytes!("video.bin")
        .chunks(3)
        .map(|chunk| BltPixel::new(chunk[0], chunk[1], chunk[2]))
        .collect::<Vec<_>>();

    // Open graphics output protocol.
    let gop_handle = bt.get_handle_for_protocol::<GraphicsOutput>()?;
    let mut gop = bt.open_protocol_exclusive::<GraphicsOutput>(gop_handle)?;

    // Get screen resolution
    let (width, height) = gop.current_mode_info().resolution();

    info!("Current resolution is {}x{}", width, height);

    let center_x = width / 2;
    let center_y = height / 2;

    let start_x = center_x - 320 / 2;
    let start_y = center_y - 240 / 2;

    for (i, frame) in data.chunks(76800).enumerate() {
        info!("Processing frame {}, size {}", i, frame.len());
        let buffer = Buffer::new((320, 240), (start_x, start_y), frame);
        buffer.blit(&mut gop)?;
        bt.stall(40_000);
    }

    Ok(())
}

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    info!("Hello World!");

    let bt = system_table.boot_services();
    rick(bt).unwrap();

    Status::SUCCESS
}
