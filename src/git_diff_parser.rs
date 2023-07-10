use crate::coberta_xml_parser::Coverage;
use crate::git_patch_parser::parse_patch_string;

pub struct DiffFiles {
    pub files: Vec<(String, Vec<usize>)>,
    file_extensions: Vec<String>,
}

impl DiffFiles {
    pub fn new(diff_file_string: &str, file_extensions: Vec<String>) -> Self {
        // Parse the diff file and return file paths and line numbers changed
        let patches = parse_patch_string(diff_file_string);

        let mut files_changed = DiffFiles {
            files: Vec::new(),
            file_extensions: file_extensions.clone(),
        };

        for patch in patches {
            let file_path = patch.file_path;

            // Only code files
            if !file_extensions.iter().any(|ext| file_path.ends_with(ext)) {
                continue;
            }

            let mut lines_changed: Vec<usize> = Vec::new();

            for hunk in patch.hunks {
                let line_range = hunk.new;
                for line_number in line_range.start..line_range.start + line_range.count {
                    lines_changed.push(line_number as usize);
                }
            }

            files_changed.files.push((file_path, lines_changed));
        }

        files_changed
    }

    fn count_lines_covered(
        &self,
        lines_changed: Vec<usize>,
        lines_covered: Vec<usize>,
        lines_with_code: Vec<usize>,
    ) -> (usize, usize) {
        let mut lines_covered_count = 0;
        let mut total_lines_changed_with_code = 0;
        for line in &lines_changed {
            if lines_with_code.contains(line) {
                total_lines_changed_with_code += 1;
                if lines_covered.contains(line) {
                    lines_covered_count += 1;
                }
            }
        }

        (lines_covered_count, total_lines_changed_with_code)
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

            let (lines_covered_count, total_lines_changed_with_code) =
                self.count_lines_covered(lines_changed, lines_covered, lines_with_code);

            if total_lines_changed_with_code == 0 {
                continue;
            }

            let percentage_covered = format!(
                "{:.2}",
                lines_covered_count as f32 / total_lines_changed_with_code as f32 * 100.0,
            );

            println!("  File changed: {file_path}");
            println!("  Lines changed: {lines_covered_count}/{total_lines_changed_with_code}: {percentage_covered}‰",);

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

    #[test]
    fn test_count_lines_covered() {
        let diff_files = DiffFiles {
            files: Vec::new(),
            file_extensions: Vec::new(),
        };

        // 1
        // Lines with code covers all lines
        let lines_changed = vec![1, 3, 5];
        let lines_covered = vec![1, 2, 5];
        let lines_with_code = vec![1, 2, 3, 4, 5];

        let result = diff_files.count_lines_covered(lines_changed, lines_covered, lines_with_code);

        assert_eq!(result.0, 2);
        assert_eq!(result.1, 3);

        // 2
        // Lines with code only covers some of the changed lines
        let lines_changed_2 = vec![1, 3, 5];
        let lines_covered_2 = vec![1, 2, 5];
        let lines_with_code_2 = vec![1, 2, 3];

        let (lines_covered_count, total_lines_changed_with_code) =
            diff_files.count_lines_covered(lines_changed_2, lines_covered_2, lines_with_code_2);

        assert_eq!(lines_covered_count, 1);
        assert_eq!(total_lines_changed_with_code, 2);
    }
}
