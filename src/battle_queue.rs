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

use crate::dex_data::ID;
use serde::{Deserialize, Serialize};

// Function modules
mod new;
mod shift;
mod peek;
mod peek_end;
mod push;
mod unshift;
mod len;
mod is_empty;
mod clear;
mod cancel_action;
mod cancel_move;
mod will_move;
mod will_move_pokemon;
mod will_switch;
mod will_act;
mod insert_run_switch;
mod sort;
mod prioritize_action;
mod iter;
mod iter_mut;
mod will_act_full;
mod change_action;
mod insert_in_order;
mod add_choice;
mod add_choice_raw;
mod debug;
mod entries;
mod entries_mut;
mod find;
mod remove_where;
mod get_order_for_choice;
mod splice;

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
            Action::Pokemon(a) => {
                if a.order != 0 {
                    a.order
                } else {
                    match a.choice {
                        PokemonActionType::Start => 2,
                        PokemonActionType::BeforeTurn => 4,
                        PokemonActionType::RunSwitch => 101,
                        PokemonActionType::MegaEvo
                        | PokemonActionType::MegaEvoX
                        | PokemonActionType::MegaEvoY => 104,
                        PokemonActionType::RunDynamax => 105,
                        PokemonActionType::Terastallize => 106,
                        PokemonActionType::Shift => 200,
                        PokemonActionType::Event => 200,
                        PokemonActionType::Residual => 300,
                    }
                }
            }
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
