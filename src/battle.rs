//! Simulator Battle
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This file is where the battle simulation itself happens.
//! The most important part of the simulation is the event system.

use std::collections::HashSet;
use serde::{Deserialize, Serialize};

use crate::dex_data::{ID, GameType, SideID, EffectState};
use crate::field::Field;
use crate::battle_queue::BattleQueue;
use crate::pokemon::PokemonSet;
use crate::side::{Side, RequestState};
use crate::prng::{PRNG, PRNGSeed};

/// Battle options
#[derive(Debug, Clone, Default)]
pub struct BattleOptions {
    pub format_id: ID,
    pub seed: Option<PRNGSeed>,
    pub rated: bool,
    pub debug: bool,
    pub strict_choices: bool,
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Battle {
    /// Format ID
    pub format_id: ID,
    /// Game type (singles, doubles, etc.)
    pub game_type: GameType,
    /// Generation
    pub gen: u8,
    /// Number of active pokemon per half-field
    pub active_per_half: usize,

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
    /// Winner (side ID string)
    pub winner: Option<String>,

    /// Last move used in battle
    pub last_move: Option<ID>,
    /// Last damage dealt (for Counter in Gen 1)
    pub last_damage: u32,

    /// Effect order counter
    pub effect_order: u32,

    /// Debug mode
    pub debug_mode: bool,
    /// Rated match
    pub rated: bool,
    /// Strict choices (errors on invalid choices)
    pub strict_choices: bool,

    /// Hints shown to players
    pub hints: HashSet<String>,
}

impl Battle {
    /// Create a new battle
    pub fn new(options: BattleOptions) -> Self {
        let seed = options.seed.clone().unwrap_or_else(PRNG::generate_seed);
        let prng = PRNG::new(Some(seed.clone()));

        // Clone format_id before moving it into the struct
        let format_id_str = options.format_id.as_str().to_string();

        // Determine game settings from format
        let game_type = GameType::Singles; // Default, would be parsed from format
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
            game_type,
            gen,
            active_per_half,
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
            winner: None,
            last_move: None,
            last_damage: 0,
            effect_order: 0,
            debug_mode: options.debug,
            rated: options.rated,
            strict_choices: options.strict_choices,
            hints: HashSet::new(),
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

    /// Add a player to the battle
    pub fn set_player(&mut self, side_id: SideID, options: PlayerOptions) {
        let n = side_id.index();

        // Ensure sides vector is large enough
        while self.sides.len() <= n {
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

        let side = Side::new(side_id, n, options.name, options.team, self.active_per_half);
        self.sides[n] = side;

        self.input_log.push(format!(">player {} {{\"name\":\"{}\"}}",
            side_id.to_str(), self.sides[n].name));

        // Check if we can start
        self.check_start();
    }

    /// Check if we can start the battle
    fn check_start(&mut self) {
        // Need at least 2 players with Pokemon
        let ready_count = self.sides.iter()
            .filter(|s| !s.name.is_empty() && !s.pokemon.is_empty())
            .count();

        let needed = match self.game_type {
            GameType::Multi | GameType::FreeForAll => 4,
            _ => 2,
        };

        if ready_count >= needed && !self.started {
            self.start();
        }
    }

    /// Start the battle
    pub fn start(&mut self) {
        if self.started {
            return;
        }
        self.started = true;

        // Set up foe references
        if self.sides.len() >= 2 {
            self.sides[0].foe_index = Some(1);
            self.sides[1].foe_index = Some(0);
            if self.sides.len() >= 4 {
                self.sides[0].ally_index = Some(2);
                self.sides[1].ally_index = Some(3);
                self.sides[2].foe_index = Some(1);
                self.sides[3].foe_index = Some(0);
                self.sides[2].ally_index = Some(0);
                self.sides[3].ally_index = Some(1);
            }
        }

        // Log player info - collect first to avoid borrow conflict
        let side_info: Vec<_> = self.sides.iter()
            .map(|s| (s.id_str(), s.name.clone()))
            .collect();
        for (id_str, name) in side_info {
            self.add_log("player", &[id_str, &name, "", ""]);
        }

        // Team preview or direct start
        self.add_log("teampreview", &[]);
        self.request_state = BattleRequestState::TeamPreview;
        for side in &mut self.sides {
            side.request_state = RequestState::TeamPreview;
        }

        // For now, skip team preview and go straight to battle
        // In a full implementation, we'd wait for team choices
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

        for (side_idx, slot, pokemon_idx) in switch_ops {
            self.switch_in(side_idx, slot, pokemon_idx);
        }

        self.request_state = BattleRequestState::Move;
        for side in &mut self.sides {
            side.request_state = RequestState::Move;
        }
    }

    /// Switch a Pokemon in
    pub fn switch_in(&mut self, side_index: usize, slot: usize, pokemon_index: usize) {
        if let Some(side) = self.sides.get_mut(side_index) {
            side.switch_in(slot, pokemon_index);
            if let Some(pokemon) = side.get_active(slot) {
                let details = pokemon.details();
                let _fullname = pokemon.fullname(side.id_str());
                let hp = format!("{}/{}", pokemon.hp, pokemon.maxhp);
                self.log.push(format!("|switch|{}: {}|{}|{}",
                    side.id_str(), pokemon.name, details, hp));
            }
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
    pub fn get_all_active(&self) -> Vec<(usize, usize, &crate::pokemon::Pokemon)> {
        let mut result = Vec::new();
        for (side_idx, side) in self.sides.iter().enumerate() {
            for (slot, opt_idx) in side.active.iter().enumerate() {
                if let Some(poke_idx) = opt_idx {
                    if let Some(pokemon) = side.pokemon.get(*poke_idx) {
                        if !pokemon.is_fainted() {
                            result.push((side_idx, slot, pokemon));
                        }
                    }
                }
            }
        }
        result
    }

    /// Check if battle is over
    pub fn check_win(&mut self) -> Option<SideID> {
        let mut alive_sides = Vec::new();
        for (i, side) in self.sides.iter().enumerate() {
            if !side.has_lost() {
                alive_sides.push(i);
            }
        }

        if alive_sides.len() == 1 {
            let winner_idx = alive_sides[0];
            self.ended = true;
            self.winner = Some(self.sides[winner_idx].id_str().to_string());
            return match winner_idx {
                0 => Some(SideID::P1),
                1 => Some(SideID::P2),
                2 => Some(SideID::P3),
                _ => Some(SideID::P4),
            };
        } else if alive_sides.is_empty() {
            // Tie
            self.ended = true;
            self.winner = None;
        }

        None
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

        // Parse and validate choice
        let parts: Vec<&str> = choice.split_whitespace().collect();
        if parts.is_empty() {
            return Err("Empty choice".to_string());
        }

        match parts[0] {
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
                    return Err("Switch choice requires Pokemon number".to_string());
                }
                // Would validate and add to queue here
                Ok(())
            }
            "team" => {
                // Parse team order choice (for team preview)
                Ok(())
            }
            "pass" => {
                Ok(())
            }
            _ => Err(format!("Unknown choice type: {}", parts[0])),
        }
    }

    /// Get the battle log as a string
    pub fn get_log(&self) -> String {
        self.log.join("\n")
    }
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
            }),
            p2: Some(PlayerOptions {
                name: "Bob".to_string(),
                team: create_test_team(),
                avatar: None,
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
            }),
            p2: Some(PlayerOptions {
                name: "Bob".to_string(),
                team: create_test_team(),
                avatar: None,
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
