//! Common types for item callbacks

use crate::dex_data::ID;

/// Result from an item handler
#[derive(Debug, Clone)]
pub enum ItemHandlerResult {
    /// No return value (undefined in JS)
    Undefined,
    /// Return false
    False,
    /// Return true
    True,
    /// Return null (blocks action)
    Null,
    /// Return 0
    Zero,
    /// Return a number
    Number(i32),
    /// Chain modifier (numerator, denominator)
    ChainModify(u32, u32),
}

/// Item definition placeholder
/// TODO: Import actual ItemDef from items module when available
#[derive(Debug, Clone, Default)]
pub struct ItemDef {
    pub id: ID,
    pub name: String,
}
