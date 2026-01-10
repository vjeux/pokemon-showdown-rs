//! Simulator Side
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Represents one side (player) in a battle.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::dex_data::{SideID, ID};
use crate::event_system::EffectState;
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
    /// Choice type (move, switch, etc.)
    /// JavaScript: choice: 'move' | 'switch' | 'instaswitch' | 'revivalblessing' | 'team' | 'shift' | 'pass'
    /// TODO: Rust uses enum instead of string union
    pub choice: ChoiceType,

    /// Pokemon making this choice
    /// JavaScript: pokemon?: Pokemon
    /// TODO: Rust uses pokemon_index instead of Pokemon reference due to ownership
    pub pokemon_index: usize,

    /// Target location (-3 to 3)
    /// JavaScript: targetLoc?: number
    pub target_loc: Option<i8>,
    /// Move ID being used
    /// JavaScript: moveid?: ID
    pub move_id: Option<ID>,

    /// ActiveMove for this action
    /// JavaScript: move?: ActiveMove
    pub move_action: Option<crate::battle_actions::ActiveMove>,

    /// Target Pokemon
    /// JavaScript: target?: Pokemon
    /// TODO: Rust uses target_loc instead of Pokemon reference due to ownership

    /// Switch/team index
    /// JavaScript: index?: number
    pub switch_index: Option<usize>,

    /// Side making this choice
    /// JavaScript: side?: Side
    /// TODO: Rust can infer from pokemon_index, Side reference not stored

    /// Mega Evolution flag
    /// JavaScript: mega?: boolean
    pub mega: bool,

    /// Mega Evolution X form
    /// JavaScript: megax?: boolean | null
    pub megax: Option<bool>,

    /// Mega Evolution Y form
    /// JavaScript: megay?: boolean | null
    pub megay: Option<bool>,

    /// Z-Move being used
    /// JavaScript: zmove?: string
    pub zmove: Option<String>,
    /// Max Move being used
    /// JavaScript: maxMove?: string
    pub max_move: Option<String>,
    /// Terastallize type
    /// JavaScript: terastallize?: string
    pub terastallize: Option<String>,

    /// Move priority (for ordering)
    /// JavaScript: priority?: number
    pub priority: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// JavaScript equivalent: ChosenAction.choice type (sim/side.ts)
/// JavaScript: 'move' | 'switch' | 'instaswitch' | 'revivalblessing' | 'team' | 'shift' | 'pass'
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
/// JavaScript equivalent: Choice (sim/side.ts)
/// 11 fields in JavaScript
pub struct Choice {
    /// Cannot undo this choice
    /// JavaScript: cantUndo: boolean
    pub cant_undo: bool,
    /// Error message if choice is invalid
    /// JavaScript: error: string
    pub error: String,
    /// Chosen actions for active Pokemon
    /// JavaScript: actions: ChosenAction[]
    pub actions: Vec<ChosenAction>,
    /// Number of forced switches remaining
    /// JavaScript: forcedSwitchesLeft: number
    pub forced_switches_left: usize,
    /// Number of forced passes remaining
    /// JavaScript: forcedPassesLeft: number
    pub forced_passes_left: usize,
    /// Pokemon indices switching in
    /// JavaScript: switchIns: Set<number>
    /// TODO: Rust uses Vec instead of Set
    pub switch_ins: Vec<usize>,
    /// Using a Z-Move this turn
    /// JavaScript: zMove: boolean
    pub z_move: bool,
    /// Using Mega Evolution this turn
    /// JavaScript: mega: boolean
    pub mega: bool,
    /// Using Ultra Burst this turn
    /// JavaScript: ultra: boolean
    pub ultra: bool,
    /// Using Dynamax this turn
    /// JavaScript: dynamax: boolean
    pub dynamax: bool,
    /// Using Terastallize this turn
    /// JavaScript: terastallize: boolean
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
pub struct Side {
    // Core references (readonly in JavaScript)
    // TODO: These should be references, not indices
    // pub battle: &Battle - needs lifetime management
    // pub foe: &Side - use foe_index instead
    // pub ally_side: Option<&Side> - use ally_index instead

    /// Side ID (p1, p2, p3, p4)
    /// JavaScript: readonly id: SideID
    pub id: SideID,
    /// Index in battle.sides
    /// JavaScript: readonly n: number
    pub n: usize,

    /// Player name
    /// JavaScript: name: string
    pub name: String,
    /// Player avatar
    /// JavaScript: avatar: string
    pub avatar: String,

    /// The team (PokemonSets)
    /// JavaScript: team: PokemonSet[]
    pub team: Vec<PokemonSet>,
    /// The Pokemon objects
    /// JavaScript: pokemon: Pokemon[]
    pub pokemon: Vec<Pokemon>,
    /// Currently active Pokemon (indices into self.pokemon)
    /// JavaScript: active: Pokemon[]
    /// TODO: Rust uses Option<usize> indices instead of Pokemon references due to ownership
    pub active: Vec<Option<usize>>,

    /// Number of Pokemon left (not fainted)
    /// JavaScript: pokemonLeft: number
    pub pokemon_left: usize,
    /// Whether Z-move has been used
    /// JavaScript: zMoveUsed: boolean
    pub z_move_used: bool,
    /// Whether Dynamax has been used
    /// JavaScript: dynamaxUsed: boolean
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
    /// JavaScript: totalFainted: number
    pub total_fainted: usize,

    /// Last selected move (Gen 1 only)
    /// JavaScript: lastSelectedMove: ID
    pub last_selected_move: ID,
    /// Last move used (Gen 1)
    /// JavaScript: lastMove: Move | null
    /// TODO: Rust uses ID instead of Move object
    pub last_move: Option<ID>,

    /// Side conditions (Stealth Rock, Spikes, etc.)
    /// JavaScript: sideConditions: {[id: string]: EffectState}
    pub side_conditions: HashMap<ID, EffectState>,
    /// Slot conditions (per-slot effects)
    /// JavaScript: slotConditions: {[id: string]: EffectState}[]
    pub slot_conditions: Vec<HashMap<ID, EffectState>>,

    /// Current request state
    /// JavaScript: requestState: 'teamPreview' | 'move' | 'switch' | ''
    /// TODO: Rust uses enum instead of string union
    pub request_state: RequestState,
    /// Active request sent to player
    /// JavaScript: activeRequest: AnyObject | null
    #[serde(skip)]
    pub active_request: Option<crate::choice::BattleRequest>,
    /// Current choice
    /// JavaScript: choice: Choice
    pub choice: Choice,

    // TODO: DELETE - Not in JavaScript Side (JavaScript has foe: Side, not foe_index)
    /// Foe side index (used instead of direct reference)
    pub foe_index: Option<usize>,
    // TODO: DELETE - Not in JavaScript Side (JavaScript has allySide: Side | null, not ally_index)
    /// Ally side index (multi battles, used instead of direct reference)
    pub ally_index: Option<usize>,
}

impl Side {
}
