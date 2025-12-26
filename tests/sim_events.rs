//! Test battle events system
//! Port of pokemon-showdown-js/test/sim/events.js

mod common;

use pokemon_showdown::Battle;
use std::sync::{Arc, Mutex};

/// Test: Battle#onEvent - should allow the addition of one or more event handlers
#[test]
fn test_on_event_allows_multiple_handlers() {
    let mut battle = common::create_battle(
        common::CreateBattleOptions::default(),
        [
            vec![pokemon!(
                species: "Pidgeot",
                ability: "keeneye",
                moves: ["bulkup"]
            )],
            vec![pokemon!(
                species: "Talonflame",
                ability: "galewings",
                moves: ["peck"]
            )],
        ],
    );

    let event_count = Arc::new(Mutex::new(0));
    let event_count2 = Arc::new(Mutex::new(0));

    let count1 = event_count.clone();
    let count2 = event_count.clone();
    let count2_2 = event_count2.clone();

    // Add first Hit event handler
    battle.on_event("Hit", move |_ctx| {
        *count1.lock().unwrap() += 1;
        None
    });

    // Add second Hit event handler
    battle.on_event("Hit", move |_ctx| {
        *count2.lock().unwrap() += 1;
        *count2_2.lock().unwrap() += 1;
        None
    });

    // Add ModifyDamage event handler
    battle.on_event("ModifyDamage", move |_ctx| {
        Some(5)
    });

    // Transition out of team preview to Move state
    battle.start_battle();

    battle.make_choices("move bulkup", "move peck");

    // bulkup is used (counts as hit) + peck hits (counts as hit)
    // Each handler is called for each hit
    assert_eq!(*event_count.lock().unwrap(), 4);
    assert_eq!(*event_count2.lock().unwrap(), 2);

    // Check that ModifyDamage worked - damage should be exactly 5
    let p1_pokemon = &battle.sides[0].pokemon[0];
    let damage = p1_pokemon.maxhp - p1_pokemon.hp;
    assert_eq!(damage, 5);
}

/// Test: Battle#onEvent - should support and resolve priorities correctly
#[test]
fn test_on_event_priorities() {
    let mut battle = common::create_battle(
        common::CreateBattleOptions::default(),
        [
            vec![pokemon!(
                species: "Pidgeot",
                ability: "keeneye",
                moves: ["bulkup"]
            )],
            vec![pokemon!(
                species: "Talonflame",
                ability: "galewings",
                moves: ["peck"]
            )],
        ],
    );

    let event_count = Arc::new(Mutex::new(0));

    // Add 9 handlers with different priorities
    for i in 0..9 {
        let count = event_count.clone();
        let expected = i;
        battle.on_event_priority("ModifyDamage", -(i as i32), move |_ctx| {
            let current = *count.lock().unwrap();
            assert_eq!(current, expected, "Handler {} called out of order", i);
            *count.lock().unwrap() += 1;
            None
        });
    }

    // Transition out of team preview to Move state
    battle.start_battle();

    battle.make_choices("move bulkup", "move peck");
    assert_eq!(*event_count.lock().unwrap(), 9);
}

/// Test: Battle#onEvent - should panic if event_id is empty
#[test]
#[should_panic(expected = "Event handlers must have an event to listen to")]
fn test_on_event_requires_callback() {
    let mut battle = common::create_battle(
        common::CreateBattleOptions::default(),
        [
            vec![pokemon!(
                species: "Pidgeot",
                ability: "keeneye",
                moves: ["bulkup"]
            )],
            vec![pokemon!(
                species: "Talonflame",
                ability: "galewings",
                moves: ["peck"]
            )],
        ],
    );

    // This should panic because event_id is empty
    // In Rust, we can't call on_event without a callback (type system prevents it),
    // so instead we test that empty event_id causes a panic
    battle.on_event("", |_ctx| None);
}
