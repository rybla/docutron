use anyhow::Result;

fn main() -> Result<()> {
    validate()?;

    println!();
    let label = changelog()?;
    println!();
    commit(&label)?;

    Ok(())
}

fn validate() -> Result<()> {
    println!("Begin validation.");

    println!();
    format()?;
    println!();
    clippy()?;
    println!();
    test()?;

    println!();
    println!("Validation completed successfully.");

    Ok(())
}

fn format() -> Result<()> {
    println!("Begin formatting.");

    let result = std::process::Command::new("cargo")
        .arg("fmt")
        .arg("--all")
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .output()?;
    if !result.status.success() {
        return Err(anyhow::anyhow!("Formatting failed."));
    }

    println!("Formatting completed successfully.");

    Ok(())
}

fn clippy() -> Result<()> {
    println!("Begin typechecking with clippy.");

    let result = std::process::Command::new("cargo")
        .arg("clippy")
        .arg("--all-targets")
        .arg("--all-features")
        .arg("--")
        .arg("-D")
        .arg("warnings")
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .output()?;
    if !result.status.success() {
        return Err(anyhow::anyhow!("Clippy failed."));
    }

    println!("Typechecking with clippy completed successfully.");

    Ok(())
}

fn test() -> Result<()> {
    println!("Begin test suite.");

    let result = std::process::Command::new("cargo")
        .arg("test")
        .arg("--all-targets")
        .arg("--all-features")
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .output()?;

    if !result.status.success() {
        return Err(anyhow::anyhow!("Test suite failed."));
    }

    println!("Test suite completed successfully.");

    Ok(())
}

fn changelog() -> Result<String> {
    println!("Begin creating changelog entry.");

    println!("Write a detailed 1-paragraph summary of your change. Press 'Enter' when finished.");
    let mut summary = String::new();
    std::io::stdin().read_line(&mut summary).unwrap();
    println!(
        "Write a 1-sentence label that encapsulates the changelog entry. Press 'Enter' when finished."
    );

    let mut label = String::new();
    std::io::stdin().read_line(&mut label).unwrap();
    let label = label.trim();

    let timestamp = chrono::Local::now().format("%Y-%m-%d-%H-%M-%S");
    let filename = format!("changelog/{timestamp} {label}.md");
    std::fs::create_dir_all("changelog").unwrap();
    std::fs::write(
        &filename,
        format!("# CHANGELOG {timestamp} {label}\n\n{summary}\n"),
    )
    .unwrap();

    println!("Changelog entry saved to {filename}");

    Ok(label.to_string())
}

fn commit(label: &str) -> Result<()> {
    println!("Beging creating new git commit.");

    let result = std::process::Command::new("git")
        .arg("add")
        .arg(".")
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .output()?;
    if !result.status.success() {
        return Err(anyhow::anyhow!("Git add failed."));
    }

    let result = std::process::Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(label)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .output()?;
    if !result.status.success() {
        return Err(anyhow::anyhow!("Git commit failed."));
    }

    println!("Git commit completed successfully.");

    Ok(())
}
