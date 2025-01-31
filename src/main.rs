// Commitgenius - An AI-powered conventional commit message generator
// Uses Ollama for local LLM inference to create meaningful commit messages

use std::process::Command;
use clap::Parser;
use anyhow::{Result, anyhow};
use reqwest;
use serde_json::{json, Value};
use git2::{Repository, Status};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Model to use for generating commit messages
    #[arg(short, long)]
    model: Option<String>,
}

async fn get_available_models() -> Result<Vec<String>> {
    let output = Command::new("ollama")
        .arg("list")
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("Failed to get model list from Ollama"));
    }

    let output_str = String::from_utf8(output.stdout)?;
    let models: Vec<String> = output_str
        .lines()
        .skip(1) // Skip header line
        .filter_map(|line| line.split_whitespace().next())
        .map(String::from)
        .collect();

    Ok(models)
}

fn select_model(preferred: Option<String>, available: &[String]) -> String {
    // If user specified a model and it's available, use it
    if let Some(model) = preferred {
        if available.contains(&model) {
            return model;
        }
        eprintln!("Warning: Specified model '{}' not found. Falling back to available model.", model);
    }

    // Try to use qwen2.5:7b if available
    if available.contains(&String::from("qwen2.5:7b")) {
        return String::from("qwen2.5:7b");
    }

    // Otherwise use the first available model
    available.first()
        .map(String::from)
        .unwrap_or_else(|| String::from("neural-chat")) // Final fallback
}

async fn ensure_ollama_running() -> Result<()> {
    // Check if Ollama is running
    let client = reqwest::Client::new();
    match client.get("http://localhost:11434/api/version").send().await {
        Ok(_) => return Ok(()),
        Err(_) => println!("Ollama is not running. Starting Ollama..."),
    }

    // Try to start Ollama
    let _start = Command::new("ollama")
        .arg("serve")
        .spawn()?;

    // Wait for Ollama to start (max 30 seconds)
    for _ in 0..30 {
        if client.get("http://localhost:11434/api/version").send().await.is_ok() {
            return Ok(());
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    Err(anyhow!("Failed to start Ollama after 30 seconds"))
}

async fn ensure_model_available(model: &str) -> Result<()> {
    let available_models = get_available_models().await?;
    
    if !available_models.contains(&model.to_string()) {
        println!("Model {} not found. Pulling...", model);
        let pull = Command::new("ollama")
            .args(["pull", model])
            .spawn()?
            .wait()?;

        if !pull.success() {
            return Err(anyhow!("Failed to pull model {}", model));
        }
    }
    Ok(())
}

fn get_git_diff() -> Result<String> {
    let repo = Repository::open(".")?;
    let mut diff_string = String::new();

    for entry in repo.statuses(None)?.iter() {
        if entry.status() != Status::CURRENT {
            let old_file = repo.find_blob(entry.head_to_index().unwrap().old_id())?;
            let new_file = repo.find_blob(entry.head_to_index().unwrap().new_id())?;
            
            diff_string.push_str(&format!("File: {}\n", entry.path().unwrap()));
            diff_string.push_str(&format!("Old:\n{}\n", String::from_utf8_lossy(&old_file.content())));
            diff_string.push_str(&format!("New:\n{}\n", String::from_utf8_lossy(&new_file.content())));
        }
    }

    Ok(diff_string)
}

async fn generate_commit_message(model: &str, diff: &str) -> Result<String> {
    println!("Generating commit message for changes using {}...", model);
    
    let client = reqwest::Client::new();
    let prompt = format!(
        "Generate a concise conventional commit message for the following git diff. \
         Use the format: <type>(<scope>): <description>\n\nDiff:\n{}", 
        diff
    );

    println!("Sending request to Ollama...");
    let response = client.post("http://localhost:11434/api/generate")
        .json(&json!({
            "model": model,
            "prompt": prompt,
            "stream": false
        }))
        .send()
        .await?;

    println!("Got response from Ollama, parsing...");
    let response_json: Value = response.json().await?;
    let message = response_json["response"]
        .as_str()
        .ok_or_else(|| anyhow!("Invalid response format"))?
        .trim()
        .to_string();

    Ok(message)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Ensure Ollama is running
    ensure_ollama_running().await?;

    // Get available models and select one
    let available_models = get_available_models().await?;
    if available_models.is_empty() {
        return Err(anyhow!("No models available in Ollama"));
    }

    let selected_model = select_model(args.model, &available_models);
    println!("Using model: {}", selected_model);

    // Ensure the selected model is available
    ensure_model_available(&selected_model).await?;

    // Get git diff
    let diff = get_git_diff()?;
    if diff.is_empty() {
        return Err(anyhow!("No changes to commit"));
    }

    // Generate commit message
    let commit_message = generate_commit_message(&selected_model, &diff).await?;

    // Create commit
    let status = Command::new("git")
        .args(["commit", "-m", &commit_message])
        .status()?;

    if status.success() {
        println!("Successfully created commit: {}", commit_message);
        Ok(())
    } else {
        Err(anyhow!("Failed to create commit"))
    }
}
