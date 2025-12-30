use crate::*;

impl Battle {

    /// Handle move events
    /// Rust helper method - JavaScript's singleEvent() directly invokes move[`on${eventId}`] callbacks
    /// This method dispatches to move_callbacks module based on event name
    /// Routes to move-specific handlers for all event types (AfterHit, BasePower, Damage, etc.)
    pub fn handle_move_event(
        &mut self,
        event_id: &str,
        move_id: &str,
        target: Option<(usize, usize)>,
    ) -> crate::event::EventResult {
        use crate::data::move_callbacks;
        use crate::event::EventResult;

        let source = self.current_event.as_ref().and_then(|e| e.source);
        let source_pos = source.unwrap_or((0, 0));
        let _target_pos = target.unwrap_or((0, 0));

        match event_id {
            "AfterHit" => {
                if let Some(target_pos) = target {
                    move_callbacks::dispatch_on_after_hit(self, move_id, source_pos, target_pos)
                } else {
                    EventResult::Continue
                }
            }
            "AfterMove" => {
                move_callbacks::dispatch_on_after_move(self, move_id, source_pos, target)
            }
            "AfterMoveSecondarySelf" => move_callbacks::dispatch_on_after_move_secondary_self(
                self, move_id, source_pos, target,
            ),
            "AfterSubDamage" => {
                // TODO: AfterSubDamage needs damage value from relay_var
                move_callbacks::dispatch_on_after_sub_damage(self, move_id, source_pos, 0, target)
            }
            "BasePower" => {
                // Get base_power from relay_var
                let base_power = self
                    .current_event
                    .as_ref()
                    .and_then(|e| e.relay_var)
                    .unwrap_or(0);

                let result = move_callbacks::dispatch_on_base_power(self, move_id, base_power, source_pos, target);
                result
            }
            "Damage" => {
                // TODO: Damage event needs damage, target_pos, source_pos, and effect_id
                // This requires architectural changes to thread these values through dispatch
                EventResult::Continue
            }
            "DamagePriority" => {
                // No moves implement DamagePriority event
                EventResult::Continue
            }
            "DisableMove" => move_callbacks::dispatch_on_disable_move(self, move_id, source_pos),
            "Effectiveness" => move_callbacks::dispatch_on_effectiveness(self, move_id, 0, "", source_pos),
            "Hit" => {
                if let Some(target_pos) = target {
                    move_callbacks::dispatch_on_hit(self, move_id, source_pos, Some(target_pos))
                } else {
                    EventResult::Continue
                }
            }
            "HitField" => move_callbacks::dispatch_on_hit_field(self, move_id, source_pos, target),
            "HitSide" => move_callbacks::dispatch_on_hit_side(self, move_id, source_pos),
            "ModifyMove" => {
                move_callbacks::dispatch_on_modify_move(self, move_id, source_pos, target)
            }
            "ModifyPriority" => {
                move_callbacks::dispatch_on_modify_priority(self, move_id, source_pos)
            }
            "ModifyTarget" => move_callbacks::dispatch_on_modify_target(self, move_id, source_pos),
            "ModifyType" => move_callbacks::dispatch_on_modify_type(self, move_id, source_pos, target),
            "MoveFail" => move_callbacks::dispatch_on_move_fail(self, move_id, source_pos),
            "PrepareHit" => {
                move_callbacks::dispatch_on_prepare_hit(self, move_id, source_pos, target)
            }
            "Try" => move_callbacks::dispatch_on_try(self, move_id, source_pos, target),
            "TryHit" => {
                if let Some(target_pos) = target {
                    move_callbacks::dispatch_on_try_hit(self, move_id, source_pos, target_pos)
                } else {
                    EventResult::Continue
                }
            }
            "TryImmunity" => move_callbacks::dispatch_on_try_immunity(self, move_id, source_pos),
            "TryMove" => move_callbacks::dispatch_on_try_move(self, move_id, source_pos, target),
            "UseMoveMessage" => {
                move_callbacks::dispatch_on_use_move_message(self, move_id, source_pos)
            }
            _ => EventResult::Continue,
        }
    }
}
