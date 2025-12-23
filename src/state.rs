//! Battle State Serialization
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module handles battle state serialization and deserialization,
//! enabling battle saving, loading, and replay functionality.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::dex_data::{ID, EffectState};
use crate::prng::PRNGSeed;

/// Serializable battle state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleState {
    /// Format ID
    pub format_id: ID,
    /// PRNG seed at start of battle
    pub seed: PRNGSeed,
    /// Current turn number
    pub turn: u32,
    /// Active turn (for mid-turn states)
    pub active_turn: bool,
    /// Battle ended
    pub ended: bool,
    /// Winner (if ended)
    pub winner: Option<String>,
    /// Side states
    pub sides: [SideState; 2],
    /// Field state
    pub field: FieldState,
    /// Input log (all player choices)
    pub input_log: Vec<String>,
    /// Battle log (all output messages)
    pub log: Vec<String>,
}

/// Serializable side state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SideState {
    /// Side ID (p1, p2)
    pub id: String,
    /// Player name
    pub name: String,
    /// All Pokemon on this side
    pub pokemon: Vec<PokemonState>,
    /// Active Pokemon indices
    pub active: Vec<Option<usize>>,
    /// Side conditions with layer counts
    pub side_conditions: HashMap<ID, u8>,
    /// Slot conditions (Wish, etc.)
    pub slot_conditions: HashMap<usize, HashMap<ID, EffectState>>,
}

/// Serializable Pokemon state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonState {
    /// Species name
    pub species: String,
    /// Nickname (if different from species)
    pub name: String,
    /// Level
    pub level: u8,
    /// Gender
    pub gender: String,
    /// Current HP
    pub hp: u32,
    /// Maximum HP
    pub maxhp: u32,
    /// Current ability ID
    pub ability: ID,
    /// Base ability ID (before Transform, etc.)
    pub base_ability: ID,
    /// Current item ID (empty if no item)
    pub item: ID,
    /// Original item (before Knock Off, etc.)
    pub original_item: ID,
    /// Current types
    pub types: Vec<String>,
    /// Base types (original)
    pub base_types: Vec<String>,
    /// Move slots
    pub moves: Vec<MoveSlotState>,
    /// Current stats (after modifications)
    pub stats: StatsState,
    /// Base stats (before modifications)
    pub base_stats: StatsState,
    /// Stat boosts
    pub boosts: BoostsState,
    /// Non-volatile status
    pub status: Option<String>,
    /// Status data (sleep turns, toxic counter, etc.)
    pub status_state: Option<EffectState>,
    /// Volatile conditions
    pub volatiles: HashMap<ID, EffectState>,
    /// Is this Pokemon active?
    pub is_active: bool,
    /// Is this Pokemon fainted?
    pub fainted: bool,
    /// Position in team (0-5)
    pub position: usize,
    /// Last move used
    pub last_move: Option<ID>,
    /// Last move turn
    pub last_move_turn: Option<u32>,
    /// Transform target (if transformed)
    pub transformed: bool,
}

/// Serializable move slot state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveSlotState {
    /// Move ID
    pub id: ID,
    /// Current PP
    pub pp: u8,
    /// Maximum PP
    pub maxpp: u8,
    /// Is this move disabled?
    pub disabled: bool,
    /// Used this turn?
    pub used: bool,
}

/// Serializable stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsState {
    pub hp: u32,
    pub atk: u32,
    pub def: u32,
    pub spa: u32,
    pub spd: u32,
    pub spe: u32,
}

/// Serializable boosts
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BoostsState {
    pub atk: i8,
    pub def: i8,
    pub spa: i8,
    pub spd: i8,
    pub spe: i8,
    pub accuracy: i8,
    pub evasion: i8,
}

/// Serializable field state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldState {
    /// Current weather ID (empty = no weather)
    pub weather: ID,
    /// Weather state
    pub weather_state: Option<EffectState>,
    /// Current terrain ID (empty = no terrain)
    pub terrain: ID,
    /// Terrain state
    pub terrain_state: Option<EffectState>,
    /// Pseudo-weather conditions
    pub pseudo_weather: HashMap<ID, EffectState>,
}

/// Input log entry for replay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputLogEntry {
    /// Turn number
    pub turn: u32,
    /// Side index (0 or 1)
    pub side: usize,
    /// Choice string (e.g., "move 1", "switch 2")
    pub choice: String,
}

/// Replay data for deterministic replay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayData {
    /// Format ID
    pub format_id: ID,
    /// Initial PRNG seed
    pub seed: PRNGSeed,
    /// Player 1 team (in import format)
    pub p1_team: String,
    /// Player 2 team (in import format)
    pub p2_team: String,
    /// All input choices in order
    pub inputs: Vec<InputLogEntry>,
}

impl Default for StatsState {
    fn default() -> Self {
        Self {
            hp: 0,
            atk: 0,
            def: 0,
            spa: 0,
            spd: 0,
            spe: 0,
        }
    }
}

impl Default for FieldState {
    fn default() -> Self {
        Self {
            weather: ID::empty(),
            weather_state: None,
            terrain: ID::empty(),
            terrain_state: None,
            pseudo_weather: HashMap::new(),
        }
    }
}

impl BattleState {
    /// Create a new empty battle state
    pub fn new(format_id: ID, seed: PRNGSeed) -> Self {
        Self {
            format_id,
            seed,
            turn: 0,
            active_turn: false,
            ended: false,
            winner: None,
            sides: [
                SideState {
                    id: "p1".to_string(),
                    name: String::new(),
                    pokemon: Vec::new(),
                    active: Vec::new(),
                    side_conditions: HashMap::new(),
                    slot_conditions: HashMap::new(),
                },
                SideState {
                    id: "p2".to_string(),
                    name: String::new(),
                    pokemon: Vec::new(),
                    active: Vec::new(),
                    side_conditions: HashMap::new(),
                    slot_conditions: HashMap::new(),
                },
            ],
            field: FieldState::default(),
            input_log: Vec::new(),
            log: Vec::new(),
        }
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Serialize to pretty JSON string
    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

impl ReplayData {
    /// Create new replay data
    pub fn new(format_id: ID, seed: PRNGSeed, p1_team: String, p2_team: String) -> Self {
        Self {
            format_id,
            seed,
            p1_team,
            p2_team,
            inputs: Vec::new(),
        }
    }

    /// Add an input to the replay
    pub fn add_input(&mut self, turn: u32, side: usize, choice: String) {
        self.inputs.push(InputLogEntry { turn, side, choice });
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Deserialize from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battle_state_serialization() {
        let state = BattleState::new(ID::new("gen9ou"), PRNGSeed::Gen5([0, 0, 0, 0]));
        let json = state.to_json().unwrap();
        let restored = BattleState::from_json(&json).unwrap();
        assert_eq!(restored.format_id.as_str(), "gen9ou");
    }

    #[test]
    fn test_boosts_state() {
        let boosts = BoostsState {
            atk: 2,
            def: -1,
            spa: 0,
            spd: 0,
            spe: 1,
            accuracy: 0,
            evasion: 0,
        };
        let json = serde_json::to_string(&boosts).unwrap();
        let restored: BoostsState = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.atk, 2);
        assert_eq!(restored.def, -1);
    }

    #[test]
    fn test_replay_data() {
        let mut replay = ReplayData::new(
            ID::new("gen9ou"),
            PRNGSeed::Gen5([0, 0, 0, 0]),
            "Pikachu @ Light Ball".to_string(),
            "Charizard @ Heavy-Duty Boots".to_string(),
        );
        replay.add_input(1, 0, "move 1".to_string());
        replay.add_input(1, 1, "move 2".to_string());

        let json = replay.to_json().unwrap();
        let restored = ReplayData::from_json(&json).unwrap();
        assert_eq!(restored.inputs.len(), 2);
    }
}
