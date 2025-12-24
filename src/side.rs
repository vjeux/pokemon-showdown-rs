//! Simulator Side
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Represents one side (player) in a battle.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::dex_data::{ID, EffectState, SideID};
use crate::pokemon::{Pokemon, PokemonSet};

/// A single action that can be chosen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChosenAction {
    pub choice: ChoiceType,
    pub pokemon_index: usize,
    pub target_loc: Option<i8>,
    pub move_id: Option<ID>,
    pub switch_index: Option<usize>,
    pub mega: bool,
    pub zmove: Option<String>,
    pub max_move: Option<String>,
    pub terastallize: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
pub struct Choice {
    pub cant_undo: bool,
    pub error: String,
    pub actions: Vec<ChosenAction>,
    pub forced_switches_left: usize,
    pub forced_passes_left: usize,
    pub switch_ins: Vec<usize>,
    pub z_move: bool,
    pub mega: bool,
    pub ultra: bool,
    pub dynamax: bool,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequestState {
    None,
    TeamPreview,
    Move,
    Switch,
}

impl Default for RequestState {
    fn default() -> Self {
        RequestState::None
    }
}

/// A side in the battle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Side {
    /// Side ID (p1, p2, p3, p4)
    pub id: SideID,
    /// Index in battle.sides
    pub n: usize,

    /// Player name
    pub name: String,
    /// Player avatar
    pub avatar: String,

    /// The team (PokemonSets)
    pub team: Vec<PokemonSet>,
    /// The Pokemon objects
    pub pokemon: Vec<Pokemon>,
    /// Currently active Pokemon (indices into self.pokemon)
    pub active: Vec<Option<usize>>,

    /// Number of Pokemon left (not fainted)
    pub pokemon_left: usize,
    /// Whether Z-move has been used
    pub z_move_used: bool,
    /// Whether Dynamax has been used
    pub dynamax_used: bool,

    /// Last Pokemon to faint last turn
    pub fainted_last_turn: Option<usize>,
    /// Last Pokemon to faint this turn
    pub fainted_this_turn: Option<usize>,
    /// Total Pokemon fainted
    pub total_fainted: usize,

    /// Last selected move (Gen 1 only)
    pub last_selected_move: ID,
    /// Last move used (Gen 1)
    pub last_move: Option<ID>,

    /// Side conditions (Stealth Rock, Spikes, etc.)
    pub side_conditions: HashMap<ID, EffectState>,
    /// Slot conditions (per-slot effects)
    pub slot_conditions: Vec<HashMap<ID, EffectState>>,

    /// Current request state
    pub request_state: RequestState,
    /// Current choice
    pub choice: Choice,

    /// Foe side index
    pub foe_index: Option<usize>,
    /// Ally side index (multi battles)
    pub ally_index: Option<usize>,
}

impl Side {
    /// Create a new side
    pub fn new(id: SideID, n: usize, name: String, team: Vec<PokemonSet>, active_count: usize) -> Self {
        let pokemon: Vec<Pokemon> = team.iter().enumerate()
            .map(|(i, set)| Pokemon::new(set, n, i))
            .collect();

        let pokemon_left = pokemon.len();
        let slot_conditions = (0..active_count)
            .map(|_| HashMap::new())
            .collect();

        Self {
            id,
            n,
            name,
            avatar: String::new(),
            team,
            pokemon,
            active: vec![None; active_count],
            pokemon_left,
            z_move_used: false,
            dynamax_used: false,
            fainted_last_turn: None,
            fainted_this_turn: None,
            total_fainted: 0,
            last_selected_move: ID::empty(),
            last_move: None,
            side_conditions: HashMap::new(),
            slot_conditions,
            request_state: RequestState::None,
            choice: Choice::new(),
            foe_index: None,
            ally_index: None,
        }
    }

    /// Get the Side ID as a string
    pub fn id_str(&self) -> &'static str {
        self.id.to_str()
    }

    /// Get a reference to a Pokemon by index
    pub fn get_pokemon(&self, index: usize) -> Option<&Pokemon> {
        self.pokemon.get(index)
    }

    /// Get a mutable reference to a Pokemon by index
    pub fn get_pokemon_mut(&mut self, index: usize) -> Option<&mut Pokemon> {
        self.pokemon.get_mut(index)
    }

    /// Get the active Pokemon in a slot
    pub fn get_active(&self, slot: usize) -> Option<&Pokemon> {
        self.active.get(slot).and_then(|opt| {
            opt.and_then(|idx| self.pokemon.get(idx))
        })
    }

    /// Get a mutable reference to the active Pokemon in a slot
    pub fn get_active_mut(&mut self, slot: usize) -> Option<&mut Pokemon> {
        if let Some(Some(idx)) = self.active.get(slot) {
            self.pokemon.get_mut(*idx)
        } else {
            None
        }
    }

    /// Get all active Pokemon
    pub fn all_active(&self) -> Vec<&Pokemon> {
        self.active.iter()
            .filter_map(|opt| opt.and_then(|idx| self.pokemon.get(idx)))
            .collect()
    }

    /// Count active Pokemon
    pub fn active_count(&self) -> usize {
        self.active.iter().filter(|opt| opt.is_some()).count()
    }

    /// Switch a Pokemon into a slot
    pub fn switch_in(&mut self, slot: usize, pokemon_index: usize) -> bool {
        if slot >= self.active.len() || pokemon_index >= self.pokemon.len() {
            return false;
        }

        // Switch out current occupant
        if let Some(old_idx) = self.active[slot] {
            if let Some(old_pokemon) = self.pokemon.get_mut(old_idx) {
                old_pokemon.clear_switch_state();
            }
        }

        // Switch in new Pokemon
        self.active[slot] = Some(pokemon_index);
        if let Some(pokemon) = self.pokemon.get_mut(pokemon_index) {
            pokemon.is_active = true;
            pokemon.position = slot;
            pokemon.newly_switched = true;
            pokemon.previously_switched_in += 1;
        }

        true
    }

    /// Switch out the Pokemon in a slot
    pub fn switch_out(&mut self, slot: usize) -> Option<usize> {
        if slot >= self.active.len() {
            return None;
        }

        let old_idx = self.active[slot].take()?;
        if let Some(pokemon) = self.pokemon.get_mut(old_idx) {
            pokemon.clear_switch_state();
        }
        Some(old_idx)
    }

    /// Add a side condition
    pub fn add_side_condition(&mut self, id: ID, duration: Option<u32>) -> bool {
        if self.side_conditions.contains_key(&id) {
            return false;
        }
        let mut state = EffectState::new(id.clone());
        state.duration = duration;
        self.side_conditions.insert(id, state);
        true
    }

    /// Check if a side condition is active
    pub fn has_side_condition(&self, id: &ID) -> bool {
        self.side_conditions.contains_key(id)
    }

    /// Get side condition state
    pub fn get_side_condition(&self, id: &ID) -> Option<&EffectState> {
        self.side_conditions.get(id)
    }

    /// Get mutable side condition state
    pub fn get_side_condition_mut(&mut self, id: &ID) -> Option<&mut EffectState> {
        self.side_conditions.get_mut(id)
    }

    /// Remove a side condition
    pub fn remove_side_condition(&mut self, id: &ID) -> bool {
        self.side_conditions.remove(id).is_some()
    }

    /// Add a slot condition
    pub fn add_slot_condition(&mut self, slot: usize, id: ID, duration: Option<u32>) -> bool {
        if slot >= self.slot_conditions.len() {
            return false;
        }
        if self.slot_conditions[slot].contains_key(&id) {
            return false;
        }
        let mut state = EffectState::new(id.clone());
        state.duration = duration;
        self.slot_conditions[slot].insert(id, state);
        true
    }

    /// Check if a slot condition is active
    pub fn has_slot_condition(&self, slot: usize, id: &ID) -> bool {
        self.slot_conditions.get(slot)
            .map(|conds| conds.contains_key(id))
            .unwrap_or(false)
    }

    /// Remove a slot condition
    pub fn remove_slot_condition(&mut self, slot: usize, id: &ID) -> bool {
        self.slot_conditions.get_mut(slot)
            .map(|conds| conds.remove(id).is_some())
            .unwrap_or(false)
    }

    /// Count unfainted Pokemon
    pub fn count_unfainted(&self) -> usize {
        self.pokemon.iter().filter(|p| !p.is_fainted()).count()
    }

    /// Check if side has lost
    pub fn has_lost(&self) -> bool {
        self.count_unfainted() == 0
    }

    /// Get Pokemon that can switch in
    pub fn get_switchable(&self) -> Vec<usize> {
        self.pokemon.iter().enumerate()
            .filter(|(_, p)| !p.is_fainted() && !p.is_active)
            .map(|(i, _)| i)
            .collect()
    }

    /// Faint a Pokemon
    pub fn faint_pokemon(&mut self, slot: usize) {
        if let Some(Some(idx)) = self.active.get(slot) {
            let idx = *idx;
            if let Some(pokemon) = self.pokemon.get_mut(idx) {
                pokemon.fainted = true;
                pokemon.faint_queued = false;
                pokemon.hp = 0;
            }
            self.fainted_this_turn = Some(idx);
            self.total_fainted += 1;
            self.pokemon_left = self.pokemon_left.saturating_sub(1);
            self.active[slot] = None;
        }
    }

    /// Clear turn state
    pub fn clear_turn_state(&mut self) {
        self.fainted_last_turn = self.fainted_this_turn;
        self.fainted_this_turn = None;
        self.choice.clear();

        for pokemon in &mut self.pokemon {
            pokemon.clear_turn_state();
        }
    }

    /// Get the choice as a string
    pub fn get_choice(&self) -> String {
        self.choice.actions.iter().map(|action| {
            match action.choice {
                ChoiceType::Move => {
                    let mut s = format!("move {}", action.move_id.as_ref().map(|id| id.as_str()).unwrap_or(""));
                    if let Some(target) = action.target_loc {
                        s.push_str(&format!(" {}", target));
                    }
                    if action.mega {
                        s.push_str(" mega");
                    }
                    if let Some(ref z) = action.zmove {
                        s.push_str(&format!(" zmove {}", z));
                    }
                    s
                }
                ChoiceType::Switch => {
                    format!("switch {}", action.switch_index.unwrap_or(0) + 1)
                }
                ChoiceType::Team => {
                    format!("team {}", action.pokemon_index + 1)
                }
                ChoiceType::Pass => "pass".to_string(),
                _ => format!("{:?}", action.choice),
            }
        }).collect::<Vec<_>>().join(", ")
    }

    /// Add an entry hazard with layer support
    /// Spikes and Toxic Spikes can have multiple layers
    pub fn add_hazard(&mut self, hazard_id: &ID) -> bool {
        let hazard_name = hazard_id.as_str();
        let max_layers = match hazard_name {
            "spikes" => 3,
            "toxicspikes" => 2,
            "stealthrock" | "stickyweb" => 1,
            _ => 1,
        };

        if let Some(state) = self.side_conditions.get_mut(hazard_id) {
            // Already have this hazard - try to add another layer
            let current_layers = state.layers.unwrap_or(1);
            if current_layers >= max_layers {
                return false; // Already at max layers
            }
            state.layers = Some(current_layers + 1);
            true
        } else {
            // Add new hazard with 1 layer
            let mut state = EffectState::new(hazard_id.clone());
            state.layers = Some(1);
            self.side_conditions.insert(hazard_id.clone(), state);
            true
        }
    }

    /// Get the number of layers of a hazard
    pub fn get_hazard_layers(&self, hazard_id: &ID) -> u32 {
        self.side_conditions.get(hazard_id)
            .and_then(|state| state.layers)
            .unwrap_or(0)
    }

    /// Check if choice for this turn is complete
    /// Equivalent to side.ts isChoiceDone()
    pub fn is_choice_done(&self) -> bool {
        match self.request_state {
            RequestState::None => true,
            RequestState::TeamPreview => {
                // For team preview, we need enough actions for the picked team size
                // Simplified: just check if we have any actions
                !self.choice.actions.is_empty()
            }
            RequestState::Move | RequestState::Switch => {
                // Need one action per active slot
                let active_slots = self.active.len();
                self.choice.actions.len() >= active_slots
            }
        }
    }

    /// Calculate Stealth Rock damage based on type effectiveness
    pub fn calc_stealth_rock_damage(defender_types: &[String], max_hp: u32) -> u32 {
        // Stealth Rock does 12.5% base, modified by Rock type effectiveness
        let mut effectiveness = 1.0;
        for def_type in defender_types {
            effectiveness *= match def_type.to_lowercase().as_str() {
                // Rock is super effective against
                "fire" | "ice" | "flying" | "bug" => 2.0,
                // Rock is resisted by
                "fighting" | "ground" | "steel" => 0.5,
                _ => 1.0,
            };
        }

        // Base 1/8 (12.5%) * effectiveness
        let damage_frac = 0.125 * effectiveness;
        ((max_hp as f64 * damage_frac) as u32).max(1)
    }

    /// Calculate Spikes damage based on number of layers
    pub fn calc_spikes_damage(layers: u32, max_hp: u32) -> u32 {
        // Spikes: 1 layer = 1/8, 2 layers = 1/6, 3 layers = 1/4
        let damage_frac = match layers {
            1 => 1.0 / 8.0,
            2 => 1.0 / 6.0,
            _ => 1.0 / 4.0,
        };
        ((max_hp as f64 * damage_frac) as u32).max(1)
    }

    // ==========================================
    // Methods ported from side.ts
    // ==========================================

    /// String representation of Side
    pub fn to_string(&self) -> String {
        format!("{}: {}", self.id_str(), self.name)
    }

    /// Check if this side can dynamax now (Gen 8)
    pub fn can_dynamax_now(&self, gen: u8, game_type: &str, turn: u32) -> bool {
        if gen != 8 {
            return false;
        }
        // In multi battles, players alternate turns for dynamaxing
        if game_type == "multi" {
            let allowed_indices = match turn % 2 {
                1 => [0, 1],
                _ => [2, 3],
            };
            if !allowed_indices.contains(&self.n) {
                return false;
            }
        }
        !self.dynamax_used
    }

    /// Get allies (all active Pokemon on this side)
    pub fn allies(&self, include_fainted: bool) -> Vec<usize> {
        let mut allies = Vec::new();
        for active_idx in &self.active {
            if let Some(idx) = active_idx {
                if let Some(pokemon) = self.pokemon.get(*idx) {
                    if include_fainted || !pokemon.is_fainted() {
                        allies.push(*idx);
                    }
                }
            }
        }
        allies
    }

    /// Get foes (active Pokemon on foe sides)
    /// Returns vec of (side_index, pokemon_index) for each foe
    pub fn foes_active(&self) -> Vec<usize> {
        // This would need access to the battle to get foe side
        // For now return empty - caller should use battle context
        Vec::new()
    }

    /// Check if a Pokemon is an ally
    pub fn has_ally(&self, pokemon_side_index: usize) -> bool {
        pokemon_side_index == self.n || self.ally_index == Some(pokemon_side_index)
    }

    /// Add a Pokemon to the team
    pub fn add_pokemon(&mut self, set: PokemonSet) -> Option<usize> {
        if self.pokemon.len() >= 24 {
            return None;
        }
        let pos = self.pokemon.len();
        let pokemon = Pokemon::new(&set, self.n, pos);
        self.pokemon.push(pokemon);
        self.team.push(set);
        self.pokemon_left += 1;
        Some(pos)
    }

    /// Get a random foe (would need RNG in real implementation)
    pub fn random_foe(&self) -> Option<usize> {
        // This is a stub - real implementation needs battle context
        None
    }

    /// Get total Pokemon left on foe side
    pub fn foe_pokemon_left(&self) -> usize {
        // This is a stub - needs battle context
        0
    }

    /// Get slot condition data
    pub fn get_slot_condition(&self, slot: usize, id: &ID) -> Option<&EffectState> {
        self.slot_conditions.get(slot)?.get(id)
    }

    /// Get mutable slot condition data
    pub fn get_slot_condition_mut(&mut self, slot: usize, id: &ID) -> Option<&mut EffectState> {
        self.slot_conditions.get_mut(slot)?.get_mut(id)
    }

    /// Clear the current choice
    pub fn clear_choice(&mut self, request_state: RequestState, forced_switches: usize, forced_passes: usize) {
        self.choice = Choice {
            cant_undo: false,
            error: String::new(),
            actions: Vec::new(),
            forced_switches_left: forced_switches,
            forced_passes_left: forced_passes,
            switch_ins: Vec::new(),
            z_move: false,
            mega: false,
            ultra: false,
            dynamax: false,
            terastallize: false,
        };
        self.request_state = request_state;
    }

    /// Get the current choice action index
    pub fn get_choice_index(&self) -> usize {
        self.choice.actions.len()
    }

    /// Choose pass action
    pub fn choose_pass(&mut self) -> bool {
        let index = self.get_choice_index();
        if index >= self.active.len() {
            return false;
        }

        match self.request_state {
            RequestState::Switch => {
                if self.choice.forced_passes_left == 0 {
                    return false;
                }
                self.choice.forced_passes_left -= 1;
            }
            RequestState::Move => {
                // Check if the Pokemon is fainted
                if let Some(Some(pokemon_idx)) = self.active.get(index) {
                    if let Some(pokemon) = self.pokemon.get(*pokemon_idx) {
                        if !pokemon.is_fainted() {
                            return false; // Can't pass if not fainted
                        }
                    }
                }
            }
            _ => return false,
        }

        self.choice.actions.push(ChosenAction {
            choice: ChoiceType::Pass,
            pokemon_index: index,
            target_loc: None,
            move_id: None,
            switch_index: None,
            mega: false,
            zmove: None,
            max_move: None,
            terastallize: None,
        });
        true
    }

    /// Choose switch action
    pub fn choose_switch(&mut self, slot: usize) -> Result<(), String> {
        let index = self.get_choice_index();
        if index >= self.active.len() {
            return Err("You sent more switches than needed".to_string());
        }

        if slot >= self.pokemon.len() {
            return Err(format!("You don't have a Pokemon in slot {}", slot + 1));
        }

        if slot < self.active.len() {
            return Err("Can't switch to an active Pokemon".to_string());
        }

        if self.choice.switch_ins.contains(&slot) {
            return Err(format!("Pokemon in slot {} already switching in", slot + 1));
        }

        let target = self.pokemon.get(slot).ok_or("Invalid slot")?;
        if target.is_fainted() {
            return Err("Can't switch to a fainted Pokemon".to_string());
        }

        self.choice.switch_ins.push(slot);

        let choice_type = if self.request_state == RequestState::Switch {
            if self.choice.forced_switches_left == 0 {
                return Err("No more forced switches".to_string());
            }
            self.choice.forced_switches_left -= 1;
            ChoiceType::InstaSwitch
        } else {
            ChoiceType::Switch
        };

        self.choice.actions.push(ChosenAction {
            choice: choice_type,
            pokemon_index: index,
            target_loc: None,
            move_id: None,
            switch_index: Some(slot),
            mega: false,
            zmove: None,
            max_move: None,
            terastallize: None,
        });

        Ok(())
    }

    /// Choose move action
    pub fn choose_move(&mut self, move_id: ID, target_loc: Option<i8>, mega: bool, zmove: Option<String>, max_move: Option<String>, terastallize: Option<String>) -> Result<(), String> {
        let index = self.get_choice_index();
        if index >= self.active.len() {
            return Err("You sent more choices than unfainted Pokemon".to_string());
        }

        if self.request_state != RequestState::Move {
            return Err(format!("Can't move: You need a {:?} response", self.request_state));
        }

        // Check mega/dynamax/tera restrictions
        if mega && self.choice.mega {
            return Err("You can only mega evolve once per battle".to_string());
        }
        if zmove.is_some() && self.choice.z_move {
            return Err("You can only use a Z-move once per battle".to_string());
        }
        if max_move.is_some() && self.choice.dynamax {
            return Err("You can only Dynamax once per battle".to_string());
        }
        if terastallize.is_some() && self.choice.terastallize {
            return Err("You can only Terastallize once per battle".to_string());
        }

        self.choice.actions.push(ChosenAction {
            choice: ChoiceType::Move,
            pokemon_index: index,
            target_loc,
            move_id: Some(move_id),
            switch_index: None,
            mega,
            zmove: zmove.clone(),
            max_move: max_move.clone(),
            terastallize: terastallize.clone(),
        });

        if mega {
            self.choice.mega = true;
        }
        if zmove.is_some() {
            self.choice.z_move = true;
        }
        if max_move.is_some() {
            self.choice.dynamax = true;
        }
        if terastallize.is_some() {
            self.choice.terastallize = true;
        }

        Ok(())
    }

    /// Choose team action (team preview)
    pub fn choose_team(&mut self, positions: Vec<usize>) -> Result<(), String> {
        if self.request_state != RequestState::TeamPreview {
            return Err("Not in team preview phase".to_string());
        }

        for (i, pos) in positions.iter().enumerate() {
            if *pos >= self.pokemon.len() {
                return Err(format!("No Pokemon in slot {}", pos + 1));
            }
            if positions[..i].contains(pos) {
                return Err(format!("Pokemon in slot {} selected twice", pos + 1));
            }
        }

        for (index, pos) in positions.iter().enumerate() {
            self.choice.switch_ins.push(*pos);
            self.choice.actions.push(ChosenAction {
                choice: ChoiceType::Team,
                pokemon_index: *pos,
                target_loc: None,
                move_id: None,
                switch_index: Some(index),
                mega: false,
                zmove: None,
                max_move: None,
                terastallize: None,
            });
        }

        Ok(())
    }

    /// Choose shift action (triples)
    pub fn choose_shift(&mut self) -> Result<(), String> {
        let index = self.get_choice_index();
        if index >= self.active.len() {
            return Err(format!("No Pokemon in slot {}", index + 1));
        }
        if self.request_state != RequestState::Move {
            return Err("Can only shift during move phase".to_string());
        }
        if self.active.len() != 3 {
            return Err("Can only shift in triples".to_string());
        }
        if index == 1 {
            return Err("Can only shift from edge to center".to_string());
        }

        self.choice.actions.push(ChosenAction {
            choice: ChoiceType::Shift,
            pokemon_index: index,
            target_loc: None,
            move_id: None,
            switch_index: None,
            mega: false,
            zmove: None,
            max_move: None,
            terastallize: None,
        });

        Ok(())
    }

    /// Auto-choose remaining actions
    pub fn auto_choose(&mut self) -> bool {
        match self.request_state {
            RequestState::TeamPreview => {
                if !self.is_choice_done() {
                    let positions: Vec<usize> = (0..self.pokemon.len()).collect();
                    let _ = self.choose_team(positions);
                }
            }
            RequestState::Switch => {
                let mut iterations = 0;
                while !self.is_choice_done() && iterations < 10 {
                    // Find first available switch target
                    for i in self.active.len()..self.pokemon.len() {
                        if !self.choice.switch_ins.contains(&i) {
                            if let Some(pokemon) = self.pokemon.get(i) {
                                if !pokemon.is_fainted() {
                                    let _ = self.choose_switch(i);
                                    break;
                                }
                            }
                        }
                    }
                    iterations += 1;
                }
            }
            RequestState::Move => {
                let mut iterations = 0;
                while !self.is_choice_done() && iterations < 10 {
                    let index = self.get_choice_index();
                    if let Some(Some(pokemon_idx)) = self.active.get(index) {
                        let pokemon = &self.pokemon[*pokemon_idx];
                        if pokemon.is_fainted() {
                            self.choose_pass();
                        } else {
                            // Try first available move
                            if let Some(first_move) = pokemon.move_slots.first() {
                                let move_id = first_move.id.clone();
                                let _ = self.choose_move(move_id, None, false, None, None, None);
                            } else {
                                // Struggle
                                let _ = self.choose_move(ID::new("struggle"), None, false, None, None, None);
                            }
                        }
                    }
                    iterations += 1;
                }
            }
            RequestState::None => {}
        }
        true
    }

    /// Get picked team size for team preview
    pub fn picked_team_size(&self, rule_table_size: Option<usize>) -> usize {
        rule_table_size.unwrap_or(self.pokemon.len()).min(self.pokemon.len())
    }

    /// Destroy side (cleanup)
    pub fn destroy(&mut self) {
        self.pokemon.clear();
        self.active.clear();
        self.choice.actions.clear();
        self.side_conditions.clear();
        for slot_cond in &mut self.slot_conditions {
            slot_cond.clear();
        }
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
                moves: vec!["Thunderbolt".to_string()],
                ..Default::default()
            },
            PokemonSet {
                name: "Charizard".to_string(),
                species: "Charizard".to_string(),
                ability: "Blaze".to_string(),
                moves: vec!["Flamethrower".to_string()],
                ..Default::default()
            },
        ]
    }

    #[test]
    fn test_side_creation() {
        let team = create_test_team();
        let side = Side::new(SideID::P1, 0, "Test Player".to_string(), team, 1);

        assert_eq!(side.name, "Test Player");
        assert_eq!(side.pokemon.len(), 2);
        assert_eq!(side.pokemon_left, 2);
        assert_eq!(side.active.len(), 1);
    }

    #[test]
    fn test_switch_in_out() {
        let team = create_test_team();
        let mut side = Side::new(SideID::P1, 0, "Test".to_string(), team, 1);

        // Switch in first Pokemon
        assert!(side.switch_in(0, 0));
        assert!(side.get_active(0).is_some());
        assert_eq!(side.active_count(), 1);

        // Switch to second Pokemon
        assert!(side.switch_in(0, 1));
        assert_eq!(side.get_active(0).unwrap().name, "Charizard");
    }

    #[test]
    fn test_side_conditions() {
        let team = create_test_team();
        let mut side = Side::new(SideID::P1, 0, "Test".to_string(), team, 1);

        let rocks = ID::new("stealthrock");
        assert!(side.add_side_condition(rocks.clone(), None));
        assert!(side.has_side_condition(&rocks));
        assert!(!side.add_side_condition(rocks.clone(), None)); // Already exists

        assert!(side.remove_side_condition(&rocks));
        assert!(!side.has_side_condition(&rocks));
    }

    #[test]
    fn test_faint() {
        let team = create_test_team();
        let mut side = Side::new(SideID::P1, 0, "Test".to_string(), team, 1);

        side.switch_in(0, 0);
        assert_eq!(side.count_unfainted(), 2);

        side.faint_pokemon(0);
        assert_eq!(side.count_unfainted(), 1);
        assert_eq!(side.pokemon_left, 1);
        assert!(side.active[0].is_none());
    }
}
