use anyhow::Result;
use octocrab::Octocrab;
use std::env;
use std::process::Command;

pub struct GithubClient {
    client: Octocrab,
}

impl GithubClient {
    pub fn new() -> Result<Self> {
        let token = env::var("GITHUB_TOKEN")
            .map_err(|_| anyhow::anyhow!("GITHUB_TOKEN not found in environment. Please set it in your .env file"))?;

        let client = Octocrab::builder()
            .personal_token(token)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to create GitHub client: {}", e))?;

        Ok(Self { client })
    }

    // Get repository owner and name from git remote URL
    fn get_repo_info(&self) -> Result<(String, String)> {
        let output = Command::new("git")
            .args(["remote", "get-url", "origin"])
            .output()?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("Failed to get git remote URL"));
        }

        let url = String::from_utf8(output.stdout)?;
        let url = url.trim();

        // Parse GitHub URL (supports both HTTPS and SSH formats)
        let parts: Vec<&str> = if url.starts_with("https://") {
            // Format: https://github.com/owner/repo.git
            url.trim_start_matches("https://github.com/")
                .trim_end_matches(".git")
                .split('/')
                .collect()
        } else {
            // Format: git@github.com:owner/repo.git
            url.trim_start_matches("git@github.com:")
                .trim_end_matches(".git")
                .split('/')
                .collect()
        };

        if parts.len() != 2 {
            return Err(anyhow::anyhow!("Invalid GitHub URL format"));
        }

        Ok((parts[0].to_string(), parts[1].to_string()))
    }

    pub async fn create_pull_request(
        &self,
        title: &str,
        body: &str,
        head: &str,
        base: &str,
    ) -> Result<()> {
        // Get the target repository info from git remote
        let (owner, repo) = self.get_repo_info()?;

        // Get the fork owner (the user who owns the token)
        let fork_owner = self
            .client
            .current()
            .user()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get current user: {}", e))?
            .login;

        // Format head as 'username:branch'
        let head = format!("{}:{}", fork_owner, head);

        println!("Creating PR from {} to {}/{} {}", head, owner, repo, base);

        self.client
            .pulls(owner, repo)
            .create(title, head, base)
            .body(body)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create pull request: {}", e))?;

        Ok(())
    }
}
