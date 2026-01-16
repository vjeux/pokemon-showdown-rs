// Rust-specific implementation for accessing effect state safely
// This provides a closure-based API that mirrors JavaScript's this.effectState

use crate::event_system::EffectState;
use crate::battle::{EffectType, Effect};
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
                let pos = ctx.effect_holder?;
                let pokemon = self.pokemon_at_mut(pos.0, pos.1)?;
                if ctx.id.as_str() == "counter" {
                    debug_elog!("[WITH_EFFECT_STATE] Condition: id={}, pos={:?}, found volatile={}",
                        ctx.id.as_str(), pos, pokemon.volatiles.contains_key(&ctx.id));
                    if let Some(state) = pokemon.volatiles.get(&ctx.id) {
                        debug_elog!("[WITH_EFFECT_STATE] BEFORE: slot={:?}, damage={:?}", state.slot, state.damage);
                    }
                }
                let state = pokemon.volatiles.get_mut(&ctx.id)?;
                let result = f(state);
                if ctx.id.as_str() == "counter" {
                    debug_elog!("[WITH_EFFECT_STATE] AFTER: slot={:?}, damage={:?}", state.slot, state.damage);
                }
                Some(result)
            }
            EffectType::Status => {
                // Status condition on a Pokemon (burn, paralysis, etc.)
                let pos = ctx.effect_holder?;
                let pokemon = self.pokemon_at_mut(pos.0, pos.1)?;
                Some(f(&mut pokemon.status_state))
            }
            EffectType::Ability => {
                // Ability state on a Pokemon
                let pos = ctx.effect_holder?;
                let pokemon = self.pokemon_at_mut(pos.0, pos.1)?;
                Some(f(&mut pokemon.ability_state))
            }
            EffectType::Item => {
                // Item state on a Pokemon
                let pos = ctx.effect_holder?;
                let pokemon = self.pokemon_at_mut(pos.0, pos.1)?;
                Some(f(&mut pokemon.item_state))
            }
            EffectType::SideCondition => {
                // Side condition state
                let side_idx = ctx.side_index?;
                if side_idx >= self.sides.len() {
                    return None;
                }
                let state = self.sides[side_idx].side_conditions.get_mut(&ctx.id)?;
                Some(f(state))
            }
            EffectType::SlotCondition => {
                // For slot conditions, effect_holder is (side_idx, party_idx) of the Pokemon
                // We need to use pokemon.position to look up the slot condition
                let pos = ctx.effect_holder?;
                if pos.0 >= self.sides.len() {
                    return None;
                }
                // Get the Pokemon's active position to look up slot_conditions
                let slot = self.sides.get(pos.0)?.pokemon.get(pos.1)?.position;
                let state = self.sides[pos.0].slot_conditions.get_mut(slot)?.get_mut(&ctx.id)?;
                Some(f(state))
            }
            EffectType::Weather => {
                // Weather state on field
                Some(f(&mut self.field.weather_state))
            }
            EffectType::Terrain => {
                // Terrain state on field
                Some(f(&mut self.field.terrain_state))
            }
            EffectType::FieldCondition => {
                // Pseudo-weather state on field
                let state = self.field.pseudo_weather.get_mut(&ctx.id)?;
                Some(f(state))
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
                let pos = ctx.effect_holder?;
                let pokemon = self.pokemon_at(pos.0, pos.1)?;
                let state = pokemon.volatiles.get(&ctx.id)?;
                Some(f(state))
            }
            EffectType::Status => {
                let pos = ctx.effect_holder?;
                let pokemon = self.pokemon_at(pos.0, pos.1)?;
                Some(f(&pokemon.status_state))
            }
            EffectType::Ability => {
                let pos = ctx.effect_holder?;
                let pokemon = self.pokemon_at(pos.0, pos.1)?;
                Some(f(&pokemon.ability_state))
            }
            EffectType::Item => {
                let pos = ctx.effect_holder?;
                let pokemon = self.pokemon_at(pos.0, pos.1)?;
                Some(f(&pokemon.item_state))
            }
            EffectType::SideCondition => {
                let side_idx = ctx.side_index?;
                if side_idx >= self.sides.len() {
                    return None;
                }
                let state = self.sides[side_idx].side_conditions.get(&ctx.id)?;
                Some(f(state))
            }
            EffectType::SlotCondition => {
                // For slot conditions, effect_holder is (side_idx, party_idx) of the Pokemon
                // We need to use pokemon.position to look up the slot condition
                let pos = ctx.effect_holder?;
                if pos.0 >= self.sides.len() {
                    return None;
                }
                // Get the Pokemon's active position to look up slot_conditions
                let pokemon = self.sides.get(pos.0)?.pokemon.get(pos.1)?;
                let slot = pokemon.position;
                let state = self.sides[pos.0].slot_conditions.get(slot)?.get(&ctx.id)?;
                Some(f(state))
            }
            EffectType::Weather => {
                Some(f(&self.field.weather_state))
            }
            EffectType::Terrain => {
                Some(f(&self.field.terrain_state))
            }
            EffectType::FieldCondition => {
                let state = self.field.pseudo_weather.get(&ctx.id)?;
                Some(f(state))
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
    pub fn current_effect_holder(&self) -> Option<(usize, usize)> {
        self.effect.as_ref().and_then(|ctx| ctx.effect_holder)
    }
}
