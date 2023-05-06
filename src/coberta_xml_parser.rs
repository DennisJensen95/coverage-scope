use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
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
    line_rate: String,
    #[serde(rename = "branches-covered")]
    _branches_covered: String,
    #[serde(rename = "branches-valid")]
    _branches_valid: String,
    #[serde(rename = "branch-rate")]
    _branch_rate: String,
    #[serde(rename = "complexity")]
    _complexity: String,
    #[serde(rename = "sources")]
    _sources: Sources,
    packages: Packages,
}

#[derive(Debug, Deserialize, Clone)]
struct Sources {
    #[serde(rename = "source")]
    _source: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct Packages {
    #[serde(rename = "package")]
    list_of_packages: Vec<Package>,
}

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Deserialize, Clone)]
struct Classes {
    class: Vec<Class>,
}

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Deserialize, Clone)]
struct Methods {}

#[derive(Debug, Deserialize, Clone)]
struct Lines {
    line: Vec<Line>,
}

#[derive(Debug, Deserialize, Clone)]
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

    fn get_lines_covered(&self, file_path: String) -> Vec<usize> {
        let mut lines_covered: Vec<usize> = Vec::new();

        for class in &self.classes.class {
            if class.get_filepath() == file_path {
                lines_covered.append(&mut class.get_lines_covered());
            }
        }
        lines_covered
    }

    fn get_total_lines(&self, file_path: String) -> Vec<usize> {
        let mut lines_total: Vec<usize> = Vec::new();

        for class in &self.classes.class {
            if class.get_filepath() == file_path {
                for line in &class.lines.line {
                    let line_number = line.number.parse::<usize>();
                    match line_number {
                        Ok(n) => lines_total.push(n),
                        Err(e) => println!("Error: {e}"),
                    }
                }
            }
        }
        lines_total
    }
}

impl Coverage {
    pub fn new(file_string: &str) -> Coverage {
        let coverage: Coverage = serde_xml_rs::from_str(file_string).unwrap();
        coverage
    }

    pub fn get_total_coverage(&self) -> f64 {
        self.line_rate.parse::<f64>().unwrap() * 100.0
    }

    pub fn get_lines_covered(&self, file_path: &str) -> Vec<usize> {
        let mut lines_covered: Vec<usize> = Vec::new();

        for package in &self.packages.list_of_packages {
            for file in package.get_files() {
                if file == file_path {
                    lines_covered.append(&mut package.get_lines_covered(file_path.into()));
                }
            }
        }

        lines_covered
    }

    pub fn get_lines_with_code(&self, file_path: &str) -> Vec<usize> {
        let mut lines_with_code: Vec<usize> = Vec::new();

        for package in &self.packages.list_of_packages {
            for file in package.get_files() {
                if file == file_path {
                    lines_with_code.append(&mut package.get_total_lines(file_path.into()));
                }
            }
        }

        lines_with_code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_lines_covered() {
        let file_string = std::fs::read_to_string("assets/coberta_coverage/coverage.xml").unwrap();
        let coverage = Coverage::new(&file_string);

        let files_covered = coverage.packages.list_of_packages[0].get_files();
        assert_eq!(files_covered.len(), 2);

        // new.py files covered 0
        let lines_covered = coverage.get_lines_covered("test_repo/new.py");
        assert_eq!(lines_covered.len(), 0);

        // some_module.py files covered 2
        let lines_covered = coverage.get_lines_covered("test_repo/some_module.py");
        assert_eq!(lines_covered.len(), 3);
    }

    #[test]
    fn test_get_files() {
        let file_string = std::fs::read_to_string("assets/coberta_coverage/coverage.xml").unwrap();
        let coverage = Coverage::new(&file_string);

        let files_covered = coverage.packages.list_of_packages[0].get_files();
        assert_eq!(files_covered.len(), 2);
    }

    #[test]
    fn test_get_total_coverage() {
        let file_string = std::fs::read_to_string("assets/coberta_coverage/coverage.xml").unwrap();
        let coverage = Coverage::new(&file_string);

        let total_coverage = coverage.get_total_coverage();
        assert_eq!(total_coverage, 37.5);
    }

    #[test]
    fn test_read_another_coverage_file() {
        let file_string =
            std::fs::read_to_string("assets/coberta_coverage/ler_gml_backend_coverage.xml")
                .unwrap();
        let coverage = Coverage::new(&file_string);

        let total_coverage = coverage.get_total_coverage();
        assert_eq!(total_coverage, 93.76);
    }
}
