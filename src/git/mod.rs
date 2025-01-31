use anyhow::Result;
use std::process::Command;

pub fn get_git_diff() -> Result<String> {
    let output = Command::new("git")
        .args(["diff", "--staged"])
        .output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Failed to get git diff"));
    }

    Ok(String::from_utf8(output.stdout)?)
}

pub fn stage_files(files: &Option<Vec<String>>) -> Result<()> {
    if let Some(files) = files {
        for file in files {
            let status = Command::new("git")
                .args(["add", file])
                .status()?;

            if !status.success() {
                return Err(anyhow::anyhow!("Failed to stage file: {}", file));
            }
        }
    }
    Ok(())
}

pub fn create_commit(message: &str) -> Result<()> {
    let status = Command::new("git")
        .args(["commit", "-m", message])
        .status()?;

    if !status.success() {
        return Err(anyhow::anyhow!("Failed to create commit"));
    }
    Ok(())
}

pub fn get_current_branch() -> Result<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Failed to get current branch"));
    }

    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

pub fn get_commits_since_base(base_branch: &str) -> Result<Vec<String>> {
    let output = Command::new("git")
        .args(["log", &format!("{}..HEAD", base_branch), "--pretty=format:%s"])
        .output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Failed to get commits"));
    }

    Ok(String::from_utf8(output.stdout)?
        .lines()
        .map(String::from)
        .collect())
}
