//! Simulator Side
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Represents one side (player) in a battle.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::dex_data::{EffectState, SideID, ID};
use crate::pokemon::{Pokemon, PokemonSet};

// Function modules
mod new;
mod id_str;
mod get_pokemon;
mod get_pokemon_mut;
mod get_active;
mod get_active_mut;
mod all_active;
mod active_count;
mod switch_in;
mod switch_out;
mod add_side_condition;
mod has_side_condition;
mod get_side_condition;
mod get_side_condition_mut;
mod remove_side_condition;
mod add_slot_condition;
mod has_slot_condition;
mod remove_slot_condition;
mod count_unfainted;
mod has_lost;
mod get_switchable;
mod faint_pokemon;
mod clear_turn_state;
mod get_choice;
mod is_choice_done;
mod clear_choice;
mod can_dynamax_now;
mod allies;
mod foes_active;
mod has_ally;
mod add_pokemon;
mod random_foe;
mod foe_pokemon_left;
mod get_slot_condition;
mod get_slot_condition_mut;
mod get_choice_index;
mod choose_pass;
mod choose_switch;
mod choose_move;
mod choose_team;
mod choose_shift;
mod auto_choose;
mod picked_team_size;
mod destroy;
mod active_team;
mod emit_choice_error;
mod emit_request;
mod foes;
mod foe_sides_with_conditions;
mod get_request_data;
mod get_side_condition_data;
mod send;
mod to_json;
mod update_disabled_request;
mod update_request_for_pokemon;
mod choose;

/// A single action that can be chosen
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: ChosenAction (sim/side.ts)
/// 15 fields in JavaScript
pub struct ChosenAction {
    pub choice: ChoiceType,

    // TODO: In JavaScript this is pokemon: Pokemon (optional reference)
    // Rust uses pokemon_index instead
    pub pokemon_index: usize,

    pub target_loc: Option<i8>,
    pub move_id: Option<ID>,

    /// ActiveMove for this action
    /// JavaScript: move?: ActiveMove
    pub move_action: Option<crate::battle_actions::ActiveMove>,

    // TODO: In JavaScript this is target: Pokemon (optional reference)
    // Rust uses target_loc instead

    // TODO: In JavaScript this is index: number (optional)
    // May correspond to switch_index in Rust?
    pub switch_index: Option<usize>,

    // TODO: In JavaScript this is side: Side (optional reference)
    // Rust can infer from pokemon_index

    pub mega: bool,

    /// Mega Evolution X form
    /// JavaScript: megax?: boolean | null
    pub megax: Option<bool>,

    /// Mega Evolution Y form
    /// JavaScript: megay?: boolean | null
    pub megay: Option<bool>,

    pub zmove: Option<String>,
    pub max_move: Option<String>,
    pub terastallize: Option<String>,

    /// Move priority (for ordering)
    /// JavaScript: priority?: number
    pub priority: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChoiceType {
    Move,
    Switch,
    InstaSwitch,
    RevivalBlessing,
    Team,
    Shift,
    Pass,
}

/// One single turn's choice for one single player
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Choice {
    pub cant_undo: bool,
    pub error: String,
    pub actions: Vec<ChosenAction>,
    pub forced_switches_left: usize,
    pub forced_passes_left: usize,
    pub switch_ins: Vec<usize>,
    pub z_move: bool,
    pub mega: bool,
    pub ultra: bool,
    pub dynamax: bool,
    pub terastallize: bool,
}

impl Choice {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

/// Request state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
/// JavaScript equivalent: RequestState (sim/battle.ts)
pub enum RequestState {
    #[default]
    None,
    TeamPreview,
    Move,
    Switch,
}

/// A side in the battle
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: Side (sim/side.ts)
/// 77 fields in JavaScript
/// JavaScript equivalent: Side (sim/global-types.ts)
pub struct Side {
    // Core references (readonly in JavaScript)
    // TODO: These should be references, not indices
    // pub battle: &Battle - needs lifetime management
    // pub foe: &Side - use foe_index instead
    // pub ally_side: Option<&Side> - use ally_index instead

    /// Side ID (p1, p2, p3, p4)
    pub id: SideID,
    /// Index in battle.sides
    pub n: usize,

    /// Player name
    pub name: String,
    /// Player avatar
    pub avatar: String,

    /// The team (PokemonSets)
    pub team: Vec<PokemonSet>,
    /// The Pokemon objects
    pub pokemon: Vec<Pokemon>,
    /// Currently active Pokemon (indices into self.pokemon)
    pub active: Vec<Option<usize>>,

    /// Number of Pokemon left (not fainted)
    pub pokemon_left: usize,
    /// Whether Z-move has been used
    pub z_move_used: bool,
    /// Whether Dynamax has been used
    pub dynamax_used: bool,

    /// Last Pokemon to faint last turn
    /// JavaScript: faintedLastTurn: Pokemon | null
    /// TODO: Rust uses index instead of Pokemon reference due to ownership
    pub fainted_last_turn: Option<usize>,
    /// Last Pokemon to faint this turn
    /// JavaScript: faintedThisTurn: Pokemon | null
    /// TODO: Rust uses index instead of Pokemon reference due to ownership
    pub fainted_this_turn: Option<usize>,
    /// Total Pokemon fainted
    pub total_fainted: usize,

    /// Last selected move (Gen 1 only)
    pub last_selected_move: ID,
    /// Last move used (Gen 1)
    /// JavaScript: lastMove: Move | null
    /// TODO: Rust uses ID instead of Move object
    pub last_move: Option<ID>,

    /// Side conditions (Stealth Rock, Spikes, etc.)
    pub side_conditions: HashMap<ID, EffectState>,
    /// Slot conditions (per-slot effects)
    pub slot_conditions: Vec<HashMap<ID, EffectState>>,

    /// Current request state
    pub request_state: RequestState,
    /// Active request sent to player
    #[serde(skip)]
    pub active_request: Option<crate::choice::BattleRequest>,
    /// Current choice
    pub choice: Choice,

    /// Foe side index (used instead of direct reference)
    pub foe_index: Option<usize>,
    /// Ally side index (multi battles, used instead of direct reference)
    pub ally_index: Option<usize>,
}

impl Side {
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_team() -> Vec<PokemonSet> {
        vec![
            PokemonSet {
                name: "Pikachu".to_string(),
                species: "Pikachu".to_string(),
                ability: "Static".to_string(),
                moves: vec!["Thunderbolt".to_string()],
                ..Default::default()
            },
            PokemonSet {
                name: "Charizard".to_string(),
                species: "Charizard".to_string(),
                ability: "Blaze".to_string(),
                moves: vec!["Flamethrower".to_string()],
                ..Default::default()
            },
        ]
    }

    #[test]
    fn test_side_creation() {
        let team = create_test_team();
        let side = Side::new(SideID::P1, 0, "Test Player".to_string(), team, 1);

        assert_eq!(side.name, "Test Player");
        assert_eq!(side.pokemon.len(), 2);
        assert_eq!(side.pokemon_left, 2);
        assert_eq!(side.active.len(), 1);
    }

    #[test]
    fn test_switch_in_out() {
        let team = create_test_team();
        let mut side = Side::new(SideID::P1, 0, "Test".to_string(), team, 1);

        // Switch in first Pokemon
        assert!(side.switch_in(0, 0));
        assert!(side.get_active(0).is_some());
        assert_eq!(side.active_count(), 1);

        // Switch to second Pokemon
        assert!(side.switch_in(0, 1));
        assert_eq!(side.get_active(0).unwrap().name, "Charizard");
    }

    #[test]
    fn test_side_conditions() {
        let team = create_test_team();
        let mut side = Side::new(SideID::P1, 0, "Test".to_string(), team, 1);

        let rocks = ID::new("stealthrock");
        assert!(side.add_side_condition(rocks.clone(), None));
        assert!(side.has_side_condition(&rocks));
        assert!(!side.add_side_condition(rocks.clone(), None)); // Already exists

        assert!(side.remove_side_condition(&rocks));
        assert!(!side.has_side_condition(&rocks));
    }

    #[test]
    fn test_faint() {
        let team = create_test_team();
        let mut side = Side::new(SideID::P1, 0, "Test".to_string(), team, 1);

        side.switch_in(0, 0);
        assert_eq!(side.count_unfainted(), 2);

        side.faint_pokemon(0);
        assert_eq!(side.count_unfainted(), 1);
        assert_eq!(side.pokemon_left, 1);
        assert!(side.active[0].is_none());
    }
}
