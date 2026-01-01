const {Battle} = require('./../pokemon-showdown-ts/dist/sim/battle');
const {PRNG} = require('./../pokemon-showdown-ts/dist/sim/prng');
const fs = require('fs');

const battle = new Battle({formatid: 'gen9randombattle'});
battle.prng = new PRNG([0, 0, 0, 1]);

// Wrap PRNG to count calls
let totalPrngCalls = 0;
const originalNext = battle.prng.rng.next.bind(battle.prng.rng);
battle.prng.rng.next = function() {
    totalPrngCalls++;
    const result = originalNext();
    if (totalPrngCalls >= 137 && totalPrngCalls <= 150) {
        console.log(`  PRNG call #${totalPrngCalls}: ${result}`);
    }
    return result;
};

// Wrap spreadMoveHit to log secondaries check
const BattleActions = require('./../pokemon-showdown-ts/dist/sim/battle-actions').BattleActions;
const originalSpreadMoveHit = BattleActions.prototype.spreadMoveHit;
BattleActions.prototype.spreadMoveHit = function(targets, pokemon, move, moveData, isSecondary, isSelf) {
    if (move.id === 'zenheadbutt' && battle.turn === 30) {
        console.log(`[JS SPREAD_MOVE_HIT] Zen Headbutt T30: moveData.secondaries = ${JSON.stringify(moveData.secondaries)}`);
        console.log(`[JS SPREAD_MOVE_HIT] Zen Headbutt T30: move.secondaries = ${JSON.stringify(move.secondaries)}`);
        console.log(`[JS SPREAD_MOVE_HIT] Zen Headbutt T30: targets before secondaries check = ${targets.map((t, i) => t ? `${i}:${t.name}` : `${i}:false`)}`);
    }
    return originalSpreadMoveHit.call(this, targets, pokemon, move, moveData, isSecondary, isSelf);
};

// Load teams from JSON file
const teams = JSON.parse(fs.readFileSync('teams-js.json', 'utf8'));

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

// Run until PRNG reaches 133
while (totalPrngCalls < 133) {
    battle.makeChoices('default', 'default');
}

// Turn 1: PRNG 133->133 (switch)
battle.makeChoices('default', 'default');

// Turn 2: PRNG 133->137 (Zacian takes damage)
battle.makeChoices('default', 'default');

console.log('========== After PRNG 137 ==========');
console.log(`Battle turn: ${battle.turn}`);
console.log(`PRNG calls: ${totalPrngCalls}`);
console.log(`P1 active: ${battle.sides[0].active.map(p => p ? `${p.name} (${p.hp}/${p.maxhp})` : 'null').join(', ')}`);
console.log(`P2 active: ${battle.sides[1].active.map(p => p ? `${p.name} (${p.hp}/${p.maxhp})` : 'null').join(', ')}`);

// Turn 3 (this should diverge)
const prngBefore = totalPrngCalls;
const logBefore = battle.log.length;
console.log('\n========== makeChoices #3 (expected divergence) ==========');
battle.makeChoices('default', 'default');
const prngAfter = totalPrngCalls;
console.log(`PRNG: ${prngBefore} -> ${prngAfter} (${prngAfter - prngBefore} calls)`);
console.log(`Battle turn: ${battle.turn}`);
console.log(`P1 active: ${battle.sides[0].active.map(p => p ? `${p.name} (${p.hp}/${p.maxhp})` : 'null').join(', ')}`);
console.log(`P2 active: ${battle.sides[1].active.map(p => p ? `${p.name} (${p.hp}/${p.maxhp})` : 'null').join(', ')}`);

console.log('\n========== Battle Log ==========');
const newLog = battle.log.slice(logBefore);
console.log(newLog.join('\n'));
