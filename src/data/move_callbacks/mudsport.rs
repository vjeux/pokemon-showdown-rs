//! Mud Sport Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

pub mod condition {
    use super::*;

    /// onFieldStart(field, source) {
    ///     this.add('-fieldstart', 'move: Mud Sport', `[of] ${source}`);
    /// }
    pub fn on_field_start(
        battle: &mut Battle,
        _field_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // this.add('-fieldstart', 'move: Mud Sport', `[of] ${source}`);
        if let Some(source) = source_pos {
            let source_arg = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                source_pokemon.get_slot()
            };

            battle.add(
                "-fieldstart",
                &[
                    "move: Mud Sport".into(),
                    format!("[of] {}", source_arg).into(),
                ],
            );
        } else {
            battle.add("-fieldstart", &["move: Mud Sport".into()]);
        }

        EventResult::Continue
    }

    /// onBasePower(basePower, attacker, defender, move) {
    ///     if (move.type === 'Electric') {
    ///         this.debug('mud sport weaken');
    ///         return this.chainModify([1352, 4096]);
    ///     }
    /// }
    pub fn on_base_power(
        battle: &mut Battle,
        _base_power: i32,
        _pokemon_pos: (usize, usize),
        _target_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // Get the move type from the ACTIVE move (not the base move data)
        // Hidden Power and other moves may have variable types that are set on the active move
        let move_type = battle.active_move.as_ref()
            .map(|m| m.borrow().move_type.clone());

        debug_elog!("[MUDSPORT] on_base_power: move_type={:?}", move_type);

        // if (move.type === 'Electric') {
        if move_type.as_deref() == Some("Electric") {
            // this.debug('mud sport weaken');
            // (debug is typically not needed in Rust implementation)

            debug_elog!("[MUDSPORT] Electric type detected, applying chain_modify_fraction(1352, 4096)");
            let _modifier_before = battle.event.as_ref().map(|e| e.modifier).unwrap_or(0);
            // return this.chainModify([1352, 4096]);
            battle.chain_modify_fraction(1352, 4096);
            let _modifier_after = battle.event.as_ref().map(|e| e.modifier).unwrap_or(0);
            debug_elog!("[MUDSPORT] modifier: {} -> {}", _modifier_before, _modifier_after);
            return EventResult::Continue;
        }

        EventResult::Continue
    }

    /// onFieldEnd() {
    ///     this.add('-fieldend', 'move: Mud Sport');
    /// }
    pub fn on_field_end(battle: &mut Battle) -> EventResult {
        // this.add('-fieldend', 'move: Mud Sport');
        battle.add("-fieldend", &["move: Mud Sport".into()]);

        EventResult::Continue
    }
}
