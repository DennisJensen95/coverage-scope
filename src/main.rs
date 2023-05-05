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
    fn print(&self) {
        println!("package: name: {:#?}", self.name);
        println!("package: line rate: {:#?}", self.line_rate);
        println!("package: branch rate: {:#?}", self.branch_rate);
        println!("package: complexity: {:#?}", self.complexity);

        println!("Filename: {:#?}", self.classes.class.filename);
        println!("Lines covered: {:#?}", self.get_lines_covered());
    }

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

fn main() {
    // Read xml file
    let file_string = std::fs::read_to_string("coverage.xml").unwrap();
    let diff_file_string = run_command(command: "git diff main");

    let patches = match Patch::from_multiple(&diff_file_string) {
        Ok(p) => p,
        Err(e) => panic!("Error: {}", e),
    };

    for patch in patches {
        println!("patch: {:#?}", patch);
    }

    // Parse xml file
    let coverage: Coverage = serde_xml_rs::from_str(&file_string).unwrap();

    println!("coverage: {:#?}", coverage.version);
    println!("coverage: {:#?}", coverage.timestamp);

    for package in coverage.packages.list_of_packages {
        let file_path = package.get_filepath();
        let lines_covered = package.get_lines_covered();

        println!("Filepath: {:#?}", file_path);
        println!("Lines covered: {:#?}", lines_covered);
    }
}
