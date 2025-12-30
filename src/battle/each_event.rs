use crate::*;
use crate::battle::PriorityItem;

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
    // 			// TODO: further research when updates happen
    // 			this.eachEvent('Update');
    // 		}
    // 	}
    //
    pub fn each_event(&mut self, event_id: &str, effect: Option<&ID>) {
        eprintln!("DEBUG: each_event called with event_id={}", event_id);
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

        eprintln!("DEBUG: each_event has {} active Pokemon to sort", actives.len());

        // JS: this.speedSort(actives, (a, b) => b.speed - a.speed);
        // Sort by speed (highest first) using speed_sort to handle ties with PRNG
        self.speed_sort(&mut actives, |item| PriorityItem {
            order: None,
            priority: 0,
            speed: item.2,  // Speed is the third element of the tuple
            sub_order: 0,
            effect_order: 0,
            index: 0,
        });

        // Run event on each
        for (side_idx, poke_idx, _speed) in actives {
            self.run_event(event_id, Some((side_idx, poke_idx)), None, effect, None);
        }

        // Weather update triggers Update event in Gen 7+
        if event_id == "Weather" && self.gen >= 7 {
            self.each_event("Update", None);
        }
    }
}
