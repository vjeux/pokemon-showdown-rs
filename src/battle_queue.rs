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

// Type modules
mod move_action;
mod switch_action;
mod team_action;
mod field_action;
mod pokemon_action;
mod action;
mod battle_queue_struct;

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
mod will_move_mut;
mod will_move_pokemon;
mod will_switch;
mod will_act;
mod sort;
mod prioritize_action;
mod iter;
mod iter_mut;
mod change_action;
mod insert_in_order;
mod insert_choice;
mod add_choice;
mod add_choice_raw;
mod debug;
mod entries;
mod entries_mut;
mod find;
mod splice;
mod resolve_action;

// Re-export types from submodules
pub use move_action::{MoveAction, MoveActionType};
pub use switch_action::{SwitchAction, SwitchActionType};
pub use team_action::{TeamAction, TeamActionType};
pub use field_action::{FieldAction, FieldActionType};
pub use pokemon_action::{PokemonAction, PokemonActionType};
pub use action::Action;
pub use battle_queue_struct::BattleQueue;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle::Battle;
    use crate::dex_data::ID;
    use crate::Dex;

    #[test]
    fn test_queue_basic_operations() {
        let mut queue = BattleQueue::new();
        assert!(queue.is_empty());

        let action = Action::Field(FieldAction {
            choice: FieldActionType::Start,
            priority: 0,
            sub_order: 0,
            effect_order: 0,
        });
        queue.push(action);
        assert_eq!(queue.len(), 1);

        let popped = queue.shift();
        assert!(popped.is_some());
        assert!(queue.is_empty());
    }

    #[test]
    fn test_queue_sorting() {
        // Create a minimal battle for sorting
        let dex = Dex::new();
        let mut battle = Battle::new("gen9randombattle", dex);

        let mut queue = BattleQueue::new();

        // Add actions with different priorities and speeds
        queue.push(Action::Move(MoveAction {
            choice: MoveActionType::Move,
            order: 200,
            priority: 0,
            fractional_priority: 0.0,
            speed: 50.0,
            sub_order: 0,
            effect_order: 0,
            pokemon_index: 0,
            side_index: 0,
            target_loc: 0,
            original_target: None,
            move_id: ID::new("tackle"),
            mega: false,
            zmove: None,
            max_move: None,
            source_effect: None,
            terastallize: None,
            move_priority_modified: None,
            prankster_boosted: false,
        }));

        queue.push(Action::Move(MoveAction {
            choice: MoveActionType::Move,
            order: 200,
            priority: 1, // Higher priority
            fractional_priority: 0.0,
            speed: 30.0,
            sub_order: 0,
            effect_order: 0,
            pokemon_index: 1,
            side_index: 1,
            target_loc: 0,
            original_target: None,
            move_id: ID::new("quickattack"),
            mega: false,
            zmove: None,
            max_move: None,
            source_effect: None,
            terastallize: None,
            move_priority_modified: None,
            prankster_boosted: false,
        }));

        queue.push(Action::Move(MoveAction {
            choice: MoveActionType::Move,
            order: 200,
            priority: 0,
            fractional_priority: 0.0,
            speed: 100.0, // Highest speed
            sub_order: 0,
            effect_order: 0,
            pokemon_index: 2,
            side_index: 0,
            target_loc: 0,
            original_target: None,
            move_id: ID::new("thunderbolt"),
            mega: false,
            zmove: None,
            max_move: None,
            source_effect: None,
            terastallize: None,
            move_priority_modified: None,
            prankster_boosted: false,
        }));

        // Sort using the battle reference
        queue.sort(&mut battle);

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
