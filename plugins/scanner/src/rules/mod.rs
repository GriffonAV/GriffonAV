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

impl RuleIndex {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    pub fn select_rules(&self, ft: FileType, stage: ScanStage) -> Option<&Rules> {
        let key = RuleKey {
            file_type: ft,
            stage: stage,
        };
        if let Some(r) = self.rules.get(&key) {
            return Some(r);
        }
        let generic_key = RuleKey {
            file_type: FileType::GenericBinary,
            stage,
        };
        self.rules.get(&generic_key)
    }
}

pub fn load_rule_index<P: AsRef<Path>>(dir: P) -> Result<RuleIndex> {
    let mut buckets: HashMap<RuleKey, Vec<String>> = HashMap::new();

    // ! important todo : split rules when multiple rules in one file, assign to multiple buckets
    let re_file_type = Regex::new(r#"file_type\s*=\s*\"([^\"]+)\""#).unwrap();
    let re_stage = Regex::new(r#"stage\s*=\s*\"([^\"]+)\""#).unwrap();

    let mut index = RuleIndex::new();

    Ok(index)
}
