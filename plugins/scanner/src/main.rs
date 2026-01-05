use scanner::{load_yara_rules, scan_file};
use std::time::Instant;
use walkdir::WalkDir;

mod file_context;
mod rules_engine;
use file_context::{FileType, ScanStage};
use rules_engine::Engine;

fn main() {
    env_logger::init();

    // let engine = rEngine {
    //     rule_index: std::sync::Arc::new(
    //         rules_engine::load_rule_index("rules").expect("Failed to load rule index"),
    //     ),
    //     config: rules_engine::EngineConfig { rules_dir: None },
    // };
    // if engine.rule_index.rules.is_empty() {
    //     return;
    // }
    let engine = Engine::from_dir("./rules");
    if engine.is_err() {
        eprintln!(
            "Failed to initialize rules engine: {}",
            engine.err().unwrap()
        );
        return;
    }
    let engine = engine.unwrap();
    engine.select_rules(FileType::GenericBinary, ScanStage::Pre);
    return;

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
            if let Err(e) = file_context::get(entry.path()) {
                eprintln!(
                    "Failed to get file context for {}: {}",
                    entry.path().display(),
                    e
                );
            }
            let hits = scan_file(&rules, entry.path());
            if hits > 0 {
                println!("[ALERT] {:?} matched {} rules", entry.path(), hits);
                total_hits += hits;
            }
            files_scanned += 1;
        }
    }

    println!(
        "Finished. Scanned {} files. Total matches: {} in {:.2?}",
        files_scanned,
        total_hits,
        scan_start.elapsed()
    );
}
