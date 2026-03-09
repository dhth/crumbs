use anyhow::Context;
use std::path::Path;
use std::process::Command;

pub fn current_branch(path: &Path) -> anyhow::Result<Option<String>> {
    let output = Command::new("git")
        .args(["branch", "--show-current"])
        .current_dir(path)
        .output()
        .context("couldn't run git to determine current branch")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!(
            "git branch --show-current failed with status {}: {}",
            output.status,
            stderr.trim()
        );
    }

    let branch = String::from_utf8(output.stdout).context("command output was not valid utf-8")?;
    let branch = branch.trim();

    if branch.is_empty() {
        return Ok(None);
    }

    Ok(Some(branch.to_owned()))
}
