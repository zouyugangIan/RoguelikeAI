pub mod engine;

use anyhow::Result;
use engine::LlamaEngine;
use std::path::PathBuf;

pub struct InferenceEngine {
    inner: Option<LlamaEngine>,
}

impl InferenceEngine {
    pub fn new() -> Self {
        // In dev mode, we might not have the model yet.
        // We look for it in absolute path for now.
        let model_path = PathBuf::from("c:/BW/develop/RoguelikeAI/models/llama-3-8b.gguf");
        let tokenizer_path = PathBuf::from("c:/BW/develop/RoguelikeAI/models/tokenizer.json");
        
        let inner = if model_path.exists() {
            println!("Loading model from {:?}", model_path);
            LlamaEngine::load(model_path, tokenizer_path).ok()
        } else {
            println!("Model not found, running in Mock Mode");
            None
        };

        Self { inner }
    }

    pub async fn generate(&self, prompt: &str) -> Result<String> {
        match &self.inner {
            Some(engine) => {
                // We need mutability, in real app we wrap Engine in Mutex inside the struct
                // For this demo, we clone or just retun a static string to prove it loaded
                Ok(format!("Real Model Loaded! But generation loop needs mutable access. Input: {}", prompt))
            },
            None => {
                Ok(format!("[Mock Mode] Analysis for: {}\n\nOptimization Suggestion:\nUse Vec::with_capacity to avoid reallocations.", prompt))
            }
        }
    }
}
