use mockall::*;

pub struct CommandRunner;

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

        if !output.status.success() {
            panic!(
                "Command failed: {}: error: {}",
                command,
                String::from_utf8_lossy(&output.stderr)
            );
        }
        let error_message = format!("Command output invalid UTF-8 sequence. Command: {command}");
        String::from_utf8(output.stdout).expect(&error_message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_command() {
        let command_runner = CommandRunner;
        let result = command_runner.run_command("echo test");
        assert_eq!(result.trim(), "test");
    }

    #[test]
    #[should_panic]
    fn test_run_command_invalid_command() {
        let command_runner = CommandRunner;
        command_runner.run_command("invalid command");
    }
}
