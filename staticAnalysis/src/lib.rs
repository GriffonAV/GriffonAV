use std::fs;
use std::path::Path;
use yara_x::{Compiler, Scanner};

pub fn load_yara_rules(dir: &str) -> yara_x::Rules {
    let mut compiler = Compiler::new();

    for entry in std::fs::read_dir(dir).unwrap() {
        let p = entry.unwrap().path();
        if p.is_file() {
            if let Some(ext) = p.extension() {
                if ext != "yar" && ext != "yara" { continue; }
            }
            let contents = fs::read_to_string(&p).unwrap();
            compiler.add_source(contents.as_str()).unwrap();
            break;
        }
    }

    compiler.build()
}

pub fn scan_file(rules: &yara_x::Rules, path: &str) -> usize {
    let input = fs::read(path).expect("Unable to read sample");
    let mut scanner = Scanner::new(rules);
    let results = scanner.scan(&input).unwrap();
    results.matching_rules().len()
}
