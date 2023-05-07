use std::sync::{Arc, Mutex};
use std::time::Duration;

use color::Color;
use effect::{Effect, EffectWrapper};
use project::{Cue, Project};
use runner::Runner;

mod color;
mod effect;
mod project;
mod runner;

fn main() {
    let project = Project {
        framerate: 50,
        global_bpm: 60.0,
        cues: vec![Cue {
            effects: vec![
                EffectWrapper {
                    bpm_factor: 1.0,
                    effect: Effect::StaticColor {
                        color: Color::black(),
                    },
                },
                EffectWrapper {
                    bpm_factor: 1.0,
                    effect: Effect::Wave {
                        color: Color::green(),
                        wave_type: effect::WaveType::Sine,
                        repeats: 1.0,
                    },
                },
            ],
        }],
    };

    let runner = Arc::new(Mutex::new(Runner::new(project)));

    std::thread::spawn({
        let runner = runner.clone();
        move || {
            let new_project = Project {
                framerate: 60,
                global_bpm: 60.0,
                cues: vec![Cue {
                    effects: vec![EffectWrapper {
                        bpm_factor: 1.0,
                        effect: Effect::StaticColor {
                            color: Color::red(),
                        },
                    }],
                }],
            };

            std::thread::sleep(Duration::from_millis(1000));

            runner.lock().unwrap().update_project(new_project);
        }
    });

    loop {
        runner.lock().unwrap().proceed();
        std::thread::yield_now();
    }
}
