// Colors ABGR8888 format
pub const BG: u32 = 0xFF20_0810;
pub const WHITE: u32 = 0xFFFF_FFFF;
pub const LIME: u32 = 0xFF44_FF88;
pub const GRAY: u32 = 0xFFA0_A0B0;

pub const LOGO_W: usize = 128;
pub const LOGO_H: usize = 128;
pub const LOGO_X: usize = (psp::SCREEN_WIDTH as usize - LOGO_W) / 2;
pub const LOGO_Y: usize = 8;

pub const AUDIO_CHUNK: usize = 2048;

pub static CAT_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/cat.raw"));
pub static SHRIMP_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/logo.raw"));
pub static SOUND_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sound.raw"));
