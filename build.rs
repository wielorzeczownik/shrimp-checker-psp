use std::{fs, path::PathBuf};

fn main() {
  let out = PathBuf::from(std::env::var("OUT_DIR").unwrap());
  let assets = PathBuf::from("assets");

  // Generate psp::module!
  {
    let ver = env!("CARGO_PKG_VERSION");
    let mut parts = ver.splitn(3, '.').filter_map(|p| p.parse::<u32>().ok());
    let major = parts.next().unwrap_or(0);
    let minor = parts.next().unwrap_or(0);
    fs::write(
      out.join("module_info.rs"),
      format!("psp::module!(\"shrimp_checker\", {major}, {minor});\n"),
    )
    .unwrap();
  }

  println!(
    "cargo:rerun-if-changed={}",
    assets.join("logo.png").display()
  );
  println!(
    "cargo:rerun-if-changed={}",
    assets.join("cat.webp").display()
  );
  println!(
    "cargo:rerun-if-changed={}",
    assets.join("sound.mp3").display()
  );

  // Logo: Resize to 128x128 and save as raw RGBA8 pixel data.
  {
    let img = image::open(assets.join("logo.png"))
      .expect("open logo.png")
      .resize_exact(128, 128, image::imageops::FilterType::Lanczos3)
      .into_rgba8();
    let mut raw: Vec<u8> = Vec::with_capacity(128 * 128 * 4);
    for pixel in img.pixels() {
      raw.extend_from_slice(&pixel.0);
    }
    fs::write(out.join("logo.raw"), raw).unwrap();
  }

  // Cat: 480×272 (full PSP screen), cover-crop to preserve aspect ratio
  {
    let img = image::open(assets.join("cat.webp"))
      .expect("open cat.webp")
      .resize_to_fill(480, 272, image::imageops::FilterType::Lanczos3)
      .into_rgba8();
    let mut raw: Vec<u8> = Vec::with_capacity(480 * 272 * 4);
    for pixel in img.pixels() {
      raw.extend_from_slice(&pixel.0);
    }
    fs::write(out.join("cat.raw"), raw).unwrap();
  }

  // Sound: Decode MP3 to raw, signed 16-bit little-endian stereo PCM.
  {
    let mp3 = fs::read(assets.join("sound.mp3")).expect("read sound.mp3");
    let mut decoder = minimp3::Decoder::new(mp3.as_slice());
    let mut pcm: Vec<i16> = Vec::new();
    loop {
      match decoder.next_frame() {
        Ok(minimp3::Frame { data, .. }) => pcm.extend_from_slice(&data),
        Err(minimp3::Error::Eof) => break,
        Err(err) => panic!("MP3 decode: {:?}", err),
      }
    }
    let raw: Vec<u8> = pcm.iter().flat_map(|sample| sample.to_le_bytes()).collect();
    fs::write(out.join("sound.raw"), raw).unwrap();
  }
}
