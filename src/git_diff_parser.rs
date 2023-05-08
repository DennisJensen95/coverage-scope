use patch::Patch;

use crate::coberta_xml_parser::Coverage;

pub struct DiffFiles {
    pub files: Vec<(String, Vec<usize>)>,
    file_extensions: Vec<String>,
}

impl DiffFiles {
    pub fn new(diff_file_string: &str, file_extensions: Vec<String>) -> DiffFiles {
        // Parse the diff file and return file paths and line numbers changed
        let patches = match Patch::from_multiple(diff_file_string) {
            Ok(p) => p,
            Err(e) => panic!("Error parsing diff file: {e}"),
        };

        let mut files_changed = DiffFiles {
            files: Vec::new(),
            file_extensions: file_extensions.clone(),
        };

        for patch in patches {
            let file_path = patch.new.path;

            // Only code files
            if !file_extensions.iter().any(|ext| file_path.ends_with(ext)) {
                continue;
            }

            let mut lines_changed: Vec<usize> = Vec::new();

            for hunk in patch.hunks {
                let line_range = hunk.new_range;
                for line_number in line_range.start..line_range.start + line_range.count {
                    lines_changed.push(line_number as usize);
                }
            }

            // Remove b/ from the start of the file path
            let file_path = file_path[2..].to_string();

            files_changed.files.push((file_path, lines_changed));
        }

        files_changed
    }

    pub fn calculate_line_coverage(&self, coverage: Coverage) -> f32 {
        let mut total_lines_changed = 0;
        let mut total_lines_covered = 0;
        for file in self.files.clone() {
            let file_path = file.0;

            // Only code files
            if !self
                .file_extensions
                .iter()
                .any(|ext| file_path.ends_with(ext))
            {
                continue;
            }

            let lines_changed = file.1;
            let lines_covered = coverage.get_lines_covered(file_path.as_str());
            let lines_with_code = coverage.get_lines_with_code(file_path.as_str());

            // Count the lines covered out the lines changed
            let mut lines_covered_count = 0;
            let mut total_lines_changed_with_code = 0;
            for line in &lines_changed {
                if lines_covered.contains(line) {
                    lines_covered_count += 1;
                }

                if lines_with_code.contains(line) {
                    total_lines_changed_with_code += 1;
                }
            }

            total_lines_changed += total_lines_changed_with_code;
            total_lines_covered += lines_covered_count;
        }

        total_lines_covered as f32 / total_lines_changed as f32 * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_lines_covered() {
        let diff_file_string = std::fs::read_to_string("assets/diff_files/coverage.diff")
            .expect("Unable to read file");

        let diff_files = DiffFiles::new(&diff_file_string, vec![".py".to_string()]);

        assert_eq!(diff_files.files.len(), 2);

        let coverage_string = std::fs::read_to_string("assets/coberta_coverage/coverage.xml")
            .expect("Unable to read file");
        let coverage = Coverage::new(&coverage_string);
        let line_coverage = diff_files.calculate_line_coverage(coverage);

        // We only changed 4 lines of code and half of them are covered
        assert_eq!(line_coverage, 50.0);
    }

    #[test]
    fn test_parse_tricky_diff() {
        let diff_file_string =
            std::fs::read_to_string("assets/diff_files/tricky.diff").expect("Unable to read file");

        let diff_files = DiffFiles::new(&diff_file_string, vec![".rs".to_string()]);

        assert_eq!(diff_files.files.len(), 1);

        let mut total_lines_changed = 0;
        for file in diff_files.files {
            total_lines_changed += file.1.len();
        }

        assert_eq!(total_lines_changed, 215);
    }
}
