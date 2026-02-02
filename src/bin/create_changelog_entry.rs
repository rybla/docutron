fn main() {
    println!(
        "Write a detailed summary of your changes. Include concise written explanations and short code snippets."
    );
    let mut summary = String::new();
    std::io::stdin().read_line(&mut summary).unwrap();
    println!("Write a 1-sentence label that encapsulates the changelog entry.");

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
}
