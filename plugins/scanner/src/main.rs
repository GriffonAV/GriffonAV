use anyhow::Result;
use scanner::{
    load_yara_rules, scan_file,
    scanner_engine::{self, MultiThreadScanner},
};
use std::time::Instant;
use walkdir::WalkDir;

use scanner::file_context::{FileType, ScanStage};
use scanner::rules_engine::RulesEngine;
use scanner::scanner_engine::ScanReport;

use std::sync::Arc;

fn main() {
    env_logger::init();

    eprintln!("Loading rules...");
    let load_start = Instant::now();

    let mut engine = RulesEngine::from_dir("./rules").unwrap();
    engine.select_rules(FileType::GenericBinary, ScanStage::Pre);
    let engine: Arc<RulesEngine> = Arc::new(engine);

    let scanner_engine: MultiThreadScanner = MultiThreadScanner::new(engine.clone()).unwrap();

    // let rules = load_yara_rules("rules");
    eprintln!("Rules loaded in {:.2?}", load_start.elapsed());

    eprintln!("Scanning samples...");
    // let mut total_hits = 0;
    // let mut files_scanned = 0;

    // for entry in WalkDir::new("samples").into_iter().filter_map(|e| e.ok()) {
    //     if entry.path().is_file() {
    //         // Optional: Skip hidden files or huge files if needed
    //         if let Err(e) = file_context::get(entry.path()) {
    //             eprintln!(
    //                 "Failed to get file context for {}: {}",
    //                 entry.path().display(),
    //                 e
    //             );
    //         }
    //         let hits = scan_file(&rules, entry.path());
    //         if hits > 0 {
    //             println!("[ALERT] {:?} matched {} rules", entry.path(), hits);
    //             total_hits += hits;
    //         }
    //         files_scanned += 1;
    //     }
    // }
    let path: &str = "samples";
    let result: Result<ScanReport> = scanner_engine.scan_directory(path);
    match result {
        Ok(report) => {
            eprintln!("Scan completed: \n{}", report);
        }
        Err(e) => {
            eprintln!("Error during scan: {}", e);
        }
    }
}
