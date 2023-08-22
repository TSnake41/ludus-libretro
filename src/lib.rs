mod libretro_device;

use std::fs;

use libretro_device::{LibretroAudioDevice, LibretroVideo};
use libretro_rs::{
    libretro_core, RetroAudioInfo, RetroCore, RetroEnvironment, RetroGame, RetroJoypadButton,
    RetroLoadGameResult, RetroPixelFormat, RetroRegion, RetroRuntime, RetroSystemInfo,
    RetroVideoInfo,
};
use ludus::{ButtonState, Cart, CartReadingError, NES_HEIGHT, NES_WIDTH};

struct Emulator {
    console: Option<Box<ludus::Console>>,
    video: LibretroVideo,
}

impl Emulator {
    fn load(&mut self, data: &[u8]) -> Result<(), CartReadingError> {
        self.console = Some(Box::new(ludus::Console::new(
            Cart::from_bytes(data)?,
            SAMPLE_RATE,
        )));

        Ok(())
    }

    fn load_file(&mut self, path: &str) -> Result<(), CartReadingError> {
        let Ok(content) = fs::read(path) else {
            return Err(CartReadingError::UnrecognisedFormat)
        };

        self.load(&content)
    }
}

const SAMPLE_RATE: u32 = 24000;

fn make_success_result() -> RetroLoadGameResult {
    RetroLoadGameResult::Success {
        audio: RetroAudioInfo::new(SAMPLE_RATE as _),
        video: RetroVideoInfo::new(60.0, NES_WIDTH as _, NES_HEIGHT as _)
            .with_pixel_format(RetroPixelFormat::XRGB8888),
        region: RetroRegion::NTSC,
    }
}

impl RetroCore for Emulator {
    fn init(env: &RetroEnvironment) -> Self {
        env.set_pixel_format(RetroPixelFormat::XRGB8888);
        env.set_support_no_game(true);

        Self {
            console: None,
            video: LibretroVideo::default(),
        }
    }

    fn get_system_info() -> RetroSystemInfo {
        RetroSystemInfo::new("Ludus", "0.1").with_valid_extensions(&["nes"])
    }

    fn reset(&mut self, _: &RetroEnvironment) {
        if let Some(console) = &mut self.console {
            console.reset();
        }
    }

    fn run(&mut self, _: &RetroEnvironment, runtime: &RetroRuntime) {
        if let Some(console) = self.console.as_mut() {
            console.step_frame(&mut LibretroAudioDevice(runtime), &mut self.video);
            runtime.upload_video_frame(self.video.0.as_ref(), NES_WIDTH as _, NES_HEIGHT as _, 0);

            console.update_controller(ButtonState {
                a: runtime.is_joypad_button_pressed(0, RetroJoypadButton::A),
                b: runtime.is_joypad_button_pressed(0, RetroJoypadButton::B),
                select: runtime.is_joypad_button_pressed(0, RetroJoypadButton::Select),
                start: runtime.is_joypad_button_pressed(0, RetroJoypadButton::Start),
                up: runtime.is_joypad_button_pressed(0, RetroJoypadButton::Up),
                down: runtime.is_joypad_button_pressed(0, RetroJoypadButton::Down),
                left: runtime.is_joypad_button_pressed(0, RetroJoypadButton::Left),
                right: runtime.is_joypad_button_pressed(0, RetroJoypadButton::Right),
            })
        }
    }

    fn load_game(
        &mut self,
        env: &RetroEnvironment,
        game: Option<RetroGame>,
    ) -> RetroLoadGameResult {
        env.set_pixel_format(RetroPixelFormat::XRGB8888);

        match game {
            None => make_success_result(),
            Some(RetroGame::None { meta: _ }) => {
                self.console = None;

                make_success_result()
            }
            Some(RetroGame::Data { meta: _, data }) => match self.load(data) {
                Ok(()) => make_success_result(),
                Err(_) => RetroLoadGameResult::Failure,
            },
            Some(RetroGame::Path { meta: _, path }) => match self.load_file(path) {
                Ok(()) => make_success_result(),
                Err(_) => RetroLoadGameResult::Failure,
            },
        }
    }
}

libretro_core!(Emulator);
