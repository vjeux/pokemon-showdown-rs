//! Ability Callback Handlers
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module contains the actual callback implementations for each ability,
//! translated from data/abilities.ts in the JavaScript codebase.
//!
//! Handlers receive a mutable reference to Battle and Pokemon indices,
//! calling methods on Battle like add_log(), heal(), damage(), boost(), etc.
//!
//! Note: Only a subset of abilities are implemented here as a proof of concept.
//! The full implementation would be extensive given the 300+ abilities.

use crate::battle::Battle;
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
    /// Return 0 (for onEffectiveness)
    Zero,
    /// Return a number
    Number(i32),
    /// Chain modifier applied (numerator, denominator)
    ChainModify(u32, u32),
    /// Block the action (immunity)
    Immune,
}

// =============================================================================
// HANDLER CONTEXT - information passed to handlers
// =============================================================================

/// Context for ability handlers
#[derive(Debug, Clone)]
pub struct AbilityContext {
    /// The Pokemon with the ability (side_index, pokemon_index)
    pub pokemon: (usize, usize),
    /// The source of the effect (if any) - attacker
    pub source: Option<(usize, usize)>,
    /// The target of the effect (if any)
    pub target: Option<(usize, usize)>,
    /// Move type (if applicable)
    pub move_type: Option<String>,
    /// Move category (if applicable)
    pub move_category: Option<String>,
    /// Move flags (if applicable)
    pub move_flags: MoveFlags,
    /// Move base power (if applicable)
    pub base_power: Option<u32>,
    /// Damage amount (if applicable)
    pub damage: Option<u32>,
    /// Type modifier from move hit data
    pub type_mod: i32,
    /// Effect ID (if applicable)
    pub effect_id: Option<String>,
    /// Effect type (if applicable)
    pub effect_type: Option<String>,
    /// Status being applied (if applicable)
    pub status: Option<String>,
    /// Whether this is a contact move
    pub is_contact: bool,
}

impl Default for AbilityContext {
    fn default() -> Self {
        Self {
            pokemon: (0, 0),
            source: None,
            target: None,
            move_type: None,
            move_category: None,
            move_flags: MoveFlags::default(),
            base_power: None,
            damage: None,
            type_mod: 0,
            effect_id: None,
            effect_type: None,
            status: None,
            is_contact: false,
        }
    }
}

/// Move flags relevant to ability callbacks
#[derive(Debug, Clone, Default)]
pub struct MoveFlags {
    pub bypasssub: bool,
    pub contact: bool,
    pub punch: bool,
    pub bite: bool,
    pub pulse: bool,
    pub sound: bool,
    pub bullet: bool,
}

// =============================================================================
// ABILITY HANDLERS
// =============================================================================

// -----------------------------------------------------------------------------
// INTIMIDATE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// intimidate: {
//     onStart(pokemon) {
//         let dominated = false;
//         for (const target of pokemon.adjacentFoes()) {
//             if (!target.volatiles['substitute']) {
//                 this.boost({atk: -1}, target, pokemon, null, true);
//                 dominated = true;
//             }
//         }
//         if (dominated) this.hint("Intimidate lowered the Attack of all opposing Pokémon by 1 stage.");
//     },
//     name: "Intimidate",
//     rating: 3.5,
// },

pub mod intimidate {
    use super::*;

    /// onStart - Lower foe's Attack on switch-in
    pub fn on_start(battle: &mut Battle, ctx: &AbilityContext) -> AbilityHandlerResult {
        let (side_idx, poke_idx) = ctx.pokemon;
        if let Some(pokemon) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
            let pokemon_pos = pokemon.position;
            battle.add_log("-ability", &[&format!("{}", pokemon_pos), "Intimidate"]);
        }
        // Collect foe indices first to avoid borrow issues
        let foe_side = 1 - side_idx;
        let foe_indices: Vec<usize> = if let Some(foe_side_data) = battle.sides.get(foe_side) {
            foe_side_data.pokemon.iter().enumerate()
                .filter(|(_, foe)| foe.is_active && !foe.fainted)
                .map(|(idx, _)| idx)
                .collect()
        } else {
            vec![]
        };
        // Now apply boosts
        for foe_idx in foe_indices {
            battle.boost(&[("atk", -1)], (foe_side, foe_idx), Some(ctx.pokemon), Some("ability: Intimidate"));
        }
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// LEVITATE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// levitate: {
//     onTryHit(target, source, move) {
//         if (target !== source && move.type === 'Ground') {
//             this.add('-immune', target, '[from] ability: Levitate');
//             return null;
//         }
//     },
//     flags: {breakable: 1},
//     name: "Levitate",
//     rating: 3.5,
// },

pub mod levitate {
    use super::*;

    /// onTryHit - immunity to Ground moves
    pub fn on_try_hit(_battle: &mut Battle, ctx: &AbilityContext) -> AbilityHandlerResult {
        if ctx.move_type.as_deref() == Some("Ground") {
            return AbilityHandlerResult::Immune;
        }
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// FLASH FIRE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// flashfire: {
//     onTryHit(target, source, move) {
//         if (target !== source && move.type === 'Fire') {
//             move.accuracy = true;
//             if (!target.addVolatile('flashfire')) {
//                 this.add('-immune', target, '[from] ability: Flash Fire');
//             }
//             return null;
//         }
//     },
//     onEnd(pokemon) {
//         pokemon.removeVolatile('flashfire');
//     },
//     condition: {
//         noCopy: true,
//         onStart(target, source, effect) {
//             this.add('-start', target, 'ability: Flash Fire');
//         },
//         onModifyAtkPriority: 5,
//         onModifyAtk(atk, attacker, defender, move) {
//             if (move.type === 'Fire' && attacker.hasAbility('flashfire')) {
//                 return this.chainModify(1.5);
//             }
//         },
//         onModifySpAPriority: 5,
//         onModifySpA(atk, attacker, defender, move) {
//             if (move.type === 'Fire' && attacker.hasAbility('flashfire')) {
//                 return this.chainModify(1.5);
//             }
//         },
//         onEnd(target) {
//             this.add('-end', target, 'ability: Flash Fire', '[silent]');
//         },
//     },
//     flags: {breakable: 1},
//     name: "Flash Fire",
//     rating: 3.5,
// },

pub mod flashfire {
    use super::*;

    /// onTryHit - immunity to Fire, set flashfire volatile
    pub fn on_try_hit(battle: &mut Battle, ctx: &AbilityContext) -> AbilityHandlerResult {
        if ctx.move_type.as_deref() == Some("Fire") {
            let (side_idx, poke_idx) = ctx.pokemon;
            let position = battle.sides.get(side_idx)
                .and_then(|s| s.pokemon.get(poke_idx))
                .map(|p| p.position);
            if let Some(pokemon) = battle.sides.get_mut(side_idx).and_then(|s| s.pokemon.get_mut(poke_idx)) {
                if !pokemon.volatiles.contains_key(&ID::new("flashfire")) {
                    pokemon.volatiles.insert(ID::new("flashfire"), Default::default());
                }
            }
            if let Some(pos) = position {
                battle.add_log("-start", &[&format!("{}", pos), "ability: Flash Fire"]);
            }
            return AbilityHandlerResult::Immune;
        }
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// WATER ABSORB
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// waterabsorb: {
//     onTryHit(target, source, move) {
//         if (target !== source && move.type === 'Water') {
//             if (!this.heal(target.baseMaxhp / 4)) {
//                 this.add('-immune', target, '[from] ability: Water Absorb');
//             }
//             return null;
//         }
//     },
//     flags: {breakable: 1},
//     name: "Water Absorb",
//     rating: 3.5,
// },

pub mod waterabsorb {
    use super::*;

    /// onTryHit - immunity to Water, heal 25%
    pub fn on_try_hit(battle: &mut Battle, ctx: &AbilityContext) -> AbilityHandlerResult {
        if ctx.move_type.as_deref() == Some("Water") {
            let (side_idx, poke_idx) = ctx.pokemon;
            if let Some(pokemon) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
                if pokemon.hp < pokemon.maxhp {
                    let amount = pokemon.base_maxhp / 4;
                    battle.heal(amount, ctx.pokemon, None, Some("ability: Water Absorb"));
                }
            }
            return AbilityHandlerResult::Immune;
        }
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// SPEED BOOST
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// speedboost: {
//     onResidualOrder: 28,
//     onResidualSubOrder: 2,
//     onResidual(pokemon) {
//         if (pokemon.activeTurns) {
//             this.boost({spe: 1});
//         }
//     },
//     name: "Speed Boost",
//     rating: 4.5,
// },

pub mod speedboost {
    use super::*;

    /// onResidual - boost Speed by 1 at end of turn
    pub fn on_residual(battle: &mut Battle, ctx: &AbilityContext) -> AbilityHandlerResult {
        let (side_idx, poke_idx) = ctx.pokemon;
        if let Some(pokemon) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
            if pokemon.is_active && !pokemon.fainted {
                battle.boost(&[("spe", 1)], ctx.pokemon, Some(ctx.pokemon), Some("ability: Speed Boost"));
            }
        }
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// DRIZZLE
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// drizzle: {
//     onStart(source) {
//         this.field.setWeather('raindance');
//     },
//     name: "Drizzle",
//     rating: 4,
// },

pub mod drizzle {
    use super::*;

    /// onStart - set Rain weather on switch-in
    pub fn on_start(battle: &mut Battle, ctx: &AbilityContext) -> AbilityHandlerResult {
        let (side_idx, poke_idx) = ctx.pokemon;
        if let Some(pokemon) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
            let pokemon_pos = pokemon.position;
            battle.add_log("-weather", &["RainDance", &format!("[from] ability: Drizzle"), &format!("[of] {}", pokemon_pos)]);
        }
        battle.field.set_weather(ID::new("raindance"), Some(5));
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// TECHNICIAN
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// technician: {
//     onBasePowerPriority: 30,
//     onBasePower(basePower, attacker, defender, move) {
//         const basePowerAfterMultiplier = this.modify(basePower, move.basePowerModifier || 1);
//         if (basePowerAfterMultiplier <= 60) {
//             return this.chainModify(1.5);
//         }
//     },
//     name: "Technician",
//     rating: 3.5,
// },

pub mod technician {
    use super::*;

    /// onBasePower - 1.5x for moves with BP <= 60
    pub fn on_base_power(_battle: &mut Battle, ctx: &AbilityContext) -> AbilityHandlerResult {
        if let Some(bp) = ctx.base_power {
            if bp <= 60 {
                return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
            }
        }
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// GUTS
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// guts: {
//     onModifyAtkPriority: 5,
//     onModifyAtk(atk, pokemon) {
//         if (pokemon.status) {
//             return this.chainModify(1.5);
//         }
//     },
//     name: "Guts",
//     rating: 3.5,
// },

pub mod guts {
    use super::*;

    /// onModifyAtk - 1.5x Attack when statused
    pub fn on_modify_atk(battle: &mut Battle, ctx: &AbilityContext) -> AbilityHandlerResult {
        let (side_idx, poke_idx) = ctx.pokemon;
        if let Some(pokemon) = battle.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
            if !pokemon.status.is_empty() {
                return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
            }
        }
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// ROUGH SKIN
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// roughskin: {
//     onDamagingHitOrder: 1,
//     onDamagingHit(damage, target, source, move) {
//         if (this.checkMoveMakesContact(move, source, target, true)) {
//             this.damage(source.baseMaxhp / 8, source, target);
//         }
//     },
//     name: "Rough Skin",
//     rating: 2.5,
// },

pub mod roughskin {
    use super::*;

    /// onDamagingHit - deal 1/8 damage to attacker on contact
    pub fn on_damaging_hit(battle: &mut Battle, ctx: &AbilityContext) -> AbilityHandlerResult {
        if ctx.is_contact {
            if let Some(source) = ctx.source {
                let (source_side, source_idx) = source;
                if let Some(attacker) = battle.sides.get(source_side).and_then(|s| s.pokemon.get(source_idx)) {
                    let damage = attacker.base_maxhp / 8;
                    battle.damage(damage, source, Some(ctx.pokemon), Some("ability: Rough Skin"));
                }
            }
        }
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// IMMUNITY
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// immunity: {
//     onUpdate(pokemon) {
//         if (pokemon.status === 'psn' || pokemon.status === 'tox') {
//             this.add('-activate', pokemon, 'ability: Immunity');
//             pokemon.cureStatus();
//         }
//     },
//     onSetStatus(status, target, source, effect) {
//         if (status.id !== 'psn' && status.id !== 'tox') return;
//         if ((effect as Move)?.status) {
//             this.add('-immune', target, '[from] ability: Immunity');
//         }
//         return false;
//     },
//     flags: {breakable: 1},
//     name: "Immunity",
//     rating: 2,
// },

pub mod immunity {
    use super::*;

    /// onSetStatus - immune to poison
    pub fn on_set_status(_battle: &mut Battle, ctx: &AbilityContext) -> AbilityHandlerResult {
        let status = ctx.status.as_deref();
        if status == Some("psn") || status == Some("tox") {
            return AbilityHandlerResult::False;
        }
        AbilityHandlerResult::Undefined
    }
}

// -----------------------------------------------------------------------------
// LIMBER
// -----------------------------------------------------------------------------
// JS Source (data/abilities.ts):
// limber: {
//     onUpdate(pokemon) {
//         if (pokemon.status === 'par') {
//             this.add('-activate', pokemon, 'ability: Limber');
//             pokemon.cureStatus();
//         }
//     },
//     onSetStatus(status, target, source, effect) {
//         if (status.id !== 'par') return;
//         if ((effect as Move)?.status) {
//             this.add('-immune', target, '[from] ability: Limber');
//         }
//         return false;
//     },
//     flags: {breakable: 1},
//     name: "Limber",
//     rating: 2,
// },

pub mod limber {
    use super::*;

    /// onSetStatus - immune to paralysis
    pub fn on_set_status(_battle: &mut Battle, ctx: &AbilityContext) -> AbilityHandlerResult {
        if ctx.status.as_deref() == Some("par") {
            return AbilityHandlerResult::False;
        }
        AbilityHandlerResult::Undefined
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
        let mut ctx = AbilityContext::default();
        ctx.base_power = Some(40);
        let result = technician::on_base_power(&mut create_mock_battle(), &ctx);
        assert!(matches!(result, AbilityHandlerResult::ChainModify(6144, 4096)));
        ctx.base_power = Some(80);
        let result2 = technician::on_base_power(&mut create_mock_battle(), &ctx);
        assert!(matches!(result2, AbilityHandlerResult::Undefined));
    }

    #[test]
    fn test_levitate_immunity() {
        let mut ctx = AbilityContext::default();
        ctx.move_type = Some("Ground".to_string());
        let result = levitate::on_try_hit(&mut create_mock_battle(), &ctx);
        assert!(matches!(result, AbilityHandlerResult::Immune));
        ctx.move_type = Some("Fire".to_string());
        let result2 = levitate::on_try_hit(&mut create_mock_battle(), &ctx);
        assert!(matches!(result2, AbilityHandlerResult::Undefined));
    }

    #[test]
    fn test_immunity_blocks_poison() {
        let mut ctx = AbilityContext::default();
        ctx.status = Some("psn".to_string());
        let result = immunity::on_set_status(&mut create_mock_battle(), &ctx);
        assert!(matches!(result, AbilityHandlerResult::False));
        ctx.status = Some("brn".to_string());
        let result2 = immunity::on_set_status(&mut create_mock_battle(), &ctx);
        assert!(matches!(result2, AbilityHandlerResult::Undefined));
    }

    fn create_mock_battle() -> Battle {
        use crate::battle::BattleOptions;
        Battle::new(BattleOptions::default())
    }
}
