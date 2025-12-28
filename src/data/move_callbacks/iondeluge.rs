//! Ion Deluge Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// onFieldStart(target, source, sourceEffect) {
    ///     this.add('-fieldactivate', 'move: Ion Deluge');
    ///     this.hint(`Normal-type moves become Electric-type after using ${sourceEffect}.`);
    /// }
    pub fn on_field_start(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>) -> EventResult {
        // this.add('-fieldactivate', 'move: Ion Deluge');
        battle.add("-fieldactivate", &["move: Ion Deluge".into()]);

        // this.hint(`Normal-type moves become Electric-type after using ${sourceEffect}.`);
        let source_effect = battle.current_effect_state.as_ref()
            .and_then(|state| state.source_effect.as_ref().map(|s| s.to_string()))
            .unwrap_or_else(|| "Ion Deluge".to_string());
        battle.hint(&format!("Normal-type moves become Electric-type after using {}.", source_effect));

        EventResult::Continue
    }

    /// onModifyType(move) {
    ///     if (move.type === 'Normal') {
    ///         move.type = 'Electric';
    ///         this.debug(move.name + "'s type changed to Electric");
    ///     }
    /// }
    pub fn on_modify_type(battle: &mut Battle, move_id: &str) -> EventResult {
        use crate::dex_data::ID;

        // if (move.type === 'Normal') {
        //     move.type = 'Electric';
        //     this.debug(move.name + "'s type changed to Electric");
        // }
        let (is_normal, move_name) = {
            if let Some(ref active_move) = battle.active_move {
                (active_move.move_type == ID::from("normal"), active_move.name.clone())
            } else {
                return EventResult::Continue;
            }
        };

        if is_normal {
            if let Some(ref mut active_move) = battle.active_move {
                active_move.move_type = ID::from("electric");
            }
            battle.debug(&format!("{}'s type changed to Electric", move_name));
        }

        EventResult::Continue
    }
}
