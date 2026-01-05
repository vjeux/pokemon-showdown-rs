// NOTE: This method is NOT in JavaScript - Rust-specific implementation

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

        // IMPORTANT: run_event and single_event have OPPOSITE parameter semantics!
        //
        // For events called via run_event (like BasePower):
        //   - JavaScript calls: runEvent('BasePower', source, target, move, basePower)
        //   - But runEvent signature is: runEvent(eventid, target, source, ...)
        //   - So source (attacker) goes into "target" parameter
        //   - And target (defender) goes into "source" parameter
        //   - Therefore: event.target = attacker, event.source = defender
        //   - IMPORTANT: run_event sets self.event (not self.current_event)
        //
        // For events called via single_event:
        //   - TWO DIFFERENT PATTERNS:
        //
        //   Pattern A (ModifyMove, ModifyType, TryMove, UseMoveMessage, AfterMoveSecondarySelf):
        //     JavaScript: singleEvent('ModifyMove', move, null, pokemon, target, ...)
        //     - target param = pokemon (user), source param = target (target of move)
        //     - current_event.target = pokemon (user), current_event.source = target
        //     - User is in current_event.target
        //
        //   Pattern B (AfterHit, Hit, TryHit, PrepareHit, etc.):
        //     JavaScript: singleEvent('AfterHit', move, target, source, ...)
        //     - target param = target, source param = source (user)
        //     - current_event.target = target, current_event.source = source (user)
        //     - User is in current_event.source
        //
        // BasePower is called via run_event, so we need to extract from event.target
        // ModifyMove/ModifyType/TryMove/UseMoveMessage use Pattern A, extract from current_event.target
        // Other events use Pattern B, extract from current_event.source

        let (source_pos, target_pos) = if event_id == "BasePower" {
            // For BasePower (run_event): attacker is in event.target, defender is in event.source
            // IMPORTANT: BasePower is called via run_event, which sets self.event (not self.current_event)
            let attacker = self.event.as_ref().and_then(|e| e.target).unwrap_or((0, 0));
            let defender = self.event.as_ref().and_then(|e| e.source);
            (attacker, defender)
        } else if event_id == "ModifyMove" || event_id == "ModifyType" || event_id == "TryMove" || event_id == "UseMoveMessage" || event_id == "AfterMoveSecondarySelf" {
            // Pattern A: user is in current_event.target, target is in current_event.source
            let user = self.current_event.as_ref().and_then(|e| e.target).unwrap_or((0, 0));
            let target_of_move = self.current_event.as_ref().and_then(|e| e.source);
            (user, target_of_move)
        } else {
            // Pattern B: user is in current_event.source, target is in current_event.target
            let attacker = self.current_event.as_ref().and_then(|e| e.source).unwrap_or((0, 0));
            (attacker, target)
        };

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
                // Get damage from relay_var
                let damage = self
                    .current_event
                    .as_ref()
                    .and_then(|e| e.relay_var)
                    .unwrap_or(0);

                move_callbacks::dispatch_on_after_sub_damage(self, move_id, source_pos, damage, target)
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

                if let Some(target_pos) = target {
                    move_callbacks::dispatch_on_damage(
                        self,
                        move_id,
                        damage,
                        target_pos,
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
