use anyhow::Result;
use octocrab::Octocrab;
use std::env;

pub struct GithubClient {
    client: Octocrab,
    owner: String,
    repo: String,
}

impl GithubClient {
    pub fn new() -> Result<Self> {
        let token = env::var("GITHUB_TOKEN")
            .map_err(|_| anyhow::anyhow!("GITHUB_TOKEN environment variable not set"))?;
        
        let owner = env::var("GITHUB_REPO_OWNER")
            .map_err(|_| anyhow::anyhow!("GITHUB_REPO_OWNER environment variable not set"))?;
        
        let repo = env::var("GITHUB_REPO_NAME")
            .map_err(|_| anyhow::anyhow!("GITHUB_REPO_NAME environment variable not set"))?;

        let client = Octocrab::builder()
            .personal_token(token)
            .build()?;

        Ok(Self {
            client,
            owner,
            repo,
        })
    }

    pub async fn create_pull_request(
        &self,
        title: &str,
        body: &str,
        head: &str,
        base: &str,
    ) -> Result<()> {
        self.client
            .pulls(self.owner.clone(), self.repo.clone())
            .create(title, head, base)
            .body(body)
            .send()
            .await?;

        Ok(())
    }
}
