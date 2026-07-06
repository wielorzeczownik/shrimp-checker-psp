#![no_std]
#![no_main]

mod audio;
mod gfx;

use gfx::{clear, draw_centered, draw_image, draw_str, fill_rect};
use psp::sys::{self, CtrlButtons, SceCtrlData};

// Colors, ABGR8888 format.
const BG: u32 = 0xFF20_0810;
const WHITE: u32 = 0xFFFF_FFFF;
const LIME: u32 = 0xFF44_FF88;
const GRAY: u32 = 0xFFA0_A0B0;

const LOGO_W: usize = 128;
const LOGO_H: usize = 128;
const LOGO_X: usize = (psp::SCREEN_WIDTH as usize - LOGO_W) / 2;
const LOGO_Y: usize = 8;

static CAT_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/cat.raw"));
static SHRIMP_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/logo.raw"));

include!(concat!(env!("OUT_DIR"), "/module_info.rs"));

#[derive(Clone, Copy, PartialEq)]
enum Screen {
  Question,
  Yes,
}

// `psp::module!` requires an unmangled `psp_main` with the default Rust ABI.
#[allow(clippy::no_mangle_with_rust_abi)]
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
    unsafe { sys::sceCtrlReadBufferPositive(&raw mut pad, 1) };
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
