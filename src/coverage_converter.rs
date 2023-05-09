use chrono::Local;
use lcov2cobertura::{coverage_to_string, parse_file, NullDemangler};

// Enum of supported coverage formats
#[derive(Debug, PartialEq)]
enum CoverageFormat {
    LcovCoverage,
    CoberturaCoverage,
}

#[derive(Debug)]
pub struct CoverageConverter {
    coverage_file: String,
    original_coverage_format: CoverageFormat,
}

impl CoverageFormat {
    pub fn new(coverage_file: &str) -> CoverageFormat {
        if coverage_file.ends_with(".lcov") || coverage_file.ends_with(".info") {
            CoverageFormat::LcovCoverage
        } else if coverage_file.ends_with(".xml") {
            CoverageFormat::CoberturaCoverage
        } else {
            panic!("Unsupported coverage format")
        }
    }
}

impl CoverageConverter {
    pub fn new(coverage_file_path: &str) -> Self {
        let coverage_format = CoverageFormat::new(coverage_file_path);

        Self {
            coverage_file: coverage_file_path.into(),
            original_coverage_format: coverage_format,
        }
    }

    fn covert_lcov_to_cobertura(&self) -> String {
        // No excludes
        let excludes = vec![];
        let base_dir = String::from(".");
        let coverage_data = parse_file(&self.coverage_file, &base_dir, &excludes)
            .expect("Failed to parse coverage file to LCOV Coverage Data");

        // Get current time stamp
        let current_time = Local::now().timestamp();
        let default_demangler = NullDemangler::new();

        coverage_to_string(&coverage_data, current_time as u64, default_demangler)
            .expect("Failed to convert coverage data to LCOV")
    }

    pub fn convert_to_cobertura(&self) -> String {
        match self.original_coverage_format {
            CoverageFormat::LcovCoverage => self.covert_lcov_to_cobertura(),
            CoverageFormat::CoberturaCoverage => {
                std::fs::read_to_string(&self.coverage_file).expect("Failed to read coverage file")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_coverage_format() {
        let coverage_converter = CoverageFormat::new("coverage.lcov");
        assert_eq!(coverage_converter, CoverageFormat::LcovCoverage);
    }

    #[test]
    #[should_panic]
    fn test_detect_coverage_format_invalid() {
        CoverageConverter::new("coverage.invalid");
    }

    #[test]
    fn test_converting_lcov_file() {
        // Converting lcov file to cobertura
        let coverage_converter = CoverageConverter::new("assets/lcov_files/lcov_file_1.info");
        coverage_converter.convert_to_cobertura();

        // Converting cobertura file to cobertura
        let cobertura_converter = CoverageConverter::new("assets/coberta_coverage/coverage.xml");
        cobertura_converter.convert_to_cobertura();
    }
}
