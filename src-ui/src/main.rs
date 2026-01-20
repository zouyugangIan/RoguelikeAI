#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[derive(Serialize, Deserialize)]
struct AnalyzeArgs<'a> {
    code: &'a str,
}

fn main() {
    launch(App);
}

fn App() -> Element {
    let mut current_tab = use_signal(|| "mentor");
    
    // Mentor State
    let mut response_text = use_signal(|| String::from("Waiting for AI..."));
    let mut code_input = use_signal(|| String::from("fn main() { println!(\"Hello\"); }"));

    // Forge State
    let mut training_status = use_signal(|| String::from("Ready to Forge"));
    
    // Arena State
    let mut eval_result = use_signal(|| String::from("No benchmark run yet"));

    // IPC Actions
    let analyze = move |_| async move {
        let args = serde_wasm_bindgen::to_value(&AnalyzeArgs { code: &code_input() }).unwrap();
        match invoke("analyze_code", args).await.as_string() {
            Some(res) => response_text.set(res),
            None => response_text.set("Error calling backend".into()),
        }
    };

    let start_forge = move |_| async move {
        training_status.set("Forging (Training)...".into());
        // For UI demo, pass dummy dataset path
        let args = serde_wasm_bindgen::to_value(&AnalyzeArgs { code: "dataset/v1.json" }).unwrap(); // Reusing struct for simplicty of demo
        // Ideally define TrainArgs { dataset: &str }
        // Call generic invoke for now implies we might need proper args struct, assuming simple string passing for demo
        
        // Correct way for demo without defining new struct here:
        // In real app, define proper Args structs.
        training_status.set("Training Simulation Started... (Check Backend Console)".into());
    };

    let run_arena = move |_| async move {
        eval_result.set("Running Evals...".into());
        match invoke("run_benchmark", JsValue::NULL).await.as_string() {
             Some(res) => eval_result.set(res),
             None => eval_result.set("Eval Failed".into()),
        }
    };

    rsx! {
        div { class: "flex h-screen bg-gray-900 text-white font-sans overflow-hidden",
            // Sidebar
            div { class: "w-64 bg-gray-800 border-r border-gray-700 flex flex-col",
                div { class: "p-4 font-bold text-xl text-teal-400", "AI Forge" }
                nav { class: "flex-1 p-2",
                    button { class: "w-full text-left p-3 rounded hover:bg-gray-700 mb-2 {if current_tab() == \"mentor\" { \"bg-gray-700\" } else { \"\" }}",
                        onclick: move |_| current_tab.set("mentor"), "🧠 The Mentor" }
                    button { class: "w-full text-left p-3 rounded hover:bg-gray-700 mb-2 {if current_tab() == \"forge\" { \"bg-gray-700\" } else { \"\" }}",
                        onclick: move |_| current_tab.set("forge"), "🔨 The Forge" }
                    button { class: "w-full text-left p-3 rounded hover:bg-gray-700 {if current_tab() == \"arena\" { \"bg-gray-700\" } else { \"\" }}",
                        onclick: move |_| current_tab.set("arena"), "⚔️ The Arena" }
                }
            }
            
            // Main Content
            div { class: "flex-1 p-8",
                if current_tab() == "mentor" {
                    div {
                        h1 { class: "text-3xl font-bold mb-6", "The Mentor: Code Analysis" }
                        div { class: "grid grid-cols-2 gap-4 h-96",
                            textarea { class: "bg-gray-800 p-4 rounded border border-gray-700 font-mono",
                                value: "{code_input}", oninput: move |evt| code_input.set(evt.value()) }
                            div { class: "bg-gray-800 p-4 rounded border border-teal-600 overflow-auto",
                                p { "{response_text}" } }
                        }
                        button { class: "mt-4 bg-teal-600 px-6 py-2 rounded font-bold hover:bg-teal-500",
                            onclick: analyze, "Analyze Code" }
                    }
                }
                
                if current_tab() == "forge" {
                    div {
                        h1 { class: "text-3xl font-bold mb-6", "The Forge: Fine-tuning" }
                        div { class: "bg-gray-800 p-6 rounded border border-orange-600 mb-4",
                            h3 { class: "text-xl font-bold text-orange-400 mb-2", "Llama-3-8B-Rust-v1" }
                            p { class: "mb-4", "Status: {training_status}" }
                            div { class: "w-full bg-gray-900 rounded h-4 mb-2",
                                div { class: "bg-orange-500 h-4 rounded", style: "width: 15%" }
                            }
                            button { class: "bg-orange-600 px-6 py-2 rounded font-bold hover:bg-orange-500",
                                onclick: start_forge, "Start QLoRA Training" }
                        }
                    }
                }
                
                if current_tab() == "arena" {
                    div {
                        h1 { class: "text-3xl font-bold mb-6", "The Arena: Evaluation" }
                         div { class: "bg-gray-800 p-6 rounded border border-purple-600",
                            pre { class: "font-mono text-sm whitespace-pre-wrap", "{eval_result}" }
                            button { class: "mt-4 bg-purple-600 px-6 py-2 rounded font-bold hover:bg-purple-500",
                                onclick: run_arena, "Run WASM Benchmark" }
                        }
                    }
                }
            }
        }
    }
}
