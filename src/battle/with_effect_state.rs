// Rust-specific implementation for accessing effect state safely
// This provides a closure-based API that mirrors JavaScript's this.effectState

use crate::event_system::EffectState;
use crate::battle::{EffectHolder, EffectType, Effect};
use crate::Battle;

impl Battle {
    /// Access the current effect's state with a closure
    /// This is the safe way to modify effect state - no cloning needed
    ///
    /// JavaScript equivalent: this.effectState
    ///
    /// The closure receives a mutable reference to the EffectState,
    /// which is looked up based on effect.
    ///
    /// # Example
    /// ```ignore
    /// battle.with_effect_state(|state| {
    ///     state.lost_focus = Some(true);
    /// });
    /// ```
    pub fn with_effect_state<F, R>(&mut self, f: F) -> Option<R>
    where
        F: FnOnce(&mut EffectState) -> R,
    {
        let ctx = self.effect.clone()?;

        match ctx.effect_type {
            EffectType::Condition => {
                // Volatile condition on a Pokemon
                let EffectHolder::Pokemon(side_idx, poke_idx) = ctx.effect_holder? else { return None };
                let pokemon = self.pokemon_at_mut(side_idx, poke_idx)?;
                if ctx.id.as_str() == "counter" {
                    debug_elog!("[WITH_EFFECT_STATE] Condition: id={}, pos=({},{}), found volatile={}",
                        ctx.id.as_str(), side_idx, poke_idx, pokemon.volatiles.contains_key(&ctx.id));
                    if let Some(_state) = pokemon.volatiles.get(&ctx.id) {
                        let _borrowed = _state.borrow();
                        debug_elog!("[WITH_EFFECT_STATE] BEFORE: slot={:?}, damage={:?}", _borrowed.slot, _borrowed.damage);
                    }
                }
                let state = pokemon.volatiles.get(&ctx.id)?;
                let mut borrowed = state.borrow_mut();
                let result = f(&mut *borrowed);
                if ctx.id.as_str() == "counter" {
                    debug_elog!("[WITH_EFFECT_STATE] AFTER: slot={:?}, damage={:?}", borrowed.slot, borrowed.damage);
                }
                Some(result)
            }
            EffectType::Status => {
                // Status condition on a Pokemon (burn, paralysis, etc.)
                let EffectHolder::Pokemon(side_idx, poke_idx) = ctx.effect_holder? else { return None };
                let pokemon = self.pokemon_at_mut(side_idx, poke_idx)?;
                let mut borrowed = pokemon.status_state.borrow_mut();
                Some(f(&mut *borrowed))
            }
            EffectType::Ability => {
                // Ability state on a Pokemon
                let EffectHolder::Pokemon(side_idx, poke_idx) = ctx.effect_holder? else { return None };
                let pokemon = self.pokemon_at_mut(side_idx, poke_idx)?;
                let boosts_before = pokemon.ability_state.borrow().boosts;
                debug_elog!("[WITH_EFFECT_STATE Ability WRITE] pos=({},{}), ability={}, ability_state.boosts BEFORE={:?}",
                    side_idx, poke_idx, pokemon.ability, boosts_before);
                let mut borrowed = pokemon.ability_state.borrow_mut();
                let result = Some(f(&mut *borrowed));
                debug_elog!("[WITH_EFFECT_STATE Ability WRITE] ability_state.boosts AFTER={:?}", borrowed.boosts);
                // Add stack trace when boosts are cleared
                if pokemon.ability.as_str() == "opportunist" && boosts_before.is_some() && borrowed.boosts.is_none() {
                    debug_elog!("[WITH_EFFECT_STATE Ability WRITE] BOOSTS WERE CLEARED! Stack trace: {}", std::backtrace::Backtrace::force_capture());
                }
                result
            }
            EffectType::Item => {
                // Item state on a Pokemon
                let EffectHolder::Pokemon(side_idx, poke_idx) = ctx.effect_holder? else { return None };
                let pokemon = self.pokemon_at_mut(side_idx, poke_idx)?;
                let mut borrowed = pokemon.item_state.borrow_mut();
                Some(f(&mut *borrowed))
            }
            EffectType::SideCondition => {
                // Side condition state
                let side_idx = ctx.side_index?;
                if side_idx >= self.sides.len() {
                    return None;
                }
                let state = self.sides[side_idx].side_conditions.get(&ctx.id)?;
                let mut borrowed = state.borrow_mut();
                Some(f(&mut *borrowed))
            }
            EffectType::SlotCondition => {
                // For slot conditions, effect_holder is (side_idx, party_idx) of the Pokemon
                // We need to use pokemon.position to look up the slot condition
                let EffectHolder::Pokemon(side_idx, poke_idx) = ctx.effect_holder? else { return None };
                if side_idx >= self.sides.len() {
                    return None;
                }
                // Get the Pokemon's active position to look up slot_conditions
                let slot = self.sides.get(side_idx)?.pokemon.get(poke_idx)?.position;
                let state = self.sides[side_idx].slot_conditions.get(slot)?.get(&ctx.id)?;
                let mut borrowed = state.borrow_mut();
                Some(f(&mut *borrowed))
            }
            EffectType::Weather => {
                // Weather state on field
                let mut borrowed = self.field.weather_state.borrow_mut();
                Some(f(&mut *borrowed))
            }
            EffectType::Terrain => {
                // Terrain state on field
                let mut borrowed = self.field.terrain_state.borrow_mut();
                Some(f(&mut *borrowed))
            }
            EffectType::FieldCondition => {
                // Pseudo-weather state on field
                let state = self.field.pseudo_weather.get(&ctx.id)?;
                let mut borrowed = state.borrow_mut();
                Some(f(&mut *borrowed))
            }
            _ => None,
        }
    }

    /// Read-only access to the current effect's state
    pub fn with_effect_state_ref<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&EffectState) -> R,
    {
        let ctx = self.effect.as_ref()?;

        match ctx.effect_type {
            EffectType::Condition => {
                let EffectHolder::Pokemon(side_idx, poke_idx) = ctx.effect_holder.clone()? else { return None };
                let pokemon = self.pokemon_at(side_idx, poke_idx)?;
                let state = pokemon.volatiles.get(&ctx.id)?;
                let borrowed = state.borrow();
                Some(f(&*borrowed))
            }
            EffectType::Status => {
                let EffectHolder::Pokemon(side_idx, poke_idx) = ctx.effect_holder.clone()? else { return None };
                let pokemon = self.pokemon_at(side_idx, poke_idx)?;
                let borrowed = pokemon.status_state.borrow();
                Some(f(&*borrowed))
            }
            EffectType::Ability => {
                let EffectHolder::Pokemon(side_idx, poke_idx) = ctx.effect_holder.clone()? else { return None };
                let pokemon = self.pokemon_at(side_idx, poke_idx)?;
                debug_elog!("[WITH_EFFECT_STATE_REF Ability READ] pos=({},{}), ability={}, ability_state.boosts={:?}",
                    side_idx, poke_idx, pokemon.ability, pokemon.ability_state.borrow().boosts);
                let borrowed = pokemon.ability_state.borrow();
                Some(f(&*borrowed))
            }
            EffectType::Item => {
                let EffectHolder::Pokemon(side_idx, poke_idx) = ctx.effect_holder.clone()? else { return None };
                let pokemon = self.pokemon_at(side_idx, poke_idx)?;
                let borrowed = pokemon.item_state.borrow();
                Some(f(&*borrowed))
            }
            EffectType::SideCondition => {
                let side_idx = ctx.side_index?;
                if side_idx >= self.sides.len() {
                    return None;
                }
                let state = self.sides[side_idx].side_conditions.get(&ctx.id)?;
                let borrowed = state.borrow();
                Some(f(&*borrowed))
            }
            EffectType::SlotCondition => {
                // For slot conditions, effect_holder is (side_idx, party_idx) of the Pokemon
                // We need to use pokemon.position to look up the slot condition
                let EffectHolder::Pokemon(side_idx, poke_idx) = ctx.effect_holder.clone()? else { return None };
                if side_idx >= self.sides.len() {
                    return None;
                }
                // Get the Pokemon's active position to look up slot_conditions
                let pokemon = self.sides.get(side_idx)?.pokemon.get(poke_idx)?;
                let slot = pokemon.position;
                let state = self.sides[side_idx].slot_conditions.get(slot)?.get(&ctx.id)?;
                let borrowed = state.borrow();
                Some(f(&*borrowed))
            }
            EffectType::Weather => {
                let borrowed = self.field.weather_state.borrow();
                Some(f(&*borrowed))
            }
            EffectType::Terrain => {
                let borrowed = self.field.terrain_state.borrow();
                Some(f(&*borrowed))
            }
            EffectType::FieldCondition => {
                let state = self.field.pseudo_weather.get(&ctx.id)?;
                let borrowed = state.borrow();
                Some(f(&*borrowed))
            }
            _ => None,
        }
    }

    /// Set the current effect context before calling a handler
    /// Used by run_event and similar functions
    pub fn set_effect_context(&mut self, ctx: Effect) -> Option<Effect> {
        std::mem::replace(&mut self.effect, Some(ctx))
    }

    /// Clear the current effect context
    pub fn clear_effect_context(&mut self) -> Option<Effect> {
        self.effect.take()
    }

    /// Restore a previous effect context
    pub fn restore_effect_context(&mut self, ctx: Option<Effect>) {
        self.effect = ctx;
    }

    /// Get the current effect ID (replaces current_effect field)
    pub fn current_effect_id(&self) -> Option<&crate::dex_data::ID> {
        self.effect.as_ref().map(|ctx| &ctx.id)
    }

    /// Get the current effect type
    pub fn current_effect_type(&self) -> Option<EffectType> {
        self.effect.as_ref().map(|ctx| ctx.effect_type)
    }

    /// Check if the current effect is Prankster boosted
    pub fn is_prankster_boosted(&self) -> bool {
        self.effect.as_ref().map(|ctx| ctx.prankster_boosted).unwrap_or(false)
    }

    /// Get the current effect holder position
    pub fn current_effect_holder(&self) -> Option<EffectHolder> {
        self.effect.as_ref().and_then(|ctx| ctx.effect_holder.clone())
    }
}
