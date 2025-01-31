use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct GenerateResponse {
    response: String,
}

pub async fn ensure_ollama_running() -> Result<()> {
    let client = Client::new();
    match client.get("http://localhost:11434/api/version").send().await {
        Ok(_) => return Ok(()),
        Err(_) => println!("Ollama is not running. Starting Ollama..."),
    }

    let _start = Command::new("ollama")
        .arg("serve")
        .spawn()?;

    for _ in 0..30 {
        if client.get("http://localhost:11434/api/version").send().await.is_ok() {
            return Ok(());
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    Err(anyhow::anyhow!("Failed to start Ollama after 30 seconds"))
}

pub async fn get_available_models() -> Result<Vec<String>> {
    let output = Command::new("ollama")
        .arg("list")
        .output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Failed to get model list from Ollama"));
    }

    let output_str = String::from_utf8(output.stdout)?;
    let models: Vec<String> = output_str
        .lines()
        .skip(1)
        .filter_map(|line| line.split_whitespace().next())
        .map(String::from)
        .collect();

    Ok(models)
}

pub fn select_model(preferred: Option<String>, available: &[String]) -> String {
    preferred
        .and_then(|p| {
            if available.contains(&p) {
                Some(p)
            } else {
                None
            }
        })
        .unwrap_or_else(|| "qwen2.5:7b".to_string())
}

pub async fn ensure_model_available(model: &str) -> Result<()> {
    let available_models = get_available_models().await?;
    if !available_models.contains(&model.to_string()) {
        println!("Model {} not found. Pulling...", model);
        let status = Command::new("ollama")
            .args(["pull", model])
            .status()?;

        if !status.success() {
            return Err(anyhow::anyhow!("Failed to pull model {}", model));
        }
    }
    Ok(())
}

pub async fn generate_commit_message(model: &str, diff: &str) -> Result<String> {
    let client = Client::new();
    let prompt = format!(
        "Generate a concise and descriptive commit message for the following git diff:\n\n{}",
        diff
    );

    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&GenerateRequest {
            model: model.to_string(),
            prompt,
            stream: false,
        })
        .send()
        .await?
        .json::<GenerateResponse>()
        .await?;

    Ok(response.response.trim().to_string())
}

pub async fn generate_pr_description(model: &str, commits: &[String], diff: &str) -> Result<String> {
    let client = Client::new();
    let commits_str = commits.join("\n");
    let prompt = format!(
        "Generate a detailed pull request description based on the following commits and diff:\n\nCommits:\n{}\n\nDiff:\n{}",
        commits_str, diff
    );

    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&GenerateRequest {
            model: model.to_string(),
            prompt,
            stream: false,
        })
        .send()
        .await?
        .json::<GenerateResponse>()
        .await?;

    Ok(response.response.trim().to_string())
}
