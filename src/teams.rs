//! Teams Module
//!
//! Handles team serialization and deserialization
//! Equivalent to teams.ts in Pokemon Showdown

use crate::{PokemonSet, Dex};
use crate::dex_data::{StatsTable, Gender};

pub struct Teams;

impl Teams {
    /// Pack a team into compact string format
    /// Equivalent to Teams.pack() in teams.ts (teams.ts:119-212)
    ///
    /// JavaScript (teams.ts:119-212):
    ///   pack(team: PokemonSet[] | null): string {
    ///     if (!team) return '';
    ///     // ... (see full implementation below)
    ///   }
    pub fn pack(team: &[PokemonSet]) -> String {
        // JS: if (!team) return '';
        if team.is_empty() {
            return String::new();
        }

        let mut buf = String::new();

        for set in team {
            // Separator between Pokemon
            // JS: if (buf) buf += ']';
            if !buf.is_empty() {
                buf.push(']');
            }

            // name
            // JS: buf += (set.name || set.species);
            buf.push_str(if set.name.is_empty() {
                &set.species
            } else {
                &set.name
            });

            // species
            // JS: const id = this.packName(set.species || set.name);
            // JS: buf += `|${this.packName(set.name || set.species) === id ? '' : id}`;
            let species_id = Self::pack_name(if set.species.is_empty() {
                &set.name
            } else {
                &set.species
            });
            let name_id = Self::pack_name(if set.name.is_empty() {
                &set.species
            } else {
                &set.name
            });
            buf.push('|');
            if name_id != species_id {
                buf.push_str(&species_id);
            }

            // item
            // JS: buf += `|${this.packName(set.item)}`;
            buf.push('|');
            buf.push_str(&Self::pack_name(&set.item));

            // ability
            // JS: buf += `|${this.packName(set.ability)}`;
            buf.push('|');
            buf.push_str(&Self::pack_name(&set.ability));

            // moves
            // JS: buf += '|' + set.moves.map(this.packName).join(',');
            buf.push('|');
            let packed_moves: Vec<String> = set.moves.iter()
                .map(|m| Self::pack_name(m))
                .collect();
            buf.push_str(&packed_moves.join(","));

            // nature
            // JS: buf += `|${set.nature || ''}`;
            buf.push('|');
            buf.push_str(&set.nature);

            // evs
            // JS: let evs = '|';
            // JS: if (set.evs) { evs = `|${set.evs['hp']||''},${...}` }
            // JS: if (evs === '|,,,,,') buf += '|'; else buf += evs;
            let evs_str = format!(
                "|{},{},{},{},{},{}",
                if set.evs.hp == 0 { String::new() } else { set.evs.hp.to_string() },
                if set.evs.atk == 0 { String::new() } else { set.evs.atk.to_string() },
                if set.evs.def == 0 { String::new() } else { set.evs.def.to_string() },
                if set.evs.spa == 0 { String::new() } else { set.evs.spa.to_string() },
                if set.evs.spd == 0 { String::new() } else { set.evs.spd.to_string() },
                if set.evs.spe == 0 { String::new() } else { set.evs.spe.to_string() }
            );
            if evs_str == "|,,,,," {
                buf.push('|');
            } else {
                buf.push_str(&evs_str);
            }

            // gender
            // JS: if (set.gender) buf += `|${set.gender}`; else buf += '|';
            buf.push('|');
            let gender_str = set.gender.to_str();
            if !gender_str.is_empty() {
                buf.push_str(gender_str);
            }

            // ivs
            // JS: let ivs = '|';
            // JS: if (set.ivs) { ivs = `|${getIv(set.ivs,'hp')},${...}` }
            // JS: if (ivs === '|,,,,,') buf += '|'; else buf += ivs;
            // Helper: getIv returns '' if iv is 31 or undefined, else returns the value
            fn get_iv(iv: i32) -> String {
                if iv == 31 {
                    String::new()
                } else {
                    iv.to_string()
                }
            }
            let ivs_str = format!(
                "|{},{},{},{},{},{}",
                get_iv(set.ivs.hp),
                get_iv(set.ivs.atk),
                get_iv(set.ivs.def),
                get_iv(set.ivs.spa),
                get_iv(set.ivs.spd),
                get_iv(set.ivs.spe)
            );
            if ivs_str == "|,,,,," {
                buf.push('|');
            } else {
                buf.push_str(&ivs_str);
            }

            // shiny
            // JS: if (set.shiny) buf += '|S'; else buf += '|';
            buf.push('|');
            if set.shiny {
                buf.push('S');
            }

            // level
            // JS: if (set.level && set.level !== 100) buf += `|${set.level}`; else buf += '|';
            buf.push('|');
            if set.level != 0 && set.level != 100 {
                buf.push_str(&set.level.to_string());
            }

            // happiness
            // JS: if (set.happiness !== undefined && set.happiness !== 255) buf += `|${set.happiness}`; else buf += '|';
            buf.push('|');
            if set.happiness != 0 && set.happiness != 255 {
                buf.push_str(&set.happiness.to_string());
            }

            // Optional fields (hpType, pokeball, gigantamax, dynamaxLevel, teraType)
            // JS: if (set.pokeball || set.hpType || set.gigantamax || (set.dynamaxLevel !== undefined && set.dynamaxLevel !== 10) || set.teraType) {
            let has_optional = set.pokeball != "pokeball" && !set.pokeball.is_empty()
                || set.hptype.is_some()
                || set.gigantamax
                || (set.dynamax_level != 0 && set.dynamax_level != 10)
                || set.tera_type.is_some();

            if has_optional {
                // hpType
                // JS: buf += `,${set.hpType || ''}`;
                buf.push(',');
                if let Some(ref hp_type) = set.hptype {
                    buf.push_str(hp_type);
                }

                // pokeball
                // JS: buf += `,${this.packName(set.pokeball || '')}`;
                buf.push(',');
                if set.pokeball != "pokeball" && !set.pokeball.is_empty() {
                    buf.push_str(&Self::pack_name(&set.pokeball));
                }

                // gigantamax
                // JS: buf += `,${set.gigantamax ? 'G' : ''}`;
                buf.push(',');
                if set.gigantamax {
                    buf.push('G');
                }

                // dynamaxLevel
                // JS: buf += `,${set.dynamaxLevel !== undefined && set.dynamaxLevel !== 10 ? set.dynamaxLevel : ''}`;
                buf.push(',');
                if set.dynamax_level != 0 && set.dynamax_level != 10 {
                    buf.push_str(&set.dynamax_level.to_string());
                }

                // teraType
                // JS: buf += `,${set.teraType || ''}`;
                buf.push(',');
                if let Some(ref tera_type) = set.tera_type {
                    buf.push_str(tera_type);
                }
            }
        }

        buf
    }

    /// Unpack a team from compact string format
    /// Equivalent to Teams.unpack() in teams.ts (teams.ts:214-345)
    ///
    /// JavaScript (teams.ts:214-345):
    ///   unpack(buf: string): PokemonSet[] | null {
    ///     if (!buf) return null;
    ///     // ... (see full implementation below)
    ///   }
    pub fn unpack(buf: &str, dex: &Dex) -> Option<Vec<PokemonSet>> {
        // JS: if (!buf) return null;
        if buf.is_empty() {
            return None;
        }

        // JS: if (buf.startsWith('[') && buf.endsWith(']')) {
        // JS:   try { buf = this.pack(JSON.parse(buf)); } catch { return null; }
        // JS: }
        // Note: Rust doesn't have JSON parsing of PokemonSet[] built-in
        // This handles JSON array format - skip for now, return None
        if buf.starts_with('[') && buf.ends_with(']') {
            // TODO: Implement JSON parsing if needed
            return None;
        }

        let mut team = Vec::new();
        let mut i = 0;

        // JS: for (let count = 0; count < 24; count++)
        // limit to 24 Pokemon
        for _ in 0..24 {
            let mut set = PokemonSet::default();

            // name
            // JS: j = buf.indexOf('|', i);
            // JS: if (j < 0) return null;
            // JS: set.name = buf.substring(i, j);
            let j = match buf[i..].find('|') {
                Some(pos) => i + pos,
                None => return Some(team), // End of buffer
            };
            set.name = buf[i..j].to_string();
            i = j + 1;

            // species
            // JS: j = buf.indexOf('|', i);
            // JS: if (j < 0) return null;
            // JS: set.species = this.unpackName(buf.substring(i, j), Dex.species) || set.name;
            let j = match buf[i..].find('|') {
                Some(pos) => i + pos,
                None => return Some(team),
            };
            let species_str = &buf[i..j];
            set.species = if species_str.is_empty() {
                set.name.clone()
            } else {
                Self::unpack_name(species_str, Some(|name: &str| {
                    let id = crate::dex_data::ID::new(name);
                    dex.species.get(&id)
                        .filter(|s| s.exists)
                        .map(|s| s.name.clone())
                }))
            };
            i = j + 1;

            // item
            // JS: j = buf.indexOf('|', i);
            // JS: if (j < 0) return null;
            // JS: set.item = this.unpackName(buf.substring(i, j), Dex.items);
            let j = match buf[i..].find('|') {
                Some(pos) => i + pos,
                None => return Some(team),
            };
            set.item = Self::unpack_name(&buf[i..j], Some(|name: &str| {
                let id = crate::dex_data::ID::new(name);
                dex.items.get(&id).map(|item| item.name.clone())
            }));
            i = j + 1;

            // ability
            // JS: j = buf.indexOf('|', i);
            // JS: if (j < 0) return null;
            // JS: const ability = buf.substring(i, j);
            // JS: const species = Dex.species.get(set.species);
            // JS: set.ability = ['', '0', '1', 'H', 'S'].includes(ability) ?
            // JS:   species.abilities[ability as '0' || '0'] || (ability === '' ? '' : '!!!ERROR!!!') :
            // JS:   this.unpackName(ability, Dex.abilities);
            let j = match buf[i..].find('|') {
                Some(pos) => i + pos,
                None => return Some(team),
            };
            let ability_str = &buf[i..j];
            set.ability = if matches!(ability_str, "" | "0" | "1" | "H" | "S") {
                // Special ability slot codes - would need species data to resolve
                // For now, use unpack_name as fallback
                Self::unpack_name(ability_str, Some(|name: &str| {
                    let id = crate::dex_data::ID::new(name);
                    dex.abilities.get(&id).map(|a| a.name.clone())
                }))
            } else {
                Self::unpack_name(ability_str, Some(|name: &str| {
                    let id = crate::dex_data::ID::new(name);
                    dex.abilities.get(&id).map(|a| a.name.clone())
                }))
            };
            i = j + 1;

            // moves
            // JS: j = buf.indexOf('|', i);
            // JS: if (j < 0) return null;
            // JS: set.moves = buf.substring(i, j).split(',', 24).map(name => this.unpackName(name, Dex.moves));
            let j = match buf[i..].find('|') {
                Some(pos) => i + pos,
                None => return Some(team),
            };
            set.moves = buf[i..j]
                .split(',')
                .take(24)
                .map(|name| Self::unpack_name(name, Some(|n: &str| {
                    let id = crate::dex_data::ID::new(n);
                    dex.moves.get(&id).map(|m| m.name.clone())
                })))
                .collect();
            i = j + 1;

            // nature
            // JS: j = buf.indexOf('|', i);
            // JS: if (j < 0) return null;
            // JS: set.nature = this.unpackName(buf.substring(i, j), Dex.natures);
            let j = match buf[i..].find('|') {
                Some(pos) => i + pos,
                None => return Some(team),
            };
            set.nature = Self::unpack_name(&buf[i..j], Some(|name: &str| {
                let id = crate::dex_data::ID::new(name);
                dex.natures.get(&id).map(|n| n.name.clone())
            }));
            i = j + 1;

            // evs
            // JS: j = buf.indexOf('|', i);
            // JS: if (j < 0) return null;
            // JS: if (j !== i) {
            // JS:   const evs = buf.substring(i, j).split(',', 6);
            // JS:   set.evs = { hp: Number(evs[0]) || 0, ... };
            // JS: }
            let j = match buf[i..].find('|') {
                Some(pos) => i + pos,
                None => return Some(team),
            };
            if j != i {
                let evs_parts: Vec<&str> = buf[i..j].split(',').take(6).collect();
                set.evs = StatsTable {
                    hp: evs_parts.get(0).and_then(|s| s.parse().ok()).unwrap_or(0),
                    atk: evs_parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0),
                    def: evs_parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0),
                    spa: evs_parts.get(3).and_then(|s| s.parse().ok()).unwrap_or(0),
                    spd: evs_parts.get(4).and_then(|s| s.parse().ok()).unwrap_or(0),
                    spe: evs_parts.get(5).and_then(|s| s.parse().ok()).unwrap_or(0),
                };
            }
            i = j + 1;

            // gender
            // JS: j = buf.indexOf('|', i);
            // JS: if (j < 0) return null;
            // JS: if (i !== j) set.gender = buf.substring(i, j);
            let j = match buf[i..].find('|') {
                Some(pos) => i + pos,
                None => return Some(team),
            };
            if i != j {
                set.gender = Gender::parse(&buf[i..j]);
            }
            i = j + 1;

            // ivs
            // JS: j = buf.indexOf('|', i);
            // JS: if (j < 0) return null;
            // JS: if (j !== i) {
            // JS:   const ivs = buf.substring(i, j).split(',', 6);
            // JS:   set.ivs = { hp: ivs[0] === '' ? 31 : Number(ivs[0]) || 0, ... };
            // JS: }
            let j = match buf[i..].find('|') {
                Some(pos) => i + pos,
                None => return Some(team),
            };
            if j != i {
                let ivs_parts: Vec<&str> = buf[i..j].split(',').take(6).collect();
                set.ivs = StatsTable {
                    hp: ivs_parts.get(0).map(|s| if s.is_empty() { 31 } else { s.parse().unwrap_or(0) }).unwrap_or(31),
                    atk: ivs_parts.get(1).map(|s| if s.is_empty() { 31 } else { s.parse().unwrap_or(0) }).unwrap_or(31),
                    def: ivs_parts.get(2).map(|s| if s.is_empty() { 31 } else { s.parse().unwrap_or(0) }).unwrap_or(31),
                    spa: ivs_parts.get(3).map(|s| if s.is_empty() { 31 } else { s.parse().unwrap_or(0) }).unwrap_or(31),
                    spd: ivs_parts.get(4).map(|s| if s.is_empty() { 31 } else { s.parse().unwrap_or(0) }).unwrap_or(31),
                    spe: ivs_parts.get(5).map(|s| if s.is_empty() { 31 } else { s.parse().unwrap_or(0) }).unwrap_or(31),
                };
            }
            i = j + 1;

            // shiny
            // JS: j = buf.indexOf('|', i);
            // JS: if (j < 0) return null;
            // JS: if (i !== j) set.shiny = true;
            let j = match buf[i..].find('|') {
                Some(pos) => i + pos,
                None => return Some(team),
            };
            if i != j {
                set.shiny = true;
            }
            i = j + 1;

            // level
            // JS: j = buf.indexOf('|', i);
            // JS: if (j < 0) return null;
            // JS: if (i !== j) set.level = parseInt(buf.substring(i, j));
            let j = match buf[i..].find('|') {
                Some(pos) => i + pos,
                None => return Some(team),
            };
            if i != j {
                set.level = buf[i..j].parse().unwrap_or(100);
            } else {
                set.level = 100;
            }
            i = j + 1;

            // happiness + optional fields
            // JS: j = buf.indexOf(']', i);
            // JS: let misc;
            // JS: if (j < 0) {
            // JS:   if (i < buf.length) misc = buf.substring(i).split(',', 6);
            // JS: } else {
            // JS:   if (i !== j) misc = buf.substring(i, j).split(',', 6);
            // JS: }
            let j = buf[i..].find(']').map(|pos| i + pos);
            let misc: Option<Vec<&str>> = if let Some(j_pos) = j {
                if i != j_pos {
                    Some(buf[i..j_pos].split(',').take(6).collect())
                } else {
                    None
                }
            } else if i < buf.len() {
                Some(buf[i..].split(',').take(6).collect())
            } else {
                None
            };

            // JS: if (misc) {
            // JS:   set.happiness = (misc[0] ? Number(misc[0]) : 255);
            // JS:   set.hpType = misc[1] || '';
            // JS:   set.pokeball = this.unpackName(misc[2] || '', Dex.items);
            // JS:   set.gigantamax = !!misc[3];
            // JS:   set.dynamaxLevel = (misc[4] ? Number(misc[4]) : 10);
            // JS:   set.teraType = misc[5];
            // JS: }
            if let Some(ref misc_parts) = misc {
                set.happiness = misc_parts.get(0)
                    .and_then(|s| if s.is_empty() { None } else { s.parse().ok() })
                    .unwrap_or(255);
                set.hptype = misc_parts.get(1)
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string());
                set.pokeball = misc_parts.get(2)
                    .map(|s| Self::unpack_name(s, Some(|name: &str| {
                        let id = crate::dex_data::ID::new(name);
                        dex.items.get(&id).map(|item| item.name.clone())
                    })))
                    .unwrap_or_else(|| "pokeball".to_string());
                set.gigantamax = misc_parts.get(3)
                    .map(|s| !s.is_empty())
                    .unwrap_or(false);
                set.dynamax_level = misc_parts.get(4)
                    .and_then(|s| if s.is_empty() { None } else { s.parse().ok() })
                    .unwrap_or(10);
                set.tera_type = misc_parts.get(5)
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string());
            } else {
                set.happiness = 255;
                set.pokeball = "pokeball".to_string();
                set.dynamax_level = 10;
            }

            team.push(set);

            // JS: if (j < 0) break;
            // JS: i = j + 1;
            if j.is_none() {
                break;
            }
            i = j.unwrap() + 1;
        }

        Some(team)
    }

    /// Pack a name by removing non-alphanumeric characters
    /// Equivalent to Teams.packName() in teams.ts (teams.ts:348-351)
    ///
    /// JavaScript (teams.ts:348-351):
    ///   packName(this: void, name: string | undefined | null) {
    ///     if (!name) return '';
    ///     return name.replace(/[^A-Za-z0-9]+/g, '');
    ///   }
    fn pack_name(name: &str) -> String {
        // JS: if (!name) return '';
        if name.is_empty() {
            return String::new();
        }

        // JS: return name.replace(/[^A-Za-z0-9]+/g, '');
        // Note: JavaScript preserves case, only removes non-alphanumeric
        name.chars()
            .filter(|c| c.is_alphanumeric())
            .collect()
    }

    /// Unpack a name using Dex lookup or heuristic formatting
    /// Equivalent to Teams.unpackName() in teams.ts (teams.ts:354-361)
    ///
    /// JavaScript (teams.ts:354-361):
    ///   unpackName(name: string, dexTable?: { get: (name: string) => AnyObject }) {
    ///     if (!name) return '';
    ///     if (dexTable) {
    ///       const obj = dexTable.get(name);
    ///       if (obj.exists) return obj.name;
    ///     }
    ///     return name.replace(/([0-9]+)/g, ' $1 ').replace(/([A-Z])/g, ' $1').replace(/[ ][ ]/g, ' ').trim();
    ///   }
    fn unpack_name<F>(name: &str, lookup_fn: Option<F>) -> String
    where
        F: Fn(&str) -> Option<String>
    {
        // JS: if (!name) return '';
        if name.is_empty() {
            return String::new();
        }

        // JS: if (dexTable) {
        // JS:   const obj = dexTable.get(name);
        // JS:   if (obj.exists) return obj.name;
        // JS: }
        if let Some(lookup) = lookup_fn {
            if let Some(canonical_name) = lookup(name) {
                return canonical_name;
            }
        }

        // JS: return name.replace(/([0-9]+)/g, ' $1 ').replace(/([A-Z])/g, ' $1').replace(/[ ][ ]/g, ' ').trim();
        // Heuristic: add spaces before numbers and uppercase letters
        let mut result = String::new();
        let mut prev_was_space = false;

        for ch in name.chars() {
            if ch.is_numeric() || ch.is_uppercase() {
                if !result.is_empty() && !prev_was_space {
                    result.push(' ');
                    prev_was_space = true;
                }
                result.push(ch);
                prev_was_space = false;
            } else {
                result.push(ch);
                prev_was_space = ch == ' ';
            }
        }

        result.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dex_data::StatsTable;

    #[test]
    fn test_pack_name() {
        assert_eq!(Teams::pack_name("Pikachu"), "Pikachu");
        assert_eq!(Teams::pack_name("Mr. Mime"), "MrMime");
        assert_eq!(Teams::pack_name("Farfetch'd"), "Farfetchd");
        assert_eq!(Teams::pack_name("Light Ball"), "LightBall");
        assert_eq!(Teams::pack_name(""), "");
    }

    #[test]
    fn test_pack_empty_team() {
        let team = vec![];
        assert_eq!(Teams::pack(&team), "");
    }

    #[test]
    fn test_pack_single_pokemon_basic() {
        let team = vec![PokemonSet {
            name: String::new(),
            species: "Pikachu".to_string(),
            item: "Light Ball".to_string(),
            ability: "Static".to_string(),
            moves: vec!["Thunderbolt".to_string(), "Quick Attack".to_string()],
            nature: "Jolly".to_string(),
            gender: crate::dex_data::Gender::Male,
            evs: StatsTable::default(),
            ivs: StatsTable { hp: 31, atk: 31, def: 31, spa: 31, spd: 31, spe: 31 },
            level: 100,
            shiny: false,
            happiness: 255,
            pokeball: "pokeball".to_string(),
            hptype: None,
            dynamax_level: 10,
            gigantamax: false,
            tera_type: None,
        }];

        let packed = Teams::pack(&team);
        // Format: Pikachu||LightBall|Static|Thunderbolt,QuickAttack|Jolly||||M|||||
        assert!(packed.starts_with("Pikachu|"));
        assert!(packed.contains("|LightBall|"));
        assert!(packed.contains("|Static|"));
        assert!(packed.contains("Thunderbolt,QuickAttack"));
        assert!(packed.contains("|Jolly|"));
    }

    #[test]
    fn test_pack_with_optional_fields() {
        let team = vec![PokemonSet {
            name: String::new(),
            species: "Charizard".to_string(),
            item: String::new(),
            ability: "Blaze".to_string(),
            moves: vec!["Flamethrower".to_string()],
            nature: "Timid".to_string(),
            gender: crate::dex_data::Gender::None,
            evs: StatsTable::default(),
            ivs: StatsTable { hp: 31, atk: 31, def: 31, spa: 31, spd: 31, spe: 31 },
            level: 100,
            shiny: false,
            happiness: 255,
            pokeball: "cherishball".to_string(),
            hptype: Some("Fire".to_string()),
            dynamax_level: 10,
            gigantamax: true,
            tera_type: Some("Dragon".to_string()),
        }];

        let packed = Teams::pack(&team);
        // Should have optional fields: ,Fire,cherishball,G,,Dragon
        assert!(packed.contains(",Fire,"));  // hpType
        assert!(packed.contains(",cherishball,"));  // pokeball
        assert!(packed.contains(",G,"));  // gigantamax
        assert!(packed.ends_with(",Dragon"));  // teraType
    }

    #[test]
    fn test_pack_multiple_pokemon() {
        let team = vec![
            PokemonSet {
                name: "Pika".to_string(),
                species: "Pikachu".to_string(),
                item: String::new(),
                ability: "Static".to_string(),
                moves: vec!["Thunderbolt".to_string()],
                nature: String::new(),
                gender: crate::dex_data::Gender::None,
                evs: StatsTable::default(),
                ivs: StatsTable { hp: 31, atk: 31, def: 31, spa: 31, spd: 31, spe: 31 },
                level: 100,
                shiny: false,
                happiness: 255,
                pokeball: "pokeball".to_string(),
                hptype: None,
                dynamax_level: 10,
                gigantamax: false,
                tera_type: None,
            },
            PokemonSet {
                name: String::new(),
                species: "Charizard".to_string(),
                item: String::new(),
                ability: "Blaze".to_string(),
                moves: vec!["Flamethrower".to_string()],
                nature: String::new(),
                gender: crate::dex_data::Gender::None,
                evs: StatsTable::default(),
                ivs: StatsTable { hp: 31, atk: 31, def: 31, spa: 31, spd: 31, spe: 31 },
                level: 100,
                shiny: false,
                happiness: 255,
                pokeball: "pokeball".to_string(),
                hptype: None,
                dynamax_level: 10,
                gigantamax: false,
                tera_type: None,
            },
        ];

        let packed = Teams::pack(&team);
        // Should have ] separator between Pokemon
        assert!(packed.contains("]"));
        assert!(packed.contains("Pika|"));
        assert!(packed.contains("]Charizard|"));
    }
}
