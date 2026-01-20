use anyhow::{Error as E, Result};
use candle_core::{DType, Device, Tensor};
use candle_transformers::models::quantized_llama as model;
use model::ModelWeights;
use tokenizers::Tokenizer;
use std::path::PathBuf;

pub struct LlamaEngine {
    model: ModelWeights,
    tokenizer: Tokenizer,
}

impl LlamaEngine {
    pub fn load(model_path: PathBuf, tokenizer_path: PathBuf) -> Result<Self> {
        let device = Device::Cpu; // Fallback to CPU for compatibility
        
        // Load GGUF
        let mut file = std::fs::File::open(&model_path)?;
        let model = ModelWeights::from_gguf(&mut file, &mut file, &device)?;
        
        // Load Tokenizer
        let tokenizer = Tokenizer::from_file(&tokenizer_path).map_err(E::msg)?;
        
        Ok(Self { model, tokenizer })
    }

    pub fn generate(&mut self, prompt: &str, sample_len: usize) -> Result<String> {
        use candle_transformers::generation::LogitsProcessor;

        // 1. Encode Prompt
        let tokens = self.tokenizer.encode(prompt, true).map_err(E::msg)?;
        let prompt_tokens = tokens.get_ids();
        let mut all_tokens = Vec::new(); // Accumulate for decoding
        let mut response = String::new();

        // 2. Init LogitsProcessor (Temp 0.8, Top-P 0.9, seed 299792458)
        let mut logits_processor = LogitsProcessor::new(299792458, Some(0.8), Some(0.9));
        let device = Device::Cpu;

        let mut next_token = *prompt_tokens.last().unwrap();
        let mut pos = 0;

        // 3. Generation Loop
        for index in 0..prompt_tokens.len() + sample_len {
            // Context window management would go here (omitted for brevity)
            let input = if index < prompt_tokens.len() {
                Tensor::new(&[prompt_tokens[index]], &device)?.unsqueeze(0)?
            } else {
                Tensor::new(&[next_token], &device)?.unsqueeze(0)?
            };

            let logits = self.model.forward(&input, pos)?;
            let logits = logits.squeeze(0)?.squeeze(0)?;

            // Sample
            if index < prompt_tokens.len() - 1 {
                // Still processing prompt, just advance
                next_token = prompt_tokens[index + 1];
            } else {
                // Generate new token
                next_token = logits_processor.sample(&logits)?;
                all_tokens.push(next_token);
                
                // Decode incrementally
                if let Some(t) = self.tokenizer.id_to_token(next_token) {
                    let t = t.replace(' ', " ").replace("<0x0A>", "\n");
                    response.push_str(&t);
                }
            }
            pos += 1;
        }

        Ok(response)
    }
}
