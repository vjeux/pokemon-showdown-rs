// All conditions are now loaded from data/conditions.json
// This file is kept for compatibility with existing code that references condition types
//
// Condition data comes from:
// 1. data/conditions.json - base condition data (name, effectType, duration, etc.)
// 2. src/data/condition_callbacks/ - custom event handlers per condition
//
// Access conditions via Battle.dex.conditions.get(id)

// Re-export condition types from dex
pub use crate::dex::{ConditionData, ConditionType};
