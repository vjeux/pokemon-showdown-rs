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

// Function modules

// Function modules
mod new;
mod pokemon_at;
mod pokemon_at_mut;
mod set_player;
mod get_team;
mod start;
mod random;
mod random_chance;
mod sample;
mod get_side;
mod get_side_mut;
mod p1;
mod p2;
mod get_all_active;
mod check_win;
mod next_effect_order;
mod init_effect_state;
mod choose;
mod get_log;
mod make_choices;
mod commit_choices;
mod compare_action_priority;
mod insert_run_switch_action;
mod insert_field_action;
mod queue_insert_choice;
mod field_event_switch_in;
mod get_move_target;
mod faint_messages;
mod debug;
mod extract_channel_messages;
mod get_debug_log;
mod add;
mod add_move;
mod add_side_condition;
mod call_duration_callback;
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
mod modify_active_move;
mod suppressing_ability;
mod get_all_pokemon;
mod is_adjacent;
mod is_ally;
mod is_pokemon_fainted;
mod get_random_target;
mod update_speed;
mod damage;
mod decrement_active_move_actions;
mod direct_damage;
mod heal;
mod boost;
mod faint;
mod check_fainted;
mod compare_priority;
mod compare_redirect_order;
mod compare_left_to_right_order;
mod speed_sort;
mod sort_action_queue;
mod shuffle_range;
mod get_pokemon;
mod get_pokemon_mut;
mod can_switch;
mod get_random_switchable;
mod possible_switches;
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
mod get_pokemon_stat;
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
mod single_event_side;
mod get_effect_type;
mod dispatch_single_event;
mod handle_ability_event;
mod handle_item_event;
mod handle_move_event;
mod handle_condition_event;
mod run_event;
mod with_effect_state;
mod find_event_handlers;
mod priority_event;
mod get_event_modifier;
mod get_move_hit_data;
mod set_event_modifier;
mod randomizer;
mod each_event;
mod get_target;
mod get_at_loc;
mod undo_choice;
mod spread_damage;
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
mod has_callback;
mod get_overflowed_turn_count;
mod get_requests;
mod resolve_priority;
mod retarget_last_move;
mod run_pick_team;
mod send_updates;
mod set_trapped;
mod set_weather;
mod set_terrain;
mod clear_weather;
mod clear_terrain;
mod effective_weather;
mod suppressing_weather;
mod is_weather;
mod is_terrain;
mod remove_pseudo_weather;
mod add_pseudo_weather;
mod show_open_team_sheets;
mod spread_modify;
mod stat_modify;
mod tiebreak;
mod to_json;
mod find_battle_event_handlers;
mod find_field_event_handlers;
mod find_side_event_handlers;
mod find_pokemon_event_handlers;
mod get_type_effectiveness_mod;
mod on_event;
mod on_event_priority;
mod run_custom_event_handlers;
mod use_move;

/// JavaScript equivalent: { side: SideID, secret: string, shared: string }
/// 3 fields in JavaScript
pub struct SplitMessage {
    /// Side ID for side-specific content
    /// JavaScript: side: SideID
    pub side: SideID,
    /// Secret message (shown only to this side)
    /// JavaScript: secret: string
    pub secret: String,
    /// Shared message (shown to all players)
    /// JavaScript: shared: string
    pub shared: String,
}

/// Argument type for battle.add() - can be a Pokemon reference or a string
/// This allows mixing types like: battle.add("-activate", &[pokemon.into(), "ability: Immunity".into()])
/// TODO: Not in JavaScript - Rust-specific enum for type-safe protocol message arguments
/// JavaScript uses spread arguments with mixed types (Pokemon | string | Function)
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
/// JavaScript equivalent: EventListener (sim/battle.ts)
/// 10 fields in JavaScript
pub struct EventListener {
    /// Event name/variant (e.g., "Damage", "AllyDamage", "SourceDamage")
    /// Not in JavaScript - Rust-specific field to track event variant
    pub event_name: String,
    /// Effect that owns this handler
    /// JavaScript: effect: BasicEffect (via effectId)
    pub effect_id: ID,
    /// Type of effect (Ability, Item, Move, Status, etc.)
    /// JavaScript: effectType: EffectType
    pub effect_type: EffectType,
    /// Target Pokemon (optional)
    /// JavaScript: target?: Pokemon
    /// TODO: Rust uses (side_idx, poke_idx) tuple instead of Pokemon reference due to ownership
    pub target: Option<(usize, usize)>,
    /// Index for multi-target events
    /// JavaScript: index?: number
    pub index: Option<usize>,
    /// Effect state
    /// JavaScript: state?: EffectState
    pub state: Option<EffectState>,
    /// Effect holder (Pokemon/Side/Field/Battle)
    /// JavaScript: effectHolder?: Pokemon
    /// TODO: Rust uses (side_idx, poke_idx) tuple instead of Pokemon reference due to ownership
    pub effect_holder: Option<(usize, usize)>,
    /// Order value (false = first in JS, represented as Option<i32>)
    /// JavaScript: order?: false | number
    /// TODO: Rust cannot represent the union type (false | number), uses Option<i32>
    pub order: Option<i32>,
    /// Priority value (higher = earlier)
    /// JavaScript: priority: number
    pub priority: i32,
    /// Sub-order for same priority
    /// JavaScript: subOrder: number
    pub sub_order: i32,
    /// Effect order (for hazards and abilities with same priority)
    /// JavaScript: effectOrder?: number
    pub effect_order: Option<i32>,
    /// Speed stat (for speed-based sorting)
    /// JavaScript: speed?: number
    pub speed: Option<f64>,
}

/// Effect type - matches JavaScript effectType
/// Used to determine event handler priority ordering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
/// JavaScript equivalent: EffectType (sim/global-types.ts)
pub enum EffectType {
    ZMove,
    #[default]
    Condition,
    SlotCondition,
    SideCondition,
    FieldCondition,
    Weather,
    Terrain,  // Added to match JavaScript effectType
    Format,
    Rule,
    Ruleset,
    Ability,
    Item,
    Move,
    Status,
}

/// Effect - represents an effect with its ID and type
/// JavaScript equivalent: Effect interface (sim/global-types.ts)
/// In JavaScript, Effect has many fields (id, name, effectType, flags, etc.)
/// In Rust, we only need the essential fields for singleEvent: id and effectType
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Effect {
    /// Effect ID (e.g., "stall", "intimidate", "leftovers")
    pub id: ID,
    /// Type of effect (Ability, Item, Move, Condition, etc.)
    pub effect_type: EffectType,
}

impl Effect {
    /// Create a new Effect with the given id and effect_type
    pub fn new(id: ID, effect_type: EffectType) -> Self {
        Self { id, effect_type }
    }

    /// Get the Effect's ID
    pub fn get_id(&self) -> &ID {
        &self.id
    }

    /// Create an Ability effect
    pub fn ability(id: impl Into<ID>) -> Self {
        Self::new(id.into(), EffectType::Ability)
    }

    /// Create an Item effect
    pub fn item(id: impl Into<ID>) -> Self {
        Self::new(id.into(), EffectType::Item)
    }

    /// Create a Move effect
    pub fn move_(id: impl Into<ID>) -> Self {
        Self::new(id.into(), EffectType::Move)
    }

    /// Create a Condition (volatile) effect
    pub fn condition(id: impl Into<ID>) -> Self {
        Self::new(id.into(), EffectType::Condition)
    }

    /// Create a Status effect
    pub fn status(id: impl Into<ID>) -> Self {
        Self::new(id.into(), EffectType::Status)
    }

    /// Create a Weather effect
    pub fn weather(id: impl Into<ID>) -> Self {
        Self::new(id.into(), EffectType::Weather)
    }

    /// Create a Terrain effect
    pub fn terrain(id: impl Into<ID>) -> Self {
        Self::new(id.into(), EffectType::Terrain)
    }

    /// Create a SideCondition effect
    pub fn side_condition(id: impl Into<ID>) -> Self {
        Self::new(id.into(), EffectType::SideCondition)
    }

    /// Create a SlotCondition effect
    pub fn slot_condition(id: impl Into<ID>) -> Self {
        Self::new(id.into(), EffectType::SlotCondition)
    }

    /// Create a FieldCondition (pseudo-weather) effect
    pub fn field_condition(id: impl Into<ID>) -> Self {
        Self::new(id.into(), EffectType::FieldCondition)
    }

    /// Get the ID as a string reference
    pub fn as_str(&self) -> &str {
        self.id.as_str()
    }
}

impl std::fmt::Display for Effect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

/// Type alias for event callback functions
pub type EventCallback = Box<dyn Fn(&EventInfo) -> Option<i32> + Send + Sync>;

/// Context for the current effect being processed
/// Used by with_effect_state to look up the correct state
/// Rust-specific: JavaScript uses this.effectState which is a reference
/// Consolidates: current_effect, current_effect_state, current_effect_data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EffectContext {
    /// Effect ID (e.g., "focuspunch", "intimidate")
    pub effect_id: ID,
    /// Type of effect
    pub effect_type: EffectType,
    /// Pokemon that holds this effect (for volatiles, abilities, items, status)
    pub effect_holder: Option<(usize, usize)>,
    /// Side index (for side conditions)
    pub side_index: Option<usize>,
    /// Whether this effect was Prankster boosted
    pub prankster_boosted: bool,
}

/// Type alias for spread move hit result (damages, targets)
pub type SpreadMoveHitResult = (crate::battle_actions::SpreadMoveDamage, crate::battle_actions::SpreadMoveTargets);

/// Custom event handler registered via onEvent (for testing)
/// JavaScript: { callback, target, priority, order, subOrder }
/// JavaScript equivalent: CustomEventHandler (sim/battle.ts)
/// 5 fields in JavaScript
pub struct CustomEventHandler {
    /// The callback function - receives EventInfo for event context
    /// JavaScript: callback: (this: Battle, ...args: any[]) => any
    pub callback: EventCallback,
    /// Target effect ID
    /// JavaScript: target (Effect object ID)
    pub target_id: ID,
    /// Target effect type
    /// JavaScript: target.effectType
    pub target_type: EffectType,
    /// Priority for event ordering (higher = earlier)
    /// JavaScript: priority: number
    pub priority: i32,
    /// Order value
    /// JavaScript: order: boolean
    pub order: bool,
    /// Sub-order for same priority
    /// JavaScript: subOrder: number
    pub sub_order: i32,
}

impl std::fmt::Debug for CustomEventHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CustomEventHandler")
            .field("target_id", &self.target_id)
            .field("target_type", &self.target_type)
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
/// TODO: Not in JavaScript - Rust-specific enum for switch operation results
/// JavaScript switch methods return undefined or false on failure
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchResult {
    /// Switch succeeded
    Success,
    /// Switch failed
    Failed,
    /// Pokemon fainted from Pursuit before switch completed
    PursuitFaint,
}

/// Event information passed through the event system
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: Event object (sim/battle.ts)
/// JavaScript stores this in battle.event during event processing
/// 10+ fields in JavaScript (context object with dynamic properties)
pub struct EventInfo {
    /// Event ID/name
    /// JavaScript: event.id (implicit, based on which event is running)
    pub id: String,
    /// Target of the event (side_idx, poke_idx) or None for field/battle
    /// JavaScript: event.target (Pokemon reference)
    /// TODO: Rust uses indices instead of Pokemon reference due to ownership
    pub target: Option<(usize, usize)>,
    /// Source of the event
    /// JavaScript: event.source (Pokemon reference)
    /// TODO: Rust uses indices instead of Pokemon reference due to ownership
    pub source: Option<(usize, usize)>,
    /// Effect that caused the event
    /// JavaScript: event.effect (Effect object)
    pub effect: Option<Effect>,
    /// Modifier accumulated during event processing (4096 = 1.0x)
    /// JavaScript: event.modifier (number)
    pub modifier: i32,
    /// Relay variable for event callbacks
    /// JavaScript: relayVar parameter in event callbacks
    /// Can hold various types via EventResult variants:
    /// - Number(i32) for numeric values (damage, crit ratio, etc.)
    /// - Float(f64) for fractional values (priority, etc.)
    /// - Boost(BoostsTable) for stat modifications
    /// - String for type strings (immunity, etc.)
    /// - Secondaries(Vec<SecondaryEffect>) for move secondary effects
    /// - Boolean for true/false checks
    /// - etc.
    pub relay_var: Option<crate::event::EventResult>,
    /// Type parameter for Effectiveness events
    /// JavaScript: type parameter in runEvent('Effectiveness', this, type, move, typeMod)
    /// Used to pass the defender type being checked for type effectiveness
    pub type_param: Option<String>,
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
            type_param: None,
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
            type_param: None,
        }
    }
}

/// Faint queue entry data
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: FaintQueue entry (sim/battle.ts)
/// 3 fields in JavaScript
pub struct FaintData {
    /// Pokemon that fainted (side_idx, poke_idx)
    /// JavaScript: target (Pokemon reference)
    /// TODO: Rust uses indices instead of Pokemon reference due to ownership
    pub target: (usize, usize),
    /// Source of the faint (side_idx, poke_idx) or None
    /// JavaScript: source (Pokemon reference or null)
    /// TODO: Rust uses indices instead of Pokemon reference due to ownership
    pub source: Option<(usize, usize)>,
    /// Effect that caused the faint
    /// JavaScript: effect (Effect object or null)
    pub effect: Option<Effect>,
}

/// Battle creation options
#[derive(Debug, Clone, Default)]
/// JavaScript equivalent: BattleOptions (sim/battle.ts)
/// 14 fields in JavaScript
pub struct BattleOptions {
    /// Format ID
    /// JavaScript: formatid: ID
    pub format_id: ID,
    /// Format name
    /// JavaScript: format?: string
    pub format_name: Option<String>,
    /// Game type (singles, doubles, etc.)
    /// JavaScript: gameType?: GameType
    pub game_type: Option<GameType>,
    /// PRNG seed
    /// JavaScript: seed?: PRNGSeed
    pub seed: Option<PRNGSeed>,
    /// Rated match
    /// JavaScript: rated?: boolean | string
    pub rated: Option<String>,
    /// Debug mode
    /// JavaScript: debug?: boolean
    pub debug: bool,
    /// Strict choice validation
    /// JavaScript: strictChoices?: boolean
    pub strict_choices: bool,
    /// Force random chance outcome
    /// JavaScript: forceRandomChance?: boolean
    pub force_random_chance: Option<bool>,
    /// Player 1 options
    /// JavaScript: p1?: PlayerOptions
    pub p1: Option<PlayerOptions>,
    /// Player 2 options
    /// JavaScript: p2?: PlayerOptions
    pub p2: Option<PlayerOptions>,
    /// Player 3 options (multi battles)
    /// JavaScript: p3?: PlayerOptions
    pub p3: Option<PlayerOptions>,
    /// Player 4 options (multi battles)
    /// JavaScript: p4?: PlayerOptions
    pub p4: Option<PlayerOptions>,
}

/// Player/side creation options
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: PlayerOptions (sim/global-types.ts)
/// 5 fields in JavaScript
pub struct PlayerOptions {
    /// Player name
    /// JavaScript: name: string
    pub name: String,
    /// Player's team
    /// JavaScript: team: PokemonSet[] | string
    /// Rust: Uses TeamFormat enum to represent union type
    pub team: TeamFormat,
    /// Player avatar
    /// JavaScript: avatar?: string
    pub avatar: Option<String>,
    /// Player rating
    /// JavaScript: rating?: number | string
    pub rating: Option<String>,
    /// RNG seed for team generation
    /// JavaScript: seed?: PRNGSeed
    /// PRNGSeed is a string like "sodium,abc123" or "gen5,xyz" or "1234,abc"
    pub seed: Option<String>,
}

/// Team format - either packed string or array of Pokemon sets
/// JavaScript equivalent: PokemonSet[] | string
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TeamFormat {
    /// Empty team (will be generated)
    Empty,
    /// Packed team string (needs unpacking with Teams::unpack)
    Packed(String),
    /// Array of Pokemon sets
    Sets(Vec<PokemonSet>),
}

impl Default for TeamFormat {
    fn default() -> Self {
        TeamFormat::Empty
    }
}

/// Request state for the whole battle
/// TODO: Not in JavaScript - Rust-specific enum for tracking battle request state
/// JavaScript uses string literals for request types ('move' | 'switch' | 'teampreview' | null)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BattleRequestState {
    #[default]
    None,
    TeamPreview,
    Move,
    Switch,
}

/// The main Battle struct
#[derive(Serialize, Deserialize)]
/// JavaScript equivalent: Battle (sim/battle.ts)
/// 153 fields in JavaScript
pub struct Battle {
    /// Battle ID
    /// JavaScript: readonly id: ID
    pub id: ID,

    // TODO: DELETE - Not in JavaScript Battle (JavaScript has format.id, not separate field)
    /// Format ID
    pub format_id: ID,
    // TODO: DELETE - Not in JavaScript Battle (JavaScript has format.name, not separate field)
    /// Format name (e.g., "Gen 9 OU")
    pub format_name: String,
    /// Full format object
    /// JavaScript: readonly format: Format
    #[serde(skip)]
    pub format: Option<crate::data::formats::Format>,
    /// Format data effect state
    /// JavaScript: readonly formatData: EffectState
    pub format_data: EffectState,

    /// Game type (singles, doubles, etc.)
    /// JavaScript: readonly gameType: GameType
    pub game_type: GameType,
    /// Generation
    /// JavaScript: readonly gen: number
    pub gen: u8,
    /// Number of active pokemon per half-field
    /// JavaScript: readonly activePerHalf: number
    pub active_per_half: usize,
    /// Dex for accessing Pokemon data
    /// JavaScript: readonly dex: ModdedDex
    #[serde(skip)]
    pub dex: crate::dex::Dex,
    /// Rule table for format rules
    /// JavaScript: readonly ruleTable: Map<string, string>
    #[serde(skip)]
    pub rule_table: Option<crate::data::formats::RuleTable>,

    /// The battle field
    /// JavaScript: readonly field: Field
    pub field: Field,
    /// The sides (players)
    /// JavaScript: readonly sides: Side[]
    pub sides: Vec<Side>,
    /// The action queue
    /// JavaScript: readonly queue: BattleQueue
    pub queue: BattleQueue,
    /// Speed order for active Pokemon (JS: speedOrder)
    /// Maps active pokemon to their position: side.n * sides.length + position
    /// JavaScript: speedOrder: Pokemon[]
    /// TODO: Rust uses indices instead of Pokemon references due to ownership
    pub speed_order: Vec<usize>,

    /// Random number generator
    /// JavaScript: readonly prng: PRNG
    #[serde(skip)]
    pub prng: PRNG,
    /// Starting PRNG seed
    /// JavaScript: readonly prngSeed: PRNGSeed
    pub prng_seed: PRNGSeed,

    /// Battle log
    /// JavaScript: readonly log: string[]
    pub log: Vec<String>,
    /// Input log
    /// JavaScript: readonly inputLog: string[]
    pub input_log: Vec<String>,
    /// Message log (for clients)
    /// JavaScript: readonly messageLog: string[]
    pub message_log: Vec<String>,

    /// Report exact HP values (not percentages)
    /// JavaScript: reportExactHP: boolean
    pub report_exact_hp: bool,

    /// Report HP as percentages
    /// JavaScript: reportPercentages: boolean
    pub report_percentages: bool,

    /// Current request state
    /// JavaScript: requestState: 'teamPreview' | 'move' | 'switch' | ''
    /// TODO: Rust uses enum instead of string union
    pub request_state: BattleRequestState,
    // TODO: DELETE - Not in JavaScript Battle class
    /// Whether requests have been sent to players
    pub sent_requests: bool,
    /// Current turn number
    /// JavaScript: turn: number
    pub turn: i32,
    /// Is it mid-turn?
    /// JavaScript: midTurn: boolean
    pub mid_turn: bool,
    /// Has the battle started?
    /// JavaScript: started: boolean
    pub started: bool,
    /// Has the battle ended?
    /// JavaScript: ended: boolean
    pub ended: bool,
    /// Was this battle deserialized from saved state?
    /// JavaScript: deserialized?: boolean
    pub deserialized: bool,
    /// Winner (side ID string)
    /// JavaScript: winner?: string
    pub winner: Option<String>,

    /// Battle actions handler
    /// JavaScript: actions: BattleActions
    // TODO: Add this field - requires adding lifetime parameter to Battle struct
    // #[serde(skip)]
    // pub actions: Option<crate::battle_actions::BattleActions<'a>>,

    /// Last move used in battle (full ActiveMove, not just ID)
    /// JavaScript: lastMove: ActiveMove | null
    pub last_move: Option<crate::battle_actions::ActiveMove>,
    /// Last successful move this turn (for Dancer ability)
    /// JavaScript: lastSuccessfulMoveThisTurn?: ID
    pub last_successful_move_this_turn: Option<ID>,
    /// Last move log line index (for attrLastMove)
    /// JavaScript: lastMoveLine: number
    pub last_move_line: i32,
    /// Last damage dealt (for Counter in Gen 1)
    /// JavaScript: lastDamage: number
    pub last_damage: i32,
    /// Quick Claw roll result for this turn (Gen 2-3)
    /// JavaScript: quickClawRoll: boolean
    pub quick_claw_roll: bool,

    /// Currently active move being executed
    /// JavaScript: activeMove: ActiveMove | null
    pub active_move: Option<crate::battle_actions::ActiveMove>,

    /// Flag to indicate we're executing a queued future move (doomdesire, futuresight)
    /// This is used to prevent future move callbacks from queuing another future move
    /// when they should be dealing damage instead
    pub executing_future_move: bool,

    /// Pokemon currently using a move
    /// JavaScript: activePokemon: Pokemon | null
    /// TODO: Rust uses (side_idx, poke_idx) tuple instead of Pokemon reference due to ownership
    pub active_pokemon: Option<(usize, usize)>, // (side_idx, poke_idx)
    /// Target of the current move
    /// JavaScript: activeTarget: Pokemon | null
    /// TODO: Rust uses (side_idx, poke_idx) tuple instead of Pokemon reference due to ownership
    pub active_target: Option<(usize, usize)>, // (side_idx, poke_idx)

    /// Effect order counter
    /// JavaScript: effectOrder: number
    pub effect_order: i32,

    /// Current effect (JavaScript: effect: Effect)
    /// NOTE: Rust also has current_effect for internal use
    /// JavaScript: effect: Effect | null
    pub effect: Option<ID>,
    /// Current effect state (JavaScript: effectState: EffectState)
    /// NOTE: Rust also has current_effect_state for internal use
    /// JavaScript: effectState: EffectState
    pub effect_state: EffectState,
    /// Current event object (JavaScript: event: AnyObject)
    /// NOTE: Rust also has current_event for internal use
    /// JavaScript: event: AnyObject
    pub event: Option<EventInfo>,

    // TODO: DELETE - Not in JavaScript Battle class
    /// Event depth for recursion tracking
    pub event_depth: u8,
    // TODO: DELETE - Not in JavaScript Battle class (use `event` field instead)
    /// Current event being processed (internal tracking)
    pub current_event: Option<EventInfo>,
    /// Current effect context for with_effect_state lookups
    /// Rust-specific: Stores info needed to look up the current effect's state
    /// Consolidates: current_effect, current_effect_state, current_effect_data
    pub current_effect_context: Option<EffectContext>,

    /// Log position for sent messages
    /// JavaScript: sentLogPos: number
    pub sent_log_pos: usize,
    /// Whether end message has been sent
    /// JavaScript: sentEnd: boolean
    pub sent_end: bool,

    /// Team generator (for random battles)
    /// JavaScript: teamGenerator: ReturnType<typeof Teams.getGenerator> | null
    #[serde(skip)]
    pub team_generator: Option<Box<dyn std::any::Any + Send + Sync>>,

    /// Output callback for sending updates to clients
    /// JavaScript: send?: (type: string, data: string | string[]) => void
    #[serde(skip)]
    pub send: Option<Box<dyn Fn(&str, &str) + Send + Sync>>,

    /// Debug mode
    /// JavaScript: debugMode?: boolean
    pub debug_mode: bool,
    /// Rated match (boolean true or string description)
    /// JavaScript: rated?: boolean | string
    /// TODO: Rust uses Option<String> instead of the union type
    pub rated: Option<String>,
    /// Strict choices (errors on invalid choices)
    /// JavaScript: strictChoices?: boolean
    pub strict_choices: bool,
    /// Support choice cancellation (allow undo)
    /// JavaScript: supportCancel: boolean
    pub support_cancel: bool,
    /// Force random chance outcome (for testing)
    /// JavaScript: forceRandomChance?: boolean
    pub force_random_chance: Option<bool>,

    /// Hints shown to players
    /// JavaScript: hints: Set<string>
    pub hints: HashSet<String>,

    /// Custom event handlers (for testing)
    /// Maps event name (e.g., "onHit") to list of handlers
    /// JavaScript: events?: {[eventid: string]: EventHandler[]}
    #[serde(skip)]
    pub events: std::collections::HashMap<String, Vec<CustomEventHandler>>,

    /// Faint queue - Pokemon waiting to faint
    /// JavaScript: faintQueue: FaintData[]
    pub faint_queue: Vec<FaintData>,
}

/// Priority item for sorting actions/handlers
/// TODO: Not in JavaScript - Rust-specific helper for sorting priority queues
/// JavaScript uses inline sorting with anonymous comparator functions
#[derive(Debug, Clone, Default)]
pub struct PriorityItem {
    pub order: Option<i32>,
    pub priority: i32,
    pub speed: f64,  // Changed from i32 to f64 to match JavaScript's fractional speeds
    pub sub_order: i32,
    pub effect_order: i32,
    pub index: usize,
}

impl Battle {
    /// Battle result constant: NOT_FAIL
    /// JavaScript: readonly NOT_FAIL: ''
    pub const NOT_FAIL: &'static str = "";

    /// Battle result constant: HIT_SUBSTITUTE
    /// JavaScript: readonly HIT_SUBSTITUTE: 0
    pub const HIT_SUBSTITUTE: i32 = 0;

    /// Battle result constant: FAIL
    /// JavaScript: readonly FAIL: false
    pub const FAIL: bool = false;

    /// Battle result constant: SILENT_FAIL
    /// JavaScript: readonly SILENT_FAIL: null
    /// Note: In Rust, we use Option::None to represent null
    pub const SILENT_FAIL: Option<i32> = None;
}

// =========================================================================
// Debug trait (manual implementation since send callback can't derive Debug)
// =========================================================================

impl std::fmt::Debug for Battle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Battle")
            .field("format_id", &self.format_id)
            .field("format_name", &self.format_name)
            .field("game_type", &self.game_type)
            .field("gen", &self.gen)
            .field("turn", &self.turn)
            .field("started", &self.started)
            .field("ended", &self.ended)
            .field("winner", &self.winner)
            .field("send", &"<callback>")
            .finish_non_exhaustive()
    }
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
                team: TeamFormat::Sets(create_test_team()),
                avatar: None,
                rating: None,
            }),
            p2: Some(PlayerOptions {
                name: "Bob".to_string(),
                team: TeamFormat::Sets(create_test_team()),
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
