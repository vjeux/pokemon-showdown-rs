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
    pub order: u32,
    /// Priority of the action (higher = earlier)
    pub priority: i8,
    /// Fractional priority (higher = earlier)
    pub fractional_priority: f64,
    /// Speed of pokemon using move (higher = earlier if priority tie)
    pub speed: u32,
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
    pub order: u32,
    /// Priority of the action
    pub priority: i8,
    /// Speed of pokemon switching
    pub speed: u32,
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
    /// Priority
    pub priority: i8,
    /// Speed
    pub speed: u32,
    /// Pokemon index
    pub pokemon_index: usize,
    /// Side index
    pub side_index: usize,
    /// Event name (for event actions)
    pub event: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PokemonActionType {
    MegaEvo,
    MegaEvoX,
    MegaEvoY,
    Shift,
    RunSwitch,
    Event,
    RunDynamax,
    Terastallize,
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
    pub fn order(&self) -> u32 {
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
            Action::Pokemon(a) => match a.choice {
                PokemonActionType::RunSwitch => 101,
                PokemonActionType::MegaEvo |
                PokemonActionType::MegaEvoX |
                PokemonActionType::MegaEvoY => 104,
                PokemonActionType::RunDynamax => 105,
                PokemonActionType::Terastallize => 106,
                PokemonActionType::Shift => 200,
                PokemonActionType::Event => 200,
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
    pub fn speed(&self) -> u32 {
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
    pub fn shift(&mut self) -> Option<Action> {
        if self.list.is_empty() {
            None
        } else {
            Some(self.list.remove(0))
        }
    }

    /// Peek at the next action without removing
    pub fn peek(&self) -> Option<&Action> {
        self.list.first()
    }

    /// Peek at the last action
    pub fn peek_end(&self) -> Option<&Action> {
        self.list.last()
    }

    /// Push an action to the end
    pub fn push(&mut self, action: Action) {
        self.list.push(action);
    }

    /// Unshift an action to the front
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
    pub fn clear(&mut self) {
        self.list.clear();
    }

    /// Cancel all actions for a specific pokemon
    pub fn cancel_action(&mut self, side_index: usize, pokemon_index: usize) -> bool {
        let old_len = self.list.len();
        self.list.retain(|action| {
            !(action.side_index() == Some(side_index) && action.pokemon_index() == Some(pokemon_index))
        });
        self.list.len() != old_len
    }

    /// Cancel move action for a specific pokemon
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

    /// Check if a pokemon will switch this turn
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
    pub fn will_act(&self) -> bool {
        self.list.iter().any(|action| {
            matches!(action, Action::Move(_) | Action::Switch(_))
        })
    }

    /// Sort the queue by priority
    /// Order: order (lower first), priority (higher first), speed (higher first)
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
