use clap::{Parser, Subcommand};
use reqwest::Client;
use serde_json::json;
use std::env;

#[derive(Parser)]
#[command(version, about = "Rust SuperBase Task Cli")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    ///创建一个job
    Add { title: String },
    ///查看最近的job
    List,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let key = env::var("SUPABASE_SECRET_KEY")?;
    let url = env::var("SUPABASE_TASK_URL")?;
    let client = Client::new();
    let cli = Cli::parse();

    // 根据命令构建请求
    let request = match cli.command {
        Command::Add { title } => client
            .post(url)
            .header("apikey", key)
            .json(&json!({"title":title})),
        Command::List => client.get(url).header("apikey", key),
    };

    let response = request.send().await?;
    let status = response.status();
    // 解析 JSON 响应
    let body: serde_json::Value = response.json().await?;

    println!("{}", serde_json::to_string_pretty(&body)?);

    if !status.is_success() {
        return Err(std::io::Error::other(format!("HTTP {status}")).into());
    }

    //     // 1. 写入数据库
    //     let response = client
    //         .post(&url)
    //         .header("apikey", key.as_str())
    //         .json(&json!({
    //             "title": "Task created from Rust!"
    //         }))
    //         .send()
    //         .await?;
    //
    //     let status = response.status();
    //     let body = response.text().await?;
    //     println!("POST {status}: {body}");
    //
    //     if !status.is_success() {
    //         return Ok(());
    //     }
    //
    //     // 2. 读取数据库
    //     let response = client
    //         .get(url)
    //         .header("apikey", key.as_str())
    //         .send()
    //         .await?;
    //
    // let status = response.status();
    // let body = response.text().await?;
    // println!("GET {status}: {body}");

    Ok(())
}
