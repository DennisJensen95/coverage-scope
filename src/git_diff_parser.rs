use patch::Patch;

pub struct DiffFiles {
    pub files: Vec<(String, Vec<usize>)>,
}

impl DiffFiles {
    pub fn new(diff_file_string: &str) -> DiffFiles {
        // Parse the diff file and return file paths and line numbers changed
        let patches = match Patch::from_multiple(&diff_file_string) {
            Ok(p) => p,
            Err(e) => panic!("Error parsing diff file: {}", e),
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

            files_changed.files.push((file_path.into(), lines_changed));
        }

        files_changed
    }
}
