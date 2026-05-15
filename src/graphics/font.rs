use font8x8::legacy::BASIC_LEGACY;
use uefi::proto::console::gop::FrameBuffer;

pub fn draw_char(fb: &mut FrameBuffer, stride: usize, x: usize, y: usize, c: char, color: [u8; 4]) {
    let idx = c as usize;
    if idx >= BASIC_LEGACY.len() { return; }
    let glyph = BASIC_LEGACY[idx];
    for (row, byte) in glyph.iter().enumerate() {
        for col in 0..8usize {
            if byte & (1 << col) != 0 {
                let offset = ((y + row) * stride + (x + col)) * 4;
                unsafe { fb.write_value(offset, color); }
            }
        }
    }
}

pub fn draw_str(fb: &mut FrameBuffer, stride: usize, x: usize, y: usize, s: &str, color: [u8; 4]) {
    for (i, c) in s.chars().enumerate() {
        draw_char(fb, stride, x + i * 8, y, c, color);
    }
}
