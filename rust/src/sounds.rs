use godot::classes::{AudioStream, AudioStreamPlayer};
use godot::obj::{Gd, NewAlloc, NewGd};
use godot::prelude::GodotClass;
use godot::tools::load;

#[derive(GodotClass)]
#[class(no_init, base=AudioStreamPlayer)]
pub struct GodotSounds {
    #[allow(dead_code)]
    stream: Gd<AudioStream>,
    pub player: Gd<AudioStreamPlayer>,
}

impl GodotSounds {
    pub fn empty() -> Gd<Self> {
        Gd::from_object(Self {
            stream: AudioStream::new_gd(),
            player: AudioStreamPlayer::new_alloc(),
        })
    }

    pub fn from_path(path: &str) -> Gd<Self> {
        let stream: Gd<AudioStream> = load(path);
        let mut player = AudioStreamPlayer::new_alloc();
        player.set_stream(&stream);

        Gd::from_object(Self { stream, player })
    }

    #[allow(dead_code)]
    pub fn play(&mut self) {
        self.player.play();
    }
}
