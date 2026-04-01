use crate::constants::{AUDIO_CHUNK, SOUND_BYTES};
use psp::sys;

static mut AUDIO_BUF: [i16; AUDIO_CHUNK * 2] = [0; AUDIO_CHUNK * 2];

/// Plays the embedded sound, blocking until playback completes.
pub fn play_sound() {
  let pcm: &[i16] = unsafe {
    core::slice::from_raw_parts(SOUND_BYTES.as_ptr() as *const i16, SOUND_BYTES.len() / 2)
  };

  unsafe {
    let channel = sys::sceAudioChReserve(
      sys::AUDIO_NEXT_CHANNEL,
      AUDIO_CHUNK as i32,
      sys::AudioFormat::Stereo,
    );
    if channel < 0 {
      return;
    }

    let buf = &raw mut AUDIO_BUF;
    let frame_samples = AUDIO_CHUNK * 2;
    let mut pos = 0;

    while pos < pcm.len() {
      let end = (pos + frame_samples).min(pcm.len());
      let count = end - pos;
      (&mut (*buf))[..count].copy_from_slice(&pcm[pos..end]);
      (&mut (*buf))[count..].fill(0);
      sys::sceAudioOutputBlocking(
        channel,
        sys::AUDIO_VOLUME_MAX as i32,
        (*buf).as_ptr() as *mut core::ffi::c_void,
      );
      pos += frame_samples;
    }

    sys::sceAudioChRelease(channel);
  }
}
