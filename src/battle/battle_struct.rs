//! Battle Struct

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::battle_queue::BattleQueue;
use crate::dex_data::{GameType, ID};
use crate::event_system::{EffectState, SharedEffectState};
use crate::field::Field;
use crate::side::Side;
use crate::prng::{PRNGSeed, PRNG};

use super::{
    BattleRequestState, CustomEventHandler, Effect, EventInfo, FaintData,
};

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
    pub format_data: SharedEffectState,

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

    /// Last move used in battle (full ActiveMove, not just ID)
    /// JavaScript: lastMove: ActiveMove | null
    pub last_move: Option<crate::battle_actions::SharedActiveMove>,
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
    pub active_move: Option<crate::battle_actions::SharedActiveMove>,

    /// Flag to indicate we're executing a queued future move (doomdesire, futuresight)
    /// This is used to prevent future move callbacks from queuing another future move
    /// when they should be dealing damage instead
    pub executing_future_move: bool,

    /// Pokemon currently using a move
    /// JavaScript: activePokemon: Pokemon | null
    /// TODO: Rust uses (side_idx, poke_idx) tuple instead of Pokemon reference due to ownership
    pub active_pokemon: Option<(usize, usize)>,
    /// Target of the current move
    /// JavaScript: activeTarget: Pokemon | null
    /// TODO: Rust uses (side_idx, poke_idx) tuple instead of Pokemon reference due to ownership
    pub active_target: Option<(usize, usize)>,

    /// Effect order counter
    /// JavaScript: effectOrder: number
    pub effect_order: i32,

    /// Current effect (JavaScript: effect: Effect)
    /// JavaScript: effect: Effect | null
    pub effect: Option<Effect>,
    /// Current effect state (JavaScript: effectState: EffectState)
    /// JavaScript: effectState: EffectState
    pub effect_state: SharedEffectState,
    /// Current event object (JavaScript: event: AnyObject)
    /// JavaScript: event: AnyObject
    pub event: Option<EventInfo>,

    // TODO: DELETE - Not in JavaScript Battle class
    /// Event depth for recursion tracking
    pub event_depth: u8,

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

    /// Residual pokemon HP tracking for Emergency Exit
    /// JavaScript: residualPokemon (local variable in runAction)
    /// Stores (side_index, pokemon_index, original_hp) for all active pokemon before residual
    pub residual_pokemon: Vec<(usize, usize, i32)>,
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
