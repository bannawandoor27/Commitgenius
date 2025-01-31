use anyhow::Result;
use clap::Parser;
use commitgenius::{ai, git, github};
use std::env;

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

#[tokio::main]
async fn main() -> Result<()> {
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
    let commit_message = ai::generate_commit_message(&selected_model, &diff).await?;
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
            .create_pull_request(&commit_message, &pr_description, &current_branch, &args.base)
            .await?;
        
        println!("Created pull request from {} to {}", current_branch, args.base);
    }

    Ok(())
}
