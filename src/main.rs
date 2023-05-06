use clap::Parser;

mod coberta_xml_parser;
mod git_diff_parser;

use coberta_xml_parser::Coverage;
use git_diff_parser::DiffFiles;

// Make a function that runs a system command
fn run_command(command: &str) -> String {
    let output = std::process::Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8(output.stdout).unwrap();
    stdout
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Coverage file to use
    #[arg(short, long)]
    coverage_file: String,

    /// Branch to compare against
    #[arg(short, long)]
    branch: String,

    /// Fail if coverage is below this threshold
    #[arg(short, long, default_value = "0.0")]
    threshold: f32,
}

fn main() {
    let args = Args::parse();

    // Read xml file
    let file_string = match std::fs::read_to_string(&args.coverage_file) {
        Ok(f) => f,
        Err(e) => panic!("Error reading file: {}", e),
    };

    // Diff command
    let cmd = String::from("git diff ") + &args.branch + " --diff-filter=d";
    println!("Running command: {}", cmd);
    let diff_file_string = run_command(&cmd);

    // Parse diff file
    let diff_files = DiffFiles::new(&diff_file_string);
    let coverage: Coverage = Coverage::new(&file_string);

    println!("Coverage file: {}", args.coverage_file);
    println!("Count of files changed: {}", diff_files.files.len());

    let mut total_lines_changed = 0;
    let mut total_lines_covered = 0;
    for file in diff_files.files {
        let file_path = file.0;

        // Check if the file is a code file
        if !(file_path.ends_with(".rs") || file_path.ends_with(".py")) {
            continue;
        }

        let lines_changed = file.1;
        let lines_covered = coverage.get_lines_covered(file_path.as_str());

        // Count the lines covered out the lines changed
        let mut lines_covered_count = 0;
        for line in &lines_changed {
            if lines_covered.contains(&line) {
                lines_covered_count += 1;
            }
        }

        total_lines_changed += lines_changed.len();
        total_lines_covered += lines_covered_count;
    }

    let coverage_percentage = total_lines_covered as f32 / total_lines_changed as f32 * 100.0;

    println!("Total lines changed in new commit: {}", total_lines_changed);
    println!("Total lines covered in new commit: {}", total_lines_covered);
    println!("Coverage percentage: {:.2}%", coverage_percentage);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_diff_file() {
        // Read diff file from assets/diff_files/coverage.diff
        let diff_file_string = std::fs::read_to_string("assets/diff_files/coverage.diff").unwrap();
    }
}
