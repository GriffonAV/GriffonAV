use rustav::{load_yara_rules, scan_file};

fn main() {
    let rules = load_yara_rules("rules");
    let hits = scan_file(&rules, "samples/sample_001.txt");
    println!("Matched {} rules", hits);
}
