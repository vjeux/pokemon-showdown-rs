//! Simulator Battle
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This file is where the battle simulation itself happens.
//! The most important part of the simulation is the event system.

use std::collections::HashSet;
use serde::{Deserialize, Serialize};

use crate::dex_data::{ID, GameType, SideID, EffectState};
use crate::field::{Field, get_weather_type_modifier, get_terrain_damage_modifier, get_weather_damage_fraction, get_grassy_terrain_heal};
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
                            let priority = 0i8; // Would look up move priority from dex
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

        // Sort by priority (desc), then speed (desc), then random
        let tie_breaker = self.random(2) == 0;
        actions.sort_by(|a, b| {
            let priority_cmp = b.3.cmp(&a.3); // Higher priority first
            if priority_cmp != std::cmp::Ordering::Equal {
                return priority_cmp;
            }
            let speed_cmp = b.4.cmp(&a.4); // Higher speed first
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
                    }
                }
                _ => {}
            }
        }

        // End of turn
        self.run_residual();

        // Check for fainted Pokemon
        self.faint_messages();

        // Check win condition
        if self.check_win().is_some() {
            return;
        }

        // Start next turn
        self.next_turn();
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

        // Set last move
        self.sides[attacker_side].pokemon[attacker_idx].move_this_turn = Some(move_id.clone());
        self.sides[attacker_side].pokemon[attacker_idx].last_move_used = Some(move_id.clone());

        // Check if target is valid
        if target_side >= self.sides.len() || target_idx >= self.sides[target_side].pokemon.len() {
            return;
        }

        let target_fainted = self.sides[target_side].pokemon[target_idx].is_fainted();
        if target_fainted {
            self.add_log("-notarget", &[]);
            return;
        }

        // For now, apply basic damage based on move ID
        // In a full implementation, we'd look up move data from the Dex
        let damage = self.calculate_move_damage(attacker_side, attacker_idx, target_side, target_idx, move_id);

        if damage > 0 {
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

            // Apply burn damage reduction if physical move and attacker is burned
            // (this is already factored into calculate_move_damage)
        }

        // Apply secondary effects based on move
        self.apply_move_secondary(attacker_side, attacker_idx, target_side, target_idx, move_id);
    }

    /// Calculate damage for a move (basic implementation)
    fn calculate_move_damage(&mut self, attacker_side: usize, attacker_idx: usize, target_side: usize, target_idx: usize, move_id: &ID) -> u32 {
        // Get base power from common moves (simplified)
        let (base_power, category, move_type): (u32, &str, &str) = match move_id.as_str() {
            "thunderbolt" => (90, "Special", "Electric"),
            "flamethrower" => (90, "Special", "Fire"),
            "icebeam" => (90, "Special", "Ice"),
            "surf" => (90, "Special", "Water"),
            "psychic" => (90, "Special", "Psychic"),
            "earthquake" => (100, "Physical", "Ground"),
            "tackle" => (40, "Physical", "Normal"),
            "quickattack" => (40, "Physical", "Normal"),
            "slash" => (70, "Physical", "Normal"),
            "bodyslam" => (85, "Physical", "Normal"),
            "hyperbeam" => (150, "Special", "Normal"),
            "dragonclaw" => (80, "Physical", "Dragon"),
            "crunch" => (80, "Physical", "Dark"),
            "shadowball" => (80, "Special", "Ghost"),
            "sludgebomb" => (90, "Special", "Poison"),
            "closecombat" => (120, "Physical", "Fighting"),
            "stoneedge" => (100, "Physical", "Rock"),
            "ironhead" => (80, "Physical", "Steel"),
            "energyball" => (90, "Special", "Grass"),
            "scald" => (80, "Special", "Water"),
            "uturn" => (70, "Physical", "Bug"),
            "voltswitch" => (70, "Special", "Electric"),
            "knockoff" => (65, "Physical", "Dark"),
            "bravebird" => (120, "Physical", "Flying"),
            "flareblitz" => (120, "Physical", "Fire"),
            "woodhammer" => (120, "Physical", "Grass"),
            "headsmash" => (150, "Physical", "Rock"),
            "doubleedge" => (120, "Physical", "Normal"),
            "takedown" => (90, "Physical", "Normal"),
            "wildcharge" => (90, "Physical", "Electric"),
            "thunderwave" | "willowisp" | "toxic" | "spore" | "sleeppowder" | "bulkup" | "swordsdance" | "nastyplot" | "calmmind" | "agility" |
            "stealthrock" | "spikes" | "toxicspikes" | "stickyweb" | "defog" | "rapidspin" |
            "protect" | "detect" | "substitute" | "recover" | "roost" | "softboiled" | "moonlight" | "synthesis" | "morningsun" |
            "wish" | "healbell" | "aromatherapy" | "haze" | "whirlwind" | "roar" | "dragontail" | "circlethrow" |
            "taunt" | "encore" | "disable" | "trick" | "switcheroo" => {
                return 0; // Status/hazard/utility moves - no damage
            }
            _ => (50, "Physical", "Normal"), // Default for unknown moves
        };

        if base_power == 0 {
            return 0;
        }

        // Extract all needed data from Pokemon before any mutable operations
        let (attack_stat, defense_stat, atk_boost, def_boost, level, attacker_types, attacker_status, defender_types, defender_name) = {
            let attacker = &self.sides[attacker_side].pokemon[attacker_idx];
            let defender = &self.sides[target_side].pokemon[target_idx];

            let (attack_stat, defense_stat) = if category == "Physical" {
                (attacker.stored_stats.atk as u32, defender.stored_stats.def as u32)
            } else {
                (attacker.stored_stats.spa as u32, defender.stored_stats.spd as u32)
            };

            let (atk_boost, def_boost) = if category == "Physical" {
                (attacker.boosts.atk, defender.boosts.def)
            } else {
                (attacker.boosts.spa, defender.boosts.spd)
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
            )
        };

        // Check type immunity
        let type_effectiveness = self.get_type_effectiveness(move_type, &defender_types);
        if type_effectiveness == 0.0 {
            let side_id = self.sides[target_side].id_str();
            let target_name = format!("{}: {}", side_id, defender_name);
            self.add_log("-immune", &[&target_name]);
            return 0;
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
        let damage = if has_stab {
            (damage as f64 * 1.5) as u32
        } else {
            damage
        };

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
        let weather_mod = get_weather_type_modifier(weather, move_type);
        let damage = (damage as f64 * weather_mod) as u32;

        // Terrain type modifier (grounded Pokemon only)
        // Check if attacker is grounded (simplified - not Flying type and no Levitate)
        let attacker_grounded = !attacker_types.iter().any(|t| t.to_lowercase() == "flying");
        let terrain = self.field.terrain.as_str();
        let terrain_mod = get_terrain_damage_modifier(terrain, move_type, attacker_grounded);
        let damage = (damage as f64 * terrain_mod) as u32;

        // Log effectiveness
        if type_effectiveness > 1.0 {
            self.add_log("-supereffective", &[]);
        } else if type_effectiveness < 1.0 && type_effectiveness > 0.0 {
            self.add_log("-resisted", &[]);
        }

        // Critical hit check (1/24 chance in Gen 7+)
        let crit_roll = self.random(24);
        if crit_roll == 0 {
            self.add_log("-crit", &[]);
            return (damage as f64 * 1.5) as u32;
        }

        damage.max(1)
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

    /// Apply secondary effects from a move
    fn apply_move_secondary(&mut self, attacker_side: usize, _attacker_idx: usize, target_side: usize, target_idx: usize, move_id: &ID) {
        match move_id.as_str() {
            "thunderbolt" | "thunder" => {
                // 10% chance to paralyze
                if self.random(10) == 0 {
                    self.apply_status(target_side, target_idx, "par");
                }
            }
            "flamethrower" | "fireblast" => {
                // 10% chance to burn
                if self.random(10) == 0 {
                    self.apply_status(target_side, target_idx, "brn");
                }
            }
            "icebeam" | "blizzard" => {
                // 10% chance to freeze
                if self.random(10) == 0 {
                    self.apply_status(target_side, target_idx, "frz");
                }
            }
            "scald" => {
                // 30% chance to burn
                if self.random(10) < 3 {
                    self.apply_status(target_side, target_idx, "brn");
                }
            }
            "bodyslam" => {
                // 30% chance to paralyze
                if self.random(10) < 3 {
                    self.apply_status(target_side, target_idx, "par");
                }
            }
            "thunderwave" => {
                self.apply_status(target_side, target_idx, "par");
            }
            "willowisp" => {
                self.apply_status(target_side, target_idx, "brn");
            }
            "toxic" => {
                self.apply_status(target_side, target_idx, "tox");
            }
            "spore" | "sleeppowder" => {
                self.apply_status(target_side, target_idx, "slp");
            }
            "swordsdance" => {
                self.apply_boost(attacker_side, target_idx, "atk", 2);
            }
            "nastyplot" => {
                self.apply_boost(attacker_side, target_idx, "spa", 2);
            }
            "calmmind" => {
                self.apply_boost(attacker_side, target_idx, "spa", 1);
                self.apply_boost(attacker_side, target_idx, "spd", 1);
            }
            "bulkup" => {
                self.apply_boost(attacker_side, target_idx, "atk", 1);
                self.apply_boost(attacker_side, target_idx, "def", 1);
            }
            "agility" => {
                self.apply_boost(attacker_side, target_idx, "spe", 2);
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
            _ => {}
        }
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
            }
        }

        // Decrement field condition durations
        let expired = self.field.decrement_durations();
        for effect_id in expired {
            self.add_log("-fieldend", &[effect_id.as_str()]);
        }
    }

    /// Process faint messages
    fn faint_messages(&mut self) {
        for side_idx in 0..self.sides.len() {
            let mut slots_to_faint = Vec::new();

            for slot in 0..self.sides[side_idx].active.len() {
                if let Some(poke_idx) = self.sides[side_idx].active[slot] {
                    if self.sides[side_idx].pokemon[poke_idx].hp == 0 {
                        slots_to_faint.push((slot, poke_idx));
                    }
                }
            }

            for (slot, poke_idx) in slots_to_faint {
                let name = {
                    let side_id = self.sides[side_idx].id_str();
                    let pokemon = &self.sides[side_idx].pokemon[poke_idx];
                    format!("{}: {}", side_id, pokemon.name)
                };
                self.add_log("faint", &[&name]);
                self.sides[side_idx].faint_pokemon(slot);
            }
        }
    }

    /// Start the next turn
    fn next_turn(&mut self) {
        // Clear turn state
        for side in &mut self.sides {
            side.clear_turn_state();
        }

        self.turn += 1;
        self.add_log("turn", &[&self.turn.to_string()]);

        // Set up new request
        self.request_state = BattleRequestState::Move;
        for side in &mut self.sides {
            side.request_state = RequestState::Move;
        }
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
