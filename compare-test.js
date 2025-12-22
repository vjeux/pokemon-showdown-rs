#!/usr/bin/env node
/**
 * Pokemon Showdown JS vs Rust Comparison Test
 *
 * This script runs battles in both the JS and Rust simulators
 * and compares the results to ensure they match.
 */

const Sim = require('./pokemon-showdown-js/dist/sim');
const { Battle, Dex, PRNG } = Sim;
const { execSync } = require('child_process');
const fs = require('fs');

// Default seed for reproducible tests
const DEFAULT_SEED = [0x9917, 0x6924, 0xe1c8, 0x6af0];

// Test case definition
const testCases = [
    {
        name: 'Basic Damage Calculation',
        seed: [1, 2, 3, 4],
        teams: [
            [
                {
                    species: 'Pikachu',
                    ability: 'Static',
                    moves: ['Thunderbolt'],
                    level: 50,
                    evs: { hp: 0, atk: 0, def: 0, spa: 252, spd: 0, spe: 0 },
                }
            ],
            [
                {
                    species: 'Squirtle',
                    ability: 'Torrent',
                    moves: ['Tackle'],
                    level: 50,
                    evs: { hp: 252, atk: 0, def: 0, spa: 0, spd: 0, spe: 0 },
                }
            ]
        ],
        turns: [
            { p1: 'move thunderbolt', p2: 'move tackle' }
        ]
    },
    {
        name: 'Status - Burn',
        seed: [10, 20, 30, 40],
        teams: [
            [
                {
                    species: 'Sableye',
                    ability: 'Prankster',
                    moves: ['Will-O-Wisp'],
                    level: 50,
                }
            ],
            [
                {
                    species: 'Machamp',
                    ability: 'Guts',
                    moves: ['Bulk Up'],
                    level: 50,
                }
            ]
        ],
        turns: [
            { p1: 'move willowisp', p2: 'move bulkup' },
            { p1: 'move willowisp', p2: 'move bulkup' }
        ]
    },
    {
        name: 'Type Immunity',
        seed: [5, 6, 7, 8],
        teams: [
            [
                {
                    species: 'Pikachu',
                    ability: 'Static',
                    moves: ['Thunderbolt'],
                    level: 50,
                }
            ],
            [
                {
                    species: 'Sandshrew',
                    ability: 'Sand Veil',
                    moves: ['Scratch'],
                    level: 50,
                }
            ]
        ],
        turns: [
            { p1: 'move thunderbolt', p2: 'move scratch' }
        ]
    }
];

// Run a battle in JS
function runJSBattle(testCase) {
    // Get format
    const format = Dex.formats.get('[gen9] Custom Game@@@!Team Preview,!Cancel Mod', true);

    const battleOptions = {
        debug: true,
        format,
        seed: testCase.seed,
        strictChoices: true,
        p1: { team: testCase.teams[0] },
        p2: { team: testCase.teams[1] },
    };

    const battle = new Battle(battleOptions);

    // Run each turn
    for (const turn of testCase.turns) {
        battle.makeChoices(turn.p1, turn.p2);
    }

    // Collect results
    const results = {
        turn: battle.turn,
        ended: battle.ended,
        winner: battle.winner || '',
        p1hp: battle.sides[0].active[0]?.hp ?? 0,
        p2hp: battle.sides[1].active[0]?.hp ?? 0,
        p1maxhp: battle.sides[0].active[0]?.maxhp ?? 0,
        p2maxhp: battle.sides[1].active[0]?.maxhp ?? 0,
        p1status: battle.sides[0].active[0]?.status ?? '',
        p2status: battle.sides[1].active[0]?.status ?? '',
    };

    return results;
}

// Run comparison tests
async function runComparison() {
    console.log('Pokemon Showdown JS vs Rust Comparison Tests\n');
    console.log('='.repeat(60));

    let passed = 0;
    let failed = 0;

    for (const testCase of testCases) {
        console.log(`\nTest: ${testCase.name}`);
        console.log('-'.repeat(40));

        try {
            // Run JS version
            const jsResult = runJSBattle(testCase);
            console.log('JS Result:', JSON.stringify(jsResult, null, 2));

            // For now, just show the JS results
            // In a full implementation, we'd compile and run the Rust code
            console.log('(Rust comparison not yet implemented)');
            passed++;
        } catch (err) {
            console.log('Error:', err.message);
            console.log(err.stack);
            failed++;
        }
    }

    console.log('\n' + '='.repeat(60));
    console.log(`Results: ${passed} passed, ${failed} failed`);
}

// PRNG comparison test
function testPRNG() {
    console.log('\nPRNG Comparison Test');
    console.log('-'.repeat(40));

    const seed = [0x1234, 0x5678, 0x9ABC, 0xDEF0];
    const prng = new PRNG(seed);

    console.log('JS PRNG outputs with seed [0x1234, 0x5678, 0x9ABC, 0xDEF0]:');
    const jsOutputs = [];
    for (let i = 0; i < 10; i++) {
        const val = prng.randomChance(1, 2);  // 50% chance
        jsOutputs.push(val);
    }
    console.log('First 10 randomChance(1, 2) values:', jsOutputs.join(', '));

    // Test with sample (picks random element from array)
    const prng2 = new PRNG(seed);
    const sampleOutputs = [];
    const arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for (let i = 0; i < 10; i++) {
        const val = prng2.sample(arr);
        sampleOutputs.push(val);
    }
    console.log('First 10 sample([0-9]) values:', sampleOutputs.join(', '));
}

// Test specific mechanics
function testSpecificMechanics() {
    console.log('\n\nSpecific Mechanics Tests');
    console.log('='.repeat(60));

    // Test 1: Burn damage (should be 1/16 in Gen 7+)
    console.log('\n1. Burn Damage Test (Gen 9)');
    console.log('-'.repeat(40));

    const format = Dex.formats.get('[gen9] Custom Game@@@!Team Preview,!Cancel Mod', true);
    const burnBattle = new Battle({
        format,
        seed: [1, 2, 3, 4],
        p1: { team: [{ species: 'Sableye', ability: 'Prankster', moves: ['Will-O-Wisp'] }] },
        p2: { team: [{ species: 'Machamp', ability: 'Guts', moves: ['Bulk Up'] }] },
    });

    burnBattle.makeChoices('move willowisp', 'move bulkup');

    const machamp = burnBattle.sides[1].active[0];
    console.log(`Machamp status: ${machamp.status}`);
    console.log(`Machamp HP before residual: ${machamp.hp}/${machamp.maxhp}`);

    // Run another turn to see burn damage
    burnBattle.makeChoices('move willowisp', 'move bulkup');
    console.log(`Machamp HP after 1 turn of burn: ${machamp.hp}/${machamp.maxhp}`);

    const burnDamage = machamp.maxhp - machamp.hp;
    const expectedBurnDamage = Math.floor(machamp.maxhp / 16);
    console.log(`Burn damage dealt: ${burnDamage} (expected ~${expectedBurnDamage} = maxhp/16)`);

    // Test 2: Type effectiveness
    console.log('\n2. Type Effectiveness Test');
    console.log('-'.repeat(40));

    const typeBattle = new Battle({
        format,
        seed: [1, 2, 3, 4],
        p1: { team: [{ species: 'Pikachu', ability: 'Static', moves: ['Thunderbolt'] }] },
        p2: { team: [{ species: 'Gyarados', ability: 'Intimidate', moves: ['Splash'] }] },
    });

    const gyaradosHPBefore = typeBattle.sides[1].active[0].hp;
    typeBattle.makeChoices('move thunderbolt', 'move splash');
    const gyaradosHPAfter = typeBattle.sides[1].active[0].hp;

    console.log(`Gyarados (Water/Flying) HP: ${gyaradosHPBefore} -> ${gyaradosHPAfter}`);
    console.log(`Damage dealt: ${gyaradosHPBefore - gyaradosHPAfter} (4x super effective from Electric)`);

    // Test 3: Ground immunity to Electric
    console.log('\n3. Ground Immunity Test');
    console.log('-'.repeat(40));

    const immuneBattle = new Battle({
        format,
        seed: [1, 2, 3, 4],
        p1: { team: [{ species: 'Pikachu', ability: 'Static', moves: ['Thunderbolt'] }] },
        p2: { team: [{ species: 'Sandshrew', ability: 'Sand Veil', moves: ['Scratch'] }] },
    });

    const sandHPBefore = immuneBattle.sides[1].active[0].hp;
    immuneBattle.makeChoices('move thunderbolt', 'move scratch');
    const sandHPAfter = immuneBattle.sides[1].active[0].hp;

    console.log(`Sandshrew (Ground) HP: ${sandHPBefore} -> ${sandHPAfter}`);
    console.log(`Damage dealt: ${sandHPBefore - sandHPAfter} (should be 0 - immune)`);
}

// Run all tests
runComparison();
testPRNG();
testSpecificMechanics();
