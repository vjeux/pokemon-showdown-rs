use crate::*;
use crate::battle::PriorityItem;
use crate::battle::Effect;
use crate::event::EventResult;

impl Battle {

    /// Run an event on each active Pokemon in speed order
    /// Equivalent to battle.ts eachEvent()
    // TypeScript source:
    // /**
    // 	 * Runs an event with no source on each Pokémon on the field, in Speed order.
    // 	 */
    // 	eachEvent(eventid: string, effect?: Effect | null, relayVar?: boolean) {
    // 		const actives = this.getAllActive();
    // 		if (!effect && this.effect) effect = this.effect;
    // 		this.speedSort(actives, (a, b) => b.speed - a.speed);
    // 		for (const pokemon of actives) {
    // 			this.runEvent(eventid, pokemon, null, effect, relayVar);
    // 		}
    // 		if (eventid === 'Weather' && this.gen >= 7) {
    // 			// Further research when updates happen
    // 			this.eachEvent('Update');
    // 		}
    // 	}
    //
    pub fn each_event(&mut self, event_id: &str, effect: Option<&Effect>, relay_var: Option<bool>) {
        // JS: const actives = this.getAllActive();
        // Collect all active Pokemon with their speeds
        let mut actives: Vec<(usize, usize, i32)> = Vec::new();
        for (side_idx, side) in self.sides.iter().enumerate() {
            for poke_idx in side.active.iter().flatten() {
                if let Some(pokemon) = side.pokemon.get(*poke_idx) {
                    if !pokemon.fainted {
                        actives.push((side_idx, *poke_idx, pokemon.speed));
                    }
                }
            }
        }

        // JS: if (!effect && this.effect) effect = this.effect;
        // JavaScript's this.effect is the current effect context on the Battle object
        // (set during handler processing), NOT this.event.effect
        let effect_owned: Option<Effect>;
        let effect = if effect.is_none() {
            effect_owned = self.effect.clone();
            effect_owned.as_ref()
        } else {
            effect
        };

        // JS: this.speedSort(actives, (a, b) => b.speed - a.speed);
        // Sort by speed (highest first) using speed_sort to handle ties with PRNG
        self.speed_sort_with_callsite(&mut actives, |item| PriorityItem {
            order: None,
            priority: 0,
            fractional_priority: 0.0,
            speed: item.2 as f64,  // Speed is the third element of the tuple
            sub_order: 0,
            effect_order: 0,
            index: 0,
        }, &format!("each_event:{}", event_id));

        // JS: for (const pokemon of actives) {
        // JS:     this.runEvent(eventid, pokemon, null, effect, relayVar);
        // JS: }
        // Convert boolean relay_var to EventResult for run_event
        let relay_var_result = match relay_var {
            Some(true) => EventResult::Boolean(true),
            Some(false) => EventResult::Boolean(false),
            None => EventResult::Continue,
        };
        for (side_idx, poke_idx, _speed) in actives {
            self.run_event(
                event_id,
                Some(crate::event::EventTarget::Pokemon((side_idx, poke_idx))),
                None,
                effect,
                relay_var_result.clone(),
                false,
                false,
            );
        }

        // JS: if (eventid === 'Weather' && this.gen >= 7) {
        // JS:     // Further research when updates happen
        // JS:     this.eachEvent('Update');
        // JS: }
        // ✅ NOW IMPLEMENTED: Weather events trigger Update in Gen 7+
        if event_id == "Weather" && self.gen >= 7 {
            self.each_event("Update", None, None);
        }
    }
}
