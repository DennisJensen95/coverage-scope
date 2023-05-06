use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "coverage")]
pub struct Coverage {
    #[serde(rename = "version")]
    _version: String,
    #[serde(rename = "timestamp")]
    _timestamp: String,
    #[serde(rename = "lines-valid")]
    _lines_valid: String,
    #[serde(rename = "lines-covered")]
    _lines_covered: String,
    #[serde(rename = "line-rate")]
    _line_rate: String,
    #[serde(rename = "branches-covered")]
    _branches_covered: String,
    #[serde(rename = "branches-valid")]
    _branches_valid: String,
    #[serde(rename = "branch-rate")]
    _branch_rate: String,
    #[serde(rename = "complexity")]
    _complexity: String,
    #[serde(rename = "version")]
    _sources: Sources,
    packages: Packages,
}

#[derive(Debug, Deserialize)]
struct Sources {
    #[serde(rename = "source")]
    _source: String,
}

#[derive(Debug, Deserialize)]
struct Packages {
    #[serde(rename = "package")]
    list_of_packages: Vec<Package>,
}

#[derive(Debug, Deserialize)]
struct Package {
    #[serde(rename = "name")]
    _name: String,
    #[serde(rename = "line-rate")]
    _line_rate: String,
    #[serde(rename = "branch-rate")]
    _branch_rate: String,
    #[serde(rename = "complexity")]
    _complexity: String,
    classes: Classes,
}

#[derive(Debug, Deserialize)]
struct Classes {
    class: Vec<Class>,
}

#[derive(Debug, Deserialize)]
struct Class {
    #[serde(rename = "name")]
    _name: String,
    #[serde(rename = "filename")]
    filename: String,
    #[serde(rename = "complexity")]
    _complexity: String,
    #[serde(rename = "line-rate")]
    _line_rate: String,
    #[serde(rename = "branch-rate")]
    _branch_rate: String,
    #[serde(rename = "methods")]
    _methods: Option<Methods>,
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
                    Err(e) => println!("Error: {e}"),
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
        let coverage: Coverage = serde_xml_rs::from_str(file_string).unwrap();
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
