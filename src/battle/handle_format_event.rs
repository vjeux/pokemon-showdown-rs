// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::dex_data::ID;
use crate::event::{EventResult, EventTarget};

impl Battle {

    /// Handle format/rule events
    /// Rust helper method - JavaScript's singleEvent() directly invokes format callbacks
    /// This method dispatches to rule-specific handlers based on effect_id
    pub fn handle_format_event(
        &mut self,
        event_id: &str,
        effect_id: &str,
        target: Option<&EventTarget>,
        source: Option<(usize, usize)>,
    ) -> EventResult {
        // Get target pokemon position from EventTarget
        let target_pos = target.and_then(|t| t.as_pokemon());

        debug_elog!("[FORMAT_EVENT] event='{}', effect='{}', target={:?}, source={:?}",
            event_id, effect_id, target_pos, source);

        match effect_id {
            "sleepclausemod" => {
                match event_id {
                    "SetStatus" => self.sleep_clause_mod_on_set_status(target_pos, source),
                    _ => EventResult::Continue,
                }
            }
            _ => EventResult::Continue,
        }
    }

    /// Sleep Clause Mod onSetStatus
    /// JavaScript source (rulesets.ts):
    /// ```js
    /// onSetStatus(status, target, source) {
    ///     if (source?.isAlly(target)) {
    ///         return;
    ///     }
    ///     if (status.id === 'slp') {
    ///         for (const pokemon of target.side.pokemon) {
    ///             if (pokemon.hp && pokemon.status === 'slp') {
    ///                 if (!pokemon.statusState.source ||
    ///                     !pokemon.statusState.source.isAlly(pokemon)) {
    ///                     this.add('-message', 'Sleep Clause Mod activated.');
    ///                     return false;
    ///                 }
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    fn sleep_clause_mod_on_set_status(
        &mut self,
        target_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        debug_elog!("[SLEEP_CLAUSE_MOD] onSetStatus called, target={:?}, source={:?}", target_pos, source_pos);

        // Get target position
        let (target_side, _target_idx) = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // JavaScript: if (source?.isAlly(target)) { return; }
        // If source is ally of target, allow the sleep (return Continue = don't block)
        if let Some((source_side, _)) = source_pos {
            if source_side == target_side {
                debug_elog!("[SLEEP_CLAUSE_MOD] Source is ally of target, allowing status");
                return EventResult::Continue;
            }
        }

        // Get the status being set from relay_var
        let status_id = if let Some(ref event) = self.event {
            match &event.relay_var {
                Some(EventResult::String(s)) => ID::from(s.as_str()),
                _ => {
                    // Also try effect
                    event.effect.as_ref().map(|e| e.id.clone()).unwrap_or_else(|| ID::from(""))
                }
            }
        } else {
            return EventResult::Continue;
        };

        debug_elog!("[SLEEP_CLAUSE_MOD] Status being set: '{}'", status_id.as_str());

        // JavaScript: if (status.id === 'slp')
        if status_id.as_str() != "slp" {
            return EventResult::Continue;
        }

        // JavaScript: for (const pokemon of target.side.pokemon)
        // Check all Pokemon on target's side
        // Access sides directly using index
        let side_pokemon_len = self.sides.get(target_side)
            .map(|s| s.pokemon.len())
            .unwrap_or(0);

        for i in 0..side_pokemon_len {
            // Get pokemon data immutably
            let (hp, status, status_source) = {
                let side = match self.sides.get(target_side) {
                    Some(s) => s,
                    None => continue,
                };
                let pokemon = match side.pokemon.get(i) {
                    Some(p) => p,
                    None => continue,
                };
                (pokemon.hp, pokemon.status.clone(), pokemon.status_state.source.clone())
            };

            // Skip the target itself - we only check OTHER pokemon
            // (The target doesn't have slp yet, so this check would never match anyway)

            // JavaScript: if (pokemon.hp && pokemon.status === 'slp')
            if hp > 0 && status.as_str() == "slp" {
                debug_elog!("[SLEEP_CLAUSE_MOD] Found sleeping Pokemon at index {}, checking source", i);

                // JavaScript: if (!pokemon.statusState.source || !pokemon.statusState.source.isAlly(pokemon))
                // The existing sleeping Pokemon's source must NOT be an ally for sleep clause to trigger
                // This allows Rest to bypass Sleep Clause (since Rest puts yourself to sleep, source is ally)
                let source_is_ally = match &status_source {
                    Some(src_pos) => src_pos.0 == target_side, // Source is on same side as the sleeping Pokemon
                    None => false, // No source recorded
                };

                if !source_is_ally {
                    // Sleep Clause Mod activated!
                    debug_elog!("[SLEEP_CLAUSE_MOD] Activated! Blocking sleep on target");
                    self.add("-message", &[crate::battle::Arg::Str("Sleep Clause Mod activated.")]);
                    return EventResult::Boolean(false);
                }
            }
        }

        // No blocking condition found, allow the status
        EventResult::Continue
    }
}
