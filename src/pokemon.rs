//! Simulator Pokemon
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module contains the Pokemon struct and related types.

use std::collections::HashMap;
use std::fmt;
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
    pub maybe_locked: bool, // Choice items may lock next turn
    pub switch_flag: bool,
    pub force_switch_flag: bool,
    pub newly_switched: bool,
    pub being_called_back: bool,
    pub dragged_in: Option<usize>,
    pub skip_before_switch_out_event_flag: bool,
    pub stats_raised_this_turn: bool,
    pub stats_lowered_this_turn: bool,

    // Ability-specific flags
    pub sword_boost: bool,  // Intrepid Sword / Dauntless Shield

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
            maybe_locked: false,
            switch_flag: false,
            force_switch_flag: false,
            newly_switched: false,
            being_called_back: false,
            dragged_in: None,
            skip_before_switch_out_event_flag: false,
            stats_raised_this_turn: false,
            stats_lowered_this_turn: false,

            sword_boost: false,

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

    /// Get the slot ID for protocol messages (e.g., "p1a", "p2b")
    pub fn get_slot(&self) -> String {
        let position_letter = match self.position {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            _ => 'a',
        };
        format!("p{}{}", self.side_index + 1, position_letter)
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

    // ==========================================
    // Methods ported from pokemon.ts
    // ==========================================

    /// String representation of Pokemon
    pub fn to_string(&self) -> String {
        if self.is_active {
            format!("{}{}", self.get_slot(), &self.name)
        } else {
            self.fullname(&format!("p{}", self.side_index + 1))
        }
    }

    /// Get updated details string for protocol
    pub fn get_updated_details(&self) -> String {
        let mut details = self.species_id.as_str().to_string();
        if self.level != 100 {
            details.push_str(&format!(", L{}", self.level));
        }
        if self.gender != Gender::None {
            details.push_str(&format!(", {}", self.gender.to_str()));
        }
        details
    }

    /// Calculate a stat with boost
    pub fn calculate_stat(&self, stat: StatID, boost: i8, modifier: f64) -> u32 {
        if stat == StatID::HP {
            return self.maxhp;
        }

        // Get base stat
        let base_stat = self.stored_stats.get(stat) as u32;

        // Apply boost
        let clamped_boost = boost.clamp(-6, 6);
        let boost_table: [f64; 7] = [1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0];

        let boosted_stat = if clamped_boost >= 0 {
            (base_stat as f64 * boost_table[clamped_boost as usize]) as u32
        } else {
            (base_stat as f64 / boost_table[(-clamped_boost) as usize]) as u32
        };

        // Apply modifier
        ((boosted_stat as f64) * modifier) as u32
    }

    /// Get best stat (for Beast Boost, Quark Drive, Protosynthesis)
    pub fn get_best_stat(&self, unboosted: bool) -> StatID {
        let stats = [StatID::Atk, StatID::Def, StatID::SpA, StatID::SpD, StatID::Spe];
        let mut best_stat = StatID::Atk;
        let mut best_value = 0;

        for stat in stats {
            let value = self.get_stat(stat, unboosted);
            if value > best_value {
                best_value = value;
                best_stat = stat;
            }
        }

        best_stat
    }

    /// Check if Pokemon has a specific type
    pub fn has_type(&self, type_name: &str) -> bool {
        self.get_types(false).iter().any(|t| t.to_lowercase() == type_name.to_lowercase())
    }

    /// Check if Pokemon has any of the given types
    pub fn has_any_type(&self, types: &[&str]) -> bool {
        let pokemon_types = self.get_types(false);
        for t in types {
            if pokemon_types.iter().any(|pt| pt.to_lowercase() == t.to_lowercase()) {
                return true;
            }
        }
        false
    }

    /// Mark Pokemon as fainted and queue faint
    /// Returns the amount of damage dealt (HP before faint)
    pub fn faint(&mut self) -> u32 {
        if self.fainted || self.faint_queued {
            return 0;
        }
        let damage = self.hp;
        self.hp = 0;
        self.switch_flag = false;
        self.faint_queued = true;
        damage
    }

    /// Apply damage to Pokemon
    /// Returns actual damage dealt
    pub fn damage(&mut self, amount: u32) -> u32 {
        if self.hp == 0 || amount == 0 {
            return 0;
        }
        let actual = amount.min(self.hp);
        self.hp = self.hp.saturating_sub(amount);
        if self.hp == 0 {
            self.faint_queued = true;
        }
        actual
    }

    /// Check if this Pokemon is an ally of another
    pub fn is_ally(&self, other_side_index: usize) -> bool {
        self.side_index == other_side_index
    }

    /// Check if this is the same Pokemon (by position and side)
    /// JavaScript pattern: if (target === pokemon) continue;
    pub fn is_same(&self, other: &Pokemon) -> bool {
        self.side_index == other.side_index && self.position == other.position
    }

    /// Check if another Pokemon is adjacent (for targeting)
    pub fn is_adjacent(&self, other_position: usize, other_fainted: bool, active_per_half: usize) -> bool {
        if self.fainted || other_fainted {
            return false;
        }
        if active_per_half <= 2 {
            return self.position != other_position;
        }
        // For triples, only adjacent positions can target each other
        (self.position as i32 - other_position as i32).abs() <= 1
    }

    /// Get capped boost - returns the actual change that would be applied
    pub fn get_capped_boost(&self, stat: crate::dex_data::BoostID, amount: i8) -> i8 {
        let current = self.boosts.get(stat);
        let new_value = (current + amount).clamp(-6, 6);
        new_value - current
    }

    /// Boost stat by amount, respecting caps
    pub fn boost_by(&mut self, stat: crate::dex_data::BoostID, amount: i8) -> i8 {
        let delta = self.get_capped_boost(stat, amount);
        self.boosts.boost(stat, delta);
        if delta > 0 {
            self.stats_raised_this_turn = true;
        } else if delta < 0 {
            self.stats_lowered_this_turn = true;
        }
        delta
    }

    /// Set a specific boost value
    pub fn set_boost(&mut self, stat: crate::dex_data::BoostID, value: i8) {
        let clamped = value.clamp(-6, 6);
        self.boosts.set(stat, clamped);
    }

    /// Clear ability (set to empty)
    pub fn clear_ability(&mut self) -> ID {
        let old = self.ability.clone();
        self.ability = ID::empty();
        self.ability_state = EffectState::new(ID::empty());
        old
    }

    /// Set ability
    pub fn set_ability(&mut self, ability_id: ID) -> ID {
        let old = self.ability.clone();
        self.ability = ability_id.clone();
        self.ability_state = EffectState::new(ability_id);
        old
    }

    /// Get ability ID
    pub fn get_ability(&self) -> &ID {
        &self.ability
    }

    /// Clear item
    pub fn clear_item(&mut self) -> ID {
        let old = self.item.clone();
        self.item = ID::empty();
        self.item_state = EffectState::new(ID::empty());
        old
    }

    /// Set item
    pub fn set_item(&mut self, item_id: ID) -> bool {
        if self.hp == 0 || !self.is_active {
            return false;
        }
        self.item = item_id.clone();
        self.item_state = EffectState::new(item_id);
        true
    }

    /// Take item (remove and return it)
    pub fn take_item(&mut self) -> Option<ID> {
        if self.item.is_empty() {
            return None;
        }
        let item = self.item.clone();
        self.item = ID::empty();
        self.item_state = EffectState::new(ID::empty());
        Some(item)
    }

    /// Get item ID
    pub fn get_item(&self) -> &ID {
        &self.item
    }

    /// Set HP directly, returns delta
    pub fn set_hp(&mut self, hp: u32) -> i32 {
        if self.hp == 0 {
            return 0;
        }
        let clamped = hp.clamp(1, self.maxhp);
        let delta = clamped as i32 - self.hp as i32;
        self.hp = clamped;
        delta
    }

    /// Record that a move was used
    pub fn move_used(&mut self, move_id: ID, target_loc: Option<i8>) {
        self.last_move = Some(move_id.clone());
        self.last_move_used = Some(move_id.clone());
        self.last_move_target_loc = target_loc;
        self.move_this_turn = Some(move_id);
    }

    /// Check if this is the last active Pokemon on the side
    pub fn is_last_active(&self) -> bool {
        // This would need access to the side to properly implement
        // For now, just return true if active
        self.is_active
    }

    /// Check if ability is being suppressed
    pub fn ignoring_ability(&self) -> bool {
        // Gen 5+: inactive Pokemon have abilities suppressed
        if !self.is_active {
            return true;
        }
        // Transformed Pokemon with certain abilities
        if self.transformed {
            // Would need to check ability flags
        }
        // Gastro Acid volatile
        if self.has_volatile(&ID::new("gastroacid")) {
            return true;
        }
        false
    }

    /// Check if item is being ignored
    pub fn ignoring_item(&self) -> bool {
        // Gen 5+: inactive Pokemon have items suppressed
        if !self.is_active {
            return true;
        }
        // Embargo volatile
        if self.has_volatile(&ID::new("embargo")) {
            return true;
        }
        // Klutz ability
        if self.ability.as_str() == "klutz" {
            return true;
        }
        false
    }

    /// Update max HP (for forme changes)
    pub fn update_max_hp(&mut self, new_base_max_hp: u32) {
        if new_base_max_hp == self.base_maxhp {
            return;
        }
        self.base_maxhp = new_base_max_hp;
        let old_maxhp = self.maxhp;
        self.maxhp = new_base_max_hp;

        // Adjust current HP proportionally
        if self.hp > 0 {
            let hp_lost = old_maxhp - self.hp;
            self.hp = self.maxhp.saturating_sub(hp_lost).max(1);
        }
    }

    /// Check if Pokemon is in Sky Drop
    pub fn is_sky_dropped(&self) -> bool {
        self.has_volatile(&ID::new("skydrop"))
    }

    /// Get move slot data
    pub fn get_move_data(&self, move_id: &ID) -> Option<&MoveSlot> {
        self.move_slots.iter().find(|slot| &slot.id == move_id)
    }

    /// Get mutable move slot data
    pub fn get_move_data_mut(&mut self, move_id: &ID) -> Option<&mut MoveSlot> {
        self.move_slots.iter_mut().find(|slot| &slot.id == move_id)
    }

    /// Get positive boost count (for Stored Power)
    pub fn get_positive_boosts(&self) -> i32 {
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

    /// Get HP as if not dynamaxed
    pub fn get_undynamaxed_hp(&self) -> u32 {
        if self.has_volatile(&ID::new("dynamax")) {
            // Dynamaxed HP is doubled, so divide by 2
            ((self.hp as f64) * (self.base_maxhp as f64) / (self.maxhp as f64)).ceil() as u32
        } else {
            self.hp
        }
    }

    /// Try to trap the Pokemon
    pub fn try_trap(&mut self, is_hidden: bool) -> bool {
        if self.trapped && is_hidden {
            return true;
        }
        self.trapped = true;
        true
    }

    /// Record that this Pokemon was attacked
    pub fn got_attacked(&mut self, move_id: ID, damage: u32, _source_side: usize, _source_pos: usize) {
        self.last_damage = damage;
        self.times_attacked += 1;
        // Would store in attackedBy array in full implementation
        let _ = move_id; // Use to avoid warning
    }

    /// Get locked move (for multi-turn moves)
    pub fn get_locked_move(&self) -> Option<&ID> {
        self.locked_move.as_ref()
    }

    /// Check if max move is disabled
    pub fn max_move_disabled(&self, base_move_id: &ID) -> bool {
        // Check if the base move has PP
        if let Some(move_data) = self.get_move_data(base_move_id) {
            if move_data.pp == 0 {
                return true;
            }
        }
        // Status moves are disabled by Assault Vest or Taunt when dynamaxed
        // Would need move data to check category
        false
    }

    /// Transform into another Pokemon
    pub fn transform_into(&mut self, target: &Pokemon) -> bool {
        if self.fainted || target.fainted || self.transformed {
            return false;
        }
        if target.has_volatile(&ID::new("substitute")) {
            return false;
        }
        if target.transformed {
            return false;
        }

        // Copy species
        self.species_id = target.species_id.clone();
        self.transformed = true;
        self.weight_hg = target.weight_hg;

        // Copy types
        self.types = target.types.clone();
        self.added_type = target.added_type.clone();

        // Copy stats
        self.stored_stats = target.stored_stats.clone();

        // Copy moves with reduced PP
        self.move_slots = target.move_slots.iter().map(|slot| {
            MoveSlot {
                id: slot.id.clone(),
                move_name: slot.move_name.clone(),
                pp: 5.min(slot.maxpp),
                maxpp: 5.min(slot.maxpp),
                target: slot.target.clone(),
                disabled: false,
                disabled_source: None,
                used: false,
                virtual_move: true,
            }
        }).collect();

        // Copy boosts
        self.boosts = target.boosts.clone();

        // Copy ability
        self.ability = target.ability.clone();

        true
    }

    /// Copy volatiles from another Pokemon (for Baton Pass)
    pub fn copy_volatile_from_full(&mut self, source: &Pokemon, is_shed_tail: bool) {
        // Copy boosts unless Shed Tail
        if !is_shed_tail {
            self.boosts = source.boosts.clone();
        }

        // List of volatiles that can be copied
        let copyable_volatiles = [
            "aquaring", "confusion", "curse", "embargo", "focusenergy",
            "gmaxchistrike", "healblock", "ingrain", "laserfocus",
            "leechseed", "magnetrise", "perishsong", "powertrick",
            "substitute", "telekinesis", "torment",
        ];

        for volatile_name in copyable_volatiles {
            if is_shed_tail && volatile_name != "substitute" {
                continue;
            }
            let id = ID::new(volatile_name);
            if let Some(state) = source.get_volatile(&id) {
                self.volatiles.insert(id, state.clone());
            }
        }
    }

    /// Set species (for forme changes and Transform)
    pub fn set_species(&mut self, species_id: ID, types: Vec<String>, weight_hg: u32) {
        self.species_id = species_id;
        self.types = types.clone();
        self.base_types = types;
        self.weight_hg = weight_hg;
    }

    /// Forme change
    pub fn forme_change(&mut self, new_species_id: ID, new_types: Vec<String>, new_ability: Option<ID>) {
        self.species_id = new_species_id;
        self.types = new_types;
        if let Some(ability) = new_ability {
            self.ability = ability.clone();
            self.ability_state = EffectState::new(ability);
        }
    }

    /// Clear all turn state at end of turn
    pub fn clear_turn_state_full(&mut self) {
        self.move_last_turn_result = self.move_this_turn_result;
        self.move_this_turn = None;
        self.move_this_turn_result = None;
        self.hurt_this_turn = None;
        self.used_item_this_turn = false;
        self.stats_raised_this_turn = false;
        self.stats_lowered_this_turn = false;
    }

    // ==========================================
    // Additional methods from pokemon.ts
    // ==========================================

    /// Get moves as string list
    /// Equivalent to moves getter in pokemon.ts
    pub fn get_moves(&self) -> Vec<String> {
        self.move_slots.iter().map(|slot| slot.id.as_str().to_string()).collect()
    }

    /// Get base moves as string list
    /// Equivalent to baseMoves getter in pokemon.ts
    pub fn get_base_moves(&self) -> Vec<String> {
        self.base_move_slots.iter().map(|slot| slot.id.as_str().to_string()).collect()
    }

    /// Convert to JSON representation
    /// Equivalent to toJSON in pokemon.ts
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
            "species": self.species_id.as_str(),
            "hp": self.hp,
            "maxhp": self.maxhp,
            "level": self.level,
            "status": if self.status.is_empty() { None } else { Some(self.status.as_str()) },
            "isActive": self.is_active,
            "position": self.position,
            "side": self.side_index
        })
    }

    /// Get combat power (for Pokemon Go style formats)
    /// Equivalent to getCombatPower in pokemon.ts
    pub fn get_combat_power(&self) -> u32 {
        // Simplified formula based on stats
        let atk = self.stored_stats.atk as u32;
        let def = self.stored_stats.def as u32;
        let sta = (self.stored_stats.hp as u32).max(10); // Use HP as stamina proxy

        (((atk as f64) * (def as f64).powf(0.5) * (sta as f64).powf(0.5)) / 10.0) as u32
    }

    /// Get location of a target Pokemon relative to this one
    /// Equivalent to getLocOf in pokemon.ts
    pub fn get_loc_of(&self, target_side_index: usize, target_position: usize, active_per_half: usize) -> i8 {
        if self.side_index == target_side_index {
            // Same side - positive numbers
            (target_position as i8 - self.position as i8 + 1).max(-(active_per_half as i8))
        } else {
            // Opposing side - negative numbers
            -(target_position as i8 + 1)
        }
    }

    /// Get Pokemon at a location relative to this one
    /// Returns (side_index, position)
    /// Equivalent to getAtLoc in pokemon.ts
    pub fn get_at_loc(&self, target_loc: i8, active_per_half: usize) -> Option<(usize, usize)> {
        if target_loc == 0 {
            return None;
        }

        if target_loc > 0 {
            // Same side
            let pos = (self.position as i8 + target_loc - 1) as usize;
            if pos < active_per_half {
                Some((self.side_index, pos))
            } else {
                None
            }
        } else {
            // Opposite side
            let foe_side = if self.side_index == 0 { 1 } else { 0 };
            let pos = (-target_loc - 1) as usize;
            if pos < active_per_half {
                Some((foe_side, pos))
            } else {
                None
            }
        }
    }

    /// Get smart targets for move
    /// Equivalent to getSmartTargets in pokemon.ts
    pub fn get_smart_targets(&self, target_side: usize, target_pos: usize, move_smart_target: bool) -> Vec<(usize, usize)> {
        if !move_smart_target {
            return vec![(target_side, target_pos)];
        }

        // Smart targeting redirects to a valid target if original fainted
        // Would need battle context for full implementation
        vec![(target_side, target_pos)]
    }

    /// Get last attacker info
    /// Equivalent to getLastAttackedBy in pokemon.ts
    pub fn get_last_attacked_by(&self) -> Option<(ID, u32)> {
        // Would need attacked_by tracking for full implementation
        None
    }

    /// Get last damager info
    /// Equivalent to getLastDamagedBy in pokemon.ts
    pub fn get_last_damaged_by(&self, filter_out_same_side: bool) -> Option<(ID, u32)> {
        if self.last_damage == 0 {
            return None;
        }
        // Would need more tracking for full implementation
        None
    }

    /// Get Dynamax request data
    /// Equivalent to getDynamaxRequest in pokemon.ts
    pub fn get_dynamax_request(&self, can_dynamax: bool) -> Option<serde_json::Value> {
        if !can_dynamax || self.has_volatile(&ID::new("dynamax")) {
            return None;
        }

        Some(serde_json::json!({
            "canDynamax": true
        }))
    }

    /// Get move request data for protocol
    /// Equivalent to getMoveRequestData in pokemon.ts
    pub fn get_move_request_data(&self) -> serde_json::Value {
        let moves: Vec<serde_json::Value> = self.move_slots.iter().map(|slot| {
            serde_json::json!({
                "move": slot.move_name,
                "id": slot.id.as_str(),
                "pp": slot.pp,
                "maxpp": slot.maxpp,
                "target": slot.target,
                "disabled": slot.disabled
            })
        }).collect();

        serde_json::json!({
            "moves": moves,
            "canDynamax": self.can_gigantamax.is_some() || self.dynamax_level > 0
        })
    }

    /// Get switch request data for protocol
    /// Equivalent to getSwitchRequestData in pokemon.ts
    pub fn get_switch_request_data(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
            "species": self.species_id.as_str(),
            "level": self.level,
            "hp": self.hp,
            "maxhp": self.maxhp,
            "status": if self.status.is_empty() { serde_json::Value::Null } else { serde_json::Value::String(self.status.as_str().to_string()) },
            "moves": self.get_moves(),
            "ability": self.ability.as_str(),
            "item": self.item.as_str()
        })
    }

    /// Try to set status with immunity checks
    /// Equivalent to trySetStatus in pokemon.ts
    pub fn try_set_status(&mut self, status_id: ID, _source_effect: Option<&str>) -> bool {
        // Check if already has status
        if !self.status.is_empty() {
            return false;
        }

        // Check for type-based immunities
        let status_str = status_id.as_str();
        match status_str {
            "brn" => {
                // Fire types immune to burn
                if self.has_type("fire") {
                    return false;
                }
            }
            "par" => {
                // Electric types immune to paralysis (Gen 6+)
                if self.has_type("electric") {
                    return false;
                }
            }
            "psn" | "tox" => {
                // Poison and Steel types immune to poison
                if self.has_type("poison") || self.has_type("steel") {
                    return false;
                }
            }
            "frz" => {
                // Ice types immune to freeze
                if self.has_type("ice") {
                    return false;
                }
            }
            _ => {}
        }

        self.set_status(status_id)
    }

    /// Use held item
    /// Equivalent to useItem in pokemon.ts
    pub fn use_item(&mut self) -> Option<ID> {
        if self.item.is_empty() {
            return None;
        }
        let item = self.item.clone();
        self.used_item_this_turn = true;
        self.last_item = item.clone();
        self.item = ID::empty();
        self.item_state = EffectState::new(ID::empty());
        Some(item)
    }

    /// Eat held item (berries)
    /// Equivalent to eatItem in pokemon.ts
    pub fn eat_item(&mut self, is_forced: bool) -> Option<ID> {
        if self.item.is_empty() {
            return None;
        }

        // Would check if item is edible (berry)
        // For now, same as use_item
        self.use_item()
    }

    /// Run type effectiveness check
    /// Equivalent to runEffectiveness in pokemon.ts
    pub fn run_effectiveness(&self, move_type: &str) -> f64 {
        crate::data::typechart::get_effectiveness_multi(move_type, &self.types)
    }

    /// Run immunity check
    /// Equivalent to runImmunity in pokemon.ts
    pub fn run_immunity(&self, move_type: &str) -> bool {
        let effectiveness = self.run_effectiveness(move_type);
        effectiveness > 0.0
    }

    /// Run status immunity check
    /// Equivalent to runStatusImmunity in pokemon.ts
    pub fn run_status_immunity(&self, status: &str) -> bool {
        match status {
            "brn" => !self.has_type("fire"),
            "par" => !self.has_type("electric"),
            "psn" | "tox" => !self.has_type("poison") && !self.has_type("steel"),
            "frz" => !self.has_type("ice"),
            "slp" => true, // No type immunity to sleep
            _ => true,
        }
    }

    /// Remove linked volatiles
    /// Equivalent to removeLinkedVolatiles in pokemon.ts
    pub fn remove_linked_volatiles(&mut self, linked_status: &ID) {
        // Remove volatiles that are linked to this one
        // For example, Leech Seed removes when source switches
        let to_remove: Vec<ID> = self.volatiles.keys()
            .filter(|k| k.as_str().starts_with(linked_status.as_str()))
            .cloned()
            .collect();

        for id in to_remove {
            self.volatiles.remove(&id);
        }
    }

    /// Clear volatile with switch flag handling
    /// Equivalent to clearVolatile in pokemon.ts
    pub fn clear_volatile_full(&mut self, include_switch_flags: bool) {
        self.volatiles.clear();
        self.boosts = BoostsTable::default();

        if include_switch_flags {
            self.switch_flag = false;
            self.force_switch_flag = false;
        }

        self.last_move = None;
        self.last_move_used = None;
        self.move_this_turn = None;
    }

    /// Get nature
    /// Equivalent to getNature in pokemon.ts
    /// Note: Nature is applied at stat calculation time; we return default here
    pub fn get_nature(&self) -> &str {
        // In battle, the nature is already applied to stored_stats
        // The actual nature value would need to be stored if needed
        "Hardy" // Default neutral nature
    }

    /// Get status object
    /// Equivalent to getStatus in pokemon.ts
    pub fn get_status(&self) -> Option<&ID> {
        if self.status.is_empty() {
            None
        } else {
            Some(&self.status)
        }
    }

    /// Destroy/cleanup Pokemon
    /// Equivalent to destroy in pokemon.ts
    pub fn destroy(&mut self) {
        self.volatiles.clear();
        self.move_slots.clear();
        self.base_move_slots.clear();
    }

    // =========================================================================
    // TARGET METHODS (ported from pokemon.ts)
    // These methods return indices instead of Pokemon references since the
    // actual Pokemon are owned by the Battle.
    // =========================================================================

    /// Get indices of all allies including self
    /// Equivalent to pokemon.ts alliesAndSelf()
    ///
    /// Returns (side_index, pokemon_index) pairs for all Pokemon on this side
    /// that are alive. In actual use, the battle would filter by active status.
    pub fn allies_and_self_stub(&self) -> Vec<(usize, usize)> {
        // This is a stub - full implementation needs battle context
        // Would return all pokemon on the same side that are alive
        vec![(self.side_index, self.position)]
    }

    /// Get indices of allies (not including self)
    /// Equivalent to pokemon.ts allies()
    pub fn allies_stub(&self) -> Vec<(usize, usize)> {
        // This is a stub - full implementation needs battle context
        vec![]
    }

    /// Get indices of adjacent allies (for triples)
    /// Equivalent to pokemon.ts adjacentAllies()
    pub fn adjacent_allies_stub(&self, active_per_half: usize) -> Vec<(usize, usize)> {
        // In singles/doubles, all allies are adjacent
        // In triples, only adjacent positions
        let _ = active_per_half;
        vec![]
    }

    /// Get indices of foes
    /// Equivalent to pokemon.ts foes()
    ///
    /// foe_side_index is the opponent's side index (0 or 1)
    /// include_fainted: whether to include fainted pokemon
    pub fn foes_stub(&self, foe_side_index: usize, include_fainted: bool) -> Vec<(usize, usize)> {
        // This is a stub - full implementation needs battle context
        let _ = (foe_side_index, include_fainted);
        vec![]
    }

    /// Get indices of adjacent foes
    /// Equivalent to pokemon.ts adjacentFoes()
    pub fn adjacent_foes_stub(&self, foe_side_index: usize, active_per_half: usize) -> Vec<(usize, usize)> {
        // This is a stub - full implementation needs battle context
        let _ = (foe_side_index, active_per_half);
        vec![]
    }

    /// Clear the pokemon's status
    /// Equivalent to pokemon.ts clearStatus()
    pub fn clear_status(&mut self) -> bool {
        if !self.status.is_empty() {
            self.status = ID::empty();
            self.status_state = crate::dex_data::EffectState::new(ID::empty());
            true
        } else {
            false
        }
    }

    /// Get move hit data for tracking hit results
    /// Equivalent to pokemon.ts getMoveHitData()
    ///
    /// Returns a new MoveHitData struct for this target
    pub fn get_move_hit_data(&self, _move_id: &ID) -> MoveHitData {
        // Get the stored move hit data if it exists, otherwise create new
        // In the actual implementation, this would be stored per-move per-turn
        MoveHitData::default()
    }

    /// Get move targets for an active move
    /// Equivalent to pokemon.ts getMoveTargets()
    ///
    /// Returns lists of target indices and pressure target indices
    /// This is a stub that returns placeholder data.
    pub fn get_move_targets_stub(
        &self,
        target_side_index: usize,
        target_position: usize,
        _move_target_type: &str,
    ) -> GetMoveTargetsResult {
        // This is a stub - full implementation needs battle context and move data
        GetMoveTargetsResult {
            targets: vec![(target_side_index, target_position)],
            pressure_targets: vec![],
        }
    }
}

/// Result of getMoveTargets
#[derive(Debug, Clone, Default)]
pub struct GetMoveTargetsResult {
    /// Target pokemon indices (side_index, position)
    pub targets: Vec<(usize, usize)>,
    /// Pressure targets for PP deduction
    pub pressure_targets: Vec<(usize, usize)>,
}

/// Move hit data for tracking crit, type effectiveness, etc.
#[derive(Debug, Clone, Default)]
pub struct MoveHitData {
    /// Was this hit a critical hit?
    pub crit: bool,
    /// Type effectiveness modifier (-2 to 2, for 0.25x to 4x)
    pub type_mod: i8,
    /// Actual damage dealt
    pub damage: u32,
    /// Did the move hit the substitute instead?
    pub hit_substitute: bool,
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
