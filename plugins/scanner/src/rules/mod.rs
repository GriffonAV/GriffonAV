use std::collections::HashMap;

use file_context::{FileType, ScanStage};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct RuleKey {
    pub file_type: FileType,
    pub stage: ScanStage,
}

#[derive(Debug)]
pub struct RuleIndex {
    pub rules: HashMap<RuleKey, Rules>,
}
