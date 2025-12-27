//! Common types for move callbacks

use crate::dex_data::ID;

/// Status object
#[derive(Debug, Clone, Default)]
pub struct Status {
    pub id: String,
}

/// Effect object
#[derive(Debug, Clone, Default)]
pub struct Effect {
    pub id: String,
    pub effect_type: String,
    pub status: Option<String>,
}
