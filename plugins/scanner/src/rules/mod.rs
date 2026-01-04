// ! important todo : split rules when multiple rules in one file, assign to multiple buckets

use std::collections::HashMap;

use crate::file_context::{FileType, ScanStage};
use yara_x::Rules;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct RuleKey {
    pub file_type: FileType,
    pub stage: ScanStage,
}

#[derive(Debug)]
pub struct RuleIndex {
    pub rules: HashMap<RuleKey, Rules>,
}
