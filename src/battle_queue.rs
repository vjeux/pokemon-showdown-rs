//! Simulator Battle Action Queue
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! The action queue is the core of the battle simulation. A rough overview of
//! the core battle loop:
//!
//! - chosen moves/switches are added to the action queue
//! - the action queue is sorted in speed/priority order
//! - we go through the action queue
//! - repeat

use serde::{Deserialize, Serialize};
use crate::dex_data::ID;

/// Move action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveAction {
    /// Action type
    pub choice: MoveActionType,
    /// Order for sorting (lower = earlier)
    pub order: i32,
    /// Priority of the action (higher = earlier)
    pub priority: i8,
    /// Fractional priority (higher = earlier)
    pub fractional_priority: f64,
    /// Speed of pokemon using move (higher = earlier if priority tie)
    pub speed: i32,
    /// Index of the pokemon doing the move
    pub pokemon_index: usize,
    /// Side index of the pokemon
    pub side_index: usize,
    /// Location of the target, relative to pokemon's side
    pub target_loc: i8,
    /// Move ID
    pub move_id: ID,
    /// True if mega evolving
    pub mega: bool,
    /// Z-move name if using Z-move
    pub zmove: Option<String>,
    /// Max move name if dynamaxed
    pub max_move: Option<String>,
    /// Tera type if terastallizing
    pub terastallize: Option<String>,
    /// Modified move priority for Quick Guard detection (Gen 6+)
    /// JavaScript: action.move.priority = priority
    /// Stores the priority value assigned to the move itself, allowing Quick Guard
    /// to detect if the move's priority was artificially enhanced (e.g., by Prankster)
    pub move_priority_modified: Option<i8>,
}

impl MoveAction {
    /// Get move data from Dex
    /// Equivalent to accessing action.move in TypeScript
    /// Returns MoveData for this action's move
    pub fn get_move<'a>(&self, dex: &'a crate::dex::Dex) -> Option<&'a crate::dex::MoveData> {
        dex.get_move(self.move_id.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoveActionType {
    Move,
    BeforeTurnMove,
    PriorityChargeMove,
}

/// Switch action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitchAction {
    /// Action type
    pub choice: SwitchActionType,
    /// Order for sorting
    pub order: i32,
    /// Priority of the action
    pub priority: i8,
    /// Speed of pokemon switching
    pub speed: i32,
    /// Index of the pokemon doing the switch
    pub pokemon_index: usize,
    /// Side index of the pokemon
    pub side_index: usize,
    /// Index of pokemon to switch to
    pub target_index: usize,
    /// Effect that caused the switch
    pub source_effect: Option<ID>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SwitchActionType {
    Switch,
    InstaSwitch,
    RevivalBlessing,
}

/// Team preview choice action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamAction {
    /// Priority (always 1 for team actions)
    pub priority: i8,
    /// Pokemon index
    pub pokemon_index: usize,
    /// Side index
    pub side_index: usize,
    /// New index in team order
    pub index: usize,
}

/// Field action (not done by a pokemon)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldAction {
    /// Action type
    pub choice: FieldActionType,
    /// Priority
    pub priority: i8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldActionType {
    Start,
    Residual,
    Pass,
    BeforeTurn,
}

/// Pokemon action (misc actions by a single pokemon)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonAction {
    /// Action type
    pub choice: PokemonActionType,
    /// Order for sorting
    pub order: i32,
    /// Priority
    pub priority: i8,
    /// Speed
    pub speed: i32,
    /// Pokemon index
    pub pokemon_index: usize,
    /// Side index
    pub side_index: usize,
    /// Event name (for event actions)
    pub event: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PokemonActionType {
    Start,
    BeforeTurn,
    MegaEvo,
    MegaEvoX,
    MegaEvoY,
    Shift,
    RunSwitch,
    Event,
    RunDynamax,
    Terastallize,
    Residual,
}

/// All possible actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    Move(MoveAction),
    Switch(SwitchAction),
    Team(TeamAction),
    Field(FieldAction),
    Pokemon(PokemonAction),
}

impl Action {
    /// Get the order value for sorting
    pub fn order(&self) -> i32 {
        match self {
            Action::Move(a) => a.order,
            Action::Switch(a) => a.order,
            Action::Team(_) => 1,
            Action::Field(a) => match a.choice {
                FieldActionType::Start => 2,
                FieldActionType::BeforeTurn => 4,
                FieldActionType::Pass => 200,
                FieldActionType::Residual => 300,
            },
            Action::Pokemon(a) => if a.order != 0 { a.order } else {
                match a.choice {
                    PokemonActionType::Start => 2,
                    PokemonActionType::BeforeTurn => 4,
                    PokemonActionType::RunSwitch => 101,
                    PokemonActionType::MegaEvo |
                    PokemonActionType::MegaEvoX |
                    PokemonActionType::MegaEvoY => 104,
                    PokemonActionType::RunDynamax => 105,
                    PokemonActionType::Terastallize => 106,
                    PokemonActionType::Shift => 200,
                    PokemonActionType::Event => 200,
                    PokemonActionType::Residual => 300,
                }
            },
        }
    }

    /// Get the priority value for sorting
    pub fn priority(&self) -> i8 {
        match self {
            Action::Move(a) => a.priority,
            Action::Switch(a) => a.priority,
            Action::Team(a) => a.priority,
            Action::Field(a) => a.priority,
            Action::Pokemon(a) => a.priority,
        }
    }

    /// Get the speed value for sorting
    pub fn speed(&self) -> i32 {
        match self {
            Action::Move(a) => a.speed,
            Action::Switch(a) => a.speed,
            Action::Team(_) => 1,
            Action::Field(_) => 1,
            Action::Pokemon(a) => a.speed,
        }
    }

    /// Get the fractional priority
    pub fn fractional_priority(&self) -> f64 {
        match self {
            Action::Move(a) => a.fractional_priority,
            _ => 0.0,
        }
    }

    /// Get pokemon index if applicable
    pub fn pokemon_index(&self) -> Option<usize> {
        match self {
            Action::Move(a) => Some(a.pokemon_index),
            Action::Switch(a) => Some(a.pokemon_index),
            Action::Team(a) => Some(a.pokemon_index),
            Action::Field(_) => None,
            Action::Pokemon(a) => Some(a.pokemon_index),
        }
    }

    /// Get side index if applicable
    pub fn side_index(&self) -> Option<usize> {
        match self {
            Action::Move(a) => Some(a.side_index),
            Action::Switch(a) => Some(a.side_index),
            Action::Team(a) => Some(a.side_index),
            Action::Field(_) => None,
            Action::Pokemon(a) => Some(a.side_index),
        }
    }

    /// Check if this is a move action
    pub fn is_move(&self) -> bool {
        matches!(self, Action::Move(_))
    }

    /// Check if this is a switch action
    pub fn is_switch(&self) -> bool {
        matches!(self, Action::Switch(_))
    }

    /// Check if this is a runSwitch action
    pub fn is_run_switch(&self) -> bool {
        matches!(self, Action::Pokemon(p) if p.choice == PokemonActionType::RunSwitch)
    }

    /// Get the switch target (side_idx, pokemon_idx) for Pokemon actions
    pub fn get_switch_target(&self) -> Option<(usize, usize)> {
        match self {
            Action::Pokemon(a) => Some((a.side_index, a.pokemon_index)),
            _ => None,
        }
    }
}

/// The battle queue - manages the order of actions in a turn
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BattleQueue {
    /// List of actions
    pub list: Vec<Action>,
}

impl BattleQueue {
    /// Create a new empty battle queue
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    /// Get the next action (removes from front)
    // TypeScript source:
    // 
    // 
    // 	shift() {
    // 		return this.list.shift();
    // 	}
    //
    pub fn shift(&mut self) -> Option<Action> {
        if self.list.is_empty() {
            None
        } else {
            Some(self.list.remove(0))
        }
    }

    /// Peek at the next action without removing
    // 	peek(end?: boolean): Action | undefined {
    // 		return this.list[end ? this.list.length - 1 : 0];
    // 	}
    //
    pub fn peek(&self) -> Option<&Action> {
        self.list.first()
    }

    /// Peek at the last action
    pub fn peek_end(&self) -> Option<&Action> {
        self.list.last()
    }

    /// Push an action to the end
    // 	push(action: Action) {
    // 		return this.list.push(action);
    // 	}
    //
    pub fn push(&mut self, action: Action) {
        self.list.push(action);
    }

    /// Unshift an action to the front
    // 	unshift(action: Action) {
    // 		return this.list.unshift(action);
    // 	}
    //
    pub fn unshift(&mut self, action: Action) {
        self.list.insert(0, action);
    }

    /// Get the number of actions
    pub fn len(&self) -> usize {
        self.list.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    /// Clear all actions
    // 
    // 	clear() {
    // 		this.list = [];
    // 	}
    //
    pub fn clear(&mut self) {
        self.list.clear();
    }

    /// Cancel all actions for a specific pokemon
    // 
    // 	cancelAction(pokemon: Pokemon) {
    // 		const oldLength = this.list.length;
    // 		for (let i = 0; i < this.list.length; i++) {
    // 			if (this.list[i].pokemon === pokemon) {
    // 				this.list.splice(i, 1);
    // 				i--;
    // 			}
    // 		}
    // 		return this.list.length !== oldLength;
    // 	}
    //
    pub fn cancel_action(&mut self, side_index: usize, pokemon_index: usize) -> bool {
        let old_len = self.list.len();
        self.list.retain(|action| {
            !(action.side_index() == Some(side_index) && action.pokemon_index() == Some(pokemon_index))
        });
        self.list.len() != old_len
    }

    /// Cancel move action for a specific pokemon
    // 
    // 	cancelMove(pokemon: Pokemon) {
    // 		for (const [i, action] of this.list.entries()) {
    // 			if (action.choice === 'move' && action.pokemon === pokemon) {
    // 				this.list.splice(i, 1);
    // 				return true;
    // 			}
    // 		}
    // 		return false;
    // 	}
    //
    pub fn cancel_move(&mut self, side_index: usize, pokemon_index: usize) -> bool {
        let pos = self.list.iter().position(|action| {
            action.is_move() &&
            action.side_index() == Some(side_index) &&
            action.pokemon_index() == Some(pokemon_index)
        });
        if let Some(i) = pos {
            self.list.remove(i);
            true
        } else {
            false
        }
    }

    /// Check if a pokemon will move this turn
    // 
    // 	willMove(pokemon: Pokemon) {
    // 		if (pokemon.fainted) return null;
    // 		for (const action of this.list) {
    // 			if (action.choice === 'move' && action.pokemon === pokemon) {
    // 				return action;
    // 			}
    // 		}
    // 		return null;
    // 	}
    //
    pub fn will_move(&self, side_index: usize, pokemon_index: usize) -> Option<&MoveAction> {
        for action in &self.list {
            if let Action::Move(move_action) = action {
                if move_action.side_index == side_index && move_action.pokemon_index == pokemon_index {
                    return Some(move_action);
                }
            }
        }
        None
    }

    /// Check if a pokemon will move this turn (by Pokemon reference)
    /// JavaScript pattern: this.queue.willMove(target)
    pub fn will_move_pokemon(&self, pokemon: &crate::pokemon::Pokemon) -> Option<&MoveAction> {
        self.will_move(pokemon.side_index, pokemon.position)
    }

    /// Check if a pokemon will switch this turn
    // 
    // 	willSwitch(pokemon: Pokemon) {
    // 		for (const action of this.list) {
    // 			if (['switch', 'instaswitch'].includes(action.choice) && action.pokemon === pokemon) {
    // 				return action;
    // 			}
    // 		}
    // 		return null;
    // 	}
    //
    pub fn will_switch(&self, side_index: usize, pokemon_index: usize) -> Option<&SwitchAction> {
        for action in &self.list {
            if let Action::Switch(switch_action) = action {
                if switch_action.side_index == side_index && switch_action.pokemon_index == pokemon_index {
                    return Some(switch_action);
                }
            }
        }
        None
    }

    /// Check if any pokemon will act
    // 
    // 	willAct() {
    // 		for (const action of this.list) {
    // 			if (['move', 'switch', 'instaswitch', 'shift'].includes(action.choice)) {
    // 				return action;
    // 			}
    // 		}
    // 		return null;
    // 	}
    //
    pub fn will_act(&self) -> bool {
        self.list.iter().any(|action| {
            matches!(action, Action::Move(_) | Action::Switch(_))
        })
    }

    /// Insert a runSwitch action for a pokemon
    /// This queues the switch-in effects to happen at the right time
    pub fn insert_run_switch(&mut self, side_index: usize, pokemon_index: usize) {
        let action = Action::Pokemon(PokemonAction {
            choice: PokemonActionType::RunSwitch,
            order: 101,
            priority: 0,
            speed: 1, // Speed doesn't matter for runSwitch
            pokemon_index,
            side_index,
            event: None,
        });
        self.list.push(action);
    }

    /// Insert a choice at the front of the queue (for immediate execution)
    // TypeScript source:
    // /**
    // 	 * Inserts the passed action into the action queue when it normally
    // 	 * would have happened (sorting by priority/speed), without
    // 	 * re-sorting the existing actions.
    // 	 */
    // 	insertChoice(choices: ActionChoice | ActionChoice[], midTurn = false) {
    // 		if (Array.isArray(choices)) {
    // 			for (const choice of choices) {
    // 				this.insertChoice(choice);
    // 			}
    // 			return;
    // 		}
    // 		const choice = choices;
    // 
    // 		if (choice.pokemon) {
    // 			choice.pokemon.updateSpeed();
    // 		}
    // 		const actions = this.resolveAction(choice, midTurn);
    // 
    // 		let firstIndex = null;
    // 		let lastIndex = null;
    // 		for (const [i, curAction] of this.list.entries()) {
    // 			const compared = this.battle.comparePriority(actions[0], curAction);
    // 			if (compared <= 0 && firstIndex === null) {
    // 				firstIndex = i;
    // 			}
    // 			if (compared < 0) {
    // 				lastIndex = i;
    // 				break;
    // 			}
    // 		}
    // 
    // 		if (firstIndex === null) {
    // 			this.list.push(...actions);
    // 		} else {
    // 			if (lastIndex === null) lastIndex = this.list.length;
    // 			const index = firstIndex === lastIndex ? firstIndex : this.battle.random(firstIndex, lastIndex + 1);
    // 			this.list.splice(index, 0, ...actions);
    // 		}
    // 	}
    //
    pub fn insert_choice(&mut self, action: Action) {
        self.list.insert(0, action);
    }

    /// Sort the queue by priority
    /// Order: order (lower first), priority (higher first), speed (higher first)
    // 
    // 	sort() {
    // 		// this.log.push('SORT ' + this.debugQueue());
    // 		this.battle.speedSort(this.list);
    // 		return this;
    // 	}
    //
    pub fn sort(&mut self) {
        self.list.sort_by(|a, b| {
            // Order: lower first
            let order_cmp = a.order().cmp(&b.order());
            if order_cmp != std::cmp::Ordering::Equal {
                return order_cmp;
            }

            // Priority: higher first
            let priority_cmp = b.priority().cmp(&a.priority());
            if priority_cmp != std::cmp::Ordering::Equal {
                return priority_cmp;
            }

            // Fractional priority: higher first
            let frac_a = a.fractional_priority();
            let frac_b = b.fractional_priority();
            if frac_a != frac_b {
                return frac_b.partial_cmp(&frac_a).unwrap_or(std::cmp::Ordering::Equal);
            }

            // Speed: higher first
            b.speed().cmp(&a.speed())
        });
    }

    /// Prioritize an action (move it to the front)
    // TypeScript source:
    // /**
    // 	 * Makes the passed action happen next (skipping speed order).
    // 	 */
    // 	prioritizeAction(action: MoveAction | SwitchAction, sourceEffect?: Effect) {
    // 		for (const [i, curAction] of this.list.entries()) {
    // 			if (curAction === action) {
    // 				this.list.splice(i, 1);
    // 				break;
    // 			}
    // 		}
    // 		action.sourceEffect = sourceEffect;
    // 		action.order = 3;
    // 		this.list.unshift(action);
    // 	}
    //
    pub fn prioritize_action(&mut self, side_index: usize, pokemon_index: usize) -> bool {
        let pos = self.list.iter().position(|action| {
            action.side_index() == Some(side_index) &&
            action.pokemon_index() == Some(pokemon_index)
        });
        if let Some(i) = pos {
            let action = self.list.remove(i);
            self.list.insert(0, action);
            true
        } else {
            false
        }
    }

    /// Get an iterator over the actions
    pub fn iter(&self) -> impl Iterator<Item = &Action> {
        self.list.iter()
    }

    /// Get a mutable iterator over the actions
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Action> {
        self.list.iter_mut()
    }

    // ==========================================
    // Methods ported from battle-queue.ts
    // ==========================================

    /// Check if will act (has move/switch/shift action)
    pub fn will_act_full(&self) -> Option<&Action> {
        for action in &self.list {
            match action {
                Action::Move(_) | Action::Switch(_) => return Some(action),
                Action::Pokemon(p) if p.choice == PokemonActionType::Shift => return Some(action),
                _ => {}
            }
        }
        None
    }

    /// Change an action for a pokemon (cancel and reinsert)
    // TypeScript source:
    // /**
    // 	 * Changes a pokemon's action, and inserts its new action
    // 	 * in priority order.
    // 	 *
    // 	 * You'd normally want the OverrideAction event (which doesn't
    // 	 * change priority order).
    // 	 */
    // 	changeAction(pokemon: Pokemon, action: ActionChoice) {
    // 		this.cancelAction(pokemon);
    // 		if (!action.pokemon) action.pokemon = pokemon;
    // 		this.insertChoice(action);
    // 	}
    //
    pub fn change_action(&mut self, side_index: usize, pokemon_index: usize, new_action: Action) {
        self.cancel_action(side_index, pokemon_index);
        // Insert in priority order
        self.insert_in_order(new_action);
    }

    /// Insert action maintaining sort order
    pub fn insert_in_order(&mut self, action: Action) {
        // Find the right position based on priority
        let mut insert_pos = self.list.len();

        for (i, existing) in self.list.iter().enumerate() {
            // Order: lower first
            let order_cmp = action.order().cmp(&existing.order());
            if order_cmp == std::cmp::Ordering::Less {
                insert_pos = i;
                break;
            } else if order_cmp == std::cmp::Ordering::Greater {
                continue;
            }

            // Priority: higher first
            if action.priority() > existing.priority() {
                insert_pos = i;
                break;
            } else if action.priority() < existing.priority() {
                continue;
            }

            // Fractional priority: higher first
            if action.fractional_priority() > existing.fractional_priority() {
                insert_pos = i;
                break;
            } else if action.fractional_priority() < existing.fractional_priority() {
                continue;
            }

            // Speed: higher first
            if action.speed() > existing.speed() {
                insert_pos = i;
                break;
            }
        }

        self.list.insert(insert_pos, action);
    }

    /// Add one or more action choices and resolve them
    // 
    // 	addChoice(choices: ActionChoice | ActionChoice[]) {
    // 		if (!Array.isArray(choices)) choices = [choices];
    // 		for (const choice of choices) {
    // 			const resolvedChoices = this.resolveAction(choice);
    // 			this.list.push(...resolvedChoices);
    // 			for (const resolvedChoice of resolvedChoices) {
    // 				if (resolvedChoice && resolvedChoice.choice === 'move' && resolvedChoice.move.id !== 'recharge') {
    // 					resolvedChoice.pokemon.side.lastSelectedMove = resolvedChoice.move.id;
    // 				}
    // 			}
    // 		}
    // 	}
    //
    pub fn add_choice(&mut self, action: Action) {
        self.list.push(action);
    }

    /// Debug output for queue state
    // 
    // 	debug(action?: any): string {
    // 		if (action) {
    // 			return `${action.order || ''}:${action.priority || ''}:${action.speed || ''}:${action.subOrder || ''} - ${action.choice}${action.pokemon ? ' ' + action.pokemon : ''}${action.move ? ' ' + action.move : ''}`;
    // 		}
    // 		return this.list.map(
    // 			queueAction => this.debug(queueAction)
    // 		).join('\n') + '\n';
    // 	}
    //
    pub fn debug(&self) -> String {
        self.list.iter().map(|action| {
            match action {
                Action::Move(m) => format!(
                    "{}:{}:{}:0 - move {} (side {}, pos {})",
                    m.order, m.priority, m.speed, m.move_id.as_str(), m.side_index, m.pokemon_index
                ),
                Action::Switch(s) => format!(
                    "{}:{}:{}:0 - switch (side {}, pos {} -> {})",
                    s.order, s.priority, s.speed, s.side_index, s.pokemon_index, s.target_index
                ),
                Action::Team(t) => format!(
                    "1:{}:1:0 - team (side {}, pos {})",
                    t.priority, t.side_index, t.pokemon_index
                ),
                Action::Field(f) => format!(
                    "{}:{}:1:0 - {:?}",
                    action.order(), f.priority, f.choice
                ),
                Action::Pokemon(p) => format!(
                    "{}:{}:{}:0 - {:?} (side {}, pos {})",
                    action.order(), p.priority, p.speed, p.choice, p.side_index, p.pokemon_index
                ),
            }
        }).collect::<Vec<_>>().join("\n")
    }

    /// Get entries as iterator with indices
    // 	entries() {
    // 		return this.list.entries();
    // 	}
    //
    pub fn entries(&self) -> impl Iterator<Item = (usize, &Action)> {
        self.list.iter().enumerate()
    }

    /// Get mutable entries
    pub fn entries_mut(&mut self) -> impl Iterator<Item = (usize, &mut Action)> {
        self.list.iter_mut().enumerate()
    }

    /// Find a specific action by predicate
    pub fn find<F>(&self, predicate: F) -> Option<&Action>
    where
        F: Fn(&Action) -> bool,
    {
        self.list.iter().find(|action| predicate(action))
    }

    /// Remove actions matching predicate
    pub fn remove_where<F>(&mut self, predicate: F) -> Vec<Action>
    where
        F: Fn(&Action) -> bool,
    {
        let mut removed = Vec::new();
        let mut i = 0;
        while i < self.list.len() {
            if predicate(&self.list[i]) {
                removed.push(self.list.remove(i));
            } else {
                i += 1;
            }
        }
        removed
    }

    /// Prioritize an action by action itself (move to front with new order)
    pub fn prioritize_action_ref(&mut self, action: &Action) -> bool {
        let pos = self.list.iter().position(|a| {
            a.side_index() == action.side_index() &&
            a.pokemon_index() == action.pokemon_index() &&
            std::mem::discriminant(a) == std::mem::discriminant(action)
        });
        if let Some(i) = pos {
            let mut removed = self.list.remove(i);
            // Set order to 3 (high priority)
            match &mut removed {
                Action::Move(m) => m.order = 3,
                Action::Switch(s) => s.order = 3,
                _ => {}
            }
            self.list.insert(0, removed);
            true
        } else {
            false
        }
    }

    /// Resolve an action choice into one or more actions
    /// Equivalent to resolveAction in battle-queue.ts
    /// This creates the appropriate order values and may add additional actions
    /// (like megaEvo, terastallize, etc.) based on the choice
    // TypeScript source:
    // /**
    // 	 * Takes an ActionChoice, and fills it out into a full Action object.
    // 	 *
    // 	 * Returns an array of Actions because some ActionChoices (like mega moves)
    // 	 * resolve to two Actions (mega evolution + use move)
    // 	 */
    // 	resolveAction(action: ActionChoice, midTurn = false): Action[] {
    // 		if (!action) throw new Error(`Action not passed to resolveAction`);
    // 		if (action.choice === 'pass') return [];
    // 		const actions = [action];
    // 
    // 		if (!action.side && action.pokemon) action.side = action.pokemon.side;
    // 		if (!action.move && action.moveid) action.move = this.battle.dex.getActiveMove(action.moveid);
    // 		if (!action.order) {
    // 			const orders: { [choice: string]: number } = {
    // 				team: 1,
    // 				start: 2,
    // 				instaswitch: 3,
    // 				beforeTurn: 4,
    // 				beforeTurnMove: 5,
    // 				revivalblessing: 6,
    // 
    // 				runSwitch: 101,
    // 				switch: 103,
    // 				megaEvo: 104,
    // 				megaEvoX: 104,
    // 				megaEvoY: 104,
    // 				runDynamax: 105,
    // 				terastallize: 106,
    // 				priorityChargeMove: 107,
    // 
    // 				shift: 200,
    // 				// default is 200 (for moves)
    // 
    // 				residual: 300,
    // 			};
    // 			if (action.choice in orders) {
    // 				action.order = orders[action.choice];
    // 			} else {
    // 				action.order = 200;
    // 				if (!['move', 'event'].includes(action.choice)) {
    // 					throw new Error(`Unexpected orderless action ${action.choice}`);
    // 				}
    // 			}
    // 		}
    // 		if (!midTurn) {
    // 			if (action.choice === 'move') {
    // 				if (!action.maxMove && !action.zmove && action.move.beforeTurnCallback) {
    // 					actions.unshift(...this.resolveAction({
    // 						choice: 'beforeTurnMove', pokemon: action.pokemon, move: action.move, targetLoc: action.targetLoc,
    // 					}));
    // 				}
    // 				if (action.mega && !action.pokemon.isSkyDropped()) {
    // 					actions.unshift(...this.resolveAction({
    // 						choice: 'megaEvo',
    // 						pokemon: action.pokemon,
    // 					}));
    // 				}
    // 				if (action.megax && !action.pokemon.isSkyDropped()) {
    // 					actions.unshift(...this.resolveAction({
    // 						choice: 'megaEvoX',
    // 						pokemon: action.pokemon,
    // 					}));
    // 				}
    // 				if (action.megay && !action.pokemon.isSkyDropped()) {
    // 					actions.unshift(...this.resolveAction({
    // 						choice: 'megaEvoY',
    // 						pokemon: action.pokemon,
    // 					}));
    // 				}
    // 				if (action.terastallize && !action.pokemon.terastallized) {
    // 					actions.unshift(...this.resolveAction({
    // 						choice: 'terastallize',
    // 						pokemon: action.pokemon,
    // 					}));
    // 				}
    // 				if (action.maxMove && !action.pokemon.volatiles['dynamax']) {
    // 					actions.unshift(...this.resolveAction({
    // 						choice: 'runDynamax',
    // 						pokemon: action.pokemon,
    // 					}));
    // 				}
    // 				if (!action.maxMove && !action.zmove && action.move.priorityChargeCallback) {
    // 					actions.unshift(...this.resolveAction({
    // 						choice: 'priorityChargeMove',
    // 						pokemon: action.pokemon,
    // 						move: action.move,
    // 					}));
    // 				}
    // 				action.fractionalPriority = this.battle.runEvent('FractionalPriority', action.pokemon, null, action.move, 0);
    // 			} else if (['switch', 'instaswitch'].includes(action.choice)) {
    // 				if (typeof action.pokemon.switchFlag === 'string') {
    // 					action.sourceEffect = this.battle.dex.moves.get(action.pokemon.switchFlag as ID) as any;
    // 				}
    // 				action.pokemon.switchFlag = false;
    // 			}
    // 		}
    // 
    // 		const deferPriority = this.battle.gen === 7 && action.mega && action.mega !== 'done';
    // 		if (action.move) {
    // 			let target = null;
    // 			action.move = this.battle.dex.getActiveMove(action.move);
    // 
    // 			if (!action.targetLoc) {
    // 				target = this.battle.getRandomTarget(action.pokemon, action.move);
    // 				// TODO: what actually happens here?
    // 				if (target) action.targetLoc = action.pokemon.getLocOf(target);
    // 			}
    // 			action.originalTarget = action.pokemon.getAtLoc(action.targetLoc);
    // 		}
    // 		if (!deferPriority) this.battle.getActionSpeed(action);
    // 		return actions as any;
    // 	}
    //
    pub fn resolve_action(&self, action: &mut Action, mid_turn: bool) -> Vec<Action> {
        let mut actions = Vec::new();

        // Set order based on choice type
        match action {
            Action::Move(m) => {
                // Set order if not already set
                if m.order == 0 {
                    m.order = match m.choice {
                        MoveActionType::Move => 200,
                        MoveActionType::BeforeTurnMove => 5,
                        MoveActionType::PriorityChargeMove => 107,
                    };
                }

                if !mid_turn {
                    // Add mega evolution action if needed
                    if m.mega {
                        actions.push(Action::Pokemon(PokemonAction {
                            choice: PokemonActionType::MegaEvo,
                            order: 104,
                            priority: 0,
                            speed: m.speed,
                            pokemon_index: m.pokemon_index,
                            side_index: m.side_index,
                            event: None,
                        }));
                    }

                    // Add terastallize action if needed
                    if m.terastallize.is_some() {
                        actions.push(Action::Pokemon(PokemonAction {
                            choice: PokemonActionType::Terastallize,
                            order: 106,
                            priority: 0,
                            speed: m.speed,
                            pokemon_index: m.pokemon_index,
                            side_index: m.side_index,
                            event: None,
                        }));
                    }

                    // Add dynamax action if using max move
                    if m.max_move.is_some() {
                        actions.push(Action::Pokemon(PokemonAction {
                            choice: PokemonActionType::RunDynamax,
                            order: 105,
                            priority: 0,
                            speed: m.speed,
                            pokemon_index: m.pokemon_index,
                            side_index: m.side_index,
                            event: None,
                        }));
                    }
                }
            }
            Action::Switch(s) => {
                if s.order == 0 {
                    s.order = match s.choice {
                        SwitchActionType::Switch => 103,
                        SwitchActionType::InstaSwitch => 3,
                        SwitchActionType::RevivalBlessing => 6,
                    };
                }
            }
            Action::Team(_) => {
                // Team actions have order 1
            }
            Action::Field(_) => {
                // Field actions keep their order
            }
            Action::Pokemon(p) => {
                if p.order == 0 {
                    p.order = match p.choice {
                        PokemonActionType::Start => 2,
                        PokemonActionType::BeforeTurn => 4,
                        PokemonActionType::RunSwitch => 101,
                        PokemonActionType::MegaEvo => 104,
                        PokemonActionType::MegaEvoX => 104,
                        PokemonActionType::MegaEvoY => 104,
                        PokemonActionType::RunDynamax => 105,
                        PokemonActionType::Terastallize => 106,
                        PokemonActionType::Shift | PokemonActionType::Event => 200,
                        PokemonActionType::Residual => 300,
                    };
                }
            }
        }

        // Add the main action
        actions.push(action.clone());

        actions
    }

    /// Get the order value for a choice type
    pub fn get_order_for_choice(choice: &str) -> i32 {
        match choice {
            "team" => 1,
            "start" => 2,
            "instaswitch" => 3,
            "beforeTurn" => 4,
            "beforeTurnMove" => 5,
            "revivalblessing" => 6,
            "runSwitch" => 101,
            "switch" => 103,
            "megaEvo" | "megaEvoX" | "megaEvoY" => 104,
            "runDynamax" => 105,
            "terastallize" => 106,
            "priorityChargeMove" => 107,
            "shift" | "move" => 200,
            "residual" => 300,
            _ => 200,
        }
    }

    /// Get an iterator over (index, action) tuples
    /// Equivalent to JavaScript array.entries()
    /// TypeScript: for (const [i, action] of battle.queue.list.entries())
    pub fn entries(&self) -> impl Iterator<Item = (usize, &Action)> {
        self.list.iter().enumerate()
    }

    /// Remove count entries starting at index
    /// Equivalent to JavaScript array.splice(index, count)
    /// TypeScript: battle.queue.list.splice(index, count)
    pub fn splice(&mut self, index: usize, count: usize) {
        if index >= self.list.len() {
            return;
        }
        let end = std::cmp::min(index + count, self.list.len());
        self.list.drain(index..end);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_basic_operations() {
        let mut queue = BattleQueue::new();
        assert!(queue.is_empty());

        let action = Action::Field(FieldAction {
            choice: FieldActionType::Start,
            priority: 0,
        });
        queue.push(action);
        assert_eq!(queue.len(), 1);

        let popped = queue.shift();
        assert!(popped.is_some());
        assert!(queue.is_empty());
    }

    #[test]
    fn test_queue_sorting() {
        let mut queue = BattleQueue::new();

        // Add actions with different priorities and speeds
        queue.push(Action::Move(MoveAction {
            choice: MoveActionType::Move,
            order: 200,
            priority: 0,
            fractional_priority: 0.0,
            speed: 50,
            pokemon_index: 0,
            side_index: 0,
            target_loc: 0,
            move_id: ID::new("tackle"),
            mega: false,
            zmove: None,
            max_move: None,
            terastallize: None,
            move_priority_modified: None,
        }));

        queue.push(Action::Move(MoveAction {
            choice: MoveActionType::Move,
            order: 200,
            priority: 1, // Higher priority
            fractional_priority: 0.0,
            speed: 30,
            pokemon_index: 1,
            side_index: 1,
            target_loc: 0,
            move_id: ID::new("quickattack"),
            mega: false,
            zmove: None,
            max_move: None,
            terastallize: None,
            move_priority_modified: None,
        }));

        queue.push(Action::Move(MoveAction {
            choice: MoveActionType::Move,
            order: 200,
            priority: 0,
            fractional_priority: 0.0,
            speed: 100, // Highest speed
            pokemon_index: 2,
            side_index: 0,
            target_loc: 0,
            move_id: ID::new("thunderbolt"),
            mega: false,
            zmove: None,
            max_move: None,
            terastallize: None,
            move_priority_modified: None,
        }));

        queue.sort();

        // Quick Attack (priority 1) should be first
        let first = queue.shift().unwrap();
        assert!(matches!(first, Action::Move(m) if m.move_id.as_str() == "quickattack"));

        // Thunderbolt (speed 100) should be second
        let second = queue.shift().unwrap();
        assert!(matches!(second, Action::Move(m) if m.move_id.as_str() == "thunderbolt"));

        // Tackle (speed 50) should be last
        let third = queue.shift().unwrap();
        assert!(matches!(third, Action::Move(m) if m.move_id.as_str() == "tackle"));
    }
}
