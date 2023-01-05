use anyhow::{Context, Result};
use subprocess;

fn main() -> Result<()> {
    let git_status = subprocess::Exec::cmd("git")
        .args(&vec!["status", "--porcelain"])
        .capture()
        .context("Could not run git status.")?;

    println!("{:?}", git_status.stdout_str());

    Ok(())
}
