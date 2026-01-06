use crate::rules_engine::Engine;
use anyhow::Result;
use std::path::Path;

use super::scan_report::ScanReport;
use std::sync::Arc;

pub struct MultiThreadScanner {
    engine: Arc<Engine>,
    num_threads: usize,
}

impl MultiThreadScanner {
    pub fn new(engine: Arc<Engine>) -> Result<Self> {
        Self::with_threads(engine, 0)
    }

    pub fn with_threads(engine: Arc<Engine>, num_threads: usize) -> Result<Self> {
        let num_threads = if num_threads == 0 {
            num_cpus::get()
        } else {
            num_threads
        };

        Ok(Self {
            engine,
            num_threads,
        })
    }

    pub fn scan_directory<P: AsRef<Path>>(&self, path: P) -> Result<ScanReport> {
        todo!("test");
    }
}
