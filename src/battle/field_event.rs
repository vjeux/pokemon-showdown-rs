use crate::*;

impl Battle {

    /// Run event on field (weather, terrain, pseudo-weather)
    /// Equivalent to battle.ts fieldEvent()
    // TypeScript source:
    // /**
    // 	 * Runs an event with no source on each effect on the field, in Speed order.
    // 	 *
    // 	 * Unlike `eachEvent`, this contains a lot of other handling and is only intended for
    // 	 * the 'Residual' and 'SwitchIn' events.
    // 	 */
    // 	fieldEvent(eventid: string, targets?: Pokemon[]) {
    // 		const callbackName = `on${eventid}`;
    // 		let getKey: undefined | 'duration';
    // 		if (eventid === 'Residual') {
    // 			getKey = 'duration';
    // 		}
    // 		let handlers = this.findFieldEventHandlers(this.field, `onField${eventid}`, getKey);
    // 		for (const side of this.sides) {
    // 			if (side.n < 2 || !side.allySide) {
    // 				handlers = handlers.concat(this.findSideEventHandlers(side, `onSide${eventid}`, getKey));
    // 			}
    // 			for (const active of side.active) {
    // 				if (!active) continue;
    // 				if (eventid === 'SwitchIn') {
    // 					handlers = handlers.concat(this.findPokemonEventHandlers(active, `onAny${eventid}`));
    // 				}
    // 				if (targets && !targets.includes(active)) continue;
    // 				handlers = handlers.concat(this.findPokemonEventHandlers(active, callbackName, getKey));
    // 				handlers = handlers.concat(this.findSideEventHandlers(side, callbackName, undefined, active));
    // 				handlers = handlers.concat(this.findFieldEventHandlers(this.field, callbackName, undefined, active));
    // 				handlers = handlers.concat(this.findBattleEventHandlers(callbackName, getKey, active));
    // 			}
    // 		}
    // 		this.speedSort(handlers);
    // 		while (handlers.length) {
    // 			const handler = handlers[0];
    // 			handlers.shift();
    // 			const effect = handler.effect;
    // 			if ((handler.effectHolder as Pokemon).fainted) {
    // 				if (!(handler.state?.isSlotCondition)) continue;
    // 			}
    // 			if (eventid === 'Residual' && handler.end && handler.state?.duration) {
    // 				handler.state.duration--;
    // 				if (!handler.state.duration) {
    // 					const endCallArgs = handler.endCallArgs || [handler.effectHolder, effect.id];
    // 					handler.end.call(...endCallArgs as [any, ...any[]]);
    // 					if (this.ended) return;
    // 					continue;
    // 				}
    // 			}
    //
    // 			// effect may have been removed by a prior handler, i.e. Toxic Spikes being absorbed during a double switch
    // 			if (handler.state?.target instanceof Pokemon) {
    // 				let expectedStateLocation;
    // 				if (effect.effectType === 'Ability' && !handler.state.id.startsWith('ability:')) {
    // 					expectedStateLocation = handler.state.target.abilityState;
    // 				} else if (effect.effectType === 'Item' && !handler.state.id.startsWith('item:')) {
    // 					expectedStateLocation = handler.state.target.itemState;
    // 				} else if (effect.effectType === 'Status') {
    // 					expectedStateLocation = handler.state.target.statusState;
    // 				} else {
    // 					expectedStateLocation = handler.state.target.volatiles[effect.id];
    // 				}
    // 				if (expectedStateLocation !== handler.state) {
    // 					continue;
    // 				}
    // 			} else if (handler.state?.target instanceof Side && !handler.state.isSlotCondition) {
    // 				if ((handler.state.target.sideConditions[effect.id] !== handler.state)) {
    // 					continue;
    // 				}
    // 			} else if (handler.state?.target instanceof Field) {
    // 				let expectedStateLocation;
    // 				if (effect.effectType === 'Weather') {
    // 					expectedStateLocation = handler.state.target.weatherState;
    // 				} else if (effect.effectType === 'Terrain') {
    // 					expectedStateLocation = handler.state.target.terrainState;
    // 				} else {
    // 					expectedStateLocation = handler.state.target.pseudoWeather[effect.id];
    // 				}
    // 				if (expectedStateLocation !== handler.state) {
    // 					continue;
    // 				}
    // 			}
    //
    // 			let handlerEventid = eventid;
    // 			if ((handler.effectHolder as Side).sideConditions) handlerEventid = `Side${eventid}`;
    // 			if ((handler.effectHolder as Field).pseudoWeather) handlerEventid = `Field${eventid}`;
    // 			if (handler.callback) {
    // 				this.singleEvent(handlerEventid, effect, handler.state, handler.effectHolder, null, null, undefined, handler.callback);
    // 			}
    //
    // 			this.faintMessages();
    // 			if (this.ended) return;
    // 		}
    // 	}
    //
    // TODO: EXTREMELY INCOMPLETE IMPLEMENTATION - ~5% of TypeScript logic
    // Current implementation only runs events on weather/terrain/pseudo-weather
    // Missing from TypeScript version (battle.ts:484-568, 85 lines):
    // 1. Find handlers from field, sides, and Pokemon (findFieldEventHandlers, findSideEventHandlers, findPokemonEventHandlers, findBattleEventHandlers)
    // 2. Handle targets parameter filtering
    // 3. Handle SwitchIn special case (onAnySwitchIn)
    // 4. Sort handlers by speed (speedSort)
    // 5. Check fainted Pokemon (skip unless slot condition)
    // 6. Handle Residual event duration decrements and end callbacks
    // 7. Verify effect state still exists (handle removed effects like Toxic Spikes absorption)
    // 8. Determine handler event ID (Field/Side/normal)
    // 9. Call singleEvent with callback
    // 10. Call faintMessages and check if battle ended
    //
    // Helper methods exist: find_field_event_handlers.rs, find_side_event_handlers.rs, etc.
    // This needs FULL reimplementation to match TypeScript 1-to-1.
    //
    pub fn field_event(&mut self, event_id: &str) {
        // SIMPLIFIED VERSION - only handles weather/terrain/pseudo-weather
        // Run on weather
        if !self.field.weather.is_empty() {
            let weather_id = self.field.weather.clone();
            self.single_event(event_id, &weather_id, None, None, None);
        }

        // Run on terrain
        if !self.field.terrain.is_empty() {
            let terrain_id = self.field.terrain.clone();
            self.single_event(event_id, &terrain_id, None, None, None);
        }

        // Run on pseudo-weather
        let pseudo_weather_ids: Vec<ID> = self.field.pseudo_weather.keys().cloned().collect();
        for pw_id in pseudo_weather_ids {
            self.single_event(event_id, &pw_id, None, None, None);
        }
    }
}
