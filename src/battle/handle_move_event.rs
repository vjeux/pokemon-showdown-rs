// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::data::move_callbacks;
use crate::event::EventResult;

impl Battle {

    /// Handle move events
    /// Rust helper method - JavaScript's singleEvent() directly invokes move[`on${eventId}`] callbacks
    /// This method dispatches to move_callbacks module based on event name
    ///
    /// JavaScript singleEvent calls callbacks with: [target, source, sourceEffect]
    // TODO: Verify move parameter type matches JavaScript's ActiveMove usage
    pub fn handle_move_event(
        &mut self,
        event_id: &str,
        move_id: &ID,
        target: Option<&crate::event::EventTarget>,
        source_pos: Option<(usize, usize)>,
    ) -> crate::event::EventResult {
        // Extract pokemon position from EventTarget
        let target_pos = target.and_then(|t| t.as_pokemon());

        // Clone active_move to pass to dispatch functions
        let active_move_clone = self.active_move.clone();
        debug_elog!("[HANDLE_MOVE_EVENT] event_id={}, move_id={}, active_move={:?}", event_id, move_id.as_str(), active_move_clone.as_ref().map(|m| m.id.as_str()));

        match event_id {
            "AfterHit" => {
                move_callbacks::dispatch_on_after_hit(self, active_move_clone.as_ref(), target_pos.unwrap_or((0,0)), source_pos.unwrap_or((0,0)))
            }
            "AfterMove" => {
                move_callbacks::dispatch_on_after_move(self, active_move_clone.as_ref(), target_pos.unwrap_or((0,0)), source_pos)
            }
            "AfterMoveSecondarySelf" => move_callbacks::dispatch_on_after_move_secondary_self(
                self, active_move_clone.as_ref(), target_pos.unwrap_or((0,0)), source_pos,
            ),
            "AfterSubDamage" => {
                // Get damage from relay_var
                let damage = self
                    .event
                    .as_ref()
                    .and_then(|e| match &e.relay_var { Some(EventResult::Number(n)) => Some(*n), _ => None })
                    .unwrap_or(0);

                // JavaScript: onAfterSubDamage(damage, target, source, move)
                // - target = the Pokemon with Substitute (receiving damage)
                // - source = the attacker (using the move)
                //
                // dispatch_on_after_sub_damage expects:
                // - pokemon_pos = the source/attacker
                // - target_pos = the target/substitute owner
                //
                // In this context:
                // - source_pos = the attacker (passed to this function)
                // - target_pos = extracted from EventTarget (the target)
                move_callbacks::dispatch_on_after_sub_damage(self, active_move_clone.as_ref(), source_pos.unwrap_or((0,0)), damage, target_pos)
            }
            "BasePower" => {
                // Get base_power from relay_var
                let base_power = self
                    .event
                    .as_ref()
                    .and_then(|e| match &e.relay_var { Some(EventResult::Number(n)) => Some(*n), _ => None })
                    .unwrap_or(0);

                // BasePower event is for abilities/items to modify base power
                // Note: basePowerCallback is handled in getDamage, not here
                //
                // JavaScript: onBasePower(basePower, source, target, move)
                // run_event is called from get_damage.rs as:
                //   run_event("BasePower", EventTarget::Pokemon(source_pos), Some(target_pos), ...)
                // So:
                //   target (EventTarget) contains the SOURCE (attacker)
                //   source_pos contains the TARGET (defender) - naming is confusing!
                //
                // dispatch_on_base_power signature is (target_pos, source_pos) but
                // the callbacks were written expecting the SECOND param to be target.
                // So we pass (source_pos, target_pos) to match the callback expectations:
                //   - First param (named target_pos in dispatch but unused by callbacks)
                //   - Second param (named source_pos in dispatch) = actual target (defender)
                let result = move_callbacks::dispatch_on_base_power(self, active_move_clone.as_ref(), base_power, target_pos.unwrap_or((0,0)), source_pos);
                result
            }
            "Damage" => {
                // Extract all parameters immutably first
                let damage = self
                    .event
                    .as_ref()
                    .and_then(|e| match &e.relay_var { Some(EventResult::Number(n)) => Some(*n), _ => None })
                    .unwrap_or(0);

                let event_effect = self.event.as_ref().and_then(|e| e.effect.clone());

                move_callbacks::dispatch_on_damage(
                    self,
                    active_move_clone.as_ref(),
                    damage,
                    target_pos.unwrap_or((0,0)),
                    source_pos,
                    event_effect.as_ref(),
                )
            }
            "DisableMove" => move_callbacks::dispatch_on_disable_move(self, move_id.as_str(), target_pos.unwrap_or((0,0))),
            "Effectiveness" => {
                // Extract type_mod from relay_var and target_type from type_param
                let (type_mod, target_type) = {
                    let type_mod = self
                        .event
                        .as_ref()
                        .and_then(|e| match &e.relay_var { Some(EventResult::Number(n)) => Some(*n), _ => None })
                        .unwrap_or(0);

                    let target_type = self
                        .event
                        .as_ref()
                        .and_then(|e| e.type_param.clone())
                        .unwrap_or_default();

                    (type_mod, target_type)
                };

                move_callbacks::dispatch_on_effectiveness(self, active_move_clone.as_ref(), type_mod, &target_type, target_pos.unwrap_or((0,0)))
            }
            "Hit" => {
                // Check effect_type to distinguish between regular onHit and self.onHit
                // JavaScript: moveData.onHit vs moveData.self.onHit
                let effect_type = self.effect.as_ref().map(|e| e.effect_type);
                if effect_type == Some(crate::battle::EffectType::MoveSelf) {
                    // Self-targeting onHit callback (JavaScript: moveData.self.onHit)
                    // source_effect (the move) is available via self.event.effect
                    // Clone to avoid borrow issues
                    let source_effect = self.event.as_ref().and_then(|e| e.effect.clone());
                    move_callbacks::dispatch_self_on_hit(self, active_move_clone.as_ref(), target_pos.unwrap_or((0,0)), source_pos, source_effect.as_ref())
                } else {
                    // Regular onHit targets the move target (JavaScript: moveData.onHit)
                    move_callbacks::dispatch_on_hit(self, active_move_clone.as_ref(), target_pos.unwrap_or((0,0)), source_pos)
                }
            }
            "HitField" => move_callbacks::dispatch_on_hit_field(self, active_move_clone.as_ref(), target_pos.unwrap_or((0,0)), source_pos),
            "HitSide" => move_callbacks::dispatch_on_hit_side(self, active_move_clone.as_ref(), target_pos.unwrap_or((0,0))),
            "ModifyMove" => {
                move_callbacks::dispatch_on_modify_move(self, target_pos.unwrap_or((0,0)), source_pos)
            }
            "ModifyPriority" => {
                // Get priority from relay_var
                let priority = self
                    .event
                    .as_ref()
                    .and_then(|e| match &e.relay_var { Some(EventResult::Number(n)) => Some(*n), _ => None })
                    .unwrap_or(0);
                // IMPORTANT: Use move_id parameter to get the move, NOT active_move_clone
                // This is called from get_action_speed before self.active_move is set
                let move_for_callback = self.dex.get_active_move(move_id.as_str());
                move_callbacks::dispatch_on_modify_priority(self, move_for_callback.as_ref(), target_pos.unwrap_or((0,0)), priority)
            }
            "ModifyTarget" => move_callbacks::dispatch_on_modify_target(self, active_move_clone.as_ref(), target_pos.unwrap_or((0,0))),
            "ModifyType" => move_callbacks::dispatch_on_modify_type(self, active_move_clone.as_ref(), target_pos.unwrap_or((0,0)), source_pos),
            "MoveFail" => {
                // CRITICAL: Use move_id to look up the move, NOT self.active_move!
                // When nested moves are called (e.g., Mirror Move calling Jump Kick),
                // self.active_move is set to the inner move (Jump Kick).
                // But when MoveFail is called for the outer move (Mirror Move),
                // we need to call Mirror Move's onMoveFail, not Jump Kick's.
                // JavaScript passes the move directly to singleEvent, so we must use move_id.
                let move_for_callback = self.dex.get_active_move(move_id.as_str());
                move_callbacks::dispatch_on_move_fail(self, move_for_callback.as_ref(), target_pos, source_pos)
            }

            "PrepareHit" => {
                move_callbacks::dispatch_on_prepare_hit(self, active_move_clone.as_ref(), target_pos.unwrap_or((0,0)), source_pos)
            }
            "Try" => move_callbacks::dispatch_on_try(self, active_move_clone.as_ref(), target_pos.unwrap_or((0,0)), source_pos),
            "TryHit" => {
                move_callbacks::dispatch_on_try_hit(self, active_move_clone.as_ref(), target_pos.unwrap_or((0,0)), source_pos.unwrap_or((0,0)))
            }
            "TryImmunity" => {
                if let Some(tgt) = target_pos {
                    move_callbacks::dispatch_on_try_immunity(self, active_move_clone.as_ref(), tgt, source_pos)
                } else {
                    EventResult::Continue
                }
            }
            "TryMove" => move_callbacks::dispatch_on_try_move(self, active_move_clone.as_ref(), target_pos.unwrap_or((0,0)), source_pos),
            "UseMoveMessage" => {
                move_callbacks::dispatch_on_use_move_message(self, active_move_clone.as_ref(), target_pos.unwrap_or((0,0)))
            }
            _ => EventResult::Continue,
        }
    }
}
