//! Simulator Battle
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This file is where the battle simulation itself happens.
//! The most important part of the simulation is the event system.

use std::collections::{HashSet, HashMap};
use serde::{Deserialize, Serialize};

use crate::dex_data::{ID, GameType, SideID, StatsTable, StatID};
use crate::event_system::EffectState;
use crate::field::Field;
use crate::battle_queue::BattleQueue;
use crate::pokemon::{Pokemon, PokemonSet};
use crate::side::{Side, RequestState, Choice};
use crate::prng::{PRNG, PRNGSeed};
use crate::data::formats::{get_format, Format, DexFormats};

/// Split message for side-specific content
/// JavaScript equivalent: { side: SideID, secret: string, shared: string }
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

/// Custom event handler registered via onEvent (for testing)
/// JavaScript: { callback, target, priority, order, subOrder }
pub struct CustomEventHandler {
    /// The callback function - now receives EventContext instead of &mut Battle
    /// This eliminates the circular reference and unsafe code
    pub callback: Box<dyn Fn(&EventContext) -> Option<i32> + Send + Sync>,
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
}

impl EventInfo {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            target: None,
            source: None,
            effect: None,
            modifier: 4096,
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
    /// Create a new battle
    /// Equivalent to TypeScript Battle constructor (battle.ts:191)
    pub fn new(options: BattleOptions) -> Self {
        let seed = options.seed.clone().unwrap_or_else(PRNG::generate_seed);
        let prng = PRNG::new(Some(seed.clone()));

        // Clone format_id before moving it into the struct
        let format_id_str = options.format_id.as_str().to_string();

        // Determine game settings from format
        let game_type = options.game_type.unwrap_or(GameType::Singles);
        let gen = 9; // Default to latest gen
        let active_per_half = match game_type {
            GameType::Triples => 3,
            GameType::Doubles | GameType::Multi | GameType::FreeForAll => 2,
            _ => 1,
        };
        let player_count = match game_type {
            GameType::Multi | GameType::FreeForAll => 4,
            _ => 2,
        };

        let sides = Vec::new(); // Sides will be added via set_player

        // Load dex
        let dex = crate::dex::Dex::load_default().expect("Failed to load Dex");

        // Load rule_table from format
        // JavaScript: this.ruleTable = this.dex.formats.getRuleTable(this.format);
        let rule_table = if let Some(format_def) = get_format(&ID::new(&format_id_str)) {
            // Create a Format from the FormatDef
            let format = Format::from_def(format_def);
            // Create DexFormats and get the rule table
            let dex_formats = DexFormats::new();
            Some(dex_formats.get_rule_table(&format))
        } else {
            None
        };

        let mut battle = Self {
            format_id: options.format_id,
            format_name: options.format_name.unwrap_or_else(|| format_id_str.clone()),
            game_type,
            gen,
            active_per_half,
            dex,
            rule_table,
            field: Field::new(),
            sides,
            queue: BattleQueue::new(),
            prng,
            prng_seed: seed.clone(),
            log: Vec::new(),
            input_log: Vec::new(),
            request_state: BattleRequestState::None,
            sent_requests: false,  // JavaScript: sentRequests defaults to false
            turn: 0,
            mid_turn: false,
            started: false,
            ended: false,
            deserialized: false,
            winner: None,
            last_move: None,
            last_successful_move_this_turn: None,
            last_move_line: -1,
            last_damage: 0,
            active_move: None,
            active_pokemon: None,
            active_target: None,
            effect_order: 0,
            event_depth: 0,
            current_event: None,
            current_effect: None,
            current_effect_state: None,
            current_effect_data: None,
            sent_log_pos: 0,
            debug_mode: options.debug,
            rated: options.rated,
            strict_choices: options.strict_choices,
            support_cancel: false, // JavaScript default: supportCancel defaults to false
            force_random_chance: if options.debug {
                options.force_random_chance
            } else {
                None
            },
            hints: HashSet::new(),
            events: std::collections::HashMap::new(),
            faint_queue: Vec::new(),
        };

        // Initialize sides vector
        for _ in 0..player_count {
            // Placeholder - will be filled by set_player
        }

        // Log start
        battle.input_log.push(format!(">start {{\"formatid\":\"{}\",\"seed\":\"{}\"}}",
            format_id_str, seed.to_string()));
        battle.add_log("gametype", &[&game_type_to_string(&game_type)]);

        // Add players if provided
        if let Some(p1) = options.p1 {
            battle.set_player(SideID::P1, p1);
        }
        if let Some(p2) = options.p2 {
            battle.set_player(SideID::P2, p2);
        }
        if let Some(p3) = options.p3 {
            battle.set_player(SideID::P3, p3);
        }
        if let Some(p4) = options.p4 {
            battle.set_player(SideID::P4, p4);
        }

        battle
    }

    // ========================================================================
    // Pokemon Iterator Helpers
    // ========================================================================
    // These methods provide a cleaner API for iterating over Pokemon without
    // exposing the internal (side_idx, poke_idx) representation.

    /// Get a Pokemon reference by indices (immutable)
    /// Helper method to reduce boilerplate when accessing Pokemon by (side_idx, poke_idx)
    #[inline]
    pub fn pokemon_at(&self, side_idx: usize, poke_idx: usize) -> Option<&Pokemon> {
        self.sides.get(side_idx)?.pokemon.get(poke_idx)
    }

    /// Get a Pokemon reference by indices (mutable)
    /// Helper method to reduce boilerplate when accessing Pokemon by (side_idx, poke_idx)
    #[inline]
    pub fn pokemon_at_mut(&mut self, side_idx: usize, poke_idx: usize) -> Option<&mut Pokemon> {
        self.sides.get_mut(side_idx)?.pokemon.get_mut(poke_idx)
    }

    /// Add or update a player in the battle
    ///
    // TypeScript source:
    // 
    // 
    // 	setPlayer(slot: SideID, options: PlayerOptions) {
    // 		let side;
    // 		let didSomething = true;
    // 		const slotNum = parseInt(slot[1]) - 1;
    // 		if (!this.sides[slotNum]) {
    // 			// create player
    // 			const team = this.getTeam(options);
    // 			side = new Side(options.name || `Player ${slotNum + 1}`, this, slotNum, team);
    // 			if (options.avatar) side.avatar = `${options.avatar}`;
    // 			this.sides[slotNum] = side;
    // 		} else {
    // 			// edit player
    // 			side = this.sides[slotNum];
    // 			didSomething = false;
    // 			if (options.name && side.name !== options.name) {
    // 				side.name = options.name;
    // 				didSomething = true;
    // 			}
    // 			if (options.avatar && side.avatar !== `${options.avatar}`) {
    // 				side.avatar = `${options.avatar}`;
    // 				didSomething = true;
    // 			}
    // 			if (options.team) throw new Error(`Player ${slot} already has a team!`);
    // 		}
    // 		if (options.team && typeof options.team !== 'string') {
    // 			options.team = Teams.pack(options.team);
    // 		}
    // 		if (!didSomething) return;
    // 		this.inputLog.push(`>player ${slot} ` + JSON.stringify(options));
    // 		this.add('player', side.id, side.name, side.avatar, options.rating || '');
    // 
    // 		// Start the battle if it's ready to start
    // 		if (this.sides.every(playerSide => !!playerSide) && !this.started) this.start();
    // 	}
    //
    pub fn set_player(&mut self, side_id: SideID, options: PlayerOptions) {
        let slot_num = side_id.index();
        let mut did_something = true;

        // Ensure sides vector is large enough
        while self.sides.len() <= slot_num {
            self.sides.push(Side::new(
                match self.sides.len() {
                    0 => SideID::P1,
                    1 => SideID::P2,
                    2 => SideID::P3,
                    _ => SideID::P4,
                },
                self.sides.len(),
                String::new(),
                Vec::new(),
                self.active_per_half,
            ));
        }

        // Check if this is a new player or editing existing
        let is_new = self.sides[slot_num].name.is_empty() && self.sides[slot_num].pokemon.is_empty();

        if is_new {
            // Create player
            let team = options.team.clone(); // For now, assume team is already unpacked
            let name = if !options.name.is_empty() {
                options.name.clone()
            } else {
                format!("Player {}", slot_num + 1)
            };

            let mut side = Side::new(side_id, slot_num, name, team, self.active_per_half);
            if let Some(avatar) = &options.avatar {
                side.avatar = avatar.clone();
            }
            self.sides[slot_num] = side;
        } else {
            // Edit player
            did_something = false;

            if !options.name.is_empty() && self.sides[slot_num].name != options.name {
                self.sides[slot_num].name = options.name.clone();
                did_something = true;
            }

            if let Some(avatar) = &options.avatar {
                if self.sides[slot_num].avatar != *avatar {
                    self.sides[slot_num].avatar = avatar.clone();
                    did_something = true;
                }
            }

            if !options.team.is_empty() {
                panic!("Player {} already has a team!", side_id.to_str());
            }
        }

        if !did_something {
            return;
        }

        // Log to inputLog
        // Format options as JSON - simplified version for now
        let mut options_json = format!("{{\"name\":\"{}\"", options.name);
        if let Some(avatar) = &options.avatar {
            options_json.push_str(&format!(",\"avatar\":\"{}\"", avatar));
        }
        if let Some(rating) = &options.rating {
            options_json.push_str(&format!(",\"rating\":\"{}\"", rating));
        }
        options_json.push_str("}");

        self.input_log.push(format!(">player {} {}", side_id.to_str(), options_json));

        // this.add('player', side.id, side.name, side.avatar, options.rating || '');
        // Clone the values to avoid borrow checker issues
        let side_id_str = self.sides[slot_num].id_str().to_string();
        let side_name = self.sides[slot_num].name.clone();
        let side_avatar = self.sides[slot_num].avatar.clone();
        let rating_str = options.rating.as_deref().unwrap_or("").to_string();

        self.add("player", &[
            Arg::Str(&side_id_str),
            Arg::Str(&side_name),
            Arg::Str(&side_avatar),
            Arg::Str(&rating_str),
        ]);

        // Start the battle if it's ready to start
        // if (this.sides.every(playerSide => !!playerSide) && !this.started)
        let all_sides_ready = self.sides.iter().all(|s| !s.name.is_empty() && !s.pokemon.is_empty());
        if all_sides_ready && !self.started {
            self.start();
        }
    }

    /// Start the battle
    /// Source: battle.ts:1859
    /// Deserialized games should use restart()
    // 
    // 	start() {
    // 		// Deserialized games should use restart()
    // 		if (this.deserialized) return;
    // 		// need all players to start
    // 		if (!this.sides.every(side => !!side)) throw new Error(`Missing sides: ${this.sides}`);
    // 
    // 		if (this.started) throw new Error(`Battle already started`);
    // 
    // 		const format = this.format;
    // 		this.started = true;
    // 		if (this.gameType === 'multi') {
    // 			this.sides[1].foe = this.sides[2]!;
    // 			this.sides[0].foe = this.sides[3]!;
    // 			this.sides[2]!.foe = this.sides[1];
    // 			this.sides[3]!.foe = this.sides[0];
    // 			this.sides[1].allySide = this.sides[3]!;
    // 			this.sides[0].allySide = this.sides[2]!;
    // 			this.sides[2]!.allySide = this.sides[0];
    // 			this.sides[3]!.allySide = this.sides[1];
    // 			// sync side conditions
    // 			this.sides[2]!.sideConditions = this.sides[0].sideConditions;
    // 			this.sides[3]!.sideConditions = this.sides[1].sideConditions;
    // 		} else {
    // 			this.sides[1].foe = this.sides[0];
    // 			this.sides[0].foe = this.sides[1];
    // 			if (this.sides.length > 2) { // ffa
    // 				this.sides[2]!.foe = this.sides[3]!;
    // 				this.sides[3]!.foe = this.sides[2]!;
    // 			}
    // 		}
    // 
    // 		this.add('gen', this.gen);
    // 
    // 		this.add('tier', format.name);
    // 		if (this.rated) {
    // 			if (this.rated === 'Rated battle') this.rated = true;
    // 			this.add('rated', typeof this.rated === 'string' ? this.rated : '');
    // 		}
    // 
    // 		format.onBegin?.call(this);
    // 		for (const rule of this.ruleTable.keys()) {
    // 			if ('+*-!'.includes(rule.charAt(0))) continue;
    // 			const subFormat = this.dex.formats.get(rule);
    // 			subFormat.onBegin?.call(this);
    // 		}
    // 
    // 		if (this.sides.some(side => !side.pokemon[0])) {
    // 			throw new Error('Battle not started: A player has an empty team.');
    // 		}
    // 
    // 		if (this.debugMode) {
    // 			this.checkEVBalance();
    // 		}
    // 
    // 		if (format.customRules) {
    // 			const plural = format.customRules.length === 1 ? '' : 's';
    // 			const open = format.customRules.length <= 5 ? ' open' : '';
    // 			this.add(`raw|<div class="infobox"><details class="readmore"${open}><summary><strong>${format.customRules.length} custom rule${plural}:</strong></summary> ${format.customRules.join(', ')}</details></div>`);
    // 		}
    // 
    // 		this.runPickTeam();
    // 		this.queue.addChoice({ choice: 'start' });
    // 		this.midTurn = true;
    // 		if (!this.requestState) this.turnLoop();
    // 	}
    //
    pub fn start(&mut self) {
        // JS: if (this.deserialized) return;
        if self.deserialized {
            return;
        }

        // JS: if (!this.sides.every(side => !!side)) throw new Error(`Missing sides`);
        // Rust: Check that all sides exist and are valid
        if self.sides.is_empty() {
            panic!("Missing sides");
        }

        // JS: if (this.started) throw new Error(`Battle already started`);
        if self.started {
            panic!("Battle already started");
        }

        // JS: this.started = true;
        self.started = true;

        // JS: Set up foe and ally references based on game type
        match self.game_type {
            GameType::Multi => {
                // JS: Multi battles (4 sides)
                if self.sides.len() >= 4 {
                    // JS: this.sides[1].foe = this.sides[2];
                    self.sides[1].foe_index = Some(2);
                    // JS: this.sides[0].foe = this.sides[3];
                    self.sides[0].foe_index = Some(3);
                    // JS: this.sides[2].foe = this.sides[1];
                    self.sides[2].foe_index = Some(1);
                    // JS: this.sides[3].foe = this.sides[0];
                    self.sides[3].foe_index = Some(0);

                    // JS: this.sides[1].allySide = this.sides[3];
                    self.sides[1].ally_index = Some(3);
                    // JS: this.sides[0].allySide = this.sides[2];
                    self.sides[0].ally_index = Some(2);
                    // JS: this.sides[2].allySide = this.sides[0];
                    self.sides[2].ally_index = Some(0);
                    // JS: this.sides[3].allySide = this.sides[1];
                    self.sides[3].ally_index = Some(1);

                    // JS: sync side conditions
                    // JS: this.sides[2].sideConditions = this.sides[0].sideConditions;
                    let side_0_conditions = self.sides[0].side_conditions.clone();
                    self.sides[2].side_conditions = side_0_conditions;
                    // JS: this.sides[3].sideConditions = this.sides[1].sideConditions;
                    let side_1_conditions = self.sides[1].side_conditions.clone();
                    self.sides[3].side_conditions = side_1_conditions;
                }
            }
            GameType::FreeForAll => {
                // JS: FFA battles
                if self.sides.len() >= 4 {
                    // JS: this.sides[2].foe = this.sides[3];
                    self.sides[2].foe_index = Some(3);
                    // JS: this.sides[3].foe = this.sides[2];
                    self.sides[3].foe_index = Some(2);
                }
                // Fall through to set up sides 0 and 1
                if self.sides.len() >= 2 {
                    // JS: this.sides[1].foe = this.sides[0];
                    self.sides[1].foe_index = Some(0);
                    // JS: this.sides[0].foe = this.sides[1];
                    self.sides[0].foe_index = Some(1);
                }
            }
            _ => {
                // JS: Singles/Doubles battles (2 sides)
                if self.sides.len() >= 2 {
                    // JS: this.sides[1].foe = this.sides[0];
                    self.sides[1].foe_index = Some(0);
                    // JS: this.sides[0].foe = this.sides[1];
                    self.sides[0].foe_index = Some(1);
                }
            }
        }

        // JS: this.add('gen', this.gen);
        self.add_log("gen", &[&self.gen.to_string()]);

        // JS: this.add('tier', format.name);
        let format_name = self.format_name.clone();
        self.add_log("tier", &[&format_name]);

        // JS: if (this.rated) { ... }
        if let Some(ref rated) = self.rated.clone() {
            // JS: if (this.rated === 'Rated battle') this.rated = true;
            let rated_str = if rated == "Rated battle" { "" } else { rated };
            // JS: this.add('rated', typeof this.rated === 'string' ? this.rated : '');
            self.add_log("rated", &[rated_str]);
        }

        // JS: format.onBegin?.call(this);
        // TODO: Format callbacks require a callback registration system
        // JavaScript formats can have onBegin, onBeforeMove, onAfterMove, etc.
        // These cannot be deserialized from JSON - they must be registered separately
        // Similar to event system: Battle::on_event("FormatBegin", callback)

        // For now, emit an event that format-specific code can hook into
        self.run_event("FormatBegin", None, None, None, None);

        // JS: for (const rule of this.ruleTable.keys()) { subFormat.onBegin?.call(this); }
        if let Some(ref rule_table) = self.rule_table {
            // Collect rule keys to avoid borrow checker issues
            let rule_keys: Vec<String> = rule_table.keys()
                .map(|s| s.clone())
                .collect();

            for rule in rule_keys {
                // JS: if ('+*-!'.includes(rule.charAt(0))) continue;
                if let Some(first_char) = rule.chars().next() {
                    if first_char == '+' || first_char == '*' || first_char == '-' || first_char == '!' {
                        continue;
                    }
                }

                // JS: const subFormat = this.dex.formats.get(rule);
                // JS: subFormat.onBegin?.call(this);
                // TODO: Look up subFormat from Dex and invoke its onBegin callback
                // This requires:
                // 1. Dex::get_format() method to look up formats by ID
                // 2. Format callback registration system
                // For now, emit an event that rule-specific code can hook into
                self.run_event(&format!("RuleBegin:{}", rule), None, None, None, None);
            }
        }

        // JS: if (this.sides.some(side => !side.pokemon[0])) { throw new Error('...'); }
        if self.sides.iter().any(|side| side.pokemon.is_empty()) {
            panic!("Battle not started: A player has an empty team.");
        }

        // JS: if (this.debugMode) { this.checkEVBalance(); }
        if self.debug_mode {
            self.check_ev_balance();
        }

        // JS: if (format.customRules) { this.add(`raw|...`); }
        // Find the format and display custom rules if they exist
        if let Some(format) = self.dex.formats.iter().find(|f| {
            ID::new(&f.name.to_lowercase().replace(" ", "").replace("-", "")) == self.format_id
        }) {
            if let Some(custom_rules) = &format.custom_rules {
                if !custom_rules.is_empty() {
                    // JS: const plural = format.customRules.length === 1 ? '' : 's';
                    let plural = if custom_rules.len() == 1 { "" } else { "s" };
                    // JS: const open = format.customRules.length <= 5 ? ' open' : '';
                    let open = if custom_rules.len() <= 5 { " open" } else { "" };
                    // JS: this.add(`raw|<div class="infobox"><details class="readmore"${open}><summary><strong>${format.customRules.length} custom rule${plural}:</strong></summary> ${format.customRules.join(', ')}</details></div>`);
                    let rules_joined = custom_rules.join(", ");
                    self.add_log("raw", &[&format!("<div class=\"infobox\"><details class=\"readmore\"{open}><summary><strong>{} custom rule{plural}:</strong></summary> {rules_joined}</details></div>", custom_rules.len())]);
                }
            }
        }

        // JS: this.runPickTeam();
        self.run_pick_team();

        // JS: this.queue.addChoice({ choice: 'start' });
        use crate::battle_queue::{Action, FieldAction, FieldActionType};
        self.queue.add_choice(Action::Field(FieldAction {
            choice: FieldActionType::Start,
            priority: 0,
        }));

        // JS: this.midTurn = true;
        self.mid_turn = true;

        // JS: if (!this.requestState) this.turnLoop();
        if self.request_state == BattleRequestState::None {
            self.turn_loop();
        }
    }

    /// Start the first turn (after team preview)
    /// Equivalent to TypeScript runAction() case 'start' (battle.ts:2629-2700)
    /// Note: In TS this is part of runAction switch statement, extracted to separate method in Rust
    pub fn start_battle(&mut self) {
        self.add_log("start", &[]);
        self.turn = 1;
        self.add_log("turn", &[&self.turn.to_string()]);

        // Collect switch-in operations first to avoid borrow conflict
        let num_sides = self.sides.len();
        let active_per_half = self.active_per_half;
        let pokemon_counts: Vec<usize> = self.sides.iter().map(|s| s.pokemon.len()).collect();

        let mut switch_ops = Vec::new();
        for side_idx in 0..num_sides {
            for slot in 0..active_per_half {
                if slot < pokemon_counts[side_idx] {
                    switch_ops.push((side_idx, slot, slot));
                }
            }
        }

        for (side_idx, slot, pokemon_idx) in &switch_ops {
            self.switch_in(*side_idx, *slot, *pokemon_idx, None, false);
        }

        // Trigger switch-in abilities after all Pokemon are on the field
        for (side_idx, _slot, pokemon_idx) in switch_ops {
            self.trigger_switch_in_abilities(side_idx, pokemon_idx);
        }

        self.request_state = BattleRequestState::Move;
        for side in &mut self.sides {
            side.request_state = RequestState::Move;
        }
    }

    /// Switch a Pokemon in
    /// 1:1 port of switchIn from battle-actions.ts
    /// Returns false if switch failed, true if successful, or "pursuitfaint" string converted to SwitchResult
    pub fn switch_in(
        &mut self,
        side_index: usize,
        pos: usize,
        pokemon_index: usize,
        source_effect: Option<&ID>,
        is_drag: bool,
    ) -> SwitchResult {
        // Check if pokemon exists and is not already active
        let side = match self.sides.get(side_index) {
            Some(s) => s,
            None => return SwitchResult::Failed,
        };

        let pokemon_is_active = match side.pokemon.get(pokemon_index) {
            Some(p) => p.is_active,
            None => return SwitchResult::Failed,
        };

        if pokemon_is_active {
            self.hint("A switch failed because the Pokémon trying to switch in is already in.", false, None);
            return SwitchResult::Failed;
        }

        if pos >= side.active.len() {
            return SwitchResult::Failed;
        }

        // Get the old active Pokemon index if any
        let old_active_idx = side.active.get(pos).and_then(|&opt| opt);

        // Handle old active Pokemon switching out
        if let Some(old_idx) = old_active_idx {
            let side = &self.sides[side_index];
            let old_pokemon = &side.pokemon[old_idx];

            if old_pokemon.hp > 0 {
                // Mark as being called back
                self.sides[side_index].pokemon[old_idx].being_called_back = true;

                // Run BeforeSwitchOut event (unless skipBeforeSwitchOutEventFlag or is_drag)
                let skip_event = self.sides[side_index].pokemon[old_idx].skip_before_switch_out_event_flag;
                if !skip_event && !is_drag {
                    self.run_event("BeforeSwitchOut", Some((side_index, old_idx)), None, None, None);
                    if self.gen >= 5 {
                        self.each_event_internal("Update");
                    }
                }

                self.sides[side_index].pokemon[old_idx].skip_before_switch_out_event_flag = false;

                // Run SwitchOut event
                if !self.run_event_bool("SwitchOut", Some((side_index, old_idx)), None, None) {
                    return SwitchResult::Failed;
                }

                // Check if fainted from Pursuit
                if self.sides[side_index].pokemon[old_idx].hp == 0 {
                    return SwitchResult::PursuitFaint;
                }

                // Will definitely switch out at this point
                self.sides[side_index].pokemon[old_idx].illusion = None;

                // Trigger End events for ability and item
                let ability_id = self.sides[side_index].pokemon[old_idx].ability.clone();
                self.single_event("End", &ability_id, Some((side_index, old_idx)), None, None);
                let item_id = self.sides[side_index].pokemon[old_idx].item.clone();
                self.single_event("End", &item_id, Some((side_index, old_idx)), None, None);

                // Cancel any queued action
                self.queue.cancel_action(side_index, old_idx);

                // Clear volatiles on old Pokemon
                self.sides[side_index].pokemon[old_idx].clear_volatiles();
            }

            // Update old active state
            let old_position = self.sides[side_index].pokemon[pokemon_index].position;
            {
                let old_pokemon = &mut self.sides[side_index].pokemon[old_idx];
                old_pokemon.is_active = false;
                old_pokemon.is_started = false;
                old_pokemon.used_item_this_turn = false;
                old_pokemon.stats_raised_this_turn = false;
                old_pokemon.stats_lowered_this_turn = false;
                old_pokemon.position = old_position;
                if old_pokemon.fainted {
                    old_pokemon.status = ID::empty();
                }
            }

            // Swap positions
            let new_position = pos;
            self.sides[side_index].pokemon[pokemon_index].position = new_position;
            self.sides[side_index].pokemon.swap(pokemon_index, old_idx);
        }

        // Set up new active Pokemon
        {
            let side = &mut self.sides[side_index];
            let pokemon = &mut side.pokemon[pokemon_index];

            pokemon.is_active = true;
            side.active[pos] = Some(pokemon_index);
            pokemon.active_turns = 0;
            pokemon.active_move_actions = 0;

            // Reset move.used for all moves
            for move_slot in &mut pokemon.move_slots {
                move_slot.used = false;
            }

            // Initialize ability and item state
            pokemon.ability_state = EffectState::new(pokemon.ability.clone());
            pokemon.item_state = EffectState::new(pokemon.item.clone());
        }

        // Run BeforeSwitchIn event
        self.run_event("BeforeSwitchIn", Some((side_index, pokemon_index)), None, None, None);

        // Log the switch
        let (details, hp_display) = {
            let pokemon = &self.sides[side_index].pokemon[pokemon_index];
            let details = pokemon.details();
            let hp = format!("{}/{}", pokemon.hp, pokemon.maxhp);
            (details, hp)
        };
        let side_id = self.sides[side_index].id_str().to_string();
        let pokemon_name = self.sides[side_index].pokemon[pokemon_index].name.clone();

        let event_type = if is_drag { "drag" } else { "switch" };
        if let Some(effect) = source_effect {
            self.log.push(format!("|{}|{}: {}|{}|{}|[from] {}",
                event_type, side_id, pokemon_name, details, hp_display, effect.as_str()));
        } else {
            self.log.push(format!("|{}|{}: {}|{}|{}",
                event_type, side_id, pokemon_name, details, hp_display));
        }

        // Gen 2 drag tracking
        if is_drag && self.gen == 2 {
            self.sides[side_index].pokemon[pokemon_index].dragged_in = Some(self.turn as usize);
        }
        self.sides[side_index].pokemon[pokemon_index].previously_switched_in += 1;

        // Apply hazards (ability triggers are called separately)
        self.apply_hazards(side_index, pos, pokemon_index);

        // Run switch or queue it
        if is_drag && self.gen >= 5 {
            // runSwitch happens immediately so that Mold Breaker can make hazards bypass Clear Body and Levitate
            self.run_switch(side_index, pokemon_index);
        } else {
            self.queue.insert_run_switch(side_index, pokemon_index);
        }

        SwitchResult::Success
    }

    /// Run each event for all active Pokemon
    fn each_event_internal(&mut self, event_name: &str) {
        let active: Vec<(usize, usize)> = self.get_all_active(false);
        for (side_idx, poke_idx) in active {
            let effect_id = ID::new(event_name);
            self.single_event(event_name, &effect_id, Some((side_idx, poke_idx)), None, None);
        }
    }

    /// Add a log entry
    /// Equivalent to Battle.add() in battle.ts (called throughout for protocol logging)
    //
    // 	add(...args: (ProtocolArg | ProtocolArgs)[]) {
    // 		this.log.push(`|${args.join('|')}`);
    // 	}
    //
    pub fn add_log(&mut self, event_type: &str, args: &[&str]) {
        let mut entry = format!("|{}", event_type);
        for arg in args {
            entry.push('|');
            entry.push_str(arg);
        }
        self.log.push(entry);
    }

    /// Random number in [0, n)
    // 
    // 	random(m?: number, n?: number) {
    // 		return this.prng.random(m, n);
    // 	}
    //
    pub fn random(&mut self, n: i32) -> i32 {
        self.prng.random_int(n)
    }

    /// Random chance
    // 
    // 	randomChance(numerator: number, denominator: number) {
    // 		if (this.forceRandomChance !== null) return this.forceRandomChance;
    // 		return this.prng.randomChance(numerator, denominator);
    // 	}
    //
    pub fn random_chance(&mut self, numerator: i32, denominator: i32) -> bool {
        if let Some(forced) = self.force_random_chance {
            return forced;
        }
        self.prng.random_chance(numerator, denominator)
    }

    /// Sample from a slice
    // 
    // 	sample<T>(items: readonly T[]): T {
    // 		return this.prng.sample(items);
    // 	}
    //
    pub fn sample<'a, T>(&mut self, items: &'a [T]) -> Option<&'a T> {
        self.prng.sample(items)
    }

    /// Shuffle a slice in place
    /// JavaScript calls this.prng.shuffle() directly (no Battle wrapper method)
    /// This is a Rust convenience wrapper following the pattern of sample/random/random_chance
    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        self.prng.shuffle(items);
    }

    /// Get a side by ID
    // 
    // 	getSide(sideid: SideID): Side {
    // 		return this.sides[parseInt(sideid[1]) - 1];
    // 	}
    //
    pub fn get_side(&self, side_id: SideID) -> Option<&Side> {
        self.sides.get(side_id.index())
    }

    /// Get a mutable side by ID
    /// Rust-specific helper for mutable access (JavaScript doesn't need this due to no borrow checker)
    pub fn get_side_mut(&mut self, side_id: SideID) -> Option<&mut Side> {
        self.sides.get_mut(side_id.index())
    }

    /// Get P1
    //
    // 	get p1() {
    // 		return this.sides[0];
    // 	}
    //
    pub fn p1(&self) -> Option<&Side> {
        self.sides.get(0)
    }

    /// Get P2
    //
    // 	get p2() {
    // 		return this.sides[1];
    // 	}
    //
    pub fn p2(&self) -> Option<&Side> {
        self.sides.get(1)
    }

    /// Get all active Pokemon
    /// Get all active Pokemon, optionally including fainted ones
    /// Equivalent to battle.ts getAllActive(includeFainted?)
    // 
    // 	getAllActive(includeFainted?: boolean) {
    // 		const pokemonList: Pokemon[] = [];
    // 		for (const side of this.sides) {
    // 			for (const pokemon of side.active) {
    // 				if (pokemon && (includeFainted || !pokemon.fainted)) {
    // 					pokemonList.push(pokemon);
    // 				}
    // 			}
    // 		}
    // 		return pokemonList;
    // 	}
    //
    pub fn get_all_active(&self, include_fainted: bool) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        for (side_idx, side) in self.sides.iter().enumerate() {
            for (slot, opt_idx) in side.active.iter().enumerate() {
                if let Some(poke_idx) = opt_idx {
                    if let Some(pokemon) = side.pokemon.get(*poke_idx) {
                        if include_fainted || !pokemon.is_fainted() {
                            result.push((side_idx, *poke_idx));
                        }
                    }
                }
            }
        }
        result
    }

    /// Check if battle is over
    /// Check if the battle has a winner
    /// Equivalent to battle.ts checkWin()
    // 
    // 	checkWin(faintData?: Battle['faintQueue'][0]) {
    // 		if (this.sides.every(side => !side.pokemonLeft)) {
    // 			this.win(faintData && this.gen > 4 ? faintData.target.side : null);
    // 			return true;
    // 		}
    // 		for (const side of this.sides) {
    // 			if (!side.foePokemonLeft()) {
    // 				this.win(side);
    // 				return true;
    // 			}
    // 		}
    // 	}
    //
    pub fn check_win(&mut self, faint_data: Option<FaintData>) -> bool {
        // JavaScript: checkWin(faintData?: Battle['faintQueue'][0])

        // Check if all sides have no Pokemon left - tie/draw scenario
        if self.sides.iter().all(|side| side.pokemon_left == 0) {
            // JS: this.win(faintData && this.gen > 4 ? faintData.target.side : null);
            // In Gen 5+, the side that fainted last wins
            let winner = if let Some(faint_data) = faint_data {
                if self.gen > 4 {
                    // Extract the side_idx from faint_data.target (side_idx, poke_idx)
                    let (side_idx, _) = faint_data.target;
                    // Get the Side's ID
                    Some(self.sides[side_idx].id)
                } else {
                    None
                }
            } else {
                None
            };
            self.win(winner);
            return true;
        }

        // Check each side to see if all their foes have no Pokemon left
        for (i, side) in self.sides.iter().enumerate() {
            // Check if this side's foes have no Pokemon left
            let foe_pokemon_left = self.get_foe_pokemon_left(i);
            if foe_pokemon_left == 0 {
                // This side wins
                self.win(Some(side.id));
                return true;
            }
        }

        false
    }

    /// Get total Pokemon left for all foes of a side
    /// Rust helper - JavaScript calls side.foePokemonLeft() directly
    /// This helper calculates foe Pokemon count from the Battle level
    fn get_foe_pokemon_left(&self, side_idx: usize) -> usize {
        match self.game_type {
            GameType::FreeForAll => {
                // In FFA, all other sides are foes
                self.sides.iter().enumerate()
                    .filter(|(i, _)| *i != side_idx)
                    .map(|(_, s)| s.pokemon_left)
                    .sum()
            }
            GameType::Multi => {
                // In multi battles, the opposing team is the foe
                // P1/P3 are a team, P2/P4 are a team
                if side_idx == 0 || side_idx == 2 {
                    // Foes are P2 and P4
                    self.sides.get(1).map(|s| s.pokemon_left).unwrap_or(0) +
                    self.sides.get(3).map(|s| s.pokemon_left).unwrap_or(0)
                } else {
                    // Foes are P1 and P3
                    self.sides.get(0).map(|s| s.pokemon_left).unwrap_or(0) +
                    self.sides.get(2).map(|s| s.pokemon_left).unwrap_or(0)
                }
            }
            _ => {
                // Singles or doubles - foe is the opposite side
                let foe_idx = if side_idx == 0 { 1 } else { 0 };
                self.sides.get(foe_idx).map(|s| s.pokemon_left).unwrap_or(0)
            }
        }
    }

    /// End the battle (placeholder method - JavaScript has empty implementation)
    /// The actual battle-ending logic is in win()
    // TypeScript source:
    //  end() {}
    //
    pub fn end(&mut self) {
        // JavaScript implementation is empty - placeholder method
        // All battle-ending logic should use win() instead
    }

    /// Get the next effect order number
    /// Rust helper - JavaScript uses this.effectOrder++ inline in initEffectState
    pub fn next_effect_order(&mut self) -> i32 {
        self.effect_order += 1;
        self.effect_order
    }

    /// Initialize an effect state
    // 
    // 	initEffectState(obj: Partial<EffectState>, effectOrder?: number): EffectState {
    // 		if (!obj.id) obj.id = '';
    // 		if (effectOrder !== undefined) {
    // 			obj.effectOrder = effectOrder;
    // 		} else if (obj.id && obj.target && (!(obj.target instanceof Pokemon) || obj.target.isActive)) {
    // 			obj.effectOrder = this.effectOrder++;
    // 		} else {
    // 			obj.effectOrder = 0;
    // 		}
    // 		return obj as EffectState;
    // 	}
    //
    pub fn init_effect_state(&mut self, id: ID) -> EffectState {
        let mut state = EffectState::new(id);
        state.effect_order = self.next_effect_order();
        state
    }

    /// Process a choice from a player
    // TypeScript source:
    // /**
    // 	 * Takes a choice string passed from the client. Starts the next
    // 	 * turn if all required choices have been made.
    // 	 */
    // 	choose(sideid: SideID, input: string) {
    // 		const side = this.getSide(sideid);
    // 
    // 		if (!side.choose(input)) {
    // 			if (!side.choice.error) {
    // 				side.emitChoiceError(`Unknown error for choice: ${input}. If you're not using a custom client, please report this as a bug.`);
    // 			}
    // 			return false;
    // 		}
    // 
    // 		if (!side.isChoiceDone()) {
    // 			side.emitChoiceError(`Incomplete choice: ${input} - missing other pokemon`);
    // 			return false;
    // 		}
    // 		if (this.allChoicesDone()) this.commitChoices();
    // 		return true;
    // 	}
    //
    pub fn choose(&mut self, side_id: SideID, choice: &str) -> Result<(), String> {
        let side_idx = side_id.index();
        if side_idx >= self.sides.len() {
            return Err(format!("Invalid side: {}", side_id.to_str()));
        }

        self.input_log.push(format!(">{} {}", side_id.to_str(), choice));

        // Check if this is a comma-separated choice (for Doubles/Triples with multiple active Pokemon)
        if choice.contains(',') {
            // Split by comma and validate each sub-choice
            let sub_choices: Vec<&str> = choice.split(',').map(|s| s.trim()).collect();

            // Validate each sub-choice individually with its pokemon_index
            for (pokemon_index, sub_choice) in sub_choices.iter().enumerate() {
                self.validate_single_choice(side_id, sub_choice, Some(pokemon_index))?;
            }
            Ok(())
        } else {
            // Single choice (Singles or single slot in Doubles/Triples)
            // In singles, pokemon_index is 0
            self.validate_single_choice(side_id, choice, Some(0))
        }
    }

    /// Validate a single choice (not comma-separated)
    /// pokemon_index: Which active slot this choice is for (0-2 in triples, 0-1 in doubles, 0 in singles)
    fn validate_single_choice(&mut self, side_id: SideID, choice: &str, pokemon_index: Option<usize>) -> Result<(), String> {
        // Parse and validate choice
        let parts: Vec<&str> = choice.split_whitespace().collect();
        if parts.is_empty() {
            return Err("Empty choice".to_string());
        }

        let choice_type = parts[0];

        // Validate choice based on current request state
        match self.request_state {
            BattleRequestState::TeamPreview => {
                // During team preview, only 'team' choices are valid
                if choice_type != "team" {
                    return Err(format!("[Invalid choice] Can't {} for Team Preview: You must select a team order", choice_type));
                }
            }
            BattleRequestState::Switch => {
                // During switch request, only 'switch' (and 'pass' in non-singles) are valid
                if choice_type == "switch" {
                    // Switch is always valid
                } else if choice_type == "pass" {
                    // Pass is only valid in Doubles/Triples, not in Singles
                    if matches!(self.game_type, GameType::Singles) {
                        return Err("[Invalid choice] Can't pass during switch request in Singles: You must switch a Pokemon".to_string());
                    }
                } else {
                    return Err(format!("[Invalid choice] Can't {} during switch request: You must switch a Pokemon", choice_type));
                }
            }
            _ => {
                // In other states (Move, None), certain choices are not valid
                if choice_type == "team" && !matches!(self.request_state, BattleRequestState::TeamPreview) {
                    return Err("[Invalid choice] Team choices are only valid during Team Preview".to_string());
                }
                // Pass validation:
                // - During Switch requests: always valid (except in Singles)
                // - During Move requests in Doubles/Triples: valid for fainted Pokemon slots
                // - Otherwise: invalid
                if choice_type == "pass" {
                    if matches!(self.request_state, BattleRequestState::Switch) {
                        // Already handled above in the Switch case
                    } else if matches!(self.request_state, BattleRequestState::Move) {
                        // In doubles/triples during Move request, pass is valid for fainted Pokemon
                        if !matches!(self.game_type, GameType::Doubles | GameType::Triples) {
                            return Err("[Invalid choice] Can't pass: You can only pass during switch requests".to_string());
                        }
                        // Check if this slot has a fainted Pokemon
                        let side_idx = side_id.index();
                        if let Some(poke_idx) = pokemon_index {
                            if let Some(active_poke_idx) = self.sides[side_idx].active.get(poke_idx).and_then(|&x| x) {
                                let pokemon = &self.sides[side_idx].pokemon[active_poke_idx];
                                if !pokemon.is_fainted() {
                                    return Err("[Invalid choice] Can't pass: Pokemon is not fainted".to_string());
                                }
                            }
                        }
                    } else {
                        return Err("[Invalid choice] Can't pass: You can only pass during switch requests".to_string());
                    }
                }
            }
        }

        match choice_type {
            "move" => {
                // Parse move choice: move <move> [target] [modifier]
                // Examples: "move 1", "move tackle 2", "move 1 +1 mega", "move shadowball zmove 1"
                // Handle multi-word moves like "Conversion 2"
                if parts.len() < 2 {
                    return Err("Move choice requires move name/number".to_string());
                }

                // Try to parse move name - could be multi-word like "Conversion 2"
                // Start with the longest possible name and work backwards
                let mut move_name_parts = 1;
                let mut target_count = 0;
                let mut has_mega = false;
                let mut has_zmove = false;
                let mut has_dynamax = false;
                let mut has_ultra = false;

                // If first part after "move" is a number, it's move slot (1, 2, 3, 4)
                // Otherwise, try to match move names
                if parts[1].parse::<usize>().is_err() {
                    // Not a number - could be multi-word move name
                    // Try matching progressively longer names
                    // e.g. "move Conversion 2 zmove 2" could be:
                    //   - "Conversion" (move) + "2" (target) + "zmove" + "2" (target again - error)
                    //   - "Conversion 2" (move) + "zmove" + "2" (target)

                    // For now, simple heuristic: if next part is numeric and not a modifier, include it in move name
                    // This handles cases like "Conversion 2" where "2" is part of the move name
                    let mut i = 2;
                    while i < parts.len() && move_name_parts < 3 { // Limit to 3 words max
                        let part = parts[i];
                        // If it's a number and we don't have modifiers yet, could be part of move name
                        let is_modifier = matches!(part.to_lowercase().as_str(),
                            "mega" | "zmove" | "dynamax" | "max" | "ultra");

                        // Targets are: positive numbers, or explicit + relative (not -)
                        let is_explicit_relative = part.starts_with('+') &&
                                                   part.len() > 1 &&
                                                   part[1..].parse::<i32>().is_ok();
                        let is_absolute = part.parse::<usize>().is_ok() && !part.starts_with('-');
                        let is_likely_target = is_explicit_relative || (is_absolute && i > 2);

                        if is_modifier || is_likely_target {
                            break; // Stop extending move name
                        }

                        // Check if this could be part of move name (number in position 2 could be like "2" in "Conversion 2")
                        if i == 2 && part.parse::<usize>().is_ok() {
                            // Might be "Conversion 2" - include it
                            move_name_parts += 1;
                            i += 1;
                        } else {
                            break;
                        }
                    }
                }

                // Now scan modifiers and targets starting after move name
                let mut i = 1 + move_name_parts;
                while i < parts.len() {
                    let part = parts[i];

                    // Check if it's a modifier
                    match part.to_lowercase().as_str() {
                        "mega" => {
                            if has_mega || has_zmove || has_ultra {
                                return Err("[Invalid choice] Can't combine multiple evolution/burst modifiers".to_string());
                            }
                            has_mega = true;
                        }
                        "zmove" => {
                            if has_mega || has_zmove || has_dynamax {
                                return Err("[Invalid choice] Can't combine multiple move modifiers".to_string());
                            }
                            has_zmove = true;
                        }
                        "dynamax" | "max" => {
                            if has_zmove || has_dynamax {
                                return Err("[Invalid choice] Can't combine multiple move modifiers".to_string());
                            }
                            has_dynamax = true;
                        }
                        "ultra" => {
                            if has_mega || has_ultra {
                                return Err("[Invalid choice] Can't combine multiple evolution/burst modifiers".to_string());
                            }
                            has_ultra = true;
                        }
                        _ => {
                            // Check if it's a target (positive number or explicit + relative target)
                            // Only + prefix is valid for relative targets, not -
                            let is_explicit_relative = part.starts_with('+') &&
                                                       part.len() > 1 &&
                                                       part[1..].parse::<i32>().is_ok();
                            let is_absolute = part.parse::<usize>().is_ok() && !part.starts_with('-');

                            let is_target = is_explicit_relative || is_absolute;

                            if is_target {
                                target_count += 1;
                                if target_count > 1 {
                                    return Err("[Invalid choice] Move can only have one target".to_string());
                                }
                            } else {
                                // Check if it looks like an invalid target (negative number)
                                if part.starts_with('-') && part[1..].parse::<i32>().is_ok() {
                                    return Err("[Invalid choice] Invalid target format".to_string());
                                }
                                // Not a modifier or target - might be part of move name
                                // For now, we'll allow it (multi-word move names)
                            }
                        }
                    }
                    i += 1;
                }

                // In Singles, zmove and mega require a target
                if matches!(self.game_type, GameType::Singles) {
                    if (has_zmove || has_mega) && target_count == 0 {
                        return Err("[Invalid choice] Z-moves and Mega Evolution require a target in Singles".to_string());
                    }
                }

                // Validate that Pokemon has required item for zmove/mega
                if has_zmove || has_mega {
                    let side_idx = side_id.index();
                    if let Some(poke_idx) = pokemon_index {
                        // Get the active Pokemon at this slot
                        if let Some(active_poke_idx) = self.sides[side_idx].active.get(poke_idx).and_then(|&x| x) {
                            let pokemon = &self.sides[side_idx].pokemon[active_poke_idx];
                            let item_id = pokemon.item.as_str();

                            if has_zmove {
                                // Check if Pokemon has a Z-crystal
                                // Z-crystals end with "iumz" (e.g., "normaliumz", "firiumz")
                                if !item_id.ends_with("iumz") {
                                    return Err("[Invalid choice] Can't use Z-Move: Pokemon doesn't have a Z-Crystal".to_string());
                                }
                            }

                            if has_mega {
                                // Check if Pokemon has a mega stone
                                // Mega stones end with "ite" (e.g., "gengarite", "charizarditex")
                                if !item_id.ends_with("ite") && !item_id.ends_with("itex") && !item_id.ends_with("itey") {
                                    return Err("[Invalid choice] Can't Mega Evolve: Pokemon doesn't have a Mega Stone".to_string());
                                }
                            }
                        }
                    }
                }

                Ok(())
            }
            "switch" => {
                // Parse switch choice
                if parts.len() < 2 {
                    return Err("Switch choice requires Pokemon number or name".to_string());
                }
                // Validate that the argument is either a number or a valid Pokemon name
                // Accept both numeric (e.g., "switch 2") and name-based (e.g., "switch Bulbasaur")
                let arg = parts[1];
                if arg.parse::<usize>().is_err() {
                    // Not a number - check if it's an obvious invalid word
                    // Reject common English words that are clearly not Pokemon names
                    let lowercase_arg = arg.to_lowercase();
                    let invalid_words = vec!["first", "second", "third", "fourth", "fifth", "last", "next"];
                    if invalid_words.contains(&lowercase_arg.as_str()) {
                        return Err(format!("[Invalid choice] Can't switch: You must specify a Pokemon by number or name"));
                    }
                    // Otherwise assume it's a Pokemon name (will be validated during execution)
                }
                // Would validate and add to queue here
                Ok(())
            }
            "team" => {
                // Parse team order choice (for team preview)
                if parts.len() < 2 {
                    return Err("Team choice requires Pokemon order".to_string());
                }
                // Validate that the argument is a number
                match parts[1].parse::<usize>() {
                    Ok(num) => {
                        // Pokemon numbering is 1-based, reject 0
                        if num == 0 {
                            return Err("[Invalid choice] Can't choose for Team Preview: You must select a team order".to_string());
                        }
                        Ok(())
                    }
                    Err(_) => {
                        return Err("[Invalid choice] Can't choose for Team Preview: You must select a team order".to_string());
                    }
                }
            }
            "pass" => {
                // Pass should not have any choice details
                if parts.len() > 1 {
                    return Err("[Invalid choice] Pass does not accept any arguments".to_string());
                }
                Ok(())
            }
            "shift" => {
                // Shift is only valid in triples
                if !matches!(self.game_type, GameType::Triples) {
                    return Err("[Invalid choice] Shift is only valid in Triple Battles".to_string());
                }
                // Shift cannot be used in the center position (slot 1)
                if let Some(poke_idx) = pokemon_index {
                    if poke_idx == 1 {
                        return Err("[Invalid choice] The center Pokemon cannot shift position".to_string());
                    }
                }
                // Shift does not accept any arguments
                if parts.len() > 1 {
                    return Err("[Invalid choice] Shift does not accept any arguments".to_string());
                }
                Ok(())
            }
            _ => Err(format!("Unknown choice type: {}", choice_type)),
        }
    }

    /// Get the battle log as a string
    /// Rust convenience method - JavaScript directly accesses this.log array
    pub fn get_log(&self) -> String {
        self.log.join("\n")
    }

    /// Make choices for both sides and run the turn
    /// This is the main entry point for progressing the battle
    // TypeScript source:
    // /**
    // 	 * Convenience method for easily making choices.
    // 	 */
    // 	makeChoices(...inputs: string[]) {
    // 		if (inputs.length) {
    // 			for (const [i, input] of inputs.entries()) {
    // 				if (input) this.sides[i].choose(input);
    // 			}
    // 		} else {
    // 			for (const side of this.sides) {
    // 				side.autoChoose();
    // 			}
    // 		}
    // 		this.commitChoices();
    // 	}
    //
    pub fn make_choices(&mut self, p1_choice: &str, p2_choice: &str) {
        // Parse and validate choices
        self.parse_choice(SideID::P1, p1_choice);
        self.parse_choice(SideID::P2, p2_choice);

        // Log choices
        if !p1_choice.is_empty() {
            self.input_log.push(format!(">p1 {}", p1_choice));
        }
        if !p2_choice.is_empty() {
            self.input_log.push(format!(">p2 {}", p2_choice));
        }

        // Run the turn
        self.commit_choices();
    }

    /// Parse a choice string and store the actions
    /// Rust helper - JavaScript has side.choose() which handles parsing
    /// This is split out in Rust to work around borrow checker constraints
    fn parse_choice(&mut self, side_id: SideID, choice: &str) {
        let side_idx = side_id.index();
        if side_idx >= self.sides.len() {
            return;
        }

        // Clear previous choice
        self.sides[side_idx].choice.clear();

        // Parse the choice string
        let parts: Vec<&str> = choice.split(',').map(|s| s.trim()).collect();

        for (slot, part) in parts.iter().enumerate() {
            if slot >= self.active_per_half {
                break;
            }

            let words: Vec<&str> = part.split_whitespace().collect();
            if words.is_empty() {
                continue;
            }

            let action = match words[0] {
                "move" => {
                    let move_id = if words.len() > 1 {
                        // Could be a move name or number
                        if let Ok(num) = words[1].parse::<usize>() {
                            // Move by number (1-indexed)
                            if let Some(Some(poke_idx)) = self.sides[side_idx].active.get(slot) {
                                if let Some(pokemon) = self.sides[side_idx].pokemon.get(*poke_idx) {
                                    if num > 0 && num <= pokemon.move_slots.len() {
                                        Some(pokemon.move_slots[num - 1].id.clone())
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            Some(ID::new(words[1]))
                        }
                    } else {
                        None
                    };

                    // Check Choice item lock - if locked, override with locked move
                    let move_id = if let Some(Some(poke_idx)) = self.sides[side_idx].active.get(slot) {
                        if let Some(pokemon) = self.sides[side_idx].pokemon.get(*poke_idx) {
                            if let Some(ref locked) = pokemon.locked_move {
                                Some(locked.clone())
                            } else {
                                move_id
                            }
                        } else {
                            move_id
                        }
                    } else {
                        move_id
                    };

                    // Parse target if present
                    let target_loc = if words.len() > 2 {
                        words[2].parse::<i8>().ok()
                    } else {
                        None
                    };

                    // Check for mega/zmove flags
                    let mega = words.iter().any(|&w| w == "mega");
                    let terastallize = if words.iter().any(|&w| w == "terastallize") {
                        self.sides[side_idx].get_active(slot)
                            .and_then(|p| p.tera_type.clone())
                    } else {
                        None
                    };

                    crate::side::ChosenAction {
                        choice: crate::side::ChoiceType::Move,
                        pokemon_index: slot,
                        target_loc,
                        move_id,
                        switch_index: None,
                        mega,
                        zmove: None,
                        max_move: None,
                        terastallize,
                    }
                }
                "switch" => {
                    let switch_idx = if words.len() > 1 {
                        words[1].parse::<usize>().ok().map(|n| n.saturating_sub(1))
                    } else {
                        None
                    };

                    crate::side::ChosenAction {
                        choice: crate::side::ChoiceType::Switch,
                        pokemon_index: slot,
                        target_loc: None,
                        move_id: None,
                        switch_index: switch_idx,
                        mega: false,
                        zmove: None,
                        max_move: None,
                        terastallize: None,
                    }
                }
                "pass" | _ => {
                    crate::side::ChosenAction {
                        choice: crate::side::ChoiceType::Pass,
                        pokemon_index: slot,
                        target_loc: None,
                        move_id: None,
                        switch_index: None,
                        mega: false,
                        zmove: None,
                        max_move: None,
                        terastallize: None,
                    }
                }
            };

            self.sides[side_idx].choice.actions.push(action);
        }
    }

    /// Commit choices and run the turn
    // 
    // 	commitChoices() {
    // 		this.updateSpeed();
    // 
    // 		// Sometimes you need to make switch choices mid-turn (e.g. U-turn,
    // 		// fainting). When this happens, the rest of the turn is saved (and not
    // 		// re-sorted), but the new switch choices are sorted and inserted before
    // 		// the rest of the turn.
    // 		const oldQueue = this.queue.list;
    // 		this.queue.clear();
    // 		if (!this.allChoicesDone()) throw new Error("Not all choices done");
    // 
    // 		for (const side of this.sides) {
    // 			const choice = side.getChoice();
    // 			if (choice) this.inputLog.push(`>${side.id} ${choice}`);
    // 		}
    // 		for (const side of this.sides) {
    // 			this.queue.addChoice(side.choice.actions);
    // 		}
    // 		this.clearRequest();
    // 
    // 		this.queue.sort();
    // 		this.queue.list.push(...oldQueue);
    // 
    // 		this.requestState = '';
    // 		for (const side of this.sides) {
    // 			side.activeRequest = null;
    // 		}
    // 
    // 		this.turnLoop();
    // 
    // 		// workaround for tests
    // 		if (this.log.length - this.sentLogPos > 500) this.sendUpdates();
    // 	}
    //
    fn commit_choices(&mut self) {
        // Build action queue
        self.queue.clear();

        // Collect all move actions with their priorities and speeds
        let mut actions: Vec<(usize, usize, crate::side::ChosenAction, i8, i32)> = Vec::new();

        for side_idx in 0..self.sides.len() {
            for action in &self.sides[side_idx].choice.actions {
                match action.choice {
                    crate::side::ChoiceType::Move => {
                        let pokemon_idx = self.sides[side_idx].active.get(action.pokemon_index)
                            .and_then(|opt| *opt);
                        if let Some(poke_idx) = pokemon_idx {
                            let priority = if let Some(ref move_id) = action.move_id {
                                self.get_move_priority(move_id)
                            } else {
                                0
                            };
                            let speed = self.sides[side_idx].pokemon[poke_idx].stored_stats.spe as i32;
                            actions.push((side_idx, poke_idx, action.clone(), priority, speed));
                        }
                    }
                    crate::side::ChoiceType::Switch => {
                        // Switches happen before moves (priority 7 effectively)
                        // When switching into an empty slot (after fainting), there's no current Pokemon
                        let pokemon_idx = self.sides[side_idx].active.get(action.pokemon_index)
                            .and_then(|opt| *opt);

                        // Get speed for ordering (use 0 if slot is empty)
                        let speed = if let Some(poke_idx) = pokemon_idx {
                            self.sides[side_idx].pokemon[poke_idx].stored_stats.spe as i32
                        } else {
                            0 // Empty slot - use speed 0 for ordering
                        };

                        // Use pokemon_idx if available, otherwise use 0 as placeholder
                        // The actual switch will use action.switch_index to determine what to switch in
                        let poke_idx_for_queue = pokemon_idx.unwrap_or(0);
                        actions.push((side_idx, poke_idx_for_queue, action.clone(), 7, speed));
                    }
                    _ => {}
                }
            }
        }

        // Check if Trick Room is active (reverses speed order)
        let trick_room_id = ID::new("trickroom");
        let trick_room_active = self.field.has_pseudo_weather(&trick_room_id);

        // Sort by priority (desc), then speed (desc, or asc if Trick Room), then random
        let tie_breaker = self.random(2) == 0;
        actions.sort_by(|a, b| {
            let priority_cmp = b.3.cmp(&a.3); // Higher priority first
            if priority_cmp != std::cmp::Ordering::Equal {
                return priority_cmp;
            }
            // Speed comparison: normally higher speed first, but reversed in Trick Room
            let speed_cmp = if trick_room_active {
                a.4.cmp(&b.4) // Lower speed first in Trick Room
            } else {
                b.4.cmp(&a.4) // Higher speed first normally
            };
            if speed_cmp != std::cmp::Ordering::Equal {
                return speed_cmp;
            }
            // Speed tie - use random tie breaker
            if tie_breaker {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });

        // Execute actions in order
        for (side_idx, poke_idx, action, _, _) in actions {
            if self.ended {
                break;
            }

            match action.choice {
                crate::side::ChoiceType::Switch => {
                    if let Some(switch_to) = action.switch_index {
                        // Use the slot from action.pokemon_index directly
                        // This works for both occupied and empty slots (after fainting)
                        let slot = action.pokemon_index;
                        self.do_switch(side_idx, slot, switch_to);
                    }
                }
                crate::side::ChoiceType::Move => {
                    if let Some(move_id) = &action.move_id {
                        let target_loc = action.target_loc.unwrap_or(0);
                        self.run_move(side_idx, poke_idx, move_id, target_loc);

                        // Handle pivot switch (U-Turn, Volt Switch, Flip Turn)
                        let pivot_switch_id = ID::new("pivotswitch");
                        if self.sides[side_idx].pokemon[poke_idx].has_volatile(&pivot_switch_id) {
                            self.sides[side_idx].pokemon[poke_idx].remove_volatile(&pivot_switch_id);

                            // Find a valid switch target
                            let switch_target = self.find_valid_switch_target(side_idx, poke_idx);
                            if let Some(target_idx) = switch_target {
                                // Get slot from the Pokemon's position
                                let slot = self.sides[side_idx].pokemon.get(poke_idx)
                                    .map(|p| p.position)
                                    .unwrap_or(0);
                                self.do_switch(side_idx, slot, target_idx);
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // End of turn
        self.run_residual();

        // Check for fainted Pokemon
        self.faint_messages(false, false, true);

        // Check win condition
        if self.check_win(None) {
            return;
        }

        // Start next turn
        self.next_turn();
    }

    /// Find a valid switch target for pivot moves (U-Turn, Volt Switch, etc.)
    fn find_valid_switch_target(&self, side_idx: usize, current_poke_idx: usize) -> Option<usize> {
        // Find the first non-active, non-fainted Pokemon
        for (idx, pokemon) in self.sides[side_idx].pokemon.iter().enumerate() {
            if idx != current_poke_idx && !pokemon.is_active && !pokemon.is_fainted() {
                return Some(idx);
            }
        }
        None
    }

    /// Execute a switch
    fn do_switch(&mut self, side_idx: usize, slot: usize, switch_to: usize) {
        if side_idx >= self.sides.len() {
            return;
        }

        // Check if switch_to Pokemon is valid
        if switch_to >= self.sides[side_idx].pokemon.len() {
            return;
        }
        if self.sides[side_idx].pokemon[switch_to].is_fainted() {
            return;
        }
        if self.sides[side_idx].pokemon[switch_to].is_active {
            return;
        }

        // JS: if (action.choice === 'switch' && action.pokemon.status) {
        //         this.singleEvent('CheckShow', this.dex.abilities.getByID('naturalcure'), null, action.pokemon);
        //     }
        // Check if the switching Pokemon has a status condition
        if let Some(active_idx) = self.sides[side_idx].active.get(slot).copied().flatten() {
            let has_status = !self.sides[side_idx].pokemon[active_idx].status.is_empty();
            if has_status {
                let naturalcure_id = ID::new("naturalcure");
                self.single_event("CheckShow", &naturalcure_id, Some((side_idx, active_idx)), None, None);
            }
        }

        // Get the old Pokemon's name for logging
        let _old_name = self.sides[side_idx].get_active(slot)
            .map(|p| p.name.clone())
            .unwrap_or_default();

        // Perform the switch
        self.sides[side_idx].switch_in(slot, switch_to);

        // Log the switch
        if let Some(pokemon) = self.sides[side_idx].get_active(slot) {
            let side_id = self.sides[side_idx].id_str();
            let details = pokemon.details();
            let hp = format!("{}/{}", pokemon.hp, pokemon.maxhp);
            self.log.push(format!("|switch|{}: {}|{}|{}", side_id, pokemon.name, details, hp));
        }

        // Apply entry hazard damage
        self.apply_hazards(side_idx, slot, switch_to);

        // Trigger switch-in abilities
        self.trigger_switch_in_abilities(side_idx, switch_to);
    }

    /// Drag in a random Pokemon (for moves like Whirlwind/Roar)
    /// Equivalent to battle-actions.ts dragIn()
    /// 1:1 port of dragIn from battle-actions.ts
    pub fn drag_in(&mut self, side_idx: usize, slot: usize) -> bool {
        // Get a random switchable Pokemon
        let switch_target = match self.get_random_switchable(side_idx) {
            Some(idx) => idx,
            None => return false,
        };

        // Check if there's an active Pokemon in that slot
        let old_active = self.sides[side_idx].active.get(slot).copied().flatten();
        if old_active.is_none() {
            return false;
        }

        // Check if the old Pokemon can be dragged out (not fainted)
        let old_poke_idx = old_active.unwrap();
        if self.sides[side_idx].pokemon[old_poke_idx].hp == 0 {
            return false;
        }

        // Run DragOut event (abilities/items that prevent being dragged out)
        if !self.run_event_bool("DragOut", Some((side_idx, old_poke_idx)), None, None) {
            return false;
        }

        // Call switchIn with is_drag = true
        match self.switch_in(side_idx, slot, switch_target, None, true) {
            SwitchResult::Success => true,
            _ => false,
        }
    }

    /// Execute a switch with optional drag flag
    /// Rust helper - breaks down switch logic for borrow checker
    /// JavaScript integrates this into switch/drag handling
    fn do_switch_with_drag(&mut self, side_idx: usize, slot: usize, switch_to: usize, is_drag: bool) {
        if side_idx >= self.sides.len() {
            return;
        }

        // Check if switch_to Pokemon is valid
        if switch_to >= self.sides[side_idx].pokemon.len() {
            return;
        }
        if self.sides[side_idx].pokemon[switch_to].is_fainted() {
            return;
        }
        if self.sides[side_idx].pokemon[switch_to].is_active {
            return;
        }

        // Perform the switch
        self.sides[side_idx].switch_in(slot, switch_to);

        // Log the switch/drag
        if let Some(pokemon) = self.sides[side_idx].get_active(slot) {
            let side_id = self.sides[side_idx].id_str();
            let details = pokemon.details();
            let hp = format!("{}/{}", pokemon.hp, pokemon.maxhp);
            let event = if is_drag { "drag" } else { "switch" };
            self.log.push(format!("|{}|{}: {}|{}|{}", event, side_id, pokemon.name, details, hp));
        }

        // Apply entry hazard damage
        self.apply_hazards(side_idx, slot, switch_to);

        // Trigger switch-in abilities
        self.trigger_switch_in_abilities(side_idx, switch_to);

        // In Gen 5+, run switch immediately for drags
        if is_drag && self.gen >= 5 {
            self.run_switch(side_idx, switch_to);
        }
    }

    /// Run switch-in effects for a Pokemon
    /// 1:1 port of battle-actions.ts runSwitch()
    pub fn run_switch(&mut self, side_idx: usize, poke_idx: usize) {
        // Collect all switchers - consume all consecutive runSwitch actions
        let mut switchers_in: Vec<(usize, usize)> = vec![(side_idx, poke_idx)];

        // Collect any additional runSwitch actions from the queue
        while let Some(action) = self.queue.peek() {
            if action.is_run_switch() {
                if let Some((s, p)) = action.get_switch_target() {
                    switchers_in.push((s, p));
                }
                self.queue.shift();
            } else {
                break;
            }
        }

        // Speed sort all active Pokemon
        self.update_speed();

        // Run SwitchIn field event for all switchers
        self.field_event_switch_in(&switchers_in);

        // Mark all switchers as started and clear draggedIn
        for (s_idx, p_idx) in &switchers_in {
            if self.sides[*s_idx].pokemon[*p_idx].hp == 0 {
                continue;
            }
            self.sides[*s_idx].pokemon[*p_idx].is_started = true;
            self.sides[*s_idx].pokemon[*p_idx].dragged_in = None;
        }
    }

    /// Run field event for switch-in (Intimidate, etc.)
    fn field_event_switch_in(&mut self, switchers: &[(usize, usize)]) {
        // Run SwitchIn event for each switcher
        for (s_idx, p_idx) in switchers {
            let effect_id = ID::new("switchin");
            self.single_event("SwitchIn", &effect_id, Some((*s_idx, *p_idx)), None, None);
        }
    }

    /// Trigger abilities that activate on switch-in
    fn trigger_switch_in_abilities(&mut self, side_idx: usize, poke_idx: usize) {
        // Use event system to trigger SwitchIn abilities
        // This will call handle_ability_event which handles Intimidate, Drizzle, etc.
        self.run_event("SwitchIn", Some((side_idx, poke_idx)), None, None, None);

        // TODO: Migrate all switch-in ability logic to ability_callbacks/
        // Old data-driven code removed - abilities should use event handlers instead
    }

    /// Apply entry hazard damage to a Pokemon that just switched in
    fn apply_hazards(&mut self, side_idx: usize, _slot: usize, poke_idx: usize) {
        // Get Pokemon data before hazard checks
        let (maxhp, pokemon_types, pokemon_name, is_grounded) = {
            let pokemon = &self.sides[side_idx].pokemon[poke_idx];
            let is_grounded = !pokemon.types.iter().any(|t| t.to_lowercase() == "flying");
            (pokemon.maxhp, pokemon.types.clone(), pokemon.name.clone(), is_grounded)
        };

        let side_id = self.sides[side_idx].id_str();
        let full_name = format!("{}: {}", side_id, pokemon_name);

        // Stealth Rock - affects all Pokemon, damage based on Rock type effectiveness
        let sr_id = ID::new("stealthrock");
        if self.sides[side_idx].has_side_condition(&sr_id) {
            let damage = Side::calc_stealth_rock_damage(&pokemon_types, maxhp);
            self.sides[side_idx].pokemon[poke_idx].take_damage(damage);

            let hp = self.sides[side_idx].pokemon[poke_idx].hp;
            self.add_log("-damage", &[&full_name, &format!("{}/{}", hp, maxhp), "[from] Stealth Rock"]);
        }

        // Spikes - only affects grounded Pokemon
        let spikes_id = ID::new("spikes");
        if is_grounded && self.sides[side_idx].has_side_condition(&spikes_id) {
            let layers = self.sides[side_idx].get_hazard_layers(&spikes_id);
            if layers > 0 {
                let damage = Side::calc_spikes_damage(layers, maxhp);
                self.sides[side_idx].pokemon[poke_idx].take_damage(damage);

                let hp = self.sides[side_idx].pokemon[poke_idx].hp;
                self.add_log("-damage", &[&full_name, &format!("{}/{}", hp, maxhp), "[from] Spikes"]);
            }
        }

        // Toxic Spikes - only affects grounded Pokemon, poisons them
        let tspikes_id = ID::new("toxicspikes");
        if is_grounded && self.sides[side_idx].has_side_condition(&tspikes_id) {
            let layers = self.sides[side_idx].get_hazard_layers(&tspikes_id);

            // Poison types absorb Toxic Spikes
            let is_poison = pokemon_types.iter().any(|t| t.to_lowercase() == "poison");
            if is_poison {
                // Poison type absorbs Toxic Spikes
                self.sides[side_idx].remove_side_condition(&tspikes_id);
                self.add_log("-sideend", &[side_id, "Toxic Spikes", "[from] ability: Poison Touch"]); // Generic message
            } else if !self.sides[side_idx].pokemon[poke_idx].status.is_empty() {
                // Already has a status - can't be poisoned
            } else {
                // Apply poison (1 layer) or toxic (2 layers)
                let status = if layers >= 2 { "tox" } else { "psn" };
                self.sides[side_idx].pokemon[poke_idx].set_status(ID::new(status));

                let status_msg = if layers >= 2 { "badly poisoned" } else { "poisoned" };
                self.add_log("-status", &[&full_name, status, &format!("[from] Toxic Spikes")]);
            }
        }

        // Sticky Web - lowers Speed by 1 stage (only affects grounded Pokemon)
        let sticky_web_id = ID::new("stickyweb");
        if is_grounded && self.sides[side_idx].has_side_condition(&sticky_web_id) {
            let current_spe = self.sides[side_idx].pokemon[poke_idx].boosts.spe;
            if current_spe > -6 {
                self.sides[side_idx].pokemon[poke_idx].boosts.spe = (current_spe - 1).max(-6);
                self.add_log("-boost", &[&full_name, "spe", "-1", "[from] Sticky Web"]);
            }
        }
    }

    /// Execute a move
    fn run_move(&mut self, side_idx: usize, poke_idx: usize, move_id: &ID, target_loc: i8) {
        if side_idx >= self.sides.len() {
            return;
        }

        // Check if Pokemon can still move
        let can_act = {
            let pokemon = &self.sides[side_idx].pokemon[poke_idx];
            !pokemon.is_fainted() && pokemon.is_active
        };

        if !can_act {
            return;
        }

        // Get target side and Pokemon
        let (target_side_idx, target_poke_idx) = self.get_move_target(side_idx, target_loc);

        // Log the move use
        let (attacker_name, move_name) = {
            let side_id = self.sides[side_idx].id_str();
            let pokemon = &self.sides[side_idx].pokemon[poke_idx];
            (format!("{}: {}", side_id, pokemon.name), move_id.as_str().to_string())
        };
        self.add_log("move", &[&attacker_name, &move_name]);

        // Deduct PP
        self.sides[side_idx].pokemon[poke_idx].deduct_pp(move_id, 1);

        // Execute the move using battle_actions module
        // This calls the line-by-line port of battle-actions.ts useMove()
        crate::battle_actions::use_move(
            self,
            move_id,
            (side_idx, poke_idx),
            Some((target_side_idx, target_poke_idx)),
            None, // source_effect
            None, // z_move
            None, // max_move
        );
    }

    /// Get the target for a move based on target_loc
    fn get_move_target(&self, side_idx: usize, target_loc: i8) -> (usize, usize) {
        // In singles, target_loc is typically 0 (auto-target foe) or specific
        // Positive = foe position, Negative = ally position
        let foe_idx = if side_idx == 0 { 1 } else { 0 };

        if target_loc <= 0 {
            // Default to first active foe
            let target_poke_idx = self.sides.get(foe_idx)
                .and_then(|s| s.active.get(0))
                .and_then(|opt| *opt)
                .unwrap_or(0);
            (foe_idx, target_poke_idx)
        } else {
            // Specific target position
            let slot = (target_loc.abs() - 1) as usize;
            let target_poke_idx = self.sides.get(foe_idx)
                .and_then(|s| s.active.get(slot))
                .and_then(|opt| *opt)
                .unwrap_or(0);
            (foe_idx, target_poke_idx)
        }
    }

    /// Get move priority (-7 to +5)
    fn get_move_priority(&self, move_id: &ID) -> i8 {
        // Use move data from MoveDef
        if let Some(move_def) = self.dex.get_move(move_id.as_str()) {
            move_def.priority
        } else {
            0 // Default priority for unknown moves
        }
    }

    /// Get number of hits for multi-hit moves
    fn get_multi_hit_count(&mut self, move_id: &ID) -> i32 {
        // Extract multihit data before calling mutable method
        let multihit_data = if let Some(move_def) = self.dex.get_move(move_id.as_str()) {
            move_def.multihit.clone()
        } else {
            None
        };

        // Process multihit data
        if let Some(multihit) = multihit_data {
            match multihit {
                crate::dex::Multihit::Fixed(n) => return n,
                crate::dex::Multihit::Range(min, max) => {
                    // Variable hit move - distribute as: 35% min, 35% min+1, 15% max-1, 15% max
                    let roll = self.random(100);
                    return if roll < 35 {
                        min
                    } else if roll < 70 {
                        (min + 1).min(max)
                    } else if roll < 85 {
                        (max - 1).max(min)
                    } else {
                        max
                    };
                }
            }
        }

        1 // Default: single hit
    }

    /// Get move accuracy (0-100, where 100+ means never miss)
    fn get_move_accuracy(&self, move_id: &ID) -> i32 {
        // Use move data from MoveData
        if let Some(move_def) = self.dex.get_move(move_id.as_str()) {
            match move_def.accuracy {
                crate::dex::Accuracy::Percent(p) => p,
                crate::dex::Accuracy::AlwaysHits => 101, // > 100 means always hits
            }
        } else {
            100 // Default accuracy for unknown moves
        }
    }

    /// Apply confusion volatile to a Pokemon
    fn apply_confusion(&mut self, side_idx: usize, poke_idx: usize) {
        if side_idx >= self.sides.len() || poke_idx >= self.sides[side_idx].pokemon.len() {
            return;
        }

        let confusion_id = ID::new("confusion");
        if self.sides[side_idx].pokemon[poke_idx].has_volatile(&confusion_id) {
            return; // Already confused
        }

        // Confusion lasts 2-5 turns
        let duration = 2 + self.random(4);
        self.sides[side_idx].pokemon[poke_idx].add_volatile(confusion_id.clone());
        if let Some(state) = self.sides[side_idx].pokemon[poke_idx].get_volatile_mut(&confusion_id) {
            state.duration = Some(duration);
        }

        let name = {
            let side_id = self.sides[side_idx].id_str();
            let pokemon = &self.sides[side_idx].pokemon[poke_idx];
            format!("{}: {}", side_id, pokemon.name)
        };
        self.add_log("-start", &[&name, "confusion"]);
    }

    /// Remove all entry hazards from a side
    fn remove_all_hazards(&mut self, side_idx: usize) {
        if side_idx >= self.sides.len() {
            return;
        }

        let side_id = self.sides[side_idx].id_str();
        let hazards = ["stealthrock", "spikes", "toxicspikes", "stickyweb"];

        for hazard_name in hazards {
            let hazard_id = ID::new(hazard_name);
            if self.sides[side_idx].remove_side_condition(&hazard_id) {
                let display_name = match hazard_name {
                    "stealthrock" => "Stealth Rock",
                    "spikes" => "Spikes",
                    "toxicspikes" => "Toxic Spikes",
                    "stickyweb" => "Sticky Web",
                    _ => hazard_name,
                };
                self.add_log("-sideend", &[side_id, display_name]);
            }
        }
    }

    /// Apply a status condition
    fn apply_status(&mut self, side_idx: usize, poke_idx: usize, status: &str) {
        if side_idx >= self.sides.len() || poke_idx >= self.sides[side_idx].pokemon.len() {
            return;
        }

        // First check if status can be applied
        {
            let pokemon = &self.sides[side_idx].pokemon[poke_idx];

            // Check if already has status
            if !pokemon.status.is_empty() {
                return;
            }

            // Check type immunities
            let has_type = |t: &str| pokemon.types.iter().any(|pt| pt.to_lowercase() == t.to_lowercase());

            match status {
                "brn" if has_type("fire") => return,
                "par" if has_type("electric") && self.gen >= 6 => return,
                "psn" | "tox" if has_type("poison") || has_type("steel") => return,
                "frz" if has_type("ice") => return,
                _ => {}
            }
        }

        // Get random duration for sleep before mutating
        let sleep_duration = if status == "slp" {
            Some(1 + self.random(3))
        } else {
            None
        };

        // Now apply the status
        let pokemon = &mut self.sides[side_idx].pokemon[poke_idx];
        pokemon.set_status(ID::new(status));

        // Set duration for sleep/toxic
        match status {
            "slp" => {
                pokemon.status_state.duration = sleep_duration;
            }
            "tox" => {
                pokemon.status_state.duration = Some(1); // Toxic counter starts at 1
            }
            _ => {}
        }

        // Get name for logging
        let name = pokemon.name.clone();
        let side_id = self.sides[side_idx].id_str();
        let full_name = format!("{}: {}", side_id, name);
        self.add_log("-status", &[&full_name, status]);
    }

    /// Cure status condition of a pokemon
    /// Matches JavaScript pokemon.ts cureStatus(silent = false)
    ///
    pub fn cure_status(&mut self, target: (usize, usize)) -> bool {
        let (side_idx, poke_idx) = target;

        // JS: if (!this.hp || !this.status) return false;
        let (has_hp, has_status, status, name) = if let Some(side) = self.sides.get(side_idx) {
            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                (pokemon.hp > 0, !pokemon.status.is_empty(), pokemon.status.as_str().to_string(), pokemon.name.clone())
            } else {
                return false;
            }
        } else {
            return false;
        };

        if !has_hp || !has_status {
            return false;
        }

        // JS: this.battle.add('-curestatus', this, this.status, silent ? '[silent]' : '[msg]');
        let side_id = self.sides[side_idx].id_str();
        let full_name = format!("{}: {}", side_id, name);
        self.add_log("-curestatus", &[&full_name, &status, "[msg]"]);

        // JS: if (this.status === 'slp' && this.removeVolatile('nightmare')) { ... }
        if status == "slp" {
            let removed_nightmare = if let Some(side) = self.sides.get_mut(side_idx) {
                if let Some(pokemon) = side.pokemon.get_mut(poke_idx) {
                    pokemon.volatiles.remove(&ID::new("nightmare")).is_some()
                } else {
                    false
                }
            } else {
                false
            };

            if removed_nightmare {
                self.add_log("-end", &[&full_name, "Nightmare", "[silent]"]);
            }
        }

        // JS: this.setStatus('');
        if let Some(side) = self.sides.get_mut(side_idx) {
            if let Some(pokemon) = side.pokemon.get_mut(poke_idx) {
                pokemon.set_status(ID::new(""));
                pokemon.status_state.duration = None;
            }
        }

        // JS: return true;
        true
    }

    /// Apply a stat boost
    fn apply_boost(&mut self, side_idx: usize, poke_idx: usize, stat: &str, amount: i8) {
        if side_idx >= self.sides.len() || poke_idx >= self.sides[side_idx].pokemon.len() {
            return;
        }

        let (name, actual_change) = {
            let pokemon = &mut self.sides[side_idx].pokemon[poke_idx];
            let old_boost = match stat {
                "atk" => pokemon.boosts.atk,
                "def" => pokemon.boosts.def,
                "spa" => pokemon.boosts.spa,
                "spd" => pokemon.boosts.spd,
                "spe" => pokemon.boosts.spe,
                _ => return,
            };

            let new_boost = (old_boost + amount).clamp(-6, 6);
            let actual_change = new_boost - old_boost;

            if actual_change == 0 {
                return;
            }

            match stat {
                "atk" => pokemon.boosts.atk = new_boost,
                "def" => pokemon.boosts.def = new_boost,
                "spa" => pokemon.boosts.spa = new_boost,
                "spd" => pokemon.boosts.spd = new_boost,
                "spe" => pokemon.boosts.spe = new_boost,
                _ => return,
            }

            (pokemon.name.clone(), actual_change)
        };

        let side_id = self.sides[side_idx].id_str();
        let full_name = format!("{}: {}", side_id, name);
        self.add_log("-boost", &[&full_name, stat, &actual_change.to_string()]);
    }

    /// Process end-of-turn residual effects
    /// Equivalent to battle.ts case 'residual' (battle.ts:2810-2817)
    fn run_residual(&mut self) {
        // this.add('');
        self.add_log("", &[]);

        // this.clearActiveMove(true);
        self.clear_active_move(true);

        // this.updateSpeed();
        self.update_speed();

        // residualPokemon = this.getAllActive().map(pokemon => [pokemon, pokemon.getUndynamaxedHP()] as const);
        // Note: We don't track residualPokemon yet for EmergencyExit handling
        // This will be needed when implementing EmergencyExit abilities

        // this.fieldEvent('Residual');
        self.field_event("Residual");

        // if (!this.ended) this.add('upkeep');
        if !self.ended {
            self.add_log("upkeep", &[]);
        }
    }

    /// Process faint messages
    /// Equivalent to battle.ts faintMessages(lastFirst?, forceCheck?, checkWin?)
    ///
    // 
    // 	faintMessages(lastFirst = false, forceCheck = false, checkWin = true) {
    // 		if (this.ended) return;
    // 		const length = this.faintQueue.length;
    // 		if (!length) {
    // 			if (forceCheck && this.checkWin()) return true;
    // 			return false;
    // 		}
    // 		if (lastFirst) {
    // 			this.faintQueue.unshift(this.faintQueue[this.faintQueue.length - 1]);
    // 			this.faintQueue.pop();
    // 		}
    // 		let faintQueueLeft, faintData;
    // 		while (this.faintQueue.length) {
    // 			faintQueueLeft = this.faintQueue.length;
    // 			faintData = this.faintQueue.shift()!;
    // 			const pokemon: Pokemon = faintData.target;
    // 			if (!pokemon.fainted && this.runEvent('BeforeFaint', pokemon, faintData.source, faintData.effect)) {
    // 				this.add('faint', pokemon);
    // 				if (pokemon.side.pokemonLeft) pokemon.side.pokemonLeft--;
    // 				if (pokemon.side.totalFainted < 100) pokemon.side.totalFainted++;
    // 				this.runEvent('Faint', pokemon, faintData.source, faintData.effect);
    // 				this.singleEvent('End', pokemon.getAbility(), pokemon.abilityState, pokemon);
    // 				this.singleEvent('End', pokemon.getItem(), pokemon.itemState, pokemon);
    // 				if (pokemon.formeRegression && !pokemon.transformed) {
    // 					// before clearing volatiles
    // 					pokemon.baseSpecies = this.dex.species.get(pokemon.set.species || pokemon.set.name);
    // 					pokemon.baseAbility = toID(pokemon.set.ability);
    // 				}
    // 				pokemon.clearVolatile(false);
    // 				pokemon.fainted = true;
    // 				pokemon.illusion = null;
    // 				pokemon.isActive = false;
    // 				pokemon.isStarted = false;
    // 				delete pokemon.terastallized;
    // 				if (pokemon.formeRegression) {
    // 					// after clearing volatiles
    // 					pokemon.details = pokemon.getUpdatedDetails();
    // 					this.add('detailschange', pokemon, pokemon.details, '[silent]');
    // 					pokemon.updateMaxHp();
    // 					pokemon.formeRegression = false;
    // 				}
    // 				pokemon.side.faintedThisTurn = pokemon;
    // 				if (this.faintQueue.length >= faintQueueLeft) checkWin = true;
    // 			}
    // 		}
    // 
    // 		if (this.gen <= 1) {
    // 			// in gen 1, fainting skips the rest of the turn
    // 			// residuals don't exist in gen 1
    // 			this.queue.clear();
    // 			// Fainting clears accumulated Bide damage
    // 			for (const pokemon of this.getAllActive()) {
    // 				if (pokemon.volatiles['bide']?.damage) {
    // 					pokemon.volatiles['bide'].damage = 0;
    // 					this.hint("Desync Clause Mod activated!");
    // 					this.hint("In Gen 1, Bide's accumulated damage is reset to 0 when a Pokemon faints.");
    // 				}
    // 			}
    // 		} else if (this.gen <= 3 && this.gameType === 'singles') {
    // 			// in gen 3 or earlier, fainting in singles skips to residuals
    // 			for (const pokemon of this.getAllActive()) {
    // 				if (this.gen <= 2) {
    // 					// in gen 2, fainting skips moves only
    // 					this.queue.cancelMove(pokemon);
    // 				} else {
    // 					// in gen 3, fainting skips all moves and switches
    // 					this.queue.cancelAction(pokemon);
    // 				}
    // 			}
    // 		}
    // 
    // 		if (checkWin && this.checkWin(faintData)) return true;
    // 
    // 		if (faintData && length) {
    // 			this.runEvent('AfterFaint', faintData.target, faintData.source, faintData.effect, length);
    // 		}
    // 		return false;
    // 	}
    //
    fn faint_messages(&mut self, last_first: bool, force_check: bool, mut check_win: bool) -> bool {
        // JS: if (this.ended) return;
        if self.ended {
            return false;
        }

        // For compatibility: Add any Pokemon with HP=0 to faint_queue if they're not already there
        // This handles cases where damage was applied without calling Battle.faint()
        for side_idx in 0..self.sides.len() {
            for poke_idx in 0..self.sides[side_idx].pokemon.len() {
                if self.sides[side_idx].pokemon[poke_idx].hp == 0 &&
                   !self.sides[side_idx].pokemon[poke_idx].fainted {
                    // Check if already in queue
                    let already_queued = self.faint_queue.iter().any(|fd| fd.target == (side_idx, poke_idx));
                    if !already_queued {
                        self.faint_queue.push(FaintData {
                            target: (side_idx, poke_idx),
                            source: None,
                            effect: None,
                        });
                    }
                }
            }
        }

        // JS: const length = this.faintQueue.length;
        let length = self.faint_queue.len();

        // JS: if (!length) { if (forceCheck && this.checkWin()) return true; return false; }
        if length == 0 {
            if force_check && self.check_win(None) {
                return true;
            }
            return false;
        }

        // JS: if (lastFirst) { this.faintQueue.unshift(this.faintQueue[this.faintQueue.length - 1]); this.faintQueue.pop(); }
        if last_first && !self.faint_queue.is_empty() {
            let last = self.faint_queue.pop().unwrap();
            self.faint_queue.insert(0, last);
        }

        let mut last_faint_data: Option<FaintData> = None;

        // JS: while (this.faintQueue.length)
        while !self.faint_queue.is_empty() {
            let faint_queue_left = self.faint_queue.len();
            let faint_data = self.faint_queue.remove(0); // JS: faintData = this.faintQueue.shift()!;
            let (side_idx, poke_idx) = faint_data.target;

            // Check if pokemon is already fainted
            let already_fainted = self.sides[side_idx].pokemon[poke_idx].fainted;

            // JS: if (!pokemon.fainted && this.runEvent('BeforeFaint', pokemon, faintData.source, faintData.effect))
            if !already_fainted {
                // Run BeforeFaint event - can be cancelled by returning false
                let before_faint_result = self.run_event_bool(
                    "BeforeFaint",
                    Some((side_idx, poke_idx)),
                    faint_data.source,
                    faint_data.effect.as_ref(),
                );

                if !before_faint_result {
                    // BeforeFaint was cancelled, skip this faint
                    continue;
                }

                // JS: this.add('faint', pokemon);
                let pokemon_name = {
                    let side_id = self.sides[side_idx].id_str();
                    let pokemon = &self.sides[side_idx].pokemon[poke_idx];
                    format!("{}: {}", side_id, pokemon.name)
                };
                self.add_log("faint", &[&pokemon_name]);

                // JS: if (pokemon.side.pokemonLeft) pokemon.side.pokemonLeft--;
                if self.sides[side_idx].pokemon_left > 0 {
                    self.sides[side_idx].pokemon_left -= 1;
                }

                // JS: if (pokemon.side.totalFainted < 100) pokemon.side.totalFainted++;
                if self.sides[side_idx].total_fainted < 100 {
                    self.sides[side_idx].total_fainted += 1;
                }

                // JS: this.runEvent('Faint', pokemon, faintData.source, faintData.effect);
                self.run_event(
                    "Faint",
                    Some((side_idx, poke_idx)),
                    faint_data.source,
                    faint_data.effect.as_ref(),
                    None,  // relay_var
                );

                // JS: this.singleEvent('End', pokemon.getAbility(), pokemon.abilityState, pokemon);
                // JS: this.singleEvent('End', pokemon.getItem(), pokemon.itemState, pokemon);
                // Get ability and item IDs before they're cleared
                let ability_id = self.sides[side_idx].pokemon[poke_idx].ability.clone();
                let item_id = self.sides[side_idx].pokemon[poke_idx].item.clone();

                // Call End event for ability
                if !ability_id.is_empty() {
                    self.single_event("End", &ability_id, Some((side_idx, poke_idx)), None, None);
                }

                // Call End event for item
                if !item_id.is_empty() {
                    self.single_event("End", &item_id, Some((side_idx, poke_idx)), None, None);
                }

                // JS: pokemon.clearVolatile(false);
                self.sides[side_idx].pokemon[poke_idx].clear_volatile_full(false);

                // JS: pokemon.fainted = true;
                self.sides[side_idx].pokemon[poke_idx].fainted = true;

                // JS: pokemon.illusion = null;
                self.sides[side_idx].pokemon[poke_idx].illusion = None;

                // JS: pokemon.isActive = false;
                self.sides[side_idx].pokemon[poke_idx].is_active = false;

                // Note: In Pokemon Showdown, fainted Pokemon remain in the active array
                // until they are explicitly replaced by a switch. They are not removed immediately.
                // This allows the battle to track which slots need replacement.
                // The old code removed them here, but that's incorrect:
                // // Remove from active slots
                // for slot in 0..self.sides[side_idx].active.len() {
                //     if let Some(idx) = self.sides[side_idx].active[slot] {
                //         if idx == poke_idx {
                //             self.sides[side_idx].active[slot] = None;
                //             break;
                //         }
                //     }
                // }

                // JS: pokemon.isStarted = false;
                self.sides[side_idx].pokemon[poke_idx].is_started = false;

                // JS: delete pokemon.terastallized;
                self.sides[side_idx].pokemon[poke_idx].terastallized = None;

                // JS: if (this.faintQueue.length >= faintQueueLeft) checkWin = true;
                if self.faint_queue.len() >= faint_queue_left {
                    check_win = true;
                }
            }

            last_faint_data = Some(faint_data);
        }

        // JS: if (this.gen <= 1) { this.queue.clear(); ... }
        if self.gen <= 1 {
            // Gen 1: fainting skips the rest of the turn
            // JS: this.queue.clear();
            self.queue.clear();

            // JS: Fainting clears accumulated Bide damage
            // JS: for (const pokemon of this.getAllActive()) {
            // JS:     if (pokemon.volatiles['bide']?.damage) {
            // JS:         pokemon.volatiles['bide'].damage = 0;
            // JS:         this.hint("Desync Clause Mod activated!");
            // JS:         this.hint("In Gen 1, Bide's accumulated damage is reset to 0 when a Pokemon faints.");
            // JS:     }
            // JS: }
            let mut bide_cleared = false;
            let bide_id = ID::new("bide");

            for side in &mut self.sides {
                for pokemon in &mut side.pokemon {
                    if !pokemon.is_active {
                        continue;
                    }

                    // Check if pokemon has bide volatile with damage > 0
                    if let Some(bide_state) = pokemon.volatiles.get_mut(&bide_id) {
                        // Check if the bide state has a damage field
                        if let Some(damage_value) = bide_state.data.get("damage") {
                            if let Some(damage) = damage_value.as_i64() {
                                if damage > 0 {
                                    // Reset damage to 0
                                    bide_state.data.insert("damage".to_string(), serde_json::json!(0));
                                    bide_cleared = true;
                                }
                            }
                        }
                    }
                }
            }

            if bide_cleared {
                self.hint("Desync Clause Mod activated!", false, None);
                self.hint("In Gen 1, Bide's accumulated damage is reset to 0 when a Pokemon faints.", false, None);
            }
        }

        // JS: else if (this.gen <= 3 && this.gameType === 'singles') { ... }
        else if self.gen <= 3 && self.game_type == GameType::Singles {
            // in gen 3 or earlier, fainting in singles skips to residuals
            // JS: for (const pokemon of this.getAllActive()) { ... }

            // Collect active pokemon positions to avoid borrow checker issues
            let active_positions: Vec<(usize, usize)> = self.sides.iter().enumerate()
                .flat_map(|(side_idx, side)| {
                    side.active.iter()
                        .filter_map(move |&opt_idx| {
                            opt_idx.map(|poke_idx| (side_idx, poke_idx))
                        })
                })
                .collect();

            for (side_idx, pokemon_idx) in active_positions {
                if self.gen <= 2 {
                    // in gen 2, fainting skips moves only
                    // JS: this.queue.cancelMove(pokemon);
                    self.queue.cancel_move(side_idx, pokemon_idx);
                } else {
                    // in gen 3, fainting skips all moves and switches
                    // JS: this.queue.cancelAction(pokemon);
                    self.queue.cancel_action(side_idx, pokemon_idx);
                }
            }
        }

        // JS: if (checkWin && this.checkWin(faintData)) return true;
        if check_win && self.check_win(last_faint_data.clone()) {
            return true;
        }

        // JS: if (faintData && length) { this.runEvent('AfterFaint', faintData.target, faintData.source, faintData.effect, length); }
        if let Some(ref faint_data) = last_faint_data {
            if length > 0 {
                self.run_event(
                    "AfterFaint",
                    Some(faint_data.target),
                    faint_data.source,
                    faint_data.effect.as_ref(),
                    None,  // relay_var
                );
            }
        }

        // JS: return false;
        false
    }

    /// Start the next turn
    fn next_turn(&mut self) {
        // Clear turn state
        for side in &mut self.sides {
            side.clear_turn_state();
        }

        self.turn += 1;
        self.add_log("turn", &[&self.turn.to_string()]);

        // Check if any side has fainted active Pokemon that need to be replaced
        let mut needs_switch = false;
        for side in &self.sides {
            // First check if this side has any available Pokemon to switch in
            let has_available_pokemon = side.pokemon.iter().any(|p| !p.is_fainted() && !p.is_active);

            if has_available_pokemon {
                // Only require switches if there are Pokemon available to switch in
                for &active_idx in &side.active {
                    // Check if slot is empty (Pokemon fainted) OR if the Pokemon in slot is fainted
                    if active_idx.is_none() {
                        needs_switch = true;
                        break;
                    } else if let Some(poke_idx) = active_idx {
                        if side.pokemon.get(poke_idx).map(|p| p.is_fainted()).unwrap_or(false) {
                            needs_switch = true;
                            break;
                        }
                    }
                }
            }
            if needs_switch {
                break;
            }
        }

        // Set up new request
        if needs_switch {
            // If there are fainted active Pokemon, request switches
            self.request_state = BattleRequestState::Switch;
            for side in &mut self.sides {
                side.request_state = RequestState::Switch;
            }
        } else {
            // Otherwise, request moves as normal
            self.request_state = BattleRequestState::Move;
            for side in &mut self.sides {
                side.request_state = RequestState::Move;
            }
        }
    }

    // =========================================================================
    // Methods ported from battle.ts
    // =========================================================================

    /// Log a debug message if debug mode is enabled
    /// Equivalent to battle.ts debug()
    //
    // 	debug(activity: string) {
    // 		if (this.debugMode) {
    // 			this.add('debug', activity);
    // 		}
    // 	}
    //
    pub fn debug(&mut self, activity: &str) {
        if self.debug_mode {
            self.add_log("debug", &[activity]);
        }
    }

    /// Get the debug log (all lines)
    // 
    // 	getDebugLog() {
    // 		const channelMessages = extractChannelMessages(this.log.join('\n'), [-1]);
    // 		return channelMessages[-1].join('\n');
    // 	}
    //
    pub fn get_debug_log(&self) -> String {
        self.log.join("\n")
    }

    /// Add a log entry - matches JS this.add()
    /// JavaScript: add(...parts: (Part | (() => { side: SideID, secret: string, shared: string }))[])
    /// Usage: battle.add("-activate", &[pokemon.into(), "ability: Immunity".into()])
    // 
    // 	add(...parts: (Part | (() => { side: SideID, secret: string, shared: string }))[]) {
    // 		if (!parts.some(part => typeof part === 'function')) {
    // 			this.log.push(`|${parts.join('|')}`);
    // 			return;
    // 		}
    // 
    // 		let side: SideID | null = null;
    // 		const secret = [];
    // 		const shared = [];
    // 		for (const part of parts) {
    // 			if (typeof part === 'function') {
    // 				const split = part();
    // 				if (side && side !== split.side) throw new Error("Multiple sides passed to add");
    // 				side = split.side;
    // 				secret.push(split.secret);
    // 				shared.push(split.shared);
    // 			} else {
    // 				secret.push(part);
    // 				shared.push(part);
    // 			}
    // 		}
    // 		this.addSplit(side!, secret, shared);
    // 	}
    //
    pub fn add(&mut self, event_type: &str, args: &[Arg]) {
        // JS: if (!parts.some(part => typeof part === 'function'))
        let has_split_fn = args.iter().any(|arg| matches!(arg, Arg::SplitFn(_)));

        if !has_split_fn {
            // JS: this.log.push(`|${parts.join('|')}`);
            let mut entry = format!("|{}", event_type);
            for arg in args {
                entry.push('|');
                entry.push_str(&arg.to_string());
            }
            self.log.push(entry);
            return;
        }

        // JS: let side: SideID | null = null;
        let mut side: Option<SideID> = None;
        let mut secret = Vec::new();
        let mut shared = Vec::new();

        // Add event_type to both secret and shared
        secret.push(event_type.to_string());
        shared.push(event_type.to_string());

        // JS: for (const part of parts)
        for arg in args {
            match arg {
                Arg::SplitFn(func) => {
                    // JS: if (typeof part === 'function')
                    let split = func();

                    // JS: if (side && side !== split.side) throw new Error("Multiple sides passed to add");
                    if let Some(existing_side) = side {
                        if existing_side != split.side {
                            panic!("Multiple sides passed to add");
                        }
                    }

                    // JS: side = split.side;
                    side = Some(split.side);

                    // JS: secret.push(split.secret); shared.push(split.shared);
                    secret.push(split.secret);
                    shared.push(split.shared);
                }
                _ => {
                    // JS: secret.push(part); shared.push(part);
                    let s = arg.to_string();
                    secret.push(s.clone());
                    shared.push(s);
                }
            }
        }

        // JS: this.addSplit(side!, secret, shared);
        if let Some(side_id) = side {
            let side_str = match side_id {
                SideID::P1 => "p1",
                SideID::P2 => "p2",
                SideID::P3 => "p3",
                SideID::P4 => "p4",
            };
            self.add_split(side_str, &secret.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
                          Some(&shared.iter().map(|s| s.as_str()).collect::<Vec<_>>()));
        }
    }

    /// Add a move log entry and track the last move line
    // 
    // 	addMove(...args: (string | number | Function | AnyObject)[]) {
    // 		this.lastMoveLine = this.log.length;
    // 		// eslint-disable-next-line @typescript-eslint/no-base-to-string
    // 		this.log.push(`|${args.join('|')}`);
    // 	}
    //
    pub fn add_move(&mut self, parts: &[&str]) {
        self.last_move_line = self.log.len() as i32;
        let entry = format!("|{}", parts.join("|"));
        self.log.push(entry);
    }

    /// Show a hint to the player
    /// Equivalent to battle.ts hint(hint, once?, side?)
    ///
    // 
    // 	hint(hint: string, once?: boolean, side?: Side) {
    // 		if (this.hints.has(side ? `${side.id}|${hint}` : hint)) return;
    // 
    // 		if (side) {
    // 			this.addSplit(side.id, ['-hint', hint]);
    // 		} else {
    // 			this.add('-hint', hint);
    // 		}
    // 
    // 		if (once) this.hints.add(side ? `${side.id}|${hint}` : hint);
    // 	}
    //
    pub fn hint(&mut self, hint_text: &str, once: bool, side_id: Option<SideID>) {
        // JS: if (this.hints.has(side ? `${side.id}|${hint}` : hint)) return;
        let hint_key = if let Some(sid) = side_id {
            format!("{}|{}", sid.to_str(), hint_text)
        } else {
            hint_text.to_string()
        };

        if self.hints.contains(&hint_key) {
            return;
        }

        // JS: if (side) { this.addSplit(side.id, ['-hint', hint]); } else { this.add('-hint', hint); }
        if let Some(sid) = side_id {
            self.add_split(sid.to_str(), &["-hint", hint_text], None);
        } else {
            self.add("-hint", &[Arg::Str(hint_text)]);
        }

        // JS: if (once) this.hints.add(side ? `${side.id}|${hint}` : hint);
        if once {
            self.hints.insert(hint_key);
        }
    }

    /// Truncate a number to an integer (equivalent to Math.trunc or this.trunc in JS)
    /// Note: JavaScript's dex.trunc() uses bitwise operations for 32-bit integer conversion
    /// Dex.trunc implementation from dex.ts:
    //
    // 	trunc(this: void, num: number, bits = 0) {
    // 		if (bits) return (num >>> 0) % (2 ** bits);
    // 		return num >>> 0;
    // 	}
    //
    /// Battle assigns: this.trunc = this.dex.trunc; (battle.ts:201)
    /// Rust uses num.trunc() as i32 which is functionally equivalent for positive values
    #[inline]
    pub fn trunc(&self, num: f64) -> i32 {
        num.trunc() as i32
    }

    /// Chain two modifiers together
    /// Equivalent to battle.ts chain(previousMod, nextMod)
    ///
    // 
    // 	chain(previousMod: number | number[], nextMod: number | number[]) {
    // 		// previousMod or nextMod can be either a number or an array [numerator, denominator]
    // 		if (Array.isArray(previousMod)) {
    // 			previousMod = this.trunc(previousMod[0] * 4096 / previousMod[1]);
    // 		} else {
    // 			previousMod = this.trunc(previousMod * 4096);
    // 		}
    // 
    // 		if (Array.isArray(nextMod)) {
    // 			nextMod = this.trunc(nextMod[0] * 4096 / nextMod[1]);
    // 		} else {
    // 			nextMod = this.trunc(nextMod * 4096);
    // 		}
    // 		return ((previousMod * nextMod + 2048) >> 12) / 4096; // M'' = ((M * M') + 0x800) >> 12
    // 	}
    //
    pub fn chain(&self, previous_mod: (i32, i32), next_mod: (i32, i32)) -> f64 {
        // JS: previousMod = this.trunc(previousMod[0] * 4096 / previousMod[1]);
        let prev = self.trunc((previous_mod.0 * 4096) as f64 / previous_mod.1 as f64);
        // JS: nextMod = this.trunc(nextMod[0] * 4096 / nextMod[1]);
        let next = self.trunc((next_mod.0 * 4096) as f64 / next_mod.1 as f64);
        // JS: return ((previousMod * nextMod + 2048) >> 12) / 4096;
        let result = ((prev * next) + 2048) >> 12;
        result as f64 / 4096.0
    }

    /// Chain two modifiers together (number variant)
    /// Rust convenience method - JavaScript chain() accepts both numbers and arrays via dynamic typing
    /// When both modifiers are simple multipliers (not fractions), this avoids tuple construction
    pub fn chain_f(&self, previous_mod: f64, next_mod: f64) -> f64 {
        // JS: previousMod = this.trunc(previousMod * 4096);
        let prev = self.trunc(previous_mod * 4096.0);
        // JS: nextMod = this.trunc(nextMod * 4096);
        let next = self.trunc(next_mod * 4096.0);
        // JS: return ((previousMod * nextMod + 2048) >> 12) / 4096;
        let result = ((prev * next) + 2048) >> 12;
        result as f64 / 4096.0
    }

    /// Apply a modifier to a value
    /// Equivalent to battle.ts modify(value, numerator, denominator)
    ///
    // 
    // 	modify(value: number, numerator: number | number[], denominator = 1) {
    // 		// You can also use:
    // 		// modify(value, [numerator, denominator])
    // 		// modify(value, fraction) - assuming you trust JavaScript's floating-point handler
    // 		if (Array.isArray(numerator)) {
    // 			denominator = numerator[1];
    // 			numerator = numerator[0];
    // 		}
    // 		const tr = this.trunc;
    // 		const modifier = tr(numerator * 4096 / denominator);
    // 		return tr((tr(value * modifier) + 2048 - 1) / 4096);
    // 	}
    //
    pub fn modify(&self, value: i32, numerator: i32, denominator: i32) -> i32 {
        // JS: const modifier = tr(numerator * 4096 / denominator);
        let modifier = self.trunc((numerator * 4096) as f64 / denominator as f64);
        // JS: return tr((tr(value * modifier) + 2048 - 1) / 4096);
        let inner = self.trunc((value * modifier) as f64);
        self.trunc((inner + 2048 - 1) as f64 / 4096.0)
    }

    /// Apply a modifier to a value using a tuple for numerator/denominator
    /// Rust convenience method - JavaScript modify() accepts arrays: modify(value, [numerator, denominator])
    /// This provides a type-safe alternative using Rust tuples
    pub fn modify_tuple(&self, value: i32, fraction: (i32, i32)) -> i32 {
        self.modify(value, fraction.0, fraction.1)
    }

    /// Apply a floating-point modifier to a value
    /// Rust convenience method - JavaScript modify() accepts floats directly: modify(value, 1.5)
    /// This provides type-safe handling when the modifier is already a float
    pub fn modify_f(&self, value: i32, multiplier: f64) -> i32 {
        // JS: const modifier = tr(numerator * 4096 / denominator);
        let modifier = self.trunc(multiplier * 4096.0);
        // JS: return tr((tr(value * modifier) + 2048 - 1) / 4096);
        let inner = self.trunc((value * modifier) as f64);
        self.trunc((inner + 2048 - 1) as f64 / 4096.0)
    }

    /// Clamp an integer to a minimum value
    /// Equivalent to battle.ts clampIntRange(value, min, max)
    ///
    // TypeScript source:
    // 	clampIntRange(num: any, min?: number, max?: number) {
    // 		if (typeof num !== 'number') return 0;
    // 		num = Math.floor(num);
    // 		if (min !== undefined) num = Math.max(num, min);
    // 		if (max !== undefined) num = Math.min(num, max);
    // 		return num;
    // 	}
    //
    pub fn clamp_int_range(&self, value: i32, min: Option<i32>, max: Option<i32>) -> i32 {
        let mut result = value;
        if let Some(min_val) = min {
            result = result.max(min_val);
        }
        if let Some(max_val) = max {
            result = result.min(max_val);
        }
        result
    }


    /// Get the current event's modifier (4096 = 1.0x)
    /// Rust accessor method - JavaScript directly accesses this.event.modifier field
    /// Used in event handlers to get the current relay variable modifier
    pub fn event_modifier(&self) -> i32 {
        self.current_event.as_ref().map(|e| e.modifier).unwrap_or(4096)
    }

    /// Declare a tie
    /// Equivalent to battle.ts tie()
    // 
    // 	tie() {
    // 		return this.win();
    // 	}
    //
    pub fn tie(&mut self) -> bool {
        self.win(None)
    }

    /// Declare a winner
    /// Equivalent to battle.ts win() (battle.ts:1474-1497)
    /// Pass None for a tie, or Some(side_id) for a winner
    // 
    // 	win(side?: SideID | '' | Side | null) {
    // 		if (this.ended) return false;
    // 		if (side && typeof side === 'string') {
    // 			side = this.getSide(side);
    // 		} else if (!side || !this.sides.includes(side)) {
    // 			side = null;
    // 		}
    // 		this.winner = side ? side.name : '';
    // 
    // 		this.add('');
    // 		if (side?.allySide) {
    // 			this.add('win', side.name + ' & ' + side.allySide.name);
    // 		} else if (side) {
    // 			this.add('win', side.name);
    // 		} else {
    // 			this.add('tie');
    // 		}
    // 		this.ended = true;
    // 		this.requestState = '';
    // 		for (const s of this.sides) {
    // 			if (s) s.activeRequest = null;
    // 		}
    // 		return true;
    // 	}
    //
    pub fn win(&mut self, side: Option<SideID>) -> bool {
        // JavaScript: if (this.ended) return false;
        if self.ended {
            return false;
        }

        // JavaScript: if (side && typeof side === 'string') side = this.getSide(side);
        // (Rust uses SideID enum, not string)

        // JavaScript: else if (!side || !this.sides.includes(side)) side = null;
        // (Rust Option<SideID> handles this)

        // JavaScript: this.winner = side ? side.name : '';
        let (winner_name, ally_name) = if let Some(side_id) = side {
            if let Some(side) = self.get_side(side_id) {
                let ally = side.ally_index
                    .and_then(|idx| self.sides.get(idx))
                    .map(|s| s.name.clone());
                (Some(side.name.clone()), ally)
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };

        self.winner = winner_name.clone();

        // JavaScript: this.add('');
        self.add_log("", &[]);

        // JavaScript: if (side?.allySide) { this.add('win', side.name + ' & ' + side.allySide.name); }
        if let (Some(ref name), Some(ref ally)) = (&winner_name, &ally_name) {
            let combined = format!("{} & {}", name, ally);
            self.add_log("win", &[&combined]);
        } else if let Some(ref name) = winner_name {
            // JavaScript: else if (side) { this.add('win', side.name); }
            self.add_log("win", &[name]);
        } else {
            // JavaScript: else { this.add('tie'); }
            self.add_log("tie", &[]);
        }

        // JavaScript: this.ended = true;
        self.ended = true;

        // JavaScript: this.requestState = '';
        self.request_state = BattleRequestState::None;

        // JavaScript: for (const s of this.sides) { if (s) s.activeRequest = null; }
        for side in &mut self.sides {
            side.request_state = RequestState::None;
        }

        // JavaScript: return true;
        true
    }

    /// Force a side to lose
    /// Equivalent to battle.ts lose() (battle.ts:1499-1518)
    // 
    // 	lose(side: SideID | Side) {
    // 		if (typeof side === 'string') {
    // 			side = this.getSide(side);
    // 		}
    // 		if (!side) return; // can happen if a battle crashes
    // 		if (this.gameType !== 'freeforall') {
    // 			return this.win(side.foe);
    // 		}
    // 		if (!side.pokemonLeft) return;
    // 
    // 		side.pokemonLeft = 0;
    // 		side.active[0]?.faint();
    // 		this.faintMessages(false, true);
    // 		if (!this.ended && side.requestState) {
    // 			side.emitRequest({ wait: true, side: side.getRequestData() });
    // 			side.clearChoice();
    // 			if (this.allChoicesDone()) this.commitChoices();
    // 		}
    // 		return true;
    // 	}
    //
    pub fn lose(&mut self, side_id: SideID) {
        // JavaScript: if (typeof side === 'string') side = this.getSide(side);
        // (Rust already has SideID, no conversion needed)

        // JavaScript: if (!side) return; // can happen if a battle crashes
        // (Rust SideID is always valid)

        // JavaScript: if (this.gameType !== 'freeforall') return this.win(side.foe);
        if self.game_type != GameType::FreeForAll {
            let foe_id = match side_id {
                SideID::P1 => SideID::P2,
                SideID::P2 => SideID::P1,
                SideID::P3 => SideID::P4,
                SideID::P4 => SideID::P3,
            };
            self.win(Some(foe_id));
            return;
        }

        // JavaScript: if (!side.pokemonLeft) return;
        if let Some(side) = self.sides.get(side_id.index()) {
            if side.pokemon_left == 0 {
                return;
            }
        }

        // JavaScript: side.pokemonLeft = 0;
        if let Some(side) = self.sides.get_mut(side_id.index()) {
            side.pokemon_left = 0;

            // JavaScript: side.active[0]?.faint();
            if let Some(Some(poke_idx)) = side.active.get(0) {
                if let Some(pokemon) = side.pokemon.get_mut(*poke_idx) {
                    pokemon.faint();
                }
            }
        }

        // JavaScript: this.faintMessages(false, true);
        self.faint_messages(false, true, true);

        // JavaScript: if (!this.ended && side.requestState) { ... }
        if !self.ended {
            if let Some(side) = self.sides.get_mut(side_id.index()) {
                if side.request_state != RequestState::None {
                    // JavaScript: side.emitRequest({ wait: true, side: side.getRequestData() });
                    // (We don't have emitRequest in Rust, skip for now)

                    // JavaScript: side.clearChoice();
                    side.choice = Choice::new();

                    // JavaScript: if (this.allChoicesDone()) this.commitChoices();
                    // (Would need to implement allChoicesDone and commitChoices)
                }
            }
        }
    }

    /// Force a win for a specific side
    /// Equivalent to battle.ts forceWin()
    // 
    // 	forceWin(side: SideID | null = null) {
    // 		if (this.ended) return false;
    // 		this.inputLog.push(side ? `>forcewin ${side}` : `>forcetie`);
    // 		return this.win(side);
    // 	}
    //
    pub fn force_win(&mut self, side: Option<SideID>) -> bool {
        if self.ended {
            return false;
        }
        // Log to inputLog (if we had inputLog field, would log here)
        // JavaScript: this.inputLog.push(side ? `>forcewin ${side}` : `>forcetie`);
        self.win(side)
    }

    /// Set the currently active move
    /// Equivalent to battle.ts setActiveMove()
    // 
    // 	setActiveMove(move?: ActiveMove | null, pokemon?: Pokemon | null, target?: Pokemon | null) {
    // 		this.activeMove = move || null;
    // 		this.activePokemon = pokemon || null;
    // 		this.activeTarget = target || pokemon || null;
    // 	}
    //
    pub fn set_active_move(&mut self, move_id: Option<ID>, pokemon: Option<(usize, usize)>, target: Option<(usize, usize)>) {
        self.active_move = move_id.and_then(|id| self.dex.get_active_move(&id.to_string()));
        self.active_pokemon = pokemon;
        self.active_target = target.or(pokemon);
    }

    /// Clear the currently active move
    /// Equivalent to battle.ts clearActiveMove()
    // 
    // 	clearActiveMove(failed?: boolean) {
    // 		if (this.activeMove) {
    // 			if (!failed) {
    // 				this.lastMove = this.activeMove;
    // 			}
    // 			this.activeMove = null;
    // 			this.activePokemon = null;
    // 			this.activeTarget = null;
    // 		}
    // 	}
    //
    pub fn clear_active_move(&mut self, failed: bool) {
        if self.active_move.is_some() {
            if !failed {
                self.last_move = self.active_move.as_ref().map(|m| m.id.clone());
                self.active_move = None;
            } else {
                self.active_move = None;
            }
            self.active_pokemon = None;
            self.active_target = None;
        }
    }

    /// Check if an ability is being suppressed (Mold Breaker, etc.)
    /// Equivalent to battle.ts suppressingAbility()
    // 
    // 	suppressingAbility(target?: Pokemon) {
    // 		return this.activePokemon && this.activePokemon.isActive && (this.activePokemon !== target || this.gen < 8) &&
    // 			this.activeMove && this.activeMove.ignoreAbility && !target?.hasItem('Ability Shield');
    // 	}
    //
    pub fn suppressing_ability(&self, target: Option<(usize, usize)>) -> bool {
        // Check if there's an active Pokemon using a move that ignores abilities
        if let Some((active_side, active_idx)) = self.active_pokemon {
            if let Some(side) = self.sides.get(active_side) {
                if let Some(pokemon) = side.pokemon.get(active_idx) {
                    if !pokemon.is_active {
                        return false;
                    }

                    // Check if active move ignores abilities
                    if let Some(ref move_id) = self.active_move {
                        // Mold Breaker, Teravolt, Turboblaze
                        let ability = pokemon.ability.as_str();
                        let ignores = ability == "moldbreaker" || ability == "teravolt" || ability == "turboblaze";

                        // In Gen 8+, can't suppress your own ability
                        if self.gen >= 8 {
                            if let Some(t) = target {
                                if t == (active_side, active_idx) {
                                    return false;
                                }
                            }
                        }

                        // Check if target has Ability Shield
                        if let Some((target_side, target_idx)) = target {
                            if let Some(tside) = self.sides.get(target_side) {
                                if let Some(tpoke) = tside.pokemon.get(target_idx) {
                                    if tpoke.item.as_str() == "abilityshield" {
                                        return false;
                                    }
                                }
                            }
                        }

                        return ignores;
                    }
                }
            }
        }
        false
    }

    /// Get all Pokemon in the battle (not just active)
    /// Equivalent to battle.ts getAllPokemon()
    // 
    // 	getAllPokemon() {
    // 		const pokemonList: Pokemon[] = [];
    // 		for (const side of this.sides) {
    // 			pokemonList.push(...side.pokemon);
    // 		}
    // 		return pokemonList;
    // 	}
    //
    pub fn get_all_pokemon(&self) -> Vec<&crate::pokemon::Pokemon> {
        let mut result = Vec::new();
        for side in &self.sides {
            for pokemon in &side.pokemon {
                result.push(pokemon);
            }
        }
        result
    }

    /// Check if two pokemon are adjacent
    /// Equivalent to Pokemon.isAdjacent() in pokemon.ts
    //
    // isAdjacent(pokemon2: Pokemon) {
    //     if (this.fainted || pokemon2.fainted) return false;
    //     if (this.battle.activePerHalf <= 2) return this !== pokemon2;
    //     if (this.side === pokemon2.side) return Math.abs(this.position - pokemon2.position) === 1;
    //     return Math.abs(this.position + pokemon2.position + 1 - this.side.active.length) <= 1;
    // }
    //
    pub fn is_adjacent(&self, pos1: (usize, usize), pos2: (usize, usize)) -> bool {
        let (side1, idx1) = pos1;
        let (side2, idx2) = pos2;

        // Get both pokemon
        let poke1 = match self.sides.get(side1).and_then(|s| s.pokemon.get(idx1)) {
            Some(p) => p,
            None => return false,
        };
        let poke2 = match self.sides.get(side2).and_then(|s| s.pokemon.get(idx2)) {
            Some(p) => p,
            None => return false,
        };

        // JS: if (this.fainted || pokemon2.fainted) return false;
        if poke1.is_fainted() || poke2.is_fainted() {
            return false;
        }

        // JS: if (this.battle.activePerHalf <= 2) return this !== pokemon2;
        if self.active_per_half <= 2 {
            return pos1 != pos2;
        }

        // JS: if (this.side === pokemon2.side) return Math.abs(this.position - pokemon2.position) === 1;
        if side1 == side2 {
            let pos_diff = (poke1.position as i32 - poke2.position as i32).abs();
            return pos_diff == 1;
        }

        // JS: return Math.abs(this.position + pokemon2.position + 1 - this.side.active.length) <= 1;
        let active_length = self.sides.get(side1).map(|s| s.active.len()).unwrap_or(0);
        let sum = poke1.position as i32 + poke2.position as i32 + 1 - active_length as i32;
        sum.abs() <= 1
    }

    /// Get adjacent allies of a pokemon
    /// Equivalent to Pokemon.adjacentAllies() in pokemon.ts
    //
    // adjacentAllies(): Pokemon[] {
    //     return this.side.allies().filter(ally => this.isAdjacent(ally));
    // }
    //
    pub fn adjacent_allies(&self, side_idx: usize, poke_idx: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        let pokemon_pos = (side_idx, poke_idx);

        // Get allies from the same side (active team)
        if let Some(side) = self.sides.get(side_idx) {
            for (i, active_slot) in side.active.iter().enumerate() {
                if let Some(ally_idx) = active_slot {
                    let ally_pos = (side_idx, *ally_idx);
                    // Skip if it's the same pokemon
                    if ally_pos == pokemon_pos {
                        continue;
                    }
                    // Check if alive
                    if let Some(ally) = side.pokemon.get(*ally_idx) {
                        if !ally.is_fainted() && self.is_adjacent(pokemon_pos, ally_pos) {
                            result.push(ally_pos);
                        }
                    }
                }
            }

            // For multi battles, also check ally side
            // JS: activeTeam() - if (this.battle.gameType !== 'multi') return this.active;
            if self.game_type == GameType::Multi {
                // Get the ally side (n % 2 + 2 for sides beyond first two)
                let ally_side_idx = if side_idx < 2 {
                    side_idx + 2
                } else {
                    side_idx - 2
                };

                if let Some(ally_side) = self.sides.get(ally_side_idx) {
                    for (i, active_slot) in ally_side.active.iter().enumerate() {
                        if let Some(ally_idx) = active_slot {
                            let ally_pos = (ally_side_idx, *ally_idx);
                            if let Some(ally) = ally_side.pokemon.get(*ally_idx) {
                                if !ally.is_fainted() && self.is_adjacent(pokemon_pos, ally_pos) {
                                    result.push(ally_pos);
                                }
                            }
                        }
                    }
                }
            }
        }

        result
    }

    /// Get adjacent foes of a pokemon
    /// Equivalent to Pokemon.adjacentFoes() in pokemon.ts
    //
    // adjacentFoes(): Pokemon[] {
    //     if (this.battle.activePerHalf <= 2) return this.side.foes();
    //     return this.side.foes().filter(foe => this.isAdjacent(foe));
    // }
    //
    pub fn adjacent_foes(&self, side_idx: usize, poke_idx: usize) -> Vec<(usize, usize)> {
        let pokemon_pos = (side_idx, poke_idx);
        let mut result = Vec::new();

        // JS: if (this.battle.activePerHalf <= 2) return this.side.foes();
        // For singles and doubles, all active foes are adjacent
        if self.active_per_half <= 2 {
            // Get foe side(s)
            for (foe_side_idx, foe_side) in self.sides.iter().enumerate() {
                // Skip own side
                if foe_side_idx == side_idx {
                    continue;
                }
                // For non-multi battles, opposing sides are foes
                if self.game_type != GameType::Multi || (foe_side_idx % 2) != (side_idx % 2) {
                    for active_slot in &foe_side.active {
                        if let Some(foe_idx) = active_slot {
                            if let Some(foe) = foe_side.pokemon.get(*foe_idx) {
                                if !foe.is_fainted() {
                                    result.push((foe_side_idx, *foe_idx));
                                }
                            }
                        }
                    }
                }
            }
        } else {
            // For triple battles and beyond, filter by adjacency
            // JS: return this.side.foes().filter(foe => this.isAdjacent(foe));
            for (foe_side_idx, foe_side) in self.sides.iter().enumerate() {
                // Skip own side
                if foe_side_idx == side_idx {
                    continue;
                }
                // For non-multi battles, opposing sides are foes
                if self.game_type != GameType::Multi || (foe_side_idx % 2) != (side_idx % 2) {
                    for active_slot in &foe_side.active {
                        if let Some(foe_idx) = active_slot {
                            let foe_pos = (foe_side_idx, *foe_idx);
                            if let Some(foe) = foe_side.pokemon.get(*foe_idx) {
                                if !foe.is_fainted() && self.is_adjacent(pokemon_pos, foe_pos) {
                                    result.push(foe_pos);
                                }
                            }
                        }
                    }
                }
            }
        }

        result
    }

    /// Get all allies including self
    /// Equivalent to pokemon.ts alliesAndSelf() which calls side.allies()
    ///
    // 	alliesAndSelf(): Pokemon[] {
    // 		return this.side.allies();
    // 	}
    //
    // side.allies(all?: boolean) implementation:
    // 	allies(all?: boolean) {
    // 		// called during the first switch-in, so `active` can still contain nulls at this point
    // 		let allies = this.activeTeam().filter(ally => ally);
    // 		if (!all) allies = allies.filter(ally => !!ally.hp);
    //
    // 		return allies;
    // 	}
    //
    pub fn allies_and_self(&self, side_idx: usize, include_fainted: bool) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        if let Some(side) = self.sides.get(side_idx) {
            // Iterate through active slots on this side
            for active_slot in &side.active {
                if let Some(poke_idx) = active_slot {
                    if let Some(pokemon) = side.pokemon.get(*poke_idx) {
                        // Include if: include_fainted=true OR pokemon has HP
                        if include_fainted || pokemon.hp > 0 {
                            result.push((side_idx, *poke_idx));
                        }
                    }
                }
            }
        }

        result
    }

    /// Get all foes
    /// Equivalent to pokemon.ts foes() which calls side.foes()
    ///
    // 	foes(all?: boolean): Pokemon[] {
    // 		return this.side.foes(all);
    // 	}
    //
    // side.foes(all?: boolean) implementation:
    // 	foes(all?: boolean) {
    // 		if (this.battle.gameType === 'freeforall') {
    // 			return this.battle.sides.map(side => side.active[0])
    // 				.filter(pokemon => pokemon && pokemon.side !== this && (all || !!pokemon.hp));
    // 		}
    // 		return this.foe.allies(all);
    // 	}
    //
    pub fn foes(&self, side_idx: usize, include_fainted: bool) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        // JS: if (this.battle.gameType === 'freeforall') {
        if self.game_type == GameType::FreeForAll {
            // Return all active pokemon from other sides
            for (foe_side_idx, foe_side) in self.sides.iter().enumerate() {
                if foe_side_idx != side_idx {
                    if let Some(Some(foe_idx)) = foe_side.active.first() {
                        if let Some(foe) = foe_side.pokemon.get(*foe_idx) {
                            if include_fainted || foe.hp > 0 {
                                result.push((foe_side_idx, *foe_idx));
                            }
                        }
                    }
                }
            }
        } else {
            // JS: return this.foe.allies(all);
            // Get foe side index
            let foe_side_idx = if side_idx == 0 { 1 } else { 0 };
            // Return allies from foe side
            result = self.allies_and_self(foe_side_idx, include_fainted);
        }

        result
    }

    /// Get move targets and pressure targets
    /// Equivalent to pokemon.ts getMoveTargets()
    ///
    // 	getMoveTargets(move: ActiveMove, target: Pokemon): { targets: Pokemon[], pressureTargets: Pokemon[] } {
    // 		let targets: Pokemon[] = [];
    //
    // 		switch (move.target) {
    // 		case 'all':
    // 		case 'foeSide':
    // 		case 'allySide':
    // 		case 'allyTeam':
    // 			if (!move.target.startsWith('foe')) {
    // 				targets.push(...this.alliesAndSelf());
    // 			}
    // 			if (!move.target.startsWith('ally')) {
    // 				targets.push(...this.foes(true));
    // 			}
    // 			if (targets.length && !targets.includes(target)) {
    // 				this.battle.retargetLastMove(targets[targets.length - 1]);
    // 			}
    // 			break;
    // 		case 'allAdjacent':
    // 			targets.push(...this.adjacentAllies());
    // 			// falls through
    // 		case 'allAdjacentFoes':
    // 			targets.push(...this.adjacentFoes());
    // 			if (targets.length && !targets.includes(target)) {
    // 				this.battle.retargetLastMove(targets[targets.length - 1]);
    // 			}
    // 			break;
    // 		case 'allies':
    // 			targets = this.alliesAndSelf();
    // 			break;
    // 		default:
    // 			const selectedTarget = target;
    // 			if (!target || (target.fainted && !target.isAlly(this)) && this.battle.gameType !== 'freeforall') {
    // 				// If a targeted foe faints, the move is retargeted
    // 				const possibleTarget = this.battle.getRandomTarget(this, move);
    // 				if (!possibleTarget) return { targets: [], pressureTargets: [] };
    // 				target = possibleTarget;
    // 			}
    // 			if (this.battle.activePerHalf > 1 && !move.tracksTarget) {
    // 				const isCharging = move.flags['charge'] && !this.volatiles['twoturnmove'] &&
    // 					!(move.id.startsWith('solarb') && ['sunnyday', 'desolateland'].includes(this.effectiveWeather())) &&
    // 					!(move.id === 'electroshot' && ['raindance', 'primordialsea'].includes(this.effectiveWeather())) &&
    // 					!(this.hasItem('powerherb') && move.id !== 'skydrop');
    // 				if (!isCharging && !(move.id === 'pursuit' && (target.beingCalledBack || target.switchFlag))) {
    // 					target = this.battle.priorityEvent('RedirectTarget', this, this, move, target);
    // 				}
    // 			}
    // 			if (move.smartTarget) {
    // 				targets = this.getSmartTargets(target, move);
    // 				target = targets[0];
    // 			} else {
    // 				targets.push(target);
    // 			}
    // 			if (target.fainted && !move.flags['futuremove']) {
    // 				return { targets: [], pressureTargets: [] };
    // 			}
    // 			if (selectedTarget !== target) {
    // 				this.battle.retargetLastMove(target);
    // 			}
    // 		}
    //
    // 		// Resolve apparent targets for Pressure.
    // 		let pressureTargets = targets;
    // 		if (move.target === 'foeSide') {
    // 			pressureTargets = [];
    // 		}
    // 		if (move.flags['mustpressure']) {
    // 			pressureTargets = this.foes();
    // 		}
    //
    // 		return { targets, pressureTargets };
    // 	}
    //
    pub fn get_move_targets(
        &mut self,
        user_pos: (usize, usize),
        move_id: &ID,
        mut target: Option<(usize, usize)>,
    ) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
        let mut targets: Vec<(usize, usize)> = Vec::new();

        // Get move data to access target and flags
        let (move_target, has_mustpressure, has_futuremove, has_smart_target) = match self.dex.get_move(move_id.as_str()) {
            Some(m) => (
                m.target.clone(),
                m.flags.contains_key("mustpressure"),
                m.flags.contains_key("futuremove"),
                m.smart_target.unwrap_or(false)
            ),
            None => return (vec![], vec![]),
        };

        // Handle different target types
        match move_target.as_str() {
            "all" | "foeSide" | "allySide" | "allyTeam" => {
                // JS: if (!move.target.startsWith('foe')) targets.push(...this.alliesAndSelf());
                if !move_target.starts_with("foe") {
                    targets.extend(self.allies_and_self(user_pos.0, true));
                }
                // JS: if (!move.target.startsWith('ally')) targets.push(...this.foes(true));
                if !move_target.starts_with("ally") {
                    targets.extend(self.foes(user_pos.0, true));
                }
                // JS: if (targets.length && !targets.includes(target)) this.battle.retargetLastMove(targets[targets.length - 1]);
                if !targets.is_empty() && !target.map_or(false, |t| targets.contains(&t)) {
                    if let Some(&last_target) = targets.last() {
                        self.retarget_last_move(last_target);
                    }
                }
            }
            "allAdjacent" => {
                // JS: targets.push(...this.adjacentAllies());
                targets.extend(self.adjacent_allies(user_pos.0, user_pos.1));
                // falls through to allAdjacentFoes
                targets.extend(self.adjacent_foes(user_pos.0, user_pos.1));
                if !targets.is_empty() && !target.map_or(false, |t| targets.contains(&t)) {
                    if let Some(&last_target) = targets.last() {
                        self.retarget_last_move(last_target);
                    }
                }
            }
            "allAdjacentFoes" => {
                // JS: targets.push(...this.adjacentFoes());
                targets.extend(self.adjacent_foes(user_pos.0, user_pos.1));
                if !targets.is_empty() && !target.map_or(false, |t| targets.contains(&t)) {
                    if let Some(&last_target) = targets.last() {
                        self.retarget_last_move(last_target);
                    }
                }
            }
            "allies" => {
                // JS: targets = this.alliesAndSelf();
                targets = self.allies_and_self(user_pos.0, false);
            }
            _ => {
                // Default case - single target moves
                let selected_target = target;

                // JS: if (!target || (target.fainted && !target.isAlly(this)) && this.battle.gameType !== 'freeforall')
                if target.is_none()
                    || (self.is_pokemon_fainted(target.unwrap())
                        && !self.is_ally(target.unwrap(), user_pos)
                        && self.game_type != GameType::FreeForAll)
                {
                    // JS: const possibleTarget = this.battle.getRandomTarget(this, move);
                    target = self.get_random_target(user_pos.0, user_pos.1, &move_target);
                    if target.is_none() {
                        return (vec![], vec![]);
                    }
                }

                // JS: if (this.battle.activePerHalf > 1 && !move.tracksTarget) {
                //       target = this.battle.priorityEvent('RedirectTarget', this, this, move, target);
                //     }
                if self.active_per_half > 1 {
                    // Get move data to check tracksTarget
                    if let Some(move_data) = self.dex.get_move(move_id.as_str()) {
                        if !move_data.tracks_target.unwrap_or(false) {
                            // Encode current target position for relay variable
                            // Format: side_idx * 10 + pokemon_idx
                            if let Some((target_side, target_pos)) = target {
                                let encoded_target = (target_side as i32 * 10) + target_pos as i32;

                                // Call RedirectTarget priority event
                                // JS: target = this.battle.priorityEvent('RedirectTarget', this, this, move, target);
                                // Priority events can modify the relay variable (encoded target)
                                // This allows abilities like Storm Drain and Lightning Rod to redirect moves
                                if let Some(new_encoded) = self.priority_event(
                                    "RedirectTarget",
                                    Some(user_pos),
                                    Some(user_pos),
                                    Some(move_id),
                                    Some(encoded_target),
                                ) {
                                    // Decode the new target position
                                    let new_side = (new_encoded / 10) as usize;
                                    let new_pos = (new_encoded % 10) as usize;
                                    target = Some((new_side, new_pos));
                                }
                            }
                        }
                    }
                }

                // JS: if (move.smartTarget) {
                //       targets = this.getSmartTargets(target, move);
                //       target = targets[0];
                //     }
                // Smart targeting for Dragon Darts: if target fainted, retarget to adjacent ally
                if has_smart_target {
                    if let Some((target_side, target_pos)) = target {
                        // Get target's first adjacent ally
                        let target2 = {
                            let adjacent_allies = self.adjacent_allies(target_side, target_pos);
                            adjacent_allies.first().copied()
                        };

                        // Check if target2 is valid (exists, is not self, has HP)
                        if let Some((ally_side, ally_pos)) = target2 {
                            let target2_is_self = (ally_side == user_pos.0 && ally_pos == user_pos.1);
                            let target2_has_hp = !self.is_pokemon_fainted((ally_side, ally_pos));
                            let target_has_hp = !self.is_pokemon_fainted((target_side, target_pos));

                            if !target2_is_self && target2_has_hp {
                                if !target_has_hp {
                                    // Target fainted, use target2 instead
                                    // JS: if (!target.hp) { move.smartTarget = false; return [target2]; }
                                    targets.push((ally_side, ally_pos));
                                    target = Some((ally_side, ally_pos));
                                } else {
                                    // Both targets alive, hit both
                                    // JS: return [target, target2];
                                    targets.push((target_side, target_pos));
                                    target = Some((target_side, target_pos));
                                }
                            } else {
                                // target2 invalid, just use target
                                // JS: if (!target2 || target2 === this || !target2.hp) { move.smartTarget = false; return [target]; }
                                targets.push((target_side, target_pos));
                            }
                        } else {
                            // No target2, just use target
                            targets.push((target_side, target_pos));
                        }
                    }
                } else {
                    // Not smart targeting, just add target
                    if let Some(t) = target {
                        targets.push(t);
                    }
                }

                // JS: if (target.fainted && !move.flags['futuremove']) return { targets: [], pressureTargets: [] };
                if target.map_or(false, |t| self.is_pokemon_fainted(t)) && !has_futuremove {
                    return (vec![], vec![]);
                }

                // JS: if (selectedTarget !== target) this.battle.retargetLastMove(target);
                if selected_target != target {
                    if let Some(t) = target {
                        self.retarget_last_move(t);
                    }
                }
            }
        }

        // Resolve apparent targets for Pressure
        // JS: let pressureTargets = targets;
        let mut pressure_targets = targets.clone();

        // JS: if (move.target === 'foeSide') pressureTargets = [];
        if move_target == "foeSide" {
            pressure_targets.clear();
        }

        // JS: if (move.flags['mustpressure']) pressureTargets = this.foes();
        if has_mustpressure {
            pressure_targets = self.foes(user_pos.0, true);
        }

        (targets, pressure_targets)
    }

    /// Check if two pokemon are allies
    fn is_ally(&self, pos1: (usize, usize), pos2: (usize, usize)) -> bool {
        // JS: return !!pokemon && (this.side === pokemon.side || this.side.allySide === pokemon.side);
        if pos1.0 == pos2.0 {
            return true;
        }
        // Check if pos2's side is an ally of pos1's side (for multi battles)
        if let Some(side1) = self.sides.get(pos1.0) {
            if let Some(ally_idx) = side1.ally_index {
                if ally_idx == pos2.0 {
                    return true;
                }
            }
        }
        false
    }

    /// Check if pokemon is fainted
    /// Rust helper method - JavaScript directly accesses pokemon.fainted or checks pokemon.hp <= 0
    /// This provides a safe accessor that handles the tuple position format used in Rust
    fn is_pokemon_fainted(&self, pos: (usize, usize)) -> bool {
        self.sides
            .get(pos.0)
            .and_then(|s| s.pokemon.get(pos.1))
            .map_or(true, |p| p.is_fainted())
    }

    /// Get random target for a move
    /// Equivalent to battle.ts getRandomTarget()
    ///
    // 
    // 	getRandomTarget(pokemon: Pokemon, move: string | Move) {
    // 		// A move was used without a chosen target
    // 
    // 		// For instance: Metronome chooses Ice Beam. Since the user didn't
    // 		// choose a target when choosing Metronome, Ice Beam's target must
    // 		// be chosen randomly.
    // 
    // 		// The target is chosen randomly from possible targets, EXCEPT that
    // 		// moves that can target either allies or foes will only target foes
    // 		// when used without an explicit target.
    // 
    // 		move = this.dex.moves.get(move);
    // 		if (['self', 'all', 'allySide', 'allyTeam', 'adjacentAllyOrSelf'].includes(move.target)) {
    // 			return pokemon;
    // 		} else if (move.target === 'adjacentAlly') {
    // 			if (this.gameType === 'singles') return null;
    // 			const adjacentAllies = pokemon.adjacentAllies();
    // 			return adjacentAllies.length ? this.sample(adjacentAllies) : null;
    // 		}
    // 		if (this.gameType === 'singles') return pokemon.side.foe.active[0];
    // 
    // 		if (this.activePerHalf > 2) {
    // 			if (move.target === 'adjacentFoe' || move.target === 'normal' || move.target === 'randomNormal') {
    // 				// even if a move can target an ally, auto-resolution will never make it target an ally
    // 				// i.e. if both your opponents faint before you use Flamethrower, it will fail instead of targeting your ally
    // 				const adjacentFoes = pokemon.adjacentFoes();
    // 				if (adjacentFoes.length) return this.sample(adjacentFoes);
    // 				// no valid target at all, return slot directly across for any possible redirection
    // 				return pokemon.side.foe.active[pokemon.side.foe.active.length - 1 - pokemon.position];
    // 			}
    // 		}
    // 		return pokemon.side.randomFoe() || pokemon.side.foe.active[0];
    // 	}
    //
    pub fn get_random_target(&mut self, user_side: usize, user_idx: usize, move_target: &str) -> Option<(usize, usize)> {
        // JS: if (['self', 'all', 'allySide', 'allyTeam', 'adjacentAllyOrSelf'].includes(move.target)) return pokemon;
        if move_target == "Self" || move_target == "All" || move_target == "AllySide"
            || move_target == "AllyTeam" || move_target == "AdjacentAllyOrSelf" {
            return Some((user_side, user_idx));
        }

        // JS: else if (move.target === 'adjacentAlly') {
        // JS:     if (this.gameType === 'singles') return null;
        // JS:     const adjacentAllies = pokemon.adjacentAllies();
        // JS:     return adjacentAllies.length ? this.sample(adjacentAllies) : null;
        // JS: }
        if move_target == "AdjacentAlly" {
            if self.game_type == GameType::Singles {
                return None;
            }
            let adjacent_allies = self.adjacent_allies(user_side, user_idx);
            if !adjacent_allies.is_empty() {
                return self.sample(&adjacent_allies).copied();
            }
            return None;
        }

        // JS: if (this.gameType === 'singles') return pokemon.side.foe.active[0];
        if self.game_type == GameType::Singles {
            let foe_side = if user_side == 0 { 1 } else { 0 };
            if foe_side < self.sides.len() {
                if let Some(side) = self.sides.get(foe_side) {
                    if let Some(Some(poke_idx)) = side.active.get(0) {
                        return Some((foe_side, *poke_idx));
                    }
                }
            }
            return None;
        }

        // JS: if (this.activePerHalf > 2) {
        if self.active_per_half > 2 {
            // JS: if (move.target === 'adjacentFoe' || move.target === 'normal' || move.target === 'randomNormal') {
            if move_target == "AdjacentFoe" || move_target == "Normal" || move_target == "RandomNormal" {
                // JS: const adjacentFoes = pokemon.adjacentFoes();
                // JS: if (adjacentFoes.length) return this.sample(adjacentFoes);
                let adjacent_foes = self.adjacent_foes(user_side, user_idx);
                if !adjacent_foes.is_empty() {
                    return self.sample(&adjacent_foes).copied();
                }

                // JS: return pokemon.side.foe.active[pokemon.side.foe.active.length - 1 - pokemon.position];
                // No valid adjacent target, return slot directly across for any possible redirection
                let foe_side = if user_side == 0 { 1 } else { 0 };
                if foe_side < self.sides.len() {
                    if let Some(side) = self.sides.get(foe_side) {
                        let position = if let Some(user_side_ref) = self.sides.get(user_side) {
                            if let Some(pokemon) = user_side_ref.pokemon.get(user_idx) {
                                pokemon.position
                            } else {
                                0
                            }
                        } else {
                            0
                        };
                        let target_slot = side.active.len().saturating_sub(1).saturating_sub(position);
                        if let Some(Some(poke_idx)) = side.active.get(target_slot) {
                            return Some((foe_side, *poke_idx));
                        }
                    }
                }
            }
        }

        // JS: return pokemon.side.randomFoe() || pokemon.side.foe.active[0];
        let foe_side = if user_side == 0 { 1 } else { 0 };
        if foe_side < self.sides.len() {
            // Try to get a random active foe
            let valid_targets: Vec<usize> = self.sides[foe_side].pokemon.iter()
                .enumerate()
                .filter(|(_, p)| p.is_active && !p.is_fainted())
                .map(|(idx, _)| idx)
                .collect();

            if !valid_targets.is_empty() {
                let random_idx = self.random(valid_targets.len() as i32) as usize;
                return Some((foe_side, valid_targets[random_idx]));
            }

            // Fallback: return first active
            if let Some(Some(poke_idx)) = self.sides[foe_side].active.get(0) {
                return Some((foe_side, *poke_idx));
            }
        }

        None
    }

    /// Update speed for all active Pokemon
    /// Equivalent to battle.ts updateSpeed()
    ///
    // 
    // 	updateSpeed() {
    // 		for (const pokemon of this.getAllActive()) {
    // 			pokemon.updateSpeed();
    // 		}
    // 	}
    //
    /// Update speed for all active Pokemon
    /// Equivalent to TypeScript updateSpeed() (battle.ts:387-391)
    pub fn update_speed(&mut self) {
        // Collect indices first to avoid borrow checker issues
        let indices: Vec<(usize, usize)> = self.get_all_active(false);

        // Update each active Pokemon's speed
        for (side_idx, poke_idx) in indices {
            if let Some(side) = self.sides.get_mut(side_idx) {
                if let Some(pokemon) = side.pokemon.get_mut(poke_idx) {
                    pokemon.update_speed();
                }
            }
        }
    }

    /// Deal damage to a Pokemon
    /// Matches JavaScript battle.ts:2165-2176 damage()
    /// This is a wrapper around spreadDamage for a single target
    /// Deal damage to Pokemon
    /// Equivalent to battle.ts damage() (battle.ts:2165-2175)
    ///
    // 
    // 	damage(
    // 		damage: number, target: Pokemon | null = null, source: Pokemon | null = null,
    // 		effect: 'drain' | 'recoil' | Effect | null = null, instafaint = false
    // 	) {
    // 		if (this.event) {
    // 			target ||= this.event.target;
    // 			source ||= this.event.source;
    // 			effect ||= this.effect;
    // 		}
    // 		return this.spreadDamage([damage], [target], source, effect, instafaint)[0];
    // 	}
    //
    pub fn damage(
        &mut self,
        damage: i32,
        target: Option<(usize, usize)>,
        source: Option<(usize, usize)>,
        effect: Option<&ID>,
        instafaint: bool,
    ) -> Option<i32> {
        // JS: if (this.event) { target ||= this.event.target; source ||= this.event.source; effect ||= this.effect; }
        // Extract event context values first to avoid borrow checker issues
        let (event_target, event_source, event_effect) = if let Some(ref event) = self.current_event {
            (event.target, event.source, event.effect.clone())
        } else {
            (None, None, None)
        };

        let target = target.or(event_target);
        let source = source.or(event_source);
        let effect_owned: Option<ID>;
        let effect = if effect.is_none() {
            effect_owned = event_effect;
            effect_owned.as_ref()
        } else {
            effect
        };

        // JavaScript: return this.spreadDamage([damage], [target], source, effect, instafaint)[0];
        let result = self.spread_damage(
            &[Some(damage)],
            &[target],
            source,
            effect,
            instafaint,
        );
        result.get(0).copied().flatten()
    }

    /// Deal direct damage (bypasses most effects)
    /// Matches JavaScript battle.ts:2177-2230 directDamage()
    ///
    // 
    // 	directDamage(damage: number, target?: Pokemon, source: Pokemon | null = null, effect: Effect | null = null) {
    // 		if (this.event) {
    // 			target ||= this.event.target;
    // 			source ||= this.event.source;
    // 			effect ||= this.effect;
    // 		}
    // 		if (!target?.hp) return 0;
    // 		if (!damage) return 0;
    // 		damage = this.clampIntRange(damage, 1);
    // 
    // 		if (typeof effect === 'string' || !effect) effect = this.dex.conditions.getByID((effect || '') as ID);
    // 
    // 		// In Gen 1 BUT NOT STADIUM, Substitute also takes confusion and HJK recoil damage
    // 		if (this.gen <= 1 && this.dex.currentMod !== 'gen1stadium' &&
    // 			['confusion', 'jumpkick', 'highjumpkick'].includes(effect.id)) {
    // 			// Confusion and recoil damage can be countered
    // 			this.lastDamage = damage;
    // 			if (target.volatiles['substitute']) {
    // 				const hint = "In Gen 1, if a Pokemon with a Substitute hurts itself due to confusion or Jump Kick/Hi Jump Kick recoil and the target";
    // 				// if the move was a self-targeting move, the source is the same as the target. We need to check the opposing substitute
    // 				const foe = target.side.foe.active[0];
    // 				if (foe?.volatiles['substitute']) {
    // 					foe.volatiles['substitute'].hp -= damage;
    // 					if (foe.volatiles['substitute'].hp <= 0) {
    // 						foe.removeVolatile('substitute');
    // 						foe.subFainted = true;
    // 					} else {
    // 						this.add('-activate', foe, 'Substitute', '[damage]');
    // 					}
    // 					this.hint(hint + " has a Substitute, the target's Substitute takes the damage.");
    // 					return damage;
    // 				} else {
    // 					this.hint(hint + " does not have a Substitute there is no damage dealt.");
    // 					return 0;
    // 				}
    // 			}
    // 		}
    // 
    // 		damage = target.damage(damage, source, effect);
    // 		switch (effect.id) {
    // 		case 'strugglerecoil':
    // 			this.add('-damage', target, target.getHealth, '[from] recoil');
    // 			break;
    // 		case 'confusion':
    // 			this.add('-damage', target, target.getHealth, '[from] confusion');
    // 			break;
    // 		default:
    // 			this.add('-damage', target, target.getHealth);
    // 			break;
    // 		}
    // 		if (target.fainted) this.faint(target);
    // 		return damage;
    // 	}
    //
    pub fn direct_damage(
        &mut self,
        mut damage: i32,
        target: Option<(usize, usize)>,
        source: Option<(usize, usize)>,
        effect: Option<&ID>,
    ) -> i32 {
        // JS: if (this.event) { target ||= this.event.target; source ||= this.event.source; effect ||= this.effect; }
        // Extract event context values first to avoid borrow checker issues
        let (event_target, event_source, event_effect) = if let Some(ref event) = self.current_event {
            (event.target, event.source, event.effect.clone())
        } else {
            (None, None, None)
        };

        let target = target.or(event_target);
        let _source = source.or(event_source);  // Not used in current implementation but matches JS signature
        let effect_owned: Option<ID>;
        let effect = if effect.is_none() {
            effect_owned = event_effect;
            effect_owned.as_ref()
        } else {
            effect
        };

        // Get target, handle None case
        let target_pos = match target {
            Some(pos) => pos,
            None => return 0,
        };

        // Check if target has HP
        let (has_hp, _) = if let Some(side) = self.sides.get(target_pos.0) {
            if let Some(pokemon) = side.pokemon.get(target_pos.1) {
                (pokemon.hp > 0, pokemon.is_active)
            } else {
                return 0;
            }
        } else {
            return 0;
        };

        if !has_hp {
            return 0;
        }
        if damage == 0 {
            return 0;
        }

        // Clamp damage to at least 1
        damage = damage.max(1);

        let effect_id = effect.map(|e| e.as_str()).unwrap_or("");

        // Gen 1 special case: Substitute takes confusion and HJK recoil damage
        // JavaScript: if (this.gen <= 1 && this.dex.currentMod !== 'gen1stadium' && ...)
        if self.gen <= 1 && ["confusion", "jumpkick", "highjumpkick"].contains(&effect_id) {
            // Confusion and recoil damage can be countered
            self.last_damage = damage as i32;

            // Check if target has Substitute volatile
            let substitute_id = ID::new("substitute");
            let has_substitute = if let Some(side) = self.sides.get(target_pos.0) {
                if let Some(pokemon) = side.pokemon.get(target_pos.1) {
                    pokemon.volatiles.contains_key(&substitute_id)
                } else {
                    false
                }
            } else {
                false
            };

            if has_substitute {
                // Check if foe has Substitute
                let foe_side = if target_pos.0 == 0 { 1 } else { 0 };
                let foe_has_substitute = if foe_side < self.sides.len() {
                    if let Some(side) = self.sides.get(foe_side) {
                        if let Some(active_idx) = side.active.get(0) {
                            if let Some(poke_idx) = active_idx {
                                if let Some(pokemon) = side.pokemon.get(*poke_idx) {
                                    pokemon.volatiles.contains_key(&substitute_id)
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                };

                let hint = "In Gen 1, if a Pokemon with a Substitute hurts itself due to confusion or Jump Kick/Hi Jump Kick recoil and the target";

                if foe_has_substitute {
                    // Damage foe's substitute
                    // JS: foe.volatiles['substitute'].hp -= damage;
                    // Get foe position
                    let foe_pos = if foe_side < self.sides.len() {
                        if let Some(side) = self.sides.get(foe_side) {
                            if let Some(active_idx) = side.active.get(0) {
                                active_idx.map(|idx| (foe_side, idx))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    if let Some((foe_side_idx, foe_poke_idx)) = foe_pos {
                        // Deduct damage from substitute HP
                        let mut sub_destroyed = false;
                        if let Some(foe_pokemon) = self.sides.get_mut(foe_side_idx)
                            .and_then(|s| s.pokemon.get_mut(foe_poke_idx)) {

                            if let Some(sub_state) = foe_pokemon.volatiles.get_mut(&substitute_id) {
                                // Get current HP from volatile data
                                let current_hp = sub_state.data.get("hp")
                                    .and_then(|v| v.as_i64())
                                    .unwrap_or(0) as i32;

                                let new_hp = current_hp - damage as i32;

                                if new_hp <= 0 {
                                    // JS: foe.volatiles['substitute'].hp <= 0
                                    sub_destroyed = true;
                                } else {
                                    // Update HP
                                    sub_state.data.insert("hp".to_string(), serde_json::json!(new_hp));
                                }
                            }
                        }

                        // Handle substitute destruction or damage log
                        if sub_destroyed {
                            // JS: foe.removeVolatile('substitute'); foe.subFainted = true;
                            if let Some(foe_pokemon) = self.sides.get_mut(foe_side_idx)
                                .and_then(|s| s.pokemon.get_mut(foe_poke_idx)) {
                                foe_pokemon.volatiles.remove(&substitute_id);
                                foe_pokemon.sub_fainted = true;
                            }
                        } else {
                            // JS: this.add('-activate', foe, 'Substitute', '[damage]');
                            let foe_ident = if let Some(foe_pokemon) = self.sides.get(foe_side_idx)
                                .and_then(|s| s.pokemon.get(foe_poke_idx)) {
                                foe_pokemon.get_slot()
                            } else {
                                String::new()
                            };
                            if !foe_ident.is_empty() {
                                self.add_log("-activate", &[&foe_ident, "Substitute", "[damage]"]);
                            }
                        }
                    }

                    self.hint(&format!("{} has a Substitute, the target's Substitute takes the damage.", hint), false, None);
                    return damage;
                } else {
                    self.hint(&format!("{} does not have a Substitute there is no damage dealt.", hint), false, None);
                    return 0;
                }
            }
        }

        // Apply damage using Pokemon's damage method
        let actual_damage = if let Some(side) = self.sides.get_mut(target_pos.0) {
            if let Some(pokemon) = side.pokemon.get_mut(target_pos.1) {
                let old_hp = pokemon.hp;
                pokemon.hp = pokemon.hp.saturating_sub(damage as i32);
                (old_hp - pokemon.hp) as i32
            } else {
                0
            }
        } else {
            0
        };

        // Add damage log message based on effect
        self.add_direct_damage_log(target_pos, effect);

        // Check if target fainted
        if let Some(side) = self.sides.get(target_pos.0) {
            if let Some(pokemon) = side.pokemon.get(target_pos.1) {
                if pokemon.hp == 0 {
                    // JS: if (pokemon.hp <= 0) pokemon.faint(source, effect);
                    let effect_str = effect.map(|e| e.as_str());
                    self.faint(target_pos, source, effect_str);
                }
            }
        }

        actual_damage
    }

    /// Helper to add direct damage log messages
    /// Rust helper method - JavaScript has this logic inline in directDamage() method (battle.ts:2217-2226)
    /// Extracted for borrow checker compatibility
    fn add_direct_damage_log(&mut self, target: (usize, usize), effect: Option<&ID>) {
        let (side_idx, poke_idx) = target;

        // Get target health string
        let health_str = if let Some(side) = self.sides.get(side_idx) {
            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                format!("{}/{}", pokemon.hp, pokemon.maxhp)
            } else {
                return;
            }
        } else {
            return;
        };

        let target_str = format!("p{}a", side_idx + 1);
        let effect_id = effect.map(|e| e.as_str()).unwrap_or("");

        // Special case handling
        match effect_id {
            "strugglerecoil" => {
                self.add_log("-damage", &[&target_str, &health_str, "[from] recoil"]);
            }
            "confusion" => {
                self.add_log("-damage", &[&target_str, &health_str, "[from] confusion"]);
            }
            _ => {
                self.add_log("-damage", &[&target_str, &health_str]);
            }
        }
    }

    /// Heal a Pokemon
    /// Matches JavaScript battle.ts:2231-2274 heal()
    /// Heal HP
    /// Equivalent to battle.ts heal() (battle.ts:2231-2273)
    ///
    // 
    // 	heal(damage: number, target?: Pokemon, source: Pokemon | null = null, effect: 'drain' | Effect | null = null) {
    // 		if (this.event) {
    // 			target ||= this.event.target;
    // 			source ||= this.event.source;
    // 			effect ||= this.effect;
    // 		}
    // 		if (effect === 'drain') effect = this.dex.conditions.getByID(effect as ID);
    // 		if (damage && damage <= 1) damage = 1;
    // 		damage = this.trunc(damage);
    // 		// for things like Liquid Ooze, the Heal event still happens when nothing is healed.
    // 		damage = this.runEvent('TryHeal', target, source, effect, damage);
    // 		if (!damage) return damage;
    // 		if (!target?.hp) return false;
    // 		if (!target.isActive) return false;
    // 		if (target.hp >= target.maxhp) return false;
    // 		const finalDamage = target.heal(damage, source, effect);
    // 		switch (effect?.id) {
    // 		case 'leechseed':
    // 		case 'rest':
    // 			this.add('-heal', target, target.getHealth, '[silent]');
    // 			break;
    // 		case 'drain':
    // 			this.add('-heal', target, target.getHealth, '[from] drain', `[of] ${source}`);
    // 			break;
    // 		case 'wish':
    // 			break;
    // 		case 'zpower':
    // 			this.add('-heal', target, target.getHealth, '[zeffect]');
    // 			break;
    // 		default:
    // 			if (!effect) break;
    // 			if (effect.effectType === 'Move') {
    // 				this.add('-heal', target, target.getHealth);
    // 			} else if (source && source !== target) {
    // 				this.add('-heal', target, target.getHealth, `[from] ${effect.fullname}`, `[of] ${source}`);
    // 			} else {
    // 				this.add('-heal', target, target.getHealth, `[from] ${effect.fullname}`);
    // 			}
    // 			break;
    // 		}
    // 		this.runEvent('Heal', target, source, effect, finalDamage);
    // 		return finalDamage;
    // 	}
    //
    pub fn heal(
        &mut self,
        mut damage: i32,
        target: Option<(usize, usize)>,
        source: Option<(usize, usize)>,
        effect: Option<&ID>,
    ) -> Option<i32> {
        // JS: if (this.event) { target ||= this.event.target; source ||= this.event.source; effect ||= this.effect; }
        // Extract event context values first to avoid borrow checker issues
        let (event_target, event_source, event_effect) = if let Some(ref event) = self.current_event {
            (event.target, event.source, event.effect.clone())
        } else {
            (None, None, None)
        };

        let target = target.or(event_target);
        let source = source.or(event_source);
        let effect_owned: Option<ID>;
        let effect = if effect.is_none() {
            effect_owned = event_effect;
            effect_owned.as_ref()
        } else {
            effect
        };

        // Clamp damage to at least 1
        if damage > 0 && damage <= 1 {
            damage = 1;
        }
        // JavaScript: damage = this.trunc(damage);
        // Already an integer in Rust

        // Get target, handle None case
        let target_pos = match target {
            Some(pos) => pos,
            None => return Some(0),
        };

        // Fire TryHeal event
        // JavaScript: damage = this.runEvent('TryHeal', target, source, effect, damage);
        let event_result = self.run_event("TryHeal", Some(target_pos), source, effect, Some(damage));
        damage = match event_result {
            Some(d) => d,
            None => return None,  // Event cancelled healing
        };

        if damage == 0 {
            return Some(0);
        }

        // Check target validity
        let (side_idx, poke_idx) = target_pos;
        let (has_hp, is_active, at_max_hp) = if let Some(side) = self.sides.get(side_idx) {
            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                (pokemon.hp > 0, pokemon.is_active, pokemon.hp >= pokemon.maxhp)
            } else {
                return None;  // JavaScript returns false
            }
        } else {
            return None;
        };

        if !has_hp {
            return None;
        }
        if !is_active {
            return None;
        }
        if at_max_hp {
            return None;
        }

        // Apply healing using Pokemon's heal method
        let final_damage = if let Some(side) = self.sides.get_mut(side_idx) {
            if let Some(pokemon) = side.pokemon.get_mut(poke_idx) {
                let old_hp = pokemon.hp;
                pokemon.hp = (pokemon.hp + damage as i32).min(pokemon.maxhp);
                (pokemon.hp - old_hp) as i32
            } else {
                0
            }
        } else {
            0
        };

        // Add heal log message
        self.add_heal_log(target_pos, source, effect);

        // Fire Heal event
        // JavaScript: this.runEvent('Heal', target, source, effect, finalDamage);
        self.run_event("Heal", Some(target_pos), source, effect, Some(final_damage));

        Some(final_damage)
    }


    /// Helper to add heal log messages
    /// Rust helper method - JavaScript has this logic inline in heal() method (battle.ts:2246-2268)
    /// Extracted for borrow checker compatibility
    fn add_heal_log(&mut self, target: (usize, usize), source: Option<(usize, usize)>, effect: Option<&ID>) {
        let (side_idx, poke_idx) = target;

        // Get target health string
        let health_str = if let Some(side) = self.sides.get(side_idx) {
            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                format!("{}/{}", pokemon.hp, pokemon.maxhp)
            } else {
                return;
            }
        } else {
            return;
        };

        let target_str = format!("p{}a", side_idx + 1);
        let effect_id = effect.map(|e| e.as_str()).unwrap_or("");

        // Special case handling
        match effect_id {
            "leechseed" | "rest" => {
                self.add_log("-heal", &[&target_str, &health_str, "[silent]"]);
            }
            "drain" => {
                if let Some(src) = source {
                    let src_str = format!("p{}a", src.0 + 1);
                    let of_str = format!("[of] {}", src_str);
                    self.add_log("-heal", &[&target_str, &health_str, "[from] drain", &of_str]);
                } else {
                    self.add_log("-heal", &[&target_str, &health_str, "[from] drain"]);
                }
            }
            "wish" => {
                // Don't add any log for wish
            }
            "zpower" => {
                self.add_log("-heal", &[&target_str, &health_str, "[zeffect]"]);
            }
            "" => {
                // No effect - no log
            }
            _ => {
                // Default heal log
                // JS: if (effect.effectType === 'Move') { this.add('-heal', target, target.getHealth); }
                // JS: else if (source && source !== target) { this.add('-heal', target, target.getHealth, `[from] ${effect.fullname}`, `[of] ${source}`); }
                // JS: else { this.add('-heal', target, target.getHealth, `[from] ${effect.fullname}`); }

                // Check if effect type is Move
                let is_move = effect.map_or(false, |e| self.get_effect_type(e) == "Move");

                if is_move {
                    // Move effects: just show heal without [from] tag
                    self.add_log("-heal", &[&target_str, &health_str]);
                } else if let Some(src) = source {
                    // Non-move effects with source: show [from] effect [of] source
                    let src_str = format!("p{}a", src.0 + 1);
                    let from_str = format!("[from] {}", effect_id);
                    let of_str = format!("[of] {}", src_str);
                    self.add_log("-heal", &[&target_str, &health_str, &from_str, &of_str]);
                } else {
                    // Non-move effects without source: show [from] effect
                    let from_str = format!("[from] {}", effect_id);
                    self.add_log("-heal", &[&target_str, &health_str, &from_str]);
                }
            }
        }
    }


    /// Boost a Pokemon's stats (legacy signature for compatibility)
    /// TODO: This should be migrated to use the new boost_new() method
    /// Apply stat boosts to a Pokemon
    /// Equivalent to battle.ts boost() (battle.ts:1974-2043)
    ///
    // 
    // 	boost(
    // 		boost: SparseBoostsTable, target: Pokemon | null = null, source: Pokemon | null = null,
    // 		effect: Effect | null = null, isSecondary = false, isSelf = false
    // 	) {
    // 		if (this.event) {
    // 			target ||= this.event.target;
    // 			source ||= this.event.source;
    // 			effect ||= this.effect;
    // 		}
    // 		if (!target?.hp) return 0;
    // 		if (!target.isActive) return false;
    // 		if (this.gen > 5 && !target.side.foePokemonLeft()) return false;
    // 		boost = this.runEvent('ChangeBoost', target, source, effect, { ...boost });
    // 		boost = target.getCappedBoost(boost);
    // 		boost = this.runEvent('TryBoost', target, source, effect, { ...boost });
    // 		let success = null;
    // 		let boosted = isSecondary;
    // 		let boostName: BoostID;
    // 		for (boostName in boost) {
    // 			const currentBoost: SparseBoostsTable = {
    // 				[boostName]: boost[boostName],
    // 			};
    // 			let boostBy = target.boostBy(currentBoost);
    // 			let msg = '-boost';
    // 			if (boost[boostName]! < 0 || target.boosts[boostName] === -6) {
    // 				msg = '-unboost';
    // 				boostBy = -boostBy;
    // 			}
    // 			if (boostBy) {
    // 				success = true;
    // 				switch (effect?.id) {
    // 				case 'bellydrum': case 'angerpoint':
    // 					this.add('-setboost', target, 'atk', target.boosts['atk'], '[from] ' + effect.fullname);
    // 					break;
    // 				case 'bellydrum2':
    // 					this.add(msg, target, boostName, boostBy, '[silent]');
    // 					this.hint("In Gen 2, Belly Drum boosts by 2 when it fails.");
    // 					break;
    // 				case 'zpower':
    // 					this.add(msg, target, boostName, boostBy, '[zeffect]');
    // 					break;
    // 				default:
    // 					if (!effect) break;
    // 					if (effect.effectType === 'Move') {
    // 						this.add(msg, target, boostName, boostBy);
    // 					} else if (effect.effectType === 'Item') {
    // 						this.add(msg, target, boostName, boostBy, '[from] item: ' + effect.name);
    // 					} else {
    // 						if (effect.effectType === 'Ability' && !boosted) {
    // 							this.add('-ability', target, effect.name, 'boost');
    // 							boosted = true;
    // 						}
    // 						this.add(msg, target, boostName, boostBy);
    // 					}
    // 					break;
    // 				}
    // 				this.runEvent('AfterEachBoost', target, source, effect, currentBoost);
    // 			} else if (effect?.effectType === 'Ability') {
    // 				if (isSecondary || isSelf) this.add(msg, target, boostName, boostBy);
    // 			} else if (!isSecondary && !isSelf) {
    // 				this.add(msg, target, boostName, boostBy);
    // 			}
    // 		}
    // 		this.runEvent('AfterBoost', target, source, effect, boost);
    // 		if (success) {
    // 			if (Object.values(boost).some(x => x > 0)) target.statsRaisedThisTurn = true;
    // 			if (Object.values(boost).some(x => x < 0)) target.statsLoweredThisTurn = true;
    // 		}
    // 		return success;
    // 	}
    //
    pub fn boost(&mut self, boosts: &[(&str, i8)], target: (usize, usize), source: Option<(usize, usize)>, effect: Option<&str>) -> bool {
        let (target_side, target_idx) = target;

        // JS: if (!target?.hp) return 0;
        // JS: if (!target.isActive) return false;
        {
            let side = match self.sides.get(target_side) {
                Some(s) => s,
                None => return false,
            };
            let pokemon = match side.pokemon.get(target_idx) {
                Some(p) => p,
                None => return false,
            };
            if pokemon.hp == 0 || !pokemon.is_active {
                return false;
            }
        }

        // JS: if (this.gen > 5 && !target.side.foePokemonLeft()) return false;
        if self.gen > 5 {
            let foe_side = if target_side == 0 { 1 } else { 0 };
            if foe_side < self.sides.len() {
                let foe_pokemon_left = self.sides[foe_side].pokemon.iter()
                    .any(|p| p.hp > 0);
                if !foe_pokemon_left {
                    return false;
                }
            }
        }

        // JS: boost = this.runEvent('ChangeBoost', target, source, effect, { ...boost });
        // This event allows abilities/items to modify boost amounts before they're applied
        // Note: Full boost modification would require infrastructure changes to return modified boosts
        // For now, we call the event so abilities can react, even if they can't modify the boost amounts
        let effect_id = effect.map(|s| ID::new(s));
        self.run_event("ChangeBoost", Some(target), source, effect_id.as_ref(), None);

        // JS: boost = target.getCappedBoost(boost);
        // Clamp boosts to [-6, 6] range - done per-stat below

        // JS: boost = this.runEvent('TryBoost', target, source, effect, { ...boost });
        // This event can prevent boosts from being applied (e.g., Clear Body ability)
        // If the event handler needs to cancel boosts, it should set a flag or modify Pokemon state
        // Note: JavaScript can return null to cancel all boosts - we call the event for side effects
        self.run_event("TryBoost", Some(target), source, effect_id.as_ref(), None);

        let mut success = false;
        let mut stats_raised = false;
        let mut stats_lowered = false;

        // Get Pokemon name for logging
        let pokemon_name = if let Some(side) = self.sides.get(target_side) {
            if let Some(pokemon) = side.pokemon.get(target_idx) {
                format!("{}: {}", side.id_str(), pokemon.name)
            } else {
                return false;
            }
        } else {
            return false;
        };

        // JS: for (boostName in boost) { ... }
        for (stat, amount) in boosts {
            if let Some(side) = self.sides.get_mut(target_side) {
                if let Some(pokemon) = side.pokemon.get_mut(target_idx) {
                    let current = match *stat {
                        "atk" => &mut pokemon.boosts.atk,
                        "def" => &mut pokemon.boosts.def,
                        "spa" => &mut pokemon.boosts.spa,
                        "spd" => &mut pokemon.boosts.spd,
                        "spe" => &mut pokemon.boosts.spe,
                        "accuracy" => &mut pokemon.boosts.accuracy,
                        "evasion" => &mut pokemon.boosts.evasion,
                        _ => continue,
                    };

                    let old = *current;
                    // JS: boostBy = target.boostBy(currentBoost);
                    *current = (*current + amount).clamp(-6, 6);
                    let actual = *current - old;

                    // JS: if (boostBy) { success = true; ... }
                    if actual != 0 {
                        success = true;
                        if actual > 0 {
                            stats_raised = true;
                        } else {
                            stats_lowered = true;
                        }

                        let msg = if actual > 0 { "-boost" } else { "-unboost" };
                        let boost_str = actual.abs().to_string();

                        // JS: Special effect handling (bellydrum, angerpoint, zpower, etc.)
                        // For now, simplified logging
                        if let Some(eff) = effect {
                            self.add_log(msg, &[&pokemon_name, stat, &boost_str, &format!("[from] {}", eff)]);
                        } else {
                            self.add_log(msg, &[&pokemon_name, stat, &boost_str]);
                        }

                        // JS: this.runEvent('AfterEachBoost', target, source, effect, currentBoost);
                        self.run_event("AfterEachBoost", Some(target), source, None, None);
                    }
                }
            }
        }

        // JS: this.runEvent('AfterBoost', target, source, effect, boost);
        self.run_event("AfterBoost", Some(target), source, None, None);

        // JS: if (Object.values(boost).some(x => x > 0)) target.statsRaisedThisTurn = true;
        // JS: if (Object.values(boost).some(x => x < 0)) target.statsLoweredThisTurn = true;
        if success {
            if let Some(side) = self.sides.get_mut(target_side) {
                if let Some(pokemon) = side.pokemon.get_mut(target_idx) {
                    if stats_raised {
                        pokemon.stats_raised_this_turn = true;
                    }
                    if stats_lowered {
                        pokemon.stats_lowered_this_turn = true;
                    }
                }
            }
        }

        success
    }

    /// Helper to boost stats from a HashMap
    /// Rust helper method - JavaScript boost() accepts SparseBoostsTable (object with stat names as keys)
    /// This helper converts HashMap format to the Vec<(&str, i8)> format used by boost()
    /// Allows calling boost() when stat boosts are stored in HashMap format
    fn boost_stats(&mut self, target_side: usize, target_idx: usize, boosts_map: &HashMap<String, i32>) {
        // Convert HashMap to Vec of tuples for boost method
        let mut boosts: Vec<(&str, i8)> = Vec::new();
        for (stat, &val) in boosts_map.iter() {
            boosts.push((stat.as_str(), val as i8));
        }

        self.boost(&boosts, (target_side, target_idx), None, None);
    }

    /// Faint a Pokemon
    /// Equivalent to battle.ts faint()
    // 
    // 	faint(pokemon: Pokemon, source?: Pokemon, effect?: Effect) {
    // 		pokemon.faint(source, effect);
    // 	}
    //
    pub fn faint(&mut self, target: (usize, usize), source: Option<(usize, usize)>, effect: Option<&str>) {
        // JS: pokemon.faint(source, effect)
        // JS: battle.faintQueue.push({target: pokemon, source, effect})

        let (target_side, target_idx) = target;
        if let Some(side) = self.sides.get_mut(target_side) {
            if let Some(pokemon) = side.pokemon.get_mut(target_idx) {
                pokemon.faint();

                // In JavaScript, pokemon.faint() sets fainted=true immediately
                // In Rust, we need to set it here as well for compatibility
                pokemon.fainted = true;

                // JS: this.faintQueue.push({target, source, effect});
                let effect_id = effect.map(|e| ID::new(e));
                self.faint_queue.push(FaintData {
                    target,
                    source,
                    effect: effect_id,
                });
            }
        }
    }

    /// Check all active Pokemon for fainting and update their status
    /// Equivalent to battle.ts checkFainted() (battle.ts:2487-2496)
    ///
    // 
    // 	checkFainted() {
    // 		for (const side of this.sides) {
    // 			for (const pokemon of side.active) {
    // 				if (pokemon.fainted) {
    // 					pokemon.status = 'fnt' as ID;
    // 					pokemon.switchFlag = true;
    // 				}
    // 			}
    // 		}
    // 	}
    //
    pub fn check_fainted(&mut self) {
        for side in &mut self.sides {
            for pokemon in &mut side.pokemon {
                if pokemon.is_active && pokemon.fainted {
                    pokemon.status = ID::new("fnt");
                    pokemon.switch_flag = true;
                }
            }
        }
    }

    /// Clamp a value to an integer range
    /// Equivalent to Utils.clampIntRange in battle.ts
    /// JavaScript source from lib/utils.ts:
    //
    // /** Forces num to be an integer (between min and max). */
    // export function clampIntRange(num: any, min?: number, max?: number): number {
    // 	if (typeof num !== 'number') num = 0;
    // 	num = Math.floor(num);
    // 	if (min !== undefined && num < min) num = min;
    // 	if (max !== undefined && num > max) num = max;
    // 	return num;
    // }
    //

    // =========================================================================
    // Priority and Speed Sorting Methods (ported from battle.ts)
    // =========================================================================

    /// Compare priority of two actions/handlers
    /// Equivalent to battle.ts comparePriority()
    /// Returns negative if a comes first, positive if b comes first, 0 if equal
    // TypeScript source:
    // /**
    // 	 * The default sort order for actions, but also event listeners.
    // 	 *
    // 	 * 1. Order, low to high (default last)
    // 	 * 2. Priority, high to low (default 0)
    // 	 * 3. Speed, high to low (default 0)
    // 	 * 4. SubOrder, low to high (default 0)
    // 	 * 5. EffectOrder, low to high (default 0)
    // 	 *
    // 	 * Doesn't reference `this` so doesn't need to be bound.
    // 	 */
    // 	comparePriority(this: void, a: AnyObject, b: AnyObject) {
    // 		return -((b.order || 4294967296) - (a.order || 4294967296)) ||
    // 			((b.priority || 0) - (a.priority || 0)) ||
    // 			((b.speed || 0) - (a.speed || 0)) ||
    // 			-((b.subOrder || 0) - (a.subOrder || 0)) ||
    // 			-((b.effectOrder || 0) - (a.effectOrder || 0)) ||
    // 			0;
    // 	}
    //
    pub fn compare_priority(a: &PriorityItem, b: &PriorityItem) -> std::cmp::Ordering {
        // 1. Order, low to high (default last)
        let order_cmp = a.order.unwrap_or(i32::MAX).cmp(&b.order.unwrap_or(i32::MAX));
        if order_cmp != std::cmp::Ordering::Equal {
            return order_cmp;
        }

        // 2. Priority, high to low (default 0)
        let priority_cmp = b.priority.cmp(&a.priority);
        if priority_cmp != std::cmp::Ordering::Equal {
            return priority_cmp;
        }

        // 3. Speed, high to low (default 0)
        let speed_cmp = b.speed.cmp(&a.speed);
        if speed_cmp != std::cmp::Ordering::Equal {
            return speed_cmp;
        }

        // 4. SubOrder, low to high (default 0)
        let sub_order_cmp = a.sub_order.cmp(&b.sub_order);
        if sub_order_cmp != std::cmp::Ordering::Equal {
            return sub_order_cmp;
        }

        // 5. EffectOrder, low to high (default 0)
        a.effect_order.cmp(&b.effect_order)
    }

    /// Compare for redirect order (abilities like Lightning Rod)
    // 
    // 	static compareRedirectOrder(this: void, a: AnyObject, b: AnyObject) {
    // 		return ((b.priority || 0) - (a.priority || 0)) ||
    // 			((b.speed || 0) - (a.speed || 0)) ||
    // 			((a.effectHolder?.abilityState && b.effectHolder?.abilityState) ?
    // 				-(b.effectHolder.abilityState.effectOrder - a.effectHolder.abilityState.effectOrder) : 0) ||
    // 				0;
    // 	}
    //
    pub fn compare_redirect_order(a: &PriorityItem, b: &PriorityItem) -> std::cmp::Ordering {
        // Priority first
        let priority_cmp = b.priority.cmp(&a.priority);
        if priority_cmp != std::cmp::Ordering::Equal {
            return priority_cmp;
        }

        // Speed second
        let speed_cmp = b.speed.cmp(&a.speed);
        if speed_cmp != std::cmp::Ordering::Equal {
            return speed_cmp;
        }

        // Effect order (for abilities with same priority/speed)
        b.effect_order.cmp(&a.effect_order)
    }

    /// Compare for left-to-right order (hazards, etc.)
    // 
    // 	static compareLeftToRightOrder(this: void, a: AnyObject, b: AnyObject) {
    // 		return -((b.order || 4294967296) - (a.order || 4294967296)) ||
    // 			((b.priority || 0) - (a.priority || 0)) ||
    // 			-((b.index || 0) - (a.index || 0)) ||
    // 			0;
    // 	}
    //
    pub fn compare_left_to_right_order(a: &PriorityItem, b: &PriorityItem) -> std::cmp::Ordering {
        // Order first
        let order_cmp = a.order.unwrap_or(i32::MAX).cmp(&b.order.unwrap_or(i32::MAX));
        if order_cmp != std::cmp::Ordering::Equal {
            return order_cmp;
        }

        // Priority second
        let priority_cmp = b.priority.cmp(&a.priority);
        if priority_cmp != std::cmp::Ordering::Equal {
            return priority_cmp;
        }

        // Index (position) - lower index first
        a.index.cmp(&b.index)
    }

    /// Sort a list, resolving speed ties randomly (the way the games do)
    /// Equivalent to battle.ts speedSort()
    // TypeScript source:
    // /** Sort a list, resolving speed ties the way the games do. */
    // 	speedSort<T extends AnyObject>(list: T[], comparator: (a: T, b: T) => number = this.comparePriority) {
    // 		if (list.length < 2) return;
    // 		let sorted = 0;
    // 		// This is a Selection Sort - not the fastest sort in general, but
    // 		// actually faster than QuickSort for small arrays like the ones
    // 		// `speedSort` is used for.
    // 		// More importantly, it makes it easiest to resolve speed ties
    // 		// properly.
    // 		while (sorted + 1 < list.length) {
    // 			let nextIndexes = [sorted];
    // 			// grab list of next indexes
    // 			for (let i = sorted + 1; i < list.length; i++) {
    // 				const delta = comparator(list[nextIndexes[0]], list[i]);
    // 				if (delta < 0) continue;
    // 				if (delta > 0) nextIndexes = [i];
    // 				if (delta === 0) nextIndexes.push(i);
    // 			}
    // 			// put list of next indexes where they belong
    // 			for (let i = 0; i < nextIndexes.length; i++) {
    // 				const index = nextIndexes[i];
    // 				if (index !== sorted + i) {
    // 					// nextIndexes is guaranteed to be in order, so it will never have
    // 					// been disturbed by an earlier swap
    // 					[list[sorted + i], list[index]] = [list[index], list[sorted + i]];
    // 				}
    // 			}
    // 			if (nextIndexes.length > 1) {
    // 				this.prng.shuffle(list, sorted, sorted + nextIndexes.length);
    // 			}
    // 			sorted += nextIndexes.length;
    // 		}
    // 	}
    //
    pub fn speed_sort<T, F>(&mut self, list: &mut [T], mut get_priority: F)
    where
        F: FnMut(&T) -> PriorityItem,
    {
        if list.len() < 2 {
            return;
        }

        // Selection sort with random tie-breaking
        let mut sorted = 0;
        while sorted + 1 < list.len() {
            let mut next_indexes = vec![sorted];

            // Find the next item(s) with highest priority
            for i in (sorted + 1)..list.len() {
                let cmp = Self::compare_priority(&get_priority(&list[next_indexes[0]]), &get_priority(&list[i]));
                match cmp {
                    std::cmp::Ordering::Less => continue,
                    std::cmp::Ordering::Greater => next_indexes = vec![i],
                    std::cmp::Ordering::Equal => next_indexes.push(i),
                }
            }

            // Put the next items where they belong
            for (offset, &index) in next_indexes.iter().enumerate() {
                if index != sorted + offset {
                    list.swap(sorted + offset, index);
                }
            }

            // If there were ties, shuffle them randomly
            if next_indexes.len() > 1 {
                let end = sorted + next_indexes.len();
                self.shuffle_range(list, sorted, end);
            }

            sorted += next_indexes.len();
        }
    }

    /// Shuffle a range of a slice in place
    /// Rust helper method - JavaScript uses prng.shuffle(list, start, end) inline
    /// This method is called from speed_sort() to shuffle tied items
    /// JavaScript: this.prng.shuffle(list, sorted, sorted + nextIndexes.length);
    fn shuffle_range<T>(&mut self, list: &mut [T], start: usize, end: usize) {
        for i in start..end {
            let j = start + (self.random((end - start) as i32) as usize);
            list.swap(i, j);
        }
    }

    // =========================================================================
    // Pokemon Lookup Methods (ported from battle.ts)
    // =========================================================================

    /// Get a Pokemon by its full name (e.g., "p1: Pikachu")
    /// Equivalent to battle.ts getPokemon()
    // 
    // 	getPokemon(fullname: string | Pokemon) {
    // 		if (typeof fullname !== 'string') fullname = fullname.fullname;
    // 		for (const side of this.sides) {
    // 			for (const pokemon of side.pokemon) {
    // 				if (pokemon.fullname === fullname) return pokemon;
    // 			}
    // 		}
    // 		return null;
    // 	}
    //
    pub fn get_pokemon(&self, fullname: &str) -> Option<(usize, usize, &crate::pokemon::Pokemon)> {
        for (side_idx, side) in self.sides.iter().enumerate() {
            for (poke_idx, pokemon) in side.pokemon.iter().enumerate() {
                let poke_fullname = format!("{}: {}", side.id_str(), pokemon.name);
                if poke_fullname == fullname {
                    return Some((side_idx, poke_idx, pokemon));
                }
            }
        }
        None
    }

    /// Get a Pokemon by its full name (mutable)
    /// Rust helper method - JavaScript getPokemon() can return mutable references directly
    /// Rust requires separate methods for immutable and mutable borrows
    /// Returns position tuple (side_index, pokemon_index) instead of reference for flexibility
    pub fn get_pokemon_mut(&mut self, fullname: &str) -> Option<(usize, usize)> {
        for (side_idx, side) in self.sides.iter().enumerate() {
            for (poke_idx, pokemon) in side.pokemon.iter().enumerate() {
                let poke_fullname = format!("{}: {}", side.id_str(), pokemon.name);
                if poke_fullname == fullname {
                    return Some((side_idx, poke_idx));
                }
            }
        }
        None
    }

    /// Check if a side can switch
    /// Equivalent to battle.ts canSwitch()
    // 
    // 	canSwitch(side: Side) {
    // 		return this.possibleSwitches(side).length;
    // 	}
    //
    pub fn can_switch(&self, side_idx: usize) -> usize {
        self.possible_switches(side_idx).len()
    }

    /// Get a random switchable Pokemon
    /// Equivalent to battle.ts getRandomSwitchable()
    // 
    // 	getRandomSwitchable(side: Side) {
    // 		const canSwitchIn = this.possibleSwitches(side);
    // 		return canSwitchIn.length ? this.sample(canSwitchIn) : null;
    // 	}
    //
    pub fn get_random_switchable(&mut self, side_idx: usize) -> Option<usize> {
        let switches = self.possible_switches(side_idx);
        if switches.is_empty() {
            return None;
        }
        let idx = self.random(switches.len() as i32) as usize;
        Some(switches[idx])
    }

    /// Get list of Pokemon that can switch in
    // 
    // 	private possibleSwitches(side: Side) {
    // 		if (!side.pokemonLeft) return [];
    // 
    // 		const canSwitchIn = [];
    // 		for (let i = side.active.length; i < side.pokemon.length; i++) {
    // 			const pokemon = side.pokemon[i];
    // 			if (!pokemon.fainted) {
    // 				canSwitchIn.push(pokemon);
    // 			}
    // 		}
    // 		return canSwitchIn;
    // 	}
    //
    fn possible_switches(&self, side_idx: usize) -> Vec<usize> {
        if let Some(side) = self.sides.get(side_idx) {
            let mut can_switch_in = Vec::new();
            for (idx, pokemon) in side.pokemon.iter().enumerate() {
                // Skip active Pokemon
                if pokemon.is_active {
                    continue;
                }
                if !pokemon.is_fainted() {
                    can_switch_in.push(idx);
                }
            }
            return can_switch_in;
        }
        Vec::new()
    }

    // =========================================================================
    // Targeting Methods (ported from battle.ts)
    // =========================================================================

    /// Get the location of a target Pokemon from a source Pokemon's perspective
    /// Equivalent to pokemon.ts getLocOf() (pokemon.ts:768-773)
    ///
    /// JS: getLocOf(target: Pokemon) {
    ///   const positionOffset = Math.floor(target.side.n / 2) * target.side.active.length;
    ///   const position = target.position + positionOffset + 1;
    ///   const sameHalf = (this.side.n % 2) === (target.side.n % 2);
    ///   return sameHalf ? -position : position;
    /// }
    pub fn get_loc_of(&self, source: (usize, usize), target: (usize, usize)) -> i32 {
        let (target_side_idx, target_poke_idx) = target;

        if let Some(target_side) = self.sides.get(target_side_idx) {
            if let Some(target_pokemon) = target_side.pokemon.get(target_poke_idx) {
                // const positionOffset = Math.floor(target.side.n / 2) * target.side.active.length;
                let position_offset = (target_side.n / 2) * target_side.active.len();
                // const position = target.position + positionOffset + 1;
                let position = (target_pokemon.position + position_offset + 1) as i32;

                // const sameHalf = (this.side.n % 2) === (target.side.n % 2);
                if let Some(source_side) = self.sides.get(source.0) {
                    let same_half = (source_side.n % 2) == (target_side.n % 2);
                    // return sameHalf ? -position : position;
                    return if same_half { -position } else { position };
                }
            }
        }
        0
    }

    /// Check if a target location is valid
    /// Equivalent to battle.ts validTargetLoc()
    // TypeScript source:
    // /**
    // 	 * Returns whether a proposed target for a move is valid.
    // 	 */
    // 	validTargetLoc(targetLoc: number, source: Pokemon, targetType: string) {
    // 		if (targetLoc === 0) return true;
    // 		const numSlots = this.activePerHalf;
    // 		const sourceLoc = source.getLocOf(source);
    // 		if (Math.abs(targetLoc) > numSlots) return false;
    // 		const isSelf = (sourceLoc === targetLoc);
    // 		const isFoe = (this.gameType === 'freeforall' ? !isSelf : targetLoc > 0);
    // 		const acrossFromTargetLoc = -(numSlots + 1 - targetLoc);
    // 		const isAdjacent = (targetLoc > 0 ?
    // 			Math.abs(acrossFromTargetLoc - sourceLoc) <= 1 :
    // 			Math.abs(targetLoc - sourceLoc) === 1);
    // 
    // 		if (this.gameType === 'freeforall' && targetType === 'adjacentAlly') {
    // 			// moves targeting one ally can instead target foes in Battle Royal
    // 			return isAdjacent;
    // 		}
    // 
    // 		switch (targetType) {
    // 		case 'randomNormal':
    // 		case 'scripted':
    // 		case 'normal':
    // 			return isAdjacent;
    // 		case 'adjacentAlly':
    // 			return isAdjacent && !isFoe;
    // 		case 'adjacentAllyOrSelf':
    // 			return isAdjacent && !isFoe || isSelf;
    // 		case 'adjacentFoe':
    // 			return isAdjacent && isFoe;
    // 		case 'any':
    // 			return !isSelf;
    // 		}
    // 		return false;
    // 	}
    //
    pub fn valid_target_loc(&self, target_loc: i32, source: (usize, usize), target_type: &str) -> bool {
        // JS: if (targetLoc === 0) return true;
        if target_loc == 0 {
            return true;
        }

        // JS: const numSlots = this.activePerHalf;
        let num_slots = self.active_per_half as i32;

        // JS: const sourceLoc = source.getLocOf(source);
        let source_loc = self.get_loc_of(source, source);

        // JS: if (Math.abs(targetLoc) > numSlots) return false;
        if target_loc.abs() > num_slots {
            return false;
        }

        // JS: const isSelf = (sourceLoc === targetLoc);
        let is_self = source_loc == target_loc;

        // JS: const isFoe = (this.gameType === 'freeforall' ? !isSelf : targetLoc > 0);
        let is_foe = if self.game_type == GameType::FreeForAll {
            !is_self
        } else {
            target_loc > 0
        };

        // JS: const acrossFromTargetLoc = -(numSlots + 1 - targetLoc);
        let across_from_target_loc = -(num_slots + 1 - target_loc);

        // JS: const isAdjacent = (targetLoc > 0 ?
        //        Math.abs(acrossFromTargetLoc - sourceLoc) <= 1 :
        //        Math.abs(targetLoc - sourceLoc) === 1);
        let is_adjacent = if target_loc > 0 {
            (across_from_target_loc - source_loc).abs() <= 1
        } else {
            (target_loc - source_loc).abs() == 1
        };

        // JS: if (this.gameType === 'freeforall' && targetType === 'adjacentAlly') {
        //       return isAdjacent;
        //     }
        if self.game_type == GameType::FreeForAll && target_type == "adjacentAlly" {
            return is_adjacent;
        }

        // JS: switch (targetType)
        match target_type {
            "randomNormal" | "scripted" | "normal" => {
                // JS: return isAdjacent;
                is_adjacent
            }
            "adjacentAlly" => {
                // JS: return isAdjacent && !isFoe;
                is_adjacent && !is_foe
            }
            "adjacentAllyOrSelf" => {
                // JS: return isAdjacent && !isFoe || isSelf;
                (is_adjacent && !is_foe) || is_self
            }
            "adjacentFoe" => {
                // JS: return isAdjacent && isFoe;
                is_adjacent && is_foe
            }
            "any" => {
                // JS: return !isSelf;
                !is_self
            }
            _ => {
                // JS: return false;
                false
            }
        }
    }

    /// Check if a target is valid for a move
    /// Equivalent to battle.ts validTarget()
    // 
    // 	validTarget(target: Pokemon, source: Pokemon, targetType: string) {
    // 		return this.validTargetLoc(source.getLocOf(target), source, targetType);
    // 	}
    //
    pub fn valid_target(&self, target: (usize, usize), source: (usize, usize), target_type: &str) -> bool {
        // JS: return this.validTargetLoc(source.getLocOf(target), source, targetType);
        let target_loc = self.get_loc_of(source, target);
        self.valid_target_loc(target_loc, source, target_type)
    }

    /// Get Pokemon at a specific slot location
    /// Get Pokemon at a slot string (e.g., "p1a", "p2b")
    /// Equivalent to battle.ts getAtSlot()
    // 
    // 	getAtSlot(slot: PokemonSlot): Pokemon;
    //
    pub fn get_at_slot(&self, slot: Option<&str>) -> Option<&Pokemon> {
        // JS: if (!slot) return null;
        let slot_str = slot?;

        if slot_str.len() < 3 {
            return None;
        }

        // JS: const side = this.sides[slot.charCodeAt(1) - 49]; // 49 is '1'
        let side_char = slot_str.chars().nth(1)?;
        let side_idx = (side_char as i32).checked_sub(49)? as usize; // 49 is '1'

        // JS: const position = slot.charCodeAt(2) - 97; // 97 is 'a'
        let pos_char = slot_str.chars().nth(2)?;
        let position = (pos_char as i32).checked_sub(97)? as usize; // 97 is 'a'

        // JS: const positionOffset = Math.floor(side.n / 2) * side.active.length;
        // JS: return side.active[position - positionOffset];
        let side = self.sides.get(side_idx)?;
        let position_offset = (side.n / 2) * side.active.len();
        let adjusted_position = position.checked_sub(position_offset)?;

        let poke_idx = side.active.get(adjusted_position)?.as_ref()?;
        side.pokemon.get(*poke_idx)
    }

    // =========================================================================
    // Turn Management Methods (ported from battle.ts)
    // =========================================================================

    /// End the current turn
    /// Equivalent to battle.ts endTurn() (battle.ts:1577-1754)
    ///
    // 
    // 	endTurn() {
    // 		this.turn++;
    // 		this.lastSuccessfulMoveThisTurn = null;
    // 
    // 		const dynamaxEnding: Pokemon[] = [];
    // 		for (const pokemon of this.getAllActive()) {
    // 			if (pokemon.volatiles['dynamax']?.turns === 3) {
    // 				dynamaxEnding.push(pokemon);
    // 			}
    // 		}
    // 		if (dynamaxEnding.length > 1) {
    // 			this.updateSpeed();
    // 			this.speedSort(dynamaxEnding);
    // 		}
    // 		for (const pokemon of dynamaxEnding) {
    // 			pokemon.removeVolatile('dynamax');
    // 		}
    // 
    // 		// Gen 1 partial trapping ends when either Pokemon or a switch in faints to residual damage
    // 		if (this.gen === 1) {
    // 			for (const pokemon of this.getAllActive()) {
    // 				if (pokemon.volatiles['partialtrappinglock']) {
    // 					const target = pokemon.volatiles['partialtrappinglock'].locked;
    // 					if (target.hp <= 0 || !target.volatiles['partiallytrapped']) {
    // 						delete pokemon.volatiles['partialtrappinglock'];
    // 					}
    // 				}
    // 				if (pokemon.volatiles['partiallytrapped']) {
    // 					const source = pokemon.volatiles['partiallytrapped'].source;
    // 					if (source.hp <= 0 || !source.volatiles['partialtrappinglock']) {
    // 						delete pokemon.volatiles['partiallytrapped'];
    // 					}
    // 				}
    // 				if (pokemon.volatiles['fakepartiallytrapped']) {
    // 					const counterpart = pokemon.volatiles['fakepartiallytrapped'].counterpart;
    // 					if (counterpart.hp <= 0 || !counterpart.volatiles['fakepartiallytrapped']) {
    // 						delete pokemon.volatiles['fakepartiallytrapped'];
    // 					}
    // 				}
    // 			}
    // 		}
    // 
    // 		const trappedBySide: boolean[] = [];
    // 		const stalenessBySide: ('internal' | 'external' | undefined)[] = [];
    // 		for (const side of this.sides) {
    // 			let sideTrapped = true;
    // 			let sideStaleness: 'internal' | 'external' | undefined;
    // 			for (const pokemon of side.active) {
    // 				if (!pokemon) continue;
    // 				pokemon.moveThisTurn = '';
    // 				pokemon.newlySwitched = false;
    // 				pokemon.moveLastTurnResult = pokemon.moveThisTurnResult;
    // 				pokemon.moveThisTurnResult = undefined;
    // 				if (this.turn !== 1) {
    // 					pokemon.usedItemThisTurn = false;
    // 					pokemon.statsRaisedThisTurn = false;
    // 					pokemon.statsLoweredThisTurn = false;
    // 					// It shouldn't be possible in a normal battle for a Pokemon to be damaged before turn 1's move selection
    // 					// However, this could be potentially relevant in certain OMs
    // 					pokemon.hurtThisTurn = null;
    // 				}
    // 
    // 				pokemon.maybeDisabled = false;
    // 				pokemon.maybeLocked = false;
    // 				for (const moveSlot of pokemon.moveSlots) {
    // 					moveSlot.disabled = false;
    // 					moveSlot.disabledSource = '';
    // 				}
    // 				this.runEvent('DisableMove', pokemon);
    // 				for (const moveSlot of pokemon.moveSlots) {
    // 					const activeMove = this.dex.getActiveMove(moveSlot.id);
    // 					this.singleEvent('DisableMove', activeMove, null, pokemon);
    // 					if (activeMove.flags['cantusetwice'] && pokemon.lastMove?.id === moveSlot.id) {
    // 						pokemon.disableMove(pokemon.lastMove.id);
    // 					}
    // 				}
    // 
    // 				// If it was an illusion, it's not any more
    // 				if (pokemon.getLastAttackedBy() && this.gen >= 7) pokemon.knownType = true;
    // 
    // 				for (let i = pokemon.attackedBy.length - 1; i >= 0; i--) {
    // 					const attack = pokemon.attackedBy[i];
    // 					if (attack.source.isActive) {
    // 						attack.thisTurn = false;
    // 					} else {
    // 						pokemon.attackedBy.splice(pokemon.attackedBy.indexOf(attack), 1);
    // 					}
    // 				}
    // 
    // 				if (this.gen >= 7 && !pokemon.terastallized) {
    // 					// In Gen 7, the real type of every Pokemon is visible to all players via the bottom screen while making choices
    // 					const seenPokemon = pokemon.illusion || pokemon;
    // 					const realTypeString = seenPokemon.getTypes(true).join('/');
    // 					if (realTypeString !== seenPokemon.apparentType) {
    // 						this.add('-start', pokemon, 'typechange', realTypeString, '[silent]');
    // 						seenPokemon.apparentType = realTypeString;
    // 						if (pokemon.addedType) {
    // 							// The typechange message removes the added type, so put it back
    // 							this.add('-start', pokemon, 'typeadd', pokemon.addedType, '[silent]');
    // 						}
    // 					}
    // 				}
    // 
    // 				pokemon.trapped = pokemon.maybeTrapped = false;
    // 				this.runEvent('TrapPokemon', pokemon);
    // 				if (!pokemon.knownType || this.dex.getImmunity('trapped', pokemon)) {
    // 					this.runEvent('MaybeTrapPokemon', pokemon);
    // 				}
    // 				// canceling switches would leak information
    // 				// if a foe might have a trapping ability
    // 				if (this.gen > 2) {
    // 					for (const source of pokemon.foes()) {
    // 						const species = (source.illusion || source).species;
    // 						if (!species.abilities) continue;
    // 						for (const abilitySlot in species.abilities) {
    // 							const abilityName = species.abilities[abilitySlot as keyof Species['abilities']];
    // 							if (abilityName === source.ability) {
    // 								// pokemon event was already run above so we don't need
    // 								// to run it again.
    // 								continue;
    // 							}
    // 							const ruleTable = this.ruleTable;
    // 							if ((ruleTable.has('+hackmons') || !ruleTable.has('obtainableabilities')) && !this.format.team) {
    // 								// hackmons format
    // 								continue;
    // 							} else if (abilitySlot === 'H' && species.unreleasedHidden) {
    // 								// unreleased hidden ability
    // 								continue;
    // 							}
    // 							const ability = this.dex.abilities.get(abilityName);
    // 							if (ruleTable.has('-ability:' + ability.id)) continue;
    // 							if (pokemon.knownType && !this.dex.getImmunity('trapped', pokemon)) continue;
    // 							this.singleEvent('FoeMaybeTrapPokemon', ability, {}, pokemon, source);
    // 						}
    // 					}
    // 				}
    // 
    // 				if (pokemon.fainted) continue;
    // 
    // 				sideTrapped = sideTrapped && pokemon.trapped;
    // 				const staleness = pokemon.volatileStaleness || pokemon.staleness;
    // 				if (staleness) sideStaleness = sideStaleness === 'external' ? sideStaleness : staleness;
    // 				pokemon.activeTurns++;
    // 			}
    // 			trappedBySide.push(sideTrapped);
    // 			stalenessBySide.push(sideStaleness);
    // 			side.faintedLastTurn = side.faintedThisTurn;
    // 			side.faintedThisTurn = null;
    // 		}
    // 
    // 		if (this.maybeTriggerEndlessBattleClause(trappedBySide, stalenessBySide)) return;
    // 
    // 		if (this.gameType === 'triples' && this.sides.every(side => side.pokemonLeft === 1)) {
    // 			// If both sides have one Pokemon left in triples and they are not adjacent, they are both moved to the center.
    // 			const actives = this.getAllActive();
    // 			if (actives.length > 1 && !actives[0].isAdjacent(actives[1])) {
    // 				this.swapPosition(actives[0], 1, '[silent]');
    // 				this.swapPosition(actives[1], 1, '[silent]');
    // 				this.add('-center');
    // 			}
    // 		}
    // 
    // 		this.add('turn', this.turn);
    // 		if (this.gameType === 'multi') {
    // 			for (const side of this.sides) {
    // 				if (side.canDynamaxNow()) {
    // 					if (this.turn === 1) {
    // 						this.addSplit(side.id, ['-candynamax', side.id]);
    // 					} else {
    // 						this.add('-candynamax', side.id);
    // 					}
    // 				}
    // 			}
    // 		}
    // 		if (this.gen === 2) this.quickClawRoll = this.randomChance(60, 256);
    // 		if (this.gen === 3) this.quickClawRoll = this.randomChance(1, 5);
    // 
    // 		this.makeRequest('move');
    // 	}
    //
    pub fn end_turn(&mut self) {
        self.turn += 1;

        // JS: this.lastSuccessfulMoveThisTurn = null;
        self.last_successful_move_this_turn = None;

        // Dynamax 3-turn removal
        // JS: const dynamaxEnding: Pokemon[] = [];
        // JS: for (const pokemon of this.getAllActive()) {
        // JS:     if (pokemon.volatiles['dynamax']?.turns === 3) {
        // JS:         dynamaxEnding.push(pokemon);
        // JS:     }
        // JS: }
        let dynamax_id = ID::new("dynamax");
        let mut dynamax_ending: Vec<(usize, usize)> = Vec::new();

        for side_idx in 0..self.sides.len() {
            for active_idx in 0..self.sides[side_idx].active.len() {
                if let Some(Some(poke_idx)) = self.sides[side_idx].active.get(active_idx) {
                    if let Some(pokemon) = self.sides[side_idx].pokemon.get(*poke_idx) {
                        // Check if Pokemon has dynamax volatile with turns === 3
                        if let Some(dynamax_state) = pokemon.volatiles.get(&dynamax_id) {
                            let turns = dynamax_state.data.get("turns")
                                .and_then(|v| v.as_i64())
                                .unwrap_or(0);
                            if turns == 3 {
                                dynamax_ending.push((side_idx, *poke_idx));
                            }
                        }
                    }
                }
            }
        }

        // JS: if (dynamaxEnding.length > 1) {
        // JS:     this.updateSpeed();
        // JS:     this.speedSort(dynamaxEnding);
        // JS: }
        if dynamax_ending.len() > 1 {
            // Update speed for all Pokemon ending Dynamax
            for &(side_idx, poke_idx) in &dynamax_ending {
                if let Some(pokemon) = self.sides.get_mut(side_idx)
                    .and_then(|s| s.pokemon.get_mut(poke_idx)) {
                    pokemon.update_speed();
                }
            }

            // Speed sort the Pokemon ending Dynamax
            dynamax_ending.sort_by(|&(side_a, poke_a), &(side_b, poke_b)| {
                let speed_a = self.sides.get(side_a)
                    .and_then(|s| s.pokemon.get(poke_a))
                    .map(|p| p.speed)
                    .unwrap_or(0);
                let speed_b = self.sides.get(side_b)
                    .and_then(|s| s.pokemon.get(poke_b))
                    .map(|p| p.speed)
                    .unwrap_or(0);
                speed_b.cmp(&speed_a) // Higher speed first
            });
        }

        // JS: for (const pokemon of dynamaxEnding) {
        // JS:     pokemon.removeVolatile('dynamax');
        // JS: }
        for (side_idx, poke_idx) in dynamax_ending {
            if let Some(pokemon) = self.sides.get_mut(side_idx)
                .and_then(|s| s.pokemon.get_mut(poke_idx)) {
                pokemon.volatiles.remove(&dynamax_id);
            }
        }

        // Gen 1 partial trapping cleanup
        // JS: if (this.gen === 1) { ... }
        if self.gen == 1 {
            let partialtrappinglock_id = ID::new("partialtrappinglock");
            let partiallytrapped_id = ID::new("partiallytrapped");
            let fakepartiallytrapped_id = ID::new("fakepartiallytrapped");

            // Collect which volatiles need to be removed (to avoid borrow checker issues)
            let mut volatiles_to_remove: Vec<((usize, usize), ID)> = Vec::new();

            for side_idx in 0..self.sides.len() {
                for active_idx in 0..self.sides[side_idx].active.len() {
                    if let Some(Some(poke_idx)) = self.sides[side_idx].active.get(active_idx) {
                        let pos = (side_idx, *poke_idx);

                        // Check partialtrappinglock
                        if let Some(pokemon) = self.sides[side_idx].pokemon.get(*poke_idx) {
                            if let Some(lock_state) = pokemon.volatiles.get(&partialtrappinglock_id) {
                                // JS: const target = pokemon.volatiles['partialtrappinglock'].locked;
                                // The locked target is stored in the volatile's data
                                let should_remove = if let Some(locked_data) = lock_state.data.get("locked") {
                                    // Extract target position (stored as side_idx * 10 + poke_idx)
                                    if let Some(locked_val) = locked_data.as_i64() {
                                        let target_side = (locked_val / 10) as usize;
                                        let target_poke = (locked_val % 10) as usize;

                                        // JS: if (target.hp <= 0 || !target.volatiles['partiallytrapped'])
                                        if let Some(target) = self.sides.get(target_side)
                                            .and_then(|s| s.pokemon.get(target_poke)) {
                                            target.hp <= 0 || !target.volatiles.contains_key(&partiallytrapped_id)
                                        } else {
                                            true // Target doesn't exist, remove
                                        }
                                    } else {
                                        true // Invalid data, remove
                                    }
                                } else {
                                    true // No locked data, remove
                                };

                                if should_remove {
                                    volatiles_to_remove.push((pos, partialtrappinglock_id.clone()));
                                }
                            }

                            // Check partiallytrapped
                            if let Some(trapped_state) = pokemon.volatiles.get(&partiallytrapped_id) {
                                // JS: const source = pokemon.volatiles['partiallytrapped'].source;
                                let should_remove = if let Some(source_data) = trapped_state.data.get("source") {
                                    // Extract source position
                                    if let Some(source_val) = source_data.as_i64() {
                                        let source_side = (source_val / 10) as usize;
                                        let source_poke = (source_val % 10) as usize;

                                        // JS: if (source.hp <= 0 || !source.volatiles['partialtrappinglock'])
                                        if let Some(source) = self.sides.get(source_side)
                                            .and_then(|s| s.pokemon.get(source_poke)) {
                                            source.hp <= 0 || !source.volatiles.contains_key(&partialtrappinglock_id)
                                        } else {
                                            true // Source doesn't exist, remove
                                        }
                                    } else {
                                        true // Invalid data, remove
                                    }
                                } else {
                                    true // No source data, remove
                                };

                                if should_remove {
                                    volatiles_to_remove.push((pos, partiallytrapped_id.clone()));
                                }
                            }

                            // Check fakepartiallytrapped
                            if let Some(fake_state) = pokemon.volatiles.get(&fakepartiallytrapped_id) {
                                // JS: const counterpart = pokemon.volatiles['fakepartiallytrapped'].counterpart;
                                let should_remove = if let Some(counterpart_data) = fake_state.data.get("counterpart") {
                                    // Extract counterpart position
                                    if let Some(counterpart_val) = counterpart_data.as_i64() {
                                        let counterpart_side = (counterpart_val / 10) as usize;
                                        let counterpart_poke = (counterpart_val % 10) as usize;

                                        // JS: if (counterpart.hp <= 0 || !counterpart.volatiles['fakepartiallytrapped'])
                                        if let Some(counterpart) = self.sides.get(counterpart_side)
                                            .and_then(|s| s.pokemon.get(counterpart_poke)) {
                                            counterpart.hp <= 0 || !counterpart.volatiles.contains_key(&fakepartiallytrapped_id)
                                        } else {
                                            true // Counterpart doesn't exist, remove
                                        }
                                    } else {
                                        true // Invalid data, remove
                                    }
                                } else {
                                    true // No counterpart data, remove
                                };

                                if should_remove {
                                    volatiles_to_remove.push((pos, fakepartiallytrapped_id.clone()));
                                }
                            }
                        }
                    }
                }
            }

            // Remove the volatiles
            for ((side_idx, poke_idx), volatile_id) in volatiles_to_remove {
                if let Some(pokemon) = self.sides.get_mut(side_idx)
                    .and_then(|s| s.pokemon.get_mut(poke_idx)) {
                    pokemon.volatiles.remove(&volatile_id);
                }
            }
        }

        // Collect pokemon positions and move slots for DisableMove events (to avoid borrow checker issues)
        let mut pokemon_positions: Vec<(usize, usize)> = Vec::new();
        let mut disable_move_data: Vec<((usize, usize), ID)> = Vec::new();

        // Reset Pokemon turn-specific fields
        for side in &mut self.sides {
            for pokemon in &mut side.pokemon {
                if !pokemon.is_active {
                    continue;
                }

                // JS: pokemon.moveThisTurn = '';
                pokemon.move_this_turn = None;

                // JS: pokemon.newlySwitched = false;
                pokemon.newly_switched = false;

                // JS: pokemon.moveLastTurnResult = pokemon.moveThisTurnResult;
                // JS: pokemon.moveThisTurnResult = undefined;
                pokemon.move_last_turn_result = pokemon.move_this_turn_result;
                pokemon.move_this_turn_result = None;

                if self.turn != 1 {
                    // JS: pokemon.usedItemThisTurn = false;
                    pokemon.used_item_this_turn = false;

                    // JS: pokemon.statsRaisedThisTurn = false;
                    pokemon.stats_raised_this_turn = false;

                    // JS: pokemon.statsLoweredThisTurn = false;
                    pokemon.stats_lowered_this_turn = false;

                    // JS: pokemon.hurtThisTurn = null;
                    pokemon.hurt_this_turn = None;
                }

                // JS: pokemon.maybeDisabled = false;
                pokemon.maybe_disabled = false;

                // JS: pokemon.maybeLocked = false;
                pokemon.maybe_locked = false;

                // JS: for (const moveSlot of pokemon.moveSlots) { moveSlot.disabled = false; }
                // Reset all move slots to not disabled (this happens in Pokemon struct reset)

                // Collect pokemon position for DisableMove event
                let pokemon_pos = (pokemon.side_index, pokemon.position);
                pokemon_positions.push(pokemon_pos);

                // Collect move slot data for later single_event calls
                for move_slot in &pokemon.move_slots {
                    disable_move_data.push((pokemon_pos, move_slot.id.clone()));
                }

                // JS: pokemon.trapped = pokemon.maybeTrapped = false;
                pokemon.trapped = false;
                pokemon.maybe_trapped = false;
            }
        }

        // Call runEvent('DisableMove') for each active pokemon (after the mutable borrow ends)
        // JS: this.runEvent('DisableMove', pokemon);
        // This allows abilities like Assault Vest, Gorilla Tactics, etc. to disable moves
        for pokemon_pos in &pokemon_positions {
            self.run_event("DisableMove", Some(*pokemon_pos), None, None, None);
        }

        // Call singleEvent('DisableMove') for each move (allows move-specific disable logic)
        // JS: for (const moveSlot of pokemon.moveSlots) { this.singleEvent('DisableMove', activeMove, null, pokemon); }
        for (pokemon_pos, move_id) in disable_move_data {
            self.single_event("DisableMove", &move_id, Some(pokemon_pos), None, None);
        }

        // Call TrapPokemon and MaybeTrapPokemon events for each active pokemon
        // JS: this.runEvent('TrapPokemon', pokemon);
        // JS: if (!pokemon.knownType || this.dex.getImmunity('trapped', pokemon)) { this.runEvent('MaybeTrapPokemon', pokemon); }
        for &pokemon_pos in &pokemon_positions {
            // TrapPokemon event - allows moves/abilities to trap pokemon (e.g., Mean Look)
            self.run_event("TrapPokemon", Some(pokemon_pos), None, None, None);

            // MaybeTrapPokemon event - conditional trapping based on type immunity
            // JS: if (!pokemon.knownType || this.dex.getImmunity('trapped', pokemon))
            let should_run_maybe_trap = {
                let pokemon = &self.sides[pokemon_pos.0].pokemon[pokemon_pos.1];
                // Run if type is not known OR if not immune to trapped status
                pokemon.known_type.is_none() || pokemon.run_status_immunity("trapped")
            };

            if should_run_maybe_trap {
                self.run_event("MaybeTrapPokemon", Some(pokemon_pos), None, None, None);
            }
        }

        // Check for foe abilities that might trap pokemon (Gen 3+)
        // JS: for (const source of pokemon.foes()) { ... check species.abilities ... }
        if self.gen >= 3 {
            for &(side_idx, poke_idx) in &pokemon_positions {
                // Get adjacent foes for this pokemon
                let foes = self.adjacent_foes(side_idx, poke_idx);

                for (foe_side_idx, foe_idx) in foes {
                    // TODO: Full implementation requires:
                    // 1. Check species.abilities for all ability slots
                    // 2. Check ruleTable for +hackmons and obtainableabilities
                    // 3. Check for unreleased hidden abilities
                    // 4. Call singleEvent('FoeMaybeTrapPokemon', ability, {}, pokemon, source)
                    // For now, this is a stub to document the requirement

                    // The simplified version just notes that foe abilities could affect trapping
                    // Full implementation would iterate through all possible abilities of the foe's species
                }
            }
        }

        self.add_log("", &[]);
        self.add_log("turn", &[&self.turn.to_string()]);
    }

    /// Main turn loop
    /// Equivalent to battle.ts turnLoop()
    // TypeScript source:
    // /**
    // 	 * Generally called at the beginning of a turn, to go through the
    // 	 * turn one action at a time.
    // 	 *
    // 	 * If there is a mid-turn decision (like U-Turn), this will return
    // 	 * and be called again later to resume the turn.
    // 	 */
    // 	turnLoop() {
    // 		this.add('');
    // 		this.add('t:', Math.floor(Date.now() / 1000));
    // 		if (this.requestState) this.requestState = '';
    // 
    // 		if (!this.midTurn) {
    // 			this.queue.insertChoice({ choice: 'beforeTurn' });
    // 			this.queue.addChoice({ choice: 'residual' });
    // 			this.midTurn = true;
    // 		}
    // 
    // 		let action;
    // 		while ((action = this.queue.shift())) {
    // 			this.runAction(action);
    // 			if (this.requestState || this.ended) return;
    // 		}
    // 
    // 		this.endTurn();
    // 		this.midTurn = false;
    // 		this.queue.clear();
    // 	}
    //
    pub fn turn_loop(&mut self) {
        self.add_log("", &[]);

        // Add timestamp (JS: this.add('t:', Math.floor(Date.now() / 1000)))
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.add_log("t:", &[&timestamp.to_string()]);

        if self.request_state != BattleRequestState::None {
            self.request_state = BattleRequestState::None;
        }

        if !self.mid_turn {
            // Would insert beforeTurn and residual actions into queue
            self.mid_turn = true;
        }

        // Process the action queue
        while let Some(action) = self.queue.shift() {
            self.run_action(&action);
            if self.request_state != BattleRequestState::None || self.ended {
                return;
            }
        }

        self.end_turn();
        self.mid_turn = false;
        self.queue.clear();
    }

    /// Run a single action from the queue
    /// Equivalent to battle.ts runAction()
    // 
    // 	runAction(action: Action) {
    // 		const pokemonOriginalHP = action.pokemon?.hp;
    // 		let residualPokemon: (readonly [Pokemon, number])[] = [];
    // 		// returns whether or not we ended in a callback
    // 		switch (action.choice) {
    // 		case 'start': {
    // 			for (const side of this.sides) {
    // 				if (side.pokemonLeft) side.pokemonLeft = side.pokemon.length;
    // 				this.add('teamsize', side.id, side.pokemon.length);
    // 			}
    // 
    // 			this.add('start');
    // 
    // 			// Change Zacian/Zamazenta into their Crowned formes
    // 			for (const pokemon of this.getAllPokemon()) {
    // 				let rawSpecies: Species | null = null;
    // 				if (pokemon.species.id === 'zacian' && pokemon.item === 'rustedsword') {
    // 					rawSpecies = this.dex.species.get('Zacian-Crowned');
    // 				} else if (pokemon.species.id === 'zamazenta' && pokemon.item === 'rustedshield') {
    // 					rawSpecies = this.dex.species.get('Zamazenta-Crowned');
    // 				}
    // 				if (!rawSpecies) continue;
    // 				const species = pokemon.setSpecies(rawSpecies);
    // 				if (!species) continue;
    // 				pokemon.baseSpecies = rawSpecies;
    // 				pokemon.details = pokemon.getUpdatedDetails();
    // 				pokemon.setAbility(species.abilities['0'], null, null, true);
    // 				pokemon.baseAbility = pokemon.ability;
    // 
    // 				const behemothMove: { [k: string]: string } = {
    // 					'Zacian-Crowned': 'behemothblade', 'Zamazenta-Crowned': 'behemothbash',
    // 				};
    // 				const ironHeadIndex = pokemon.baseMoves.indexOf('ironhead');
    // 				if (ironHeadIndex >= 0) {
    // 					const move = this.dex.moves.get(behemothMove[rawSpecies.name]);
    // 					pokemon.baseMoveSlots[ironHeadIndex] = {
    // 						move: move.name,
    // 						id: move.id,
    // 						pp: move.noPPBoosts ? move.pp : move.pp * 8 / 5,
    // 						maxpp: move.noPPBoosts ? move.pp : move.pp * 8 / 5,
    // 						target: move.target,
    // 						disabled: false,
    // 						disabledSource: '',
    // 						used: false,
    // 					};
    // 					pokemon.moveSlots = pokemon.baseMoveSlots.slice();
    // 				}
    // 			}
    // 
    // 			this.format.onBattleStart?.call(this);
    // 			for (const rule of this.ruleTable.keys()) {
    // 				if ('+*-!'.includes(rule.charAt(0))) continue;
    // 				const subFormat = this.dex.formats.get(rule);
    // 				subFormat.onBattleStart?.call(this);
    // 			}
    // 
    // 			for (const side of this.sides) {
    // 				for (let i = 0; i < side.active.length; i++) {
    // 					if (!side.pokemonLeft) {
    // 						// forfeited before starting
    // 						side.active[i] = side.pokemon[i];
    // 						side.active[i].fainted = true;
    // 						side.active[i].hp = 0;
    // 					} else {
    // 						this.actions.switchIn(side.pokemon[i], i);
    // 					}
    // 				}
    // 			}
    // 			for (const pokemon of this.getAllPokemon()) {
    // 				this.singleEvent('Start', this.dex.conditions.getByID(pokemon.species.id), pokemon.speciesState, pokemon);
    // 			}
    // 			this.midTurn = true;
    // 			break;
    // 		}
    // 
    // 		case 'move':
    // 			if (!action.pokemon.isActive) return false;
    // 			if (action.pokemon.fainted) return false;
    // 			this.actions.runMove(action.move, action.pokemon, action.targetLoc, {
    // 				sourceEffect: action.sourceEffect, zMove: action.zmove,
    // 				maxMove: action.maxMove, originalTarget: action.originalTarget,
    // 			});
    // 			break;
    // 		case 'megaEvo':
    // 			this.actions.runMegaEvo(action.pokemon);
    // 			break;
    // 		case 'megaEvoX':
    // 			this.actions.runMegaEvoX?.(action.pokemon);
    // 			break;
    // 		case 'megaEvoY':
    // 			this.actions.runMegaEvoY?.(action.pokemon);
    // 			break;
    // 		case 'runDynamax':
    // 			action.pokemon.addVolatile('dynamax');
    // 			action.pokemon.side.dynamaxUsed = true;
    // 			if (action.pokemon.side.allySide) action.pokemon.side.allySide.dynamaxUsed = true;
    // 			break;
    // 		case 'terastallize':
    // 			this.actions.terastallize(action.pokemon);
    // 			break;
    // 		case 'beforeTurnMove':
    // 			if (!action.pokemon.isActive) return false;
    // 			if (action.pokemon.fainted) return false;
    // 			this.debug('before turn callback: ' + action.move.id);
    // 			const target = this.getTarget(action.pokemon, action.move, action.targetLoc);
    // 			if (!target) return false;
    // 			if (!action.move.beforeTurnCallback) throw new Error(`beforeTurnMove has no beforeTurnCallback`);
    // 			action.move.beforeTurnCallback.call(this, action.pokemon, target);
    // 			break;
    // 		case 'priorityChargeMove':
    // 			if (!action.pokemon.isActive) return false;
    // 			if (action.pokemon.fainted) return false;
    // 			this.debug('priority charge callback: ' + action.move.id);
    // 			if (!action.move.priorityChargeCallback) throw new Error(`priorityChargeMove has no priorityChargeCallback`);
    // 			action.move.priorityChargeCallback.call(this, action.pokemon);
    // 			break;
    // 
    // 		case 'event':
    // 			this.runEvent(action.event!, action.pokemon);
    // 			break;
    // 		case 'team':
    // 			if (action.index === 0) {
    // 				action.pokemon.side.pokemon = [];
    // 			}
    // 			action.pokemon.side.pokemon.push(action.pokemon);
    // 			action.pokemon.position = action.index;
    // 			// we return here because the update event would crash since there are no active pokemon yet
    // 			return;
    // 
    // 		case 'pass':
    // 			return;
    // 		case 'instaswitch':
    // 		case 'switch':
    // 			if (action.choice === 'switch' && action.pokemon.status) {
    // 				this.singleEvent('CheckShow', this.dex.abilities.getByID('naturalcure' as ID), null, action.pokemon);
    // 			}
    // 			if (this.actions.switchIn(action.target, action.pokemon.position, action.sourceEffect) === 'pursuitfaint') {
    // 				// a pokemon fainted from Pursuit before it could switch
    // 				if (this.gen <= 4) {
    // 					// in gen 2-4, the switch still happens
    // 					this.hint("Previously chosen switches continue in Gen 2-4 after a Pursuit target faints.");
    // 					action.priority = -101;
    // 					this.queue.unshift(action);
    // 					break;
    // 				} else {
    // 					// in gen 5+, the switch is cancelled
    // 					this.hint("A Pokemon can't switch between when it runs out of HP and when it faints");
    // 					break;
    // 				}
    // 			}
    // 			break;
    // 		case 'revivalblessing':
    // 			action.pokemon.side.pokemonLeft++;
    // 			if (action.target.position < action.pokemon.side.active.length) {
    // 				this.queue.addChoice({
    // 					choice: 'instaswitch',
    // 					pokemon: action.target,
    // 					target: action.target,
    // 				});
    // 			}
    // 			action.target.fainted = false;
    // 			action.target.faintQueued = false;
    // 			action.target.subFainted = false;
    // 			action.target.status = '';
    // 			action.target.hp = 1; // Needed so hp functions works
    // 			action.target.sethp(action.target.maxhp / 2);
    // 			this.add('-heal', action.target, action.target.getHealth, '[from] move: Revival Blessing');
    // 			action.pokemon.side.removeSlotCondition(action.pokemon, 'revivalblessing');
    // 			break;
    // 		case 'runSwitch':
    // 			this.actions.runSwitch(action.pokemon);
    // 			break;
    // 		case 'shift':
    // 			if (!action.pokemon.isActive) return false;
    // 			if (action.pokemon.fainted) return false;
    // 			this.swapPosition(action.pokemon, 1);
    // 			break;
    // 
    // 		case 'beforeTurn':
    // 			this.eachEvent('BeforeTurn');
    // 			break;
    // 		case 'residual':
    // 			this.add('');
    // 			this.clearActiveMove(true);
    // 			this.updateSpeed();
    // 			residualPokemon = this.getAllActive().map(pokemon => [pokemon, pokemon.getUndynamaxedHP()] as const);
    // 			this.fieldEvent('Residual');
    // 			if (!this.ended) this.add('upkeep');
    // 			break;
    // 		}
    // 
    // 		// phazing (Roar, etc)
    // 		for (const side of this.sides) {
    // 			for (const pokemon of side.active) {
    // 				if (pokemon.forceSwitchFlag) {
    // 					if (pokemon.hp) this.actions.dragIn(pokemon.side, pokemon.position);
    // 					pokemon.forceSwitchFlag = false;
    // 				}
    // 			}
    // 		}
    // 
    // 		this.clearActiveMove();
    // 
    // 		// fainting
    // 
    // 		this.faintMessages();
    // 		if (this.ended) return true;
    // 
    // 		// switching (fainted pokemon, U-turn, Baton Pass, etc)
    // 
    // 		if (!this.queue.peek() || (this.gen <= 3 && ['move', 'residual'].includes(this.queue.peek()!.choice))) {
    // 			// in gen 3 or earlier, switching in fainted pokemon is done after
    // 			// every move, rather than only at the end of the turn.
    // 			this.checkFainted();
    // 		} else if (['megaEvo', 'megaEvoX', 'megaEvoY'].includes(action.choice) && this.gen === 7) {
    // 			this.eachEvent('Update');
    // 			// In Gen 7, the action order is recalculated for a Pokémon that mega evolves.
    // 			for (const [i, queuedAction] of this.queue.list.entries()) {
    // 				if (queuedAction.pokemon === action.pokemon && queuedAction.choice === 'move') {
    // 					this.queue.list.splice(i, 1);
    // 					queuedAction.mega = 'done';
    // 					this.queue.insertChoice(queuedAction, true);
    // 					break;
    // 				}
    // 			}
    // 			return false;
    // 		} else if (this.queue.peek()?.choice === 'instaswitch') {
    // 			return false;
    // 		}
    // 
    // 		if (this.gen >= 5 && action.choice !== 'start') {
    // 			this.eachEvent('Update');
    // 			for (const [pokemon, originalHP] of residualPokemon) {
    // 				const maxhp = pokemon.getUndynamaxedHP(pokemon.maxhp);
    // 				if (pokemon.hp && pokemon.getUndynamaxedHP() <= maxhp / 2 && originalHP > maxhp / 2) {
    // 					this.runEvent('EmergencyExit', pokemon);
    // 				}
    // 			}
    // 		}
    // 
    // 		if (action.choice === 'runSwitch') {
    // 			const pokemon = action.pokemon;
    // 			if (pokemon.hp && pokemon.hp <= pokemon.maxhp / 2 && pokemonOriginalHP! > pokemon.maxhp / 2) {
    // 				this.runEvent('EmergencyExit', pokemon);
    // 			}
    // 		}
    // 
    // 		const switches = this.sides.map(
    // 			side => side.active.some(pokemon => pokemon && !!pokemon.switchFlag)
    // 		);
    // 
    // 		for (let i = 0; i < this.sides.length; i++) {
    // 			let reviveSwitch = false; // Used to ignore the fake switch for Revival Blessing
    // 			if (switches[i] && !this.canSwitch(this.sides[i])) {
    // 				for (const pokemon of this.sides[i].active) {
    // 					if (this.sides[i].slotConditions[pokemon.position]['revivalblessing']) {
    // 						reviveSwitch = true;
    // 						continue;
    // 					}
    // 					pokemon.switchFlag = false;
    // 				}
    // 				if (!reviveSwitch) switches[i] = false;
    // 			} else if (switches[i]) {
    // 				for (const pokemon of this.sides[i].active) {
    // 					if (
    // 						pokemon.hp && pokemon.switchFlag && pokemon.switchFlag !== 'revivalblessing' &&
    // 						!pokemon.skipBeforeSwitchOutEventFlag
    // 					) {
    // 						this.runEvent('BeforeSwitchOut', pokemon);
    // 						pokemon.skipBeforeSwitchOutEventFlag = true;
    // 						this.faintMessages(); // Pokemon may have fainted in BeforeSwitchOut
    // 						if (this.ended) return true;
    // 						if (pokemon.fainted) {
    // 							switches[i] = this.sides[i].active.some(sidePokemon => sidePokemon && !!sidePokemon.switchFlag);
    // 						}
    // 					}
    // 				}
    // 			}
    // 		}
    // 
    // 		for (const playerSwitch of switches) {
    // 			if (playerSwitch) {
    // 				this.makeRequest('switch');
    // 				return true;
    // 			}
    // 		}
    // 
    // 		if (this.gen < 5) this.eachEvent('Update');
    // 
    // 		if (this.gen >= 8 && (this.queue.peek()?.choice === 'move' || this.queue.peek()?.choice === 'runDynamax')) {
    // 			// In gen 8, speed is updated dynamically so update the queue's speed properties and sort it.
    // 			this.updateSpeed();
    // 			for (const queueAction of this.queue.list) {
    // 				if (queueAction.pokemon) this.getActionSpeed(queueAction);
    // 			}
    // 			this.queue.sort();
    // 		}
    // 
    // 		return false;
    // 	}
    //
    pub fn run_action(&mut self, action: &crate::battle_queue::Action) {
        use crate::battle_queue::{Action, FieldActionType};

        match action {
            Action::Move(move_action) => {
                let side_idx = move_action.side_index;
                let poke_idx = move_action.pokemon_index;
                let move_id = &move_action.move_id;
                let target_loc = move_action.target_loc;

                // Check if Pokemon can still act
                if let Some(side) = self.sides.get(side_idx) {
                    if let Some(pokemon) = side.pokemon.get(poke_idx) {
                        if pokemon.is_fainted() {
                            return;
                        }
                    } else {
                        return;
                    }
                } else {
                    return;
                }

                self.run_move(side_idx, poke_idx, move_id, target_loc);
            }
            Action::Switch(switch_action) => {
                let side_idx = switch_action.side_index;
                let poke_idx = switch_action.pokemon_index;
                let target = switch_action.target_index;

                self.do_switch(side_idx, poke_idx, target);
            }
            Action::Field(field_action) => {
                match field_action.choice {
                    FieldActionType::Residual => {
                        self.run_residual();
                    }
                    FieldActionType::BeforeTurn => {
                        // JS: this.eachEvent('BeforeTurn');
                        self.each_event("BeforeTurn", None);
                    }
                    FieldActionType::Start => {
                        // JS: for (const side of this.sides) { if (side.pokemonLeft) side.pokemonLeft = side.pokemon.length; this.add('teamsize', side.id, side.pokemon.length); }
                        for side_idx in 0..self.sides.len() {
                            let team_size = self.sides[side_idx].pokemon.len();
                            if self.sides[side_idx].pokemon_left > 0 {
                                self.sides[side_idx].pokemon_left = team_size;
                            }
                            let side_id = format!("p{}", side_idx + 1);
                            self.add_log("teamsize", &[&side_id, &team_size.to_string()]);
                        }

                        // JS: this.add('start');
                        self.add_log("start", &[]);

                        // TODO: Zacian/Zamazenta forme changes (requires species transformation logic)
                        // JS: Change Zacian/Zamazenta into their Crowned formes
                        // This requires:
                        // - Species data lookup (dex.species.get)
                        // - pokemon.setSpecies()
                        // - pokemon.setAbility()
                        // - Move slot modification (behemothblade/behemothbash replacement)

                        // TODO: Format callbacks (requires format infrastructure)
                        // JS: this.format.onBattleStart?.call(this);
                        // JavaScript formats can have onBattleStart callbacks
                        // These cannot be deserialized from JSON - must be registered separately
                        // For now, emit an event that format-specific code can hook into
                        self.run_event("BattleStart", None, None, None, None);

                        // JS: for (const rule of this.ruleTable.keys()) { ... }
                        if let Some(ref rule_table) = self.rule_table {
                            let rule_keys: Vec<String> = rule_table.keys()
                                .map(|s| s.clone())
                                .collect();

                            for rule in rule_keys {
                                // Skip rules starting with +, *, -, !
                                if let Some(first_char) = rule.chars().next() {
                                    if first_char == '+' || first_char == '*' || first_char == '-' || first_char == '!' {
                                        continue;
                                    }
                                }

                                // JS: const subFormat = this.dex.formats.get(rule);
                                // JS: subFormat.onBattleStart?.call(this);
                                // Emit event for rule-specific battle start hooks
                                self.run_event(&format!("RuleBattleStart:{}", rule), None, None, None, None);
                            }
                        }

                        // JS: for (const side of this.sides) { for (let i = 0; i < side.active.length; i++) { this.actions.switchIn(side.pokemon[i], i); } }
                        // Switch in all active Pokemon at battle start
                        for side_idx in 0..self.sides.len() {
                            let active_length = self.sides[side_idx].active.len();
                            for i in 0..active_length {
                                // Switch in the ith Pokemon from the team into the ith active position
                                // JS: this.actions.switchIn(side.pokemon[i], i);
                                self.switch_in(side_idx, i, i, None, false);
                            }
                        }

                        // JS: for (const pokemon of this.getAllPokemon()) { this.singleEvent('Start', ...); }
                        // Call Start event for each Pokemon's species
                        for side_idx in 0..self.sides.len() {
                            for poke_idx in 0..self.sides[side_idx].pokemon.len() {
                                let species_id = self.sides[side_idx].pokemon[poke_idx].species_id.clone();
                                self.single_event("Start", &species_id, Some((side_idx, poke_idx)), None, None);
                            }
                        }

                        // JS: this.midTurn = true;
                        self.mid_turn = true;
                    }
                    FieldActionType::Pass => {
                        // Pass action - do nothing
                    }
                }
            }
            Action::Team(_) => {
                // Team preview action handled elsewhere
            }
            Action::Pokemon(_) => {
                // Pokemon actions (mega evo, terastallize, etc.)
            }
        }
    }

    /// Check if all choices are done
    /// Equivalent to battle.ts allChoicesDone()
    /// Check if all players have made their choices
    /// Equivalent to battle.ts allChoicesDone() (battle.ts:3059-3068)
    ///
    // TypeScript source:
    // /**
    // 	 * returns true if both decisions are complete
    // 	 */
    // 	allChoicesDone() {
    // 		let totalActions = 0;
    // 		for (const side of this.sides) {
    // 			if (side.isChoiceDone()) {
    // 				if (!this.supportCancel) side.choice.cantUndo = true;
    // 				totalActions++;
    // 			}
    // 		}
    // 		return totalActions >= this.sides.length;
    // 	}
    //
    pub fn all_choices_done(&mut self) -> bool {
        // JS: let totalActions = 0;
        let mut total_actions = 0;

        // Get picked_team_size from rule_table if available
        let picked_team_size = self.rule_table.as_ref().and_then(|rt| rt.picked_team_size);

        // JS: for (const side of this.sides)
        for side in &mut self.sides {
            // JS: if (side.isChoiceDone())
            if side.is_choice_done(picked_team_size) {
                // JS: if (!this.supportCancel) side.choice.cantUndo = true;
                if !self.support_cancel {
                    side.choice.cant_undo = true;
                }

                // JS: totalActions++;
                total_actions += 1;
            }
        }

        // JS: return totalActions >= this.sides.length;
        total_actions >= self.sides.len()
    }

    // =========================================================================
    // Miscellaneous Methods (ported from battle.ts)
    // =========================================================================

    /// Check if move makes contact
    /// Equivalent to battle.ts checkMoveMakesContact()
    // 
    // 	checkMoveMakesContact(move: ActiveMove, attacker: Pokemon, defender: Pokemon, announcePads = false) {
    // 		if (move.flags['contact'] && attacker.hasItem('protectivepads')) {
    // 			if (announcePads) {
    // 				this.add('-activate', defender, this.effect.fullname);
    // 				this.add('-activate', attacker, 'item: Protective Pads');
    // 			}
    // 			return false;
    // 		}
    // 		return !!move.flags['contact'];
    // 	}
    //
    pub fn check_move_makes_contact(&self, move_id: &ID, attacker: (usize, usize)) -> bool {
        // Check if move has contact flag
        if let Some(move_def) = self.dex.get_move(move_id.as_str()) {
            if !move_def.flags.contains_key("contact") {
                return false;
            }

            // JS: if (move.flags['contact'] && attacker.hasItem('protectivepads'))
            let (side_idx, poke_idx) = attacker;
            if let Some(side) = self.sides.get(side_idx) {
                if let Some(pokemon) = side.pokemon.get(poke_idx) {
                    // Protective Pads prevents contact
                    if pokemon.item.as_str() == "protectivepads" {
                        return false;
                    }
                }
            }

            return true;
        }
        false
    }

    /// Get action speed for sorting the action queue
    /// Equivalent to battle.ts getActionSpeed() (battle.ts:2590-2627)
    ///
    // 
    // 	getActionSpeed(action: AnyObject) {
    // 		if (action.choice === 'move') {
    // 			let move = action.move;
    // 			if (action.zmove) {
    // 				const zMoveName = this.actions.getZMove(action.move, action.pokemon, true);
    // 				if (zMoveName) {
    // 					const zMove = this.dex.getActiveMove(zMoveName);
    // 					if (zMove.exists && zMove.isZ) {
    // 						move = zMove;
    // 					}
    // 				}
    // 			}
    // 			if (action.maxMove) {
    // 				const maxMoveName = this.actions.getMaxMove(action.maxMove, action.pokemon);
    // 				if (maxMoveName) {
    // 					const maxMove = this.actions.getActiveMaxMove(action.move, action.pokemon);
    // 					if (maxMove.exists && maxMove.isMax) {
    // 						move = maxMove;
    // 					}
    // 				}
    // 			}
    // 			// take priority from the base move, so abilities like Prankster only apply once
    // 			// (instead of compounding every time `getActionSpeed` is called)
    // 			let priority = this.dex.moves.get(move.id).priority;
    // 			// Grassy Glide priority
    // 			priority = this.singleEvent('ModifyPriority', move, null, action.pokemon, null, null, priority);
    // 			priority = this.runEvent('ModifyPriority', action.pokemon, null, move, priority);
    // 			action.priority = priority + action.fractionalPriority;
    // 			// In Gen 6, Quick Guard blocks moves with artificially enhanced priority.
    // 			if (this.gen > 5) action.move.priority = priority;
    // 		}
    // 
    // 		if (!action.pokemon) {
    // 			action.speed = 1;
    // 		} else {
    // 			action.speed = action.pokemon.getActionSpeed();
    // 		}
    // 	}
    //
    pub fn get_action_speed(&mut self, action: &mut crate::battle_queue::Action) {
        use crate::battle_queue::Action;

        match action {
            Action::Move(ref mut move_action) => {
                // JS: if (action.choice === 'move')

                // Get the move (considering Z-Move/Max Move transformations)
                let mut move_id = move_action.move_id.clone();

                // JS: if (action.zmove) {
                //         const zMoveName = this.actions.getZMove(action.move, action.pokemon, true);
                //         if (zMoveName) {
                //             const zMove = this.dex.getActiveMove(zMoveName);
                //             if (zMove.exists && zMove.isZ) {
                //                 move = zMove;
                //             }
                //         }
                //     }
                if let Some(ref zmove_name) = move_action.zmove {
                    // Z-Move transformation: get the active Z-Move
                    if let Some(z_move) = self.dex.get_active_move(zmove_name) {
                        if z_move.is_z {
                            // Use Z-Move for priority calculation
                            move_id = z_move.id;
                        }
                    }
                }

                // JS: if (action.maxMove) {
                //         const maxMoveName = this.actions.getMaxMove(action.maxMove, action.pokemon);
                //         if (maxMoveName) {
                //             const maxMove = this.actions.getActiveMaxMove(action.move, action.pokemon);
                //             if (maxMove.exists && maxMove.isMax) {
                //                 move = maxMove;
                //             }
                //         }
                //     }
                if let Some(ref max_move_name) = move_action.max_move {
                    // Max Move transformation: get the active Max Move
                    if let Some(max_move) = self.dex.get_active_move(max_move_name) {
                        if max_move.is_max {
                            // Use Max Move for priority calculation
                            move_id = max_move.id;
                        }
                    }
                }

                // JS: let priority = this.dex.moves.get(move.id).priority;
                // Get base priority from move data in Dex (from potentially transformed move)
                let mut priority = if let Some(move_data) = self.dex.get_move_by_id(&move_id) {
                    move_data.priority
                } else {
                    // Fallback to action priority if move not found in Dex
                    move_action.priority
                };

                // JS: priority = this.singleEvent('ModifyPriority', move, null, action.pokemon, null, null, priority);
                // Allows move-specific priority modification (e.g., Grassy Glide in Grassy Terrain)
                let move_id_ref = &move_action.move_id.clone();
                let pokemon_pos = (move_action.side_index, move_action.pokemon_index);
                let result = self.single_event("ModifyPriority", move_id_ref, Some(pokemon_pos), None, None);
                if let Some(new_priority) = result.number() {
                    priority = new_priority as i8;
                }

                // JS: priority = this.runEvent('ModifyPriority', action.pokemon, null, move, priority);
                // Allows ability/item-based priority modification (e.g., Prankster, Quick Claw)
                let effect_id = move_action.move_id.clone();
                let relay_result = self.run_event(
                    "ModifyPriority",
                    Some(pokemon_pos),
                    None,
                    Some(&effect_id),
                    Some(priority as i32),
                );
                if let Some(modified_priority) = relay_result {
                    priority = modified_priority as i8;
                }

                // JS: action.priority = priority + action.fractionalPriority;
                // Note: fractionalPriority is already applied when action is created
                // Just ensure priority is set correctly
                move_action.priority = priority;

                // JS: if (this.gen > 5) action.move.priority = priority;
                // In Gen 6+, Quick Guard checks if move priority was artificially enhanced
                // Store the modified priority value for the move itself
                if self.gen > 5 {
                    move_action.move_priority_modified = Some(priority);
                }

                // JS: action.speed = action.pokemon.getActionSpeed();
                let pokemon_speed = self.get_pokemon_action_speed(move_action.side_index, move_action.pokemon_index);
                move_action.speed = pokemon_speed;
            }
            Action::Switch(ref mut switch_action) => {
                // JS: if (!action.pokemon) { action.speed = 1; }
                // For switches, get the pokemon's speed
                let pokemon_speed = self.get_pokemon_action_speed(switch_action.side_index, switch_action.pokemon_index);
                switch_action.speed = pokemon_speed;
            }
            Action::Pokemon(ref mut poke_action) => {
                // Get pokemon speed for pokemon actions
                let pokemon_speed = self.get_pokemon_action_speed(poke_action.side_index, poke_action.pokemon_index);
                poke_action.speed = pokemon_speed;
            }
            _ => {
                // Field and Team actions don't have speed
            }
        }
    }

    /// Get a Pokemon's action speed (called by pokemon.getActionSpeed() in JS)
    /// This is the helper method for getting base Pokemon speed
    fn get_pokemon_action_speed(&self, side_idx: usize, poke_idx: usize) -> i32 {
        if let Some(side) = self.sides.get(side_idx) {
            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                // Apply speed boosts
                let base_speed = pokemon.stored_stats.spe as i32;
                let stage = pokemon.boosts.spe;
                let multiplier = if stage >= 0 {
                    (2 + stage as i32) as f64 / 2.0
                } else {
                    2.0 / (2 + (-stage) as i32) as f64
                };
                return (base_speed as f64 * multiplier) as i32;
            }
        }
        0
    }

    /// Swap position of a Pokemon to a new position on their side
    /// Equivalent to battle.ts swapPosition()
    // 
    // 	swapPosition(pokemon: Pokemon, newPosition: number, attributes?: string) {
    // 		if (newPosition >= pokemon.side.active.length) {
    // 			throw new Error("Invalid swap position");
    // 		}
    // 		const target = pokemon.side.active[newPosition];
    // 		if (newPosition !== 1 && (!target || target.fainted)) return false;
    // 
    // 		this.add('swap', pokemon, newPosition, attributes || '');
    // 
    // 		const side = pokemon.side;
    // 		side.pokemon[pokemon.position] = target;
    // 		side.pokemon[newPosition] = pokemon;
    // 		side.active[pokemon.position] = side.pokemon[pokemon.position];
    // 		side.active[newPosition] = side.pokemon[newPosition];
    // 		if (target) target.position = pokemon.position;
    // 		pokemon.position = newPosition;
    // 		this.runEvent('Swap', target, pokemon);
    // 		this.runEvent('Swap', pokemon, target);
    // 		return true;
    // 	}
    //
    pub fn swap_position(&mut self, pokemon: (usize, usize), new_position: usize, attributes: Option<&str>) -> bool {
        let (side_idx, poke_idx) = pokemon;

        // JS: if (newPosition >= pokemon.side.active.length)
        if side_idx >= self.sides.len() {
            return false;
        }

        let active_len = self.active_per_half;
        if new_position >= active_len {
            return false; // throw new Error("Invalid swap position");
        }

        // Get pokemon's current position
        let current_pos = if let Some(side) = self.sides.get(side_idx) {
            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                pokemon.position
            } else {
                return false;
            }
        } else {
            return false;
        };

        // JS: const target = pokemon.side.active[newPosition];
        // JS: if (newPosition !== 1 && (!target || target.fainted)) return false;
        let target_idx = if let Some(side) = self.sides.get(side_idx) {
            if let Some(target_active) = side.active.get(new_position) {
                if new_position != 1 {
                    if let Some(&idx) = target_active.as_ref() {
                        if let Some(target_poke) = side.pokemon.get(idx) {
                            if target_poke.fainted {
                                return false;
                            }
                        }
                        Some(idx)
                    } else {
                        return false;
                    }
                } else {
                    target_active.clone()
                }
            } else {
                return false;
            }
        } else {
            return false;
        };

        // Log the swap
        let pokemon_str = if let Some(side) = self.sides.get(side_idx) {
            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                format!("{}: {}", side.id_str(), pokemon.name)
            } else {
                return false;
            }
        } else {
            return false;
        };

        self.add("swap", &[Arg::String(pokemon_str), Arg::String(new_position.to_string()), Arg::Str(attributes.unwrap_or(""))]);

        // Perform the swap
        // JS: side.pokemon[pokemon.position] = target;
        // JS: side.pokemon[newPosition] = pokemon;
        // JS: side.active[pokemon.position] = side.pokemon[pokemon.position];
        // JS: side.active[newPosition] = side.pokemon[newPosition];
        if let Some(side) = self.sides.get_mut(side_idx) {
            // Swap in active array
            side.active.swap(current_pos, new_position);

            // Update positions
            if let Some(pokemon) = side.pokemon.get_mut(poke_idx) {
                pokemon.position = new_position;
            }
            if let Some(target_idx) = target_idx {
                if let Some(target) = side.pokemon.get_mut(target_idx) {
                    target.position = current_pos;
                }
            }
        }

        // JS: this.runEvent('Swap', target, pokemon);
        // JS: this.runEvent('Swap', pokemon, target);
        // Fire Swap events for both pokemon involved
        if let Some(target_idx) = target_idx {
            // First event: Swap on target (source=pokemon)
            self.run_event("Swap", Some((side_idx, target_idx)), Some((side_idx, poke_idx)), None, None);
            // Second event: Swap on pokemon (source=target)
            self.run_event("Swap", Some((side_idx, poke_idx)), Some((side_idx, target_idx)), None, None);
        }

        true
    }

    /// Get move category, defaulting to "Physical" if move not found
    /// Equivalent to battle.ts getCategory() (battle.ts:2350-2352)
    ///
    // 
    // 	getCategory(move: string | Move) {
    // 		return this.dex.moves.get(move).category || 'Physical';
    // 	}
    //
    pub fn get_category(&self, move_id: &ID) -> String {
        if let Some(move_def) = self.dex.get_move(move_id.as_str()) {
            return move_def.category.clone();
        }
        "Physical".to_string()
    }

    /// Clear request state
    /// Equivalent to battle.ts clearRequest() (battle.ts:1364-1370)
    // 
    // 	clearRequest() {
    // 		this.requestState = '';
    // 		for (const side of this.sides) {
    // 			side.activeRequest = null;
    // 			side.clearChoice();
    // 		}
    // 	}
    //
    pub fn clear_request(&mut self) {
        // JavaScript: this.requestState = '';
        self.request_state = BattleRequestState::None;

        // JavaScript: for (const side of this.sides) { side.activeRequest = null; side.clearChoice(); }
        for side in &mut self.sides {
            side.request_state = RequestState::None;
            side.choice = Choice::new();  // clearChoice()
        }
    }

    /// Make a request for player input
    /// Equivalent to battle.ts makeRequest()
    ///
    // 
    // 	makeRequest(type?: RequestState) {
    // 		if (type) {
    // 			this.requestState = type;
    // 			for (const side of this.sides) {
    // 				side.clearChoice();
    // 			}
    // 		} else {
    // 			type = this.requestState;
    // 		}
    // 
    // 		for (const side of this.sides) {
    // 			side.activeRequest = null;
    // 		}
    // 
    // 		if (type === 'teampreview') {
    // 			// `pickedTeamSize = 6` means the format wants the user to select
    // 			// the entire team order, unlike `pickedTeamSize = undefined` which
    // 			// will only ask the user to select their lead(s).
    // 			const pickedTeamSize = this.ruleTable.pickedTeamSize;
    // 			this.add(`teampreview${pickedTeamSize ? `|${pickedTeamSize}` : ''}`);
    // 		}
    // 
    // 		const requests = this.getRequests(type);
    // 		for (let i = 0; i < this.sides.length; i++) {
    // 			this.sides[i].activeRequest = requests[i];
    // 		}
    // 		this.sentRequests = false;
    // 
    // 		if (this.sides.every(side => side.isChoiceDone())) {
    // 			throw new Error(`Choices are done immediately after a request`);
    // 		}
    // 	}
    //
    pub fn make_request(&mut self, request_type: Option<BattleRequestState>) {
        // JS: if (type) { this.requestState = type; ... } else { type = this.requestState; }
        let req_type = if let Some(rt) = request_type {
            self.request_state = rt;
            // JS: for (const side of this.sides) { side.clearChoice(); }
            for side in &mut self.sides {
                side.clear_choice(rt);
            }
            rt
        } else {
            self.request_state
        };

        // JS: for (const side of this.sides) { side.activeRequest = null; }
        for side in &mut self.sides {
            side.active_request = None;
        }

        // JS: if (type === 'teampreview') { ... this.add(`teampreview${pickedTeamSize ? `|${pickedTeamSize}` : ''}`); }
        if matches!(req_type, BattleRequestState::TeamPreview) {
            // JS: const pickedTeamSize = this.ruleTable.pickedTeamSize;
            // JS: this.add(`teampreview${pickedTeamSize ? `|${pickedTeamSize}` : ''}`);
            if let Some(ref rule_table) = self.rule_table {
                if let Some(picked_team_size) = rule_table.picked_team_size {
                    self.add("-", &[Arg::Str("teampreview"), Arg::String(picked_team_size.to_string())]);
                } else {
                    self.add("-", &[Arg::Str("teampreview")]);
                }
            } else {
                self.add("-", &[Arg::Str("teampreview")]);
            }
        }

        // JS: const requests = this.getRequests(type);
        // JS: for (let i = 0; i < this.sides.length; i++) { this.sides[i].activeRequest = requests[i]; }
        // TODO: Implement full getRequests() logic and activeRequest assignment
        // For now, just set request state
        let side_request = match req_type {
            BattleRequestState::Move => RequestState::Move,
            BattleRequestState::Switch => RequestState::Switch,
            BattleRequestState::TeamPreview => RequestState::TeamPreview,
            BattleRequestState::None => RequestState::None,
        };
        for side in &mut self.sides {
            side.request_state = side_request;
        }

        // JS: this.sentRequests = false;
        self.sent_requests = false;

        // JS: if (this.sides.every(side => side.isChoiceDone())) { throw new Error(...); }
        // Safety check to prevent infinite loops
        let picked_team_size = self.rule_table.as_ref().and_then(|rt| rt.picked_team_size);
        if self.sides.iter().all(|side| side.is_choice_done(picked_team_size)) {
            panic!("Choices are done immediately after a request");
        }
    }

    /// Check and trigger Endless Battle Clause
    /// Equivalent to battle.ts maybeTriggerEndlessBattleClause() (battle.ts:1757-1856)
    ///
    // 
    // 	maybeTriggerEndlessBattleClause(
    // 		trappedBySide: boolean[], stalenessBySide: ('internal' | 'external' | undefined)[]
    // 	) {
    // 		// Gen 1 Endless Battle Clause triggers
    // 		// These are checked before the 100 turn minimum as the battle cannot progress if they are true
    // 		if (this.gen <= 1) {
    // 			const noProgressPossible = this.sides.every(side => {
    // 				const foeAllGhosts = side.foe.pokemon.every(pokemon => pokemon.fainted || pokemon.hasType('Ghost'));
    // 				const foeAllTransform = side.foe.pokemon.every(pokemon => (
    // 					pokemon.fainted ||
    // 					// true if transforming into this pokemon would lead to an endless battle
    // 					// Transform will fail (depleting PP) if used against Ditto in Stadium 1
    // 					(this.dex.currentMod !== 'gen1stadium' || pokemon.species.id !== 'ditto') &&
    // 					// there are some subtleties such as a Mew with only Transform and auto-fail moves,
    // 					// but it's unlikely to come up in a real game so there's no need to handle it
    // 					pokemon.moves.every(moveid => moveid === 'transform')
    // 				));
    // 				return side.pokemon.every(pokemon => (
    // 					pokemon.fainted ||
    // 					// frozen pokemon can't thaw in gen 1 without outside help
    // 					pokemon.status === 'frz' ||
    // 					// a pokemon can't lose PP if it Transforms into a pokemon with only Transform
    // 					(pokemon.moves.every(moveid => moveid === 'transform') && foeAllTransform) ||
    // 					// Struggle can't damage yourself if every foe is a Ghost
    // 					(pokemon.moveSlots.every(slot => slot.pp === 0) && foeAllGhosts)
    // 				));
    // 			});
    // 			if (noProgressPossible) {
    // 				this.add('-message', `This battle cannot progress. Endless Battle Clause activated!`);
    // 				return this.tie();
    // 			}
    // 		}
    // 
    // 		if (this.turn <= 100) return;
    // 
    // 		// the turn limit is not a part of Endless Battle Clause
    // 		if (this.turn >= 1000) {
    // 			this.add('message', `It is turn 1000. You have hit the turn limit!`);
    // 			this.tie();
    // 			return true;
    // 		}
    // 		if (
    // 			(this.turn >= 500 && this.turn % 100 === 0) || // every 100 turns past turn 500,
    // 			(this.turn >= 900 && this.turn % 10 === 0) || // every 10 turns past turn 900,
    // 			this.turn >= 990 // every turn past turn 990
    // 		) {
    // 			const turnsLeft = 1000 - this.turn;
    // 			const turnsLeftText = (turnsLeft === 1 ? `1 turn` : `${turnsLeft} turns`);
    // 			this.add('bigerror', `You will auto-tie if the battle doesn't end in ${turnsLeftText} (on turn 1000).`);
    // 		}
    // 
    // 		if (!this.ruleTable.has('endlessbattleclause')) return;
    // 		// for now, FFA doesn't support Endless Battle Clause
    // 		if (this.format.gameType === 'freeforall') return;
    // 
    // 		// Are all Pokemon on every side stale, with at least one side containing an externally stale Pokemon?
    // 		if (!stalenessBySide.every(s => !!s) || !stalenessBySide.some(s => s === 'external')) return;
    // 
    // 		// Can both sides switch to a non-stale Pokemon?
    // 		const canSwitch = [];
    // 		for (const [i, trapped] of trappedBySide.entries()) {
    // 			canSwitch[i] = false;
    // 			if (trapped) break;
    // 			const side = this.sides[i];
    // 
    // 			for (const pokemon of side.pokemon) {
    // 				if (!pokemon.fainted && !(pokemon.volatileStaleness || pokemon.staleness)) {
    // 					canSwitch[i] = true;
    // 					break;
    // 				}
    // 			}
    // 		}
    // 		if (canSwitch.every(s => s)) return;
    // 
    // 		// Endless Battle Clause activates - we determine the winner by looking at each side's sets.
    // 		const losers: Side[] = [];
    // 		for (const side of this.sides) {
    // 			let berry = false; // Restorative Berry
    // 			let cycle = false; // Harvest or Recycle
    // 			for (const pokemon of side.pokemon) {
    // 				berry = RESTORATIVE_BERRIES.has(toID(pokemon.set.item));
    // 				if (['harvest', 'pickup'].includes(toID(pokemon.set.ability)) ||
    // 					pokemon.set.moves.map(toID).includes('recycle' as ID)) {
    // 					cycle = true;
    // 				}
    // 				if (berry && cycle) break;
    // 			}
    // 			if (berry && cycle) losers.push(side);
    // 		}
    // 
    // 		if (losers.length === 1) {
    // 			const loser = losers[0];
    // 			this.add('-message', `${loser.name}'s team started with the rudimentary means to perform restorative berry-cycling and thus loses.`);
    // 			return this.win(loser.foe);
    // 		}
    // 		if (losers.length === this.sides.length) {
    // 			this.add('-message', `Each side's team started with the rudimentary means to perform restorative berry-cycling.`);
    // 		}
    // 
    // 		return this.tie();
    // 	}
    //
    pub fn maybe_trigger_endless_battle_clause(&mut self) -> bool {
        // JS: if (this.turn <= 100) return;
        if self.turn <= 100 {
            return false;
        }

        // JS: if (this.turn >= 1000) { this.add('message', ...); this.tie(); return true; }
        if self.turn >= 1000 {
            self.add_log("message", &["It is turn 1000. You have hit the turn limit!"]);
            self.tie();
            return true;
        }

        // JS: Turn limit warnings
        // if ((turn >= 500 && turn % 100 === 0) || (turn >= 900 && turn % 10 === 0) || turn >= 990)
        if (self.turn >= 500 && self.turn % 100 == 0) ||
           (self.turn >= 900 && self.turn % 10 == 0) ||
           self.turn >= 990 {
            let turns_left = 1000 - self.turn;
            let turns_text = if turns_left == 1 {
                "1 turn".to_string()
            } else {
                format!("{} turns", turns_left)
            };
            self.add_log("bigerror", &[&format!("You will auto-tie if the battle doesn't end in {} (on turn 1000).", turns_text)]);
        }

        // TODO: Gen 1 no-progress checks (requires Pokemon.hasType())
        // TODO: Staleness checks (requires Pokemon.volatileStaleness, Pokemon.staleness)
        // TODO: Berry cycling checks (requires Harvest, Recycle, RESTORATIVE_BERRIES)

        false
    }

    /// Restart the battle (for testing)
    /// Restart a deserialized battle
    /// Equivalent to battle.ts restart() (battle.ts:1925-1929)
    ///
    /// JS: restart(send?: (type: string, data: string | string[]) => void) {
    ///   if (!this.deserialized) throw new Error('Attempt to restart a battle which has not been deserialized');
    ///   (this as any).send = send;
    /// }
    // 
    // 	restart(send?: (type: string, data: string | string[]) => void) {
    // 		if (!this.deserialized) throw new Error('Attempt to restart a battle which has not been deserialized');
    // 
    // 		(this as any).send = send;
    // 	}
    //
    pub fn restart(&mut self) {
        // JS: if (!this.deserialized) throw new Error(...)
        if !self.deserialized {
            panic!("Attempt to restart a battle which has not been deserialized");
        }

        // JS: (this as any).send = send;
        // Note: Rust doesn't have a send function parameter like JS, so this is a no-op
        // The send function in JS is used for sending updates to clients
    }

    /// Reset the PRNG with a new seed
    /// Equivalent to battle.ts resetRNG()
    // TypeScript source:
    // /** Note that passing `undefined` resets to the starting seed, but `null` will roll a new seed */
    // 	resetRNG(seed: PRNGSeed | null = this.prngSeed) {
    // 		this.prng = new PRNG(seed);
    // 		this.add('message', "The battle's RNG was reset.");
    // 	}
    //
    pub fn reset_rng(&mut self, seed: Option<crate::prng::PRNGSeed>) {
        let new_seed = seed.unwrap_or_else(|| self.prng_seed.clone());
        self.prng = PRNG::new(Some(new_seed.clone()));
        self.prng_seed = new_seed;
        self.add_log("message", &["The battle's RNG was reset."]);
    }

    /// Join a player to a battle slot
    /// Equivalent to battle.ts join()
    ///
    /// This is a convenience method that wraps setPlayer
    /// Join a player to the battle
    /// Equivalent to battle.ts join() (battle.ts:3261-3264)
    ///
    // TypeScript source:
    // /** @deprecated */
    // 	join(slot: SideID, name: string, avatar: string, team: PokemonSet[] | string | null) {
    // 		this.setPlayer(slot, { name, avatar, team });
    // 		return this.getSide(slot);
    // 	}
    //
    pub fn join(
        &mut self,
        slot: SideID,
        name: &str,
        avatar: &str,
        team: Option<Vec<crate::pokemon::Pokemon>>,
    ) -> Option<usize> {
        // JS: this.setPlayer(slot, { name, avatar, team });
        let options = PlayerOptions {
            name: name.to_string(),
            avatar: if avatar.is_empty() { None } else { Some(avatar.to_string()) },
            team: vec![], // Team is handled separately in Rust
            rating: None,
        };
        self.set_player(slot, options);

        // If a team was provided, replace the side's pokemon
        if let Some(pokemon_team) = team {
            if let Some(side) = self.get_side_mut(slot) {
                side.pokemon = pokemon_team;
            }
        }

        // JS: return this.getSide(slot);
        // In JavaScript, this returns the Side object
        // In Rust, we return the side index since we can't return a reference
        Some(slot.index())
    }

    /// Destroy the battle (cleanup for garbage collection)
    /// Equivalent to battle.ts destroy() (battle.ts:3346-3367)
    ///
    /// JS: destroy() {
    ///   this.field.destroy();
    ///   for (let i = 0; i < this.sides.length; i++) {
    ///     if (this.sides[i]) this.sides[i].destroy();
    ///   }
    ///   for (const action of this.queue.list) delete (action as any).pokemon;
    ///   this.queue.battle = null!;
    ///   (this as any).log = [];
    /// }
    // 
    // 	destroy() {
    // 		// deallocate ourself
    // 
    // 		// deallocate children and get rid of references to them
    // 		this.field.destroy();
    // 		(this as any).field = null!;
    // 
    // 		for (let i = 0; i < this.sides.length; i++) {
    // 			if (this.sides[i]) {
    // 				this.sides[i].destroy();
    // 				this.sides[i] = null!;
    // 			}
    // 		}
    // 		for (const action of this.queue.list) {
    // 			delete (action as any).pokemon;
    // 		}
    // 
    // 		this.queue.battle = null!;
    // 		this.queue = null!;
    // 		// in case the garbage collector really sucks, at least deallocate the log
    // 		(this as any).log = [];
    // 	}
    //
    pub fn destroy(&mut self) {
        // Note: In Rust, cleanup is handled automatically by the Drop trait
        // The JS version manually breaks circular references for garbage collection
        // This is not needed in Rust, so this method is effectively a no-op

        // We can still clear collections to free memory explicitly if desired
        self.sides.clear();
        self.queue.clear();
        self.log.clear();
    }

    // =========================================================================
    // EVENT SYSTEM (ported from battle.ts)
    // =========================================================================

    /// Single event - runs a single callback on an effect
    /// Equivalent to battle.ts singleEvent() (lines 571-652)
    ///
    /// This fires a single event handler with full suppression logic.
    // TypeScript source:
    // /** The entire event system revolves around this function and runEvent. */
    // 	singleEvent(
    // 		eventid: string, effect: Effect, state: EffectState | Record<string, never> | null,
    // 		target: string | Pokemon | Side | Field | Battle | null, source?: string | Pokemon | Effect | false | null,
    // 		sourceEffect?: Effect | string | null, relayVar?: any, customCallback?: unknown
    // 	) {
    // 		if (this.eventDepth >= 8) {
    // 			// oh fuck
    // 			this.add('message', 'STACK LIMIT EXCEEDED');
    // 			this.add('message', 'PLEASE REPORT IN BUG THREAD');
    // 			this.add('message', 'Event: ' + eventid);
    // 			this.add('message', 'Parent event: ' + this.event.id);
    // 			throw new Error("Stack overflow");
    // 		}
    // 		if (this.log.length - this.sentLogPos > 1000) {
    // 			this.add('message', 'LINE LIMIT EXCEEDED');
    // 			this.add('message', 'PLEASE REPORT IN BUG THREAD');
    // 			this.add('message', 'Event: ' + eventid);
    // 			this.add('message', 'Parent event: ' + this.event.id);
    // 			throw new Error("Infinite loop");
    // 		}
    // 		// this.add('Event: ' + eventid + ' (depth ' + this.eventDepth + ')');
    // 		let hasRelayVar = true;
    // 		if (relayVar === undefined) {
    // 			relayVar = true;
    // 			hasRelayVar = false;
    // 		}
    // 
    // 		if (effect.effectType === 'Status' && (target instanceof Pokemon) && target.status !== effect.id) {
    // 			// it's changed; call it off
    // 			return relayVar;
    // 		}
    // 		if (eventid === 'SwitchIn' && effect.effectType === 'Ability' && effect.flags['breakable'] &&
    // 			this.suppressingAbility(target as Pokemon)) {
    // 			this.debug(eventid + ' handler suppressed by Mold Breaker');
    // 			return relayVar;
    // 		}
    // 		if (eventid !== 'Start' && eventid !== 'TakeItem' && effect.effectType === 'Item' &&
    // 			(target instanceof Pokemon) && target.ignoringItem()) {
    // 			this.debug(eventid + ' handler suppressed by Embargo, Klutz or Magic Room');
    // 			return relayVar;
    // 		}
    // 		if (eventid !== 'End' && effect.effectType === 'Ability' && (target instanceof Pokemon) && target.ignoringAbility()) {
    // 			this.debug(eventid + ' handler suppressed by Gastro Acid or Neutralizing Gas');
    // 			return relayVar;
    // 		}
    // 		if (
    // 			effect.effectType === 'Weather' && eventid !== 'FieldStart' && eventid !== 'FieldResidual' &&
    // 			eventid !== 'FieldEnd' && this.field.suppressingWeather()
    // 		) {
    // 			this.debug(eventid + ' handler suppressed by Air Lock');
    // 			return relayVar;
    // 		}
    // 
    // 		const callback = customCallback || (effect as any)[`on${eventid}`];
    // 		if (callback === undefined) return relayVar;
    // 
    // 		const parentEffect = this.effect;
    // 		const parentEffectState = this.effectState;
    // 		const parentEvent = this.event;
    // 
    // 		this.effect = effect;
    // 		this.effectState = state as EffectState || this.initEffectState({});
    // 		this.event = { id: eventid, target, source, effect: sourceEffect };
    // 		this.eventDepth++;
    // 
    // 		const args = [target, source, sourceEffect];
    // 		if (hasRelayVar) args.unshift(relayVar);
    // 
    // 		let returnVal;
    // 		if (typeof callback === 'function') {
    // 			returnVal = callback.apply(this, args);
    // 		} else {
    // 			returnVal = callback;
    // 		}
    // 
    // 		this.eventDepth--;
    // 		this.effect = parentEffect;
    // 		this.effectState = parentEffectState;
    // 		this.event = parentEvent;
    // 
    // 		return returnVal === undefined ? relayVar : returnVal;
    // 	}
    //
    pub fn single_event(
        &mut self,
        event_id: &str,
        effect_id: &ID,
        target: Option<(usize, usize)>,
        source: Option<(usize, usize)>,
        _source_effect: Option<&ID>,
    ) -> crate::event::EventResult {
        use crate::event::EventResult;

        // JavaScript: if (this.eventDepth >= 8) throw Error
        if self.event_depth >= 8 {
            self.add_log("message", &["STACK LIMIT EXCEEDED"]);
            self.add_log("message", &["PLEASE REPORT IN BUG THREAD"]);
            self.add_log("message", &[&format!("Event: {}", event_id)]);
            if let Some(ref evt) = self.current_event {
                self.add_log("message", &[&format!("Parent event: {}", evt.id)]);
            }
            return EventResult::Boolean(false);
        }

        // JavaScript: if (this.log.length - this.sentLogPos > 1000) throw Error
        if self.log.len() - self.sent_log_pos > 1000 {
            self.add_log("message", &["LINE LIMIT EXCEEDED"]);
            self.add_log("message", &["PLEASE REPORT IN BUG THREAD"]);
            self.add_log("message", &[&format!("Event: {}", event_id)]);
            if let Some(ref evt) = self.current_event {
                self.add_log("message", &[&format!("Parent event: {}", evt.id)]);
            }
            return EventResult::Boolean(false);
        }

        // Determine effect type for suppression checks
        let effect_type = self.get_effect_type(effect_id);

        // SUPPRESSION CHECKS (from JavaScript battle.ts:598-622)

        // JavaScript: if (effect.effectType === 'Status' && target.status !== effect.id) return relayVar
        if effect_type == "Status" {
            if let Some((side_idx, poke_idx)) = target {
                if let Some(side) = self.sides.get(side_idx) {
                    if let Some(pokemon) = side.pokemon.get(poke_idx) {
                        if pokemon.status != *effect_id {
                            return EventResult::Continue;
                        }
                    }
                }
            }
        }

        // JavaScript: if (eventid === 'SwitchIn' && effect.effectType === 'Ability' && effect.flags['breakable'] && this.suppressingAbility(target))
        if event_id == "SwitchIn" && effect_type == "Ability" {
            if self.suppressing_ability(target) {
                self.debug(&format!("{} handler suppressed by Mold Breaker", event_id));
                return EventResult::Continue;
            }
        }

        // JavaScript: if (eventid !== 'Start' && eventid !== 'TakeItem' && effect.effectType === 'Item' && target.ignoringItem())
        if event_id != "Start" && event_id != "TakeItem" && effect_type == "Item" {
            if let Some((side_idx, poke_idx)) = target {
                if let Some(side) = self.sides.get(side_idx) {
                    if let Some(pokemon) = side.pokemon.get(poke_idx) {
                        if pokemon.ignoring_item() {
                            self.debug(&format!("{} handler suppressed by Embargo, Klutz or Magic Room", event_id));
                            return EventResult::Continue;
                        }
                    }
                }
            }
        }

        // JavaScript: if (eventid !== 'End' && effect.effectType === 'Ability' && target.ignoringAbility())
        if event_id != "End" && effect_type == "Ability" {
            if let Some((side_idx, poke_idx)) = target {
                if let Some(side) = self.sides.get(side_idx) {
                    if let Some(pokemon) = side.pokemon.get(poke_idx) {
                        if pokemon.ignoring_ability() {
                            self.debug(&format!("{} handler suppressed by Gastro Acid or Neutralizing Gas", event_id));
                            return EventResult::Continue;
                        }
                    }
                }
            }
        }

        // JavaScript: if (effect.effectType === 'Weather' && eventid !== 'FieldStart' && eventid !== 'FieldResidual' && eventid !== 'FieldEnd' && this.field.suppressingWeather())
        if effect_type == "Weather" && event_id != "FieldStart" && event_id != "FieldResidual" && event_id != "FieldEnd" {
            if self.field.suppressing_weather() {
                self.debug(&format!("{} handler suppressed by Air Lock", event_id));
                return EventResult::Continue;
            }
        }

        // Save parent event context
        let parent_event = self.current_event.take();
        let parent_effect = self.current_effect.take();
        let parent_effect_state = self.current_effect_state.take();

        // Set up current event
        self.current_event = Some(EventInfo {
            id: event_id.to_string(),
            target,
            source,
            effect: Some(effect_id.clone()),
            modifier: 4096,
        });
        self.current_effect = Some(effect_id.clone());
        self.event_depth += 1;

        // Dispatch based on effect type
        let result = self.dispatch_single_event(event_id, effect_id, target, source);

        // Restore parent context
        self.event_depth -= 1;
        self.current_event = parent_event;
        self.current_effect = parent_effect;
        self.current_effect_state = parent_effect_state;

        result
    }

    /// Get effect type for an effect ID
    /// Rust helper method - JavaScript determines effect type dynamically via duck typing
    /// This method checks the effect ID against dex lookups to categorize it
    /// Returns: "Ability", "Item", "Move", "Status", "Volatile", "Weather", "Terrain", or "Unknown"
    fn get_effect_type(&self, effect_id: &ID) -> &str {
        // Check if it's an ability
        if self.dex.get_ability(effect_id.as_str()).is_some() {
            return "Ability";
        }
        // Check if it's an item
        if self.dex.get_item(effect_id.as_str()).is_some() {
            return "Item";
        }
        // Check if it's a move
        if self.dex.get_move(effect_id.as_str()).is_some() {
            return "Move";
        }
        // Check if it's a condition
        if let Some(condition) = crate::data::conditions::get_condition(effect_id) {
            // Conditions can be Status, Volatile, Weather, Terrain, etc.
            if crate::data::conditions::is_status_condition(effect_id) {
                return "Status";
            }
            if crate::data::conditions::is_volatile_condition(effect_id) {
                return "Volatile";
            }
            // Check for weather/terrain by ID
            let id_str = effect_id.as_str();
            if ["sunnyday", "raindance", "sandstorm", "hail", "snow", "harsh sunshine", "heavy rain", "strong winds"].contains(&id_str) {
                return "Weather";
            }
            if ["electricterrain", "grassyterrain", "mistyterrain", "psychicterrain"].contains(&id_str) {
                return "Terrain";
            }
        }
        "Unknown"
    }

    /// Dispatch a single event to the appropriate handler
    /// Rust helper method - JavaScript's singleEvent() calls handler functions directly
    /// This method routes events to specialized handlers based on effect type
    /// Routes to: handle_ability_event, handle_item_event, handle_move_event, handle_condition_event
    fn dispatch_single_event(
        &mut self,
        event_id: &str,
        effect_id: &ID,
        target: Option<(usize, usize)>,
        _source: Option<(usize, usize)>,
    ) -> crate::event::EventResult {
        use crate::event::EventResult;

        let effect_str = effect_id.as_str();

        // Handle ability events
        if self.dex.get_ability(effect_id.as_str()).is_some() {
            return self.handle_ability_event(event_id, effect_id, target);
        }

        // Handle item events
        if self.dex.get_item(effect_id.as_str()).is_some() {
            return self.handle_item_event(event_id, effect_id, target);
        }

        // Handle move events
        if let Some(_move_def) = self.dex.get_move(effect_id.as_str()) {
            return self.handle_move_event(event_id, effect_str, target);
        }

        // Handle condition events (status, volatile, weather, terrain)
        if let Some(_condition) = crate::data::conditions::get_condition(effect_id) {
            return self.handle_condition_event(event_id, effect_str, target);
        }

        EventResult::Continue
    }

    /// Handle ability events
    /// Rust helper method - JavaScript's singleEvent() directly invokes ability[`on${eventId}`] callbacks
    /// This method dispatches to ability_callbacks module based on event name
    /// Routes to ability-specific handlers for all event types (AfterBoost, ModifyDamage, etc.)
    fn handle_ability_event(
        &mut self,
        event_id: &str,
        ability_id: &ID,
        target: Option<(usize, usize)>,
    ) -> crate::event::EventResult {
        use crate::event::EventResult;
        use crate::data::ability_callbacks;

        let pokemon_pos = target.unwrap_or((0, 0));

        match event_id {
            "AfterBoost" => ability_callbacks::dispatch_on_after_boost(self, ability_id.as_str(), pokemon_pos),
            "AfterEachBoost" => ability_callbacks::dispatch_on_after_each_boost(self, ability_id.as_str(), pokemon_pos),
            "AfterMoveSecondary" => ability_callbacks::dispatch_on_after_move_secondary(self, ability_id.as_str(), pokemon_pos),
            "AfterMoveSecondarySelf" => ability_callbacks::dispatch_on_after_move_secondary_self(self, ability_id.as_str(), pokemon_pos),
            "AfterSetStatus" => ability_callbacks::dispatch_on_after_set_status(self, ability_id.as_str(), pokemon_pos),
            "AfterTerastallization" => ability_callbacks::dispatch_on_after_terastallization(self, ability_id.as_str(), pokemon_pos),
            "AfterUseItem" => ability_callbacks::dispatch_on_after_use_item(self, ability_id.as_str(), pokemon_pos),
            "AllyAfterUseItem" => ability_callbacks::dispatch_on_ally_after_use_item(self, ability_id.as_str(), pokemon_pos),
            "AllyBasePower" => ability_callbacks::dispatch_on_ally_base_power(self, ability_id.as_str(), pokemon_pos),
            "AllyBasePowerPriority" => ability_callbacks::dispatch_on_ally_base_power_priority(self, ability_id.as_str(), pokemon_pos),
            "AllyFaint" => ability_callbacks::dispatch_on_ally_faint(self, ability_id.as_str(), pokemon_pos),
            "AllyModifyAtk" => ability_callbacks::dispatch_on_ally_modify_atk(self, ability_id.as_str(), pokemon_pos),
            "AllyModifyAtkPriority" => ability_callbacks::dispatch_on_ally_modify_atk_priority(self, ability_id.as_str(), pokemon_pos),
            "AllyModifySpD" => ability_callbacks::dispatch_on_ally_modify_sp_d(self, ability_id.as_str(), pokemon_pos),
            "AllyModifySpDPriority" => ability_callbacks::dispatch_on_ally_modify_sp_d_priority(self, ability_id.as_str(), pokemon_pos),
            "AllySetStatus" => ability_callbacks::dispatch_on_ally_set_status(self, ability_id.as_str(), pokemon_pos),
            "AllyTryAddVolatile" => ability_callbacks::dispatch_on_ally_try_add_volatile(self, ability_id.as_str(), pokemon_pos),
            "AllyTryBoost" => ability_callbacks::dispatch_on_ally_try_boost(self, ability_id.as_str(), pokemon_pos),
            "AllyTryHitSide" => ability_callbacks::dispatch_on_ally_try_hit_side(self, ability_id.as_str(), pokemon_pos),
            "AnyAccuracy" => ability_callbacks::dispatch_on_any_accuracy(self, ability_id.as_str(), pokemon_pos),
            "AnyAfterMega" => ability_callbacks::dispatch_on_any_after_mega(self, ability_id.as_str(), pokemon_pos),
            "AnyAfterMove" => ability_callbacks::dispatch_on_any_after_move(self, ability_id.as_str(), pokemon_pos),
            "AnyAfterSetStatus" => ability_callbacks::dispatch_on_any_after_set_status(self, ability_id.as_str(), pokemon_pos),
            "AnyAfterTerastallization" => ability_callbacks::dispatch_on_any_after_terastallization(self, ability_id.as_str(), pokemon_pos),
            "AnyBasePower" => ability_callbacks::dispatch_on_any_base_power(self, ability_id.as_str(), pokemon_pos),
            "AnyBasePowerPriority" => ability_callbacks::dispatch_on_any_base_power_priority(self, ability_id.as_str(), pokemon_pos),
            "AnyBeforeMove" => ability_callbacks::dispatch_on_any_before_move(self, ability_id.as_str(), pokemon_pos),
            "AnyDamage" => ability_callbacks::dispatch_on_any_damage(self, ability_id.as_str(), pokemon_pos),
            "AnyFaint" => ability_callbacks::dispatch_on_any_faint(self, ability_id.as_str(), pokemon_pos),
            "AnyFaintPriority" => ability_callbacks::dispatch_on_any_faint_priority(self, ability_id.as_str(), pokemon_pos),
            "AnyInvulnerability" => ability_callbacks::dispatch_on_any_invulnerability(self, ability_id.as_str(), pokemon_pos),
            "AnyInvulnerabilityPriority" => ability_callbacks::dispatch_on_any_invulnerability_priority(self, ability_id.as_str(), pokemon_pos),
            "AnyModifyAccuracy" => ability_callbacks::dispatch_on_any_modify_accuracy(self, ability_id.as_str(), pokemon_pos),
            "AnyModifyAccuracyPriority" => ability_callbacks::dispatch_on_any_modify_accuracy_priority(self, ability_id.as_str(), pokemon_pos),
            "AnyModifyAtk" => ability_callbacks::dispatch_on_any_modify_atk(self, ability_id.as_str(), pokemon_pos),
            "AnyModifyBoost" => ability_callbacks::dispatch_on_any_modify_boost(self, ability_id.as_str(), pokemon_pos),
            "AnyModifyDamage" => ability_callbacks::dispatch_on_any_modify_damage(self, ability_id.as_str(), pokemon_pos),
            "AnyModifyDef" => ability_callbacks::dispatch_on_any_modify_def(self, ability_id.as_str(), pokemon_pos),
            "AnyModifySpA" => ability_callbacks::dispatch_on_any_modify_sp_a(self, ability_id.as_str(), pokemon_pos),
            "AnyModifySpD" => ability_callbacks::dispatch_on_any_modify_sp_d(self, ability_id.as_str(), pokemon_pos),
            "AnyRedirectTarget" => ability_callbacks::dispatch_on_any_redirect_target(self, ability_id.as_str(), pokemon_pos),
            "AnySetWeather" => ability_callbacks::dispatch_on_any_set_weather(self, ability_id.as_str(), pokemon_pos),
            "AnySwitchIn" => ability_callbacks::dispatch_on_any_switch_in(self, ability_id.as_str(), pokemon_pos),
            "AnySwitchInPriority" => ability_callbacks::dispatch_on_any_switch_in_priority(self, ability_id.as_str(), pokemon_pos),
            "AnyTryMove" => ability_callbacks::dispatch_on_any_try_move(self, ability_id.as_str(), pokemon_pos),
            "AnyTryPrimaryHit" => ability_callbacks::dispatch_on_any_try_primary_hit(self, ability_id.as_str(), pokemon_pos),
            "BasePower" => ability_callbacks::dispatch_on_base_power(self, ability_id.as_str(), pokemon_pos),
            "BasePowerPriority" => ability_callbacks::dispatch_on_base_power_priority(self, ability_id.as_str(), pokemon_pos),
            "BeforeMove" => ability_callbacks::dispatch_on_before_move(self, ability_id.as_str(), pokemon_pos),
            "BeforeMovePriority" => ability_callbacks::dispatch_on_before_move_priority(self, ability_id.as_str(), pokemon_pos),
            "BeforeSwitchIn" => ability_callbacks::dispatch_on_before_switch_in(self, ability_id.as_str(), pokemon_pos),
            "ChangeBoost" => ability_callbacks::dispatch_on_change_boost(self, ability_id.as_str(), pokemon_pos),
            "CheckShow" => ability_callbacks::dispatch_on_check_show(self, ability_id.as_str(), pokemon_pos),
            "CriticalHit" => ability_callbacks::dispatch_on_critical_hit(self, ability_id.as_str(), pokemon_pos),
            "Damage" => ability_callbacks::dispatch_on_damage(self, ability_id.as_str(), pokemon_pos),
            "DamagePriority" => ability_callbacks::dispatch_on_damage_priority(self, ability_id.as_str(), pokemon_pos),
            "DamagingHit" => ability_callbacks::dispatch_on_damaging_hit(self, ability_id.as_str(), pokemon_pos),
            "DamagingHitOrder" => ability_callbacks::dispatch_on_damaging_hit_order(self, ability_id.as_str(), pokemon_pos),
            "DeductPP" => ability_callbacks::dispatch_on_deduct_p_p(self, ability_id.as_str(), pokemon_pos),
            "DisableMove" => ability_callbacks::dispatch_on_disable_move(self, ability_id.as_str(), pokemon_pos),
            "DragOut" => ability_callbacks::dispatch_on_drag_out(self, ability_id.as_str(), pokemon_pos),
            "DragOutPriority" => ability_callbacks::dispatch_on_drag_out_priority(self, ability_id.as_str(), pokemon_pos),
            "EatItem" => ability_callbacks::dispatch_on_eat_item(self, ability_id.as_str(), pokemon_pos),
            "Effectiveness" => ability_callbacks::dispatch_on_effectiveness(self, ability_id.as_str(), pokemon_pos),
            "EmergencyExit" => ability_callbacks::dispatch_on_emergency_exit(self, ability_id.as_str(), pokemon_pos),
            "End" => ability_callbacks::dispatch_on_end(self, ability_id.as_str(), pokemon_pos),
            "Faint" => ability_callbacks::dispatch_on_faint(self, ability_id.as_str(), pokemon_pos),
            "Flinch" => ability_callbacks::dispatch_on_flinch(self, ability_id.as_str(), pokemon_pos),
            "FoeAfterBoost" => ability_callbacks::dispatch_on_foe_after_boost(self, ability_id.as_str(), pokemon_pos),
            "FoeMaybeTrapPokemon" => ability_callbacks::dispatch_on_foe_maybe_trap_pokemon(self, ability_id.as_str(), pokemon_pos),
            "FoeTrapPokemon" => ability_callbacks::dispatch_on_foe_trap_pokemon(self, ability_id.as_str(), pokemon_pos),
            "FoeTryEatItem" => ability_callbacks::dispatch_on_foe_try_eat_item(self, ability_id.as_str(), pokemon_pos),
            "FoeTryMove" => ability_callbacks::dispatch_on_foe_try_move(self, ability_id.as_str(), pokemon_pos),
            "FractionalPriority" => ability_callbacks::dispatch_on_fractional_priority(self, ability_id.as_str(), pokemon_pos),
            "FractionalPriorityPriority" => ability_callbacks::dispatch_on_fractional_priority_priority(self, ability_id.as_str(), pokemon_pos),
            "Hit" => ability_callbacks::dispatch_on_hit(self, ability_id.as_str(), pokemon_pos),
            "Immunity" => ability_callbacks::dispatch_on_immunity(self, ability_id.as_str(), pokemon_pos),
            "ModifyAccuracy" => ability_callbacks::dispatch_on_modify_accuracy(self, ability_id.as_str(), pokemon_pos),
            "ModifyAccuracyPriority" => ability_callbacks::dispatch_on_modify_accuracy_priority(self, ability_id.as_str(), pokemon_pos),
            "ModifyAtk" => ability_callbacks::dispatch_on_modify_atk(self, ability_id.as_str(), pokemon_pos),
            "ModifyAtkPriority" => ability_callbacks::dispatch_on_modify_atk_priority(self, ability_id.as_str(), pokemon_pos),
            "ModifyCritRatio" => ability_callbacks::dispatch_on_modify_crit_ratio(self, ability_id.as_str(), pokemon_pos),
            "ModifyDamage" => ability_callbacks::dispatch_on_modify_damage(self, ability_id.as_str(), pokemon_pos),
            "ModifyDef" => ability_callbacks::dispatch_on_modify_def(self, ability_id.as_str(), pokemon_pos),
            "ModifyDefPriority" => ability_callbacks::dispatch_on_modify_def_priority(self, ability_id.as_str(), pokemon_pos),
            "ModifyMove" => ability_callbacks::dispatch_on_modify_move(self, ability_id.as_str(), pokemon_pos),
            "ModifyMovePriority" => ability_callbacks::dispatch_on_modify_move_priority(self, ability_id.as_str(), pokemon_pos),
            "ModifyPriority" => ability_callbacks::dispatch_on_modify_priority(self, ability_id.as_str(), pokemon_pos),
            "ModifySTAB" => ability_callbacks::dispatch_on_modify_s_t_a_b(self, ability_id.as_str(), pokemon_pos),
            "ModifySecondaries" => ability_callbacks::dispatch_on_modify_secondaries(self, ability_id.as_str(), pokemon_pos),
            "ModifySpA" => ability_callbacks::dispatch_on_modify_sp_a(self, ability_id.as_str(), pokemon_pos),
            "ModifySpAPriority" => ability_callbacks::dispatch_on_modify_sp_a_priority(self, ability_id.as_str(), pokemon_pos),
            "ModifySpe" => ability_callbacks::dispatch_on_modify_spe(self, ability_id.as_str(), pokemon_pos),
            "ModifyType" => ability_callbacks::dispatch_on_modify_type(self, ability_id.as_str(), pokemon_pos),
            "ModifyTypePriority" => ability_callbacks::dispatch_on_modify_type_priority(self, ability_id.as_str(), pokemon_pos),
            "ModifyWeight" => ability_callbacks::dispatch_on_modify_weight(self, ability_id.as_str(), pokemon_pos),
            "ModifyWeightPriority" => ability_callbacks::dispatch_on_modify_weight_priority(self, ability_id.as_str(), pokemon_pos),
            "PrepareHit" => ability_callbacks::dispatch_on_prepare_hit(self, ability_id.as_str(), pokemon_pos),
            "Residual" => ability_callbacks::dispatch_on_residual(self, ability_id.as_str(), pokemon_pos),
            "ResidualOrder" => ability_callbacks::dispatch_on_residual_order(self, ability_id.as_str(), pokemon_pos),
            "ResidualSubOrder" => ability_callbacks::dispatch_on_residual_sub_order(self, ability_id.as_str(), pokemon_pos),
            "SetStatus" => ability_callbacks::dispatch_on_set_status(self, ability_id.as_str(), pokemon_pos),
            "SideConditionStart" => ability_callbacks::dispatch_on_side_condition_start(self, ability_id.as_str(), pokemon_pos),
            "SourceAfterFaint" => ability_callbacks::dispatch_on_source_after_faint(self, ability_id.as_str(), pokemon_pos),
            "SourceBasePower" => ability_callbacks::dispatch_on_source_base_power(self, ability_id.as_str(), pokemon_pos),
            "SourceBasePowerPriority" => ability_callbacks::dispatch_on_source_base_power_priority(self, ability_id.as_str(), pokemon_pos),
            "SourceDamagingHit" => ability_callbacks::dispatch_on_source_damaging_hit(self, ability_id.as_str(), pokemon_pos),
            "SourceModifyAccuracy" => ability_callbacks::dispatch_on_source_modify_accuracy(self, ability_id.as_str(), pokemon_pos),
            "SourceModifyAccuracyPriority" => ability_callbacks::dispatch_on_source_modify_accuracy_priority(self, ability_id.as_str(), pokemon_pos),
            "SourceModifyAtk" => ability_callbacks::dispatch_on_source_modify_atk(self, ability_id.as_str(), pokemon_pos),
            "SourceModifyAtkPriority" => ability_callbacks::dispatch_on_source_modify_atk_priority(self, ability_id.as_str(), pokemon_pos),
            "SourceModifyDamage" => ability_callbacks::dispatch_on_source_modify_damage(self, ability_id.as_str(), pokemon_pos),
            "SourceModifyDamagePriority" => ability_callbacks::dispatch_on_source_modify_damage_priority(self, ability_id.as_str(), pokemon_pos),
            "SourceModifySecondaries" => ability_callbacks::dispatch_on_source_modify_secondaries(self, ability_id.as_str(), pokemon_pos),
            "SourceModifySpA" => ability_callbacks::dispatch_on_source_modify_sp_a(self, ability_id.as_str(), pokemon_pos),
            "SourceModifySpAPriority" => ability_callbacks::dispatch_on_source_modify_sp_a_priority(self, ability_id.as_str(), pokemon_pos),
            "SourceTryHeal" => ability_callbacks::dispatch_on_source_try_heal(self, ability_id.as_str(), pokemon_pos),
            "SourceTryPrimaryHit" => ability_callbacks::dispatch_on_source_try_primary_hit(self, ability_id.as_str(), pokemon_pos),
            "Start" => ability_callbacks::dispatch_on_start(self, ability_id.as_str(), pokemon_pos),
            "SwitchIn" => ability_callbacks::dispatch_on_switch_in(self, ability_id.as_str(), pokemon_pos),
            "SwitchInPriority" => ability_callbacks::dispatch_on_switch_in_priority(self, ability_id.as_str(), pokemon_pos),
            "SwitchOut" => ability_callbacks::dispatch_on_switch_out(self, ability_id.as_str(), pokemon_pos),
            "TakeItem" => ability_callbacks::dispatch_on_take_item(self, ability_id.as_str(), pokemon_pos),
            "TerrainChange" => ability_callbacks::dispatch_on_terrain_change(self, ability_id.as_str(), pokemon_pos),
            "TryAddVolatile" => ability_callbacks::dispatch_on_try_add_volatile(self, ability_id.as_str(), pokemon_pos),
            "TryBoost" => ability_callbacks::dispatch_on_try_boost(self, ability_id.as_str(), pokemon_pos),
            "TryBoostPriority" => ability_callbacks::dispatch_on_try_boost_priority(self, ability_id.as_str(), pokemon_pos),
            "TryEatItem" => ability_callbacks::dispatch_on_try_eat_item(self, ability_id.as_str(), pokemon_pos),
            "TryEatItemPriority" => ability_callbacks::dispatch_on_try_eat_item_priority(self, ability_id.as_str(), pokemon_pos),
            "TryHeal" => ability_callbacks::dispatch_on_try_heal(self, ability_id.as_str(), pokemon_pos),
            "TryHit" => ability_callbacks::dispatch_on_try_hit(self, ability_id.as_str(), pokemon_pos),
            "TryHitPriority" => ability_callbacks::dispatch_on_try_hit_priority(self, ability_id.as_str(), pokemon_pos),
            "Update" => ability_callbacks::dispatch_on_update(self, ability_id.as_str(), pokemon_pos),
            "Weather" => ability_callbacks::dispatch_on_weather(self, ability_id.as_str(), pokemon_pos),
            "WeatherChange" => ability_callbacks::dispatch_on_weather_change(self, ability_id.as_str(), pokemon_pos),
            _ => EventResult::Continue,
        }
    }

    /// Handle item events
    /// Rust helper method - JavaScript's singleEvent() directly invokes item[`on${eventId}`] callbacks
    /// This method dispatches to item_callbacks module based on event name
    /// Routes to item-specific handlers for all event types (AfterBoost, ModifyDamage, Eat, etc.)
    fn handle_item_event(
        &mut self,
        event_id: &str,
        item_id: &ID,
        target: Option<(usize, usize)>,
    ) -> crate::event::EventResult {
        use crate::event::EventResult;
        use crate::data::item_callbacks;

        let source = self.current_event.as_ref().and_then(|e| e.source);
        let pokemon_pos = target.unwrap_or((0, 0));

        match event_id {
            "AfterBoost" => item_callbacks::dispatch_on_after_boost(self, item_id.as_str(), pokemon_pos),
            "AfterMoveSecondary" => item_callbacks::dispatch_on_after_move_secondary(self, item_id.as_str(), pokemon_pos),
            "AfterMoveSecondaryPriority" => item_callbacks::dispatch_on_after_move_secondary_priority(self, item_id.as_str(), pokemon_pos),
            "AfterMoveSecondarySelf" => {
                if let Some(source_pos) = source {
                    item_callbacks::dispatch_on_after_move_secondary_self(self, item_id.as_str(), source_pos, target)
                } else {
                    EventResult::Continue
                }
            }
            "AfterMoveSecondarySelfPriority" => item_callbacks::dispatch_on_after_move_secondary_self_priority(self, item_id.as_str(), pokemon_pos),
            "AfterSetStatus" => item_callbacks::dispatch_on_after_set_status(self, item_id.as_str(), pokemon_pos),
            "AfterSetStatusPriority" => item_callbacks::dispatch_on_after_set_status_priority(self, item_id.as_str(), pokemon_pos),
            "AfterSubDamage" => item_callbacks::dispatch_on_after_sub_damage(self, item_id.as_str(), pokemon_pos),
            "AnyAfterMega" => item_callbacks::dispatch_on_any_after_mega(self, item_id.as_str(), pokemon_pos),
            "AnyAfterMove" => item_callbacks::dispatch_on_any_after_move(self, item_id.as_str(), pokemon_pos),
            "AnyAfterTerastallization" => item_callbacks::dispatch_on_any_after_terastallization(self, item_id.as_str(), pokemon_pos),
            "AnyPseudoWeatherChange" => item_callbacks::dispatch_on_any_pseudo_weather_change(self, item_id.as_str(), pokemon_pos),
            "AnySwitchIn" => item_callbacks::dispatch_on_any_switch_in(self, item_id.as_str(), pokemon_pos),
            "AnySwitchInPriority" => item_callbacks::dispatch_on_any_switch_in_priority(self, item_id.as_str(), pokemon_pos),
            "Attract" => item_callbacks::dispatch_on_attract(self, item_id.as_str(), pokemon_pos),
            "AttractPriority" => item_callbacks::dispatch_on_attract_priority(self, item_id.as_str(), pokemon_pos),
            "BasePower" => item_callbacks::dispatch_on_base_power(self, item_id.as_str(), pokemon_pos),
            "BasePowerPriority" => item_callbacks::dispatch_on_base_power_priority(self, item_id.as_str(), pokemon_pos),
            "ChargeMove" => item_callbacks::dispatch_on_charge_move(self, item_id.as_str(), pokemon_pos),
            "Damage" => item_callbacks::dispatch_on_damage(self, item_id.as_str(), pokemon_pos),
            "DamagePriority" => item_callbacks::dispatch_on_damage_priority(self, item_id.as_str(), pokemon_pos),
            "DamagingHit" => item_callbacks::dispatch_on_damaging_hit(self, item_id.as_str(), pokemon_pos),
            "DamagingHitOrder" => item_callbacks::dispatch_on_damaging_hit_order(self, item_id.as_str(), pokemon_pos),
            "DisableMove" => item_callbacks::dispatch_on_disable_move(self, item_id.as_str(), pokemon_pos),
            "Drive" => item_callbacks::dispatch_on_drive(self, item_id.as_str(), pokemon_pos),
            "Eat" => item_callbacks::dispatch_on_eat(self, item_id.as_str(), pokemon_pos),
            "Effectiveness" => item_callbacks::dispatch_on_effectiveness(self, item_id.as_str(), pokemon_pos),
            "End" => item_callbacks::dispatch_on_end(self, item_id.as_str(), pokemon_pos),
            "FoeAfterBoost" => item_callbacks::dispatch_on_foe_after_boost(self, item_id.as_str(), pokemon_pos),
            "FractionalPriority" => item_callbacks::dispatch_on_fractional_priority(self, item_id.as_str(), pokemon_pos),
            "FractionalPriorityPriority" => item_callbacks::dispatch_on_fractional_priority_priority(self, item_id.as_str(), pokemon_pos),
            "Hit" => item_callbacks::dispatch_on_hit(self, item_id.as_str(), pokemon_pos),
            "Immunity" => item_callbacks::dispatch_on_immunity(self, item_id.as_str(), pokemon_pos),
            "MaybeTrapPokemon" => item_callbacks::dispatch_on_maybe_trap_pokemon(self, item_id.as_str(), pokemon_pos),
            "MaybeTrapPokemonPriority" => item_callbacks::dispatch_on_maybe_trap_pokemon_priority(self, item_id.as_str(), pokemon_pos),
            "Memory" => item_callbacks::dispatch_on_memory(self, item_id.as_str(), pokemon_pos),
            "ModifyAccuracy" => item_callbacks::dispatch_on_modify_accuracy(self, item_id.as_str(), pokemon_pos),
            "ModifyAccuracyPriority" => item_callbacks::dispatch_on_modify_accuracy_priority(self, item_id.as_str(), pokemon_pos),
            "ModifyAtk" => item_callbacks::dispatch_on_modify_atk(self, item_id.as_str(), pokemon_pos),
            "ModifyAtkPriority" => item_callbacks::dispatch_on_modify_atk_priority(self, item_id.as_str(), pokemon_pos),
            "ModifyCritRatio" => item_callbacks::dispatch_on_modify_crit_ratio(self, item_id.as_str(), pokemon_pos),
            "ModifyDamage" => item_callbacks::dispatch_on_modify_damage(self, item_id.as_str(), pokemon_pos),
            "ModifyDef" => item_callbacks::dispatch_on_modify_def(self, item_id.as_str(), pokemon_pos),
            "ModifyDefPriority" => item_callbacks::dispatch_on_modify_def_priority(self, item_id.as_str(), pokemon_pos),
            "ModifyMove" => item_callbacks::dispatch_on_modify_move(self, item_id.as_str(), pokemon_pos),
            "ModifyMovePriority" => item_callbacks::dispatch_on_modify_move_priority(self, item_id.as_str(), pokemon_pos),
            "ModifySecondaries" => item_callbacks::dispatch_on_modify_secondaries(self, item_id.as_str(), pokemon_pos),
            "ModifySpA" => item_callbacks::dispatch_on_modify_sp_a(self, item_id.as_str(), pokemon_pos),
            "ModifySpAPriority" => item_callbacks::dispatch_on_modify_sp_a_priority(self, item_id.as_str(), pokemon_pos),
            "ModifySpD" => item_callbacks::dispatch_on_modify_sp_d(self, item_id.as_str(), pokemon_pos),
            "ModifySpDPriority" => item_callbacks::dispatch_on_modify_sp_d_priority(self, item_id.as_str(), pokemon_pos),
            "ModifySpe" => item_callbacks::dispatch_on_modify_spe(self, item_id.as_str(), pokemon_pos),
            "ModifyWeight" => item_callbacks::dispatch_on_modify_weight(self, item_id.as_str(), pokemon_pos),
            "NegateImmunity" => item_callbacks::dispatch_on_negate_immunity(self, item_id.as_str(), pokemon_pos),
            "Plate" => item_callbacks::dispatch_on_plate(self, item_id.as_str(), pokemon_pos),
            "Residual" => item_callbacks::dispatch_on_residual(self, item_id.as_str(), pokemon_pos),
            "ResidualOrder" => item_callbacks::dispatch_on_residual_order(self, item_id.as_str(), pokemon_pos),
            "ResidualSubOrder" => item_callbacks::dispatch_on_residual_sub_order(self, item_id.as_str(), pokemon_pos),
            "SetAbility" => item_callbacks::dispatch_on_set_ability(self, item_id.as_str(), pokemon_pos),
            "SourceModifyAccuracy" => item_callbacks::dispatch_on_source_modify_accuracy(self, item_id.as_str(), pokemon_pos),
            "SourceModifyAccuracyPriority" => item_callbacks::dispatch_on_source_modify_accuracy_priority(self, item_id.as_str(), pokemon_pos),
            "SourceModifyDamage" => item_callbacks::dispatch_on_source_modify_damage(self, item_id.as_str(), pokemon_pos),
            "SourceTryPrimaryHit" => item_callbacks::dispatch_on_source_try_primary_hit(self, item_id.as_str(), pokemon_pos),
            "Start" => item_callbacks::dispatch_on_start(self, item_id.as_str(), pokemon_pos),
            "SwitchIn" => item_callbacks::dispatch_on_switch_in(self, item_id.as_str(), pokemon_pos),
            "SwitchInPriority" => item_callbacks::dispatch_on_switch_in_priority(self, item_id.as_str(), pokemon_pos),
            "TakeItem" => item_callbacks::dispatch_on_take_item(self, item_id.as_str(), pokemon_pos),
            "TerrainChange" => item_callbacks::dispatch_on_terrain_change(self, item_id.as_str(), pokemon_pos),
            "TrapPokemon" => item_callbacks::dispatch_on_trap_pokemon(self, item_id.as_str(), pokemon_pos),
            "TrapPokemonPriority" => item_callbacks::dispatch_on_trap_pokemon_priority(self, item_id.as_str(), pokemon_pos),
            "TryBoost" => item_callbacks::dispatch_on_try_boost(self, item_id.as_str(), pokemon_pos),
            "TryBoostPriority" => item_callbacks::dispatch_on_try_boost_priority(self, item_id.as_str(), pokemon_pos),
            "TryEatItem" => item_callbacks::dispatch_on_try_eat_item(self, item_id.as_str(), pokemon_pos),
            "TryHeal" => item_callbacks::dispatch_on_try_heal(self, item_id.as_str(), pokemon_pos),
            "TryHealPriority" => item_callbacks::dispatch_on_try_heal_priority(self, item_id.as_str(), pokemon_pos),
            "TryHit" => item_callbacks::dispatch_on_try_hit(self, item_id.as_str(), pokemon_pos),
            "Update" => item_callbacks::dispatch_on_update(self, item_id.as_str(), pokemon_pos),
            "Use" => item_callbacks::dispatch_on_use(self, item_id.as_str(), pokemon_pos),
            "UseItem" => item_callbacks::dispatch_on_use_item(self, item_id.as_str(), pokemon_pos),
            _ => EventResult::Continue,
        }
    }

    /// Handle move events
    /// Rust helper method - JavaScript's singleEvent() directly invokes move[`on${eventId}`] callbacks
    /// This method dispatches to move_callbacks module based on event name
    /// Routes to move-specific handlers for all event types (AfterHit, BasePower, Damage, etc.)
    fn handle_move_event(
        &mut self,
        event_id: &str,
        move_id: &str,
        target: Option<(usize, usize)>,
    ) -> crate::event::EventResult {
        use crate::event::EventResult;
        use crate::data::move_callbacks;

        let source = self.current_event.as_ref().and_then(|e| e.source);
        let source_pos = source.unwrap_or((0, 0));
        let target_pos = target.unwrap_or((0, 0));

        match event_id {
            "AfterHit" => {
                if let Some(target_pos) = target {
                    move_callbacks::dispatch_on_after_hit(self, move_id, source_pos, target_pos)
                } else {
                    EventResult::Continue
                }
            }
            "AfterMove" => move_callbacks::dispatch_on_after_move(self, move_id, source_pos, target),
            "AfterMoveSecondarySelf" => move_callbacks::dispatch_on_after_move_secondary_self(self, move_id, source_pos, target),
            "AfterSubDamage" => {
                // TODO: AfterSubDamage needs damage value from relay_var
                move_callbacks::dispatch_on_after_sub_damage(self, move_id, source_pos, 0, target)
            }
            "BasePower" => {
                // TODO: BasePower event needs base_power value from relay_var
                // This requires architectural changes to thread relay_var through dispatch
                EventResult::Continue
            }
            "Damage" => {
                // TODO: Damage event needs damage, target_pos, source_pos, and effect_id
                // This requires architectural changes to thread these values through dispatch
                EventResult::Continue
            }
            "DamagePriority" => {
                // No moves implement DamagePriority event
                EventResult::Continue
            }
            "DisableMove" => move_callbacks::dispatch_on_disable_move(self, move_id, source_pos),
            "Effectiveness" => move_callbacks::dispatch_on_effectiveness(self, move_id, source_pos),
            "Hit" => {
                if let Some(target_pos) = target {
                    move_callbacks::dispatch_on_hit(self, move_id, source_pos, Some(target_pos))
                } else {
                    EventResult::Continue
                }
            }
            "HitField" => move_callbacks::dispatch_on_hit_field(self, move_id, source_pos, target),
            "HitSide" => move_callbacks::dispatch_on_hit_side(self, move_id, source_pos),
            "ModifyMove" => move_callbacks::dispatch_on_modify_move(self, move_id, source_pos, target),
            "ModifyPriority" => move_callbacks::dispatch_on_modify_priority(self, move_id, source_pos),
            "ModifyTarget" => move_callbacks::dispatch_on_modify_target(self, move_id, source_pos),
            "ModifyType" => move_callbacks::dispatch_on_modify_type(self, move_id, source_pos),
            "MoveFail" => move_callbacks::dispatch_on_move_fail(self, move_id, source_pos),
            "PrepareHit" => move_callbacks::dispatch_on_prepare_hit(self, move_id, source_pos, target),
            "Try" => move_callbacks::dispatch_on_try(self, move_id, source_pos, target),
            "TryHit" => {
                if let Some(target_pos) = target {
                    move_callbacks::dispatch_on_try_hit(self, move_id, source_pos, target_pos)
                } else {
                    EventResult::Continue
                }
            }
            "TryImmunity" => move_callbacks::dispatch_on_try_immunity(self, move_id, source_pos),
            "TryMove" => move_callbacks::dispatch_on_try_move(self, move_id, source_pos, target),
            "UseMoveMessage" => move_callbacks::dispatch_on_use_move_message(self, move_id, source_pos),
            _ => EventResult::Continue,
        }
    }

    /// Check volatile condition TryHit events
    /// Returns true if the move should proceed, false if blocked (e.g., by Protect)
    fn check_volatile_try_hit(
        &mut self,
        target: (usize, usize),
        source: (usize, usize),
        move_id: &ID,
    ) -> bool {
        let (target_side, target_idx) = target;

        // Get list of volatiles on the target
        let volatile_ids: Vec<ID> = if let Some(side) = self.sides.get(target_side) {
            if let Some(pokemon) = side.pokemon.get(target_idx) {
                pokemon.volatiles.keys().cloned().collect()
            } else {
                return true; // No target, let move proceed
            }
        } else {
            return true; // No target, let move proceed
        };

        // Check each volatile for onTryHit callback
        for volatile_id in volatile_ids {
            match volatile_id.as_str() {
                "banefulbunker" | "protect" => {
                    // TODO: Call the condition's onTryHit callback when move_callbacks is reimplemented
                    // For now, Protect-like moves don't fully block in this stub
                }
                _ => {
                    // TODO: Add other volatile conditions with onTryHit here
                }
            }
        }

        // Move proceeds
        true
    }

    /// Handle condition events (status, volatile, weather, terrain)
    /// Rust helper method - JavaScript's singleEvent() directly invokes condition[`on${eventId}`] callbacks
    /// This method dispatches to condition_callbacks module based on event name
    /// Routes to condition-specific handlers for all event types (Residual, BeforeMove, Weather, etc.)
    fn handle_condition_event(
        &mut self,
        event_id: &str,
        condition_id: &str,
        target: Option<(usize, usize)>,
    ) -> crate::event::EventResult {
        use crate::event::EventResult;
        use crate::data::condition_callbacks;

        let pokemon_pos = target.unwrap_or((0, 0));

        match event_id {
            "AfterMove" => condition_callbacks::dispatch_on_after_move(self, condition_id, pokemon_pos),
            "AfterMoveSecondary" => condition_callbacks::dispatch_on_after_move_secondary(self, condition_id, pokemon_pos),
            "BasePower" => condition_callbacks::dispatch_on_base_power(self, condition_id, pokemon_pos),
            "BasePowerPriority" => condition_callbacks::dispatch_on_base_power_priority(self, condition_id, pokemon_pos),
            "BeforeMove" => condition_callbacks::dispatch_on_before_move(self, condition_id, pokemon_pos),
            "BeforeMovePriority" => condition_callbacks::dispatch_on_before_move_priority(self, condition_id, pokemon_pos),
            "BeforeSwitchOut" => condition_callbacks::dispatch_on_before_switch_out(self, condition_id, pokemon_pos),
            "BeforeSwitchOutPriority" => condition_callbacks::dispatch_on_before_switch_out_priority(self, condition_id, pokemon_pos),
            "BeforeTurn" => condition_callbacks::dispatch_on_before_turn(self, condition_id, pokemon_pos),
            "DamagingHit" => condition_callbacks::dispatch_on_damaging_hit(self, condition_id, pokemon_pos),
            "DisableMove" => condition_callbacks::dispatch_on_disable_move(self, condition_id, pokemon_pos),
            "DragOut" => condition_callbacks::dispatch_on_drag_out(self, condition_id, pokemon_pos),
            "DragOutPriority" => condition_callbacks::dispatch_on_drag_out_priority(self, condition_id, pokemon_pos),
            "Effectiveness" => condition_callbacks::dispatch_on_effectiveness(self, condition_id, pokemon_pos),
            "EffectivenessPriority" => condition_callbacks::dispatch_on_effectiveness_priority(self, condition_id, pokemon_pos),
            "End" => condition_callbacks::dispatch_on_end(self, condition_id, pokemon_pos),
            "FieldEnd" => condition_callbacks::dispatch_on_field_end(self, condition_id, pokemon_pos),
            "FieldResidual" => condition_callbacks::dispatch_on_field_residual(self, condition_id, pokemon_pos),
            "FieldResidualOrder" => condition_callbacks::dispatch_on_field_residual_order(self, condition_id, pokemon_pos),
            "FieldStart" => condition_callbacks::dispatch_on_field_start(self, condition_id, pokemon_pos),
            "Immunity" => condition_callbacks::dispatch_on_immunity(self, condition_id, pokemon_pos),
            "Invulnerability" => condition_callbacks::dispatch_on_invulnerability(self, condition_id, pokemon_pos),
            "LockMove" => condition_callbacks::dispatch_on_lock_move(self, condition_id, pokemon_pos),
            "ModifyDef" => condition_callbacks::dispatch_on_modify_def(self, condition_id, pokemon_pos),
            "ModifyDefPriority" => condition_callbacks::dispatch_on_modify_def_priority(self, condition_id, pokemon_pos),
            "ModifyMove" => condition_callbacks::dispatch_on_modify_move(self, condition_id, pokemon_pos),
            "ModifySpD" => condition_callbacks::dispatch_on_modify_sp_d(self, condition_id, pokemon_pos),
            "ModifySpDPriority" => condition_callbacks::dispatch_on_modify_sp_d_priority(self, condition_id, pokemon_pos),
            "ModifySpe" => condition_callbacks::dispatch_on_modify_spe(self, condition_id, pokemon_pos),
            "ModifySpePriority" => condition_callbacks::dispatch_on_modify_spe_priority(self, condition_id, pokemon_pos),
            "MoveAborted" => condition_callbacks::dispatch_on_move_aborted(self, condition_id, pokemon_pos),
            "Residual" => condition_callbacks::dispatch_on_residual(self, condition_id, pokemon_pos),
            "ResidualOrder" => condition_callbacks::dispatch_on_residual_order(self, condition_id, pokemon_pos),
            "ResidualPriority" => condition_callbacks::dispatch_on_residual_priority(self, condition_id, pokemon_pos),
            "Restart" => condition_callbacks::dispatch_on_restart(self, condition_id, pokemon_pos),
            "SourceModifyDamage" => condition_callbacks::dispatch_on_source_modify_damage(self, condition_id, pokemon_pos),
            "StallMove" => condition_callbacks::dispatch_on_stall_move(self, condition_id, pokemon_pos),
            "Start" => condition_callbacks::dispatch_on_start(self, condition_id, pokemon_pos),
            "SwitchIn" => condition_callbacks::dispatch_on_switch_in(self, condition_id, pokemon_pos),
            "TrapPokemon" => condition_callbacks::dispatch_on_trap_pokemon(self, condition_id, pokemon_pos),
            "TrapPokemonPriority" => condition_callbacks::dispatch_on_trap_pokemon_priority(self, condition_id, pokemon_pos),
            "TryAddVolatile" => condition_callbacks::dispatch_on_try_add_volatile(self, condition_id, pokemon_pos),
            "TryMove" => condition_callbacks::dispatch_on_try_move(self, condition_id, pokemon_pos),
            "TryMovePriority" => condition_callbacks::dispatch_on_try_move_priority(self, condition_id, pokemon_pos),
            "Type" => condition_callbacks::dispatch_on_type(self, condition_id, pokemon_pos),
            "TypePriority" => condition_callbacks::dispatch_on_type_priority(self, condition_id, pokemon_pos),
            "Weather" => condition_callbacks::dispatch_on_weather(self, condition_id, pokemon_pos),
            "WeatherModifyDamage" => condition_callbacks::dispatch_on_weather_modify_damage(self, condition_id, pokemon_pos),
            _ => EventResult::Continue,
        }
    }

    /// Handle side condition events (SideStart, SideEnd, AnyModifyDamage, etc.)
    /// Calls the appropriate callback for each side condition
    /// Rust helper method - JavaScript's singleEvent() directly invokes side condition callbacks
    /// This method dispatches to move_callbacks for side condition events
    /// Currently only implements auroraveil - more side conditions need to be added
    fn handle_side_condition_event(
        &mut self,
        event_id: &str,
        side_idx: usize,
        condition_id: &ID,
    ) {
        match condition_id.as_str() {
            "auroraveil" => {
                match event_id {
                    "SideStart" => {
//                         let _result = crate::data::move_callbacks::auroraveil::on_side_start(self, side_idx);
                    }
                    "SideEnd" => {
//                         let _result = crate::data::move_callbacks::auroraveil::on_side_end(self, side_idx);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    /// Run event on all relevant handlers
    /// Equivalent to battle.ts runEvent()
    ///
    /// This is a simplified version that handles common event patterns.
    // TypeScript source:
    // /**
    // 	 * runEvent is the core of Pokemon Showdown's event system.
    // 	 *
    // 	 * Basic usage
    // 	 * ===========
    // 	 *
    // 	 *   this.runEvent('Blah')
    // 	 * will trigger any onBlah global event handlers.
    // 	 *
    // 	 *   this.runEvent('Blah', target)
    // 	 * will additionally trigger any onBlah handlers on the target, onAllyBlah
    // 	 * handlers on any active pokemon on the target's team, and onFoeBlah
    // 	 * handlers on any active pokemon on the target's foe's team
    // 	 *
    // 	 *   this.runEvent('Blah', target, source)
    // 	 * will additionally trigger any onSourceBlah handlers on the source
    // 	 *
    // 	 *   this.runEvent('Blah', target, source, effect)
    // 	 * will additionally pass the effect onto all event handlers triggered
    // 	 *
    // 	 *   this.runEvent('Blah', target, source, effect, relayVar)
    // 	 * will additionally pass the relayVar as the first argument along all event
    // 	 * handlers
    // 	 *
    // 	 * You may leave any of these null. For instance, if you have a relayVar but
    // 	 * no source or effect:
    // 	 *   this.runEvent('Damage', target, null, null, 50)
    // 	 *
    // 	 * Event handlers
    // 	 * ==============
    // 	 *
    // 	 * Items, abilities, statuses, and other effects like SR, confusion, weather,
    // 	 * or Trick Room can have event handlers. Event handlers are functions that
    // 	 * can modify what happens during an event.
    // 	 *
    // 	 * event handlers are passed:
    // 	 *   function (target, source, effect)
    // 	 * although some of these can be blank.
    // 	 *
    // 	 * certain events have a relay variable, in which case they're passed:
    // 	 *   function (relayVar, target, source, effect)
    // 	 *
    // 	 * Relay variables are variables that give additional information about the
    // 	 * event. For instance, the damage event has a relayVar which is the amount
    // 	 * of damage dealt.
    // 	 *
    // 	 * If a relay variable isn't passed to runEvent, there will still be a secret
    // 	 * relayVar defaulting to `true`, but it won't get passed to any event
    // 	 * handlers.
    // 	 *
    // 	 * After an event handler is run, its return value helps determine what
    // 	 * happens next:
    // 	 * 1. If the return value isn't `undefined`, relayVar is set to the return
    // 	 *    value
    // 	 * 2. If relayVar is falsy, no more event handlers are run
    // 	 * 3. Otherwise, if there are more event handlers, the next one is run and
    // 	 *    we go back to step 1.
    // 	 * 4. Once all event handlers are run (or one of them results in a falsy
    // 	 *    relayVar), relayVar is returned by runEvent
    // 	 *
    // 	 * As a shortcut, an event handler that isn't a function will be interpreted
    // 	 * as a function that returns that value.
    // 	 *
    // 	 * You can have return values mean whatever you like, but in general, we
    // 	 * follow the convention that returning `false` or `null` means
    // 	 * stopping or interrupting the event.
    // 	 *
    // 	 * For instance, returning `false` from a TrySetStatus handler means that
    // 	 * the pokemon doesn't get statused.
    // 	 *
    // 	 * If a failed event usually results in a message like "But it failed!"
    // 	 * or "It had no effect!", returning `null` will suppress that message and
    // 	 * returning `false` will display it. Returning `null` is useful if your
    // 	 * event handler already gave its own custom failure message.
    // 	 *
    // 	 * Returning `undefined` means "don't change anything" or "keep going".
    // 	 * A function that does nothing but return `undefined` is the equivalent
    // 	 * of not having an event handler at all.
    // 	 *
    // 	 * Returning a value means that that value is the new `relayVar`. For
    // 	 * instance, if a Damage event handler returns 50, the damage event
    // 	 * will deal 50 damage instead of whatever it was going to deal before.
    // 	 *
    // 	 * Useful values
    // 	 * =============
    // 	 *
    // 	 * In addition to all the methods and attributes of Dex, Battle, and
    // 	 * Scripts, event handlers have some additional values they can access:
    // 	 *
    // 	 * this.effect:
    // 	 *   the Effect having the event handler
    // 	 * this.effectState:
    // 	 *   the data store associated with the above Effect. This is a plain Object
    // 	 *   and you can use it to store data for later event handlers.
    // 	 * this.effectState.target:
    // 	 *   the Pokemon, Side, or Battle that the event handler's effect was
    // 	 *   attached to.
    // 	 * this.event.id:
    // 	 *   the event ID
    // 	 * this.event.target, this.event.source, this.event.effect:
    // 	 *   the target, source, and effect of the event. These are the same
    // 	 *   variables that are passed as arguments to the event handler, but
    // 	 *   they're useful for functions called by the event handler.
    // 	 */
    // 	runEvent(
    // 		eventid: string, target?: Pokemon | Pokemon[] | Side | Battle | null, source?: string | Pokemon | false | null,
    // 		sourceEffect?: Effect | null, relayVar?: any, onEffect?: boolean, fastExit?: boolean
    // 	) {
    // 		// if (Battle.eventCounter) {
    // 		// 	if (!Battle.eventCounter[eventid]) Battle.eventCounter[eventid] = 0;
    // 		// 	Battle.eventCounter[eventid]++;
    // 		// }
    // 		if (this.eventDepth >= 8) {
    // 			// oh fuck
    // 			this.add('message', 'STACK LIMIT EXCEEDED');
    // 			this.add('message', 'PLEASE REPORT IN BUG THREAD');
    // 			this.add('message', 'Event: ' + eventid);
    // 			this.add('message', 'Parent event: ' + this.event.id);
    // 			throw new Error("Stack overflow");
    // 		}
    // 		if (!target) target = this;
    // 		let effectSource = null;
    // 		if (source instanceof Pokemon) effectSource = source;
    // 		const handlers = this.findEventHandlers(target, eventid, effectSource);
    // 		if (onEffect) {
    // 			if (!sourceEffect) throw new Error("onEffect passed without an effect");
    // 			const callback = (sourceEffect as any)[`on${eventid}`];
    // 			if (callback !== undefined) {
    // 				if (Array.isArray(target)) throw new Error("");
    // 				handlers.unshift(this.resolvePriority({
    // 					effect: sourceEffect, callback, state: this.initEffectState({}), end: null, effectHolder: target,
    // 				}, `on${eventid}`));
    // 			}
    // 		}
    // 
    // 		if (['Invulnerability', 'TryHit', 'DamagingHit', 'EntryHazard'].includes(eventid)) {
    // 			handlers.sort(Battle.compareLeftToRightOrder);
    // 		} else if (fastExit) {
    // 			handlers.sort(Battle.compareRedirectOrder);
    // 		} else {
    // 			this.speedSort(handlers);
    // 		}
    // 		let hasRelayVar = 1;
    // 		const args = [target, source, sourceEffect];
    // 		// console.log('Event: ' + eventid + ' (depth ' + this.eventDepth + ') t:' + target.id + ' s:' + (!source || source.id) + ' e:' + effect.id);
    // 		if (relayVar === undefined || relayVar === null) {
    // 			relayVar = true;
    // 			hasRelayVar = 0;
    // 		} else {
    // 			args.unshift(relayVar);
    // 		}
    // 
    // 		const parentEvent = this.event;
    // 		this.event = { id: eventid, target, source, effect: sourceEffect, modifier: 1 };
    // 		this.eventDepth++;
    // 
    // 		let targetRelayVars = [];
    // 		if (Array.isArray(target)) {
    // 			if (Array.isArray(relayVar)) {
    // 				targetRelayVars = relayVar;
    // 			} else {
    // 				for (let i = 0; i < target.length; i++) targetRelayVars[i] = true;
    // 			}
    // 		}
    // 		for (const handler of handlers) {
    // 			if (handler.index !== undefined) {
    // 				// TODO: find a better way to do this
    // 				if (!targetRelayVars[handler.index] && !(targetRelayVars[handler.index] === 0 &&
    // 					eventid === 'DamagingHit')) continue;
    // 				if (handler.target) {
    // 					args[hasRelayVar] = handler.target;
    // 					this.event.target = handler.target;
    // 				}
    // 				if (hasRelayVar) args[0] = targetRelayVars[handler.index];
    // 			}
    // 			const effect = handler.effect;
    // 			const effectHolder = handler.effectHolder;
    // 			// this.debug('match ' + eventid + ': ' + status.id + ' ' + status.effectType);
    // 			if (effect.effectType === 'Status' && (effectHolder as Pokemon).status !== effect.id) {
    // 				// it's changed; call it off
    // 				continue;
    // 			}
    // 			if (effect.effectType === 'Ability' && effect.flags['breakable'] &&
    // 				this.suppressingAbility(effectHolder as Pokemon)) {
    // 				if (effect.flags['breakable']) {
    // 					this.debug(eventid + ' handler suppressed by Mold Breaker');
    // 					continue;
    // 				}
    // 				if (!effect.num) {
    // 					// ignore attacking events for custom abilities
    // 					const AttackingEvents = {
    // 						BeforeMove: 1,
    // 						BasePower: 1,
    // 						Immunity: 1,
    // 						RedirectTarget: 1,
    // 						Heal: 1,
    // 						SetStatus: 1,
    // 						CriticalHit: 1,
    // 						ModifyAtk: 1, ModifyDef: 1, ModifySpA: 1, ModifySpD: 1, ModifySpe: 1, ModifyAccuracy: 1,
    // 						ModifyBoost: 1,
    // 						ModifyDamage: 1,
    // 						ModifySecondaries: 1,
    // 						ModifyWeight: 1,
    // 						TryAddVolatile: 1,
    // 						TryHit: 1,
    // 						TryHitSide: 1,
    // 						TryMove: 1,
    // 						Boost: 1,
    // 						DragOut: 1,
    // 						Effectiveness: 1,
    // 					};
    // 					if (eventid in AttackingEvents) {
    // 						this.debug(eventid + ' handler suppressed by Mold Breaker');
    // 						continue;
    // 					} else if (eventid === 'Damage' && sourceEffect && sourceEffect.effectType === 'Move') {
    // 						this.debug(eventid + ' handler suppressed by Mold Breaker');
    // 						continue;
    // 					}
    // 				}
    // 			}
    // 			if (eventid !== 'Start' && eventid !== 'SwitchIn' && eventid !== 'TakeItem' &&
    // 				effect.effectType === 'Item' && (effectHolder instanceof Pokemon) && effectHolder.ignoringItem()) {
    // 				if (eventid !== 'Update') {
    // 					this.debug(eventid + ' handler suppressed by Embargo, Klutz or Magic Room');
    // 				}
    // 				continue;
    // 			} else if (
    // 				eventid !== 'End' && effect.effectType === 'Ability' &&
    // 				(effectHolder instanceof Pokemon) && effectHolder.ignoringAbility()
    // 			) {
    // 				if (eventid !== 'Update') {
    // 					this.debug(eventid + ' handler suppressed by Gastro Acid or Neutralizing Gas');
    // 				}
    // 				continue;
    // 			}
    // 			if (
    // 				(effect.effectType === 'Weather' || eventid === 'Weather') &&
    // 				eventid !== 'Residual' && eventid !== 'End' && this.field.suppressingWeather()
    // 			) {
    // 				this.debug(eventid + ' handler suppressed by Air Lock');
    // 				continue;
    // 			}
    // 			let returnVal;
    // 			if (typeof handler.callback === 'function') {
    // 				const parentEffect = this.effect;
    // 				const parentEffectState = this.effectState;
    // 				this.effect = handler.effect;
    // 				this.effectState = handler.state || this.initEffectState({});
    // 				this.effectState.target = effectHolder;
    // 
    // 				returnVal = handler.callback.apply(this, args);
    // 
    // 				this.effect = parentEffect;
    // 				this.effectState = parentEffectState;
    // 			} else {
    // 				returnVal = handler.callback;
    // 			}
    // 
    // 			if (returnVal !== undefined) {
    // 				relayVar = returnVal;
    // 				if (!relayVar || fastExit) {
    // 					if (handler.index !== undefined) {
    // 						targetRelayVars[handler.index] = relayVar;
    // 						if (targetRelayVars.every(val => !val)) break;
    // 					} else {
    // 						break;
    // 					}
    // 				}
    // 				if (hasRelayVar) {
    // 					args[0] = relayVar;
    // 				}
    // 			}
    // 		}
    // 
    // 		this.eventDepth--;
    // 		if (typeof relayVar === 'number' && relayVar === Math.abs(Math.floor(relayVar))) {
    // 			// this.debug(eventid + ' modifier: 0x' +
    // 			// 	('0000' + (this.event.modifier * 4096).toString(16)).slice(-4).toUpperCase());
    // 			relayVar = this.modify(relayVar, this.event.modifier);
    // 		}
    // 		this.event = parentEvent;
    // 
    // 		return Array.isArray(target) ? targetRelayVars : relayVar;
    // 	}
    //
    pub fn run_event(
        &mut self,
        event_id: &str,
        target: Option<(usize, usize)>,
        source: Option<(usize, usize)>,
        source_effect: Option<&ID>,
        relay_var: Option<i32>,
    ) -> Option<i32> {
        use crate::event::EventResult;

        // Check stack depth
        if self.event_depth >= 8 {
            self.add_log("message", &["STACK LIMIT EXCEEDED"]);
            return None;
        }

        // Save parent event context
        let parent_event = self.current_event.take();
        self.event_depth += 1;

        // Set up current event
        self.current_event = Some(EventInfo {
            id: event_id.to_string(),
            target,
            source,
            effect: source_effect.cloned(),
            modifier: 4096,
        });

        let mut result = relay_var;

        // Find and run all handlers for this event
        let handlers = self.find_event_handlers(event_id, target, source);

        for (effect_id, holder_target) in handlers {
            let event_result = self.dispatch_single_event(event_id, &effect_id, holder_target, source);

            match event_result {
                EventResult::Boolean(false) => {
                    result = None;
                    break;
                }
                EventResult::Stop => {
                    break;
                }
                EventResult::Number(n) => {
                    // Numbers are in 4096 basis points (e.g., 4096 = 1.0x, 6144 = 1.5x, 2048 = 0.5x)
                    if let Some(ref mut r) = result {
                        *r = (*r as f64 * (n as f64 / 4096.0)) as i32;
                    }
                }
                _ => {}
            }
        }

        // Run custom event handlers (registered via onEvent in tests)
        if let Some(custom_result) = self.run_custom_event_handlers(event_id) {
            result = Some(custom_result);
        }

        // Apply event modifier if we have a numeric result
        if let (Some(ref mut r), Some(ref event)) = (&mut result, &self.current_event) {
            if event.modifier != 4096 {
                *r = self.modify_internal(*r, event.modifier);
            }
        }

        // Restore parent context
        self.event_depth -= 1;
        self.current_event = parent_event;

        result
    }

    /// Run event and return boolean
    /// Rust convenience method - JavaScript runEvent() returns various types (undefined, null, number, etc.)
    /// This method simplifies the API by returning true if event succeeded (returned Some value), false otherwise
    /// Used when callers only need to know if the event succeeded, not the actual relay value
    pub fn run_event_bool(
        &mut self,
        event_id: &str,
        target: Option<(usize, usize)>,
        source: Option<(usize, usize)>,
        source_effect: Option<&ID>,
    ) -> bool {
        self.run_event(event_id, target, source, source_effect, Some(1)).is_some()
    }

    /// Find all event handlers for an event
    /// Equivalent to battle.ts findEventHandlers()
    ///
    /// JavaScript (battle.ts:1036-1096):
    /// - Handles array targets recursively
    /// - Distinguishes Pokemon/Side/Battle types
    /// - Implements bubble down for Side events
    /// - Adds prefixed handlers (onAlly, onFoe, onAny, onSource)
    ///
    /// Rust Implementation Notes:
    /// - Currently simplified: only handles Pokemon targets (no Side/Battle distinction)
    /// - No array recursion (not used in current Rust codebase)
    /// - Calls specialized find*EventHandlers methods
    /// - TODO: Add prefixed handler support when needed
    // 
    // 	findEventHandlers(target: Pokemon | Pokemon[] | Side | Battle, eventName: string, source?: Pokemon | null) {
    // 		let handlers: EventListener[] = [];
    // 		if (Array.isArray(target)) {
    // 			for (const [i, pokemon] of target.entries()) {
    // 				// console.log(`Event: ${eventName}, Target: ${pokemon}, ${i}`);
    // 				const curHandlers = this.findEventHandlers(pokemon, eventName, source);
    // 				for (const handler of curHandlers) {
    // 					handler.target = pokemon; // Original "effectHolder"
    // 					handler.index = i;
    // 				}
    // 				handlers = handlers.concat(curHandlers);
    // 			}
    // 			return handlers;
    // 		}
    // 		// events that target a Pokemon normally bubble up to the Side
    // 		const shouldBubbleDown = target instanceof Side;
    // 		// events usually run through EachEvent should never have any handlers besides `on${eventName}` so don't check for them
    // 		const prefixedHandlers = !['BeforeTurn', 'Update', 'Weather', 'WeatherChange', 'TerrainChange'].includes(eventName);
    // 		if (target instanceof Pokemon && (target.isActive || source?.isActive)) {
    // 			handlers = this.findPokemonEventHandlers(target, `on${eventName}`);
    // 			if (prefixedHandlers) {
    // 				for (const allyActive of target.alliesAndSelf()) {
    // 					handlers.push(...this.findPokemonEventHandlers(allyActive, `onAlly${eventName}`));
    // 					handlers.push(...this.findPokemonEventHandlers(allyActive, `onAny${eventName}`));
    // 				}
    // 				for (const foeActive of target.foes()) {
    // 					handlers.push(...this.findPokemonEventHandlers(foeActive, `onFoe${eventName}`));
    // 					handlers.push(...this.findPokemonEventHandlers(foeActive, `onAny${eventName}`));
    // 				}
    // 			}
    // 			target = target.side;
    // 		}
    // 		if (source && prefixedHandlers) {
    // 			handlers.push(...this.findPokemonEventHandlers(source, `onSource${eventName}`));
    // 		}
    // 		if (target instanceof Side) {
    // 			for (const side of this.sides) {
    // 				if (shouldBubbleDown) {
    // 					for (const active of side.active) {
    // 						if (side === target || side === target.allySide) {
    // 							handlers = handlers.concat(this.findPokemonEventHandlers(active, `on${eventName}`));
    // 						} else if (prefixedHandlers) {
    // 							handlers = handlers.concat(this.findPokemonEventHandlers(active, `onFoe${eventName}`));
    // 						}
    // 						if (prefixedHandlers) handlers = handlers.concat(this.findPokemonEventHandlers(active, `onAny${eventName}`));
    // 					}
    // 				}
    // 				if (side.n < 2 || !side.allySide) {
    // 					if (side === target || side === target.allySide) {
    // 						handlers.push(...this.findSideEventHandlers(side, `on${eventName}`));
    // 					} else if (prefixedHandlers) {
    // 						handlers.push(...this.findSideEventHandlers(side, `onFoe${eventName}`));
    // 					}
    // 					if (prefixedHandlers) handlers.push(...this.findSideEventHandlers(side, `onAny${eventName}`));
    // 				}
    // 			}
    // 		}
    // 		handlers.push(...this.findFieldEventHandlers(this.field, `on${eventName}`));
    // 		handlers.push(...this.findBattleEventHandlers(`on${eventName}`));
    // 		return handlers;
    // 	}
    //
    fn find_event_handlers(
        &self,
        event_id: &str,
        target: Option<(usize, usize)>,
        source: Option<(usize, usize)>,
    ) -> Vec<(ID, Option<(usize, usize)>)> {
        let mut handlers = Vec::new();

        // JavaScript: const prefixedHandlers = !['BeforeTurn', 'Update', 'Weather', 'WeatherChange', 'TerrainChange'].includes(eventName);
        let event_name = event_id.trim_start_matches("on");
        let prefixed_handlers = !matches!(event_name, "BeforeTurn" | "Update" | "Weather" | "WeatherChange" | "TerrainChange");

        // JavaScript: if (target instanceof Pokemon && (target.isActive || source?.isActive))
        // Rust: We only handle Pokemon targets currently
        if let Some(target_pos) = target {
            // JavaScript: handlers = this.findPokemonEventHandlers(target, `on${eventName}`);
            let mut pokemon_handlers = self.find_pokemon_event_handlers(event_id, target_pos);
            handlers.append(&mut pokemon_handlers);

            if prefixed_handlers {
                let (target_side, _target_idx) = target_pos;

                // Add prefixed handlers (onAlly, onFoe, onAny)
                // JavaScript:
                // for (const allyActive of target.alliesAndSelf()) {
                //     handlers.push(...this.findPokemonEventHandlers(allyActive, `onAlly${eventName}`));
                //     handlers.push(...this.findPokemonEventHandlers(allyActive, `onAny${eventName}`));
                // }

                // Get all active Pokemon on target's side (allies and self)
                if let Some(side) = self.sides.get(target_side) {
                    for (slot_idx, opt_poke_idx) in side.active.iter().enumerate() {
                        if let Some(poke_idx) = opt_poke_idx {
                            let ally_pos = (target_side, *poke_idx);
                            // onAlly handlers
                            let ally_event = format!("onAlly{}", event_name);
                            let mut ally_handlers = self.find_pokemon_event_handlers(&ally_event, ally_pos);
                            handlers.append(&mut ally_handlers);

                            // onAny handlers
                            let any_event = format!("onAny{}", event_name);
                            let mut any_handlers = self.find_pokemon_event_handlers(&any_event, ally_pos);
                            handlers.append(&mut any_handlers);
                        }
                    }
                }

                // JavaScript:
                // for (const foeActive of target.foes()) {
                //     handlers.push(...this.findPokemonEventHandlers(foeActive, `onFoe${eventName}`));
                //     handlers.push(...this.findPokemonEventHandlers(foeActive, `onAny${eventName}`));
                // }

                // Get all active Pokemon on opposing side(s) (foes)
                for (side_idx, side) in self.sides.iter().enumerate() {
                    if side_idx != target_side {
                        for (slot_idx, opt_poke_idx) in side.active.iter().enumerate() {
                            if let Some(poke_idx) = opt_poke_idx {
                                let foe_pos = (side_idx, *poke_idx);
                                // onFoe handlers
                                let foe_event = format!("onFoe{}", event_name);
                                let mut foe_handlers = self.find_pokemon_event_handlers(&foe_event, foe_pos);
                                handlers.append(&mut foe_handlers);

                                // onAny handlers
                                let any_event = format!("onAny{}", event_name);
                                let mut any_handlers = self.find_pokemon_event_handlers(&any_event, foe_pos);
                                handlers.append(&mut any_handlers);
                            }
                        }
                    }
                }
            }
        }

        // JavaScript: if (source && prefixedHandlers) {
        //     handlers.push(...this.findPokemonEventHandlers(source, `onSource${eventName}`));
        // }
        if let Some(source_pos) = source {
            if prefixed_handlers {
                let source_event = format!("onSource{}", event_name);
                let mut source_handlers = self.find_pokemon_event_handlers(&source_event, source_pos);
                handlers.append(&mut source_handlers);
            }
        }

        // JavaScript: handlers.push(...this.findFieldEventHandlers(this.field, `on${eventName}`));
        let mut field_handlers = self.find_field_event_handlers(event_id);
        handlers.append(&mut field_handlers);

        // JavaScript: handlers.push(...this.findBattleEventHandlers(`on${eventName}`));
        let battle_handler_ids = self.find_battle_event_handlers(event_id);
        for id in battle_handler_ids {
            handlers.push((id, None));
        }

        handlers
    }

    /// Priority event - exits on first non-undefined result
    /// Equivalent to battle.ts priorityEvent()
    // TypeScript source:
    // /**
    // 	 * priorityEvent works just like runEvent, except it exits and returns
    // 	 * on the first non-undefined value instead of only on null/false.
    // 	 */
    // 	priorityEvent(
    // 		eventid: string, target: Pokemon | Side | Battle, source?: Pokemon | null,
    // 		effect?: Effect, relayVar?: any, onEffect?: boolean
    // 	): any {
    // 		return this.runEvent(eventid, target, source, effect, relayVar, onEffect, true);
    // 	}
    //
    pub fn priority_event(
        &mut self,
        event_id: &str,
        target: Option<(usize, usize)>,
        source: Option<(usize, usize)>,
        effect: Option<&ID>,
        relay_var: Option<i32>,
    ) -> Option<i32> {
        // For priority events, we use fastExit behavior
        self.run_event(event_id, target, source, effect, relay_var)
    }

    /// Get event modifier (4096 = 1.0x)
    /// Rust convenience method - JavaScript accesses this.event.modifier directly
    /// This method provides safe access to current event's modifier value
    /// Returns 4096 (1.0x) if no event is active
    pub fn get_event_modifier(&self) -> i32 {
        self.current_event.as_ref().map(|e| e.modifier).unwrap_or(4096)
    }

    /// Set event modifier (for chainModify pattern, 4096 = 1.0x)
    /// Rust convenience method - JavaScript sets this.event.modifier directly
    /// This method chains modifiers by multiplying: modifier = (current * new + 2048) >> 12
    /// Used by chainModify() to accumulate multiple damage/stat modifiers
    pub fn set_event_modifier(&mut self, modifier: i32) {
        if let Some(ref mut event) = self.current_event {
            // Chain modifiers by multiplying in 4096 basis points
            let current = event.modifier as i64;
            let new = modifier as i64;
            event.modifier = ((current * new + 2048) >> 12) as i32;
        }
    }

    // =========================================================================
    // ADDITIONAL METHODS (ported from battle.ts)
    // =========================================================================

    /// Apply damage randomization (85%-100%)
    /// Equivalent to battle.ts randomizer(baseDamage)
    ///
    // 
    // 	randomizer(baseDamage: number) {
    // 		const tr = this.trunc;
    // 		return tr(tr(baseDamage * (100 - this.random(16))) / 100);
    // 	}
    //
    pub fn randomizer(&mut self, base_damage: i32) -> i32 {
        // JS: return tr(tr(baseDamage * (100 - this.random(16))) / 100);
        // Damage = baseDamage * (100 - random(16)) / 100
        // This gives range 85% to 100% damage
        let roll = 100 - self.prng.random_int(16) as i32;
        let inner = self.trunc(base_damage as f64 * roll as f64);
        self.trunc(inner as f64 / 100.0)
    }

    /// Run an event on each active Pokemon in speed order
    /// Equivalent to battle.ts eachEvent()
    // TypeScript source:
    // /**
    // 	 * Runs an event with no source on each Pokémon on the field, in Speed order.
    // 	 */
    // 	eachEvent(eventid: string, effect?: Effect | null, relayVar?: boolean) {
    // 		const actives = this.getAllActive();
    // 		if (!effect && this.effect) effect = this.effect;
    // 		this.speedSort(actives, (a, b) => b.speed - a.speed);
    // 		for (const pokemon of actives) {
    // 			this.runEvent(eventid, pokemon, null, effect, relayVar);
    // 		}
    // 		if (eventid === 'Weather' && this.gen >= 7) {
    // 			// TODO: further research when updates happen
    // 			this.eachEvent('Update');
    // 		}
    // 	}
    //
    pub fn each_event(&mut self, event_id: &str, effect: Option<&ID>) {
        // Collect all active Pokemon with their speeds
        let mut actives: Vec<(usize, usize, i32)> = Vec::new();
        for (side_idx, side) in self.sides.iter().enumerate() {
            for (_slot, active_idx) in side.active.iter().enumerate() {
                if let Some(poke_idx) = active_idx {
                    if let Some(pokemon) = side.pokemon.get(*poke_idx) {
                        if !pokemon.fainted {
                            actives.push((side_idx, *poke_idx, pokemon.speed));
                        }
                    }
                }
            }
        }

        // Sort by speed (highest first)
        actives.sort_by(|a, b| b.2.cmp(&a.2));

        // Run event on each
        for (side_idx, poke_idx, _speed) in actives {
            self.run_event(event_id, Some((side_idx, poke_idx)), None, effect, None);
        }

        // Weather update triggers Update event in Gen 7+
        if event_id == "Weather" && self.gen >= 7 {
            self.each_event("Update", None);
        }
    }

    /// Get target for a move
    /// Equivalent to battle.ts getTarget()
    ///
    // 
    // 	getTarget(pokemon: Pokemon, move: string | Move, targetLoc: number, originalTarget?: Pokemon) {
    // 		move = this.dex.moves.get(move);
    // 
    // 		let tracksTarget = move.tracksTarget;
    // 		// Stalwart sets trackTarget in ModifyMove, but ModifyMove happens after getTarget, so
    // 		// we need to manually check for Stalwart here
    // 		if (pokemon.hasAbility(['stalwart', 'propellertail'])) tracksTarget = true;
    // 		if (tracksTarget && originalTarget?.isActive) {
    // 			// smart-tracking move's original target is on the field: target it
    // 			return originalTarget;
    // 		}
    // 
    // 		// banning Dragon Darts from directly targeting itself is done in side.ts, but
    // 		// Dragon Darts can target itself if Ally Switch is used afterwards
    // 		if (move.smartTarget) {
    // 			const curTarget = pokemon.getAtLoc(targetLoc);
    // 			return curTarget && !curTarget.fainted ? curTarget : this.getRandomTarget(pokemon, move);
    // 		}
    // 
    // 		// Fails if the target is the user and the move can't target its own position
    // 		const selfLoc = pokemon.getLocOf(pokemon);
    // 		if (
    // 			['adjacentAlly', 'any', 'normal'].includes(move.target) && targetLoc === selfLoc &&
    // 			!pokemon.volatiles['twoturnmove'] && !pokemon.volatiles['iceball'] && !pokemon.volatiles['rollout']
    // 		) {
    // 			return move.flags['futuremove'] ? pokemon : null;
    // 		}
    // 		if (move.target !== 'randomNormal' && this.validTargetLoc(targetLoc, pokemon, move.target)) {
    // 			const target = pokemon.getAtLoc(targetLoc);
    // 			if (target?.fainted) {
    // 				if (this.gameType === 'freeforall') {
    // 					// Target is a fainted opponent in a free-for-all battle; attack shouldn't retarget
    // 					return target;
    // 				}
    // 				if (target.isAlly(pokemon)) {
    // 					if (move.target === 'adjacentAllyOrSelf' && this.gen !== 5) {
    // 						return pokemon;
    // 					}
    // 					// Target is a fainted ally: attack shouldn't retarget
    // 					return target;
    // 				}
    // 			}
    // 			if (target && !target.fainted) {
    // 				// Target is unfainted: use selected target location
    // 				return target;
    // 			}
    // 
    // 			// Chosen target not valid,
    // 			// retarget randomly with getRandomTarget
    // 		}
    // 		return this.getRandomTarget(pokemon, move);
    // 	}
    //
    pub fn get_target(
        &mut self,
        user: (usize, usize),
        move_id: &ID,
        target_loc: i8,
        original_target: Option<(usize, usize)>,
    ) -> Option<(usize, usize)> {
        // JS: move = this.dex.moves.get(move);
        let move_def = self.dex.get_move(move_id.as_str())?;
        let target_type = format!("{:?}", move_def.target);

        // JS: let tracksTarget = move.tracksTarget;
        // JS: if (pokemon.hasAbility(['stalwart', 'propellertail'])) tracksTarget = true;
        let (user_side, user_idx) = user;
        let mut tracks_target = move_def.tracks_target.unwrap_or(false);

        // Check if Pokemon has Stalwart or Propeller Tail abilities
        if let Some(side) = self.sides.get(user_side) {
            if let Some(pokemon) = side.pokemon.get(user_idx) {
                let ability = pokemon.ability.as_str();
                if ability == "stalwart" || ability == "propellertail" {
                    tracks_target = true;
                }
            }
        }

        // JS: if (tracksTarget && originalTarget?.isActive) return originalTarget;
        if tracks_target {
            if let Some((target_side, target_idx)) = original_target {
                if let Some(side) = self.sides.get(target_side) {
                    if let Some(pokemon) = side.pokemon.get(target_idx) {
                        if pokemon.is_active {
                            return Some((target_side, target_idx));
                        }
                    }
                }
            }
        }

        // JS: if (move.smartTarget) {
        // JS:     const curTarget = pokemon.getAtLoc(targetLoc);
        // JS:     return curTarget && !curTarget.fainted ? curTarget : this.getRandomTarget(pokemon, move);
        // JS: }
        if move_def.smart_target.unwrap_or(false) {
            // Try to get the Pokemon at target location
            if let Some(cur_target) = self.get_at_loc(user, target_loc) {
                // If target exists and is not fainted, use it
                if !self.is_pokemon_fainted(cur_target) {
                    return Some(cur_target);
                }
            }
            // Otherwise, get a random target
            return self.get_random_target(user.0, user.1, &target_type);
        }

        // JS: const selfLoc = pokemon.getLocOf(pokemon);
        // JS: if (
        // JS:     ['adjacentAlly', 'any', 'normal'].includes(move.target) && targetLoc === selfLoc &&
        // JS:     !pokemon.volatiles['twoturnmove'] && !pokemon.volatiles['iceball'] && !pokemon.volatiles['rollout']
        // JS: ) {
        // JS:     return move.flags['futuremove'] ? pokemon : null;
        // JS: }
        // Fails if the target is the user and the move can't target its own position
        let self_loc = self.get_loc_of(user, user);
        if (target_type == "AdjacentAlly" || target_type == "Any" || target_type == "Normal")
            && target_loc as i32 == self_loc
        {
            // Check if Pokemon has volatiles that allow self-targeting
            let has_self_targeting_volatile = if let Some(side) = self.sides.get(user_side) {
                if let Some(pokemon) = side.pokemon.get(user_idx) {
                    pokemon.volatiles.contains_key(&ID::new("twoturnmove"))
                        || pokemon.volatiles.contains_key(&ID::new("iceball"))
                        || pokemon.volatiles.contains_key(&ID::new("rollout"))
                } else {
                    false
                }
            } else {
                false
            };

            if !has_self_targeting_volatile {
                // If move has futuremove flag, return user (self), otherwise return None
                let has_futuremove = move_def.flags.get("futuremove").map(|&v| v != 0).unwrap_or(false);
                return if has_futuremove {
                    Some(user)
                } else {
                    None
                };
            }
        }

        // JS: if (move.target !== 'randomNormal' && this.validTargetLoc(targetLoc, pokemon, move.target)) {
        if target_type != "RandomNormal" && self.valid_target_loc(target_loc as i32, user, &target_type) {
            // JS: const target = pokemon.getAtLoc(targetLoc);
            if let Some(target) = self.get_at_loc(user, target_loc) {
                let (target_side, target_idx) = target;
                if let Some(side) = self.sides.get(target_side) {
                    if let Some(pokemon) = side.pokemon.get(target_idx) {
                        // JS: if (target?.fainted) {
                        if pokemon.fainted {
                            // JS: if (this.gameType === 'freeforall') return target;
                            if self.game_type == GameType::FreeForAll {
                                return Some(target);
                            }
                            // JS: if (target.isAlly(pokemon)) {
                            // JS:     if (move.target === 'adjacentAllyOrSelf' && this.gen !== 5) return pokemon;
                            // JS:     return target;
                            // JS: }
                            if self.is_ally(target, user) {
                                if target_type == "AdjacentAllyOrSelf" && self.gen != 5 {
                                    return Some(user);
                                }
                                // Target is a fainted ally: attack shouldn't retarget
                                return Some(target);
                            }
                        }
                        // JS: if (target && !target.fainted) return target;
                        if !pokemon.fainted {
                            return Some(target);
                        }
                    }
                }
            }
        }

        // JS: return this.getRandomTarget(pokemon, move);
        self.get_random_target(user_side, user_idx, &target_type)
    }

    /// Get Pokemon at a location relative to a source Pokemon
    /// Helper for get_target
    /// Rust helper method - JavaScript uses pokemon.getAtLoc(targetLoc) directly on Pokemon object
    /// This method implements the logic as a Battle method since Pokemon doesn't have access to Battle state
    /// Returns (side_index, pokemon_index) tuple if Pokemon exists at location, None otherwise
    fn get_at_loc(&self, source: (usize, usize), target_loc: i8) -> Option<(usize, usize)> {
        let (source_side, _source_idx) = source;

        if target_loc == 0 {
            return None;
        }

        let (target_side, slot) = if target_loc > 0 {
            // Opponent's side
            let foe_side = if source_side == 0 { 1 } else { 0 };
            (foe_side, (target_loc - 1) as usize)
        } else {
            // Own side (negative)
            (source_side, (-target_loc - 1) as usize)
        };

        if slot >= self.active_per_half {
            return None;
        }

        // Get active Pokemon at slot
        if let Some(side) = self.sides.get(target_side) {
            if let Some(Some(poke_idx)) = side.active.get(slot) {
                return Some((target_side, *poke_idx));
            }
        }

        None
    }

    /// Undo a player's choice
    /// Equivalent to battle.ts undoChoice()
    // 
    // 	undoChoice(sideid: SideID) {
    // 		const side = this.getSide(sideid);
    // 		if (!side.requestState) return;
    // 
    // 		if (side.choice.cantUndo) {
    // 			side.emitChoiceError(`Can't undo: A trapping/disabling effect would cause undo to leak information`);
    // 			return;
    // 		}
    // 
    // 		let updated = false;
    // 		if (side.requestState === 'move') {
    // 			for (const action of side.choice.actions) {
    // 				const pokemon = action.pokemon;
    // 				if (action.choice !== 'move' || !pokemon) continue;
    // 				if (side.updateRequestForPokemon(pokemon, req => side.updateDisabledRequest(pokemon, req))) {
    // 					updated = true;
    // 				}
    // 			}
    // 		}
    // 
    // 		side.clearChoice();
    // 
    // 		if (updated) side.emitRequest(side.activeRequest!, true);
    // 	}
    //
    pub fn undo_choice(&mut self, side_id: SideID) -> bool {
        let side_idx = side_id.index();

        if let Some(side) = self.sides.get_mut(side_idx) {
            if side.choice.cant_undo {
                return false;
            }
            side.choice.clear();
            return true;
        }

        false
    }

    /// Spread damage to multiple targets
    /// Matches JavaScript battle.ts:2045-2164 spreadDamage()
    // 
    // 	spreadDamage(
    // 		damage: SpreadMoveDamage, targetArray: (false | Pokemon | null)[] | null = null,
    // 		source: Pokemon | null = null, effect: 'drain' | 'recoil' | Effect | null = null, instafaint = false
    // 	) {
    // 		if (!targetArray) return [0];
    // 		const retVals: (number | false | undefined)[] = [];
    // 		if (typeof effect === 'string' || !effect) effect = this.dex.conditions.getByID((effect || '') as ID);
    // 		for (const [i, curDamage] of damage.entries()) {
    // 			const target = targetArray[i];
    // 			let targetDamage = curDamage;
    // 			if (!(targetDamage || targetDamage === 0)) {
    // 				retVals[i] = targetDamage;
    // 				continue;
    // 			}
    // 			if (!target || !target.hp) {
    // 				retVals[i] = 0;
    // 				continue;
    // 			}
    // 			if (!target.isActive) {
    // 				retVals[i] = false;
    // 				continue;
    // 			}
    // 			if (targetDamage !== 0) targetDamage = this.clampIntRange(targetDamage, 1);
    // 
    // 			if (effect.id !== 'struggle-recoil') { // Struggle recoil is not affected by effects
    // 				if (effect.effectType === 'Weather' && !target.runStatusImmunity(effect.id)) {
    // 					this.debug('weather immunity');
    // 					retVals[i] = 0;
    // 					continue;
    // 				}
    // 				targetDamage = this.runEvent('Damage', target, source, effect, targetDamage, true);
    // 				if (!(targetDamage || targetDamage === 0)) {
    // 					this.debug('damage event failed');
    // 					retVals[i] = curDamage === true ? undefined : targetDamage;
    // 					continue;
    // 				}
    // 			}
    // 			if (targetDamage !== 0) targetDamage = this.clampIntRange(targetDamage, 1);
    // 
    // 			if (this.gen <= 1) {
    // 				if (this.dex.currentMod === 'gen1stadium' ||
    // 					!['recoil', 'drain', 'leechseed'].includes(effect.id) && effect.effectType !== 'Status') {
    // 					this.lastDamage = targetDamage;
    // 				}
    // 			}
    // 
    // 			retVals[i] = targetDamage = target.damage(targetDamage, source, effect);
    // 			if (targetDamage !== 0) target.hurtThisTurn = target.hp;
    // 			if (source && effect.effectType === 'Move') source.lastDamage = targetDamage;
    // 
    // 			const name = effect.fullname === 'tox' ? 'psn' : effect.fullname;
    // 			switch (effect.id) {
    // 			case 'partiallytrapped':
    // 				this.add('-damage', target, target.getHealth, '[from] ' + target.volatiles['partiallytrapped'].sourceEffect.fullname, '[partiallytrapped]');
    // 				break;
    // 			case 'powder':
    // 				this.add('-damage', target, target.getHealth, '[silent]');
    // 				break;
    // 			case 'confused':
    // 				this.add('-damage', target, target.getHealth, '[from] confusion');
    // 				break;
    // 			default:
    // 				if (effect.effectType === 'Move' || !name) {
    // 					this.add('-damage', target, target.getHealth);
    // 				} else if (source && (source !== target || effect.effectType === 'Ability')) {
    // 					this.add('-damage', target, target.getHealth, `[from] ${name}`, `[of] ${source}`);
    // 				} else {
    // 					this.add('-damage', target, target.getHealth, `[from] ${name}`);
    // 				}
    // 				break;
    // 			}
    // 
    // 			if (targetDamage && effect.effectType === 'Move') {
    // 				if (this.gen <= 1 && effect.recoil && source) {
    // 					if (this.dex.currentMod !== 'gen1stadium' || target.hp > 0) {
    // 						const amount = this.clampIntRange(Math.floor(targetDamage * effect.recoil[0] / effect.recoil[1]), 1);
    // 						this.damage(amount, source, target, 'recoil');
    // 					}
    // 				}
    // 				if (this.gen <= 4 && effect.drain && source) {
    // 					const amount = this.clampIntRange(Math.floor(targetDamage * effect.drain[0] / effect.drain[1]), 1);
    // 					// Draining can be countered in gen 1
    // 					if (this.gen <= 1) this.lastDamage = amount;
    // 					this.heal(amount, source, target, 'drain');
    // 				}
    // 				if (this.gen > 4 && effect.drain && source) {
    // 					const amount = Math.round(targetDamage * effect.drain[0] / effect.drain[1]);
    // 					this.heal(amount, source, target, 'drain');
    // 				}
    // 			}
    // 		}
    // 
    // 		if (instafaint) {
    // 			for (const [i, target] of targetArray.entries()) {
    // 				if (!retVals[i] || !target) continue;
    // 
    // 				if (target.hp <= 0) {
    // 					this.debug(`instafaint: ${this.faintQueue.map(entry => entry.target.name)}`);
    // 					this.faintMessages(true);
    // 					if (this.gen <= 2) {
    // 						target.faint();
    // 						if (this.gen <= 1) {
    // 							this.queue.clear();
    // 							// Fainting clears accumulated Bide damage
    // 							for (const pokemon of this.getAllActive()) {
    // 								if (pokemon.volatiles['bide']?.damage) {
    // 									pokemon.volatiles['bide'].damage = 0;
    // 									this.hint("Desync Clause Mod activated!");
    // 									this.hint("In Gen 1, Bide's accumulated damage is reset to 0 when a Pokemon faints.");
    // 								}
    // 							}
    // 						}
    // 					}
    // 				}
    // 			}
    // 		}
    // 
    // 		return retVals;
    // 	}
    //
    pub fn spread_damage(
        &mut self,
        damages: &[Option<i32>],  // Can be true (max damage), false, number, or undefined
        targets: &[Option<(usize, usize)>],
        source: Option<(usize, usize)>,
        effect: Option<&ID>,
        instafaint: bool,
    ) -> Vec<Option<i32>> {
        let mut ret_vals: Vec<Option<i32>> = Vec::new();

        // Process damage for each target
        for i in 0..damages.len() {
            let cur_damage = damages.get(i).copied().flatten();
            let target = targets.get(i).copied().flatten();

            // Handle undefined/null damage
            if cur_damage.is_none() {
                ret_vals.push(None);
                continue;
            }

            let mut target_damage = cur_damage.unwrap();

            // Handle missing or fainted target
            if target.is_none() {
                ret_vals.push(Some(0));
                continue;
            }

            let target_pos = target.unwrap();
            let (side_idx, poke_idx) = target_pos;

            // Check if target exists and has HP
            let (has_hp, is_active) = if let Some(side) = self.sides.get(side_idx) {
                if let Some(pokemon) = side.pokemon.get(poke_idx) {
                    (pokemon.hp > 0, pokemon.is_active)
                } else {
                    (false, false)
                }
            } else {
                (false, false)
            };

            if !has_hp {
                ret_vals.push(Some(0));
                continue;
            }

            if !is_active {
                ret_vals.push(None);  // JavaScript returns false
                continue;
            }

            // Clamp damage to at least 1 if non-zero
            if target_damage != 0 {
                target_damage = target_damage.max(1);
            }

            // JavaScript: if (effect.id !== 'struggle-recoil')
            let effect_id = effect.map(|e| e.as_str()).unwrap_or("");
            if effect_id != "strugglerecoil" {
                // Check weather immunity
                // JavaScript: if (effect.effectType === 'Weather' && !target.runStatusImmunity(effect.id))
                if let Some(eff) = effect {
                    let effect_type = self.get_effect_type(eff);
                    if effect_type == "Weather" {
                        // Check if target is immune to this weather effect
                        if let Some(side) = self.sides.get(side_idx) {
                            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                                if !pokemon.run_status_immunity(effect_id) {
                                    // Target is immune to this weather damage
                                    ret_vals.push(Some(0));
                                    continue;
                                }
                            }
                        }
                    }
                }

                // Fire Damage event
                // JavaScript: targetDamage = this.runEvent('Damage', target, source, effect, targetDamage, true);
                let event_result = self.run_event(
                    "Damage",
                    Some(target_pos),
                    source,
                    effect,
                    Some(target_damage)
                );

                if let Some(modified_damage) = event_result {
                    target_damage = modified_damage;
                } else {
                    // Event failed
                    self.debug("damage event failed");
                    ret_vals.push(None);
                    continue;
                }
            }

            // Clamp damage again after event
            if target_damage != 0 {
                target_damage = target_damage.max(1);
            }

            // Gen 1: set lastDamage for certain effects
            if self.gen <= 1 {
                // JavaScript: if (!['recoil', 'drain', 'leechseed'].includes(effect.id) && effect.effectType !== 'Status')
                if effect_id != "recoil" && effect_id != "drain" && effect_id != "leechseed" {
                    self.last_damage = target_damage as i32;
                }
            }

            // Apply damage using Pokemon's damage method
            let actual_damage = if let Some(side) = self.sides.get_mut(side_idx) {
                if let Some(pokemon) = side.pokemon.get_mut(poke_idx) {
                    let dmg = pokemon.damage(target_damage as i32);
                    dmg as i32
                } else {
                    0
                }
            } else {
                0
            };

            target_damage = actual_damage;
            ret_vals.push(Some(target_damage));

            // Set hurtThisTurn
            if target_damage != 0 {
                if let Some(side) = self.sides.get_mut(side_idx) {
                    if let Some(pokemon) = side.pokemon.get_mut(poke_idx) {
                        pokemon.hurt_this_turn = Some(pokemon.hp);
                    }
                }
            }

            // Set source.lastDamage for moves
            // JS: if (source && effect.effectType === 'Move') source.lastDamage = targetDamage;
            if source.is_some() && effect.map_or(false, |e| self.get_effect_type(e) == "Move") {
                if let Some((src_side, src_idx)) = source {
                    if let Some(side) = self.sides.get_mut(src_side) {
                        if let Some(pokemon) = side.pokemon.get_mut(src_idx) {
                            pokemon.last_damage = target_damage as i32;
                        }
                    }
                }
            }

            // Add damage log message
            self.add_damage_log(target_pos, source, effect);

            // Check if Pokemon fainted and add to faint queue
            // JavaScript: if (pokemon.hp <= 0) pokemon.faint(source, effect);
            if let Some(side) = self.sides.get(target_pos.0) {
                if let Some(pokemon) = side.pokemon.get(target_pos.1) {
                    if pokemon.hp == 0 {
                        let effect_str = effect.map(|e| e.as_str());
                        self.faint(target_pos, source, effect_str);
                    }
                }
            }

            // Handle recoil and drain for moves
            // JavaScript: if (targetDamage && effect.effectType === 'Move')
            // JS: 			if (targetDamage && effect.effectType === 'Move') {
            // JS: 				if (this.gen <= 1 && effect.recoil && source) {
            // JS: 					if (this.dex.currentMod !== 'gen1stadium' || target.hp > 0) {
            // JS: 						const amount = this.clampIntRange(Math.floor(targetDamage * effect.recoil[0] / effect.recoil[1]), 1);
            // JS: 						this.damage(amount, source, target, 'recoil');
            // JS: 					}
            // JS: 				}
            // JS: 				if (this.gen <= 4 && effect.drain && source) {
            // JS: 					const amount = this.clampIntRange(Math.floor(targetDamage * effect.drain[0] / effect.drain[1]), 1);
            // JS: 					// Draining can be countered in gen 1
            // JS: 					if (this.gen <= 1) this.lastDamage = amount;
            // JS: 					this.heal(amount, source, target, 'drain');
            // JS: 				}
            // JS: 				if (this.gen > 4 && effect.drain && source) {
            // JS: 					const amount = Math.round(targetDamage * effect.drain[0] / effect.drain[1]);
            // JS: 					this.heal(amount, source, target, 'drain');
            // JS: 				}
            // JS: 			}
            if target_damage > 0 && effect.map_or(false, |e| self.get_effect_type(e) == "Move") {
                // Get move data to check for recoil and drain (extract data first to avoid borrow issues)
                let (recoil_data, drain_data) = if let Some(eff) = effect {
                    if let Some(move_data) = self.dex.get_move(eff.as_str()) {
                        (move_data.recoil, move_data.drain)
                    } else {
                        (None, None)
                    }
                } else {
                    (None, None)
                };

                // Gen 1 recoil damage
                if self.gen <= 1 && recoil_data.is_some() && source.is_some() {
                    let (recoil_num, recoil_denom) = recoil_data.unwrap();
                    let amount = ((target_damage as f64 * recoil_num as f64) / recoil_denom as f64).floor() as i32;
                    let amount = self.clamp_int_range(amount, Some(1), Some(i32::MAX));

                    let recoil_id = ID::new("recoil");
                    self.damage(amount, source, target, Some(&recoil_id), false);
                }

                // Gen 1-4 drain healing
                if self.gen <= 4 && drain_data.is_some() && source.is_some() {
                    let (drain_num, drain_denom) = drain_data.unwrap();
                    let amount = ((target_damage as f64 * drain_num as f64) / drain_denom as f64).floor() as i32;
                    let amount = self.clamp_int_range(amount, Some(1), Some(i32::MAX));

                    // Draining can be countered in gen 1
                    if self.gen <= 1 {
                        self.last_damage = amount;
                    }

                    let drain_id = ID::new("drain");
                    self.heal(amount, source, target, Some(&drain_id));
                }

                // Gen 5+ drain healing (uses round instead of floor)
                if self.gen > 4 && drain_data.is_some() && source.is_some() {
                    let (drain_num, drain_denom) = drain_data.unwrap();
                    let amount = ((target_damage as f64 * drain_num as f64) / drain_denom as f64).round() as i32;

                    let drain_id = ID::new("drain");
                    self.heal(amount, source, target, Some(&drain_id));
                }
            }
        }

        // Handle instafaint
        if instafaint {
            for i in 0..targets.len() {
                if ret_vals.get(i).copied().flatten().is_none() {
                    continue;
                }
                if let Some(Some(target_pos)) = targets.get(i) {
                    let should_faint = if let Some(side) = self.sides.get(target_pos.0) {
                        if let Some(pokemon) = side.pokemon.get(target_pos.1) {
                            pokemon.hp == 0
                        } else {
                            false
                        }
                    } else {
                        false
                    };

                    if should_faint {
                        self.debug(&format!("instafaint"));
                        // JS: this.faintMessages(true);
                        self.faint_messages(true, false, true);

                        // Gen 1-2 special handling
                        if self.gen <= 2 {
                            if let Some(side) = self.sides.get_mut(target_pos.0) {
                                if let Some(pokemon) = side.pokemon.get_mut(target_pos.1) {
                                    pokemon.faint();
                                }
                            }

                            // Gen 1: Clear queue and reset Bide
                            // JS: if (this.gen <= 1) {
                            // JS:     this.queue.clear();
                            // JS:     // Fainting clears accumulated Bide damage
                            // JS:     for (const pokemon of this.getAllActive()) {
                            // JS:         if (pokemon.volatiles['bide']?.damage) {
                            // JS:             pokemon.volatiles['bide'].damage = 0;
                            // JS:             this.hint("Desync Clause Mod activated!");
                            // JS:             this.hint("In Gen 1, Bide's accumulated damage is reset to 0 when a Pokemon faints.");
                            // JS:         }
                            // JS:     }
                            // JS: }
                            if self.gen <= 1 {
                                self.queue.clear();

                                // Reset Bide damage for all active Pokemon
                                let mut bide_cleared = false;
                                let bide_id = ID::new("bide");

                                for side in &mut self.sides {
                                    for pokemon in &mut side.pokemon {
                                        if !pokemon.is_active {
                                            continue;
                                        }

                                        // Check if pokemon has bide volatile with damage > 0
                                        if let Some(bide_state) = pokemon.volatiles.get_mut(&bide_id) {
                                            // Check if the bide state has a damage field
                                            if let Some(damage_value) = bide_state.data.get("damage") {
                                                if let Some(damage) = damage_value.as_i64() {
                                                    if damage > 0 {
                                                        // Clear Bide damage
                                                        bide_state.data.insert("damage".to_string(), serde_json::json!(0));
                                                        bide_cleared = true;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                if bide_cleared {
                                    self.hint("Desync Clause Mod activated!", false, None);
                                    self.hint("In Gen 1, Bide's accumulated damage is reset to 0 when a Pokemon faints.", false, None);
                                }
                            }
                        }
                    }
                }
            }
        }

        ret_vals
    }

    /// Helper to add damage log messages
    /// Matches JavaScript battle.ts:2088-2112
    /// Rust helper method - JavaScript has this logic inline in spreadDamage()
    /// This method extracts the damage logging logic for code organization
    /// Handles special cases: partiallytrapped, powder, confused, and default damage logs
    fn add_damage_log(&mut self, target: (usize, usize), source: Option<(usize, usize)>, effect: Option<&ID>) {
        let (side_idx, poke_idx) = target;

        // Get target health string
        let health_str = if let Some(side) = self.sides.get(side_idx) {
            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                format!("{}/{}", pokemon.hp, pokemon.maxhp)
            } else {
                return;
            }
        } else {
            return;
        };

        let target_str = format!("p{}a", side_idx + 1);
        let effect_id = effect.map(|e| e.as_str()).unwrap_or("");

        // Special case handling
        match effect_id {
            "partiallytrapped" => {
                // Get source effect name from volatiles
                // JS: '[from] ' + target.volatiles['partiallytrapped'].sourceEffect.fullname
                let source_effect_name = if let Some(side) = self.sides.get(side_idx) {
                    if let Some(pokemon) = side.pokemon.get(poke_idx) {
                        let trap_id = ID::new("partiallytrapped");
                        if let Some(trap_state) = pokemon.volatiles.get(&trap_id) {
                            // Extract sourceEffect.fullname from data HashMap
                            if let Some(source_effect) = trap_state.data.get("sourceEffect") {
                                source_effect.get("fullname")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("partiallytrapped")
                            } else {
                                "partiallytrapped"
                            }
                        } else {
                            "partiallytrapped"
                        }
                    } else {
                        "partiallytrapped"
                    }
                } else {
                    "partiallytrapped"
                };

                let from_str = format!("[from] {}", source_effect_name);
                self.add_log("-damage", &[&target_str, &health_str, &from_str, "[partiallytrapped]"]);
            }
            "powder" => {
                self.add_log("-damage", &[&target_str, &health_str, "[silent]"]);
            }
            "confused" => {
                self.add_log("-damage", &[&target_str, &health_str, "[from] confusion"]);
            }
            _ => {
                // Default damage log
                if effect.is_none() {
                    self.add_log("-damage", &[&target_str, &health_str]);
                } else if let Some(src) = source {
                    let src_str = format!("p{}a", src.0 + 1);
                    let from_str = format!("[from] {}", effect_id);
                    let of_str = format!("[of] {}", src_str);
                    self.add_log("-damage", &[&target_str, &health_str, &from_str, &of_str]);
                } else {
                    let from_str = format!("[from] {}", effect_id);
                    self.add_log("-damage", &[&target_str, &health_str, &from_str]);
                }
            }
        }
    }

    /// Final stat modification with 4096 denominator
    /// Apply final modifier to value
    /// Equivalent to battle.ts finalModify() (battle.ts:2344-2347)
    ///
    // 
    // 	finalModify(relayVar: number) {
    // 		relayVar = this.modify(relayVar, this.event.modifier);
    // 		this.event.modifier = 1;
    // 		return relayVar;
    // 	}
    //
    pub fn final_modify(&mut self, value: i32) -> i32 {
        // JS: relayVar = this.modify(relayVar, this.event.modifier);
        let modifier = self.get_event_modifier();
        let result = self.modify_internal(value, modifier);

        // JS: this.event.modifier = 1;
        if let Some(ref mut event) = self.current_event {
            event.modifier = 4096;
        }

        result
    }

    /// Internal modifier calculation with 4096 fixed-point arithmetic
    /// Rust helper method - JavaScript uses inline calculation or this.modify()
    /// This method implements 4096-based fixed-point multiplication
    /// Formula: result = ((value * modifier + 2048) >> 12).max(1)
    /// modifier is in 4096 basis points (e.g., 6144 = 1.5x, 2048 = 0.5x)
    fn modify_internal(&self, value: i32, modifier: i32) -> i32 {
        // 4096-based fixed-point multiplication
        // modifier is already in 4096 basis (e.g., 6144 = 1.5x)
        let result = value as i64 * modifier as i64;
        ((result + 2048) >> 12).max(1) as i32
    }

    // =========================================================================
    // REMAINING METHODS (ported from battle.ts for complete 1:1 port)
    // =========================================================================

    /// Add split message for different players
    /// Equivalent to battle.ts addSplit()
    ///
    // 
    // 	addSplit(side: SideID, secret: Part[], shared?: Part[]) {
    // 		this.log.push(`|split|${side}`);
    // 		this.add(...secret);
    // 		if (shared) {
    // 			this.add(...shared);
    // 		} else {
    // 			this.log.push('');
    // 		}
    // 	}
    //
    pub fn add_split(&mut self, side_id: &str, secret: &[&str], shared: Option<&[&str]>) {
        // JS: this.log.push(`|split|${side}`);
        self.log.push(format!("|split|{}", side_id));

        // JS: this.add(...secret);
        if !secret.is_empty() {
            let entry = format!("|{}", secret.join("|"));
            self.log.push(entry);
        }

        // JS: if (shared) { this.add(...shared); } else { this.log.push(''); }
        if let Some(shared_parts) = shared {
            if !shared_parts.is_empty() {
                let entry = format!("|{}", shared_parts.join("|"));
                self.log.push(entry);
            }
        } else {
            self.log.push(String::new());
        }
    }

    /// Attribute damage to last move
    /// Equivalent to battle.ts attrLastMove()
    // 
    // 	attrLastMove(...args: (string | number | Function | AnyObject)[]) {
    // 		if (this.lastMoveLine < 0) return;
    // 		if (this.log[this.lastMoveLine].startsWith('|-anim|')) {
    // 			if (args.includes('[still]')) {
    // 				this.log.splice(this.lastMoveLine, 1);
    // 				this.lastMoveLine = -1;
    // 				return;
    // 			}
    // 		} else if (args.includes('[still]')) {
    // 			// If no animation plays, the target should never be known
    // 			const parts = this.log[this.lastMoveLine].split('|');
    // 			parts[4] = '';
    // 			this.log[this.lastMoveLine] = parts.join('|');
    // 		}
    // 		// eslint-disable-next-line @typescript-eslint/no-base-to-string
    // 		this.log[this.lastMoveLine] += `|${args.join('|')}`;
    // 	}
    //
    pub fn attr_last_move(&mut self, args: &[&str]) {
        // JS: if (this.lastMoveLine < 0) return;
        if self.last_move_line < 0 {
            return;
        }

        let line_idx = self.last_move_line as usize;
        if line_idx >= self.log.len() {
            return;
        }

        // Check if it's an animation line
        if self.log[line_idx].starts_with("|-anim|") {
            // JS: if (args.includes('[still]'))
            if args.contains(&"[still]") {
                // Remove the animation line
                self.log.remove(line_idx);
                self.last_move_line = -1;
                return;
            }
        } else if args.contains(&"[still]") {
            // If no animation plays, the target should never be known
            let parts: Vec<&str> = self.log[line_idx].split('|').collect();
            let mut new_parts = parts.clone();
            if new_parts.len() > 4 {
                new_parts[4] = "";
            }
            self.log[line_idx] = new_parts.join("|");
        }

        // Append attributes to the log line
        let attrs = args.join("|");
        self.log[line_idx] = format!("{}|{}", self.log[line_idx], attrs);
    }

    /// Chain modify event modifier
    /// Equivalent to battle.ts chainModify() (battle.ts:2291-2300)
    ///
    // 
    // 	chainModify(numerator: number | number[], denominator = 1) {
    // 		const previousMod = this.trunc(this.event.modifier * 4096);
    //
    // 		if (Array.isArray(numerator)) {
    // 			denominator = numerator[1];
    // 			numerator = numerator[0];
    // 		}
    // 		const nextMod = this.trunc(numerator * 4096 / denominator);
    // 		this.event.modifier = ((previousMod * nextMod + 2048) >> 12) / 4096;
    // 	}
    //

    /// Chain modify the event modifier using a simple multiplier
    /// Most common usage: battle.chain_modify(2.0) to double the value
    pub fn chain_modify(&mut self, multiplier: f32) -> i32 {
        // Convert multiplier to numerator/denominator (e.g., 2.0 -> 2/1, 1.5 -> 3/2)
        let numerator = (multiplier * 4096.0) as i32;
        let denominator = 4096;
        self.chain_modify_fraction(numerator, denominator)
    }

    /// Chain modify the event modifier using a fraction
    /// Used for precise ratios: battle.chain_modify_fraction(3, 2) for 1.5x
    pub fn chain_modify_fraction(&mut self, numerator: i32, denominator: i32) -> i32 {
        if let Some(ref mut event) = self.current_event {
            // Extract modifier value first to avoid borrow checker issues
            let modifier = event.modifier;

            // JS: const previousMod = this.trunc(this.event.modifier * 4096);
            // Modifier is already in 4096 basis points
            let previous_mod = modifier;

            // JS: const nextMod = this.trunc(numerator * 4096 / denominator);
            let next_mod = ((numerator as i64 * 4096) / denominator as i64) as i32;

            // JS: this.event.modifier = ((previousMod * nextMod + 2048) >> 12) / 4096;
            // Result stays in 4096 basis points
            let new_modifier = ((previous_mod as i64 * next_mod as i64 + 2048) >> 12) as i32;
            event.modifier = new_modifier;
            new_modifier
        } else {
            4096 // Default modifier (1.0 in 4096 basis points)
        }
    }

    /// Check if the active move has Sheer Force boost
    /// Equivalent to move.hasSheerForce in battle-actions.ts
    /// Returns true if the active move would have secondary effects suppressed by Sheer Force
    pub fn active_move_has_sheer_force(&self) -> bool {
        // TODO: Implement proper hasSheerForce logic
        // In TypeScript: this.hasSheerForce = !!(data.hasSheerForce && !this.secondaries);
        // For now, return false to allow compilation
        false
    }

    /// Check if teams have balanced EVs
    /// Equivalent to battle.ts checkEVBalance()
    ///
    // 
    // 	checkEVBalance() {
    // 		let limitedEVs: boolean | null = null;
    // 		for (const side of this.sides) {
    // 			const sideLimitedEVs = !side.pokemon.some(
    // 				pokemon => Object.values(pokemon.set.evs).reduce((a, b) => a + b, 0) > 510
    // 			);
    // 			if (limitedEVs === null) {
    // 				limitedEVs = sideLimitedEVs;
    // 			} else if (limitedEVs !== sideLimitedEVs) {
    // 				this.add('bigerror', "Warning: One player isn't adhering to a 510 EV limit, and the other player is.");
    // 			}
    // 		}
    // 	}
    //
    pub fn check_ev_balance(&mut self) {
        let mut limited_evs: Option<bool> = None;
        let mut needs_warning = false;

        for side in &self.sides {
            // Check if any Pokemon on this side has >510 total EVs
            let side_limited_evs = !side.team.iter().any(|set| {
                let total_evs = set.evs.hp
                    + set.evs.atk
                    + set.evs.def
                    + set.evs.spa
                    + set.evs.spd
                    + set.evs.spe;
                total_evs > 510
            });

            if let Some(limited) = limited_evs {
                if limited != side_limited_evs {
                    needs_warning = true;
                }
            } else {
                limited_evs = Some(side_limited_evs);
            }
        }

        if needs_warning {
            self.add("bigerror", &[Arg::Str("Warning: One player isn't adhering to a 510 EV limit, and the other player is.")]);
        }
    }

    /// Clear effect state data
    /// Equivalent to battle.ts clearEffectState()
    // 
    // 	clearEffectState(state: EffectState) {
    // 		state.id = '';
    // 		for (const k in state) {
    // 			if (k === 'id' || k === 'target') {
    // 				continue;
    // 			} else if (k === 'effectOrder') {
    // 				state.effectOrder = 0;
    // 			} else {
    // 				delete state[k];
    // 			}
    // 		}
    // 	}
    //
    pub fn clear_effect_state(&mut self, target: (usize, usize), effect_id: &ID) {
        if let Some(side) = self.sides.get_mut(target.0) {
            if let Some(pokemon) = side.pokemon.get_mut(target.1) {
                pokemon.volatiles.remove(effect_id);
            }
        }
    }

    /// Log a debug error message
    /// Equivalent to battle.ts debugError() (battle.ts:3158-3160)
    ///
    // 
    // 	debugError(activity: string) {
    // 		this.add('debug', activity);
    // 	}
    //
    pub fn debug_error(&mut self, activity: &str) {
        self.add("debug", &[Arg::Str(activity)]);
    }

    /// Run event on field (weather, terrain, pseudo-weather)
    /// Equivalent to battle.ts fieldEvent()
    // TypeScript source:
    // /**
    // 	 * Runs an event with no source on each effect on the field, in Speed order.
    // 	 *
    // 	 * Unlike `eachEvent`, this contains a lot of other handling and is only intended for
    // 	 * the 'Residual' and 'SwitchIn' events.
    // 	 */
    // 	fieldEvent(eventid: string, targets?: Pokemon[]) {
    // 		const callbackName = `on${eventid}`;
    // 		let getKey: undefined | 'duration';
    // 		if (eventid === 'Residual') {
    // 			getKey = 'duration';
    // 		}
    // 		let handlers = this.findFieldEventHandlers(this.field, `onField${eventid}`, getKey);
    // 		for (const side of this.sides) {
    // 			if (side.n < 2 || !side.allySide) {
    // 				handlers = handlers.concat(this.findSideEventHandlers(side, `onSide${eventid}`, getKey));
    // 			}
    // 			for (const active of side.active) {
    // 				if (!active) continue;
    // 				if (eventid === 'SwitchIn') {
    // 					handlers = handlers.concat(this.findPokemonEventHandlers(active, `onAny${eventid}`));
    // 				}
    // 				if (targets && !targets.includes(active)) continue;
    // 				handlers = handlers.concat(this.findPokemonEventHandlers(active, callbackName, getKey));
    // 				handlers = handlers.concat(this.findSideEventHandlers(side, callbackName, undefined, active));
    // 				handlers = handlers.concat(this.findFieldEventHandlers(this.field, callbackName, undefined, active));
    // 				handlers = handlers.concat(this.findBattleEventHandlers(callbackName, getKey, active));
    // 			}
    // 		}
    // 		this.speedSort(handlers);
    // 		while (handlers.length) {
    // 			const handler = handlers[0];
    // 			handlers.shift();
    // 			const effect = handler.effect;
    // 			if ((handler.effectHolder as Pokemon).fainted) {
    // 				if (!(handler.state?.isSlotCondition)) continue;
    // 			}
    // 			if (eventid === 'Residual' && handler.end && handler.state?.duration) {
    // 				handler.state.duration--;
    // 				if (!handler.state.duration) {
    // 					const endCallArgs = handler.endCallArgs || [handler.effectHolder, effect.id];
    // 					handler.end.call(...endCallArgs as [any, ...any[]]);
    // 					if (this.ended) return;
    // 					continue;
    // 				}
    // 			}
    // 
    // 			// effect may have been removed by a prior handler, i.e. Toxic Spikes being absorbed during a double switch
    // 			if (handler.state?.target instanceof Pokemon) {
    // 				let expectedStateLocation;
    // 				if (effect.effectType === 'Ability' && !handler.state.id.startsWith('ability:')) {
    // 					expectedStateLocation = handler.state.target.abilityState;
    // 				} else if (effect.effectType === 'Item' && !handler.state.id.startsWith('item:')) {
    // 					expectedStateLocation = handler.state.target.itemState;
    // 				} else if (effect.effectType === 'Status') {
    // 					expectedStateLocation = handler.state.target.statusState;
    // 				} else {
    // 					expectedStateLocation = handler.state.target.volatiles[effect.id];
    // 				}
    // 				if (expectedStateLocation !== handler.state) {
    // 					continue;
    // 				}
    // 			} else if (handler.state?.target instanceof Side && !handler.state.isSlotCondition) {
    // 				if ((handler.state.target.sideConditions[effect.id] !== handler.state)) {
    // 					continue;
    // 				}
    // 			} else if (handler.state?.target instanceof Field) {
    // 				let expectedStateLocation;
    // 				if (effect.effectType === 'Weather') {
    // 					expectedStateLocation = handler.state.target.weatherState;
    // 				} else if (effect.effectType === 'Terrain') {
    // 					expectedStateLocation = handler.state.target.terrainState;
    // 				} else {
    // 					expectedStateLocation = handler.state.target.pseudoWeather[effect.id];
    // 				}
    // 				if (expectedStateLocation !== handler.state) {
    // 					continue;
    // 				}
    // 			}
    // 
    // 			let handlerEventid = eventid;
    // 			if ((handler.effectHolder as Side).sideConditions) handlerEventid = `Side${eventid}`;
    // 			if ((handler.effectHolder as Field).pseudoWeather) handlerEventid = `Field${eventid}`;
    // 			if (handler.callback) {
    // 				this.singleEvent(handlerEventid, effect, handler.state, handler.effectHolder, null, null, undefined, handler.callback);
    // 			}
    // 
    // 			this.faintMessages();
    // 			if (this.ended) return;
    // 		}
    // 	}
    //
    pub fn field_event(&mut self, event_id: &str) {
        // Run on weather
        if !self.field.weather.is_empty() {
            let weather_id = self.field.weather.clone();
            self.single_event(event_id, &weather_id, None, None, None);
        }

        // Run on terrain
        if !self.field.terrain.is_empty() {
            let terrain_id = self.field.terrain.clone();
            self.single_event(event_id, &terrain_id, None, None, None);
        }

        // Run on pseudo-weather
        let pseudo_weather_ids: Vec<ID> = self.field.pseudo_weather.keys().cloned().collect();
        for pw_id in pseudo_weather_ids {
            self.single_event(event_id, &pw_id, None, None, None);
        }
    }

    /// Get callback for an effect's event handler
    /// Equivalent to battle.ts getCallback()
    // 
    // 	getCallback(target: Pokemon | Side | Field | Battle, effect: Effect, callbackName: string) {
    // 		let callback: Function | undefined = (effect as any)[callbackName];
    // 		// Abilities and items Start at different times during the SwitchIn event, so we run their onStart handlers
    // 		// during the SwitchIn event instead of running the Start event during switch-ins
    // 		// gens 4 and before still use the old system, though
    // 		if (
    // 			callback === undefined && target instanceof Pokemon && this.gen >= 5 && callbackName === 'onSwitchIn' &&
    // 			!(effect as any).onAnySwitchIn && (['Ability', 'Item'].includes(effect.effectType) || (
    // 				// Innate abilities/items
    // 				effect.effectType === 'Status' && ['ability', 'item'].includes(effect.id.split(':')[0])
    // 			))
    // 		) {
    // 			callback = (effect as any).onStart;
    // 		}
    // 		return callback;
    // 	}
    //
    pub fn get_callback(&self, _effect_id: &ID, _event_id: &str) -> Option<String> {
        // In the full implementation, this would look up event handlers
        // For now, return None
        None
    }

    /// Get overflowed turn count (for endless battle detection and Gen 8+ move timing)
    /// Equivalent to battle.ts getOverflowedTurnCount() (battle.ts:3317-3319)
    /// Used by Wish, Future Sight, and other delayed moves
    // TypeScript source:
    // /**
    // 	 * Currently, we treat Team Preview as turn 0, but the games start counting their turns at turn 0
    // 	 * There is also overflow that occurs in Gen 8+ that affects moves like Wish / Future Sight
    // 	 * https://www.smogon.com/forums/threads/10352797
    // 	 */
    // 	getOverflowedTurnCount(): number {
    // 		return this.gen >= 8 ? (this.turn - 1) % 256 : this.turn - 1;
    // 	}
    //
    pub fn get_overflowed_turn_count(&self) -> i32 {
        // JavaScript: return this.gen >= 8 ? (this.turn - 1) % 256 : this.turn - 1;
        if self.gen >= 8 {
            (self.turn.saturating_sub(1)) % 256
        } else {
            self.turn.saturating_sub(1)
        }
    }

    /// Get requests for all players
    /// Equivalent to battle.ts getRequests()
    // 
    // 	getRequests(type: RequestState) {
    // 		// default to no request
    // 		const requests: ChoiceRequest[] = Array(this.sides.length).fill(null);
    // 
    // 		switch (type) {
    // 		case 'switch':
    // 			for (let i = 0; i < this.sides.length; i++) {
    // 				const side = this.sides[i];
    // 				if (!side.pokemonLeft) continue;
    // 				const switchTable = side.active.map(pokemon => !!pokemon?.switchFlag);
    // 				if (switchTable.some(Boolean)) {
    // 					requests[i] = { forceSwitch: switchTable, side: side.getRequestData() };
    // 				}
    // 			}
    // 			break;
    // 
    // 		case 'teampreview':
    // 			for (let i = 0; i < this.sides.length; i++) {
    // 				const side = this.sides[i];
    // 				const maxChosenTeamSize = this.ruleTable.pickedTeamSize || undefined;
    // 				requests[i] = { teamPreview: true, maxChosenTeamSize, side: side.getRequestData() };
    // 			}
    // 			break;
    // 
    // 		default:
    // 			for (let i = 0; i < this.sides.length; i++) {
    // 				const side = this.sides[i];
    // 				if (!side.pokemonLeft) continue;
    // 				const activeData = side.active.map(pokemon => pokemon?.getMoveRequestData());
    // 				requests[i] = { active: activeData, side: side.getRequestData() };
    // 				if (side.allySide) {
    // 					(requests[i] as MoveRequest).ally = side.allySide.getRequestData(true);
    // 				}
    // 			}
    // 			break;
    // 		}
    // 
    // 		const multipleRequestsExist = requests.filter(Boolean).length >= 2;
    // 		for (let i = 0; i < this.sides.length; i++) {
    // 			if (requests[i]) {
    // 				if (!this.supportCancel || !multipleRequestsExist) requests[i].noCancel = true;
    // 			} else {
    // 				requests[i] = { wait: true, side: this.sides[i].getRequestData() };
    // 			}
    // 		}
    // 
    // 		return requests;
    // 	}
    //
    pub fn get_requests(&self) -> Vec<serde_json::Value> {
        self.sides.iter().map(|side| {
            serde_json::json!({
                "side": side.id_str(),
                "rqid": self.turn, // Use turn as request ID
                "requestType": match self.request_state {
                    BattleRequestState::Move => "move",
                    BattleRequestState::Switch => "switch",
                    BattleRequestState::TeamPreview => "teampreview",
                    BattleRequestState::None => "none",
                }
            })
        }).collect()
    }

    /// Get team based on player options
    /// Equivalent to battle.ts getTeam()
    // 
    // 	// players
    // 
    // 	getTeam(options: PlayerOptions): PokemonSet[] {
    // 		let team = options.team;
    // 		if (typeof team === 'string') team = Teams.unpack(team);
    // 		if (team) return team;
    // 
    // 		if (!options.seed) {
    // 			options.seed = PRNG.generateSeed();
    // 		}
    // 
    // 		if (!this.teamGenerator) {
    // 			this.teamGenerator = Teams.getGenerator(this.format, options.seed);
    // 		} else {
    // 			this.teamGenerator.setSeed(options.seed);
    // 		}
    // 
    // 		team = this.teamGenerator.getTeam(options);
    // 		return team as PokemonSet[];
    // 	}
    //
    pub fn get_team(&self, side_idx: usize) -> Option<&[crate::pokemon::Pokemon]> {
        self.sides.get(side_idx).map(|s| s.pokemon.as_slice())
    }

    /// Resolve event handler priority
    /// Equivalent to battle.ts resolvePriority()
    ///
    /// JavaScript Source (battle.ts:950-1017):
    /// Takes an EventListenerWithoutPriority and enriches it with priority/order/subOrder
    /// based on effect callback properties and effectType ordering
    // 
    // 	resolvePriority(h: EventListenerWithoutPriority, callbackName: string) {
    // 		const handler = h as EventListener;
    // 		handler.order = (handler.effect as any)[`${callbackName}Order`] || false;
    // 		handler.priority = (handler.effect as any)[`${callbackName}Priority`] || 0;
    // 		handler.subOrder = (handler.effect as any)[`${callbackName}SubOrder`] || 0;
    // 		if (!handler.subOrder) {
    // 			// https://www.smogon.com/forums/threads/sword-shield-battle-mechanics-research.3655528/page-59#post-8685465
    // 			const effectTypeOrder: { [k in EffectType]?: number } = {
    // 				// Z-Move: 1,
    // 				Condition: 2,
    // 				// Slot Condition: 3,
    // 				// Side Condition: 4,
    // 				// Field Condition: 5, (includes weather but also terrains and pseudoweathers)
    // 				Weather: 5,
    // 				Format: 5,
    // 				Rule: 5,
    // 				Ruleset: 5,
    // 				// Poison Touch: 6, (also includes Perish Body)
    // 				Ability: 7,
    // 				Item: 8,
    // 				// Stall: 9,
    // 			};
    // 			handler.subOrder = effectTypeOrder[handler.effect.effectType] || 0;
    // 			if (handler.effect.effectType === 'Condition') {
    // 				if (handler.state?.target instanceof Side) {
    // 					if (handler.state.isSlotCondition) {
    // 						// slot condition
    // 						handler.subOrder = 3;
    // 					} else {
    // 						// side condition
    // 						handler.subOrder = 4;
    // 					}
    // 				} else if (handler.state?.target instanceof Field) {
    // 					// field condition
    // 					handler.subOrder = 5;
    // 				}
    // 			} else if (handler.effect.effectType === 'Ability') {
    // 				if (handler.effect.name === 'Poison Touch' || handler.effect.name === 'Perish Body') {
    // 					handler.subOrder = 6;
    // 				} else if (handler.effect.name === 'Stall') {
    // 					handler.subOrder = 9;
    // 				}
    // 			}
    // 		}
    // 		if (callbackName.endsWith('SwitchIn') || callbackName.endsWith('RedirectTarget')) {
    // 			// If multiple hazards are present on one side, their event handlers all perfectly tie in speed, priority,
    // 			// and subOrder. They should activate in the order they were created, which is where effectOrder comes in.
    // 			// This also applies to speed ties for which ability like Lightning Rod redirects moves.
    // 			// TODO: In-game, other events are also sorted this way, but that's an implementation for another refactor
    // 			handler.effectOrder = handler.state?.effectOrder;
    // 		}
    // 		if (handler.effectHolder && (handler.effectHolder as Pokemon).getStat) {
    // 			const pokemon = handler.effectHolder as Pokemon;
    // 			handler.speed = pokemon.speed;
    // 			if (handler.effect.effectType === 'Ability' && handler.effect.name === 'Magic Bounce' &&
    // 				callbackName === 'onAllyTryHitSide') {
    // 				handler.speed = pokemon.getStat('spe', true, true);
    // 			}
    // 			if (callbackName.endsWith('SwitchIn')) {
    // 				// Pokemon speeds including ties are resolved before all onSwitchIn handlers and aren't re-sorted in-between
    // 				// so we subtract a fractional speed from each Pokemon's respective event handlers by using the index of their
    // 				// unique field position in a pre-sorted-by-speed array
    // 				const fieldPositionValue = pokemon.side.n * this.sides.length + pokemon.position;
    // 				handler.speed -= this.speedOrder.indexOf(fieldPositionValue) / (this.activePerHalf * 2);
    // 			}
    // 		}
    // 		return handler;
    // 	}
    //
    pub fn resolve_priority(&mut self, handler: &mut EventListener, callback_name: &str) {
        // JS: handler.order = (handler.effect as any)[`${callbackName}Order`] || false;
        // JS: handler.priority = (handler.effect as any)[`${callbackName}Priority`] || 0;
        // JS: handler.subOrder = (handler.effect as any)[`${callbackName}SubOrder`] || 0;
        //
        // In Rust, we don't have dynamic property access, so we assume these are already set
        // or we get them from ability/item/move data lookups.
        // For now, we'll set defaults and then calculate subOrder based on effectType.

        // If subOrder is not already set, calculate it based on effectType
        if handler.sub_order == 0 {
            // https://www.smogon.com/forums/threads/sword-shield-battle-mechanics-research.3655528/page-59#post-8685465
            handler.sub_order = match handler.effect_type {
                EffectType::ZMove => 1,
                EffectType::Condition => 2,
                EffectType::SlotCondition => 3,
                EffectType::SideCondition => 4,
                EffectType::FieldCondition => 5,
                EffectType::Weather => 5,
                EffectType::Format => 5,
                EffectType::Rule => 5,
                EffectType::Ruleset => 5,
                EffectType::Ability => {
                    // JS: if (handler.effect.name === 'Poison Touch' || handler.effect.name === 'Perish Body') { handler.subOrder = 6; }
                    // JS: else if (handler.effect.name === 'Stall') { handler.subOrder = 9; }
                    let ability_name = handler.effect_id.as_str();
                    if ability_name == "poisontouch" || ability_name == "perishbody" {
                        6
                    } else if ability_name == "stall" {
                        9
                    } else {
                        7
                    }
                }
                EffectType::Item => 8,
                _ => 0,
            };

            // JS: if (handler.effect.effectType === 'Condition' && handler.state?.target instanceof Side)
            // Check if this is a slot/side/field condition based on effect_type already set above
            // (This was already handled in match statement above)
        }

        // JS: if (callbackName.endsWith('SwitchIn') || callbackName.endsWith('RedirectTarget'))
        if callback_name.ends_with("SwitchIn") || callback_name.ends_with("RedirectTarget") {
            // JS: handler.effectOrder = handler.state?.effectOrder;
            if let Some(state) = &handler.state {
                handler.effect_order = Some(state.effect_order);
            }
        }

        // JS: if (handler.effectHolder && (handler.effectHolder as Pokemon).getStat)
        if let Some(effect_holder) = handler.effect_holder {
            // Get Pokemon speed
            if let Some(side) = self.sides.get(effect_holder.0) {
                if let Some(pokemon) = side.pokemon.get(effect_holder.1) {
                    handler.speed = Some(pokemon.stored_stats.spe as i32);

                    // JS: if (handler.effect.effectType === 'Ability' && handler.effect.name === 'Magic Bounce' && callbackName === 'onAllyTryHitSide')
                    if handler.effect_type == EffectType::Ability
                        && handler.effect_id.as_str() == "magicbounce"
                        && callback_name == "onAllyTryHitSide" {
                        // JS: handler.speed = pokemon.getStat('spe', true, true);
                        // Get unmodified speed stat (unboosted=true, unmodified=true)
                        // When both flags are true, getStat returns storedStats without any modifications
                        handler.speed = Some(pokemon.get_stat(StatID::Spe, true));
                    }

                    // JS: if (callbackName.endsWith('SwitchIn'))
                    if callback_name.ends_with("SwitchIn") {
                        // JS: const fieldPositionValue = pokemon.side.n * this.sides.length + pokemon.position;
                        // JS: handler.speed -= this.speedOrder.indexOf(fieldPositionValue) / (this.activePerHalf * 2);
                        //
                        // This adjusts speed for switch-in ordering by subtracting a fractional value
                        // based on field position in the pre-sorted speed order
                        //
                        // TODO: Requires this.speedOrder array which stores pre-sorted field positions
                        // For now, skip this adjustment
                    }
                }
            }
        }
    }

    /// Retarget the last executed move
    /// Equivalent to battle.ts retargetLastMove()
    // 
    // 	retargetLastMove(newTarget: Pokemon) {
    // 		if (this.lastMoveLine < 0) return;
    // 		const parts = this.log[this.lastMoveLine].split('|');
    // 		parts[4] = newTarget.toString();
    // 		this.log[this.lastMoveLine] = parts.join('|');
    // 	}
    //
    pub fn retarget_last_move(&mut self, new_target: (usize, usize)) {
        // JS: if (this.lastMoveLine < 0) return;
        if self.last_move_line < 0 {
            return;
        }

        let line_idx = self.last_move_line as usize;
        if line_idx >= self.log.len() {
            return;
        }

        // Get the new target's string representation
        let new_target_str = if let Some(side) = self.sides.get(new_target.0) {
            if let Some(pokemon) = side.pokemon.get(new_target.1) {
                format!("{}: {}", side.id_str(), pokemon.name)
            } else {
                return;
            }
        } else {
            return;
        };

        // Split the log line and update target (part 4)
        // JS: const parts = this.log[this.lastMoveLine].split('|');
        // JS: parts[4] = newTarget.toString();
        let parts: Vec<&str> = self.log[line_idx].split('|').collect();
        let mut new_parts: Vec<String> = parts.iter().map(|s| s.to_string()).collect();
        if new_parts.len() > 4 {
            new_parts[4] = new_target_str;
        }
        self.log[line_idx] = new_parts.join("|");
    }

    /// Handle team preview phase
    /// Equivalent to battle.ts runPickTeam()
    ///
    // 
    // 	runPickTeam() {
    // 		// onTeamPreview handlers are expected to show full teams to all active sides,
    // 		// and send a 'teampreview' request for players to pick their leads / team order.
    // 		this.format.onTeamPreview?.call(this);
    // 		for (const rule of this.ruleTable.keys()) {
    // 			if ('+*-!'.includes(rule.charAt(0))) continue;
    // 			const subFormat = this.dex.formats.get(rule);
    // 			subFormat.onTeamPreview?.call(this);
    // 		}
    // 
    // 		if (this.requestState === 'teampreview') {
    // 			return;
    // 		}
    // 
    // 		if (this.ruleTable.pickedTeamSize) {
    // 			// There was no onTeamPreview handler (e.g. Team Preview rule missing).
    // 			// Players must still pick their own Pokémon, so we show them privately.
    // 			this.add('clearpoke');
    // 			for (const pokemon of this.getAllPokemon()) {
    // 				// Still need to hide these formes since they change on battle start
    // 				const details = pokemon.details.replace(', shiny', '')
    // 					.replace(/(Zacian|Zamazenta)(?!-Crowned)/g, '$1-*')
    // 					.replace(/(Xerneas)(-[a-zA-Z?-]+)?/g, '$1-*');
    // 				this.addSplit(pokemon.side.id, ['poke', pokemon.side.id, details, '']);
    // 			}
    // 			this.makeRequest('teampreview');
    // 		}
    // 	}
    //
    pub fn run_pick_team(&mut self) {
        // JS: this.format.onTeamPreview?.call(this);
        // TODO: Implement format.onTeamPreview callback

        // JS: for (const rule of this.ruleTable.keys()) { ... subFormat.onTeamPreview?.call(this); }
        // TODO: Implement ruleTable iteration and subFormat.onTeamPreview callbacks

        // JS: if (this.requestState === 'teampreview') { return; }
        if matches!(self.request_state, BattleRequestState::TeamPreview) {
            return;
        }

        // JS: if (this.ruleTable.pickedTeamSize) { ... }
        // If pickedTeamSize is set, show Pokemon privately (no onTeamPreview handler ran)
        if let Some(ref rule_table) = self.rule_table {
            if rule_table.picked_team_size.is_some() {
                // There was no onTeamPreview handler (e.g. Team Preview rule missing).
                // Players must still pick their own Pokémon, so we show them privately.
                // JS: this.add('clearpoke');
                self.add_log("clearpoke", &[]);

                // JS: for (const pokemon of this.getAllPokemon()) { ... }
                // Collect Pokemon data first to avoid borrow checker issues
                let all_pokemon = self.get_all_pokemon();
                let pokemon_data: Vec<(String, usize)> = all_pokemon.iter().map(|pokemon| {
                    // Still need to hide these formes since they change on battle start
                    // JS: const details = pokemon.details.replace(', shiny', '')
                    //         .replace(/(Zacian|Zamazenta)(?!-Crowned)/g, '$1-*')
                    //         .replace(/(Xerneas)(-[a-zA-Z?-]+)?/g, '$1-*');
                    let mut details = pokemon.details().clone();
                    details = details.replace(", shiny", "");

                    // Hide Zacian/Zamazenta forme (becomes Crowned on battle start)
                    if details.contains("Zacian") && !details.contains("Zacian-Crowned") {
                        details = details.replace("Zacian", "Zacian-*");
                    }
                    if details.contains("Zamazenta") && !details.contains("Zamazenta-Crowned") {
                        details = details.replace("Zamazenta", "Zamazenta-*");
                    }

                    // Hide Xerneas forme - replace "Xerneas" or "Xerneas-<forme>" with "Xerneas-*"
                    // JS regex: /(Xerneas)(-[a-zA-Z?-]+)?/g -> $1-*
                    if details.contains("Xerneas") {
                        if let Some(pos) = details.find("Xerneas") {
                            let before = &details[..pos];
                            let after = &details[pos + 7..]; // "Xerneas" is 7 chars

                            // Find where the forme name ends (at comma or end of string)
                            let after_cleaned = if after.starts_with('-') {
                                // Has a forme suffix - find the next comma or use empty string
                                if let Some(comma_pos) = after.find(',') {
                                    &after[comma_pos..]
                                } else {
                                    ""
                                }
                            } else {
                                // No forme suffix, keep the rest as-is
                                after
                            };

                            details = format!("{}Xerneas-*{}", before, after_cleaned);
                        }
                    }

                    (details, pokemon.side_index)
                }).collect();

                // Drop the immutable borrow
                drop(all_pokemon);

                // Now we can call methods that need mutable borrows
                for (details, side_index) in pokemon_data {
                    // JS: this.addSplit(pokemon.side.id, ['poke', pokemon.side.id, details, '']);
                    let side_id = if side_index == 0 { "p1" } else { "p2" };
                    self.add_split(side_id, &["poke", side_id, &details, ""], None);
                }

                // JS: this.makeRequest('teampreview');
                self.make_request(Some(BattleRequestState::TeamPreview));
                return;
            }
        }

        // For now, assume we need team preview and call makeRequest
        // JS: this.makeRequest('teampreview');
        self.make_request(Some(BattleRequestState::TeamPreview));
    }

    /// Send updates to connected players
    /// Equivalent to battle.ts sendUpdates()
    // 
    // 	sendUpdates() {
    // 		if (this.sentLogPos >= this.log.length) return;
    // 		this.send('update', this.log.slice(this.sentLogPos));
    // 		if (!this.sentRequests) {
    // 			for (const side of this.sides) side.emitRequest();
    // 			this.sentRequests = true;
    // 		}
    // 		this.sentLogPos = this.log.length;
    // 
    // 		if (!this.sentEnd && this.ended) {
    // 			const log = {
    // 				winner: this.winner,
    // 				seed: this.prngSeed,
    // 				turns: this.turn,
    // 				p1: this.sides[0].name,
    // 				p2: this.sides[1].name,
    // 				p3: this.sides[2]?.name,
    // 				p4: this.sides[3]?.name,
    // 				p1team: this.sides[0].team,
    // 				p2team: this.sides[1].team,
    // 				p3team: this.sides[2]?.team,
    // 				p4team: this.sides[3]?.team,
    // 				score: [this.sides[0].pokemonLeft, this.sides[1].pokemonLeft],
    // 				inputLog: this.inputLog,
    // 			};
    // 			if (this.sides[2]) {
    // 				log.score.push(this.sides[2].pokemonLeft);
    // 			} else {
    // 				delete log.p3;
    // 				delete log.p3team;
    // 			}
    // 			if (this.sides[3]) {
    // 				log.score.push(this.sides[3].pokemonLeft);
    // 			} else {
    // 				delete log.p4;
    // 				delete log.p4team;
    // 			}
    // 			this.send('end', JSON.stringify(log));
    // 			this.sentEnd = true;
    // 		}
    // 	}
    //
    pub fn send_updates(&mut self) {
        // In the full implementation, this would send log entries to players
        // For now, update sent position
        self.sent_log_pos = self.log.len();
    }

    /// Show open team sheets to players
    /// Equivalent to battle.ts showOpenTeamSheets()
    // 
    // 	showOpenTeamSheets() {
    // 		if (this.turn !== 0) return;
    // 		for (const side of this.sides) {
    // 			const team = side.pokemon.map(pokemon => {
    // 				const set = pokemon.set;
    // 				const newSet: PokemonSet = {
    // 					name: '',
    // 					species: set.species,
    // 					item: set.item,
    // 					ability: set.ability,
    // 					moves: set.moves,
    // 					nature: '',
    // 					gender: pokemon.gender,
    // 					evs: null!,
    // 					ivs: null!,
    // 					level: set.level,
    // 				};
    // 				if (this.gen === 8) newSet.gigantamax = set.gigantamax;
    // 				if (this.gen === 9) newSet.teraType = set.teraType;
    // 				// Only display Hidden Power type if the Pokemon has Hidden Power
    // 				// This is based on how team sheets were written in past VGC formats
    // 				if (set.moves.some(m => this.dex.moves.get(m).id === 'hiddenpower')) newSet.hpType = set.hpType;
    // 				// This is done so the client doesn't flag Zacian/Zamazenta as illusions
    // 				// when they use their signature move
    // 				if ((toID(set.species) === 'zacian' && toID(set.item) === 'rustedsword') ||
    // 					(toID(set.species) === 'zamazenta' && toID(set.item) === 'rustedshield')) {
    // 					newSet.species = Dex.species.get(set.species + 'crowned').name;
    // 					const crowned: { [k: string]: string } = {
    // 						'Zacian-Crowned': 'behemothblade', 'Zamazenta-Crowned': 'behemothbash',
    // 					};
    // 					const ironHeadIndex = set.moves.map(toID).indexOf('ironhead' as ID);
    // 					if (ironHeadIndex >= 0) {
    // 						newSet.moves[ironHeadIndex] = crowned[newSet.species];
    // 					}
    // 				}
    // 				return newSet;
    // 			});
    // 
    // 			this.add('showteam', side.id, Teams.pack(team));
    // 		}
    // 	}
    //
    pub fn show_open_team_sheets(&mut self, _side_idx: Option<usize>) {
        // In the full implementation, this would reveal team sheets
        self.add_log("-message", &["Team sheets revealed"]);
    }

    /// Calculate modified stats from base stats
    /// Equivalent to battle.ts spreadModify(baseStats, set)
    ///
    // TypeScript source:
    // /** Given a table of base stats and a pokemon set, return the actual stats. */
    // 	spreadModify(baseStats: StatsTable, set: PokemonSet): StatsTable {
    // 		const modStats: StatsTable = { hp: 0, atk: 0, def: 0, spa: 0, spd: 0, spe: 0 };
    // 		for (const statName in baseStats) {
    // 			modStats[statName as StatID] = this.statModify(baseStats, set, statName as StatID);
    // 		}
    // 		return modStats;
    // 	}
    //
    pub fn spread_modify(&self, base_stats: &StatsTable, set: &PokemonSet) -> StatsTable {
        StatsTable {
            hp: self.stat_modify(base_stats, set, "hp"),
            atk: self.stat_modify(base_stats, set, "atk"),
            def: self.stat_modify(base_stats, set, "def"),
            spa: self.stat_modify(base_stats, set, "spa"),
            spd: self.stat_modify(base_stats, set, "spd"),
            spe: self.stat_modify(base_stats, set, "spe"),
        }
    }

    /// Calculate a single stat from base stat, IVs, EVs, level, and nature
    /// Equivalent to battle.ts statModify(baseStats, set, statName)
    ///
    // 
    // 	statModify(baseStats: StatsTable, set: PokemonSet, statName: StatID): number {
    // 		const tr = this.trunc;
    // 		let stat = baseStats[statName];
    // 		if (statName === 'hp') {
    // 			return tr(tr(2 * stat + set.ivs[statName] + tr(set.evs[statName] / 4) + 100) * set.level / 100 + 10);
    // 		}
    // 		stat = tr(tr(2 * stat + set.ivs[statName] + tr(set.evs[statName] / 4)) * set.level / 100 + 5);
    // 		const nature = this.dex.natures.get(set.nature);
    // 		// Natures are calculated with 16-bit truncation.
    // 		// This only affects Eternatus-Eternamax in Pure Hackmons.
    // 		if (nature.plus === statName) {
    // 			stat = this.ruleTable.has('overflowstatmod') ? Math.min(stat, 595) : stat;
    // 			stat = tr(tr(stat * 110, 16) / 100);
    // 		} else if (nature.minus === statName) {
    // 			stat = this.ruleTable.has('overflowstatmod') ? Math.min(stat, 728) : stat;
    // 			stat = tr(tr(stat * 90, 16) / 100);
    // 		}
    // 		return stat;
    // 	}
    //
    pub fn stat_modify(&self, base_stats: &StatsTable, set: &PokemonSet, stat_name: &str) -> i32 {
        let base_stat = match stat_name {
            "hp" => base_stats.hp,
            "atk" => base_stats.atk,
            "def" => base_stats.def,
            "spa" => base_stats.spa,
            "spd" => base_stats.spd,
            "spe" => base_stats.spe,
            _ => return 0,
        };

        let iv = match stat_name {
            "hp" => set.ivs.hp,
            "atk" => set.ivs.atk,
            "def" => set.ivs.def,
            "spa" => set.ivs.spa,
            "spd" => set.ivs.spd,
            "spe" => set.ivs.spe,
            _ => 31,
        } as i32;

        let ev = match stat_name {
            "hp" => set.evs.hp,
            "atk" => set.evs.atk,
            "def" => set.evs.def,
            "spa" => set.evs.spa,
            "spd" => set.evs.spd,
            "spe" => set.evs.spe,
            _ => 0,
        } as i32;

        if stat_name == "hp" {
            // JS: return tr(tr(2 * stat + set.ivs[statName] + tr(set.evs[statName] / 4) + 100) * set.level / 100 + 10);
            let ev_contrib = self.trunc(ev as f64 / 4.0);
            let inner = self.trunc((2 * base_stat + iv + ev_contrib + 100) as f64);
            return self.trunc(inner as f64 * set.level as f64 / 100.0 + 10.0);
        }

        // Non-HP stats
        // JS: stat = tr(tr(2 * stat + set.ivs[statName] + tr(set.evs[statName] / 4)) * set.level / 100 + 5);
        let ev_contrib = self.trunc(ev as f64 / 4.0);
        let inner = self.trunc((2 * base_stat + iv + ev_contrib) as f64);
        let mut stat = self.trunc(inner as f64 * set.level as f64 / 100.0 + 5.0);

        // Apply nature
        // JS: const nature = this.dex.natures.get(set.nature);
        // JS: if (nature.plus === statName) {
        //       stat = this.ruleTable.has('overflowstatmod') ? Math.min(stat, 595) : stat;
        //       stat = tr(tr(stat * 110, 16) / 100);
        //     }
        // JS: else if (nature.minus === statName) {
        //       stat = this.ruleTable.has('overflowstatmod') ? Math.min(stat, 728) : stat;
        //       stat = tr(tr(stat * 90, 16) / 100);
        //     }
        if !set.nature.is_empty() {
            if let Some(nature_data) = self.dex.get_nature(&set.nature) {
                // Check if this stat is boosted by nature (+10%)
                if let Some(ref plus) = nature_data.plus {
                    if plus == stat_name {
                        // Apply overflow protection if rule exists
                        // This only affects Eternatus-Eternamax in Pure Hackmons
                        if let Some(ref rule_table) = self.rule_table {
                            if rule_table.has("overflowstatmod") {
                                stat = stat.min(595);
                            }
                        }
                        // Apply 1.1x multiplier with 16-bit truncation
                        // Natures are calculated with 16-bit truncation
                        stat = crate::dex::Dex::trunc(crate::dex::Dex::trunc(stat as f64 * 110.0, 16) as f64 / 100.0, 0);
                    }
                }

                // Check if this stat is reduced by nature (-10%)
                if let Some(ref minus) = nature_data.minus {
                    if minus == stat_name {
                        // Apply overflow protection if rule exists
                        if let Some(ref rule_table) = self.rule_table {
                            if rule_table.has("overflowstatmod") {
                                stat = stat.min(728);
                            }
                        }
                        // Apply 0.9x multiplier with 16-bit truncation
                        stat = crate::dex::Dex::trunc(crate::dex::Dex::trunc(stat as f64 * 90.0, 16) as f64 / 100.0, 0);
                    }
                }
            }
        }

        stat.max(0)
    }

    /// Execute tiebreak logic
    /// Equivalent to battle.ts tiebreak()
    /// Tiebreaker logic for determining winner when time runs out
    /// Equivalent to battle.ts tiebreak() (battle.ts:1421-1462)
    ///
    // 
    // 	tiebreak() {
    // 		if (this.ended) return false;
    // 
    // 		this.inputLog.push(`>tiebreak`);
    // 		this.add('message', "Time's up! Going to tiebreaker...");
    // 		const notFainted = this.sides.map(side => (
    // 			side.pokemon.filter(pokemon => !pokemon.fainted).length
    // 		));
    // 		this.add('-message', this.sides.map((side, i) => (
    // 			`${side.name}: ${notFainted[i]} Pokemon left`
    // 		)).join('; '));
    // 		const maxNotFainted = Math.max(...notFainted);
    // 		let tiedSides = this.sides.filter((side, i) => notFainted[i] === maxNotFainted);
    // 		if (tiedSides.length <= 1) {
    // 			return this.win(tiedSides[0]);
    // 		}
    // 
    // 		const hpPercentage = tiedSides.map(side => (
    // 			side.pokemon.map(pokemon => pokemon.hp / pokemon.maxhp).reduce((a, b) => a + b) * 100 / 6
    // 		));
    // 		this.add('-message', tiedSides.map((side, i) => (
    // 			`${side.name}: ${Math.round(hpPercentage[i])}% total HP left`
    // 		)).join('; '));
    // 		const maxPercentage = Math.max(...hpPercentage);
    // 		tiedSides = tiedSides.filter((side, i) => hpPercentage[i] === maxPercentage);
    // 		if (tiedSides.length <= 1) {
    // 			return this.win(tiedSides[0]);
    // 		}
    // 
    // 		const hpTotal = tiedSides.map(side => (
    // 			side.pokemon.map(pokemon => pokemon.hp).reduce((a, b) => a + b)
    // 		));
    // 		this.add('-message', tiedSides.map((side, i) => (
    // 			`${side.name}: ${Math.round(hpTotal[i])} total HP left`
    // 		)).join('; '));
    // 		const maxTotal = Math.max(...hpTotal);
    // 		tiedSides = tiedSides.filter((side, i) => hpTotal[i] === maxTotal);
    // 		if (tiedSides.length <= 1) {
    // 			return this.win(tiedSides[0]);
    // 		}
    // 		return this.tie();
    // 	}
    //
    pub fn tiebreak(&mut self) -> Option<usize> {
        // JS: if (this.ended) return false;
        if self.ended {
            return None;
        }

        // Helper to convert usize to SideID
        let to_side_id = |idx: usize| -> SideID {
            match idx {
                0 => SideID::P1,
                1 => SideID::P2,
                2 => SideID::P3,
                _ => SideID::P4,
            }
        };

        // JS: this.add('message', "Time's up! Going to tiebreaker...");
        self.add("message", &[Arg::Str("Time's up! Going to tiebreaker...")]);

        // JS: const notFainted = this.sides.map(side => (
        //         side.pokemon.filter(pokemon => !pokemon.fainted).length
        //     ));
        let not_fainted: Vec<usize> = self.sides.iter()
            .map(|side| side.pokemon.iter().filter(|p| !p.fainted).count())
            .collect();

        // Log Pokemon counts
        let mut messages = Vec::new();
        for (i, side) in self.sides.iter().enumerate() {
            messages.push(format!("{}: {} Pokemon left", side.name, not_fainted[i]));
        }
        self.add("-message", &[Arg::Str(&messages.join("; "))]);

        // JS: const maxNotFainted = Math.max(...notFainted);
        // JS: let tiedSides = this.sides.filter((side, i) => notFainted[i] === maxNotFainted);
        let max_not_fainted = *not_fainted.iter().max().unwrap_or(&0);
        let mut tied_sides: Vec<usize> = (0..self.sides.len())
            .filter(|&i| not_fainted[i] == max_not_fainted)
            .collect();

        // JS: if (tiedSides.length <= 1) return this.win(tiedSides[0]);
        if tied_sides.len() == 1 {
            self.win(Some(to_side_id(tied_sides[0])));
            return Some(tied_sides[0]);
        }
        if tied_sides.is_empty() {
            self.tie();
            return None;
        }

        // JS: const hpPercentage = tiedSides.map(side => (
        //         side.pokemon.map(pokemon => pokemon.hp / pokemon.maxhp).reduce((a, b) => a + b) * 100 / 6
        //     ));
        let hp_percentage: Vec<f64> = tied_sides.iter()
            .map(|&side_idx| {
                let side = &self.sides[side_idx];
                let total: f64 = side.pokemon.iter()
                    .map(|p| if p.maxhp > 0 { p.hp as f64 / p.maxhp as f64 } else { 0.0 })
                    .sum();
                total * 100.0 / 6.0
            })
            .collect();

        // Log HP percentages
        let mut messages = Vec::new();
        for (i, &side_idx) in tied_sides.iter().enumerate() {
            messages.push(format!("{}: {}% total HP left",
                self.sides[side_idx].name,
                hp_percentage[i].round() as i32));
        }
        self.add("-message", &[Arg::Str(&messages.join("; "))]);

        // JS: const maxPercentage = Math.max(...hpPercentage);
        // JS: tiedSides = tiedSides.filter((side, i) => hpPercentage[i] === maxPercentage);
        let max_percentage = hp_percentage.iter().cloned().fold(0.0_f64, f64::max);
        tied_sides = tied_sides.into_iter()
            .enumerate()
            .filter(|(i, _)| (hp_percentage[*i] - max_percentage).abs() < 0.0001)
            .map(|(_, side_idx)| side_idx)
            .collect();

        // JS: if (tiedSides.length <= 1) return this.win(tiedSides[0]);
        if tied_sides.len() == 1 {
            self.win(Some(to_side_id(tied_sides[0])));
            return Some(tied_sides[0]);
        }
        if tied_sides.is_empty() {
            self.tie();
            return None;
        }

        // JS: const hpTotal = tiedSides.map(side => (
        //         side.pokemon.map(pokemon => pokemon.hp).reduce((a, b) => a + b)
        //     ));
        let hp_total: Vec<i32> = tied_sides.iter()
            .map(|&side_idx| {
                self.sides[side_idx].pokemon.iter()
                    .map(|p| p.hp)
                    .sum()
            })
            .collect();

        // Log HP totals
        let mut messages = Vec::new();
        for (i, &side_idx) in tied_sides.iter().enumerate() {
            messages.push(format!("{}: {} total HP left",
                self.sides[side_idx].name,
                hp_total[i]));
        }
        self.add("-message", &[Arg::Str(&messages.join("; "))]);

        // JS: const maxTotal = Math.max(...hpTotal);
        // JS: tiedSides = tiedSides.filter((side, i) => hpTotal[i] === maxTotal);
        let max_total = *hp_total.iter().max().unwrap_or(&0);
        tied_sides = tied_sides.into_iter()
            .enumerate()
            .filter(|(i, _)| hp_total[*i] == max_total)
            .map(|(_, side_idx)| side_idx)
            .collect();

        // JS: if (tiedSides.length <= 1) return this.win(tiedSides[0]);
        if tied_sides.len() == 1 {
            self.win(Some(to_side_id(tied_sides[0])));
            return Some(tied_sides[0]);
        }

        // JS: return this.tie();
        self.tie();
        None
    }

    /// Convert battle to JSON value
    /// Equivalent to battle.ts toJSON()
    ///
    // 
    // 	toJSON(): AnyObject {
    // 		return State.serializeBattle(this);
    // 	}
    //
    pub fn to_json(&self) -> serde_json::Value {
        // Delegate to state::serialize_battle just like JavaScript
        crate::state::serialize_battle(self)
    }

    /// Convert battle to string representation
    /// Equivalent to battle.ts toString()
    // 
    // 	toString() {
    // 		return `Battle: ${this.format}`;
    // 	}
    //
    pub fn to_string(&self) -> String {
        format!(
            "Battle: {} (Turn {}, {})",
            self.format_id.as_str(),
            self.turn,
            if self.ended { "ended" } else { "ongoing" }
        )
    }

    /// Find battle-level event handlers
    /// Equivalent to battle.ts findBattleEventHandlers()
    // 
    // 	findBattleEventHandlers(callbackName: string, getKey?: 'duration', customHolder?: Pokemon) {
    // 		const handlers: EventListener[] = [];
    // 
    // 		let callback;
    // 		const format = this.format;
    // 		callback = this.getCallback(this, format, callbackName);
    // 		if (callback !== undefined || (getKey && this.formatData[getKey])) {
    // 			handlers.push(this.resolvePriority({
    // 				effect: format, callback, state: this.formatData, end: null, effectHolder: customHolder || this,
    // 			}, callbackName));
    // 		}
    // 		if (this.events && (callback = this.events[callbackName]) !== undefined) {
    // 			for (const handler of callback) {
    // 				const state = (handler.target.effectType === 'Format') ? this.formatData : null;
    // 				handlers.push({
    // 					effect: handler.target, callback: handler.callback, state, end: null, effectHolder: customHolder || this,
    // 					priority: handler.priority, order: handler.order, subOrder: handler.subOrder,
    // 				});
    // 			}
    // 		}
    // 		return handlers;
    // 	}
    //
    pub fn find_battle_event_handlers(&self, event_id: &str) -> Vec<ID> {
        // In the full implementation, this would return format/rule handlers
        let _ = event_id;
        Vec::new()
    }

    /// Find field event handlers
    /// Equivalent to battle.ts findFieldEventHandlers()
    // 
    // 	findFieldEventHandlers(field: Field, callbackName: string, getKey?: 'duration', customHolder?: Pokemon) {
    // 		const handlers: EventListener[] = [];
    // 
    // 		let callback;
    // 		for (const id in field.pseudoWeather) {
    // 			const pseudoWeatherState = field.pseudoWeather[id];
    // 			const pseudoWeather = this.dex.conditions.getByID(id as ID);
    // 			callback = this.getCallback(field, pseudoWeather, callbackName);
    // 			if (callback !== undefined || (getKey && pseudoWeatherState[getKey])) {
    // 				handlers.push(this.resolvePriority({
    // 					effect: pseudoWeather, callback, state: pseudoWeatherState,
    // 					end: customHolder ? null : field.removePseudoWeather, effectHolder: customHolder || field,
    // 				}, callbackName));
    // 			}
    // 		}
    // 		const weather = field.getWeather();
    // 		callback = this.getCallback(field, weather, callbackName);
    // 		if (callback !== undefined || (getKey && this.field.weatherState[getKey])) {
    // 			handlers.push(this.resolvePriority({
    // 				effect: weather, callback, state: this.field.weatherState,
    // 				end: customHolder ? null : field.clearWeather, effectHolder: customHolder || field,
    // 			}, callbackName));
    // 		}
    // 		const terrain = field.getTerrain();
    // 		callback = this.getCallback(field, terrain, callbackName);
    // 		if (callback !== undefined || (getKey && field.terrainState[getKey])) {
    // 			handlers.push(this.resolvePriority({
    // 				effect: terrain, callback, state: field.terrainState,
    // 				end: customHolder ? null : field.clearTerrain, effectHolder: customHolder || field,
    // 			}, callbackName));
    // 		}
    // 
    // 		return handlers;
    // 	}
    //
    pub fn find_field_event_handlers(&self, event_id: &str) -> Vec<(ID, Option<(usize, usize)>)> {
        let mut handlers = Vec::new();
        let _ = event_id;

        // Add weather handler
        if !self.field.weather.is_empty() {
            handlers.push((self.field.weather.clone(), None));
        }

        // Add terrain handler
        if !self.field.terrain.is_empty() {
            handlers.push((self.field.terrain.clone(), None));
        }

        // Add pseudo-weather handlers
        for pw_id in self.field.pseudo_weather.keys() {
            handlers.push((pw_id.clone(), None));
        }

        handlers
    }

    /// Find side event handlers
    /// Equivalent to battle.ts findSideEventHandlers()
    // 
    // 	findSideEventHandlers(side: Side, callbackName: string, getKey?: 'duration', customHolder?: Pokemon) {
    // 		const handlers: EventListener[] = [];
    // 
    // 		for (const id in side.sideConditions) {
    // 			const sideConditionData = side.sideConditions[id];
    // 			const sideCondition = this.dex.conditions.getByID(id as ID);
    // 			const callback = this.getCallback(side, sideCondition, callbackName);
    // 			if (callback !== undefined || (getKey && sideConditionData[getKey])) {
    // 				handlers.push(this.resolvePriority({
    // 					effect: sideCondition, callback, state: sideConditionData,
    // 					end: customHolder ? null : side.removeSideCondition, effectHolder: customHolder || side,
    // 				}, callbackName));
    // 			}
    // 		}
    // 		return handlers;
    // 	}
    //
    pub fn find_side_event_handlers(&self, _event_id: &str, side_idx: usize) -> Vec<(ID, Option<(usize, usize)>)> {
        let mut handlers = Vec::new();

        if let Some(side) = self.sides.get(side_idx) {
            // Add side condition handlers
            for sc_id in side.side_conditions.keys() {
                handlers.push((sc_id.clone(), None));
            }
        }

        handlers
    }

    /// Find Pokemon event handlers
    /// Equivalent to battle.ts findPokemonEventHandlers() (battle.ts:1098-1157)
    ///
    // 
    // 	findPokemonEventHandlers(pokemon: Pokemon, callbackName: string, getKey?: 'duration') {
    // 		const handlers: EventListener[] = [];
    // 
    // 		const status = pokemon.getStatus();
    // 		let callback = this.getCallback(pokemon, status, callbackName);
    // 		if (callback !== undefined || (getKey && pokemon.statusState[getKey])) {
    // 			handlers.push(this.resolvePriority({
    // 				effect: status, callback, state: pokemon.statusState, end: pokemon.clearStatus, effectHolder: pokemon,
    // 			}, callbackName));
    // 		}
    // 		for (const id in pokemon.volatiles) {
    // 			const volatileState = pokemon.volatiles[id];
    // 			const volatile = this.dex.conditions.getByID(id as ID);
    // 			callback = this.getCallback(pokemon, volatile, callbackName);
    // 			if (callback !== undefined || (getKey && volatileState[getKey])) {
    // 				handlers.push(this.resolvePriority({
    // 					effect: volatile, callback, state: volatileState, end: pokemon.removeVolatile, effectHolder: pokemon,
    // 				}, callbackName));
    // 			}
    // 		}
    // 		const ability = pokemon.getAbility();
    // 		callback = this.getCallback(pokemon, ability, callbackName);
    // 		if (callback !== undefined || (getKey && pokemon.abilityState[getKey])) {
    // 			handlers.push(this.resolvePriority({
    // 				effect: ability, callback, state: pokemon.abilityState, end: pokemon.clearAbility, effectHolder: pokemon,
    // 			}, callbackName));
    // 		}
    // 		const item = pokemon.getItem();
    // 		callback = this.getCallback(pokemon, item, callbackName);
    // 		if (callback !== undefined || (getKey && pokemon.itemState[getKey])) {
    // 			handlers.push(this.resolvePriority({
    // 				effect: item, callback, state: pokemon.itemState, end: pokemon.clearItem, effectHolder: pokemon,
    // 			}, callbackName));
    // 		}
    // 		const species = pokemon.baseSpecies;
    // 		callback = this.getCallback(pokemon, species, callbackName);
    // 		if (callback !== undefined) {
    // 			handlers.push(this.resolvePriority({
    // 				effect: species, callback, state: pokemon.speciesState, end() {}, effectHolder: pokemon,
    // 			}, callbackName));
    // 		}
    // 		const side = pokemon.side;
    // 		for (const conditionid in side.slotConditions[pokemon.position]) {
    // 			const slotConditionState = side.slotConditions[pokemon.position][conditionid];
    // 			const slotCondition = this.dex.conditions.getByID(conditionid as ID);
    // 			callback = this.getCallback(pokemon, slotCondition, callbackName);
    // 			if (callback !== undefined || (getKey && slotConditionState[getKey])) {
    // 				handlers.push(this.resolvePriority({
    // 					effect: slotCondition,
    // 					callback,
    // 					state: slotConditionState,
    // 					end: side.removeSlotCondition,
    // 					endCallArgs: [side, pokemon, slotCondition.id],
    // 					effectHolder: pokemon,
    // 				}, callbackName));
    // 			}
    // 		}
    // 
    // 		return handlers;
    // 	}
    //
    pub fn find_pokemon_event_handlers(&self, _event_id: &str, target: (usize, usize)) -> Vec<(ID, Option<(usize, usize)>)> {
        let mut handlers = Vec::new();
        let (side_idx, poke_idx) = target;

        if let Some(side) = self.sides.get(side_idx) {
            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                // JS: const status = pokemon.getStatus();
                // Add status handler
                if !pokemon.status.is_empty() {
                    handlers.push((pokemon.status.clone(), Some(target)));
                }

                // JS: for (const id in pokemon.volatiles)
                // Add volatile handlers
                for volatile_id in pokemon.volatiles.keys() {
                    handlers.push((volatile_id.clone(), Some(target)));
                }

                // JS: const ability = pokemon.getAbility();
                // Add ability handler
                if !pokemon.ability.is_empty() {
                    handlers.push((pokemon.ability.clone(), Some(target)));
                }

                // JS: const item = pokemon.getItem();
                // Add item handler
                if !pokemon.item.is_empty() {
                    handlers.push((pokemon.item.clone(), Some(target)));
                }

                // JS: const species = pokemon.baseSpecies;
                // Add species handler (NEW! - was missing)
                handlers.push((pokemon.species_id.clone(), Some(target)));

                // JS: for (const conditionid in side.slotConditions[pokemon.position])
                // Add slot condition handlers (NEW! - was missing)
                if let Some(slot_conds) = side.slot_conditions.get(pokemon.position) {
                    for slot_cond_id in slot_conds.keys() {
                        handlers.push((slot_cond_id.clone(), Some(target)));
                    }
                }
            }
        }

        handlers
    }

    // =========================================================================
    // MOVE EXECUTION - Core damage and hit logic
    // Ported from battle-actions.ts
    // =========================================================================

    /// Get damage for a move
    /// Equivalent to getDamage() in battle-actions.ts:1583
    ///
    /// Returns:
    /// - Some(damage) - amount of damage to deal
    /// - Some(0) - move succeeds but deals 0 damage
    /// - None - move fails (no message)
    ///
    /// The false case (with error message) is handled by returning 0 and
    /// letting the caller add the fail message.
    pub fn get_damage(
        &mut self,
        source_pos: (usize, usize),
        target_pos: (usize, usize),
        move_id: &ID,
    ) -> Option<i32> {
        // Get move data
        let move_data = match self.dex.get_move(move_id.as_str()) {
            Some(m) => m.clone(),
            None => return None,
        };

        // Check immunity first
        // JavaScript: if (!target.runImmunity(move, !suppressMessages))
        // For now, we'll do a basic type check (full immunity checking would be more complex)
        let (target_side, target_poke) = target_pos;
        let target_types = if let Some(side) = self.sides.get(target_side) {
            if let Some(pokemon) = side.pokemon.get(target_poke) {
                pokemon.types.clone()
            } else {
                return None;
            }
        } else {
            return None;
        };

        // Check type immunity
        let effectiveness = crate::data::typechart::get_effectiveness_multi(&move_data.move_type, &target_types);
        if effectiveness == 0.0 {
            return None; // Immune
        }

        // OHKO moves
        if move_data.ohko.is_some() {
            let target_hp = if let Some(side) = self.sides.get(target_side) {
                if let Some(pokemon) = side.pokemon.get(target_poke) {
                    if self.gen == 3 {
                        pokemon.hp
                    } else {
                        pokemon.maxhp
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            };
            return Some(target_hp);
        }

        // Fixed damage moves
        if let Some(ref heal_tuple) = move_data.heal {
            // Heal moves have (numerator, denominator) format
            // But damage field would be different - this is actually heal, not damage
            // For actual fixed damage, we'd check move.damage field
            // For now, skip this
        }

        let mut base_power = move_data.base_power;
        if base_power == 0 {
            return Some(0); // undefined in JS - no damage dealt, move continues
        }

        // Calculate critical hit
        // JavaScript: let critRatio = this.battle.runEvent('ModifyCritRatio', source, target, move, move.critRatio || 0);
        let mut crit_ratio = move_data.crit_ratio;

        // Trigger ModifyCritRatio event to allow abilities to modify crit ratio
        if let Some(modified_crit) = self.run_event("ModifyCritRatio", Some(source_pos), Some(target_pos), Some(&move_data.id), Some(crit_ratio)) {
            crit_ratio = modified_crit;
        }

        // Clamp crit ratio based on generation
        let crit_mult = if self.gen <= 5 {
            crit_ratio = crit_ratio.clamp(0, 5);
            [0, 16, 8, 4, 3, 2]
        } else if self.gen == 6 {
            crit_ratio = crit_ratio.clamp(0, 4);
            [0, 16, 8, 2, 1, 0] // Padded to size 6, last element never accessed
        } else {
            crit_ratio = crit_ratio.clamp(0, 4);
            [0, 24, 8, 2, 1, 0] // Padded to size 6, last element never accessed
        };

        // Determine if this is a critical hit
        // JavaScript: moveHit.crit = move.willCrit || false; if (move.willCrit === undefined && critRatio) moveHit.crit = this.battle.randomChance(1, critMult[critRatio]);
        let mut is_crit = false;
        if crit_ratio > 0 && crit_ratio < crit_mult.len() as i32 {
            let crit_chance = crit_mult[crit_ratio as usize];
            if crit_chance > 0 {
                is_crit = self.random(crit_chance) == 0;
            }
        }

        // Trigger CriticalHit event to allow abilities to prevent/modify crit
        // JavaScript: if (moveHit.crit) moveHit.crit = this.battle.runEvent('CriticalHit', target, null, move);
        if is_crit {
            is_crit = self.run_event_bool("CriticalHit", Some(target_pos), None, Some(&move_data.id));
        }

        // Trigger BasePower event to allow abilities/items to modify base power
        // JavaScript: basePower = this.battle.runEvent('BasePower', source, target, move, basePower, true);
        if let Some(modified_bp) = self.run_event("BasePower", Some(source_pos), Some(target_pos), Some(&move_data.id), Some(base_power)) {
            base_power = modified_bp;
        }

        // Get attacker level
        let level = if let Some(side) = self.sides.get(source_pos.0) {
            if let Some(pokemon) = side.pokemon.get(source_pos.1) {
                pokemon.level as i32
            } else {
                return None;
            }
        } else {
            return None;
        };

        // Determine attack and defense stats
        let is_physical = move_data.category == "Physical";

        // Get attack stat with boosts
        let attack = if let Some(side) = self.sides.get(source_pos.0) {
            if let Some(pokemon) = side.pokemon.get(source_pos.1) {
                if is_physical {
                    let boost = pokemon.boosts.atk;
                    let base_stat = pokemon.stored_stats.atk as i32;
                    crate::battle_actions::BattleActions::calculate_stat_with_boost(base_stat, boost)
                } else {
                    let boost = pokemon.boosts.spa;
                    let base_stat = pokemon.stored_stats.spa as i32;
                    crate::battle_actions::BattleActions::calculate_stat_with_boost(base_stat, boost)
                }
            } else {
                return None;
            }
        } else {
            return None;
        };

        // Get defense stat with boosts
        let defense = if let Some(side) = self.sides.get(target_pos.0) {
            if let Some(pokemon) = side.pokemon.get(target_pos.1) {
                if is_physical {
                    let boost = pokemon.boosts.def;
                    let base_stat = pokemon.stored_stats.def as i32;
                    crate::battle_actions::BattleActions::calculate_stat_with_boost(base_stat, boost)
                } else {
                    let boost = pokemon.boosts.spd;
                    let base_stat = pokemon.stored_stats.spd as i32;
                    crate::battle_actions::BattleActions::calculate_stat_with_boost(base_stat, boost)
                }
            } else {
                return None;
            }
        } else {
            return None;
        };

        // Base damage calculation
        // JavaScript: int(int(int(2 * L / 5 + 2) * A * P / D) / 50)
        let base_damage = ((2 * level / 5 + 2) * base_power * attack / defense.max(1)) / 50;

        // Call modifyDamage for the full calculation (pass is_crit for damage multiplier)
        let damage = self.modify_damage(base_damage, source_pos, target_pos, &move_data, is_crit);

        Some(damage)
    }

    /// Apply damage modifiers
    /// Equivalent to modifyDamage() in battle-actions.ts:1722
    fn modify_damage(
        &mut self,
        mut base_damage: i32,
        source_pos: (usize, usize),
        target_pos: (usize, usize),
        move_data: &crate::dex::MoveData,
        is_crit: bool,
    ) -> i32 {
        // Add 2 to base damage
        base_damage += 2;

        // Apply critical hit multiplier
        // JavaScript: if (isCrit) baseDamage = tr(baseDamage * (move.critModifier || (this.battle.gen >= 6 ? 1.5 : 2)));
        if is_crit {
            let crit_multiplier = if self.gen >= 6 { 1.5 } else { 2.0 };
            base_damage = (base_damage as f64 * crit_multiplier) as i32;
        }

        // Get source and target data
        let (source_types, target_types) = {
            let source_types = if let Some(side) = self.sides.get(source_pos.0) {
                if let Some(pokemon) = side.pokemon.get(source_pos.1) {
                    pokemon.types.clone()
                } else {
                    vec![]
                }
            } else {
                vec![]
            };

            let target_types = if let Some(side) = self.sides.get(target_pos.0) {
                if let Some(pokemon) = side.pokemon.get(target_pos.1) {
                    pokemon.types.clone()
                } else {
                    vec![]
                }
            } else {
                vec![]
            };

            (source_types, target_types)
        };

        // Apply STAB (Same Type Attack Bonus)
        let has_stab = source_types.iter().any(|t| t == &move_data.move_type);
        if has_stab {
            base_damage = (base_damage as f64 * 1.5) as i32;
        }

        // Apply type effectiveness
        let effectiveness = crate::data::typechart::get_effectiveness_multi(&move_data.move_type, &target_types);
        base_damage = (base_damage as f64 * effectiveness) as i32;

        // Random factor (85-100%)
        let random_factor = 85 + self.random(16);
        base_damage = base_damage * random_factor / 100;

        // Trigger ModifyDamage event to allow custom damage modification
        // JavaScript: damage = this.battle.runEvent('ModifyDamage', pokemon, target, move, damage)
        if let Some(modified) = self.run_event("ModifyDamage", Some(target_pos), Some(source_pos), Some(&move_data.id), Some(base_damage)) {
            base_damage = modified;
        }

        base_damage.max(1)
    }

    /// Try to hit targets with a spread move
    /// Equivalent to trySpreadMoveHit() in battle-actions.ts:545
    ///
    /// This is the main entry point for move execution with the 7-step pipeline
    pub fn try_spread_move_hit(
        &mut self,
        targets: &[(usize, usize)],
        pokemon_pos: (usize, usize),
        move_id: &ID,
    ) -> bool {
        if targets.is_empty() {
            return false;
        }

        // For now, implement a simplified version that just calls spreadMoveHit
        // The full implementation would have the 7-step pipeline

        let mut target_list: Vec<Option<(usize, usize)>> = targets.iter().map(|&t| Some(t)).collect();

        let (damages, final_targets) = self.spread_move_hit(&target_list, pokemon_pos, move_id, false, false);

        // Check if any target was hit
        for (i, damage) in damages.iter().enumerate() {
            if let Some(dmg) = damage {
                if *dmg != 0 || final_targets.get(i).and_then(|t| *t).is_some() {
                    return true;
                }
            }
        }

        false
    }

    /// Spread move hit - handles individual target hit processing
    /// Equivalent to spreadMoveHit() in battle-actions.ts:1043
    ///
    /// Returns (damages, targets) where damages[i] corresponds to targets[i]
    fn spread_move_hit(
        &mut self,
        targets: &[Option<(usize, usize)>],
        source_pos: (usize, usize),
        move_id: &ID,
        is_secondary: bool,
        is_self: bool,
    ) -> (Vec<Option<i32>>, Vec<Option<(usize, usize)>>) {
        let mut damages: Vec<Option<i32>> = vec![Some(0); targets.len()];
        let mut final_targets = targets.to_vec();

        // Get move data
        let move_data = match self.dex.get_move(move_id.as_str()) {
            Some(m) => m.clone(),
            None => return (damages, final_targets),
        };

        // Step 1: TryHit event
        if !is_secondary && !is_self {
            for (i, &target) in targets.iter().enumerate() {
                if let Some(target_pos) = target {
                    // JavaScript: hitResult = this.battle.singleEvent('TryHit', moveData, {}, target, pokemon, move);
                    let hit_result = self.single_event("TryHit", move_id, Some(target_pos), Some(source_pos), Some(move_id));

                    // If TryHit returns false, the move fails
                    if matches!(hit_result, crate::event::EventResult::Boolean(false)) {
                        damages[i] = None;
                        final_targets[i] = None;
                    }
                }
            }
        }

        // Step 1.5: Accuracy check
        // JavaScript: accuracy = this.battle.runEvent('Accuracy', target, pokemon, move, accuracy);
        if !is_secondary && !is_self {
            for (i, &target) in targets.iter().enumerate() {
                if let Some(target_pos) = target {
                    // Skip if already failed TryHit
                    if damages[i].is_none() {
                        continue;
                    }

                    // Get base accuracy from move
                    let base_accuracy = match self.dex.get_move(move_id.as_str()) {
                        Some(m) => match m.accuracy {
                            crate::dex::Accuracy::Percent(p) => p,
                            crate::dex::Accuracy::AlwaysHits => {
                                // Always hits, skip accuracy check
                                continue;
                            }
                        },
                        None => continue,
                    };

                    // Trigger Accuracy event to allow abilities/items to modify accuracy
                    // JavaScript: accuracy = this.battle.runEvent('Accuracy', target, pokemon, move, accuracy);
                    let mut accuracy = base_accuracy;
                    if let Some(modified_acc) = self.run_event("Accuracy", Some(target_pos), Some(source_pos), Some(move_id), Some(accuracy)) {
                        accuracy = modified_acc;
                    }

                    // Check if move hits based on accuracy
                    // JavaScript: if (accuracy !== true && !this.battle.randomChance(accuracy, 100))
                    if accuracy < 100 {
                        let roll = self.random(100);
                        if roll >= accuracy {
                            // Move missed
                            damages[i] = None;
                            final_targets[i] = None;
                            // TODO: Add miss message: this.battle.add('-miss', pokemon, target);
                        }
                    }
                }
            }
        }

        // Step 2: Get damage for each target
        damages = self.get_spread_damage(&damages, targets, source_pos, move_id, is_secondary, is_self);

        // Step 3: Apply damage using spread_damage
        let damage_vals: Vec<Option<i32>> = damages.clone();
        let applied_damages = self.spread_damage(
            &damage_vals,
            &final_targets,
            Some(source_pos),
            Some(move_id),
            false,
        );

        for (i, &applied) in applied_damages.iter().enumerate() {
            damages[i] = applied;
            if applied.is_none() || applied == Some(0) {
                // Don't clear target on 0 damage - that's still a hit
                // Only clear on None (failed)
                if applied.is_none() {
                    final_targets[i] = None;
                }
            }
        }

        // Step 3.5: Trigger Hit events for successful hits
        // JavaScript: this.battle.runEvent('Hit', target, pokemon, move)
        for (i, &target) in final_targets.iter().enumerate() {
            if let Some(target_pos) = target {
                // Only trigger Hit if we actually dealt damage or the move succeeded
                if damages[i].is_some() {
                    self.run_event("Hit", Some(target_pos), Some(source_pos), Some(move_id), None);
                }
            }
        }

        // Step 4: Run move effects (boosts, status, healing, etc.)
        damages = self.run_move_effects(&damages, &final_targets, source_pos, &move_data, is_secondary, is_self);

        (damages, final_targets)
    }

    /// Get damage for each target in a spread move
    /// Equivalent to getSpreadDamage() in battle-actions.ts:1163
    fn get_spread_damage(
        &mut self,
        damages: &[Option<i32>],
        targets: &[Option<(usize, usize)>],
        source_pos: (usize, usize),
        move_id: &ID,
        _is_secondary: bool,
        _is_self: bool,
    ) -> Vec<Option<i32>> {
        let mut result_damages = damages.to_vec();

        for (i, &target) in targets.iter().enumerate() {
            if let Some(target_pos) = target {
                // Calculate damage using getDamage
                let cur_damage = self.get_damage(source_pos, target_pos, move_id);
                result_damages[i] = cur_damage;
            } else {
                result_damages[i] = None;
            }
        }

        result_damages
    }

    /// Run move effects (boosts, healing, status, etc.)
    /// Equivalent to runMoveEffects() in battle-actions.ts:1201
    fn run_move_effects(
        &mut self,
        damages: &[Option<i32>],
        targets: &[Option<(usize, usize)>],
        _source_pos: (usize, usize),
        move_data: &crate::dex::MoveData,
        _is_secondary: bool,
        _is_self: bool,
    ) -> Vec<Option<i32>> {
        let mut result_damages = damages.to_vec();

        for (i, &target) in targets.iter().enumerate() {
            if target.is_none() {
                continue;
            }

            let target_pos = target.unwrap();

            // Apply boosts
            if let Some(ref boosts_map) = move_data.boosts {
                // Convert HashMap<String, i32> to BoostsTable
                let mut boosts_table = crate::dex_data::BoostsTable::default();
                for (stat_name, &value) in boosts_map.iter() {
                    match stat_name.to_lowercase().as_str() {
                        "atk" => boosts_table.atk = value as i8,
                        "def" => boosts_table.def = value as i8,
                        "spa" => boosts_table.spa = value as i8,
                        "spd" => boosts_table.spd = value as i8,
                        "spe" => boosts_table.spe = value as i8,
                        "accuracy" => boosts_table.accuracy = value as i8,
                        "evasion" => boosts_table.evasion = value as i8,
                        _ => {}
                    }
                }

                // Apply boosts to target
                if let Some(side) = self.sides.get_mut(target_pos.0) {
                    if let Some(pokemon) = side.pokemon.get_mut(target_pos.1) {
                        for boost_id in crate::dex_data::BoostID::all() {
                            let change = boosts_table.get(*boost_id);
                            if change != 0 {
                                pokemon.boosts.boost(*boost_id, change);
                            }
                        }
                    }
                }
            }

            // Apply healing
            if let Some((heal_num, heal_denom)) = move_data.heal {
                let target_maxhp = if let Some(side) = self.sides.get(target_pos.0) {
                    if let Some(pokemon) = side.pokemon.get(target_pos.1) {
                        pokemon.maxhp
                    } else {
                        0
                    }
                } else {
                    0
                };

                if target_maxhp > 0 {
                    let heal_amount = target_maxhp * heal_num / heal_denom;
                    // Apply healing
                    let (current_hp, max_hp) = if let Some(side) = self.sides.get_mut(target_pos.0) {
                        if let Some(pokemon) = side.pokemon.get_mut(target_pos.1) {
                            let old_hp = pokemon.hp;
                            pokemon.hp = (pokemon.hp + heal_amount).min(pokemon.maxhp);
                            let healed = pokemon.hp - old_hp;
                            if healed > 0 {
                                (pokemon.hp, pokemon.maxhp)
                            } else {
                                (0, 0) // No healing occurred
                            }
                        } else {
                            (0, 0)
                        }
                    } else {
                        (0, 0)
                    };

                    // Log healing after releasing mutable borrow
                    if current_hp > 0 {
                        self.add_log("-heal", &[
                            &format!("p{}a", target_pos.0 + 1),
                            &format!("{}/{}", current_hp, max_hp),
                        ]);
                    }
                }
            }

            // Apply status
            if let Some(ref status) = move_data.status {
                if let Some(side) = self.sides.get_mut(target_pos.0) {
                    if let Some(pokemon) = side.pokemon.get_mut(target_pos.1) {
                        // Simple status application (full version would check immunity)
                        if pokemon.status.is_empty() {
                            pokemon.status = crate::dex_data::ID::new(status);
                            self.add_log("-status", &[
                                &format!("p{}a", target_pos.0 + 1),
                                status,
                            ]);
                        }
                    }
                }
            }

            // Keep damage result
            // In the real implementation, we'd check if any effects failed
            // and update result_damages[i] accordingly
        }

        result_damages
    }
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

fn game_type_to_string(game_type: &GameType) -> String {
    match game_type {
        GameType::Singles => "singles".to_string(),
        GameType::Doubles => "doubles".to_string(),
        GameType::Triples => "triples".to_string(),
        GameType::Rotation => "rotation".to_string(),
        GameType::Multi => "multi".to_string(),
        GameType::FreeForAll => "freeforall".to_string(),
    }
}

impl Battle {
    /// Register a custom event handler (for testing)
    /// JavaScript: onEvent(eventid: string, target: Format, ...rest: AnyObject[])
    ///
    /// # Arguments
    /// * `event_id` - Event name (e.g., "Hit", "ModifyDamage")
    /// * `callback` - Function to call when event fires
    ///
    /// # Example
    /// ```ignore
    /// battle.on_event("Hit", |ctx| {
    ///     println!("Hit event on {:?}", ctx.target);
    ///     None // Return None for no value, Some(n) to return a value
    /// });
    /// ```
    // TypeScript source:
    // /**
    // 	 * Use this function to attach custom event handlers to a battle. See Battle#runEvent for
    // 	 * more information on how to write callbacks for event handlers.
    // 	 *
    // 	 * Try to use this sparingly. Most event handlers can be simply placed in a format instead.
    // 	 *
    // 	 *     this.onEvent(eventid, target, callback)
    // 	 * will set the callback as an event handler for the target when eventid is called with the
    // 	 * default priority. Currently only valid formats are supported as targets but this will
    // 	 * eventually be expanded to support other target types.
    // 	 *
    // 	 *     this.onEvent(eventid, target, priority, callback)
    // 	 * will set the callback as an event handler for the target when eventid is called with the
    // 	 * provided priority. Priority can either be a number or an object that contains the priority,
    // 	 * order, and subOrder for the event handler as needed (undefined keys will use default values)
    // 	 */
    // 	onEvent(eventid: string, target: Format, ...rest: AnyObject[]) { // rest = [priority, callback]
    // 		if (!eventid) throw new TypeError("Event handlers must have an event to listen to");
    // 		if (!target) throw new TypeError("Event handlers must have a target");
    // 		if (!rest.length) throw new TypeError("Event handlers must have a callback");
    // 
    // 		if (target.effectType !== 'Format') {
    // 			throw new TypeError(`${target.name} is a ${target.effectType} but only Format targets are supported right now`);
    // 		}
    // 
    // 		let callback, priority, order, subOrder, data;
    // 		if (rest.length === 1) {
    // 			[callback] = rest;
    // 			priority = 0;
    // 			order = false;
    // 			subOrder = 0;
    // 		} else {
    // 			[data, callback] = rest;
    // 			if (typeof data === 'object') {
    // 				priority = data['priority'] || 0;
    // 				order = data['order'] || false;
    // 				subOrder = data['subOrder'] || 0;
    // 			} else {
    // 				priority = data || 0;
    // 				order = false;
    // 				subOrder = 0;
    // 			}
    // 		}
    // 
    // 		const eventHandler = { callback, target, priority, order, subOrder };
    // 
    // 		if (!this.events) this.events = {};
    // 		const callbackName = `on${eventid}`;
    // 		const eventHandlers = this.events[callbackName];
    // 		if (eventHandlers === undefined) {
    // 			this.events[callbackName] = [eventHandler];
    // 		} else {
    // 			eventHandlers.push(eventHandler);
    // 		}
    // 	}
    //
    pub fn on_event<F>(&mut self, event_id: &str, callback: F)
    where
        F: Fn(&EventContext) -> Option<i32> + Send + Sync + 'static,
    {
        self.on_event_priority(event_id, 0, callback);
    }

    /// Register a custom event handler with priority (for testing)
    /// JavaScript: onEvent(eventid: string, target: Format, priority: number, callback)
    ///
    /// # Arguments
    /// * `event_id` - Event name (e.g., "Hit", "ModifyDamage")
    /// * `priority` - Priority value (higher = called earlier)
    /// * `callback` - Function to call when event fires
    pub fn on_event_priority<F>(&mut self, event_id: &str, priority: i32, callback: F)
    where
        F: Fn(&EventContext) -> Option<i32> + Send + Sync + 'static,
    {
        if event_id.is_empty() {
            panic!("Event handlers must have an event to listen to");
        }

        let callback_name = format!("on{}", event_id);

        let handler = CustomEventHandler {
            callback: Box::new(callback),
            priority,
            order: false,
            sub_order: 0,
        };

        self.events.entry(callback_name)
            .or_insert_with(Vec::new)
            .push(handler);
    }

    /// Call custom event handlers for a given event
    /// Returns the last non-None value returned by a handler, if any
    ///
    /// This version is SAFE - no unsafe code needed because callbacks
    /// receive EventContext instead of &mut Battle, breaking the circular reference
    fn run_custom_event_handlers(&mut self, event_name: &str) -> Option<i32> {
        let callback_name = format!("on{}", event_name);

        // Check if there are any custom handlers for this event
        if !self.events.contains_key(&callback_name) {
            return None;
        }

        // Get sorted indices by priority (higher priority first)
        let sorted_indices: Vec<usize> = {
            let handlers = self.events.get(&callback_name).unwrap();
            let mut indices: Vec<usize> = (0..handlers.len()).collect();
            indices.sort_by(|&a, &b| {
                let pa = handlers[a].priority;
                let pb = handlers[b].priority;
                pb.cmp(&pa).then_with(|| a.cmp(&b)) // Descending priority, stable sort
            });
            indices
        };

        // Create EventContext from current state
        // We extract this before iterating to avoid borrow checker issues
        let event_context = if let Some(ref event_info) = self.current_event {
            EventContext::from_event_info(event_name, event_info, None)
        } else {
            EventContext::minimal(event_name)
        };

        let mut last_result = None;

        // SAFE: No unsafe code needed!
        // We can borrow self.events immutably and call callbacks safely
        // because callbacks don't receive &mut Battle anymore
        let handlers = self.events.get(&callback_name).unwrap();

        for &index in &sorted_indices {
            if index >= handlers.len() {
                continue;
            }

            // Call the callback with EventContext
            // This is completely safe - no circular borrowing!
            if let Some(result) = (handlers[index].callback)(&event_context) {
                last_result = Some(result);
            }
        }

        last_result
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

// =========================================================================
// Display trait (equivalent to battle.ts toString())
// =========================================================================

impl std::fmt::Display for Battle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Battle: {}", self.format_id.as_str())
    }
}
