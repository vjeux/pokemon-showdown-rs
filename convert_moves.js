// Script to convert Pokemon Showdown moves.ts to Rust format
const fs = require('fs');

// Read the moves file content from stdin or file
const movesContent = fs.readFileSync('/dev/stdin', 'utf8');

// Parse the moves - we'll use regex to extract each move block
const moveRegex = /\t([a-z0-9]+):\s*\{([^}]+(?:\{[^}]*\}[^}]*)*)\}/g;

// Helper to extract a property value
function extractProp(content, prop, defaultVal = null) {
    const regex = new RegExp(`${prop}:\\s*([^,\\n]+)`);
    const match = content.match(regex);
    if (!match) return defaultVal;
    let val = match[1].trim();
    // Remove trailing comma
    if (val.endsWith(',')) val = val.slice(0, -1);
    return val;
}

function extractString(content, prop, defaultVal = '') {
    const regex = new RegExp(`${prop}:\\s*"([^"]+)"`);
    const match = content.match(regex);
    return match ? match[1] : defaultVal;
}

function extractNumber(content, prop, defaultVal = 0) {
    const val = extractProp(content, prop);
    if (val === null || val === 'true') return defaultVal;
    return parseInt(val) || defaultVal;
}

function extractBool(content, prop) {
    const val = extractProp(content, prop);
    return val === 'true' || val === '1';
}

// Extract flags
function extractFlags(content) {
    const flagsMatch = content.match(/flags:\s*\{([^}]*)\}/);
    if (!flagsMatch) return {};
    const flagsStr = flagsMatch[1];
    const flags = {};
    const flagRegex = /(\w+):\s*(\d)/g;
    let match;
    while ((match = flagRegex.exec(flagsStr)) !== null) {
        flags[match[1]] = true;
    }
    return flags;
}

// Extract boosts
function extractBoosts(content, section = '') {
    const pattern = section ?
        new RegExp(`${section}:\\s*\\{[^}]*boosts:\\s*\\{([^}]*)\\}`) :
        /\bboosts:\s*\{([^}]*)\}/;
    const match = content.match(pattern);
    if (!match) return null;
    const boostsStr = match[1];
    const boosts = [];
    const boostRegex = /(\w+):\s*(-?\d+)/g;
    let m;
    while ((m = boostRegex.exec(boostsStr)) !== null) {
        boosts.push([m[1], parseInt(m[2])]);
    }
    return boosts.length > 0 ? boosts : null;
}

// Extract secondary effect
function extractSecondary(content) {
    // Check for secondary: null
    if (content.match(/secondary:\s*null/)) return null;

    const secondaryMatch = content.match(/secondary:\s*\{([^}]*(?:\{[^}]*\}[^}]*)*)\}/);
    if (!secondaryMatch) return null;

    const secContent = secondaryMatch[1];
    const secondary = {};

    const chance = extractNumber(secContent, 'chance', 100);
    secondary.chance = chance;

    const status = extractString(secContent, 'status');
    if (status) secondary.status = status;

    const volatileStatus = extractString(secContent, 'volatileStatus');
    if (volatileStatus) secondary.volatileStatus = volatileStatus;

    const boosts = extractBoosts(secContent);
    if (boosts) secondary.boosts = boosts;

    return secondary;
}

// Extract drain [numerator, denominator]
function extractDrain(content) {
    const match = content.match(/drain:\s*\[(\d+),\s*(\d+)\]/);
    if (!match) return null;
    return [parseInt(match[1]), parseInt(match[2])];
}

// Extract recoil
function extractRecoil(content) {
    const match = content.match(/recoil:\s*\[(\d+),\s*(\d+)\]/);
    if (!match) return null;
    return [parseInt(match[1]), parseInt(match[2])];
}

// Extract multihit
function extractMultihit(content) {
    // Can be a number or [min, max]
    const arrayMatch = content.match(/multihit:\s*\[(\d+),\s*(\d+)\]/);
    if (arrayMatch) {
        return [parseInt(arrayMatch[1]), parseInt(arrayMatch[2])];
    }
    const numMatch = content.match(/multihit:\s*(\d+)/);
    if (numMatch) {
        const n = parseInt(numMatch[1]);
        return [n, n];
    }
    return null;
}

// Convert category
function categoryToRust(cat) {
    if (!cat) return 'MoveCategory::Physical';
    const c = cat.replace(/"/g, '');
    switch (c) {
        case 'Physical': return 'MoveCategory::Physical';
        case 'Special': return 'MoveCategory::Special';
        case 'Status': return 'MoveCategory::Status';
        default: return 'MoveCategory::Physical';
    }
}

// Convert target
function targetToRust(target) {
    if (!target) return 'MoveTargetType::Normal';
    const t = target.replace(/"/g, '');
    switch (t) {
        case 'normal': return 'MoveTargetType::Normal';
        case 'self': return 'MoveTargetType::Self_';
        case 'allySide': return 'MoveTargetType::AllySide';
        case 'foeSide': return 'MoveTargetType::FoeSide';
        case 'all': return 'MoveTargetType::All';
        case 'allAdjacent': return 'MoveTargetType::AllAdjacent';
        case 'allAdjacentFoes': return 'MoveTargetType::AllAdjacentFoes';
        case 'adjacentAlly': return 'MoveTargetType::Ally';
        case 'adjacentAllyOrSelf': return 'MoveTargetType::Ally';
        case 'any': return 'MoveTargetType::Any';
        case 'randomNormal': return 'MoveTargetType::RandomNormal';
        case 'allies': return 'MoveTargetType::AllySide';
        case 'allyTeam': return 'MoveTargetType::AllySide';
        default: return 'MoveTargetType::Normal';
    }
}

// Generate Rust code for a move
function moveToRust(id, content) {
    const name = extractString(content, 'name', id);
    const basePower = extractNumber(content, 'basePower', 0);
    const accuracy = extractProp(content, 'accuracy');
    const accuracyVal = accuracy === 'true' ? 0 : (parseInt(accuracy) || 100);
    const pp = extractNumber(content, 'pp', 5);
    const category = extractProp(content, 'category');
    const moveType = extractString(content, 'type', 'Normal');
    const priority = extractNumber(content, 'priority', 0);
    const target = extractProp(content, 'target');
    const critRatio = extractNumber(content, 'critRatio', 0);
    const flags = extractFlags(content);
    const secondary = extractSecondary(content);
    const boosts = extractBoosts(content);
    const drain = extractDrain(content);
    const recoil = extractRecoil(content);
    const multihit = extractMultihit(content);

    // Check for special properties
    const isNonstandard = extractProp(content, 'isNonstandard');
    const isZ = extractProp(content, 'isZ');
    const isMax = extractProp(content, 'isMax');

    // Build Rust struct
    let rust = `    map.insert(ID::new("${id}"), MoveDef {\n`;
    rust += `        id: ID::new("${id}"),\n`;
    rust += `        name: "${name}".to_string(),\n`;
    rust += `        base_power: ${basePower},\n`;
    rust += `        accuracy: ${accuracyVal},\n`;
    rust += `        pp: ${pp},\n`;
    rust += `        category: ${categoryToRust(category)},\n`;
    rust += `        move_type: "${moveType}".to_string(),\n`;
    rust += `        priority: ${priority},\n`;
    rust += `        target: ${targetToRust(target)},\n`;

    if (critRatio > 0) {
        rust += `        crit_ratio: ${critRatio},\n`;
    }

    // Flags
    const flagsList = Object.keys(flags);
    if (flagsList.length > 0) {
        rust += `        flags: MoveFlags { `;
        rust += flagsList.map(f => `${f}: true`).join(', ');
        rust += `, ..Default::default() },\n`;
    }

    // Boosts (self boosts for status moves)
    if (boosts && category && category.includes('Status')) {
        rust += `        self_boosts: Some(vec![${boosts.map(b => `("${b[0]}".to_string(), ${b[1]})`).join(', ')}]),\n`;
    } else if (boosts) {
        rust += `        boosts: Some(vec![${boosts.map(b => `("${b[0]}".to_string(), ${b[1]})`).join(', ')}]),\n`;
    }

    // Drain
    if (drain) {
        const fraction = drain[0] / drain[1];
        rust += `        drain: Some(${fraction.toFixed(4)}),\n`;
    }

    // Recoil
    if (recoil) {
        const fraction = recoil[0] / recoil[1];
        rust += `        recoil: Some(${fraction.toFixed(4)}),\n`;
    }

    // Multihit
    if (multihit) {
        rust += `        multi_hit: Some((${multihit[0]}, ${multihit[1]})),\n`;
    }

    // Secondary effects
    if (secondary) {
        rust += `        secondaries: vec![SecondaryEffect {\n`;
        rust += `            chance: ${secondary.chance},\n`;
        if (secondary.status) {
            rust += `            status: Some("${secondary.status}".to_string()),\n`;
        }
        if (secondary.volatileStatus) {
            rust += `            volatile_status: Some("${secondary.volatileStatus}".to_string()),\n`;
        }
        if (secondary.boosts) {
            rust += `            boosts: Some(vec![${secondary.boosts.map(b => `("${b[0]}".to_string(), ${b[1]})`).join(', ')}]),\n`;
        }
        rust += `            ..Default::default()\n`;
        rust += `        }],\n`;
    }

    // Z/Max moves
    if (isZ) {
        rust += `        is_z: true,\n`;
    }
    if (isMax) {
        rust += `        is_max: true,\n`;
    }

    rust += `        ..Default::default()\n`;
    rust += `    });\n`;

    return rust;
}

// Parse all moves
const moves = [];
let match;
while ((match = moveRegex.exec(movesContent)) !== null) {
    const id = match[1];
    const content = match[2];
    moves.push({ id, content });
}

// Generate header
let output = `//! Data-driven Move Definitions
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module defines moves as data structures with their properties,
//! AUTO-GENERATED from the original moves.ts file.

use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::dex_data::ID;
use crate::event::MoveFlags;

/// Move category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoveCategory {
    Physical,
    Special,
    Status,
}

impl Default for MoveCategory {
    fn default() -> Self {
        MoveCategory::Physical
    }
}

/// Move target type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoveTargetType {
    Normal,
    Self_,
    AllySide,
    FoeSide,
    All,
    AllAdjacent,
    AllAdjacentFoes,
    Ally,
    Any,
    RandomNormal,
}

impl Default for MoveTargetType {
    fn default() -> Self {
        MoveTargetType::Normal
    }
}

/// Secondary effect of a move
#[derive(Debug, Clone, Default)]
pub struct SecondaryEffect {
    pub chance: u8,
    pub status: Option<String>,
    pub volatile_status: Option<String>,
    pub boosts: Option<Vec<(String, i8)>>,
    pub self_boost: bool,
    pub flinch: bool,
}

/// Move definition
#[derive(Debug, Clone)]
pub struct MoveDef {
    pub id: ID,
    pub name: String,
    pub base_power: u32,
    pub accuracy: u8,
    pub pp: u8,
    pub category: MoveCategory,
    pub move_type: String,
    pub flags: MoveFlags,
    pub priority: i8,
    pub target: MoveTargetType,
    pub is_z: bool,
    pub is_max: bool,
    pub crit_ratio: u8,
    pub multi_hit: Option<(u8, u8)>,
    pub recoil: Option<f64>,
    pub recoil_hp: Option<f64>,
    pub drain: Option<f64>,
    pub self_drops: Option<Vec<(String, i8)>>,
    pub self_boosts: Option<Vec<(String, i8)>>,
    pub boosts: Option<Vec<(String, i8)>>,
    pub status: Option<String>,
    pub volatile_status: Option<String>,
    pub self_volatile: Option<String>,
    pub side_condition: Option<String>,
    pub slot_condition: Option<String>,
    pub weather: Option<String>,
    pub terrain: Option<String>,
    pub pseudo_weather: Option<String>,
    pub is_pivot: bool,
    pub force_switch: bool,
    pub secondaries: Vec<SecondaryEffect>,
    pub heal: Option<f64>,
    pub heal_by_weather: Option<HashMap<String, f64>>,
    pub ohko: bool,
    pub thaws_user: bool,
    pub ignores_ability: bool,
    pub breaks_protect: bool,
}

impl Default for MoveDef {
    fn default() -> Self {
        Self {
            id: ID::empty(),
            name: String::new(),
            base_power: 0,
            accuracy: 100,
            pp: 5,
            category: MoveCategory::Physical,
            move_type: "Normal".to_string(),
            flags: MoveFlags::default(),
            priority: 0,
            target: MoveTargetType::Normal,
            is_z: false,
            is_max: false,
            crit_ratio: 0,
            multi_hit: None,
            recoil: None,
            recoil_hp: None,
            drain: None,
            self_drops: None,
            self_boosts: None,
            boosts: None,
            status: None,
            volatile_status: None,
            self_volatile: None,
            side_condition: None,
            slot_condition: None,
            weather: None,
            terrain: None,
            pseudo_weather: None,
            is_pivot: false,
            force_switch: false,
            secondaries: Vec::new(),
            heal: None,
            heal_by_weather: None,
            ohko: false,
            thaws_user: false,
            ignores_ability: false,
            breaks_protect: false,
        }
    }
}

impl MoveDef {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: ID::new(id),
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn is_status(&self) -> bool {
        self.category == MoveCategory::Status
    }
}

/// All moves
pub static MOVES: Lazy<HashMap<ID, MoveDef>> = Lazy::new(|| {
    let mut map = HashMap::new();

`;

// Generate each move
for (const move of moves) {
    output += moveToRust(move.id, move.content);
    output += '\n';
}

output += `
    map
});

/// Get move definition by ID
pub fn get_move(id: &ID) -> Option<&'static MoveDef> {
    MOVES.get(id)
}

/// Check if a move is a pivot move
pub fn is_pivot_move(move_id: &ID) -> bool {
    get_move(move_id).map_or(false, |m| m.is_pivot)
}

/// Check if a move is a status move
pub fn is_status_move(move_id: &ID) -> bool {
    get_move(move_id).map_or(false, |m| m.is_status())
}

/// Get move base power
pub fn get_base_power(move_id: &ID) -> u32 {
    get_move(move_id).map_or(0, |m| m.base_power)
}

/// Get move accuracy
pub fn get_accuracy(move_id: &ID) -> u8 {
    get_move(move_id).map_or(100, |m| m.accuracy)
}

/// Get move category
pub fn get_category(move_id: &ID) -> MoveCategory {
    get_move(move_id).map_or(MoveCategory::Physical, |m| m.category)
}

/// Get move type
pub fn get_move_type(move_id: &ID) -> String {
    get_move(move_id).map_or("Normal".to_string(), |m| m.move_type.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_count() {
        assert!(MOVES.len() > 900, "Should have 900+ moves");
    }

    #[test]
    fn test_earthquake() {
        let eq = get_move(&ID::new("earthquake")).expect("Should have earthquake");
        assert_eq!(eq.base_power, 100);
        assert_eq!(eq.accuracy, 100);
        assert!(matches!(eq.category, MoveCategory::Physical));
        assert_eq!(eq.move_type, "Ground");
    }

    #[test]
    fn test_status_moves() {
        assert!(is_status_move(&ID::new("thunderwave")));
        assert!(is_status_move(&ID::new("swordsdance")));
        assert!(!is_status_move(&ID::new("earthquake")));
    }
}
`;

console.log(output);
