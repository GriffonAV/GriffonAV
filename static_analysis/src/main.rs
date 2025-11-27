use static_analysis::{load_yara_rules, scan_file};
use walkdir::WalkDir;
use std::time::Instant;

fn main() {
    println!("Loading rules...");
    let load_start = Instant::now();
    let rules = load_yara_rules("rules");
    println!("Rules loaded in {:.2?}", load_start.elapsed());

    println!("Scanning samples...");
    let scan_start = Instant::now();
    let mut total_hits = 0;
    let mut files_scanned = 0;

    for entry in WalkDir::new("samples").into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() {
            // Optional: Skip hidden files or huge files if needed
            let hits = scan_file(&rules, entry.path());
            if hits > 0 {
                println!("[ALERT] {:?} matched {} rules", entry.path(), hits);
                total_hits += hits;
            }
            files_scanned += 1;
        }
    }

    println!("Finished. Scanned {} files. Total matches: {} in {:.2?}", 
        files_scanned, total_hits, scan_start.elapsed());
}