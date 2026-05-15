#![no_main]
#![no_std]

mod graphics;

use graphics::font;
use graphics::gfx;

use uefi::prelude::*;
use uefi::proto::console::gop::{BltPixel, GraphicsOutput};

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    let handle = uefi::boot::get_handle_for_protocol::<GraphicsOutput>().unwrap();
    let mut gop = uefi::boot::open_protocol_exclusive::<GraphicsOutput>(handle).unwrap();

    let (width, height, stride) = gfx::init_gop(&mut gop, (1280, 720));

    gfx::draw_gradient(&mut gop, width, height, stride);

    let label = "hi my name is scat pearl and im loading the kernel for you";
    let box_w = label.len() * 8 + 20;
    let box_h = 40;
    let box_x = (width - box_w) / 2;
    let box_y = (height - box_h) / 2;
    gfx::fill_rect(&mut gop, box_x, box_y, box_w, box_h, BltPixel::new(0, 0, 0));

    let text_x = box_x + 10;
    let text_y = box_y + (box_h - 8) / 2;
    let mut fb = gop.frame_buffer();
    font::draw_str(&mut fb, stride, text_x, text_y, label, [255, 255, 255, 0]);

    loop {}
}
