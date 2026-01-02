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
    pub fn new(set: &PokemonSet, side_index: usize, position: usize) -> Self {
        // TODO: implement the same logic as JavaScript

        let species_id = ID::new(&set.species);
        let ability_id = ID::new(&set.ability);
        let item_id = ID::new(&set.item);

        // Convert moves to move slots
        // Note: is_z will be set to false initially, should be updated with Dex access
        let move_slots: Vec<MoveSlot> = set
            .moves
            .iter()
            .map(|m| {
                let id = ID::new(m);
                MoveSlot::new(id, m.clone(), 5, 5) // Default PP and is_z=false, will be set by Dex
            })
            .collect();

        Self {
            name: if set.name.is_empty() {
                set.species.clone()
            } else {
                set.name.clone()
            },
            species_id: species_id.clone(),
            base_species: species_id.clone(),
            level: set.level,
            gender: set.gender,
            nature: set.nature.clone(),
            happiness: set.happiness,
            pokeball: ID::new(&set.pokeball),
            dynamax_level: set.dynamax_level,
            gigantamax: set.gigantamax,

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
            max_hp_undynamaxed: 0,
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

            locked_move: None,

            types: Vec::new(),
            added_type: None,
            base_types: Vec::new(),
            known_type: None, // Initially null, set when type becomes known (e.g., Illusion breaks)
            apparent_type: None, // Initially null, set when type is revealed to players

            tera_type: set.tera_type.clone(),
            terastallized: None,
            can_terastallize: set.tera_type.clone(),

            base_move_slots: move_slots.clone(),
            move_slots,

            hp_type: set.hptype.clone(),

            status: ID::empty(),
            status_state: EffectState::new(ID::empty()),
            volatiles: HashMap::new(),

            fainted: false,
            faint_queued: false,
            transformed: false,
            illusion: None,

            trapped: false,
            maybe_trapped: false,
            maybe_disabled: false,
            maybe_locked: false,
            switch_flag: false,
            force_switch_flag: false,
            newly_switched: false,
            being_called_back: false,
            dragged_in: None,
            skip_before_switch_out_event_flag: false,
            stats_raised_this_turn: false,
            stats_lowered_this_turn: false,
            sub_fainted: false,

            sword_boost: false,

            last_move: None,
            last_move_used: None,
            last_move_target_loc: None,
            move_this_turn: None,
            move_this_turn_result: None,
            move_last_turn_result: None,
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
        }
    }
}
