use crate::file_context::{FileType, ScanStage};
use crate::rules_engine::Engine;
use std::sync::{Arc, Mutex};

pub struct MultiThreadScanner {
    engine: Arc<Engine>,
    num_threads: usize,
}
