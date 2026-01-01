#!/usr/bin/env node

const {Dex} = require('./../pokemon-showdown-ts/dist/sim');
const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const fs = require('fs');

const seedValue = 1;
const teams = JSON.parse(fs.readFileSync('teams-js.json', 'utf8'));

const battle = new Battle({
    formatid: 'gen9randombattle',
    seed: [0, 0, 0, seedValue],
});

battle.setPlayer('p1', {
    name: 'Player 1',
    team: teams.p1.map(p => ({
        name: p.name,
        species: p.species,
        level: p.level,
        ability: p.ability,
        item: p.item,
        nature: p.nature,
        gender: p.gender,
        moves: p.moves,
        evs: p.evs,
        ivs: p.ivs,
    })),
});

battle.setPlayer('p2', {
    name: 'Player 2',
    team: teams.p2.map(p => ({
        name: p.name,
        species: p.species,
        level: p.level,
        ability: p.ability,
        item: p.item,
        nature: p.nature,
        gender: p.gender,
        moves: p.moves,
        evs: p.evs,
        ivs: p.ivs,
    })),
});

// Patch randomizer to log damage calculation
const BattleActions = require('./../pokemon-showdown-ts/dist/sim/battle-actions').BattleActions;
const originalRandomizer = battle.randomizer.bind(battle);
battle.randomizer = function(baseDamage) {
    const roll = this.random(16);
    const multiplier = 100 - roll;
    const product = baseDamage * multiplier;
    const inner = this.trunc(product);
    const division = inner / 100;
    const result = this.trunc(division);

    console.log(`[JS RANDOMIZER] base_damage=${baseDamage}, roll=${roll}, multiplier=${multiplier}, product=${product}, inner=${inner}, division=${division}, result=${result}`);

    return result;
};

// Patch PRNG to track calls
let prngCallCount = 0;
if (battle.prng && battle.prng.rng && battle.prng.rng.next) {
    const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
    battle.prng.rng.next = function() {
        const value = originalNext();
        prngCallCount++;
        if (prngCallCount >= 69 && prngCallCount <= 77) {
            console.log(`[JS PRNG] Call #${prngCallCount} -> ${value}`);
        }
        return value;
    };
}

// Run to turn 21
for (let turn = 1; turn <= 21; turn++) {
    if (turn === 21) {
        console.log(`\n========== TURN ${turn} ==========`);

        // Log Cinderace HP before turn 21
        const p2a = battle.sides[1].active[0];
        if (p2a && p2a.name.includes('Cinderace')) {
            console.log(`[BEFORE TURN 21] Cinderace HP: ${p2a.hp}/${p2a.maxhp}`);
        }

        const logBefore = battle.log.length;
        battle.makeChoices('default', 'default');
        for (let i = logBefore; i < battle.log.length; i++) {
            console.log(battle.log[i]);
        }
    } else {
        battle.makeChoices('default', 'default');
    }

    if (battle.ended) break;
}
