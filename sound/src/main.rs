extern crate sdl2;

use sdl2::audio::{AudioCallback, AudioSpecDesired};
use std::time::Duration;

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;
    fn callback(&mut self, out: &mut [f32]) {
        //Generate a sqquare wave
        for x in out.iter_mut() {
            *x = if self.phase < 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

fn main() {
    println!("main() has been called");
    let sdl_context = sdl2::init().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();

    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1), //mono
        samples: None,     //default sample size
    };

    match audio_subsystem.open_playback(None, &desired_spec, |spec| {
        //Show obtained AudioSpec
        //let strspec = format!("{:?}", &spec) ;
        //println!("{}", &strspec);

        //initialize the audio callback
        SquareWave {
            phase_inc: (44.0 / spec.freq as f32),
            phase: 0.0,
            volume: 0.2,
        }
    }) {
        Ok(device) => {
            device.resume();
            std::thread::sleep(Duration::from_millis(2_000));
        }
        Err(error) => {
            eprintln!("{}", error);
        }
    }

    //Start Playback

    //Play for 2 seconds
    //

    //Device is automatically closed when dropped
}
