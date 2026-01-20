use anyhow::Result;
use tokio::process::Command;

pub struct Trainer;

impl Trainer {
    pub async fn start_finetuning(dataset_path: &str) -> Result<String> {
        // Call the Python script
        let output = Command::new("python")
            .arg("../training_scripts/qlora_finetune.py")
            .arg("--dataset")
            .arg(dataset_path)
            .output()
            .await?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Ok(format!("Training Failed: {}", String::from_utf8_lossy(&output.stderr)))
        }
    }
}
