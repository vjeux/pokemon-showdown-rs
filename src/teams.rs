//! Teams
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Functions for converting and generating teams.

use crate::dex_data::{Gender, StatsTable};
use crate::pokemon::PokemonSet;

/// Options for exporting teams
#[derive(Debug, Clone, Default)]
pub struct ExportOptions {
    /// Hide stats (EVs/IVs/Nature)
    pub hide_stats: bool,
    /// Remove nicknames (use species name instead)
    pub remove_nicknames: bool,
}

/// Convert a name to ID format (lowercase, no spaces/special chars)
/// Equivalent to toID() in the TypeScript codebase
pub fn to_id(name: &str) -> String {
    name.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

/// Pack a name by removing non-alphanumeric characters (but keeping case)
/// Equivalent to Teams.packName() in teams.ts
// TypeScript source:
// /** Very similar to toID but without the lowercase conversion */
// 	packName(this: void, name: string | undefined | null) {
// 		if (!name) return '';
// 		return name.replace(/[^A-Za-z0-9]+/g, '');
// 	}
//
pub fn pack_name(name: &str) -> String {
    name.chars()
        .filter(|c| c.is_alphanumeric())
        .collect()
}

/// Unpack a name to a more readable format (best-effort without dex)
/// Equivalent to Teams.unpackName() in teams.ts
// TypeScript source:
// /** Will not entirely recover a packed name, but will be a pretty readable guess */
// 	unpackName(name: string, dexTable?: { get: (name: string) => AnyObject }) {
// 		if (!name) return '';
// 		if (dexTable) {
// 			const obj = dexTable.get(name);
// 			if (obj.exists) return obj.name;
// 		}
// 		return name.replace(/([0-9]+)/g, ' $1 ').replace(/([A-Z])/g, ' $1').replace(/[ ][ ]/g, ' ').trim();
// 	}
//
pub fn unpack_name(name: &str) -> String {
    if name.is_empty() {
        return String::new();
    }
    let mut result = String::new();
    let chars: Vec<char> = name.chars().collect();
    for (i, c) in chars.iter().enumerate() {
        if i > 0 {
            if c.is_uppercase() && chars[i - 1].is_lowercase() {
                result.push(' ');
            } else if c.is_numeric() && chars[i - 1].is_alphabetic() {
                result.push(' ');
            } else if c.is_alphabetic() && chars[i - 1].is_numeric() {
                result.push(' ');
            }
        }
        result.push(*c);
    }
    while result.contains("  ") {
        result = result.replace("  ", " ");
    }
    result.trim().to_string()
}

/// Pack a team into the compact string format
pub fn pack_team(team: &[PokemonSet]) -> String {
    let mut buf = String::new();

    for (i, set) in team.iter().enumerate() {
        if i > 0 {
            buf.push(']');
        }

        let name = if set.name.is_empty() { &set.species } else { &set.name };
        buf.push_str(name);

        let species_id = to_id(&set.species);
        let name_id = to_id(name);
        buf.push('|');
        if name_id != species_id {
            buf.push_str(&species_id);
        }

        buf.push('|');
        buf.push_str(&to_id(&set.item));

        buf.push('|');
        buf.push_str(&to_id(&set.ability));

        buf.push('|');
        let moves: Vec<String> = set.moves.iter().map(|m| to_id(m)).collect();
        buf.push_str(&moves.join(","));

        buf.push('|');
        if set.nature != "Hardy" && !set.nature.is_empty() {
            buf.push_str(&set.nature);
        }

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

        buf.push('|');
        buf.push_str(set.gender.to_str());

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

        buf.push('|');
        if set.shiny {
            buf.push('S');
        }

        buf.push('|');
        if set.level != 100 {
            buf.push_str(&set.level.to_string());
        }

        buf.push('|');
        if set.happiness != 255 {
            buf.push_str(&set.happiness.to_string());
        }

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

        set.name = parts[0].to_string();
        set.species = if parts[1].is_empty() {
            set.name.clone()
        } else {
            parts[1].to_string()
        };
        set.item = parts[2].to_string();
        set.ability = parts[3].to_string();
        set.moves = parts[4].split(',')
            .filter(|m| !m.is_empty())
            .map(|m| m.to_string())
            .collect();
        set.nature = if parts[5].is_empty() {
            "Hardy".to_string()
        } else {
            parts[5].to_string()
        };

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

        set.gender = match parts[7] {
            "M" => Gender::Male,
            "F" => Gender::Female,
            _ => Gender::None,
        };

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

        if parts.len() > 9 {
            set.shiny = parts[9] == "S";
        }

        if parts.len() > 10 && !parts[10].is_empty() {
            set.level = parts[10].parse().unwrap_or(100);
        }

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
    export_team_with_options(team, &ExportOptions::default())
}

/// Export a team with options
pub fn export_team_with_options(team: &[PokemonSet], options: &ExportOptions) -> String {
    let mut output = String::new();
    for set in team {
        output.push_str(&export_set(set, options));
        output.push('\n');
    }
    output
}

/// Export a single Pokemon set
// TypeScript source:
// 
// 
// 	exportSet(set: PokemonSet, { hideStats, removeNicknames }: ExportOptions = {}) {
// 		let out = ``;
// 
// 		// core
// 		if (typeof removeNicknames === 'function' && set.name && set.name !== set.species) {
// 			set.name = removeNicknames(set.name) || set.species;
// 		}
// 		if (set.name && set.name !== set.species && removeNicknames !== true) {
// 			out += `${set.name} (${set.species})`;
// 		} else {
// 			out += set.species;
// 		}
// 		if (set.gender === 'M') out += ` (M)`;
// 		if (set.gender === 'F') out += ` (F)`;
// 		if (set.item) out += ` @ ${set.item}`;
// 		out += `  \n`;
// 
// 		if (set.ability) {
// 			out += `Ability: ${set.ability}  \n`;
// 		}
// 
// 		// details
// 		if (set.level && set.level !== 100) {
// 			out += `Level: ${set.level}  \n`;
// 		}
// 		if (set.shiny) {
// 			out += `Shiny: Yes  \n`;
// 		}
// 		if (typeof set.happiness === 'number' && set.happiness !== 255 && !isNaN(set.happiness)) {
// 			out += `Happiness: ${set.happiness}  \n`;
// 		}
// 		if (set.pokeball) {
// 			out += `Pokeball: ${set.pokeball}  \n`;
// 		}
// 		if (set.hpType) {
// 			out += `Hidden Power: ${set.hpType}  \n`;
// 		}
// 		if (typeof set.dynamaxLevel === 'number' && set.dynamaxLevel !== 10 && !isNaN(set.dynamaxLevel)) {
// 			out += `Dynamax Level: ${set.dynamaxLevel}  \n`;
// 		}
// 		if (set.gigantamax) {
// 			out += `Gigantamax: Yes  \n`;
// 		}
// 		if (set.teraType) {
// 			out += `Tera Type: ${set.teraType}  \n`;
// 		}
// 
// 		// stats
// 		if (!hideStats) {
// 			if (set.evs) {
// 				const stats = Dex.stats.ids().map(
// 					stat => set.evs[stat] ?
// 						`${set.evs[stat]} ${Dex.stats.shortNames[stat]}` : ``
// 				).filter(Boolean);
// 				if (stats.length) {
// 					out += `EVs: ${stats.join(" / ")}  \n`;
// 				}
// 			}
// 			if (set.nature) {
// 				out += `${set.nature} Nature  \n`;
// 			}
// 			if (set.ivs) {
// 				const stats = Dex.stats.ids().map(
// 					stat => (set.ivs[stat] !== 31 && set.ivs[stat] !== undefined) ?
// 						`${set.ivs[stat] || 0} ${Dex.stats.shortNames[stat]}` : ``
// 				).filter(Boolean);
// 				if (stats.length) {
// 					out += `IVs: ${stats.join(" / ")}  \n`;
// 				}
// 			}
// 		}
// 
// 		// moves
// 		for (let move of set.moves) {
// 			if (move.startsWith(`Hidden Power `) && move.charAt(13) !== '[') {
// 				move = `Hidden Power [${move.slice(13)}]`;
// 			}
// 			out += `- ${move}  \n`;
// 		}
// 
// 		return out;
// 	}
//
pub fn export_set(set: &PokemonSet, options: &ExportOptions) -> String {
    let mut output = String::new();

    let display_name = if options.remove_nicknames || set.name.is_empty() || set.name == set.species {
        set.species.clone()
    } else {
        format!("{} ({})", set.name, set.species)
    };
    output.push_str(&display_name);

    if set.gender == Gender::Male {
        output.push_str(" (M)");
    } else if set.gender == Gender::Female {
        output.push_str(" (F)");
    }

    if !set.item.is_empty() {
        output.push_str(" @ ");
        output.push_str(&set.item);
    }
    output.push_str("  \n");

    if !set.ability.is_empty() {
        output.push_str("Ability: ");
        output.push_str(&set.ability);
        output.push_str("  \n");
    }

    if set.level != 100 {
        output.push_str("Level: ");
        output.push_str(&set.level.to_string());
        output.push_str("  \n");
    }

    if set.shiny {
        output.push_str("Shiny: Yes  \n");
    }

    if set.happiness != 255 {
        output.push_str("Happiness: ");
        output.push_str(&set.happiness.to_string());
        output.push_str("  \n");
    }

    if !set.pokeball.is_empty() {
        output.push_str("Pokeball: ");
        output.push_str(&set.pokeball);
        output.push_str("  \n");
    }

    if let Some(ref hptype) = set.hptype {
        output.push_str("Hidden Power: ");
        output.push_str(hptype);
        output.push_str("  \n");
    }

    if set.dynamax_level != 10 {
        output.push_str("Dynamax Level: ");
        output.push_str(&set.dynamax_level.to_string());
        output.push_str("  \n");
    }

    if set.gigantamax {
        output.push_str("Gigantamax: Yes  \n");
    }

    if let Some(ref tera) = set.tera_type {
        output.push_str("Tera Type: ");
        output.push_str(tera);
        output.push_str("  \n");
    }

    if !options.hide_stats {
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
            output.push_str("  \n");
        }

        if !set.nature.is_empty() && set.nature != "Hardy" {
            output.push_str(&set.nature);
            output.push_str(" Nature  \n");
        }

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
            output.push_str("  \n");
        }
    }

    for mv in &set.moves {
        let move_display = if mv.starts_with("Hidden Power ") && !mv.contains('[') {
            format!("Hidden Power [{}]", &mv[13..])
        } else {
            mv.clone()
        };
        output.push_str("- ");
        output.push_str(&move_display);
        output.push_str("  \n");
    }

    output
}

fn get_stat_id(stat_name: &str) -> Option<&'static str> {
    match stat_name.to_lowercase().as_str() {
        "hp" => Some("hp"),
        "atk" | "attack" => Some("atk"),
        "def" | "defense" => Some("def"),
        "spa" | "spatk" | "spattack" | "specialattack" => Some("spa"),
        "spd" | "spdef" | "spdefense" | "specialdefense" => Some("spd"),
        "spe" | "speed" => Some("spe"),
        _ => None,
    }
}

/// Parse a line from an exported team
// 
// 	parseExportedTeamLine(line: string, isFirstLine: boolean, set: PokemonSet, aggressive?: boolean) {
// 		if (isFirstLine) {
// 			let item;
// 			[line, item] = line.split(' @ ');
// 			if (item) {
// 				set.item = item;
// 				if (toID(set.item) === 'noitem') set.item = '';
// 			}
// 			if (line.endsWith(' (M)')) {
// 				set.gender = 'M';
// 				line = line.slice(0, -4);
// 			}
// 			if (line.endsWith(' (F)')) {
// 				set.gender = 'F';
// 				line = line.slice(0, -4);
// 			}
// 			if (line.endsWith(')') && line.includes('(')) {
// 				const [name, species] = line.slice(0, -1).split('(');
// 				set.species = Dex.species.get(species).name;
// 				set.name = name.trim();
// 			} else {
// 				set.species = Dex.species.get(line).name;
// 				set.name = '';
// 			}
// 		} else if (line.startsWith('Trait: ')) {
// 			line = line.slice(7);
// 			set.ability = aggressive ? toID(line) : line;
// 		} else if (line.startsWith('Ability: ')) {
// 			line = line.slice(9);
// 			set.ability = aggressive ? toID(line) : line;
// 		} else if (line === 'Shiny: Yes') {
// 			set.shiny = true;
// 		} else if (line.startsWith('Level: ')) {
// 			line = line.slice(7);
// 			set.level = +line;
// 		} else if (line.startsWith('Happiness: ')) {
// 			line = line.slice(11);
// 			set.happiness = +line;
// 		} else if (line.startsWith('Pokeball: ')) {
// 			line = line.slice(10);
// 			set.pokeball = aggressive ? toID(line) : line;
// 		} else if (line.startsWith('Hidden Power: ')) {
// 			line = line.slice(14);
// 			set.hpType = aggressive ? toID(line) : line;
// 		} else if (line.startsWith('Tera Type: ')) {
// 			line = line.slice(11);
// 			set.teraType = aggressive ? line.replace(/[^a-zA-Z0-9]/g, '') : line;
// 		} else if (line === 'Gigantamax: Yes') {
// 			set.gigantamax = true;
// 		} else if (line.startsWith('EVs: ')) {
// 			line = line.slice(5);
// 			const evLines = line.split('/');
// 			set.evs = { hp: 0, atk: 0, def: 0, spa: 0, spd: 0, spe: 0 };
// 			for (const evLine of evLines) {
// 				const [statValue, statName] = evLine.trim().split(' ');
// 				const statid = Dex.stats.getID(statName);
// 				if (!statid) continue;
// 				const value = parseInt(statValue);
// 				set.evs[statid] = value;
// 			}
// 		} else if (line.startsWith('IVs: ')) {
// 			line = line.slice(5);
// 			const ivLines = line.split('/');
// 			set.ivs = { hp: 31, atk: 31, def: 31, spa: 31, spd: 31, spe: 31 };
// 			for (const ivLine of ivLines) {
// 				const [statValue, statName] = ivLine.trim().split(' ');
// 				const statid = Dex.stats.getID(statName);
// 				if (!statid) continue;
// 				let value = parseInt(statValue);
// 				if (isNaN(value)) value = 31;
// 				set.ivs[statid] = value;
// 			}
// 		} else if (/^[A-Za-z]+ (N|n)ature/.test(line)) {
// 			let natureIndex = line.indexOf(' Nature');
// 			if (natureIndex === -1) natureIndex = line.indexOf(' nature');
// 			if (natureIndex === -1) return;
// 			line = line.substr(0, natureIndex);
// 			if (line !== 'undefined') set.nature = aggressive ? toID(line) : line;
// 		} else if (line.startsWith('-') || line.startsWith('~')) {
// 			line = line.slice(line.charAt(1) === ' ' ? 2 : 1);
// 			if (line.startsWith('Hidden Power [')) {
// 				const hpType = line.slice(14, -1);
// 				line = 'Hidden Power ' + hpType;
// 				if (!set.ivs && Dex.types.isName(hpType)) {
// 					set.ivs = { hp: 31, atk: 31, def: 31, spa: 31, spd: 31, spe: 31 };
// 					const hpIVs = Dex.types.get(hpType).HPivs || {};
// 					for (const statid in hpIVs) {
// 						set.ivs[statid as StatID] = hpIVs[statid as StatID]!;
// 					}
// 				}
// 			}
// 			if (line === 'Frustration' && set.happiness === undefined) {
// 				set.happiness = 0;
// 			}
// 			set.moves.push(line);
// 		}
// 	}
//
pub fn parse_exported_team_line(line: &str, is_first_line: bool, set: &mut PokemonSet, aggressive: bool) {
    let line = line.trim();

    if is_first_line {
        let mut remaining = line.to_string();

        if let Some(idx) = remaining.find(" @ ") {
            set.item = remaining[idx + 3..].to_string();
            remaining = remaining[..idx].to_string();
        }

        if remaining.ends_with(" (M)") {
            set.gender = Gender::Male;
            remaining = remaining[..remaining.len() - 4].to_string();
        } else if remaining.ends_with(" (F)") {
            set.gender = Gender::Female;
            remaining = remaining[..remaining.len() - 4].to_string();
        }

        if remaining.ends_with(')') && remaining.contains('(') {
            let paren_idx = remaining.rfind('(').unwrap();
            let species = remaining[paren_idx + 1..remaining.len() - 1].trim().to_string();
            let name = remaining[..paren_idx].trim().to_string();
            set.species = if aggressive { to_id(&species) } else { species };
            set.name = name;
        } else {
            set.species = if aggressive { to_id(&remaining) } else { remaining.clone() };
            set.name = String::new();
        }
    } else if line.starts_with("Trait: ") || line.starts_with("Ability: ") {
        let ability = if line.starts_with("Trait: ") { &line[7..] } else { &line[9..] };
        set.ability = if aggressive { to_id(ability) } else { ability.to_string() };
    } else if line == "Shiny: Yes" {
        set.shiny = true;
    } else if line.starts_with("Level: ") {
        set.level = line[7..].parse().unwrap_or(100);
    } else if line.starts_with("Happiness: ") {
        set.happiness = line[11..].parse().unwrap_or(255);
    } else if line.starts_with("Pokeball: ") {
        let pokeball = &line[10..];
        set.pokeball = if aggressive { to_id(pokeball) } else { pokeball.to_string() };
    } else if line.starts_with("Hidden Power: ") {
        let hptype = &line[14..];
        set.hptype = Some(if aggressive { to_id(hptype) } else { hptype.to_string() });
    } else if line.starts_with("Tera Type: ") {
        let tera = &line[11..];
        set.tera_type = Some(if aggressive {
            tera.chars().filter(|c| c.is_alphanumeric()).collect()
        } else {
            tera.to_string()
        });
    } else if line == "Gigantamax: Yes" {
        set.gigantamax = true;
    } else if line.starts_with("Dynamax Level: ") {
        set.dynamax_level = line[15..].parse().unwrap_or(10);
    } else if line.starts_with("EVs: ") {
        let ev_str = &line[5..];
        set.evs = StatsTable { hp: 0, atk: 0, def: 0, spa: 0, spd: 0, spe: 0 };
        for part in ev_str.split('/') {
            let part = part.trim();
            let parts: Vec<&str> = part.split_whitespace().collect();
            if parts.len() >= 2 {
                if let (Ok(value), Some(stat_id)) = (parts[0].parse::<i32>(), get_stat_id(parts[1])) {
                    match stat_id {
                        "hp" => set.evs.hp = value,
                        "atk" => set.evs.atk = value,
                        "def" => set.evs.def = value,
                        "spa" => set.evs.spa = value,
                        "spd" => set.evs.spd = value,
                        "spe" => set.evs.spe = value,
                        _ => {}
                    }
                }
            }
        }
    } else if line.starts_with("IVs: ") {
        let iv_str = &line[5..];
        set.ivs = StatsTable { hp: 31, atk: 31, def: 31, spa: 31, spd: 31, spe: 31 };
        for part in iv_str.split('/') {
            let part = part.trim();
            let parts: Vec<&str> = part.split_whitespace().collect();
            if parts.len() >= 2 {
                if let (Ok(value), Some(stat_id)) = (parts[0].parse::<i32>(), get_stat_id(parts[1])) {
                    match stat_id {
                        "hp" => set.ivs.hp = value,
                        "atk" => set.ivs.atk = value,
                        "def" => set.ivs.def = value,
                        "spa" => set.ivs.spa = value,
                        "spd" => set.ivs.spd = value,
                        "spe" => set.ivs.spe = value,
                        _ => {}
                    }
                }
            }
        }
    } else if line.contains(" Nature") || line.contains(" nature") {
        let nature_idx = line.find(" Nature").or_else(|| line.find(" nature"));
        if let Some(idx) = nature_idx {
            let nature = &line[..idx];
            if nature != "undefined" {
                set.nature = if aggressive { to_id(nature) } else { nature.to_string() };
            }
        }
    } else if line.starts_with('-') || line.starts_with('~') {
        let move_str = if line.chars().nth(1) == Some(' ') { &line[2..] } else { &line[1..] };
        let move_name = if move_str.starts_with("Hidden Power [") {
            let hp_type = &move_str[14..move_str.len() - 1];
            format!("Hidden Power {}", hp_type)
        } else {
            move_str.to_string()
        };
        if move_name == "Frustration" && set.happiness == 255 {
            set.happiness = 0;
        }
        set.moves.push(move_name);
    }
}

/// Import a team from any format (JSON, packed, or human-readable)
pub fn import_team(buffer: &str, aggressive: bool) -> Option<Vec<PokemonSet>> {
    let buffer = buffer.trim();

    if buffer.is_empty() {
        return Some(Vec::new());
    }

    if buffer.starts_with('[') {
        if let Ok(mut team) = serde_json::from_str::<Vec<PokemonSet>>(buffer) {
            if aggressive {
                for set in &mut team {
                    set.name = to_id(&set.name);
                    set.species = to_id(&set.species);
                    set.item = to_id(&set.item);
                    set.ability = to_id(&set.ability);
                    set.nature = to_id(&set.nature);
                    set.moves = set.moves.iter().map(|m| to_id(m)).collect();
                }
            }
            return Some(team);
        }
    }

    let lines: Vec<&str> = buffer.lines().collect();
    if lines.len() == 1 && lines[0].contains('|') {
        return unpack_team(lines[0]);
    }

    let mut sets: Vec<PokemonSet> = Vec::new();
    let mut cur_set: Option<PokemonSet> = None;

    for line in lines {
        let line = line.trim();

        if line.is_empty() || line == "---" {
            if let Some(set) = cur_set.take() {
                sets.push(set);
            }
        } else if line.starts_with("===") {
            if let Some(set) = cur_set.take() {
                sets.push(set);
            }
        } else if cur_set.is_none() {
            let mut set = PokemonSet {
                name: String::new(),
                species: String::new(),
                item: String::new(),
                ability: String::new(),
                gender: Gender::None,
                nature: String::new(),
                evs: StatsTable { hp: 0, atk: 0, def: 0, spa: 0, spd: 0, spe: 0 },
                ivs: StatsTable { hp: 31, atk: 31, def: 31, spa: 31, spd: 31, spe: 31 },
                level: 100,
                happiness: 255,
                shiny: false,
                pokeball: String::new(),
                hptype: None,
                dynamax_level: 10,
                gigantamax: false,
                tera_type: None,
                moves: Vec::new(),
            };
            parse_exported_team_line(line, true, &mut set, aggressive);
            cur_set = Some(set);
        } else {
            parse_exported_team_line(line, false, cur_set.as_mut().unwrap(), aggressive);
        }
    }

    if let Some(set) = cur_set {
        sets.push(set);
    }

    if sets.is_empty() {
        None
    } else {
        Some(sets)
    }
}

/// Stub for team generator - requires random-battles data modules
// 
// 	getGenerator(format: Format | string, seed: PRNG | PRNGSeed | null = null) {
// 		let TeamGenerator;
// 		format = Dex.formats.get(format);
// 		let mod = format.mod;
// 		if (format.mod === 'monkeyspaw') mod = 'gen9';
// 		const formatID = toID(format);
// 		if (mod === 'gen9ssb') {
// 			TeamGenerator = require(`../data/mods/gen9ssb/random-teams`).default;
// 		} else if (formatID.includes('gen9babyrandombattle')) {
// 			TeamGenerator = require(`../data/random-battles/gen9baby/teams`).default;
// 		} else if (formatID.includes('gen9randombattle') && format.ruleTable?.has('+pokemontag:cap')) {
// 			TeamGenerator = require(`../data/random-battles/gen9cap/teams`).default;
// 		} else if (formatID.includes('gen9freeforallrandombattle')) {
// 			TeamGenerator = require(`../data/random-battles/gen9ffa/teams`).default;
// 		} else {
// 			TeamGenerator = require(`../data/random-battles/${mod}/teams`).default;
// 		}
// 
// 		return new TeamGenerator(format, seed);
// 	}
//
pub fn get_generator(_format: &str, _seed: Option<u64>) -> Option<()> {
    None
}

/// Stub for team generation - requires random-battles data modules
// 
// 	generate(format: Format | string, options: PlayerOptions | null = null): PokemonSet[] {
// 		return this.getGenerator(format, options?.seed).getTeam(options);
// 	}
//
pub fn generate(_format: &str, _seed: Option<u64>) -> Option<Vec<PokemonSet>> {
    None
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
    fn test_pack_name() {
        assert_eq!(pack_name("Pikachu"), "Pikachu");
        assert_eq!(pack_name("Life Orb"), "LifeOrb");
        assert_eq!(pack_name("Choice Scarf"), "ChoiceScarf");
    }

    #[test]
    fn test_unpack_name() {
        assert_eq!(unpack_name("LifeOrb"), "Life Orb");
        assert_eq!(unpack_name("ChoiceScarf"), "Choice Scarf");
        assert_eq!(unpack_name("TapuKoko"), "Tapu Koko");
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
        assert!(packed.contains("Pikachu"));
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

    #[test]
    fn test_import_team() {
        let exported = r#"
Garchomp @ Life Orb
Ability: Rough Skin
Level: 50
EVs: 252 Atk / 4 SpD / 252 Spe
Jolly Nature
- Earthquake
- Outrage
"#;
        let team = import_team(exported, false).unwrap();
        assert_eq!(team.len(), 1);
        assert_eq!(team[0].species, "Garchomp");
        assert_eq!(team[0].item, "Life Orb");
        assert_eq!(team[0].ability, "Rough Skin");
        assert_eq!(team[0].level, 50);
        assert_eq!(team[0].evs.atk, 252);
        assert_eq!(team[0].evs.spe, 252);
        assert_eq!(team[0].nature, "Jolly");
        assert!(team[0].moves.contains(&"Earthquake".to_string()));
    }
}
