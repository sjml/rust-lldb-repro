use std::process;

use anyhow::{Context, Result};
use subprocess;

fn broken() -> Result<()> {
    let git_status = subprocess::Exec::cmd("git")
        .args(&vec!["status", "--porcelain"])
        .capture()
        .context("Could not run git status.")?;

    println!("{:?}", git_status.stdout_str());

    Ok(())
}

fn works() -> Result<()> {
    let output = process::Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()
        .context("Could not run git status.")?;

    println!("{:?}", std::str::from_utf8(&output.stdout)?);

    Ok(())
}

fn main() -> Result<()> {
    // broken()
    works()
}
