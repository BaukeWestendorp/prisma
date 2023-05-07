// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use common::project::Project;
use common::runner::Runner;

fn main() {
    std::thread::spawn(move || {
        let project: Project =
            serde_json::from_str(include_str!("project.json")).expect("failed to get project.json");
        let runner = Arc::new(Mutex::new(Runner::new(project, "localhost:7200")));
        loop {
            runner.lock().unwrap().proceed();
            std::thread::yield_now();
        }
    });

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
