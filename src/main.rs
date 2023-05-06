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
}

fn main() {
    let args = Args::parse();

    // Read xml file
    let file_string = match std::fs::read_to_string(&args.coverage_file) {
        Ok(f) => f,
        Err(e) => panic!("Error reading file: {e}"),
    };

    // Diff command
    let cmd = String::from("git diff ") + &args.branch + " --diff-filter=d";
    println!("Running command: {cmd}");
    let diff_file_string = run_command(&cmd);

    // Parse diff file
    let diff_files = DiffFiles::new(&diff_file_string);
    let coverage: Coverage = Coverage::new(&file_string);

    let line_coverage_percentage = diff_files.calculate_line_coverage(coverage);

    println!("Coverage percentage: {line_coverage_percentage:.2}%",);
}
