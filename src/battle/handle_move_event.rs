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
        let _ = move_id; // No longer needed directly, using active_move_clone

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

                move_callbacks::dispatch_on_after_sub_damage(self, active_move_clone.as_ref(), target_pos.unwrap_or((0,0)), damage, source_pos)
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
                let result = move_callbacks::dispatch_on_base_power(self, active_move_clone.as_ref(), base_power, target_pos.unwrap_or((0,0)), source_pos);
                result
            }
            "Damage" => {
                // Extract all parameters immutably first
                let (damage, effect_id) = {
                    let damage = self
                        .event
                        .as_ref()
                        .and_then(|e| match &e.relay_var { Some(EventResult::Number(n)) => Some(*n), _ => None })
                        .unwrap_or(0);

                    let effect_id = self
                        .event
                        .as_ref()
                        .and_then(|e| e.effect.as_ref())
                        .map(|eff| eff.id.to_string());

                    (damage, effect_id)
                };

                move_callbacks::dispatch_on_damage(
                    self,
                    active_move_clone.as_ref(),
                    damage,
                    target_pos.unwrap_or((0,0)),
                    source_pos,
                    effect_id.as_deref(),
                )
            }
            "DisableMove" => move_callbacks::dispatch_on_disable_move(self, active_move_clone.as_ref(), target_pos.unwrap_or((0,0))),
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
                // Call both regular onHit and self.onHit callbacks
                // Regular onHit targets the move target, self.onHit targets the move user
                let result = move_callbacks::dispatch_on_hit(self, active_move_clone.as_ref(), target_pos.unwrap_or((0,0)), source_pos);

                // For self callbacks, only call dispatch_self_on_hit if target != source.
                // If target == source, we're in self_drops calling moveHit for self effects,
                // and we don't want to recursively call self.onHit again.
                let self_result = match (source_pos, target_pos) {
                    (Some(src), Some(tgt)) if src != tgt => {
                        move_callbacks::dispatch_self_on_hit(self, active_move_clone.as_ref(), src, target_pos)
                    }
                    (Some(src), None) => {
                        move_callbacks::dispatch_self_on_hit(self, active_move_clone.as_ref(), src, target_pos)
                    }
                    _ => EventResult::Continue,
                };

                // If either returns a non-Continue result, use that; otherwise Continue
                match (result, self_result) {
                    (EventResult::Continue, EventResult::Continue) => EventResult::Continue,
                    (EventResult::Continue, other) => other,
                    (other, _) => other,
                }
            }
            "HitField" => move_callbacks::dispatch_on_hit_field(self, active_move_clone.as_ref(), target_pos.unwrap_or((0,0)), source_pos),
            "HitSide" => move_callbacks::dispatch_on_hit_side(self, active_move_clone.as_ref(), target_pos.unwrap_or((0,0))),
            "ModifyMove" => {
                move_callbacks::dispatch_on_modify_move(self, active_move_clone.as_ref(), target_pos.unwrap_or((0,0)), source_pos)
            }
            "ModifyPriority" => {
                move_callbacks::dispatch_on_modify_priority(self, active_move_clone.as_ref(), target_pos.unwrap_or((0,0)))
            }
            "ModifyTarget" => move_callbacks::dispatch_on_modify_target(self, active_move_clone.as_ref(), target_pos.unwrap_or((0,0))),
            "ModifyType" => move_callbacks::dispatch_on_modify_type(self, active_move_clone.as_ref(), target_pos.unwrap_or((0,0)), source_pos),
            "MoveFail" => move_callbacks::dispatch_on_move_fail(self, active_move_clone.as_ref(), target_pos.unwrap_or((0,0))),
            "PrepareHit" => {
                move_callbacks::dispatch_on_prepare_hit(self, active_move_clone.as_ref(), target_pos.unwrap_or((0,0)), source_pos)
            }
            "Try" => move_callbacks::dispatch_on_try(self, active_move_clone.as_ref(), target_pos.unwrap_or((0,0)), source_pos),
            "TryHit" => {
                move_callbacks::dispatch_on_try_hit(self, active_move_clone.as_ref(), target_pos.unwrap_or((0,0)), source_pos.unwrap_or((0,0)))
            }
            "TryImmunity" => {
                if let Some(tgt) = target_pos {
                    move_callbacks::dispatch_on_try_immunity(self, active_move_clone.as_ref(), tgt)
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
