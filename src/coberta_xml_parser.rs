use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "coverage")]
pub struct Coverage {
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
    class: Vec<Class>,
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

impl Class {
    fn get_filepath(&self) -> String {
        self.filename.clone()
    }

    fn get_lines_covered(&self) -> Vec<usize> {
        let mut lines_covered: Vec<usize> = Vec::new();

        for line in &self.lines.line {
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

// Do an implementation of the Package
impl Package {
    fn get_files(&self) -> Vec<String> {
        let mut files: Vec<String> = Vec::new();

        for class in &self.classes.class {
            files.push(class.get_filepath());
        }
        files
    }

    fn get_lines_covered(&self) -> Vec<usize> {
        let mut lines_covered: Vec<usize> = Vec::new();

        for class in &self.classes.class {
            lines_covered.append(&mut class.get_lines_covered());
        }
        lines_covered
    }
}

impl Coverage {
    pub fn new(file_string: &str) -> Coverage {
        let coverage: Coverage = serde_xml_rs::from_str(&file_string).unwrap();
        coverage
    }

    pub fn get_lines_covered(&self, file_path: &str) -> Vec<usize> {
        let mut lines_covered: Vec<usize> = Vec::new();

        for package in &self.packages.list_of_packages {
            for file in package.get_files() {
                if file == file_path {
                    lines_covered.append(&mut package.get_lines_covered());
                }
            }
        }

        lines_covered
    }
}
