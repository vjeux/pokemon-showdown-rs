const fs = require('fs');
const path = require('path');

// Load the conditions module
const { Conditions } = require('/Users/vjeux/random/showdown/pokemon-showdown-ts/dist/data/conditions.js');

// Convert to JSON-safe object by extracting only metadata
const jsonConditions = {};

for (const [id, condition] of Object.entries(Conditions)) {
  const jsonCond = {};

  // Copy simple properties
  if (condition.name) jsonCond.name = condition.name;
  if (condition.effectType) jsonCond.effectType = condition.effectType;
  if (condition.duration !== undefined) jsonCond.duration = condition.duration;
  if (condition.noCopy) jsonCond.noCopy = condition.noCopy;
  if (condition.affectsFainted) jsonCond.affectsFainted = condition.affectsFainted;
  if (condition.counterMax !== undefined) jsonCond.counterMax = condition.counterMax;

  // Copy priority properties
  const priorities = [
    'onModifySpePriority',
    'onBeforeMovePriority',
    'onModifySpDPriority',
    'onModifyDefPriority',
    'onEffectivenessPriority',
    'onBasePowerPriority',
    'onFieldResidualOrder',
    'onResidualOrder',
    'onBeforeSwitchOutPriority',
    'onDragOutPriority',
    'onResidualPriority',
    'onTrapPokemonPriority',
    'onTypePriority',
    'onTryMovePriority'
  ];

  for (const prop of priorities) {
    if (condition[prop] !== undefined) {
      jsonCond[prop] = condition[prop];
    }
  }

  // Special boolean properties
  if (condition.onInvulnerability === false) {
    jsonCond.onInvulnerability = false;
  }

  jsonConditions[id] = jsonCond;
}

// Write to JSON file
const outputPath = path.join(__dirname, '..', 'data', 'conditions.json');
fs.writeFileSync(outputPath, JSON.stringify(jsonConditions, null, 2) + '\n');

console.log(`Generated ${outputPath} with ${Object.keys(jsonConditions).length} conditions`);
