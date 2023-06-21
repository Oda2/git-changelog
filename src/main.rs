use clap::{App, Arg};
use std::fs::File;
use std::io::Write;
use std::process::Command;

fn main() {
    let matches = App::new("Changelog Generator")
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .takes_value(true),
        )
        .get_matches();

    let output_file = matches.value_of("output").unwrap_or("changelog.md");

    let output = Command::new("git")
        .arg("log")
        .arg("--date=short")
        .arg("--pretty=format:%h %ad %s")
        .output()
        .expect("Failed to execute git log command");

    let commits = String::from_utf8_lossy(&output.stdout);
    let commits = commits.trim().split('\n');

    let mut file = File::create(output_file).expect("Failed to create output file");

    let mut current_tag = String::new();

    for commit in commits {
        let commit_parts: Vec<&str> = commit.splitn(3, ' ').collect();
        let commit_hash = commit_parts[0];
        let commit_date = commit_parts[1];
        let commit_message = commit_parts[2];

        let output = Command::new("git")
            .arg("describe")
            .arg("--exact-match")
            .arg("--tags")
            .arg(commit_hash)
            .output()
            .expect("Failed to execute git describe command");

        if output.status.success() {
            let tag = String::from_utf8_lossy(&output.stdout).trim().to_string();

            if !current_tag.is_empty() && current_tag != tag {
                writeln!(file).expect("Failed to write to output file");
            }

            current_tag = tag;
            writeln!(file, "{}", current_tag).expect("Failed to write to output file");
        }

        writeln!(
            file,
            "- {} - {} - {}",
            commit_message, commit_date, commit_hash
        )
        .expect("Failed to write to output file");
    }
}
