// Commitgenius - An AI-powered conventional commit message generator
// Uses Ollama for local LLM inference to create meaningful commit messages

use anyhow::{Context, Result};
use clap::Parser;
use git2::Repository;
use reqwest::Client;
use serde_json::{json, Value};
use std::process::Command;
use std::thread;
use std::time::Duration;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "qwen2.5:7b")]
    model: String,
}

async fn check_ollama_status() -> Result<bool> {
    let client = Client::new();
    match client.get("http://localhost:11434/api/version").send().await {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

fn start_ollama() -> Result<()> {
    println!("Starting Ollama service...");
    
    // Start ollama serve in the background
    #[cfg(target_os = "macos")]
    Command::new("sh")
        .args(["-c", "ollama serve > /dev/null 2>&1 &"])
        .spawn()
        .context("Failed to start Ollama service")?;
    
    // Give it some time to start
    thread::sleep(Duration::from_secs(2));
    Ok(())
}

async fn ensure_model_available(model: &str) -> Result<()> {
    let output = Command::new("ollama")
        .args(["list"])
        .output()
        .context("Failed to list Ollama models")?;
    
    let models = String::from_utf8_lossy(&output.stdout);
    if !models.contains(model) {
        println!("Model {} not found. Pulling it now...", model);
        let pull = Command::new("ollama")
            .args(["pull", model])
            .spawn()
            .context("Failed to pull model")?;
            
        // Wait for pull to complete
        thread::sleep(Duration::from_secs(5));
    }
    Ok(())
}

fn ensure_ollama_installed() -> Result<()> {
    match Command::new("ollama").arg("--version").output() {
        Ok(_) => Ok(()),
        Err(_) => {
            println!("Ollama not found. Installing via Homebrew...");
            let brew_output = Command::new("brew")
                .args(["install", "ollama"])
                .output()
                .context("Failed to install Ollama via Homebrew")?;
            
            if !brew_output.status.success() {
                anyhow::bail!("Failed to install Ollama: {}", 
                    String::from_utf8_lossy(&brew_output.stderr));
            }
            Ok(())
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Ensure Ollama is installed and running
    ensure_ollama_installed()?;
    
    let mut retries = 3;
    while !check_ollama_status().await? && retries > 0 {
        start_ollama()?;
        thread::sleep(Duration::from_secs(3));
        retries -= 1;
    }
    
    if !check_ollama_status().await? {
        anyhow::bail!("Failed to start Ollama service after multiple attempts");
    }
    
    // Ensure the model is available
    ensure_model_available(&cli.model).await?;
    
    // Get git diff
    let diff = get_git_diff()?;
    if diff.is_empty() {
        println!("No changes to commit!");
        return Ok(());
    }

    println!("Generating commit message for changes using {}...", cli.model);
    
    // Generate commit message using Ollama
    let commit_msg = generate_commit_message(&diff, &cli.model).await?;
    
    // Create git commit
    create_git_commit(&commit_msg)?;
    
    println!("Successfully created commit: {}", commit_msg);
    Ok(())
}

fn get_git_diff() -> Result<String> {
    let output = Command::new("git")
        .args(["diff", "--cached"])
        .output()
        .context("Failed to execute git diff")?;
    
    if !output.status.success() {
        anyhow::bail!("Git diff command failed");
    }
    
    String::from_utf8(output.stdout)
        .context("Failed to parse git diff output")
}

async fn generate_commit_message(diff: &str, model: &str) -> Result<String> {
    let prompt = format!(
        "Generate a concise conventional commit message for the following git diff. \
        Use one of these types: feat, fix, docs, style, refactor, test, chore. \
        Format: <type>(<optional scope>): <description>\n\nGit diff:\n{}", 
        diff
    );

    println!("Sending request to Ollama...");
    let client = Client::new();
    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&json!({
            "model": model,
            "prompt": prompt,
            "stream": false
        }))
        .send()
        .await
        .context("Failed to send request to Ollama")?;

    println!("Got response from Ollama, parsing...");
    let response: Value = response
        .json()
        .await
        .context("Failed to parse Ollama response")?;

    let commit_msg = response["response"]
        .as_str()
        .context("Failed to get response from Ollama")?
        .trim()
        .to_string();

    Ok(commit_msg)
}

fn create_git_commit(message: &str) -> Result<()> {
    let repo = Repository::open(".")?;
    let signature = repo.signature()?;
    let tree_id = repo.index()?.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    
    let head = repo.head()?;
    let parent = repo.find_commit(head.target().unwrap())?;
    
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &[&parent],
    )?;
    
    Ok(())
}
