//! The battle queue struct

use serde::{Deserialize, Serialize};

use super::Action;

/// The battle queue - manages the order of actions in a turn
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// JavaScript equivalent: BattleQueue (sim/battle-queue.ts)
/// 32 fields in JavaScript
pub struct BattleQueue {
    /// List of actions
    pub list: Vec<Action>,
}

impl BattleQueue {
}
