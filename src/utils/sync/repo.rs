use std::{
    fs,
    io::{Error, ErrorKind, Result},
    path::Path,
    process::Command,
};

/// Fixed name of the synced database blob inside the sync repo.
const FILE_NAME: &str = "orivo.sqlite.gz";

/// Ensures `dir` is a local clone of `repo_full_name` (`owner/name`): clones it via `gh` if
/// missing, otherwise fetches the latest remote state into it.
pub fn ensure_clone(dir: &Path, repo_full_name: &str) -> Result<()> {
    if dir.join(".git").exists() {
        println!("fetching latest changes from {repo_full_name}...");
        run(dir, &["fetch", "origin"])?;
        return Ok(());
    }

    println!("cloning {repo_full_name}...");
    if let Some(parent) = dir.parent() {
        fs::create_dir_all(parent)?;
    }

    let output = Command::new("gh")
        .args(["repo", "clone", repo_full_name, &dir.to_string_lossy()])
        .output()
        .map_err(|_| io_err("`gh` (GitHub CLI) is not installed; see https://cli.github.com"))?;

    if !output.status.success() {
        return Err(io_err(
            String::from_utf8_lossy(&output.stderr).trim().to_string(),
        ));
    }

    Ok(())
}

/// Returns the clone's current branch name (works even for a freshly created, commit-less
/// repo, since `HEAD` still symbolically points at the default branch).
pub fn current_branch(dir: &Path) -> Result<String> {
    run(dir, &["symbolic-ref", "--short", "HEAD"])
}

/// Reads the sync file's bytes as committed on `origin/<branch>`, or `None` if it doesn't
/// exist yet (e.g. nothing has been pushed to the repo before).
pub fn read_remote_file(dir: &Path, branch: &str) -> Option<Vec<u8>> {
    Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(["show", &format!("origin/{branch}:{FILE_NAME}")])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| output.stdout)
}

/// Writes `bytes` as the sync file and pushes it to `branch`, overwriting the repo's
/// previous snapshot in place: subsequent syncs amend and force-push the same single commit
/// rather than growing the repo's history forever.
pub fn commit_and_push(dir: &Path, branch: &str, bytes: &[u8], message: &str) -> Result<()> {
    fs::write(dir.join(FILE_NAME), bytes)?;
    run(dir, &["add", FILE_NAME])?;

    let has_commit = run(dir, &["rev-parse", "--verify", "-q", "HEAD"]).is_ok();
    if has_commit {
        run(dir, &["commit", "--amend", "-m", message])?;
    } else {
        run(dir, &["commit", "-m", message])?;
    }

    println!("pushing to GitHub...");
    run(
        dir,
        &[
            "push",
            "--force-with-lease",
            "origin",
            &format!("HEAD:{branch}"),
        ],
    )?;

    Ok(())
}

/// Runs `git -C dir <args>`, returning trimmed stdout on success.
fn run(dir: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .map_err(|_| io_err("`git` is not installed"))?;

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
