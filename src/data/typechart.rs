//! Data-driven Type Chart
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module contains the complete type effectiveness chart
//! for all 18 Pokemon types.

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// All Pokemon types
pub const TYPES: [&str; 18] = [
    "Normal", "Fire", "Water", "Electric", "Grass", "Ice", "Fighting", "Poison", "Ground",
    "Flying", "Psychic", "Bug", "Rock", "Ghost", "Dragon", "Dark", "Steel", "Fairy",
];

/// Type effectiveness values
/// JavaScript equivalent: Type effectiveness multipliers (sim/dex-data.ts)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Effectiveness {
    Immune,    // 0x
    Resistant, // 0.5x
    Neutral,   // 1x
    Weak,      // 2x
}

impl Effectiveness {
    pub fn multiplier(&self) -> f64 {
        match self {
            Effectiveness::Immune => 0.0,
            Effectiveness::Resistant => 0.5,
            Effectiveness::Neutral => 1.0,
            Effectiveness::Weak => 2.0,
        }
    }
}

/// Type chart as static HashMap
/// Key: (attacking_type, defending_type) -> multiplier
pub static TYPE_CHART: Lazy<HashMap<(&'static str, &'static str), f64>> = Lazy::new(|| {
    let mut chart = HashMap::new();

    // Initialize all to neutral (1.0)
    for atk in &TYPES {
        for def in &TYPES {
            chart.insert((*atk, *def), 1.0);
        }
    }

    // Normal type
    chart.insert(("Normal", "Rock"), 0.5);
    chart.insert(("Normal", "Steel"), 0.5);
    chart.insert(("Normal", "Ghost"), 0.0);

    // Fire type
    chart.insert(("Fire", "Fire"), 0.5);
    chart.insert(("Fire", "Water"), 0.5);
    chart.insert(("Fire", "Grass"), 2.0);
    chart.insert(("Fire", "Ice"), 2.0);
    chart.insert(("Fire", "Bug"), 2.0);
    chart.insert(("Fire", "Rock"), 0.5);
    chart.insert(("Fire", "Dragon"), 0.5);
    chart.insert(("Fire", "Steel"), 2.0);

    // Water type
    chart.insert(("Water", "Fire"), 2.0);
    chart.insert(("Water", "Water"), 0.5);
    chart.insert(("Water", "Grass"), 0.5);
    chart.insert(("Water", "Ground"), 2.0);
    chart.insert(("Water", "Rock"), 2.0);
    chart.insert(("Water", "Dragon"), 0.5);

    // Electric type
    chart.insert(("Electric", "Water"), 2.0);
    chart.insert(("Electric", "Electric"), 0.5);
    chart.insert(("Electric", "Grass"), 0.5);
    chart.insert(("Electric", "Ground"), 0.0);
    chart.insert(("Electric", "Flying"), 2.0);
    chart.insert(("Electric", "Dragon"), 0.5);

    // Grass type
    chart.insert(("Grass", "Fire"), 0.5);
    chart.insert(("Grass", "Water"), 2.0);
    chart.insert(("Grass", "Grass"), 0.5);
    chart.insert(("Grass", "Poison"), 0.5);
    chart.insert(("Grass", "Ground"), 2.0);
    chart.insert(("Grass", "Flying"), 0.5);
    chart.insert(("Grass", "Bug"), 0.5);
    chart.insert(("Grass", "Rock"), 2.0);
    chart.insert(("Grass", "Dragon"), 0.5);
    chart.insert(("Grass", "Steel"), 0.5);

    // Ice type
    chart.insert(("Ice", "Fire"), 0.5);
    chart.insert(("Ice", "Water"), 0.5);
    chart.insert(("Ice", "Grass"), 2.0);
    chart.insert(("Ice", "Ice"), 0.5);
    chart.insert(("Ice", "Ground"), 2.0);
    chart.insert(("Ice", "Flying"), 2.0);
    chart.insert(("Ice", "Dragon"), 2.0);
    chart.insert(("Ice", "Steel"), 0.5);

    // Fighting type
    chart.insert(("Fighting", "Normal"), 2.0);
    chart.insert(("Fighting", "Ice"), 2.0);
    chart.insert(("Fighting", "Poison"), 0.5);
    chart.insert(("Fighting", "Flying"), 0.5);
    chart.insert(("Fighting", "Psychic"), 0.5);
    chart.insert(("Fighting", "Bug"), 0.5);
    chart.insert(("Fighting", "Rock"), 2.0);
    chart.insert(("Fighting", "Ghost"), 0.0);
    chart.insert(("Fighting", "Dark"), 2.0);
    chart.insert(("Fighting", "Steel"), 2.0);
    chart.insert(("Fighting", "Fairy"), 0.5);

    // Poison type
    chart.insert(("Poison", "Grass"), 2.0);
    chart.insert(("Poison", "Poison"), 0.5);
    chart.insert(("Poison", "Ground"), 0.5);
    chart.insert(("Poison", "Rock"), 0.5);
    chart.insert(("Poison", "Ghost"), 0.5);
    chart.insert(("Poison", "Steel"), 0.0);
    chart.insert(("Poison", "Fairy"), 2.0);

    // Ground type
    chart.insert(("Ground", "Fire"), 2.0);
    chart.insert(("Ground", "Electric"), 2.0);
    chart.insert(("Ground", "Grass"), 0.5);
    chart.insert(("Ground", "Poison"), 2.0);
    chart.insert(("Ground", "Flying"), 0.0);
    chart.insert(("Ground", "Bug"), 0.5);
    chart.insert(("Ground", "Rock"), 2.0);
    chart.insert(("Ground", "Steel"), 2.0);

    // Flying type
    chart.insert(("Flying", "Electric"), 0.5);
    chart.insert(("Flying", "Grass"), 2.0);
    chart.insert(("Flying", "Fighting"), 2.0);
    chart.insert(("Flying", "Bug"), 2.0);
    chart.insert(("Flying", "Rock"), 0.5);
    chart.insert(("Flying", "Steel"), 0.5);

    // Psychic type
    chart.insert(("Psychic", "Fighting"), 2.0);
    chart.insert(("Psychic", "Poison"), 2.0);
    chart.insert(("Psychic", "Psychic"), 0.5);
    chart.insert(("Psychic", "Dark"), 0.0);
    chart.insert(("Psychic", "Steel"), 0.5);

    // Bug type
    chart.insert(("Bug", "Fire"), 0.5);
    chart.insert(("Bug", "Grass"), 2.0);
    chart.insert(("Bug", "Fighting"), 0.5);
    chart.insert(("Bug", "Poison"), 0.5);
    chart.insert(("Bug", "Flying"), 0.5);
    chart.insert(("Bug", "Psychic"), 2.0);
    chart.insert(("Bug", "Ghost"), 0.5);
    chart.insert(("Bug", "Dark"), 2.0);
    chart.insert(("Bug", "Steel"), 0.5);
    chart.insert(("Bug", "Fairy"), 0.5);

    // Rock type
    chart.insert(("Rock", "Fire"), 2.0);
    chart.insert(("Rock", "Ice"), 2.0);
    chart.insert(("Rock", "Fighting"), 0.5);
    chart.insert(("Rock", "Ground"), 0.5);
    chart.insert(("Rock", "Flying"), 2.0);
    chart.insert(("Rock", "Bug"), 2.0);
    chart.insert(("Rock", "Steel"), 0.5);

    // Ghost type
    chart.insert(("Ghost", "Normal"), 0.0);
    chart.insert(("Ghost", "Psychic"), 2.0);
    chart.insert(("Ghost", "Ghost"), 2.0);
    chart.insert(("Ghost", "Dark"), 0.5);

    // Dragon type
    chart.insert(("Dragon", "Dragon"), 2.0);
    chart.insert(("Dragon", "Steel"), 0.5);
    chart.insert(("Dragon", "Fairy"), 0.0);

    // Dark type
    chart.insert(("Dark", "Fighting"), 0.5);
    chart.insert(("Dark", "Psychic"), 2.0);
    chart.insert(("Dark", "Ghost"), 2.0);
    chart.insert(("Dark", "Dark"), 0.5);
    chart.insert(("Dark", "Fairy"), 0.5);

    // Steel type
    chart.insert(("Steel", "Fire"), 0.5);
    chart.insert(("Steel", "Water"), 0.5);
    chart.insert(("Steel", "Electric"), 0.5);
    chart.insert(("Steel", "Ice"), 2.0);
    chart.insert(("Steel", "Rock"), 2.0);
    chart.insert(("Steel", "Steel"), 0.5);
    chart.insert(("Steel", "Fairy"), 2.0);

    // Fairy type
    chart.insert(("Fairy", "Fire"), 0.5);
    chart.insert(("Fairy", "Fighting"), 2.0);
    chart.insert(("Fairy", "Poison"), 0.5);
    chart.insert(("Fairy", "Dragon"), 2.0);
    chart.insert(("Fairy", "Dark"), 2.0);
    chart.insert(("Fairy", "Steel"), 0.5);

    chart
});

/// Get type effectiveness multiplier
pub fn get_effectiveness(attacking_type: &str, defending_type: &str) -> f64 {
    // Normalize case
    let atk = normalize_type(attacking_type);
    let def = normalize_type(defending_type);

    *TYPE_CHART.get(&(atk, def)).unwrap_or(&1.0)
}

/// Get combined effectiveness against multiple types
pub fn get_effectiveness_multi(attacking_type: &str, defending_types: &[String]) -> f64 {
    defending_types
        .iter()
        .map(|def_type| get_effectiveness(attacking_type, def_type))
        .product()
}

/// Normalize type name to proper case
fn normalize_type(type_name: &str) -> &'static str {
    let lower = type_name.to_lowercase();
    match lower.as_str() {
        "normal" => "Normal",
        "fire" => "Fire",
        "water" => "Water",
        "electric" => "Electric",
        "grass" => "Grass",
        "ice" => "Ice",
        "fighting" => "Fighting",
        "poison" => "Poison",
        "ground" => "Ground",
        "flying" => "Flying",
        "psychic" => "Psychic",
        "bug" => "Bug",
        "rock" => "Rock",
        "ghost" => "Ghost",
        "dragon" => "Dragon",
        "dark" => "Dark",
        "steel" => "Steel",
        "fairy" => "Fairy",
        _ => "Normal", // Fallback
    }
}

/// Check if a type is immune to another
pub fn is_immune(attacking_type: &str, defending_type: &str) -> bool {
    get_effectiveness(attacking_type, defending_type) == 0.0
}

/// Check if a type is super effective against another
pub fn is_super_effective(attacking_type: &str, defending_type: &str) -> bool {
    get_effectiveness(attacking_type, defending_type) > 1.0
}

/// Check if a type is not very effective against another
pub fn is_not_very_effective(attacking_type: &str, defending_type: &str) -> bool {
    let eff = get_effectiveness(attacking_type, defending_type);
    eff > 0.0 && eff < 1.0
}

/// Get all types that the given type is super effective against
pub fn get_super_effective_targets(attacking_type: &str) -> Vec<&'static str> {
    TYPES
        .iter()
        .filter(|def| is_super_effective(attacking_type, def))
        .copied()
        .collect()
}

/// Get all types that the given type is not very effective against
pub fn get_resisted_by(attacking_type: &str) -> Vec<&'static str> {
    TYPES
        .iter()
        .filter(|def| is_not_very_effective(attacking_type, def))
        .copied()
        .collect()
}

/// Get all types that the given type is immune to
pub fn get_immunities(attacking_type: &str) -> Vec<&'static str> {
    TYPES
        .iter()
        .filter(|def| is_immune(attacking_type, def))
        .copied()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fire_vs_grass() {
        assert_eq!(get_effectiveness("Fire", "Grass"), 2.0);
        assert!(is_super_effective("Fire", "Grass"));
    }

    #[test]
    fn test_water_vs_fire() {
        assert_eq!(get_effectiveness("Water", "Fire"), 2.0);
    }

    #[test]
    fn test_electric_vs_ground() {
        assert_eq!(get_effectiveness("Electric", "Ground"), 0.0);
        assert!(is_immune("Electric", "Ground"));
    }

    #[test]
    fn test_normal_vs_ghost() {
        assert_eq!(get_effectiveness("Normal", "Ghost"), 0.0);
        assert!(is_immune("Normal", "Ghost"));
    }

    #[test]
    fn test_ghost_vs_normal() {
        assert_eq!(get_effectiveness("Ghost", "Normal"), 0.0);
    }

    #[test]
    fn test_fighting_vs_dark() {
        assert_eq!(get_effectiveness("Fighting", "Dark"), 2.0);
    }

    #[test]
    fn test_fairy_vs_dragon() {
        assert_eq!(get_effectiveness("Fairy", "Dragon"), 2.0);
    }

    #[test]
    fn test_dragon_vs_fairy() {
        assert_eq!(get_effectiveness("Dragon", "Fairy"), 0.0);
        assert!(is_immune("Dragon", "Fairy"));
    }

    #[test]
    fn test_multi_type() {
        // Fire vs Water/Flying (Gyarados) = 0.5 * 1.0 = 0.5
        let types = vec!["Water".to_string(), "Flying".to_string()];
        assert_eq!(get_effectiveness_multi("Fire", &types), 0.5);

        // Ground vs Water/Flying = 2.0 * 0.0 = 0.0
        assert_eq!(get_effectiveness_multi("Ground", &types), 0.0);

        // Ice vs Ground/Flying (Landorus) = 2.0 * 2.0 = 4.0
        let lando = vec!["Ground".to_string(), "Flying".to_string()];
        assert_eq!(get_effectiveness_multi("Ice", &lando), 4.0);
    }

    #[test]
    fn test_case_insensitivity() {
        assert_eq!(get_effectiveness("fire", "grass"), 2.0);
        assert_eq!(get_effectiveness("WATER", "fire"), 2.0);
        assert_eq!(get_effectiveness("Fire", "GRASS"), 2.0);
    }

    #[test]
    fn test_super_effective_targets() {
        let targets = get_super_effective_targets("Fire");
        assert!(targets.contains(&"Grass"));
        assert!(targets.contains(&"Ice"));
        assert!(targets.contains(&"Bug"));
        assert!(targets.contains(&"Steel"));
    }

    #[test]
    fn test_immunities() {
        let immunities = get_immunities("Ground");
        assert!(immunities.contains(&"Flying"));
        assert_eq!(immunities.len(), 1);
    }
}
