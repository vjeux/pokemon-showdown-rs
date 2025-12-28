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
#[derive(Default)]
pub enum RequestState {
    #[default]
    None,
    TeamPreview,
    Move,
    Switch,
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
    /// Active request sent to player
    #[serde(skip)]
    pub active_request: Option<crate::choice::BattleRequest>,
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
            active_request: None,
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
    // TypeScript source:
    // 
    // 
    // 	addSideCondition(
    // 		status: string | Condition, source: Pokemon | 'debug' | null = null, sourceEffect: Effect | null = null
    // 	): boolean {
    // 		if (!source && this.battle.event?.target) source = this.battle.event.target;
    // 		if (source === 'debug') source = this.active[0];
    // 		if (!source) throw new Error(`setting sidecond without a source`);
    // 		if (!source.getSlot) source = (source as any as Side).active[0];
    // 
    // 		status = this.battle.dex.conditions.get(status);
    // 		if (this.sideConditions[status.id]) {
    // 			if (!(status as any).onSideRestart) return false;
    // 			return this.battle.singleEvent('SideRestart', status, this.sideConditions[status.id], this, source, sourceEffect);
    // 		}
    // 		this.sideConditions[status.id] = this.battle.initEffectState({
    // 			id: status.id,
    // 			target: this,
    // 			source,
    // 			sourceSlot: source.getSlot(),
    // 			duration: status.duration,
    // 		});
    // 		if (status.durationCallback) {
    // 			this.sideConditions[status.id].duration =
    // 				status.durationCallback.call(this.battle, this.active[0], source, sourceEffect);
    // 		}
    // 		if (!this.battle.singleEvent('SideStart', status, this.sideConditions[status.id], this, source, sourceEffect)) {
    // 			delete this.sideConditions[status.id];
    // 			return false;
    // 		}
    // 		this.battle.runEvent('SideConditionStart', this, source, status);
    // 		return true;
    // 	}
    //
    pub fn add_side_condition(&mut self, id: ID, duration: Option<i32>) -> bool {
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
    // 
    // 	getSideCondition(status: string | Effect): Effect | null {
    // 		status = this.battle.dex.conditions.get(status) as Effect;
    // 		if (!this.sideConditions[status.id]) return null;
    // 		return status;
    // 	}
    //
    pub fn get_side_condition(&self, id: &ID) -> Option<&EffectState> {
        self.side_conditions.get(id)
    }

    /// Get mutable side condition state
    pub fn get_side_condition_mut(&mut self, id: &ID) -> Option<&mut EffectState> {
        self.side_conditions.get_mut(id)
    }

    /// Remove a side condition
    // 
    // 	removeSideCondition(status: string | Effect): boolean {
    // 		status = this.battle.dex.conditions.get(status) as Effect;
    // 		if (!this.sideConditions[status.id]) return false;
    // 		this.battle.singleEvent('SideEnd', status, this.sideConditions[status.id], this);
    // 		delete this.sideConditions[status.id];
    // 		return true;
    // 	}
    //
    pub fn remove_side_condition(&mut self, id: &ID) -> bool {
        self.side_conditions.remove(id).is_some()
    }

    /// Add a slot condition
    // 
    // 	addSlotCondition(
    // 		target: Pokemon | number, status: string | Condition, source: Pokemon | 'debug' | null = null,
    // 		sourceEffect: Effect | null = null
    // 	) {
    // 		source ??= this.battle.event?.target || null;
    // 		if (source === 'debug') source = this.active[0];
    // 		if (target instanceof Pokemon) target = target.position;
    // 		if (!source) throw new Error(`setting sidecond without a source`);
    // 
    // 		status = this.battle.dex.conditions.get(status);
    // 		if (this.slotConditions[target][status.id]) {
    // 			if (!status.onRestart) return false;
    // 			return this.battle.singleEvent('Restart', status, this.slotConditions[target][status.id], this, source, sourceEffect);
    // 		}
    // 		const conditionState = this.slotConditions[target][status.id] = this.battle.initEffectState({
    // 			id: status.id,
    // 			target: this,
    // 			source,
    // 			sourceSlot: source.getSlot(),
    // 			isSlotCondition: true,
    // 			duration: status.duration,
    // 		});
    // 		if (status.durationCallback) {
    // 			conditionState.duration =
    // 				status.durationCallback.call(this.battle, this.active[0], source, sourceEffect);
    // 		}
    // 		if (!this.battle.singleEvent('Start', status, conditionState, this.active[target], source, sourceEffect)) {
    // 			delete this.slotConditions[target][status.id];
    // 			return false;
    // 		}
    // 		return true;
    // 	}
    //
    pub fn add_slot_condition(&mut self, slot: usize, id: ID, duration: Option<i32>) -> bool {
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
    // 
    // 	removeSlotCondition(target: Pokemon | number, status: string | Effect) {
    // 		if (target instanceof Pokemon) target = target.position;
    // 		status = this.battle.dex.conditions.get(status) as Effect;
    // 		if (!this.slotConditions[target][status.id]) return false;
    // 		this.battle.singleEvent('End', status, this.slotConditions[target][status.id], this.active[target]);
    // 		delete this.slotConditions[target][status.id];
    // 		return true;
    // 	}
    //
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
    // TypeScript source:
    // /** convert a Choice into a choice string */
    // 	getChoice() {
    // 		if (this.choice.actions.length > 1 && this.choice.actions.every(action => action.choice === 'team')) {
    // 			return `team ` + this.choice.actions.map(action => action.pokemon!.position + 1).join(', ');
    // 		}
    // 		return this.choice.actions.map(action => {
    // 			switch (action.choice) {
    // 			case 'move':
    // 				let details = ``;
    // 				if (action.targetLoc && this.active.length > 1) details += ` ${action.targetLoc > 0 ? '+' : ''}${action.targetLoc}`;
    // 				if (action.mega) details += (action.pokemon!.item === 'ultranecroziumz' ? ` ultra` : ` mega`);
    // 				if (action.megax) details += ` megax`;
    // 				if (action.megay) details += ` megay`;
    // 				if (action.zmove) details += ` zmove`;
    // 				if (action.maxMove) details += ` dynamax`;
    // 				if (action.terastallize) details += ` terastallize`;
    // 				return `move ${action.moveid}${details}`;
    // 			case 'switch':
    // 			case 'instaswitch':
    // 			case 'revivalblessing':
    // 				return `switch ${action.target!.position + 1}`;
    // 			case 'team':
    // 				return `team ${action.pokemon!.position + 1}`;
    // 			default:
    // 				return action.choice;
    // 			}
    // 		}).join(', ');
    // 	}
    //
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
    pub fn get_hazard_layers(&self, hazard_id: &ID) -> i32 {
        self.side_conditions.get(hazard_id)
            .and_then(|state| state.layers)
            .unwrap_or(0)
    }

    /// Check if choice for this turn is complete
    /// Equivalent to side.ts isChoiceDone()
    // 
    // 	isChoiceDone() {
    // 		if (!this.requestState) return true;
    // 		if (this.choice.forcedSwitchesLeft) return false;
    // 
    // 		if (this.requestState === 'teampreview') {
    // 			return this.choice.actions.length >= this.pickedTeamSize();
    // 		}
    // 
    // 		// current request is move/switch
    // 		this.getChoiceIndex(); // auto-pass
    // 		return this.choice.actions.length >= this.active.length;
    // 	}
    //
    pub fn is_choice_done(&self, picked_team_size: Option<usize>) -> bool {
        // if (!this.requestState) return true;
        if matches!(self.request_state, RequestState::None) {
            return true;
        }

        // if (this.choice.forcedSwitchesLeft) return false;
        if self.choice.forced_switches_left > 0 {
            return false;
        }

        match self.request_state {
            RequestState::TeamPreview => {
                // return this.choice.actions.length >= this.pickedTeamSize();
                let team_size = self.picked_team_size(picked_team_size);
                self.choice.actions.len() >= team_size
            }
            RequestState::Move | RequestState::Switch => {
                // this.getChoiceIndex(); // auto-pass
                // TODO: Call getChoiceIndex() for auto-pass
                // This would auto-add pass actions for fainted Pokemon
                // For now, just check actions length

                // return this.choice.actions.length >= this.active.length;
                self.choice.actions.len() >= self.active.len()
            }
            RequestState::None => true,
        }
    }

    /// Clear choice for the turn
    /// Equivalent to side.ts clearChoice()
    //
    // 	clearChoice() {
    // 		let forcedSwitches = 0;
    // 		let forcedPasses = 0;
    // 		if (this.battle.requestState === 'switch') {
    // 			const canSwitchOut = this.active.filter(pokemon => pokemon?.switchFlag).length;
    // 			const canSwitchIn = this.pokemon.slice(this.active.length).filter(pokemon => pokemon && !pokemon.fainted).length;
    // 			forcedSwitches = Math.min(canSwitchOut, canSwitchIn);
    // 			forcedPasses = canSwitchOut - forcedSwitches;
    // 		}
    // 		this.choice = {
    // 			cantUndo: false,
    // 			error: ``,
    // 			actions: [],
    // 			forcedSwitchesLeft: forcedSwitches,
    // 			forcedPassesLeft: forcedPasses,
    // 			switchIns: new Set(),
    // 			zMove: false,
    // 			mega: false,
    // 			ultra: false,
    // 			dynamax: false,
    // 			terastallize: false,
    // 		};
    // 	}
    //
    pub fn clear_choice(&mut self, battle_request_state: crate::battle::BattleRequestState) {
        let mut forced_switches = 0;
        let mut forced_passes = 0;

        if matches!(battle_request_state, crate::battle::BattleRequestState::Switch) {
            // Count active Pokemon with switchFlag set
            let can_switch_out = self.active.iter()
                .filter_map(|&opt_idx| opt_idx.and_then(|idx| self.pokemon.get(idx)))
                .filter(|p| p.switch_flag)
                .count();

            // Count benched Pokemon that aren't fainted
            let can_switch_in = self.pokemon.iter()
                .skip(self.active.len())
                .filter(|p| !p.is_fainted())
                .count();

            forced_switches = can_switch_out.min(can_switch_in);
            forced_passes = can_switch_out - forced_switches;
        }

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
    }

    /// Calculate Stealth Rock damage based on type effectiveness
    pub fn calc_stealth_rock_damage(defender_types: &[String], max_hp: i32) -> i32 {
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
        ((max_hp as f64 * damage_frac) as i32).max(1)
    }

    /// Calculate Spikes damage based on number of layers
    pub fn calc_spikes_damage(layers: i32, max_hp: i32) -> i32 {
        // Spikes: 1 layer = 1/8, 2 layers = 1/6, 3 layers = 1/4
        let damage_frac = match layers {
            1 => 1.0 / 8.0,
            2 => 1.0 / 6.0,
            _ => 1.0 / 4.0,
        };
        ((max_hp as f64 * damage_frac) as i32).max(1)
    }

    // ==========================================
    // Methods ported from side.ts
    // ==========================================

    /// String representation of Side
    //
    // 	toString() {
    // 		return `${this.id}: ${this.name}`;
    // 	}
    //
    /// Check if this side can dynamax now (Gen 8)
    // 
    // 	canDynamaxNow(): boolean {
    // 		if (this.battle.gen !== 8) return false;
    // 		// In multi battles, players on a team are alternatingly given the option to dynamax each turn
    // 		// On turn 1, the players on their team's respective left have the first chance (p1 and p2)
    // 		if (this.battle.gameType === 'multi' && this.battle.turn % 2 !== [1, 1, 0, 0][this.n]) return false;
    // 		// if (this.battle.gameType === 'multitriples' && this.battle.turn % 3 !== [1, 1, 2, 2, 0, 0][this.side.n]) {
    // 		//		return false;
    // 		// }
    // 		return !this.dynamaxUsed;
    // 	}
    //
    pub fn can_dynamax_now(&self, gen: u8, game_type: &str, turn: i32) -> bool {
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
    // 	allies(all?: boolean) {
    // 		// called during the first switch-in, so `active` can still contain nulls at this point
    // 		let allies = this.activeTeam().filter(ally => ally);
    // 		if (!all) allies = allies.filter(ally => !!ally.hp);
    // 
    // 		return allies;
    // 	}
    //
    pub fn allies(&self, include_fainted: bool) -> Vec<usize> {
        let mut allies = Vec::new();
        for idx in self.active.iter().flatten() {
            if let Some(pokemon) = self.pokemon.get(*idx) {
                if include_fainted || !pokemon.is_fainted() {
                    allies.push(*idx);
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
    // 	hasAlly(pokemon: Pokemon) {
    // 		return pokemon.side === this || pokemon.side === this.allySide;
    // 	}
    //
    pub fn has_ally(&self, pokemon_side_index: usize) -> bool {
        pokemon_side_index == self.n || self.ally_index == Some(pokemon_side_index)
    }

    /// Add a Pokemon to the team
    // 
    // 	addPokemon(set: PokemonSet) {
    // 		if (this.pokemon.length >= 24) return null;
    // 		const newPokemon = new Pokemon(set, this);
    // 		newPokemon.position = this.pokemon.length;
    // 		this.pokemon.push(newPokemon);
    // 		this.pokemonLeft++;
    // 		return newPokemon;
    // 	}
    //
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
    // 
    // 	randomFoe() {
    // 		const actives = this.foes();
    // 		if (!actives.length) return null;
    // 		return this.battle.sample(actives);
    // 	}
    //
    pub fn random_foe(&self) -> Option<usize> {
        // This is a stub - real implementation needs battle context
        None
    }

    /// Get total Pokemon left on foe side
    // 	foePokemonLeft() {
    // 		if (this.battle.gameType === 'freeforall') {
    // 			return this.battle.sides.filter(side => side !== this).map(side => side.pokemonLeft).reduce((a, b) => a + b);
    // 		}
    // 
    // 		if (this.foe.allySide) return this.foe.pokemonLeft + this.foe.allySide.pokemonLeft;
    // 
    // 		return this.foe.pokemonLeft;
    // 	}
    //
    pub fn foe_pokemon_left(&self) -> usize {
        // This is a stub - needs battle context
        0
    }

    /// Get slot condition data
    // 
    // 	getSlotCondition(target: Pokemon | number, status: string | Effect) {
    // 		if (target instanceof Pokemon) target = target.position;
    // 		status = this.battle.dex.conditions.get(status) as Effect;
    // 		if (!this.slotConditions[target][status.id]) return null;
    // 		return status;
    // 	}
    //
    pub fn get_slot_condition(&self, slot: usize, id: &ID) -> Option<&EffectState> {
        self.slot_conditions.get(slot)?.get(id)
    }

    /// Get mutable slot condition data
    pub fn get_slot_condition_mut(&mut self, slot: usize, id: &ID) -> Option<&mut EffectState> {
        self.slot_conditions.get_mut(slot)?.get_mut(id)
    }


    /// Get the current choice action index
    // 
    // 	getChoiceIndex(isPass?: boolean) {
    // 		let index = this.choice.actions.length;
    // 
    // 		if (!isPass) {
    // 			switch (this.requestState) {
    // 			case 'move':
    // 				// auto-pass
    // 				while (
    // 					index < this.active.length &&
    // 					(this.active[index].fainted || this.active[index].volatiles['commanding'])
    // 				) {
    // 					this.choosePass();
    // 					index++;
    // 				}
    // 				break;
    // 			case 'switch':
    // 				while (index < this.active.length && !this.active[index].switchFlag) {
    // 					this.choosePass();
    // 					index++;
    // 				}
    // 				break;
    // 			}
    // 		}
    // 
    // 		return index;
    // 	}
    //
    pub fn get_choice_index(&self) -> usize {
        self.choice.actions.len()
    }

    /// Choose pass action
    // 
    // 	choosePass(): boolean | Side {
    // 		const index = this.getChoiceIndex(true);
    // 		if (index >= this.active.length) return false;
    // 		const pokemon: Pokemon = this.active[index];
    // 
    // 		switch (this.requestState) {
    // 		case 'switch':
    // 			if (pokemon.switchFlag) { // This condition will always happen if called by Battle#choose()
    // 				if (!this.choice.forcedPassesLeft) {
    // 					return this.emitChoiceError(`Can't pass: You need to switch in a Pokémon to replace ${pokemon.name}`);
    // 				}
    // 				this.choice.forcedPassesLeft--;
    // 			}
    // 			break;
    // 		case 'move':
    // 			if (!pokemon.fainted && !pokemon.volatiles['commanding']) {
    // 				return this.emitChoiceError(`Can't pass: Your ${pokemon.name} must make a move (or switch)`);
    // 			}
    // 			break;
    // 		default:
    // 			return this.emitChoiceError(`Can't pass: Not a move or switch request`);
    // 		}
    // 
    // 		this.choice.actions.push({
    // 			choice: 'pass',
    // 		} as ChosenAction);
    // 		return true;
    // 	}
    //
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
    // 
    // 	chooseSwitch(slotText?: string) {
    // 		if (this.requestState !== 'move' && this.requestState !== 'switch') {
    // 			return this.emitChoiceError(`Can't switch: You need a ${this.requestState} response`);
    // 		}
    // 		const index = this.getChoiceIndex();
    // 		if (index >= this.active.length) {
    // 			if (this.requestState === 'switch') {
    // 				return this.emitChoiceError(`Can't switch: You sent more switches than Pokémon that need to switch`);
    // 			}
    // 			return this.emitChoiceError(`Can't switch: You sent more choices than unfainted Pokémon`);
    // 		}
    // 		const pokemon = this.active[index];
    // 		let slot;
    // 		if (!slotText) {
    // 			if (this.requestState !== 'switch') {
    // 				return this.emitChoiceError(`Can't switch: You need to select a Pokémon to switch in`);
    // 			}
    // 			if (this.slotConditions[pokemon.position]['revivalblessing']) {
    // 				slot = 0;
    // 				while (!this.pokemon[slot].fainted) slot++;
    // 			} else {
    // 				if (!this.choice.forcedSwitchesLeft) return this.choosePass();
    // 				slot = this.active.length;
    // 				while (this.choice.switchIns.has(slot) || this.pokemon[slot].fainted) slot++;
    // 			}
    // 		} else {
    // 			slot = parseInt(slotText) - 1;
    // 		}
    // 		if (isNaN(slot) || slot < 0) {
    // 			// maybe it's a name/species id!
    // 			slot = -1;
    // 			for (const [i, mon] of this.pokemon.entries()) {
    // 				if (slotText!.toLowerCase() === mon.name.toLowerCase() || toID(slotText) === mon.species.id) {
    // 					slot = i;
    // 					break;
    // 				}
    // 			}
    // 			if (slot < 0) {
    // 				return this.emitChoiceError(`Can't switch: You do not have a Pokémon named "${slotText}" to switch to`);
    // 			}
    // 		}
    // 		if (slot >= this.pokemon.length) {
    // 			return this.emitChoiceError(`Can't switch: You do not have a Pokémon in slot ${slot + 1} to switch to`);
    // 		} else if (slot < this.active.length && !this.slotConditions[pokemon.position]['revivalblessing']) {
    // 			return this.emitChoiceError(`Can't switch: You can't switch to an active Pokémon`);
    // 		} else if (this.choice.switchIns.has(slot)) {
    // 			return this.emitChoiceError(`Can't switch: The Pokémon in slot ${slot + 1} can only switch in once`);
    // 		}
    // 		const targetPokemon = this.pokemon[slot];
    // 
    // 		if (this.slotConditions[pokemon.position]['revivalblessing']) {
    // 			if (!targetPokemon.fainted) {
    // 				return this.emitChoiceError(`Can't switch: You have to pass to a fainted Pokémon`);
    // 			}
    // 			// Should always subtract, but stop at 0 to prevent errors.
    // 			this.choice.forcedSwitchesLeft = this.battle.clampIntRange(this.choice.forcedSwitchesLeft - 1, 0);
    // 			pokemon.switchFlag = false;
    // 			this.choice.actions.push({
    // 				choice: 'revivalblessing',
    // 				pokemon,
    // 				target: targetPokemon,
    // 			} as ChosenAction);
    // 			return true;
    // 		}
    // 
    // 		if (targetPokemon.fainted) {
    // 			return this.emitChoiceError(`Can't switch: You can't switch to a fainted Pokémon`);
    // 		}
    // 
    // 		if (this.requestState === 'move') {
    // 			if (pokemon.trapped) {
    // 				return this.emitChoiceError(`Can't switch: The active Pokémon is trapped`, { pokemon, update: req => {
    // 					let updated = false;
    // 					if (req.maybeTrapped) {
    // 						delete req.maybeTrapped;
    // 						updated = true;
    // 					}
    // 					if (!req.trapped) {
    // 						req.trapped = true;
    // 						updated = true;
    // 					}
    // 					return updated;
    // 				} });
    // 			} else if (pokemon.maybeTrapped) {
    // 				this.choice.cantUndo = true;
    // 			}
    // 		} else if (this.requestState === 'switch') {
    // 			if (!this.choice.forcedSwitchesLeft) {
    // 				throw new Error(`Player somehow switched too many Pokemon`);
    // 			}
    // 			this.choice.forcedSwitchesLeft--;
    // 		}
    // 
    // 		this.choice.switchIns.add(slot);
    // 
    // 		this.choice.actions.push({
    // 			choice: (this.requestState === 'switch' ? 'instaswitch' : 'switch'),
    // 			pokemon,
    // 			target: targetPokemon,
    // 		} as ChosenAction);
    // 
    // 		return true;
    // 	}
    //
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
    // 
    // 	chooseMove(
    // 		moveText?: string | number,
    // 		targetLoc = 0,
    // 		event: 'mega' | 'megax' | 'megay' | 'zmove' | 'ultra' | 'dynamax' | 'terastallize' | '' = ''
    // 	) {
    // 		if (this.requestState !== 'move') {
    // 			return this.emitChoiceError(`Can't move: You need a ${this.requestState} response`);
    // 		}
    // 		const index = this.getChoiceIndex();
    // 		if (index >= this.active.length) {
    // 			return this.emitChoiceError(`Can't move: You sent more choices than unfainted Pokémon.`);
    // 		}
    // 		const autoChoose = !moveText;
    // 		const pokemon: Pokemon = this.active[index];
    // 
    // 		// Parse moveText (name or index)
    // 		// If the move is not found, the action is invalid without requiring further inspection.
    // 
    // 		const request = pokemon.getMoveRequestData();
    // 		let moveid = '';
    // 		let targetType = '';
    // 		if (autoChoose) moveText = 1;
    // 		if (typeof moveText === 'number' || (moveText && /^[0-9]+$/.test(moveText))) {
    // 			// Parse a one-based move index.
    // 			const moveIndex = Number(moveText) - 1;
    // 			if (moveIndex < 0 || moveIndex >= request.moves.length || !request.moves[moveIndex]) {
    // 				return this.emitChoiceError(`Can't move: Your ${pokemon.name} doesn't have a move ${moveIndex + 1}`);
    // 			}
    // 			moveid = request.moves[moveIndex].id;
    // 			targetType = request.moves[moveIndex].target!;
    // 		} else {
    // 			// Parse a move ID.
    // 			// Move names are also allowed, but may cause ambiguity (see client issue #167).
    // 			moveid = toID(moveText);
    // 			if (moveid.startsWith('hiddenpower')) {
    // 				moveid = 'hiddenpower';
    // 			}
    // 			for (const move of request.moves) {
    // 				if (move.id !== moveid) continue;
    // 				targetType = move.target || 'normal';
    // 				break;
    // 			}
    // 			if (!targetType && ['', 'dynamax'].includes(event) && request.maxMoves) {
    // 				for (const [i, moveRequest] of request.maxMoves.maxMoves.entries()) {
    // 					if (moveid === moveRequest.move) {
    // 						moveid = request.moves[i].id;
    // 						targetType = moveRequest.target;
    // 						event = 'dynamax';
    // 						break;
    // 					}
    // 				}
    // 			}
    // 			if (!targetType && ['', 'zmove'].includes(event) && request.canZMove) {
    // 				for (const [i, moveRequest] of request.canZMove.entries()) {
    // 					if (!moveRequest) continue;
    // 					if (moveid === toID(moveRequest.move)) {
    // 						moveid = request.moves[i].id;
    // 						targetType = moveRequest.target;
    // 						event = 'zmove';
    // 						break;
    // 					}
    // 				}
    // 			}
    // 			if (!targetType) {
    // 				if (moveid !== 'testfight') {
    // 					return this.emitChoiceError(`Can't move: Your ${pokemon.name} doesn't have a move matching ${moveid}`);
    // 				}
    // 			}
    // 		}
    // 
    // 		const moves = pokemon.getMoves();
    // 		if (autoChoose) {
    // 			for (const [i, move] of request.moves.entries()) {
    // 				if (move.disabled) continue;
    // 				if (i < moves.length && move.id === moves[i].id && moves[i].disabled) continue;
    // 				moveid = move.id;
    // 				targetType = move.target!;
    // 				break;
    // 			}
    // 		}
    // 		const move = this.battle.dex.moves.get(moveid);
    // 
    // 		// Z-move
    // 
    // 		const zMove = event === 'zmove' ? this.battle.actions.getZMove(move, pokemon) : undefined;
    // 		if (event === 'zmove' && !zMove) {
    // 			return this.emitChoiceError(`Can't move: ${pokemon.name} can't use ${move.name} as a Z-move`);
    // 		}
    // 		if (zMove && this.choice.zMove) {
    // 			return this.emitChoiceError(`Can't move: You can't Z-move more than once per battle`);
    // 		}
    // 
    // 		if (zMove) targetType = this.battle.dex.moves.get(zMove).target;
    // 
    // 		// Dynamax
    // 		// Is dynamaxed or will dynamax this turn.
    // 		const maxMove = (event === 'dynamax' || pokemon.volatiles['dynamax']) ?
    // 			this.battle.actions.getMaxMove(move, pokemon) : undefined;
    // 		if (event === 'dynamax' && !maxMove) {
    // 			return this.emitChoiceError(`Can't move: ${pokemon.name} can't use ${move.name} as a Max Move`);
    // 		}
    // 
    // 		if (maxMove) targetType = this.battle.dex.moves.get(maxMove).target;
    // 
    // 		// Validate targeting
    // 
    // 		if (autoChoose || moveid === 'testfight') {
    // 			targetLoc = 0;
    // 		} else if (this.battle.actions.targetTypeChoices(targetType)) {
    // 			if (!targetLoc && this.active.length >= 2) {
    // 				return this.emitChoiceError(`Can't move: ${move.name} needs a target`);
    // 			}
    // 			if (!this.battle.validTargetLoc(targetLoc, pokemon, targetType)) {
    // 				return this.emitChoiceError(`Can't move: Invalid target for ${move.name}`);
    // 			}
    // 		} else {
    // 			if (targetLoc) {
    // 				return this.emitChoiceError(`Can't move: You can't choose a target for ${move.name}`);
    // 			}
    // 		}
    // 
    // 		const lockedMove = pokemon.getLockedMove();
    // 		if (lockedMove) {
    // 			let lockedMoveTargetLoc = pokemon.lastMoveTargetLoc || 0;
    // 			const lockedMoveID = toID(lockedMove);
    // 			if (pokemon.volatiles[lockedMoveID]?.targetLoc) {
    // 				lockedMoveTargetLoc = pokemon.volatiles[lockedMoveID].targetLoc;
    // 			}
    // 			if (pokemon.maybeLocked) this.choice.cantUndo = true;
    // 			this.choice.actions.push({
    // 				choice: 'move',
    // 				pokemon,
    // 				targetLoc: lockedMoveTargetLoc,
    // 				moveid: lockedMoveID,
    // 			});
    // 			return true;
    // 		} else if (!moves.length) {
    // 			// Override action and use Struggle if there are no enabled moves with PP
    // 			// Gen 4 and earlier announce a Pokemon has no moves left before the turn begins, and only to that player's side.
    // 			if (this.battle.gen <= 4) this.send('-activate', pokemon, 'move: Struggle');
    // 			if (pokemon.maybeLocked) this.choice.cantUndo = true;
    // 			this.choice.actions.push({
    // 				choice: 'move',
    // 				pokemon,
    // 				moveid: 'struggle',
    // 			});
    // 			return true;
    // 		} else if (moveid === 'testfight') {
    // 			// test fight button
    // 			if (!pokemon.maybeLocked) {
    // 				return this.emitChoiceError(`Can't move: ${pokemon.name}'s Fight button is known to be safe`);
    // 			}
    // 			this.updateRequestForPokemon(pokemon, req => this.updateDisabledRequest(pokemon, req));
    // 			this.emitRequest(this.activeRequest!, true);
    // 			this.choice.error = 'Hack to avoid sending error messages to the client :D';
    // 			return false;
    // 		} else if (maxMove) {
    // 			// Dynamaxed; only Taunt and Assault Vest disable Max Guard, but the base move must have PP remaining
    // 			if (pokemon.maxMoveDisabled(move)) {
    // 				return this.emitChoiceError(`Can't move: ${pokemon.name}'s ${maxMove.name} is disabled`);
    // 			}
    // 		} else if (!zMove) {
    // 			// Check for disabled moves
    // 			let isEnabled = false;
    // 			let disabledSource = '';
    // 			for (const m of moves) {
    // 				if (m.id !== moveid) continue;
    // 				if (!m.disabled) {
    // 					isEnabled = true;
    // 					break;
    // 				} else if (m.disabledSource) {
    // 					disabledSource = m.disabledSource;
    // 				}
    // 			}
    // 			if (!isEnabled) {
    // 				// Request a different choice
    // 				if (autoChoose) throw new Error(`autoChoose chose a disabled move`);
    // 				return this.emitChoiceError(`Can't move: ${pokemon.name}'s ${move.name} is disabled`, { pokemon, update: req => {
    // 					let updated = this.updateDisabledRequest(pokemon, req);
    // 					for (const m of req.moves) {
    // 						if (m.id === moveid) {
    // 							if (!m.disabled) {
    // 								m.disabled = true;
    // 								updated = true;
    // 							}
    // 							if (m.disabledSource !== disabledSource) {
    // 								m.disabledSource = disabledSource;
    // 								updated = true;
    // 							}
    // 							break;
    // 						}
    // 					}
    // 					return updated;
    // 				} });
    // 			}
    // 			// The chosen move is valid yay
    // 		}
    // 
    // 		// Mega evolution
    // 
    // 		const mixandmega = this.battle.format.mod === 'mixandmega';
    // 		const mega = (event === 'mega');
    // 		const megax = (event === 'megax');
    // 		const megay = (event === 'megay');
    // 		if (mega && !pokemon.canMegaEvo) {
    // 			return this.emitChoiceError(`Can't move: ${pokemon.name} can't mega evolve`);
    // 		}
    // 		if (megax && !pokemon.canMegaEvoX) {
    // 			return this.emitChoiceError(`Can't move: ${pokemon.name} can't mega evolve X`);
    // 		}
    // 		if (megay && !pokemon.canMegaEvoY) {
    // 			return this.emitChoiceError(`Can't move: ${pokemon.name} can't mega evolve Y`);
    // 		}
    // 		if ((mega || megax || megay) && this.choice.mega && !mixandmega) {
    // 			return this.emitChoiceError(`Can't move: You can only mega-evolve once per battle`);
    // 		}
    // 		const ultra = (event === 'ultra');
    // 		if (ultra && !pokemon.canUltraBurst) {
    // 			return this.emitChoiceError(`Can't move: ${pokemon.name} can't ultra burst`);
    // 		}
    // 		if (ultra && this.choice.ultra && !mixandmega) {
    // 			return this.emitChoiceError(`Can't move: You can only ultra burst once per battle`);
    // 		}
    // 		let dynamax = (event === 'dynamax');
    // 		const canDynamax = (this.activeRequest as MoveRequest)?.active[this.active.indexOf(pokemon)].canDynamax;
    // 		if (dynamax && (this.choice.dynamax || !canDynamax)) {
    // 			if (pokemon.volatiles['dynamax']) {
    // 				dynamax = false;
    // 			} else {
    // 				if (this.battle.gen !== 8) {
    // 					return this.emitChoiceError(`Can't move: Dynamaxing doesn't outside of Gen 8.`);
    // 				} else if (pokemon.side.canDynamaxNow()) {
    // 					return this.emitChoiceError(`Can't move: ${pokemon.name} can't Dynamax now.`);
    // 				} else if (pokemon.side.allySide?.canDynamaxNow()) {
    // 					return this.emitChoiceError(`Can't move: It's your partner's turn to Dynamax.`);
    // 				}
    // 				return this.emitChoiceError(`Can't move: You can only Dynamax once per battle.`);
    // 			}
    // 		}
    // 		const terastallize = (event === 'terastallize');
    // 		if (terastallize && !pokemon.canTerastallize) {
    // 			// Make this work properly
    // 			return this.emitChoiceError(`Can't move: ${pokemon.name} can't Terastallize.`);
    // 		}
    // 		if (terastallize && this.choice.terastallize) {
    // 			return this.emitChoiceError(`Can't move: You can only Terastallize once per battle.`);
    // 		}
    // 		if (terastallize && this.battle.gen !== 9) {
    // 			// Make this work properly
    // 			return this.emitChoiceError(`Can't move: You can only Terastallize in Gen 9.`);
    // 		}
    // 
    // 		this.choice.actions.push({
    // 			choice: 'move',
    // 			pokemon,
    // 			targetLoc,
    // 			moveid,
    // 			mega: mega || ultra,
    // 			megax,
    // 			megay,
    // 			zmove: zMove,
    // 			maxMove: maxMove ? maxMove.id : undefined,
    // 			terastallize: terastallize ? pokemon.teraType : undefined,
    // 		});
    // 
    // 		if (pokemon.maybeDisabled && (this.battle.gameType === 'singles' || (
    // 			this.battle.gen <= 3 && !this.battle.actions.targetTypeChoices(targetType)
    // 		))) {
    // 			this.choice.cantUndo = true;
    // 		}
    // 
    // 		if (mega || megax || megay) this.choice.mega = true;
    // 		if (ultra) this.choice.ultra = true;
    // 		if (zMove) this.choice.zMove = true;
    // 		if (dynamax) this.choice.dynamax = true;
    // 		if (terastallize) this.choice.terastallize = true;
    // 
    // 		return true;
    // 	}
    //
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
    // 
    // 	chooseTeam(data?: string) {
    // 		if (this.requestState !== 'teampreview') {
    // 			return this.emitChoiceError(`Can't choose for Team Preview: You're not in a Team Preview phase`);
    // 		}
    // 
    // 		const ruleTable = this.battle.ruleTable;
    // 		let positions = data ? data.split(data.includes(',') ? ',' : '').map(datum => parseInt(datum) - 1) :
    // 			[...this.pokemon.keys()]; // autoChoose
    // 		const pickedTeamSize = this.pickedTeamSize();
    // 
    // 		// make sure positions is exactly of length pickedTeamSize
    // 		// - If too big: the client automatically sends a full list, so we just trim it down to size
    // 		positions.splice(pickedTeamSize);
    // 		// - If too small: we intentionally support only sending leads and having the sim fill in the rest
    // 		if (positions.length < pickedTeamSize) {
    // 			for (let i = 0; i < pickedTeamSize; i++) {
    // 				if (!positions.includes(i)) positions.push(i);
    // 				// duplicate in input, let the rest of the code handle the error message
    // 				if (positions.length >= pickedTeamSize) break;
    // 			}
    // 		}
    // 
    // 		for (const [index, pos] of positions.entries()) {
    // 			if (isNaN(pos) || pos < 0 || pos >= this.pokemon.length) {
    // 				return this.emitChoiceError(`Can't choose for Team Preview: You do not have a Pokémon in slot ${pos + 1}`);
    // 			}
    // 			if (positions.indexOf(pos) !== index) {
    // 				return this.emitChoiceError(`Can't choose for Team Preview: The Pokémon in slot ${pos + 1} can only switch in once`);
    // 			}
    // 		}
    // 
    // 		const result = ruleTable.onChooseTeam?.[0].call(this.battle, positions, this.pokemon, !data);
    // 		if (result) {
    // 			if (typeof result === 'string') {
    // 				return this.emitChoiceError(`Can't choose for Team Preview: ${result}`);
    // 			}
    // 			if (result.length < pickedTeamSize) {
    // 				throw new Error(`onChooseTeam from ${ruleTable.onChooseTeam![1]} returned a team of size ${result.length}, which is less than the required size of ${pickedTeamSize}`);
    // 			}
    // 			positions = result.slice(0, pickedTeamSize);
    // 		}
    // 
    // 		for (const [index, pos] of positions.entries()) {
    // 			this.choice.switchIns.add(pos);
    // 			this.choice.actions.push({
    // 				choice: 'team',
    // 				index,
    // 				pokemon: this.pokemon[pos],
    // 				priority: -index,
    // 			} as ChosenAction);
    // 		}
    // 
    // 		return true;
    // 	}
    //
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
    // 
    // 	chooseShift() {
    // 		const index = this.getChoiceIndex();
    // 		if (index >= this.active.length) {
    // 			return this.emitChoiceError(`Can't shift: You do not have a Pokémon in slot ${index + 1}`);
    // 		} else if (this.requestState !== 'move') {
    // 			return this.emitChoiceError(`Can't shift: You can only shift during a move phase`);
    // 		} else if (this.battle.gameType !== 'triples') {
    // 			return this.emitChoiceError(`Can't shift: You can only shift to the center in triples`);
    // 		} else if (index === 1) {
    // 			return this.emitChoiceError(`Can't shift: You can only shift from the edge to the center`);
    // 		}
    // 		const pokemon: Pokemon = this.active[index];
    // 
    // 		this.choice.actions.push({
    // 			choice: 'shift',
    // 			pokemon,
    // 		} as ChosenAction);
    // 
    // 		return true;
    // 	}
    //
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
    // TypeScript source:
    // /** Automatically finish a choice if not currently complete. */
    // 	autoChoose() {
    // 		if (this.requestState === 'teampreview') {
    // 			if (!this.isChoiceDone()) this.chooseTeam();
    // 		} else if (this.requestState === 'switch') {
    // 			let i = 0;
    // 			while (!this.isChoiceDone()) {
    // 				if (!this.chooseSwitch()) throw new Error(`autoChoose switch crashed: ${this.choice.error}`);
    // 				i++;
    // 				if (i > 10) throw new Error(`autoChoose failed: infinite looping`);
    // 			}
    // 		} else if (this.requestState === 'move') {
    // 			let i = 0;
    // 			while (!this.isChoiceDone()) {
    // 				if (!this.chooseMove()) throw new Error(`autoChoose crashed: ${this.choice.error}`);
    // 				i++;
    // 				if (i > 10) throw new Error(`autoChoose failed: infinite looping`);
    // 			}
    // 		}
    // 		return true;
    // 	}
    //
    pub fn auto_choose(&mut self) -> bool {
        match self.request_state {
            RequestState::TeamPreview => {
                if !self.is_choice_done(None) {
                    let positions: Vec<usize> = (0..self.pokemon.len()).collect();
                    let _ = self.choose_team(positions);
                }
            }
            RequestState::Switch => {
                let mut iterations = 0;
                while !self.is_choice_done(None) && iterations < 10 {
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
                while !self.is_choice_done(None) && iterations < 10 {
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
    // TypeScript source:
    // /**
    // 	 * The number of pokemon you must choose in Team Preview.
    // 	 *
    // 	 * Note that PS doesn't support choosing fewer than this number of pokemon.
    // 	 * In the games, it is sometimes possible to bring fewer than this, but
    // 	 * since that's nearly always a mistake, we haven't gotten around to
    // 	 * supporting it.
    // 	 */
    // 	pickedTeamSize() {
    // 		return Math.min(this.pokemon.length, this.battle.ruleTable.pickedTeamSize || Infinity);
    // 	}
    //
    pub fn picked_team_size(&self, rule_table_size: Option<usize>) -> usize {
        rule_table_size.unwrap_or(self.pokemon.len()).min(self.pokemon.len())
    }

    /// Destroy side (cleanup)
    // 
    // 	destroy() {
    // 		// deallocate ourself
    // 
    // 		// deallocate children and get rid of references to them
    // 		for (const pokemon of this.pokemon) {
    // 			if (pokemon) pokemon.destroy();
    // 		}
    // 
    // 		for (const action of this.choice.actions) {
    // 			delete action.side;
    // 			delete action.pokemon;
    // 			delete action.target;
    // 		}
    // 		this.choice.actions = [];
    // 
    // 		// get rid of some possibly-circular references
    // 		this.pokemon = [];
    // 		this.active = [];
    // 		this.foe = null!;
    // 		(this as any).battle = null!;
    // 	}
    //
    pub fn destroy(&mut self) {
        self.pokemon.clear();
        self.active.clear();
        self.choice.actions.clear();
        self.side_conditions.clear();
        for slot_cond in &mut self.slot_conditions {
            slot_cond.clear();
        }
    }

    // =========================================================================
    // REMAINING METHODS (ported from side.ts for complete 1:1 port)
    // =========================================================================

    /// Get active team (for multi battles)
    /// Equivalent to side.ts activeTeam()
    // 	activeTeam() {
    // 		if (this.battle.gameType !== 'multi') return this.active;
    // 
    // 		return this.battle.sides[this.n % 2].active.concat(this.battle.sides[this.n % 2 + 2].active);
    // 	}
    //
    pub fn active_team(&self) -> Vec<usize> {
        self.active.iter()
            .filter_map(|&idx| idx)
            .collect()
    }

    /// Emit choice error to client
    /// Equivalent to side.ts emitChoiceError()
    // 
    // 	emitChoiceError(
    // 		message: string, update?: { pokemon: Pokemon, update: (req: PokemonMoveRequestData) => boolean | void }
    // 	) {
    // 		this.choice.error = message;
    // 		const updated = update ? this.updateRequestForPokemon(update.pokemon, update.update) : null;
    // 		const type = `[${updated ? 'Unavailable' : 'Invalid'} choice]`;
    // 		this.battle.send('sideupdate', `${this.id}\n|error|${type} ${message}`);
    // 		if (updated) this.emitRequest(this.activeRequest!, true);
    // 		if (this.battle.strictChoices) throw new Error(`${type} ${message}`);
    // 		return false;
    // 	}
    //
    pub fn emit_choice_error(&self, message: &str) -> bool {
        // In the full implementation, this would send to client
        // For now, just return false to indicate error was emitted
        let _ = message;
        false
    }

    /// Emit request to client
    /// Equivalent to side.ts emitRequest()
    // 
    // 	emitRequest(update: ChoiceRequest = this.activeRequest!, updatedRequest = false) {
    // 		if (updatedRequest) (this.activeRequest as MoveRequest | SwitchRequest).update = true;
    // 		this.battle.send('sideupdate', `${this.id}\n|request|${JSON.stringify(update)}`);
    // 		this.activeRequest = update;
    // 	}
    //
    pub fn emit_request(&self, request: &serde_json::Value) {
        // In the full implementation, this would send to client
        let _ = request;
    }

    /// Get all foes
    /// Equivalent to side.ts foes()
    // 	foes(all?: boolean) {
    // 		if (this.battle.gameType === 'freeforall') {
    // 			return this.battle.sides.map(side => side.active[0])
    // 				.filter(pokemon => pokemon && pokemon.side !== this && (all || !!pokemon.hp));
    // 		}
    // 		return this.foe.allies(all);
    // 	}
    //
    pub fn foes<'a>(&self, battle_sides: &'a [Side]) -> Vec<&'a Pokemon> {
        // Return all active Pokemon from opponent sides
        let mut foes = Vec::new();
        for (idx, side) in battle_sides.iter().enumerate() {
            if idx != self.n {
                for &active_idx in &side.active {
                    if let Some(poke_idx) = active_idx {
                        if let Some(pokemon) = side.pokemon.get(poke_idx) {
                            if !pokemon.fainted {
                                foes.push(pokemon);
                            }
                        }
                    }
                }
            }
        }
        foes
    }

    /// Iterate through foe side conditions
    /// Equivalent to side.ts foeSidesWithConditions()
    // TypeScript source:
    // /** Intended as a way to iterate through all foe side conditions - do not use for anything else. */
    // 	foeSidesWithConditions() {
    // 		if (this.battle.gameType === 'freeforall') return this.battle.sides.filter(side => side !== this);
    // 
    // 		return [this.foe];
    // 	}
    //
    pub fn foe_sides_with_conditions<'a>(&self, battle_sides: &'a [Side]) -> Vec<&'a Side> {
        battle_sides.iter()
            .filter(|s| s.n != self.n && !s.side_conditions.is_empty())
            .collect()
    }

    /// Get request data for protocol
    /// Equivalent to side.ts getRequestData()
    // 
    // 	getRequestData(forAlly?: boolean): SideRequestData {
    // 		const data: SideRequestData = {
    // 			name: this.name,
    // 			id: this.id,
    // 			pokemon: [] as PokemonSwitchRequestData[],
    // 		};
    // 		for (const pokemon of this.pokemon) {
    // 			data.pokemon.push(pokemon.getSwitchRequestData(forAlly));
    // 		}
    // 		return data;
    // 	}
    //
    pub fn get_request_data(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
            "id": self.id_str(),
            "pokemon": self.pokemon.iter().enumerate().map(|(i, p)| {
                serde_json::json!({
                    "ident": format!("{}: {}", self.id_str(), p.name),
                    "details": format!("{}, L{}", p.species_id.as_str(), p.level),
                    "condition": format!("{}/{}", p.hp, p.maxhp),
                    "active": self.active.contains(&Some(i)),
                    "moves": p.move_slots.iter().map(|m| m.id.as_str().to_string()).collect::<Vec<_>>(),
                    "ability": p.ability.as_str(),
                    "item": p.item.as_str()
                })
            }).collect::<Vec<_>>()
        })
    }

    /// Get side condition data
    /// Equivalent to side.ts getSideConditionData()
    // 
    // 	getSideConditionData(status: string | Effect): AnyObject {
    // 		status = this.battle.dex.conditions.get(status) as Effect;
    // 		return this.sideConditions[status.id] || null;
    // 	}
    //
    pub fn get_side_condition_data(&self, id: &ID) -> Option<&EffectState> {
        self.side_conditions.get(id)
    }

    /// Send message to client
    /// Equivalent to side.ts send()
    // 
    // 	send(...parts: (string | number | Function | AnyObject)[]) {
    // 		const sideUpdate = '|' + parts.map(part => {
    // 			if (typeof part !== 'function') return part;
    // 			return part(this);
    // 		}).join('|');
    // 		this.battle.send('sideupdate', `${this.id}\n${sideUpdate}`);
    // 	}
    //
    pub fn send(&self, _message: &str) {
        // In the full implementation, this would send to client
    }

    /// Convert to JSON
    /// Equivalent to side.ts toJSON()
    // 
    // 	toJSON(): AnyObject {
    // 		return State.serializeSide(this);
    // 	}
    //
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
            "id": self.id_str(),
            "n": self.n,
            "pokemonLeft": self.pokemon_left,
            "active": self.active,
            "sideConditions": self.side_conditions.keys().map(|k| k.as_str().to_string()).collect::<Vec<_>>()
        })
    }

    /// Update disabled moves in request
    /// Equivalent to side.ts updateDisabledRequest()
    // 
    // 	updateDisabledRequest(pokemon: Pokemon, req: PokemonMoveRequestData) {
    // 		let updated = false;
    // 		if (pokemon.maybeLocked) {
    // 			pokemon.maybeLocked = false;
    // 			delete req.maybeLocked;
    // 			updated = true;
    // 		}
    // 		if (pokemon.maybeDisabled && this.battle.gameType !== 'singles') {
    // 			if (this.battle.gen >= 4) {
    // 				pokemon.maybeDisabled = false;
    // 				delete req.maybeDisabled;
    // 				updated = true;
    // 			}
    // 			for (const m of req.moves) {
    // 				const disabled = pokemon.getMoveData(m.id)?.disabled;
    // 				if (disabled && (this.battle.gen >= 4 || this.battle.actions.targetTypeChoices(m.target!))) {
    // 					m.disabled = true;
    // 					updated = true;
    // 				}
    // 			}
    // 		}
    // 		if (req.moves.every(m => m.disabled || m.id === 'struggle')) {
    // 			if (req.canMegaEvo) {
    // 				req.canMegaEvo = false;
    // 				updated = true;
    // 			}
    // 			if (req.canMegaEvoX) {
    // 				req.canMegaEvoX = false;
    // 				updated = true;
    // 			}
    // 			if (req.canMegaEvoY) {
    // 				req.canMegaEvoY = false;
    // 				updated = true;
    // 			}
    // 			if (req.canUltraBurst) {
    // 				req.canUltraBurst = false;
    // 				updated = true;
    // 			}
    // 			if (req.canZMove) {
    // 				req.canZMove = undefined;
    // 				updated = true;
    // 			}
    // 			if (req.canDynamax) {
    // 				req.canDynamax = false;
    // 				delete req.maxMoves;
    // 				updated = true;
    // 			}
    // 			if (req.canTerastallize) {
    // 				req.canTerastallize = undefined;
    // 				updated = true;
    // 			}
    // 		}
    // 		return updated;
    // 	}
    //
    pub fn update_disabled_request(&mut self, pokemon_idx: usize) {
        // In the full implementation, this would update move disabled states
        // For now, just iterate through moves and check PP
        if let Some(pokemon) = self.pokemon.get_mut(pokemon_idx) {
            for slot in &mut pokemon.move_slots {
                if slot.pp == 0 {
                    slot.disabled = true;
                }
            }
        }
    }

    /// Update request for specific Pokemon
    /// Equivalent to side.ts updateRequestForPokemon()
    // 
    // 	updateRequestForPokemon(pokemon: Pokemon, update: (req: PokemonMoveRequestData) => boolean | void) {
    // 		if (!(this.activeRequest as MoveRequest)?.active) {
    // 			throw new Error(`Can't update a request without active Pokemon`);
    // 		}
    // 		const req = (this.activeRequest as MoveRequest).active[pokemon.position];
    // 		if (!req) throw new Error(`Pokemon not found in request's active field`);
    // 		return update(req) ?? true;
    // 	}
    //
    pub fn update_request_for_pokemon(&mut self, pokemon_idx: usize) {
        // In the full implementation, this would update request data
        // For now, just update disabled moves
        self.update_disabled_request(pokemon_idx);
    }

    /// Process a choice command
    /// Equivalent to side.ts choose()
    // 
    // 	choose(input: string) {
    // 		if (!this.requestState) {
    // 			return this.emitChoiceError(
    // 				this.battle.ended ? `Can't do anything: The game is over` : `Can't do anything: It's not your turn`
    // 			);
    // 		}
    // 
    // 		if (this.choice.cantUndo) {
    // 			return this.emitChoiceError(`Can't undo: A trapping/disabling effect would cause undo to leak information`);
    // 		}
    // 
    // 		this.clearChoice();
    // 
    // 		const choiceStrings = (input.startsWith('team ') ? [input] : input.split(','));
    // 
    // 		if (choiceStrings.length > this.active.length) {
    // 			return this.emitChoiceError(
    // 				`Can't make choices: You sent choices for ${choiceStrings.length} Pokémon, but this is a ${this.battle.gameType} game!`
    // 			);
    // 		}
    // 
    // 		for (const choiceString of choiceStrings) {
    // 			let [choiceType, data] = Utils.splitFirst(choiceString.trim(), ' ');
    // 			data = data.trim();
    // 			if (choiceType === 'testfight') {
    // 				choiceType = 'move';
    // 				data = 'testfight';
    // 			}
    // 
    // 			switch (choiceType) {
    // 			case 'move':
    // 				const original = data;
    // 				const error = () => this.emitChoiceError(`Conflicting arguments for "move": ${original}`);
    // 				let targetLoc: number | undefined;
    // 				let event: 'mega' | 'megax' | 'megay' | 'zmove' | 'ultra' | 'dynamax' | 'terastallize' | '' = '';
    // 				while (true) {
    // 					// If data ends with a number, treat it as a target location.
    // 					// We need to special case 'Conversion 2' so it doesn't get
    // 					// confused with 'Conversion' erroneously sent with the target
    // 					// '2' (since Conversion targets 'self', targetLoc can't be 2).
    // 					if (/\s(?:-|\+)?[1-3]$/.test(data) && toID(data) !== 'conversion2') {
    // 						if (targetLoc !== undefined) return error();
    // 						targetLoc = parseInt(data.slice(-2));
    // 						data = data.slice(0, -2).trim();
    // 					} else if (data.endsWith(' mega')) {
    // 						if (event) return error();
    // 						event = 'mega';
    // 						data = data.slice(0, -5);
    // 					} else if (data.endsWith(' megax')) {
    // 						if (event) return error();
    // 						event = 'megax';
    // 						data = data.slice(0, -6);
    // 					} else if (data.endsWith(' megay')) {
    // 						if (event) return error();
    // 						event = 'megay';
    // 						data = data.slice(0, -6);
    // 					} else if (data.endsWith(' zmove')) {
    // 						if (event) return error();
    // 						event = 'zmove';
    // 						data = data.slice(0, -6);
    // 					} else if (data.endsWith(' ultra')) {
    // 						if (event) return error();
    // 						event = 'ultra';
    // 						data = data.slice(0, -6);
    // 					} else if (data.endsWith(' dynamax')) {
    // 						if (event) return error();
    // 						event = 'dynamax';
    // 						data = data.slice(0, -8);
    // 					} else if (data.endsWith(' gigantamax')) {
    // 						if (event) return error();
    // 						event = 'dynamax';
    // 						data = data.slice(0, -11);
    // 					} else if (data.endsWith(' max')) {
    // 						if (event) return error();
    // 						event = 'dynamax';
    // 						data = data.slice(0, -4);
    // 					} else if (data.endsWith(' terastal')) {
    // 						if (event) return error();
    // 						event = 'terastallize';
    // 						data = data.slice(0, -9);
    // 					} else if (data.endsWith(' terastallize')) {
    // 						if (event) return error();
    // 						event = 'terastallize';
    // 						data = data.slice(0, -13);
    // 					} else {
    // 						break;
    // 					}
    // 				}
    // 				if (!this.chooseMove(data, targetLoc, event)) return false;
    // 				break;
    // 			case 'switch':
    // 				this.chooseSwitch(data);
    // 				break;
    // 			case 'shift':
    // 				if (data) return this.emitChoiceError(`Unrecognized data after "shift": ${data}`);
    // 				if (!this.chooseShift()) return false;
    // 				break;
    // 			case 'team':
    // 				if (!this.chooseTeam(data)) return false;
    // 				break;
    // 			case 'pass':
    // 			case 'skip':
    // 				if (data) return this.emitChoiceError(`Unrecognized data after "pass": ${data}`);
    // 				if (!this.choosePass()) return false;
    // 				break;
    // 			case 'auto':
    // 			case 'default':
    // 				this.autoChoose();
    // 				break;
    // 			default:
    // 				this.emitChoiceError(`Unrecognized choice: ${choiceString}`);
    // 				break;
    // 			}
    // 		}
    // 
    // 		return !this.choice.error;
    // 	}
    //
    pub fn choose(&mut self, input: &str) -> Result<bool, String> {
        // Parse and execute choice commands
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return Err("Empty choice".to_string());
        }

        match parts[0] {
            "pass" => {
                self.choose_pass();
                Ok(true)
            }
            "switch" => {
                if parts.len() < 2 {
                    return Err("Switch requires target".to_string());
                }
                let target: usize = parts[1].parse().map_err(|_| "Invalid switch target")?;
                self.choose_switch(target - 1)?; // 1-indexed in protocol
                Ok(true)
            }
            "move" => {
                if parts.len() < 2 {
                    return Err("Move requires target".to_string());
                }
                let move_idx: usize = parts[1].parse().map_err(|_| "Invalid move")?;
                let index = self.get_choice_index();
                if let Some(Some(poke_idx)) = self.active.get(index) {
                    if let Some(slot) = self.pokemon[*poke_idx].move_slots.get(move_idx - 1) {
                        let move_id = slot.id.clone();
                        self.choose_move(move_id, None, false, None, None, None)?;
                        Ok(true)
                    } else {
                        Err("Invalid move index".to_string())
                    }
                } else {
                    Err("No active Pokemon".to_string())
                }
            }
            "team" => {
                let positions: Result<Vec<usize>, _> = parts[1..]
                    .iter()
                    .map(|s| s.parse::<usize>().map(|n| n - 1))
                    .collect();
                match positions {
                    Ok(pos) => {
                        self.choose_team(pos)?;
                        Ok(true)
                    }
                    Err(_) => Err("Invalid team positions".to_string())
                }
            }
            _ => Err(format!("Unknown choice: {}", parts[0]))
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
