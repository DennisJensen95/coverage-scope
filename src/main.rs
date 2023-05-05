use clap::Parser;
use patch::Patch;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "coverage")]
struct Coverage {
    version: String,
    timestamp: String,
    #[serde(rename = "lines-valid")]
    lines_valid: String,
    #[serde(rename = "lines-covered")]
    lines_covered: String,
    #[serde(rename = "line-rate")]
    line_rate: String,
    #[serde(rename = "branches-covered")]
    branches_covered: String,
    #[serde(rename = "branches-valid")]
    branches_valid: String,
    #[serde(rename = "branch-rate")]
    branch_rate: String,
    complexity: String,
    sources: Sources,
    packages: Packages,
}

#[derive(Debug, Deserialize)]
struct Sources {
    source: String,
}

#[derive(Debug, Deserialize)]
struct Packages {
    #[serde(rename = "package")]
    list_of_packages: Vec<Package>,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    #[serde(rename = "line-rate")]
    line_rate: String,
    #[serde(rename = "branch-rate")]
    branch_rate: String,
    complexity: String,
    classes: Classes,
}

#[derive(Debug, Deserialize)]
struct Classes {
    class: Class,
}

#[derive(Debug, Deserialize)]
struct Class {
    name: String,
    filename: String,
    complexity: String,
    #[serde(rename = "line-rate")]
    line_rate: String,
    #[serde(rename = "branch-rate")]
    branch_rate: String,
    methods: Option<Methods>,
    lines: Lines,
}

#[derive(Debug, Deserialize)]
struct Methods {}

#[derive(Debug, Deserialize)]
struct Lines {
    line: Vec<Line>,
}

#[derive(Debug, Deserialize)]
struct Line {
    number: String,
    hits: String,
}

// Do an implementation of the Package
impl Package {
    fn get_filepath(&self) -> String {
        self.classes.class.filename.clone()
    }

    fn get_lines_covered(&self) -> Vec<usize> {
        let mut lines_covered: Vec<usize> = Vec::new();

        for line in &self.classes.class.lines.line {
            if line.hits != "0" {
                let line_number = line.number.parse::<usize>();
                match line_number {
                    Ok(n) => lines_covered.push(n),
                    Err(e) => println!("Error: {}", e),
                }
            }
        }
        lines_covered
    }
}

impl Coverage {
    fn new(file_string: &str) -> Coverage {
        let coverage: Coverage = serde_xml_rs::from_str(&file_string).unwrap();
        coverage
    }

    fn get_lines_covered(&self, file_path: &str) -> Vec<usize> {
        let mut lines_covered: Vec<usize> = Vec::new();

        for package in &self.packages.list_of_packages {
            if package.get_filepath() == file_path {
                lines_covered = package.get_lines_covered();
            }
        }

        lines_covered
    }
}

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

struct DiffFiles {
    files: Vec<(String, Vec<usize>)>,
}

impl DiffFiles {
    fn new(diff_file_string: &str) -> DiffFiles {
        // Parse the diff file and return file paths and line numbers changed
        let patches = match Patch::from_multiple(&diff_file_string) {
            Ok(p) => p,
            Err(e) => panic!("Error parsing diff file: {}", e),
        };

        let mut files_changed = DiffFiles { files: Vec::new() };

        for patch in patches {
            let file_path = patch.new.path;

            let mut lines_changed: Vec<usize> = Vec::new();

            for hunk in patch.hunks {
                let line_range = hunk.new_range;
                for line_number in line_range.start..line_range.start + line_range.count {
                    lines_changed.push(line_number as usize);
                }
            }

            // Remove b/ from the start of the file path
            let file_path = file_path[2..].to_string();

            files_changed.files.push((file_path.into(), lines_changed));
        }

        files_changed
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
        Err(e) => panic!("Error reading file: {}", e),
    };

    // Diff command
    let cmd = String::from("git diff ") + &args.branch;
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

        println!("File: {}", file_path);
        println!("Lines changed: {:?}", lines_changed);
        println!("Lines covered: {:?}", lines_covered);
    }

    let coverage_percentage = total_lines_covered as f32 / total_lines_changed as f32 * 100.0;

    println!("Total lines changed: {}", total_lines_changed);
    println!("Total lines covered: {}", total_lines_covered);
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
