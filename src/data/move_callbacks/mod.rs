//! Move Callback Handlers
//!
//! This module provides dispatch functions for move callbacks.
//! Individual move implementations will be added as needed.

use crate::battle::Battle;
use crate::event::EventResult;

// =========================================================================
// DISPATCH FUNCTIONS
//
// These functions route move events to specific move implementations.
// They return EventResult directly, with EventResult::Continue for no match.
// =========================================================================

/// Dispatch onBasePower callbacks (read-only)
pub fn dispatch_on_base_power(
    _battle: &Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onModifyType callbacks (read-only)
pub fn dispatch_on_modify_type(
    _battle: &Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onModifyMove callbacks (mutates battle)
pub fn dispatch_on_modify_move(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onTryHit callbacks (read-only)
pub fn dispatch_on_try_hit(
    _battle: &Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
    _target_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onTryMove callbacks (read-only)
pub fn dispatch_on_try_move(
    _battle: &Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onPrepareHit callbacks (mutates battle)
pub fn dispatch_on_prepare_hit(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
    _target_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onHit callbacks (mutates battle)
pub fn dispatch_on_hit(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
    _target_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onAfterHit callbacks (mutates battle)
pub fn dispatch_on_after_hit(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
    _target_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onAfterMove callbacks (mutates battle)
pub fn dispatch_on_after_move(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onDamage callbacks (read-only)
pub fn dispatch_on_damage(
    _battle: &Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
    _target_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onModifyDamage callbacks (read-only)
pub fn dispatch_on_modify_damage(
    _battle: &Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
    _target_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}
