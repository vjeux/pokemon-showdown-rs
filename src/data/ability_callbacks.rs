//! Ability Callback Handlers
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module contains the actual callback implementations for each ability,
//! translated from data/abilities.ts in the JavaScript codebase.
//!
//! Function signatures match the JS handlers exactly:
//! - onStart(pokemon) -> on_start(battle, pokemon)
//! - onTryHit(target, source, move) -> on_try_hit(battle, target, source, move_)
//! - onDamagingHit(damage, target, source, move) -> on_damaging_hit(battle, damage, target, source, move_)
//! - etc.
//!
//! Note: Only a subset of abilities are implemented here as a proof of concept.
//! The full implementation would be extensive given the 300+ abilities.

use crate::battle::{Battle, Move};
use crate::pokemon::Pokemon;
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
    pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
        // let activated = false;
        let mut activated = false;

        // Collect foe data first to avoid borrow issues (Rust-specific)
        let foe_side = 1 - pokemon.side_index;
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
                battle.add_log("-ability", &[&format!("{}", pokemon.position), "Intimidate", "boost"]);
                // activated = true;
                activated = true;
            }
            // if (target.volatiles['substitute']) {
            if has_substitute {
                // this.add('-immune', target);
                if let Some(foe) = battle.sides.get(foe_side).and_then(|s| s.pokemon.get(foe_idx)) {
                    battle.add_log("-immune", &[&format!("{}", foe.position)]);
                }
            } else {
                // this.boost({ atk: -1 }, target, pokemon, null, true);
                battle.boost(&[("atk", -1)], (foe_side, foe_idx), Some((pokemon.side_index, pokemon.position)), None);
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
    pub fn on_try_hit(battle: &mut Battle, target: &mut Pokemon, source: &Pokemon, move_: &Move) -> AbilityHandlerResult {
        // if (target !== source && move.type === 'Fire')
        if target.position != source.position && move_.move_type == "Fire" {
            // move.accuracy = true; // (handled elsewhere in battle logic)

            // if (!target.addVolatile('flashfire'))
            let already_had_flashfire = target.volatiles.contains_key(&ID::new("flashfire"));
            target.volatiles.insert(ID::new("flashfire"), Default::default());

            if already_had_flashfire {
                // this.add('-immune', target, '[from] ability: Flash Fire');
                battle.add_log("-immune", &[&format!("{}", target.position), "[from] ability: Flash Fire"]);
            }
            // return null;
            return AbilityHandlerResult::Null;
        }
        AbilityHandlerResult::Undefined
    }

    /// onEnd(pokemon)
    pub fn on_end(pokemon: &mut Pokemon) -> AbilityHandlerResult {
        // pokemon.removeVolatile('flashfire');
        pokemon.volatiles.remove(&ID::new("flashfire"));
        AbilityHandlerResult::Undefined
    }

    // condition handlers (for the flashfire volatile status)
    pub mod condition {
        use super::*;

        // noCopy: true, // doesn't get copied by Baton Pass
        pub const NO_COPY: bool = true;

        /// condition.onStart(target)
        pub fn on_start(battle: &mut Battle, target: &Pokemon) -> AbilityHandlerResult {
            // this.add('-start', target, 'ability: Flash Fire');
            battle.add_log("-start", &[&format!("{}", target.position), "ability: Flash Fire"]);
            AbilityHandlerResult::Undefined
        }

        // onModifyAtkPriority: 5,
        pub const ON_MODIFY_ATK_PRIORITY: i32 = 5;

        /// condition.onModifyAtk(atk, attacker, defender, move)
        pub fn on_modify_atk(_atk: u32, attacker: &Pokemon, _defender: &Pokemon, move_: &Move) -> AbilityHandlerResult {
            // if (move.type === 'Fire' && attacker.hasAbility('flashfire'))
            if move_.move_type == "Fire" && attacker.ability == ID::new("flashfire") {
                // this.debug('Flash Fire boost');
                // return this.chainModify(1.5);
                return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
            }
            AbilityHandlerResult::Undefined
        }

        // onModifySpAPriority: 5,
        pub const ON_MODIFY_SPA_PRIORITY: i32 = 5;

        /// condition.onModifySpA(atk, attacker, defender, move)
        pub fn on_modify_spa(_atk: u32, attacker: &Pokemon, _defender: &Pokemon, move_: &Move) -> AbilityHandlerResult {
            // if (move.type === 'Fire' && attacker.hasAbility('flashfire'))
            if move_.move_type == "Fire" && attacker.ability == ID::new("flashfire") {
                // this.debug('Flash Fire boost');
                // return this.chainModify(1.5);
                return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
            }
            AbilityHandlerResult::Undefined
        }

        /// condition.onEnd(target)
        pub fn on_end(battle: &mut Battle, target: &Pokemon) -> AbilityHandlerResult {
            // this.add('-end', target, 'ability: Flash Fire', '[silent]');
            battle.add_log("-end", &[&format!("{}", target.position), "ability: Flash Fire", "[silent]"]);
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
    pub fn on_try_hit(battle: &mut Battle, target: &Pokemon, source: &Pokemon, move_: &Move) -> AbilityHandlerResult {
        // if (target !== source && move.type === 'Water')
        if target.position != source.position && move_.move_type == "Water" {
            // if (!this.heal(target.baseMaxhp / 4))
            let heal_amount = target.base_maxhp / 4;
            let target_ref = (target.side_index, target.position);
            let healed = battle.heal(heal_amount, target_ref, None, Some("ability: Water Absorb"));

            if !healed {
                // this.add('-immune', target, '[from] ability: Water Absorb');
                battle.add_log("-immune", &[&format!("{}", target.position), "[from] ability: Water Absorb"]);
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
    pub fn on_residual(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
        // if (pokemon.activeTurns)
        if pokemon.active_turns > 0 {
            // this.boost({ spe: 1 });
            let pokemon_ref = (pokemon.side_index, pokemon.position);
            battle.boost(&[("spe", 1)], pokemon_ref, None, None);
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
    pub fn on_start(battle: &mut Battle, source: &Pokemon) -> AbilityHandlerResult {
        // if (source.species.id === 'kyogre' && source.item === 'blueorb') return;
        if source.species_id == ID::new("kyogre") && source.item == ID::new("blueorb") {
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
    pub fn on_base_power(base_power: u32, _attacker: &Pokemon, _defender: &Pokemon, _move: &Move) -> AbilityHandlerResult {
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
    pub fn on_modify_atk(_atk: u32, pokemon: &Pokemon) -> AbilityHandlerResult {
        // if (pokemon.status)
        if !pokemon.status.is_empty() {
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
    pub fn on_damaging_hit(battle: &mut Battle, _damage: u32, target: &Pokemon, source: &Pokemon, move_: &Move) -> AbilityHandlerResult {
        // if (this.checkMoveMakesContact(move, source, target, true))
        let source_ref = (source.side_index, source.position);
        let target_ref = (target.side_index, target.position);
        if battle.check_move_makes_contact(move_, source_ref, target_ref, true) {
            // this.damage(source.baseMaxhp / 8, source, target);
            let damage = source.base_maxhp / 8;
            battle.damage(damage, source_ref, Some(target_ref), None);
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
    pub fn on_update(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
        // if (pokemon.status === 'psn' || pokemon.status === 'tox')
        if pokemon.status.as_str() == "psn" || pokemon.status.as_str() == "tox" {
            // this.add('-activate', pokemon, 'ability: Immunity');
            battle.add_log("-activate", &[&format!("{}", pokemon.position), "ability: Immunity"]);
            // pokemon.cureStatus();
            let pokemon_ref = (pokemon.side_index, pokemon.position);
            battle.cure_status(pokemon_ref);
        }
        AbilityHandlerResult::Undefined
    }

    /// onSetStatus(status, target, source, effect)
    pub fn on_set_status(battle: &mut Battle, status: &Status, target: &Pokemon, _source: Option<&Pokemon>, effect: &Effect) -> AbilityHandlerResult {
        // if (status.id !== 'psn' && status.id !== 'tox') return;
        if status.id != "psn" && status.id != "tox" {
            return AbilityHandlerResult::Undefined;
        }

        // if ((effect as Move)?.status)
        if effect.effect_type == "Move" && effect.status.is_some() {
            // this.add('-immune', target, '[from] ability: Immunity');
            battle.add_log("-immune", &[&format!("{}", target.position), "[from] ability: Immunity"]);
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
    pub fn on_update(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
        // if (pokemon.status === 'par')
        if pokemon.status.as_str() == "par" {
            // this.add('-activate', pokemon, 'ability: Limber');
            battle.add_log("-activate", &[&format!("{}", pokemon.position), "ability: Limber"]);
            // pokemon.cureStatus();
            let pokemon_ref = (pokemon.side_index, pokemon.position);
            battle.cure_status(pokemon_ref);
        }
        AbilityHandlerResult::Undefined
    }

    /// onSetStatus(status, target, source, effect)
    pub fn on_set_status(battle: &mut Battle, status: &Status, target: &Pokemon, _source: Option<&Pokemon>, effect: &Effect) -> AbilityHandlerResult {
        // if (status.id !== 'par') return;
        if status.id != "par" {
            return AbilityHandlerResult::Undefined;
        }

        // if ((effect as Move)?.status)
        if effect.effect_type == "Move" && effect.status.is_some() {
            // this.add('-immune', target, '[from] ability: Limber');
            battle.add_log("-immune", &[&format!("{}", target.position), "[from] ability: Limber"]);
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
        let pokemon = create_mock_pokemon();
        let move_ = Move::default();

        // 40 BP should get boost
        let result = technician::on_base_power(40, &pokemon, &pokemon, &move_);
        assert!(matches!(result, AbilityHandlerResult::ChainModify(6144, 4096)));

        // 80 BP should not get boost
        let result2 = technician::on_base_power(80, &pokemon, &pokemon, &move_);
        assert!(matches!(result2, AbilityHandlerResult::Undefined));
    }

    #[test]
    fn test_guts_boost() {
        let mut pokemon = create_mock_pokemon();

        // No status - no boost
        let result = guts::on_modify_atk(100, &pokemon);
        assert!(matches!(result, AbilityHandlerResult::Undefined));

        // With status - boost
        pokemon.status = ID::new("brn");
        let result2 = guts::on_modify_atk(100, &pokemon);
        assert!(matches!(result2, AbilityHandlerResult::ChainModify(6144, 4096)));
    }

    #[test]
    fn test_immunity_blocks_poison() {
        let mut battle = create_mock_battle();
        let pokemon = create_mock_pokemon();

        let psn_status = Status { id: "psn".to_string() };
        let brn_status = Status { id: "brn".to_string() };
        let effect = Effect::default();

        let result = immunity::on_set_status(&mut battle, &psn_status, &pokemon, None, &effect);
        assert!(matches!(result, AbilityHandlerResult::False));

        let result2 = immunity::on_set_status(&mut battle, &brn_status, &pokemon, None, &effect);
        assert!(matches!(result2, AbilityHandlerResult::Undefined));
    }

    fn create_mock_battle() -> Battle {
        use crate::battle::BattleOptions;
        Battle::new(BattleOptions::default())
    }

    fn create_mock_pokemon() -> Pokemon {
        Pokemon::default()
    }
}
