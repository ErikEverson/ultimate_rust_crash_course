use std::error::Error;

use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>>  {
    let mut audio = Audio::new();
    audio.add("", ".wave")
    audio.add("", ".wave")
    audio.add("", ".wave")
    audio.add("", ".wave")
    audio.add("", ".wave")
    audio.add("", ".wave")
}
