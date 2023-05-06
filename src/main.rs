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

    match String::from_utf8(output.stdout) {
        Ok(s) => s,
        Err(e) => panic!("Invalid UTF-8 sequence: {e}"),
    }
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

    /// Path to the git repository
    /// If not specified, the current directory is used
    #[arg(short, long)]
    git_dir: Option<String>,

    /// Fail on total coverage below threshold
    #[arg(short, long)]
    fail_on_total: bool,
}

fn change_directory(dir: Option<String>) {
    // Check if dir exists
    if dir.is_some() {
        let git_dir = dir.unwrap();
        if !std::path::Path::new(&git_dir).exists() {
            panic!("Directory does not exist: {git_dir}");
        }

        std::env::set_current_dir(git_dir.clone())
            .expect(format!("Error changing directory to {git_dir}").as_str());
    }
}

fn main() {
    let args = Args::parse();

    // Format string
    let file_string = std::fs::read_to_string(&args.coverage_file)
        .expect(format!("Error reading file {}", args.coverage_file).as_str());

    change_directory(args.git_dir);

    // Diff command
    let cmd = String::from("git diff ") + &args.branch + " --diff-filter=d";
    println!("Running command: {cmd}");
    let diff_file_string = run_command(&cmd);

    // Parse diff file
    let coverage: Coverage = Coverage::new(&file_string);
    let mut threshold_not_met = false;

    if !diff_file_string.is_empty() {
        let diff_files = DiffFiles::new(&diff_file_string);
        let line_coverage_percentage = diff_files.calculate_line_coverage(coverage.clone());
        println!("Current changes coverage is {line_coverage_percentage:.2}%");

        if line_coverage_percentage < args.threshold {
            println!(
                "Current changes coverage is below threshold of {:.2}%",
                args.threshold
            );
            threshold_not_met = true;
        }
    }

    let total_coverage_percentage = coverage.get_total_coverage();
    println!("Total coverage is {total_coverage_percentage:.2}%");

    if args.fail_on_total && total_coverage_percentage < args.threshold as f64 {
        println!(
            "Total coverage is below threshold of {:.2}%",
            args.threshold
        );
        threshold_not_met = true;
    }

    if threshold_not_met {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_lines_covered() {}
}
