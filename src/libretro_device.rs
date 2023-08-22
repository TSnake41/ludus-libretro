use libretro_rs::RetroRuntime;
use ludus::{AudioDevice, VideoDevice, NES_HEIGHT, NES_WIDTH};

pub struct LibretroVideo(pub Box<[u8; NES_WIDTH * NES_HEIGHT * 4]>);

impl Default for LibretroVideo {
    fn default() -> Self {
        Self(Box::new([0; NES_WIDTH * NES_HEIGHT * 4]))
    }
}

impl VideoDevice for LibretroVideo {
    fn blit_pixels(&mut self, pixels: &ludus::PixelBuffer) {
        pixels.as_ref().iter().enumerate().for_each(|(i, &pixel)| {
            self.0[(i * 4)..((i + 1) * 4)].copy_from_slice(&pixel.to_ne_bytes());
        });
    }
}

fn denormalize(value: f32) -> i16 {
    (value.clamp(-1.0, 1.0) * i16::MAX as f32) as i16
}

pub struct LibretroAudioDevice<'a>(pub &'a RetroRuntime);

impl AudioDevice for LibretroAudioDevice<'_> {
    fn push_sample(&mut self, sample: f32) {
        self.0
            .upload_audio_sample(denormalize(sample), denormalize(sample))
    }
}
