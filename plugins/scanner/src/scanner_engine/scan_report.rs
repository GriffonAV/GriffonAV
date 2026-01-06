use super::scan_match::ScanMatch;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum ScanResult {
    Clean {
        file_path: String,
        file_type: String,
        scan_duration_ms: u64,
    },
    Infected {
        file_path: String,
        file_type: String,
        matches: Vec<ScanMatch>,
        scan_duration_ms: u64,
        threat_level: u32,
    },
    Error {
        file_path: String,
        error: String,
        scan_duration_ms: u64,
    },
    Skipped {
        file_path: String,
        reason: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanReport {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub scan_path: String,
    pub total_files: usize,
    pub infected_files: usize,
    pub clean_files: usize,
    pub errors: usize,
    pub skipped_files: usize,
    pub scan_duration_secs: f64,
    pub results: Vec<ScanResult>,
}
