//! BattleActions::canMegaEvo - Check if Pokemon can Mega Evolve
//!
//! 1:1 port of canMegaEvo from battle-actions.ts

use crate::*;
use crate::dex_data::to_id;

/// Check if Pokemon can Mega Evolve
/// Equivalent to battle-actions.ts canMegaEvo()
///
/// canMegaEvo(pokemon: Pokemon) {
///     const species = pokemon.baseSpecies;
///     const altForme = species.otherFormes && this.dex.species.get(species.otherFormes[0]);
///     const item = pokemon.getItem();
///     // Mega Rayquaza
///     if ((this.battle.gen <= 7 || this.battle.ruleTable.has('+pokemontag:past') ||
///         this.battle.ruleTable.has('+pokemontag:future')) &&
///         altForme?.isMega && altForme?.requiredMove &&
///         pokemon.baseMoves.includes(toID(altForme.requiredMove)) && !item.zMove) {
///         return altForme.name;
///     }
///     // Temporary hardcode until generation shift
///     if ((species.baseSpecies === "Floette" || species.baseSpecies === "Zygarde") && item.megaEvolves === species.name) {
///         return item.megaStone as string;
///     }
///     // a hacked-in Megazard X can mega evolve into Megazard Y, but not into Megazard X
///     if (Array.isArray(item.megaStone)) {
///         // FIXME: Change to species.name when champions comes
///         const index = (item.megaEvolves as string[]).indexOf(species.baseSpecies);
///         if (index < 0) return null;
///         return item.megaStone[index];
///         // FIXME: Change to species.name when champions comes
///     } else if (item.megaEvolves === species.baseSpecies && item.megaStone !== species.name) {
///         return item.megaStone;
///     }
///     return null;
/// }
pub fn can_mega_evo(
    battle: &Battle,
    side_index: usize,
    pokemon_index: usize,
) -> Option<String> {
    // Get pokemon
    let pokemon = match battle.sides.get(side_index)
        .and_then(|s| s.pokemon.get(pokemon_index)) {
        Some(p) => p,
        None => return None,
    };

    // const species = pokemon.baseSpecies;
    let species = match battle.dex.species().get(&pokemon.base_species.as_str()) {
        Some(s) => s,
        None => return None,
    };

    // const altForme = species.otherFormes && this.dex.species.get(species.otherFormes[0]);
    let alt_forme = if !species.other_formes.is_empty() {
        battle.dex.species().get(&species.other_formes[0])
    } else {
        None
    };

    // const item = pokemon.getItem();
    let item = battle.dex.items().get(pokemon.item.as_str());

    // Early return if no item
    let item = match item {
        Some(i) => i,
        None => return None,
    };

    // Mega Rayquaza
    // if ((this.battle.gen <= 7 || this.battle.ruleTable.has('+pokemontag:past') ||
    //     this.battle.ruleTable.has('+pokemontag:future')) &&
    //     altForme?.isMega && altForme?.requiredMove &&
    //     pokemon.baseMoves.includes(toID(altForme.requiredMove)) && !item.zMove) {
    //     return altForme.name;
    // }
    if let Some(ref alt) = alt_forme {
        let gen_check = battle.gen <= 7
            || battle.rule_table.as_ref().map(|rt| rt.has("+pokemontag:past")).unwrap_or(false)
            || battle.rule_table.as_ref().map(|rt| rt.has("+pokemontag:future")).unwrap_or(false);

        if gen_check && alt.is_mega && alt.required_move.is_some() {
            if let Some(ref required_move) = alt.required_move {
                let base_moves = pokemon.get_base_moves();
                let required_move_id = to_id(required_move);
                if base_moves.contains(&required_move_id) && item.z_move.is_none() {
                    return Some(alt.name.clone());
                }
            }
        }
    }

    // Temporary hardcode until generation shift
    // if ((species.baseSpecies === "Floette" || species.baseSpecies === "Zygarde") && item.megaEvolves === species.name) {
    //     return item.megaStone as string;
    // }
    if (species.base_species == Some("Floette".to_string())
        || species.base_species == Some("Zygarde".to_string()))
        && item.mega_evolves.as_ref().map(|me| me.contains(&species.name)).unwrap_or(false)
    {
        if let Some(ref mega_stone) = item.mega_stone {
            // mega_stone can be a single string or multiple strings
            // For now, handle single string case
            match mega_stone {
                crate::dex::StringOrVec::Single(s) => return Some(s.clone()),
                crate::dex::StringOrVec::Multiple(v) if !v.is_empty() => return Some(v[0].clone()),
                _ => {}
            }
        }
    }

    // a hacked-in Megazard X can mega evolve into Megazard Y, but not into Megazard X
    // if (Array.isArray(item.megaStone)) {
    //     const index = (item.megaEvolves as string[]).indexOf(species.baseSpecies);
    //     if (index < 0) return null;
    //     return item.megaStone[index];
    // } else if (item.megaEvolves === species.baseSpecies && item.megaStone !== species.name) {
    //     return item.megaStone;
    // }
    if let Some(ref mega_stone) = item.mega_stone {
        match mega_stone {
            crate::dex::StringOrVec::Multiple(stones) => {
                // Array case
                if let Some(ref mega_evolves) = item.mega_evolves {
                    match mega_evolves {
                        crate::dex::StringOrVec::Multiple(evolves) => {
                            if let Some(ref base_species_name) = species.base_species {
                                if let Some(index) = evolves.iter().position(|e| e == base_species_name) {
                                    if index < stones.len() {
                                        return Some(stones[index].clone());
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                None
            }
            crate::dex::StringOrVec::Single(stone) => {
                // Single string case
                if let Some(ref mega_evolves) = item.mega_evolves {
                    let species_base = species.base_species.as_ref().unwrap_or(&species.name);
                    if mega_evolves.contains(species_base) && stone != &species.name {
                        return Some(stone.clone());
                    }
                }
                None
            }
            _ => None
        }
    } else {
        None
    }
}
