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
