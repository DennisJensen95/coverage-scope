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
    _complexity: Option<String>,
    #[serde(rename = "sources")]
    sources: Sources,
    packages: Packages,
}

#[derive(Debug, Deserialize, Clone)]
struct Sources {
    #[serde(rename = "source")]
    source: Vec<String>,
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
    _complexity: Option<String>,
    classes: Classes,
}

#[derive(Debug, Deserialize, Clone)]
struct Classes {
    class: Option<Vec<Class>>,
}

#[derive(Debug, Deserialize, Clone)]
struct Class {
    #[serde(rename = "name")]
    _name: String,
    #[serde(rename = "filename")]
    filename: String,
    #[serde(rename = "complexity")]
    _complexity: Option<String>,
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
    line: Option<Vec<Line>>,
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

    fn get_filename(&self) -> String {
        let filepath = self.get_filepath();
        match filepath.split('/').last() {
            Some(p) => String::from(p),
            None => filepath,
        }
    }

    fn get_lines_covered(&self) -> Vec<usize> {
        let mut lines_covered: Vec<usize> = Vec::new();

        if self.lines.line.is_none() {
            return lines_covered;
        }

        for line in &self.lines.line.clone().unwrap() {
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

    fn get_all_lines(&self) -> Vec<usize> {
        let mut all_lines: Vec<usize> = Vec::new();

        if self.lines.line.is_none() {
            return all_lines;
        }

        for line in &self.lines.line.clone().unwrap() {
            let line_number = line.number.parse::<usize>();
            match line_number {
                Ok(n) => all_lines.push(n),
                Err(e) => println!("Error: {e}"),
            }
        }
        all_lines
    }
}

// Do an implementation of the Package
impl Package {
    fn get_classes(&self) -> Vec<Class> {
        if self.classes.class.is_none() {
            return Vec::new();
        }
        self.classes.class.clone().unwrap()
    }
}

impl Coverage {
    pub fn new(file_string: &str) -> Coverage {
        // Remove BOM if it exists (C# support)
        let clean_string = file_string.trim_start_matches('\u{feff}');

        let coverage: Coverage = serde_xml_rs::from_str(clean_string).unwrap();
        coverage
    }

    pub fn get_total_coverage(&self) -> f64 {
        self.line_rate.parse::<f64>().unwrap() * 100.0
    }

    pub fn get_lines_covered(&self, file_path: &str) -> Vec<usize> {
        let class = match self.match_filepath_to_package(file_path) {
            Some(p) => p,
            None => return Vec::new(),
        };

        class.get_lines_covered()
    }

    pub fn get_lines_with_code(&self, file_path: &str) -> Vec<usize> {
        let class = match self.match_filepath_to_package(file_path) {
            Some(p) => p,
            None => return Vec::new(),
        };

        class.get_all_lines()
    }

    pub fn get_file_extensions(&self) -> Vec<String> {
        let mut file_extensions: Vec<String> = Vec::new();

        for package in &self.packages.list_of_packages {
            for class in package.get_classes() {
                let filename = class.get_filename();

                let file_extension = match filename.split('.').last() {
                    Some(p) => p,
                    None => continue,
                };

                let extension = format!(".{}", file_extension);

                if !file_extensions.contains(&extension.to_string()) {
                    // Push file extension and prepend a . to it
                    file_extensions.push(extension);
                }
            }
        }
        file_extensions
    }

    fn match_filepath_to_package(&self, file_path: &str) -> Option<Class> {
        let potential_prepend_paths_sources = &self.sources.source;

        for package in &self.packages.list_of_packages {
            for potential_path in potential_prepend_paths_sources {
                for class in package.get_classes() {
                    let filename = class.get_filename();
                    let file_path_of_class = class.get_filepath();

                    // First check if a source plus a filename matches the file_path
                    let mut potential_filpath_with_filename = potential_path.clone();
                    potential_filpath_with_filename.push('/');
                    potential_filpath_with_filename.push_str(&filename);
                    if potential_filpath_with_filename.contains(file_path) {
                        return Some(class);
                    }

                    // Then check if a source plus a filepath matches the file_path
                    let mut potential_path = potential_path.clone();
                    potential_path.push('/');
                    potential_path.push_str(&file_path_of_class);
                    if potential_path.contains(file_path) {
                        return Some(class);
                    }

                    // Then check if the filename matches the file_path
                    if file_path_of_class == file_path {
                        return Some(class);
                    }

                    // Then check if the filename matches the file_path
                    if filename == file_path {
                        return Some(class);
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_lines_covered() {
        let file_string = std::fs::read_to_string("assets/coberta_coverage/coverage.xml").unwrap();
        let coverage = Coverage::new(&file_string);

        let files_covered = coverage.packages.list_of_packages[0].classes.class.clone();
        assert_eq!(
            files_covered.expect("Could not unwrap files covered").len(),
            3
        );

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

        let files_covered = coverage.packages.list_of_packages[0].classes.class.clone();
        assert_eq!(
            files_covered.expect("Could not unwrap files covered").len(),
            3
        );
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
        assert_eq!(total_coverage, 95.32000000000001);
    }

    #[test]
    fn test_get_lines_with_code() {
        let file_string =
            std::fs::read_to_string("assets/coberta_coverage/test_1_coverage.xml").unwrap();
        let coverage = Coverage::new(&file_string);

        let lines_with_code = coverage.get_lines_with_code("src/analytics/firebase_interface.py");
        assert_eq!(lines_with_code.len(), 11);
    }

    #[test]
    fn test_bom_removal() {
        let file_string = std::fs::read_to_string("assets/coberta_coverage/c_sharp.xml").unwrap();
        let coverage = Coverage::new(&file_string);

        let lines_with_code =
            coverage.get_lines_with_code("Controllers/WeatherForecastController.cs");
        assert_eq!(lines_with_code.len(), 17);
    }

    #[test]
    fn test_get_file_extensions() {
        let file_string =
            std::fs::read_to_string("assets/coberta_coverage/test_1_coverage.xml").unwrap();
        let coverage = Coverage::new(&file_string);

        let file_extensions = coverage.get_file_extensions();
        println!("{:?}", file_extensions);
        assert_eq!(file_extensions.len(), 1);
        assert_eq!(file_extensions[0], ".py");
    }
}
