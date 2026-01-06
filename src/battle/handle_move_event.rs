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
        target_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
    ) -> crate::event::EventResult {
        let move_str = move_id.as_str();

        match event_id {
            "AfterHit" => {
                move_callbacks::dispatch_on_after_hit(self, move_str, target_pos.unwrap_or((0,0)), source_pos.unwrap_or((0,0)))
            }
            "AfterMove" => {
                move_callbacks::dispatch_on_after_move(self, move_str, target_pos.unwrap_or((0,0)), source_pos)
            }
            "AfterMoveSecondarySelf" => move_callbacks::dispatch_on_after_move_secondary_self(
                self, move_str, target_pos.unwrap_or((0,0)), source_pos,
            ),
            "AfterSubDamage" => {
                // Get damage from relay_var
                let damage = self
                    .current_event
                    .as_ref()
                    .and_then(|e| match &e.relay_var { Some(EventResult::Number(n)) => Some(*n), _ => None })
                    .unwrap_or(0);

                move_callbacks::dispatch_on_after_sub_damage(self, move_str, target_pos.unwrap_or((0,0)), damage, source_pos)
            }
            "BasePower" => {
                // Get base_power from relay_var
                let base_power = self
                    .current_event
                    .as_ref()
                    .and_then(|e| match &e.relay_var { Some(EventResult::Number(n)) => Some(*n), _ => None })
                    .unwrap_or(0);

                // BasePower event is for abilities/items to modify base power
                // Note: basePowerCallback is handled in getDamage, not here
                let result = move_callbacks::dispatch_on_base_power(self, move_str, base_power, target_pos.unwrap_or((0,0)), source_pos);
                result
            }
            "Damage" => {
                // Extract all parameters immutably first
                let (damage, effect_id) = {
                    let damage = self
                        .current_event
                        .as_ref()
                        .and_then(|e| match &e.relay_var { Some(EventResult::Number(n)) => Some(*n), _ => None })
                        .unwrap_or(0);

                    let effect_id = self
                        .current_event
                        .as_ref()
                        .and_then(|e| e.effect.as_ref())
                        .map(|id| id.to_string());

                    (damage, effect_id)
                };

                move_callbacks::dispatch_on_damage(
                    self,
                    move_str,
                    damage,
                    target_pos.unwrap_or((0,0)),
                    source_pos,
                    effect_id.as_deref(),
                )
            }
            "DisableMove" => move_callbacks::dispatch_on_disable_move(self, move_str, target_pos.unwrap_or((0,0))),
            "Effectiveness" => move_callbacks::dispatch_on_effectiveness(self, move_str, 0, "", target_pos.unwrap_or((0,0))),
            "Hit" => {
                move_callbacks::dispatch_on_hit(self, move_str, target_pos.unwrap_or((0,0)), source_pos)
            }
            "HitField" => move_callbacks::dispatch_on_hit_field(self, move_str, target_pos.unwrap_or((0,0)), source_pos),
            "HitSide" => move_callbacks::dispatch_on_hit_side(self, move_str, target_pos.unwrap_or((0,0))),
            "ModifyMove" => {
                move_callbacks::dispatch_on_modify_move(self, move_str, target_pos.unwrap_or((0,0)), source_pos)
            }
            "ModifyPriority" => {
                move_callbacks::dispatch_on_modify_priority(self, move_str, target_pos.unwrap_or((0,0)))
            }
            "ModifyTarget" => move_callbacks::dispatch_on_modify_target(self, move_str, target_pos.unwrap_or((0,0))),
            "ModifyType" => move_callbacks::dispatch_on_modify_type(self, move_str, target_pos.unwrap_or((0,0)), source_pos),
            "MoveFail" => move_callbacks::dispatch_on_move_fail(self, move_str, target_pos.unwrap_or((0,0))),
            "PrepareHit" => {
                move_callbacks::dispatch_on_prepare_hit(self, move_str, target_pos.unwrap_or((0,0)), source_pos)
            }
            "Try" => move_callbacks::dispatch_on_try(self, move_str, target_pos.unwrap_or((0,0)), source_pos),
            "TryHit" => {
                move_callbacks::dispatch_on_try_hit(self, move_str, target_pos.unwrap_or((0,0)), source_pos.unwrap_or((0,0)))
            }
            "TryImmunity" => {
                if let Some(tgt) = target_pos {
                    move_callbacks::dispatch_on_try_immunity(self, move_str, tgt)
                } else {
                    EventResult::Continue
                }
            }
            "TryMove" => move_callbacks::dispatch_on_try_move(self, move_str, target_pos.unwrap_or((0,0)), source_pos),
            "UseMoveMessage" => {
                move_callbacks::dispatch_on_use_move_message(self, move_str, target_pos.unwrap_or((0,0)))
            }
            _ => EventResult::Continue,
        }
    }
}
