use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "coverage")]
struct Coverage {
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
    class: Class,
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

fn main() {
    // Read xml file
    let file_string = std::fs::read_to_string("test_repo/coverage.xml").unwrap();

    // Parse xml file
    let coverage: Coverage = serde_xml_rs::from_str(&file_string).unwrap();

    println!("coverage: {:#?}", coverage.version);
    println!("coverage: {:#?}", coverage.timestamp);

    for package in coverage.packages.list_of_packages {
        println!("package: {:#?}", package.name);
        println!("package: {:#?}", package.classes.class);
    }
}
