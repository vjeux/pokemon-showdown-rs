//! A Pokemon in battle

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

use crate::dex_data::{BoostsTable, Gender, StatsTable, ID};
use crate::event_system::EffectState;

use super::{Attacker, MoveSlot, PokemonSet, TrappedState};

/// A Pokemon in battle
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: Pokemon (sim/pokemon.ts)
/// 167 fields in JavaScript
pub struct Pokemon {
    // Core references (readonly in JavaScript)
    // TODO: These should be references/indices, not owned data
    // pub side: &Side - needs lifetime management
    // pub battle: &Battle - needs lifetime management
    /// Original team builder set (readonly)
    /// JavaScript: readonly set: PokemonSet
    pub set: PokemonSet,

    // Identity (readonly in JavaScript)
    /// Pokemon name (nickname)
    /// JavaScript: readonly name: string
    pub name: String,
    /// Full name with side prefix (readonly, e.g., "p1: Pikachu")
    /// JavaScript: readonly fullname: string
    pub fullname: String,
    // TODO: DELETE - Not in JavaScript Pokemon (Rust uses ID, JavaScript has Species object)
    pub species_id: ID,
    /// Full species object (not just ID)
    /// JavaScript: species: Species
    // TODO: Change from ID to Species struct when available
    // pub species: Species,
    pub base_species: ID,
    /// Full base species object (not just ID)
    /// JavaScript: baseSpecies: Species
    // TODO: Change from ID to Species struct when available
    // pub base_species_obj: Species,
    /// Species effect state
    /// JavaScript: speciesState: EffectState
    pub species_state: EffectState,
    /// Pokemon level (1-100)
    /// JavaScript: level: number
    pub level: u8,
    /// Pokemon gender
    /// JavaScript: gender: GenderName
    pub gender: Gender,
    /// Happiness/Friendship value
    /// JavaScript: happiness: number
    pub happiness: u8,
    /// Pokeball type
    /// JavaScript: pokeball: ID
    pub pokeball: ID,
    /// Dynamax level (0-10, Gen 8+)
    /// JavaScript: dynamaxLevel: number
    pub dynamax_level: u8,
    /// Can Gigantamax?
    /// JavaScript: gigantamax: boolean
    pub gigantamax: bool,

    /// Details string sent to clients
    /// JavaScript: details: string
    pub details: String,

    // Position
    /// Position on field (0-based index)
    /// JavaScript: position: number
    pub position: usize,
    // TODO: DELETE - Not in JavaScript Pokemon (Rust-specific for tracking)
    pub side_index: usize,
    /// Is this Pokemon active on the field?
    /// JavaScript: isActive: boolean
    pub is_active: bool,

    // Stats
    /// Base stored stats (before modifications)
    /// JavaScript: baseStoredStats: StatsExceptHPTable
    pub base_stored_stats: StatsTable,
    /// Stored stats (excluding HP)
    /// JavaScript: storedStats: StatsExceptHPTable
    // TODO: Change to StatsExceptHPTable when that type exists
    pub stored_stats: StatsTable,
    /// Stat boosts/drops (-6 to +6)
    /// JavaScript: boosts: BoostsTable
    pub boosts: BoostsTable,
    /// Maximum HP
    /// JavaScript: maxhp: number
    pub maxhp: i32,
    /// Base maximum HP (before Dynamax)
    /// JavaScript: baseMaxhp: number
    pub base_maxhp: i32,
    /// Current HP
    /// JavaScript: hp: number
    pub hp: i32,

    // Ability
    /// Base ability (from species)
    /// JavaScript: baseAbility: ID
    pub base_ability: ID,
    /// Current ability
    /// JavaScript: ability: ID
    pub ability: ID,
    /// Ability effect state
    /// JavaScript: abilityState: EffectState
    pub ability_state: EffectState,

    // Item
    /// Current held item
    /// JavaScript: item: ID
    pub item: ID,
    /// Item effect state
    /// JavaScript: itemState: EffectState
    pub item_state: EffectState,
    /// Last item held (for Recycle)
    /// JavaScript: lastItem: ID
    pub last_item: ID,
    /// Used item this turn (for Unburden)
    /// JavaScript: usedItemThisTurn: boolean
    pub used_item_this_turn: bool,
    /// Ate a berry this turn
    /// JavaScript: ateBerry: boolean
    pub ate_berry: bool,
    /// Item was knocked off
    /// JavaScript: itemKnockedOff: boolean
    pub item_knocked_off: bool,

    // Types
    /// Current types
    /// JavaScript: types: string[]
    pub types: Vec<String>,
    /// Added type (from Forest's Curse or Trick-or-Treat)
    /// JavaScript: addedType: string
    pub added_type: String,
    /// Base types (before transformations)
    /// JavaScript: baseTypes: string[]
    pub base_types: Vec<String>,
    /// Known type flag (for type reveal mechanics)
    /// JavaScript: knownType: boolean
    pub known_type: bool,
    /// Apparent type (for type reveal mechanics)
    /// JavaScript: apparentType: string
    pub apparent_type: String,

    // Tera
    /// Tera type
    /// JavaScript: teraType?: string
    pub tera_type: Option<String>,
    /// Terastallized type
    /// JavaScript: terastallized?: string
    pub terastallized: Option<String>,
    /// Can terastallize (tera type if available)
    /// JavaScript: canTerastallize?: string
    pub can_terastallize: Option<String>,

    // Move slots
    /// Base move slots (before transformations)
    /// JavaScript: baseMoveSlots: MoveSlot[]
    pub base_move_slots: Vec<MoveSlot>,
    /// Current move slots
    /// JavaScript: moveSlots: MoveSlot[]
    pub move_slots: Vec<MoveSlot>,

    // Hidden Power type and power
    /// Hidden Power type
    /// JavaScript: hpType: string
    pub hp_type: String,
    /// Hidden Power base power
    /// JavaScript: hpPower: number
    pub hp_power: u8,
    /// Base Hidden Power type
    /// JavaScript: baseHpType: string
    pub base_hp_type: String,
    /// Base Hidden Power power
    /// JavaScript: baseHpPower: number
    pub base_hp_power: u8,

    // Status
    /// Status condition (par, slp, etc.)
    /// JavaScript: status: ID
    pub status: ID,
    /// Status effect state
    /// JavaScript: statusState: EffectState
    pub status_state: EffectState,
    /// Whether to show cure message
    /// JavaScript: showCure?: boolean
    pub show_cure: Option<bool>,
    /// Volatile status conditions
    /// JavaScript: volatiles: { [id: string]: EffectState }
    pub volatiles: HashMap<ID, EffectState>,

    // Battle state
    /// Has this Pokemon fainted?
    /// JavaScript: fainted: boolean
    pub fainted: bool,
    /// Is this Pokemon queued to faint?
    /// JavaScript: faintQueued: boolean
    pub faint_queued: bool,
    /// Has this Pokemon transformed?
    /// JavaScript: transformed: boolean
    pub transformed: bool,
    /// Illusion Pokemon index
    /// JavaScript: illusion?: Pokemon
    /// TODO: Rust uses index instead of Pokemon reference
    pub illusion: Option<usize>, // Index of pokemon providing illusion
    /// Forme regression flag (for Zacian/Zamazenta)
    /// JavaScript: formeRegression: boolean
    pub forme_regression: bool,

    // Flags
    /// Trapped state (None, Visible, or Hidden)
    /// JavaScript: trapped: false | true | 'hidden'
    /// TODO: Rust uses enum instead of the union type
    pub trapped: TrappedState,
    /// Maybe trapped (need to check abilities)
    /// JavaScript: maybeTrapped: boolean
    pub maybe_trapped: bool,
    /// Maybe disabled (need to check Imprison)
    /// JavaScript: maybeDisabled: boolean
    pub maybe_disabled: bool,
    /// Maybe locked by choice item
    /// JavaScript: maybeLocked: boolean | null
    pub maybe_locked: Option<bool>, // Choice items may lock next turn
    /// Switch flag (ID of move that forced switch, or None)
    /// JavaScript: switchFlag: ID | boolean
    /// In JavaScript: string (move ID) or false
    /// In Rust: Some(String) for move ID, None for false
    pub switch_flag: Option<String>,
    /// Force switch flag
    /// JavaScript: forceSwitchFlag: boolean
    pub force_switch_flag: bool,
    /// Newly switched in this turn
    /// JavaScript: newlySwitched: boolean
    pub newly_switched: bool,
    /// Being called back by U-turn/Baton Pass
    /// JavaScript: beingCalledBack: boolean
    pub being_called_back: bool,
    /// Dragged in by Red Card/Roar/etc
    /// JavaScript: draggedIn: number | null
    pub dragged_in: Option<i32>,
    /// Skip BeforeSwitchOut event flag
    /// JavaScript: skipBeforeSwitchOutEventFlag: boolean
    pub skip_before_switch_out_event_flag: bool,
    /// Stats were raised this turn
    /// JavaScript: statsRaisedThisTurn: boolean
    pub stats_raised_this_turn: bool,
    /// Stats were lowered this turn
    /// JavaScript: statsLoweredThisTurn: boolean
    pub stats_lowered_this_turn: bool,
    /// Substitute fainted (Gen 1 specific)
    /// JavaScript: subFainted: boolean | null
    pub sub_fainted: Option<bool>,

    // Ability-specific flags
    /// Intrepid Sword boost applied
    /// JavaScript: swordBoost: boolean
    pub sword_boost: bool, // Intrepid Sword
    /// Dauntless Shield boost flag
    /// JavaScript: shieldBoost: boolean
    pub shield_boost: bool,
    /// Truant ability turn tracking
    /// JavaScript: truantTurn: boolean
    pub truant_turn: bool,
    /// Bond ability triggered flag (Greninja)
    /// JavaScript: bondTriggered: boolean
    pub bond_triggered: bool,
    /// Hero message displayed flag (Palafin)
    /// JavaScript: heroMessageDisplayed: boolean
    pub hero_message_displayed: bool,
    /// Syrup Bomb triggered flag
    /// JavaScript: syrupTriggered: boolean
    pub syrup_triggered: bool,

    // Turn state
    /// Last move used
    /// JavaScript: lastMove: ActiveMove | null
    // TODO: Change from Option<ID> to Option<ActiveMove>
    pub last_move: Option<ID>,
    /// Last move for Encore tracking (Gen 2+)
    /// JavaScript: lastMoveEncore?: ActiveMove | null
    // TODO: Change from Option<ID> to Option<ActiveMove>
    pub last_move_encore: Option<ID>, // Gen 2 only - for Encore tracking
    /// Last move used (for repetition tracking)
    /// JavaScript: lastMoveUsed: ActiveMove | null
    // TODO: Change from Option<ID> to Option<ActiveMove>
    pub last_move_used: Option<ID>,
    /// Last move target location
    /// JavaScript: lastMoveTargetLoc?: number
    pub last_move_target_loc: Option<i8>,
    /// Move used this turn
    /// JavaScript: moveThisTurn: string | boolean
    // TODO: Change from Option<ID> to support both string and boolean
    pub move_this_turn: Option<ID>,
    /// Move result this turn
    /// JavaScript: moveThisTurnResult?: boolean
    pub move_this_turn_result: Option<bool>,
    /// Move result last turn
    /// JavaScript: moveLastTurnResult?: boolean
    pub move_last_turn_result: Option<bool>,
    /// Hurt this turn (damage taken)
    /// JavaScript: hurtThisTurn?: number
    pub hurt_this_turn: Option<i32>,
    /// Last damage dealt
    /// JavaScript: lastDamage: number
    pub last_damage: i32,
    /// Times attacked (for multi-hit tracking)
    /// JavaScript: timesAttacked: number
    pub times_attacked: i32,

    // Counters
    /// Turns active on field
    /// JavaScript: activeTurns: number
    pub active_turns: i32,
    /// Move actions this activation
    /// JavaScript: activeMoveActions: number
    pub active_move_actions: i32,
    /// Previously switched in count
    /// JavaScript: previouslySwitchedIn: number
    pub previously_switched_in: i32,
    /// Has been started (sent out)
    /// JavaScript: isStarted: boolean
    pub is_started: bool,
    /// During a move execution
    /// JavaScript: duringMove: boolean
    pub during_move: bool,

    // Attack tracking for revenge/payback/etc mechanics
    /// Pokemon that attacked this Pokemon
    /// JavaScript: attackedBy: Attacker[]
    pub attacked_by: Vec<Attacker>,

    // Calculated values
    /// Weight in hectograms
    /// JavaScript: weightkg: number (but stored as hg)
    pub weight_hg: i32,
    /// Speed stat
    /// JavaScript: speed: number
    pub speed: i32,

    // Mega/Dynamax state
    /// Can Mega Evolve (item name)
    /// JavaScript: canMegaEvo?: string
    pub can_mega_evo: Option<String>,
    /// Can Mega Evolve into X forme
    /// JavaScript: canMegaEvoX: string | false | null | undefined
    pub can_mega_evo_x: Option<String>,
    /// Can Mega Evolve into Y forme
    /// JavaScript: canMegaEvoY: string | false | null | undefined
    pub can_mega_evo_y: Option<String>,
    /// Can Ultra Burst
    /// JavaScript: canUltraBurst?: string
    pub can_ultra_burst: Option<String>,
    /// Can Gigantamax (G-Max move)
    /// JavaScript: canGigantamax?: string
    pub can_gigantamax: Option<String>,

    // Stellar boost tracking (Gen 9)
    /// Types that have used Stellar boost
    /// JavaScript: stellarBoostedTypes: string[]
    pub stellar_boosted_types: Vec<String>,

    // Staleness tracking for endless battle clause
    /// Staleness state
    /// JavaScript: staleness?: string
    pub staleness: Option<String>,
    /// Pending staleness
    /// JavaScript: pendingStaleness?: string
    pub pending_staleness: Option<String>,
    /// Volatile staleness
    /// JavaScript: volatileStaleness?: string
    pub volatile_staleness: Option<String>,

    /// Modified stats (for stat modifications)
    /// JavaScript: modifiedStats?: StatsExceptHPTable
    // TODO: Change to StatsExceptHPTable when that type exists
    pub modified_stats: Option<StatsTable>,
}

impl Pokemon {
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
