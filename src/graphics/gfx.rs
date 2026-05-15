use uefi::proto::console::gop::{BltOp, BltPixel, GraphicsOutput};

pub fn init_gop(gop: &mut GraphicsOutput, preferred: (usize, usize)) -> (usize, usize, usize) {
    if let Some(mode) = gop.modes().find(|m| m.info().resolution() == preferred) {
        gop.set_mode(&mode).unwrap();
    }
    let (width, height) = gop.current_mode_info().resolution();
    let stride = gop.current_mode_info().stride();
    (width, height, stride)
}

pub fn draw_gradient(gop: &mut GraphicsOutput, width: usize, height: usize, stride: usize) {
    let mut fb = gop.frame_buffer();
    for y in 0..height {
        for x in 0..width {
            let offset = (y * stride + x) * 4;
            let r = (x * 255 / width) as u8;
            let b = (y * 255 / height) as u8;
            unsafe { fb.write_value(offset, [b, 0u8, r, 0u8]); }
        }
    }
}

pub fn fill_rect(gop: &mut GraphicsOutput, x: usize, y: usize, w: usize, h: usize, color: BltPixel) {
    gop.blt(BltOp::VideoFill { color, dest: (x, y), dims: (w, h) }).unwrap();
}
