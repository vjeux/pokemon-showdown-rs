//! Simulator Pokemon
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module contains the Pokemon struct and related types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

use crate::dex_data::{BoostsTable, Gender, StatsTable, ID};
use crate::event_system::EffectState;

// Function modules
mod new;
mod update_move_z_flags;
mod fullname;
mod details;
mod is_fainted;
mod get_health;
mod take_damage;
mod heal;
mod set_status;
mod cure_status;
mod has_status;
mod add_volatile;
mod remove_volatile;
mod has_volatile;
mod get_volatile;
mod get_volatile_mut;
mod clear_volatiles;
mod get_stat;
mod boost;
mod clear_boosts;
mod update_speed;
mod clear_turn_state;
mod clear_switch_state;
mod get_slot;
mod can_switch;
mod get_move_pp;
mod deduct_pp;
mod set_pp;
mod has_move;
mod get_types;
mod is_grounded;
mod is_semi_invulnerable;
mod is_protected;
mod effective_weather;
mod has_item;
mod has_ability;
mod copy_volatile_from;
mod get_weight;
mod set_type;
mod add_type;
mod positive_boosts;
mod get_action_speed;
mod disable_move;
mod get_usable_moves;
mod can_tera;
mod terastallize;
mod get_updated_details;
mod calculate_stat;
mod get_best_stat;
mod has_type;
mod faint;
mod damage;
mod is_ally;
mod is_adjacent;
mod get_capped_boost;
mod boost_by;
mod set_boost;
mod clear_ability;
mod set_ability;
mod get_ability;
mod clear_item;
mod set_item;
mod take_item;
mod get_item;
mod set_hp;
mod move_used;
mod is_last_active;
mod ignoring_ability;
mod ignoring_item;
mod update_max_hp;
mod is_sky_dropped;
mod get_move_data;
mod get_move_data_mut;
mod get_undynamaxed_hp;
mod try_trap;
mod got_attacked;
mod get_locked_move;
mod max_move_disabled;
mod transform_into;
mod copy_volatile_from_full;
mod set_species;
mod forme_change;
mod get_base_species_name;
mod get_base_species_base_species;
mod get_forme;
mod get_base_species_num;
mod get_moves;
mod get_base_moves;
mod to_json;
mod get_combat_power;
mod get_loc_of;
mod get_at_loc;
mod get_smart_targets;
mod get_last_attacked_by;
mod get_last_damaged_by;
mod get_dynamax_request;
mod get_move_request_data;
mod get_switch_request_data;
mod try_set_status;
mod use_item;
mod eat_item;
mod run_effectiveness;
mod run_immunity;
mod run_status_immunity;
mod remove_linked_volatiles;
mod clear_volatile_full;
mod get_nature;
mod get_status;
mod destroy;
mod allies_and_self_stub;
mod allies_stub;
mod adjacent_allies_stub;
mod foes_stub;
mod adjacent_foes_stub;
mod clear_status;
mod get_move_hit_data;
mod get_move_targets_stub;

/// A Pokemon's move slot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveSlot {
    pub id: ID,
    pub move_name: String,
    pub pp: u8,
    pub maxpp: u8,
    pub target: Option<String>,
    pub disabled: bool,
    pub disabled_source: Option<String>,
    pub used: bool,
    pub virtual_move: bool,
    pub is_z: bool,
}

impl MoveSlot {
    pub fn new(id: ID, move_name: String, pp: u8, maxpp: u8) -> Self {
        Self {
            id,
            move_name,
            pp,
            maxpp,
            target: None,
            disabled: false,
            disabled_source: None,
            used: false,
            virtual_move: false,
            is_z: false,
        }
    }
}

/// Record of a Pokemon that attacked this Pokemon
/// Equivalent to Attacker interface in pokemon.ts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attacker {
    /// Source Pokemon (side_idx, poke_idx)
    pub source: (usize, usize),
    /// Damage dealt
    pub damage: i32,
    /// Whether this attack happened this turn
    pub this_turn: bool,
    /// Move ID used
    pub move_id: Option<ID>,
    /// Source slot
    pub slot: (usize, usize),
}

/// Pokemon set - the team builder representation of a Pokemon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonSet {
    pub name: String,
    pub species: String,
    pub item: String,
    pub ability: String,
    pub moves: Vec<String>,
    pub nature: String,
    pub gender: Gender,
    pub evs: StatsTable,
    pub ivs: StatsTable,
    pub level: u8,
    pub shiny: bool,
    pub happiness: u8,
    pub pokeball: String,
    pub hptype: Option<String>,
    pub dynamax_level: u8,
    pub gigantamax: bool,
    pub tera_type: Option<String>,
}

impl Default for PokemonSet {
    fn default() -> Self {
        Self {
            name: String::new(),
            species: String::new(),
            item: String::new(),
            ability: String::new(),
            moves: Vec::new(),
            nature: "Hardy".to_string(),
            gender: Gender::None,
            evs: StatsTable::default(),
            ivs: StatsTable::new(31, 31, 31, 31, 31, 31),
            level: 100,
            shiny: false,
            happiness: 255,
            pokeball: String::new(),
            hptype: None,
            dynamax_level: 10,
            gigantamax: false,
            tera_type: None,
        }
    }
}

/// A Pokemon in battle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pokemon {
    // Identity
    pub name: String,
    pub species_id: ID,
    pub base_species: ID,
    pub level: u8,
    pub gender: Gender,
    pub nature: String,
    pub happiness: u8,
    pub pokeball: ID,
    pub dynamax_level: u8,
    pub gigantamax: bool,

    // Position
    pub position: usize,
    pub side_index: usize,
    pub is_active: bool,

    // Stats
    pub base_stored_stats: StatsTable,
    pub stored_stats: StatsTable,
    pub boosts: BoostsTable,
    pub maxhp: i32,
    pub base_maxhp: i32,
    pub hp: i32,
    pub max_hp_undynamaxed: i32,

    // Ability
    pub base_ability: ID,
    pub ability: ID,
    pub ability_state: EffectState,

    // Item
    pub item: ID,
    pub item_state: EffectState,
    pub last_item: ID,
    pub used_item_this_turn: bool,
    pub ate_berry: bool,
    pub item_knocked_off: bool,

    // Choice item lock (Choice Band/Scarf/Specs)
    pub locked_move: Option<ID>,

    // Types
    pub types: Vec<String>,
    pub added_type: Option<String>,
    pub base_types: Vec<String>,
    pub known_type: Option<String>, // Known type for illusion/disguise mechanics
    pub apparent_type: Option<String>, // Type string shown to players (for type reveal mechanic)

    // Tera
    pub tera_type: Option<String>,
    pub terastallized: Option<String>,
    pub can_terastallize: Option<String>,

    // Move slots
    pub base_move_slots: Vec<MoveSlot>,
    pub move_slots: Vec<MoveSlot>,

    // Hidden Power type
    pub hp_type: Option<String>,

    // Status
    pub status: ID,
    pub status_state: EffectState,
    pub volatiles: HashMap<ID, EffectState>,

    // Battle state
    pub fainted: bool,
    pub faint_queued: bool,
    pub transformed: bool,
    pub illusion: Option<usize>, // Index of pokemon providing illusion

    // Flags
    pub trapped: bool,
    pub maybe_trapped: bool,
    pub maybe_disabled: bool,
    pub maybe_locked: bool, // Choice items may lock next turn
    pub switch_flag: bool,
    pub force_switch_flag: bool,
    pub newly_switched: bool,
    pub being_called_back: bool,
    pub dragged_in: Option<usize>,
    pub skip_before_switch_out_event_flag: bool,
    pub stats_raised_this_turn: bool,
    pub stats_lowered_this_turn: bool,
    pub sub_fainted: bool, // Gen 1: Substitute fainted (different from main faint)

    // Ability-specific flags
    pub sword_boost: bool, // Intrepid Sword / Dauntless Shield

    // Turn state
    pub last_move: Option<ID>,
    pub last_move_used: Option<ID>,
    pub last_move_target_loc: Option<i8>,
    pub move_this_turn: Option<ID>,
    pub move_this_turn_result: Option<bool>,
    pub move_last_turn_result: Option<bool>,
    pub hurt_this_turn: Option<i32>,
    pub last_damage: i32,
    pub times_attacked: i32,

    // Counters
    pub active_turns: i32,
    pub active_move_actions: i32,
    pub previously_switched_in: i32,
    pub is_started: bool,
    pub during_move: bool,

    // Attack tracking for revenge/payback/etc mechanics
    pub attacked_by: Vec<Attacker>,

    // Calculated values
    pub weight_hg: i32,
    pub speed: i32,

    // Mega/Dynamax state
    pub can_mega_evo: Option<String>,
    pub can_ultra_burst: Option<String>,
    pub can_gigantamax: Option<String>,

    // Stellar boost tracking (Gen 9)
    pub stellar_boosted_types: Vec<String>,

    // Staleness tracking for endless battle clause
    pub staleness: Option<String>,
    pub pending_staleness: Option<String>,
    pub volatile_staleness: Option<String>,
}

impl Pokemon {
}

/// Result of getMoveTargets
#[derive(Debug, Clone, Default)]
pub struct GetMoveTargetsResult {
    /// Target pokemon indices (side_index, position)
    pub targets: Vec<(usize, usize)>,
    /// Pressure targets for PP deduction
    pub pressure_targets: Vec<(usize, usize)>,
}

/// Move hit data for tracking crit, type effectiveness, etc.
#[derive(Debug, Clone, Default)]
pub struct MoveHitData {
    /// Was this hit a critical hit?
    pub crit: bool,
    /// Type effectiveness modifier (-2 to 2, for 0.25x to 4x)
    pub type_mod: i8,
    /// Actual damage dealt
    pub damage: i32,
    /// Did the move hit the substitute instead?
    pub hit_substitute: bool,
}

// =============================================================================
// Display implementation for Pokemon
// =============================================================================
// In JS: this.fullname = `${this.side.id}: ${this.name}`;
// and toString() returns the position + name for active pokemon

impl fmt::Display for Pokemon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format: "p1a: Pikachu" for active, "p1: Pikachu" for inactive
        let side_id = format!("p{}", self.side_index + 1);
        if self.is_active {
            // Active pokemon include position letter (a, b, c, etc.)
            let pos_letter = (b'a' + self.position as u8) as char;
            write!(f, "{}{}: {}", side_id, pos_letter, self.name)
        } else {
            write!(f, "{}: {}", side_id, self.name)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pokemon_creation() {
        let set = PokemonSet {
            name: "Pikachu".to_string(),
            species: "Pikachu".to_string(),
            item: "Light Ball".to_string(),
            ability: "Static".to_string(),
            moves: vec!["Thunderbolt".to_string(), "Volt Tackle".to_string()],
            level: 50,
            gender: Gender::Male,
            ..Default::default()
        };
        let pokemon = Pokemon::new(&set, 0, 0);

        assert_eq!(pokemon.name, "Pikachu");
        assert_eq!(pokemon.level, 50);
        assert_eq!(pokemon.gender, Gender::Male);
        assert_eq!(pokemon.move_slots.len(), 2);
    }

    #[test]
    fn test_damage_and_heal() {
        let set = PokemonSet {
            name: "Pikachu".to_string(),
            species: "Pikachu".to_string(),
            ..Default::default()
        };
        let mut pokemon = Pokemon::new(&set, 0, 0);
        pokemon.hp = 100;
        pokemon.maxhp = 100;

        let damage = pokemon.take_damage(30);
        assert_eq!(damage, 30);
        assert_eq!(pokemon.hp, 70);

        let healed = pokemon.heal(50);
        assert_eq!(healed, 30); // Can only heal up to maxhp
        assert_eq!(pokemon.hp, 100);
    }

    #[test]
    fn test_status() {
        let set = PokemonSet {
            name: "Pikachu".to_string(),
            species: "Pikachu".to_string(),
            ..Default::default()
        };
        let mut pokemon = Pokemon::new(&set, 0, 0);

        assert!(pokemon.set_status(ID::new("par")));
        assert!(pokemon.has_status("par"));
        assert!(!pokemon.set_status(ID::new("brn"))); // Already has status

        assert!(pokemon.cure_status().is_some());
        assert!(!pokemon.has_status("par"));
    }

    #[test]
    fn test_volatiles() {
        let set = PokemonSet {
            name: "Pikachu".to_string(),
            species: "Pikachu".to_string(),
            ..Default::default()
        };
        let mut pokemon = Pokemon::new(&set, 0, 0);

        let confusion = ID::new("confusion");
        assert!(pokemon.add_volatile(confusion.clone()));
        assert!(pokemon.has_volatile(&confusion));
        assert!(!pokemon.add_volatile(confusion.clone())); // Already has it

        assert!(pokemon.remove_volatile(&confusion));
        assert!(!pokemon.has_volatile(&confusion));
    }
}
