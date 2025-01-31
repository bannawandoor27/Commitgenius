use anyhow::Result;
use clap::Parser;
use commitgenius::{ai, git, github};
use dotenv::dotenv;
use serde_json::json;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Model to use for generating messages
    #[arg(short, long)]
    model: Option<String>,

    /// Files to commit. Use "." for all changes, or specify individual files
    #[arg(default_value = None)]
    files: Option<Vec<String>>,

    /// Create a pull request after committing
    #[arg(short, long)]
    pull_request: bool,

    /// Base branch for pull request (default: develop)
    #[arg(short, long, default_value = "develop")]
    base: String,
}

fn clean_commit_message(message: &str) -> String {
    // Only clean if the message starts and ends with code block markers
    if message.trim().starts_with("```") && message.trim().ends_with("```") {
        message
            .trim()
            .replace("```", "")
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>()
            .join("\n")
    } else {
        message.trim().to_string()
    }
}

async fn generate_commit_message(model: &str, diff: &str) -> Result<String> {
    println!("Generating commit message for changes using {}...", model);
    
    let client = reqwest::Client::new();
    let prompt = format!(
        "Generate a conventional commit message for the following git diff. \
         IMPORTANT RULES:\n\
         1. Use the format: <type>(<scope>): <description>\n\
         2. Keep it concise and clear\n\
         3. DO NOT use any markdown formatting\n\
         4. DO NOT wrap the message in code blocks\n\
         5. DO NOT add any extra formatting or annotations\n\
         6. The message should be ready to use with 'git commit -m'\n\n\
         Diff:\n{}", 
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
    let response_json: serde_json::Value = response.json().await?;
    let message = response_json["response"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid response format"))?;

    Ok(clean_commit_message(message))
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    let args = Args::parse();

    // Stage files if specified
    git::stage_files(&args.files)?;

    // Ensure Ollama is running
    ai::ensure_ollama_running().await?;

    // Get available models and select one
    let available_models = ai::get_available_models().await?;
    if available_models.is_empty() {
        return Err(anyhow::anyhow!("No models available in Ollama"));
    }

    let selected_model = ai::select_model(args.model, &available_models);
    println!("Using model: {}", selected_model);

    // Ensure the selected model is available
    ai::ensure_model_available(&selected_model).await?;

    // Get git diff
    let diff = git::get_git_diff()?;
    if diff.is_empty() {
        return Err(anyhow::anyhow!("No changes to commit"));
    }

    // Generate and create commit
    let commit_message = generate_commit_message(&selected_model, &diff).await?;
    git::create_commit(&commit_message)?;
    println!("Created commit: {}", commit_message);

    // Create pull request if requested
    if args.pull_request {
        let current_branch = git::get_current_branch()?;
        let commits = git::get_commits_since_base(&args.base)?;

        if commits.is_empty() {
            return Err(anyhow::anyhow!("No commits to create PR for"));
        }

        let pr_description = ai::generate_pr_description(&selected_model, &commits, &diff).await?;

        let github_client = github::GithubClient::new()?;
        github_client
            .create_pull_request(
                &commit_message,
                &pr_description,
                &current_branch,
                &args.base,
            )
            .await?;

        println!(
            "Created pull request from {} to {}",
            current_branch, args.base
        );
    }

    Ok(())
}
