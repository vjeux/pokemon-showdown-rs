use crate::*;

impl Battle {

    /// Clear effect state data
    /// Equivalent to battle.ts clearEffectState()
    //
    // 	clearEffectState(state: EffectState) {
    // 		state.id = '';
    // 		for (const k in state) {
    // 			if (k === 'id' || k === 'target') {
    // 				continue;
    // 			} else if (k === 'effectOrder') {
    // 				state.effectOrder = 0;
    // 			} else {
    // 				delete state[k];
    // 			}
    // 		}
    // 	}
    //
    pub fn clear_effect_state(&self, state: &mut crate::event_system::EffectState) {
        // JS: state.id = '';
        state.id = ID::empty();

        // JS: state.effectOrder = 0;
        state.effect_order = 0;

        // Reset all other fields to None/default
        // 'target' is preserved as per TypeScript logic
        state.source = None;
        state.duration = None;
        state.created_turn = None;
        state.time = None;
        state.side = None;
        state.target_side = None;
        state.source_effect = None;
        state.source_slot = None;
        state.hit_count = None;
        state.contact_hit_count = None;
        state.prankster_boosted = false;
        state.move_id = None;
        state.target_loc = None;
        state.counter = None;
        state.turns = None;
        state.true_duration = None;
        state.start_time = None;
        state.bound_divisor = None;
        state.stage = None;
        state.layers = None;
        state.has_dragon_type = None;
        state.got_hit = None;
        state.accuracy = None;
        state.slot = None;
        state.damage = None;
        state.linked_pokemon = None;
        state.linked_status = None;
        state.ending_turn = None;
        state.counterpart = None;
        state.locked = None;
        state.locked_target = None;
        state.multiplier = None;
        state.hp = None;
        state.starting_turn = None;
        state.lost_focus = None;
        state.move_slot_index = None;
        state.def = None;
        state.spd = None;
        state.berry = None;
        state.choice_lock = None;
        state.syrup_triggered = None;
        state.unnerved = None;
        state.embodied = None;
        state.fallen = None;
        state.gluttony = None;
        state.ending = None;
        state.shield_boost = None;
        state.from_booster = None;
        state.best_stat = None;
        state.libero = None;
        state.seek = None;
        state.resisted = None;
        state.checked_anger_shell = None;
        state.checked_berserk = None;
        state.busted = None;
        state.berry_weaken = None;
        state.boosts = None;
        state.ready = None;
        state.eject = None;
        state.started = None;
        state.weather_suppress = None;
        state.inactive = None;
        state.allies = None;
        state.sources = None;
        state.magnitude = None;
        state.move_data = None;
    }
}
