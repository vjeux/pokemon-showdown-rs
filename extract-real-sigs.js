#!/usr/bin/env node

const fs = require('fs');

// Read mod.rs to get all signatures
const modContent = fs.readFileSync('src/data/ability_callbacks/mod.rs', 'utf8');

// Extract all dispatcher signatures precisely
const dispatcherSignatures = new Map();
const lines = modContent.split('\n');

for (let i = 0; i < lines.length; i++) {
  if (lines[i].includes('pub fn dispatch_')) {
    const match = lines[i].match(/pub fn (dispatch_\w+)\(/);
    if (match) {
      const funcName = match[1];

      // Collect full signature across multiple lines
      let sig = '';
      let j = i;
      while (j < lines.length && !lines[j].trim().startsWith(')')) {
        sig += lines[j] + '\n';
        j++;
      }
      if (j < lines.length) {
        sig += lines[j]; // Add the closing ) line
      }

      // Now extract the parameters - everything between ability_id: &str, and ) -> EventResult
      // The signature looks like:
      // pub fn dispatch_on_xxx(
      //     battle: &mut Battle,
      //     ability_id: &str,
      //     param1: Type1, param2: Type2, ...
      // ) -> EventResult

      // First, join all lines and normalize whitespace
      const normalized = sig.replace(/\n/g, ' ').replace(/\s+/g, ' ');

      // Extract everything after "ability_id: &str," and before ")"
      // Use a more precise regex that handles the EventResult return type
      const paramMatch = normalized.match(/ability_id:\s*&str,\s*(.*?)\s*\)\s*->\s*EventResult/);

      if (paramMatch && paramMatch[1].trim()) {
        const paramsStr = paramMatch[1].trim();
        dispatcherSignatures.set(funcName, paramsStr);
      } else {
        // No parameters after ability_id
        dispatcherSignatures.set(funcName, '');
      }
    }
  }
}

console.log(`Found ${dispatcherSignatures.size} dispatchers\n`);

// Generate default args for each parameter type
function getDefaultArgs(paramStr) {
  if (!paramStr) return [];

  // Split on commas, but handle nested types properly
  const params = [];
  let current = '';
  let depth = 0;

  for (let i = 0; i < paramStr.length; i++) {
    const char = paramStr[i];
    if (char === '<' || char === '(') depth++;
    if (char === '>' || char === ')') depth--;

    if (char === ',' && depth === 0) {
      params.push(current.trim());
      current = '';
    } else {
      current += char;
    }
  }
  if (current.trim()) params.push(current.trim());

  return params.map(param => {
    // Extract parameter name and type
    const parts = param.split(':');
    if (parts.length < 2) return 'todo!()';

    const paramName = parts[0].trim();
    const type = parts[1].trim();

    // Generate default values based on type
    if (type === 'i32' || type === 'i64') return '0';
    if (type === 'f32' || type === 'f64') return '0.0';
    if (type === 'bool') return 'false';
    if (type === '&str') return '""';
    if (type.includes('Option<(usize, usize)>')) {
      // Check parameter name to decide Some vs None
      if (paramName.includes('pokemon_pos') || paramName.includes('target_pos') && paramName === 'target_pos') {
        // For the first pokemon_pos or direct target_pos, use Some(pokemon_pos)
        return 'Some(pokemon_pos)';
      }
      // For source_pos or other optional positions, use None
      return 'None';
    }
    if (type.includes('(usize, usize)')) return 'pokemon_pos';
    if (type.includes('Option<&str>')) return 'None';
    if (type.startsWith('Option<')) return 'None';

    return 'todo!()';
  });
}

// Create comprehensive mapping
const mapping = {};
const sortedDispatchers = Array.from(dispatcherSignatures.entries()).sort((a, b) => a[0].localeCompare(b[0]));

console.log('Dispatcher signatures:\n');
console.log('='.repeat(80) + '\n');

sortedDispatchers.forEach(([name, params]) => {
  const args = getDefaultArgs(params);
  mapping[name] = {
    params: params,
    args: args
  };

  console.log(`${name}:`);
  if (params) {
    console.log(`  Params: ${params}`);
  }
  if (args.length > 0) {
    console.log(`  Args: self, ability_id.as_str(), ${args.join(', ')}`);
  } else {
    console.log(`  Args: self, ability_id.as_str()`);
  }
  console.log('');
});

fs.writeFileSync('real-dispatcher-mapping.json', JSON.stringify(mapping, null, 2));
console.log(`\nSaved mapping to real-dispatcher-mapping.json (${sortedDispatchers.length} dispatchers)`);
