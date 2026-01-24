// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use std::collections::VecDeque;
use crate::battle_queue::BattleQueue;

impl BattleQueue {
    /// Create a new empty battle queue
    pub fn new() -> Self {
        Self { list: VecDeque::new() }
    }
}
