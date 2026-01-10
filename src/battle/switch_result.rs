//! Switch Result

/// Result of a switch operation
/// TODO: Not in JavaScript - Rust-specific enum for switch operation results
/// JavaScript switch methods return undefined or false on failure
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchResult {
    /// Switch succeeded
    Success,
    /// Switch failed
    Failed,
    /// Pokemon fainted from Pursuit before switch completed
    PursuitFaint,
}
