use patch::Patch;

use crate::coberta_xml_parser::Coverage;

pub struct DiffFiles {
    pub files: Vec<(String, Vec<usize>)>,
}

impl DiffFiles {
    pub fn new(diff_file_string: &str) -> DiffFiles {
        // Parse the diff file and return file paths and line numbers changed
        let patches = match Patch::from_multiple(diff_file_string) {
            Ok(p) => p,
            Err(e) => panic!("Error parsing diff file: {e}"),
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

            files_changed.files.push((file_path, lines_changed));
        }

        files_changed
    }

    pub fn calculate_line_coverage(&self, coverage: Coverage) -> f32 {
        let mut total_lines_changed = 0;
        let mut total_lines_covered = 0;
        for file in self.files.clone() {
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
                if lines_covered.contains(line) {
                    lines_covered_count += 1;
                }
            }

            total_lines_changed += lines_changed.len();
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

        let diff_files = DiffFiles::new(&diff_file_string);

        assert_eq!(diff_files.files.len(), 3);

        let coverage_string = std::fs::read_to_string("assets/coberta_coverage/coverage.xml")
            .expect("Unable to read file");
        let coverage = Coverage::new(&coverage_string);
        let line_coverage = diff_files.calculate_line_coverage(coverage);

        // 3/12 lines covered 25% coverage
        assert_eq!(line_coverage, 25.0);
    }
}
