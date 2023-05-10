// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use common::project::PrismaProject;
use common::runner::Runner;

#[derive(Debug)]
struct RunnerState {
    runner: Arc<Mutex<Runner>>,
}

#[tauri::command]
fn update_project(new_project: PrismaProject, project_state: tauri::State<'_, RunnerState>) {
    project_state
        .runner
        .lock()
        .unwrap()
        .update_project(new_project.clone());
}

fn main() {
    let initial_project = PrismaProject {
        framerate: 50,
        global_bpm: 60.0,
        effect_layers: vec![],
    };
    let runner = Arc::new(Mutex::new(Runner::new(initial_project, "localhost:7200")));

    std::thread::spawn({
        let runner = runner.clone();
        move || loop {
            runner.lock().unwrap().proceed();
            std::thread::yield_now();
        }
    });

    tauri::Builder::default()
        .manage(RunnerState { runner })
        .invoke_handler(tauri::generate_handler![update_project])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
