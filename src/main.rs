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
    let project: Project = serde_json::from_str(include_str!("project.json")).unwrap();

    let runner = Arc::new(Mutex::new(Runner::new(
        project,
        // "localhost:7200",
        "kilowatt-cruiser.local:80",
    )));

    // std::thread::spawn({
    //     let runner = runner.clone();
    //     move || {
    //         let new_project = Project {
    //             framerate: 60,
    //             global_bpm: 60.0,
    //             effects: vec![EffectWrapper {
    //                 bpm_factor: 1.0,
    //                 range: LedRange { min: 0, max: 60 },
    //                 effect: Effect::StaticColor {
    //                     color: Color::white(),
    //                 },
    //             }],
    //         };

    //         std::thread::sleep(Duration::from_millis(1000));

    //         runner.lock().unwrap().update_project(new_project);
    //     }
    // });

    loop {
        runner.lock().unwrap().proceed();
        std::thread::yield_now();
    }
}
