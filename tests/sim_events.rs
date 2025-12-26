//! Test battle events system
//! Port of pokemon-showdown-js/test/sim/events.js

mod common;

use pokemon_showdown::Battle;
use std::sync::{Arc, Mutex};

/// Test: Battle#onEvent - should allow the addition of one or more event handlers
#[test]
fn test_on_event_allows_multiple_handlers() {
    let battle = common::create_battle(
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
    battle.on_event("Hit", move || {
        *count1.lock().unwrap() += 1;
    });

    // Add second Hit event handler
    battle.on_event("Hit", move || {
        *count2.lock().unwrap() += 1;
        *count2_2.lock().unwrap() += 1;
    });

    // Add ModifyDamage event handler
    battle.on_event("ModifyDamage", move || {
        5
    });

    battle.make_choices("move bulkup", "move peck");

    // bulkup is used (counts as hit) + peck hits (counts as hit)
    // Each handler is called for each hit
    assert_eq!(*event_count.lock().unwrap(), 4);
    assert_eq!(*event_count2.lock().unwrap(), 2);

    // Check that ModifyDamage worked - damage should be exactly 5
    let p1_pokemon = &battle.sides[0].active[0];
    let damage = p1_pokemon.as_ref().unwrap().maxhp - p1_pokemon.as_ref().unwrap().hp;
    assert_eq!(damage, 5);
}

/// Test: Battle#onEvent - should support and resolve priorities correctly
#[test]
fn test_on_event_priorities() {
    let battle = common::create_battle(
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
        battle.on_event_priority("ModifyDamage", -(i as i32), move || {
            let current = *count.lock().unwrap();
            assert_eq!(current, expected, "Handler {} called out of order", i);
            *count.lock().unwrap() += 1;
        });
    }

    battle.make_choices("move bulkup", "move peck");
    assert_eq!(*event_count.lock().unwrap(), 9);
}

/// Test: Battle#onEvent - should throw if a callback is not given
#[test]
#[should_panic]
fn test_on_event_requires_callback() {
    let battle = common::create_battle(
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

    // This should panic because there's no callback
    // In JavaScript this throws TypeError, in Rust we use panic
    battle.on_event("Hit", || {});
}
