//! Simulator Pokemon
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module contains the Pokemon struct and related types.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::dex_data::{ID, Gender, StatsTable, BoostsTable, EffectState, StatID};

/// A Pokemon's move slot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveSlot {
    pub id: ID,
    pub move_name: String,
    pub pp: u8,
    pub maxpp: u8,
    pub target: Option<String>,
    pub disabled: bool,
    pub disabled_source: Option<String>,
    pub used: bool,
    pub virtual_move: bool,
}

impl MoveSlot {
    pub fn new(id: ID, move_name: String, pp: u8, maxpp: u8) -> Self {
        Self {
            id,
            move_name,
            pp,
            maxpp,
            target: None,
            disabled: false,
            disabled_source: None,
            used: false,
            virtual_move: false,
        }
    }
}

/// Pokemon set - the team builder representation of a Pokemon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonSet {
    pub name: String,
    pub species: String,
    pub item: String,
    pub ability: String,
    pub moves: Vec<String>,
    pub nature: String,
    pub gender: Gender,
    pub evs: StatsTable,
    pub ivs: StatsTable,
    pub level: u8,
    pub shiny: bool,
    pub happiness: u8,
    pub pokeball: String,
    pub hptype: Option<String>,
    pub dynamax_level: u8,
    pub gigantamax: bool,
    pub tera_type: Option<String>,
}

impl Default for PokemonSet {
    fn default() -> Self {
        Self {
            name: String::new(),
            species: String::new(),
            item: String::new(),
            ability: String::new(),
            moves: Vec::new(),
            nature: "Hardy".to_string(),
            gender: Gender::None,
            evs: StatsTable::default(),
            ivs: StatsTable::new(31, 31, 31, 31, 31, 31),
            level: 100,
            shiny: false,
            happiness: 255,
            pokeball: String::new(),
            hptype: None,
            dynamax_level: 10,
            gigantamax: false,
            tera_type: None,
        }
    }
}

/// A Pokemon in battle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pokemon {
    // Identity
    pub name: String,
    pub species_id: ID,
    pub level: u8,
    pub gender: Gender,
    pub happiness: u8,
    pub pokeball: ID,
    pub dynamax_level: u8,
    pub gigantamax: bool,

    // Position
    pub position: usize,
    pub side_index: usize,
    pub is_active: bool,

    // Stats
    pub base_stored_stats: StatsTable,
    pub stored_stats: StatsTable,
    pub boosts: BoostsTable,
    pub maxhp: u32,
    pub base_maxhp: u32,
    pub hp: u32,

    // Ability
    pub base_ability: ID,
    pub ability: ID,
    pub ability_state: EffectState,

    // Item
    pub item: ID,
    pub item_state: EffectState,
    pub last_item: ID,
    pub used_item_this_turn: bool,
    pub ate_berry: bool,
    pub item_knocked_off: bool,

    // Choice item lock (Choice Band/Scarf/Specs)
    pub locked_move: Option<ID>,

    // Types
    pub types: Vec<String>,
    pub added_type: Option<String>,
    pub base_types: Vec<String>,

    // Tera
    pub tera_type: Option<String>,
    pub terastallized: Option<String>,
    pub can_terastallize: Option<String>,

    // Move slots
    pub base_move_slots: Vec<MoveSlot>,
    pub move_slots: Vec<MoveSlot>,

    // Status
    pub status: ID,
    pub status_state: EffectState,
    pub volatiles: HashMap<ID, EffectState>,

    // Battle state
    pub fainted: bool,
    pub faint_queued: bool,
    pub transformed: bool,
    pub illusion: Option<usize>, // Index of pokemon providing illusion

    // Flags
    pub trapped: bool,
    pub maybe_trapped: bool,
    pub maybe_disabled: bool,
    pub switch_flag: bool,
    pub force_switch_flag: bool,
    pub newly_switched: bool,
    pub being_called_back: bool,
    pub dragged_in: Option<usize>,

    // Turn state
    pub last_move: Option<ID>,
    pub last_move_used: Option<ID>,
    pub last_move_target_loc: Option<i8>,
    pub move_this_turn: Option<ID>,
    pub move_this_turn_result: Option<bool>,
    pub move_last_turn_result: Option<bool>,
    pub hurt_this_turn: Option<u32>,
    pub last_damage: u32,
    pub times_attacked: u32,

    // Counters
    pub active_turns: u32,
    pub active_move_actions: u32,
    pub previously_switched_in: u32,
    pub is_started: bool,
    pub during_move: bool,

    // Calculated values
    pub weight_hg: u32,
    pub speed: u32,

    // Mega/Dynamax state
    pub can_mega_evo: Option<String>,
    pub can_ultra_burst: Option<String>,
    pub can_gigantamax: Option<String>,

    // Stellar boost tracking (Gen 9)
    pub stellar_boosted_types: Vec<String>,

    // Staleness tracking for endless battle clause
    pub staleness: Option<String>,
    pub pending_staleness: Option<String>,
    pub volatile_staleness: Option<String>,
}

impl Pokemon {
    /// Create a new Pokemon from a PokemonSet
    pub fn new(set: &PokemonSet, side_index: usize, position: usize) -> Self {
        let species_id = ID::new(&set.species);
        let ability_id = ID::new(&set.ability);
        let item_id = ID::new(&set.item);

        // Convert moves to move slots
        let move_slots: Vec<MoveSlot> = set.moves.iter().map(|m| {
            let id = ID::new(m);
            MoveSlot::new(id, m.clone(), 5, 5) // Default PP, will be set by Dex
        }).collect();

        Self {
            name: if set.name.is_empty() { set.species.clone() } else { set.name.clone() },
            species_id: species_id.clone(),
            level: set.level,
            gender: set.gender,
            happiness: set.happiness,
            pokeball: ID::new(&set.pokeball),
            dynamax_level: set.dynamax_level,
            gigantamax: set.gigantamax,

            position,
            side_index,
            is_active: false,

            base_stored_stats: StatsTable::default(),
            stored_stats: StatsTable::default(),
            boosts: BoostsTable::new(),
            maxhp: 100,
            base_maxhp: 100,
            hp: 100,

            base_ability: ability_id.clone(),
            ability: ability_id,
            ability_state: EffectState::new(ID::empty()),

            item: item_id,
            item_state: EffectState::new(ID::empty()),
            last_item: ID::empty(),
            used_item_this_turn: false,
            ate_berry: false,
            item_knocked_off: false,

            locked_move: None,

            types: Vec::new(),
            added_type: None,
            base_types: Vec::new(),

            tera_type: set.tera_type.clone(),
            terastallized: None,
            can_terastallize: set.tera_type.clone(),

            base_move_slots: move_slots.clone(),
            move_slots,

            status: ID::empty(),
            status_state: EffectState::new(ID::empty()),
            volatiles: HashMap::new(),

            fainted: false,
            faint_queued: false,
            transformed: false,
            illusion: None,

            trapped: false,
            maybe_trapped: false,
            maybe_disabled: false,
            switch_flag: false,
            force_switch_flag: false,
            newly_switched: false,
            being_called_back: false,
            dragged_in: None,

            last_move: None,
            last_move_used: None,
            last_move_target_loc: None,
            move_this_turn: None,
            move_this_turn_result: None,
            move_last_turn_result: None,
            hurt_this_turn: None,
            last_damage: 0,
            times_attacked: 0,

            active_turns: 0,
            active_move_actions: 0,
            previously_switched_in: 0,
            is_started: false,
            during_move: false,

            weight_hg: 0,
            speed: 0,

            can_mega_evo: None,
            can_ultra_burst: None,
            can_gigantamax: if set.gigantamax { Some(set.species.clone()) } else { None },

            stellar_boosted_types: Vec::new(),

            staleness: None,
            pending_staleness: None,
            volatile_staleness: None,
        }
    }

    /// Get the fullname for protocol messages
    pub fn fullname(&self, side_id: &str) -> String {
        format!("{}: {}", side_id, self.name)
    }

    /// Get details string for protocol
    pub fn details(&self) -> String {
        let mut details = self.species_id.as_str().to_string();
        if self.level != 100 {
            details.push_str(&format!(", L{}", self.level));
        }
        if self.gender != Gender::None {
            details.push_str(&format!(", {}", self.gender.to_str()));
        }
        // Could add shiny, tera state, etc.
        details
    }

    /// Check if this Pokemon is fainted
    pub fn is_fainted(&self) -> bool {
        self.fainted || self.hp == 0
    }

    /// Get current HP percentage
    pub fn hp_percent(&self) -> f64 {
        if self.maxhp == 0 {
            return 0.0;
        }
        (self.hp as f64 / self.maxhp as f64) * 100.0
    }

    /// Take damage
    pub fn take_damage(&mut self, damage: u32) -> u32 {
        let actual = damage.min(self.hp);
        self.hp = self.hp.saturating_sub(damage);
        if self.hp == 0 && !self.fainted {
            self.faint_queued = true;
        }
        self.hurt_this_turn = Some(self.hp);
        self.last_damage = actual;
        actual
    }

    /// Heal HP
    pub fn heal(&mut self, amount: u32) -> u32 {
        if self.hp >= self.maxhp {
            return 0;
        }
        let actual = amount.min(self.maxhp - self.hp);
        self.hp += actual;
        actual
    }

    /// Set status condition
    pub fn set_status(&mut self, status: ID) -> bool {
        if !self.status.is_empty() {
            return false;
        }
        self.status = status.clone();
        self.status_state = EffectState::new(status);
        true
    }

    /// Cure status condition
    pub fn cure_status(&mut self) -> bool {
        if self.status.is_empty() {
            return false;
        }
        self.status = ID::empty();
        self.status_state = EffectState::new(ID::empty());
        true
    }

    /// Check if Pokemon has a specific status
    pub fn has_status(&self, status: &str) -> bool {
        self.status.as_str() == status.to_lowercase()
    }

    /// Add a volatile condition
    pub fn add_volatile(&mut self, id: ID) -> bool {
        if self.volatiles.contains_key(&id) {
            return false;
        }
        self.volatiles.insert(id.clone(), EffectState::new(id));
        true
    }

    /// Remove a volatile condition
    pub fn remove_volatile(&mut self, id: &ID) -> bool {
        self.volatiles.remove(id).is_some()
    }

    /// Check if Pokemon has a specific volatile
    pub fn has_volatile(&self, id: &ID) -> bool {
        self.volatiles.contains_key(id)
    }

    /// Get volatile state
    pub fn get_volatile(&self, id: &ID) -> Option<&EffectState> {
        self.volatiles.get(id)
    }

    /// Get mutable volatile state
    pub fn get_volatile_mut(&mut self, id: &ID) -> Option<&mut EffectState> {
        self.volatiles.get_mut(id)
    }

    /// Clear all volatile conditions
    pub fn clear_volatiles(&mut self) {
        self.volatiles.clear();
    }

    /// Get a stat value with boosts applied
    pub fn get_stat(&self, stat: StatID, unboosted: bool) -> u32 {
        let base = self.stored_stats.get(stat) as u32;
        if unboosted {
            return base;
        }

        let boost = match stat {
            StatID::HP => return base,
            StatID::Atk => self.boosts.atk,
            StatID::Def => self.boosts.def,
            StatID::SpA => self.boosts.spa,
            StatID::SpD => self.boosts.spd,
            StatID::Spe => self.boosts.spe,
        };

        let (numerator, denominator) = if boost >= 0 {
            (2 + boost as u32, 2)
        } else {
            (2, 2 + (-boost) as u32)
        };

        base * numerator / denominator
    }

    /// Apply a boost
    pub fn boost(&mut self, boost_id: crate::dex_data::BoostID, amount: i8) -> i8 {
        self.boosts.boost(boost_id, amount)
    }

    /// Clear all boosts
    pub fn clear_boosts(&mut self) {
        self.boosts.clear();
    }

    /// Update speed (called when boosts or conditions change)
    pub fn update_speed(&mut self) {
        self.speed = self.get_stat(StatID::Spe, false);
    }

    /// Reset for a new turn
    pub fn clear_turn_state(&mut self) {
        self.move_last_turn_result = self.move_this_turn_result;
        self.move_this_turn = None;
        self.move_this_turn_result = None;
        self.hurt_this_turn = None;
        self.used_item_this_turn = false;
    }

    /// Reset for switching out
    pub fn clear_switch_state(&mut self) {
        self.is_active = false;
        self.is_started = false;
        self.clear_volatiles();
        self.clear_boosts();
        self.last_move = None;
        self.switch_flag = false;
        self.force_switch_flag = false;
        self.trapped = false;
        self.maybe_trapped = false;
        self.newly_switched = false;
        self.being_called_back = false;
        self.active_turns = 0;
        self.active_move_actions = 0;
        self.locked_move = None; // Clear Choice item lock
    }

    /// Get the slot ID for protocol messages
    pub fn get_slot(&self) -> String {
        format!("p{}a", self.side_index + 1)
    }

    /// Check if Pokemon can switch out
    pub fn can_switch(&self) -> bool {
        !self.trapped && !self.fainted
    }

    /// Get move PP for a move
    pub fn get_move_pp(&self, move_id: &ID) -> Option<u8> {
        self.move_slots.iter()
            .find(|slot| &slot.id == move_id)
            .map(|slot| slot.pp)
    }

    /// Deduct PP for a move
    pub fn deduct_pp(&mut self, move_id: &ID, amount: u8) -> bool {
        if let Some(slot) = self.move_slots.iter_mut().find(|s| &s.id == move_id) {
            slot.pp = slot.pp.saturating_sub(amount);
            slot.used = true;
            true
        } else {
            false
        }
    }

    /// Check if Pokemon has a specific move
    pub fn has_move(&self, move_id: &str) -> bool {
        let id = crate::dex_data::to_id(move_id);
        self.move_slots.iter().any(|slot| slot.id.as_str() == id)
    }

    /// Get the types of this Pokemon
    pub fn get_types(&self, exclude_added: bool) -> Vec<String> {
        let mut types = self.types.clone();
        if !exclude_added {
            if let Some(ref added) = self.added_type {
                if !types.contains(added) {
                    types.push(added.clone());
                }
            }
        }
        // Handle Terastallization
        if let Some(ref tera) = self.terastallized {
            return vec![tera.clone()];
        }
        types
    }

    /// Check if Pokemon is grounded (affected by Ground moves and terrain)
    pub fn is_grounded(&self) -> bool {
        // Flying type or Levitate makes you not grounded
        if self.types.iter().any(|t| t.to_lowercase() == "flying") {
            return false;
        }
        if self.ability.as_str() == "levitate" {
            return false;
        }
        // Air Balloon makes you not grounded
        if self.item.as_str() == "airballoon" {
            return false;
        }
        // Magnet Rise volatile
        if self.has_volatile(&ID::new("magnetrise")) {
            return false;
        }
        // Telekinesis volatile
        if self.has_volatile(&ID::new("telekinesis")) {
            return false;
        }
        // Iron Ball or Gravity or Ingrain makes you grounded even if flying
        if self.item.as_str() == "ironball" {
            return true;
        }
        if self.has_volatile(&ID::new("ingrain")) {
            return true;
        }
        true
    }

    /// Check if Pokemon is semi-invulnerable (Fly, Dig, Dive, etc.)
    pub fn is_semi_invulnerable(&self) -> bool {
        self.has_volatile(&ID::new("fly")) ||
        self.has_volatile(&ID::new("bounce")) ||
        self.has_volatile(&ID::new("skydrop")) ||
        self.has_volatile(&ID::new("dig")) ||
        self.has_volatile(&ID::new("dive")) ||
        self.has_volatile(&ID::new("phantomforce")) ||
        self.has_volatile(&ID::new("shadowforce"))
    }

    /// Check if Pokemon is protected
    pub fn is_protected(&self) -> bool {
        self.has_volatile(&ID::new("protect")) ||
        self.has_volatile(&ID::new("banefulbunker")) ||
        self.has_volatile(&ID::new("kingsshield")) ||
        self.has_volatile(&ID::new("spikyshield")) ||
        self.has_volatile(&ID::new("silktrap")) ||
        self.has_volatile(&ID::new("burningbulwark"))
    }

    /// Get effective weather considering abilities
    pub fn effective_weather(&self, field_weather: &str) -> String {
        // Cloud Nine and Air Lock negate weather
        // This would normally check all Pokemon on field
        // For now just return the field weather
        field_weather.to_string()
    }

    /// Check if Pokemon has a specific item
    pub fn has_item(&self, items: &[&str]) -> bool {
        let item_id = self.item.as_str();
        items.iter().any(|&i| crate::dex_data::to_id(i) == item_id)
    }

    /// Check if Pokemon has a specific ability
    pub fn has_ability(&self, abilities: &[&str]) -> bool {
        let ability_id = self.ability.as_str();
        abilities.iter().any(|&a| crate::dex_data::to_id(a) == ability_id)
    }

    /// Copy volatiles from another Pokemon (for Baton Pass, etc.)
    pub fn copy_volatile_from(&mut self, source: &Pokemon, copy_type: &str) {
        match copy_type {
            "copyvolatile" | "batonpass" => {
                // Copy stat boosts
                self.boosts = source.boosts.clone();

                // Copy certain volatiles
                let copyable = [
                    "aquaring", "confusion", "curse", "embargo", "focusenergy",
                    "gmaxchistrike", "healblock", "ingrain", "laserfocus",
                    "leechseed", "magnetrise", "perishsong", "powertrick",
                    "substitute", "telekinesis", "torment",
                ];

                for volatile_id in &copyable {
                    let id = ID::new(volatile_id);
                    if source.has_volatile(&id) {
                        if let Some(state) = source.get_volatile(&id) {
                            self.volatiles.insert(id, state.clone());
                        }
                    }
                }
            }
            "shedtail" => {
                // Shed Tail only copies the substitute
                let sub_id = ID::new("substitute");
                if source.has_volatile(&sub_id) {
                    if let Some(state) = source.get_volatile(&sub_id) {
                        self.volatiles.insert(sub_id, state.clone());
                    }
                }
            }
            _ => {}
        }
    }

    /// Get the weight in hectograms
    pub fn get_weight(&self) -> u32 {
        // Base weight would come from species data
        // For now return stored weight
        self.weight_hg
    }

    /// Set a new type (for moves like Soak, Forest's Curse, etc.)
    pub fn set_type(&mut self, new_types: Vec<String>) {
        self.types = new_types;
    }

    /// Add a type (for Forest's Curse, Trick-or-Treat)
    pub fn add_type(&mut self, new_type: String) {
        if !self.types.contains(&new_type) {
            self.added_type = Some(new_type);
        }
    }

    /// Get positive boost count (for Stored Power, etc.)
    pub fn positive_boosts(&self) -> i32 {
        let mut count = 0;
        if self.boosts.atk > 0 { count += self.boosts.atk as i32; }
        if self.boosts.def > 0 { count += self.boosts.def as i32; }
        if self.boosts.spa > 0 { count += self.boosts.spa as i32; }
        if self.boosts.spd > 0 { count += self.boosts.spd as i32; }
        if self.boosts.spe > 0 { count += self.boosts.spe as i32; }
        if self.boosts.accuracy > 0 { count += self.boosts.accuracy as i32; }
        if self.boosts.evasion > 0 { count += self.boosts.evasion as i32; }
        count
    }

    /// Get the action speed (speed used for turn order)
    pub fn get_action_speed(&self) -> u32 {
        let mut speed = self.get_stat(StatID::Spe, false);

        // Paralysis halves speed
        if self.status.as_str() == "par" {
            speed /= 2;
        }

        speed
    }

    /// Disable a move
    pub fn disable_move(&mut self, move_id: &str, source: Option<String>) {
        let id = crate::dex_data::to_id(move_id);
        if let Some(slot) = self.move_slots.iter_mut().find(|s| s.id.as_str() == id) {
            slot.disabled = true;
            slot.disabled_source = source;
        }
    }

    /// Enable all disabled moves
    pub fn enable_moves(&mut self) {
        for slot in &mut self.move_slots {
            slot.disabled = false;
            slot.disabled_source = None;
        }
    }

    /// Get usable moves (not disabled, has PP)
    pub fn get_usable_moves(&self) -> Vec<&MoveSlot> {
        self.move_slots.iter()
            .filter(|slot| !slot.disabled && slot.pp > 0)
            .collect()
    }

    /// Check if Pokemon can terastallize
    pub fn can_tera(&self) -> bool {
        self.terastallized.is_none() && self.can_terastallize.is_some()
    }

    /// Terastallize the Pokemon
    pub fn terastallize(&mut self) {
        if let Some(ref tera_type) = self.can_terastallize {
            self.terastallized = Some(tera_type.clone());
            self.can_terastallize = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_set() -> PokemonSet {
        PokemonSet {
            name: "Pikachu".to_string(),
            species: "Pikachu".to_string(),
            item: "Light Ball".to_string(),
            ability: "Static".to_string(),
            moves: vec!["Thunderbolt".to_string(), "Volt Tackle".to_string()],
            level: 50,
            gender: Gender::Male,
            ..Default::default()
        }
    }

    #[test]
    fn test_pokemon_creation() {
        let set = create_test_set();
        let pokemon = Pokemon::new(&set, 0, 0);

        assert_eq!(pokemon.name, "Pikachu");
        assert_eq!(pokemon.level, 50);
        assert_eq!(pokemon.gender, Gender::Male);
        assert_eq!(pokemon.move_slots.len(), 2);
    }

    #[test]
    fn test_damage_and_heal() {
        let set = create_test_set();
        let mut pokemon = Pokemon::new(&set, 0, 0);
        pokemon.hp = 100;
        pokemon.maxhp = 100;

        let damage = pokemon.take_damage(30);
        assert_eq!(damage, 30);
        assert_eq!(pokemon.hp, 70);

        let healed = pokemon.heal(50);
        assert_eq!(healed, 30); // Can only heal up to maxhp
        assert_eq!(pokemon.hp, 100);
    }

    #[test]
    fn test_status() {
        let set = create_test_set();
        let mut pokemon = Pokemon::new(&set, 0, 0);

        assert!(pokemon.set_status(ID::new("par")));
        assert!(pokemon.has_status("par"));
        assert!(!pokemon.set_status(ID::new("brn"))); // Already has status

        assert!(pokemon.cure_status());
        assert!(!pokemon.has_status("par"));
    }

    #[test]
    fn test_volatiles() {
        let set = create_test_set();
        let mut pokemon = Pokemon::new(&set, 0, 0);

        let confusion = ID::new("confusion");
        assert!(pokemon.add_volatile(confusion.clone()));
        assert!(pokemon.has_volatile(&confusion));
        assert!(!pokemon.add_volatile(confusion.clone())); // Already has it

        assert!(pokemon.remove_volatile(&confusion));
        assert!(!pokemon.has_volatile(&confusion));
    }
}
