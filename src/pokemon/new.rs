// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::dex_data::BoostsTable;
use crate::event_system::EffectState;
use crate::dex_data::StatsTable;
use crate::pokemon::MoveSlot;
use std::collections::HashMap;

impl Pokemon {
    /// Create a new Pokemon from a PokemonSet
    // constructor(set: string | AnyObject, side: Side) {
    //     this.side = side;
    //     this.battle = side.battle;
    //
    //     this.m = {};
    //
    //     const pokemonScripts = this.battle.format.pokemon || this.battle.dex.data.Scripts.pokemon;
    //     if (pokemonScripts) Object.assign(this, pokemonScripts);
    //
    //     if (typeof set === 'string') set = { name: set };
    //
    //     this.baseSpecies = this.battle.dex.species.get(set.species || set.name);
    //     if (!this.baseSpecies.exists) {
    //         throw new Error(`Unidentified species: ${this.baseSpecies.name}`);
    //     }
    //     this.set = set as PokemonSet;
    //
    //     this.species = this.baseSpecies;
    //     if (set.name === set.species || !set.name) {
    //         set.name = this.baseSpecies.baseSpecies;
    //     }
    //     this.speciesState = this.battle.initEffectState({ id: this.species.id });
    //
    //     this.name = set.name.substr(0, 20);
    //     this.fullname = `${this.side.id}: ${this.name}`;
    //
    //     set.level = this.battle.clampIntRange(set.adjustLevel || set.level || 100, 1, 9999);
    //     this.level = set.level;
    //     const genders: { [key: string]: GenderName | null } = { __proto__: null, M: 'M', F: 'F', N: 'N' };
    //     this.gender = genders[set.gender] || this.species.gender || this.battle.sample(['M', 'F']);
    //     if (this.gender === 'N') this.gender = '';
    //     this.happiness = typeof set.happiness === 'number' ? this.battle.clampIntRange(set.happiness, 0, 255) : 255;
    //     if (this.battle.format.mod === 'gen7letsgo') this.happiness = 70;
    //     this.pokeball = toID(this.set.pokeball) || 'pokeball' as ID;
    //     this.dynamaxLevel = typeof set.dynamaxLevel === 'number' ? this.battle.clampIntRange(set.dynamaxLevel, 0, 10) : 10;
    //     this.gigantamax = this.set.gigantamax || false;
    //
    //     this.baseMoveSlots = [];
    //     this.moveSlots = [];
    //     if (!this.set.moves?.length) {
    //         throw new Error(`Set ${this.name} has no moves`);
    //     }
    //     for (const moveid of this.set.moves) {
    //         let move = this.battle.dex.moves.get(moveid);
    //         if (!move.id) continue;
    //         if (move.id === 'hiddenpower' && move.type !== 'Normal') {
    //             if (!set.hpType) set.hpType = move.type;
    //             move = this.battle.dex.moves.get('hiddenpower');
    //         }
    //         let basepp = move.noPPBoosts ? move.pp : move.pp * 8 / 5;
    //         if (this.battle.gen < 3) basepp = Math.min(61, basepp);
    //         this.baseMoveSlots.push({
    //             move: move.name,
    //             id: move.id,
    //             pp: basepp,
    //             maxpp: basepp,
    //             target: move.target,
    //             disabled: false,
    //             disabledSource: '',
    //             used: false,
    //         });
    //     }
    //
    //     this.position = 0;
    //     this.details = this.getUpdatedDetails();
    //
    //     this.status = '';
    //     this.statusState = this.battle.initEffectState({});
    //     this.volatiles = {};
    //     this.showCure = undefined;
    //
    //     if (!this.set.evs) {
    //         this.set.evs = { hp: 0, atk: 0, def: 0, spa: 0, spd: 0, spe: 0 };
    //     }
    //     if (!this.set.ivs) {
    //         this.set.ivs = { hp: 31, atk: 31, def: 31, spa: 31, spd: 31, spe: 31 };
    //     }
    //     const stats: StatsTable = { hp: 31, atk: 31, def: 31, spe: 31, spa: 31, spd: 31 };
    //     let stat: StatID;
    //     for (stat in stats) {
    //         if (!this.set.evs[stat]) this.set.evs[stat] = 0;
    //         if (!this.set.ivs[stat] && this.set.ivs[stat] !== 0) this.set.ivs[stat] = 31;
    //     }
    //     for (stat in this.set.evs) {
    //         this.set.evs[stat] = this.battle.clampIntRange(this.set.evs[stat], 0, 255);
    //     }
    //     for (stat in this.set.ivs) {
    //         this.set.ivs[stat] = this.battle.clampIntRange(this.set.ivs[stat], 0, 31);
    //     }
    //     if (this.battle.gen && this.battle.gen <= 2) {
    //         // We represent DVs using even IVs. Ensure they are in fact even.
    //         for (stat in this.set.ivs) {
    //             this.set.ivs[stat] &= 30;
    //         }
    //     }
    //
    //     const hpData = this.battle.dex.getHiddenPower(this.set.ivs);
    //     this.hpType = set.hpType || hpData.type;
    //     this.hpPower = hpData.power;
    //
    //     this.baseHpType = this.hpType;
    //     this.baseHpPower = this.hpPower;
    //
    //     // initialized in this.setSpecies(this.baseSpecies)
    //     this.baseStoredStats = null!;
    //     this.storedStats = { atk: 0, def: 0, spa: 0, spd: 0, spe: 0 };
    //     this.boosts = { atk: 0, def: 0, spa: 0, spd: 0, spe: 0, accuracy: 0, evasion: 0 };
    //
    //     this.baseAbility = toID(set.ability);
    //     this.ability = this.baseAbility;
    //     this.abilityState = this.battle.initEffectState({ id: this.ability, target: this });
    //
    //     this.item = toID(set.item);
    //     this.itemState = this.battle.initEffectState({ id: this.item, target: this });
    //     this.lastItem = '';
    //     this.usedItemThisTurn = false;
    //     this.ateBerry = false;
    //     this.itemKnockedOff = false;
    //
    //     this.trapped = false;
    //     this.maybeTrapped = false;
    //     this.maybeDisabled = false;
    //     this.maybeLocked = false;
    //
    //     this.illusion = null;
    //     this.transformed = false;
    //
    //     this.fainted = false;
    //     this.faintQueued = false;
    //     this.subFainted = null;
    //
    //     this.formeRegression = false;
    //
    //     this.types = this.baseSpecies.types;
    //     this.baseTypes = this.types;
    //     this.addedType = '';
    //     this.knownType = true;
    //     this.apparentType = this.baseSpecies.types.join('/');
    //     // Every Pokemon has a Terastal type
    //     this.teraType = this.set.teraType || this.types[0];
    //
    //     this.switchFlag = false;
    //     this.forceSwitchFlag = false;
    //     this.skipBeforeSwitchOutEventFlag = false;
    //     this.draggedIn = null;
    //     this.newlySwitched = false;
    //     this.beingCalledBack = false;
    //
    //     this.lastMove = null;
    //     // This is used in gen 2 only, here to avoid code repetition.
    //     // Only declared if gen 2 to avoid declaring an object we aren't going to need.
    //     if (this.battle.gen === 2) this.lastMoveEncore = null;
    //     this.lastMoveUsed = null;
    //     this.moveThisTurn = '';
    //     this.statsRaisedThisTurn = false;
    //     this.statsLoweredThisTurn = false;
    //     this.hurtThisTurn = null;
    //     this.lastDamage = 0;
    //     this.attackedBy = [];
    //     this.timesAttacked = 0;
    //
    //     this.isActive = false;
    //     this.activeTurns = 0;
    //     this.activeMoveActions = 0;
    //     this.previouslySwitchedIn = 0;
    //     this.truantTurn = false;
    //     this.bondTriggered = false;
    //     this.heroMessageDisplayed = false;
    //     this.swordBoost = false;
    //     this.shieldBoost = false;
    //     this.syrupTriggered = false;
    //     this.stellarBoostedTypes = [];
    //     this.isStarted = false;
    //     this.duringMove = false;
    //
    //     this.weighthg = 1;
    //     this.speed = 0;
    //
    //     this.canMegaEvo = this.battle.actions.canMegaEvo(this);
    //     this.canMegaEvoX = this.battle.actions.canMegaEvoX?.(this);
    //     this.canMegaEvoY = this.battle.actions.canMegaEvoY?.(this);
    //     this.canUltraBurst = this.battle.actions.canUltraBurst(this);
    //     this.canGigantamax = this.baseSpecies.canGigantamax || null;
    //     this.canTerastallize = this.battle.actions.canTerastallize(this);
    //
    //     // This is used in gen 1 only, here to avoid code repetition.
    //     // Only declared if gen 1 to avoid declaring an object we aren't going to need.
    //     if (this.battle.gen === 1) this.modifiedStats = { atk: 0, def: 0, spa: 0, spd: 0, spe: 0 };
    //
    //     this.maxhp = 0;
    //     this.baseMaxhp = 0;
    //     this.hp = 0;
    //     this.clearVolatile();
    //     this.hp = this.maxhp;
    // }
    pub fn new(set: &PokemonSet, side_index: usize, position: usize, dex: &crate::dex::Dex) -> Self {
        // Note: This is a Rust-specific constructor - JavaScript uses a class constructor
        // Note: Rust implementation is simplified and delegates some initialization to Battle
        // Note: JavaScript constructor is much more complex with Dex access, script loading, etc.

        let species_id = ID::new(&set.species);
        let ability_id = ID::new(&set.ability);
        let item_id = ID::new(&set.item);

        // JavaScript logic for Pokemon name:
        // if (set.name === set.species || !set.name) {
        //     set.name = this.baseSpecies.baseSpecies;
        // }
        // this.name = set.name.substr(0, 20);
        let pokemon_name = if set.name == set.species || set.name.is_empty() {
            // Get baseSpecies.baseSpecies
            if let Some(species_data) = dex.species().get(&set.species) {
                let base_species_name = species_data.base_species.as_ref().unwrap_or(&species_data.name);
                if let Some(base_species_data) = dex.species().get(base_species_name) {
                    base_species_data.base_species.as_ref()
                        .unwrap_or(&base_species_data.name)
                        .clone()
                } else {
                    set.species.clone()
                }
            } else {
                set.species.clone()
            }
        } else {
            set.name.clone()
        };
        // Truncate to 20 chars
        let name = if pokemon_name.len() > 20 {
            pokemon_name.chars().take(20).collect()
        } else {
            pokemon_name
        };

        // Convert moves to move slots
        // PP will be calculated later by Dex based on move data
        let move_slots: Vec<MoveSlot> = set
            .moves
            .iter()
            .map(|m| {
                let id = ID::new(m);
                MoveSlot::new(id, m.clone(), 5, 5) // Default PP, will be set by Dex
            })
            .collect();

        // JavaScript logic: if a Hidden Power type variant move is in the set, use its type as hpType
        // This happens in the Pokemon constructor loop:
        //   if (!set.hpType) set.hpType = move.type;
        // where move is a Hidden Power variant like "hiddenpowersteel"
        let hp_type_from_moves: Option<String> = if set.hptype.is_none() {
            let mut found_type: Option<String> = None;
            for move_id in &set.moves {
                let move_lower = move_id.to_lowercase();
                if move_lower.starts_with("hiddenpower") && move_lower.len() > 11 {
                    // Extract the type from the move ID (e.g., "hiddenpowersteel" -> "Steel")
                    let type_part = &move_lower[11..]; // Everything after "hiddenpower"
                    // Remove any numeric suffix (e.g., "steel60" -> "steel")
                    let type_str: String = type_part.chars().take_while(|c| c.is_alphabetic()).collect();
                    if !type_str.is_empty() {
                        // Capitalize the first letter
                        let capitalized = type_str.chars().next().unwrap().to_uppercase().to_string()
                            + &type_str[1..];
                        found_type = Some(capitalized);
                        break;
                    }
                }
            }
            found_type
        } else {
            None
        };

        Self {
            set: set.clone(),
            name,
            fullname: String::new(), // Will be set by fullname() method
            species_id: species_id.clone(),
            base_species: species_id.clone(),
            species_state: EffectState::new(ID::empty()),
            level: set.level,
            gender: set.gender,
            happiness: set.happiness,
            pokeball: ID::new(&set.pokeball),
            dynamax_level: set.dynamax_level,
            gigantamax: set.gigantamax,

            details: String::new(), // Will be set by details() method

            position,
            side_index,
            is_active: false,

            base_stored_stats: StatsTable::default(),
            stored_stats: StatsTable::default(),
            boosts: BoostsTable::new(),
            // JS Pokemon constructor doesn't set maxhp/hp directly
            // They're calculated later in clearVolatile() -> setSpecies() -> spreadModify()
            // Initialize to 0 so setSpecies() knows to calculate them
            maxhp: 0,
            base_maxhp: 0,
            hp: 0,

            base_ability: ability_id.clone(),
            ability: ability_id,
            ability_state: EffectState::new(ID::empty()),

            item: item_id,
            item_state: EffectState::new(ID::empty()),
            last_item: ID::empty(),
            used_item_this_turn: false,
            ate_berry: false,
            item_knocked_off: false,

            types: Vec::new(),
            // JavaScript: this.addedType = '';
            added_type: String::new(),
            base_types: Vec::new(),
            known_type: false, // Initially false, set to true when type becomes known
            // JavaScript: this.apparentType = this.baseSpecies.types.join('/');
            // Initially set to empty, will be updated in set_species
            apparent_type: String::new(),

            tera_type: set.tera_type.clone(),
            terastallized: None,
            can_terastallize: set.tera_type.clone(),

            base_move_slots: move_slots.clone(),
            move_slots,

            // Calculate Hidden Power type and power from IVs
            // JavaScript: const hpData = this.battle.dex.getHiddenPower(this.set.ivs);
            //             this.hpType = set.hpType || hpData.type;
            //             this.hpPower = hpData.power;
            // Note: In JavaScript, if a Hidden Power type variant move is in the set,
            // the move.type is used to set set.hpType during Pokemon construction
            hp_type: {
                let (hp_type_from_ivs, _) = crate::dex::Dex::get_hidden_power(&set.ivs);
                // Priority: explicit set.hptype > type from HP move variant > IV-calculated type
                set.hptype.clone()
                    .or_else(|| hp_type_from_moves.clone())
                    .unwrap_or_else(|| hp_type_from_ivs.to_string())
            },
            hp_power: {
                let (_, hp_power_from_ivs) = crate::dex::Dex::get_hidden_power(&set.ivs);
                hp_power_from_ivs as u8
            },
            base_hp_type: {
                let (hp_type_from_ivs, _) = crate::dex::Dex::get_hidden_power(&set.ivs);
                // Priority: explicit set.hptype > type from HP move variant > IV-calculated type
                set.hptype.clone()
                    .or_else(|| hp_type_from_moves.clone())
                    .unwrap_or_else(|| hp_type_from_ivs.to_string())
            },
            base_hp_power: {
                let (_, hp_power_from_ivs) = crate::dex::Dex::get_hidden_power(&set.ivs);
                hp_power_from_ivs as u8
            },

            status: ID::empty(),
            status_state: EffectState::new(ID::empty()),
            show_cure: None,
            volatiles: HashMap::new(),

            fainted: false,
            faint_queued: false,
            transformed: false,
            illusion: None,
            forme_regression: false,

            trapped: TrappedState::None,
            maybe_trapped: false,
            maybe_disabled: false,
            // JavaScript: this.maybeLocked = false;
            maybe_locked: Some(false),
            switch_flag: None,
            force_switch_flag: false,
            newly_switched: false,
            being_called_back: false,
            dragged_in: None,
            skip_before_switch_out_event_flag: false,
            stats_raised_this_turn: false,
            stats_lowered_this_turn: false,
            // JavaScript: this.subFainted = null;
            sub_fainted: None,

            sword_boost: false,
            shield_boost: false,
            truant_turn: false,
            bond_triggered: false,
            hero_message_displayed: false,
            syrup_triggered: false,

            last_move: None,
            last_move_encore: None,
            last_move_used: None,
            last_move_target_loc: None,
            move_this_turn: None,
            move_this_turn_result: crate::battle_actions::MoveResult::Undefined,
            move_last_turn_result: crate::battle_actions::MoveResult::Undefined,
            hurt_this_turn: None,
            last_damage: 0,
            times_attacked: 0,

            active_turns: 0,
            active_move_actions: 0,
            previously_switched_in: 0,
            is_started: false,
            during_move: false,

            attacked_by: Vec::new(),

            weight_hg: 0,
            speed: 0,

            can_mega_evo: None,
            can_mega_evo_x: None,
            can_mega_evo_y: None,
            can_ultra_burst: None,
            can_gigantamax: if set.gigantamax {
                Some(set.species.clone())
            } else {
                None
            },

            stellar_boosted_types: Vec::new(),

            staleness: None,
            pending_staleness: None,
            volatile_staleness: None,

            modified_stats: None,
        }
    }
}
