//! Priority Item

/// Priority item for sorting actions/handlers
/// TODO: Not in JavaScript - Rust-specific helper for sorting priority queues
/// JavaScript uses inline sorting with anonymous comparator functions
#[derive(Debug, Clone, Default)]
pub struct PriorityItem {
    pub order: Option<i32>,
    pub priority: i32,
    pub fractional_priority: f64,
    pub speed: f64,
    pub sub_order: i32,
    pub effect_order: i32,
    pub index: usize,
}
