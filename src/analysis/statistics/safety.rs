use std::collections::HashMap;
use crate::types::Warning;

#[derive(Debug, Default)]
pub struct SafetyStatistics {
    pub total_issues: usize,
    pub casting_details: CastingStatistics,
    pub unsafe_details: UnsafeStatistics,
    pub thread_safety_details: ThreadSafetyStatistics,
}

#[derive(Debug, Default)]
pub struct CastingStatistics {
    pub total_casts: usize,
    pub by_type: HashMap<String, usize>,
    pub risky_patterns: HashMap<String, usize>,
}

#[derive(Debug, Default)]
pub struct UnsafeStatistics {
    pub total_unsafe: usize,
    pub raw_pointers: usize,
    pub ffi_calls: usize,
    pub mutable_statics: usize,
}

#[derive(Debug, Default)]
pub struct ThreadSafetyStatistics {
    pub total_issues: usize,
    pub send_sync_violations: usize,
    pub data_races: usize,
    pub lock_issues: usize,
}

impl SafetyStatistics {
    pub fn update(&mut self, warning: &Warning) {
        self.total_issues += 1;
        match warning.message.split_whitespace().next().unwrap_or("") {
            "Type" => {
                self.casting_details.total_casts += 1;
                self.casting_details.by_type
                    .entry(warning.message.clone())
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            },
            "Unsafe" => {
                self.unsafe_details.total_unsafe += 1;
                if warning.message.contains("raw pointer") {
                    self.unsafe_details.raw_pointers += 1;
                }
                if warning.message.contains("FFI") {
                    self.unsafe_details.ffi_calls += 1;
                }
                if warning.message.contains("static mut") {
                    self.unsafe_details.mutable_statics += 1;
                }
            },
            "Thread" => {
                self.thread_safety_details.total_issues += 1;
                if warning.message.contains("Send") || warning.message.contains("Sync") {
                    self.thread_safety_details.send_sync_violations += 1;
                }
                if warning.message.contains("data race") {
                    self.thread_safety_details.data_races += 1;
                }
                if warning.message.contains("lock") {
                    self.thread_safety_details.lock_issues += 1;
                }
            },
            _ => {},
        }
    }
} 