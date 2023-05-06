use clap::Parser;

mod coberta_xml_parser;
mod git_diff_parser;

use coberta_xml_parser::Coverage;
use git_diff_parser::DiffFiles;
use mockall::*;

struct CommandRunner;

#[automock]
pub trait CommandRunnerTrait {
    fn run_command(&self, command: &str) -> String;
}

impl CommandRunnerTrait for CommandRunner {
    fn run_command(&self, command: &str) -> String {
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

    /// Fail if coverage of changed files is below this threshold
    #[arg(long, default_value = "0.0")]
    threshold_change: f32,

    /// Fail if total coverage is below this threshold
    #[arg(long, default_value = "0.0")]
    threshold_total: f32,

    /// Path to the git repository
    /// If not specified, the current directory is used
    #[arg(short, long)]
    git_dir: Option<String>,
}

fn change_directory(dir: Option<String>) {
    // Check if dir exists
    if let Some(dir) = dir {
        let git_dir = dir;

        if !std::path::Path::new(&git_dir).exists() {
            panic!("Directory does not exist: {git_dir}");
        }

        std::env::set_current_dir(git_dir.clone())
            .unwrap_or_else(|_| panic!("Error changing directory to {git_dir}"));
    }
}

fn get_coverage_on_diff(diff_string: &str, coverage: &Coverage, threshold: f32) -> bool {
    let diff_files = DiffFiles::new(diff_string);
    let line_coverage_percentage = diff_files.calculate_line_coverage(coverage.clone());
    println!("Current changes coverage is {line_coverage_percentage:.2}%");
    let mut threshold_not_met = false;
    if line_coverage_percentage < threshold {
        println!(
            "Current changes coverage is below threshold of {:.2}%",
            threshold
        );
        threshold_not_met = true;
    }

    threshold_not_met
}

fn run_app(args: Args, command_runner: &dyn CommandRunnerTrait) -> bool {
    // Format string
    let file_string = std::fs::read_to_string(&args.coverage_file)
        .unwrap_or_else(|_| panic!("Error reading file {}", args.coverage_file));

    change_directory(args.git_dir);

    // Diff command
    let cmd = String::from("git diff ") + &args.branch + " --diff-filter=d";
    println!("Running command: {cmd}");
    let diff_file_string = command_runner.run_command(&cmd);

    // Parse diff file
    let coverage: Coverage = Coverage::new(&file_string);
    let mut threshold_not_met = false;

    if !diff_file_string.is_empty() {
        threshold_not_met =
            get_coverage_on_diff(&diff_file_string, &coverage, args.threshold_change);
    } else {
        println!("No changes found");
    }

    let total_coverage_percentage = coverage.get_total_coverage();
    println!("Total coverage is {total_coverage_percentage:.2}%");

    if total_coverage_percentage < args.threshold_total as f64 {
        println!(
            "Total coverage is below threshold of {:.2}%",
            args.threshold_total
        );
        threshold_not_met = true;
    }

    !threshold_not_met
}

fn main() {
    let args = Args::parse();
    let command_runner = CommandRunner;

    let state = run_app(args, &command_runner);
    if !state {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::MockCommandRunnerTrait;
    use super::*;

    #[test]
    fn test_change_directory() {
        // Test existing directory
        change_directory(Some(String::from(".")));
        assert_eq!(
            std::env::current_dir().unwrap().display().to_string(),
            std::env::current_dir().unwrap().display().to_string()
        );

        // Test non-existing directory
        assert!(
            std::panic::catch_unwind(|| change_directory(Some(String::from(
                "non-existing-directory"
            ))))
            .is_err()
        );
    }

    #[test]
    fn test_run_command() {
        let command_runner = CommandRunner;
        let result = command_runner.run_command("echo test");
        assert_eq!(result.trim(), "test");
    }

    #[test]
    fn test_get_coverage_on_diff() {
        // Set up sample input data
        let diff_string = std::fs::read_to_string("assets/diff_files/coverage.diff").unwrap();
        let coverage_string =
            std::fs::read_to_string("assets/coberta_coverage/coverage.xml").unwrap();
        let coverage = Coverage::new(&coverage_string);

        // Test with threshold below actual coverage
        let result1 = get_coverage_on_diff(diff_string.as_str(), &coverage, 80.0);
        assert!(result1);

        // Test with threshold above actual coverage
        let result2 = get_coverage_on_diff(diff_string.as_str(), &coverage, 20.0);
        assert!(!result2);
    }

    #[test]
    fn test_run_app() {
        // Set up sample input data
        let expected_success_args = Args {
            coverage_file: "assets/coberta_coverage/repo_coverage.xml".to_string(),
            git_dir: Some(".".into()),
            branch: "main".to_string(),
            threshold_change: 5.0,
            threshold_total: 0.0,
        };
        let expected_fail_args_not_high_coverage = Args {
            coverage_file: "assets/coberta_coverage/repo_coverage.xml".to_string(),
            git_dir: Some(".".into()),
            branch: "main".to_string(),
            threshold_change: 100.0,
            threshold_total: 0.0,
        };

        let expected_fail_args_fail_on_total = Args {
            coverage_file: "assets/coberta_coverage/repo_coverage.xml".to_string(),
            git_dir: Some(".".into()),
            branch: "main".to_string(),
            threshold_change: 0.0,
            threshold_total: 100.0,
        };

        // Diff from file
        let diff_string = std::fs::read_to_string("assets/diff_files/repo_test.diff").unwrap();

        // Mocked command runner
        let mut mock_command_runner = MockCommandRunnerTrait::new();
        mock_command_runner
            .expect_run_command()
            .return_const(diff_string);

        // Run the app with the mocked runner and expect an ok
        let state = run_app(expected_success_args, &mock_command_runner);
        assert!(state);

        // Run the app with the mocked runner and expect a too low coverage
        let state = run_app(expected_fail_args_not_high_coverage, &mock_command_runner);
        assert!(!state);

        // Run the app with the mocked runner and expect a too low coverage because of total and current coverage
        let state = run_app(expected_fail_args_fail_on_total, &mock_command_runner);
        assert!(!state);
    }
}
