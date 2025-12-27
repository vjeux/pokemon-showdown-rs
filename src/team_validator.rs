//! Team Validator
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module handles team validation for battle formats,
//! ensuring teams comply with format rules.

use std::collections::{HashMap, HashSet};
use crate::dex_data::ID;
use crate::data::formats::{get_format, FormatDef};
use crate::data::species::get_species;
use crate::dex::Dex;

// =========================================================================
// PokemonSources - tracks possible ways to get a Pokemon with a given set
// Equivalent to PokemonSources class in team-validator.ts
// =========================================================================

/// Represents a set of possible ways to get a Pokemon with a given set.
///
/// `PokemonSources::new()` creates an empty set;
/// `PokemonSources::allow_all(gen)` allows all Pokemon from that gen.
///
/// The set is mainly stored as a Vec `sources`, but for sets that
/// could be sourced from anywhere (for instance, TM moves), we
/// instead just set `sources_before` to a number meaning "any
/// source at or before this gen is possible."
#[derive(Debug, Clone, Default)]
pub struct PokemonSources {
    /// A set of specific possible PokemonSources
    pub sources: Vec<String>,
    /// If nonzero: the set also contains all possible sources from
    /// this gen and earlier
    pub sources_before: u8,
    /// The set requires sources from this gen or later
    pub sources_after: u8,
    /// Whether hidden ability is required
    pub is_hidden: Option<bool>,
    /// Limited egg moves
    pub limited_egg_moves: Option<Vec<ID>>,
    /// Possibly limited egg moves
    pub possibly_limited_egg_moves: Option<Vec<ID>>,
    /// Tradeback limited egg moves
    pub tradeback_limited_egg_moves: Option<Vec<ID>>,
    /// Level up egg moves
    pub level_up_egg_moves: Option<Vec<ID>>,
    /// Pomeg egg moves
    pub pomeg_egg_moves: Option<Vec<ID>>,
    /// Sketch moves
    pub sketch_moves: Option<Vec<ID>>,
    /// Track Baton Pass for some formats
    pub baton_pass: Option<String>,
    /// Gigantamax capability
    pub gmaxed: Option<bool>,
    /// Hatch counter
    pub hatch_counter: Option<u8>,
}

impl PokemonSources {
    /// Create a new empty PokemonSources
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a PokemonSources that allows all sources from gen and before
    pub fn allow_all(gen: u8) -> Self {
        Self {
            sources_before: gen,
            ..Default::default()
        }
    }

    /// Get the size of the sources set
    // TypeScript source:
    // 
    // 	size() {
    // 		if (this.sourcesBefore) return Infinity;
    // 		return this.sources.length;
    // 	}
    //
    pub fn size(&self) -> usize {
        self.sources.len() + if self.sources_before > 0 { 1 } else { 0 }
    }

    /// Check if there are no possible sources
    pub fn is_empty(&self) -> bool {
        self.sources.is_empty() && self.sources_before == 0
    }

    /// Add a source
    // TypeScript source:
    // 
    // 	add(source: PokemonSource, limitedEggMove?: ID | null) {
    // 		if (this.sources[this.sources.length - 1] !== source) this.sources.push(source);
    // 		if (limitedEggMove) {
    // 			if (source.substr(0, 3) === '1ET') {
    // 				this.tradebackLimitedEggMoves = [limitedEggMove];
    // 			}
    // 		}
    // 		if (limitedEggMove && this.limitedEggMoves !== null) {
    // 			this.limitedEggMoves = [limitedEggMove];
    // 		} else if (limitedEggMove === null) {
    // 			this.limitedEggMoves = null;
    // 		}
    // 	}
    //
    pub fn add(&mut self, source: String) {
        if !self.sources.contains(&source) {
            self.sources.push(source);
        }
    }

    /// Add a source with a specific generation
    // TypeScript source:
    // 
    // 	addGen(sourceGen: number) {
    // 		this.sourcesBefore = Math.max(this.sourcesBefore, sourceGen);
    // 		this.limitedEggMoves = null;
    // 	}
    //
    pub fn add_gen(&mut self, source: &str, gen: u8) {
        let full_source = format!("{}{}", gen, source);
        self.add(full_source);
    }

    /// Check if a source is included
    pub fn has(&self, source: &str) -> bool {
        if self.sources.contains(&source.to_string()) {
            return true;
        }
        // Check if within sources_before
        if self.sources_before > 0 && !source.is_empty() {
            if let Some(first_char) = source.chars().next() {
                if let Some(gen) = first_char.to_digit(10) {
                    return (gen as u8) <= self.sources_before;
                }
            }
        }
        false
    }

    /// Mark all sources at or before gen as valid
    pub fn set_sources_before(&mut self, gen: u8) {
        if self.sources_before < gen {
            self.sources_before = gen;
        }
    }

    /// Mark that sources after gen are required
    pub fn set_sources_after(&mut self, gen: u8) {
        if self.sources_after < gen {
            self.sources_after = gen;
        }
    }

    /// Get all sources as a combined vector
    pub fn get_all(&self) -> Vec<String> {
        let mut all = self.sources.clone();
        if self.sources_before > 0 {
            for gen in 1..=self.sources_before {
                all.push(format!("{}G", gen));
            }
        }
        all
    }

    /// Intersect with another PokemonSources
    pub fn intersect(&mut self, other: &PokemonSources) {
        // Remove sources not in other
        self.sources.retain(|s| other.has(s));

        // Update sources_before to be the min
        if other.sources_before > 0 && self.sources_before > 0 {
            self.sources_before = self.sources_before.min(other.sources_before);
        } else if self.sources_before == 0 {
            // Keep as is
        } else if other.sources_before == 0 {
            self.sources_before = 0;
        }
    }

    /// Union with another PokemonSources
    pub fn union(&mut self, other: &PokemonSources) {
        for source in &other.sources {
            self.add(source.clone());
        }
        self.sources_before = self.sources_before.max(other.sources_before);
    }

    /// Clear all sources
    pub fn clear(&mut self) {
        self.sources.clear();
        self.sources_before = 0;
    }
}

/// Validation error type
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    /// Team has too many Pokemon
    TeamTooLarge { max: usize, actual: usize },
    /// Team has too few Pokemon
    TeamTooSmall { min: usize, actual: usize },
    /// Pokemon has too many moves
    TooManyMoves { pokemon: String, count: usize },
    /// Pokemon has no moves
    NoMoves { pokemon: String },
    /// Pokemon has duplicate moves
    DuplicateMove { pokemon: String, move_name: String },
    /// Pokemon species is banned
    BannedPokemon { pokemon: String },
    /// Move is banned
    BannedMove { pokemon: String, move_name: String },
    /// Ability is banned
    BannedAbility { pokemon: String, ability: String },
    /// Item is banned
    BannedItem { pokemon: String, item: String },
    /// Species clause violation (duplicate species)
    SpeciesClause { species: String },
    /// Item clause violation (duplicate items)
    ItemClause { item: String },
    /// Pokemon level too high
    LevelTooHigh { pokemon: String, level: u8, max: u8 },
    /// Invalid species
    InvalidSpecies { pokemon: String },
    /// Invalid move
    InvalidMove { pokemon: String, move_name: String },
    /// Invalid ability
    InvalidAbility { pokemon: String, ability: String },
    /// Invalid item
    InvalidItem { pokemon: String, item: String },
    /// Invalid EV spread (total > 510)
    InvalidEVs { pokemon: String, total: i32 },
    /// Single EV too high (> 252)
    EVTooHigh { pokemon: String, stat: String, value: i32 },
    /// Invalid IV (> 31)
    InvalidIV { pokemon: String, stat: String, value: i32 },
    /// Nickname too long
    NicknameTooLong { pokemon: String, length: usize },
    /// Duplicate nickname
    DuplicateNickname { nickname: String },
    /// Format not found
    FormatNotFound { format: String },
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::TeamTooLarge { max, actual } =>
                write!(f, "Team has {} Pokemon but max is {}", actual, max),
            ValidationError::TeamTooSmall { min, actual } =>
                write!(f, "Team has {} Pokemon but min is {}", actual, min),
            ValidationError::TooManyMoves { pokemon, count } =>
                write!(f, "{} has {} moves (max 4)", pokemon, count),
            ValidationError::NoMoves { pokemon } =>
                write!(f, "{} has no moves", pokemon),
            ValidationError::DuplicateMove { pokemon, move_name } =>
                write!(f, "{} has duplicate move: {}", pokemon, move_name),
            ValidationError::BannedPokemon { pokemon } =>
                write!(f, "{} is banned", pokemon),
            ValidationError::BannedMove { pokemon, move_name } =>
                write!(f, "{} has banned move: {}", pokemon, move_name),
            ValidationError::BannedAbility { pokemon, ability } =>
                write!(f, "{} has banned ability: {}", pokemon, ability),
            ValidationError::BannedItem { pokemon, item } =>
                write!(f, "{} has banned item: {}", pokemon, item),
            ValidationError::SpeciesClause { species } =>
                write!(f, "Species Clause: multiple {} on team", species),
            ValidationError::ItemClause { item } =>
                write!(f, "Item Clause: multiple Pokemon holding {}", item),
            ValidationError::LevelTooHigh { pokemon, level, max } =>
                write!(f, "{} is level {} but max is {}", pokemon, level, max),
            ValidationError::InvalidSpecies { pokemon } =>
                write!(f, "Invalid species: {}", pokemon),
            ValidationError::InvalidMove { pokemon, move_name } =>
                write!(f, "{} has invalid move: {}", pokemon, move_name),
            ValidationError::InvalidAbility { pokemon, ability } =>
                write!(f, "{} has invalid ability: {}", pokemon, ability),
            ValidationError::InvalidItem { pokemon, item } =>
                write!(f, "{} has invalid item: {}", pokemon, item),
            ValidationError::InvalidEVs { pokemon, total } =>
                write!(f, "{} has {} total EVs (max 510)", pokemon, total),
            ValidationError::EVTooHigh { pokemon, stat, value } =>
                write!(f, "{} has {} {} EVs (max 252)", pokemon, value, stat),
            ValidationError::InvalidIV { pokemon, stat, value } =>
                write!(f, "{} has {} {} IVs (max 31)", pokemon, value, stat),
            ValidationError::NicknameTooLong { pokemon, length } =>
                write!(f, "{} nickname is {} chars (max 18)", pokemon, length),
            ValidationError::DuplicateNickname { nickname } =>
                write!(f, "Duplicate nickname: {}", nickname),
            ValidationError::FormatNotFound { format } =>
                write!(f, "Format not found: {}", format),
        }
    }
}

/// Pokemon set for validation
#[derive(Debug, Clone)]
pub struct ValidatorSet {
    pub species: String,
    pub nickname: Option<String>,
    pub item: Option<String>,
    pub ability: String,
    pub moves: Vec<String>,
    pub nature: Option<String>,
    pub evs: EVSpread,
    pub ivs: IVSpread,
    pub level: u8,
    pub gender: Option<String>,
    pub shiny: bool,
    pub happiness: u8,
}

impl Default for ValidatorSet {
    fn default() -> Self {
        Self {
            species: String::new(),
            nickname: None,
            item: None,
            ability: String::new(),
            moves: Vec::new(),
            nature: None,
            evs: EVSpread::default(),
            ivs: IVSpread::default(),
            level: 100,
            gender: None,
            shiny: false,
            happiness: 255,
        }
    }
}

/// EV spread
#[derive(Debug, Clone, Copy, Default)]
pub struct EVSpread {
    pub hp: i32,
    pub atk: i32,
    pub def: i32,
    pub spa: i32,
    pub spd: i32,
    pub spe: i32,
}

impl EVSpread {
    pub fn total(&self) -> i32 {
        self.hp + self.atk + self.def + self.spa + self.spd + self.spe
    }

    pub fn is_valid(&self) -> bool {
        self.total() <= 510
            && self.hp <= 252
            && self.atk <= 252
            && self.def <= 252
            && self.spa <= 252
            && self.spd <= 252
            && self.spe <= 252
    }
}

/// IV spread
#[derive(Debug, Clone, Copy)]
pub struct IVSpread {
    pub hp: i32,
    pub atk: i32,
    pub def: i32,
    pub spa: i32,
    pub spd: i32,
    pub spe: i32,
}

impl Default for IVSpread {
    fn default() -> Self {
        Self {
            hp: 31,
            atk: 31,
            def: 31,
            spa: 31,
            spd: 31,
            spe: 31,
        }
    }
}

impl IVSpread {
    pub fn is_valid(&self) -> bool {
        self.hp <= 31
            && self.atk <= 31
            && self.def <= 31
            && self.spa <= 31
            && self.spd <= 31
            && self.spe <= 31
    }
}

/// Team validator
pub struct TeamValidator {
    format_id: ID,
    format: Option<&'static FormatDef>,
    dex: Dex,
}

impl TeamValidator {
    /// Create a new team validator for a format
    pub fn new(format: &str) -> Self {
        let format_id = ID::new(format);
        let format_def = get_format(&format_id);
        let dex = Dex::load_default().expect("Failed to load dex");
        Self {
            format_id,
            format: format_def,
            dex,
        }
    }

    /// Validate a team
    // TypeScript source:
    // 
    // 
    // 	validateTeam(
    // 		team: PokemonSet[] | null,
    // 		options: {
    // 			removeNicknames?: boolean,
    // 			skipSets?: { [name: string]: { [key: string]: boolean } },
    // 		} = {}
    // 	): string[] | null {
    // 		if (team && this.format.validateTeam) {
    // 			return this.format.validateTeam.call(this, team, options) || null;
    // 		}
    // 		return this.baseValidateTeam(team, options);
    // 	}
    //
    pub fn validate_team(&self, team: &[ValidatorSet]) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        // Check format exists
        let format = match self.format {
            Some(f) => f,
            None => {
                errors.push(ValidationError::FormatNotFound {
                    format: self.format_id.as_str().to_string(),
                });
                return errors;
            }
        };

        // Check team size
        if team.len() > format.team_size {
            errors.push(ValidationError::TeamTooLarge {
                max: format.team_size,
                actual: team.len(),
            });
        }
        if team.len() < format.min_team_size {
            errors.push(ValidationError::TeamTooSmall {
                min: format.min_team_size,
                actual: team.len(),
            });
        }

        // Track for clause checking
        let mut seen_species: HashMap<String, usize> = HashMap::new();
        let mut seen_items: HashMap<String, usize> = HashMap::new();
        let mut seen_nicknames: HashSet<String> = HashSet::new();

        // Check Species Clause
        let has_species_clause = format.rulesets.contains(&"Species Clause");
        let has_item_clause = format.rulesets.contains(&"Item Clause");
        let has_nickname_clause = format.rulesets.contains(&"Nickname Clause");

        for (i, pokemon) in team.iter().enumerate() {
            // Validate individual Pokemon
            errors.extend(self.validate_pokemon(pokemon, format));

            // Species clause
            let species_id = ID::new(&pokemon.species).as_str().to_string();
            *seen_species.entry(species_id.clone()).or_insert(0) += 1;
            if has_species_clause && seen_species[&species_id] > 1 {
                errors.push(ValidationError::SpeciesClause {
                    species: pokemon.species.clone(),
                });
            }

            // Item clause
            if let Some(ref item) = pokemon.item {
                let item_id = ID::new(item).as_str().to_string();
                if !item_id.is_empty() {
                    *seen_items.entry(item_id.clone()).or_insert(0) += 1;
                    if has_item_clause && seen_items[&item_id] > 1 {
                        errors.push(ValidationError::ItemClause {
                            item: item.clone(),
                        });
                    }
                }
            }

            // Nickname clause
            let nickname = pokemon.nickname.as_ref().unwrap_or(&pokemon.species);
            if has_nickname_clause {
                let nick_lower = nickname.to_lowercase();
                if seen_nicknames.contains(&nick_lower) {
                    errors.push(ValidationError::DuplicateNickname {
                        nickname: nickname.clone(),
                    });
                }
                seen_nicknames.insert(nick_lower);
            }
        }

        errors
    }

    /// Validate a single Pokemon
    fn validate_pokemon(&self, pokemon: &ValidatorSet, format: &FormatDef) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        let pokemon_name = pokemon.nickname.as_ref().unwrap_or(&pokemon.species).clone();

        // Check species exists
        let species_id = ID::new(&pokemon.species);
        if get_species(&species_id).is_none() {
            errors.push(ValidationError::InvalidSpecies {
                pokemon: pokemon_name.clone(),
            });
        }

        // Check species not banned
        if self.is_banned(&pokemon.species) {
            errors.push(ValidationError::BannedPokemon {
                pokemon: pokemon_name.clone(),
            });
        }

        // Check level
        if pokemon.level > format.max_level {
            errors.push(ValidationError::LevelTooHigh {
                pokemon: pokemon_name.clone(),
                level: pokemon.level,
                max: format.max_level,
            });
        }

        // Check moves
        if pokemon.moves.is_empty() {
            errors.push(ValidationError::NoMoves {
                pokemon: pokemon_name.clone(),
            });
        }
        if pokemon.moves.len() > 4 {
            errors.push(ValidationError::TooManyMoves {
                pokemon: pokemon_name.clone(),
                count: pokemon.moves.len(),
            });
        }

        // Check for duplicate moves and banned moves
        let mut seen_moves: HashSet<String> = HashSet::new();
        for move_name in &pokemon.moves {
            let move_id = ID::new(move_name);
            let move_id_str = move_id.as_str().to_string();

            // Check move exists
            // TODO: Re-enable when Dex is available in validator
            // if get_move(&move_id).is_none() {
            //     errors.push(ValidationError::InvalidMove {
            //         pokemon: pokemon_name.clone(),
            //         move_name: move_name.clone(),
            //     });
            // }

            // Check duplicate
            if seen_moves.contains(&move_id_str) {
                errors.push(ValidationError::DuplicateMove {
                    pokemon: pokemon_name.clone(),
                    move_name: move_name.clone(),
                });
            }
            seen_moves.insert(move_id_str);

            // Check banned
            if self.is_banned(move_name) {
                errors.push(ValidationError::BannedMove {
                    pokemon: pokemon_name.clone(),
                    move_name: move_name.clone(),
                });
            }
        }

        // Check ability
        let ability_id = ID::new(&pokemon.ability);
        if self.dex.get_ability(&pokemon.ability).is_none() && !pokemon.ability.is_empty() {
            errors.push(ValidationError::InvalidAbility {
                pokemon: pokemon_name.clone(),
                ability: pokemon.ability.clone(),
            });
        }
        if self.is_banned(&pokemon.ability) {
            errors.push(ValidationError::BannedAbility {
                pokemon: pokemon_name.clone(),
                ability: pokemon.ability.clone(),
            });
        }

        // Check item
        if let Some(ref item) = pokemon.item {
            let item_id = ID::new(item);
            if self.dex.get_item(item).is_none() && !item.is_empty() {
                errors.push(ValidationError::InvalidItem {
                    pokemon: pokemon_name.clone(),
                    item: item.clone(),
                });
            }
            if self.is_banned(item) {
                errors.push(ValidationError::BannedItem {
                    pokemon: pokemon_name.clone(),
                    item: item.clone(),
                });
            }
        }

        // Check EVs
        if pokemon.evs.total() > 510 {
            errors.push(ValidationError::InvalidEVs {
                pokemon: pokemon_name.clone(),
                total: pokemon.evs.total(),
            });
        }
        if pokemon.evs.hp > 252 {
            errors.push(ValidationError::EVTooHigh {
                pokemon: pokemon_name.clone(),
                stat: "HP".to_string(),
                value: pokemon.evs.hp,
            });
        }
        if pokemon.evs.atk > 252 {
            errors.push(ValidationError::EVTooHigh {
                pokemon: pokemon_name.clone(),
                stat: "Atk".to_string(),
                value: pokemon.evs.atk,
            });
        }
        if pokemon.evs.def > 252 {
            errors.push(ValidationError::EVTooHigh {
                pokemon: pokemon_name.clone(),
                stat: "Def".to_string(),
                value: pokemon.evs.def,
            });
        }
        if pokemon.evs.spa > 252 {
            errors.push(ValidationError::EVTooHigh {
                pokemon: pokemon_name.clone(),
                stat: "SpA".to_string(),
                value: pokemon.evs.spa,
            });
        }
        if pokemon.evs.spd > 252 {
            errors.push(ValidationError::EVTooHigh {
                pokemon: pokemon_name.clone(),
                stat: "SpD".to_string(),
                value: pokemon.evs.spd,
            });
        }
        if pokemon.evs.spe > 252 {
            errors.push(ValidationError::EVTooHigh {
                pokemon: pokemon_name.clone(),
                stat: "Spe".to_string(),
                value: pokemon.evs.spe,
            });
        }

        // Check IVs
        if pokemon.ivs.hp > 31 {
            errors.push(ValidationError::InvalidIV {
                pokemon: pokemon_name.clone(),
                stat: "HP".to_string(),
                value: pokemon.ivs.hp,
            });
        }
        if pokemon.ivs.atk > 31 {
            errors.push(ValidationError::InvalidIV {
                pokemon: pokemon_name.clone(),
                stat: "Atk".to_string(),
                value: pokemon.ivs.atk,
            });
        }
        if pokemon.ivs.def > 31 {
            errors.push(ValidationError::InvalidIV {
                pokemon: pokemon_name.clone(),
                stat: "Def".to_string(),
                value: pokemon.ivs.def,
            });
        }
        if pokemon.ivs.spa > 31 {
            errors.push(ValidationError::InvalidIV {
                pokemon: pokemon_name.clone(),
                stat: "SpA".to_string(),
                value: pokemon.ivs.spa,
            });
        }
        if pokemon.ivs.spd > 31 {
            errors.push(ValidationError::InvalidIV {
                pokemon: pokemon_name.clone(),
                stat: "SpD".to_string(),
                value: pokemon.ivs.spd,
            });
        }
        if pokemon.ivs.spe > 31 {
            errors.push(ValidationError::InvalidIV {
                pokemon: pokemon_name.clone(),
                stat: "Spe".to_string(),
                value: pokemon.ivs.spe,
            });
        }

        // Check nickname length
        if let Some(ref nickname) = pokemon.nickname {
            if nickname.len() > 18 {
                errors.push(ValidationError::NicknameTooLong {
                    pokemon: pokemon_name.clone(),
                    length: nickname.len(),
                });
            }
        }

        errors
    }

    /// Check if something is banned in this format
    fn is_banned(&self, thing: &str) -> bool {
        if let Some(format) = self.format {
            let thing_lower = thing.to_lowercase();
            let thing_id = ID::new(thing).as_str().to_string();
            format.bans.iter().any(|b| {
                let b_lower = b.to_lowercase();
                let b_id = ID::new(b).as_str().to_string();
                b_lower == thing_lower || b_id == thing_id
            })
        } else {
            false
        }
    }

    // =========================================================================
    // Additional methods ported from team-validator.ts
    // =========================================================================

    /// Get event-only data for a species
    /// Equivalent to TeamValidator.getEventOnlyData() in team-validator.ts
    /// Returns None if the species isn't event-only
    // TypeScript source:
    // 
    // 
    // 	getEventOnlyData(species: Species, noRecurse?: boolean): { species: Species, eventData: EventInfo[] } | null {
    // 		const dex = this.dex;
    // 		const learnset = dex.species.getLearnsetData(species.id);
    // 		if (!learnset?.eventOnly) {
    // 			if (noRecurse) return null;
    // 			return this.getEventOnlyData(dex.species.get(species.prevo), true);
    // 		}
    // 
    // 		if (!learnset.eventData && species.forme) {
    // 			return this.getEventOnlyData(dex.species.get(species.baseSpecies), true);
    // 		}
    // 		if (!learnset.eventData) {
    // 			throw new Error(`Event-only species ${species.name} has no eventData table`);
    // 		}
    // 
    // 		return { species, eventData: learnset.eventData };
    // 	}
    //
    pub fn get_event_only_data(&self, species: &str) -> Option<Vec<String>> {
        // Stub implementation - would need event data to fully implement
        // In practice, this would look up the species' eventData
        let _ = species;
        None
    }

    /// Get the validation species for a set
    /// Handles form differences between what's used in battle vs what's validated
    /// Equivalent to TeamValidator.getValidationSpecies() in team-validator.ts
    // TypeScript source:
    // 
    // 
    // 	getValidationSpecies(set: PokemonSet): { outOfBattleSpecies: Species, tierSpecies: Species } {
    // 		const dex = this.dex;
    // 		const ruleTable = this.ruleTable;
    // 		const species = dex.species.get(set.species);
    // 		const item = dex.items.get(set.item);
    // 		const ability = dex.abilities.get(set.ability);
    // 
    // 		let outOfBattleSpecies = species;
    // 		let tierSpecies = species;
    // 		if (ability.id === 'battlebond' && toID(species.baseSpecies) === 'greninja' &&
    // 			this.format.mod !== 'gen9legendsou') {
    // 			outOfBattleSpecies = dex.species.get('greninjabond');
    // 			if (ruleTable.has('obtainableformes')) {
    // 				tierSpecies = outOfBattleSpecies;
    // 			}
    // 		}
    // 		if (ability.id === 'owntempo' && toID(species.baseSpecies) === 'rockruff') {
    // 			outOfBattleSpecies = dex.species.get('rockruffdusk');
    // 			if (ruleTable.has('obtainableformes')) {
    // 				tierSpecies = outOfBattleSpecies;
    // 			}
    // 		}
    // 
    // 		if (ruleTable.has('obtainableformes')) {
    // 			const canMegaEvo = dex.gen <= 7 || ruleTable.has('+pokemontag:past');
    // 			if (item.megaEvolves?.includes(species.name)) {
    // 				if (!item.megaStone) throw new Error(`Item ${item.name} has no base form for mega evolution`);
    // 				if (Array.isArray(item.megaEvolves)) {
    // 					const idx = item.megaEvolves.indexOf(species.name);
    // 					tierSpecies = dex.species.get(item.megaStone[idx]);
    // 				} else {
    // 					tierSpecies = dex.species.get(item.megaStone as string);
    // 				}
    // 			} else if (item.id === 'redorb' && species.id === 'groudon') {
    // 				tierSpecies = dex.species.get('Groudon-Primal');
    // 			} else if (item.id === 'blueorb' && species.id === 'kyogre') {
    // 				tierSpecies = dex.species.get('Kyogre-Primal');
    // 			} else if (
    // 				canMegaEvo && species.id === 'rayquaza' && set.moves.map(toID).includes('dragonascent' as ID) &&
    // 				!ruleTable.has('megarayquazaclause')
    // 			) {
    // 				tierSpecies = dex.species.get('Rayquaza-Mega');
    // 			} else if (item.id === 'rustedsword' && species.id === 'zacian') {
    // 				tierSpecies = dex.species.get('Zacian-Crowned');
    // 			} else if (item.id === 'rustedshield' && species.id === 'zamazenta') {
    // 				tierSpecies = dex.species.get('Zamazenta-Crowned');
    // 			}
    // 		}
    // 
    // 		return { outOfBattleSpecies, tierSpecies };
    // 	}
    //
    pub fn get_validation_species(&self, species: &str) -> (String, String) {
        // For now, return the same species for both
        // Full implementation would handle formes like Zygarde-Complete
        let species_id = ID::new(species).as_str().to_string();
        (species_id.clone(), species_id)
    }

    /// Validate moves for a Pokemon
    /// Equivalent to TeamValidator.validateMoves() in team-validator.ts
    // TypeScript source:
    // 
    // 
    // 	validateMoves(
    // 		species: Species, moves: string[], setSources: PokemonSources, set?: Partial<PokemonSet>,
    // 		name: string = species.name, moveLegalityWhitelist: { [k: string]: true | undefined } = {}
    // 	) {
    // 		const dex = this.dex;
    // 		const ruleTable = this.ruleTable;
    // 
    // 		const problems = [];
    // 
    // 		const checkCanLearn = (ruleTable.checkCanLearn?.[0] || this.checkCanLearn);
    // 		for (const moveName of moves) {
    // 			const move = dex.moves.get(moveName);
    // 			if (moveLegalityWhitelist[move.id]) continue;
    // 			const problem = checkCanLearn.call(this, move, species, setSources, set);
    // 			if (problem) {
    // 				problems.push(`${name}${problem}`);
    // 				break;
    // 			}
    // 		}
    // 
    // 		if (setSources.size() && setSources.moveEvoCarryCount > 3) {
    // 			if (setSources.sourcesBefore < 6) setSources.sourcesBefore = 0;
    // 			setSources.sources = setSources.sources.filter(
    // 				source => source.charAt(1) === 'E' && parseInt(source.charAt(0)) >= 6
    // 			);
    // 			if (!setSources.size()) {
    // 				problems.push(`${name} needs to know ${species.evoMove || 'a Fairy-type move'} to evolve, so it can only know 3 other moves from ${species.prevo}.`);
    // 			}
    // 		}
    // 
    // 		if (problems.length) return problems;
    // 
    // 		if (setSources.isHidden) {
    // 			setSources.sources = setSources.sources.filter(
    // 				source => parseInt(source.charAt(0)) >= 5
    // 			);
    // 			if (setSources.sourcesBefore < 5) setSources.sourcesBefore = 0;
    // 			const canUseAbilityPatch = dex.gen >= 8 && this.format.mod !== 'gen8dlc1';
    // 			if (!setSources.size() && !canUseAbilityPatch) {
    // 				problems.push(`${name} has a hidden ability - it can't have moves only learned before gen 5.`);
    // 				return problems;
    // 			}
    // 		}
    // 
    // 		if (setSources.babyOnly && setSources.sources.length) {
    // 			const baby = dex.species.get(setSources.babyOnly);
    // 			const babyEvo = toID(baby.evos[0]);
    // 			setSources.sources = setSources.sources.filter(source => {
    // 				if (source.charAt(1) === 'S') {
    // 					const sourceId = source.split(' ')[1];
    // 					if (sourceId !== baby.id) return false;
    // 				}
    // 				if (source.charAt(1) === 'E') {
    // 					if (babyEvo && source.slice(2) === babyEvo) return false;
    // 				}
    // 				if (source.charAt(1) === 'D') {
    // 					if (babyEvo && source.slice(2) === babyEvo) return false;
    // 				}
    // 				return true;
    // 			});
    // 			if (!setSources.size()) {
    // 				problems.push(`${name}'s event/egg moves are from an evolution, and are incompatible with its moves from ${baby.name}.`);
    // 			}
    // 		}
    // 		if (setSources.babyOnly && setSources.size() && this.gen > 2) {
    // 			// there do theoretically exist evo/tradeback incompatibilities in
    // 			// gen 2, but those are very complicated to validate and should be
    // 			// handled separately anyway, so for now we just treat them all as
    // 			// legal (competitively relevant ones can be manually banned)
    // 			const baby = dex.species.get(setSources.babyOnly);
    // 			setSources.sources = setSources.sources.filter(source => {
    // 				if (baby.gen > parseInt(source.charAt(0)) && !source.startsWith('1ST')) return false;
    // 				if (baby.gen > 2 && source === '7V') return false;
    // 				return true;
    // 			});
    // 			if (setSources.sourcesBefore < baby.gen) setSources.sourcesBefore = 0;
    // 			if (!setSources.size()) {
    // 				problems.push(`${name} has moves from before Gen ${baby.gen}, which are incompatible with its moves from ${baby.name}.`);
    // 			}
    // 		}
    // 
    // 		return problems;
    // 	}
    //
    pub fn validate_moves(&self, pokemon: &ValidatorSet) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        let pokemon_name = pokemon.nickname.as_ref().unwrap_or(&pokemon.species).clone();

        // Check move count
        if pokemon.moves.len() > 4 {
            errors.push(ValidationError::TooManyMoves {
                pokemon: pokemon_name.clone(),
                count: pokemon.moves.len(),
            });
        }

        // Check for duplicates
        let mut seen: HashSet<String> = HashSet::new();
        for move_name in &pokemon.moves {
            let move_id = ID::new(move_name).as_str().to_string();
            if seen.contains(&move_id) {
                errors.push(ValidationError::DuplicateMove {
                    pokemon: pokemon_name.clone(),
                    move_name: move_name.clone(),
                });
            }
            seen.insert(move_id);

            // Check if banned
            if self.is_banned(move_name) {
                errors.push(ValidationError::BannedMove {
                    pokemon: pokemon_name.clone(),
                    move_name: move_name.clone(),
                });
            }
        }

        errors
    }

    /// Check if a Pokemon can learn a move
    /// Equivalent to TeamValidator.checkCanLearn() in team-validator.ts
    ///
    /// This is a simplified implementation. The full TypeScript version
    /// handles complex learnset inheritance, event-only moves, egg moves,
    /// transfer moves, etc.
    ///
    /// Returns None if the move can be learned, or an error message if not
    // TypeScript source:
    // /** Returns null if you can learn the move, or a string explaining why you can't learn it */
    // 	checkCanLearn(
    // 		move: Move,
    // 		originalSpecies: Species,
    // 		setSources = this.allSources(originalSpecies),
    // 		set: Partial<PokemonSet> = {}
    // 	): string | null {
    // 		const dex = this.dex;
    // 		if (!setSources.size()) throw new Error(`Bad sources passed to checkCanLearn`);
    // 
    // 		move = dex.moves.get(move);
    // 		const moveid = move.id;
    // 		const baseSpecies = dex.species.get(originalSpecies);
    // 
    // 		const format = this.format;
    // 		const ruleTable = this.ruleTable;
    // 		const level = set.level || 100;
    // 		const canLearnSpecies: ID[] = [];
    // 
    // 		let cantLearnReason = null;
    // 
    // 		let limit1 = true;
    // 		let sketch = false;
    // 		let blockedHM = false;
    // 
    // 		let babyOnly = '';
    // 		let minLearnGen = dex.gen;
    // 
    // 		// This is a pretty complicated algorithm
    // 
    // 		// Abstractly, what it does is construct the union of sets of all
    // 		// possible ways this pokemon could be obtained, and then intersect
    // 		// it with a the pokemon's existing set of all possible ways it could
    // 		// be obtained. If this intersection is non-empty, the move is legal.
    // 
    // 		// set of possible sources of a pokemon with this move
    // 		const moveSources = new PokemonSources();
    // 
    // 		/**
    // 		 * The format doesn't allow Pokemon traded from the future
    // 		 * (This is everything except in Gen 1 Tradeback)
    // 		 */
    // 		const noFutureGen = !ruleTable.has('allowtradeback');
    // 		/**
    // 		 * The format allows Sketch to copy moves in Gen 8
    // 		 */
    // 		const canSketchPostGen7Moves = ruleTable.has('sketchpostgen7moves') || this.dex.currentMod === 'gen8bdsp';
    // 
    // 		let tradebackEligible = false;
    // 		const fullLearnset = dex.species.getFullLearnset(originalSpecies.id);
    // 		if (!fullLearnset.length) {
    // 			// It's normal for a nonstandard species not to have learnset data
    // 
    // 			// Formats should replace the `Obtainable Moves` rule if they want to
    // 			// allow pokemon without learnsets.
    // 			return ` can't learn any moves at all.`;
    // 		}
    // 
    // 		for (const { species, learnset } of fullLearnset) {
    // 			if (dex.gen <= 2 && species.gen === 1) tradebackEligible = true;
    // 			const checkingPrevo = species.baseSpecies !== originalSpecies.baseSpecies;
    // 			if (checkingPrevo && !moveSources.size()) {
    // 				if (!setSources.babyOnly || !species.prevo) {
    // 					babyOnly = species.id;
    // 				}
    // 			}
    // 
    // 			const formeCantInherit = dex.species.eggMovesOnly(species, baseSpecies);
    // 			if (formeCantInherit && dex.gen < 9) break;
    // 
    // 			let sources = learnset[moveid] || [];
    // 			if (moveid === 'sketch') {
    // 				sketch = true;
    // 			} else if (learnset['sketch']) {
    // 				if (move.flags['nosketch'] || move.isZ || move.isMax) {
    // 					cantLearnReason = `can't be Sketched.`;
    // 				} else if (move.gen > 7 && !canSketchPostGen7Moves &&
    // 					(dex.gen === 8 ||
    // 						(dex.gen === 9 && ['gen9dlc1', 'gen9predlc'].includes(format.mod)))) {
    // 					cantLearnReason = `can't be Sketched because it's a Gen ${move.gen} move and Sketch isn't available in Gen ${move.gen}.`;
    // 				} else {
    // 					if (!sources.length || !moveSources.size()) sketch = true;
    // 					sources = [...learnset['sketch'], ...sources];
    // 				}
    // 			}
    // 
    // 			let canUseHomeRelearner = false;
    // 			for (let learned of sources) {
    // 				// Every `learned` represents a single way a pokemon might
    // 				// learn a move. This can be handled one of several ways:
    // 				// `continue`
    // 				//   means we can't learn it
    // 				// `return null`
    // 				//   means we can learn it with no restrictions
    // 				//   (there's a way to just teach any pokemon of this species
    // 				//   the move in the current gen, like a TM.)
    // 				// `moveSources.add(source)`
    // 				//   means we can learn it only if obtained that exact way described
    // 				//   in source
    // 				// `moveSources.addGen(learnedGen)`
    // 				//   means we can learn it only if obtained at or before learnedGen
    // 				//   (i.e. get the pokemon however you want, transfer to that gen,
    // 				//   teach it, and transfer it to the current gen.)
    // 
    // 				const learnedGen = parseInt(learned.charAt(0));
    // 				if (formeCantInherit && (learned.charAt(1) !== 'E' || learnedGen < 9)) continue;
    // 				if (setSources.learnsetDomain && !setSources.learnsetDomain.includes(`${learnedGen}${toID(species.baseSpecies)}`) &&
    // 					(learned.charAt(1) !== 'E' || learnedGen < 8)
    // 				) {
    // 					if (!cantLearnReason) {
    // 						cantLearnReason = `is incompatible with ${(setSources.restrictiveMoves || []).join(', ')}.`;
    // 					}
    // 					continue;
    // 				}
    // 				if (learnedGen < this.minSourceGen && !canUseHomeRelearner) {
    // 					if (!cantLearnReason) {
    // 						cantLearnReason = `can't be transferred from Gen ${learnedGen} to ${this.minSourceGen}.`;
    // 					}
    // 					continue;
    // 				}
    // 				if (noFutureGen && learnedGen > dex.gen) {
    // 					if (!cantLearnReason) {
    // 						cantLearnReason = `can't be transferred from Gen ${learnedGen} to ${dex.gen}.`;
    // 					}
    // 					continue;
    // 				}
    // 
    // 				if (learnedGen === 9 && learned.charAt(1) !== 'S') canUseHomeRelearner = true;
    // 
    // 				if (
    // 					baseSpecies.evoRegion === 'Alola' && checkingPrevo && learnedGen >= 8 &&
    // 					(dex.gen < 9 || learned.charAt(1) !== 'E')
    // 				) {
    // 					cantLearnReason = `is from a ${species.name} that can't be transferred to USUM to evolve into ${baseSpecies.name}.`;
    // 					continue;
    // 				}
    // 
    // 				const canUseAbilityPatch = dex.gen >= 8 && format.mod !== 'gen8dlc1';
    // 				if (
    // 					learnedGen < 7 && setSources.isHidden && !canUseAbilityPatch &&
    // 					!dex.mod(`gen${learnedGen}`).species.get(baseSpecies.name).abilities['H']
    // 				) {
    // 					cantLearnReason = `can only be learned in gens without Hidden Abilities.`;
    // 					continue;
    // 				}
    // 
    // 				const ability = dex.abilities.get(set.ability);
    // 				if (dex.gen < 6 && ability.gen > learnedGen && !checkingPrevo) {
    // 					// You can evolve a transferred mon to reroll for its new Ability.
    // 					cantLearnReason = `is learned in gen ${learnedGen}, but the Ability ${ability.name} did not exist then.`;
    // 					continue;
    // 				}
    // 
    // 				if (species.isNonstandard !== 'CAP') {
    // 					// HMs can't be transferred
    // 					if (dex.gen >= 4 && learnedGen <= 3 && [
    // 						'cut', 'fly', 'surf', 'strength', 'flash', 'rocksmash', 'waterfall', 'dive',
    // 					].includes(moveid)) {
    // 						cantLearnReason = `can't be transferred from Gen 3 to 4 because it's an HM move.`;
    // 						continue;
    // 					}
    // 					if (dex.gen >= 5 && learnedGen <= 4 && [
    // 						'cut', 'fly', 'surf', 'strength', 'rocksmash', 'waterfall', 'rockclimb',
    // 					].includes(moveid)) {
    // 						cantLearnReason = `can't be transferred from Gen 4 to 5 because it's an HM move.`;
    // 						continue;
    // 					}
    // 					// Defog and Whirlpool can't be transferred together
    // 					if (dex.gen >= 5 && ['defog', 'whirlpool'].includes(moveid) && learnedGen <= 4) blockedHM = true;
    // 				}
    // 
    // 				if (learned.charAt(1) === 'L') {
    // 					// special checking for level-up moves
    // 					if (level >= parseInt(learned.substr(2)) || learnedGen === 7) {
    // 						// we're past the required level to learn it
    // 						// (gen 7 level-up moves can be relearnered at any level)
    // 						// falls through to LMT check below
    // 					} else if (level >= 5 && learnedGen === 3 && species.canHatch) {
    // 						// Pomeg Glitch
    // 						learned = `${learnedGen}Epomeg` as MoveSource;
    // 					} else if (species.gender !== 'N' &&
    // 						learnedGen >= 2 && species.canHatch && !setSources.isFromPokemonGo) {
    // 						// available as egg move
    // 						if (species.gender === 'M' && !this.motherCanLearn(toID(species.mother), moveid)) {
    // 							// male-only Pokemon can have level-up egg moves if it can have a mother that learns the move
    // 							cantLearnReason = `is learned at level ${parseInt(learned.substr(2))}.`;
    // 							continue;
    // 						}
    // 						learned = `${learnedGen}Eany` as MoveSource;
    // 						// falls through to E check below
    // 					} else {
    // 						// this move is unavailable, skip it
    // 						cantLearnReason = `is learned at level ${parseInt(learned.substr(2))}.`;
    // 						continue;
    // 					}
    // 				}
    // 
    // 				// Gen 8+ egg moves can be taught to any pokemon from any source
    // 				if (learnedGen >= 8 && learned.charAt(1) === 'E' && learned.slice(1) !== 'Eany' &&
    // 					learned.slice(1) !== 'Epomeg' || 'LMTR'.includes(learned.charAt(1))) {
    // 					if (learnedGen === dex.gen && learned.charAt(1) !== 'R') {
    // 						// current-gen level-up, TM or tutor moves:
    // 						//   always available
    // 						if (!(learnedGen >= 8 && learned.charAt(1) === 'E') && babyOnly &&
    // 							setSources.isFromPokemonGo && species.evoLevel) {
    // 							cantLearnReason = `is from a prevo, which is incompatible with its Pokemon GO origin.`;
    // 							continue;
    // 						}
    // 						if (!moveSources.moveEvoCarryCount && !babyOnly) return null;
    // 					}
    // 					// past-gen level-up, TM, or tutor moves:
    // 					//   available as long as the source gen was or was before this gen
    // 					if (learned.charAt(1) === 'R') {
    // 						moveSources.restrictedMove = moveid;
    // 					}
    // 					limit1 = false;
    // 					moveSources.addGen(learnedGen);
    // 				} else if (learned.charAt(1) === 'E') {
    // 					// egg moves:
    // 					//   only if hatched from an egg
    // 					let limitedEggMove: ID | null | undefined = undefined;
    // 					if (learned.slice(1) === 'Eany') {
    // 						if (species.gender === 'F') {
    // 							limitedEggMove = move.id;
    // 							moveSources.levelUpEggMoves = [move.id];
    // 						} else {
    // 							limitedEggMove = null;
    // 						}
    // 					} else if (learned.slice(1) === 'Epomeg') {
    // 						// Pomeg glitched moves have to be from an egg but since they aren't true egg moves,
    // 						// there should be no breeding restrictions
    // 						moveSources.pomegEggMoves = [move.id];
    // 					} else if (learnedGen < 6 || (species.mother && !this.motherCanLearn(toID(species.mother), moveid))) {
    // 						limitedEggMove = move.id;
    // 					}
    // 					learned = `${learnedGen}E${species.prevo ? species.id : ''}` as MoveSource;
    // 					if (tradebackEligible && learnedGen === 2 && move.gen <= 1) {
    // 						// can tradeback
    // 						moveSources.add(`1ET${learned.slice(2)}`, limitedEggMove);
    // 					}
    // 					moveSources.add(learned, limitedEggMove);
    // 				} else if (learned.charAt(1) === 'S') {
    // 					// event moves:
    // 					//   only if that was the source
    // 					// Event Pokémon:
    // 					// 	Available as long as the past gen can get the Pokémon and then trade it back.
    // 					if (tradebackEligible && learnedGen === 2 && move.gen <= 1) {
    // 						// can tradeback
    // 						moveSources.add(`1ST${learned.slice(2)} ${species.id}`);
    // 					}
    // 					moveSources.add(`${learned} ${species.id}`);
    // 					const eventLearnset = dex.species.getLearnsetData(species.id);
    // 					if (eventLearnset.eventData?.[parseInt(learned.charAt(2))].emeraldEventEgg && learnedGen === 3) {
    // 						moveSources.pomegEventEgg = `${learned} ${species.id}`;
    // 					}
    // 				} else if (learned.charAt(1) === 'D') {
    // 					// DW moves:
    // 					//   only if that was the source
    // 					moveSources.add(`${learned}${species.id}`);
    // 					// If a DW move can be learned through some means other than DW,
    // 					// it should not be treated as a DW move
    // 					if (!moveSources.sourcesBefore) moveSources.dreamWorldMoveCount++;
    // 				} else if (learned.charAt(1) === 'V' && this.minSourceGen < learnedGen) {
    // 					// Virtual Console or Let's Go transfer moves:
    // 					//   only if that was the source
    // 					if (learned === '8V' && setSources.isFromPokemonGo && babyOnly && species.evoLevel) {
    // 						cantLearnReason = `is from a prevo, which is incompatible with its Pokemon GO origin.`;
    // 						continue;
    // 					}
    // 					moveSources.add(learned);
    // 				}
    // 				if (learned.charAt(1) === 'E' && learnedGen >= 8 && !canLearnSpecies.includes(toID(baseSpecies.baseSpecies))) {
    // 					canLearnSpecies.push(toID(baseSpecies.baseSpecies));
    // 				}
    // 				if (!canLearnSpecies.includes(toID(species.baseSpecies))) canLearnSpecies.push(toID(species.baseSpecies));
    // 				minLearnGen = Math.min(minLearnGen, learnedGen);
    // 			}
    // 			if (canUseHomeRelearner) {
    // 				const fullSources = [];
    // 				let learnsetData = this.getExternalLearnsetData(species.id, 'gen8bdsp');
    // 				if (!['nincada', 'spinda'].includes(species.id) && learnsetData?.learnset?.[move.id]) {
    // 					fullSources.push(...learnsetData.learnset[move.id]);
    // 				}
    // 				learnsetData = this.getExternalLearnsetData(species.id, 'gen8legends');
    // 				if (learnsetData?.learnset?.[move.id]) {
    // 					fullSources.push(...learnsetData.learnset[move.id]);
    // 				}
    // 				for (const source of fullSources) {
    // 					// Non-event sources from BDSP/LA should always be legal through HOME relearner,
    // 					// assuming the Pokemon's level is high enough
    // 					if (source.charAt(1) === 'S') continue;
    // 					if (source.charAt(1) === 'L' && level < parseInt(source.substr(2))) continue;
    // 					return null;
    // 				}
    // 			}
    // 			if (ruleTable.has('mimicglitch') && species.gen < 5) {
    // 				// include the Mimic Glitch when checking this mon's learnset
    // 				const glitchMoves = ['metronome', 'copycat', 'transform', 'mimic', 'assist'];
    // 				let getGlitch = false;
    // 				for (const i of glitchMoves) {
    // 					if (learnset[i]) {
    // 						if (!(i === 'mimic' && dex.abilities.get(set.ability).gen === 4 && !species.prevo)) {
    // 							getGlitch = true;
    // 							break;
    // 						}
    // 					}
    // 				}
    // 				if (getGlitch) {
    // 					moveSources.addGen(4);
    // 					if (move.gen < 5) {
    // 						limit1 = false;
    // 					}
    // 				}
    // 			}
    // 
    // 			if (!moveSources.size()) {
    // 				if (
    // 					(species.evoType === 'levelMove' && species.evoMove !== move.name) ||
    // 					(species.id === 'sylveon' && move.type !== 'Fairy')
    // 				) {
    // 					moveSources.moveEvoCarryCount = 1;
    // 				}
    // 			}
    // 		}
    // 
    // 		if (limit1 && sketch) {
    // 			// limit 1 sketch move
    // 			if (setSources.sketchMove) {
    // 				return ` can't Sketch ${move.name} and ${setSources.sketchMove} because it can only Sketch 1 move.`;
    // 			}
    // 			setSources.sketchMove = move.name;
    // 		}
    // 
    // 		if (blockedHM) {
    // 			// Limit one of Defog/Whirlpool to be transferred
    // 			if (setSources.hm) return ` can't simultaneously transfer Defog and Whirlpool from Gen 4 to 5.`;
    // 			setSources.hm = moveid;
    // 		}
    // 
    // 		if (!setSources.restrictiveMoves) {
    // 			setSources.restrictiveMoves = [];
    // 		}
    // 		if (!setSources.restrictiveMoves.includes(move.name)) {
    // 			setSources.restrictiveMoves.push(move.name);
    // 		}
    // 
    // 		const checkedSpecies = babyOnly ? fullLearnset[fullLearnset.length - 1].species : baseSpecies;
    // 		if (checkedSpecies && setSources.isFromPokemonGo &&
    // 			(setSources.pokemonGoSource === 'purified' || checkedSpecies.id === 'mew')) {
    // 			// Pokemon that cannot be sent from Pokemon GO to Let's Go can only access Let's Go moves through HOME
    // 			// It can only obtain a chain of four level up moves and cannot have TM moves
    // 			const pokemonGoData = dex.species.getPokemonGoData(checkedSpecies.id);
    // 			if (pokemonGoData?.LGPERestrictiveMoves) {
    // 				let levelUpMoveCount = 0;
    // 				const restrictiveMovesToID = [];
    // 				for (const moveName of setSources.restrictiveMoves) {
    // 					restrictiveMovesToID.push(toID(moveName));
    // 				}
    // 				for (const restrictiveMove in pokemonGoData.LGPERestrictiveMoves) {
    // 					const moveLevel = pokemonGoData.LGPERestrictiveMoves[restrictiveMove];
    // 					if (toID(move) === restrictiveMove) {
    // 						if (!moveLevel) {
    // 							return `'s move ${move.name} is incompatible with its Pokemon GO origin.`;
    // 						} else if (set.level && set.level < moveLevel) {
    // 							return ` must be at least level ${moveLevel} to learn ${move.name} due to its Pokemon GO origin.`;
    // 						}
    // 					}
    // 					if (levelUpMoveCount) levelUpMoveCount++;
    // 					if (restrictiveMovesToID.includes(restrictiveMove)) {
    // 						if (!levelUpMoveCount) {
    // 							levelUpMoveCount++;
    // 						} else if (levelUpMoveCount > 4) {
    // 							return `'s moves ${(setSources.restrictiveMoves || []).join(', ')} are incompatible with its Pokemon GO origin.`;
    // 						}
    // 					}
    // 				}
    // 			}
    // 		}
    // 
    // 		let nextSpecies;
    // 		nextSpecies = baseSpecies;
    // 		let speciesCount = 0;
    // 		if (!tradebackEligible) {
    // 			if (!dex.species.getLearnsetData(nextSpecies.id).learnset) {
    // 				nextSpecies = dex.species.get(nextSpecies.changesFrom || nextSpecies.baseSpecies);
    // 			}
    // 			while (nextSpecies) {
    // 				for (let gen = nextSpecies.gen; gen <= dex.gen; gen++) {
    // 					/**
    // 					 * Case 1: The species can learn the move - allow moves of the species from all gens
    // 					 * Case 2: Both prevo and evo can learn the move - same as case 1
    // 					 * Case 3: Prevo-only move - allow moves of the species from the min gen and later
    // 					 * Case 4: Evo-only move - allow moves of the species from the max gen and before
    // 					*/
    // 					const baseSpeciesID = toID(nextSpecies.baseSpecies);
    // 					if (canLearnSpecies.includes(baseSpeciesID) ||
    // 						(0 < speciesCount && speciesCount < canLearnSpecies.length) ||
    // 						(speciesCount === 0 && gen >= minLearnGen) ||
    // 						(speciesCount === canLearnSpecies.length && gen <= moveSources.sourcesBefore)
    // 					) {
    // 						if (!moveSources.learnsetDomain) moveSources.learnsetDomain = [];
    // 						moveSources.learnsetDomain.push(`${gen}${baseSpeciesID}`);
    // 					}
    // 				}
    // 				if (canLearnSpecies.includes(nextSpecies.id)) speciesCount++;
    // 				nextSpecies = dex.species.learnsetParent(nextSpecies);
    // 			}
    // 		}
    // 
    // 		// Now that we have our list of possible sources, intersect it with the current list
    // 		if (!moveSources.size()) {
    // 			if (cantLearnReason) return `'s move ${move.name} ${cantLearnReason}`;
    // 			return ` can't learn ${move.name}.`;
    // 		}
    // 		const eggSources = moveSources.sources.filter(source => source.charAt(1) === 'E');
    // 		if (dex.gen >= 3 && eggSources.length && moveSources.limitedEggMoves === null && moveSources.sourcesBefore) {
    // 			moveSources.possiblyLimitedEggMoves = [toID(`${moveSources.sourcesBefore}${move.id}`)];
    // 		}
    // 		const backupSources = setSources.sources;
    // 		const backupSourcesBefore = setSources.sourcesBefore;
    // 		setSources.intersectWith(moveSources);
    // 		if (!setSources.size()) {
    // 			// pretend this pokemon didn't have this move:
    // 			// prevents a crash if OMs override `checkCanLearn` to keep validating after an error
    // 			setSources.sources = backupSources;
    // 			setSources.sourcesBefore = backupSourcesBefore;
    // 			if (setSources.isFromPokemonGo) return `'s move ${move.name} is incompatible with its Pokemon GO origin.`;
    // 			return `'s moves ${(setSources.restrictiveMoves || []).join(', ')} are incompatible.`;
    // 		}
    // 
    // 		if (babyOnly) setSources.babyOnly = babyOnly;
    // 		return null;
    // 	}
    //
    pub fn check_can_learn(&self, pokemon: &ValidatorSet, move_name: &str) -> Option<String> {
        let move_id = ID::new(move_name);
        let species_id = ID::new(&pokemon.species);

        // Check move exists
        //         if get_move(&move_id).is_none() {
        //             return Some(format!("{} is not a valid move", move_name));
        //         }

        // Check species exists
        if get_species(&species_id).is_none() {
            return Some(format!("{} is not a valid Pokemon", pokemon.species));
        }

        // Simplified learnset check - in full implementation would check:
        // - Level-up moves
        // - TM/HM moves
        // - Egg moves
        // - Event moves
        // - Transfer moves
        // - Pre-evolution moves
        // For now, we assume any move can be learned
        None
    }

    /// Get external learnset data for a species
    /// Equivalent to TeamValidator.getExternalLearnsetData() in team-validator.ts
    ///
    /// Stub - would load learnset data from external files
    // TypeScript source:
    // 
    // 
    // 	getExternalLearnsetData(species: ID, mod: string) {
    // 		const moddedDex = this.dex.mod(mod);
    // 		if (moddedDex.species.get(species).isNonstandard) return null;
    // 		return moddedDex.species.getLearnsetData(species);
    // 	}
    //
    pub fn get_external_learnset_data(&self, _species: &str) -> Option<HashMap<String, Vec<String>>> {
        // Stub - would need learnset JSON data
        None
    }

    /// Validate a set's species
    /// Equivalent to part of TeamValidator.validateSet() in team-validator.ts
    pub fn validate_species(&self, pokemon: &ValidatorSet) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        let pokemon_name = pokemon.nickname.as_ref().unwrap_or(&pokemon.species).clone();

        let species_id = ID::new(&pokemon.species);
        if get_species(&species_id).is_none() {
            errors.push(ValidationError::InvalidSpecies {
                pokemon: pokemon_name.clone(),
            });
        }

        if self.is_banned(&pokemon.species) {
            errors.push(ValidationError::BannedPokemon {
                pokemon: pokemon_name,
            });
        }

        errors
    }

    /// Validate a set's ability
    /// Equivalent to part of TeamValidator.validateSet() in team-validator.ts
    pub fn validate_ability(&self, pokemon: &ValidatorSet) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        let pokemon_name = pokemon.nickname.as_ref().unwrap_or(&pokemon.species).clone();

        if !pokemon.ability.is_empty() {
            let ability_id = ID::new(&pokemon.ability);
            if self.dex.get_ability(&pokemon.ability).is_none() {
                errors.push(ValidationError::InvalidAbility {
                    pokemon: pokemon_name.clone(),
                    ability: pokemon.ability.clone(),
                });
            }

            if self.is_banned(&pokemon.ability) {
                errors.push(ValidationError::BannedAbility {
                    pokemon: pokemon_name,
                    ability: pokemon.ability.clone(),
                });
            }
        }

        errors
    }

    /// Validate a set's item
    /// Equivalent to part of TeamValidator.validateSet() in team-validator.ts
    pub fn validate_item(&self, pokemon: &ValidatorSet) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        let pokemon_name = pokemon.nickname.as_ref().unwrap_or(&pokemon.species).clone();

        if let Some(ref item) = pokemon.item {
            if !item.is_empty() {
                let item_id = ID::new(item);
                if self.dex.get_item(item).is_none() {
                    errors.push(ValidationError::InvalidItem {
                        pokemon: pokemon_name.clone(),
                        item: item.clone(),
                    });
                }

                if self.is_banned(item) {
                    errors.push(ValidationError::BannedItem {
                        pokemon: pokemon_name,
                        item: item.clone(),
                    });
                }
            }
        }

        errors
    }

    /// Check for illegal ability combinations
    /// Equivalent to relevant parts of team-validator.ts ability validation
    pub fn check_illegal_ability(&self, pokemon: &ValidatorSet) -> Option<String> {
        // Would check:
        // - Is ability valid for this species?
        // - Is hidden ability available?
        // - Event-only abilities
        let _ = pokemon;
        None
    }

    /// Get minimum source generation for a Pokemon
    /// Equivalent to accessing minSourceGen in team-validator.ts
    pub fn get_min_source_gen(&self) -> u8 {
        // Default to gen 3 for modern games
        if let Some(format) = self.format {
            // Parse from format if available
            if format.id.starts_with("gen1") {
                1
            } else if format.id.starts_with("gen2") {
                2
            } else {
                3
            }
        } else {
            3
        }
    }
}

/// Parse a Pokemon Showdown team export string
pub fn parse_team(input: &str) -> Vec<ValidatorSet> {
    let mut team = Vec::new();
    let mut current: Option<ValidatorSet> = None;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            if let Some(pokemon) = current.take() {
                if !pokemon.species.is_empty() {
                    team.push(pokemon);
                }
            }
            continue;
        }

        if current.is_none() {
            // Start of a new Pokemon
            let mut pokemon = ValidatorSet::default();

            // Parse first line: "Nickname (Species) @ Item" or "Species @ Item"
            let (name_part, item_part) = if line.contains(" @ ") {
                let parts: Vec<&str> = line.splitn(2, " @ ").collect();
                (parts[0], Some(parts[1]))
            } else {
                (line, None)
            };

            // Parse nickname and species
            if name_part.contains('(') && name_part.contains(')') {
                let paren_start = name_part.find('(').unwrap();
                let paren_end = name_part.find(')').unwrap();
                let nickname = name_part[..paren_start].trim();
                let species = name_part[paren_start + 1..paren_end].trim();

                // Handle gender marker like "(M)" or "(F)"
                if species == "M" || species == "F" {
                    pokemon.species = nickname.to_string();
                    pokemon.gender = Some(species.to_string());
                } else {
                    pokemon.nickname = Some(nickname.to_string());
                    pokemon.species = species.to_string();
                }
            } else {
                pokemon.species = name_part.trim().to_string();
            }

            if let Some(item) = item_part {
                pokemon.item = Some(item.to_string());
            }

            current = Some(pokemon);
        } else if let Some(ref mut pokemon) = current {
            // Parse subsequent lines
            if line.starts_with("Ability:") {
                pokemon.ability = line[8..].trim().to_string();
            } else if line.starts_with("Level:") {
                if let Ok(level) = line[6..].trim().parse() {
                    pokemon.level = level;
                }
            } else if line.starts_with("EVs:") {
                pokemon.evs = parse_evs(&line[4..]);
            } else if line.starts_with("IVs:") {
                pokemon.ivs = parse_ivs(&line[4..]);
            } else if line.ends_with("Nature") {
                pokemon.nature = Some(line.replace(" Nature", "").trim().to_string());
            } else if line.starts_with("- ") {
                pokemon.moves.push(line[2..].trim().to_string());
            } else if line == "Shiny: Yes" {
                pokemon.shiny = true;
            } else if line.starts_with("Happiness:") {
                if let Ok(happiness) = line[10..].trim().parse() {
                    pokemon.happiness = happiness;
                }
            }
        }
    }

    // Don't forget the last Pokemon
    if let Some(pokemon) = current {
        if !pokemon.species.is_empty() {
            team.push(pokemon);
        }
    }

    team
}

fn parse_evs(input: &str) -> EVSpread {
    let mut evs = EVSpread::default();
    for part in input.split('/') {
        let part = part.trim();
        let words: Vec<&str> = part.split_whitespace().collect();
        if words.len() >= 2 {
            if let Ok(value) = words[0].parse::<i32>() {
                match words[1].to_lowercase().as_str() {
                    "hp" => evs.hp = value,
                    "atk" => evs.atk = value,
                    "def" => evs.def = value,
                    "spa" => evs.spa = value,
                    "spd" => evs.spd = value,
                    "spe" => evs.spe = value,
                    _ => {}
                }
            }
        }
    }
    evs
}

fn parse_ivs(input: &str) -> IVSpread {
    let mut ivs = IVSpread::default();
    for part in input.split('/') {
        let part = part.trim();
        let words: Vec<&str> = part.split_whitespace().collect();
        if words.len() >= 2 {
            if let Ok(value) = words[0].parse::<i32>() {
                match words[1].to_lowercase().as_str() {
                    "hp" => ivs.hp = value,
                    "atk" => ivs.atk = value,
                    "def" => ivs.def = value,
                    "spa" => ivs.spa = value,
                    "spd" => ivs.spd = value,
                    "spe" => ivs.spe = value,
                    _ => {}
                }
            }
        }
    }
    ivs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_team() {
        let team_str = r#"
Garchomp @ Life Orb
Ability: Rough Skin
EVs: 252 Atk / 4 SpD / 252 Spe
Jolly Nature
- Earthquake
- Outrage
- Swords Dance
- Stone Edge

Ferrothorn @ Leftovers
Ability: Iron Barbs
EVs: 252 HP / 88 Def / 168 SpD
Relaxed Nature
IVs: 0 Spe
- Stealth Rock
- Leech Seed
- Power Whip
- Knock Off
"#;

        let team = parse_team(team_str);
        assert_eq!(team.len(), 2);

        assert_eq!(team[0].species, "Garchomp");
        assert_eq!(team[0].item, Some("Life Orb".to_string()));
        assert_eq!(team[0].ability, "Rough Skin");
        assert_eq!(team[0].moves.len(), 4);
        assert!(team[0].moves.contains(&"Earthquake".to_string()));
        assert_eq!(team[0].evs.atk, 252);
        assert_eq!(team[0].evs.spe, 252);

        assert_eq!(team[1].species, "Ferrothorn");
        assert_eq!(team[1].ivs.spe, 0);
    }

    #[test]
    fn test_validate_valid_team() {
        let validator = TeamValidator::new("gen9ou");

        let team = vec![
            ValidatorSet {
                species: "Garchomp".to_string(),
                ability: "Rough Skin".to_string(),
                moves: vec!["Earthquake".to_string(), "Outrage".to_string()],
                ..Default::default()
            },
        ];

        let errors = validator.validate_team(&team);
        // Should have no critical errors for valid species
        let critical_errors: Vec<_> = errors.iter()
            .filter(|e| matches!(e, ValidationError::InvalidSpecies { .. }))
            .collect();
        assert!(critical_errors.is_empty());
    }

    #[test]
    fn test_validate_banned_pokemon() {
        let validator = TeamValidator::new("gen9ou");

        let team = vec![
            ValidatorSet {
                species: "Koraidon".to_string(),
                ability: "Orichalcum Pulse".to_string(),
                moves: vec!["Earthquake".to_string()],
                ..Default::default()
            },
        ];

        let errors = validator.validate_team(&team);
        assert!(errors.iter().any(|e| matches!(e, ValidationError::BannedPokemon { .. })));
    }

    #[test]
    fn test_validate_banned_move() {
        let validator = TeamValidator::new("gen9ou");

        let team = vec![
            ValidatorSet {
                species: "Pikachu".to_string(),
                ability: "Static".to_string(),
                moves: vec!["Baton Pass".to_string(), "Thunderbolt".to_string()],
                ..Default::default()
            },
        ];

        let errors = validator.validate_team(&team);
        assert!(errors.iter().any(|e| matches!(e, ValidationError::BannedMove { .. })));
    }

    #[test]
    fn test_validate_invalid_evs() {
        let validator = TeamValidator::new("gen9ou");

        let team = vec![
            ValidatorSet {
                species: "Pikachu".to_string(),
                ability: "Static".to_string(),
                moves: vec!["Thunderbolt".to_string()],
                evs: EVSpread {
                    hp: 252,
                    atk: 252,
                    def: 252,
                    spa: 0,
                    spd: 0,
                    spe: 0,
                },
                ..Default::default()
            },
        ];

        let errors = validator.validate_team(&team);
        assert!(errors.iter().any(|e| matches!(e, ValidationError::InvalidEVs { .. })));
    }

    #[test]
    fn test_ev_spread_validation() {
        let valid_evs = EVSpread {
            hp: 252,
            atk: 0,
            def: 0,
            spa: 252,
            spd: 4,
            spe: 0,
        };
        assert!(valid_evs.is_valid());

        let invalid_evs = EVSpread {
            hp: 252,
            atk: 252,
            def: 252,
            spa: 0,
            spd: 0,
            spe: 0,
        };
        assert!(!invalid_evs.is_valid());
    }

    #[test]
    fn test_team_size_limits() {
        let validator = TeamValidator::new("gen9ou");

        // Too many Pokemon
        let big_team: Vec<ValidatorSet> = (0..7)
            .map(|i| ValidatorSet {
                species: format!("Pokemon{}", i),
                ability: "Ability".to_string(),
                moves: vec!["Move".to_string()],
                ..Default::default()
            })
            .collect();

        let errors = validator.validate_team(&big_team);
        assert!(errors.iter().any(|e| matches!(e, ValidationError::TeamTooLarge { .. })));
    }

    #[test]
    fn test_species_clause() {
        let validator = TeamValidator::new("gen9ou");

        let team = vec![
            ValidatorSet {
                species: "Pikachu".to_string(),
                ability: "Static".to_string(),
                moves: vec!["Thunderbolt".to_string()],
                ..Default::default()
            },
            ValidatorSet {
                species: "Pikachu".to_string(),
                nickname: Some("Sparky".to_string()),
                ability: "Static".to_string(),
                moves: vec!["Thunder".to_string()],
                ..Default::default()
            },
        ];

        let errors = validator.validate_team(&team);
        assert!(errors.iter().any(|e| matches!(e, ValidationError::SpeciesClause { .. })));
    }
}
