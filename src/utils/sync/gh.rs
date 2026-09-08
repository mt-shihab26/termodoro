use std::{
    io::{Error, ErrorKind, Result},
    process::Command,
};

/// Returns whether the `gh` CLI is installed and signed in to a GitHub account.
pub fn is_authenticated() -> bool {
    Command::new("gh")
        .args(["auth", "status"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Returns the signed-in GitHub username.
fn username() -> Result<String> {
    run(&["api", "user", "-q", ".login"])
}

/// Returns whether `owner/orivo-data` already exists on GitHub.
fn repo_exists(full_name: &str) -> bool {
    Command::new("gh")
        .args(["repo", "view", full_name])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Returns `owner/<repo_name>` for the signed-in user, creating the repo as private if it
/// doesn't exist yet.
pub fn ensure_repo(repo_name: &str) -> Result<String> {
    println!("checking for GitHub repo {repo_name}...");
    let full_name = format!("{}/{repo_name}", username()?);

    if repo_exists(&full_name) {
        println!("found existing repo {full_name}");
    } else {
        println!("creating private repo {full_name}...");
        run(&[
            "repo",
            "create",
            &full_name,
            "--private",
            "--description",
            "orivo sync data \u{2014} managed automatically by `orivo sync`, do not edit",
        ])?;
    }

    Ok(full_name)
}

/// Runs `gh` with the given args, returning trimmed stdout on success.
fn run(args: &[&str]) -> Result<String> {
    let output = Command::new("gh")
        .args(args)
        .output()
        .map_err(|_| io_err("`gh` (GitHub CLI) is not installed; see https://cli.github.com"))?;

    if !output.status.success() {
        return Err(io_err(
            String::from_utf8_lossy(&output.stderr).trim().to_string(),
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn io_err(e: impl std::fmt::Display) -> Error {
    Error::new(ErrorKind::Other, e.to_string())
}
