//! Ability Callback Handlers
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module contains the actual callback implementations for each ability,
//! translated from data/abilities.ts in the JavaScript codebase.
//!
//! Function signatures match the JS handlers exactly:
//! - onStart(pokemon) -> on_start(pokemon)
//! - onTryHit(target, source, move) -> on_try_hit(target, source, move_)
//! - onDamagingHit(damage, target, source, move) -> on_damaging_hit(damage, target, source, move_)
//! - etc.
//!
//! Note: Only a subset of abilities are implemented here as a proof of concept.
//! The full implementation would be extensive given the 300+ abilities.

use crate::battle::{Battle, Pokemon, Move};
use crate::dex_data::ID;

// =============================================================================
// HANDLER RESULT - what the handler returns
// =============================================================================

/// Result from an ability handler - can be None (undefined in JS), or a specific value
#[derive(Debug, Clone)]
pub enum AbilityHandlerResult {
    /// No return value (undefined in JS)
    Undefined,
    /// Return false
    False,
    /// Return true
    True,
    /// Return null (blocks action)
    Null,
    /// Return 0 (for onEffectiveness)
    Zero,
    /// Return a number
    Number(i32),
    /// Chain modifier applied (numerator, denominator)
    ChainModify(u32, u32),
}

// =============================================================================
// TYPE ALIASES for cleaner signatures
// =============================================================================

/// Pokemon reference (side_index, pokemon_index)
pub type PokemonRef = (usize, usize);

/// Status object
#[derive(Debug, Clone, Default)]
pub struct Status {
    pub id: String,
}

/// Effect object
#[derive(Debug, Clone, Default)]
pub struct Effect {
    pub id: String,
    pub effect_type: String,
    /// For Move effects, whether it has a status property
    pub status: Option<String>,
}

// =============================================================================
// ABILITY HANDLERS
// =============================================================================

// -----------------------------------------------------------------------------
// INTIMIDATE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
//   intimidate: {
//       onStart(pokemon) {
//           let activated = false;
//           for (const target of pokemon.adjacentFoes()) {
//               if (!activated) {
//                   this.add('-ability', pokemon, 'Intimidate', 'boost');
//                   activated = true;
//               }
//               if (target.volatiles['substitute']) {
//                   this.add('-immune', target);
//               } else {
//                   this.boost({ atk: -1 }, target, pokemon, null, true);
//               }
//           }
//       },
//       flags: {},
//       name: "Intimidate",
//       rating: 3.5,
//       num: 22,
//   },

pub mod intimidate {
    use super::*;

    /// onStart(pokemon)
    pub fn on_start(battle: &mut Battle, pokemon: PokemonRef) -> AbilityHandlerResult {
        let (side_idx, poke_idx) = pokemon;

        // let activated = false;
        let mut activated = false;

        // Collect foe indices first to avoid borrow issues (Rust-specific)
        let foe_side = 1 - side_idx;
        let foe_data: Vec<(usize, bool)> = if let Some(foe_side_data) = battle.sides.get(foe_side) {
            foe_side_data.pokemon.iter().enumerate()
                .filter(|(_, foe)| foe.is_active && !foe.fainted)
                .map(|(idx, foe)| (idx, foe.volatiles.contains_key(&ID::new("substitute"))))
                .collect()
        } else {
            vec![]
        };

        // for (const target of pokemon.adjacentFoes())
        for (foe_idx, has_substitute) in foe_data {
            // if (!activated) {
            if !activated {
                // this.add('-ability', pokemon, 'Intimidate', 'boost');
                if let Some(poke) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
                    let pokemon_pos = poke.position;
                    battle.add_log("-ability", &[&format!("{}", pokemon_pos), "Intimidate", "boost"]);
                }
                // activated = true;
                activated = true;
            }
            // if (target.volatiles['substitute']) {
            if has_substitute {
                // this.add('-immune', target);
                if let Some(foe) = battle.sides.get(foe_side).and_then(|s| s.pokemon.get(foe_idx)) {
                    let foe_pos = foe.position;
                    battle.add_log("-immune", &[&format!("{}", foe_pos)]);
                }
            } else {
                // this.boost({ atk: -1 }, target, pokemon, null, true);
                battle.boost(&[("atk", -1)], (foe_side, foe_idx), Some(pokemon), None);
            }
        }
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// LEVITATE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
//   levitate: {
//       // airborneness implemented in sim/pokemon.js:Pokemon#isGrounded
//       flags: { breakable: 1 },
//       name: "Levitate",
//       rating: 3.5,
//       num: 26,
//   },

pub mod levitate {
    // Note: In JS, Levitate has NO handlers - just flags: { breakable: 1 }
    // The airborne check is implemented in sim/pokemon.js:Pokemon#isGrounded
    // No function to define here - the ability is handled via isGrounded()
}

// -----------------------------------------------------------------------------
// FLASH FIRE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
//   flashfire: {
//       onTryHit(target, source, move) {
//           if (target !== source && move.type === 'Fire') {
//               move.accuracy = true;
//               if (!target.addVolatile('flashfire')) {
//                   this.add('-immune', target, '[from] ability: Flash Fire');
//               }
//               return null;
//           }
//       },
//       onEnd(pokemon) {
//           pokemon.removeVolatile('flashfire');
//       },
//       condition: {
//           noCopy: true, // doesn't get copied by Baton Pass
//           onStart(target) {
//               this.add('-start', target, 'ability: Flash Fire');
//           },
//           onModifyAtkPriority: 5,
//           onModifyAtk(atk, attacker, defender, move) {
//               if (move.type === 'Fire' && attacker.hasAbility('flashfire')) {
//                   this.debug('Flash Fire boost');
//                   return this.chainModify(1.5);
//               }
//           },
//           onModifySpAPriority: 5,
//           onModifySpA(atk, attacker, defender, move) {
//               if (move.type === 'Fire' && attacker.hasAbility('flashfire')) {
//                   this.debug('Flash Fire boost');
//                   return this.chainModify(1.5);
//               }
//           },
//           onEnd(target) {
//               this.add('-end', target, 'ability: Flash Fire', '[silent]');
//           },
//       },
//       flags: { breakable: 1 },
//       name: "Flash Fire",
//       rating: 3.5,
//       num: 18,
//   },

pub mod flashfire {
    use super::*;

    /// onTryHit(target, source, move)
    pub fn on_try_hit(battle: &mut Battle, target: PokemonRef, source: PokemonRef, move_: &Move) -> AbilityHandlerResult {
        let (side_idx, poke_idx) = target;

        // if (target !== source && move.type === 'Fire')
        if target != source && move_.move_type == "Fire" {
            // move.accuracy = true; // (handled elsewhere in battle logic)

            // if (!target.addVolatile('flashfire'))
            let already_had_flashfire = battle.sides.get(side_idx)
                .and_then(|s| s.pokemon.get(poke_idx))
                .map(|p| p.volatiles.contains_key(&ID::new("flashfire")))
                .unwrap_or(false);

            // Add the volatile
            if let Some(pokemon) = battle.sides.get_mut(side_idx).and_then(|s| s.pokemon.get_mut(poke_idx)) {
                pokemon.volatiles.insert(ID::new("flashfire"), Default::default());
            }

            if already_had_flashfire {
                // this.add('-immune', target, '[from] ability: Flash Fire');
                if let Some(pokemon) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
                    let pos = pokemon.position;
                    battle.add_log("-immune", &[&format!("{}", pos), "[from] ability: Flash Fire"]);
                }
            }
            // return null;
            return AbilityHandlerResult::Null;
        }
        AbilityHandlerResult::Undefined
    }

    /// onEnd(pokemon)
    pub fn on_end(battle: &mut Battle, pokemon: PokemonRef) -> AbilityHandlerResult {
        let (side_idx, poke_idx) = pokemon;
        // pokemon.removeVolatile('flashfire');
        if let Some(poke) = battle.sides.get_mut(side_idx).and_then(|s| s.pokemon.get_mut(poke_idx)) {
            poke.volatiles.remove(&ID::new("flashfire"));
        }
        AbilityHandlerResult::Undefined
    }

    // condition handlers (for the flashfire volatile status)
    pub mod condition {
        use super::*;

        // noCopy: true, // doesn't get copied by Baton Pass
        pub const NO_COPY: bool = true;

        /// condition.onStart(target)
        pub fn on_start(battle: &mut Battle, target: PokemonRef) -> AbilityHandlerResult {
            let (side_idx, poke_idx) = target;
            // this.add('-start', target, 'ability: Flash Fire');
            if let Some(pokemon) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
                let pos = pokemon.position;
                battle.add_log("-start", &[&format!("{}", pos), "ability: Flash Fire"]);
            }
            AbilityHandlerResult::Undefined
        }

        // onModifyAtkPriority: 5,
        pub const ON_MODIFY_ATK_PRIORITY: i32 = 5;

        /// condition.onModifyAtk(atk, attacker, defender, move)
        pub fn on_modify_atk(battle: &Battle, _atk: u32, attacker: PokemonRef, _defender: PokemonRef, move_: &Move) -> AbilityHandlerResult {
            let (side_idx, poke_idx) = attacker;
            // if (move.type === 'Fire' && attacker.hasAbility('flashfire'))
            if move_.move_type == "Fire" {
                if let Some(pokemon) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
                    if pokemon.ability == ID::new("flashfire") {
                        // this.debug('Flash Fire boost');
                        // return this.chainModify(1.5);
                        return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
                    }
                }
            }
            AbilityHandlerResult::Undefined
        }

        // onModifySpAPriority: 5,
        pub const ON_MODIFY_SPA_PRIORITY: i32 = 5;

        /// condition.onModifySpA(atk, attacker, defender, move)
        pub fn on_modify_spa(battle: &Battle, _atk: u32, attacker: PokemonRef, _defender: PokemonRef, move_: &Move) -> AbilityHandlerResult {
            let (side_idx, poke_idx) = attacker;
            // if (move.type === 'Fire' && attacker.hasAbility('flashfire'))
            if move_.move_type == "Fire" {
                if let Some(pokemon) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
                    if pokemon.ability == ID::new("flashfire") {
                        // this.debug('Flash Fire boost');
                        // return this.chainModify(1.5);
                        return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
                    }
                }
            }
            AbilityHandlerResult::Undefined
        }

        /// condition.onEnd(target)
        pub fn on_end(battle: &mut Battle, target: PokemonRef) -> AbilityHandlerResult {
            let (side_idx, poke_idx) = target;
            // this.add('-end', target, 'ability: Flash Fire', '[silent]');
            if let Some(pokemon) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
                let pos = pokemon.position;
                battle.add_log("-end", &[&format!("{}", pos), "ability: Flash Fire", "[silent]"]);
            }
            AbilityHandlerResult::Undefined
        }
    }
}

// -----------------------------------------------------------------------------
// WATER ABSORB
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
//   waterabsorb: {
//       onTryHit(target, source, move) {
//           if (target !== source && move.type === 'Water') {
//               if (!this.heal(target.baseMaxhp / 4)) {
//                   this.add('-immune', target, '[from] ability: Water Absorb');
//               }
//               return null;
//           }
//       },
//       flags: { breakable: 1 },
//       name: "Water Absorb",
//       rating: 3.5,
//       num: 11,
//   },

pub mod waterabsorb {
    use super::*;

    /// onTryHit(target, source, move)
    pub fn on_try_hit(battle: &mut Battle, target: PokemonRef, source: PokemonRef, move_: &Move) -> AbilityHandlerResult {
        let (side_idx, poke_idx) = target;

        // if (target !== source && move.type === 'Water')
        if target != source && move_.move_type == "Water" {
            // if (!this.heal(target.baseMaxhp / 4))
            let heal_amount = battle.sides.get(side_idx)
                .and_then(|s| s.pokemon.get(poke_idx))
                .map(|p| p.base_maxhp / 4)
                .unwrap_or(0);

            let healed = battle.heal(heal_amount, target, None, Some("ability: Water Absorb"));

            if !healed {
                // this.add('-immune', target, '[from] ability: Water Absorb');
                if let Some(pokemon) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
                    let pos = pokemon.position;
                    battle.add_log("-immune", &[&format!("{}", pos), "[from] ability: Water Absorb"]);
                }
            }
            // return null;
            return AbilityHandlerResult::Null;
        }
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SPEED BOOST
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
//   speedboost: {
//       onResidualOrder: 28,
//       onResidualSubOrder: 2,
//       onResidual(pokemon) {
//           if (pokemon.activeTurns) {
//               this.boost({ spe: 1 });
//           }
//       },
//       flags: {},
//       name: "Speed Boost",
//       rating: 4.5,
//       num: 3,
//   },

pub mod speedboost {
    use super::*;

    // onResidualOrder: 28,
    pub const ON_RESIDUAL_ORDER: i32 = 28;
    // onResidualSubOrder: 2,
    pub const ON_RESIDUAL_SUB_ORDER: i32 = 2;

    /// onResidual(pokemon)
    pub fn on_residual(battle: &mut Battle, pokemon: PokemonRef) -> AbilityHandlerResult {
        let (side_idx, poke_idx) = pokemon;

        // if (pokemon.activeTurns)
        let active_turns = battle.sides.get(side_idx)
            .and_then(|s| s.pokemon.get(poke_idx))
            .map(|p| p.active_turns)
            .unwrap_or(0);

        if active_turns > 0 {
            // this.boost({ spe: 1 });
            battle.boost(&[("spe", 1)], pokemon, None, None);
        }
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// DRIZZLE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
//   drizzle: {
//       onStart(source) {
//           if (source.species.id === 'kyogre' && source.item === 'blueorb') return;
//           this.field.setWeather('raindance');
//       },
//       flags: {},
//       name: "Drizzle",
//       rating: 4,
//       num: 2,
//   },

pub mod drizzle {
    use super::*;

    /// onStart(source)
    pub fn on_start(battle: &mut Battle, source: PokemonRef) -> AbilityHandlerResult {
        let (side_idx, poke_idx) = source;

        // if (source.species.id === 'kyogre' && source.item === 'blueorb') return;
        let should_skip = battle.sides.get(side_idx)
            .and_then(|s| s.pokemon.get(poke_idx))
            .map(|p| p.species == ID::new("kyogre") && p.item == ID::new("blueorb"))
            .unwrap_or(false);

        if should_skip {
            return AbilityHandlerResult::Undefined;
        }

        // this.field.setWeather('raindance');
        battle.field.set_weather(ID::new("raindance"), None);
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// TECHNICIAN
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
//   technician: {
//       onBasePowerPriority: 30,
//       onBasePower(basePower, attacker, defender, move) {
//           const basePowerAfterMultiplier = this.modify(basePower, this.event.modifier);
//           this.debug(`Base Power: ${basePowerAfterMultiplier}`);
//           if (basePowerAfterMultiplier <= 60) {
//               this.debug('Technician boost');
//               return this.chainModify(1.5);
//           }
//       },
//       flags: {},
//       name: "Technician",
//       rating: 3.5,
//       num: 101,
//   },

pub mod technician {
    use super::*;

    // onBasePowerPriority: 30,
    pub const ON_BASE_POWER_PRIORITY: i32 = 30;

    /// onBasePower(basePower, attacker, defender, move)
    /// Note: basePower should already have event.modifier applied (basePowerAfterMultiplier)
    pub fn on_base_power(_battle: &Battle, base_power: u32, _attacker: PokemonRef, _defender: PokemonRef, _move: &Move) -> AbilityHandlerResult {
        // const basePowerAfterMultiplier = this.modify(basePower, this.event.modifier);
        // this.debug(`Base Power: ${basePowerAfterMultiplier}`);
        // if (basePowerAfterMultiplier <= 60) {
        if base_power <= 60 {
            // this.debug('Technician boost');
            // return this.chainModify(1.5);
            return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
        }
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// GUTS
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
//   guts: {
//       onModifyAtkPriority: 5,
//       onModifyAtk(atk, pokemon) {
//           if (pokemon.status) {
//               return this.chainModify(1.5);
//           }
//       },
//       flags: {},
//       name: "Guts",
//       rating: 3.5,
//       num: 62,
//   },

pub mod guts {
    use super::*;

    // onModifyAtkPriority: 5,
    pub const ON_MODIFY_ATK_PRIORITY: i32 = 5;

    /// onModifyAtk(atk, pokemon)
    pub fn on_modify_atk(battle: &Battle, _atk: u32, pokemon: PokemonRef) -> AbilityHandlerResult {
        let (side_idx, poke_idx) = pokemon;

        // if (pokemon.status)
        let has_status = battle.sides.get(side_idx)
            .and_then(|s| s.pokemon.get(poke_idx))
            .map(|p| !p.status.is_empty())
            .unwrap_or(false);

        if has_status {
            // return this.chainModify(1.5);
            return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
        }
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// ROUGH SKIN
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
//   roughskin: {
//       onDamagingHitOrder: 1,
//       onDamagingHit(damage, target, source, move) {
//           if (this.checkMoveMakesContact(move, source, target, true)) {
//               this.damage(source.baseMaxhp / 8, source, target);
//           }
//       },
//       flags: {},
//       name: "Rough Skin",
//       rating: 2.5,
//       num: 24,
//   },

pub mod roughskin {
    use super::*;

    // onDamagingHitOrder: 1,
    pub const ON_DAMAGING_HIT_ORDER: i32 = 1;

    /// onDamagingHit(damage, target, source, move)
    pub fn on_damaging_hit(battle: &mut Battle, _damage: u32, target: PokemonRef, source: PokemonRef, move_: &Move) -> AbilityHandlerResult {
        // if (this.checkMoveMakesContact(move, source, target, true))
        if battle.check_move_makes_contact(move_, source, target, true) {
            // this.damage(source.baseMaxhp / 8, source, target);
            let (source_side, source_idx) = source;
            let damage = battle.sides.get(source_side)
                .and_then(|s| s.pokemon.get(source_idx))
                .map(|p| p.base_maxhp / 8)
                .unwrap_or(0);

            battle.damage(damage, source, Some(target), None);
        }
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// IMMUNITY
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
//   immunity: {
//       onUpdate(pokemon) {
//           if (pokemon.status === 'psn' || pokemon.status === 'tox') {
//               this.add('-activate', pokemon, 'ability: Immunity');
//               pokemon.cureStatus();
//           }
//       },
//       onSetStatus(status, target, source, effect) {
//           if (status.id !== 'psn' && status.id !== 'tox') return;
//           if ((effect as Move)?.status) {
//               this.add('-immune', target, '[from] ability: Immunity');
//           }
//           return false;
//       },
//       flags: { breakable: 1 },
//       name: "Immunity",
//       rating: 2,
//       num: 17,
//   },

pub mod immunity {
    use super::*;

    /// onUpdate(pokemon)
    pub fn on_update(battle: &mut Battle, pokemon: PokemonRef) -> AbilityHandlerResult {
        let (side_idx, poke_idx) = pokemon;

        // if (pokemon.status === 'psn' || pokemon.status === 'tox')
        let status = battle.sides.get(side_idx)
            .and_then(|s| s.pokemon.get(poke_idx))
            .map(|p| p.status.as_str())
            .unwrap_or("");

        if status == "psn" || status == "tox" {
            // this.add('-activate', pokemon, 'ability: Immunity');
            if let Some(poke) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
                let pos = poke.position;
                battle.add_log("-activate", &[&format!("{}", pos), "ability: Immunity"]);
            }
            // pokemon.cureStatus();
            battle.cure_status(pokemon);
        }
        AbilityHandlerResult::Undefined
    }

    /// onSetStatus(status, target, source, effect)
    pub fn on_set_status(battle: &mut Battle, status: &Status, target: PokemonRef, _source: Option<PokemonRef>, effect: &Effect) -> AbilityHandlerResult {
        let (side_idx, poke_idx) = target;

        // if (status.id !== 'psn' && status.id !== 'tox') return;
        if status.id != "psn" && status.id != "tox" {
            return AbilityHandlerResult::Undefined;
        }

        // if ((effect as Move)?.status)
        if effect.effect_type == "Move" && effect.status.is_some() {
            // this.add('-immune', target, '[from] ability: Immunity');
            if let Some(pokemon) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
                let pos = pokemon.position;
                battle.add_log("-immune", &[&format!("{}", pos), "[from] ability: Immunity"]);
            }
        }
        // return false;
        AbilityHandlerResult::False
    }
}

// -----------------------------------------------------------------------------
// LIMBER
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
//   limber: {
//       onUpdate(pokemon) {
//           if (pokemon.status === 'par') {
//               this.add('-activate', pokemon, 'ability: Limber');
//               pokemon.cureStatus();
//           }
//       },
//       onSetStatus(status, target, source, effect) {
//           if (status.id !== 'par') return;
//           if ((effect as Move)?.status) {
//               this.add('-immune', target, '[from] ability: Limber');
//           }
//           return false;
//       },
//       flags: { breakable: 1 },
//       name: "Limber",
//       rating: 2,
//       num: 7,
//   },

pub mod limber {
    use super::*;

    /// onUpdate(pokemon)
    pub fn on_update(battle: &mut Battle, pokemon: PokemonRef) -> AbilityHandlerResult {
        let (side_idx, poke_idx) = pokemon;

        // if (pokemon.status === 'par')
        let status = battle.sides.get(side_idx)
            .and_then(|s| s.pokemon.get(poke_idx))
            .map(|p| p.status.as_str())
            .unwrap_or("");

        if status == "par" {
            // this.add('-activate', pokemon, 'ability: Limber');
            if let Some(poke) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
                let pos = poke.position;
                battle.add_log("-activate", &[&format!("{}", pos), "ability: Limber"]);
            }
            // pokemon.cureStatus();
            battle.cure_status(pokemon);
        }
        AbilityHandlerResult::Undefined
    }

    /// onSetStatus(status, target, source, effect)
    pub fn on_set_status(battle: &mut Battle, status: &Status, target: PokemonRef, _source: Option<PokemonRef>, effect: &Effect) -> AbilityHandlerResult {
        let (side_idx, poke_idx) = target;

        // if (status.id !== 'par') return;
        if status.id != "par" {
            return AbilityHandlerResult::Undefined;
        }

        // if ((effect as Move)?.status)
        if effect.effect_type == "Move" && effect.status.is_some() {
            // this.add('-immune', target, '[from] ability: Limber');
            if let Some(pokemon) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
                let pos = pokemon.position;
                battle.add_log("-immune", &[&format!("{}", pos), "[from] ability: Limber"]);
            }
        }
        // return false;
        AbilityHandlerResult::False
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ability_handler_result_chain_modify() {
        let result = AbilityHandlerResult::ChainModify(6144, 4096);
        match result {
            AbilityHandlerResult::ChainModify(num, denom) => {
                assert_eq!(num, 6144);
                assert_eq!(denom, 4096);
            }
            _ => panic!("Expected ChainModify"),
        }
    }

    #[test]
    fn test_technician_boost() {
        let battle = create_mock_battle();
        let pokemon = (0, 0);
        let move_ = Move::default();

        // 40 BP should get boost
        let result = technician::on_base_power(&battle, 40, pokemon, pokemon, &move_);
        assert!(matches!(result, AbilityHandlerResult::ChainModify(6144, 4096)));

        // 80 BP should not get boost
        let result2 = technician::on_base_power(&battle, 80, pokemon, pokemon, &move_);
        assert!(matches!(result2, AbilityHandlerResult::Undefined));
    }

    #[test]
    fn test_immunity_blocks_poison() {
        let mut battle = create_mock_battle();
        let target = (0, 0);

        let psn_status = Status { id: "psn".to_string() };
        let brn_status = Status { id: "brn".to_string() };
        let effect = Effect::default();

        let result = immunity::on_set_status(&mut battle, &psn_status, target, None, &effect);
        assert!(matches!(result, AbilityHandlerResult::False));

        let result2 = immunity::on_set_status(&mut battle, &brn_status, target, None, &effect);
        assert!(matches!(result2, AbilityHandlerResult::Undefined));
    }

    fn create_mock_battle() -> Battle {
        use crate::battle::BattleOptions;
        Battle::new(BattleOptions::default())
    }
}
