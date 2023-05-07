// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use project::Project;
use runner::Runner;

mod color;
mod effect;
mod project;
mod runner;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    std::thread::spawn(move || {
        let project: Project = serde_json::from_str(include_str!("project.json")).unwrap();
        let runner = Arc::new(Mutex::new(Runner::new(project, "localhost:7200")));

        loop {
            runner.lock().unwrap().proceed();
            std::thread::yield_now();
        }
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
