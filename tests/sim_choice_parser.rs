//! Test choice parser
//! Line-by-line port of pokemon-showdown-ts/test/sim/choice-parser.js

mod common;

use pokemon_showdown::SideID;

// JavaScript: describe('Choice parser', () => {
// JavaScript:     afterEach(() => battle.destroy());

// JavaScript:     describe('Team Preview requests', () => {
// JavaScript:         it('should accept only `team` choices', () => {
#[test]
fn test_team_preview_should_accept_only_team_choices() {
    // JavaScript:             battle = common.createBattle({ preview: true }, [
    // JavaScript:                 [{ species: "Mew", ability: 'synchronize', moves: ['recover'] }],
    // JavaScript:                 [{ species: "Rhydon", ability: 'prankster', moves: ['splash'] }],
    // JavaScript:             ]);
    let mut battle = common::create_battle(
        common::CreateBattleOptions {
            preview: true,
            ..Default::default()
        },
        [
            vec![pokemon!(
                species: "Mew",
                ability: "synchronize",
                moves: ["recover"]
            )],
            vec![pokemon!(
                species: "Rhydon",
                ability: "prankster",
                moves: ["splash"]
            )],
        ],
    );

    // JavaScript:             const validChoice = 'team 1';
    let valid_choice = "team 1";

    // JavaScript:             assert(battle.choose('p1', validChoice));
    assert!(battle.choose(SideID::P1, valid_choice).is_ok());

    // JavaScript:             battle.p1.clearChoice();
    // TODO: Need to implement simple clear_choice API

    // JavaScript:             assert(battle.choose('p2', validChoice));
    assert!(battle.choose(SideID::P2, valid_choice).is_ok());

    // JavaScript:             battle.p1.clearChoice();
    // TODO: Need to implement simple clear_choice API

    // JavaScript:             const badChoices = ['move 1', 'move 2 mega', 'switch 1', 'pass', 'shift'];
    let bad_choices = vec!["move 1", "move 2 mega", "switch 1", "pass", "shift"];

    // JavaScript:             for (const badChoice of badChoices) {
    // JavaScript:                 assert.throws(() => battle.choose('p1', badChoice));
    // JavaScript:             }
    for bad_choice in bad_choices {
        assert!(battle.choose(SideID::P1, bad_choice).is_err(), "Expected '{}' to be rejected", bad_choice);
    }
}

// JavaScript:         it('should reject non-numerical choice details', () => {
#[test]
fn test_team_preview_should_reject_non_numerical_choice_details() {
    // JavaScript:             battle = common.createBattle({ preview: true }, [
    // JavaScript:                 [{ species: "Mew", ability: 'synchronize', moves: ['recover'] }],
    // JavaScript:                 [{ species: "Rhydon", ability: 'prankster', moves: ['splash'] }],
    // JavaScript:             ]);
    let mut battle = common::create_battle(
        common::CreateBattleOptions {
            preview: true,
            ..Default::default()
        },
        [
            vec![pokemon!(
                species: "Mew",
                ability: "synchronize",
                moves: ["recover"]
            )],
            vec![pokemon!(
                species: "Rhydon",
                ability: "prankster",
                moves: ["splash"]
            )],
        ],
    );

    // JavaScript:             for (const side of battle.sides) {
    // JavaScript:                 assert.throws(() => battle.choose(side.id, 'team Rhydon'));
    // JavaScript:                 assert.throws(() => battle.choose(side.id, 'team Mew'));
    // JavaScript:                 assert.throws(() => battle.choose(side.id, 'team first'));
    // JavaScript:             }
    for side_id in [SideID::P1, SideID::P2] {
        assert!(battle.choose(side_id, "team Rhydon").is_err(), "Expected 'team Rhydon' to be rejected");
        assert!(battle.choose(side_id, "team Mew").is_err(), "Expected 'team Mew' to be rejected");
        assert!(battle.choose(side_id, "team first").is_err(), "Expected 'team first' to be rejected");
    }
}

// JavaScript:         it('should reject zero-based choice details', () => {
#[test]
fn test_team_preview_should_reject_zero_based_choice_details() {
    // JavaScript:             battle = common.createBattle({ preview: true }, [
    // JavaScript:                 [{ species: "Mew", ability: 'synchronize', moves: ['recover'] }],
    // JavaScript:                 [{ species: "Rhydon", ability: 'prankster', moves: ['splash'] }],
    // JavaScript:             ]);
    let mut battle = common::create_battle(
        common::CreateBattleOptions {
            preview: true,
            ..Default::default()
        },
        [
            vec![pokemon!(
                species: "Mew",
                ability: "synchronize",
                moves: ["recover"]
            )],
            vec![pokemon!(
                species: "Rhydon",
                ability: "prankster",
                moves: ["splash"]
            )],
        ],
    );

    // JavaScript:             for (const side of battle.sides) {
    // JavaScript:                 assert.throws(
    // JavaScript:                     () => battle.choose(side.id, 'team 0'),
    // JavaScript:                     /\[Invalid choice\] Can't choose for Team Preview:/i,
    // JavaScript:                     `Input should have been rejected`
    // JavaScript:                 );
    // JavaScript:             }
    for side_id in [SideID::P1, SideID::P2] {
        let result = battle.choose(side_id, "team 0");
        assert!(result.is_err(), "Input should have been rejected");
        if let Err(msg) = result {
            assert!(msg.to_lowercase().contains("[invalid choice]"), "Error should contain '[Invalid choice]': {}", msg);
            assert!(msg.to_lowercase().contains("team preview"), "Error should mention 'Team Preview': {}", msg);
        }
    }
}

// JavaScript:     describe('Switch requests', () => {
// JavaScript:         describe('Generic', () => {
// JavaScript:             it('should reject non-numerical input for `switch` choices', () => {
#[test]
fn test_switch_generic_should_reject_non_numerical_input() {
    // JavaScript:                 battle = common.createBattle();
    let mut battle = common::create_battle(
        common::CreateBattleOptions::default(),
        [
            // JavaScript:                 battle.setPlayer('p1', { team: [
            // JavaScript:                     { species: "Mew", ability: 'synchronize', moves: ['lunardance'] },
            // JavaScript:                     { species: "Bulbasaur", ability: 'overgrow', moves: ['tackle', 'growl'] },
            // JavaScript:                 ] });
            vec![
                pokemon!(species: "Mew", ability: "synchronize", moves: ["lunardance"]),
                pokemon!(species: "Bulbasaur", ability: "overgrow", moves: ["tackle", "growl"]),
            ],
            // JavaScript:                 battle.setPlayer('p2', { team: [{ species: "Rhydon", ability: 'prankster', moves: ['splash'] }] });
            vec![
                pokemon!(species: "Rhydon", ability: "prankster", moves: ["splash"]),
            ],
        ],
    );

    // JavaScript:                 battle.makeChoices('move lunardance', 'move splash');
    battle.make_choices("move lunardance", "move splash");

    // JavaScript:                 assert.throws(() => battle.choose('p1', 'switch first'));
    assert!(battle.choose(SideID::P1, "switch first").is_err(), "Expected 'switch first' to be rejected");

    // JavaScript:                 assert.throws(() => battle.choose('p1', 'switch second'));
    assert!(battle.choose(SideID::P1, "switch second").is_err(), "Expected 'switch second' to be rejected");
}

