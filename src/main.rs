use std::sync::{Arc, Mutex};
use std::time::Duration;

use color::Color;
use effect::{Effect, EffectWrapper, LedRange};
use project::Project;
use runner::Runner;

mod color;
mod effect;
mod project;
mod runner;

fn main() {
    let project = Project {
        framerate: 50,
        global_bpm: 60.0,
        effects: vec![
            EffectWrapper {
                bpm_factor: 1.0,
                range: LedRange { min: 0, max: 50 },
                effect: Effect::StaticColor {
                    color: Color::red(),
                },
            },
            EffectWrapper {
                bpm_factor: 1.0,
                range: LedRange { min: 40, max: 60 },
                effect: Effect::StaticColor {
                    color: Color::rgb(0.0, 1.0, 1.0, 0.5),
                },
            },
            EffectWrapper {
                bpm_factor: 1.0,
                range: LedRange { min: 40, max: 60 },
                effect: Effect::Wave {
                    color: Color::green(),
                    wave_type: effect::WaveType::Sine,
                    repeats: 1.0,
                },
            },
            EffectWrapper {
                bpm_factor: 1.0,
                range: LedRange { min: 5, max: 25 },
                effect: Effect::Wave {
                    color: Color::blue(),
                    wave_type: effect::WaveType::Square { pulse_width: 0.5 },
                    repeats: 0.5,
                },
            },
        ],
    };

    let runner = Arc::new(Mutex::new(Runner::new(project, "localhost:7200")));

    std::thread::spawn({
        let runner = runner.clone();
        move || {
            let new_project = Project {
                framerate: 60,
                global_bpm: 60.0,
                effects: vec![EffectWrapper {
                    bpm_factor: 1.0,
                    range: LedRange { min: 0, max: 60 },
                    effect: Effect::StaticColor {
                        color: Color::white(),
                    },
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
