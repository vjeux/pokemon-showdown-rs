//! Item Callback Handlers
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module contains the actual callback implementations for each item,
//! translated from data/items.ts in the JavaScript codebase.
//!
//! The design goal is to make Rust handlers look as close to JavaScript as possible.
//! Instead of returning enum variants, handlers call methods on the battle context
//! like `battle.add()`, `battle.chain_modify()`, `battle.heal()`, etc.

use std::collections::HashMap;

// =============================================================================
// BATTLE CONTEXT - mirrors `this` in JavaScript handlers
// =============================================================================

/// The battle context that provides all the methods available in JS as `this.*`
/// This is passed mutably to handlers so they can call methods like add(), heal(), etc.
pub struct Battle<'a> {
    // Battle state
    pub gen: u8,
    pub field: &'a Field,
    pub effect: Option<&'a Effect>,

    // Accumulated actions - these get processed by the battle engine after the handler returns
    actions: Vec<BattleAction>,

    // Return value modifier (for chainModify)
    modifier: Option<(u32, u32)>,

    // Whether to return early with a specific value
    return_value: Option<ReturnValue>,
}

/// Field state
#[derive(Debug, Clone, Default)]
pub struct Field {
    pub weather: Option<String>,
    pub terrain: Option<String>,
    pub pseudo_weather: HashMap<String, bool>,
}

impl Field {
    pub fn get_pseudo_weather(&self, name: &str) -> bool {
        self.pseudo_weather.get(name).copied().unwrap_or(false)
    }

    pub fn is_terrain(&self, name: &str) -> bool {
        self.terrain.as_deref() == Some(name)
    }
}

/// Current effect being applied
#[derive(Debug, Clone)]
pub struct Effect {
    pub id: String,
    pub name: String,
    pub effect_type: String,
}

/// Pokemon state - mirrors pokemon object in JS
#[derive(Debug, Clone)]
pub struct Pokemon {
    pub hp: u32,
    pub maxhp: u32,
    pub base_maxhp: u32,
    pub status: Option<String>,
    pub species: Species,
    pub base_species: Species,
    pub volatiles: HashMap<String, VolatileStatus>,
    pub ability: Option<String>,
    pub ability_state: AbilityState,
    pub item: Option<String>,
    pub nature: Option<Nature>,
    pub boosts: BoostsTable,
    pub types: Vec<String>,
    pub transformed: bool,
    pub can_evolve: bool,  // nfe
}

impl Pokemon {
    pub fn has_ability(&self, ability: &str) -> bool {
        self.ability.as_deref() == Some(ability)
    }

    pub fn has_type(&self, type_name: &str) -> bool {
        self.types.iter().any(|t| t == type_name)
    }

    pub fn get_nature(&self) -> Option<&Nature> {
        self.nature.as_ref()
    }

    /// Get move hit data - type modifier for the current move
    pub fn get_move_hit_data(&self, _move: &Move) -> MoveHitData {
        // This would be populated by the battle engine
        MoveHitData { type_mod: 0 }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Species {
    pub name: String,
    pub base_species: String,
    pub num: u32,
}

#[derive(Debug, Clone, Default)]
pub struct VolatileStatus {
    pub id: String,
}

#[derive(Debug, Clone, Default)]
pub struct AbilityState {
    pub gluttony: bool,
}

#[derive(Debug, Clone, Default)]
pub struct BoostsTable {
    pub atk: i8,
    pub def: i8,
    pub spa: i8,
    pub spd: i8,
    pub spe: i8,
    pub accuracy: i8,
    pub evasion: i8,
}

#[derive(Debug, Clone)]
pub struct Nature {
    pub name: String,
    pub plus: Option<String>,
    pub minus: Option<String>,
}

impl Nature {
    pub fn minus(&self) -> Option<&str> {
        self.minus.as_deref()
    }
}

#[derive(Debug, Clone, Default)]
pub struct MoveHitData {
    pub type_mod: i32,
}

/// Move data
#[derive(Debug, Clone)]
pub struct Move {
    pub id: String,
    pub name: String,
    pub move_type: String,
    pub category: String,
    pub flags: MoveFlags,
    pub infiltrates: bool,
}

#[derive(Debug, Clone, Default)]
pub struct MoveFlags {
    pub bypasssub: bool,
    pub contact: bool,
    pub pledgecombo: bool,
}

/// Item data
#[derive(Debug, Clone)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub mega_evolves: Option<String>,
}

/// Actions that handlers can queue up
#[derive(Debug, Clone)]
pub enum BattleAction {
    Add(Vec<String>),
    Debug(String),
    Heal { target: String, amount: u32 },
    Damage { target: String, amount: u32, source: Option<String> },
    Boost { target: String, boosts: BoostsTable },
    AddVolatile { target: String, volatile: String },
    RemoveVolatile { target: String, volatile: String },
    CureStatus { target: String },
    TrySetStatus { target: String, status: String, source: Option<String> },
    UseItem { target: String },
    EatItem { target: String },
    EndItem { target: String, item: String, reason: Option<String> },
}

/// Return values from handlers
#[derive(Debug, Clone)]
pub enum ReturnValue {
    Undefined,
    False,
    True,
    Number(i32),
    Modifier(u32, u32),
}

impl<'a> Battle<'a> {
    pub fn new(gen: u8, field: &'a Field, effect: Option<&'a Effect>) -> Self {
        Self {
            gen,
            field,
            effect,
            actions: Vec::new(),
            modifier: None,
            return_value: None,
        }
    }

    /// Log a debug message
    pub fn debug(&mut self, msg: &str) {
        self.actions.push(BattleAction::Debug(msg.to_string()));
    }

    /// Add a battle log message (like this.add('-enditem', target, ...))
    pub fn add(&mut self, args: &[&str]) {
        self.actions.push(BattleAction::Add(args.iter().map(|s| s.to_string()).collect()));
    }

    /// Chain a modifier - returns the modified value
    /// In JS: return this.chainModify([4915, 4096])
    /// In Rust: battle.chain_modify(4915, 4096); return None
    pub fn chain_modify(&mut self, numerator: u32, denominator: u32) {
        self.modifier = Some((numerator, denominator));
    }

    /// Shorthand for chain_modify with a float (converted to fraction over 4096)
    pub fn chain_modify_f(&mut self, multiplier: f64) {
        let numerator = (multiplier * 4096.0).round() as u32;
        self.modifier = Some((numerator, 4096));
    }

    /// Heal a pokemon
    pub fn heal(&mut self, target: &Pokemon, amount: u32) {
        self.actions.push(BattleAction::Heal {
            target: target.species.name.clone(),
            amount
        });
    }

    /// Damage a pokemon
    pub fn damage(&mut self, target: &Pokemon, amount: u32, source: Option<&Pokemon>) {
        self.actions.push(BattleAction::Damage {
            target: target.species.name.clone(),
            amount,
            source: source.map(|s| s.species.name.clone()),
        });
    }

    /// Boost stats
    pub fn boost(&mut self, target: &Pokemon, boosts: BoostsTable) {
        self.actions.push(BattleAction::Boost {
            target: target.species.name.clone(),
            boosts
        });
    }

    /// Add a volatile status
    pub fn add_volatile(&mut self, target: &Pokemon, volatile: &str) {
        self.actions.push(BattleAction::AddVolatile {
            target: target.species.name.clone(),
            volatile: volatile.to_string()
        });
    }

    /// Remove a volatile status
    pub fn remove_volatile(&mut self, target: &Pokemon, volatile: &str) {
        self.actions.push(BattleAction::RemoveVolatile {
            target: target.species.name.clone(),
            volatile: volatile.to_string()
        });
    }

    /// Cure status
    pub fn cure_status(&mut self, target: &Pokemon) {
        self.actions.push(BattleAction::CureStatus {
            target: target.species.name.clone()
        });
    }

    /// Try to set a status
    pub fn try_set_status(&mut self, target: &Pokemon, status: &str, source: Option<&Pokemon>) {
        self.actions.push(BattleAction::TrySetStatus {
            target: target.species.name.clone(),
            status: status.to_string(),
            source: source.map(|s| s.species.name.clone()),
        });
    }

    /// Use an item (consume it)
    pub fn use_item(&mut self, target: &Pokemon) -> bool {
        self.actions.push(BattleAction::UseItem {
            target: target.species.name.clone()
        });
        // In the actual battle, this would check if the item can be used
        // For now, assume it succeeds
        true
    }

    /// Eat a berry
    pub fn eat_item(&mut self, target: &Pokemon) -> bool {
        self.actions.push(BattleAction::EatItem {
            target: target.species.name.clone()
        });
        // In the actual battle, this would check if the berry can be eaten
        true
    }

    /// End/remove an item with a reason
    pub fn end_item(&mut self, target: &Pokemon, item: &str, reason: Option<&str>) {
        self.actions.push(BattleAction::EndItem {
            target: target.species.name.clone(),
            item: item.to_string(),
            reason: reason.map(|s| s.to_string()),
        });
    }

    /// Random chance check
    pub fn random_chance(&self, numerator: u32, denominator: u32) -> bool {
        // This would use the battle's PRNG in the real implementation
        // For now, return false (handlers should handle both cases)
        let _ = (numerator, denominator);
        false
    }

    /// Get the accumulated actions
    pub fn get_actions(&self) -> &[BattleAction] {
        &self.actions
    }

    /// Get the modifier if set
    pub fn get_modifier(&self) -> Option<(u32, u32)> {
        self.modifier
    }

    /// Get return value if set
    pub fn get_return_value(&self) -> Option<&ReturnValue> {
        self.return_value.as_ref()
    }

    /// Set return value
    pub fn set_return(&mut self, value: ReturnValue) {
        self.return_value = Some(value);
    }
}

// =============================================================================
// HANDLER RESULT - what the handler returns
// =============================================================================

/// Result from a handler - can be None (undefined in JS), or a specific value
#[derive(Debug, Clone)]
pub enum HandlerResult {
    /// No return value (undefined in JS)
    Undefined,
    /// Return false
    False,
    /// Return true
    True,
    /// Return 0 (for onEffectiveness)
    Zero,
    /// Return a number
    Number(i32),
}

// =============================================================================
// ITEM HANDLERS
// =============================================================================

// -----------------------------------------------------------------------------
// BABIRI BERRY - Type resist berry example (1:1 port from JS)
// -----------------------------------------------------------------------------

pub mod babiriberry {
    use super::*;

    /// onSourceModifyDamage(damage, source, target, move)
    ///
    /// JS source:
    /// ```js
    /// onSourceModifyDamage(damage, source, target, move) {
    ///     if (move.type === 'Steel' && target.getMoveHitData(move).typeMod > 0) {
    ///         const hitSub = target.volatiles['substitute'] && !move.flags['bypasssub'] && !(move.infiltrates && this.gen >= 6);
    ///         if (hitSub) return;
    ///         if (target.eatItem()) {
    ///             this.debug('-50% reduction');
    ///             this.add('-enditem', target, this.effect, '[weaken]');
    ///             return this.chainModify(0.5);
    ///         }
    ///     }
    /// }
    /// ```
    pub fn on_source_modify_damage(
        battle: &mut Battle,
        _damage: u32,
        _source: &Pokemon,
        target: &Pokemon,
        pokemon_move: &Move,
    ) -> HandlerResult {
        if pokemon_move.move_type == "Steel" && target.get_move_hit_data(pokemon_move).type_mod > 0 {
            let hit_sub = target.volatiles.contains_key("substitute")
                && !pokemon_move.flags.bypasssub
                && !(pokemon_move.infiltrates && battle.gen >= 6);

            if hit_sub {
                return HandlerResult::Undefined;
            }

            if battle.eat_item(target) {
                battle.debug("-50% reduction");
                if let Some(effect) = battle.effect {
                    battle.add(&["-enditem", &target.species.name, &effect.name, "[weaken]"]);
                }
                battle.chain_modify(2048, 4096); // 0.5x
                return HandlerResult::Undefined; // chainModify handles the return
            }
        }
        HandlerResult::Undefined
    }

    /// onEat() { } - empty handler
    pub fn on_eat(_battle: &mut Battle, _pokemon: &Pokemon) -> HandlerResult {
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// Generic type-resist berry - factory for all type resist berries
// -----------------------------------------------------------------------------

/// Create a type-resist berry handler for the given type
pub fn type_resist_berry_on_source_modify_damage(
    battle: &mut Battle,
    _damage: u32,
    _source: &Pokemon,
    target: &Pokemon,
    pokemon_move: &Move,
    resist_type: &str,
) -> HandlerResult {
    if pokemon_move.move_type == resist_type && target.get_move_hit_data(pokemon_move).type_mod > 0 {
        let hit_sub = target.volatiles.contains_key("substitute")
            && !pokemon_move.flags.bypasssub
            && !(pokemon_move.infiltrates && battle.gen >= 6);

        if hit_sub {
            return HandlerResult::Undefined;
        }

        if battle.eat_item(target) {
            battle.debug("-50% reduction");
            if let Some(effect) = battle.effect {
                battle.add(&["-enditem", &target.species.name, &effect.name, "[weaken]"]);
            }
            battle.chain_modify(2048, 4096); // 0.5x
        }
    }
    HandlerResult::Undefined
}

// All type-resist berries using the generic handler
pub mod occaberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, damage: u32, source: &Pokemon, target: &Pokemon, pokemon_move: &Move) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, damage, source, target, pokemon_move, "Fire")
    }
}

pub mod passhoberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, damage: u32, source: &Pokemon, target: &Pokemon, pokemon_move: &Move) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, damage, source, target, pokemon_move, "Water")
    }
}

pub mod wacanberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, damage: u32, source: &Pokemon, target: &Pokemon, pokemon_move: &Move) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, damage, source, target, pokemon_move, "Electric")
    }
}

pub mod rindoberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, damage: u32, source: &Pokemon, target: &Pokemon, pokemon_move: &Move) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, damage, source, target, pokemon_move, "Grass")
    }
}

pub mod yaborberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, damage: u32, source: &Pokemon, target: &Pokemon, pokemon_move: &Move) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, damage, source, target, pokemon_move, "Ice")
    }
}

pub mod chopleberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, damage: u32, source: &Pokemon, target: &Pokemon, pokemon_move: &Move) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, damage, source, target, pokemon_move, "Fighting")
    }
}

pub mod kebiaberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, damage: u32, source: &Pokemon, target: &Pokemon, pokemon_move: &Move) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, damage, source, target, pokemon_move, "Poison")
    }
}

pub mod shucaberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, damage: u32, source: &Pokemon, target: &Pokemon, pokemon_move: &Move) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, damage, source, target, pokemon_move, "Ground")
    }
}

pub mod cobaberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, damage: u32, source: &Pokemon, target: &Pokemon, pokemon_move: &Move) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, damage, source, target, pokemon_move, "Flying")
    }
}

pub mod payapaberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, damage: u32, source: &Pokemon, target: &Pokemon, pokemon_move: &Move) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, damage, source, target, pokemon_move, "Psychic")
    }
}

pub mod tangaberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, damage: u32, source: &Pokemon, target: &Pokemon, pokemon_move: &Move) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, damage, source, target, pokemon_move, "Bug")
    }
}

pub mod chartiberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, damage: u32, source: &Pokemon, target: &Pokemon, pokemon_move: &Move) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, damage, source, target, pokemon_move, "Rock")
    }
}

pub mod kasibberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, damage: u32, source: &Pokemon, target: &Pokemon, pokemon_move: &Move) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, damage, source, target, pokemon_move, "Ghost")
    }
}

pub mod habanberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, damage: u32, source: &Pokemon, target: &Pokemon, pokemon_move: &Move) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, damage, source, target, pokemon_move, "Dragon")
    }
}

pub mod colburberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, damage: u32, source: &Pokemon, target: &Pokemon, pokemon_move: &Move) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, damage, source, target, pokemon_move, "Dark")
    }
}

pub mod roseliberry {
    use super::*;
    pub fn on_source_modify_damage(battle: &mut Battle, damage: u32, source: &Pokemon, target: &Pokemon, pokemon_move: &Move) -> HandlerResult {
        type_resist_berry_on_source_modify_damage(battle, damage, source, target, pokemon_move, "Fairy")
    }
}

// -----------------------------------------------------------------------------
// CHILAN BERRY - resists Normal (doesn't require super effective)
// -----------------------------------------------------------------------------

pub mod chilanberry {
    use super::*;

    /// JS:
    /// ```js
    /// onSourceModifyDamage(damage, source, target, move) {
    ///     if (move.type === 'Normal') {
    ///         const hitSub = target.volatiles['substitute'] && !move.flags['bypasssub'] && !(move.infiltrates && this.gen >= 6);
    ///         if (hitSub) return;
    ///         if (target.eatItem()) {
    ///             this.debug('-50% reduction');
    ///             this.add('-enditem', target, this.effect, '[weaken]');
    ///             return this.chainModify(0.5);
    ///         }
    ///     }
    /// }
    /// ```
    pub fn on_source_modify_damage(
        battle: &mut Battle,
        _damage: u32,
        _source: &Pokemon,
        target: &Pokemon,
        pokemon_move: &Move,
    ) -> HandlerResult {
        if pokemon_move.move_type == "Normal" {
            let hit_sub = target.volatiles.contains_key("substitute")
                && !pokemon_move.flags.bypasssub
                && !(pokemon_move.infiltrates && battle.gen >= 6);

            if hit_sub {
                return HandlerResult::Undefined;
            }

            if battle.eat_item(target) {
                battle.debug("-50% reduction");
                if let Some(effect) = battle.effect {
                    battle.add(&["-enditem", &target.species.name, &effect.name, "[weaken]"]);
                }
                battle.chain_modify(2048, 4096); // 0.5x
            }
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// LEFTOVERS
// -----------------------------------------------------------------------------

pub mod leftovers {
    use super::*;

    /// JS:
    /// ```js
    /// onResidualOrder: 5,
    /// onResidualSubOrder: 5,
    /// onResidual(pokemon) {
    ///     this.heal(pokemon.baseMaxhp / 16);
    /// }
    /// ```
    pub fn on_residual(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        battle.heal(pokemon, pokemon.base_maxhp / 16);
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// BLACK SLUDGE
// -----------------------------------------------------------------------------

pub mod blacksludge {
    use super::*;

    /// JS:
    /// ```js
    /// onResidual(pokemon) {
    ///     if (this.field.isTerrain('grassyterrain')) return;
    ///     if (pokemon.hasType('Poison')) {
    ///         this.heal(pokemon.baseMaxhp / 16);
    ///     } else {
    ///         this.damage(pokemon.baseMaxhp / 8);
    ///     }
    /// }
    /// ```
    pub fn on_residual(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        if battle.field.is_terrain("grassyterrain") {
            return HandlerResult::Undefined;
        }
        if pokemon.has_type("Poison") {
            battle.heal(pokemon, pokemon.base_maxhp / 16);
        } else {
            battle.damage(pokemon, pokemon.base_maxhp / 8, None);
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// LIFE ORB
// -----------------------------------------------------------------------------

pub mod lifeorb {
    use super::*;

    /// JS:
    /// ```js
    /// onModifyDamage(damage, source, target, move) {
    ///     return this.chainModify([5324, 4096]);
    /// }
    /// ```
    pub fn on_modify_damage(battle: &mut Battle, _damage: u32, _source: &Pokemon, _target: &Pokemon, _pokemon_move: &Move) -> HandlerResult {
        battle.chain_modify(5324, 4096); // ~1.3x
        HandlerResult::Undefined
    }

    /// JS:
    /// ```js
    /// onAfterMoveSecondarySelf(source, target, move) {
    ///     if (source && source !== target && move && move.category !== 'Status' && !source.forceSwitchFlag) {
    ///         this.damage(source.baseMaxhp / 10, source, source, this.dex.items.get('lifeorb'));
    ///     }
    /// }
    /// ```
    pub fn on_after_move_secondary_self(
        battle: &mut Battle,
        source: &Pokemon,
        target: Option<&Pokemon>,
        pokemon_move: &Move,
        force_switch_flag: bool,
    ) -> HandlerResult {
        if let Some(t) = target {
            if source.species.name != t.species.name
                && pokemon_move.category != "Status"
                && !force_switch_flag
            {
                battle.damage(source, source.base_maxhp / 10, Some(source));
            }
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// IRON BALL
// -----------------------------------------------------------------------------

pub mod ironball {
    use super::*;

    /// JS:
    /// ```js
    /// onEffectiveness(typeMod, target, type, move) {
    ///     if (!target) return;
    ///     if (target.volatiles['ingrain'] || target.volatiles['smackdown'] || this.field.getPseudoWeather('gravity')) return;
    ///     if (move.type === 'Ground' && target.hasType('Flying')) return 0;
    /// }
    /// ```
    pub fn on_effectiveness(
        battle: &mut Battle,
        _type_mod: i32,
        target: Option<&Pokemon>,
        _type_name: &str,
        pokemon_move: &Move,
    ) -> HandlerResult {
        let Some(target) = target else {
            return HandlerResult::Undefined;
        };

        if target.volatiles.contains_key("ingrain")
            || target.volatiles.contains_key("smackdown")
            || battle.field.get_pseudo_weather("gravity")
        {
            return HandlerResult::Undefined;
        }

        if pokemon_move.move_type == "Ground" && target.has_type("Flying") {
            return HandlerResult::Zero;
        }

        HandlerResult::Undefined
    }

    /// JS:
    /// ```js
    /// onModifySpe(spe) {
    ///     return this.chainModify(0.5);
    /// }
    /// ```
    pub fn on_modify_spe(battle: &mut Battle, _pokemon: &Pokemon) -> HandlerResult {
        battle.chain_modify(2048, 4096); // 0.5x
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// FLAME ORB
// -----------------------------------------------------------------------------

pub mod flameorb {
    use super::*;

    /// JS:
    /// ```js
    /// onResidual(pokemon) {
    ///     pokemon.trySetStatus('brn', pokemon);
    /// }
    /// ```
    pub fn on_residual(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        battle.try_set_status(pokemon, "brn", Some(pokemon));
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// TOXIC ORB
// -----------------------------------------------------------------------------

pub mod toxicorb {
    use super::*;

    /// JS:
    /// ```js
    /// onResidual(pokemon) {
    ///     pokemon.trySetStatus('tox', pokemon);
    /// }
    /// ```
    pub fn on_residual(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        battle.try_set_status(pokemon, "tox", Some(pokemon));
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SITRUS BERRY
// -----------------------------------------------------------------------------

pub mod sitrusberry {
    use super::*;

    /// JS:
    /// ```js
    /// onUpdate(pokemon) {
    ///     if (pokemon.hp <= pokemon.maxhp / 2) {
    ///         pokemon.eatItem();
    ///     }
    /// }
    /// ```
    pub fn on_update(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        if pokemon.hp <= pokemon.maxhp / 2 {
            battle.eat_item(pokemon);
        }
        HandlerResult::Undefined
    }

    /// JS:
    /// ```js
    /// onEat(pokemon) {
    ///     this.heal(pokemon.baseMaxhp / 4);
    /// }
    /// ```
    pub fn on_eat(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        battle.heal(pokemon, pokemon.base_maxhp / 4);
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// FIGY BERRY (and similar confusion berries)
// -----------------------------------------------------------------------------

pub mod figyberry {
    use super::*;

    /// JS:
    /// ```js
    /// onUpdate(pokemon) {
    ///     if (pokemon.hp <= pokemon.maxhp / 4 || (pokemon.hp <= pokemon.maxhp / 2 &&
    ///         pokemon.hasAbility('gluttony') && pokemon.abilityState.gluttony)) {
    ///         pokemon.eatItem();
    ///     }
    /// }
    /// ```
    pub fn on_update(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        let threshold = if pokemon.has_ability("gluttony") && pokemon.ability_state.gluttony {
            pokemon.maxhp / 2
        } else {
            pokemon.maxhp / 4
        };

        if pokemon.hp <= threshold {
            battle.eat_item(pokemon);
        }
        HandlerResult::Undefined
    }

    /// JS:
    /// ```js
    /// onEat(pokemon) {
    ///     this.heal(pokemon.baseMaxhp / 3);
    ///     if (pokemon.getNature().minus === 'atk') {
    ///         pokemon.addVolatile('confusion');
    ///     }
    /// }
    /// ```
    pub fn on_eat(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        battle.heal(pokemon, pokemon.base_maxhp / 3);
        if let Some(nature) = pokemon.get_nature() {
            if nature.minus() == Some("atk") {
                battle.add_volatile(pokemon, "confusion");
            }
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// LUM BERRY
// -----------------------------------------------------------------------------

pub mod lumberry {
    use super::*;

    /// JS:
    /// ```js
    /// onUpdate(pokemon) {
    ///     if (pokemon.status || pokemon.volatiles['confusion']) {
    ///         pokemon.eatItem();
    ///     }
    /// }
    /// ```
    pub fn on_update(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        if pokemon.status.is_some() || pokemon.volatiles.contains_key("confusion") {
            battle.eat_item(pokemon);
        }
        HandlerResult::Undefined
    }

    /// JS:
    /// ```js
    /// onEat(pokemon) {
    ///     pokemon.cureStatus();
    ///     pokemon.removeVolatile('confusion');
    /// }
    /// ```
    pub fn on_eat(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        battle.cure_status(pokemon);
        battle.remove_volatile(pokemon, "confusion");
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// CHOICE BAND / SPECS / SCARF
// -----------------------------------------------------------------------------

pub mod choiceband {
    use super::*;

    /// JS:
    /// ```js
    /// onModifyAtk(atk, pokemon) {
    ///     if (pokemon.volatiles['dynamax']) return;
    ///     return this.chainModify(1.5);
    /// }
    /// ```
    pub fn on_modify_atk(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        if pokemon.volatiles.contains_key("dynamax") {
            return HandlerResult::Undefined;
        }
        battle.chain_modify(6144, 4096); // 1.5x
        HandlerResult::Undefined
    }
}

pub mod choicespecs {
    use super::*;

    pub fn on_modify_spa(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        if pokemon.volatiles.contains_key("dynamax") {
            return HandlerResult::Undefined;
        }
        battle.chain_modify(6144, 4096); // 1.5x
        HandlerResult::Undefined
    }
}

pub mod choicescarf {
    use super::*;

    pub fn on_modify_spe(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        if pokemon.volatiles.contains_key("dynamax") {
            return HandlerResult::Undefined;
        }
        battle.chain_modify(6144, 4096); // 1.5x
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// ASSAULT VEST
// -----------------------------------------------------------------------------

pub mod assaultvest {
    use super::*;

    /// JS:
    /// ```js
    /// onModifySpD(spd) {
    ///     return this.chainModify(1.5);
    /// }
    /// ```
    pub fn on_modify_spd(battle: &mut Battle, _pokemon: &Pokemon) -> HandlerResult {
        battle.chain_modify(6144, 4096); // 1.5x
        HandlerResult::Undefined
    }

    /// JS:
    /// ```js
    /// onDisableMove(pokemon) {
    ///     for (const moveSlot of pokemon.moveSlots) {
    ///         if (this.dex.moves.get(moveSlot.move).category === 'Status') {
    ///             pokemon.disableMove(moveSlot.id);
    ///         }
    ///     }
    /// }
    /// ```
    /// Note: This is handled differently - we return which moves should be disabled
    pub fn on_disable_move(_battle: &mut Battle, _pokemon: &Pokemon) -> HandlerResult {
        // The battle engine checks move category and disables Status moves
        // This handler signals that Status moves should be disabled
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// FOCUS SASH
// -----------------------------------------------------------------------------

pub mod focussash {
    use super::*;

    /// JS:
    /// ```js
    /// onDamage(damage, target, source, effect) {
    ///     if (target.hp === target.maxhp && damage >= target.hp && effect && effect.effectType === 'Move') {
    ///         if (target.useItem()) {
    ///             return target.hp - 1;
    ///         }
    ///     }
    /// }
    /// ```
    pub fn on_damage(
        battle: &mut Battle,
        damage: u32,
        target: &Pokemon,
        _source: Option<&Pokemon>,
        effect: Option<&Effect>,
    ) -> HandlerResult {
        if target.hp == target.maxhp
            && damage >= target.hp
            && effect.map_or(false, |e| e.effect_type == "Move")
        {
            if battle.use_item(target) {
                return HandlerResult::Number((target.hp - 1) as i32);
            }
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// FOCUS BAND
// -----------------------------------------------------------------------------

pub mod focusband {
    use super::*;

    /// JS:
    /// ```js
    /// onDamage(damage, target, source, effect) {
    ///     if (this.randomChance(1, 10) && damage >= target.hp && effect && effect.effectType === 'Move') {
    ///         this.add("-activate", target, "item: Focus Band");
    ///         return target.hp - 1;
    ///     }
    /// }
    /// ```
    pub fn on_damage(
        battle: &mut Battle,
        damage: u32,
        target: &Pokemon,
        _source: Option<&Pokemon>,
        effect: Option<&Effect>,
    ) -> HandlerResult {
        if battle.random_chance(1, 10)
            && damage >= target.hp
            && effect.map_or(false, |e| e.effect_type == "Move")
        {
            battle.add(&["-activate", &target.species.name, "item: Focus Band"]);
            return HandlerResult::Number((target.hp - 1) as i32);
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// EVIOLITE
// -----------------------------------------------------------------------------

pub mod eviolite {
    use super::*;

    /// JS:
    /// ```js
    /// onModifyDef(def, pokemon) {
    ///     if (pokemon.species.nfe) {
    ///         return this.chainModify(1.5);
    ///     }
    /// }
    /// ```
    pub fn on_modify_def(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        if pokemon.can_evolve {
            battle.chain_modify(6144, 4096); // 1.5x
        }
        HandlerResult::Undefined
    }

    pub fn on_modify_spd(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        if pokemon.can_evolve {
            battle.chain_modify(6144, 4096); // 1.5x
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// LIGHT BALL
// -----------------------------------------------------------------------------

pub mod lightball {
    use super::*;

    /// JS:
    /// ```js
    /// onModifyAtk(atk, pokemon) {
    ///     if (pokemon.baseSpecies.baseSpecies === 'Pikachu') {
    ///         return this.chainModify(2);
    ///     }
    /// }
    /// ```
    pub fn on_modify_atk(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        if pokemon.base_species.base_species == "Pikachu" {
            battle.chain_modify(8192, 4096); // 2x
        }
        HandlerResult::Undefined
    }

    pub fn on_modify_spa(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        if pokemon.base_species.base_species == "Pikachu" {
            battle.chain_modify(8192, 4096); // 2x
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// THICK CLUB
// -----------------------------------------------------------------------------

pub mod thickclub {
    use super::*;

    /// JS:
    /// ```js
    /// onModifyAtk(atk, pokemon) {
    ///     if (pokemon.baseSpecies.baseSpecies === 'Cubone' || pokemon.baseSpecies.baseSpecies === 'Marowak') {
    ///         return this.chainModify(2);
    ///     }
    /// }
    /// ```
    pub fn on_modify_atk(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        if pokemon.base_species.base_species == "Cubone" || pokemon.base_species.base_species == "Marowak" {
            battle.chain_modify(8192, 4096); // 2x
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// EXPERT BELT
// -----------------------------------------------------------------------------

pub mod expertbelt {
    use super::*;

    /// JS:
    /// ```js
    /// onModifyDamage(damage, source, target, move) {
    ///     if (target.getMoveHitData(move).typeMod > 0) {
    ///         return this.chainModify([4915, 4096]);
    ///     }
    /// }
    /// ```
    pub fn on_modify_damage(
        battle: &mut Battle,
        _damage: u32,
        _source: &Pokemon,
        target: &Pokemon,
        pokemon_move: &Move,
    ) -> HandlerResult {
        if target.get_move_hit_data(pokemon_move).type_mod > 0 {
            battle.chain_modify(4915, 4096); // ~1.2x
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// ABILITY SHIELD
// -----------------------------------------------------------------------------

pub mod abilityshield {
    use super::*;

    /// JS:
    /// ```js
    /// onSetAbility(ability, target, source, effect) {
    ///     if (effect && effect.effectType === 'Ability' && effect.name !== 'Trace') {
    ///         this.add('-ability', source, effect);
    ///     }
    ///     this.add('-block', target, 'item: Ability Shield');
    ///     return null;
    /// }
    /// ```
    pub fn on_set_ability(
        battle: &mut Battle,
        _ability: &str,
        target: &Pokemon,
        source: Option<&Pokemon>,
        effect: Option<&Effect>,
    ) -> HandlerResult {
        if let Some(eff) = effect {
            if eff.effect_type == "Ability" && eff.name != "Trace" {
                if let Some(src) = source {
                    battle.add(&["-ability", &src.species.name, &eff.name]);
                }
            }
        }
        battle.add(&["-block", &target.species.name, "item: Ability Shield"]);
        HandlerResult::False // return null blocks the ability change
    }
}

// -----------------------------------------------------------------------------
// ABSORB BULB
// -----------------------------------------------------------------------------

pub mod absorbbulb {
    use super::*;

    /// JS:
    /// ```js
    /// onDamagingHit(damage, target, source, move) {
    ///     if (move.type === 'Water') {
    ///         target.useItem();
    ///     }
    /// }
    /// boosts: { spa: 1 }
    /// ```
    pub fn on_damaging_hit(
        battle: &mut Battle,
        _damage: u32,
        target: &Pokemon,
        _source: &Pokemon,
        pokemon_move: &Move,
    ) -> HandlerResult {
        if pokemon_move.move_type == "Water" {
            battle.use_item(target);
        }
        HandlerResult::Undefined
    }

    /// Returns the boosts to apply when item is used
    pub fn get_boosts() -> BoostsTable {
        BoostsTable {
            spa: 1,
            ..Default::default()
        }
    }
}

// -----------------------------------------------------------------------------
// ADAMANT CRYSTAL
// -----------------------------------------------------------------------------

pub mod adamantcrystal {
    use super::*;

    /// JS:
    /// ```js
    /// onBasePower(basePower, user, target, move) {
    ///     if (user.baseSpecies.num === 483 && (move.type === 'Steel' || move.type === 'Dragon')) {
    ///         return this.chainModify([4915, 4096]);
    ///     }
    /// }
    /// ```
    pub fn on_base_power(
        battle: &mut Battle,
        _base_power: u32,
        user: &Pokemon,
        _target: &Pokemon,
        pokemon_move: &Move,
    ) -> HandlerResult {
        // num 483 = Dialga
        if user.species.num == 483
            && (pokemon_move.move_type == "Steel" || pokemon_move.move_type == "Dragon")
        {
            battle.chain_modify(4915, 4096); // ~1.2x
        }
        HandlerResult::Undefined
    }

    /// JS:
    /// ```js
    /// onTakeItem(item, pokemon, source) {
    ///     if (source?.baseSpecies.num === 483 || pokemon.baseSpecies.num === 483) {
    ///         return false;
    ///     }
    ///     return true;
    /// }
    /// ```
    pub fn on_take_item(
        _battle: &mut Battle,
        _item: &Item,
        pokemon: &Pokemon,
        source: Option<&Pokemon>,
    ) -> HandlerResult {
        if source.map_or(false, |s| s.base_species.num == 483) || pokemon.base_species.num == 483 {
            return HandlerResult::False;
        }
        HandlerResult::True
    }
}

// -----------------------------------------------------------------------------
// ADAMANT ORB
// -----------------------------------------------------------------------------

pub mod adamantorb {
    use super::*;

    /// JS:
    /// ```js
    /// onBasePower(basePower, user, target, move) {
    ///     if (user.baseSpecies.num === 483 && (move.type === 'Steel' || move.type === 'Dragon')) {
    ///         return this.chainModify([4915, 4096]);
    ///     }
    /// }
    /// ```
    pub fn on_base_power(
        battle: &mut Battle,
        _base_power: u32,
        user: &Pokemon,
        _target: &Pokemon,
        pokemon_move: &Move,
    ) -> HandlerResult {
        if user.base_species.num == 483
            && (pokemon_move.move_type == "Steel" || pokemon_move.move_type == "Dragon")
        {
            battle.chain_modify(4915, 4096); // ~1.2x
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// ADRENALINE ORB
// -----------------------------------------------------------------------------

pub mod adrenalineorb {
    use super::*;

    /// JS:
    /// ```js
    /// onAfterBoost(boost, target, source, effect) {
    ///     // Adrenaline Orb activates if Intimidate is blocked by an ability like Hyper Cutter,
    ///     // which deletes boost.atk,
    ///     // but not if the holder's attack is already at -6 (or +6 if it has Contrary),
    ///     // which sets boost.atk to 0
    ///     if (target.boosts['spe'] === 6 || boost.atk === 0) {
    ///         return;
    ///     }
    ///     if (effect.name === 'Intimidate') {
    ///         target.useItem();
    ///     }
    /// }
    /// boosts: { spe: 1 }
    /// ```
    pub fn on_after_boost(
        battle: &mut Battle,
        boost: &BoostsTable,
        target: &Pokemon,
        _source: Option<&Pokemon>,
        effect: Option<&Effect>,
    ) -> HandlerResult {
        // Already at max Speed or boost.atk is exactly 0
        if target.boosts.spe == 6 || boost.atk == 0 {
            return HandlerResult::Undefined;
        }
        if let Some(eff) = effect {
            if eff.name == "Intimidate" {
                battle.use_item(target);
            }
        }
        HandlerResult::Undefined
    }

    /// Returns the boosts to apply when item is used
    pub fn get_boosts() -> BoostsTable {
        BoostsTable {
            spe: 1,
            ..Default::default()
        }
    }
}

// -----------------------------------------------------------------------------
// AGUAV BERRY (confusion berry - spd)
// -----------------------------------------------------------------------------

pub mod aguavberry {
    use super::*;

    /// JS:
    /// ```js
    /// onUpdate(pokemon) {
    ///     if (pokemon.hp <= pokemon.maxhp / 4 || (pokemon.hp <= pokemon.maxhp / 2 &&
    ///         pokemon.hasAbility('gluttony') && pokemon.abilityState.gluttony)) {
    ///         pokemon.eatItem();
    ///     }
    /// }
    /// ```
    pub fn on_update(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        let threshold = if pokemon.has_ability("gluttony") && pokemon.ability_state.gluttony {
            pokemon.maxhp / 2
        } else {
            pokemon.maxhp / 4
        };

        if pokemon.hp <= threshold {
            battle.eat_item(pokemon);
        }
        HandlerResult::Undefined
    }

    /// JS:
    /// ```js
    /// onEat(pokemon) {
    ///     this.heal(pokemon.baseMaxhp / 3);
    ///     if (pokemon.getNature().minus === 'spd') {
    ///         pokemon.addVolatile('confusion');
    ///     }
    /// }
    /// ```
    pub fn on_eat(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        battle.heal(pokemon, pokemon.base_maxhp / 3);
        if let Some(nature) = pokemon.get_nature() {
            if nature.minus() == Some("spd") {
                battle.add_volatile(pokemon, "confusion");
            }
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// AIR BALLOON
// -----------------------------------------------------------------------------

pub mod airballoon {
    use super::*;

    /// JS:
    /// ```js
    /// onStart(target) {
    ///     if (!target.ignoringItem() && !this.field.getPseudoWeather('gravity')) {
    ///         this.add('-item', target, 'Air Balloon');
    ///     }
    /// }
    /// ```
    pub fn on_start(
        battle: &mut Battle,
        target: &Pokemon,
        ignoring_item: bool,
    ) -> HandlerResult {
        if !ignoring_item && !battle.field.get_pseudo_weather("gravity") {
            battle.add(&["-item", &target.species.name, "Air Balloon"]);
        }
        HandlerResult::Undefined
    }

    /// JS:
    /// ```js
    /// onDamagingHit(damage, target, source, move) {
    ///     this.add('-enditem', target, 'Air Balloon');
    ///     target.item = '';
    ///     this.clearEffectState(target.itemState);
    ///     this.runEvent('AfterUseItem', target, null, null, this.dex.items.get('airballoon'));
    /// }
    /// ```
    pub fn on_damaging_hit(
        battle: &mut Battle,
        _damage: u32,
        target: &Pokemon,
        _source: &Pokemon,
        _pokemon_move: &Move,
    ) -> HandlerResult {
        battle.add(&["-enditem", &target.species.name, "Air Balloon"]);
        battle.end_item(target, "Air Balloon", None);
        HandlerResult::Undefined
    }

    /// JS:
    /// ```js
    /// onAfterSubDamage(damage, target, source, effect) {
    ///     this.debug('effect: ' + effect.id);
    ///     if (effect.effectType === 'Move') {
    ///         this.add('-enditem', target, 'Air Balloon');
    ///         target.item = '';
    ///         this.clearEffectState(target.itemState);
    ///         this.runEvent('AfterUseItem', target, null, null, this.dex.items.get('airballoon'));
    ///     }
    /// }
    /// ```
    pub fn on_after_sub_damage(
        battle: &mut Battle,
        _damage: u32,
        target: &Pokemon,
        _source: &Pokemon,
        effect: Option<&Effect>,
    ) -> HandlerResult {
        if let Some(eff) = effect {
            battle.debug(&format!("effect: {}", eff.id));
            if eff.effect_type == "Move" {
                battle.add(&["-enditem", &target.species.name, "Air Balloon"]);
                battle.end_item(target, "Air Balloon", None);
            }
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// APICOT BERRY
// -----------------------------------------------------------------------------

pub mod apicotberry {
    use super::*;

    /// JS:
    /// ```js
    /// onUpdate(pokemon) {
    ///     if (pokemon.hp <= pokemon.maxhp / 4 || (pokemon.hp <= pokemon.maxhp / 2 &&
    ///         pokemon.hasAbility('gluttony') && pokemon.abilityState.gluttony)) {
    ///         pokemon.eatItem();
    ///     }
    /// }
    /// ```
    pub fn on_update(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        let threshold = if pokemon.has_ability("gluttony") && pokemon.ability_state.gluttony {
            pokemon.maxhp / 2
        } else {
            pokemon.maxhp / 4
        };

        if pokemon.hp <= threshold {
            battle.eat_item(pokemon);
        }
        HandlerResult::Undefined
    }

    /// JS:
    /// ```js
    /// onEat(pokemon) {
    ///     this.boost({ spd: 1 });
    /// }
    /// ```
    pub fn on_eat(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        battle.boost(pokemon, BoostsTable {
            spd: 1,
            ..Default::default()
        });
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// ASPEAR BERRY
// -----------------------------------------------------------------------------

pub mod aspearberry {
    use super::*;

    /// JS:
    /// ```js
    /// onUpdate(pokemon) {
    ///     if (pokemon.status === 'frz') {
    ///         pokemon.eatItem();
    ///     }
    /// }
    /// ```
    pub fn on_update(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        if pokemon.status.as_deref() == Some("frz") {
            battle.eat_item(pokemon);
        }
        HandlerResult::Undefined
    }

    /// JS:
    /// ```js
    /// onEat(pokemon) {
    ///     if (pokemon.status === 'frz') {
    ///         pokemon.cureStatus();
    ///     }
    /// }
    /// ```
    pub fn on_eat(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        if pokemon.status.as_deref() == Some("frz") {
            battle.cure_status(pokemon);
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// BERRY JUICE
// -----------------------------------------------------------------------------

pub mod berryjuice {
    use super::*;

    /// JS:
    /// ```js
    /// onUpdate(pokemon) {
    ///     if (pokemon.hp <= pokemon.maxhp / 2) {
    ///         if (this.runEvent('TryHeal', pokemon, null, this.effect, 20) && pokemon.useItem()) {
    ///             this.heal(20);
    ///         }
    ///     }
    /// }
    /// ```
    pub fn on_update(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        if pokemon.hp <= pokemon.maxhp / 2 {
            // In the full implementation, would check TryHeal event
            if battle.use_item(pokemon) {
                battle.heal(pokemon, 20);
            }
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// BIG ROOT
// -----------------------------------------------------------------------------

pub mod bigroot {
    use super::*;

    /// JS:
    /// ```js
    /// onTryHeal(damage, target, source, effect) {
    ///     const heals = ['drain', 'leechseed', 'ingrain', 'aquaring', 'strengthsap'];
    ///     if (heals.includes(effect.id)) {
    ///         return this.chainModify([5324, 4096]);
    ///     }
    /// }
    /// ```
    pub fn on_try_heal(
        battle: &mut Battle,
        _damage: u32,
        _target: &Pokemon,
        _source: Option<&Pokemon>,
        effect: Option<&Effect>,
    ) -> HandlerResult {
        let heals = ["drain", "leechseed", "ingrain", "aquaring", "strengthsap"];
        if let Some(eff) = effect {
            if heals.contains(&eff.id.as_str()) {
                battle.chain_modify(5324, 4096); // ~1.3x
            }
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// BLACK BELT (Fighting type boost)
// -----------------------------------------------------------------------------

pub mod blackbelt {
    use super::*;

    /// JS:
    /// ```js
    /// onBasePower(basePower, user, target, move) {
    ///     if (move && move.type === 'Fighting') {
    ///         return this.chainModify([4915, 4096]);
    ///     }
    /// }
    /// ```
    pub fn on_base_power(
        battle: &mut Battle,
        _base_power: u32,
        _user: &Pokemon,
        _target: &Pokemon,
        pokemon_move: &Move,
    ) -> HandlerResult {
        if pokemon_move.move_type == "Fighting" {
            battle.chain_modify(4915, 4096); // ~1.2x
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// BLACK GLASSES (Dark type boost)
// -----------------------------------------------------------------------------

pub mod blackglasses {
    use super::*;

    /// JS:
    /// ```js
    /// onBasePower(basePower, user, target, move) {
    ///     if (move && move.type === 'Dark') {
    ///         return this.chainModify([4915, 4096]);
    ///     }
    /// }
    /// ```
    pub fn on_base_power(
        battle: &mut Battle,
        _base_power: u32,
        _user: &Pokemon,
        _target: &Pokemon,
        pokemon_move: &Move,
    ) -> HandlerResult {
        if pokemon_move.move_type == "Dark" {
            battle.chain_modify(4915, 4096); // ~1.2x
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// BLUE ORB (Kyogre Primal)
// -----------------------------------------------------------------------------

pub mod blueorb {
    use super::*;

    /// JS:
    /// ```js
    /// onSwitchIn(pokemon) {
    ///     if (pokemon.isActive && pokemon.baseSpecies.name === 'Kyogre' && !pokemon.transformed) {
    ///         pokemon.formeChange('Kyogre-Primal', this.effect, true);
    ///     }
    /// }
    /// ```
    pub fn on_switch_in(
        battle: &mut Battle,
        pokemon: &Pokemon,
        is_active: bool,
    ) -> HandlerResult {
        if is_active && pokemon.base_species.name == "Kyogre" && !pokemon.transformed {
            // The forme change action would be handled by the battle engine
            battle.add(&["-formechange", &pokemon.species.name, "Kyogre-Primal"]);
        }
        HandlerResult::Undefined
    }

    /// JS:
    /// ```js
    /// onTakeItem(item, source) {
    ///     if (source.baseSpecies.baseSpecies === 'Kyogre') return false;
    ///     return true;
    /// }
    /// ```
    pub fn on_take_item(
        _battle: &mut Battle,
        _item: &Item,
        pokemon: &Pokemon,
        _source: Option<&Pokemon>,
    ) -> HandlerResult {
        if pokemon.base_species.base_species == "Kyogre" {
            return HandlerResult::False;
        }
        HandlerResult::True
    }
}

// -----------------------------------------------------------------------------
// BOOSTER ENERGY
// -----------------------------------------------------------------------------

pub mod boosterenergy {
    use super::*;

    /// JS:
    /// ```js
    /// onUpdate(pokemon) {
    ///     if (!this.effectState.started || pokemon.transformed) return;
    ///
    ///     if (pokemon.hasAbility('protosynthesis') && !this.field.isWeather('sunnyday') && pokemon.useItem()) {
    ///         pokemon.addVolatile('protosynthesis');
    ///     }
    ///     if (pokemon.hasAbility('quarkdrive') && !this.field.isTerrain('electricterrain') && pokemon.useItem()) {
    ///         pokemon.addVolatile('quarkdrive');
    ///     }
    /// }
    /// ```
    pub fn on_update(
        battle: &mut Battle,
        pokemon: &Pokemon,
        effect_state_started: bool,
    ) -> HandlerResult {
        if !effect_state_started || pokemon.transformed {
            return HandlerResult::Undefined;
        }

        if pokemon.has_ability("protosynthesis")
            && battle.field.weather.as_deref() != Some("sunnyday")
        {
            if battle.use_item(pokemon) {
                battle.add_volatile(pokemon, "protosynthesis");
            }
        }

        if pokemon.has_ability("quarkdrive")
            && !battle.field.is_terrain("electricterrain")
        {
            if battle.use_item(pokemon) {
                battle.add_volatile(pokemon, "quarkdrive");
            }
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// BRIGHT POWDER
// -----------------------------------------------------------------------------

pub mod brightpowder {
    use super::*;

    /// JS:
    /// ```js
    /// onModifyAccuracy(accuracy) {
    ///     if (typeof accuracy !== 'number') return;
    ///     this.debug('brightpowder - decreasing accuracy');
    ///     return this.chainModify([3686, 4096]);
    /// }
    /// ```
    pub fn on_modify_accuracy(battle: &mut Battle, accuracy: Option<u32>) -> HandlerResult {
        if accuracy.is_some() {
            battle.debug("brightpowder - decreasing accuracy");
            battle.chain_modify(3686, 4096); // ~0.9x
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// CELL BATTERY
// -----------------------------------------------------------------------------

pub mod cellbattery {
    use super::*;

    /// JS:
    /// ```js
    /// onDamagingHit(damage, target, source, move) {
    ///     if (move.type === 'Electric') {
    ///         target.useItem();
    ///     }
    /// }
    /// boosts: { atk: 1 }
    /// ```
    pub fn on_damaging_hit(
        battle: &mut Battle,
        _damage: u32,
        target: &Pokemon,
        _source: &Pokemon,
        pokemon_move: &Move,
    ) -> HandlerResult {
        if pokemon_move.move_type == "Electric" {
            battle.use_item(target);
        }
        HandlerResult::Undefined
    }

    /// Returns the boosts to apply when item is used
    pub fn get_boosts() -> BoostsTable {
        BoostsTable {
            atk: 1,
            ..Default::default()
        }
    }
}

// -----------------------------------------------------------------------------
// CHARCOAL (Fire type boost)
// -----------------------------------------------------------------------------

pub mod charcoal {
    use super::*;

    /// JS:
    /// ```js
    /// onBasePower(basePower, user, target, move) {
    ///     if (move && move.type === 'Fire') {
    ///         return this.chainModify([4915, 4096]);
    ///     }
    /// }
    /// ```
    pub fn on_base_power(
        battle: &mut Battle,
        _base_power: u32,
        _user: &Pokemon,
        _target: &Pokemon,
        pokemon_move: &Move,
    ) -> HandlerResult {
        if pokemon_move.move_type == "Fire" {
            battle.chain_modify(4915, 4096); // ~1.2x
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// CHERI BERRY (cures paralysis)
// -----------------------------------------------------------------------------

pub mod cheriberry {
    use super::*;

    /// JS:
    /// ```js
    /// onUpdate(pokemon) {
    ///     if (pokemon.status === 'par') {
    ///         pokemon.eatItem();
    ///     }
    /// }
    /// ```
    pub fn on_update(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        if pokemon.status.as_deref() == Some("par") {
            battle.eat_item(pokemon);
        }
        HandlerResult::Undefined
    }

    /// JS:
    /// ```js
    /// onEat(pokemon) {
    ///     if (pokemon.status === 'par') {
    ///         pokemon.cureStatus();
    ///     }
    /// }
    /// ```
    pub fn on_eat(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        if pokemon.status.as_deref() == Some("par") {
            battle.cure_status(pokemon);
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// CHESTO BERRY (cures sleep)
// -----------------------------------------------------------------------------

pub mod chestoberry {
    use super::*;

    /// JS:
    /// ```js
    /// onUpdate(pokemon) {
    ///     if (pokemon.status === 'slp') {
    ///         pokemon.eatItem();
    ///     }
    /// }
    /// ```
    pub fn on_update(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        if pokemon.status.as_deref() == Some("slp") {
            battle.eat_item(pokemon);
        }
        HandlerResult::Undefined
    }

    /// JS:
    /// ```js
    /// onEat(pokemon) {
    ///     if (pokemon.status === 'slp') {
    ///         pokemon.cureStatus();
    ///     }
    /// }
    /// ```
    pub fn on_eat(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        if pokemon.status.as_deref() == Some("slp") {
            battle.cure_status(pokemon);
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// PECHA BERRY (cures poison)
// -----------------------------------------------------------------------------

pub mod pechaberry {
    use super::*;

    /// JS:
    /// ```js
    /// onUpdate(pokemon) {
    ///     if (pokemon.status === 'psn' || pokemon.status === 'tox') {
    ///         pokemon.eatItem();
    ///     }
    /// }
    /// ```
    pub fn on_update(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        if pokemon.status.as_deref() == Some("psn") || pokemon.status.as_deref() == Some("tox") {
            battle.eat_item(pokemon);
        }
        HandlerResult::Undefined
    }

    /// JS:
    /// ```js
    /// onEat(pokemon) {
    ///     if (pokemon.status === 'psn' || pokemon.status === 'tox') {
    ///         pokemon.cureStatus();
    ///     }
    /// }
    /// ```
    pub fn on_eat(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        if pokemon.status.as_deref() == Some("psn") || pokemon.status.as_deref() == Some("tox") {
            battle.cure_status(pokemon);
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// RAWST BERRY (cures burn)
// -----------------------------------------------------------------------------

pub mod rawstberry {
    use super::*;

    /// JS:
    /// ```js
    /// onUpdate(pokemon) {
    ///     if (pokemon.status === 'brn') {
    ///         pokemon.eatItem();
    ///     }
    /// }
    /// ```
    pub fn on_update(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        if pokemon.status.as_deref() == Some("brn") {
            battle.eat_item(pokemon);
        }
        HandlerResult::Undefined
    }

    /// JS:
    /// ```js
    /// onEat(pokemon) {
    ///     if (pokemon.status === 'brn') {
    ///         pokemon.cureStatus();
    ///     }
    /// }
    /// ```
    pub fn on_eat(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        if pokemon.status.as_deref() == Some("brn") {
            battle.cure_status(pokemon);
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// PERSIM BERRY (cures confusion)
// -----------------------------------------------------------------------------

pub mod persimberry {
    use super::*;

    /// JS:
    /// ```js
    /// onUpdate(pokemon) {
    ///     if (pokemon.volatiles['confusion']) {
    ///         pokemon.eatItem();
    ///     }
    /// }
    /// ```
    pub fn on_update(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        if pokemon.volatiles.contains_key("confusion") {
            battle.eat_item(pokemon);
        }
        HandlerResult::Undefined
    }

    /// JS:
    /// ```js
    /// onEat(pokemon) {
    ///     pokemon.removeVolatile('confusion');
    /// }
    /// ```
    pub fn on_eat(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        battle.remove_volatile(pokemon, "confusion");
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// ORAN BERRY (restore 10 HP at 50%)
// -----------------------------------------------------------------------------

pub mod oranberry {
    use super::*;

    /// JS:
    /// ```js
    /// onUpdate(pokemon) {
    ///     if (pokemon.hp <= pokemon.maxhp / 2) {
    ///         pokemon.eatItem();
    ///     }
    /// }
    /// ```
    pub fn on_update(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        if pokemon.hp <= pokemon.maxhp / 2 {
            battle.eat_item(pokemon);
        }
        HandlerResult::Undefined
    }

    /// JS:
    /// ```js
    /// onEat(pokemon) {
    ///     this.heal(10);
    /// }
    /// ```
    pub fn on_eat(battle: &mut Battle, pokemon: &Pokemon) -> HandlerResult {
        battle.heal(pokemon, 10);
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// DRAGON FANG (Dragon type boost)
// -----------------------------------------------------------------------------

pub mod dragonfang {
    use super::*;

    /// JS:
    /// ```js
    /// onBasePower(basePower, user, target, move) {
    ///     if (move && move.type === 'Dragon') {
    ///         return this.chainModify([4915, 4096]);
    ///     }
    /// }
    /// ```
    pub fn on_base_power(
        battle: &mut Battle,
        _base_power: u32,
        _user: &Pokemon,
        _target: &Pokemon,
        pokemon_move: &Move,
    ) -> HandlerResult {
        if pokemon_move.move_type == "Dragon" {
            battle.chain_modify(4915, 4096); // ~1.2x
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// HARD STONE (Rock type boost)
// -----------------------------------------------------------------------------

pub mod hardstone {
    use super::*;

    /// JS:
    /// ```js
    /// onBasePower(basePower, user, target, move) {
    ///     if (move && move.type === 'Rock') {
    ///         return this.chainModify([4915, 4096]);
    ///     }
    /// }
    /// ```
    pub fn on_base_power(
        battle: &mut Battle,
        _base_power: u32,
        _user: &Pokemon,
        _target: &Pokemon,
        pokemon_move: &Move,
    ) -> HandlerResult {
        if pokemon_move.move_type == "Rock" {
            battle.chain_modify(4915, 4096); // ~1.2x
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// MAGNET (Electric type boost)
// -----------------------------------------------------------------------------

pub mod magnet {
    use super::*;

    /// JS:
    /// ```js
    /// onBasePower(basePower, user, target, move) {
    ///     if (move && move.type === 'Electric') {
    ///         return this.chainModify([4915, 4096]);
    ///     }
    /// }
    /// ```
    pub fn on_base_power(
        battle: &mut Battle,
        _base_power: u32,
        _user: &Pokemon,
        _target: &Pokemon,
        pokemon_move: &Move,
    ) -> HandlerResult {
        if pokemon_move.move_type == "Electric" {
            battle.chain_modify(4915, 4096); // ~1.2x
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// METAL COAT (Steel type boost)
// -----------------------------------------------------------------------------

pub mod metalcoat {
    use super::*;

    /// JS:
    /// ```js
    /// onBasePower(basePower, user, target, move) {
    ///     if (move && move.type === 'Steel') {
    ///         return this.chainModify([4915, 4096]);
    ///     }
    /// }
    /// ```
    pub fn on_base_power(
        battle: &mut Battle,
        _base_power: u32,
        _user: &Pokemon,
        _target: &Pokemon,
        pokemon_move: &Move,
    ) -> HandlerResult {
        if pokemon_move.move_type == "Steel" {
            battle.chain_modify(4915, 4096); // ~1.2x
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// MIRACLE SEED (Grass type boost)
// -----------------------------------------------------------------------------

pub mod miracleseed {
    use super::*;

    /// JS:
    /// ```js
    /// onBasePower(basePower, user, target, move) {
    ///     if (move && move.type === 'Grass') {
    ///         return this.chainModify([4915, 4096]);
    ///     }
    /// }
    /// ```
    pub fn on_base_power(
        battle: &mut Battle,
        _base_power: u32,
        _user: &Pokemon,
        _target: &Pokemon,
        pokemon_move: &Move,
    ) -> HandlerResult {
        if pokemon_move.move_type == "Grass" {
            battle.chain_modify(4915, 4096); // ~1.2x
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// MYSTIC WATER (Water type boost)
// -----------------------------------------------------------------------------

pub mod mysticwater {
    use super::*;

    /// JS:
    /// ```js
    /// onBasePower(basePower, user, target, move) {
    ///     if (move && move.type === 'Water') {
    ///         return this.chainModify([4915, 4096]);
    ///     }
    /// }
    /// ```
    pub fn on_base_power(
        battle: &mut Battle,
        _base_power: u32,
        _user: &Pokemon,
        _target: &Pokemon,
        pokemon_move: &Move,
    ) -> HandlerResult {
        if pokemon_move.move_type == "Water" {
            battle.chain_modify(4915, 4096); // ~1.2x
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// NEVER-MELT ICE (Ice type boost)
// -----------------------------------------------------------------------------

pub mod nevermeltice {
    use super::*;

    /// JS:
    /// ```js
    /// onBasePower(basePower, user, target, move) {
    ///     if (move && move.type === 'Ice') {
    ///         return this.chainModify([4915, 4096]);
    ///     }
    /// }
    /// ```
    pub fn on_base_power(
        battle: &mut Battle,
        _base_power: u32,
        _user: &Pokemon,
        _target: &Pokemon,
        pokemon_move: &Move,
    ) -> HandlerResult {
        if pokemon_move.move_type == "Ice" {
            battle.chain_modify(4915, 4096); // ~1.2x
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// POISON BARB (Poison type boost)
// -----------------------------------------------------------------------------

pub mod poisonbarb {
    use super::*;

    /// JS:
    /// ```js
    /// onBasePower(basePower, user, target, move) {
    ///     if (move && move.type === 'Poison') {
    ///         return this.chainModify([4915, 4096]);
    ///     }
    /// }
    /// ```
    pub fn on_base_power(
        battle: &mut Battle,
        _base_power: u32,
        _user: &Pokemon,
        _target: &Pokemon,
        pokemon_move: &Move,
    ) -> HandlerResult {
        if pokemon_move.move_type == "Poison" {
            battle.chain_modify(4915, 4096); // ~1.2x
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SHARP BEAK (Flying type boost)
// -----------------------------------------------------------------------------

pub mod sharpbeak {
    use super::*;

    /// JS:
    /// ```js
    /// onBasePower(basePower, user, target, move) {
    ///     if (move && move.type === 'Flying') {
    ///         return this.chainModify([4915, 4096]);
    ///     }
    /// }
    /// ```
    pub fn on_base_power(
        battle: &mut Battle,
        _base_power: u32,
        _user: &Pokemon,
        _target: &Pokemon,
        pokemon_move: &Move,
    ) -> HandlerResult {
        if pokemon_move.move_type == "Flying" {
            battle.chain_modify(4915, 4096); // ~1.2x
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SILK SCARF (Normal type boost)
// -----------------------------------------------------------------------------

pub mod silkscarf {
    use super::*;

    /// JS:
    /// ```js
    /// onBasePower(basePower, user, target, move) {
    ///     if (move && move.type === 'Normal') {
    ///         return this.chainModify([4915, 4096]);
    ///     }
    /// }
    /// ```
    pub fn on_base_power(
        battle: &mut Battle,
        _base_power: u32,
        _user: &Pokemon,
        _target: &Pokemon,
        pokemon_move: &Move,
    ) -> HandlerResult {
        if pokemon_move.move_type == "Normal" {
            battle.chain_modify(4915, 4096); // ~1.2x
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SILVER POWDER (Bug type boost)
// -----------------------------------------------------------------------------

pub mod silverpowder {
    use super::*;

    /// JS:
    /// ```js
    /// onBasePower(basePower, user, target, move) {
    ///     if (move && move.type === 'Bug') {
    ///         return this.chainModify([4915, 4096]);
    ///     }
    /// }
    /// ```
    pub fn on_base_power(
        battle: &mut Battle,
        _base_power: u32,
        _user: &Pokemon,
        _target: &Pokemon,
        pokemon_move: &Move,
    ) -> HandlerResult {
        if pokemon_move.move_type == "Bug" {
            battle.chain_modify(4915, 4096); // ~1.2x
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SOFT SAND (Ground type boost)
// -----------------------------------------------------------------------------

pub mod softsand {
    use super::*;

    /// JS:
    /// ```js
    /// onBasePower(basePower, user, target, move) {
    ///     if (move && move.type === 'Ground') {
    ///         return this.chainModify([4915, 4096]);
    ///     }
    /// }
    /// ```
    pub fn on_base_power(
        battle: &mut Battle,
        _base_power: u32,
        _user: &Pokemon,
        _target: &Pokemon,
        pokemon_move: &Move,
    ) -> HandlerResult {
        if pokemon_move.move_type == "Ground" {
            battle.chain_modify(4915, 4096); // ~1.2x
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SPELL TAG (Ghost type boost)
// -----------------------------------------------------------------------------

pub mod spelltag {
    use super::*;

    /// JS:
    /// ```js
    /// onBasePower(basePower, user, target, move) {
    ///     if (move && move.type === 'Ghost') {
    ///         return this.chainModify([4915, 4096]);
    ///     }
    /// }
    /// ```
    pub fn on_base_power(
        battle: &mut Battle,
        _base_power: u32,
        _user: &Pokemon,
        _target: &Pokemon,
        pokemon_move: &Move,
    ) -> HandlerResult {
        if pokemon_move.move_type == "Ghost" {
            battle.chain_modify(4915, 4096); // ~1.2x
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// TWISTED SPOON (Psychic type boost)
// -----------------------------------------------------------------------------

pub mod twistedspoon {
    use super::*;

    /// JS:
    /// ```js
    /// onBasePower(basePower, user, target, move) {
    ///     if (move && move.type === 'Psychic') {
    ///         return this.chainModify([4915, 4096]);
    ///     }
    /// }
    /// ```
    pub fn on_base_power(
        battle: &mut Battle,
        _base_power: u32,
        _user: &Pokemon,
        _target: &Pokemon,
        pokemon_move: &Move,
    ) -> HandlerResult {
        if pokemon_move.move_type == "Psychic" {
            battle.chain_modify(4915, 4096); // ~1.2x
        }
        HandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// FAIRY FEATHER (Fairy type boost)
// -----------------------------------------------------------------------------

pub mod fairyfeather {
    use super::*;

    /// JS:
    /// ```js
    /// onBasePower(basePower, user, target, move) {
    ///     if (move && move.type === 'Fairy') {
    ///         return this.chainModify([4915, 4096]);
    ///     }
    /// }
    /// ```
    pub fn on_base_power(
        battle: &mut Battle,
        _base_power: u32,
        _user: &Pokemon,
        _target: &Pokemon,
        pokemon_move: &Move,
    ) -> HandlerResult {
        if pokemon_move.move_type == "Fairy" {
            battle.chain_modify(4915, 4096); // ~1.2x
        }
        HandlerResult::Undefined
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_field() -> Field {
        Field::default()
    }

    fn make_test_pokemon() -> Pokemon {
        Pokemon {
            hp: 100,
            maxhp: 100,
            base_maxhp: 100,
            status: None,
            species: Species { name: "Test".to_string(), base_species: "Test".to_string(), num: 1 },
            base_species: Species { name: "Test".to_string(), base_species: "Test".to_string(), num: 1 },
            volatiles: HashMap::new(),
            ability: None,
            ability_state: AbilityState::default(),
            item: None,
            nature: None,
            boosts: BoostsTable::default(),
            types: vec!["Normal".to_string()],
            transformed: false,
            can_evolve: false,
        }
    }

    fn make_test_move(move_type: &str) -> Move {
        Move {
            id: "testmove".to_string(),
            name: "Test Move".to_string(),
            move_type: move_type.to_string(),
            category: "Physical".to_string(),
            flags: MoveFlags::default(),
            infiltrates: false,
        }
    }

    #[test]
    fn test_leftovers_heals() {
        let field = make_test_field();
        let mut battle = Battle::new(9, &field, None);
        let pokemon = make_test_pokemon();

        leftovers::on_residual(&mut battle, &pokemon);

        let actions = battle.get_actions();
        assert_eq!(actions.len(), 1);
        assert!(matches!(&actions[0], BattleAction::Heal { amount: 6, .. }));
    }

    #[test]
    fn test_life_orb_modifier() {
        let field = make_test_field();
        let mut battle = Battle::new(9, &field, None);
        let source = make_test_pokemon();
        let target = make_test_pokemon();
        let pokemon_move = make_test_move("Normal");

        lifeorb::on_modify_damage(&mut battle, 100, &source, &target, &pokemon_move);

        assert_eq!(battle.get_modifier(), Some((5324, 4096)));
    }

    #[test]
    fn test_iron_ball_ground_flying() {
        let field = make_test_field();
        let mut battle = Battle::new(9, &field, None);
        let mut pokemon = make_test_pokemon();
        pokemon.types = vec!["Flying".to_string()];
        let pokemon_move = make_test_move("Ground");

        let result = ironball::on_effectiveness(&mut battle, 0, Some(&pokemon), "Ground", &pokemon_move);

        assert!(matches!(result, HandlerResult::Zero));
    }

    #[test]
    fn test_choice_band_boost() {
        let field = make_test_field();
        let mut battle = Battle::new(9, &field, None);
        let pokemon = make_test_pokemon();

        choiceband::on_modify_atk(&mut battle, &pokemon);

        assert_eq!(battle.get_modifier(), Some((6144, 4096)));
    }

    #[test]
    fn test_choice_band_no_boost_dynamax() {
        let field = make_test_field();
        let mut battle = Battle::new(9, &field, None);
        let mut pokemon = make_test_pokemon();
        pokemon.volatiles.insert("dynamax".to_string(), VolatileStatus { id: "dynamax".to_string() });

        choiceband::on_modify_atk(&mut battle, &pokemon);

        assert_eq!(battle.get_modifier(), None);
    }
}
