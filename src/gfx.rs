use core::ptr;
use embedded_graphics::{
  mono_font::{MonoTextStyle, ascii::FONT_8X13_BOLD},
  pixelcolor::BinaryColor,
  prelude::*,
  text::{Baseline, Text},
};

pub static mut VRAM: usize = 0;

pub fn draw_image(pix: &[u8], width: usize, height: usize, dst_x: usize, dst_y: usize, bg: u32) {
  let bg_red = (bg & 0xFF) as u32;
  let bg_grn = (bg >> 8 & 0xFF) as u32;
  let bg_blu = (bg >> 16 & 0xFF) as u32;
  for row in 0..height {
    for col in 0..width {
      let idx = (row * width + col) * 4;
      let red = pix[idx] as u32;
      let grn = pix[idx + 1] as u32;
      let blu = pix[idx + 2] as u32;
      let alpha = pix[idx + 3] as u32;
      if alpha == 0 {
        continue;
      }
      let color = if alpha == 255 {
        0xFF000000 | blu << 16 | grn << 8 | red
      } else {
        let inv_alpha = 255 - alpha;
        let out_red = (red * alpha + bg_red * inv_alpha) / 255;
        let out_grn = (grn * alpha + bg_grn * inv_alpha) / 255;
        let out_blu = (blu * alpha + bg_blu * inv_alpha) / 255;
        0xFF000000 | out_blu << 16 | out_grn << 8 | out_red
      };
      put_pixel(dst_x + col, dst_y + row, color);
    }
  }
}

pub fn draw_centered(top: usize, text: &str, color: u32, scale: usize) {
  let text_width = text.len() * 8 * scale;
  let left = if text_width < psp::SCREEN_WIDTH as usize {
    (psp::SCREEN_WIDTH as usize - text_width) / 2
  } else {
    0
  };
  draw_str(left, top, text, color, scale);
}

pub fn draw_str(left: usize, top: usize, text: &str, color: u32, scale: usize) {
  let style = MonoTextStyle::new(&FONT_8X13_BOLD, BinaryColor::On);
  let mut target = ScaleTarget {
    color,
    scale,
    offset_x: left,
    offset_y: top,
  };
  Text::with_baseline(text, Point::zero(), style, Baseline::Top)
    .draw(&mut target)
    .ok();
}

pub fn clear(color: u32) {
  for row in 0..psp::SCREEN_HEIGHT as usize {
    for col in 0..psp::SCREEN_WIDTH as usize {
      put_pixel(col, row, color);
    }
  }
}

pub fn fill_rect(left: usize, top: usize, width: usize, height: usize, color: u32) {
  for row in top..top + height {
    for col in left..left + width {
      put_pixel(col, row, color);
    }
  }
}

#[inline(always)]
pub fn put_pixel(col: usize, row: usize, color: u32) {
  if col < psp::SCREEN_WIDTH as usize && row < psp::SCREEN_HEIGHT as usize {
    unsafe {
      let ptr = (VRAM + (row * psp::BUF_WIDTH as usize + col) * 4) as *mut u32;
      ptr::write_volatile(ptr, color);
    }
  }
}

struct ScaleTarget {
  color: u32,
  scale: usize,
  offset_x: usize,
  offset_y: usize,
}

impl DrawTarget for ScaleTarget {
  type Color = BinaryColor;
  type Error = core::convert::Infallible;

  fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
  where
    I: IntoIterator<Item = Pixel<BinaryColor>>,
  {
    for Pixel(point, lit) in pixels {
      if lit.is_on() {
        let draw_x = self.offset_x + point.x as usize * self.scale;
        let draw_y = self.offset_y + point.y as usize * self.scale;
        for sub_y in 0..self.scale {
          for sub_x in 0..self.scale {
            put_pixel(draw_x + sub_x, draw_y + sub_y, self.color);
          }
        }
      }
    }
    Ok(())
  }
}

impl OriginDimensions for ScaleTarget {
  fn size(&self) -> Size {
    Size::new(psp::SCREEN_WIDTH, psp::SCREEN_HEIGHT)
  }
}
