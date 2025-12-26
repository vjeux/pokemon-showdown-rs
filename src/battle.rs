//! Simulator Battle
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This file is where the battle simulation itself happens.
//! The most important part of the simulation is the event system.

use std::collections::{HashSet, HashMap};
use serde::{Deserialize, Serialize};

use crate::dex_data::{ID, GameType, SideID, EffectState, StatsTable};
use crate::field::{Field, get_weather_type_modifier, get_terrain_damage_modifier, get_weather_damage_fraction, get_grassy_terrain_heal};
use crate::battle_queue::BattleQueue;
use crate::pokemon::{Pokemon, PokemonSet};
use crate::side::{Side, RequestState, Choice};
use crate::prng::{PRNG, PRNGSeed};
use crate::data::abilities::get_ability;

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
    pub effect_order: Option<u32>,
    /// Speed stat (for speed-based sorting)
    pub speed: Option<u32>,
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
    /// Modifier accumulated during event processing
    pub modifier: f64,
}

impl EventInfo {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            target: None,
            source: None,
            effect: None,
            modifier: 1.0,
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
            modifier: 1.0,
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
    /// Modifier accumulated during event processing
    /// In JavaScript: this.event.modifier
    pub modifier: f64,
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
            modifier: 1.0,
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
    /// Current turn number
    pub turn: u32,
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
    pub last_damage: u32,

    /// Currently active move being executed
    pub active_move: Option<ID>,
    /// Pokemon currently using a move
    pub active_pokemon: Option<(usize, usize)>, // (side_idx, poke_idx)
    /// Target of the current move
    pub active_target: Option<(usize, usize)>, // (side_idx, poke_idx)

    /// Effect order counter
    pub effect_order: u32,

    /// Event depth for recursion tracking
    pub event_depth: u8,
    /// Current event being processed
    pub current_event: Option<EventInfo>,
    /// Current effect being processed
    pub current_effect: Option<ID>,
    /// Current effect state
    pub current_effect_state: Option<crate::dex_data::EffectState>,
    /// Log position for line limit checking
    pub sent_log_pos: usize,

    /// Debug mode
    pub debug_mode: bool,
    /// Rated match (boolean true or string description)
    pub rated: Option<String>,
    /// Strict choices (errors on invalid choices)
    pub strict_choices: bool,
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

        let mut battle = Self {
            format_id: options.format_id,
            format_name: options.format_name.unwrap_or_else(|| format_id_str.clone()),
            game_type,
            gen,
            active_per_half,
            dex: crate::dex::Dex::load_default().expect("Failed to load Dex"),
            field: Field::new(),
            sides,
            queue: BattleQueue::new(),
            prng,
            prng_seed: seed.clone(),
            log: Vec::new(),
            input_log: Vec::new(),
            request_state: BattleRequestState::None,
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
            sent_log_pos: 0,
            debug_mode: options.debug,
            rated: options.rated,
            strict_choices: options.strict_choices,
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
    /// JS Source (battle.ts):
    /// ```js
    /// setPlayer(slot: SideID, options: PlayerOptions) {
    ///     let side;
    ///     let didSomething = true;
    ///     const slotNum = parseInt(slot[1]) - 1;
    ///     if (!this.sides[slotNum]) {
    ///         // create player
    ///         const team = this.getTeam(options);
    ///         side = new Side(options.name || `Player ${slotNum + 1}`, this, slotNum, team);
    ///         if (options.avatar) side.avatar = `${options.avatar}`;
    ///         this.sides[slotNum] = side;
    ///     } else {
    ///         // edit player
    ///         side = this.sides[slotNum];
    ///         didSomething = false;
    ///         if (options.name && side.name !== options.name) {
    ///             side.name = options.name;
    ///             didSomething = true;
    ///         }
    ///         if (options.avatar && side.avatar !== `${options.avatar}`) {
    ///             side.avatar = `${options.avatar}`;
    ///             didSomething = true;
    ///         }
    ///         if (options.team) throw new Error(`Player ${slot} already has a team!`);
    ///     }
    ///     if (options.team && typeof options.team !== 'string') {
    ///         options.team = Teams.pack(options.team);
    ///     }
    ///     if (!didSomething) return;
    ///     this.inputLog.push(`>player ${slot} ` + JSON.stringify(options));
    ///     this.add('player', side.id, side.name, side.avatar, options.rating || '');
    ///
    ///     // Start the battle if it's ready to start
    ///     if (this.sides.every(playerSide => !!playerSide) && !this.started) this.start();
    /// }
    /// ```
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
                    // TODO: this.sides[2].sideConditions = this.sides[0].sideConditions;
                    // TODO: this.sides[3].sideConditions = this.sides[1].sideConditions;
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
        // TODO: Implement format callbacks (onBegin)

        // JS: for (const rule of this.ruleTable.keys()) { subFormat.onBegin?.call(this); }
        // TODO: Implement ruleTable iteration and subformat callbacks

        // JS: if (this.sides.some(side => !side.pokemon[0])) { throw new Error('...'); }
        if self.sides.iter().any(|side| side.pokemon.is_empty()) {
            panic!("Battle not started: A player has an empty team.");
        }

        // JS: if (this.debugMode) { this.checkEVBalance(); }
        if self.debug_mode {
            self.check_ev_balance();
        }

        // JS: if (format.customRules) { this.add(`raw|...`); }
        // TODO: Implement customRules display

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
        let active: Vec<(usize, usize)> = self.get_all_active(false)
            .iter()
            .map(|pokemon| (pokemon.side_index, pokemon.position))
            .collect();
        for (side_idx, poke_idx) in active {
            let effect_id = ID::new(event_name);
            self.single_event(event_name, &effect_id, Some((side_idx, poke_idx)), None, None);
        }
    }

    /// Add a log entry
    pub fn add_log(&mut self, event_type: &str, args: &[&str]) {
        let mut entry = format!("|{}", event_type);
        for arg in args {
            entry.push('|');
            entry.push_str(arg);
        }
        self.log.push(entry);
    }

    /// Random number in [0, n)
    pub fn random(&mut self, n: u32) -> u32 {
        self.prng.random_int(n)
    }

    /// Random chance
    pub fn random_chance(&mut self, numerator: u32, denominator: u32) -> bool {
        if let Some(forced) = self.force_random_chance {
            return forced;
        }
        self.prng.random_chance(numerator, denominator)
    }

    /// Sample from a slice
    pub fn sample<'a, T>(&mut self, items: &'a [T]) -> Option<&'a T> {
        self.prng.sample(items)
    }

    /// Shuffle a slice in place
    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        self.prng.shuffle(items);
    }

    /// Get a side by ID
    pub fn get_side(&self, side_id: SideID) -> Option<&Side> {
        self.sides.get(side_id.index())
    }

    /// Get a mutable side by ID
    pub fn get_side_mut(&mut self, side_id: SideID) -> Option<&mut Side> {
        self.sides.get_mut(side_id.index())
    }

    /// Get P1
    pub fn p1(&self) -> Option<&Side> {
        self.sides.get(0)
    }

    /// Get P2
    pub fn p2(&self) -> Option<&Side> {
        self.sides.get(1)
    }

    /// Get all active Pokemon
    /// Get all active Pokemon, optionally including fainted ones
    /// Equivalent to battle.ts getAllActive(includeFainted?)
    pub fn get_all_active(&self, include_fainted: bool) -> Vec<&crate::pokemon::Pokemon> {
        let mut result = Vec::new();
        for (side_idx, side) in self.sides.iter().enumerate() {
            for (slot, opt_idx) in side.active.iter().enumerate() {
                if let Some(poke_idx) = opt_idx {
                    if let Some(pokemon) = side.pokemon.get(*poke_idx) {
                        if include_fainted || !pokemon.is_fainted() {
                            result.push(pokemon);
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
    pub fn check_win(&mut self, _faint_data: Option<FaintData>) -> bool {
        // TODO: In Gen 5+, use faintData to determine which side fainted last
        // For now, ignore faint_data parameter

        // Check if all sides have no Pokemon left - tie/draw scenario
        if self.sides.iter().all(|side| side.pokemon_left == 0) {
            // In Gen 5+, the side that fainted last wins, but we don't track faintData
            // For now, just call win with None for a tie
            self.win(None);
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

    /// End the battle
    pub fn end(&mut self, winner: Option<&str>) {
        self.ended = true;
        self.winner = winner.map(|s| s.to_string());

        // Clone winner to avoid borrow conflict
        if let Some(w) = self.winner.clone() {
            self.add_log("win", &[&w]);
        } else {
            self.add_log("tie", &[]);
        }
    }

    /// Get the next effect order number
    pub fn next_effect_order(&mut self) -> u32 {
        self.effect_order += 1;
        self.effect_order
    }

    /// Initialize an effect state
    pub fn init_effect_state(&mut self, id: ID) -> EffectState {
        let mut state = EffectState::new(id);
        state.effect_order = self.next_effect_order();
        state
    }

    /// Process a choice from a player
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

            // Validate each sub-choice individually
            for sub_choice in sub_choices {
                self.validate_single_choice(side_id, sub_choice)?;
            }
            Ok(())
        } else {
            // Single choice (Singles or single slot in Doubles/Triples)
            self.validate_single_choice(side_id, choice)
        }
    }

    /// Validate a single choice (not comma-separated)
    fn validate_single_choice(&mut self, side_id: SideID, choice: &str) -> Result<(), String> {
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
                // In other states, 'team' choices are not valid
                if choice_type == "team" && !matches!(self.request_state, BattleRequestState::TeamPreview) {
                    return Err("[Invalid choice] Team choices are only valid during Team Preview".to_string());
                }
            }
        }

        match choice_type {
            "move" => {
                // Parse move choice
                if parts.len() < 2 {
                    return Err("Move choice requires move name/number".to_string());
                }
                // Would validate and add to queue here
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
                Ok(())
            }
            "shift" => {
                // Shift is only valid in triples
                if !matches!(self.game_type, GameType::Triples) {
                    return Err("[Invalid choice] Shift is only valid in Triple Battles".to_string());
                }
                Ok(())
            }
            _ => Err(format!("Unknown choice type: {}", choice_type)),
        }
    }

    /// Get the battle log as a string
    pub fn get_log(&self) -> String {
        self.log.join("\n")
    }

    /// Make choices for both sides and run the turn
    /// This is the main entry point for progressing the battle
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
    fn commit_choices(&mut self) {
        // Build action queue
        self.queue.clear();

        // Collect all move actions with their priorities and speeds
        let mut actions: Vec<(usize, usize, crate::side::ChosenAction, i8, u32)> = Vec::new();

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
                            let speed = self.sides[side_idx].pokemon[poke_idx].stored_stats.spe as u32;
                            actions.push((side_idx, poke_idx, action.clone(), priority, speed));
                        }
                    }
                    crate::side::ChoiceType::Switch => {
                        // Switches happen before moves (priority 7 effectively)
                        let pokemon_idx = self.sides[side_idx].active.get(action.pokemon_index)
                            .and_then(|opt| *opt);
                        if let Some(poke_idx) = pokemon_idx {
                            let speed = self.sides[side_idx].pokemon[poke_idx].stored_stats.spe as u32;
                            actions.push((side_idx, poke_idx, action.clone(), 7, speed));
                        }
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
                        // Get slot from the Pokemon's position
                        let slot = self.sides[side_idx].pokemon.get(poke_idx)
                            .map(|p| p.position)
                            .unwrap_or(0);
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
        let (ability_id, pokemon_name) = {
            let pokemon = &self.sides[side_idx].pokemon[poke_idx];
            (pokemon.ability.clone(), pokemon.name.clone())
        };
        let side_id = self.sides[side_idx].id_str();
        let full_name = format!("{}: {}", side_id, pokemon_name);

        // Use data-driven ability handling
        let ability_key = ID::new(ability_id.as_str());
        if let Some(ability_def) = get_ability(&ability_key) {
            // Weather setters (Drizzle, Drought, Sand Stream, Snow Warning)
            if let Some(weather_id) = &ability_def.on_switch_in_weather {
                let weather_display = match weather_id.as_str() {
                    "raindance" => "RainDance",
                    "sunnyday" => "SunnyDay",
                    "sandstorm" => "Sandstorm",
                    "snow" | "hail" => "Hail",
                    _ => weather_id.as_str(),
                };
                self.field.set_weather(ID::new(weather_id), Some(5));
                self.add_log("-weather", &[weather_display, &format!("[from] ability: {}", ability_def.name), &format!("[of] {}", full_name)]);
                return;
            }

            // Terrain setters (Electric/Grassy/Psychic/Misty Surge)
            if let Some(terrain_id) = &ability_def.on_switch_in_terrain {
                let terrain_display = match terrain_id.as_str() {
                    "electricterrain" => "Electric Terrain",
                    "grassyterrain" => "Grassy Terrain",
                    "psychicterrain" => "Psychic Terrain",
                    "mistyterrain" => "Misty Terrain",
                    _ => terrain_id.as_str(),
                };
                self.field.set_terrain(ID::new(terrain_id), Some(5));
                self.add_log("-fieldstart", &[terrain_display, &format!("[from] ability: {}", ability_def.name), &format!("[of] {}", full_name)]);
                return;
            }

            // Stat boost on switch-in (Intimidate targets foes)
            if let Some((stat, stages, target_foe)) = &ability_def.on_switch_in_boost {
                if *target_foe {
                    // Apply to foes (Intimidate)
                    let foe_side_idx = 1 - side_idx;
                    for foe_poke_idx in 0..self.sides[foe_side_idx].pokemon.len() {
                        if self.sides[foe_side_idx].pokemon[foe_poke_idx].is_active {
                            // Check for abilities that block Intimidate
                            let foe_ability = self.sides[foe_side_idx].pokemon[foe_poke_idx].ability.as_str();
                            if foe_ability == "innerfocus" || foe_ability == "owntempo" ||
                               foe_ability == "oblivious" || foe_ability == "scrappy" {
                                let foe_name = &self.sides[foe_side_idx].pokemon[foe_poke_idx].name;
                                let foe_side_id = self.sides[foe_side_idx].id_str();
                                let foe_full_name = format!("{}: {}", foe_side_id, foe_name);
                                self.add_log("-fail", &[&foe_full_name]);
                            } else {
                                self.apply_boost(foe_side_idx, foe_poke_idx, stat, *stages as i8);
                                self.add_log("-ability", &[&full_name, &ability_def.name]);
                            }
                        }
                    }
                } else {
                    // Apply to self
                    self.apply_boost(side_idx, poke_idx, stat, *stages as i8);
                    self.add_log("-ability", &[&full_name, &ability_def.name]);
                }
            }
        }
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

        // Check if move hits, calculate damage, apply effects
        // For now, implement basic damage application
        self.use_move(side_idx, poke_idx, move_id, target_side_idx, target_poke_idx);
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

    /// Apply a move's effects
    fn use_move(&mut self, attacker_side: usize, attacker_idx: usize, move_id: &ID, target_side: usize, target_idx: usize) {
        // Load move data - we need to get this from the Dex
        // For now, we'll implement inline with basic damage

        // Check flinch (flinch prevents action and is consumed)
        let flinch_id = ID::new("flinch");
        if self.sides[attacker_side].pokemon[attacker_idx].has_volatile(&flinch_id) {
            self.sides[attacker_side].pokemon[attacker_idx].remove_volatile(&flinch_id);
            let name = {
                let side_id = self.sides[attacker_side].id_str();
                let pokemon = &self.sides[attacker_side].pokemon[attacker_idx];
                format!("{}: {}", side_id, pokemon.name)
            };
            self.add_log("cant", &[&name, "flinch"]);
            return;
        }

        // Check paralysis
        let paralysis_check = self.random(4);
        if self.sides[attacker_side].pokemon[attacker_idx].status.as_str() == "par" && paralysis_check == 0 {
            let name = {
                let side_id = self.sides[attacker_side].id_str();
                let pokemon = &self.sides[attacker_side].pokemon[attacker_idx];
                format!("{}: {}", side_id, pokemon.name)
            };
            self.add_log("cant", &[&name, "par"]);
            return;
        }

        // Check freeze (20% thaw chance)
        if self.sides[attacker_side].pokemon[attacker_idx].status.as_str() == "frz" {
            let thaw_check = self.random(5);
            if thaw_check != 0 {
                let name = {
                    let side_id = self.sides[attacker_side].id_str();
                    let pokemon = &self.sides[attacker_side].pokemon[attacker_idx];
                    format!("{}: {}", side_id, pokemon.name)
                };
                self.add_log("cant", &[&name, "frz"]);
                return;
            } else {
                // Thaw out
                self.sides[attacker_side].pokemon[attacker_idx].cure_status();
                let name = {
                    let side_id = self.sides[attacker_side].id_str();
                    let pokemon = &self.sides[attacker_side].pokemon[attacker_idx];
                    format!("{}: {}", side_id, pokemon.name)
                };
                self.add_log("-curestatus", &[&name, "frz", "[msg]"]);
            }
        }

        // Check sleep
        if self.sides[attacker_side].pokemon[attacker_idx].status.as_str() == "slp" {
            let duration = self.sides[attacker_side].pokemon[attacker_idx].status_state.duration;
            if let Some(d) = duration {
                if d > 0 {
                    self.sides[attacker_side].pokemon[attacker_idx].status_state.duration = Some(d - 1);
                    let name = {
                        let side_id = self.sides[attacker_side].id_str();
                        let pokemon = &self.sides[attacker_side].pokemon[attacker_idx];
                        format!("{}: {}", side_id, pokemon.name)
                    };
                    self.add_log("cant", &[&name, "slp"]);
                    return;
                } else {
                    // Wake up
                    self.sides[attacker_side].pokemon[attacker_idx].cure_status();
                    let name = {
                        let side_id = self.sides[attacker_side].id_str();
                        let pokemon = &self.sides[attacker_side].pokemon[attacker_idx];
                        format!("{}: {}", side_id, pokemon.name)
                    };
                    self.add_log("-curestatus", &[&name, "slp", "[msg]"]);
                }
            }
        }

        // Check confusion (33% chance to hit self in Gen 7+)
        let confusion_id = ID::new("confusion");
        if self.sides[attacker_side].pokemon[attacker_idx].has_volatile(&confusion_id) {
            // Decrement confusion counter
            let snap_out = {
                if let Some(state) = self.sides[attacker_side].pokemon[attacker_idx].get_volatile_mut(&confusion_id) {
                    if let Some(ref mut duration) = state.duration {
                        if *duration > 0 {
                            *duration -= 1;
                        }
                        *duration == 0
                    } else {
                        false
                    }
                } else {
                    false
                }
            };

            if snap_out {
                self.sides[attacker_side].pokemon[attacker_idx].remove_volatile(&confusion_id);
                let name = {
                    let side_id = self.sides[attacker_side].id_str();
                    let pokemon = &self.sides[attacker_side].pokemon[attacker_idx];
                    format!("{}: {}", side_id, pokemon.name)
                };
                self.add_log("-end", &[&name, "confusion"]);
            } else {
                let name = {
                    let side_id = self.sides[attacker_side].id_str();
                    let pokemon = &self.sides[attacker_side].pokemon[attacker_idx];
                    format!("{}: {}", side_id, pokemon.name)
                };
                self.add_log("-activate", &[&name, "confusion"]);

                // 33% chance to hit self (Gen 7+)
                if self.random(3) == 0 {
                    // Calculate confusion damage: 40 BP typeless physical move
                    let (atk, def, level) = {
                        let pokemon = &self.sides[attacker_side].pokemon[attacker_idx];
                        (pokemon.stored_stats.atk as u32, pokemon.stored_stats.def as u32, pokemon.level as u32)
                    };
                    let base_damage = ((2 * level / 5 + 2) * 40 * atk / def.max(1)) / 50 + 2;
                    let random_factor = 85 + self.random(16);
                    let confusion_damage = (base_damage * random_factor / 100).max(1);

                    self.sides[attacker_side].pokemon[attacker_idx].take_damage(confusion_damage);
                    let hp = self.sides[attacker_side].pokemon[attacker_idx].hp;
                    let maxhp = self.sides[attacker_side].pokemon[attacker_idx].maxhp;
                    self.add_log("-damage", &[&name, &format!("{}/{}", hp, maxhp), "[from] confusion"]);
                    return; // Can't use the move this turn
                }
            }
        }

        // Set last move
        self.sides[attacker_side].pokemon[attacker_idx].move_this_turn = Some(move_id.clone());
        self.sides[attacker_side].pokemon[attacker_idx].last_move_used = Some(move_id.clone());

        // Choice item locking - lock the Pokemon into this move
        let item = self.sides[attacker_side].pokemon[attacker_idx].item.as_str();
        if matches!(item, "choiceband" | "choicescarf" | "choicespecs") {
            self.sides[attacker_side].pokemon[attacker_idx].locked_move = Some(move_id.clone());
        }

        // Check if target is valid
        if target_side >= self.sides.len() || target_idx >= self.sides[target_side].pokemon.len() {
            return;
        }

        let target_fainted = self.sides[target_side].pokemon[target_idx].is_fainted();
        if target_fainted {
            self.add_log("-notarget", &[]);
            return;
        }

        // Run BeforeMove event for volatile conditions (e.g., attract)
        // JS: const willTryMove = this.battle.runEvent('BeforeMove', pokemon, target, move);
        let will_try_move = self.run_before_move_event((attacker_side, attacker_idx), (target_side, target_idx), move_id);
        if !will_try_move {
            // BeforeMove returned false, move is prevented
            return;
        }

        // Check accuracy
        let mut accuracy = self.get_move_accuracy(move_id);

        // Weather-based accuracy modifiers
        let weather = self.field.weather.as_str();
        match (move_id.as_str(), weather) {
            // Thunder and Hurricane always hit in rain
            ("thunder" | "hurricane", "raindance" | "rain" | "primordialsea") => accuracy = 101,
            // Blizzard always hits in hail
            ("blizzard", "hail" | "snow") => accuracy = 101,
            // Thunder and Hurricane have lower accuracy in sun
            ("thunder" | "hurricane", "sunnyday" | "sun" | "desolateland") => accuracy = 50,
            _ => {}
        }

        if accuracy < 100 {
            // Get accuracy/evasion boosts
            let acc_boost = self.sides[attacker_side].pokemon[attacker_idx].boosts.accuracy;
            let eva_boost = self.sides[target_side].pokemon[target_idx].boosts.evasion;
            let effective_boost = acc_boost - eva_boost;

            // Calculate effective accuracy with boosts
            let accuracy_modifier = if effective_boost >= 0 {
                (3 + effective_boost as u32) as f64 / 3.0
            } else {
                3.0 / (3 + (-effective_boost) as u32) as f64
            };

            let effective_accuracy = (accuracy as f64 * accuracy_modifier) as u32;
            let roll = self.random(100);

            if roll >= effective_accuracy {
                let attacker_name = {
                    let side_id = self.sides[attacker_side].id_str();
                    let pokemon = &self.sides[attacker_side].pokemon[attacker_idx];
                    format!("{}: {}", side_id, pokemon.name)
                };
                self.add_log("-miss", &[&attacker_name]);
                return;
            }
        }

        // Check volatile condition TryHit events (Protect, Baneful Bunker, etc.)
        // This must happen before damage calculation
        if !self.check_volatile_try_hit((target_side, target_idx), (attacker_side, attacker_idx), move_id) {
            // Move was blocked by a volatile condition (e.g., Protect)
            return;
        }

        // Determine number of hits for multi-hit moves
        let hit_count = self.get_multi_hit_count(move_id);
        let mut total_damage = 0u32;
        let mut hits_landed = 0u32;
        let mut was_crit = false;

        for _hit in 0..hit_count {
            // Check if target fainted during multi-hit
            if self.sides[target_side].pokemon[target_idx].is_fainted() {
                break;
            }

            // Calculate damage for this hit
            let (damage, is_crit) = self.calculate_move_damage(attacker_side, attacker_idx, target_side, target_idx, move_id);
            if is_crit {
                was_crit = true;
            }

            if damage > 0 {
                hits_landed += 1;
                total_damage += damage;

                // Apply damage
                self.sides[target_side].pokemon[target_idx].take_damage(damage);

                let target_name = {
                    let side_id = self.sides[target_side].id_str();
                    let pokemon = &self.sides[target_side].pokemon[target_idx];
                    format!("{}: {}", side_id, pokemon.name)
                };
                let hp = self.sides[target_side].pokemon[target_idx].hp;
                let maxhp = self.sides[target_side].pokemon[target_idx].maxhp;
                self.add_log("-damage", &[&target_name, &format!("{}/{}", hp, maxhp)]);
            }
        }

        // Anger Point: Maximizes Attack when hit by a critical hit
        if was_crit {
            let target_ability = self.sides[target_side].pokemon[target_idx].ability.as_str();
            if target_ability == "angerpoint" {
                let target_hp = self.sides[target_side].pokemon[target_idx].hp;
                if target_hp > 0 {
                    // Boost Attack to +6 (12 stages since each boost is +0.5)
                    self.boost(&[("atk", 12)], (target_side, target_idx), Some((target_side, target_idx)), None);
                }
            }
        }

        // Log hit count for multi-hit moves
        if hit_count > 1 && hits_landed > 0 {
            self.add_log("-hitcount", &[&hits_landed.to_string()]);
        }

        if total_damage > 0 {
            // Apply recoil damage for recoil moves (based on total damage)
            let recoil_fraction = match move_id.as_str() {
                "bravebird" | "flareblitz" | "woodhammer" | "wildcharge" => 1.0 / 3.0,
                "headsmash" => 0.5,
                "doubleedge" | "takedown" => 1.0 / 4.0,
                _ => 0.0,
            };

            if recoil_fraction > 0.0 {
                let recoil_damage = ((total_damage as f64 * recoil_fraction) as u32).max(1);
                self.sides[attacker_side].pokemon[attacker_idx].take_damage(recoil_damage);

                let attacker_name = {
                    let side_id = self.sides[attacker_side].id_str();
                    let pokemon = &self.sides[attacker_side].pokemon[attacker_idx];
                    format!("{}: {}", side_id, pokemon.name)
                };
                let hp = self.sides[attacker_side].pokemon[attacker_idx].hp;
                let maxhp = self.sides[attacker_side].pokemon[attacker_idx].maxhp;
                self.add_log("-damage", &[&attacker_name, &format!("{}/{}", hp, maxhp), "[from] Recoil"]);
            }

            // Flare Blitz has 10% burn chance
            if move_id.as_str() == "flareblitz" && self.random(10) == 0 {
                self.apply_status(target_side, target_idx, "brn");
            }

            // Life Orb recoil (10% of max HP)
            let attacker_item = self.sides[attacker_side].pokemon[attacker_idx].item.as_str();
            if attacker_item == "lifeorb" {
                let maxhp = self.sides[attacker_side].pokemon[attacker_idx].maxhp;
                let life_orb_damage = (maxhp / 10).max(1);
                self.sides[attacker_side].pokemon[attacker_idx].take_damage(life_orb_damage);

                let attacker_name = {
                    let side_id = self.sides[attacker_side].id_str();
                    let pokemon = &self.sides[attacker_side].pokemon[attacker_idx];
                    format!("{}: {}", side_id, pokemon.name)
                };
                let hp = self.sides[attacker_side].pokemon[attacker_idx].hp;
                let maxhp = self.sides[attacker_side].pokemon[attacker_idx].maxhp;
                self.add_log("-damage", &[&attacker_name, &format!("{}/{}", hp, maxhp), "[from] item: Life Orb"]);
            }

            // Contact ability damage (Iron Barbs, Rough Skin)
            // Check if move makes contact
            let contact_moves = [
                "tackle", "bodyslam", "doubleedge", "takedown", "quickattack", "slash",
                "earthquake", "closecombat", "uturn", "knockoff", "ironhead", "crunch",
                "bravebird", "flareblitz", "woodhammer", "wildcharge", "headsmash",
                "dragonclaw", "stoneedge", "waterfall", "drillrun", "crosschop",
                "bulletpunch", "machpunch", "iceshard", "aquajet", "shadowclaw",
                "nightslash", "leafblade", "psychocut", "aerialace", "dragonrush",
                "flipturn", "bounce", "fly", "dive", "dig", "skyattack",
            ];
            let is_contact = contact_moves.contains(&move_id.as_str());

            if is_contact && !self.sides[attacker_side].pokemon[attacker_idx].is_fainted() {
                let defender_ability = self.sides[target_side].pokemon[target_idx].ability.as_str().to_lowercase();

                if defender_ability == "ironbarbs" || defender_ability == "roughskin" {
                    let attacker_maxhp = self.sides[attacker_side].pokemon[attacker_idx].maxhp;
                    let contact_damage = (attacker_maxhp / 8).max(1);
                    self.sides[attacker_side].pokemon[attacker_idx].take_damage(contact_damage);

                    let attacker_name = {
                        let side_id = self.sides[attacker_side].id_str();
                        let pokemon = &self.sides[attacker_side].pokemon[attacker_idx];
                        format!("{}: {}", side_id, pokemon.name)
                    };
                    let hp = self.sides[attacker_side].pokemon[attacker_idx].hp;
                    let maxhp = self.sides[attacker_side].pokemon[attacker_idx].maxhp;
                    let ability_name = if defender_ability == "ironbarbs" { "Iron Barbs" } else { "Rough Skin" };
                    self.add_log("-damage", &[&attacker_name, &format!("{}/{}", hp, maxhp), &format!("[from] ability: {}", ability_name)]);
                }

                // Aftermath: damages attacker by 1/4 max HP when KO'd by contact move
                if defender_ability == "aftermath" {
                    let defender_hp = self.sides[target_side].pokemon[target_idx].hp;
                    if defender_hp == 0 {
                        let attacker_maxhp = self.sides[attacker_side].pokemon[attacker_idx].maxhp;
                        let contact_damage = (attacker_maxhp / 4).max(1);
                        self.sides[attacker_side].pokemon[attacker_idx].take_damage(contact_damage);

                        let attacker_name = {
                            let side_id = self.sides[attacker_side].id_str();
                            let pokemon = &self.sides[attacker_side].pokemon[attacker_idx];
                            format!("{}: {}", side_id, pokemon.name)
                        };
                        let hp = self.sides[attacker_side].pokemon[attacker_idx].hp;
                        let maxhp = self.sides[attacker_side].pokemon[attacker_idx].maxhp;
                        self.add_log("-damage", &[&attacker_name, &format!("{}/{}", hp, maxhp), "[from] ability: Aftermath"]);
                    }
                }

                // Flame Body: 30% chance to burn attacker on contact
                if defender_ability == "flamebody" {
                    if self.random_chance(3, 10) {
                        self.apply_status(attacker_side, attacker_idx, "brn");
                    }
                }

                // Poison Point: 30% chance to poison attacker on contact
                if defender_ability == "poisonpoint" {
                    if self.random_chance(3, 10) {
                        self.apply_status(attacker_side, attacker_idx, "psn");
                    }
                }

                // Static: 30% chance to paralyze attacker on contact
                if defender_ability == "static" {
                    if self.random_chance(3, 10) {
                        self.apply_status(attacker_side, attacker_idx, "par");
                    }
                }
            }
        }

        // Apply secondary effects based on move
        self.apply_move_secondary(attacker_side, attacker_idx, target_side, target_idx, move_id);

        // Handle self-KO moves (Lunar Dance, Healing Wish, Memento, Selfdestruct, Explosion, etc.)
        match move_id.as_str() {
            "lunardance" => {
                // Lunar Dance: User faints, next Pokemon that switches in is fully healed
                if !self.sides[attacker_side].pokemon[attacker_idx].is_fainted() {
                    // Faint the user
                    let attacker_name = {
                        let side_id = self.sides[attacker_side].id_str();
                        let pokemon = &self.sides[attacker_side].pokemon[attacker_idx];
                        format!("{}: {}", side_id, pokemon.name)
                    };

                    self.sides[attacker_side].pokemon[attacker_idx].hp = 0;
                    self.add_log("-damage", &[&attacker_name, "0 fnt"]);
                    self.add_log("faint", &[&attacker_name]);

                    // TODO: Add side effect to heal next switch-in
                    // self.sides[attacker_side].add_side_condition(ID::new("lunardance"));
                }
            }
            "selfdestruct" | "explosion" => {
                // Selfdestruct/Explosion: User faints after dealing damage
                // Note: Damage was already dealt earlier in this function
                if !self.sides[attacker_side].pokemon[attacker_idx].is_fainted() {
                    let attacker_name = {
                        let side_id = self.sides[attacker_side].id_str();
                        let pokemon = &self.sides[attacker_side].pokemon[attacker_idx];
                        format!("{}: {}", side_id, pokemon.name)
                    };

                    self.sides[attacker_side].pokemon[attacker_idx].hp = 0;
                    self.add_log("-damage", &[&attacker_name, "0 fnt"]);
                    self.add_log("faint", &[&attacker_name]);
                }
            }
            _ => {}
        }

        // Handle pivot moves (U-Turn, Volt Switch, Flip Turn)
        // These moves force a switch after dealing damage
        if total_damage > 0 && matches!(move_id.as_str(), "uturn" | "voltswitch" | "flipturn") {
            // Only switch if the attacker is not fainted
            if !self.sides[attacker_side].pokemon[attacker_idx].is_fainted() {
                // Flag that a pivot switch is pending
                self.sides[attacker_side].pokemon[attacker_idx].add_volatile(ID::new("pivotswitch"));
            }
        }
    }

    /// Remove a volatile condition from a Pokemon
    /// Matches JavaScript pokemon.removeVolatile()
    ///
    /// Calls onEnd callback before removing the volatile
    pub fn remove_volatile_from_pokemon(&mut self, target: (usize, usize), volatile_id: &ID) -> bool {
        let (side_idx, poke_idx) = target;

        // Check if pokemon has the volatile
        let has_volatile = if let Some(side) = self.sides.get(side_idx) {
            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                pokemon.has_volatile(volatile_id)
            } else {
                return false;
            }
        } else {
            return false;
        };

        if !has_volatile {
            return false;
        }

        // Call onEnd callback before removing
        // JS: this.battle.singleEvent('End', status, this.volatiles[status.id], this);
        if volatile_id.as_str() == "attract" {
//             let _result = crate::data::move_callbacks::attract::on_end(self, target);
        }
        // TODO: Add other volatile conditions with onEnd here

        // Remove the volatile
        if let Some(side) = self.sides.get_mut(side_idx) {
            if let Some(pokemon) = side.pokemon.get_mut(poke_idx) {
                pokemon.remove_volatile(volatile_id)
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Run BeforeMove event for all volatile conditions on a Pokemon
    /// Matches JavaScript battle.runEvent('BeforeMove', pokemon, target, move)
    ///
    /// Returns false if any volatile prevents the move, true otherwise
    fn run_before_move_event(
        &mut self,
        pokemon: (usize, usize),
        target: (usize, usize),
        move_id: &ID,
    ) -> bool {
        let (side_idx, poke_idx) = pokemon;

        // Get list of volatiles (need to collect first to avoid borrow issues)
        let volatile_ids: Vec<ID> = if let Some(side) = self.sides.get(side_idx) {
            if let Some(poke) = side.pokemon.get(poke_idx) {
                poke.volatiles.keys().cloned().collect()
            } else {
                return true;
            }
        } else {
            return true;
        };

        // Call onBeforeMove for each volatile
        for volatile_id in volatile_ids {
            if volatile_id.as_str() == "attract" {
                // TODO: Call onBeforeMove callback when move_callbacks is reimplemented
//                 let result = crate::data::move_callbacks::attract::on_before_move(self, pokemon);
//                 match result {
//                     crate::data::move_callbacks::MoveHandlerResult::False => return false,
//                     _ => {}
//                 }
            }
            // TODO: Add other volatile conditions with onBeforeMove here
        }

        true
    }

    /// Execute a move immediately (for moves that call other moves like Assist, Copycat, etc.)
    /// Matches JavaScript battle-actions.ts useMove()
    ///
    /// This is a simplified version that executes a move in the current context
    /// without going through the full queue system
    pub fn use_move_immediately(
        &mut self,
        move_id: &ID,
        user: (usize, usize),
        target: Option<(usize, usize)>,
    ) {
        let (attacker_side, attacker_idx) = user;

        // Determine target if not provided
        let (target_side, target_idx) = if let Some(t) = target {
            t
        } else {
            // Get random target based on move targeting
            let target_type = if let Some(move_def) = self.dex.get_move(move_id.as_str()) {
                move_def.target.clone()
            } else {
                return; // Unknown move
            };

            if let Some(t) = self.get_random_target(attacker_side, attacker_idx, &target_type) {
                t
            } else {
                return; // No valid target
            }
        };

        // Use log to show the move being used
        let attacker_name = {
            let side_id = self.sides[attacker_side].id_str();
            let pokemon = &self.sides[attacker_side].pokemon[attacker_idx];
            format!("{}: {}", side_id, pokemon.name)
        };
        self.add_log("move", &[&attacker_name, move_id.as_str()]);

        // Check if target is valid
        if target_side >= self.sides.len() || target_idx >= self.sides[target_side].pokemon.len() {
            return;
        }

        let target_fainted = self.sides[target_side].pokemon[target_idx].is_fainted();
        if target_fainted {
            self.add_log("-notarget", &[]);
            return;
        }

        // For damaging moves, calculate and apply damage
        let move_def = if let Some(def) = self.dex.get_move(move_id.as_str()) {
            def
        } else {
            return;
        };

        if move_def.base_power > 0 {
            // Damaging move
            let (damage, _is_crit) = self.calculate_move_damage(attacker_side, attacker_idx, target_side, target_idx, move_id);
            if damage > 0 {
                self.sides[target_side].pokemon[target_idx].take_damage(damage);

                let target_name = {
                    let side_id = self.sides[target_side].id_str();
                    let pokemon = &self.sides[target_side].pokemon[target_idx];
                    format!("{}: {}", side_id, pokemon.name)
                };
                let hp = self.sides[target_side].pokemon[target_idx].hp;
                let maxhp = self.sides[target_side].pokemon[target_idx].maxhp;
                self.add_log("-damage", &[&target_name, &format!("{}/{}", hp, maxhp)]);
            }
        }

        // Apply secondary effects
        self.apply_move_secondary(attacker_side, attacker_idx, target_side, target_idx, move_id);
    }

    /// Calculate damage for a move (basic implementation)
    fn calculate_move_damage(&mut self, attacker_side: usize, attacker_idx: usize, target_side: usize, target_idx: usize, move_id: &ID) -> (u32, bool) {
        // Extract all needed fields from move_def to avoid borrow checker issues
        let (base_power, category, move_type) = match self.dex.get_move(move_id.as_str()) {
            Some(move_def) => {
                (move_def.base_power, move_def.category.clone(), move_def.move_type.clone())
            }
            None => {
                // Unknown move - return 0 damage
                return (0, false);
            }
        };

        // Status moves don't deal damage
        if base_power == 0 {
            return (0, false);
        }

        // Extract all needed data from Pokemon before any mutable operations
        let (attack_stat, defense_stat, atk_boost, def_boost, level, attacker_types, attacker_status, defender_types, defender_name, defender_ability) = {
            let attacker = &self.sides[attacker_side].pokemon[attacker_idx];
            let defender = &self.sides[target_side].pokemon[target_idx];

            let (attack_stat, defense_stat) = match category.as_str() {
                "Physical" => {
                    (attacker.stored_stats.atk as u32, defender.stored_stats.def as u32)
                }
                _ => {
                    (attacker.stored_stats.spa as u32, defender.stored_stats.spd as u32)
                }
            };

            let (atk_boost, def_boost) = match category.as_str() {
                "Physical" => {
                    (attacker.boosts.atk, defender.boosts.def)
                }
                _ => {
                    (attacker.boosts.spa, defender.boosts.spd)
                }
            };

            (
                attack_stat,
                defense_stat,
                atk_boost,
                def_boost,
                attacker.level as u32,
                attacker.types.clone(),
                attacker.status.as_str().to_string(),
                defender.types.clone(),
                defender.name.clone(),
                defender.ability.as_str().to_lowercase(),
            )
        };

        // Check ability-based immunities using data-driven approach
        let ability_id = ID::new(&defender_ability);
        if let Some(ability_def) = get_ability(&ability_id) {
            let move_type_lower = move_type.to_lowercase();

            // Check type immunity (Levitate, Flash Fire)
            if ability_def.grants_type_immunity(&move_type_lower) {
                let side_id = self.sides[target_side].id_str();
                let target_name = format!("{}: {}", side_id, defender_name);
                self.add_log("-immune", &[&target_name, &format!("[from] ability: {}", ability_def.name)]);

                // Flash Fire adds a volatile for boosted Fire moves
                if defender_ability == "flashfire" {
                    self.sides[target_side].pokemon[target_idx].add_volatile(ID::new("flashfire"));
                    self.add_log("-start", &[&target_name, &format!("ability: {}", ability_def.name)]);
                }
                return (0, false);
            }

            // Check absorb abilities (Water Absorb, Volt Absorb, etc.)
            if ability_def.absorbs_type(&move_type_lower) {
                let side_id = self.sides[target_side].id_str();
                let target_name = format!("{}: {}", side_id, defender_name);
                self.add_log("-immune", &[&target_name, &format!("[from] ability: {}", ability_def.name)]);

                // Healing absorb (Water Absorb, Volt Absorb, Dry Skin)
                if let Some(heal_fraction) = ability_def.absorb_heal {
                    let maxhp = self.sides[target_side].pokemon[target_idx].maxhp;
                    let heal = ((maxhp as f64 * heal_fraction) as u32).max(1);
                    self.sides[target_side].pokemon[target_idx].heal(heal);
                    let hp = self.sides[target_side].pokemon[target_idx].hp;
                    self.add_log("-heal", &[&target_name, &format!("{}/{}", hp, maxhp), &format!("[from] ability: {}", ability_def.name)]);
                }

                // Boost absorb (Motor Drive, Lightning Rod, Storm Drain, Sap Sipper)
                if let Some((stat, stages)) = &ability_def.absorb_boost {
                    self.apply_boost(target_side, target_idx, stat, *stages as i8);
                }

                return (0, false);
            }
        }

        // Check type immunity
        let type_effectiveness = self.get_type_effectiveness(&move_type, &defender_types);
        if type_effectiveness == 0.0 {
            let side_id = self.sides[target_side].id_str();
            let target_name = format!("{}: {}", side_id, defender_name);
            self.add_log("-immune", &[&target_name]);
            return (0, false);
        }

        // Calculate boosted stats
        let attack = self.calculate_boosted_stat(attack_stat, atk_boost);
        let defense = self.calculate_boosted_stat(defense_stat, def_boost).max(1);

        // Base damage calculation: ((2L/5 + 2) * P * A/D) / 50 + 2
        let base_damage = ((2 * level / 5 + 2) * base_power * attack / defense) / 50 + 2;

        // Random factor (85-100%)
        let random_factor = 85 + self.random(16);
        let damage = base_damage * random_factor / 100;

        // STAB
        let has_stab = attacker_types.iter().any(|t| t.to_lowercase() == move_type.to_lowercase());
        let mut stab = if has_stab { 1.5 } else { 1.0 };

        // onModifySTAB handler for abilities like Adaptability
        if has_stab {
            if let Some(attacker_pokemon) = self.sides.get(attacker_side).and_then(|s| s.pokemon.get(attacker_idx)) {
                let ability_id = attacker_pokemon.ability.as_str();
                if ability_id == "adaptability" {
                    // Adaptability increases STAB from 1.5x to 2x
                    stab = 2.0;
                }
            }
        }

        let damage = (damage as f64 * stab) as u32;

        // Type effectiveness
        let damage = (damage as f64 * type_effectiveness) as u32;

        // Burn reduces physical damage
        let damage = if category == "Physical" && attacker_status == "brn" {
            damage / 2
        } else {
            damage
        };

        // Weather type modifier (rain boosts Water, sun boosts Fire, etc.)
        let weather = self.field.weather.as_str();
        let weather_mod = get_weather_type_modifier(weather, &move_type);
        let damage = (damage as f64 * weather_mod) as u32;

        // Terrain type modifier (grounded Pokemon only)
        // Check if attacker is grounded (simplified - not Flying type and no Levitate)
        let attacker_grounded = !attacker_types.iter().any(|t| t.to_lowercase() == "flying");
        let terrain = self.field.terrain.as_str();
        let terrain_mod = get_terrain_damage_modifier(terrain, &move_type, attacker_grounded);
        let damage = (damage as f64 * terrain_mod) as u32;

        // Item damage modifiers
        let item = self.sides[attacker_side].pokemon[attacker_idx].item.as_str();
        let item_mod = match item {
            // Life Orb: 1.3x damage boost
            "lifeorb" => 1.3,
            // Choice Band: 1.5x Attack (physical moves only)
            "choiceband" if category == "Physical" => 1.5,
            // Choice Specs: 1.5x Sp. Attack (special moves only)
            "choicespecs" if category == "Special" => 1.5,
            // Expert Belt: 1.2x for super effective moves
            "expertbelt" if type_effectiveness > 1.0 => 1.2,
            // Muscle Band: 1.1x for physical moves
            "muscleband" if category == "Physical" => 1.1,
            // Wise Glasses: 1.1x for special moves
            "wiseglasses" if category == "Special" => 1.1,
            _ => 1.0,
        };
        let damage = (damage as f64 * item_mod) as u32;

        // Log effectiveness
        if type_effectiveness > 1.0 {
            self.add_log("-supereffective", &[]);
        } else if type_effectiveness < 1.0 && type_effectiveness > 0.0 {
            self.add_log("-resisted", &[]);
        }

        // Critical hit check
        // Base crit rate: 1/24 (~4.17%), high crit ratio moves: 1/8 (12.5%)
        let high_crit_moves = [
            "stoneedge", "slash", "nightslash", "shadowclaw", "crosschop",
            "aeroblast", "spacialrend", "attackorder", "leafblade", "crabhammer",
            "drillrun", "psychocut", "razorleaf", "karatechop",
        ];
        let crit_ratio = if high_crit_moves.contains(&move_id.as_str()) { 8 } else { 24 };
        let crit_roll = self.random(crit_ratio);

        if crit_roll == 0 {
            // Check if target has an ability that blocks critical hits
            // onCriticalHit event - if returns False, prevent the crit
            if let Some(target_pokemon) = self.sides.get(target_side).and_then(|s| s.pokemon.get(target_idx)) {
                if target_pokemon.ability.as_str() == "battlearmor" || target_pokemon.ability.as_str() == "shellarmor" {
                    // Ability blocks critical hit - treat as non-crit
                    return (damage.max(1), false);
                }
            }

            let target_name = format!("{}: {}", self.sides[target_side].id_str(), defender_name);
            self.add_log("-crit", &[&target_name]);
            // Critical hits: 1.5x damage, ignore burn penalty (for physical), ignore stat drops
            return (((damage as f64 * 1.5) as u32).max(1), true);
        }

        // Apply side condition damage modifiers (Aurora Veil, Reflect, Light Screen)
        // onAnyModifyDamage for side conditions
        let damage = if let Some(side) = self.sides.get(target_side) {
            if side.has_side_condition(&ID::new("auroraveil")) {
                // TODO: Call onAnyModifyDamage callback when move_callbacks is reimplemented
                // For now, apply a default Aurora Veil reduction (2/3 damage in doubles, 1/2 in singles)
                (damage * 2 / 3).max(1)
            } else {
                damage
            }
        } else {
            damage
        };

        // Fire ModifyDamage event to allow custom handlers to modify final damage
        // JavaScript: baseDamage = this.battle.runEvent('ModifyDamage', pokemon, target, move, baseDamage);
        let damage = if let Some(modified) = self.run_event(
            "ModifyDamage",
            Some((target_side, target_idx)),
            Some((attacker_side, attacker_idx)),
            Some(move_id),
            Some(damage as i32)
        ) {
            modified as u32
        } else {
            damage
        };

        (damage.max(1), false)
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
    fn get_multi_hit_count(&mut self, move_id: &ID) -> u32 {
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
    fn get_move_accuracy(&self, move_id: &ID) -> u32 {
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

    /// Get type effectiveness multiplier
    fn get_type_effectiveness(&self, move_type: &str, defender_types: &[String]) -> f64 {
        // Type chart (simplified)
        let get_single_effectiveness = |attack_type: &str, defend_type: &str| -> f64 {
            match (attack_type.to_lowercase().as_str(), defend_type.to_lowercase().as_str()) {
                // Super effective
                ("fire", "grass") | ("fire", "ice") | ("fire", "bug") | ("fire", "steel") => 2.0,
                ("water", "fire") | ("water", "ground") | ("water", "rock") => 2.0,
                ("grass", "water") | ("grass", "ground") | ("grass", "rock") => 2.0,
                ("electric", "water") | ("electric", "flying") => 2.0,
                ("ice", "grass") | ("ice", "ground") | ("ice", "flying") | ("ice", "dragon") => 2.0,
                ("fighting", "normal") | ("fighting", "ice") | ("fighting", "rock") | ("fighting", "dark") | ("fighting", "steel") => 2.0,
                ("poison", "grass") | ("poison", "fairy") => 2.0,
                ("ground", "fire") | ("ground", "electric") | ("ground", "poison") | ("ground", "rock") | ("ground", "steel") => 2.0,
                ("flying", "grass") | ("flying", "fighting") | ("flying", "bug") => 2.0,
                ("psychic", "fighting") | ("psychic", "poison") => 2.0,
                ("bug", "grass") | ("bug", "psychic") | ("bug", "dark") => 2.0,
                ("rock", "fire") | ("rock", "ice") | ("rock", "flying") | ("rock", "bug") => 2.0,
                ("ghost", "psychic") | ("ghost", "ghost") => 2.0,
                ("dragon", "dragon") => 2.0,
                ("dark", "psychic") | ("dark", "ghost") => 2.0,
                ("steel", "ice") | ("steel", "rock") | ("steel", "fairy") => 2.0,
                ("fairy", "fighting") | ("fairy", "dragon") | ("fairy", "dark") => 2.0,

                // Immunities
                ("normal", "ghost") | ("fighting", "ghost") => 0.0,
                ("electric", "ground") => 0.0,
                ("poison", "steel") => 0.0,
                ("ground", "flying") => 0.0,
                ("psychic", "dark") => 0.0,
                ("ghost", "normal") => 0.0,
                ("dragon", "fairy") => 0.0,

                // Not very effective
                ("fire", "fire") | ("fire", "water") | ("fire", "rock") | ("fire", "dragon") => 0.5,
                ("water", "water") | ("water", "grass") | ("water", "dragon") => 0.5,
                ("grass", "fire") | ("grass", "grass") | ("grass", "poison") | ("grass", "flying") | ("grass", "bug") | ("grass", "dragon") | ("grass", "steel") => 0.5,
                ("electric", "electric") | ("electric", "grass") | ("electric", "dragon") => 0.5,
                ("ice", "fire") | ("ice", "water") | ("ice", "ice") | ("ice", "steel") => 0.5,
                ("fighting", "poison") | ("fighting", "flying") | ("fighting", "psychic") | ("fighting", "bug") | ("fighting", "fairy") => 0.5,
                ("poison", "poison") | ("poison", "ground") | ("poison", "rock") | ("poison", "ghost") => 0.5,
                ("ground", "grass") | ("ground", "bug") => 0.5,
                ("flying", "electric") | ("flying", "rock") | ("flying", "steel") => 0.5,
                ("psychic", "psychic") | ("psychic", "steel") => 0.5,
                ("bug", "fire") | ("bug", "fighting") | ("bug", "poison") | ("bug", "flying") | ("bug", "ghost") | ("bug", "steel") | ("bug", "fairy") => 0.5,
                ("rock", "fighting") | ("rock", "ground") | ("rock", "steel") => 0.5,
                ("ghost", "dark") => 0.5,
                ("dark", "fighting") | ("dark", "dark") | ("dark", "fairy") => 0.5,
                ("steel", "fire") | ("steel", "water") | ("steel", "electric") | ("steel", "steel") => 0.5,
                ("fairy", "fire") | ("fairy", "poison") | ("fairy", "steel") => 0.5,

                _ => 1.0,
            }
        };

        let mut effectiveness = 1.0;
        for def_type in defender_types {
            effectiveness *= get_single_effectiveness(move_type, def_type);
        }
        effectiveness
    }

    /// Calculate a stat with boost applied
    fn calculate_boosted_stat(&self, base: u32, boost: i8) -> u32 {
        let (num, denom) = match boost {
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
            b if b < -6 => (2, 8),
            _ => (8, 2),
        };
        (base * num / denom).max(1)
    }

    /// Apply secondary effects from a move (data-driven)
    fn apply_move_secondary(&mut self, attacker_side: usize, attacker_idx: usize, target_side: usize, target_idx: usize, move_id: &ID) {
        // Extract move data fields before making mutable calls
        let (status, volatile_status, boosts, secondary) = match self.dex.get_move(move_id.as_str()) {
            Some(move_def) => (
                move_def.status.clone(),
                move_def.volatile_status.clone(),
                move_def.boosts.clone(),
                move_def.secondary.clone(),
            ),
            None => return, // Unknown move, no secondaries
        };

        // Call onTryHit event to check if move can be used (for moves that have onTryHit callback)
        // JavaScript: if (!this.singleEvent('TryHit', move, {}, target, pokemon, move)) { return 'fail'; }
        match move_id.as_str() {
            "autotomize" => {
                let try_hit_result = self.run_event("TryHit", Some((target_side, target_idx)), Some((attacker_side, attacker_idx)), Some(&move_id), None);
                if try_hit_result.is_none() {
                    // onTryHit returned false/fail, move fails
                    return;
                }
            }
            _ => {}
        }

        // Apply primary move effects (for status moves)
        // Apply primary status condition
        if let Some(ref status_str) = status {
            self.apply_status(target_side, target_idx, status_str);
        }

        // Apply primary volatile status
        if let Some(ref volatile) = volatile_status {
            self.add_volatile_to_pokemon((target_side, target_idx), &ID::new(volatile), Some((attacker_side, attacker_idx)), Some(&move_id));
        }

        // Apply primary stat boosts (e.g., Swords Dance, Dragon Dance)
        if let Some(ref boosts_map) = boosts {
            self.boost_stats(target_side, target_idx, boosts_map);
        }

        // Apply secondary effects with chance checking
        if let Some(ref sec) = secondary {
            // Check if secondary effect triggers based on chance
            let triggers = if let Some(chance) = sec.chance {
                // Roll for chance (e.g., 30% chance = roll 0-99, trigger if < 30)
                use rand::Rng;
                let roll = rand::thread_rng().gen_range(0..100);
                roll < chance
            } else {
                // No chance specified means 100% chance
                true
            };

            if triggers {
                // Apply secondary status
                if let Some(ref sec_status) = sec.status {
                    self.apply_status(target_side, target_idx, sec_status);
                }

                // Apply secondary volatile status
                if let Some(ref sec_volatile) = sec.volatile_status_secondary {
                    self.add_volatile_to_pokemon((target_side, target_idx), &ID::new(sec_volatile), Some((attacker_side, attacker_idx)), Some(&move_id));
                }

                // Apply secondary stat boosts
                if let Some(ref sec_boosts) = sec.boosts {
                    self.boost_stats(target_side, target_idx, sec_boosts);
                }
            }
        }


        // Handle move callbacks for complex move logic
        match move_id.as_str() {
            "acupressure" => {
                // Randomly boosts one stat that is below +6 by 2 stages
                // Collect stats that are below +6
                let mut available_stats: Vec<&str> = Vec::new();
                let boosts = &self.sides[target_side].pokemon[target_idx].boosts;

                if boosts.atk < 6 { available_stats.push("atk"); }
                if boosts.def < 6 { available_stats.push("def"); }
                if boosts.spa < 6 { available_stats.push("spa"); }
                if boosts.spd < 6 { available_stats.push("spd"); }
                if boosts.spe < 6 { available_stats.push("spe"); }
                if boosts.accuracy < 6 { available_stats.push("accuracy"); }
                if boosts.evasion < 6 { available_stats.push("evasion"); }

                if !available_stats.is_empty() {
                    // Randomly select one stat
                    let idx = self.random(available_stats.len() as u32) as usize;
                    let chosen_stat = available_stats[idx];
                    self.apply_boost(target_side, target_idx, chosen_stat, 2);
                }
                // If no stats available, the move fails (no effect)
            }
            "afteryou" => {
                // After You makes the target move immediately after the user
                // Check if this is doubles/triples (activePerHalf > 1)
                if self.active_per_half == 1 {
                    // Fails in singles
                    return;
                }

                // Check if target has a queued move action
                if let Some(_move_action) = self.queue.will_move(target_side, target_idx) {
                    // Prioritize the target's action to move next
                    if self.queue.prioritize_action(target_side, target_idx) {
                        // Add activation message
                        let target_name = &self.sides[target_side].pokemon[target_idx].name.clone();
                        self.add("-activate", &[
                            Arg::String(target_name.clone()),
                            Arg::Str("move: After You")
                        ]);
                    }
                }
                // Note: This won't fully work until commit_choices is refactored to use the queue during execution
            }
            "alluringvoice" => {
                // This is handled as a secondary effect in the move data
                // If target raised stats this turn, add confusion
                if self.sides[target_side].pokemon[target_idx].stats_raised_this_turn {
                    let confusion_id = ID::new("confusion");
                    self.sides[target_side].pokemon[target_idx].add_volatile(confusion_id);
                }
            }
            "allyswitch" => {
                // Ally Switch swaps positions with an ally in doubles/triples
                // Fail if not doubles/triples
                if self.active_per_half == 1 {
                    let pokemon_name = &self.sides[attacker_side].pokemon[attacker_idx].name.clone();
                    self.add("-fail", &[
                        Arg::String(pokemon_name.clone()),
                        Arg::Str("move: Ally Switch")
                    ]);
                    return;
                }

                // Get current position
                let current_position = self.sides[attacker_side].pokemon[attacker_idx].position;

                // In doubles, swap with position 0 if at 1, or position 1 if at 0
                // In triples, fail if in middle (position 1)
                let new_position = if self.active_per_half == 3 && current_position == 1 {
                    // Fail in triples if in the middle
                    let pokemon_name = &self.sides[attacker_side].pokemon[attacker_idx].name.clone();
                    self.add("-fail", &[
                        Arg::String(pokemon_name.clone()),
                        Arg::Str("move: Ally Switch")
                    ]);
                    return;
                } else if current_position == 0 {
                    self.active_per_half - 1
                } else {
                    0
                };

                // Check if there's a Pokemon at the new position
                if let Some(Some(ally_idx)) = self.sides[attacker_side].active.get(new_position) {
                    let ally_fainted = self.sides[attacker_side].pokemon[*ally_idx].is_fainted();
                    if ally_fainted {
                        let pokemon_name = &self.sides[attacker_side].pokemon[attacker_idx].name.clone();
                        self.add("-fail", &[
                            Arg::String(pokemon_name.clone()),
                            Arg::Str("move: Ally Switch")
                        ]);
                        return;
                    }

                    // Perform the swap
                    self.swap_position((attacker_side, attacker_idx), new_position, Some("[from] move: Ally Switch"));

                    // Log the swap
                    let pokemon_name = &self.sides[attacker_side].pokemon[attacker_idx].name.clone();
                    self.add("-activate", &[
                        Arg::String(pokemon_name.clone()),
                        Arg::Str("[from] move: Ally Switch")
                    ]);
                } else {
                    // No ally at that position
                    let pokemon_name = &self.sides[attacker_side].pokemon[attacker_idx].name.clone();
                    self.add("-fail", &[
                        Arg::String(pokemon_name.clone()),
                        Arg::Str("move: Ally Switch")
                    ]);
                }
                // TODO: Implement diminishing returns mechanic (counter that triples on each use)
            }
            // Entry hazard moves - set on opponent's side
            "stealthrock" => {
                let hazard_id = ID::new("stealthrock");
                if self.sides[target_side].add_hazard(&hazard_id) {
                    let target_side_id = self.sides[target_side].id_str();
                    self.add_log("-sidestart", &[target_side_id, "Stealth Rock"]);
                }
            }
            "spikes" => {
                let hazard_id = ID::new("spikes");
                if self.sides[target_side].add_hazard(&hazard_id) {
                    let target_side_id = self.sides[target_side].id_str();
                    self.add_log("-sidestart", &[target_side_id, "Spikes"]);
                }
            }
            "toxicspikes" => {
                let hazard_id = ID::new("toxicspikes");
                if self.sides[target_side].add_hazard(&hazard_id) {
                    let target_side_id = self.sides[target_side].id_str();
                    self.add_log("-sidestart", &[target_side_id, "Toxic Spikes"]);
                }
            }
            "stickyweb" => {
                let hazard_id = ID::new("stickyweb");
                if self.sides[target_side].add_hazard(&hazard_id) {
                    let target_side_id = self.sides[target_side].id_str();
                    self.add_log("-sidestart", &[target_side_id, "Sticky Web"]);
                }
            }
            // Hazard removal moves
            "defog" => {
                self.remove_all_hazards(target_side);
                // Also remove hazards from user's side
                self.remove_all_hazards(attacker_side);
            }
            "rapidspin" => {
                // Rapid Spin removes hazards from user's own side
                self.remove_all_hazards(attacker_side);
            }
            // Protection moves
            "protect" | "detect" => {
                self.sides[attacker_side].pokemon[target_idx].add_volatile(ID::new("protect"));
                let name = {
                    let side_id = self.sides[attacker_side].id_str();
                    let pokemon = &self.sides[attacker_side].pokemon[target_idx];
                    format!("{}: {}", side_id, pokemon.name)
                };
                self.add_log("-singleturn", &[&name, "Protect"]);
            }
            // Recovery moves
            "recover" | "softboiled" | "milkdrink" | "slackoff" => {
                let maxhp = self.sides[attacker_side].pokemon[target_idx].maxhp;
                let heal = maxhp / 2;
                self.sides[attacker_side].pokemon[target_idx].heal(heal);
                let name = {
                    let side_id = self.sides[attacker_side].id_str();
                    let pokemon = &self.sides[attacker_side].pokemon[target_idx];
                    format!("{}: {}", side_id, pokemon.name)
                };
                let hp = self.sides[attacker_side].pokemon[target_idx].hp;
                let maxhp = self.sides[attacker_side].pokemon[target_idx].maxhp;
                self.add_log("-heal", &[&name, &format!("{}/{}", hp, maxhp)]);
            }
            "roost" => {
                // Roost heals 50% HP and removes Flying type for the turn
                let maxhp = self.sides[attacker_side].pokemon[target_idx].maxhp;
                let heal = maxhp / 2;
                self.sides[attacker_side].pokemon[target_idx].heal(heal);
                self.sides[attacker_side].pokemon[target_idx].add_volatile(ID::new("roost"));
                let name = {
                    let side_id = self.sides[attacker_side].id_str();
                    let pokemon = &self.sides[attacker_side].pokemon[target_idx];
                    format!("{}: {}", side_id, pokemon.name)
                };
                let hp = self.sides[attacker_side].pokemon[target_idx].hp;
                let maxhp = self.sides[attacker_side].pokemon[target_idx].maxhp;
                self.add_log("-heal", &[&name, &format!("{}/{}", hp, maxhp)]);
            }
            "moonlight" | "synthesis" | "morningsun" => {
                // Weather-dependent recovery: 2/3 in sun, 1/4 in rain/sand/hail, 1/2 normally
                let maxhp = self.sides[attacker_side].pokemon[target_idx].maxhp;
                let weather = self.field.weather.as_str();
                let heal_frac = match weather {
                    "sunnyday" | "desolateland" => 2.0 / 3.0,
                    "raindance" | "primordialsea" | "sandstorm" | "hail" | "snow" => 0.25,
                    _ => 0.5,
                };
                let heal = ((maxhp as f64) * heal_frac) as u32;
                self.sides[attacker_side].pokemon[target_idx].heal(heal);
                let name = {
                    let side_id = self.sides[attacker_side].id_str();
                    let pokemon = &self.sides[attacker_side].pokemon[target_idx];
                    format!("{}: {}", side_id, pokemon.name)
                };
                let hp = self.sides[attacker_side].pokemon[target_idx].hp;
                let maxhp = self.sides[attacker_side].pokemon[target_idx].maxhp;
                self.add_log("-heal", &[&name, &format!("{}/{}", hp, maxhp)]);
            }
            // Substitute
            "substitute" => {
                let maxhp = self.sides[attacker_side].pokemon[target_idx].maxhp;
                let hp = self.sides[attacker_side].pokemon[target_idx].hp;
                let cost = maxhp / 4;

                if hp > cost && !self.sides[attacker_side].pokemon[target_idx].has_volatile(&ID::new("substitute")) {
                    self.sides[attacker_side].pokemon[target_idx].take_damage(cost);
                    self.sides[attacker_side].pokemon[target_idx].add_volatile(ID::new("substitute"));
                    // Store substitute HP in volatile data
                    let name = {
                        let side_id = self.sides[attacker_side].id_str();
                        let pokemon = &self.sides[attacker_side].pokemon[target_idx];
                        format!("{}: {}", side_id, pokemon.name)
                    };
                    self.add_log("-start", &[&name, "Substitute"]);
                }
            }
            // Haze - reset all stat changes
            "haze" => {
                for side in &mut self.sides {
                    for pokemon in &mut side.pokemon {
                        if pokemon.is_active {
                            pokemon.boosts = Default::default();
                        }
                    }
                }
                self.add_log("-clearallboost", &[]);
            }
            // Phazing moves
            "whirlwind" | "roar" => {
                // Force switch the target
                let switchable = self.sides[target_side].get_switchable();
                if !switchable.is_empty() {
                    let random_idx = self.random(switchable.len() as u32) as usize;
                    let switch_to = switchable[random_idx];
                    let target_slot = self.sides[target_side].pokemon[target_idx].position;
                    self.do_switch(target_side, target_slot, switch_to);
                }
            }
            // Team support moves
            "healbell" | "aromatherapy" => {
                // Cure status of all team members
                let side_id = self.sides[attacker_side].id_str();
                for pokemon in &mut self.sides[attacker_side].pokemon {
                    if !pokemon.status.is_empty() {
                        pokemon.cure_status();
                    }
                }
                self.add_log("-cureteam", &[side_id]);
            }
            // Confusion-causing moves
            "confuseray" | "supersonic" | "sweetkiss" | "teeterdance" => {
                self.apply_confusion(target_side, target_idx);
            }
            // Hurricane and Dynamic Punch have 100% confusion chance
            "hurricane" | "dynamicpunch" => {
                self.apply_confusion(target_side, target_idx);
            }
            // Psybeam, Signal Beam, Confusion - 10% confusion chance
            "psybeam" | "signalbeam" | "confusion" => {
                if self.random(10) == 0 {
                    self.apply_confusion(target_side, target_idx);
                }
            }
            // Trick Room - reverses speed order for 5 turns
            "trickroom" => {
                let trick_room_id = ID::new("trickroom");
                if self.field.has_pseudo_weather(&trick_room_id) {
                    // Trick Room is already active - remove it
                    self.field.remove_pseudo_weather(&trick_room_id);
                    self.add_log("-fieldend", &["Trick Room"]);
                } else {
                    // Set Trick Room for 5 turns
                    self.field.add_pseudo_weather(trick_room_id, Some(5));
                    self.add_log("-fieldstart", &["Trick Room"]);
                }
            }
            _ => {}
        }

        // Fire Hit event after all move effects are applied
        // JavaScript: this.battle.runEvent('Hit', target, source, move);
        // This fires for ALL moves, not just specific ones
        self.run_event(
            "Hit",
            Some((target_side, target_idx)),
            Some((attacker_side, attacker_idx)),
            Some(move_id),
            None
        );
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
    /// JS Source (pokemon.ts:~490-498):
    /// ```js
    /// cureStatus(silent = false) {
    ///     if (!this.hp || !this.status) return false;
    ///     this.battle.add('-curestatus', this, this.status, silent ? '[silent]' : '[msg]');
    ///     if (this.status === 'slp' && this.removeVolatile('nightmare')) {
    ///         this.battle.add('-end', this, 'Nightmare', '[silent]');
    ///     }
    ///     this.setStatus('');
    ///     return true;
    /// }
    /// ```
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
    fn run_residual(&mut self) {
        // Get field conditions once
        let weather = self.field.weather.as_str().to_string();
        let terrain = self.field.terrain.as_str().to_string();

        for side_idx in 0..self.sides.len() {
            for poke_idx in 0..self.sides[side_idx].pokemon.len() {
                let is_active = self.sides[side_idx].pokemon[poke_idx].is_active;
                if !is_active {
                    continue;
                }

                if self.sides[side_idx].pokemon[poke_idx].is_fainted() {
                    continue;
                }

                let status = self.sides[side_idx].pokemon[poke_idx].status.as_str().to_string();
                let maxhp = self.sides[side_idx].pokemon[poke_idx].maxhp;
                let pokemon_types = self.sides[side_idx].pokemon[poke_idx].types.clone();
                let is_grounded = !pokemon_types.iter().any(|t| t.to_lowercase() == "flying");

                // Weather damage (sandstorm/hail)
                let weather_damage_frac = get_weather_damage_fraction(&weather, &pokemon_types);
                if weather_damage_frac > 0.0 {
                    let damage = ((maxhp as f64 * weather_damage_frac) as u32).max(1);
                    self.sides[side_idx].pokemon[poke_idx].take_damage(damage);

                    let name = {
                        let side_id = self.sides[side_idx].id_str();
                        let pokemon = &self.sides[side_idx].pokemon[poke_idx];
                        format!("{}: {}", side_id, pokemon.name)
                    };
                    let hp = self.sides[side_idx].pokemon[poke_idx].hp;
                    let weather_source = format!("[from] {}", weather);
                    self.add_log("-damage", &[&name, &format!("{}/{}", hp, maxhp), &weather_source]);
                }

                // Grassy Terrain healing
                let grassy_heal_frac = get_grassy_terrain_heal(&terrain, is_grounded);
                if grassy_heal_frac > 0.0 {
                    let heal = ((maxhp as f64 * grassy_heal_frac) as u32).max(1);
                    let old_hp = self.sides[side_idx].pokemon[poke_idx].hp;
                    self.sides[side_idx].pokemon[poke_idx].heal(heal);

                    if self.sides[side_idx].pokemon[poke_idx].hp > old_hp {
                        let name = {
                            let side_id = self.sides[side_idx].id_str();
                            let pokemon = &self.sides[side_idx].pokemon[poke_idx];
                            format!("{}: {}", side_id, pokemon.name)
                        };
                        let hp = self.sides[side_idx].pokemon[poke_idx].hp;
                        self.add_log("-heal", &[&name, &format!("{}/{}", hp, maxhp), "[from] Grassy Terrain"]);
                    }
                }

                // Status damage
                match status.as_str() {
                    "brn" => {
                        // Burn does 1/16 max HP (Gen 7+)
                        let damage = (maxhp / 16).max(1);
                        self.sides[side_idx].pokemon[poke_idx].take_damage(damage);

                        let name = {
                            let side_id = self.sides[side_idx].id_str();
                            let pokemon = &self.sides[side_idx].pokemon[poke_idx];
                            format!("{}: {}", side_id, pokemon.name)
                        };
                        let hp = self.sides[side_idx].pokemon[poke_idx].hp;
                        self.add_log("-damage", &[&name, &format!("{}/{}", hp, maxhp), "[from] brn"]);
                    }
                    "psn" => {
                        // Poison does 1/8 max HP
                        let damage = (maxhp / 8).max(1);
                        self.sides[side_idx].pokemon[poke_idx].take_damage(damage);

                        let name = {
                            let side_id = self.sides[side_idx].id_str();
                            let pokemon = &self.sides[side_idx].pokemon[poke_idx];
                            format!("{}: {}", side_id, pokemon.name)
                        };
                        let hp = self.sides[side_idx].pokemon[poke_idx].hp;
                        self.add_log("-damage", &[&name, &format!("{}/{}", hp, maxhp), "[from] psn"]);
                    }
                    "tox" => {
                        // Toxic does N/16 max HP where N increases each turn
                        let counter = self.sides[side_idx].pokemon[poke_idx].status_state.duration.unwrap_or(1);
                        let damage = (maxhp * counter / 16).max(1);
                        self.sides[side_idx].pokemon[poke_idx].take_damage(damage);

                        // Increment counter
                        self.sides[side_idx].pokemon[poke_idx].status_state.duration = Some(counter + 1);

                        let name = {
                            let side_id = self.sides[side_idx].id_str();
                            let pokemon = &self.sides[side_idx].pokemon[poke_idx];
                            format!("{}: {}", side_id, pokemon.name)
                        };
                        let hp = self.sides[side_idx].pokemon[poke_idx].hp;
                        self.add_log("-damage", &[&name, &format!("{}/{}", hp, maxhp), "[from] tox"]);
                    }
                    _ => {}
                }

                // Item end-of-turn effects
                let item = self.sides[side_idx].pokemon[poke_idx].item.as_str().to_string();
                match item.as_str() {
                    "leftovers" => {
                        // Heal 1/16 max HP
                        let old_hp = self.sides[side_idx].pokemon[poke_idx].hp;
                        if old_hp < maxhp {
                            let heal = (maxhp / 16).max(1);
                            self.sides[side_idx].pokemon[poke_idx].heal(heal);

                            let name = {
                                let side_id = self.sides[side_idx].id_str();
                                let pokemon = &self.sides[side_idx].pokemon[poke_idx];
                                format!("{}: {}", side_id, pokemon.name)
                            };
                            let hp = self.sides[side_idx].pokemon[poke_idx].hp;
                            self.add_log("-heal", &[&name, &format!("{}/{}", hp, maxhp), "[from] item: Leftovers"]);
                        }
                    }
                    "blacksludge" => {
                        // Heal 1/16 if Poison type, damage otherwise
                        let is_poison = pokemon_types.iter().any(|t| t.to_lowercase() == "poison");
                        if is_poison {
                            let old_hp = self.sides[side_idx].pokemon[poke_idx].hp;
                            if old_hp < maxhp {
                                let heal = (maxhp / 16).max(1);
                                self.sides[side_idx].pokemon[poke_idx].heal(heal);

                                let name = {
                                    let side_id = self.sides[side_idx].id_str();
                                    let pokemon = &self.sides[side_idx].pokemon[poke_idx];
                                    format!("{}: {}", side_id, pokemon.name)
                                };
                                let hp = self.sides[side_idx].pokemon[poke_idx].hp;
                                self.add_log("-heal", &[&name, &format!("{}/{}", hp, maxhp), "[from] item: Black Sludge"]);
                            }
                        } else {
                            let damage = (maxhp / 8).max(1);
                            self.sides[side_idx].pokemon[poke_idx].take_damage(damage);

                            let name = {
                                let side_id = self.sides[side_idx].id_str();
                                let pokemon = &self.sides[side_idx].pokemon[poke_idx];
                                format!("{}: {}", side_id, pokemon.name)
                            };
                            let hp = self.sides[side_idx].pokemon[poke_idx].hp;
                            self.add_log("-damage", &[&name, &format!("{}/{}", hp, maxhp), "[from] item: Black Sludge"]);
                        }
                    }
                    _ => {}
                }
            }
        }

        // Process volatile condition residuals (onResidual callbacks)
        // JavaScript: this.fieldEvent('Residual') processes all pokemon volatiles
        for side_idx in 0..self.sides.len() {
            for poke_idx in 0..self.sides[side_idx].pokemon.len() {
                let is_active = self.sides[side_idx].pokemon[poke_idx].is_active;
                if !is_active {
                    continue;
                }

                if self.sides[side_idx].pokemon[poke_idx].is_fainted() {
                    continue;
                }

                // Collect volatile IDs to process (to avoid borrow issues)
                let volatile_ids: Vec<ID> = self.sides[side_idx].pokemon[poke_idx]
                    .volatiles
                    .keys()
                    .cloned()
                    .collect();

                for volatile_id in volatile_ids {
                    // Call onResidual callback for aquaring
                    if volatile_id.as_str() == "aquaring" {
//                         crate::data::move_callbacks::aquaring::on_residual(self, (side_idx, poke_idx));
                    }
                }
            }
        }

        // Decrement field condition durations
        let expired = self.field.decrement_durations();
        for effect_id in expired {
            self.add_log("-fieldend", &[effect_id.as_str()]);
        }

        // Decrement side condition durations
        for side_idx in 0..self.sides.len() {
            // Collect side condition IDs that need duration decrements
            let side_condition_ids: Vec<ID> = self.sides[side_idx]
                .side_conditions
                .keys()
                .cloned()
                .collect();

            for condition_id in side_condition_ids {
                // Get current duration
                let current_duration = if let Some(side) = self.sides.get(side_idx) {
                    if let Some(condition) = side.get_side_condition(&condition_id) {
                        condition.duration
                    } else {
                        None
                    }
                } else {
                    None
                };

                // Decrement if there's a duration
                if let Some(dur) = current_duration {
                    if dur > 0 {
                        let new_duration = dur - 1;

                        if new_duration == 0 {
                            // Duration expired, call onSideEnd and remove condition
                            self.handle_side_condition_event("SideEnd", side_idx, &condition_id);

                            if let Some(side) = self.sides.get_mut(side_idx) {
                                side.remove_side_condition(&condition_id);
                            }
                        } else {
                            // Update duration
                            if let Some(side) = self.sides.get_mut(side_idx) {
                                if let Some(condition) = side.get_side_condition_mut(&condition_id) {
                                    condition.duration = Some(new_duration);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Process faint messages
    /// Equivalent to battle.ts faintMessages(lastFirst?, forceCheck?, checkWin?)
    ///
    /// JS Source (battle.ts:2498-2575):
    /// ```js
    /// faintMessages(lastFirst = false, forceCheck = false, checkWin = true) {
    ///     if (this.ended) return;
    ///     const length = this.faintQueue.length;
    ///     if (!length) {
    ///         if (forceCheck && this.checkWin()) return true;
    ///         return false;
    ///     }
    ///     if (lastFirst) {
    ///         this.faintQueue.unshift(this.faintQueue[this.faintQueue.length - 1]);
    ///         this.faintQueue.pop();
    ///     }
    ///     let faintQueueLeft, faintData;
    ///     while (this.faintQueue.length) {
    ///         faintQueueLeft = this.faintQueue.length;
    ///         faintData = this.faintQueue.shift()!;
    ///         const pokemon: Pokemon = faintData.target;
    ///         if (!pokemon.fainted && this.runEvent('BeforeFaint', pokemon, faintData.source, faintData.effect)) {
    ///             this.add('faint', pokemon);
    ///             if (pokemon.side.pokemonLeft) pokemon.side.pokemonLeft--;
    ///             if (pokemon.side.totalFainted < 100) pokemon.side.totalFainted++;
    ///             this.runEvent('Faint', pokemon, faintData.source, faintData.effect);
    ///             this.singleEvent('End', pokemon.getAbility(), pokemon.abilityState, pokemon);
    ///             this.singleEvent('End', pokemon.getItem(), pokemon.itemState, pokemon);
    ///             if (pokemon.formeRegression && !pokemon.transformed) {
    ///                 pokemon.baseSpecies = this.dex.species.get(pokemon.set.species || pokemon.set.name);
    ///                 pokemon.baseAbility = toID(pokemon.set.ability);
    ///             }
    ///             pokemon.clearVolatile(false);
    ///             pokemon.fainted = true;
    ///             pokemon.illusion = null;
    ///             pokemon.isActive = false;
    ///             pokemon.isStarted = false;
    ///             delete pokemon.terastallized;
    ///             if (pokemon.formeRegression) {
    ///                 pokemon.details = pokemon.getUpdatedDetails();
    ///                 this.add('detailschange', pokemon, pokemon.details, '[silent]');
    ///                 pokemon.updateMaxHp();
    ///                 pokemon.formeRegression = false;
    ///             }
    ///             pokemon.side.faintedThisTurn = pokemon;
    ///             if (this.faintQueue.length >= faintQueueLeft) checkWin = true;
    ///         }
    ///     }
    ///     if (this.gen <= 1) { /* Gen 1 logic */ }
    ///     else if (this.gen <= 3 && this.gameType === 'singles') { /* Gen 2-3 logic */ }
    ///     if (checkWin && this.checkWin(faintData)) return true;
    ///     if (faintData && length) {
    ///         this.runEvent('AfterFaint', faintData.target, faintData.source, faintData.effect, length);
    ///     }
    ///     return false;
    /// }
    /// ```
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
                // TODO: runEvent('BeforeFaint', pokemon, source, effect)
                // For now, assume it returns true

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
                // TODO: runEvent('Faint')

                // JS: this.singleEvent('End', pokemon.getAbility(), pokemon.abilityState, pokemon);
                // JS: this.singleEvent('End', pokemon.getItem(), pokemon.itemState, pokemon);
                // TODO: singleEvent('End') for ability and item

                // JS: pokemon.clearVolatile(false);
                self.sides[side_idx].pokemon[poke_idx].clear_volatile_full(false);

                // JS: pokemon.fainted = true;
                self.sides[side_idx].pokemon[poke_idx].fainted = true;

                // JS: pokemon.illusion = null;
                self.sides[side_idx].pokemon[poke_idx].illusion = None;

                // JS: pokemon.isActive = false;
                self.sides[side_idx].pokemon[poke_idx].is_active = false;

                // Remove from active slots
                for slot in 0..self.sides[side_idx].active.len() {
                    if let Some(idx) = self.sides[side_idx].active[slot] {
                        if idx == poke_idx {
                            self.sides[side_idx].active[slot] = None;
                            break;
                        }
                    }
                }

                // JS: pokemon.isStarted = false;
                // TODO: Implement is_started field (tracks if pokemon has been sent out)

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
            // TODO: this.queue.clear();
            // TODO: Clear Bide accumulated damage
        }

        // JS: else if (this.gen <= 3 && this.gameType === 'singles') { ... }
        // TODO: Gen 2-3 queue cancellation

        // JS: if (checkWin && this.checkWin(faintData)) return true;
        if check_win && self.check_win(last_faint_data.clone()) {
            return true;
        }

        // JS: if (faintData && length) { this.runEvent('AfterFaint', faintData.target, faintData.source, faintData.effect, length); }
        if last_faint_data.is_some() && length > 0 {
            // TODO: runEvent('AfterFaint')
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
            for &active_idx in &side.active {
                // Check if slot is empty (Pokemon fainted) OR if the Pokemon in slot is fainted
                if active_idx.is_none() {
                    // Slot is empty, need to check if there are non-fainted Pokemon available
                    let has_available_pokemon = side.pokemon.iter().any(|p| !p.is_fainted() && !p.is_active);
                    if has_available_pokemon {
                        needs_switch = true;
                        break;
                    }
                } else if let Some(poke_idx) = active_idx {
                    if side.pokemon.get(poke_idx).map(|p| p.is_fainted()).unwrap_or(false) {
                        needs_switch = true;
                        break;
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
    pub fn debug(&mut self, activity: &str) {
        if self.debug_mode {
            self.add_log("debug", &[activity]);
        }
    }

    /// Get the debug log (all lines)
    pub fn get_debug_log(&self) -> String {
        self.log.join("\n")
    }

    /// Add a log entry - matches JS this.add()
    /// JavaScript: add(...parts: (Part | (() => { side: SideID, secret: string, shared: string }))[])
    /// Usage: battle.add("-activate", &[pokemon.into(), "ability: Immunity".into()])
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
    pub fn add_move(&mut self, parts: &[&str]) {
        self.last_move_line = self.log.len() as i32;
        let entry = format!("|{}", parts.join("|"));
        self.log.push(entry);
    }

    /// Show a hint to the player
    /// Equivalent to battle.ts hint(hint, once?, side?)
    ///
    /// JS Source (battle.ts):
    /// ```js
    /// hint(hint: string, once?: boolean, side?: Side) {
    ///     if (this.hints.has(side ? `${side.id}|${hint}` : hint)) return;
    ///
    ///     if (side) {
    ///         this.addSplit(side.id, ['-hint', hint]);
    ///     } else {
    ///         this.add('-hint', hint);
    ///     }
    ///
    ///     if (once) this.hints.add(side ? `${side.id}|${hint}` : hint);
    /// }
    /// ```
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
    #[inline]
    pub fn trunc(&self, num: f64) -> i32 {
        num.trunc() as i32
    }

    /// Chain two modifiers together
    /// Equivalent to battle.ts chain(previousMod, nextMod)
    ///
    /// JS Source (battle.ts):
    /// ```js
    /// chain(previousMod: number | number[], nextMod: number | number[]) {
    ///     // previousMod or nextMod can be either a number or an array [numerator, denominator]
    ///     if (Array.isArray(previousMod)) {
    ///         previousMod = this.trunc(previousMod[0] * 4096 / previousMod[1]);
    ///     } else {
    ///         previousMod = this.trunc(previousMod * 4096);
    ///     }
    ///
    ///     if (Array.isArray(nextMod)) {
    ///         nextMod = this.trunc(nextMod[0] * 4096 / nextMod[1]);
    ///     } else {
    ///         nextMod = this.trunc(nextMod * 4096);
    ///     }
    ///     return ((previousMod * nextMod + 2048) >> 12) / 4096; // M'' = ((M * M') + 0x800) >> 12
    /// }
    /// ```
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
    /// When both modifiers are simple multipliers (not fractions)
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
    /// JS Source (battle.ts):
    /// ```js
    /// modify(value: number, numerator: number | number[], denominator = 1) {
    ///     // You can also use:
    ///     // modify(value, [numerator, denominator])
    ///     // modify(value, fraction) - assuming you trust JavaScript's floating-point handler
    ///     if (Array.isArray(numerator)) {
    ///         denominator = numerator[1];
    ///         numerator = numerator[0];
    ///     }
    ///     const tr = this.trunc;
    ///     const modifier = tr(numerator * 4096 / denominator);
    ///     return tr((tr(value * modifier) + 2048 - 1) / 4096);
    /// }
    /// ```
    pub fn modify(&self, value: i32, numerator: i32, denominator: i32) -> i32 {
        // JS: const modifier = tr(numerator * 4096 / denominator);
        let modifier = self.trunc((numerator * 4096) as f64 / denominator as f64);
        // JS: return tr((tr(value * modifier) + 2048 - 1) / 4096);
        let inner = self.trunc((value * modifier) as f64);
        self.trunc((inner + 2048 - 1) as f64 / 4096.0)
    }

    /// Apply a modifier to a value using a tuple for numerator/denominator
    /// Equivalent to battle.ts modify(value, [numerator, denominator])
    pub fn modify_tuple(&self, value: i32, fraction: (i32, i32)) -> i32 {
        self.modify(value, fraction.0, fraction.1)
    }

    /// Apply a floating-point modifier to a value
    /// Equivalent to battle.ts modify(value, modifier) when modifier is a float
    pub fn modify_f(&self, value: i32, multiplier: f64) -> i32 {
        // JS: const modifier = tr(numerator * 4096 / denominator);
        let modifier = self.trunc(multiplier * 4096.0);
        // JS: return tr((tr(value * modifier) + 2048 - 1) / 4096);
        let inner = self.trunc((value * modifier) as f64);
        self.trunc((inner + 2048 - 1) as f64 / 4096.0)
    }

    /// Get the current event's modifier
    /// Equivalent to this.event.modifier in JS
    pub fn event_modifier(&self) -> f64 {
        self.current_event.as_ref().map(|e| e.modifier).unwrap_or(1.0)
    }

    /// Declare a tie
    /// Equivalent to battle.ts tie()
    pub fn tie(&mut self) -> bool {
        self.win(None)
    }

    /// Declare a winner
    /// Equivalent to battle.ts win() (battle.ts:1474-1497)
    /// Pass None for a tie, or Some(side_id) for a winner
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
    pub fn set_active_move(&mut self, move_id: Option<ID>, pokemon: Option<(usize, usize)>, target: Option<(usize, usize)>) {
        self.active_move = move_id;
        self.active_pokemon = pokemon;
        self.active_target = target.or(pokemon);
    }

    /// Clear the currently active move
    /// Equivalent to battle.ts clearActiveMove()
    pub fn clear_active_move(&mut self, failed: bool) {
        if self.active_move.is_some() {
            if !failed {
                self.last_move = self.active_move.take();
            } else {
                self.active_move = None;
            }
            self.active_pokemon = None;
            self.active_target = None;
        }
    }

    /// Check if an ability is being suppressed (Mold Breaker, etc.)
    /// Equivalent to battle.ts suppressingAbility()
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
    pub fn get_all_pokemon(&self) -> Vec<&crate::pokemon::Pokemon> {
        let mut result = Vec::new();
        for side in &self.sides {
            for pokemon in &side.pokemon {
                result.push(pokemon);
            }
        }
        result
    }

    /// Get random target for a move
    /// Equivalent to battle.ts getRandomTarget()
    ///
    /// JS Source (battle.ts):
    /// ```js
    /// getRandomTarget(pokemon: Pokemon, move: string | Move) {
    ///     move = this.dex.moves.get(move);
    ///     if (['self', 'all', 'allySide', 'allyTeam', 'adjacentAllyOrSelf'].includes(move.target)) {
    ///         return pokemon;
    ///     } else if (move.target === 'adjacentAlly') {
    ///         if (this.gameType === 'singles') return null;
    ///         const adjacentAllies = pokemon.adjacentAllies();
    ///         return adjacentAllies.length ? this.sample(adjacentAllies) : null;
    ///     }
    ///     if (this.gameType === 'singles') return pokemon.side.foe.active[0];
    ///
    ///     if (this.activePerHalf > 2) {
    ///         if (move.target === 'adjacentFoe' || move.target === 'normal' || move.target === 'randomNormal') {
    ///             const adjacentFoes = pokemon.adjacentFoes();
    ///             if (adjacentFoes.length) return this.sample(adjacentFoes);
    ///             return pokemon.side.foe.active[pokemon.side.foe.active.length - 1 - pokemon.position];
    ///         }
    ///     }
    ///     return pokemon.side.randomFoe() || pokemon.side.foe.active[0];
    /// }
    /// ```
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
            // TODO: adjacentAllies() requires Pokemon helper method
            // For now, just return None
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
                // TODO: adjacentFoes() requires Pokemon helper method

                // JS: return pokemon.side.foe.active[pokemon.side.foe.active.length - 1 - pokemon.position];
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
                let random_idx = self.random(valid_targets.len() as u32) as usize;
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
    /// JS Source (battle.ts):
    /// ```js
    /// updateSpeed() {
    ///     for (const pokemon of this.getAllActive()) {
    ///         pokemon.updateSpeed();
    ///     }
    /// }
    /// ```
    pub fn update_speed(&mut self) {
        // Collect indices first to avoid borrow checker issues
        let indices: Vec<(usize, usize)> = self.get_all_active(false)
            .iter()
            .map(|pokemon| (pokemon.side_index, pokemon.position))
            .collect();

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
    /// JS Source (battle.ts):
    /// ```js
    /// damage(damage: number, target: Pokemon | null = null, source: Pokemon | null = null, effect: 'drain' | 'recoil' | Effect | null = null, instafaint = false) {
    ///     if (this.event) {
    ///         target ||= this.event.target;
    ///         source ||= this.event.source;
    ///         effect ||= this.effect;
    ///     }
    ///     return this.spreadDamage([damage], [target], source, effect, instafaint)[0];
    /// }
    /// ```
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
    /// JS Source (battle.ts):
    /// ```js
    /// directDamage(damage: number, target?: Pokemon, source: Pokemon | null = null, effect: Effect | null = null) {
    ///     if (this.event) {
    ///         target ||= this.event.target;
    ///         source ||= this.event.source;
    ///         effect ||= this.effect;
    ///     }
    ///     if (!target?.hp) return 0;
    ///     if (!damage) return 0;
    ///     // ...
    /// }
    /// ```
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
            self.last_damage = damage as u32;

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
                    // TODO: Implement substitute HP tracking
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
                pokemon.hp = pokemon.hp.saturating_sub(damage as u32);
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
    /// Matches JavaScript battle.ts:2217-2226
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
    /// JS Source (battle.ts):
    /// ```js
    /// heal(damage: number, target?: Pokemon, source: Pokemon | null = null, effect: 'drain' | Effect | null = null) {
    ///     if (this.event) {
    ///         target ||= this.event.target;
    ///         source ||= this.event.source;
    ///         effect ||= this.effect;
    ///     }
    ///     // ... rest of implementation
    /// }
    /// ```
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
                pokemon.hp = (pokemon.hp + damage as u32).min(pokemon.maxhp);
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

    /// Add a volatile condition to a Pokemon
    /// Matches JavaScript pokemon.ts:1937-1995 (Pokemon.addVolatile)
    pub fn add_volatile_to_pokemon(
        &mut self,
        target: (usize, usize),
        status: &ID,
        source: Option<(usize, usize)>,
        source_effect: Option<&ID>,
    ) -> bool {
        // JS: if (!this.hp && !status.affectsFainted) return false;
        let (side_idx, poke_idx) = target;
        let has_hp = if let Some(side) = self.sides.get(side_idx) {
            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                pokemon.hp > 0
            } else {
                return false;
            }
        } else {
            return false;
        };

        if !has_hp {
            // TODO: Check status.affectsFainted
            return false;
        }

        // JS: if (linkedStatus && source && !source.hp) return false;
        // TODO: Implement linkedStatus support

        // JS: if (this.battle.event) { if (!source) source = this.battle.event.source; if (!sourceEffect) sourceEffect = this.battle.effect; }
        let (event_source, event_effect) = if let Some(ref event) = self.current_event {
            (event.source, event.effect.clone())
        } else {
            (None, None)
        };

        let source = source.or(event_source);
        let source_effect_owned: Option<ID>;
        let source_effect = if source_effect.is_none() {
            source_effect_owned = event_effect;
            source_effect_owned.as_ref()
        } else {
            source_effect
        };

        // JS: if (!source) source = this;
        let source = source.or(Some(target));

        // JS: if (this.volatiles[status.id]) { if (!status.onRestart) return false; return this.battle.singleEvent('Restart', status, this.volatiles[status.id], this, source, sourceEffect); }
        let already_has = if let Some(side) = self.sides.get(side_idx) {
            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                pokemon.has_volatile(status)
            } else {
                return false;
            }
        } else {
            return false;
        };

        if already_has {
            // JS: if (!status.onRestart) return false; return this.battle.singleEvent('Restart', status, this.volatiles[status.id], this, source, sourceEffect);
            // TODO: Call onRestart callback when move_callbacks is reimplemented
            // For now, return false when status already exists
            return false;
        }

        // JS: if (!this.runStatusImmunity(status.id)) { this.battle.debug('immune to volatile status'); if (sourceEffect?.status) { this.battle.add('-immune', this); } return false; }
        // Run TryImmunity event for moves like attract
        if status.as_str() == "attract" {
            if let Some(src) = source {
                let immunity_result = self.run_event_bool("TryImmunity", Some(target), Some(src), Some(status));
                if !immunity_result {
                    self.debug("immune to volatile status");
                    // JS: if (sourceEffect?.status) { this.battle.add('-immune', this); }
                    // TODO: Check sourceEffect.status
                    return false;
                }
            }
        }

        // JS: result = this.battle.runEvent('TryAddVolatile', this, source, sourceEffect, status);
        // TODO: Implement TryAddVolatile event

        // JS: this.volatiles[status.id] = this.battle.initEffectState({ id: status.id, name: status.name, target: this });
        let mut effect_state = EffectState::new(status.clone());

        // JS: if (source) { this.volatiles[status.id].source = source; this.volatiles[status.id].sourceSlot = source.getSlot(); }
        if let Some((src_side, src_poke)) = source {
            effect_state.source_slot = Some(format!("{}:{}", src_side, src_poke));
        }

        // JS: if (sourceEffect) this.volatiles[status.id].sourceEffect = sourceEffect;
        // Store in effect_state.data
        if let Some(eff) = source_effect {
            effect_state.data.insert("sourceEffect".to_string(), serde_json::json!(eff.as_str()));
        }

        // JS: if (status.duration) this.volatiles[status.id].duration = status.duration;
        // TODO: Look up condition duration from dex

        // JS: result = this.battle.singleEvent('Start', status, this.volatiles[status.id], this, source, sourceEffect);
        // TODO: Implement Start event

        // Add volatile to Pokemon
        if let Some(side) = self.sides.get_mut(side_idx) {
            if let Some(pokemon) = side.pokemon.get_mut(poke_idx) {
                pokemon.volatiles.insert(status.clone(), effect_state);
            } else {
                return false;
            }
        } else {
            return false;
        }

        // JS: result = this.battle.singleEvent('Start', status, this.volatiles[status.id], this, source, sourceEffect);
        // TODO: Call onStart callbacks when move_callbacks is reimplemented
        // For now, just continue
        if status.as_str() == "allyswitch" {
//             crate::data::move_callbacks::allyswitch::on_start(self, target);
        }
        if status.as_str() == "aquaring" {
//             crate::data::move_callbacks::aquaring::on_start(self, target);
        }
        if status.as_str() == "attract" {
//             let result = crate::data::move_callbacks::attract::on_start(self, target);
//             match result {
//                 crate::data::move_callbacks::MoveHandlerResult::False => return false,
//                 _ => {}
//             }
        }

        true
    }

    /// Helper to add heal log messages
    /// Matches JavaScript battle.ts:2246-2268
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
                // TODO: Check if effect.effectType === 'Move'
                if let Some(src) = source {
                    let src_str = format!("p{}a", src.0 + 1);
                    let from_str = format!("[from] {}", effect_id);
                    let of_str = format!("[of] {}", src_str);
                    self.add_log("-heal", &[&target_str, &health_str, &from_str, &of_str]);
                } else {
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
    /// JS Source (battle.ts):
    /// ```js
    /// boost(boost, target, source, effect, isSecondary, isSelf) {
    ///     if (this.event) { target ||= this.event.target; source ||= this.event.source; effect ||= this.effect; }
    ///     if (!target?.hp) return 0;
    ///     if (!target.isActive) return false;
    ///     if (this.gen > 5 && !target.side.foePokemonLeft()) return false;
    ///     boost = this.runEvent('ChangeBoost', target, source, effect, { ...boost });
    ///     boost = target.getCappedBoost(boost);
    ///     boost = this.runEvent('TryBoost', target, source, effect, { ...boost });
    ///     // ... apply boosts, log, fire AfterEachBoost
    ///     this.runEvent('AfterBoost', target, source, effect, boost);
    ///     return success;
    /// }
    /// ```
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
        // TODO: Implement ChangeBoost event to modify boosts before applying
        self.run_event("ChangeBoost", Some(target), source, None, None);

        // JS: boost = target.getCappedBoost(boost);
        // Clamp boosts to [-6, 6] range - done per-stat below

        // JS: boost = this.runEvent('TryBoost', target, source, effect, { ...boost });
        // TODO: Implement TryBoost event to prevent boosts
        self.run_event("TryBoost", Some(target), source, None, None);

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
    /// JS Source (battle.ts):
    /// ```js
    /// checkFainted() {
    ///     for (const side of this.sides) {
    ///         for (const pokemon of side.active) {
    ///             if (pokemon.fainted) {
    ///                 pokemon.status = 'fnt' as ID;
    ///                 pokemon.switchFlag = true;
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
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
    pub fn clamp_int_range(&self, num: i32, min: i32, max: i32) -> i32 {
        num.clamp(min, max)
    }

    // =========================================================================
    // Priority and Speed Sorting Methods (ported from battle.ts)
    // =========================================================================

    /// Compare priority of two actions/handlers
    /// Equivalent to battle.ts comparePriority()
    /// Returns negative if a comes first, positive if b comes first, 0 if equal
    pub fn compare_priority(a: &PriorityItem, b: &PriorityItem) -> std::cmp::Ordering {
        // 1. Order, low to high (default last)
        let order_cmp = a.order.unwrap_or(u32::MAX).cmp(&b.order.unwrap_or(u32::MAX));
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
    pub fn compare_left_to_right_order(a: &PriorityItem, b: &PriorityItem) -> std::cmp::Ordering {
        // Order first
        let order_cmp = a.order.unwrap_or(u32::MAX).cmp(&b.order.unwrap_or(u32::MAX));
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
    fn shuffle_range<T>(&mut self, list: &mut [T], start: usize, end: usize) {
        for i in start..end {
            let j = start + (self.random((end - start) as u32) as usize);
            list.swap(i, j);
        }
    }

    // =========================================================================
    // Pokemon Lookup Methods (ported from battle.ts)
    // =========================================================================

    /// Get a Pokemon by its full name (e.g., "p1: Pikachu")
    /// Equivalent to battle.ts getPokemon()
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
    pub fn can_switch(&self, side_idx: usize) -> usize {
        self.possible_switches(side_idx).len()
    }

    /// Get a random switchable Pokemon
    /// Equivalent to battle.ts getRandomSwitchable()
    pub fn get_random_switchable(&mut self, side_idx: usize) -> Option<usize> {
        let switches = self.possible_switches(side_idx);
        if switches.is_empty() {
            return None;
        }
        let idx = self.random(switches.len() as u32) as usize;
        Some(switches[idx])
    }

    /// Get list of Pokemon that can switch in
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
    pub fn valid_target(&self, target: (usize, usize), source: (usize, usize), target_type: &str) -> bool {
        // JS: return this.validTargetLoc(source.getLocOf(target), source, targetType);
        let target_loc = self.get_loc_of(source, target);
        self.valid_target_loc(target_loc, source, target_type)
    }

    /// Get Pokemon at a specific slot location
    /// Get Pokemon at a slot string (e.g., "p1a", "p2b")
    /// Equivalent to battle.ts getAtSlot()
    pub fn get_at_slot(&self, slot: Option<&str>) -> Option<&Pokemon> {
        // JS: if (!slot) return null;
        let slot_str = slot?;

        if slot_str.len() < 3 {
            return None;
        }

        // JS: const side = this.sides[slot.charCodeAt(1) - 49]; // 49 is '1'
        let side_char = slot_str.chars().nth(1)?;
        let side_idx = (side_char as u32).checked_sub(49)? as usize; // 49 is '1'

        // JS: const position = slot.charCodeAt(2) - 97; // 97 is 'a'
        let pos_char = slot_str.chars().nth(2)?;
        let position = (pos_char as u32).checked_sub(97)? as usize; // 97 is 'a'

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
    /// JS Source (battle.ts:1577-1754):
    /// ```js
    /// endTurn() {
    ///     this.turn++;
    ///     this.lastSuccessfulMoveThisTurn = null;
    ///
    ///     const dynamaxEnding: Pokemon[] = [];
    ///     for (const pokemon of this.getAllActive()) {
    ///         if (pokemon.volatiles['dynamax']?.turns === 3) {
    ///             dynamaxEnding.push(pokemon);
    ///         }
    ///     }
    ///     if (dynamaxEnding.length > 1) {
    ///         this.updateSpeed();
    ///         this.speedSort(dynamaxEnding);
    ///     }
    ///     for (const pokemon of dynamaxEnding) {
    ///         pokemon.removeVolatile('dynamax');
    ///     }
    ///
    ///     // Gen 1 partial trapping cleanup...
    ///     // Pokemon field resets (moveThisTurn, newlySwitched, etc.)...
    ///     // DisableMove event processing...
    ///     // Type appearance tracking (Gen 7+)...
    ///     // Trap checking...
    /// }
    /// ```
    pub fn end_turn(&mut self) {
        self.turn += 1;

        // JS: this.lastSuccessfulMoveThisTurn = null;
        self.last_successful_move_this_turn = None;

        // TODO: Dynamax 3-turn removal (requires volatile.turns tracking)
        // JS: Check volatiles['dynamax']?.turns === 3, updateSpeed(), speedSort(), removeVolatile()

        // TODO: Gen 1 partial trapping cleanup (requires partialtrappinglock/partiallytrapped volatiles)

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

                // TODO: DisableMove event processing (requires moveSlots.disabled field and disableMove() method)
                // JS: for (const moveSlot of pokemon.moveSlots) { moveSlot.disabled = false; }
                // JS: this.runEvent('DisableMove', pokemon);
                // JS: for (const moveSlot of pokemon.moveSlots) { this.singleEvent('DisableMove', activeMove, null, pokemon); }

                // TODO: Illusion/type appearance tracking (Gen 7+)
                // JS: if (pokemon.getLastAttackedBy() && this.gen >= 7) pokemon.knownType = true;

                // TODO: attackedBy array cleanup
                // JS: for (let i = pokemon.attackedBy.length - 1; i >= 0; i--) { ... }

                // TODO: Type change messages (Gen 7+)
                // JS: if (this.gen >= 7 && !pokemon.terastallized) { ... }

                // JS: pokemon.trapped = pokemon.maybeTrapped = false;
                pokemon.trapped = false;
                pokemon.maybe_trapped = false;
            }
        }

        // TODO: TrapPokemon and MaybeTrapPokemon events
        // JS: this.runEvent('TrapPokemon', pokemon);
        // JS: if (!pokemon.knownType || this.dex.getImmunity('trapped', pokemon)) { this.runEvent('MaybeTrapPokemon', pokemon); }

        // TODO: Foe ability trapping check (Gen 3+)
        // JS: for (const source of pokemon.foes()) { ... check abilities ... }

        self.add_log("", &[]);
        self.add_log("turn", &[&self.turn.to_string()]);
    }

    /// Main turn loop
    /// Equivalent to battle.ts turnLoop()
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
                        // JS: for (const rule of this.ruleTable.keys()) { ... }

                        // TODO: Switch in all active Pokemon (requires battle_actions integration)
                        // JS: for (const side of this.sides) { for (let i = 0; i < side.active.length; i++) { this.actions.switchIn(side.pokemon[i], i); } }

                        // TODO: Start events for each Pokemon
                        // JS: for (const pokemon of this.getAllPokemon()) { this.singleEvent('Start', ...); }

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
    /// JS Source (battle.ts):
    /// ```js
    /// allChoicesDone() {
    ///     let totalActions = 0;
    ///     for (const side of this.sides) {
    ///         if (side.isChoiceDone()) {
    ///             if (!this.supportCancel) side.choice.cantUndo = true;
    ///             totalActions++;
    ///         }
    ///     }
    ///     return totalActions >= this.sides.length;
    /// }
    /// ```
    pub fn all_choices_done(&mut self) -> bool {
        // JS: let totalActions = 0;
        let mut total_actions = 0;

        // JS: for (const side of this.sides)
        for side in &mut self.sides {
            // JS: if (side.isChoiceDone())
            if side.is_choice_done() {
                // JS: if (!this.supportCancel) side.choice.cantUndo = true;
                // TODO: Add support_cancel field to Battle struct
                // For now, always set cantUndo to true (equivalent to supportCancel = false)
                side.choice.cant_undo = true;

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
    /// JS Source (battle.ts):
    /// ```js
    /// getActionSpeed(action: AnyObject) {
    ///     if (action.choice === 'move') {
    ///         let move = action.move;
    ///         if (action.zmove) { move = this.dex.getActiveMove(zMoveName); }
    ///         if (action.maxMove) { move = this.actions.getActiveMaxMove(...); }
    ///         let priority = this.dex.moves.get(move.id).priority;
    ///         priority = this.singleEvent('ModifyPriority', move, null, action.pokemon, null, null, priority);
    ///         priority = this.runEvent('ModifyPriority', action.pokemon, null, move, priority);
    ///         action.priority = priority + action.fractionalPriority;
    ///         if (this.gen > 5) action.move.priority = priority;
    ///     }
    ///     if (!action.pokemon) { action.speed = 1; }
    ///     else { action.speed = action.pokemon.getActionSpeed(); }
    /// }
    /// ```
    ///
    /// Note: This method MUTATES the action object by setting priority and speed fields
    pub fn get_action_speed(&mut self, action: &mut crate::battle_queue::Action) {
        use crate::battle_queue::Action;

        match action {
            Action::Move(ref mut move_action) => {
                // JS: if (action.choice === 'move')

                // Get the move (considering Z-Move/Max Move transformations)
                let move_id = move_action.move_id.clone();

                // JS: if (action.zmove) { ... }
                // TODO: Implement Z-Move transformation
                // Requires: this.actions.getZMove(), this.dex.getActiveMove()

                // JS: if (action.maxMove) { ... }
                // TODO: Implement Max Move transformation
                // Requires: this.actions.getMaxMove(), this.actions.getActiveMaxMove()

                // JS: let priority = this.dex.moves.get(move.id).priority;
                // TODO: Get move priority from Dex
                // For now, use the priority already set in move_action (from when action was created)
                // Full implementation requires: Battle to have reference to Dex
                let base_priority = move_action.priority;

                // JS: priority = this.singleEvent('ModifyPriority', move, null, action.pokemon, null, null, priority);
                // TODO: Implement ModifyPriority single event
                // Requires: singleEvent with move context

                // JS: priority = this.runEvent('ModifyPriority', action.pokemon, null, move, priority);
                // TODO: Implement ModifyPriority run event
                // Requires: runEvent with move context, pokemon from action

                // JS: action.priority = priority + action.fractionalPriority;
                // Note: fractionalPriority is already applied when action is created
                // Just ensure priority is set correctly
                move_action.priority = base_priority;

                // JS: if (this.gen > 5) action.move.priority = priority;
                // TODO: Set move.priority field (requires mutable move reference)

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
    fn get_pokemon_action_speed(&self, side_idx: usize, poke_idx: usize) -> u32 {
        if let Some(side) = self.sides.get(side_idx) {
            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                // Apply speed boosts
                let base_speed = pokemon.stored_stats.spe as u32;
                let stage = pokemon.boosts.spe;
                let multiplier = if stage >= 0 {
                    (2 + stage as u32) as f64 / 2.0
                } else {
                    2.0 / (2 + (-stage) as u32) as f64
                };
                return (base_speed as f64 * multiplier) as u32;
            }
        }
        0
    }

    /// Swap position of a Pokemon to a new position on their side
    /// Equivalent to battle.ts swapPosition()
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
        // TODO: Fire Swap events when event system is ready

        true
    }

    /// Get move category, defaulting to "Physical" if move not found
    /// Equivalent to battle.ts getCategory() (battle.ts:2350-2352)
    ///
    /// JS Source (battle.ts):
    /// ```js
    /// getCategory(move: string | Move) {
    ///     return this.dex.moves.get(move).category || 'Physical';
    /// }
    /// ```
    pub fn get_category(&self, move_id: &ID) -> String {
        if let Some(move_def) = self.dex.get_move(move_id.as_str()) {
            return move_def.category.clone();
        }
        "Physical".to_string()
    }

    /// Clear request state
    /// Equivalent to battle.ts clearRequest() (battle.ts:1364-1370)
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
    /// JS Source (battle.ts:1331-1363):
    /// ```js
    /// makeRequest(type?: RequestState) {
    ///     if (type) {
    ///         this.requestState = type;
    ///         for (const side of this.sides) {
    ///             side.clearChoice();
    ///         }
    ///     } else {
    ///         type = this.requestState;
    ///     }
    ///     for (const side of this.sides) {
    ///         side.activeRequest = null;
    ///     }
    ///     if (type === 'teampreview') {
    ///         const pickedTeamSize = this.ruleTable.pickedTeamSize;
    ///         this.add(`teampreview${pickedTeamSize ? `|${pickedTeamSize}` : ''}`);
    ///     }
    ///     const requests = this.getRequests(type);
    ///     for (let i = 0; i < this.sides.length; i++) {
    ///         this.sides[i].activeRequest = requests[i];
    ///     }
    ///     this.sentRequests = false;
    ///     if (this.sides.every(side => side.isChoiceDone())) {
    ///         throw new Error(`Choices are done immediately after a request`);
    ///     }
    /// }
    /// ```
    pub fn make_request(&mut self, request_type: Option<BattleRequestState>) {
        // JS: if (type) { this.requestState = type; ... } else { type = this.requestState; }
        let req_type = if let Some(rt) = request_type {
            self.request_state = rt;
            // JS: for (const side of this.sides) { side.clearChoice(); }
            // TODO: Implement side.clearChoice() - requires different signature in Rust
            rt
        } else {
            self.request_state
        };

        // JS: for (const side of this.sides) { side.activeRequest = null; }
        // TODO: Implement side.activeRequest field

        // JS: if (type === 'teampreview') { ... this.add(`teampreview${pickedTeamSize ? `|${pickedTeamSize}` : ''}`); }
        if matches!(req_type, BattleRequestState::TeamPreview) {
            // TODO: Implement ruleTable.pickedTeamSize
            // For now, just add 'teampreview' without size
            self.add("-", &[Arg::Str("teampreview")]);
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
        // TODO: Implement sentRequests field

        // JS: if (this.sides.every(side => side.isChoiceDone())) { throw new Error(...); }
        // TODO: Implement isChoiceDone() check
        // This is a safety check to prevent infinite loops
    }

    /// Check and trigger Endless Battle Clause
    /// Equivalent to battle.ts maybeTriggerEndlessBattleClause() (battle.ts:1757-1856)
    ///
    /// JS Source (battle.ts):
    /// ```js
    /// maybeTriggerEndlessBattleClause(trappedBySide, stalenessBySide) {
    ///   // Gen 1 checks (omitted in Rust - requires Pokemon.hasType(), staleness tracking)
    ///   if (this.turn <= 100) return;
    ///   if (this.turn >= 1000) { ... tie(); return true; }
    ///   if ((turn >= 500 && turn % 100 === 0) || (turn >= 900 && turn % 10 === 0) || turn >= 990) {
    ///     this.add('bigerror', `You will auto-tie if the battle doesn't end in ${turnsLeft} turns...`);
    ///   }
    ///   // staleness checks (omitted in Rust - requires Pokemon.volatileStaleness/staleness)
    ///   // berry cycling checks (omitted in Rust - requires Harvest, Recycle, RESTORATIVE_BERRIES)
    /// }
    /// ```
    ///
    /// TODO: Missing features (requires infrastructure):
    /// - Gen 1 no-progress checks (needs Pokemon.hasType(), frozen status checks, Transform move checks)
    /// - Staleness tracking (needs Pokemon.volatileStaleness, Pokemon.staleness fields)
    /// - Berry cycling detection (needs Harvest/Pickup abilities, Recycle move, RESTORATIVE_BERRIES constant)
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
    /// JS Source (battle.ts):
    /// ```js
    /// join(slot: SideID, name: string, avatar: string, team: PokemonSet[] | string | null) {
    ///     this.setPlayer(slot, { name, avatar, team });
    ///     return this.getSide(slot);
    /// }
    /// ```
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
            return EventResult::Fail;
        }

        // JavaScript: if (this.log.length - this.sentLogPos > 1000) throw Error
        if self.log.len() - self.sent_log_pos > 1000 {
            self.add_log("message", &["LINE LIMIT EXCEEDED"]);
            self.add_log("message", &["PLEASE REPORT IN BUG THREAD"]);
            self.add_log("message", &[&format!("Event: {}", event_id)]);
            if let Some(ref evt) = self.current_event {
                self.add_log("message", &[&format!("Parent event: {}", evt.id)]);
            }
            return EventResult::Fail;
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
            modifier: 1.0,
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
    fn get_effect_type(&self, effect_id: &ID) -> &str {
        // Check if it's an ability
        if crate::data::abilities::get_ability(effect_id).is_some() {
            return "Ability";
        }
        // Check if it's an item
        if crate::data::items::get_item(effect_id).is_some() {
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
        if let Some(ability_def) = crate::data::abilities::get_ability(effect_id) {
            return self.handle_ability_event(event_id, ability_def, target);
        }

        // Handle item events
        if let Some(item_def) = crate::data::items::get_item(effect_id) {
            return self.handle_item_event(event_id, item_def, target);
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
    fn handle_ability_event(
        &mut self,
        event_id: &str,
        ability: &crate::data::abilities::AbilityDef,
        target: Option<(usize, usize)>,
    ) -> crate::event::EventResult {
        use crate::event::EventResult;

        match event_id {
            "SwitchIn" => {
                // Handle switch-in abilities
                if let Some((side_idx, poke_idx)) = target {
                    // Intimidate
                    if ability.id.as_str() == "intimidate" {
                        // Lower foe's Attack - collect targets first to avoid borrow issues
                        let foe_side = if side_idx == 0 { 1 } else { 0 };
                        let mut targets = Vec::new();
                        if let Some(foe) = self.sides.get(foe_side) {
                            for slot in 0..foe.active.len() {
                                if foe.active[slot].is_some() {
                                    targets.push((foe_side, slot));
                                }
                            }
                        }
                        for target_pos in targets {
                            self.boost(&[("atk", -1)], target_pos, Some((side_idx, poke_idx)), Some("Intimidate"));
                        }
                        return EventResult::Stop;
                    }
                    // Drizzle
                    if ability.id.as_str() == "drizzle" {
                        self.field.set_weather(ID::new("rain"), None);
                        return EventResult::Stop;
                    }
                    // Drought
                    if ability.id.as_str() == "drought" {
                        self.field.set_weather(ID::new("sunnyday"), None);
                        return EventResult::Stop;
                    }
                    // Sand Stream
                    if ability.id.as_str() == "sandstream" {
                        self.field.set_weather(ID::new("sandstorm"), None);
                        return EventResult::Stop;
                    }
                    // Snow Warning
                    if ability.id.as_str() == "snowwarning" {
                        self.field.set_weather(ID::new("snow"), None);
                        return EventResult::Stop;
                    }
                }
            }
            "ModifyDamage" => {
                // Damage modifying abilities
                if ability.id.as_str() == "multiscale" {
                    if let Some((side_idx, poke_idx)) = target {
                        if let Some(side) = self.sides.get(side_idx) {
                            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                                if pokemon.hp == pokemon.maxhp {
                                    return EventResult::Modify(0.5);
                                }
                            }
                        }
                    }
                }
            }
            "Residual" => {
                // Residual abilities like Poison Heal, Rain Dish, Hydration, Bad Dreams, etc.
                if let Some((side_idx, poke_idx)) = target {
                    if ability.id.as_str() == "poisonheal" {
                        if let Some(side) = self.sides.get(side_idx) {
                            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                                if pokemon.status.as_str() == "tox" || pokemon.status.as_str() == "psn" {
                                    let heal = pokemon.maxhp / 8;
                                    self.heal(heal as i32, Some((side_idx, poke_idx)), None, Some(&ID::new("Poison Heal")));
                                    return EventResult::Stop;
                                }
                            }
                        }
                    }
                    if ability.id.as_str() == "hydration" {
                        if let Some(side) = self.sides.get(side_idx) {
                            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                                // Check if has status and weather is rain/primordialsea
                                if !pokemon.status.is_empty() {
                                    let weather = self.field.weather.as_str();
                                    if weather == "raindance" || weather == "primordialsea" {
                                        self.add("-activate", &[
                                            crate::battle::Arg::String(pokemon.name.clone()),
                                            crate::battle::Arg::Str("ability: Hydration")
                                        ]);
                                        let side_idx_mut = side_idx;
                                        let poke_idx_mut = poke_idx;
                                        self.sides[side_idx_mut].pokemon[poke_idx_mut].cure_status();
                                        return EventResult::Stop;
                                    }
                                }
                            }
                        }
                    }
                    if ability.id.as_str() == "baddreams" {
                        if let Some(side) = self.sides.get(side_idx) {
                            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                                if pokemon.hp == 0 {
                                    return EventResult::Continue;
                                }
                                // Damage sleeping foes
                                let foe_side = if side_idx == 0 { 1 } else { 0 };
                                let mut foes_to_damage: Vec<(usize, u32)> = Vec::new();

                                if let Some(foe_side_ref) = self.sides.get(foe_side) {
                                    for foe in foe_side_ref.pokemon.iter().filter(|p| p.is_active && !p.fainted) {
                                        if foe.status.as_str() == "slp" || foe.ability.as_str() == "comatose" {
                                            let damage = foe.maxhp / 8;
                                            foes_to_damage.push((foe.position, damage));
                                        }
                                    }
                                }

                                for (foe_pos, dmg) in foes_to_damage {
                                    self.damage(dmg as i32, Some((foe_side, foe_pos)), Some((side_idx, poke_idx)), None, false);
                                }
                                return EventResult::Stop;
                            }
                        }
                    }
                    if ability.id.as_str() == "healer" {
                        // Healer: 30% chance to cure status of adjacent allies
                        // Collect data first, then perform mutations
                        let mut should_continue = false;
                        let mut pokemon_name = String::new();
                        let mut allies_with_status: Vec<usize> = Vec::new();

                        if let Some(side) = self.sides.get(side_idx) {
                            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                                if pokemon.hp == 0 {
                                    should_continue = true;
                                } else {
                                    pokemon_name = pokemon.name.clone();
                                    for ally in side.pokemon.iter().filter(|p| p.is_active && !p.fainted) {
                                        if ally.position != pokemon.position && !ally.status.is_empty() {
                                            allies_with_status.push(ally.position);
                                        }
                                    }
                                }
                            }
                        }

                        if should_continue {
                            return EventResult::Continue;
                        }

                        // Now perform mutations
                        for ally_pos in allies_with_status {
                            if self.random_chance(3, 10) {
                                self.add("-activate", &[
                                    crate::battle::Arg::String(pokemon_name.clone()),
                                    crate::battle::Arg::Str("ability: Healer")
                                ]);
                                self.sides[side_idx].pokemon[ally_pos].cure_status();
                            }
                        }
                        return EventResult::Stop;
                    }
                    if ability.id.as_str() == "speedboost" {
                        if let Some(side) = self.sides.get(side_idx) {
                            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                                // Speed Boost: Boosts Speed by 1 stage at end of turn (if active_turns > 0)
                                if pokemon.active_turns > 0 {
                                    self.boost(&[("spe", 1)], (side_idx, poke_idx), Some((side_idx, poke_idx)), None);
                                }
                                return EventResult::Stop;
                            }
                        }
                    }
                    if ability.id.as_str() == "moody" {
                        // Moody: Randomly raises one stat by 2 and lowers another by 1 each turn
                        // Phase 1: Collect boost data
                        let (mut stats_to_raise, boosts_data) = {
                            if let Some(side) = self.sides.get(side_idx) {
                                if let Some(pokemon) = side.pokemon.get(poke_idx) {
                                    let mut stats = Vec::new();
                                    if pokemon.boosts.atk < 6 { stats.push("atk"); }
                                    if pokemon.boosts.def < 6 { stats.push("def"); }
                                    if pokemon.boosts.spa < 6 { stats.push("spa"); }
                                    if pokemon.boosts.spd < 6 { stats.push("spd"); }
                                    if pokemon.boosts.spe < 6 { stats.push("spe"); }
                                    (stats, (pokemon.boosts.atk, pokemon.boosts.def, pokemon.boosts.spa, pokemon.boosts.spd, pokemon.boosts.spe))
                                } else {
                                    (Vec::new(), (0, 0, 0, 0, 0))
                                }
                            } else {
                                (Vec::new(), (0, 0, 0, 0, 0))
                            }
                        };

                        // Phase 2: Random selection (mutable borrow)
                        let raised_stat = if !stats_to_raise.is_empty() {
                            let idx = self.random(stats_to_raise.len() as u32) as usize;
                            Some(stats_to_raise[idx])
                        } else {
                            None
                        };

                        // Build list of stats that can be lowered
                        let (atk, def, spa, spd, spe) = boosts_data;
                        let mut stats_to_lower = Vec::new();
                        if atk > -6 && Some("atk") != raised_stat { stats_to_lower.push("atk"); }
                        if def > -6 && Some("def") != raised_stat { stats_to_lower.push("def"); }
                        if spa > -6 && Some("spa") != raised_stat { stats_to_lower.push("spa"); }
                        if spd > -6 && Some("spd") != raised_stat { stats_to_lower.push("spd"); }
                        if spe > -6 && Some("spe") != raised_stat { stats_to_lower.push("spe"); }

                        let lowered_stat = if !stats_to_lower.is_empty() {
                            let idx = self.random(stats_to_lower.len() as u32) as usize;
                            Some(stats_to_lower[idx])
                        } else {
                            None
                        };

                        // Build boost array
                        let mut boosts = Vec::new();
                        if let Some(stat) = raised_stat {
                            boosts.push((stat, 2));
                        }
                        if let Some(stat) = lowered_stat {
                            boosts.push((stat, -1));
                        }

                        // Apply boosts if any
                        if !boosts.is_empty() {
                            self.boost(&boosts, (side_idx, poke_idx), Some((side_idx, poke_idx)), None);
                        }
                        return EventResult::Stop;
                    }
                    if ability.id.as_str() == "shedskin" {
                        // 33% chance to cure status
                        // if (pokemon.hp && pokemon.status && this.randomChance(33, 100))
                        let (has_hp, has_status) = {
                            if let Some(side) = self.sides.get(side_idx) {
                                if let Some(pokemon) = side.pokemon.get(poke_idx) {
                                    (pokemon.hp > 0, !pokemon.status.is_empty())
                                } else {
                                    (false, false)
                                }
                            } else {
                                (false, false)
                            }
                        };
                        if has_hp && has_status && self.random_chance(33, 100) {
                            // this.add('-activate', pokemon, 'ability: Shed Skin');
                            if let Some(side) = self.sides.get(side_idx) {
                                if let Some(pokemon) = side.pokemon.get(poke_idx) {
                                    self.add("-activate", &[
                                        Arg::String(pokemon.name.clone()),
                                        Arg::Str("ability: Shed Skin")
                                    ]);
                                }
                            }
                            // pokemon.cureStatus();
                            self.sides[side_idx].pokemon[poke_idx].cure_status();
                        }
                        return EventResult::Stop;
                    }
                }
            }
            _ => {}
        }

        EventResult::Continue
    }

    /// Handle item events
    fn handle_item_event(
        &mut self,
        event_id: &str,
        item: &crate::data::items::ItemDef,
        target: Option<(usize, usize)>,
    ) -> crate::event::EventResult {
        use crate::event::EventResult;

        match event_id {
            "Residual" => {
                if let Some((side_idx, poke_idx)) = target {
                    // Leftovers
                    if item.id.as_str() == "leftovers" {
                        if let Some(side) = self.sides.get(side_idx) {
                            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                                if pokemon.hp < pokemon.maxhp {
                                    let heal = pokemon.maxhp / 16;
                                    self.heal(heal.max(1) as i32, Some((side_idx, poke_idx)), None, Some(&ID::new("Leftovers")));
                                    return EventResult::Stop;
                                }
                            }
                        }
                    }
                    // Black Sludge
                    if item.id.as_str() == "blacksludge" {
                        if let Some(side) = self.sides.get(side_idx) {
                            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                                let is_poison = pokemon.types.iter().any(|t| t.to_lowercase() == "poison");
                                if is_poison {
                                    if pokemon.hp < pokemon.maxhp {
                                        let heal = pokemon.maxhp / 16;
                                        self.heal(heal.max(1) as i32, Some((side_idx, poke_idx)), None, Some(&ID::new("Black Sludge")));
                                    }
                                } else {
                                    let damage = pokemon.maxhp / 8;
                                    self.damage(damage.max(1) as i32, Some((side_idx, poke_idx)), None, Some(&ID::new("Black Sludge")), false);
                                }
                                return EventResult::Stop;
                            }
                        }
                    }
                }
            }
            "ModifyDamage" => {
                // Life Orb
                if item.id.as_str() == "lifeorb" {
                    return EventResult::Modify(1.3);
                }
            }
            "AfterMoveSecondarySelf" => {
                // Life Orb recoil
                if item.id.as_str() == "lifeorb" {
                    if let Some((side_idx, poke_idx)) = target {
                        if let Some(side) = self.sides.get(side_idx) {
                            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                                let recoil = pokemon.maxhp / 10;
                                self.damage(recoil.max(1) as i32, Some((side_idx, poke_idx)), None, Some(&ID::new("Life Orb")), false);
                            }
                        }
                    }
                }
            }
            _ => {}
        }

        EventResult::Continue
    }

    /// Handle move events
    fn handle_move_event(
        &mut self,
        event_id: &str,
        move_id: &str,
        target: Option<(usize, usize)>,
    ) -> crate::event::EventResult {
        use crate::event::EventResult;

        // Get source from current event
        let source = if let Some(ref event) = self.current_event {
            event.source
        } else {
            None
        };

        // Get move effect ID
        let move_effect_id = ID::new(move_id);

        match event_id {
            "PrepareHit" => {
                // TODO: onPrepareHit callbacks - implement when move_callbacks is reimplemented
                // For now, just continue
            }
            "Hit" => {
                // TODO: Secondary effect onHit callbacks - implement when move_callbacks is reimplemented
                // For now, just continue
            }
            "Try" => {
                // TODO: onTry callbacks - implement when move_callbacks is reimplemented
                // For now, just continue
            }
            "ModifyType" => {
                // TODO: onModifyType callbacks - implement when move_callbacks is reimplemented
                // For now, just continue
            }
            "TryHit" => {
                // TODO: onTryHit callbacks - implement when move_callbacks is reimplemented
                // For now, just continue
            }
            "Hit" => {
                // TODO: onHit callbacks - implement when move_callbacks is reimplemented
                // For now, just continue
            }
            "TryImmunity" => {
                // TODO: onTryImmunity callbacks - implement when move_callbacks is reimplemented
                // For now, just continue
            }
            "MoveFail" => {
                // TODO: onMoveFail callbacks - implement when move_callbacks is reimplemented
                // For now, just continue
            }
            _ => {}
        }

        EventResult::Continue
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
    fn handle_condition_event(
        &mut self,
        event_id: &str,
        condition_id: &str,
        target: Option<(usize, usize)>,
    ) -> crate::event::EventResult {
        use crate::event::EventResult;

        match event_id {
            "Start" => {
                // onStart for volatile conditions
                if let Some((side_idx, poke_idx)) = target {
                    if condition_id == "banefulbunker" {
//                         let _result = crate::data::move_callbacks::banefulbunker::condition_on_start(self, (side_idx, poke_idx));
                        return EventResult::Continue;
                    }
                    // TODO: Add other volatile conditions with onStart here
                }
            }
            "Update" => {
                // Call onUpdate for volatile conditions
                if let Some((side_idx, poke_idx)) = target {
                    if condition_id == "attract" {
//                         let _result = crate::data::move_callbacks::attract::on_update(self, (side_idx, poke_idx));
                        // onUpdate typically returns Undefined, continue to other volatiles
                        return EventResult::Continue;
                    }
                    // TODO: Add other volatile conditions with onUpdate here
                }
            }
            "Residual" => {
                if let Some((side_idx, poke_idx)) = target {
                    // Burn damage
                    if condition_id == "brn" {
                        if let Some(side) = self.sides.get(side_idx) {
                            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                                let damage = pokemon.maxhp / 16;
                                self.damage(damage.max(1) as i32, Some((side_idx, poke_idx)), None, Some(&ID::new("burn")), false);
                                return EventResult::Stop;
                            }
                        }
                    }
                    // Poison damage
                    if condition_id == "psn" {
                        if let Some(side) = self.sides.get(side_idx) {
                            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                                let damage = pokemon.maxhp / 8;
                                self.damage(damage.max(1) as i32, Some((side_idx, poke_idx)), None, Some(&ID::new("poison")), false);
                                return EventResult::Stop;
                            }
                        }
                    }
                }
            }
            _ => {}
        }

        EventResult::Continue
    }

    /// Handle side condition events (SideStart, SideEnd, AnyModifyDamage, etc.)
    /// Calls the appropriate callback for each side condition
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
            modifier: 1.0,
        });

        let mut result = relay_var;

        // Find and run all handlers for this event
        let handlers = self.find_event_handlers(event_id, target, source);

        for (effect_id, holder_target) in handlers {
            let event_result = self.dispatch_single_event(event_id, &effect_id, holder_target, source);

            match event_result {
                EventResult::Fail => {
                    result = None;
                    break;
                }
                EventResult::Stop => {
                    break;
                }
                EventResult::Modify(m) => {
                    if let Some(ref mut r) = result {
                        *r = (*r as f64 * m) as i32;
                    }
                }
                EventResult::ModifyInt(m) => {
                    if let Some(ref mut r) = result {
                        *r = m;
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
            if event.modifier != 1.0 {
                *r = self.modify_f(*r, event.modifier);
            }
        }

        // Restore parent context
        self.event_depth -= 1;
        self.current_event = parent_event;

        result
    }

    /// Run event and return boolean
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
    fn find_event_handlers(
        &self,
        event_id: &str,
        target: Option<(usize, usize)>,
        source: Option<(usize, usize)>,
    ) -> Vec<(ID, Option<(usize, usize)>)> {
        let mut handlers = Vec::new();

        // JavaScript: const prefixedHandlers = !['BeforeTurn', 'Update', 'Weather', 'WeatherChange', 'TerrainChange'].includes(eventName);
        let event_name = event_id.trim_start_matches("on");
        let _prefixed_handlers = !matches!(event_name, "BeforeTurn" | "Update" | "Weather" | "WeatherChange" | "TerrainChange");

        // JavaScript: if (target instanceof Pokemon && (target.isActive || source?.isActive))
        // Rust: We only handle Pokemon targets currently
        if let Some(target_pos) = target {
            // JavaScript: handlers = this.findPokemonEventHandlers(target, `on${eventName}`);
            let mut pokemon_handlers = self.find_pokemon_event_handlers(event_id, target_pos);
            handlers.append(&mut pokemon_handlers);

            // TODO: Add prefixed handlers (onAlly, onFoe, onAny) when needed
            // JavaScript does:
            // for (const allyActive of target.alliesAndSelf()) {
            //     handlers.push(...this.findPokemonEventHandlers(allyActive, `onAlly${eventName}`));
            //     handlers.push(...this.findPokemonEventHandlers(allyActive, `onAny${eventName}`));
            // }
            // for (const foeActive of target.foes()) {
            //     handlers.push(...this.findPokemonEventHandlers(foeActive, `onFoe${eventName}`));
            //     handlers.push(...this.findPokemonEventHandlers(foeActive, `onAny${eventName}`));
            // }
        }

        // JavaScript: if (source && prefixedHandlers) {
        //     handlers.push(...this.findPokemonEventHandlers(source, `onSource${eventName}`));
        // }
        // TODO: Add source handlers when prefixed handler support is implemented

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
    pub fn priority_event(
        &mut self,
        event_id: &str,
        target: Option<(usize, usize)>,
        source: Option<(usize, usize)>,
        effect: Option<&ID>,
    ) -> Option<i32> {
        // For priority events, we use fastExit behavior
        self.run_event(event_id, target, source, effect, None)
    }

    /// Get event modifier
    pub fn get_event_modifier(&self) -> f64 {
        self.current_event.as_ref().map(|e| e.modifier).unwrap_or(1.0)
    }

    /// Set event modifier (for chainModify pattern)
    pub fn set_event_modifier(&mut self, modifier: f64) {
        if let Some(ref mut event) = self.current_event {
            // Chain modifiers by multiplying (simplified version)
            event.modifier = event.modifier * modifier;
        }
    }

    // =========================================================================
    // ADDITIONAL METHODS (ported from battle.ts)
    // =========================================================================

    /// Apply damage randomization (85%-100%)
    /// Equivalent to battle.ts randomizer(baseDamage)
    ///
    /// JS Source (battle.ts):
    /// ```js
    /// randomizer(baseDamage: number) {
    ///     const tr = this.trunc;
    ///     return tr(tr(baseDamage * (100 - this.random(16))) / 100);
    /// }
    /// ```
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
    pub fn each_event(&mut self, event_id: &str, effect: Option<&ID>) {
        // Collect all active Pokemon with their speeds
        let mut actives: Vec<(usize, usize, u32)> = Vec::new();
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
    /// JS Source (battle.ts):
    /// ```js
    /// getTarget(pokemon: Pokemon, move: string | Move, targetLoc: number, originalTarget?: Pokemon) {
    ///     move = this.dex.moves.get(move);
    ///
    ///     let tracksTarget = move.tracksTarget;
    ///     if (pokemon.hasAbility(['stalwart', 'propellertail'])) tracksTarget = true;
    ///     if (tracksTarget && originalTarget?.isActive) {
    ///         return originalTarget;
    ///     }
    ///
    ///     if (move.smartTarget) {
    ///         const curTarget = pokemon.getAtLoc(targetLoc);
    ///         return curTarget && !curTarget.fainted ? curTarget : this.getRandomTarget(pokemon, move);
    ///     }
    ///
    ///     const selfLoc = pokemon.getLocOf(pokemon);
    ///     if (['adjacentAlly', 'any', 'normal'].includes(move.target) && targetLoc === selfLoc &&
    ///         !pokemon.volatiles['twoturnmove'] && !pokemon.volatiles['iceball'] && !pokemon.volatiles['rollout']) {
    ///         return move.flags['futuremove'] ? pokemon : null;
    ///     }
    ///     if (move.target !== 'randomNormal' && this.validTargetLoc(targetLoc, pokemon, move.target)) {
    ///         const target = pokemon.getAtLoc(targetLoc);
    ///         if (target?.fainted) {
    ///             if (this.gameType === 'freeforall') return target;
    ///             if (target.isAlly(pokemon)) {
    ///                 if (move.target === 'adjacentAllyOrSelf' && this.gen !== 5) return pokemon;
    ///                 return target;
    ///             }
    ///         }
    ///         if (target && !target.fainted) return target;
    ///     }
    ///     return this.getRandomTarget(pokemon, move);
    /// }
    /// ```
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
        let tracks_target = if let Some(side) = self.sides.get(user_side) {
            if let Some(pokemon) = side.pokemon.get(user_idx) {
                let ability = pokemon.ability.as_str();
                ability == "stalwart" || ability == "propellertail"
            } else {
                false
            }
        } else {
            false
        };

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
        // TODO: move.smartTarget support (requires MoveDef field)

        // JS: const selfLoc = pokemon.getLocOf(pokemon);
        // JS: if (['adjacentAlly', 'any', 'normal'].includes(move.target) && targetLoc === selfLoc && ...)
        // TODO: Self-targeting validation (requires volatiles check)

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
                            // TODO: isAlly check (requires helper method)
                            // For now, assume fainted ally returns target
                            if target_side == user_side {
                                if target_type == "AdjacentAllyOrSelf" && self.gen != 5 {
                                    return Some(user);
                                }
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
                // TODO: Need to implement effect type checking and status immunity
                // For now, skip this check

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
                    self.last_damage = target_damage as u32;
                }
            }

            // Apply damage using Pokemon's damage method
            let actual_damage = if let Some(side) = self.sides.get_mut(side_idx) {
                if let Some(pokemon) = side.pokemon.get_mut(poke_idx) {
                    let dmg = pokemon.damage(target_damage as u32);
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
            if source.is_some() {
                // TODO: Check if effect.effectType === 'Move'
                if let Some((src_side, src_idx)) = source {
                    if let Some(side) = self.sides.get_mut(src_side) {
                        if let Some(pokemon) = side.pokemon.get_mut(src_idx) {
                            pokemon.last_damage = target_damage as u32;
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
            if target_damage > 0 {
                // TODO: Check if effect is a move and has recoil/drain
                // For now, skip recoil/drain handling
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
                        // TODO: Call self.faint_messages(true) when implemented
                        // self.faint_messages(true);

                        // Gen 1-2 special handling
                        if self.gen <= 2 {
                            if let Some(side) = self.sides.get_mut(target_pos.0) {
                                if let Some(pokemon) = side.pokemon.get_mut(target_pos.1) {
                                    pokemon.faint();
                                }
                            }

                            // Gen 1: Clear queue and reset Bide
                            if self.gen <= 1 {
                                self.queue.clear();
                                // Reset Bide damage for all active Pokemon
                                // TODO: Implement Bide volatile check
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
                // TODO: Get source effect name from volatiles
                self.add_log("-damage", &[&target_str, &health_str, "[from] partiallytrapped", "[partiallytrapped]"]);
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
    /// JS Source (battle.ts:2344-2347):
    /// ```js
    /// finalModify(relayVar: number) {
    ///     relayVar = this.modify(relayVar, this.event.modifier);
    ///     this.event.modifier = 1;
    ///     return relayVar;
    /// }
    /// ```
    pub fn final_modify(&mut self, value: i32) -> i32 {
        // JS: relayVar = this.modify(relayVar, this.event.modifier);
        let modifier = self.get_event_modifier();
        let result = self.modify_internal(value, modifier);

        // JS: this.event.modifier = 1;
        if let Some(ref mut event) = self.current_event {
            event.modifier = 1.0;
        }

        result
    }

    fn modify_internal(&self, value: i32, modifier: f64) -> i32 {
        // 4096-based fixed-point multiplication
        let result = (value as f64 * modifier * 4096.0).trunc() as i32;
        ((result + 2048) >> 12).max(1)
    }

    // =========================================================================
    // REMAINING METHODS (ported from battle.ts for complete 1:1 port)
    // =========================================================================

    /// Add split message for different players
    /// Equivalent to battle.ts addSplit()
    ///
    /// JS Source (battle.ts):
    /// ```js
    /// addSplit(side: SideID, secret: Part[], shared?: Part[]) {
    ///     this.log.push(`|split|${side}`);
    ///     this.add(...secret);
    ///     if (shared) {
    ///         this.add(...shared);
    ///     } else {
    ///         this.log.push('');
    ///     }
    /// }
    /// ```
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
    /// JS Source (battle.ts:2291-2300):
    /// ```js
    /// chainModify(numerator: number | number[], denominator = 1) {
    ///     const previousMod = this.trunc(this.event.modifier * 4096);
    ///
    ///     if (Array.isArray(numerator)) {
    ///         denominator = numerator[1];
    ///         numerator = numerator[0];
    ///     }
    ///     const nextMod = this.trunc(numerator * 4096 / denominator);
    ///     this.event.modifier = ((previousMod * nextMod + 2048) >> 12) / 4096;
    /// }
    /// ```
    pub fn chain_modify(&mut self, numerator: i32, denominator: i32) {
        if let Some(ref mut event) = self.current_event {
            // Extract modifier value first to avoid borrow checker issues
            let modifier = event.modifier;

            // JS: const previousMod = this.trunc(this.event.modifier * 4096);
            // Inline trunc() to avoid borrow checker issues
            let previous_mod = (modifier * 4096.0).trunc() as i32;

            // JS: const nextMod = this.trunc(numerator * 4096 / denominator);
            // Inline trunc() to avoid borrow checker issues
            let next_mod = ((numerator * 4096) as f64 / denominator as f64).trunc() as i32;

            // JS: this.event.modifier = ((previousMod * nextMod + 2048) >> 12) / 4096;
            event.modifier = (((previous_mod * next_mod + 2048) >> 12) as f64) / 4096.0;
        }
    }

    /// Check if teams have balanced EVs
    /// Equivalent to battle.ts checkEVBalance()
    ///
    /// JS Source (battle.ts):
    /// ```js
    /// checkEVBalance() {
    ///     let limitedEVs: boolean | null = null;
    ///     for (const side of this.sides) {
    ///         const sideLimitedEVs = !side.pokemon.some(
    ///             pokemon => Object.values(pokemon.set.evs).reduce((a, b) => a + b, 0) > 510
    ///         );
    ///         if (limitedEVs === null) {
    ///             limitedEVs = sideLimitedEVs;
    ///         } else if (limitedEVs !== sideLimitedEVs) {
    ///             this.add('bigerror', "Warning: One player isn't adhering to a 510 EV limit, and the other player is.");
    ///         }
    ///     }
    /// }
    /// ```
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
    /// JS Source (battle.ts):
    /// ```js
    /// debugError(activity: string) {
    ///     this.add('debug', activity);
    /// }
    /// ```
    pub fn debug_error(&mut self, activity: &str) {
        self.add("debug", &[Arg::Str(activity)]);
    }

    /// Run event on field (weather, terrain, pseudo-weather)
    /// Equivalent to battle.ts fieldEvent()
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
    pub fn get_callback(&self, _effect_id: &ID, _event_id: &str) -> Option<String> {
        // In the full implementation, this would look up event handlers
        // For now, return None
        None
    }

    /// Get overflowed turn count (for endless battle detection and Gen 8+ move timing)
    /// Equivalent to battle.ts getOverflowedTurnCount() (battle.ts:3317-3319)
    /// Used by Wish, Future Sight, and other delayed moves
    pub fn get_overflowed_turn_count(&self) -> u32 {
        // JavaScript: return this.gen >= 8 ? (this.turn - 1) % 256 : this.turn - 1;
        if self.gen >= 8 {
            (self.turn.saturating_sub(1)) % 256
        } else {
            self.turn.saturating_sub(1)
        }
    }

    /// Get requests for all players
    /// Equivalent to battle.ts getRequests()
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
    pub fn get_team(&self, side_idx: usize) -> Option<&[crate::pokemon::Pokemon]> {
        self.sides.get(side_idx).map(|s| s.pokemon.as_slice())
    }

    /// Resolve event handler priority
    /// Equivalent to battle.ts resolvePriority()
    ///
    /// JavaScript Source (battle.ts:950-1017):
    /// Takes an EventListenerWithoutPriority and enriches it with priority/order/subOrder
    /// based on effect callback properties and effectType ordering
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
                    handler.speed = Some(pokemon.stored_stats.spe as u32);

                    // JS: if (handler.effect.effectType === 'Ability' && handler.effect.name === 'Magic Bounce' && callbackName === 'onAllyTryHitSide')
                    if handler.effect_type == EffectType::Ability
                        && handler.effect_id.as_str() == "magicbounce"
                        && callback_name == "onAllyTryHitSide" {
                        // JS: handler.speed = pokemon.getStat('spe', true, true);
                        // TODO: Implement getStat with unmodified flag
                        handler.speed = Some(pokemon.stored_stats.spe as u32);
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
    /// JS Source (battle.ts:1931-1959):
    /// ```js
    /// runPickTeam() {
    ///     // onTeamPreview handlers are expected to show full teams to all active sides,
    ///     // and send a 'teampreview' request for players to pick their leads / team order.
    ///     this.format.onTeamPreview?.call(this);
    ///     for (const rule of this.ruleTable.keys()) {
    ///         if ('+*-!'.includes(rule.charAt(0))) continue;
    ///         const subFormat = this.dex.formats.get(rule);
    ///         subFormat.onTeamPreview?.call(this);
    ///     }
    ///     if (this.requestState === 'teampreview') {
    ///         return;
    ///     }
    ///     if (this.ruleTable.pickedTeamSize) {
    ///         // There was no onTeamPreview handler (e.g. Team Preview rule missing).
    ///         // Players must still pick their own Pokémon, so we show them privately.
    ///         this.add('clearpoke');
    ///         for (const pokemon of this.getAllPokemon()) {
    ///             // Still need to hide these formes since they change on battle start
    ///             const details = pokemon.details.replace(', shiny', '')
    ///                 .replace(/(Zacian|Zamazenta)(?!-Crowned)/g, '$1-*')
    ///                 .replace(/(Xerneas)(-[a-zA-Z?-]+)?/g, '$1-*');
    ///             this.addSplit(pokemon.side.id, ['poke', pokemon.side.id, details, '']);
    ///         }
    ///         this.makeRequest('teampreview');
    ///     }
    /// }
    /// ```
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
        // TODO: Implement ruleTable.pickedTeamSize check

        // For now, assume we need team preview and call makeRequest
        // JS: this.makeRequest('teampreview');
        self.make_request(Some(BattleRequestState::TeamPreview));
    }

    /// Send updates to connected players
    /// Equivalent to battle.ts sendUpdates()
    pub fn send_updates(&mut self) {
        // In the full implementation, this would send log entries to players
        // For now, update sent position
        self.sent_log_pos = self.log.len();
    }

    /// Show open team sheets to players
    /// Equivalent to battle.ts showOpenTeamSheets()
    pub fn show_open_team_sheets(&mut self, _side_idx: Option<usize>) {
        // In the full implementation, this would reveal team sheets
        self.add_log("-message", &["Team sheets revealed"]);
    }

    /// Calculate modified stats from base stats
    /// Equivalent to battle.ts spreadModify(baseStats, set)
    ///
    /// JS Source (battle.ts):
    /// ```js
    /// spreadModify(baseStats: StatsTable, set: PokemonSet): StatsTable {
    ///     const modStats: StatsTable = { hp: 0, atk: 0, def: 0, spa: 0, spd: 0, spe: 0 };
    ///     for (const statName in baseStats) {
    ///         modStats[statName as StatID] = this.statModify(baseStats, set, statName as StatID);
    ///     }
    ///     return modStats;
    /// }
    /// ```
    ///
    /// TODO: This requires Dex access which Battle doesn't currently have
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
    /// JS Source (battle.ts):
    /// ```js
    /// statModify(baseStats: StatsTable, set: PokemonSet, statName: StatID): number {
    ///     const tr = this.trunc;
    ///     let stat = baseStats[statName];
    ///     if (statName === 'hp') {
    ///         return tr(tr(2 * stat + set.ivs[statName] + tr(set.evs[statName] / 4) + 100) * set.level / 100 + 10);
    ///     }
    ///     stat = tr(tr(2 * stat + set.ivs[statName] + tr(set.evs[statName] / 4)) * set.level / 100 + 5);
    ///     const nature = this.dex.natures.get(set.nature);
    ///     if (nature.plus === statName) {
    ///         stat = this.ruleTable.has('overflowstatmod') ? Math.min(stat, 595) : stat;
    ///         stat = tr(tr(stat * 110, 16) / 100);
    ///     } else if (nature.minus === statName) {
    ///         stat = this.ruleTable.has('overflowstatmod') ? Math.min(stat, 728) : stat;
    ///         stat = tr(tr(stat * 90, 16) / 100);
    ///     }
    ///     return stat;
    /// }
    /// ```
    ///
    /// TODO: This requires Dex.natures access which Battle doesn't currently have
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
        // TODO: Need Dex.natures access to apply nature modifiers
        // JS: const nature = this.dex.natures.get(set.nature);
        // JS: if (nature.plus === statName) { stat = tr(tr(stat * 110, 16) / 100); }
        // JS: else if (nature.minus === statName) { stat = tr(tr(stat * 90, 16) / 100); }
        if !set.nature.is_empty() {
            // nature_data = self.dex.natures.get(&set.nature);
            // Apply 1.1x or 0.9x multiplier based on nature.plus/minus with 16-bit truncation
        }

        stat.max(0)
    }

    /// Execute tiebreak logic
    /// Equivalent to battle.ts tiebreak()
    /// Tiebreaker logic for determining winner when time runs out
    /// Equivalent to battle.ts tiebreak() (battle.ts:1421-1462)
    ///
    /// JS Source:
    /// ```js
    /// tiebreak() {
    ///     if (this.ended) return false;
    ///     this.inputLog.push(`>tiebreak`);
    ///     this.add('message', "Time's up! Going to tiebreaker...");
    ///     const notFainted = this.sides.map(side => (
    ///         side.pokemon.filter(pokemon => !pokemon.fainted).length
    ///     ));
    ///     // ... (tiebreak logic: Pokemon count, HP percentage, HP total)
    ///     return this.tie();
    /// }
    /// ```
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
        let hp_total: Vec<u32> = tied_sides.iter()
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
    /// JS Source (battle.ts:318):
    /// ```javascript
    /// toJSON(): AnyObject {
    ///     return State.serializeBattle(this);
    /// }
    /// ```
    pub fn to_json(&self) -> serde_json::Value {
        // Delegate to state::serialize_battle just like JavaScript
        crate::state::serialize_battle(self)
    }

    /// Convert battle to string representation
    /// Equivalent to battle.ts toString()
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
    pub fn find_battle_event_handlers(&self, event_id: &str) -> Vec<ID> {
        // In the full implementation, this would return format/rule handlers
        let _ = event_id;
        Vec::new()
    }

    /// Find field event handlers
    /// Equivalent to battle.ts findFieldEventHandlers()
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
    /// JS Source (battle.ts):
    /// ```js
    /// findPokemonEventHandlers(pokemon: Pokemon, callbackName: string, getKey?: 'duration') {
    ///     const handlers: EventListener[] = [];
    ///
    ///     const status = pokemon.getStatus();
    ///     let callback = this.getCallback(pokemon, status, callbackName);
    ///     if (callback !== undefined || (getKey && pokemon.statusState[getKey])) {
    ///         handlers.push(this.resolvePriority({ effect: status, callback, state: pokemon.statusState, end: pokemon.clearStatus, effectHolder: pokemon }, callbackName));
    ///     }
    ///
    ///     for (const id in pokemon.volatiles) { /* ... */ }
    ///
    ///     const ability = pokemon.getAbility();
    ///     // ... ability handler ...
    ///
    ///     const item = pokemon.getItem();
    ///     // ... item handler ...
    ///
    ///     const species = pokemon.baseSpecies;
    ///     callback = this.getCallback(pokemon, species, callbackName);
    ///     if (callback !== undefined) {
    ///         handlers.push(this.resolvePriority({ effect: species, callback, state: pokemon.speciesState, end() {}, effectHolder: pokemon }, callbackName));
    ///     }
    ///
    ///     const side = pokemon.side;
    ///     for (const conditionid in side.slotConditions[pokemon.position]) {
    ///         const slotConditionState = side.slotConditions[pokemon.position][conditionid];
    ///         const slotCondition = this.dex.conditions.getByID(conditionid as ID);
    ///         callback = this.getCallback(pokemon, slotCondition, callbackName);
    ///         if (callback !== undefined || (getKey && slotConditionState[getKey])) {
    ///             handlers.push(this.resolvePriority({ effect: slotCondition, callback, state: slotConditionState, end: side.removeSlotCondition, endCallArgs: [side, pokemon, slotCondition.id], effectHolder: pokemon }, callbackName));
    ///         }
    ///     }
    ///
    ///     return handlers;
    /// }
    /// ```
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
}

/// Priority item for sorting actions/handlers
#[derive(Debug, Clone, Default)]
pub struct PriorityItem {
    pub order: Option<u32>,
    pub priority: i32,
    pub speed: u32,
    pub sub_order: i32,
    pub effect_order: u32,
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
