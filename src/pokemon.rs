//! Simulator Pokemon
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module contains the Pokemon struct and related types.

use std::collections::HashMap;
use std::fmt;
use serde::{Deserialize, Serialize};

use crate::dex_data::{ID, Gender, StatsTable, BoostsTable, StatID};
use crate::event_system::EffectState;

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

/// Record of a Pokemon that attacked this Pokemon
/// Equivalent to Attacker interface in pokemon.ts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attacker {
    /// Source Pokemon (side_idx, poke_idx)
    pub source: (usize, usize),
    /// Damage dealt
    pub damage: i32,
    /// Whether this attack happened this turn
    pub this_turn: bool,
    /// Move ID used
    pub move_id: Option<ID>,
    /// Source slot
    pub slot: (usize, usize),
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
    pub base_species: ID,
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
    pub maxhp: i32,
    pub base_maxhp: i32,
    pub hp: i32,
    pub max_hp_undynamaxed: i32,

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
    pub known_type: Option<String>,  // Known type for illusion/disguise mechanics

    // Tera
    pub tera_type: Option<String>,
    pub terastallized: Option<String>,
    pub can_terastallize: Option<String>,

    // Move slots
    pub base_move_slots: Vec<MoveSlot>,
    pub move_slots: Vec<MoveSlot>,

    // Hidden Power type
    pub hp_type: Option<String>,

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
    pub sub_fainted: bool,  // Gen 1: Substitute fainted (different from main faint)

    // Ability-specific flags
    pub sword_boost: bool,  // Intrepid Sword / Dauntless Shield

    // Turn state
    pub last_move: Option<ID>,
    pub last_move_used: Option<ID>,
    pub last_move_target_loc: Option<i8>,
    pub move_this_turn: Option<ID>,
    pub move_this_turn_result: Option<bool>,
    pub move_last_turn_result: Option<bool>,
    pub hurt_this_turn: Option<i32>,
    pub last_damage: i32,
    pub times_attacked: i32,

    // Counters
    pub active_turns: i32,
    pub active_move_actions: i32,
    pub previously_switched_in: i32,
    pub is_started: bool,
    pub during_move: bool,

    // Calculated values
    pub weight_hg: i32,
    pub speed: i32,

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
            base_species: species_id.clone(),
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
            max_hp_undynamaxed: 0,
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
            known_type: None,  // Initially null, set when type becomes known (e.g., Illusion breaks)

            tera_type: set.tera_type.clone(),
            terastallized: None,
            can_terastallize: set.tera_type.clone(),

            base_move_slots: move_slots.clone(),
            move_slots,

            hp_type: set.hptype.clone(),

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
            sub_fainted: false,

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

    /// Get health string for protocol messages
    // TypeScript source:
    // 	getHealth() {
    // 		if (!this.hp) {
    // 			return {
    // 				side: this.side.id,
    // 				secret: '0 fnt',
    // 				shared: '0 fnt',
    // 				percentage: 0,
    // 			};
    // 		}
    // 		let secret = `${this.hp}/${this.maxhp}`;
    // 		const ratio = this.hp / this.maxhp;
    // 		const percentage = Math.ceil(ratio * 100);
    // 		if ((this.side.battle.reportExactHP || this.side.battle.reportPercentages) && this.side.battle.gen > 1) {
    // 			return {
    // 				side: this.side.id,
    // 				secret,
    // 				shared: (this.side.battle.reportExactHP ? secret : `${percentage}/100`),
    // 				percentage,
    // 			};
    // 		}
    // 		let ratioString: string;
    // 		if (percentage === 100 && ratio < 1) {
    // 			if (ratio >= 0.995) {
    // 				ratioString = '100y';
    // 			} else {
    // 				ratioString = '99y';
    // 			}
    // 		} else {
    // 			ratioString = percentage + 'y';
    // 		}
    // 		if (!this.status) {
    // 			secret += ' 100';
    // 		} else if (this.status === 'slp') {
    // 			secret += ` slp ${this.statusState.time}`;
    // 		} else {
    // 			secret += ` ${this.status}`;
    // 		}
    // 		return {
    // 			side: this.side.id,
    // 			secret,
    // 			shared: ratioString,
    // 			percentage,
    // 		};
    // 	}
    //
    pub fn get_health(&self) -> String {
        if self.hp == 0 {
            return "0 fnt".to_string();
        }

        let mut secret = format!("{}/{}", self.hp, self.maxhp);

        if !self.status.is_empty() {
            if self.status.as_str() == "slp" {
                // Would need statusState.time for full implementation
                secret.push_str(" slp");
            } else {
                secret.push_str(&format!(" {}", self.status.as_str()));
            }
        }

        secret
    }


    /// Take damage
    pub fn take_damage(&mut self, damage: i32) -> i32 {
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
    // TypeScript source:
    // /** Returns the amount of damage actually healed */
    // 	heal(d: number, source: Pokemon | null = null, effect: Effect | null = null) {
    // 		if (!this.hp) return false;
    // 		d = this.battle.trunc(d);
    // 		if (isNaN(d)) return false;
    // 		if (d <= 0) return false;
    // 		if (this.hp >= this.maxhp) return false;
    // 		this.hp += d;
    // 		if (this.hp > this.maxhp) {
    // 			d -= this.hp - this.maxhp;
    // 			this.hp = this.maxhp;
    // 		}
    // 		return d;
    // 	}
    //
    pub fn heal(&mut self, amount: i32) -> i32 {
        if self.hp >= self.maxhp {
            return 0;
        }
        let actual = amount.min(self.maxhp - self.hp);
        self.hp += actual;
        actual
    }

    /// Set status condition
    // TypeScript source:
    // 
    // 
    // 	setStatus(
    // 		status: string | Condition,
    // 		source: Pokemon | null = null,
    // 		sourceEffect: Effect | null = null,
    // 		ignoreImmunities = false
    // 	) {
    // 		if (!this.hp) return false;
    // 		status = this.battle.dex.conditions.get(status);
    // 		if (this.battle.event) {
    // 			if (!source) source = this.battle.event.source;
    // 			if (!sourceEffect) sourceEffect = this.battle.effect;
    // 		}
    // 		if (!source) source = this;
    // 
    // 		if (this.status === status.id) {
    // 			if ((sourceEffect as Move)?.status === this.status) {
    // 				this.battle.add('-fail', this, this.status);
    // 			} else if ((sourceEffect as Move)?.status) {
    // 				this.battle.add('-fail', source);
    // 				this.battle.attrLastMove('[still]');
    // 			}
    // 			return false;
    // 		}
    // 
    // 		if (
    // 			!ignoreImmunities && status.id && !(source?.hasAbility('corrosion') && ['tox', 'psn'].includes(status.id))
    // 		) {
    // 			// the game currently never ignores immunities
    // 			if (!this.runStatusImmunity(status.id === 'tox' ? 'psn' : status.id)) {
    // 				this.battle.debug('immune to status');
    // 				if ((sourceEffect as Move)?.status) {
    // 					this.battle.add('-immune', this);
    // 				}
    // 				return false;
    // 			}
    // 		}
    // 		const prevStatus = this.status;
    // 		const prevStatusState = this.statusState;
    // 		if (status.id) {
    // 			const result: boolean = this.battle.runEvent('SetStatus', this, source, sourceEffect, status);
    // 			if (!result) {
    // 				this.battle.debug('set status [' + status.id + '] interrupted');
    // 				return result;
    // 			}
    // 		}
    // 
    // 		this.status = status.id;
    // 		this.statusState = this.battle.initEffectState({ id: status.id, target: this });
    // 		if (source) this.statusState.source = source;
    // 		if (status.duration) this.statusState.duration = status.duration;
    // 		if (status.durationCallback) {
    // 			this.statusState.duration = status.durationCallback.call(this.battle, this, source, sourceEffect);
    // 		}
    // 
    // 		if (status.id && !this.battle.singleEvent('Start', status, this.statusState, this, source, sourceEffect)) {
    // 			this.battle.debug('status start [' + status.id + '] interrupted');
    // 			// cancel the setstatus
    // 			this.status = prevStatus;
    // 			this.statusState = prevStatusState;
    // 			return false;
    // 		}
    // 		if (status.id && !this.battle.runEvent('AfterSetStatus', this, source, sourceEffect, status)) {
    // 			return false;
    // 		}
    // 		return true;
    // 	}
    //
    pub fn set_status(&mut self, status: ID) -> bool {
        if !self.status.is_empty() {
            return false;
        }
        self.status = status.clone();
        self.status_state = EffectState::new(status);
        true
    }

    /// Cure status condition
    // TypeScript source:
    // /** Unlike clearStatus, gives cure message */
    // 	cureStatus(silent = false) {
    // 		if (!this.hp || !this.status) return false;
    // 		this.battle.add('-curestatus', this, this.status, silent ? '[silent]' : '[msg]');
    // 		if (this.status === 'slp' && this.removeVolatile('nightmare')) {
    // 			this.battle.add('-end', this, 'Nightmare', '[silent]');
    // 		}
    // 		this.setStatus('');
    // 		return true;
    // 	}
    //
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
    // 
    // 	addVolatile(
    // 		status: string | Condition, source: Pokemon | null = null, sourceEffect: Effect | null = null,
    // 		linkedStatus: string | Condition | null = null
    // 	): boolean | any {
    // 		let result;
    // 		status = this.battle.dex.conditions.get(status);
    // 		if (!this.hp && !status.affectsFainted) return false;
    // 		if (linkedStatus && source && !source.hp) return false;
    // 		if (this.battle.event) {
    // 			if (!source) source = this.battle.event.source;
    // 			if (!sourceEffect) sourceEffect = this.battle.effect;
    // 		}
    // 		if (!source) source = this;
    // 
    // 		if (this.volatiles[status.id]) {
    // 			if (!status.onRestart) return false;
    // 			return this.battle.singleEvent('Restart', status, this.volatiles[status.id], this, source, sourceEffect);
    // 		}
    // 		if (!this.runStatusImmunity(status.id)) {
    // 			this.battle.debug('immune to volatile status');
    // 			if ((sourceEffect as Move)?.status) {
    // 				this.battle.add('-immune', this);
    // 			}
    // 			return false;
    // 		}
    // 		result = this.battle.runEvent('TryAddVolatile', this, source, sourceEffect, status);
    // 		if (!result) {
    // 			this.battle.debug('add volatile [' + status.id + '] interrupted');
    // 			return result;
    // 		}
    // 		this.volatiles[status.id] = this.battle.initEffectState({ id: status.id, name: status.name, target: this });
    // 		if (source) {
    // 			this.volatiles[status.id].source = source;
    // 			this.volatiles[status.id].sourceSlot = source.getSlot();
    // 		}
    // 		if (sourceEffect) this.volatiles[status.id].sourceEffect = sourceEffect;
    // 		if (status.duration) this.volatiles[status.id].duration = status.duration;
    // 		if (status.durationCallback) {
    // 			this.volatiles[status.id].duration = status.durationCallback.call(this.battle, this, source, sourceEffect);
    // 		}
    // 		result = this.battle.singleEvent('Start', status, this.volatiles[status.id], this, source, sourceEffect);
    // 		if (!result) {
    // 			// cancel
    // 			delete this.volatiles[status.id];
    // 			return result;
    // 		}
    // 		if (linkedStatus && source) {
    // 			if (!source.volatiles[linkedStatus.toString()]) {
    // 				source.addVolatile(linkedStatus, this, sourceEffect);
    // 				source.volatiles[linkedStatus.toString()].linkedPokemon = [this];
    // 				source.volatiles[linkedStatus.toString()].linkedStatus = status;
    // 			} else {
    // 				source.volatiles[linkedStatus.toString()].linkedPokemon.push(this);
    // 			}
    // 			this.volatiles[status.toString()].linkedPokemon = [source];
    // 			this.volatiles[status.toString()].linkedStatus = linkedStatus;
    // 		}
    // 		return true;
    // 	}
    //
    pub fn add_volatile(&mut self, id: ID) -> bool {
        if self.volatiles.contains_key(&id) {
            return false;
        }
        self.volatiles.insert(id.clone(), EffectState::new(id));
        true
    }

    /// Remove a volatile condition
    // 
    // 	removeVolatile(status: string | Effect) {
    // 		if (!this.hp) return false;
    // 		status = this.battle.dex.conditions.get(status) as Effect;
    // 		if (!this.volatiles[status.id]) return false;
    // 		const { linkedPokemon, linkedStatus } = this.volatiles[status.id];
    // 		this.battle.singleEvent('End', status, this.volatiles[status.id], this);
    // 		delete this.volatiles[status.id];
    // 		if (linkedPokemon) {
    // 			this.removeLinkedVolatiles(linkedStatus, linkedPokemon);
    // 		}
    // 		return true;
    // 	}
    //
    pub fn remove_volatile(&mut self, id: &ID) -> bool {
        self.volatiles.remove(id).is_some()
    }

    /// Check if Pokemon has a specific volatile
    pub fn has_volatile(&self, id: &ID) -> bool {
        self.volatiles.contains_key(id)
    }

    /// Get volatile state
    // 
    // 	getVolatile(status: string | Effect) {
    // 		status = this.battle.dex.conditions.get(status) as Effect;
    // 		if (!this.volatiles[status.id]) return null;
    // 		return status;
    // 	}
    //
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
    // 
    // 	getStat(statName: StatIDExceptHP, unboosted?: boolean, unmodified?: boolean) {
    // 		statName = toID(statName) as StatIDExceptHP;
    // 		// @ts-expect-error type checking prevents 'hp' from being passed, but we're paranoid
    // 		if (statName === 'hp') throw new Error("Please read `maxhp` directly");
    // 
    // 		// base stat
    // 		let stat = this.storedStats[statName];
    // 
    // 		// Download ignores Wonder Room's effect, but this results in
    // 		// stat stages being calculated on the opposite defensive stat
    // 		if (unmodified && 'wonderroom' in this.battle.field.pseudoWeather) {
    // 			if (statName === 'def') {
    // 				statName = 'spd';
    // 			} else if (statName === 'spd') {
    // 				statName = 'def';
    // 			}
    // 		}
    // 
    // 		// stat boosts
    // 		if (!unboosted) {
    // 			let boosts = this.boosts;
    // 			if (!unmodified) {
    // 				boosts = this.battle.runEvent('ModifyBoost', this, null, null, { ...boosts });
    // 			}
    // 			let boost = boosts[statName];
    // 			const boostTable = [1, 1.5, 2, 2.5, 3, 3.5, 4];
    // 			if (boost > 6) boost = 6;
    // 			if (boost < -6) boost = -6;
    // 			if (boost >= 0) {
    // 				stat = Math.floor(stat * boostTable[boost]);
    // 			} else {
    // 				stat = Math.floor(stat / boostTable[-boost]);
    // 			}
    // 		}
    // 
    // 		// stat modifier effects
    // 		if (!unmodified) {
    // 			const statTable: { [s in StatIDExceptHP]: string } = { atk: 'Atk', def: 'Def', spa: 'SpA', spd: 'SpD', spe: 'Spe' };
    // 			stat = this.battle.runEvent('Modify' + statTable[statName], this, null, null, stat);
    // 		}
    // 
    // 		if (statName === 'spe' && stat > 10000 && !this.battle.format.battle?.trunc) stat = 10000;
    // 		return stat;
    // 	}
    //
    pub fn get_stat(&self, stat: StatID, unboosted: bool) -> i32 {
        let base = self.stored_stats.get(stat);
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
            (2 + boost as i32, 2)
        } else {
            (2, 2 + (-boost) as i32)
        };

        base * numerator / denominator
    }

    /// Apply a boost
    pub fn boost(&mut self, boost_id: crate::dex_data::BoostID, amount: i8) -> i8 {
        self.boosts.boost(boost_id, amount)
    }

    /// Clear all boosts
    // 
    // 	clearBoosts() {
    // 		let boostName: BoostID;
    // 		for (boostName in this.boosts) {
    // 			this.boosts[boostName] = 0;
    // 		}
    // 	}
    //
    pub fn clear_boosts(&mut self) {
        self.boosts.clear();
    }

    /// Update speed (called when boosts or conditions change)
    // 
    // 	updateSpeed() {
    // 		this.speed = this.getActionSpeed();
    // 	}
    //
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
    // 
    // 	getSlot(): PokemonSlot {
    // 		const positionOffset = Math.floor(this.side.n / 2) * this.side.active.length;
    // 		const positionLetter = 'abcdef'.charAt(this.position + positionOffset);
    // 		return (this.side.id + positionLetter) as PokemonSlot;
    // 	}
    //
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
    // 
    // 	deductPP(move: string | Move, amount?: number | null, target?: Pokemon | null | false) {
    // 		const gen = this.battle.gen;
    // 		move = this.battle.dex.moves.get(move);
    // 		const ppData = this.getMoveData(move);
    // 		if (!ppData) return 0;
    // 		ppData.used = true;
    // 		if (!ppData.pp && gen > 1) return 0;
    // 
    // 		if (!amount) amount = 1;
    // 		ppData.pp -= amount;
    // 		if (ppData.pp < 0 && gen > 1) {
    // 			amount += ppData.pp;
    // 			ppData.pp = 0;
    // 		}
    // 		return amount;
    // 	}
    //
    pub fn deduct_pp(&mut self, move_id: &ID, amount: u8) -> bool {
        if let Some(slot) = self.move_slots.iter_mut().find(|s| &s.id == move_id) {
            slot.pp = slot.pp.saturating_sub(amount);
            slot.used = true;
            true
        } else {
            false
        }
    }

    /// Set PP for a specific move (used by moves like Grudge)
    pub fn set_pp(&mut self, move_id: &ID, pp: u8) -> bool {
        if let Some(slot) = self.move_slots.iter_mut().find(|s| &s.id == move_id) {
            slot.pp = pp;
            true
        } else {
            false
        }
    }

    /// Check if Pokemon has a specific move
    // 
    // 	hasMove(moveid: string) {
    // 		moveid = toID(moveid);
    // 		if (moveid.substr(0, 11) === 'hiddenpower') moveid = 'hiddenpower';
    // 		for (const moveSlot of this.moveSlots) {
    // 			if (moveid === moveSlot.id) {
    // 				return moveid;
    // 			}
    // 		}
    // 		return false;
    // 	}
    //
    pub fn has_move(&self, move_id: &str) -> bool {
        let id = crate::dex_data::to_id(move_id);
        self.move_slots.iter().any(|slot| slot.id.as_str() == id)
    }

    /// Get the types of this Pokemon
    // 
    // 	getTypes(excludeAdded?: boolean, preterastallized?: boolean): string[] {
    // 		if (!preterastallized && this.terastallized && this.terastallized !== 'Stellar') {
    // 			return [this.terastallized];
    // 		}
    // 		const types = this.battle.runEvent('Type', this, null, null, this.types);
    // 		if (!types.length) types.push(this.battle.gen >= 5 ? 'Normal' : '???');
    // 		if (!excludeAdded && this.addedType) return types.concat(this.addedType);
    // 		return types;
    // 	}
    //
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
    // 
    // 	isGrounded(negateImmunity = false) {
    // 		if ('gravity' in this.battle.field.pseudoWeather) return true;
    // 		if ('ingrain' in this.volatiles && this.battle.gen >= 4) return true;
    // 		if ('smackdown' in this.volatiles) return true;
    // 		const item = (this.ignoringItem() ? '' : this.item);
    // 		if (item === 'ironball') return true;
    // 		// If a Fire/Flying type uses Burn Up and Roost, it becomes ???/Flying-type, but it's still grounded.
    // 		if (!negateImmunity && this.hasType('Flying') && !(this.hasType('???') && 'roost' in this.volatiles)) return false;
    // 		if (this.hasAbility('levitate') && !this.battle.suppressingAbility(this)) return null;
    // 		if ('magnetrise' in this.volatiles) return false;
    // 		if ('telekinesis' in this.volatiles) return false;
    // 		return item !== 'airballoon';
    // 	}
    //
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
    // 
    // 	isSemiInvulnerable() {
    // 		return (this.volatiles['fly'] || this.volatiles['bounce'] || this.volatiles['dive'] || this.volatiles['dig'] ||
    // 			this.volatiles['phantomforce'] || this.volatiles['shadowforce'] || this.isSkyDropped());
    // 	}
    //
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
    // TypeScript source:
    // /** Specifically: is protected against a single-target damaging move */
    // 	isProtected() {
    // 		return !!(
    // 			this.volatiles['protect'] || this.volatiles['detect'] || this.volatiles['maxguard'] ||
    // 			this.volatiles['kingsshield'] || this.volatiles['spikyshield'] || this.volatiles['banefulbunker'] ||
    // 			this.volatiles['obstruct'] || this.volatiles['silktrap'] || this.volatiles['burningbulwark']
    // 		);
    // 	}
    //
    pub fn is_protected(&self) -> bool {
        self.has_volatile(&ID::new("protect")) ||
        self.has_volatile(&ID::new("banefulbunker")) ||
        self.has_volatile(&ID::new("kingsshield")) ||
        self.has_volatile(&ID::new("spikyshield")) ||
        self.has_volatile(&ID::new("silktrap")) ||
        self.has_volatile(&ID::new("burningbulwark"))
    }

    /// Get effective weather considering abilities
    // TypeScript source:
    // /**
    // 	 * Like Field.effectiveWeather(), but ignores sun and rain if
    // 	 * the Utility Umbrella is active for the Pokemon.
    // 	 */
    // 	effectiveWeather() {
    // 		const weather = this.battle.field.effectiveWeather();
    // 		switch (weather) {
    // 		case 'sunnyday':
    // 		case 'raindance':
    // 		case 'desolateland':
    // 		case 'primordialsea':
    // 			if (this.hasItem('utilityumbrella')) return '';
    // 		}
    // 		return weather;
    // 	}
    //
    pub fn effective_weather(&self, field_weather: &str) -> String {
        // Cloud Nine and Air Lock negate weather
        // This would normally check all Pokemon on field
        // For now just return the field weather
        field_weather.to_string()
    }

    /// Check if Pokemon has a specific item
    // 
    // 	hasItem(item: string | string[]) {
    // 		if (Array.isArray(item)) {
    // 			if (!item.map(toID).includes(this.item)) return false;
    // 		} else {
    // 			if (toID(item) !== this.item) return false;
    // 		}
    // 		return !this.ignoringItem();
    // 	}
    //
    pub fn has_item(&self, items: &[&str]) -> bool {
        let item_id = self.item.as_str();
        items.iter().any(|&i| crate::dex_data::to_id(i) == item_id)
    }

    /// Check if Pokemon has a specific ability
    // 
    // 	hasAbility(ability: string | string[]) {
    // 		if (Array.isArray(ability)) {
    // 			if (!ability.map(toID).includes(this.ability)) return false;
    // 		} else {
    // 			if (toID(ability) !== this.ability) return false;
    // 		}
    // 		return !this.ignoringAbility();
    // 	}
    //
    pub fn has_ability(&self, abilities: &[&str]) -> bool {
        let ability_id = self.ability.as_str();
        abilities.iter().any(|&a| crate::dex_data::to_id(a) == ability_id)
    }

    /// Copy volatiles from another Pokemon (for Baton Pass, etc.)
    // 
    // 	copyVolatileFrom(pokemon: Pokemon, switchCause?: string | boolean) {
    // 		this.clearVolatile();
    // 		if (switchCause !== 'shedtail') this.boosts = pokemon.boosts;
    // 		for (const i in pokemon.volatiles) {
    // 			if (switchCause === 'shedtail' && i !== 'substitute') continue;
    // 			if (this.battle.dex.conditions.getByID(i as ID).noCopy) continue;
    // 			// shallow clones
    // 			this.volatiles[i] = this.battle.initEffectState({ ...pokemon.volatiles[i], target: this });
    // 			if (this.volatiles[i].linkedPokemon) {
    // 				delete pokemon.volatiles[i].linkedPokemon;
    // 				delete pokemon.volatiles[i].linkedStatus;
    // 				for (const linkedPoke of this.volatiles[i].linkedPokemon) {
    // 					const linkedPokeLinks = linkedPoke.volatiles[this.volatiles[i].linkedStatus].linkedPokemon;
    // 					linkedPokeLinks[linkedPokeLinks.indexOf(pokemon)] = this;
    // 				}
    // 			}
    // 		}
    // 		pokemon.clearVolatile();
    // 		for (const i in this.volatiles) {
    // 			const volatile = this.getVolatile(i) as Condition;
    // 			this.battle.singleEvent('Copy', volatile, this.volatiles[i], this);
    // 		}
    // 	}
    //
    pub fn copy_volatile_from(&mut self, source: &Pokemon, copy_type: &str) {
        match copy_type {
            "copyvolatile" | "batonpass" => {
                // Copy stat boosts
                self.boosts = source.boosts;

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
    // 
    // 	/* Commented out for now until a use for Combat Power is found in Let's Go
    // 	getCombatPower() {
    // 		let statSum = 0;
    // 		let awakeningSum = 0;
    // 		for (const stat in this.stats) {
    // 			statSum += this.calculateStat(stat, this.boosts[stat as BoostName]);
    // 			awakeningSum += this.calculateStat(
    // 				stat, this.boosts[stat as BoostName]) + this.set.evs[stat];
    // 		}
    // 		const combatPower = Math.floor(Math.floor(statSum * this.level * 6 / 100) +
    // 			(Math.floor(awakeningSum) * Math.floor((this.level * 4) / 100 + 2)));
    // 		return this.battle.clampIntRange(combatPower, 0, 10000);
    // 	}
    // 	*/
    // 
    // 	getWeight() {
    // 		const weighthg = this.battle.runEvent('ModifyWeight', this, null, null, this.weighthg);
    // 		return Math.max(1, weighthg);
    // 	}
    //
    pub fn get_weight(&self) -> i32 {
        // Base weight would come from species data
        // For now return stored weight
        self.weight_hg
    }

    /// Set a new type (for moves like Soak, Forest's Curse, etc.)
    // TypeScript source:
    // /**
    // 	 * Sets a type (except on Arceus, who resists type changes)
    // 	 */
    // 	setType(newType: string | string[], enforce = false) {
    // 		if (!enforce) {
    // 			// No Pokemon should be able to have Stellar as a base type
    // 			if (typeof newType === 'string' ? newType === 'Stellar' : newType.includes('Stellar')) return false;
    // 			// First type of Arceus, Silvally cannot be normally changed
    // 			if ((this.battle.gen >= 5 && (this.species.num === 493 || this.species.num === 773)) ||
    // 				(this.battle.gen === 4 && this.hasAbility('multitype'))) {
    // 				return false;
    // 			}
    // 			// Terastallized Pokemon cannot have their base type changed except via forme change
    // 			if (this.terastallized) return false;
    // 		}
    // 
    // 		if (!newType) throw new Error("Must pass type to setType");
    // 		this.types = (typeof newType === 'string' ? [newType] : newType);
    // 		this.addedType = '';
    // 		this.knownType = true;
    // 		this.apparentType = this.types.join('/');
    // 
    // 		return true;
    // 	}
    //
    pub fn set_type(&mut self, new_types: Vec<String>) {
        self.types = new_types;
    }

    /// Add a type (for Forest's Curse, Trick-or-Treat)
    // TypeScript source:
    // /** Removes any types added previously and adds another one. */
    // 	addType(newType: string) {
    // 		if (this.terastallized) return false;
    // 		this.addedType = newType;
    // 		return true;
    // 	}
    //
    pub fn add_type(&mut self, new_type: String) {
        if !self.types.contains(&new_type) {
            self.added_type = Some(new_type);
        }
    }

    /// Get positive boost count (for Stored Power, etc.)
    // 
    // 	positiveBoosts() {
    // 		let boosts = 0;
    // 		let boost: BoostID;
    // 		for (boost in this.boosts) {
    // 			if (this.boosts[boost] > 0) boosts += this.boosts[boost];
    // 		}
    // 		return boosts;
    // 	}
    //
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
    // 
    // 	getActionSpeed() {
    // 		let speed = this.getStat('spe', false, false);
    // 		const trickRoomCheck = this.battle.ruleTable.has('twisteddimensionmod') ?
    // 			!this.battle.field.getPseudoWeather('trickroom') : this.battle.field.getPseudoWeather('trickroom');
    // 		if (trickRoomCheck) {
    // 			speed = 10000 - speed;
    // 		}
    // 		return this.battle.trunc(speed, 13);
    // 	}
    //
    pub fn get_action_speed(&self) -> i32 {
        let mut speed = self.get_stat(StatID::Spe, false);

        // Paralysis halves speed
        if self.status.as_str() == "par" {
            speed /= 2;
        }

        speed
    }

    /// Disable a move
    // 
    // 	disableMove(moveid: string, isHidden?: boolean, sourceEffect?: Effect) {
    // 		if (!sourceEffect && this.battle.event) {
    // 			sourceEffect = this.battle.effect;
    // 		}
    // 		moveid = toID(moveid);
    // 
    // 		for (const moveSlot of this.moveSlots) {
    // 			if (moveSlot.id === moveid && moveSlot.disabled !== true) {
    // 				moveSlot.disabled = isHidden ? 'hidden' : true;
    // 				moveSlot.disabledSource = (sourceEffect?.name || moveSlot.move);
    // 			}
    // 		}
    // 	}
    //
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
    //
    // 	toString() {
    // 		const fullname = (this.illusion) ? this.illusion.fullname : this.fullname;
    // 		return this.isActive ? this.getSlot() + fullname.slice(2) : fullname;
    // 	}
    //
    /// Get updated details string for protocol
    // 
    // 	getUpdatedDetails(level?: number) {
    // 		let name = this.species.name;
    // 		if (['Greninja-Bond', 'Rockruff-Dusk'].includes(name)) name = this.species.baseSpecies;
    // 		if (!level) level = this.level;
    // 		return name + (level === 100 ? '' : `, L${level}`) +
    // 			(this.gender === '' ? '' : `, ${this.gender}`) + (this.set.shiny ? ', shiny' : '');
    // 	}
    //
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
    // 
    // 	calculateStat(statName: StatIDExceptHP, boost: number, modifier?: number, statUser?: Pokemon) {
    // 		statName = toID(statName) as StatIDExceptHP;
    // 		// @ts-expect-error type checking prevents 'hp' from being passed, but we're paranoid
    // 		if (statName === 'hp') throw new Error("Please read `maxhp` directly");
    // 
    // 		// base stat
    // 		let stat = this.storedStats[statName];
    // 
    // 		// Wonder Room swaps defenses before calculating anything else
    // 		if ('wonderroom' in this.battle.field.pseudoWeather) {
    // 			if (statName === 'def') {
    // 				stat = this.storedStats['spd'];
    // 			} else if (statName === 'spd') {
    // 				stat = this.storedStats['def'];
    // 			}
    // 		}
    // 
    // 		// stat boosts
    // 		let boosts: SparseBoostsTable = {};
    // 		const boostName = statName as BoostID;
    // 		boosts[boostName] = boost;
    // 		boosts = this.battle.runEvent('ModifyBoost', statUser || this, null, null, boosts);
    // 		boost = boosts[boostName]!;
    // 		const boostTable = [1, 1.5, 2, 2.5, 3, 3.5, 4];
    // 		if (boost > 6) boost = 6;
    // 		if (boost < -6) boost = -6;
    // 		if (boost >= 0) {
    // 			stat = Math.floor(stat * boostTable[boost]);
    // 		} else {
    // 			stat = Math.floor(stat / boostTable[-boost]);
    // 		}
    // 
    // 		// stat modifier
    // 		return this.battle.modify(stat, (modifier || 1));
    // 	}
    //
    pub fn calculate_stat(&self, stat: StatID, boost: i8, modifier: f64) -> i32 {
        if stat == StatID::HP {
            return self.maxhp;
        }

        // Get base stat
        let base_stat = self.stored_stats.get(stat);

        // Apply boost
        let clamped_boost = boost.clamp(-6, 6);
        let boost_table: [f64; 7] = [1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0];

        let boosted_stat = if clamped_boost >= 0 {
            (base_stat as f64 * boost_table[clamped_boost as usize]) as i32
        } else {
            (base_stat as f64 / boost_table[(-clamped_boost) as usize]) as i32
        };

        // Apply modifier
        ((boosted_stat as f64) * modifier) as i32
    }

    /// Get best stat (for Beast Boost, Quark Drive, Protosynthesis)
    // TypeScript source:
    // /**
    // 	 * Gets the Pokemon's best stat.
    // 	 * Moved to its own method due to frequent use of the same code.
    // 	 * Used by Beast Boost, Quark Drive, and Protosynthesis.
    // 	 */
    // 	getBestStat(unboosted?: boolean, unmodified?: boolean): StatIDExceptHP {
    // 		let statName: StatIDExceptHP = 'atk';
    // 		let bestStat = 0;
    // 		const stats: StatIDExceptHP[] = ['atk', 'def', 'spa', 'spd', 'spe'];
    // 		for (const i of stats) {
    // 			if (this.getStat(i, unboosted, unmodified) > bestStat) {
    // 				statName = i;
    // 				bestStat = this.getStat(i, unboosted, unmodified);
    // 			}
    // 		}
    // 
    // 		return statName;
    // 	}
    //
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
    // 
    // 	hasType(type: string | string[]) {
    // 		const thisTypes = this.getTypes();
    // 		if (typeof type === 'string') {
    // 			return thisTypes.includes(type);
    // 		}
    // 
    // 		for (const typeName of type) {
    // 			if (thisTypes.includes(typeName)) return true;
    // 		}
    // 		return false;
    // 	}
    //
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
    // TypeScript source:
    // /**
    // 	 * This function only puts the pokemon in the faint queue;
    // 	 * actually setting of this.fainted comes later when the
    // 	 * faint queue is resolved.
    // 	 *
    // 	 * Returns the amount of damage actually dealt
    // 	 */
    // 	faint(source: Pokemon | null = null, effect: Effect | null = null) {
    // 		if (this.fainted || this.faintQueued) return 0;
    // 		const d = this.hp;
    // 		this.hp = 0;
    // 		this.switchFlag = false;
    // 		this.faintQueued = true;
    // 		this.battle.faintQueue.push({
    // 			target: this,
    // 			source,
    // 			effect,
    // 		});
    // 		return d;
    // 	}
    //
    pub fn faint(&mut self) -> i32 {
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
    // 
    // 	damage(d: number, source: Pokemon | null = null, effect: Effect | null = null) {
    // 		if (!this.hp || isNaN(d) || d <= 0) return 0;
    // 		if (d < 1 && d > 0) d = 1;
    // 		d = this.battle.trunc(d);
    // 		this.hp -= d;
    // 		if (this.hp <= 0) {
    // 			d += this.hp;
    // 			this.faint(source, effect);
    // 		}
    // 		return d;
    // 	}
    //
    pub fn damage(&mut self, amount: i32) -> i32 {
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
    // 
    // 	isAlly(pokemon: Pokemon | null) {
    // 		return !!pokemon && (this.side === pokemon.side || this.side.allySide === pokemon.side);
    // 	}
    //
    pub fn is_ally(&self, other_side_index: usize) -> bool {
        self.side_index == other_side_index
    }

    /// Check if this is the same Pokemon (by position and side)
    /// JavaScript pattern: if (target === pokemon) continue;
    pub fn is_same(&self, other: &Pokemon) -> bool {
        self.side_index == other.side_index && self.position == other.position
    }

    /// Check if another Pokemon is adjacent (for targeting)
    // 
    // 	isAdjacent(pokemon2: Pokemon) {
    // 		if (this.fainted || pokemon2.fainted) return false;
    // 		if (this.battle.activePerHalf <= 2) return this !== pokemon2;
    // 		if (this.side === pokemon2.side) return Math.abs(this.position - pokemon2.position) === 1;
    // 		return Math.abs(this.position + pokemon2.position + 1 - this.side.active.length) <= 1;
    // 	}
    //
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
    // 
    // 	getCappedBoost(boosts: SparseBoostsTable) {
    // 		const cappedBoost: SparseBoostsTable = {};
    // 		let boostName: BoostID;
    // 		for (boostName in boosts) {
    // 			const boost = boosts[boostName];
    // 			if (!boost) continue;
    // 			cappedBoost[boostName] = this.battle.clampIntRange(this.boosts[boostName] + boost, -6, 6) - this.boosts[boostName];
    // 		}
    // 		return cappedBoost;
    // 	}
    //
    pub fn get_capped_boost(&self, stat: crate::dex_data::BoostID, amount: i8) -> i8 {
        let current = self.boosts.get(stat);
        let new_value = (current + amount).clamp(-6, 6);
        new_value - current
    }

    /// Boost stat by amount, respecting caps
    // 
    // 	boostBy(boosts: SparseBoostsTable) {
    // 		boosts = this.getCappedBoost(boosts);
    // 		let delta = 0;
    // 		let boostName: BoostID;
    // 		for (boostName in boosts) {
    // 			delta = boosts[boostName]!;
    // 			this.boosts[boostName] += delta;
    // 		}
    // 		return delta;
    // 	}
    //
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
    // 
    // 	setBoost(boosts: SparseBoostsTable) {
    // 		let boostName: BoostID;
    // 		for (boostName in boosts) {
    // 			this.boosts[boostName] = boosts[boostName]!;
    // 		}
    // 	}
    //
    pub fn set_boost(&mut self, stat: crate::dex_data::BoostID, value: i8) {
        let clamped = value.clamp(-6, 6);
        self.boosts.set(stat, clamped);
    }

    /// Clear ability (set to empty)
    // 
    // 	clearAbility() {
    // 		return this.setAbility('');
    // 	}
    //
    pub fn clear_ability(&mut self) -> ID {
        let old = self.ability.clone();
        self.ability = ID::empty();
        self.ability_state = EffectState::new(ID::empty());
        old
    }

    /// Set ability
    // 
    // 	setAbility(
    // 		ability: string | Ability, source?: Pokemon | null, sourceEffect?: Effect | null,
    // 		isFromFormeChange = false, isTransform = false,
    // 	) {
    // 		if (!this.hp) return false;
    // 		if (typeof ability === 'string') ability = this.battle.dex.abilities.get(ability);
    // 		if (!sourceEffect && this.battle.effect) sourceEffect = this.battle.effect;
    // 		const oldAbility = this.battle.dex.abilities.get(this.ability);
    // 		if (!isFromFormeChange) {
    // 			if (ability.flags['cantsuppress'] || this.getAbility().flags['cantsuppress']) return false;
    // 		}
    // 		if (!isFromFormeChange && !isTransform) {
    // 			const setAbilityEvent: boolean | null = this.battle.runEvent('SetAbility', this, source, sourceEffect, ability);
    // 			if (!setAbilityEvent) return setAbilityEvent;
    // 		}
    // 		this.battle.singleEvent('End', oldAbility, this.abilityState, this, source);
    // 		this.ability = ability.id;
    // 		this.abilityState = this.battle.initEffectState({ id: ability.id, target: this });
    // 		if (sourceEffect && !isFromFormeChange && !isTransform) {
    // 			if (source) {
    // 				this.battle.add('-ability', this, ability.name, oldAbility.name, `[from] ${sourceEffect.fullname}`, `[of] ${source}`);
    // 			} else {
    // 				this.battle.add('-ability', this, ability.name, oldAbility.name, `[from] ${sourceEffect.fullname}`);
    // 			}
    // 		}
    // 		if (ability.id && this.battle.gen > 3 &&
    // 			(!isTransform || oldAbility.id !== ability.id || this.battle.gen <= 4)) {
    // 			this.battle.singleEvent('Start', ability, this.abilityState, this, source);
    // 		}
    // 		return oldAbility.id;
    // 	}
    //
    pub fn set_ability(&mut self, ability_id: ID) -> ID {
        let old = self.ability.clone();
        self.ability = ability_id.clone();
        self.ability_state = EffectState::new(ability_id);
        old
    }

    /// Get ability ID
    // 
    // 	getAbility() {
    // 		return this.battle.dex.abilities.getByID(this.ability);
    // 	}
    //
    pub fn get_ability(&self) -> &ID {
        &self.ability
    }

    /// Clear item
    // 
    // 	clearItem() {
    // 		return this.setItem('');
    // 	}
    //
    pub fn clear_item(&mut self) -> ID {
        let old = self.item.clone();
        self.item = ID::empty();
        self.item_state = EffectState::new(ID::empty());
        old
    }

    /// Set item
    // 
    // 	setItem(item: string | Item, source?: Pokemon, effect?: Effect) {
    // 		if (!this.hp || !this.isActive) return false;
    // 		if (typeof item === 'string') item = this.battle.dex.items.get(item);
    // 
    // 		const effectid = this.battle.effect ? this.battle.effect.id : '';
    // 		if (RESTORATIVE_BERRIES.has('leppaberry' as ID)) {
    // 			const inflicted = ['trick', 'switcheroo'].includes(effectid);
    // 			const external = inflicted && source && !source.isAlly(this);
    // 			this.pendingStaleness = external ? 'external' : 'internal';
    // 		} else {
    // 			this.pendingStaleness = undefined;
    // 		}
    // 		const oldItem = this.getItem();
    // 		const oldItemState = this.itemState;
    // 		this.item = item.id;
    // 		this.itemState = this.battle.initEffectState({ id: item.id, target: this });
    // 		if (oldItem.exists) this.battle.singleEvent('End', oldItem, oldItemState, this);
    // 		if (item.id) {
    // 			this.battle.singleEvent('Start', item, this.itemState, this, source, effect);
    // 		}
    // 		return true;
    // 	}
    //
    pub fn set_item(&mut self, item_id: ID) -> bool {
        if self.hp == 0 || !self.is_active {
            return false;
        }
        self.item = item_id.clone();
        self.item_state = EffectState::new(item_id);
        true
    }

    /// Take item (remove and return it)
    // 
    // 	takeItem(source?: Pokemon) {
    // 		if (!this.item) return false;
    // 		if (!source) source = this;
    // 		if (this.battle.gen <= 4) {
    // 			if (source.itemKnockedOff) return false;
    // 			if (toID(this.ability) === 'multitype') return false;
    // 			if (toID(source.ability) === 'multitype') return false;
    // 		}
    // 		const item = this.getItem();
    // 		if (this.battle.runEvent('TakeItem', this, source, null, item)) {
    // 			this.item = '';
    // 			const oldItemState = this.itemState;
    // 			this.battle.clearEffectState(this.itemState);
    // 			this.pendingStaleness = undefined;
    // 			this.battle.singleEvent('End', item, oldItemState, this);
    // 			this.battle.runEvent('AfterTakeItem', this, null, null, item);
    // 			return item;
    // 		}
    // 		return false;
    // 	}
    //
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
    // 
    // 	getItem() {
    // 		return this.battle.dex.items.getByID(this.item);
    // 	}
    //
    pub fn get_item(&self) -> &ID {
        &self.item
    }

    /// Set HP directly, returns delta
    // TypeScript source:
    // /** Sets HP, returns delta */
    // 	sethp(d: number) {
    // 		if (!this.hp) return 0;
    // 		d = this.battle.trunc(d);
    // 		if (isNaN(d)) return;
    // 		if (d < 1) d = 1;
    // 		d -= this.hp;
    // 		this.hp += d;
    // 		if (this.hp > this.maxhp) {
    // 			d -= this.hp - this.maxhp;
    // 			this.hp = this.maxhp;
    // 		}
    // 		return d;
    // 	}
    //
    pub fn set_hp(&mut self, hp: i32) -> i32 {
        if self.hp == 0 {
            return 0;
        }
        let clamped = hp.clamp(1, self.maxhp);
        let delta = clamped - self.hp;
        self.hp = clamped;
        delta
    }

    /// Record that a move was used
    // 
    // 	moveUsed(move: ActiveMove, targetLoc?: number) {
    // 		this.lastMove = move;
    // 		if (this.battle.gen === 2) this.lastMoveEncore = move;
    // 		this.lastMoveTargetLoc = targetLoc;
    // 		this.moveThisTurn = move.id;
    // 	}
    //
    pub fn move_used(&mut self, move_id: ID, target_loc: Option<i8>) {
        self.last_move = Some(move_id.clone());
        self.last_move_used = Some(move_id.clone());
        self.last_move_target_loc = target_loc;
        self.move_this_turn = Some(move_id);
    }

    /// Check if this is the last active Pokemon on the side
    // 
    // 	isLastActive() {
    // 		if (!this.isActive) return false;
    // 		const allyActive = this.side.active;
    // 		for (let i = this.position + 1; i < allyActive.length; i++) {
    // 			if (allyActive[i] && !allyActive[i].fainted) return false;
    // 		}
    // 		return true;
    // 	}
    //
    pub fn is_last_active(&self) -> bool {
        // This would need access to the side to properly implement
        // For now, just return true if active
        self.is_active
    }

    /// Check if ability is being suppressed
    // 
    // 	ignoringAbility() {
    // 		if (this.battle.gen >= 5 && !this.isActive) return true;
    // 
    // 		// Certain Abilities won't activate while Transformed, even if they ordinarily couldn't be suppressed (e.g. Disguise)
    // 		if (this.getAbility().flags['notransform'] && this.transformed) return true;
    // 		if (this.getAbility().flags['cantsuppress']) return false;
    // 		if (this.volatiles['gastroacid']) return true;
    // 
    // 		// Check if any active pokemon have the ability Neutralizing Gas
    // 		if (this.hasItem('Ability Shield') || this.ability === ('neutralizinggas' as ID)) return false;
    // 		for (const pokemon of this.battle.getAllActive()) {
    // 			// can't use hasAbility because it would lead to infinite recursion
    // 			if (pokemon.ability === ('neutralizinggas' as ID) && !pokemon.volatiles['gastroacid'] &&
    // 				!pokemon.transformed && !pokemon.abilityState.ending && !this.volatiles['commanding']) {
    // 				return true;
    // 			}
    // 		}
    // 
    // 		return false;
    // 	}
    //
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
    // 
    // 	ignoringItem(isFling = false) {
    // 		if (this.getItem().isPrimalOrb) return false;
    // 		if (this.battle.gen >= 5 && !this.isActive) return true;
    // 		if (this.volatiles['embargo'] || this.battle.field.pseudoWeather['magicroom']) return true;
    // 		// check Fling first to avoid infinite recursion
    // 		if (isFling) return this.battle.gen >= 5 && this.hasAbility('klutz');
    // 		return !this.getItem().ignoreKlutz && this.hasAbility('klutz');
    // 	}
    //
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
    // 
    // 	updateMaxHp() {
    // 		const newBaseMaxHp = this.battle.statModify(this.species.baseStats, this.set, 'hp');
    // 		if (newBaseMaxHp === this.baseMaxhp) return;
    // 		this.baseMaxhp = newBaseMaxHp;
    // 		const newMaxHP = this.volatiles['dynamax'] ? (2 * this.baseMaxhp) : this.baseMaxhp;
    // 		this.hp = this.hp <= 0 ? 0 : Math.max(1, newMaxHP - (this.maxhp - this.hp));
    // 		this.maxhp = newMaxHP;
    // 		if (this.hp) this.battle.add('-heal', this, this.getHealth, '[silent]');
    // 	}
    //
    pub fn update_max_hp(&mut self, new_base_max_hp: i32) {
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
    // 
    // 	isSkyDropped() {
    // 		if (this.volatiles['skydrop']) return true;
    // 		for (const foeActive of this.side.foe.active) {
    // 			if (foeActive.volatiles['skydrop'] && foeActive.volatiles['skydrop'].source === this) {
    // 				return true;
    // 			}
    // 		}
    // 		return false;
    // 	}
    //
    pub fn is_sky_dropped(&self) -> bool {
        self.has_volatile(&ID::new("skydrop"))
    }

    /// Get move slot data
    // 
    // 	getMoveData(move: string | Move) {
    // 		move = this.battle.dex.moves.get(move);
    // 		for (const moveSlot of this.moveSlots) {
    // 			if (moveSlot.id === move.id) {
    // 				return moveSlot;
    // 			}
    // 		}
    // 		return null;
    // 	}
    //
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
    // 
    // 	getUndynamaxedHP(amount?: number) {
    // 		const hp = amount || this.hp;
    // 		if (this.volatiles['dynamax']) {
    // 			return Math.ceil(hp * this.baseMaxhp / this.maxhp);
    // 		}
    // 		return hp;
    // 	}
    //
    pub fn get_undynamaxed_hp(&self) -> i32 {
        if self.has_volatile(&ID::new("dynamax")) {
            // Dynamaxed HP is doubled, so divide by 2
            ((self.hp as f64) * (self.base_maxhp as f64) / (self.maxhp as f64)).ceil() as i32
        } else {
            self.hp
        }
    }

    /// Try to trap the Pokemon
    // 
    // 	tryTrap(isHidden = false) {
    // 		if (!this.runStatusImmunity('trapped')) return false;
    // 		if (this.trapped && isHidden) return true;
    // 		this.trapped = isHidden ? 'hidden' : true;
    // 		return true;
    // 	}
    //
    pub fn try_trap(&mut self, is_hidden: bool) -> bool {
        if self.trapped && is_hidden {
            return true;
        }
        self.trapped = true;
        true
    }

    /// Record that this Pokemon was attacked
    // 
    // 	gotAttacked(move: string | Move, damage: number | false | undefined, source: Pokemon) {
    // 		const damageNumber = (typeof damage === 'number') ? damage : 0;
    // 		move = this.battle.dex.moves.get(move);
    // 		this.attackedBy.push({
    // 			source,
    // 			damage: damageNumber,
    // 			move: move.id,
    // 			thisTurn: true,
    // 			slot: source.getSlot(),
    // 			damageValue: damage,
    // 		});
    // 	}
    //
    pub fn got_attacked(&mut self, move_id: ID, damage: i32, _source_side: usize, _source_pos: usize) {
        self.last_damage = damage;
        self.times_attacked += 1;
        // Would store in attackedBy array in full implementation
        let _ = move_id; // Use to avoid warning
    }

    /// Get locked move (for multi-turn moves)
    // TypeScript source:
    // /**
    // 	 * This refers to multi-turn moves like SolarBeam and Outrage and
    // 	 * Sky Drop, which remove all choice (no dynamax, switching, etc).
    // 	 * Don't use it for "soft locks" like Choice Band.
    // 	 */
    // 	getLockedMove(): ID | null {
    // 		const lockedMove = this.battle.runEvent('LockMove', this);
    // 		return (lockedMove === true) ? null : lockedMove;
    // 	}
    //
    pub fn get_locked_move(&self) -> Option<&ID> {
        self.locked_move.as_ref()
    }

    /// Check if max move is disabled
    // TypeScript source:
    // /** This should be passed the base move and not the corresponding max move so we can check how much PP is left. */
    // 	maxMoveDisabled(baseMove: Move | string) {
    // 		baseMove = this.battle.dex.moves.get(baseMove);
    // 		if (!this.getMoveData(baseMove.id)?.pp) return true;
    // 		return !!(baseMove.category === 'Status' && (this.hasItem('assaultvest') || this.volatiles['taunt']));
    // 	}
    //
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
    // 
    // 	transformInto(pokemon: Pokemon, effect?: Effect) {
    // 		const species = pokemon.species;
    // 		if (
    // 			pokemon.fainted || this.illusion || pokemon.illusion || (pokemon.volatiles['substitute'] && this.battle.gen >= 5) ||
    // 			(pokemon.transformed && this.battle.gen >= 2) || (this.transformed && this.battle.gen >= 5) ||
    // 			species.name === 'Eternatus-Eternamax' ||
    // 			(['Ogerpon', 'Terapagos'].includes(species.baseSpecies) && (this.terastallized || pokemon.terastallized)) ||
    // 			this.terastallized === 'Stellar'
    // 		) {
    // 			return false;
    // 		}
    // 
    // 		if (this.battle.dex.currentMod === 'gen1stadium' && (
    // 			species.name === 'Ditto' ||
    // 			(this.species.name === 'Ditto' && pokemon.moves.includes('transform'))
    // 		)) {
    // 			return false;
    // 		}
    // 
    // 		if (!this.setSpecies(species, effect, true)) return false;
    // 
    // 		this.transformed = true;
    // 		this.weighthg = pokemon.weighthg;
    // 
    // 		const types = pokemon.getTypes(true, true);
    // 		this.setType(pokemon.volatiles['roost'] ? pokemon.volatiles['roost'].typeWas : types, true);
    // 		this.addedType = pokemon.addedType;
    // 		this.knownType = this.isAlly(pokemon) && pokemon.knownType;
    // 		this.apparentType = pokemon.apparentType;
    // 
    // 		let statName: StatIDExceptHP;
    // 		for (statName in this.storedStats) {
    // 			this.storedStats[statName] = pokemon.storedStats[statName];
    // 			if (this.modifiedStats) this.modifiedStats[statName] = pokemon.modifiedStats![statName]; // Gen 1: Copy modified stats.
    // 		}
    // 		this.moveSlots = [];
    // 		this.hpType = (this.battle.gen >= 5 ? this.hpType : pokemon.hpType);
    // 		this.hpPower = (this.battle.gen >= 5 ? this.hpPower : pokemon.hpPower);
    // 		this.timesAttacked = pokemon.timesAttacked;
    // 		for (const moveSlot of pokemon.moveSlots) {
    // 			let moveName = moveSlot.move;
    // 			if (moveSlot.id === 'hiddenpower') {
    // 				moveName = 'Hidden Power ' + this.hpType;
    // 			}
    // 			this.moveSlots.push({
    // 				move: moveName,
    // 				id: moveSlot.id,
    // 				pp: moveSlot.maxpp === 1 ? 1 : 5,
    // 				maxpp: this.battle.gen >= 5 ? (moveSlot.maxpp === 1 ? 1 : 5) : moveSlot.maxpp,
    // 				target: moveSlot.target,
    // 				disabled: false,
    // 				used: false,
    // 				virtual: true,
    // 			});
    // 		}
    // 		let boostName: BoostID;
    // 		for (boostName in pokemon.boosts) {
    // 			this.boosts[boostName] = pokemon.boosts[boostName];
    // 		}
    // 		if (this.battle.gen >= 6) {
    // 			// we need to remove all of the overlapping crit volatiles before adding any of them
    // 			const volatilesToCopy = ['dragoncheer', 'focusenergy', 'gmaxchistrike', 'laserfocus'];
    // 			for (const volatile of volatilesToCopy) this.removeVolatile(volatile);
    // 			for (const volatile of volatilesToCopy) {
    // 				if (pokemon.volatiles[volatile]) {
    // 					this.addVolatile(volatile);
    // 					if (volatile === 'gmaxchistrike') this.volatiles[volatile].layers = pokemon.volatiles[volatile].layers;
    // 					if (volatile === 'dragoncheer') this.volatiles[volatile].hasDragonType = pokemon.volatiles[volatile].hasDragonType;
    // 				}
    // 			}
    // 		}
    // 		if (effect) {
    // 			this.battle.add('-transform', this, pokemon, '[from] ' + effect.fullname);
    // 		} else {
    // 			this.battle.add('-transform', this, pokemon);
    // 		}
    // 		if (this.terastallized) {
    // 			this.knownType = true;
    // 			this.apparentType = this.terastallized;
    // 		}
    // 		if (this.battle.gen > 2) this.setAbility(pokemon.ability, this, null, true, true);
    // 
    // 		// Change formes based on held items (for Transform)
    // 		// Only ever relevant in Generation 4 since Generation 3 didn't have item-based forme changes
    // 		if (this.battle.gen === 4) {
    // 			if (this.species.num === 487) {
    // 				// Giratina formes
    // 				if (this.species.name === 'Giratina' && this.item === 'griseousorb') {
    // 					this.formeChange('Giratina-Origin');
    // 				} else if (this.species.name === 'Giratina-Origin' && this.item !== 'griseousorb') {
    // 					this.formeChange('Giratina');
    // 				}
    // 			}
    // 			if (this.species.num === 493) {
    // 				// Arceus formes
    // 				const item = this.getItem();
    // 				const targetForme = (item?.onPlate ? 'Arceus-' + item.onPlate : 'Arceus');
    // 				if (this.species.name !== targetForme) {
    // 					this.formeChange(targetForme);
    // 				}
    // 			}
    // 		}
    // 
    // 		// Pokemon transformed into Ogerpon cannot Terastallize
    // 		// restoring their ability to tera after they untransform is handled ELSEWHERE
    // 		if (['Ogerpon', 'Terapagos'].includes(this.species.baseSpecies) && this.canTerastallize) this.canTerastallize = false;
    // 
    // 		return true;
    // 	}
    //
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
        self.stored_stats = target.stored_stats;

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
        self.boosts = target.boosts;

        // Copy ability
        self.ability = target.ability.clone();

        true
    }

    /// Copy volatiles from another Pokemon (for Baton Pass)
    pub fn copy_volatile_from_full(&mut self, source: &Pokemon, is_shed_tail: bool) {
        // Copy boosts unless Shed Tail
        if !is_shed_tail {
            self.boosts = source.boosts;
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
    // TypeScript source:
    // /**
    // 	 * Changes this Pokemon's species to the given speciesId (or species).
    // 	 * This function only handles changes to stats and type.
    // 	 * Use formeChange to handle changes to ability and sending client messages.
    // 	 */
    // 	setSpecies(rawSpecies: Species, source: Effect | null = this.battle.effect, isTransform = false) {
    // 		const species = this.battle.runEvent('ModifySpecies', this, null, source, rawSpecies);
    // 		if (!species) return null;
    // 		this.species = species;
    // 
    // 		this.setType(species.types, true);
    // 		this.apparentType = rawSpecies.types.join('/');
    // 		this.addedType = species.addedType || '';
    // 		this.knownType = true;
    // 		this.weighthg = species.weighthg;
    // 
    // 		const stats = this.battle.spreadModify(this.species.baseStats, this.set);
    // 		if (this.species.maxHP) stats.hp = this.species.maxHP;
    // 
    // 		if (!this.maxhp) {
    // 			this.baseMaxhp = stats.hp;
    // 			this.maxhp = stats.hp;
    // 			this.hp = stats.hp;
    // 		}
    // 
    // 		if (!isTransform) this.baseStoredStats = stats;
    // 		let statName: StatIDExceptHP;
    // 		for (statName in this.storedStats) {
    // 			this.storedStats[statName] = stats[statName];
    // 			if (this.modifiedStats) this.modifiedStats[statName] = stats[statName]; // Gen 1: Reset modified stats.
    // 		}
    // 		if (this.battle.gen <= 1) {
    // 			// Gen 1: Re-Apply burn and para drops.
    // 			if (this.status === 'par') this.modifyStat!('spe', 0.25);
    // 			if (this.status === 'brn') this.modifyStat!('atk', 0.5);
    // 		}
    // 		this.speed = this.storedStats.spe;
    // 		return species;
    // 	}
    //
    pub fn set_species(&mut self, species_id: ID, types: Vec<String>, weight_hg: i32) {
        self.species_id = species_id;
        self.types = types.clone();
        self.base_types = types;
        self.weight_hg = weight_hg;
    }

    /// Forme change
    // TypeScript source:
    // /**
    // 	 * Changes this Pokemon's forme to match the given speciesId (or species).
    // 	 * This function handles all changes to stats, ability, type, species, etc.
    // 	 * as well as sending all relevant messages sent to the client.
    // 	 */
    // 	formeChange(
    // 		speciesId: string | Species, source: Effect | null = this.battle.effect,
    // 		isPermanent?: boolean, abilitySlot = '0', message?: string
    // 	) {
    // 		const rawSpecies = this.battle.dex.species.get(speciesId);
    // 
    // 		const species = this.setSpecies(rawSpecies, source);
    // 		if (!species) return false;
    // 
    // 		if (this.battle.gen <= 2) return true;
    // 
    // 		// The species the opponent sees
    // 		const apparentSpecies =
    // 			this.illusion ? this.illusion.species.name : species.baseSpecies;
    // 		if (isPermanent) {
    // 			this.baseSpecies = rawSpecies;
    // 			this.details = this.getUpdatedDetails();
    // 			let details = (this.illusion || this).details;
    // 			if (this.terastallized) details += `, tera:${this.terastallized}`;
    // 			this.battle.add('detailschange', this, details);
    // 			this.updateMaxHp();
    // 			if (!source) {
    // 				// Tera forme
    // 				// Ogerpon/Terapagos text goes here
    // 				this.formeRegression = true;
    // 			} else if (source.effectType === 'Item') {
    // 				this.canTerastallize = null; // National Dex behavior
    // 				if (source.zMove) {
    // 					this.battle.add('-burst', this, apparentSpecies, species.requiredItem);
    // 					this.moveThisTurnResult = true; // Ultra Burst counts as an action for Truant
    // 				} else if (source.isPrimalOrb) {
    // 					if (this.illusion) {
    // 						this.ability = '';
    // 						this.battle.add('-primal', this.illusion, species.requiredItem);
    // 					} else {
    // 						this.battle.add('-primal', this, species.requiredItem);
    // 					}
    // 				} else {
    // 					this.battle.add('-mega', this, apparentSpecies, species.requiredItem);
    // 					this.moveThisTurnResult = true; // Mega Evolution counts as an action for Truant
    // 				}
    // 				this.formeRegression = true;
    // 			} else if (source.effectType === 'Status') {
    // 				// Shaymin-Sky -> Shaymin
    // 				this.battle.add('-formechange', this, species.name, message);
    // 			}
    // 		} else {
    // 			if (source?.effectType === 'Ability') {
    // 				this.battle.add('-formechange', this, species.name, message, `[from] ability: ${source.name}`);
    // 			} else {
    // 				this.battle.add('-formechange', this, this.illusion ? this.illusion.species.name : species.name, message);
    // 			}
    // 		}
    // 		if (isPermanent && (!source || !['disguise', 'iceface'].includes(source.id))) {
    // 			if (this.illusion && source) {
    // 				// Tera forme by Ogerpon or Terapagos breaks the Illusion
    // 				this.ability = ''; // Don't allow Illusion to wear off
    // 			}
    // 			const ability = species.abilities[abilitySlot] || species.abilities['0'];
    // 			// Ogerpon's forme change doesn't override permanent abilities
    // 			if (source || !this.getAbility().flags['cantsuppress']) this.setAbility(ability, null, null, true);
    // 			// However, its ability does reset upon switching out
    // 			this.baseAbility = toID(ability);
    // 		}
    // 		if (this.terastallized) {
    // 			this.knownType = true;
    // 			this.apparentType = this.terastallized;
    // 		}
    // 		return true;
    // 	}
    //
    pub fn forme_change(&mut self, new_species_id: ID, new_types: Vec<String>, new_ability: Option<ID>) {
        self.species_id = new_species_id;
        self.types = new_types;
        if let Some(ability) = new_ability {
            self.ability = ability.clone();
            self.ability_state = EffectState::new(ability);
        }
    }

    /// Get base species name for this Pokemon
    /// For formes like "Pikachu-Alola", returns "Pikachu"
    /// For base species like "Pikachu", returns "Pikachu"
    /// Equivalent to pokemon.baseSpecies.name in TypeScript
    pub fn get_base_species_name(&self, dex: &crate::dex::Dex) -> Option<String> {
        let species = dex.get_species(self.species_id.as_str())?;
        Some(species.base_species.clone().unwrap_or_else(|| species.name.clone()))
    }

    /// Get base species of base species for this Pokemon
    /// For complex formes like "Gengar-Mega", returns "Gengar"
    /// Equivalent to pokemon.baseSpecies.baseSpecies in TypeScript
    pub fn get_base_species_base_species(&self, dex: &crate::dex::Dex) -> Option<String> {
        let species = dex.get_species(self.species_id.as_str())?;
        let base_species_name = species.base_species.as_ref().unwrap_or(&species.name);
        let base_species = dex.get_species(base_species_name)?;
        Some(base_species.base_species.clone().unwrap_or_else(|| base_species.name.clone()))
    }

    /// Get forme name for this Pokemon
    /// For "Pikachu-Alola", returns Some("Alola")
    /// For "Pikachu", returns None
    /// Equivalent to pokemon.baseSpecies.forme or pokemon.species.forme in TypeScript
    pub fn get_forme(&self, dex: &crate::dex::Dex) -> Option<String> {
        let species = dex.get_species(self.species_id.as_str())?;
        species.forme.clone()
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
    // 
    // 	getMoves(lockedMove?: ID | null, restrictData?: boolean): {
    // 		move: string, id: ID, disabled?: string | boolean, disabledSource?: string,
    // 		target?: string, pp?: number, maxpp?: number,
    // 	}[] {
    // 		if (lockedMove) {
    // 			lockedMove = toID(lockedMove);
    // 			this.trapped = true;
    // 			if (lockedMove === 'recharge') {
    // 				return [{
    // 					move: 'Recharge',
    // 					id: 'recharge' as ID,
    // 				}];
    // 			}
    // 			for (const moveSlot of this.moveSlots) {
    // 				if (moveSlot.id !== lockedMove) continue;
    // 				return [{
    // 					move: moveSlot.move,
    // 					id: moveSlot.id,
    // 				}];
    // 			}
    // 			// does this happen?
    // 			return [{
    // 				move: this.battle.dex.moves.get(lockedMove).name,
    // 				id: lockedMove,
    // 			}];
    // 		}
    // 		const moves = [];
    // 		let hasValidMove = false;
    // 		for (const moveSlot of this.moveSlots) {
    // 			let moveName = moveSlot.move;
    // 			if (moveSlot.id === 'hiddenpower') {
    // 				moveName = `Hidden Power ${this.hpType}`;
    // 				if (this.battle.gen < 6) moveName += ` ${this.hpPower}`;
    // 			} else if (moveSlot.id === 'return' || moveSlot.id === 'frustration') {
    // 				const basePowerCallback = this.battle.dex.moves.get(moveSlot.id).basePowerCallback as (pokemon: Pokemon) => number;
    // 				moveName += ` ${basePowerCallback(this)}`;
    // 			}
    // 			let target = moveSlot.target;
    // 			switch (moveSlot.id) {
    // 			case 'curse':
    // 				if (!this.hasType('Ghost')) {
    // 					target = this.battle.dex.moves.get('curse').nonGhostTarget;
    // 				}
    // 				break;
    // 			case 'pollenpuff':
    // 				// Heal Block only prevents Pollen Puff from targeting an ally when the user has Heal Block
    // 				if (this.volatiles['healblock']) {
    // 					target = 'adjacentFoe';
    // 				}
    // 				break;
    // 			case 'terastarstorm':
    // 				if (this.species.name === 'Terapagos-Stellar') {
    // 					target = 'allAdjacentFoes';
    // 				}
    // 				break;
    // 			}
    // 			let disabled = moveSlot.disabled;
    // 			if (this.volatiles['dynamax']) {
    // 				// if each of a Pokemon's base moves are disabled by one of these effects, it will Struggle
    // 				const canCauseStruggle = ['Encore', 'Disable', 'Taunt', 'Assault Vest', 'Belch', 'Stuff Cheeks'];
    // 				disabled = this.maxMoveDisabled(moveSlot.id) || disabled && canCauseStruggle.includes(moveSlot.disabledSource!);
    // 			} else if (moveSlot.pp <= 0 && !this.volatiles['partialtrappinglock']) {
    // 				disabled = true;
    // 			}
    // 
    // 			if (disabled === 'hidden') {
    // 				disabled = !restrictData;
    // 			}
    // 			if (!disabled) {
    // 				hasValidMove = true;
    // 			}
    // 
    // 			moves.push({
    // 				move: moveName,
    // 				id: moveSlot.id,
    // 				pp: moveSlot.pp,
    // 				maxpp: moveSlot.maxpp,
    // 				target,
    // 				disabled,
    // 			});
    // 		}
    // 		return hasValidMove ? moves : [];
    // 	}
    //
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
    // 
    // 	toJSON(): AnyObject {
    // 		return State.serializePokemon(this);
    // 	}
    //
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
    pub fn get_combat_power(&self) -> i32 {
        // Simplified formula based on stats
        let atk = self.stored_stats.atk;
        let def = self.stored_stats.def;
        let sta = self.stored_stats.hp.max(10); // Use HP as stamina proxy

        (((atk as f64) * (def as f64).powf(0.5) * (sta as f64).powf(0.5)) / 10.0) as i32
    }

    /// Get location of a target Pokemon relative to this one
    /// Equivalent to getLocOf in pokemon.ts
    // TypeScript source:
    // /**
    // 	 * Returns a relative location: 1-3, positive for foe, and negative for ally.
    // 	 * Use `getAtLoc` to reverse.
    // 	 */
    // 	getLocOf(target: Pokemon) {
    // 		const positionOffset = Math.floor(target.side.n / 2) * target.side.active.length;
    // 		const position = target.position + positionOffset + 1;
    // 		const sameHalf = (this.side.n % 2) === (target.side.n % 2);
    // 		return sameHalf ? -position : position;
    // 	}
    //
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
    // 
    // 	getAtLoc(targetLoc: number) {
    // 		let side = this.battle.sides[targetLoc < 0 ? this.side.n % 2 : (this.side.n + 1) % 2];
    // 		targetLoc = Math.abs(targetLoc);
    // 		if (targetLoc > side.active.length) {
    // 			targetLoc -= side.active.length;
    // 			side = this.battle.sides[side.n + 2];
    // 		}
    // 		return side.active[targetLoc - 1];
    // 	}
    //
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
    // TypeScript source:
    // /** Get targets for Dragon Darts */
    // 	getSmartTargets(target: Pokemon, move: ActiveMove) {
    // 		const target2 = target.adjacentAllies()[0];
    // 		if (!target2 || target2 === this || !target2.hp) {
    // 			move.smartTarget = false;
    // 			return [target];
    // 		}
    // 		if (!target.hp) {
    // 			move.smartTarget = false;
    // 			return [target2];
    // 		}
    // 		return [target, target2];
    // 	}
    //
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
    // 
    // 	getLastAttackedBy() {
    // 		if (this.attackedBy.length === 0) return undefined;
    // 		return this.attackedBy[this.attackedBy.length - 1];
    // 	}
    //
    pub fn get_last_attacked_by(&self) -> Option<(ID, i32)> {
        // Would need attacked_by tracking for full implementation
        None
    }

    /// Get last damager info
    /// Equivalent to getLastDamagedBy in pokemon.ts
    //
    // 	getLastDamagedBy(filterOutSameSide: boolean) {
    // 		const damagedBy: Attacker[] = this.attackedBy.filter(attacker => (
    // 			typeof attacker.damageValue === 'number' &&
    // 			(filterOutSameSide === undefined || !this.isAlly(attacker.source))
    // 		));
    // 		if (damagedBy.length === 0) return undefined;
    // 		return damagedBy[damagedBy.length - 1];
    // 	}
    //
    pub fn get_last_damaged_by(&self, _filter_out_same_side: bool) -> Option<Attacker> {
        // TODO: Implement proper attacked_by tracking
        // For now, return None to allow compilation
        // Full implementation needs:
        // - attackedBy: Vec<Attacker> field on Pokemon
        // - Tracking of all attacks in battle
        // - Filtering by filterOutSameSide
        None
    }

    /// Get Dynamax request data
    /// Equivalent to getDynamaxRequest in pokemon.ts
    // 
    // 	getDynamaxRequest(skipChecks?: boolean) {
    // 		// {gigantamax?: string, maxMoves: {[k: string]: string} | null}[]
    // 		if (!skipChecks) {
    // 			if (!this.side.canDynamaxNow()) return;
    // 			if (
    // 				this.species.isMega || this.species.isPrimal || this.species.forme === "Ultra" ||
    // 				this.getItem().zMove || this.canMegaEvo
    // 			) {
    // 				return;
    // 			}
    // 			// Some pokemon species are unable to dynamax
    // 			if (this.species.cannotDynamax || this.illusion?.species.cannotDynamax) return;
    // 		}
    // 		const result: DynamaxOptions = { maxMoves: [] };
    // 		let atLeastOne = false;
    // 		for (const moveSlot of this.moveSlots) {
    // 			const move = this.battle.dex.moves.get(moveSlot.id);
    // 			const maxMove = this.battle.actions.getMaxMove(move, this);
    // 			if (maxMove) {
    // 				if (this.maxMoveDisabled(move)) {
    // 					result.maxMoves.push({ move: maxMove.id, target: maxMove.target, disabled: true });
    // 				} else {
    // 					result.maxMoves.push({ move: maxMove.id, target: maxMove.target });
    // 					atLeastOne = true;
    // 				}
    // 			}
    // 		}
    // 		if (!atLeastOne) return;
    // 		if (this.canGigantamax && this.gigantamax) result.gigantamax = this.canGigantamax;
    // 		return result;
    // 	}
    //
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
    // 
    // 	getMoveRequestData() {
    // 		let lockedMove = this.maybeLocked ? null : this.getLockedMove();
    // 
    // 		// Information should be restricted for the last active Pokémon
    // 		const isLastActive = this.isLastActive();
    // 		const canSwitchIn = this.battle.canSwitch(this.side) > 0;
    // 		let moves = this.getMoves(lockedMove, isLastActive);
    // 
    // 		if (!moves.length) {
    // 			moves = [{ move: 'Struggle', id: 'struggle' as ID, target: 'randomNormal', disabled: false }];
    // 			lockedMove = 'struggle' as ID;
    // 		}
    // 
    // 		const data: PokemonMoveRequestData = {
    // 			moves,
    // 		};
    // 
    // 		if (isLastActive) {
    // 			this.maybeDisabled = this.maybeDisabled && !lockedMove;
    // 			this.maybeLocked = this.maybeLocked || this.maybeDisabled;
    // 			if (this.maybeDisabled) {
    // 				data.maybeDisabled = this.maybeDisabled;
    // 			}
    // 			if (this.maybeLocked) {
    // 				data.maybeLocked = this.maybeLocked;
    // 			}
    // 			if (canSwitchIn) {
    // 				if (this.trapped === true) {
    // 					data.trapped = true;
    // 				} else if (this.maybeTrapped) {
    // 					data.maybeTrapped = true;
    // 				}
    // 			}
    // 		} else {
    // 			this.maybeDisabled = false;
    // 			this.maybeLocked = false;
    // 			if (canSwitchIn) {
    // 				// Discovered by selecting a valid Pokémon as a switch target and cancelling.
    // 				if (this.trapped) data.trapped = true;
    // 			}
    // 			this.maybeTrapped = false;
    // 		}
    // 
    // 		if (!lockedMove) {
    // 			if (this.canMegaEvo) data.canMegaEvo = true;
    // 			if (this.canMegaEvoX) data.canMegaEvoX = true;
    // 			if (this.canMegaEvoY) data.canMegaEvoY = true;
    // 			if (this.canUltraBurst) data.canUltraBurst = true;
    // 			const canZMove = this.battle.actions.canZMove(this);
    // 			if (canZMove) data.canZMove = canZMove;
    // 
    // 			if (this.getDynamaxRequest()) data.canDynamax = true;
    // 			if (data.canDynamax || this.volatiles['dynamax']) data.maxMoves = this.getDynamaxRequest(true);
    // 			if (this.canTerastallize) data.canTerastallize = this.canTerastallize;
    // 		}
    // 
    // 		return data;
    // 	}
    //
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
    // 
    // 	getSwitchRequestData(forAlly?: boolean): PokemonSwitchRequestData {
    // 		const entry: PokemonSwitchRequestData = {
    // 			ident: this.fullname,
    // 			details: this.details,
    // 			condition: this.getHealth().secret,
    // 			active: (this.position < this.side.active.length),
    // 			stats: {
    // 				atk: this.baseStoredStats['atk'],
    // 				def: this.baseStoredStats['def'],
    // 				spa: this.baseStoredStats['spa'],
    // 				spd: this.baseStoredStats['spd'],
    // 				spe: this.baseStoredStats['spe'],
    // 			},
    // 			moves: this[forAlly ? 'baseMoves' : 'moves'].map(move => {
    // 				if (move === 'hiddenpower') {
    // 					return `${move}${toID(this.hpType)}${this.battle.gen < 6 ? '' : this.hpPower}` as ID;
    // 				}
    // 				if (move === 'frustration' || move === 'return') {
    // 					const basePowerCallback = this.battle.dex.moves.get(move).basePowerCallback as (pokemon: Pokemon) => number;
    // 					return `${move}${basePowerCallback(this)}` as ID;
    // 				}
    // 				return move as ID;
    // 			}),
    // 			baseAbility: this.baseAbility,
    // 			item: this.item,
    // 			pokeball: this.pokeball,
    // 		};
    // 		if (this.battle.gen > 6) entry.ability = this.ability;
    // 		if (this.battle.gen >= 9) {
    // 			entry.commanding = !!this.volatiles['commanding'] && !this.fainted;
    // 			entry.reviving = this.isActive && !!this.side.slotConditions[this.position]['revivalblessing'];
    // 		}
    // 		if (this.battle.gen === 9) {
    // 			entry.teraType = this.teraType;
    // 			entry.terastallized = this.terastallized || '';
    // 		}
    // 		return entry;
    // 	}
    //
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
    // 
    // 	trySetStatus(status: string | Condition, source: Pokemon | null = null, sourceEffect: Effect | null = null) {
    // 		return this.setStatus(this.status || status, source, sourceEffect);
    // 	}
    //
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
    // 
    // 	useItem(source?: Pokemon, sourceEffect?: Effect) {
    // 		if ((!this.hp && !this.getItem().isGem) || !this.isActive) return false;
    // 		if (!this.item) return false;
    // 
    // 		if (!sourceEffect && this.battle.effect) sourceEffect = this.battle.effect;
    // 		if (!source && this.battle.event?.target) source = this.battle.event.target;
    // 		const item = this.getItem();
    // 		if (sourceEffect?.effectType === 'Item' && this.item !== sourceEffect.id && source === this) {
    // 			// if an item is telling us to eat it but we aren't holding it, we probably shouldn't eat what we are holding
    // 			return false;
    // 		}
    // 		if (this.battle.runEvent('UseItem', this, null, null, item)) {
    // 			switch (item.id) {
    // 			case 'redcard':
    // 				this.battle.add('-enditem', this, item, `[of] ${source}`);
    // 				break;
    // 			default:
    // 				if (item.isGem) {
    // 					this.battle.add('-enditem', this, item, '[from] gem');
    // 				} else {
    // 					this.battle.add('-enditem', this, item);
    // 				}
    // 				break;
    // 			}
    // 			if (item.boosts) {
    // 				this.battle.boost(item.boosts, this, source, item);
    // 			}
    // 
    // 			this.battle.singleEvent('Use', item, this.itemState, this, source, sourceEffect);
    // 
    // 			this.lastItem = this.item;
    // 			this.item = '';
    // 			this.battle.clearEffectState(this.itemState);
    // 			this.usedItemThisTurn = true;
    // 			this.battle.runEvent('AfterUseItem', this, null, null, item);
    // 			return true;
    // 		}
    // 		return false;
    // 	}
    //
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
    // 
    // 	eatItem(force?: boolean, source?: Pokemon, sourceEffect?: Effect) {
    // 		if (!this.item) return false;
    // 		if ((!this.hp && this.item !== 'jabocaberry' && this.item !== 'rowapberry') || !this.isActive) return false;
    // 
    // 		if (!sourceEffect && this.battle.effect) sourceEffect = this.battle.effect;
    // 		if (!source && this.battle.event?.target) source = this.battle.event.target;
    // 		const item = this.getItem();
    // 		if (sourceEffect?.effectType === 'Item' && this.item !== sourceEffect.id && source === this) {
    // 			// if an item is telling us to eat it but we aren't holding it, we probably shouldn't eat what we are holding
    // 			return false;
    // 		}
    // 		if (
    // 			this.battle.runEvent('UseItem', this, null, null, item) &&
    // 			(force || this.battle.runEvent('TryEatItem', this, null, null, item))
    // 		) {
    // 			this.battle.add('-enditem', this, item, '[eat]');
    // 
    // 			this.battle.singleEvent('Eat', item, this.itemState, this, source, sourceEffect);
    // 			this.battle.runEvent('EatItem', this, source, sourceEffect, item);
    // 
    // 			if (RESTORATIVE_BERRIES.has(item.id)) {
    // 				switch (this.pendingStaleness) {
    // 				case 'internal':
    // 					if (this.staleness !== 'external') this.staleness = 'internal';
    // 					break;
    // 				case 'external':
    // 					this.staleness = 'external';
    // 					break;
    // 				}
    // 				this.pendingStaleness = undefined;
    // 			}
    // 
    // 			this.lastItem = this.item;
    // 			this.item = '';
    // 			this.battle.clearEffectState(this.itemState);
    // 			this.usedItemThisTurn = true;
    // 			this.ateBerry = true;
    // 			this.battle.runEvent('AfterUseItem', this, null, null, item);
    // 			return true;
    // 		}
    // 		return false;
    // 	}
    //
    pub fn eat_item(&mut self, _is_forced: bool) -> Option<ID> {
        if self.item.is_empty() {
            return None;
        }

        // Would check if item is edible (berry)
        // For now, same as use_item
        self.use_item()
    }

    /// Run type effectiveness check
    /// Equivalent to runEffectiveness in pokemon.ts
    // 
    // 	runEffectiveness(move: ActiveMove) {
    // 		let totalTypeMod = 0;
    // 		if (this.terastallized && move.type === 'Stellar') {
    // 			totalTypeMod = 1;
    // 		} else {
    // 			for (const type of this.getTypes()) {
    // 				let typeMod = this.battle.dex.getEffectiveness(move, type);
    // 				typeMod = this.battle.singleEvent('Effectiveness', move, null, this, type, move, typeMod);
    // 				totalTypeMod += this.battle.runEvent('Effectiveness', this, type, move, typeMod);
    // 			}
    // 		}
    // 		if (this.species.name === 'Terapagos-Terastal' && this.hasAbility('Tera Shell') &&
    // 			!this.battle.suppressingAbility(this)) {
    // 			if (this.abilityState.resisted) return -1; // all hits of multi-hit move should be not very effective
    // 			if (move.category === 'Status' || move.id === 'struggle' || !this.runImmunity(move) ||
    // 				totalTypeMod < 0 || this.hp < this.maxhp) {
    // 				return totalTypeMod;
    // 			}
    // 
    // 			this.battle.add('-activate', this, 'ability: Tera Shell');
    // 			this.abilityState.resisted = true;
    // 			return -1;
    // 		}
    // 		return totalTypeMod;
    // 	}
    //
    pub fn run_effectiveness(&self, move_type: &str) -> f64 {
        crate::data::typechart::get_effectiveness_multi(move_type, &self.types)
    }

    /// Run immunity check
    /// Equivalent to runImmunity in pokemon.ts
    // TypeScript source:
    // /** false = immune, true = not immune */
    // 	runImmunity(source: ActiveMove | string, message?: string | boolean) {
    // 		if (!source) return true;
    // 		const type: string = typeof source !== 'string' ? source.type : source;
    // 		if (typeof source !== 'string') {
    // 			if (source.ignoreImmunity && (source.ignoreImmunity === true || source.ignoreImmunity[type])) {
    // 				return true;
    // 			}
    // 		}
    // 		if (!type || type === '???') return true;
    // 		if (!this.battle.dex.types.isName(type)) {
    // 			throw new Error("Use runStatusImmunity for " + type);
    // 		}
    // 
    // 		const negateImmunity = !this.battle.runEvent('NegateImmunity', this, type);
    // 		const notImmune = type === 'Ground' ?
    // 			this.isGrounded(negateImmunity) :
    // 			negateImmunity || this.battle.dex.getImmunity(type, this);
    // 		if (notImmune) return true;
    // 		if (!message) return false;
    // 		if (notImmune === null) {
    // 			this.battle.add('-immune', this, '[from] ability: Levitate');
    // 		} else {
    // 			this.battle.add('-immune', this);
    // 		}
    // 		return false;
    // 	}
    //
    pub fn run_immunity(&self, move_type: &str) -> bool {
        let effectiveness = self.run_effectiveness(move_type);
        effectiveness > 0.0
    }

    /// Run status immunity check
    /// Equivalent to runStatusImmunity in pokemon.ts
    // 
    // 	runStatusImmunity(type: string, message?: string) {
    // 		if (this.fainted) return false;
    // 		if (!type) return true;
    // 
    // 		if (!this.battle.dex.getImmunity(type, this)) {
    // 			this.battle.debug('natural status immunity');
    // 			if (message) {
    // 				this.battle.add('-immune', this);
    // 			}
    // 			return false;
    // 		}
    // 		const immunity = this.battle.runEvent('Immunity', this, null, null, type);
    // 		if (!immunity) {
    // 			this.battle.debug('artificial status immunity');
    // 			if (message && immunity !== null) {
    // 				this.battle.add('-immune', this);
    // 			}
    // 			return false;
    // 		}
    // 		return true;
    // 	}
    //
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
    // 
    // 	removeLinkedVolatiles(linkedStatus: string | Effect, linkedPokemon: Pokemon[]) {
    // 		linkedStatus = linkedStatus.toString();
    // 		for (const linkedPoke of linkedPokemon) {
    // 			const volatileData = linkedPoke.volatiles[linkedStatus];
    // 			if (!volatileData) continue;
    // 			volatileData.linkedPokemon.splice(volatileData.linkedPokemon.indexOf(this), 1);
    // 			if (volatileData.linkedPokemon.length === 0) {
    // 				linkedPoke.removeVolatile(linkedStatus);
    // 			}
    // 		}
    // 	}
    //
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
    // 
    // 	getNature() {
    // 		return this.battle.dex.natures.get(this.set.nature);
    // 	}
    //
    pub fn get_nature(&self) -> &str {
        // In battle, the nature is already applied to stored_stats
        // The actual nature value would need to be stored if needed
        "Hardy" // Default neutral nature
    }

    /// Get status object
    /// Equivalent to getStatus in pokemon.ts
    // 
    // 	getStatus() {
    // 		return this.battle.dex.conditions.getByID(this.status);
    // 	}
    //
    pub fn get_status(&self) -> Option<&ID> {
        if self.status.is_empty() {
            None
        } else {
            Some(&self.status)
        }
    }

    /// Destroy/cleanup Pokemon
    /// Equivalent to destroy in pokemon.ts
    // 
    // 	destroy() {
    // 		// deallocate ourself
    // 		// get rid of some possibly-circular references
    // 		(this as any).battle = null!;
    // 		(this as any).side = null!;
    // 	}
    //
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
    // TypeScript source:
    // /**
    // 	 * Unlike cureStatus, does not give cure message
    // 	 */
    // 	clearStatus() {
    // 		if (!this.hp || !this.status) return false;
    // 		if (this.status === 'slp' && this.removeVolatile('nightmare')) {
    // 			this.battle.add('-end', this, 'Nightmare', '[silent]');
    // 		}
    // 		this.setStatus('');
    // 		return true;
    // 	}
    //
    pub fn clear_status(&mut self) -> bool {
        if !self.status.is_empty() {
            self.status = ID::empty();
            self.status_state = crate::event_system::EffectState::new(ID::empty());
            true
        } else {
            false
        }
    }

    /// Get move hit data for tracking hit results
    /// Equivalent to pokemon.ts getMoveHitData()
    ///
    /// Returns a new MoveHitData struct for this target
    // 
    // 	getMoveHitData(move: ActiveMove) {
    // 		if (!move.moveHitData) move.moveHitData = {};
    // 		const slot = this.getSlot();
    // 		return move.moveHitData[slot] || (move.moveHitData[slot] = {
    // 			crit: false,
    // 			typeMod: 0,
    // 			zBrokeProtect: false,
    // 		});
    // 	}
    //
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
    pub damage: i32,
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
