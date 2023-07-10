pub struct Range {
    pub start: u32,
    pub count: u32,
}

pub struct Hunk {
    pub new: Range,
    pub new_lines_count: u32,
}

pub struct Patch {
    pub file_path: String,
    pub hunks: Vec<Hunk>,
    pub changed_lines: Vec<usize>,
}

pub fn parse_patch_string(patch_string: &str) -> Vec<Patch> {
    let lines = patch_string.lines();
    let mut patches: Vec<Patch> = Vec::new();
    let mut current_patch: Option<Patch> = None;
    let mut current_hunk: Option<Hunk> = None;
    let mut current_line_in_hunk: u32 = 0;

    for line in lines {
        if line.starts_with("diff --git") {
            if let Some(mut patch) = current_patch {
                if let Some(hunk) = current_hunk {
                    patch.hunks.push(hunk);
                    current_hunk = None;
                }

                patches.push(patch);
            }

            // diff --git a/.github/workflows/ci_cd.yml b/.github/workflows/ci_cd.yml
            // Split the line and get the last file path
            let file_path = match line.split(' ').last() {
                Some(path) => path.strip_prefix("b/").unwrap().to_string(),
                None => panic!("Error parsing file path from diff line: {line}"),
            };

            current_patch = Some(Patch {
                file_path,
                hunks: Vec::new(),
                changed_lines: Vec::new(),
            });
        } else if line.starts_with("@@") {
            if let Some(patch) = current_patch.as_mut() {
                if let Some(hunk) = current_hunk.take() {
                    patch.hunks.push(hunk);
                }
            }

            current_line_in_hunk = 0;
            let (line_start, hunk_line_count) = parse_hunk_header(line);
            current_hunk = Some(Hunk {
                new: Range {
                    start: line_start,
                    count: hunk_line_count,
                },
                new_lines_count: 0,
            });
        } else if line.starts_with('+') {
            current_line_in_hunk += 1;

            // Skip empty lines
            if line.trim() == "+" {
                continue;
            }

            if let Some(hunk) = current_hunk.as_mut() {
                hunk.new_lines_count += 1;
                if let Some(patch) = current_patch.as_mut() {
                    patch
                        .changed_lines
                        .push((hunk.new.start + current_line_in_hunk - 1) as usize);
                }
            }
        } else if line.starts_with('-') {
            continue;
        } else {
            current_line_in_hunk += 1;
        }
    }

    if let Some(mut patch) = current_patch {
        if let Some(hunk) = current_hunk {
            patch.hunks.push(hunk);
        }

        patches.push(patch);
    }

    patches
}

fn parse_hunk_header(hunk_header: &str) -> (u32, u32) {
    // @@ -6,6 +6,7 @@ something

    // Get string inside of @@ -6,6 +6,7 @@
    match hunk_header.split("@@").nth(1) {
        Some(line) => {
            if !line.contains('+') {
                return (0, 0);
            }

            let new_file_hunk = line.split(' ').nth(2).unwrap();

            if !new_file_hunk.contains(',') {
                let line_start = new_file_hunk.parse::<u32>().unwrap();
                return (line_start, 0);
            }

            let line_start = new_file_hunk
                .split(',')
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let hunk_line_count = new_file_hunk
                .split(',')
                .nth(1)
                .unwrap()
                .parse::<u32>()
                .unwrap();

            (line_start, hunk_line_count)
        }
        None => panic!("Error parsing line number range from hunk header: {hunk_header}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hunk_header() {
        let hunk_header = "@@ -6,6 +6,7 @@ something";
        let (line_start, line_count) = parse_hunk_header(hunk_header);
        assert_eq!(line_start, 6);
        assert_eq!(line_count, 7);
    }

    #[test]
    fn test_parse_hunk_header_negative_count() {
        let hunk_header = "@@ -6,6 @@ something";
        let (line_start, line_count) = parse_hunk_header(hunk_header);
        assert_eq!(line_start, 0);
        assert_eq!(line_count, 0);
    }

    fn check_parse_diff_file(
        str: &str,
        patches_count: usize,
        hunk_count: usize,
        lines_changed: usize,
    ) {
        let patches = parse_patch_string(str);
        assert_eq!(patches.len(), patches_count);

        // count hunks
        let mut diff_file_hunk_count = 0;
        let mut diff_file_lines_changed = 0;
        for patch in patches {
            diff_file_hunk_count += patch.hunks.len();
            diff_file_lines_changed += patch.changed_lines.len();
        }

        assert_eq!(diff_file_hunk_count, hunk_count);
        assert_eq!(diff_file_lines_changed, lines_changed);
    }

    #[test]
    fn test_parse_normal_patch_string() {
        // Read string from assets file
        let patch_string =
            std::fs::read_to_string("assets/diff_files/test_1.diff").expect("Unable to read file");
        check_parse_diff_file(&patch_string, 3, 5, 29);
    }

    #[test]
    fn test_parse_tricky_diff_patch() {
        let patch_string =
            std::fs::read_to_string("assets/diff_files/tricky.diff").expect("Unable to read file");
        check_parse_diff_file(&patch_string, 9, 23, 577)
    }
}
