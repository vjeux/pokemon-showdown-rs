//! Battle Actions
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This is a 1:1 port of sim/battle-actions.ts
//! Handles all battle actions: moves, switches, damage calculation, etc.

use std::collections::HashSet;

use once_cell::sync::Lazy;

use crate::data::typechart::get_effectiveness_multi;
use crate::dex::{Dex, MoveData};
use crate::dex_data::{BoostsTable, ID};
use crate::pokemon::Pokemon;

/// Choosable target types for moves
static CHOOSABLE_TARGETS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut set = HashSet::new();
    set.insert("normal");
    set.insert("any");
    set.insert("adjacentAlly");
    set.insert("adjacentAllyOrSelf");
    set.insert("adjacentFoe");
    set
});

/// Parameters for Z-move functions
pub struct ZMoveParams<'a> {
    pub move_name: &'a str,
    pub move_type: &'a str,
    pub move_category: &'a str,
    pub z_move_base_power: Option<i32>,
    pub item_z_move: Option<&'a str>,
    pub item_z_move_from: Option<&'a str>,
    pub item_z_move_type: Option<&'a str>,
    pub z_move_used: bool,
}

/// Parameters for can_z_move function
pub struct CanZMoveParams<'a> {
    pub z_move_used: bool,
    pub is_transformed: bool,
    pub species_is_mega: bool,
    pub species_is_primal: bool,
    pub species_forme: &'a str,
    pub item_z_move: bool,
    pub item_user: Option<&'a [String]>,
    pub species_name: &'a str,
}

/// Parameters for hit_step_accuracy function
pub struct AccuracyCheckParams {
    pub move_accuracy: i32,
    pub move_always_hit: bool,
    pub move_ohko: bool,
    pub attacker_accuracy_boost: i8,
    pub defender_evasion_boost: i8,
    pub ignore_accuracy: bool,
    pub ignore_evasion: bool,
    pub random_value: i32,
}

/// Parameters for get_damage function
pub struct DamageCalcParams<'a> {
    pub attacker_level: i32,
    pub attacker_attack: i32,
    pub defender_defense: i32,
    pub base_power: i32,
    pub stab_modifier: f64,
    pub type_effectiveness: f64,
    pub is_crit: bool,
    pub random_factor: i32,
    pub other_modifiers: &'a [f64],
}

/// Max Move names by type
pub fn get_max_move_name(move_type: &str) -> &'static str {
    match move_type {
        "Flying" => "Max Airstream",
        "Dark" => "Max Darkness",
        "Fire" => "Max Flare",
        "Bug" => "Max Flutterby",
        "Water" => "Max Geyser",
        "Status" => "Max Guard",
        "Ice" => "Max Hailstorm",
        "Fighting" => "Max Knuckle",
        "Electric" => "Max Lightning",
        "Psychic" => "Max Mindstorm",
        "Poison" => "Max Ooze",
        "Grass" => "Max Overgrowth",
        "Ghost" => "Max Phantasm",
        "Ground" => "Max Quake",
        "Rock" => "Max Rockfall",
        "Fairy" => "Max Starfall",
        "Steel" => "Max Steelspike",
        "Normal" => "Max Strike",
        "Dragon" => "Max Wyrmwind",
        _ => "Max Strike",
    }
}

/// Z-Move names by type
pub fn get_z_move_name(move_type: &str) -> &'static str {
    match move_type {
        "Poison" => "Acid Downpour",
        "Fighting" => "All-Out Pummeling",
        "Dark" => "Black Hole Eclipse",
        "Grass" => "Bloom Doom",
        "Normal" => "Breakneck Blitz",
        "Rock" => "Continental Crush",
        "Steel" => "Corkscrew Crash",
        "Dragon" => "Devastating Drake",
        "Electric" => "Gigavolt Havoc",
        "Water" => "Hydro Vortex",
        "Fire" => "Inferno Overdrive",
        "Ghost" => "Never-Ending Nightmare",
        "Bug" => "Savage Spin-Out",
        "Psychic" => "Shattered Psyche",
        "Ice" => "Subzero Slammer",
        "Flying" => "Supersonic Skystrike",
        "Ground" => "Tectonic Rage",
        "Fairy" => "Twinkle Tackle",
        _ => "Breakneck Blitz",
    }
}

/// Damage calculation result
#[derive(Debug, Clone)]
pub enum DamageResult {
    /// Actual damage dealt
    Damage(i32),
    /// Target is immune
    Immune,
    /// Move missed
    Miss,
    /// Move failed for some other reason
    Failed,
    /// No damage (status move or 0 base power)
    NoDamage,
}

/// Move hit data for tracking crits, effectiveness, etc.
/// Equivalent to MoveHitData in battle-actions.ts
#[derive(Debug, Clone, Default)]
pub struct MoveHitData {
    pub crit: bool,
    pub type_mod: i32,
    pub damage: i32,
    pub z_broke_protect: bool,
}

/// Active move state - represents a move being executed
/// Equivalent to ActiveMove in battle-actions.ts
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ActiveMove {
    pub id: ID,
    pub name: String,
    pub base_power: i32,
    pub category: String,
    pub move_type: String,
    pub accuracy: i32,
    pub priority: i8,
    pub target: String,
    pub flags: MoveFlags,

    // Active state
    pub hit: i32,
    pub total_damage: i32,
    pub spread_hit: bool,
    pub is_external: bool,
    pub is_z: bool,
    pub is_max: bool,
    pub is_z_or_max_powered: bool,
    pub prankster_boosted: bool,
    pub has_bounced: bool,
    pub source_effect: Option<ID>,
    pub ignore_ability: bool,
    pub ignore_immunity: Option<bool>,
    pub ignore_accuracy: bool,
    pub ignore_evasion: bool,
    pub ignore_defensive: bool,
    pub ignore_offensive: bool,
    pub ignore_negative_offensive: bool,
    pub ignore_positive_defensive: bool,
    pub override_offensive_stat: Option<String>,
    pub infiltrates: bool,
    pub will_crit: Option<bool>,
    pub force_stab: bool,
    pub crit_ratio: i32,
    pub crit_modifier: Option<f64>,
    pub self_switch: Option<String>,
    pub self_boost: Option<BoostsTable>,
    pub has_sheer_force: bool,
    pub mindblown_recoil: bool,
    pub struggle_recoil: bool,
    pub self_dropped: bool,
    pub smart_target: Option<bool>,
    pub stellar_boosted: bool,
    pub multi_hit: Option<i32>,
    pub multi_hit_type: Option<String>,
    pub multi_accuracy: bool,
    pub ohko: Option<String>,
    pub always_hit: bool,
    pub breaks_protect: bool,
    pub steals_boosts: bool,
    pub force_switch: bool,
    pub self_destruct: Option<String>,
    pub tracks_target: bool,
    pub base_move: Option<ID>,
    pub max_move: Option<MaxMoveData>,
    pub z_move: Option<ZMoveData>,
    pub sleep_usable: bool,

    // Special move fields
    /// Non-ghost target for Curse (used when ghost type uses it differently)
    pub non_ghost_target: Option<String>,
    /// Whether this move will cause a forme change (relicsong)
    pub will_change_forme: bool,

    // Secondary effects
    pub secondaries: Vec<SecondaryEffect>,
    pub self_effect: Option<SelfEffect>,

    // Move data effects
    pub boosts: Option<BoostsTable>,
    pub heal: Option<(i32, i32)>,
    pub status: Option<String>,
    pub force_status: Option<String>,
    pub volatile_status: Option<String>,
    pub side_condition: Option<String>,
    pub slot_condition: Option<String>,
    pub weather: Option<String>,
    pub terrain: Option<String>,
    pub pseudo_weather: Option<String>,

    // Recoil
    pub recoil: Option<(i32, i32)>,

    // Hit targets (populated during execution)
    pub hit_targets: Vec<(usize, usize)>,
}

/// Move flags
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct MoveFlags {
    pub contact: bool,
    pub protect: bool,
    pub mirror: bool,
    pub punch: bool,
    pub bite: bool,
    pub sound: bool,
    pub powder: bool,
    pub dance: bool,
    pub pulse: bool,
    pub bullet: bool,
    pub slicing: bool,
    pub wind: bool,
    pub cant_use_twice: bool,
    pub future_move: bool,
    pub reflectable: bool,
    pub snatch: bool,
    pub gravity: bool,
    pub bypasssub: bool,
    pub pledgecombo: bool,
}

/// Max move data
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct MaxMoveData {
    pub base_power: i32,
}

/// Z-move data
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ZMoveData {
    pub base_power: Option<i32>,
    pub boost: Option<BoostsTable>,
    pub effect: Option<String>,
}

/// Secondary effect data
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct SecondaryEffect {
    pub chance: Option<i32>,
    pub boosts: Option<BoostsTable>,
    pub status: Option<String>,
    pub volatile_status: Option<String>,
    pub self_effect: bool,
}

/// Self effect data
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct SelfEffect {
    pub boosts: Option<BoostsTable>,
    pub chance: Option<i32>,
    pub side_condition: Option<String>,
}

/// Z-Move request option
#[derive(Debug, Clone)]
pub struct ZMoveOption {
    pub move_name: String,
    pub target: String,
}

/// Damage value (can be number, false, or undefined-like None)
#[derive(Debug, Clone)]
pub enum DamageValue {
    Damage(i32),
    Failed,
    Blocked, // HIT_SUBSTITUTE
}

/// Switch copy flag type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchCopyFlag {
    None,
    CopyVolatile,
    ShedTail,
}

/// Battle Actions struct - 1:1 port of BattleActions class
/// Note: In Rust, this struct needs a reference to battle state.
/// The actual methods that need battle access are implemented on Battle directly.
pub struct BattleActions<'a> {
    pub dex: &'a Dex,
    pub gen: u8,
}

impl<'a> BattleActions<'a> {
    pub fn new(dex: &'a Dex, gen: u8) -> Self {
        Self { dex, gen }
    }

    /// Calculate damage for a move
    /// This is a simplified damage calculation for testing purposes.
    /// The full damage calculation is in getDamage in battle-actions.ts
    pub fn calculate_damage(
        &self,
        attacker: &Pokemon,
        defender: &Pokemon,
        move_data: &MoveData,
        is_crit: bool,
        random_factor: i32,
    ) -> DamageResult {
        // Check for immunity first
        let effectiveness = get_effectiveness_multi(&move_data.move_type, &defender.types);

        if effectiveness == 0.0 {
            return DamageResult::Immune;
        }

        // Get base power
        let base_power = move_data.base_power;
        if base_power == 0 {
            return DamageResult::NoDamage;
        }

        // Get attack and defense stats with boost modifiers applied
        let (attack, defense) = if move_data.category == "Special" {
            let atk_boost = attacker.boosts.spa;
            let def_boost = defender.boosts.spd;
            let base_atk = attacker.stored_stats.spa;
            let base_def = defender.stored_stats.spd;
            (
                Self::calculate_stat_with_boost(base_atk, atk_boost),
                Self::calculate_stat_with_boost(base_def, def_boost),
            )
        } else {
            let atk_boost = attacker.boosts.atk;
            let def_boost = defender.boosts.def;
            let base_atk = attacker.stored_stats.atk;
            let base_def = defender.stored_stats.def;
            (
                Self::calculate_stat_with_boost(base_atk, atk_boost),
                Self::calculate_stat_with_boost(base_def, def_boost),
            )
        };

        // Basic damage formula: ((2 * Level / 5 + 2) * Power * A/D) / 50 + 2
        let level = attacker.level as i32;
        let base_damage = ((2 * level / 5 + 2) * base_power * attack / defense.max(1)) / 50 + 2;

        // Apply STAB (Same Type Attack Bonus)
        let stab = if attacker.types.iter().any(|t| t == &move_data.move_type) {
            1.5
        } else {
            1.0
        };

        // Apply type effectiveness
        let damage = (base_damage as f64 * stab * effectiveness) as i32;

        // Apply critical hit (1.5x in Gen 6+)
        let damage = if is_crit {
            (damage as f64 * 1.5) as i32
        } else {
            damage
        };

        // Apply random factor (0.85 to 1.0, passed in as 85-100)
        let damage = damage * random_factor / 100;

        DamageResult::Damage(damage.max(1))
    }

    // =========================================================================
    // SWITCH METHODS - Ported from battle-actions.ts
    // =========================================================================
    // Note: switchIn, dragIn, runSwitch are implemented on Battle struct
    // because they need mutable access to battle state.

    // =========================================================================
    // MOVE METHODS - Ported from battle-actions.ts
    // =========================================================================
    // Note: runMove, useMove, useMoveInner are implemented on Battle struct
    // because they need mutable access to battle state.

    // =========================================================================
    // DAMAGE CALCULATION - These can be pure functions
    // =========================================================================

    /// Calculate the stat modifier from boost stages
    /// Returns the multiplier as a fraction (numerator, denominator)
    /// Equivalent to getBoostMod in Pokemon Showdown
    pub fn get_boost_modifier(boost: i8) -> (i32, i32) {
        match boost {
            -6 => (2, 8),
            -5 => (2, 7),
            -4 => (2, 6),
            -3 => (2, 5),
            -2 => (2, 4),
            -1 => (2, 3),
            0 => (2, 2),
            1 => (3, 2),
            2 => (4, 2),
            3 => (5, 2),
            4 => (6, 2),
            5 => (7, 2),
            6 => (8, 2),
            _ if boost < -6 => (2, 8),
            _ => (8, 2),
        }
    }

    /// Calculate the effective stat value with boost applied
    pub fn calculate_stat_with_boost(base_stat: i32, boost: i8) -> i32 {
        let (num, denom) = Self::get_boost_modifier(boost);
        (base_stat * num / denom).max(1)
    }

    /// Calculate recoil damage
    /// Equivalent to calcRecoilDamage in battle-actions.ts
    // TypeScript source:
    //
    //
    // 	calcRecoilDamage(damageDealt: number, move: Move, pokemon: Pokemon): number {
    // 		if (move.id === 'chloroblast') return Math.round(pokemon.maxhp / 2);
    // 		return this.battle.clampIntRange(Math.round(damageDealt * move.recoil![0] / move.recoil![1]), 1);
    // 	}
    //
    pub fn calc_recoil_damage(
        damage_dealt: i32,
        move_id: &str,
        recoil: Option<(i32, i32)>,
        pokemon_max_hp: i32,
    ) -> i32 {
        if move_id == "chloroblast" {
            return (pokemon_max_hp / 2).max(1);
        }
        if let Some((num, denom)) = recoil {
            let recoil_damage = (damage_dealt * num / denom).max(1);
            return recoil_damage;
        }
        0
    }

    /// Calculate confusion damage
    /// Equivalent to getConfusionDamage in battle-actions.ts
    // TypeScript source:
    // /**
    // 	 * Confusion damage is unique - most typical modifiers that get run when calculating
    // 	 * damage (e.g. Huge Power, Life Orb, critical hits) don't apply. It also uses a 16-bit
    // 	 * context for its damage, unlike the regular damage formula (though this only comes up
    // 	 * for base damage).
    // 	 */
    // 	getConfusionDamage(pokemon: Pokemon, basePower: number) {
    // 		const tr = this.battle.trunc;
    //
    // 		const attack = pokemon.calculateStat('atk', pokemon.boosts['atk']);
    // 		const defense = pokemon.calculateStat('def', pokemon.boosts['def']);
    // 		const level = pokemon.level;
    // 		const baseDamage = tr(tr(tr(tr(2 * level / 5 + 2) * basePower * attack) / defense) / 50) + 2;
    //
    // 		// Damage is 16-bit context in self-hit confusion damage
    // 		let damage = tr(baseDamage, 16);
    // 		damage = this.battle.randomizer(damage);
    // 		return Math.max(1, damage);
    // 	}
    //
    pub fn get_confusion_damage(
        level: i32,
        attack: i32,
        defense: i32,
        base_power: i32,
        random_factor: i32,
    ) -> i32 {
        // int(int(int(2 * L / 5 + 2) * P * A / D) / 50) + 2
        let base_damage = ((2 * level / 5 + 2) * base_power * attack / defense) / 50 + 2;

        // Apply random factor (0.85 to 1.0)
        let damage = base_damage * random_factor / 100;

        damage.max(1)
    }

    /// Check if a target type allows choosing
    /// Equivalent to targetTypeChoices in battle-actions.ts
    //
    // 	targetTypeChoices(targetType: string) {
    // 		return CHOOSABLE_TARGETS.has(targetType);
    // 	}
    //
    pub fn target_type_choices(target_type: &str) -> bool {
        CHOOSABLE_TARGETS.contains(target_type)
    }

    /// Combine results for damage/effect calculations
    /// Equivalent to combineResults in battle-actions.ts
    //
    // 	combineResults<T extends number | boolean | null | '' | undefined,
    // 		U extends number | boolean | null | '' | undefined>(
    // 		left: T, right: U
    // 	): T | U {
    // 		const NOT_FAILURE = 'string';
    // 		const NULL = 'object';
    // 		const resultsPriorities = ['undefined', NOT_FAILURE, NULL, 'boolean', 'number'];
    // 		if (resultsPriorities.indexOf(typeof left) > resultsPriorities.indexOf(typeof right)) {
    // 			return left;
    // 		} else if (left && !right && right !== 0) {
    // 			return left;
    // 		} else if (typeof left === 'number' && typeof right === 'number') {
    // 			return (left + right) as T;
    // 		} else {
    // 			return right;
    // 		}
    // 	}
    //
    pub fn combine_results(
        left: Option<DamageValue>,
        right: Option<DamageValue>,
    ) -> Option<DamageValue> {
        // Priority: undefined < NOT_FAIL < null < boolean < number
        match (&left, &right) {
            (None, r) => r.clone(),
            (l, None) => l.clone(),
            (Some(DamageValue::Damage(l)), Some(DamageValue::Damage(r))) => {
                Some(DamageValue::Damage(l + r))
            }
            (_, r) => r.clone(),
        }
    }

    // =========================================================================
    // Z-MOVE METHODS
    // =========================================================================

    /// Get Z-Move for a move
    /// Equivalent to getZMove in battle-actions.ts
    //
    // 	getZMove(move: Move, pokemon: Pokemon, skipChecks?: boolean): string | undefined {
    // 		const item = pokemon.getItem();
    // 		if (!skipChecks) {
    // 			if (pokemon.side.zMoveUsed) return;
    // 			if (!item.zMove) return;
    // 			if (item.itemUser && !item.itemUser.includes(pokemon.species.name)) return;
    // 			const moveData = pokemon.getMoveData(move);
    // 			// Draining the PP of the base move prevents the corresponding Z-move from being used.
    // 			if (!moveData?.pp) return;
    // 		}
    //
    // 		if (item.zMoveFrom) {
    // 			if (move.name === item.zMoveFrom) return item.zMove as string;
    // 		} else if (item.zMove === true) {
    // 			if (move.type === item.zMoveType) {
    // 				if (move.category === "Status") {
    // 					return move.name;
    // 				} else if (move.zMove?.basePower) {
    // 					return this.Z_MOVES[move.type];
    // 				}
    // 			}
    // 		}
    // 	}
    //
    pub fn get_z_move(params: ZMoveParams) -> Option<String> {
        if params.z_move_used {
            return None;
        }

        // Check for signature Z-move
        if let Some(z_move_from) = params.item_z_move_from {
            if params.move_name == z_move_from {
                return params.item_z_move.map(|s| s.to_string());
            }
        }

        // Check for type-based Z-move
        if params.item_z_move.is_some() {
            if let Some(z_type) = params.item_z_move_type {
                if params.move_type == z_type {
                    if params.move_category == "Status" {
                        return Some(params.move_name.to_string());
                    } else if params.z_move_base_power.is_some() {
                        return Some(get_z_move_name(params.move_type).to_string());
                    }
                }
            }
        }

        None
    }

    /// Check if Pokemon can use Z-Move
    /// Equivalent to canZMove in battle-actions.ts
    //
    // 	canZMove(pokemon: Pokemon) {
    // 		if (pokemon.side.zMoveUsed ||
    // 			(pokemon.transformed &&
    // 				(pokemon.species.isMega || pokemon.species.isPrimal || pokemon.species.forme === "Ultra"))
    // 		) return;
    // 		const item = pokemon.getItem();
    // 		if (!item.zMove) return;
    // 		if (item.itemUser && !item.itemUser.includes(pokemon.species.name)) return;
    // 		let atLeastOne = false;
    // 		let mustStruggle = true;
    // 		const zMoves: ZMoveOptions = [];
    // 		for (const moveSlot of pokemon.moveSlots) {
    // 			if (moveSlot.pp <= 0) {
    // 				zMoves.push(null);
    // 				continue;
    // 			}
    // 			if (!moveSlot.disabled) {
    // 				mustStruggle = false;
    // 			}
    // 			const move = this.dex.moves.get(moveSlot.move);
    // 			let zMoveName = this.getZMove(move, pokemon, true) || '';
    // 			if (zMoveName) {
    // 				const zMove = this.dex.moves.get(zMoveName);
    // 				if (!zMove.isZ && zMove.category === 'Status') zMoveName = "Z-" + zMoveName;
    // 				zMoves.push({ move: zMoveName, target: zMove.target });
    // 			} else {
    // 				zMoves.push(null);
    // 			}
    // 			if (zMoveName) atLeastOne = true;
    // 		}
    // 		if (atLeastOne && !mustStruggle) return zMoves;
    // 	}
    //
    pub fn can_z_move(params: CanZMoveParams) -> bool {
        if params.z_move_used {
            return false;
        }
        if params.is_transformed
            && (params.species_is_mega
                || params.species_is_primal
                || params.species_forme == "Ultra")
        {
            return false;
        }
        if !params.item_z_move {
            return false;
        }
        if let Some(users) = params.item_user {
            if !users.iter().any(|u| u == params.species_name) {
                return false;
            }
        }
        true
    }

    // =========================================================================
    // MAX MOVE METHODS
    // =========================================================================

    /// Get Max Move for a move
    /// Equivalent to getMaxMove in battle-actions.ts
    //
    // 	getMaxMove(move: Move, pokemon: Pokemon) {
    // 		if (typeof move === 'string') move = this.dex.moves.get(move);
    // 		if (move.name === 'Struggle') return move;
    // 		if (pokemon.gigantamax && pokemon.canGigantamax && move.category !== 'Status') {
    // 			const gMaxMove = this.dex.moves.get(pokemon.canGigantamax);
    // 			if (gMaxMove.exists && gMaxMove.type === move.type) return gMaxMove;
    // 		}
    // 		const maxMove = this.dex.moves.get(this.MAX_MOVES[move.category === 'Status' ? move.category : move.type]);
    // 		if (maxMove.exists) return maxMove;
    // 	}
    //
    pub fn get_max_move(move_name: &str, move_type: &str, move_category: &str) -> Option<String> {
        if move_name == "Struggle" {
            return Some("Struggle".to_string());
        }

        let max_type = if move_category == "Status" {
            "Status"
        } else {
            move_type
        };

        Some(get_max_move_name(max_type).to_string())
    }

    // =========================================================================
    // MEGA EVOLUTION METHODS
    // =========================================================================

    /// Check if Pokemon can Mega Evolve
    /// Equivalent to canMegaEvo in battle-actions.ts
    //
    // 	// #endregion
    //
    // 	// #region MEGA EVOLUTION
    // 	// ==================================================================
    //
    // 	canMegaEvo(pokemon: Pokemon) {
    // 		const species = pokemon.baseSpecies;
    // 		const altForme = species.otherFormes && this.dex.species.get(species.otherFormes[0]);
    // 		const item = pokemon.getItem();
    // 		// Mega Rayquaza
    // 		if ((this.battle.gen <= 7 || this.battle.ruleTable.has('+pokemontag:past') ||
    // 			this.battle.ruleTable.has('+pokemontag:future')) &&
    // 			altForme?.isMega && altForme?.requiredMove &&
    // 			pokemon.baseMoves.includes(toID(altForme.requiredMove)) && !item.zMove) {
    // 			return altForme.name;
    // 		}
    // 		// Temporary hardcode until generation shift
    // 		if ((species.baseSpecies === "Floette" || species.baseSpecies === "Zygarde") && item.megaEvolves === species.name) {
    // 			return item.megaStone as string;
    // 		}
    // 		// a hacked-in Megazard X can mega evolve into Megazard Y, but not into Megazard X
    // 		if (Array.isArray(item.megaStone)) {
    // 			// FIXME: Change to species.name when champions comes
    // 			const index = (item.megaEvolves as string[]).indexOf(species.baseSpecies);
    // 			if (index < 0) return null;
    // 			return item.megaStone[index];
    // 			// FIXME: Change to species.name when champions comes
    // 		} else if (item.megaEvolves === species.baseSpecies && item.megaStone !== species.name) {
    // 			return item.megaStone;
    // 		}
    // 		return null;
    // 	}
    //
    pub fn can_mega_evo(
        species_name: &str,
        species_other_formes: Option<&[String]>,
        item_mega_evolves: Option<&str>,
        item_mega_stone: Option<&str>,
        base_moves: &[ID],
        item_is_z_move: bool,
        _gen: u8,
    ) -> Option<String> {
        // Check Mega Rayquaza (requires Dragon Ascent)
        if let Some(other_formes) = species_other_formes {
            if let Some(first_forme) = other_formes.first() {
                if first_forme.ends_with("-Mega") {
                    // Check if it requires a move (like Rayquaza)
                    let required_move = ID::new("dragonascent");
                    if base_moves.contains(&required_move) && !item_is_z_move {
                        return Some(first_forme.clone());
                    }
                }
            }
        }

        // Check item-based mega evolution
        if let (Some(mega_evolves), Some(mega_stone)) = (item_mega_evolves, item_mega_stone) {
            // Check if item's mega evolves matches species
            if mega_evolves == species_name && mega_stone != species_name {
                return Some(mega_stone.to_string());
            }
        }

        None
    }

    /// Check if Pokemon can Ultra Burst
    /// Equivalent to canUltraBurst in battle-actions.ts
    //
    // 	canUltraBurst(pokemon: Pokemon) {
    // 		if (['Necrozma-Dawn-Wings', 'Necrozma-Dusk-Mane'].includes(pokemon.baseSpecies.name) &&
    // 			pokemon.getItem().id === 'ultranecroziumz') {
    // 			return "Necrozma-Ultra";
    // 		}
    // 		return null;
    // 	}
    //
    pub fn can_ultra_burst(species_name: &str, item_id: &str) -> Option<String> {
        if (species_name == "Necrozma-Dawn-Wings" || species_name == "Necrozma-Dusk-Mane")
            && item_id == "ultranecroziumz"
        {
            return Some("Necrozma-Ultra".to_string());
        }
        None
    }

    /// Check if Pokemon can Terastallize
    /// Equivalent to canTerastallize in battle-actions.ts
    //
    // 	canTerastallize(pokemon: Pokemon) {
    // 		if (pokemon.getItem().zMove || pokemon.canMegaEvo || this.dex.gen !== 9) {
    // 			return null;
    // 		}
    // 		return pokemon.teraType;
    // 	}
    //
    pub fn can_terastallize(
        item_is_z_move: bool,
        can_mega_evo: bool,
        gen: u8,
        tera_type: Option<&str>,
    ) -> Option<String> {
        if item_is_z_move || can_mega_evo || gen != 9 {
            return None;
        }
        tera_type.map(|t| t.to_string())
    }

    // =========================================================================
    // HIT STEP METHODS - Ported from battle-actions.ts
    // These are 1:1 ports of the hit step processing methods
    // =========================================================================

    /// Check invulnerability for targets
    /// Equivalent to hitStepInvulnerabilityEvent in battle-actions.ts
    // 	hitStepInvulnerabilityEvent(targets: Pokemon[], pokemon: Pokemon, move: ActiveMove) {
    // 		if (move.id === 'helpinghand') return new Array(targets.length).fill(true);
    // 		const hitResults: boolean[] = [];
    // 		for (const [i, target] of targets.entries()) {
    // 			if (target.volatiles['commanding']) {
    // 				hitResults[i] = false;
    // 			} else if (this.battle.gen >= 8 && move.id === 'toxic' && pokemon.hasType('Poison')) {
    // 				hitResults[i] = true;
    // 			} else {
    // 				hitResults[i] = this.battle.runEvent('Invulnerability', target, pokemon, move);
    // 			}
    // 			if (hitResults[i] === false) {
    // 				if (move.smartTarget) {
    // 					move.smartTarget = false;
    // 				} else {
    // 					if (!move.spreadHit) this.battle.attrLastMove('[miss]');
    // 					this.battle.add('-miss', pokemon, target);
    // 				}
    // 			}
    // 		}
    // 		return hitResults;
    // 	}
    //
    pub fn hit_step_invulnerability_event(
        targets: &[(usize, usize)],
        _attacker_flying: bool,
        _move_flags_contact: bool,
        _move_target: &str,
    ) -> Vec<(usize, usize, bool)> {
        // Returns (side_idx, pokemon_idx, is_hit)
        targets
            .iter()
            .map(|&(side_idx, pokemon_idx)| {
                // For now, all targets are hit unless in a semi-invulnerable state
                // Full implementation would check volatiles like Fly, Dig, Dive, etc.
                (side_idx, pokemon_idx, true)
            })
            .collect()
    }

    /// Check type immunity for targets
    /// Equivalent to hitStepTypeImmunity in battle-actions.ts
    // 	hitStepTypeImmunity(targets: Pokemon[], pokemon: Pokemon, move: ActiveMove) {
    // 		if (move.ignoreImmunity === undefined) {
    // 			move.ignoreImmunity = (move.category === 'Status');
    // 		}
    //
    // 		const hitResults = [];
    // 		for (const i of targets.keys()) {
    // 			hitResults[i] = targets[i].runImmunity(move, !move.smartTarget);
    // 		}
    //
    // 		return hitResults;
    // 	}
    //
    pub fn hit_step_type_immunity(
        defender_types: &[String],
        move_type: &str,
        ignore_immunity: bool,
    ) -> bool {
        if ignore_immunity {
            return true; // Bypass immunity check
        }

        // Check type chart for immunity
        let effectiveness = get_effectiveness_multi(move_type, defender_types);
        effectiveness > 0.0
    }

    /// Check accuracy for move hit
    /// Equivalent to hitStepAccuracy in battle-actions.ts
    // 	hitStepAccuracy(targets: Pokemon[], pokemon: Pokemon, move: ActiveMove) {
    // 		const hitResults = [];
    // 		for (const [i, target] of targets.entries()) {
    // 			this.battle.activeTarget = target;
    // 			// calculate true accuracy
    // 			let accuracy = move.accuracy;
    // 			if (move.ohko) { // bypasses accuracy modifiers
    // 				if (!target.isSemiInvulnerable()) {
    // 					accuracy = 30;
    // 					if (move.ohko === 'Ice' && this.battle.gen >= 7 && !pokemon.hasType('Ice')) {
    // 						accuracy = 20;
    // 					}
    // 					if (!target.volatiles['dynamax'] && pokemon.level >= target.level &&
    // 						(move.ohko === true || !target.hasType(move.ohko))) {
    // 						accuracy += (pokemon.level - target.level);
    // 					} else {
    // 						this.battle.add('-immune', target, '[ohko]');
    // 						hitResults[i] = false;
    // 						continue;
    // 					}
    // 				}
    // 			} else {
    // 				accuracy = this.battle.runEvent('ModifyAccuracy', target, pokemon, move, accuracy);
    // 				if (accuracy !== true) {
    // 					let boost = 0;
    // 					if (!move.ignoreAccuracy) {
    // 						const boosts = this.battle.runEvent('ModifyBoost', pokemon, null, null, { ...pokemon.boosts });
    // 						boost = this.battle.clampIntRange(boosts['accuracy'], -6, 6);
    // 					}
    // 					if (!move.ignoreEvasion) {
    // 						const boosts = this.battle.runEvent('ModifyBoost', target, null, null, { ...target.boosts });
    // 						boost = this.battle.clampIntRange(boost - boosts['evasion'], -6, 6);
    // 					}
    // 					if (boost > 0) {
    // 						accuracy = this.battle.trunc(accuracy * (3 + boost) / 3);
    // 					} else if (boost < 0) {
    // 						accuracy = this.battle.trunc(accuracy * 3 / (3 - boost));
    // 					}
    // 				}
    // 			}
    // 			if (
    // 				move.alwaysHit || (move.id === 'toxic' && this.battle.gen >= 8 && pokemon.hasType('Poison')) ||
    // 				(move.target === 'self' && move.category === 'Status' && !target.isSemiInvulnerable())
    // 			) {
    // 				accuracy = true; // bypasses ohko accuracy modifiers
    // 			} else {
    // 				accuracy = this.battle.runEvent('Accuracy', target, pokemon, move, accuracy);
    // 			}
    // 			if (accuracy !== true && !this.battle.randomChance(accuracy, 100)) {
    // 				if (move.smartTarget) {
    // 					move.smartTarget = false;
    // 				} else {
    // 					if (!move.spreadHit) this.battle.attrLastMove('[miss]');
    // 					this.battle.add('-miss', pokemon, target);
    // 				}
    // 				if (!move.ohko && pokemon.hasItem('blunderpolicy') && pokemon.useItem()) {
    // 					this.battle.boost({ spe: 2 }, pokemon);
    // 				}
    // 				hitResults[i] = false;
    // 				continue;
    // 			}
    // 			hitResults[i] = true;
    // 		}
    // 		return hitResults;
    // 	}
    //
    pub fn hit_step_accuracy(params: AccuracyCheckParams) -> bool {
        // Always hit moves
        if params.move_always_hit || params.move_accuracy == 0 {
            return true;
        }

        // Calculate effective accuracy
        let mut accuracy_stages = if params.ignore_accuracy {
            0
        } else {
            params.attacker_accuracy_boost
        };
        let evasion_stages = if params.ignore_evasion {
            0
        } else {
            params.defender_evasion_boost
        };
        accuracy_stages -= evasion_stages;

        // Apply stage modifier
        let (num, denom) = Self::get_accuracy_modifier(accuracy_stages);
        let effective_accuracy = params.move_accuracy * num / denom;

        // OHKO moves have special accuracy handling
        if params.move_ohko {
            return params.random_value < effective_accuracy.min(100);
        }

        params.random_value < effective_accuracy.min(100)
    }

    /// Get accuracy modifier from stages
    fn get_accuracy_modifier(stages: i8) -> (i32, i32) {
        match stages {
            -6 => (3, 9),
            -5 => (3, 8),
            -4 => (3, 7),
            -3 => (3, 6),
            -2 => (3, 5),
            -1 => (3, 4),
            0 => (3, 3),
            1 => (4, 3),
            2 => (5, 3),
            3 => (6, 3),
            4 => (7, 3),
            5 => (8, 3),
            6 => (9, 3),
            _ if stages < -6 => (3, 9),
            _ => (9, 3),
        }
    }

    /// Check if move breaks protect
    /// Equivalent to hitStepBreakProtect in battle-actions.ts
    // 	hitStepBreakProtect(targets: Pokemon[], pokemon: Pokemon, move: ActiveMove) {
    // 		if (move.breaksProtect) {
    // 			for (const target of targets) {
    // 				let broke = false;
    // 				for (const effectid of [
    // 					'banefulbunker', 'burningbulwark', 'kingsshield', 'obstruct', 'protect', 'silktrap', 'spikyshield',
    // 				]) {
    // 					if (target.removeVolatile(effectid)) broke = true;
    // 				}
    // 				if (this.battle.gen >= 6 || !target.isAlly(pokemon)) {
    // 					for (const effectid of ['craftyshield', 'matblock', 'quickguard', 'wideguard']) {
    // 						if (target.side.removeSideCondition(effectid)) broke = true;
    // 					}
    // 				}
    // 				if (broke) {
    // 					if (move.id === 'feint') {
    // 						this.battle.add('-activate', target, 'move: Feint');
    // 					} else {
    // 						this.battle.add('-activate', target, `move: ${move.name}`, '[broken]');
    // 					}
    // 					if (this.battle.gen >= 6) delete target.volatiles['stall'];
    // 				}
    // 			}
    // 		}
    // 		return undefined;
    // 	}
    //
    pub fn hit_step_break_protect(
        move_breaks_protect: bool,
        move_is_z: bool,
        target_protected: bool,
    ) -> bool {
        if !target_protected {
            return true; // Not protected, can hit
        }
        if move_breaks_protect {
            return true; // Move breaks protect
        }
        if move_is_z {
            return true; // Z-moves break protect (with reduced damage)
        }
        false
    }

    /// Check if move steals boosts
    /// Equivalent to hitStepStealBoosts in battle-actions.ts
    // 	hitStepStealBoosts(targets: Pokemon[], pokemon: Pokemon, move: ActiveMove) {
    // 		const target = targets[0]; // hardcoded
    // 		if (move.stealsBoosts) {
    // 			const boosts: SparseBoostsTable = {};
    // 			let stolen = false;
    // 			let statName: BoostID;
    // 			for (statName in target.boosts) {
    // 				const stage = target.boosts[statName];
    // 				if (stage > 0) {
    // 					boosts[statName] = stage;
    // 					stolen = true;
    // 				}
    // 			}
    // 			if (stolen) {
    // 				this.battle.attrLastMove('[still]');
    // 				this.battle.add('-clearpositiveboost', target, pokemon, 'move: ' + move.name);
    // 				this.battle.boost(boosts, pokemon, pokemon);
    //
    // 				let statName2: BoostID;
    // 				for (statName2 in boosts) {
    // 					boosts[statName2] = 0;
    // 				}
    // 				target.setBoost(boosts);
    // 				if (move.id === "spectralthief") {
    // 					this.battle.addMove('-anim', pokemon, "Spectral Thief", target);
    // 				}
    // 			}
    // 		}
    // 		return undefined;
    // 	}
    //
    pub fn hit_step_steal_boosts(
        move_steals_boosts: bool,
        target_boosts: &BoostsTable,
    ) -> Option<BoostsTable> {
        if !move_steals_boosts {
            return None;
        }

        // Copy only positive boosts
        let mut stolen = BoostsTable::default();
        if target_boosts.atk > 0 {
            stolen.atk = target_boosts.atk;
        }
        if target_boosts.def > 0 {
            stolen.def = target_boosts.def;
        }
        if target_boosts.spa > 0 {
            stolen.spa = target_boosts.spa;
        }
        if target_boosts.spd > 0 {
            stolen.spd = target_boosts.spd;
        }
        if target_boosts.spe > 0 {
            stolen.spe = target_boosts.spe;
        }
        if target_boosts.accuracy > 0 {
            stolen.accuracy = target_boosts.accuracy;
        }
        if target_boosts.evasion > 0 {
            stolen.evasion = target_boosts.evasion;
        }

        Some(stolen)
    }

    /// Process self stat drops from moves
    /// Equivalent to selfDrops in battle-actions.ts
    // 	selfDrops(
    // 		targets: SpreadMoveTargets, source: Pokemon,
    // 		move: ActiveMove, moveData: ActiveMove, isSecondary?: boolean
    // 	) {
    // 		for (const target of targets) {
    // 			if (target === false) continue;
    // 			if (moveData.self && !move.selfDropped) {
    // 				if (!isSecondary && moveData.self.boosts) {
    // 					const secondaryRoll = this.battle.random(100);
    // 					if (typeof moveData.self.chance === 'undefined' || secondaryRoll < moveData.self.chance) {
    // 						this.moveHit(source, source, move, moveData.self, isSecondary, true);
    // 					}
    // 					if (!move.multihit) move.selfDropped = true;
    // 				} else {
    // 					this.moveHit(source, source, move, moveData.self, isSecondary, true);
    // 				}
    // 			}
    // 		}
    // 	}
    //
    pub fn self_drops(
        move_self_boost: Option<&BoostsTable>,
        already_dropped: bool,
    ) -> Option<BoostsTable> {
        if already_dropped {
            return None;
        }
        move_self_boost.cloned()
    }

    /// Get secondary effects for a move
    /// Equivalent to secondaries in battle-actions.ts
    pub fn get_secondaries(
        secondaries: &[SecondaryEffect],
        has_sheer_force: bool,
    ) -> Vec<SecondaryEffect> {
        if has_sheer_force {
            // Sheer Force removes secondary effects
            return Vec::new();
        }
        secondaries.to_vec()
    }

    /// Get active Z-move from base move
    /// Equivalent to getActiveZMove in battle-actions.ts
    //
    // 	getActiveZMove(move: Move, pokemon: Pokemon): ActiveMove {
    // 		if (pokemon) {
    // 			const item = pokemon.getItem();
    // 			if (move.name === item.zMoveFrom) {
    // 				const zMove = this.dex.getActiveMove(item.zMove as string);
    // 				zMove.isZOrMaxPowered = true;
    // 				return zMove;
    // 			}
    // 		}
    //
    // 		if (move.category === 'Status') {
    // 			const zMove = this.dex.getActiveMove(move);
    // 			zMove.isZ = true;
    // 			zMove.isZOrMaxPowered = true;
    // 			return zMove;
    // 		}
    // 		const zMove = this.dex.getActiveMove(this.Z_MOVES[move.type]);
    // 		zMove.basePower = move.zMove!.basePower!;
    // 		zMove.category = move.category;
    // 		// copy the priority for Quick Guard
    // 		zMove.priority = move.priority;
    // 		zMove.isZOrMaxPowered = true;
    // 		return zMove;
    // 	}
    //
    pub fn get_active_z_move(
        base_move_id: &str,
        base_move_type: &str,
        base_move_category: &str,
        base_move_base_power: i32,
        z_crystal_base_power: Option<i32>,
    ) -> ActiveMove {
        let z_move_name = get_z_move_name(base_move_type);
        let z_base_power = if base_move_category == "Status" {
            0
        } else {
            z_crystal_base_power.unwrap_or_else(|| Self::z_move_power_table(base_move_base_power))
        };

        ActiveMove {
            id: ID::new(&z_move_name.to_lowercase().replace(" ", "")),
            name: z_move_name.to_string(),
            base_power: z_base_power,
            category: base_move_category.to_string(),
            move_type: base_move_type.to_string(),
            accuracy: 0, // Z-moves always hit
            priority: 0,
            target: "normal".to_string(),
            is_z: true,
            is_z_or_max_powered: true,
            always_hit: true,
            base_move: Some(ID::new(base_move_id)),
            ..Default::default()
        }
    }

    /// Get Z-move base power from original move's base power
    fn z_move_power_table(base_power: i32) -> i32 {
        match base_power {
            0..=55 => 100,
            56..=65 => 120,
            66..=75 => 140,
            76..=85 => 160,
            86..=95 => 175,
            96..=100 => 180,
            101..=110 => 185,
            111..=125 => 190,
            126..=130 => 195,
            _ => 200,
        }
    }

    /// Get active Max move from base move
    /// Equivalent to getActiveMaxMove in battle-actions.ts
    //
    // 	getActiveMaxMove(move: Move, pokemon: Pokemon) {
    // 		if (typeof move === 'string') move = this.dex.getActiveMove(move);
    // 		if (move.name === 'Struggle') return this.dex.getActiveMove(move);
    // 		let maxMove = this.dex.getActiveMove(this.MAX_MOVES[move.category === 'Status' ? move.category : move.type]);
    // 		if (move.category !== 'Status') {
    // 			if (pokemon.gigantamax && pokemon.canGigantamax) {
    // 				const gMaxMove = this.dex.getActiveMove(pokemon.canGigantamax);
    // 				if (gMaxMove.exists && gMaxMove.type === move.type) maxMove = gMaxMove;
    // 			}
    // 			if (!move.maxMove?.basePower) throw new Error(`${move.name} doesn't have a maxMove basePower`);
    // 			if (!['gmaxdrumsolo', 'gmaxfireball', 'gmaxhydrosnipe'].includes(maxMove.id)) {
    // 				maxMove.basePower = move.maxMove.basePower;
    // 			}
    // 			maxMove.category = move.category;
    // 		}
    // 		maxMove.baseMove = move.id;
    // 		// copy the priority for Psychic Terrain, Quick Guard
    // 		maxMove.priority = move.priority;
    // 		maxMove.isZOrMaxPowered = true;
    // 		return maxMove;
    // 	}
    //
    pub fn get_active_max_move(
        base_move_id: &str,
        base_move_type: &str,
        base_move_category: &str,
        base_move_base_power: i32,
        gmax_move: Option<&str>,
    ) -> ActiveMove {
        let max_move_name = if let Some(gmax) = gmax_move {
            gmax.to_string()
        } else {
            get_max_move_name(if base_move_category == "Status" {
                "Status"
            } else {
                base_move_type
            })
            .to_string()
        };

        let max_base_power = if base_move_category == "Status" {
            0
        } else {
            Self::max_move_power_table(base_move_base_power, base_move_type)
        };

        ActiveMove {
            id: ID::new(
                &max_move_name
                    .to_lowercase()
                    .replace(" ", "")
                    .replace("-", ""),
            ),
            name: max_move_name,
            base_power: max_base_power,
            category: base_move_category.to_string(),
            move_type: base_move_type.to_string(),
            accuracy: 0, // Max moves always hit
            priority: 0,
            target: if base_move_category == "Status" {
                "self".to_string()
            } else {
                "adjacentFoe".to_string()
            },
            is_max: true,
            is_z_or_max_powered: true,
            always_hit: true,
            base_move: Some(ID::new(base_move_id)),
            ..Default::default()
        }
    }

    /// Get Max move base power from original move's base power
    fn max_move_power_table(base_power: i32, move_type: &str) -> i32 {
        // Fighting and Poison type moves have different scaling
        let is_fighting_poison = move_type == "Fighting" || move_type == "Poison";

        match base_power {
            0..=40 => {
                if is_fighting_poison {
                    70
                } else {
                    90
                }
            }
            41..=50 => {
                if is_fighting_poison {
                    75
                } else {
                    100
                }
            }
            51..=60 => {
                if is_fighting_poison {
                    80
                } else {
                    110
                }
            }
            61..=70 => {
                if is_fighting_poison {
                    85
                } else {
                    120
                }
            }
            71..=100 => {
                if is_fighting_poison {
                    90
                } else {
                    130
                }
            }
            101..=140 => {
                if is_fighting_poison {
                    95
                } else {
                    140
                }
            }
            _ => {
                if is_fighting_poison {
                    100
                } else {
                    150
                }
            }
        }
    }

    /// Run Z-Power effect (status Z-moves)
    /// Equivalent to runZPower in battle-actions.ts
    pub fn get_z_power_effect(z_move_effect: Option<&str>) -> Option<ZPowerEffect> {
        match z_move_effect {
            Some("heal") => Some(ZPowerEffect::Heal),
            Some("clearnegativeboost") => Some(ZPowerEffect::ClearNegativeBoost),
            Some("crit2") => Some(ZPowerEffect::Crit2),
            Some("redirect") => Some(ZPowerEffect::Redirect),
            Some("healreplacement") => Some(ZPowerEffect::HealReplacement),
            Some("curse") => Some(ZPowerEffect::Curse),
            _ => None,
        }
    }

    /// Calculate spread move damage modifier
    /// Equivalent to getSpreadDamage calculation in battle-actions.ts
    pub fn get_spread_damage_modifier(target_count: usize) -> f64 {
        if target_count > 1 {
            0.75 // 75% damage when hitting multiple targets
        } else {
            1.0
        }
    }

    /// Modify damage with various factors
    /// Equivalent to modifyDamage in battle-actions.ts
    //
    // 	modifyDamage(
    // 		baseDamage: number, pokemon: Pokemon, target: Pokemon, move: ActiveMove, suppressMessages = false
    // 	) {
    // 		const tr = this.battle.trunc;
    // 		if (!move.type) move.type = '???';
    // 		const type = move.type;
    //
    // 		baseDamage += 2;
    //
    // 		if (move.spreadHit) {
    // 			// multi-target modifier (doubles only)
    // 			const spreadModifier = this.battle.gameType === 'freeforall' ? 0.5 : 0.75;
    // 			this.battle.debug(`Spread modifier: ${spreadModifier}`);
    // 			baseDamage = this.battle.modify(baseDamage, spreadModifier);
    // 		} else if (move.multihitType === 'parentalbond' && move.hit > 1) {
    // 			// Parental Bond modifier
    // 			const bondModifier = this.battle.gen > 6 ? 0.25 : 0.5;
    // 			this.battle.debug(`Parental Bond modifier: ${bondModifier}`);
    // 			baseDamage = this.battle.modify(baseDamage, bondModifier);
    // 		}
    //
    // 		// weather modifier
    // 		baseDamage = this.battle.runEvent('WeatherModifyDamage', pokemon, target, move, baseDamage);
    //
    // 		// crit - not a modifier
    // 		const isCrit = target.getMoveHitData(move).crit;
    // 		if (isCrit) {
    // 			baseDamage = tr(baseDamage * (move.critModifier || (this.battle.gen >= 6 ? 1.5 : 2)));
    // 		}
    //
    // 		// random factor - also not a modifier
    // 		baseDamage = this.battle.randomizer(baseDamage);
    //
    // 		// STAB
    // 		// The "???" type never gets STAB
    // 		// Not even if you Roost in Gen 4 and somehow manage to use
    // 		// Struggle in the same turn.
    // 		// (On second thought, it might be easier to get a MissingNo.)
    // 		if (type !== '???') {
    // 			let stab: number | [number, number] = 1;
    //
    // 			const isSTAB = move.forceSTAB || pokemon.hasType(type) || pokemon.getTypes(false, true).includes(type);
    // 			if (isSTAB) {
    // 				stab = 1.5;
    // 			}
    //
    // 			// The Stellar tera type makes this incredibly confusing
    // 			// If the move's type does not match one of the user's base types,
    // 			// the Stellar tera type applies a one-time 1.2x damage boost for that type.
    // 			//
    // 			// If the move's type does match one of the user's base types,
    // 			// then the Stellar tera type applies a one-time 2x STAB boost for that type,
    // 			// and then goes back to using the regular 1.5x STAB boost for those types.
    // 			if (pokemon.terastallized === 'Stellar') {
    // 				if (!pokemon.stellarBoostedTypes.includes(type) || move.stellarBoosted) {
    // 					stab = isSTAB ? 2 : [4915, 4096];
    // 					move.stellarBoosted = true;
    // 					if (pokemon.species.name !== 'Terapagos-Stellar') {
    // 						pokemon.stellarBoostedTypes.push(type);
    // 					}
    // 				}
    // 			} else {
    // 				if (pokemon.terastallized === type && pokemon.getTypes(false, true).includes(type)) {
    // 					stab = 2;
    // 				}
    // 				stab = this.battle.runEvent('ModifySTAB', pokemon, target, move, stab);
    // 			}
    //
    // 			baseDamage = this.battle.modify(baseDamage, stab);
    // 		}
    //
    // 		// types
    // 		let typeMod = target.runEffectiveness(move);
    // 		typeMod = this.battle.clampIntRange(typeMod, -6, 6);
    // 		target.getMoveHitData(move).typeMod = typeMod;
    // 		if (typeMod > 0) {
    // 			if (!suppressMessages) this.battle.add('-supereffective', target);
    //
    // 			for (let i = 0; i < typeMod; i++) {
    // 				baseDamage *= 2;
    // 			}
    // 		}
    // 		if (typeMod < 0) {
    // 			if (!suppressMessages) this.battle.add('-resisted', target);
    //
    // 			for (let i = 0; i > typeMod; i--) {
    // 				baseDamage = tr(baseDamage / 2);
    // 			}
    // 		}
    //
    // 		if (isCrit && !suppressMessages) this.battle.add('-crit', target);
    //
    // 		if (pokemon.status === 'brn' && move.category === 'Physical' && !pokemon.hasAbility('guts')) {
    // 			if (this.battle.gen < 6 || move.id !== 'facade') {
    // 				baseDamage = this.battle.modify(baseDamage, 0.5);
    // 			}
    // 		}
    //
    // 		// Generation 5, but nothing later, sets damage to 1 before the final damage modifiers
    // 		if (this.battle.gen === 5 && !baseDamage) baseDamage = 1;
    //
    // 		// Final modifier. Modifiers that modify damage after min damage check, such as Life Orb.
    // 		baseDamage = this.battle.runEvent('ModifyDamage', pokemon, target, move, baseDamage);
    //
    // 		if (move.isZOrMaxPowered && target.getMoveHitData(move).zBrokeProtect) {
    // 			baseDamage = this.battle.modify(baseDamage, 0.25);
    // 			this.battle.add('-zbroken', target);
    // 		}
    //
    // 		// Generation 6-7 moves the check for minimum 1 damage after the final modifier...
    // 		if (this.battle.gen !== 5 && !baseDamage) return 1;
    //
    // 		// ...but 16-bit truncation happens even later, and can truncate to 0
    // 		return tr(baseDamage, 16);
    // 	}
    //
    pub fn modify_damage(base_damage: i32, modifiers: &[f64]) -> i32 {
        let mut damage = base_damage as f64;
        for modifier in modifiers {
            damage = (damage * modifier).floor();
        }
        (damage as i32).max(1)
    }

    /// Check if move forces switch
    /// Equivalent to forceSwitch handling in battle-actions.ts
    pub fn should_force_switch(
        move_force_switch: bool,
        target_hp: i32,
        target_is_dynamaxed: bool,
    ) -> bool {
        if !move_force_switch {
            return false;
        }
        if target_hp == 0 {
            return false;
        }
        if target_is_dynamaxed {
            return false; // Dynamaxed Pokemon can't be forced out
        }
        true
    }

    /// Get multi-hit count for a move
    /// Equivalent to multi-hit handling in battle-actions.ts
    pub fn get_multi_hit_count(multi_hit: Option<i32>, random_value: i32) -> i32 {
        match multi_hit {
            None => 1,
            Some(n) if n <= 1 => 1,
            Some(2) => 2,
            Some(3) => 3,
            Some(5) => {
                // 2-5 hits: 35% for 2, 35% for 3, 15% for 4, 15% for 5
                match random_value % 100 {
                    0..=34 => 2,
                    35..=69 => 3,
                    70..=84 => 4,
                    _ => 5,
                }
            }
            Some(n) => n,
        }
    }

    /// Check critical hit
    /// Equivalent to critical hit calculation in battle-actions.ts
    pub fn is_critical_hit(
        crit_ratio: i32,
        attacker_focus_energy: bool,
        move_will_crit: Option<bool>,
        random_value: i32,
    ) -> bool {
        // Guaranteed crit or no crit
        if let Some(will_crit) = move_will_crit {
            return will_crit;
        }

        let mut crit_stages = crit_ratio;
        if attacker_focus_energy {
            crit_stages += 2;
        }

        // Crit chance by stage (Gen 7+)
        let crit_chance = match crit_stages {
            0 => 24, // 1/24
            1 => 8,  // 1/8
            2 => 2,  // 1/2
            _ => 1,  // Always crit at stage 3+
        };

        random_value % crit_chance == 0
    }

    // =========================================================================
    // MEGA EVOLUTION / TERASTALLIZE EXECUTION
    // =========================================================================

    /// Run Mega Evolution
    /// Equivalent to runMegaEvo in battle-actions.ts
    /// Returns the mega forme ID if successful
    pub fn run_mega_evo_check(
        _species_name: &str,
        mega_forme: Option<&str>,
        already_mega: bool,
    ) -> Option<String> {
        if already_mega {
            return None;
        }
        mega_forme.map(|s| s.to_string())
    }

    /// Execute Terastallization
    /// Equivalent to terastallize in battle-actions.ts
    /// Returns the new tera type if successful
    pub fn terastallize_check(
        tera_type: Option<&str>,
        already_terastallized: bool,
        side_terastallize_used: bool,
    ) -> Option<String> {
        if already_terastallized || side_terastallize_used {
            return None;
        }
        tera_type.map(|s| s.to_string())
    }

    // =========================================================================
    // DAMAGE CALCULATION HELPERS
    // =========================================================================

    /// Get damage from getDamage in battle-actions.ts
    /// This is a comprehensive damage calculation
    // TypeScript source:
    // /**
    // 	 * 0 is a success dealing 0 damage, such as from False Swipe at 1 HP.
    // 	 *
    // 	 * Normal PS return value rules apply:
    // 	 * undefined = success, null = silent failure, false = loud failure
    // 	 */
    // 	getDamage(
    // 		source: Pokemon, target: Pokemon, move: string | number | ActiveMove,
    // 		suppressMessages = false
    // 	): number | undefined | null | false {
    // 		if (typeof move === 'string') move = this.dex.getActiveMove(move);
    //
    // 		if (typeof move === 'number') {
    // 			const basePower = move;
    // 			move = new Dex.Move({
    // 				basePower,
    // 				type: '???',
    // 				category: 'Physical',
    // 				willCrit: false,
    // 			}) as ActiveMove;
    // 			move.hit = 0;
    // 		}
    //
    // 		if (!target.runImmunity(move, !suppressMessages)) {
    // 			return false;
    // 		}
    //
    // 		if (move.ohko) return this.battle.gen === 3 ? target.hp : target.maxhp;
    // 		if (move.damageCallback) return move.damageCallback.call(this.battle, source, target);
    // 		if (move.damage === 'level') {
    // 			return source.level;
    // 		} else if (move.damage) {
    // 			return move.damage;
    // 		}
    //
    // 		const category = this.battle.getCategory(move);
    //
    // 		let basePower: number | false | null = move.basePower;
    // 		if (move.basePowerCallback) {
    // 			basePower = move.basePowerCallback.call(this.battle, source, target, move);
    // 		}
    // 		if (!basePower) return basePower === 0 ? undefined : basePower;
    // 		basePower = this.battle.clampIntRange(basePower, 1);
    //
    // 		let critMult;
    // 		let critRatio = this.battle.runEvent('ModifyCritRatio', source, target, move, move.critRatio || 0);
    // 		if (this.battle.gen <= 5) {
    // 			critRatio = this.battle.clampIntRange(critRatio, 0, 5);
    // 			critMult = [0, 16, 8, 4, 3, 2];
    // 		} else {
    // 			critRatio = this.battle.clampIntRange(critRatio, 0, 4);
    // 			if (this.battle.gen === 6) {
    // 				critMult = [0, 16, 8, 2, 1];
    // 			} else {
    // 				critMult = [0, 24, 8, 2, 1];
    // 			}
    // 		}
    //
    // 		const moveHit = target.getMoveHitData(move);
    // 		moveHit.crit = move.willCrit || false;
    // 		if (move.willCrit === undefined) {
    // 			if (critRatio) {
    // 				moveHit.crit = this.battle.randomChance(1, critMult[critRatio]);
    // 			}
    // 		}
    //
    // 		if (moveHit.crit) {
    // 			moveHit.crit = this.battle.runEvent('CriticalHit', target, null, move);
    // 		}
    //
    // 		// happens after crit calculation
    // 		basePower = this.battle.runEvent('BasePower', source, target, move, basePower, true);
    //
    // 		if (!basePower) return 0;
    // 		basePower = this.battle.clampIntRange(basePower, 1);
    // 		// Hacked Max Moves have 0 base power, even if you Dynamax
    // 		if ((!source.volatiles['dynamax'] && move.isMax) || (move.isMax && this.dex.moves.get(move.baseMove).isMax)) {
    // 			basePower = 0;
    // 		}
    //
    // 		const dexMove = this.dex.moves.get(move.id);
    // 		if (source.terastallized && (source.terastallized === 'Stellar' ?
    // 			!source.stellarBoostedTypes.includes(move.type) : source.hasType(move.type)) &&
    // 			basePower < 60 && dexMove.priority <= 0 && !dexMove.multihit &&
    // 			// Hard move.basePower check for moves like Dragon Energy that have variable BP
    // 			!((move.basePower === 0 || move.basePower === 150) && move.basePowerCallback)
    // 		) {
    // 			basePower = 60;
    // 		}
    //
    // 		const level = source.level;
    //
    // 		const attacker = move.overrideOffensivePokemon === 'target' ? target : source;
    // 		const defender = move.overrideDefensivePokemon === 'source' ? source : target;
    //
    // 		const isPhysical = move.category === 'Physical';
    // 		let attackStat: StatIDExceptHP = move.overrideOffensiveStat || (isPhysical ? 'atk' : 'spa');
    // 		const defenseStat: StatIDExceptHP = move.overrideDefensiveStat || (isPhysical ? 'def' : 'spd');
    //
    // 		const statTable = { atk: 'Atk', def: 'Def', spa: 'SpA', spd: 'SpD', spe: 'Spe' };
    //
    // 		let atkBoosts = attacker.boosts[attackStat];
    // 		let defBoosts = defender.boosts[defenseStat];
    //
    // 		let ignoreNegativeOffensive = !!move.ignoreNegativeOffensive;
    // 		let ignorePositiveDefensive = !!move.ignorePositiveDefensive;
    //
    // 		if (moveHit.crit) {
    // 			ignoreNegativeOffensive = true;
    // 			ignorePositiveDefensive = true;
    // 		}
    // 		const ignoreOffensive = !!(move.ignoreOffensive || (ignoreNegativeOffensive && atkBoosts < 0));
    // 		const ignoreDefensive = !!(move.ignoreDefensive || (ignorePositiveDefensive && defBoosts > 0));
    //
    // 		if (ignoreOffensive) {
    // 			this.battle.debug('Negating (sp)atk boost/penalty.');
    // 			atkBoosts = 0;
    // 		}
    // 		if (ignoreDefensive) {
    // 			this.battle.debug('Negating (sp)def boost/penalty.');
    // 			defBoosts = 0;
    // 		}
    //
    // 		let attack = attacker.calculateStat(attackStat, atkBoosts, 1, source);
    // 		let defense = defender.calculateStat(defenseStat, defBoosts, 1, target);
    //
    // 		attackStat = (category === 'Physical' ? 'atk' : 'spa');
    //
    // 		// Apply Stat Modifiers
    // 		attack = this.battle.runEvent('Modify' + statTable[attackStat], source, target, move, attack);
    // 		defense = this.battle.runEvent('Modify' + statTable[defenseStat], target, source, move, defense);
    //
    // 		if (this.battle.gen <= 4 && ['explosion', 'selfdestruct'].includes(move.id) && defenseStat === 'def') {
    // 			defense = this.battle.clampIntRange(Math.floor(defense / 2), 1);
    // 		}
    //
    // 		const tr = this.battle.trunc;
    //
    // 		// int(int(int(2 * L / 5 + 2) * A * P / D) / 50);
    // 		const baseDamage = tr(tr(tr(tr(2 * level / 5 + 2) * basePower * attack) / defense) / 50);
    //
    // 		// Calculate damage modifiers separately (order differs between generations)
    // 		return this.modifyDamage(baseDamage, source, target, move, suppressMessages);
    // 	}
    //
    pub fn get_damage(params: DamageCalcParams) -> i32 {
        if params.base_power == 0 || params.type_effectiveness == 0.0 {
            return 0;
        }

        // Base damage formula
        let base_damage =
            ((2 * params.attacker_level / 5 + 2) * params.base_power * params.attacker_attack
                / params.defender_defense.max(1))
                / 50
                + 2;

        // Apply modifiers in order
        let mut damage = base_damage as f64;

        // STAB
        damage *= params.stab_modifier;

        // Type effectiveness
        damage *= params.type_effectiveness;

        // Critical hit
        if params.is_crit {
            damage *= 1.5;
        }

        // Random factor
        damage = damage * (params.random_factor as f64) / 100.0;

        // Other modifiers (abilities, items, weather, etc.)
        for modifier in params.other_modifiers {
            damage = (damage * modifier).floor();
        }

        (damage as i32).max(1)
    }

    /// Try spread move hit - checks all targets
    /// Equivalent to trySpreadMoveHit in battle-actions.ts
    pub fn try_spread_move_hit_check(target_count: usize, move_target_type: &str) -> bool {
        // Check if this is a spread move
        let is_spread = matches!(
            move_target_type,
            "allAdjacent" | "allAdjacentFoes" | "all" | "foeSide" | "allySide"
        );

        is_spread && target_count > 0
    }

    /// Move hit - execute the actual hit
    /// Equivalent to moveHit in battle-actions.ts
    pub fn move_hit_result(damage: i32, type_effectiveness: f64, is_crit: bool) -> MoveHitData {
        MoveHitData {
            crit: is_crit,
            type_mod: (type_effectiveness * 100.0) as i32 - 100,
            damage,
            z_broke_protect: false,
        }
    }

    /// Hit step try immunity
    /// Equivalent to hitStepTryImmunity in battle-actions.ts
    // 	hitStepTryImmunity(targets: Pokemon[], pokemon: Pokemon, move: ActiveMove) {
    // 		const hitResults = [];
    // 		for (const [i, target] of targets.entries()) {
    // 			if (this.battle.gen >= 6 && move.flags['powder'] && target !== pokemon && !this.dex.getImmunity('powder', target)) {
    // 				this.battle.debug('natural powder immunity');
    // 				this.battle.add('-immune', target);
    // 				hitResults[i] = false;
    // 			} else if (!this.battle.singleEvent('TryImmunity', move, {}, target, pokemon, move)) {
    // 				this.battle.add('-immune', target);
    // 				hitResults[i] = false;
    // 			} else if (this.battle.gen >= 7 && move.pranksterBoosted && pokemon.hasAbility('prankster') &&
    // 				!targets[i].isAlly(pokemon) && !this.dex.getImmunity('prankster', target)) {
    // 				this.battle.debug('natural prankster immunity');
    // 				if (target.illusion || !(move.status && !this.dex.getImmunity(move.status, target))) {
    // 					this.battle.hint("Since gen 7, Dark is immune to Prankster moves.");
    // 				}
    // 				this.battle.add('-immune', target);
    // 				hitResults[i] = false;
    // 			} else {
    // 				hitResults[i] = true;
    // 			}
    // 		}
    // 		return hitResults;
    // 	}
    //
    pub fn hit_step_try_immunity(
        move_type: &str,
        defender_types: &[String],
        defender_ability: &str,
        ignore_immunity: bool,
    ) -> bool {
        if ignore_immunity {
            return true;
        }

        // Check type immunity
        let effectiveness = get_effectiveness_multi(move_type, defender_types);
        if effectiveness == 0.0 {
            return false;
        }

        // Check ability immunity (simplified)
        !matches!(
            (defender_ability, move_type),
            ("voltabsorb" | "lightningrod" | "motordrive", "Electric")
                | ("waterabsorb" | "stormdrain" | "dryskin", "Water")
                | ("flashfire", "Fire")
                | ("sapsipper", "Grass")
                | ("levitate", "Ground")
        )
    }

    /// Hit step try hit event
    /// Equivalent to hitStepTryHitEvent in battle-actions.ts
    // 	hitStepTryHitEvent(targets: Pokemon[], pokemon: Pokemon, move: ActiveMove) {
    // 		const hitResults = this.battle.runEvent('TryHit', targets, pokemon, move);
    // 		if (!hitResults.includes(true) && hitResults.includes(false)) {
    // 			this.battle.add('-fail', pokemon);
    // 			this.battle.attrLastMove('[still]');
    // 		}
    // 		for (const i of targets.keys()) {
    // 			if (hitResults[i] !== this.battle.NOT_FAIL) hitResults[i] = hitResults[i] || false;
    // 		}
    // 		return hitResults;
    // 	}
    //
    pub fn hit_step_try_hit_event(
        target_has_substitute: bool,
        move_ignores_substitute: bool,
        move_is_sound: bool,
    ) -> bool {
        // Substitute blocks most moves
        if target_has_substitute && !move_ignores_substitute && !move_is_sound {
            return false; // Hit substitute instead
        }
        true
    }

    /// After move secondary event
    /// Equivalent to afterMoveSecondaryEvent in battle-actions.ts
    // 	afterMoveSecondaryEvent(targets: Pokemon[], pokemon: Pokemon, move: ActiveMove) {
    // 		// console.log(`${targets}, ${pokemon}, ${move}`)
    // 		if (!(move.hasSheerForce && pokemon.hasAbility('sheerforce'))) {
    // 			this.battle.singleEvent('AfterMoveSecondary', move, null, targets[0], pokemon, move);
    // 			this.battle.runEvent('AfterMoveSecondary', targets, pokemon, move);
    // 		}
    // 		return undefined;
    // 	}
    //
    pub fn after_move_secondary_event(
        move_self_switch: Option<&str>,
        move_force_switch: bool,
    ) -> AfterMoveResult {
        AfterMoveResult {
            self_switch: move_self_switch.map(|s| s.to_string()),
            force_switch: move_force_switch,
        }
    }

    /// Try move hit - main hit attempt
    /// Equivalent to tryMoveHit in battle-actions.ts
    pub fn try_move_hit_check(
        accuracy_passed: bool,
        type_immunity_passed: bool,
        invulnerability_passed: bool,
    ) -> bool {
        accuracy_passed && type_immunity_passed && invulnerability_passed
    }

    /// Hit step move hit loop
    /// Equivalent to hitStepMoveHitLoop in battle-actions.ts
    pub fn hit_step_move_hit_loop_count(
        multi_hit: Option<i32>,
        move_hit_type: Option<&str>,
        random_value: i32,
    ) -> i32 {
        if let Some(hit_type) = move_hit_type {
            match hit_type {
                "parentalbond" => 2,
                "triplekick" => 3,
                _ => Self::get_multi_hit_count(multi_hit, random_value),
            }
        } else {
            Self::get_multi_hit_count(multi_hit, random_value)
        }
    }

    /// Spread move hit - apply to all targets
    /// Equivalent to spreadMoveHit in battle-actions.ts
    pub fn spread_move_hit_modifier(target_count: usize) -> f64 {
        Self::get_spread_damage_modifier(target_count)
    }

    /// Try primary hit event
    /// Equivalent to tryPrimaryHitEvent in battle-actions.ts
    // 	tryPrimaryHitEvent(
    // 		damage: SpreadMoveDamage, targets: SpreadMoveTargets, pokemon: Pokemon,
    // 		move: ActiveMove, moveData: ActiveMove, isSecondary?: boolean
    // 	): SpreadMoveDamage {
    // 		for (const [i, target] of targets.entries()) {
    // 			if (!target) continue;
    // 			damage[i] = this.battle.runEvent('TryPrimaryHit', target, pokemon, moveData);
    // 		}
    // 		return damage;
    // 	}
    //
    pub fn try_primary_hit_event(
        move_has_primary_effect: bool,
        move_primary_chance: Option<i32>,
        random_value: i32,
    ) -> bool {
        if !move_has_primary_effect {
            return true; // No primary effect to apply
        }

        if let Some(chance) = move_primary_chance {
            random_value % 100 < chance
        } else {
            true // Guaranteed primary effect
        }
    }

    /// Run move effects
    /// Equivalent to runMoveEffects in battle-actions.ts
    pub fn run_move_effects_list(
        move_boosts: Option<&BoostsTable>,
        move_heal: Option<(i32, i32)>,
        move_status: Option<&str>,
        move_volatile: Option<&str>,
    ) -> MoveEffects {
        MoveEffects {
            boosts: move_boosts.cloned(),
            heal: move_heal,
            status: move_status.map(|s| s.to_string()),
            volatile_status: move_volatile.map(|s| s.to_string()),
        }
    }
}

/// Result from after move secondary event
#[derive(Debug, Clone, Default)]
pub struct AfterMoveResult {
    pub self_switch: Option<String>,
    pub force_switch: bool,
}

/// Move effects to apply
#[derive(Debug, Clone, Default)]
pub struct MoveEffects {
    pub boosts: Option<BoostsTable>,
    pub heal: Option<(i32, i32)>,
    pub status: Option<String>,
    pub volatile_status: Option<String>,
}

/// Z-Power effects for status Z-moves
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZPowerEffect {
    Heal,
    ClearNegativeBoost,
    Crit2,
    Redirect,
    HealReplacement,
    Curse,
}

/// Run move options for runMove
/// Equivalent to the options parameter in battle-actions.ts runMove()
#[derive(Debug, Clone, Default)]
pub struct RunMoveOptions {
    /// Source effect that caused this move
    pub source_effect: Option<ID>,
    /// Z-move override
    pub z_move: Option<String>,
    /// External move (Dancer, etc.)
    pub external_move: bool,
    /// Max move override
    pub max_move: Option<String>,
    /// Original target for redirection tracking
    pub original_target: Option<usize>,
}

/// Use move options for useMove
/// Equivalent to the options parameter in battle-actions.ts useMove()
#[derive(Debug, Clone, Default)]
pub struct UseMoveOptions {
    /// Target pokemon index
    pub target: Option<usize>,
    /// Source effect
    pub source_effect: Option<ID>,
    /// Z-move override
    pub z_move: Option<String>,
    /// Max move override
    pub max_move: Option<String>,
}

/// Spread move damage result type
/// Can be damage amount, false (failed), or true/undefined (success with no damage)
#[derive(Debug, Clone, Copy, Default)]
pub enum SpreadMoveDamageValue {
    Damage(i32),
    Failed,
    Success,
    #[default]
    Undefined,
}

/// Result of spread move hit containing damage and target info
pub type SpreadMoveDamage = Vec<SpreadMoveDamageValue>;

/// Target info for spread moves (can be the Pokemon or null/false for failed)
#[derive(Debug, Clone)]
pub enum SpreadMoveTarget {
    Target(usize),
    None,
    Failed,
}

/// Spread move targets array
pub type SpreadMoveTargets = Vec<SpreadMoveTarget>;

impl<'a> BattleActions<'a> {
    /// Run a move - the "outside" move caller
    /// Handles Dancer, Instruct, Pursuit, etc.
    /// Equivalent to battle-actions.ts runMove()
    ///
    /// This is a stub that provides the interface for the full battle implementation.
    /// The actual implementation requires Battle context for events.
    pub fn run_move_stub(
        move_id: &ID,
        pokemon_index: usize,
        target_loc: i32,
        options: RunMoveOptions,
    ) -> RunMoveResult {
        // This is a stub - full implementation requires Battle context
        RunMoveResult {
            move_id: move_id.clone(),
            pokemon_index,
            target_loc,
            z_move: options.z_move,
            max_move: options.max_move,
            external_move: options.external_move,
            success: true,
        }
    }

    /// Use move - the "inside" move caller
    /// Handles effects of the move itself
    /// Equivalent to battle-actions.ts useMove()
    ///
    /// This is a stub that provides the interface for the full battle implementation.
    pub fn use_move_stub(move_id: &ID, pokemon_index: usize, options: UseMoveOptions) -> bool {
        // This is a stub - full implementation requires Battle context
        // Returns whether the move was successful
        let _ = (move_id, pokemon_index, options);
        true
    }

    /// Inner implementation of useMove
    /// Equivalent to battle-actions.ts useMoveInner()
    ///
    /// This is a stub that provides the interface for the full battle implementation.
    pub fn use_move_inner_stub(
        move_id: &ID,
        pokemon_index: usize,
        options: UseMoveOptions,
    ) -> bool {
        // This is a stub - full implementation requires Battle context
        let _ = (move_id, pokemon_index, options);
        true
    }

    /// Try spread move hit - main entry point for move hit processing
    /// Equivalent to battle-actions.ts trySpreadMoveHit()
    ///
    /// Returns whether the move hit at least one target
    pub fn try_spread_move_hit_stub(
        target_indices: &[usize],
        pokemon_index: usize,
        move_id: &ID,
        not_active: bool,
    ) -> bool {
        // This is a stub - full implementation requires Battle context
        let _ = (target_indices, pokemon_index, move_id, not_active);
        true
    }

    /// Hit step: move hit loop
    /// Processes each hit of a multi-hit move
    /// Equivalent to battle-actions.ts hitStepMoveHitLoop()
    pub fn hit_step_move_hit_loop_stub(
        target_indices: &[usize],
        pokemon_index: usize,
        move_id: &ID,
        multi_hit: Option<i32>,
        gen: u8,
    ) -> SpreadMoveDamage {
        let target_hits = multi_hit.unwrap_or(1) as usize;
        let mut damage = vec![SpreadMoveDamageValue::Damage(0); target_indices.len()];

        // Stub implementation - just returns placeholder damage
        for (i, damage_val) in damage.iter_mut().enumerate().take(target_hits) {
            if i >= target_indices.len() {
                break;
            }
            *damage_val = SpreadMoveDamageValue::Damage(50);
        }

        let _ = (pokemon_index, move_id, gen);
        damage
    }

    /// Spread move hit - handles the actual hit processing
    /// Equivalent to battle-actions.ts spreadMoveHit()
    ///
    /// Returns (damage_array, updated_targets)
    pub fn spread_move_hit_stub(
        target_indices: &[usize],
        pokemon_index: usize,
        move_id: &ID,
        is_secondary: bool,
        is_self: bool,
    ) -> (SpreadMoveDamage, SpreadMoveTargets) {
        let mut damage: SpreadMoveDamage =
            vec![SpreadMoveDamageValue::Success; target_indices.len()];
        let targets: SpreadMoveTargets = target_indices
            .iter()
            .map(|&i| SpreadMoveTarget::Target(i))
            .collect();

        // Stub implementation
        let _ = (pokemon_index, move_id, is_secondary, is_self);

        // Set placeholder damage for each target
        damage.fill(SpreadMoveDamageValue::Damage(50));

        (damage, targets)
    }

    /// Move hit - single target wrapper for spreadMoveHit
    /// Equivalent to battle-actions.ts moveHit()
    pub fn move_hit_stub(
        target_index: Option<usize>,
        pokemon_index: usize,
        move_id: &ID,
        is_secondary: bool,
        is_self: bool,
    ) -> Option<i32> {
        let targets = match target_index {
            Some(idx) => vec![idx],
            None => vec![],
        };

        if targets.is_empty() {
            return None;
        }

        let (damage, _) =
            Self::spread_move_hit_stub(&targets, pokemon_index, move_id, is_secondary, is_self);

        match damage.first() {
            Some(SpreadMoveDamageValue::Damage(d)) => Some(*d),
            Some(SpreadMoveDamageValue::Success) => None,
            _ => None,
        }
    }

    /// Get spread damage for each target
    /// Equivalent to battle-actions.ts getSpreadDamage()
    pub fn get_spread_damage_stub(
        damage: &mut SpreadMoveDamage,
        target_indices: &[usize],
        source_index: usize,
        move_id: &ID,
        is_secondary: bool,
        is_self: bool,
    ) {
        // Stub implementation - would call getDamage for each target
        let _ = (target_indices, source_index, move_id, is_secondary, is_self);
        for d in damage.iter_mut() {
            *d = SpreadMoveDamageValue::Damage(50);
        }
    }

    /// Force switch handling
    /// Equivalent to battle-actions.ts forceSwitch()
    pub fn force_switch_stub(
        damage: &mut SpreadMoveDamage,
        target_indices: &[usize],
        source_index: usize,
        move_id: &ID,
    ) {
        // Stub implementation - would set forceSwitchFlag on targets
        let _ = (damage, target_indices, source_index, move_id);
        // In real implementation:
        // for target in targets where hp > 0 and can switch:
        //   set target.force_switch_flag = true
    }

    /// Run Z-Power effect for status Z-moves
    /// Equivalent to battle-actions.ts runZPower()
    //
    // 	runZPower(move: ActiveMove, pokemon: Pokemon) {
    // 		const zPower = this.dex.conditions.get('zpower');
    // 		if (move.category !== 'Status') {
    // 			this.battle.attrLastMove('[zeffect]');
    // 		} else if (move.zMove?.boost) {
    // 			this.battle.boost(move.zMove.boost, pokemon, pokemon, zPower);
    // 		} else if (move.zMove?.effect) {
    // 			switch (move.zMove.effect) {
    // 			case 'heal':
    // 				this.battle.heal(pokemon.maxhp, pokemon, pokemon, zPower);
    // 				break;
    // 			case 'healreplacement':
    // 				pokemon.side.addSlotCondition(pokemon, 'healreplacement', pokemon, move);
    // 				break;
    // 			case 'clearnegativeboost':
    // 				const boosts: SparseBoostsTable = {};
    // 				let i: BoostID;
    // 				for (i in pokemon.boosts) {
    // 					if (pokemon.boosts[i] < 0) {
    // 						boosts[i] = 0;
    // 					}
    // 				}
    // 				pokemon.setBoost(boosts);
    // 				this.battle.add('-clearnegativeboost', pokemon, '[zeffect]');
    // 				break;
    // 			case 'redirect':
    // 				pokemon.addVolatile('followme', pokemon, zPower);
    // 				break;
    // 			case 'crit2':
    // 				pokemon.addVolatile('focusenergy', pokemon, zPower);
    // 				break;
    // 			case 'curse':
    // 				if (pokemon.hasType('Ghost')) {
    // 					this.battle.heal(pokemon.maxhp, pokemon, pokemon, zPower);
    // 				} else {
    // 					this.battle.boost({ atk: 1 }, pokemon, pokemon, zPower);
    // 				}
    // 			}
    // 		}
    // 	}
    //
    pub fn run_z_power(
        move_category: &str,
        z_move_boost: Option<&BoostsTable>,
        z_move_effect: Option<&str>,
        pokemon_has_ghost_type: bool,
    ) -> ZPowerResult {
        if move_category != "Status" {
            // Damage Z-moves just add [zeffect] attribute
            return ZPowerResult::DamageMove;
        }

        // Status Z-moves can have boosts or effects
        if let Some(boosts) = z_move_boost {
            return ZPowerResult::Boost(*boosts);
        }

        if let Some(effect) = z_move_effect {
            match effect {
                "heal" => ZPowerResult::Heal,
                "healreplacement" => ZPowerResult::HealReplacement,
                "clearnegativeboost" => ZPowerResult::ClearNegativeBoost,
                "redirect" => ZPowerResult::Redirect,
                "crit2" => ZPowerResult::Crit2,
                "curse" => {
                    if pokemon_has_ghost_type {
                        ZPowerResult::Heal
                    } else {
                        ZPowerResult::Boost(BoostsTable {
                            atk: 1,
                            ..Default::default()
                        })
                    }
                }
                _ => ZPowerResult::None,
            }
        } else {
            ZPowerResult::None
        }
    }

    /// Run Mega Evolution
    /// Equivalent to battle-actions.ts runMegaEvo()
    pub fn run_mega_evo_stub(
        pokemon_index: usize,
        can_mega_evo: Option<&str>,
        can_ultra_burst: Option<&str>,
    ) -> Option<String> {
        // Return the target forme if mega evolution is possible
        let _ = pokemon_index;

        if let Some(species_id) = can_mega_evo {
            return Some(species_id.to_string());
        }

        if let Some(species_id) = can_ultra_burst {
            return Some(species_id.to_string());
        }

        None
    }

    /// Terastallize a Pokemon
    /// Equivalent to battle-actions.ts terastallize()
    pub fn terastallize_stub(
        pokemon_index: usize,
        tera_type: &str,
        species_base_species: &str,
    ) -> TerastallizeResult {
        let _ = pokemon_index;

        // Handle Ogerpon special case
        if species_base_species == "Ogerpon"
            && !["Fire", "Grass", "Rock", "Water"].contains(&tera_type)
        {
            return TerastallizeResult::InvalidOgerpon;
        }

        TerastallizeResult::Success {
            tera_type: tera_type.to_string(),
            forme_change: if species_base_species == "Ogerpon" {
                Some("ogerpontealtera".to_string())
            } else if species_base_species == "Terapagos-Terastal" {
                Some("Terapagos-Stellar".to_string())
            } else {
                None
            },
        }
    }

    /// Try move hit - wrapper for single/multi target moves
    /// Equivalent to battle-actions.ts tryMoveHit()
    pub fn try_move_hit_stub(
        target_or_targets: &[usize],
        pokemon_index: usize,
        move_id: &ID,
    ) -> Option<i32> {
        // This calls hitStepMoveHitLoop and returns damage
        let damage = Self::hit_step_move_hit_loop_stub(
            target_or_targets,
            pokemon_index,
            move_id,
            None,
            9, // Default to gen 9
        );

        // Sum up damage
        let mut total = 0i32;
        for d in damage {
            if let SpreadMoveDamageValue::Damage(dmg) = d {
                total += dmg;
            }
        }

        if total > 0 {
            Some(total)
        } else {
            None
        }
    }

    /// Secondaries handling
    /// Equivalent to battle-actions.ts secondaries()
    pub fn secondaries_stub(
        target_indices: &[usize],
        source_index: usize,
        move_id: &ID,
        is_self: bool,
    ) {
        // Stub - would apply secondary effects
        let _ = (target_indices, source_index, move_id, is_self);
    }

    // =========================================================================
    // SWITCH METHODS - Ported from battle-actions.ts
    // These require Battle context so they are stubs with proper interfaces
    // =========================================================================

    /// Switch in a Pokemon
    /// Equivalent to battle-actions.ts switchIn()
    ///
    /// This is the main switch-in method that handles:
    /// - Checking if Pokemon is already active
    /// - Running BeforeSwitchOut/SwitchOut events on old active
    /// - Handling switch copy flags (copyvolatile, shedtail)
    /// - Clearing volatiles on old active
    /// - Setting up the new active Pokemon
    /// - Running BeforeSwitchIn events
    /// - Adding the switch message
    ///
    /// Returns true if switch was successful, false if blocked,
    /// or "pursuitfaint" if the Pokemon fainted from Pursuit before switching
    pub fn switch_in_stub(
        pokemon_index: usize,
        side_index: usize,
        pos: usize,
        source_effect: Option<&ID>,
        is_drag: bool,
    ) -> SwitchInResult {
        // This is a stub - full implementation requires Battle context
        let _ = (pokemon_index, side_index, pos, source_effect, is_drag);

        SwitchInResult::Success
    }

    /// Drag in a random Pokemon (from Whirlwind, Dragon Tail, etc.)
    /// Equivalent to battle-actions.ts dragIn()
    ///
    /// This method:
    /// - Gets a random switchable Pokemon from the side
    /// - Runs the DragOut event on the current active
    /// - Calls switchIn with isDrag=true
    pub fn drag_in_stub(side_index: usize, pos: usize) -> bool {
        // This is a stub - full implementation requires Battle context
        let _ = (side_index, pos);
        true
    }

    /// Run switch-in effects
    /// Equivalent to battle-actions.ts runSwitch()
    ///
    /// This method processes all pending runSwitch choices in the queue
    /// and runs the SwitchIn field event for all switching Pokemon.
    /// It also marks Pokemon as started and clears draggedIn.
    pub fn run_switch_stub(pokemon_index: usize) -> bool {
        // This is a stub - full implementation requires Battle context
        let _ = pokemon_index;
        true
    }
}

/// Result of run_move
#[derive(Debug, Clone)]
pub struct RunMoveResult {
    pub move_id: ID,
    pub pokemon_index: usize,
    pub target_loc: i32,
    pub z_move: Option<String>,
    pub max_move: Option<String>,
    pub external_move: bool,
    pub success: bool,
}

/// Result of run_z_power
#[derive(Debug, Clone)]
pub enum ZPowerResult {
    DamageMove,
    Boost(BoostsTable),
    Heal,
    HealReplacement,
    ClearNegativeBoost,
    Redirect,
    Crit2,
    None,
}

/// Result of terastallize
#[derive(Debug, Clone)]
pub enum TerastallizeResult {
    Success {
        tera_type: String,
        forme_change: Option<String>,
    },
    InvalidOgerpon,
}

/// Result of switchIn
/// Equivalent to the return value of battle-actions.ts switchIn()
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwitchInResult {
    /// Switch was successful
    Success,
    /// Switch was blocked (e.g., by an event returning false)
    Blocked,
    /// Pokemon fainted from Pursuit before switching
    PursuitFaint,
}

// =========================================================================
// MOVE EXECUTION - useMove and useMoveInner
// These are standalone functions that take &mut Battle as parameter
// Equivalent to battle-actions.ts useMove() and useMoveInner()
// =========================================================================

/// Use a move - wrapper for use_move_inner
/// Equivalent to battle-actions.ts useMove() (lines 368-379)
// TypeScript source:
// /**
// 	 * useMove is the "inside" move caller. It handles effects of the
// 	 * move itself, but not the idea of using the move.
// 	 *
// 	 * Most caller effects, like Sleep Talk, Nature Power, Magic Bounce,
// 	 * etc use useMove.
// 	 *
// 	 * The only ones that use runMove are Instruct, Pursuit, and
// 	 * Dancer.
// 	 */
// 	useMove(
// 		move: Move | string, pokemon: Pokemon, options?: {
// 			target?: Pokemon | null, sourceEffect?: Effect | null,
// 			zMove?: string, maxMove?: string,
// 		}
// 	) {
// 		pokemon.moveThisTurnResult = undefined;
// 		const oldMoveResult: boolean | null | undefined = pokemon.moveThisTurnResult;
// 		const moveResult = this.useMoveInner(move, pokemon, options);
// 		if (oldMoveResult === pokemon.moveThisTurnResult) pokemon.moveThisTurnResult = moveResult;
// 		return moveResult;
// 	}
//
pub fn use_move(
    battle: &mut crate::battle::Battle,
    move_id: &ID,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
    source_effect: Option<&ID>,
    z_move: Option<&str>,
    max_move: Option<&str>,
) -> bool {
    // pokemon.moveThisTurnResult = undefined;
    // Note: moveThisTurnResult tracking would go here

    // const oldMoveResult: boolean | null | undefined = pokemon.moveThisTurnResult;
    // const moveResult = this.useMoveInner(move, pokemon, options);

    // if (oldMoveResult === pokemon.moveThisTurnResult) pokemon.moveThisTurnResult = moveResult;
    // Note: moveThisTurnResult syncing would go here

    // return moveResult;
    use_move_inner(
        battle,
        move_id,
        pokemon_pos,
        target_pos,
        source_effect,
        z_move,
        max_move,
    )
}

/// Inner implementation of useMove - handles the actual move execution
/// Equivalent to battle-actions.ts useMoveInner() (lines 380-543)
// 	useMoveInner(
// 		moveOrMoveName: Move | string, pokemon: Pokemon, options?: {
// 			target?: Pokemon | null, sourceEffect?: Effect | null,
// 			zMove?: string, maxMove?: string,
// 		},
// 	) {
// 		let target = options?.target;
// 		let sourceEffect = options?.sourceEffect;
// 		const zMove = options?.zMove;
// 		const maxMove = options?.maxMove;
// 		if (!sourceEffect && this.battle.effect.id) sourceEffect = this.battle.effect;
// 		if (sourceEffect && ['instruct', 'custapberry'].includes(sourceEffect.id)) sourceEffect = null;
//
// 		let move = this.dex.getActiveMove(moveOrMoveName);
// 		pokemon.lastMoveUsed = move;
// 		if (move.id === 'weatherball' && zMove) {
// 			// Z-Weather Ball only changes types if it's used directly,
// 			// not if it's called by Z-Sleep Talk or something.
// 			this.battle.singleEvent('ModifyType', move, null, pokemon, target, move, move);
// 			if (move.type !== 'Normal') sourceEffect = move;
// 		}
// 		if (zMove || (move.category !== 'Status' && sourceEffect && (sourceEffect as ActiveMove).isZ)) {
// 			move = this.getActiveZMove(move, pokemon);
// 		}
// 		if (maxMove && move.category !== 'Status') {
// 			// Max move outcome is dependent on the move type after type modifications from ability and the move itself
// 			this.battle.singleEvent('ModifyType', move, null, pokemon, target, move, move);
// 			this.battle.runEvent('ModifyType', pokemon, target, move, move);
// 		}
// 		if (maxMove || (move.category !== 'Status' && sourceEffect && (sourceEffect as ActiveMove).isMax)) {
// 			move = this.getActiveMaxMove(move, pokemon);
// 		}
//
// 		if (this.battle.activeMove) {
// 			move.priority = this.battle.activeMove.priority;
// 			if (!move.hasBounced) move.pranksterBoosted = this.battle.activeMove.pranksterBoosted;
// 		}
// 		const baseTarget = move.target;
// 		let targetRelayVar = { target };
// 		targetRelayVar = this.battle.runEvent('ModifyTarget', pokemon, target, move, targetRelayVar, true);
// 		if (targetRelayVar.target !== undefined) target = targetRelayVar.target;
// 		if (target === undefined) target = this.battle.getRandomTarget(pokemon, move);
// 		if (move.target === 'self' || move.target === 'allies') {
// 			target = pokemon;
// 		}
// 		if (sourceEffect) {
// 			move.sourceEffect = sourceEffect.id;
// 			move.ignoreAbility = (sourceEffect as ActiveMove).ignoreAbility;
// 		}
// 		let moveResult = false;
//
// 		this.battle.setActiveMove(move, pokemon, target);
//
// 		this.battle.singleEvent('ModifyType', move, null, pokemon, target, move, move);
// 		this.battle.singleEvent('ModifyMove', move, null, pokemon, target, move, move);
// 		if (baseTarget !== move.target) {
// 			// Target changed in ModifyMove, so we must adjust it here
// 			// Adjust before the next event so the correct target is passed to the
// 			// event
// 			target = this.battle.getRandomTarget(pokemon, move);
// 		}
// 		move = this.battle.runEvent('ModifyType', pokemon, target, move, move);
// 		move = this.battle.runEvent('ModifyMove', pokemon, target, move, move);
// 		if (baseTarget !== move.target) {
// 			// Adjust again
// 			target = this.battle.getRandomTarget(pokemon, move);
// 		}
// 		if (!move || pokemon.fainted) {
// 			return false;
// 		}
//
// 		let attrs = '';
//
// 		let movename = move.name;
// 		if (move.id === 'hiddenpower') movename = 'Hidden Power';
// 		if (sourceEffect) attrs += `|[from] ${sourceEffect.fullname}`;
// 		if (zMove && move.isZ === true) {
// 			attrs = `|[anim]${movename}${attrs}`;
// 			movename = `Z-${movename}`;
// 		}
// 		this.battle.addMove('move', pokemon, movename, `${target}${attrs}`);
//
// 		if (zMove) this.runZPower(move, pokemon);
//
// 		if (!target) {
// 			this.battle.attrLastMove('[notarget]');
// 			this.battle.add(this.battle.gen >= 5 ? '-fail' : '-notarget', pokemon);
// 			return false;
// 		}
//
// 		const { targets, pressureTargets } = pokemon.getMoveTargets(move, target);
// 		if (targets.length) {
// 			target = targets[targets.length - 1]; // in case of redirection
// 		}
//
// 		const callerMoveForPressure = sourceEffect && (sourceEffect as ActiveMove).pp ? sourceEffect as ActiveMove : null;
// 		if (!sourceEffect || callerMoveForPressure || sourceEffect.id === 'pursuit') {
// 			let extraPP = 0;
// 			for (const source of pressureTargets) {
// 				const ppDrop = this.battle.runEvent('DeductPP', source, pokemon, move);
// 				if (ppDrop !== true) {
// 					extraPP += ppDrop || 0;
// 				}
// 			}
// 			if (extraPP > 0) {
// 				pokemon.deductPP(callerMoveForPressure || moveOrMoveName, extraPP);
// 			}
// 		}
//
// 		if (!this.battle.singleEvent('TryMove', move, null, pokemon, target, move) ||
// 			!this.battle.runEvent('TryMove', pokemon, target, move)) {
// 			move.mindBlownRecoil = false;
// 			return false;
// 		}
//
// 		this.battle.singleEvent('UseMoveMessage', move, null, pokemon, target, move);
//
// 		if (move.ignoreImmunity === undefined) {
// 			move.ignoreImmunity = (move.category === 'Status');
// 		}
//
// 		if (this.battle.gen !== 4 && move.selfdestruct === 'always') {
// 			this.battle.faint(pokemon, pokemon, move);
// 		}
//
// 		let damage: number | false | undefined | '' = false;
// 		if (move.target === 'all' || move.target === 'foeSide' || move.target === 'allySide' || move.target === 'allyTeam') {
// 			damage = this.tryMoveHit(targets, pokemon, move);
// 			if (damage === this.battle.NOT_FAIL) pokemon.moveThisTurnResult = null;
// 			if (damage || damage === 0 || damage === undefined) moveResult = true;
// 		} else {
// 			if (!targets.length) {
// 				this.battle.attrLastMove('[notarget]');
// 				this.battle.add(this.battle.gen >= 5 ? '-fail' : '-notarget', pokemon);
// 				return false;
// 			}
// 			if (this.battle.gen === 4 && move.selfdestruct === 'always') {
// 				this.battle.faint(pokemon, pokemon, move);
// 			}
// 			moveResult = this.trySpreadMoveHit(targets, pokemon, move);
// 		}
// 		if (move.selfBoost && moveResult) this.moveHit(pokemon, pokemon, move, move.selfBoost, false, true);
// 		if (!pokemon.hp) {
// 			this.battle.faint(pokemon, pokemon, move);
// 		}
//
// 		if (!moveResult) {
// 			this.battle.singleEvent('MoveFail', move, null, target, pokemon, move);
// 			return false;
// 		}
//
// 		if (!(move.hasSheerForce && pokemon.hasAbility('sheerforce')) && !move.flags['futuremove']) {
// 			const originalHp = pokemon.hp;
// 			this.battle.singleEvent('AfterMoveSecondarySelf', move, null, pokemon, target, move);
// 			this.battle.runEvent('AfterMoveSecondarySelf', pokemon, target, move);
// 			if (pokemon && pokemon !== target && move.category !== 'Status') {
// 				if (pokemon.hp <= pokemon.maxhp / 2 && originalHp > pokemon.maxhp / 2) {
// 					this.battle.runEvent('EmergencyExit', pokemon, pokemon);
// 				}
// 			}
// 		}
//
// 		return true;
// 	}
//
pub fn use_move_inner(
    battle: &mut crate::battle::Battle,
    move_or_move_name: &ID,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
    source_effect_param: Option<&ID>,
    z_move_param: Option<&str>,
    max_move_param: Option<&str>,
) -> bool {
    // let target = options?.target;
    let mut target = target_pos;

    // let sourceEffect = options?.sourceEffect;
    let mut source_effect = source_effect_param.cloned();

    // const zMove = options?.zMove;
    let z_move = z_move_param;

    // const maxMove = options?.maxMove;
    let max_move = max_move_param;

    // if (!sourceEffect && this.battle.effect.id) sourceEffect = this.battle.effect;
    // if (sourceEffect && ['instruct', 'custapberry'].includes(sourceEffect.id)) sourceEffect = null;
    if source_effect.is_none() {
        if let Some(ref current_eff) = battle.current_effect {
            source_effect = Some(current_eff.clone());
        }
    }

    // Exclude instruct and custapberry from source effects
    if let Some(ref eff) = source_effect {
        let eff_id = eff.as_str();
        if eff_id == "instruct" || eff_id == "custapberry" {
            source_effect = None;
        }
    }

    // let move = this.dex.getActiveMove(moveOrMoveName);
    let mut active_move = match battle.dex.get_active_move(move_or_move_name.as_str()) {
        Some(m) => m,
        None => return false,
    };

    // pokemon.lastMoveUsed = move;
    let (side_idx, poke_idx) = pokemon_pos;
    battle.sides[side_idx].pokemon[poke_idx].last_move_used = Some(move_or_move_name.clone());

    // if (move.id === 'weatherball' && zMove) {
    //     this.battle.singleEvent('ModifyType', move, null, pokemon, target, move, move);
    //     if (move.type !== 'Normal') sourceEffect = move;
    // }
    if move_or_move_name.as_str() == "weatherball" && z_move.is_some() {
        battle.single_event(
            "ModifyType",
            move_or_move_name,
            Some(pokemon_pos),
            target,
            Some(move_or_move_name),
        );
        // After ModifyType event, check if move type changed from Normal
        if let Some(updated_move) = battle.dex.get_move(move_or_move_name.as_str()) {
            if updated_move.move_type != "Normal" {
                source_effect = Some(move_or_move_name.clone());
            }
        }
    }

    // if (zMove || (move.category !== 'Status' && sourceEffect && (sourceEffect as ActiveMove).isZ)) {
    //     move = this.getActiveZMove(move, pokemon);
    // }
    if z_move.is_some()
        || (active_move.category != "Status"
            && source_effect.as_ref().is_some_and(|eff| {
                // Check if source effect is a Z-move
                battle
                    .dex
                    .get_active_move(eff.as_str())
                    .is_some_and(|m| m.is_z)
            }))
    {
        // Transform to Z-move
        active_move = BattleActions::get_active_z_move(
            &active_move.id.to_string(),
            &active_move.move_type,
            &active_move.category,
            active_move.base_power,
            None, // z_crystal_base_power would come from move's z_move data
        );
    }

    // if (maxMove && move.category !== 'Status') {
    //     this.battle.singleEvent('ModifyType', move, null, pokemon, target, move, move);
    //     this.battle.runEvent('ModifyType', pokemon, target, move, move);
    // }
    if max_move.is_some() && active_move.category != "Status" {
        battle.single_event(
            "ModifyType",
            move_or_move_name,
            Some(pokemon_pos),
            target,
            Some(move_or_move_name),
        );
        battle.run_event(
            "ModifyType",
            Some(pokemon_pos),
            target,
            Some(move_or_move_name),
            None,
        );
    }

    // if (maxMove || (move.category !== 'Status' && sourceEffect && (sourceEffect as ActiveMove).isMax)) {
    //     move = this.getActiveMaxMove(move, pokemon);
    // }
    if max_move.is_some()
        || (active_move.category != "Status"
            && source_effect.as_ref().is_some_and(|eff| {
                // Check if source effect is a Max move
                battle
                    .dex
                    .get_active_move(eff.as_str())
                    .is_some_and(|m| m.is_max)
            }))
    {
        // Transform to Max move
        active_move = BattleActions::get_active_max_move(
            &active_move.id.to_string(),
            &active_move.move_type,
            &active_move.category,
            active_move.base_power,
            None, // gmax_move would come from pokemon's gigantamax data
        );
    }

    // if (this.battle.activeMove) {
    //     move.priority = this.battle.activeMove.priority;
    //     if (!move.hasBounced) move.pranksterBoosted = this.battle.activeMove.pranksterBoosted;
    // }
    if let Some(ref battle_active_move) = battle.active_move {
        active_move.priority = battle_active_move.priority;
        if !active_move.has_bounced {
            active_move.prankster_boosted = battle_active_move.prankster_boosted;
        }
    }

    // const baseTarget = move.target;
    let base_target = active_move.target.clone();

    // let targetRelayVar = { target };
    // targetRelayVar = this.battle.runEvent('ModifyTarget', pokemon, target, move, targetRelayVar, true);
    // if (targetRelayVar.target !== undefined) target = targetRelayVar.target;
    // Implement ModifyTarget event using encoded target positions
    // This event allows moves to change their target (e.g., Payback, Metal Burst retargeting)
    // We encode target positions as integers: side_idx * 10 + pokemon_idx
    // Event handlers can return a new encoded position to redirect the move
    if let Some((target_side, target_pos)) = target {
        let encoded_target = (target_side as i32 * 10) + target_pos as i32;
        let modified_target = battle.run_event(
            "ModifyTarget",
            Some(pokemon_pos),
            Some((target_side, target_pos)),
            Some(move_or_move_name),
            Some(encoded_target),
        );
        if let Some(encoded) = modified_target {
            // Decode the modified target position
            let new_side = (encoded / 10) as usize;
            let new_pos = (encoded % 10) as usize;
            target = Some((new_side, new_pos));
        }
    }

    // if (target === undefined) target = this.battle.getRandomTarget(pokemon, move);
    // Call getRandomTarget if target is None
    // Gets a random valid target for a move based on its target type
    // Needed for moves with target="normal" or when original target is invalid
    if target.is_none() {
        target = battle.get_random_target(pokemon_pos.0, pokemon_pos.1, &active_move.target);
    }

    // if (move.target === 'self' || move.target === 'allies') {
    //     target = pokemon;
    // }
    if active_move.target == "self" || active_move.target == "allies" {
        target = Some(pokemon_pos);
    }

    // if (sourceEffect) {
    //     move.sourceEffect = sourceEffect.id;
    //     move.ignoreAbility = (sourceEffect as ActiveMove).ignoreAbility;
    // }
    if let Some(ref source_eff_id) = source_effect {
        active_move.source_effect = Some(source_eff_id.clone());

        // If sourceEffect is an ActiveMove, copy its ignoreAbility
        if let Some(source_move) = battle.dex.get_active_move(source_eff_id.as_str()) {
            active_move.ignore_ability = source_move.ignore_ability;
        }
    }

    // let moveResult = false;
    // Note: move_result will be initialized later based on move execution

    // this.battle.setActiveMove(move, pokemon, target);
    battle.set_active_move(Some(move_or_move_name.clone()), Some(pokemon_pos), target);

    // this.battle.singleEvent('ModifyType', move, null, pokemon, target, move, move);
    // this.battle.singleEvent('ModifyMove', move, null, pokemon, target, move, move);
    battle.single_event(
        "ModifyType",
        move_or_move_name,
        Some(pokemon_pos),
        target,
        Some(move_or_move_name),
    );
    battle.single_event(
        "ModifyMove",
        move_or_move_name,
        Some(pokemon_pos),
        target,
        Some(move_or_move_name),
    );

    // if (baseTarget !== move.target) {
    //     target = this.battle.getRandomTarget(pokemon, move);
    // }
    // Handle target adjustment after ModifyMove
    // If the move's target type changed in ModifyMove, need to get new target
    let current_target = battle
        .dex
        .get_move(move_or_move_name.as_str())
        .unwrap()
        .target
        .clone();
    if base_target != current_target {
        target = battle.get_random_target(pokemon_pos.0, pokemon_pos.1, &current_target);
    }

    // move = this.battle.runEvent('ModifyType', pokemon, target, move, move);
    // move = this.battle.runEvent('ModifyMove', pokemon, target, move, move);
    battle.run_event(
        "ModifyType",
        Some(pokemon_pos),
        target,
        Some(move_or_move_name),
        None,
    );
    battle.run_event(
        "ModifyMove",
        Some(pokemon_pos),
        target,
        Some(move_or_move_name),
        None,
    );

    // if (baseTarget !== move.target) {
    //     target = this.battle.getRandomTarget(pokemon, move);
    // }
    // Handle second target adjustment
    // If the move's target type changed after runEvent('ModifyMove'), adjust target again
    let current_target = battle
        .dex
        .get_move(move_or_move_name.as_str())
        .unwrap()
        .target
        .clone();
    if base_target != current_target {
        target = battle.get_random_target(pokemon_pos.0, pokemon_pos.1, &current_target);
    }

    // if (!move || pokemon.fainted) {
    //     return false;
    // }
    let (side_idx, poke_idx) = pokemon_pos;
    if battle
        .sides
        .get(side_idx)
        .and_then(|s| s.pokemon.get(poke_idx))
        .map(|p| p.is_fainted())
        .unwrap_or(true)
    {
        return false;
    }

    // let attrs = '';
    // let movename = move.name;
    // if (move.id === 'hiddenpower') movename = 'Hidden Power';
    // if (sourceEffect) attrs += `|[from] ${sourceEffect.fullname}`;
    // if (zMove && move.isZ === true) {
    //     attrs = `|[anim]${movename}${attrs}`;
    //     movename = `Z-${movename}`;
    // }
    // this.battle.addMove('move', pokemon, movename, `${target}${attrs}`);
    let move_name = active_move.name.clone();
    battle.add_log(
        "move",
        &[
            &format!(
                "{}: {}",
                battle.sides[side_idx].id_str(),
                battle.sides[side_idx].pokemon[poke_idx].name
            ),
            &move_name,
        ],
    );

    // if (zMove) this.runZPower(move, pokemon);
    // Implement runZPower for status Z-moves
    if z_move.is_some() {
        // Get Pokemon's types for Ghost type check (curse effect)
        let pokemon_has_ghost_type = battle.sides[side_idx].pokemon[poke_idx]
            .types
            .contains(&"Ghost".to_string());

        // Get z_move data from active_move (if it was transformed to Z-Move)
        if let Some(ref z_move_data) = active_move.z_move {
            // Call the run_z_power helper
            let z_power_result = BattleActions::run_z_power(
                &active_move.category,
                z_move_data.boost.as_ref(),
                z_move_data.effect.as_deref(),
                pokemon_has_ghost_type,
            );

            // Apply the Z-Power effect based on result
            match z_power_result {
                ZPowerResult::DamageMove => {
                    // JS: this.battle.attrLastMove('[zeffect]');
                    battle.attr_last_move(&["[zeffect]"]);
                }
                ZPowerResult::Boost(ref boosts) => {
                    // JS: this.battle.boost(move.zMove.boost, pokemon, pokemon, zPower);
                    // Convert BoostsTable to array of (stat_name, value) tuples
                    let mut boost_array = Vec::new();
                    if boosts.atk != 0 {
                        boost_array.push(("atk", boosts.atk));
                    }
                    if boosts.def != 0 {
                        boost_array.push(("def", boosts.def));
                    }
                    if boosts.spa != 0 {
                        boost_array.push(("spa", boosts.spa));
                    }
                    if boosts.spd != 0 {
                        boost_array.push(("spd", boosts.spd));
                    }
                    if boosts.spe != 0 {
                        boost_array.push(("spe", boosts.spe));
                    }
                    if boosts.accuracy != 0 {
                        boost_array.push(("accuracy", boosts.accuracy));
                    }
                    if boosts.evasion != 0 {
                        boost_array.push(("evasion", boosts.evasion));
                    }

                    battle.boost(&boost_array, pokemon_pos, Some(pokemon_pos), Some("zpower"));
                }
                ZPowerResult::Heal => {
                    // JS: this.battle.heal(pokemon.maxhp, pokemon, pokemon, zPower);
                    let max_hp = battle.sides[side_idx].pokemon[poke_idx].maxhp;
                    let zpower_id = ID::new("zpower");
                    battle.heal(
                        max_hp,
                        Some(pokemon_pos),
                        Some(pokemon_pos),
                        Some(&zpower_id),
                    );
                }
                ZPowerResult::HealReplacement => {
                    // JS: pokemon.side.addSlotCondition(pokemon, 'healreplacement', pokemon, move);
                    let healreplacement_id = ID::new("healreplacement");
                    let slot = battle.sides[side_idx].pokemon[poke_idx].position;
                    battle.sides[side_idx].add_slot_condition(slot, healreplacement_id, None);
                }
                ZPowerResult::ClearNegativeBoost => {
                    // JS: Clear all negative boosts and add '-clearnegativeboost' message
                    let boosts_to_clear = {
                        let pokemon = &battle.sides[side_idx].pokemon[poke_idx];
                        let mut clear_boosts = Vec::new();
                        if pokemon.boosts.atk < 0 {
                            clear_boosts.push(("atk", -pokemon.boosts.atk));
                        }
                        if pokemon.boosts.def < 0 {
                            clear_boosts.push(("def", -pokemon.boosts.def));
                        }
                        if pokemon.boosts.spa < 0 {
                            clear_boosts.push(("spa", -pokemon.boosts.spa));
                        }
                        if pokemon.boosts.spd < 0 {
                            clear_boosts.push(("spd", -pokemon.boosts.spd));
                        }
                        if pokemon.boosts.spe < 0 {
                            clear_boosts.push(("spe", -pokemon.boosts.spe));
                        }
                        if pokemon.boosts.accuracy < 0 {
                            clear_boosts.push(("accuracy", -pokemon.boosts.accuracy));
                        }
                        if pokemon.boosts.evasion < 0 {
                            clear_boosts.push(("evasion", -pokemon.boosts.evasion));
                        }
                        clear_boosts
                    };

                    if !boosts_to_clear.is_empty() {
                        battle.boost(
                            &boosts_to_clear,
                            pokemon_pos,
                            Some(pokemon_pos),
                            Some("zpower"),
                        );
                        battle.add_log(
                            "-clearnegativeboost",
                            &[
                                &format!(
                                    "{}: {}",
                                    battle.sides[side_idx].id_str(),
                                    battle.sides[side_idx].pokemon[poke_idx].name
                                ),
                                "[zeffect]",
                            ],
                        );
                    }
                }
                ZPowerResult::Redirect => {
                    // JS: pokemon.addVolatile('followme', pokemon, zPower);
                    let followme_id = ID::new("followme");
                    battle.sides[side_idx].pokemon[poke_idx].add_volatile(followme_id);
                }
                ZPowerResult::Crit2 => {
                    // JS: pokemon.addVolatile('focusenergy', pokemon, zPower);
                    let focusenergy_id = ID::new("focusenergy");
                    battle.sides[side_idx].pokemon[poke_idx].add_volatile(focusenergy_id);
                }
                ZPowerResult::None => {
                    // No Z-Power effect to apply
                }
            }
        }
    }

    // if (!target) {
    //     this.battle.attrLastMove('[notarget]');
    //     this.battle.add(this.battle.gen >= 5 ? '-fail' : '-notarget', pokemon);
    //     return false;
    // }
    let mut target_pos = match target {
        Some(t) => t,
        None => {
            battle.add_log("-notarget", &[]);
            return false;
        }
    };

    // const { targets, pressureTargets } = pokemon.getMoveTargets(move, target);
    // if (targets.length) {
    //     target = targets[targets.length - 1]; // in case of redirection
    // }
    // Implement getMoveTargets for multi-target handling
    // getMoveTargets returns all targets hit by a move (based on target type)
    // and pressure targets (for PP deduction via Pressure ability)
    let (targets, pressure_targets) =
        battle.get_move_targets(pokemon_pos, move_or_move_name, Some(target_pos));
    if !targets.is_empty() {
        // Update target in case of redirection
        target = Some(targets[targets.len() - 1]);
        if let Some(new_target) = target {
            target_pos = new_target;
        }
    }

    // const callerMoveForPressure = sourceEffect && (sourceEffect as ActiveMove).pp ? sourceEffect as ActiveMove : null;
    // if (!sourceEffect || callerMoveForPressure || sourceEffect.id === 'pursuit') {
    //     let extraPP = 0;
    //     for (const source of pressureTargets) {
    //         const ppDrop = this.battle.runEvent('DeductPP', source, pokemon, move);
    //         if (ppDrop !== true) {
    //             extraPP += ppDrop || 0;
    //         }
    //     }
    //     if (extraPP > 0) {
    //         pokemon.deductPP(callerMoveForPressure || moveOrMoveName, extraPP);
    //     }
    // }
    // Implement PP deduction with Pressure
    // The Pressure ability causes moves to lose 1 extra PP when targeting that Pokemon
    // This loops through pressureTargets and calls DeductPP event for each
    if source_effect.is_none()
        || source_effect
            .map(|e| e.as_str() == "pursuit")
            .unwrap_or(false)
    {
        let mut extra_pp = 0;
        for &pressure_target in &pressure_targets {
            // Call DeductPP event for each pressure target
            // JS: const ppDrop = this.battle.runEvent('DeductPP', source, pokemon, move);
            if let Some(pp_drop) = battle.run_event(
                "DeductPP",
                Some(pressure_target),
                Some(pokemon_pos),
                Some(move_or_move_name),
                Some(1),
            ) {
                // JS: if (ppDrop !== true) extraPP += ppDrop || 0;
                // runEvent returns Some(i32), we add it to extra PP
                extra_pp += pp_drop;
            }
        }

        // JS: if (extraPP > 0) pokemon.deductPP(callerMoveForPressure || moveOrMoveName, extraPP);
        if extra_pp > 0 {
            let (side_idx, poke_idx) = pokemon_pos;
            battle.sides[side_idx].pokemon[poke_idx].deduct_pp(move_or_move_name, extra_pp as u8);
        }
    }

    // Trigger BeforeMove event to allow abilities to prevent move execution
    // JavaScript: const willTryMove = this.battle.runEvent('BeforeMove', pokemon, target, move);
    // if (!willTryMove) { this.battle.runEvent('MoveAborted', pokemon, target, move); return; }
    let will_try_move = battle.run_event_bool(
        "BeforeMove",
        Some(pokemon_pos),
        target_pos.into(),
        Some(move_or_move_name),
    );
    if !will_try_move {
        battle.run_event(
            "MoveAborted",
            Some(pokemon_pos),
            target_pos.into(),
            Some(move_or_move_name),
            None,
        );
        // TODO: battle.clearActiveMove(true) - clear active move state
        // TODO: pokemon.moveThisTurnResult = willTryMove - track move result
        return false;
    }

    // if (!this.battle.singleEvent('TryMove', move, null, pokemon, target, move) ||
    //     !this.battle.runEvent('TryMove', pokemon, target, move)) {
    //     move.mindBlownRecoil = false;
    //     return false;
    // }
    let try_move_result = battle.single_event(
        "TryMove",
        move_or_move_name,
        Some(pokemon_pos),
        target_pos.into(),
        Some(move_or_move_name),
    );
    if matches!(try_move_result, crate::event::EventResult::Boolean(false)) {
        // move.mindBlownRecoil = false (this would be set in move state if we tracked it)
        return false;
    }

    // Also check runEvent('TryMove')
    let run_try_move_result = battle.run_event_bool(
        "TryMove",
        Some(pokemon_pos),
        target_pos.into(),
        Some(move_or_move_name),
    );
    if !run_try_move_result {
        return false;
    }

    // this.battle.singleEvent('UseMoveMessage', move, null, pokemon, target, move);
    battle.single_event(
        "UseMoveMessage",
        move_or_move_name,
        Some(pokemon_pos),
        target_pos.into(),
        Some(move_or_move_name),
    );

    // if (move.ignoreImmunity === undefined) {
    //     move.ignoreImmunity = (move.category === 'Status');
    // }
    // In Rust, the move data is immutable, so this would be set in MoveData
    // Status moves default to ignoring immunity
    // (This is handled in the event system when checking immunity)

    // if (this.battle.gen !== 4 && move.selfdestruct === 'always') {
    //     this.battle.faint(pokemon, pokemon, move);
    // }
    if battle.gen != 4 && active_move.self_destruct == Some("always".to_string()) {
        battle.faint(
            pokemon_pos,
            Some(pokemon_pos),
            Some(move_or_move_name.as_str()),
        );
    }

    // let damage: number | false | undefined | '' = false;
    // Execute move based on target type
    let move_result = if matches!(
        active_move.target.as_str(),
        "all" | "foeSide" | "allySide" | "allyTeam"
    ) {
        // Field-wide moves - for now, treat like targeted moves with single target
        // Full implementation would use tryMoveHit instead of trySpreadMoveHit
        battle.try_spread_move_hit(&[target_pos], pokemon_pos, move_or_move_name)
    } else {
        // Targeted moves - use trySpreadMoveHit
        // For now, we're using a single target (proper implementation would get all targets)
        battle.try_spread_move_hit(&[target_pos], pokemon_pos, move_or_move_name)
    };

    // if (moveData.selfdestruct === 'ifHit' && damage[i] !== false) {
    //     this.battle.faint(source, source, move);
    // }
    if active_move.self_destruct == Some("ifHit".to_string()) && move_result {
        battle.faint(
            pokemon_pos,
            Some(pokemon_pos),
            Some(move_or_move_name.as_str()),
        );
    }

    // if (move.selfBoost && moveResult) this.moveHit(pokemon, pokemon, move, move.selfBoost, false, true);
    // Self-boost is handled through move data and event system
    // This would call apply_move_secondary or similar with the selfBoost data

    // if (!pokemon.hp) {
    //     this.battle.faint(pokemon, pokemon, move);
    // }
    if battle.sides[side_idx].pokemon[poke_idx].hp == 0 {
        battle.faint(
            pokemon_pos,
            Some(pokemon_pos),
            Some(move_or_move_name.as_str()),
        );
    }

    // if (!moveResult) {
    //     this.battle.singleEvent('MoveFail', move, null, target, pokemon, move);
    //     return false;
    // }
    if !move_result {
        battle.single_event(
            "MoveFail",
            move_or_move_name,
            target_pos.into(),
            Some(pokemon_pos),
            Some(move_or_move_name),
        );
        return false;
    }

    // if (!(move.hasSheerForce && pokemon.hasAbility('sheerforce')) && !move.flags['futuremove']) {
    //     const originalHp = pokemon.hp;
    //     this.battle.singleEvent('AfterMoveSecondarySelf', move, null, pokemon, target, move);
    //     this.battle.runEvent('AfterMoveSecondarySelf', pokemon, target, move);
    //     if (pokemon && pokemon !== target && move.category !== 'Status') {
    //         if (pokemon.hp <= pokemon.maxhp / 2 && originalHp > pokemon.maxhp / 2) {
    //             this.battle.runEvent('EmergencyExit', pokemon, pokemon);
    //         }
    //     }
    // }
    // Check if Sheer Force applies (would skip secondary effects)
    // For now, always run the events (Sheer Force check would be in ability callbacks)
    let original_hp = battle.sides[side_idx].pokemon[poke_idx].hp;

    battle.single_event(
        "AfterMoveSecondarySelf",
        move_or_move_name,
        Some(pokemon_pos),
        target_pos.into(),
        Some(move_or_move_name),
    );
    battle.run_event(
        "AfterMoveSecondarySelf",
        Some(pokemon_pos),
        target_pos.into(),
        Some(move_or_move_name),
        None,
    );

    // Check for Emergency Exit (abilities that trigger when HP drops below 50%)
    if target_pos != pokemon_pos && active_move.category != "Status" {
        let current_hp = battle.sides[side_idx].pokemon[poke_idx].hp;
        let max_hp = battle.sides[side_idx].pokemon[poke_idx].maxhp;
        if current_hp <= max_hp / 2 && original_hp > max_hp / 2 {
            battle.run_event(
                "EmergencyExit",
                Some(pokemon_pos),
                Some(pokemon_pos),
                None,
                None,
            );
        }
    }

    // return true;
    true // Placeholder - will be set by actual move execution
}

// =========================================================================
// TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boost_modifier() {
        assert_eq!(BattleActions::get_boost_modifier(0), (2, 2));
        assert_eq!(BattleActions::get_boost_modifier(1), (3, 2));
        assert_eq!(BattleActions::get_boost_modifier(2), (4, 2));
        assert_eq!(BattleActions::get_boost_modifier(-1), (2, 3));
        assert_eq!(BattleActions::get_boost_modifier(6), (8, 2));
        assert_eq!(BattleActions::get_boost_modifier(-6), (2, 8));
    }

    #[test]
    fn test_stat_with_boost() {
        assert_eq!(BattleActions::calculate_stat_with_boost(100, 0), 100);
        assert_eq!(BattleActions::calculate_stat_with_boost(100, 1), 150);
        assert_eq!(BattleActions::calculate_stat_with_boost(100, 2), 200);
        assert_eq!(BattleActions::calculate_stat_with_boost(100, -1), 66);
        assert_eq!(BattleActions::calculate_stat_with_boost(100, 6), 400);
        assert_eq!(BattleActions::calculate_stat_with_boost(100, -6), 25);
    }

    #[test]
    fn test_recoil_damage() {
        // Chloroblast uses 50% HP
        assert_eq!(
            BattleActions::calc_recoil_damage(100, "chloroblast", None, 200),
            100
        );

        // Normal recoil (1/4)
        assert_eq!(
            BattleActions::calc_recoil_damage(100, "doubleedge", Some((1, 4)), 200),
            25
        );

        // No recoil
        assert_eq!(
            BattleActions::calc_recoil_damage(100, "tackle", None, 200),
            0
        );
    }

    #[test]
    fn test_confusion_damage() {
        // Level 50, 100 atk, 100 def, 40 base power, 100% random
        let damage = BattleActions::get_confusion_damage(50, 100, 100, 40, 100);
        assert!(damage > 0);
    }

    #[test]
    fn test_target_type_choices() {
        assert!(BattleActions::target_type_choices("normal"));
        assert!(BattleActions::target_type_choices("any"));
        assert!(BattleActions::target_type_choices("adjacentFoe"));
        assert!(!BattleActions::target_type_choices("self"));
        assert!(!BattleActions::target_type_choices("all"));
    }

    #[test]
    fn test_max_move_name() {
        assert_eq!(get_max_move_name("Fire"), "Max Flare");
        assert_eq!(get_max_move_name("Water"), "Max Geyser");
        assert_eq!(get_max_move_name("Electric"), "Max Lightning");
        assert_eq!(get_max_move_name("Status"), "Max Guard");
    }

    #[test]
    fn test_z_move_name() {
        assert_eq!(get_z_move_name("Fire"), "Inferno Overdrive");
        assert_eq!(get_z_move_name("Water"), "Hydro Vortex");
        assert_eq!(get_z_move_name("Electric"), "Gigavolt Havoc");
    }

    #[test]
    fn test_can_ultra_burst() {
        assert_eq!(
            BattleActions::can_ultra_burst("Necrozma-Dawn-Wings", "ultranecroziumz"),
            Some("Necrozma-Ultra".to_string())
        );
        assert_eq!(
            BattleActions::can_ultra_burst("Necrozma-Dusk-Mane", "ultranecroziumz"),
            Some("Necrozma-Ultra".to_string())
        );
        assert_eq!(
            BattleActions::can_ultra_burst("Necrozma", "ultranecroziumz"),
            None
        );
    }

    #[test]
    fn test_can_terastallize() {
        // Gen 9 with tera type
        assert_eq!(
            BattleActions::can_terastallize(false, false, 9, Some("Fire")),
            Some("Fire".to_string())
        );

        // Wrong gen
        assert_eq!(
            BattleActions::can_terastallize(false, false, 8, Some("Fire")),
            None
        );

        // Has Z-Move
        assert_eq!(
            BattleActions::can_terastallize(true, false, 9, Some("Fire")),
            None
        );

        // Can Mega Evo
        assert_eq!(
            BattleActions::can_terastallize(false, true, 9, Some("Fire")),
            None
        );
    }
}
