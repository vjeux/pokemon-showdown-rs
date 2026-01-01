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
    if (totalPrngCalls >= 117 && totalPrngCalls <= 123) {
        console.log(`  PRNG call #${totalPrngCalls}: ${result}`);
    }
    return result;
};

// Wrap randomizer to log
const originalRandomizer = battle.randomizer.bind(battle);
battle.randomizer = function(baseDamage) {
    const roll = this.random(16);
    const multiplier = 100 - roll;
    const result = originalRandomizer(baseDamage);
    console.log(`[JS RANDOMIZER] baseDamage=${baseDamage}, roll=${roll}, multiplier=${multiplier}, result=${result}`);
    return result;
};

// Wrap modifyDamage to log
const BattleActions = require('./../pokemon-showdown-ts/dist/sim/battle-actions').BattleActions;
const originalModifyDamage = BattleActions.prototype.modifyDamage;
BattleActions.prototype.modifyDamage = function(baseDamage, pokemon, target, move, suppressMessages) {
    console.log(`[JS MODIFY_DAMAGE START] baseDamage=${baseDamage}, move=${move.id}, spreadHit=${move.spreadHit}`);
    const afterPlus2 = baseDamage + 2;
    console.log(`[JS MODIFY_DAMAGE] After +=2: ${afterPlus2}`);
    const result = originalModifyDamage.call(this, baseDamage, pokemon, target, move, suppressMessages);
    console.log(`[JS MODIFY_DAMAGE END] result=${result}`);
    return result;
};

// Wrap getDamage to log basePowerCallback
const originalGetDamage = BattleActions.prototype.getDamage;
BattleActions.prototype.getDamage = function(source, target, move, suppressMessages) {
    console.log(`[JS GET_DAMAGE START] move ${move.id}, basePower=${move.basePower}, hasBasePowerCallback=${!!move.basePowerCallback}, hasDamageCallback=${!!move.damageCallback}, isMax=${move.isMax}`);
    console.log(`[JS GET_DAMAGE] source.volatiles.dynamax=${!!source.volatiles['dynamax']}, move.baseMove=${move.baseMove}`);

    // Log basePower before and after BasePower event
    const originalRunEvent = this.battle.runEvent.bind(this.battle);
    this.battle.runEvent = function(eventName, ...args) {
        const result = originalRunEvent(eventName, ...args);
        if (eventName === 'BasePower' && args[2]?.id === 'gmaxterror') {
            console.log(`[JS GET_DAMAGE] BasePower event for gmaxterror: before=${args[3]}, after=${result || args[3]}`);
        }
        return result;
    };

    const result = originalGetDamage.call(this, source, target, move, suppressMessages);

    this.battle.runEvent = originalRunEvent;

    console.log(`[JS GET_DAMAGE END] move ${move.id} returned ${result}`);
    return result;
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

// Run 32 makeChoices to get to Turn 33
for (let i = 1; i <= 32; i++) {
    battle.makeChoices('default', 'default');
}

console.log('========== BEFORE Turn 33 ==========');
console.log(`Battle turn: ${battle.turn}`);
console.log(`PRNG calls: ${totalPrngCalls}`);
console.log(`P1 active: ${battle.sides[0].active.map(p => p ? `${p.name} (${p.hp}/${p.maxhp})` : 'null').join(', ')}`);
console.log(`P2 active: ${battle.sides[1].active.map(p => p ? `${p.name} (${p.hp}/${p.maxhp})` : 'null').join(', ')}`);

const logBefore = battle.log.length;
const prngBefore = totalPrngCalls;

console.log('\n========== Turn 33 makeChoices ==========');
battle.makeChoices('default', 'default');

const prngAfter = totalPrngCalls;
console.log(`\nPRNG: ${prngBefore} -> ${prngAfter} (${prngAfter - prngBefore} calls)`);
console.log(`Battle turn: ${battle.turn}`);
console.log(`P1 active: ${battle.sides[0].active.map(p => p ? `${p.name} (${p.hp}/${p.maxhp})` : 'null').join(', ')}`);
console.log(`P2 active: ${battle.sides[1].active.map(p => p ? `${p.name} (${p.hp}/${p.maxhp})` : 'null').join(', ')}`);

console.log('\n========== Turn 33 Log ==========');
const newLog = battle.log.slice(logBefore);
console.log(newLog.join('\n'));
