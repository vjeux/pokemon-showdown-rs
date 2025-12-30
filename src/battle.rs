//! Simulator Battle
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This file is where the battle simulation itself happens.
//! The most important part of the simulation is the event system.

use serde::{Deserialize, Serialize};
use std::collections::{HashSet};

use crate::battle_queue::BattleQueue;
use crate::dex_data::{GameType, SideID, ID};
use crate::event_system::EffectState;
use crate::field::Field;
use crate::pokemon::{Pokemon, PokemonSet};
use crate::prng::{PRNGSeed, PRNG};
use crate::side::{Side};

/// Split message for side-specific content
/// JavaScript equivalent: { side: SideID, secret: string, shared: string }

// Function modules

// Function modules
mod new;
mod pokemon_at;
mod pokemon_at_mut;
mod init_pokemon_stats;
mod set_player;
mod start;
mod start_battle;
mod switch_in;
mod each_event_internal;
mod add_log;
mod random;
mod random_range;
mod random_chance;
mod sample;
mod shuffle;
mod get_side;
mod get_side_mut;
mod p1;
mod p2;
mod get_all_active;
mod check_win;
mod get_foe_pokemon_left;
mod end;
mod next_effect_order;
mod init_effect_state;
mod choose;
mod validate_single_choice;
mod get_log;
mod make_choices;
mod parse_choice;
mod resolve_action;
mod commit_choices;
mod find_valid_switch_target;
mod do_switch;
mod drag_in;
mod do_switch_with_drag;
mod compare_action_priority;
mod insert_run_switch_action;
mod insert_field_action;
mod run_switch;
mod field_event_switch_in;
mod trigger_switch_in_abilities;
mod apply_hazards;
mod run_move;
mod get_move_target;
mod get_move_priority;
mod get_multi_hit_count;
mod get_move_accuracy;
mod apply_confusion;
mod remove_all_hazards;
mod apply_status;
mod cure_status;
mod apply_boost;
mod run_residual;
mod faint_messages;
mod next_turn;
mod debug;
mod get_debug_log;
mod add;
mod add_move;
mod hint;
mod trunc;
mod chain;
mod chain_f;
mod modify;
mod modify_tuple;
mod modify_f;
mod clamp_int_range;
mod event_modifier;
mod tie;
mod win;
mod lose;
mod force_win;
mod set_active_move;
mod clear_active_move;
mod suppressing_ability;
mod get_all_pokemon;
mod is_adjacent;
mod adjacent_allies;
mod adjacent_foes;
mod allies_and_self;
mod foes;
mod get_move_targets;
mod is_ally;
mod is_pokemon_fainted;
mod get_random_target;
mod update_speed;
mod damage;
mod direct_damage;
mod add_direct_damage_log;
mod heal;
mod add_heal_log;
mod boost;
mod boost_stats;
mod faint;
mod check_fainted;
mod compare_priority;
mod compare_redirect_order;
mod compare_left_to_right_order;
mod speed_sort;
mod shuffle_range;
mod get_pokemon;
mod get_pokemon_mut;
mod can_switch;
mod get_random_switchable;
mod possible_switches;
mod get_loc_of;
mod valid_target_loc;
mod valid_target;
mod get_at_slot;
mod end_turn;
mod turn_loop;
mod run_action;
mod all_choices_done;
mod check_move_makes_contact;
mod get_action_speed;
mod get_pokemon_action_speed;
mod swap_position;
mod get_category;
mod clear_request;
mod make_request;
mod maybe_trigger_endless_battle_clause;
mod restart;
mod reset_rng;
mod join;
mod destroy;
mod single_event;
mod get_effect_type;
mod dispatch_single_event;
mod handle_ability_event;
mod handle_item_event;
mod handle_move_event;
mod check_volatile_try_hit;
mod handle_condition_event;
mod handle_side_condition_event;
mod run_event;
mod run_event_float;
mod run_event_bool;
mod run_event_string;
mod find_event_handlers;
mod priority_event;
mod get_event_modifier;
mod set_event_modifier;
mod randomizer;
mod each_event;
mod get_target;
mod get_at_loc;
mod undo_choice;
mod spread_damage;
mod add_damage_log;
mod final_modify;
mod modify_internal;
mod add_split;
mod attr_last_move;
mod chain_modify;
mod chain_modify_fraction;
mod check_ev_balance;
mod clear_effect_state;
mod debug_error;
mod field_event;
mod get_callback;
mod get_overflowed_turn_count;
mod get_requests;
mod get_team;
mod resolve_priority;
mod retarget_last_move;
mod run_pick_team;
mod send_updates;
mod show_open_team_sheets;
mod spread_modify;
mod stat_modify;
mod tiebreak;
mod to_json;
mod find_battle_event_handlers;
mod find_field_event_handlers;
mod find_side_event_handlers;
mod find_pokemon_event_handlers;
mod get_damage;
mod get_type_effectiveness_mod;
mod modify_damage;
mod try_spread_move_hit;
mod spread_move_hit;
mod get_spread_damage;
mod run_move_effects;
mod on_event;
mod on_event_priority;
mod run_custom_event_handlers;

pub struct SplitMessage {
    pub side: SideID,
    pub secret: String,
    pub shared: String,
}

/// Argument type for battle.add() - can be a Pokemon reference or a string
/// This allows mixing types like: battle.add("-activate", &[pokemon.into(), "ability: Immunity".into()])
pub enum Arg<'a> {
    Pokemon(&'a Pokemon),
    Str(&'a str),
    String(String),
    /// Function that returns split message for side-specific content
    /// JavaScript equivalent: () => { side: SideID, secret: string, shared: string }
    SplitFn(Box<dyn Fn() -> SplitMessage + 'a>),
}

/// Event listener - matches JavaScript EventListener interface
/// JavaScript: interface EventListener extends EventListenerWithoutPriority
#[derive(Clone)]
pub struct EventListener {
    /// Effect that owns this handler
    pub effect_id: ID,
    /// Type of effect (Ability, Item, Move, Status, etc.)
    pub effect_type: EffectType,
    /// Target Pokemon (optional)
    pub target: Option<(usize, usize)>,
    /// Index for multi-target events
    pub index: Option<usize>,
    /// Effect state
    pub state: Option<EffectState>,
    /// Effect holder (Pokemon/Side/Field/Battle)
    pub effect_holder: Option<(usize, usize)>,
    /// Order value (false = first in JS, represented as Option<i32>)
    pub order: Option<i32>,
    /// Priority value (higher = earlier)
    pub priority: i32,
    /// Sub-order for same priority
    pub sub_order: i32,
    /// Effect order (for hazards and abilities with same priority)
    pub effect_order: Option<i32>,
    /// Speed stat (for speed-based sorting)
    pub speed: Option<i32>,
}

/// Effect type - matches JavaScript effectType
/// Used to determine event handler priority ordering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectType {
    ZMove,
    Condition,
    SlotCondition,
    SideCondition,
    FieldCondition,
    Weather,
    Format,
    Rule,
    Ruleset,
    Ability,
    Item,
    Move,
    Status,
}

/// Type alias for event callback functions
pub type EventCallback = Box<dyn Fn(&EventContext) -> Option<i32> + Send + Sync>;

/// Type alias for move targets result (targets, pressure_targets)
pub type MoveTargetsResult = (Vec<(usize, usize)>, Vec<(usize, usize)>);

/// Type alias for spread move hit result (damages, targets)
pub type SpreadMoveHitResult = (Vec<Option<i32>>, Vec<Option<(usize, usize)>>);

/// Custom event handler registered via onEvent (for testing)
/// JavaScript: { callback, target, priority, order, subOrder }
pub struct CustomEventHandler {
    /// The callback function - now receives EventContext instead of &mut Battle
    /// This eliminates the circular reference and unsafe code
    pub callback: EventCallback,
    /// Priority for event ordering (higher = earlier)
    pub priority: i32,
    /// Order value
    pub order: bool,
    /// Sub-order for same priority
    pub sub_order: i32,
}

impl std::fmt::Debug for CustomEventHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CustomEventHandler")
            .field("priority", &self.priority)
            .field("order", &self.order)
            .field("sub_order", &self.sub_order)
            .field("callback", &"<closure>")
            .finish()
    }
}

impl<'a> From<&'a Pokemon> for Arg<'a> {
    fn from(p: &'a Pokemon) -> Self {
        Arg::Pokemon(p)
    }
}

impl<'a> From<&'a str> for Arg<'a> {
    fn from(s: &'a str) -> Self {
        Arg::Str(s)
    }
}

impl<'a> From<String> for Arg<'a> {
    fn from(s: String) -> Self {
        Arg::String(s)
    }
}

impl std::fmt::Display for Arg<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Arg::Pokemon(p) => write!(f, "{}", p),
            Arg::Str(s) => write!(f, "{}", s),
            Arg::String(s) => write!(f, "{}", s),
            Arg::SplitFn(_) => write!(f, "[split function]"),
        }
    }
}

/// Result of a switch operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchResult {
    /// Switch succeeded
    Success,
    /// Switch failed
    Failed,
    /// Pokemon fainted from Pursuit before switch completed
    PursuitFaint,
}

/// Event information for tracking current event context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventInfo {
    /// Event ID/name
    pub id: String,
    /// Target of the event (side_idx, poke_idx) or None for field/battle
    pub target: Option<(usize, usize)>,
    /// Source of the event
    pub source: Option<(usize, usize)>,
    /// Effect that caused the event
    pub effect: Option<ID>,
    /// Modifier accumulated during event processing (4096 = 1.0x)
    pub modifier: i32,
    /// Relay variable for events that modify numeric values (crit ratio, weight, etc.)
    pub relay_var: Option<i32>,
    /// Relay variable for events that use float values (fractional priority, etc.)
    pub relay_var_float: Option<f64>,
    /// Relay variable for boost events (onTryBoost, onAfterBoost, etc.)
    pub relay_var_boost: Option<crate::dex_data::BoostsTable>,
    /// Relay variable for secondary effects (onModifySecondaries, etc.)
    pub relay_var_secondaries: Option<Vec<crate::battle_actions::SecondaryEffect>>,
    /// Relay variable for type strings (onImmunity, etc.)
    pub relay_var_type: Option<String>,
}

impl EventInfo {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            target: None,
            source: None,
            effect: None,
            modifier: 4096,
            relay_var: None,
            relay_var_float: None,
            relay_var_boost: None,
            relay_var_secondaries: None,
            relay_var_type: None,
        }
    }
}

impl Default for EventInfo {
    fn default() -> Self {
        Self {
            id: String::new(),
            target: None,
            source: None,
            effect: None,
            modifier: 4096,
            relay_var: None,
            relay_var_float: None,
            relay_var_boost: None,
            relay_var_secondaries: None,
            relay_var_type: None,
        }
    }
}

/// Context provided to custom event handlers
/// Contains read-only event information that callbacks can access
/// Equivalent to JavaScript's `this` context in event callbacks
#[derive(Debug, Clone)]
pub struct EventContext {
    /// Event ID/name (e.g., "Hit", "ModifyDamage")
    pub event_id: String,
    /// Target of the event (side_idx, poke_idx)
    pub target: Option<(usize, usize)>,
    /// Source of the event
    pub source: Option<(usize, usize)>,
    /// Effect that caused the event
    pub effect: Option<ID>,
    /// Modifier accumulated during event processing (4096 = 1.0x)
    /// In JavaScript: this.event.modifier
    pub modifier: i32,
    /// Relay variable passed to the event
    /// This is the value being modified through the event chain
    pub relay_var: Option<i32>,
}

impl EventContext {
    /// Create EventContext from EventInfo and relay_var
    fn from_event_info(event_id: &str, event_info: &EventInfo, relay_var: Option<i32>) -> Self {
        Self {
            event_id: event_id.to_string(),
            target: event_info.target,
            source: event_info.source,
            effect: event_info.effect.clone(),
            modifier: event_info.modifier,
            relay_var,
        }
    }

    /// Create minimal context for event without full EventInfo
    fn minimal(event_id: &str) -> Self {
        Self {
            event_id: event_id.to_string(),
            target: None,
            source: None,
            effect: None,
            modifier: 4096,
            relay_var: None,
        }
    }
}

/// Faint queue entry data
/// Equivalent to battle.ts FaintQueue entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaintData {
    /// Pokemon that fainted (side_idx, poke_idx)
    pub target: (usize, usize),
    /// Source of the faint (side_idx, poke_idx) or None
    pub source: Option<(usize, usize)>,
    /// Effect that caused the faint
    pub effect: Option<ID>,
}

/// Battle options
#[derive(Debug, Clone, Default)]
pub struct BattleOptions {
    pub format_id: ID,
    pub format_name: Option<String>,
    pub game_type: Option<GameType>,
    pub seed: Option<PRNGSeed>,
    pub rated: Option<String>,
    pub debug: bool,
    pub strict_choices: bool,
    pub force_random_chance: Option<bool>,
    pub p1: Option<PlayerOptions>,
    pub p2: Option<PlayerOptions>,
    pub p3: Option<PlayerOptions>,
    pub p4: Option<PlayerOptions>,
}

/// Player options
#[derive(Debug, Clone)]
pub struct PlayerOptions {
    pub name: String,
    pub team: Vec<PokemonSet>,
    pub avatar: Option<String>,
    pub rating: Option<String>,
}

/// Request state for the whole battle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BattleRequestState {
    #[default]
    None,
    TeamPreview,
    Move,
    Switch,
}

/// The main Battle struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Battle {
    /// Format ID
    pub format_id: ID,
    /// Format name (e.g., "Gen 9 OU")
    pub format_name: String,
    /// Game type (singles, doubles, etc.)
    pub game_type: GameType,
    /// Generation
    pub gen: u8,
    /// Number of active pokemon per half-field
    pub active_per_half: usize,
    /// Dex for accessing Pokemon data
    #[serde(skip)]
    pub dex: crate::dex::Dex,
    /// Rule table for format rules
    #[serde(skip)]
    pub rule_table: Option<crate::data::formats::RuleTable>,

    /// The battle field
    pub field: Field,
    /// The sides (players)
    pub sides: Vec<Side>,
    /// The action queue
    pub queue: BattleQueue,

    /// Random number generator
    #[serde(skip)]
    pub prng: PRNG,
    /// Starting PRNG seed
    pub prng_seed: PRNGSeed,

    /// Battle log
    pub log: Vec<String>,
    /// Input log
    pub input_log: Vec<String>,

    /// Current request state
    pub request_state: BattleRequestState,
    /// Whether requests have been sent to players
    /// JavaScript: sentRequests
    pub sent_requests: bool,
    /// Current turn number
    pub turn: i32,
    /// Is it mid-turn?
    pub mid_turn: bool,
    /// Has the battle started?
    pub started: bool,
    /// Has the battle ended?
    pub ended: bool,
    /// Was this battle deserialized from saved state?
    pub deserialized: bool,
    /// Winner (side ID string)
    pub winner: Option<String>,

    /// Last move used in battle
    pub last_move: Option<ID>,
    /// Last successful move this turn (for Dancer ability)
    pub last_successful_move_this_turn: Option<ID>,
    /// Last move log line index (for attrLastMove)
    pub last_move_line: i32,
    /// Last damage dealt (for Counter in Gen 1)
    pub last_damage: i32,

    /// Currently active move being executed
    pub active_move: Option<crate::battle_actions::ActiveMove>,
    /// Pokemon currently using a move
    pub active_pokemon: Option<(usize, usize)>, // (side_idx, poke_idx)
    /// Target of the current move
    pub active_target: Option<(usize, usize)>, // (side_idx, poke_idx)

    /// Effect order counter
    pub effect_order: i32,

    /// Event depth for recursion tracking
    pub event_depth: u8,
    /// Current event being processed
    pub current_event: Option<EventInfo>,
    /// Current effect being processed
    pub current_effect: Option<ID>,
    /// Current effect state
    pub current_effect_state: Option<EffectState>,
    /// Current effect metadata (name, type, pranksterBoosted, etc.)
    pub current_effect_data: Option<crate::event_system::EffectData>,
    /// Log position for line limit checking
    pub sent_log_pos: usize,

    /// Debug mode
    pub debug_mode: bool,
    /// Rated match (boolean true or string description)
    pub rated: Option<String>,
    /// Strict choices (errors on invalid choices)
    pub strict_choices: bool,
    /// Support choice cancellation (allow undo)
    /// JavaScript: supportCancel
    pub support_cancel: bool,
    /// Force random chance outcome (for testing)
    pub force_random_chance: Option<bool>,

    /// Hints shown to players
    pub hints: HashSet<String>,

    /// Custom event handlers (for testing)
    /// Maps event name (e.g., "onHit") to list of handlers
    /// JavaScript: events?: {[eventid: string]: EventHandler[]}
    #[serde(skip)]
    pub events: std::collections::HashMap<String, Vec<CustomEventHandler>>,

    /// Faint queue - Pokemon waiting to faint
    /// Equivalent to battle.ts faintQueue
    pub faint_queue: Vec<FaintData>,
}

impl Battle {
}

/// Priority item for sorting actions/handlers
#[derive(Debug, Clone, Default)]
pub struct PriorityItem {
    pub order: Option<i32>,
    pub priority: i32,
    pub speed: i32,
    pub sub_order: i32,
    pub effect_order: i32,
    pub index: usize,
}

impl Battle {
}

// =========================================================================
// Display trait (equivalent to battle.ts toString())
// =========================================================================

impl std::fmt::Display for Battle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Battle: {}", self.format_id.as_str())
    }
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
                moves: vec!["Thunderbolt".to_string(), "Quick Attack".to_string()],
                level: 50,
                ..Default::default()
            },
            PokemonSet {
                name: "Charizard".to_string(),
                species: "Charizard".to_string(),
                ability: "Blaze".to_string(),
                moves: vec!["Flamethrower".to_string(), "Dragon Claw".to_string()],
                level: 50,
                ..Default::default()
            },
        ]
    }

    #[test]
    fn test_battle_creation() {
        let battle = Battle::new(BattleOptions {
            format_id: ID::new("gen9ou"),
            ..Default::default()
        });

        assert!(!battle.started);
        assert!(!battle.ended);
        assert_eq!(battle.turn, 0);
    }

    #[test]
    fn test_battle_with_players() {
        let battle = Battle::new(BattleOptions {
            format_id: ID::new("gen9ou"),
            p1: Some(PlayerOptions {
                name: "Alice".to_string(),
                team: create_test_team(),
                avatar: None,
                rating: None,
            }),
            p2: Some(PlayerOptions {
                name: "Bob".to_string(),
                team: create_test_team(),
                avatar: None,
                rating: None,
            }),
            ..Default::default()
        });

        assert!(battle.started);
        assert_eq!(battle.sides.len(), 2);
        assert_eq!(battle.sides[0].name, "Alice");
        assert_eq!(battle.sides[1].name, "Bob");
    }

    #[test]
    fn test_battle_start() {
        let mut battle = Battle::new(BattleOptions {
            format_id: ID::new("gen9ou"),
            p1: Some(PlayerOptions {
                name: "Alice".to_string(),
                team: create_test_team(),
                avatar: None,
                rating: None,
            }),
            p2: Some(PlayerOptions {
                name: "Bob".to_string(),
                team: create_test_team(),
                avatar: None,
                rating: None,
            }),
            ..Default::default()
        });

        battle.start_battle();
        assert_eq!(battle.turn, 1);

        // Check that Pokemon are switched in
        assert!(battle.sides[0].active[0].is_some());
        assert!(battle.sides[1].active[0].is_some());
    }

    #[test]
    fn test_battle_prng_deterministic() {
        let seed = PRNGSeed::Gen5([1, 2, 3, 4]);

        let mut battle1 = Battle::new(BattleOptions {
            format_id: ID::new("gen9ou"),
            seed: Some(seed.clone()),
            ..Default::default()
        });

        let mut battle2 = Battle::new(BattleOptions {
            format_id: ID::new("gen9ou"),
            seed: Some(seed),
            ..Default::default()
        });

        // Same seed should produce same random numbers
        for _ in 0..10 {
            assert_eq!(battle1.random(100), battle2.random(100));
        }
    }
}
