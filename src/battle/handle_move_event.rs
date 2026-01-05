// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Battle {

    /// Handle move events
    /// Rust helper method - JavaScript's singleEvent() directly invokes move[`on${eventId}`] callbacks
    /// This method dispatches to move_callbacks module based on event name
    ///
    /// JavaScript singleEvent calls callbacks with: [target, source, sourceEffect]
    /// So the move callback receives:
    ///   - first param (often named "source" in JS): target from singleEvent
    ///   - second param (often named "target" in JS): source from singleEvent
    ///
    /// In Rust, we pass target and source directly from singleEvent to the move callbacks.
    pub fn handle_move_event(
        &mut self,
        event_id: &str,
        move_id: &str,
        target: Option<(usize, usize)>,
        source: Option<(usize, usize)>,
    ) -> crate::event::EventResult {
        use crate::data::move_callbacks;
        use crate::event::EventResult;

        // For most events, callbacks receive (target, source) from singleEvent
        // Some events like BasePower are called via run_event and need special handling
        let source_pos = target.unwrap_or((0, 0));
        let target_pos = source;

        match event_id {
            "AfterHit" => {
                if let Some(tgt) = target_pos {
                    move_callbacks::dispatch_on_after_hit(self, move_id, source_pos, tgt)
                } else {
                    EventResult::Continue
                }
            }
            "AfterMove" => {
                move_callbacks::dispatch_on_after_move(self, move_id, source_pos, target_pos)
            }
            "AfterMoveSecondarySelf" => move_callbacks::dispatch_on_after_move_secondary_self(
                self, move_id, source_pos, target_pos,
            ),
            "AfterSubDamage" => {
                // Get damage from relay_var
                let damage = self
                    .current_event
                    .as_ref()
                    .and_then(|e| e.relay_var)
                    .unwrap_or(0);

                move_callbacks::dispatch_on_after_sub_damage(self, move_id, source_pos, damage, target_pos)
            }
            "BasePower" => {
                // Get base_power from relay_var
                let base_power = self
                    .current_event
                    .as_ref()
                    .and_then(|e| e.relay_var)
                    .unwrap_or(0);

                // BasePower event is for abilities/items to modify base power
                // Note: basePowerCallback is handled in getDamage, not here
                let result = move_callbacks::dispatch_on_base_power(self, move_id, base_power, source_pos, target_pos);
                result
            }
            "Damage" => {
                // Extract all parameters immutably first
                let (damage, effect_id) = {
                    let damage = self
                        .current_event
                        .as_ref()
                        .and_then(|e| e.relay_var)
                        .unwrap_or(0);

                    let effect_id = self
                        .current_event
                        .as_ref()
                        .and_then(|e| e.effect.as_ref())
                        .map(|id| id.to_string());

                    (damage, effect_id)
                };

                if let Some(tgt) = target_pos {
                    move_callbacks::dispatch_on_damage(
                        self,
                        move_id,
                        damage,
                        tgt,
                        Some(source_pos),
                        effect_id.as_deref(),
                    )
                } else {
                    EventResult::Continue
                }
            }
            "DamagePriority" => {
                // No moves implement DamagePriority event
                EventResult::Continue
            }
            "DisableMove" => move_callbacks::dispatch_on_disable_move(self, move_id, source_pos),
            "Effectiveness" => move_callbacks::dispatch_on_effectiveness(self, move_id, 0, "", source_pos),
            "Hit" => {
                if let Some(tgt) = target_pos {
                    move_callbacks::dispatch_on_hit(self, move_id, source_pos, Some(tgt))
                } else {
                    EventResult::Continue
                }
            }
            "HitField" => move_callbacks::dispatch_on_hit_field(self, move_id, source_pos, target_pos),
            "HitSide" => move_callbacks::dispatch_on_hit_side(self, move_id, source_pos),
            "ModifyMove" => {
                move_callbacks::dispatch_on_modify_move(self, move_id, source_pos, target_pos)
            }
            "ModifyPriority" => {
                move_callbacks::dispatch_on_modify_priority(self, move_id, source_pos)
            }
            "ModifyTarget" => move_callbacks::dispatch_on_modify_target(self, move_id, source_pos),
            "ModifyType" => move_callbacks::dispatch_on_modify_type(self, move_id, source_pos, target_pos),
            "MoveFail" => move_callbacks::dispatch_on_move_fail(self, move_id, source_pos),
            "PrepareHit" => {
                move_callbacks::dispatch_on_prepare_hit(self, move_id, source_pos, target_pos)
            }
            "Try" => move_callbacks::dispatch_on_try(self, move_id, source_pos, target_pos),
            "TryHit" => {
                if let Some(tgt) = target_pos {
                    move_callbacks::dispatch_on_try_hit(self, move_id, source_pos, tgt)
                } else {
                    EventResult::Continue
                }
            }
            "TryImmunity" => {
                if let Some(tgt) = target_pos {
                    move_callbacks::dispatch_on_try_immunity(self, move_id, tgt)
                } else {
                    EventResult::Continue
                }
            }
            "TryMove" => move_callbacks::dispatch_on_try_move(self, move_id, source_pos, target_pos),
            "UseMoveMessage" => {
                move_callbacks::dispatch_on_use_move_message(self, move_id, source_pos)
            }
            _ => EventResult::Continue,
        }
    }
}
