#![no_std]
#![no_main]

mod audio;
mod constants;
mod gfx;

use constants::*;
use gfx::*;
use psp::sys::{self, CtrlButtons, SceCtrlData};

include!(concat!(env!("OUT_DIR"), "/module_info.rs"));

#[derive(Clone, Copy, PartialEq)]
enum Screen {
  Question,
  Yes,
}

#[unsafe(no_mangle)]
pub fn psp_main() -> i32 {
  psp::enable_home_button();

  unsafe {
    gfx::VRAM = sys::sceGeEdramGetAddr() as usize | 0x4000_0000;
  }

  unsafe {
    sys::sceDisplaySetMode(
      sys::DisplayMode::Lcd,
      psp::SCREEN_WIDTH as usize,
      psp::SCREEN_HEIGHT as usize,
    );
    sys::sceDisplaySetFrameBuf(
      gfx::VRAM as *const u8,
      psp::BUF_WIDTH as usize,
      sys::DisplayPixelFormat::Psm8888,
      sys::DisplaySetBufSync::NextFrame,
    );
    sys::sceCtrlSetSamplingCycle(0);
    sys::sceCtrlSetSamplingMode(sys::CtrlMode::Digital);
  }

  let mut screen = Screen::Question;
  let mut prev = CtrlButtons::empty();

  loop {
    unsafe { sys::sceDisplayWaitVblankStart() };

    let mut pad: SceCtrlData = unsafe { core::mem::zeroed() };
    unsafe { sys::sceCtrlReadBufferPositive(&mut pad, 1) };
    // Find out which buttons just pressed.
    let pressed = pad.buttons & !prev;
    prev = pad.buttons;

    // Handle input based on the current screen
    if screen == Screen::Question {
      if pressed.contains(CtrlButtons::CROSS) {
        screen = Screen::Yes;
      } else if pressed.contains(CtrlButtons::CIRCLE) {
        unsafe { sys::sceKernelExitGame() };
      }
    }

    match screen {
      // Draw the current screen.
      Screen::Question => draw_question(),
      Screen::Yes => {
        draw_cat();
        unsafe { sys::sceDisplayWaitVblankStart() };
        audio::play_sound();
        unsafe { sys::sceKernelExitGame() }; // Exit after the sound plays.
      }
    }
  }
}

fn draw_question() {
  clear(BG);
  draw_image(SHRIMP_BYTES, LOGO_W, LOGO_H, LOGO_X, LOGO_Y, BG);
  draw_centered(155, "Are you a shrimp?", WHITE, 2);
  fill_rect(40, 195, 400, 1, GRAY);
  draw_str(64, 208, "[X] YES", LIME, 2);
  draw_str(312, 208, "[O] NO", LIME, 2);
}

fn draw_cat() {
  draw_image(
    CAT_BYTES,
    psp::SCREEN_WIDTH as usize,
    psp::SCREEN_HEIGHT as usize,
    0,
    0,
    0,
  );
}
