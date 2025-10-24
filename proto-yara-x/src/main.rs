use std::fs;
use yara_x;

fn get_rule(folder_path: &str) -> String {
    //get file list
    let paths = std::fs::read_dir(folder_path).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() {
            // if .yar or .yara extension
            if let Some(ext) = path.extension() {
                if ext != "yar" && ext != "yara" {
                    continue;
                }
            }
            let contents =
                fs::read_to_string(path).expect("Should have been able to read the file");
            return contents;
        }
    }
    panic!("No rule file found in the specified folder");
}

fn get_file(folder_path: &str) -> String {
    //get file list
    let paths = std::fs::read_dir(folder_path).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() {
            // if .yar or .yara extension

            let contents =
                fs::read_to_string(path).expect("Should have been able to read the file");
            return contents;
        }
    }
    panic!("No rule file found in the specified folder");
}

fn main() {
    println!("Creating YARA compiler...");
    let mut compiler = yara_x::Compiler::new();

    let contents = get_rule("rules");

    println!("Adding YARA rule source...");
    compiler.add_source(contents.as_str()).unwrap();

    println!("Compiling rules...");
    let rules = compiler.build();

    println!("Creating scanner...");
    let mut scanner = yara_x::Scanner::new(&rules);

    let file_contents = get_file("dummy-files");
    let results = scanner.scan(file_contents.as_bytes()).unwrap();

    println!(
        "Number of matching rules: {}",
        results.matching_rules().len()
    );
    for rule in results.matching_rules() {
        println!("Matched rule: {}", rule.identifier());
    }

    assert_eq!(results.matching_rules().len(), 1);
    println!("Assertion passed: exactly one rule matched.");
}
