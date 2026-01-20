// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ai_core::InferenceEngine;
use tauri::State;
use std::sync::Mutex;

struct AppState {
    engine: Mutex<InferenceEngine>,
}

use ai_training::Trainer;
use ai_evals::Evaluator;

#[tauri::command]
async fn greet(name: String) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn analyze_code(state: State<'_, AppState>, code: String) -> Result<String, String> {
    let engine = state.engine.lock().unwrap();
    match engine.generate(&code).await {
        Ok(res) => Ok(res),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn start_training(dataset: String) -> Result<String, String> {
    // Phase 3: The Forge
    Trainer::start_finetuning(&dataset).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn run_benchmark() -> Result<String, String> {
    // Phase 4: The Arena
    Evaluator::run_benchmark().map_err(|e| e.to_string())
}

fn main() {
    let engine = InferenceEngine::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState { engine: Mutex::new(engine) })
        .invoke_handler(tauri::generate_handler![
            greet, 
            analyze_code, 
            start_training, 
            run_benchmark
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
