//! Teams
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Functions for converting and generating teams.

use crate::dex_data::{Gender, StatsTable};
use crate::pokemon::PokemonSet;

/// Convert a name to ID format (lowercase, no spaces/special chars)
pub fn to_id(name: &str) -> String {
    name.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

/// Pack a team into the compact string format
pub fn pack_team(team: &[PokemonSet]) -> String {
    let mut buf = String::new();

    for (i, set) in team.iter().enumerate() {
        if i > 0 {
            buf.push(']');
        }

        // name
        let name = if set.name.is_empty() { &set.species } else { &set.name };
        buf.push_str(name);

        // species
        let species_id = to_id(&set.species);
        let name_id = to_id(name);
        buf.push('|');
        if name_id != species_id {
            buf.push_str(&species_id);
        }

        // item
        buf.push('|');
        buf.push_str(&to_id(&set.item));

        // ability
        buf.push('|');
        buf.push_str(&to_id(&set.ability));

        // moves
        buf.push('|');
        let moves: Vec<String> = set.moves.iter().map(|m| to_id(m)).collect();
        buf.push_str(&moves.join(","));

        // nature
        buf.push('|');
        if set.nature != "Hardy" && !set.nature.is_empty() {
            buf.push_str(&set.nature);
        }

        // evs
        buf.push('|');
        let evs = &set.evs;
        if evs.hp != 0 || evs.atk != 0 || evs.def != 0 || evs.spa != 0 || evs.spd != 0 || evs.spe != 0 {
            let ev_str = format!("{},{},{},{},{},{}",
                if evs.hp == 0 { String::new() } else { evs.hp.to_string() },
                if evs.atk == 0 { String::new() } else { evs.atk.to_string() },
                if evs.def == 0 { String::new() } else { evs.def.to_string() },
                if evs.spa == 0 { String::new() } else { evs.spa.to_string() },
                if evs.spd == 0 { String::new() } else { evs.spd.to_string() },
                if evs.spe == 0 { String::new() } else { evs.spe.to_string() },
            );
            buf.push_str(&ev_str);
        }

        // gender
        buf.push('|');
        buf.push_str(set.gender.to_str());

        // ivs
        buf.push('|');
        let ivs = &set.ivs;
        if ivs.hp != 31 || ivs.atk != 31 || ivs.def != 31 || ivs.spa != 31 || ivs.spd != 31 || ivs.spe != 31 {
            let iv_str = format!("{},{},{},{},{},{}",
                if ivs.hp == 31 { String::new() } else { ivs.hp.to_string() },
                if ivs.atk == 31 { String::new() } else { ivs.atk.to_string() },
                if ivs.def == 31 { String::new() } else { ivs.def.to_string() },
                if ivs.spa == 31 { String::new() } else { ivs.spa.to_string() },
                if ivs.spd == 31 { String::new() } else { ivs.spd.to_string() },
                if ivs.spe == 31 { String::new() } else { ivs.spe.to_string() },
            );
            buf.push_str(&iv_str);
        }

        // shiny
        buf.push('|');
        if set.shiny {
            buf.push('S');
        }

        // level
        buf.push('|');
        if set.level != 100 {
            buf.push_str(&set.level.to_string());
        }

        // happiness
        buf.push('|');
        if set.happiness != 255 {
            buf.push_str(&set.happiness.to_string());
        }

        // Extended fields: hpType, pokeball, gigantamax, dynamaxLevel, teraType
        if set.hptype.is_some() || !set.pokeball.is_empty() || set.gigantamax
            || set.dynamax_level != 10 || set.tera_type.is_some() {
            buf.push(',');
            if let Some(ref hptype) = set.hptype {
                buf.push_str(hptype);
            }
            buf.push(',');
            if !set.pokeball.is_empty() {
                buf.push_str(&to_id(&set.pokeball));
            }
            buf.push(',');
            if set.gigantamax {
                buf.push('G');
            }
            buf.push(',');
            if set.dynamax_level != 10 {
                buf.push_str(&set.dynamax_level.to_string());
            }
            buf.push(',');
            if let Some(ref tera_type) = set.tera_type {
                buf.push_str(tera_type);
            }
        }
    }

    buf
}

/// Unpack a team from the compact string format
pub fn unpack_team(buf: &str) -> Option<Vec<PokemonSet>> {
    if buf.is_empty() {
        return Some(Vec::new());
    }

    // Handle JSON format
    if buf.starts_with('[') && buf.ends_with(']') {
        if let Ok(team) = serde_json::from_str::<Vec<PokemonSet>>(buf) {
            return Some(team);
        }
        return None;
    }

    let mut team = Vec::new();

    for pokemon_str in buf.split(']') {
        if pokemon_str.is_empty() {
            continue;
        }

        let parts: Vec<&str> = pokemon_str.split('|').collect();
        if parts.len() < 10 {
            continue;
        }

        let mut set = PokemonSet::default();

        // name (0)
        set.name = parts[0].to_string();

        // species (1)
        set.species = if parts[1].is_empty() {
            set.name.clone()
        } else {
            parts[1].to_string()
        };

        // item (2)
        set.item = parts[2].to_string();

        // ability (3)
        set.ability = parts[3].to_string();

        // moves (4)
        set.moves = parts[4].split(',')
            .filter(|m| !m.is_empty())
            .map(|m| m.to_string())
            .collect();

        // nature (5)
        set.nature = if parts[5].is_empty() {
            "Hardy".to_string()
        } else {
            parts[5].to_string()
        };

        // evs (6)
        if !parts[6].is_empty() {
            let ev_parts: Vec<&str> = parts[6].split(',').collect();
            if ev_parts.len() >= 6 {
                set.evs = StatsTable {
                    hp: ev_parts[0].parse().unwrap_or(0),
                    atk: ev_parts[1].parse().unwrap_or(0),
                    def: ev_parts[2].parse().unwrap_or(0),
                    spa: ev_parts[3].parse().unwrap_or(0),
                    spd: ev_parts[4].parse().unwrap_or(0),
                    spe: ev_parts[5].parse().unwrap_or(0),
                };
            }
        }

        // gender (7)
        set.gender = match parts[7] {
            "M" => Gender::Male,
            "F" => Gender::Female,
            _ => Gender::None,
        };

        // ivs (8)
        if parts.len() > 8 && !parts[8].is_empty() {
            let iv_parts: Vec<&str> = parts[8].split(',').collect();
            if iv_parts.len() >= 6 {
                set.ivs = StatsTable {
                    hp: if iv_parts[0].is_empty() { 31 } else { iv_parts[0].parse().unwrap_or(31) },
                    atk: if iv_parts[1].is_empty() { 31 } else { iv_parts[1].parse().unwrap_or(31) },
                    def: if iv_parts[2].is_empty() { 31 } else { iv_parts[2].parse().unwrap_or(31) },
                    spa: if iv_parts[3].is_empty() { 31 } else { iv_parts[3].parse().unwrap_or(31) },
                    spd: if iv_parts[4].is_empty() { 31 } else { iv_parts[4].parse().unwrap_or(31) },
                    spe: if iv_parts[5].is_empty() { 31 } else { iv_parts[5].parse().unwrap_or(31) },
                };
            }
        }

        // shiny (9)
        if parts.len() > 9 {
            set.shiny = parts[9] == "S";
        }

        // level (10)
        if parts.len() > 10 && !parts[10].is_empty() {
            set.level = parts[10].parse().unwrap_or(100);
        }

        // happiness and extended fields (11+)
        if parts.len() > 11 {
            let happiness_and_extended: Vec<&str> = parts[11].split(',').collect();
            if !happiness_and_extended[0].is_empty() {
                set.happiness = happiness_and_extended[0].parse().unwrap_or(255);
            }

            if happiness_and_extended.len() > 1 && !happiness_and_extended[1].is_empty() {
                set.hptype = Some(happiness_and_extended[1].to_string());
            }
            if happiness_and_extended.len() > 2 && !happiness_and_extended[2].is_empty() {
                set.pokeball = happiness_and_extended[2].to_string();
            }
            if happiness_and_extended.len() > 3 {
                set.gigantamax = happiness_and_extended[3] == "G";
            }
            if happiness_and_extended.len() > 4 && !happiness_and_extended[4].is_empty() {
                set.dynamax_level = happiness_and_extended[4].parse().unwrap_or(10);
            }
            if happiness_and_extended.len() > 5 && !happiness_and_extended[5].is_empty() {
                set.tera_type = Some(happiness_and_extended[5].to_string());
            }
        }

        team.push(set);
    }

    if team.is_empty() {
        None
    } else {
        Some(team)
    }
}

/// Export a team to the human-readable format
pub fn export_team(team: &[PokemonSet]) -> String {
    let mut output = String::new();

    for (i, set) in team.iter().enumerate() {
        if i > 0 {
            output.push('\n');
        }

        // First line
        if set.name != set.species && !set.name.is_empty() {
            output.push_str(&set.name);
            output.push_str(" (");
            output.push_str(&set.species);
            output.push(')');
        } else {
            output.push_str(&set.species);
        }

        if set.gender != Gender::None {
            output.push_str(" (");
            output.push_str(set.gender.to_str());
            output.push(')');
        }

        if !set.item.is_empty() {
            output.push_str(" @ ");
            output.push_str(&set.item);
        }
        output.push('\n');

        if !set.ability.is_empty() {
            output.push_str("Ability: ");
            output.push_str(&set.ability);
            output.push('\n');
        }

        if set.level != 100 {
            output.push_str("Level: ");
            output.push_str(&set.level.to_string());
            output.push('\n');
        }

        if set.shiny {
            output.push_str("Shiny: Yes\n");
        }

        if set.happiness != 255 {
            output.push_str("Happiness: ");
            output.push_str(&set.happiness.to_string());
            output.push('\n');
        }

        if let Some(ref tera) = set.tera_type {
            output.push_str("Tera Type: ");
            output.push_str(tera);
            output.push('\n');
        }

        // EVs
        let evs = &set.evs;
        if evs.hp != 0 || evs.atk != 0 || evs.def != 0 || evs.spa != 0 || evs.spd != 0 || evs.spe != 0 {
            output.push_str("EVs: ");
            let mut ev_parts = Vec::new();
            if evs.hp != 0 { ev_parts.push(format!("{} HP", evs.hp)); }
            if evs.atk != 0 { ev_parts.push(format!("{} Atk", evs.atk)); }
            if evs.def != 0 { ev_parts.push(format!("{} Def", evs.def)); }
            if evs.spa != 0 { ev_parts.push(format!("{} SpA", evs.spa)); }
            if evs.spd != 0 { ev_parts.push(format!("{} SpD", evs.spd)); }
            if evs.spe != 0 { ev_parts.push(format!("{} Spe", evs.spe)); }
            output.push_str(&ev_parts.join(" / "));
            output.push('\n');
        }

        if !set.nature.is_empty() && set.nature != "Hardy" {
            output.push_str(&set.nature);
            output.push_str(" Nature\n");
        }

        // IVs
        let ivs = &set.ivs;
        if ivs.hp != 31 || ivs.atk != 31 || ivs.def != 31 || ivs.spa != 31 || ivs.spd != 31 || ivs.spe != 31 {
            output.push_str("IVs: ");
            let mut iv_parts = Vec::new();
            if ivs.hp != 31 { iv_parts.push(format!("{} HP", ivs.hp)); }
            if ivs.atk != 31 { iv_parts.push(format!("{} Atk", ivs.atk)); }
            if ivs.def != 31 { iv_parts.push(format!("{} Def", ivs.def)); }
            if ivs.spa != 31 { iv_parts.push(format!("{} SpA", ivs.spa)); }
            if ivs.spd != 31 { iv_parts.push(format!("{} SpD", ivs.spd)); }
            if ivs.spe != 31 { iv_parts.push(format!("{} Spe", ivs.spe)); }
            output.push_str(&iv_parts.join(" / "));
            output.push('\n');
        }

        for mv in &set.moves {
            output.push_str("- ");
            output.push_str(mv);
            output.push('\n');
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_id() {
        assert_eq!(to_id("Pikachu"), "pikachu");
        assert_eq!(to_id("Life Orb"), "lifeorb");
        assert_eq!(to_id("Choice Scarf"), "choicescarf");
        assert_eq!(to_id("Tapu Koko"), "tapukoko");
    }

    #[test]
    fn test_pack_unpack_basic() {
        let team = vec![
            PokemonSet {
                name: "Pikachu".to_string(),
                species: "Pikachu".to_string(),
                item: "Light Ball".to_string(),
                ability: "Static".to_string(),
                moves: vec!["Thunderbolt".to_string(), "Volt Tackle".to_string()],
                nature: "Jolly".to_string(),
                level: 50,
                ..Default::default()
            },
        ];

        let packed = pack_team(&team);
        // Name is kept as-is, but species/item/ability are converted to IDs
        assert!(packed.contains("Pikachu")); // name stays as-is
        assert!(packed.contains("lightball"));
        assert!(packed.contains("static"));

        let unpacked = unpack_team(&packed).unwrap();
        assert_eq!(unpacked.len(), 1);
        assert_eq!(unpacked[0].ability, "static");
    }

    #[test]
    fn test_export_team() {
        let team = vec![
            PokemonSet {
                name: "Garchomp".to_string(),
                species: "Garchomp".to_string(),
                item: "Life Orb".to_string(),
                ability: "Rough Skin".to_string(),
                moves: vec!["Earthquake".to_string(), "Outrage".to_string()],
                nature: "Jolly".to_string(),
                evs: StatsTable { hp: 0, atk: 252, def: 0, spa: 0, spd: 4, spe: 252 },
                level: 50,
                ..Default::default()
            },
        ];

        let exported = export_team(&team);
        assert!(exported.contains("Garchomp @ Life Orb"));
        assert!(exported.contains("Ability: Rough Skin"));
        assert!(exported.contains("Level: 50"));
        assert!(exported.contains("252 Atk"));
        assert!(exported.contains("Jolly Nature"));
        assert!(exported.contains("- Earthquake"));
    }
}
