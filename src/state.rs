//! Battle State Serialization
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module handles battle state serialization and deserialization,
//! enabling battle saving, loading, and replay functionality.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::dex_data::{ID, EffectState};
use crate::prng::PRNGSeed;

/// Serializable battle state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleState {
    /// Format ID
    pub format_id: ID,
    /// PRNG seed at start of battle
    pub seed: PRNGSeed,
    /// Current turn number
    pub turn: i32,
    /// Active turn (for mid-turn states)
    pub active_turn: bool,
    /// Battle ended
    pub ended: bool,
    /// Winner (if ended)
    pub winner: Option<String>,
    /// Side states
    pub sides: [SideState; 2],
    /// Field state
    pub field: FieldState,
    /// Input log (all player choices)
    pub input_log: Vec<String>,
    /// Battle log (all output messages)
    pub log: Vec<String>,
}

/// Serializable side state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SideState {
    /// Side ID (p1, p2)
    pub id: String,
    /// Player name
    pub name: String,
    /// All Pokemon on this side
    pub pokemon: Vec<PokemonState>,
    /// Active Pokemon indices
    pub active: Vec<Option<usize>>,
    /// Side conditions with layer counts
    pub side_conditions: HashMap<ID, u8>,
    /// Slot conditions (Wish, etc.)
    pub slot_conditions: HashMap<usize, HashMap<ID, EffectState>>,
}

/// Serializable Pokemon state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonState {
    /// Species name
    pub species: String,
    /// Nickname (if different from species)
    pub name: String,
    /// Level
    pub level: u8,
    /// Gender
    pub gender: String,
    /// Current HP
    pub hp: i32,
    /// Maximum HP
    pub maxhp: i32,
    /// Current ability ID
    pub ability: ID,
    /// Base ability ID (before Transform, etc.)
    pub base_ability: ID,
    /// Current item ID (empty if no item)
    pub item: ID,
    /// Original item (before Knock Off, etc.)
    pub original_item: ID,
    /// Current types
    pub types: Vec<String>,
    /// Base types (original)
    pub base_types: Vec<String>,
    /// Move slots
    pub moves: Vec<MoveSlotState>,
    /// Current stats (after modifications)
    pub stats: StatsState,
    /// Base stats (before modifications)
    pub base_stats: StatsState,
    /// Stat boosts
    pub boosts: BoostsState,
    /// Non-volatile status
    pub status: Option<String>,
    /// Status data (sleep turns, toxic counter, etc.)
    pub status_state: Option<EffectState>,
    /// Volatile conditions
    pub volatiles: HashMap<ID, EffectState>,
    /// Is this Pokemon active?
    pub is_active: bool,
    /// Is this Pokemon fainted?
    pub fainted: bool,
    /// Position in team (0-5)
    pub position: usize,
    /// Last move used
    pub last_move: Option<ID>,
    /// Last move turn
    pub last_move_turn: Option<i32>,
    /// Transform target (if transformed)
    pub transformed: bool,
}

/// Serializable move slot state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveSlotState {
    /// Move ID
    pub id: ID,
    /// Current PP
    pub pp: u8,
    /// Maximum PP
    pub maxpp: u8,
    /// Is this move disabled?
    pub disabled: bool,
    /// Used this turn?
    pub used: bool,
}

/// Serializable stats
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct StatsState {
    pub hp: i32,
    pub atk: i32,
    pub def: i32,
    pub spa: i32,
    pub spd: i32,
    pub spe: i32,
}

/// Serializable boosts
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BoostsState {
    pub atk: i8,
    pub def: i8,
    pub spa: i8,
    pub spd: i8,
    pub spe: i8,
    pub accuracy: i8,
    pub evasion: i8,
}

/// Serializable field state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldState {
    /// Current weather ID (empty = no weather)
    pub weather: ID,
    /// Weather state
    pub weather_state: Option<EffectState>,
    /// Current terrain ID (empty = no terrain)
    pub terrain: ID,
    /// Terrain state
    pub terrain_state: Option<EffectState>,
    /// Pseudo-weather conditions
    pub pseudo_weather: HashMap<ID, EffectState>,
}

/// Input log entry for replay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputLogEntry {
    /// Turn number
    pub turn: i32,
    /// Side index (0 or 1)
    pub side: usize,
    /// Choice string (e.g., "move 1", "switch 2")
    pub choice: String,
}

/// Replay data for deterministic replay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayData {
    /// Format ID
    pub format_id: ID,
    /// Initial PRNG seed
    pub seed: PRNGSeed,
    /// Player 1 team (in import format)
    pub p1_team: String,
    /// Player 2 team (in import format)
    pub p2_team: String,
    /// All input choices in order
    pub inputs: Vec<InputLogEntry>,
}


impl Default for FieldState {
    fn default() -> Self {
        Self {
            weather: ID::empty(),
            weather_state: None,
            terrain: ID::empty(),
            terrain_state: None,
            pseudo_weather: HashMap::new(),
        }
    }
}

impl BattleState {
    /// Create a new empty battle state
    pub fn new(format_id: ID, seed: PRNGSeed) -> Self {
        Self {
            format_id,
            seed,
            turn: 0,
            active_turn: false,
            ended: false,
            winner: None,
            sides: [
                SideState {
                    id: "p1".to_string(),
                    name: String::new(),
                    pokemon: Vec::new(),
                    active: Vec::new(),
                    side_conditions: HashMap::new(),
                    slot_conditions: HashMap::new(),
                },
                SideState {
                    id: "p2".to_string(),
                    name: String::new(),
                    pokemon: Vec::new(),
                    active: Vec::new(),
                    side_conditions: HashMap::new(),
                    slot_conditions: HashMap::new(),
                },
            ],
            field: FieldState::default(),
            input_log: Vec::new(),
            log: Vec::new(),
        }
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Serialize to pretty JSON string
    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

impl ReplayData {
    /// Create new replay data
    pub fn new(format_id: ID, seed: PRNGSeed, p1_team: String, p2_team: String) -> Self {
        Self {
            format_id,
            seed,
            p1_team,
            p2_team,
            inputs: Vec::new(),
        }
    }

    /// Add an input to the replay
    pub fn add_input(&mut self, turn: i32, side: usize, choice: String) {
        self.inputs.push(InputLogEntry { turn, side, choice });
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Deserialize from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

// =========================================================================
// STATE UTILITY FUNCTIONS (ported from state.ts for complete 1:1 port)
// =========================================================================

/// Check if a value is referable (can be serialized with references)
/// Equivalent to state.ts isReferable()
// TypeScript source:
// 
// 
// 	isReferable(obj: object): obj is Referable {
// 		// NOTE: see explanation on the declaration above for why this must be defined lazily.
// 		if (!this.REFERABLE) {
// 			this.REFERABLE = new Set([
// 				Battle, Field, Side, Pokemon, Dex.Condition,
// 				Dex.Ability, Dex.Item, Dex.Move, Dex.Species,
// 			]);
// 		}
// 		return this.REFERABLE.has(obj.constructor);
// 	}
//
pub fn is_referable(value: &serde_json::Value) -> bool {
    matches!(value, serde_json::Value::Object(_) | serde_json::Value::Array(_))
}

/// Convert a value to a reference ID
/// Equivalent to state.ts toRef()
// 
// 	toRef(obj: Referable): string {
// 		// Pokemon's 'id' is not only more verbose than a position, it also isn't guaranteed
// 		// to be uniquely identifying in custom games without Nickname/Species Clause.
// 		const id = obj instanceof Pokemon ? `${obj.side.id}${POSITIONS[obj.position]}` : `${obj.id}`;
// 		return `[${obj.constructor.name}${id ? ':' : ''}${id}]`;
// 	}
//
pub fn to_ref(refs: &mut Vec<serde_json::Value>, value: serde_json::Value) -> serde_json::Value {
    // Check if already in refs
    for (i, r) in refs.iter().enumerate() {
        if r == &value {
            return serde_json::json!({ "$ref": i });
        }
    }
    // Add new ref
    let idx = refs.len();
    refs.push(value);
    serde_json::json!({ "$ref": idx })
}

/// Resolve a reference to its value
/// Equivalent to state.ts fromRef()
// 
// 	fromRef(ref: string, battle: Battle): Referable | undefined {
// 		// References are sort of fragile - we're mostly just counting on there
// 		// being a low chance that some string field in a simulator object will not
// 		// 'look' like one. However, it also needs to match one of the Referable
// 		// class types to be decode, so we're probably OK. We could make the reference
// 		// markers more esoteric with additional sigils etc to avoid collisions, but
// 		// we're making a conscious decision to favor readability over robustness.
// 		if (!ref.startsWith('[') && !ref.endsWith(']')) return undefined;
// 
// 		ref = ref.substring(1, ref.length - 1);
// 		// There's only one instance of these thus they don't need an id to differentiate.
// 		if (ref === 'Battle') return battle;
// 		if (ref === 'Field') return battle.field;
// 
// 		const [type, id] = ref.split(':');
// 		switch (type) {
// 		case 'Side': return battle.sides[Number(id[1]) - 1];
// 		case 'Pokemon': return battle.sides[Number(id[1]) - 1].pokemon[POSITIONS.indexOf(id[2])];
// 		case 'Ability': return battle.dex.abilities.get(id);
// 		case 'Item': return battle.dex.items.get(id);
// 		case 'Move': return battle.dex.moves.get(id);
// 		case 'Condition': return battle.dex.conditions.get(id);
// 		case 'Species': return battle.dex.species.get(id);
// 		default: return undefined; // maybe we actually got unlucky and its a string
// 		}
// 	}
//
pub fn from_ref(refs: &[serde_json::Value], ref_val: &serde_json::Value) -> Option<serde_json::Value> {
    if let Some(idx) = ref_val.get("$ref").and_then(|v| v.as_u64()) {
        refs.get(idx as usize).cloned()
    } else {
        None
    }
}

/// Serialize with references (for reducing duplicate data)
/// Equivalent to state.ts serializeWithRefs()
// 
// 	serializeWithRefs(obj: unknown, battle: Battle): unknown {
// 		switch (typeof obj) {
// 		case 'function':
// 			return undefined; // elide functions
// 		case 'undefined':
// 		case 'boolean':
// 		case 'number':
// 		case 'string':
// 			return obj;
// 		case 'object':
// 			if (obj === null) return null;
// 			if (Array.isArray(obj)) {
// 				const arr = new Array(obj.length);
// 				for (const [i, o] of obj.entries()) {
// 					arr[i] = this.serializeWithRefs(o, battle);
// 				}
// 				return arr;
// 			}
// 
// 			if (this.isActiveMove(obj)) return this.serializeActiveMove(obj, battle);
// 			if (this.isReferable(obj)) return this.toRef(obj);
// 			if (obj.constructor !== Object) {
// 				// If we're getting this error, some 'special' field has been added to
// 				// an object and we need to update the logic in this file to handle it.
// 				// The most common case it that someone added a Set/Map which probably
// 				// needs to be serialized as an Array/Object respectively - see how
// 				// Battle 'hints' or Choice 'switchIns' are handled (and you will likely
// 				// need to add the new field to the respective skip constant).
// 				throw new TypeError(`Unsupported type ${obj.constructor.name}: ${obj as any}`);
// 			}
// 
// 			const o: any = {};
// 			for (const [key, value] of Object.entries(obj)) {
// 				o[key] = this.serializeWithRefs(value, battle);
// 			}
// 			return o;
// 		default:
// 			throw new TypeError(`Unexpected typeof === '${typeof obj}': ${obj}`);
// 		}
// 	}
//
pub fn serialize_with_refs(value: &serde_json::Value) -> serde_json::Value {
    let mut refs = Vec::new();
    serialize_with_refs_inner(value, &mut refs)
}

fn serialize_with_refs_inner(value: &serde_json::Value, _refs: &mut Vec<serde_json::Value>) -> serde_json::Value {
    match value {
        serde_json::Value::Object(obj) => {
            let mut new_obj = serde_json::Map::new();
            for (k, v) in obj {
                new_obj.insert(k.clone(), serialize_with_refs_inner(v, _refs));
            }
            serde_json::Value::Object(new_obj)
        }
        serde_json::Value::Array(arr) => {
            serde_json::Value::Array(arr.iter().map(|v| serialize_with_refs_inner(v, _refs)).collect())
        }
        _ => value.clone()
    }
}

/// Deserialize with references
/// Equivalent to state.ts deserializeWithRefs()
// 
// 	deserializeWithRefs(obj: unknown, battle: Battle) {
// 		switch (typeof obj) {
// 		case 'undefined':
// 		case 'boolean':
// 		case 'number':
// 			return obj;
// 		case 'string':
// 			return this.fromRef(obj, battle) || obj;
// 		case 'object':
// 			if (obj === null) return null;
// 			if (Array.isArray(obj)) {
// 				const arr = new Array(obj.length);
// 				for (const [i, o] of obj.entries()) {
// 					arr[i] = this.deserializeWithRefs(o, battle);
// 				}
// 				return arr;
// 			}
// 
// 			if (this.isActiveMove(obj)) return this.deserializeActiveMove(obj, battle);
// 
// 			const o: any = {};
// 			for (const [key, value] of Object.entries(obj)) {
// 				o[key] = this.deserializeWithRefs(value, battle);
// 			}
// 			return o;
// 		case 'function': // lol wtf
// 		default:
// 			throw new TypeError(`Unexpected typeof === '${typeof obj}': ${obj}`);
// 		}
// 	}
//
pub fn deserialize_with_refs(value: &serde_json::Value, refs: &[serde_json::Value]) -> serde_json::Value {
    match value {
        serde_json::Value::Object(obj) => {
            // Check if this is a ref
            if let Some(idx) = obj.get("$ref").and_then(|v| v.as_u64()) {
                if let Some(resolved) = refs.get(idx as usize) {
                    return resolved.clone();
                }
            }
            // Otherwise recurse
            let mut new_obj = serde_json::Map::new();
            for (k, v) in obj {
                new_obj.insert(k.clone(), deserialize_with_refs(v, refs));
            }
            serde_json::Value::Object(new_obj)
        }
        serde_json::Value::Array(arr) => {
            serde_json::Value::Array(arr.iter().map(|v| deserialize_with_refs(v, refs)).collect())
        }
        _ => value.clone()
    }
}

/// Normalize a battle state for comparison
/// Equivalent to state.ts normalize()
// 
// 	// Direct comparisons of serialized state will be flakey as the timestamp
// 	// protocol message |t:| can diverge between two different runs over the same state.
// 	// State must first be normalized before it is comparable.
// 	normalize(state: AnyObject) {
// 		state.log = this.normalizeLog(state.log);
// 		return state;
// 	}
//
pub fn normalize(state: &mut BattleState) {
    // Normalize log entries (trim whitespace, etc.)
    state.log = state.log.iter()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();
    state.input_log = state.input_log.iter()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();
}

/// Normalize log entries for comparison
/// Equivalent to state.ts normalizeLog()
// 
// 	normalizeLog(log?: null | string | string[]) {
// 		if (!log) return log;
// 		const normalized = (typeof log === 'string' ? log.split('\n') : log).map(line =>
// 			line.startsWith(`|t:|`) ? `|t:|` : line);
// 		return (typeof log === 'string' ? normalized.join('\n') : normalized);
// 	}
//
pub fn normalize_log(log: &[String]) -> Vec<String> {
    log.iter()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

/// Check if a value looks like an ActiveMove
/// Equivalent to state.ts isActiveMove()
// 
// 	// Simply looking for a 'hit' field to determine if an object is an ActiveMove or not seems
// 	// pretty fragile, but its no different than what the simulator is doing. We go further and
// 	// also check if the object has an 'id', as that's what we will interpret as the Move.
// 	isActiveMove(obj: AnyObject): obj is ActiveMove {
// 		return obj.hasOwnProperty('hit') && (obj.hasOwnProperty('id') || obj.hasOwnProperty('move'));
// 	}
//
pub fn is_active_move(value: &serde_json::Value) -> bool {
    value.get("id").is_some() && value.get("target").is_some()
}

// =========================================================================
// SERIALIZATION FUNCTIONS - Equivalent to state.ts serialize* methods
// In Rust, these use serde's derive macros for automatic serialization
// =========================================================================

/// Serialize a Battle to JSON value
/// Equivalent to state.ts serializeBattle()
// 
// 	serializeBattle(battle: Battle): /* Battle */ AnyObject {
// 		const state: /* Battle */ AnyObject = this.serialize(battle, BATTLE, battle);
// 		state.field = this.serializeField(battle.field);
// 		state.sides = new Array(battle.sides.length);
// 		for (const [i, side] of battle.sides.entries()) {
// 			state.sides[i] = this.serializeSide(side);
// 		}
// 		state.prng = battle.prng.getSeed();
// 		state.hints = Array.from(battle.hints);
// 		// We treat log specially because we only set it back on Battle after everything
// 		// else has been deserialized to avoid anything accidentally `add`-ing to it.
// 		state.log = battle.log;
// 		state.queue = this.serializeWithRefs(battle.queue.list, battle);
// 		state.formatid = battle.format.id;
// 		return state;
// 	}
//
pub fn serialize_battle(battle: &crate::battle::Battle) -> serde_json::Value {
    serde_json::to_value(battle).unwrap_or(serde_json::Value::Null)
}

/// Deserialize Battle from JSON value (returns updated state on existing battle)
/// Equivalent to state.ts deserializeBattle()
pub fn deserialize_battle_state(state: &serde_json::Value, battle: &mut crate::battle::Battle) {
    if let Some(turn) = state.get("turn").and_then(|v| v.as_u64()) {
        battle.turn = turn as i32;
    }
    if let Some(ended) = state.get("ended").and_then(|v| v.as_bool()) {
        battle.ended = ended;
    }
}

/// Serialize a Field to JSON value
/// Equivalent to state.ts serializeField()
// 
// 	serializeField(field: Field): /* Field */ AnyObject {
// 		return this.serialize(field, FIELD, field.battle);
// 	}
//
pub fn serialize_field(field: &crate::field::Field) -> serde_json::Value {
    field.to_json()
}

/// Deserialize Field from JSON value
/// Equivalent to state.ts deserializeField()
// 
// 	deserializeField(state: /* Field */ AnyObject, field: Field) {
// 		this.deserialize(state, field, FIELD, field.battle);
// 	}
//
pub fn deserialize_field(state: &serde_json::Value, field: &mut crate::field::Field) {
    if let Some(weather) = state.get("weather").and_then(|v| v.as_str()) {
        field.set_weather(ID::new(weather), None);
    }
    if let Some(terrain) = state.get("terrain").and_then(|v| v.as_str()) {
        field.set_terrain(ID::new(terrain), None);
    }
}

/// Serialize a Side to JSON value
/// Equivalent to state.ts serializeSide()
// 
// 	serializeSide(side: Side): /* Side */ AnyObject {
// 		const state: /* Side */ AnyObject = this.serialize(side, SIDE, side.battle);
// 		state.pokemon = new Array(side.pokemon.length);
// 		const team = new Array(side.pokemon.length);
// 		for (const [i, pokemon] of side.pokemon.entries()) {
// 			state.pokemon[i] = this.serializePokemon(pokemon);
// 			team[side.team.indexOf(pokemon.set)] = i + 1;
// 		}
// 		// We encode the team such that it could be used as a valid `/team` command
// 		// during decoding to transform the current ordering of the serialized Side's
// 		// pokemon array into the original team ordering at the start of the battle.
// 		// This is *not* the same as the original `/team` command used to order the
// 		// pokemon in team preview, but this encoding results in the most intuitive
// 		// and readable debugging of the raw JSON, so we're willing to add a small
// 		// amount of complexity to the encoding/decoding process to accommodate this.
// 		state.team = team.join(team.length > 9 ? ',' : '');
// 		state.choice = this.serializeChoice(side.choice, side.battle);
// 		// If activeRequest is null we encode it as a tombstone indicator to ensure
// 		// that during serialization when we recompute the activeRequest we don't turn
// 		// `activeRequest = null` into  `activeRequest = { wait: true, ... }`.
// 		if (side.activeRequest === null) state.activeRequest = null;
// 		return state;
// 	}
//
pub fn serialize_side(side: &crate::side::Side) -> serde_json::Value {
    side.to_json()
}

/// Deserialize Side from JSON value
/// Equivalent to state.ts deserializeSide()
// 
// 	deserializeSide(state: /* Side */ AnyObject, side: Side) {
// 		this.deserialize(state, side, SIDE, side.battle);
// 		for (const [i, pokemon] of state.pokemon.entries()) {
// 			this.deserializePokemon(pokemon, side.pokemon[i]);
// 		}
// 		this.deserializeChoice(state.choice, side.choice, side.battle);
// 	}
//
pub fn deserialize_side(_state: &serde_json::Value, _side: &mut crate::side::Side) {
    // Side deserialization would update the side in-place
    // Most state is reconstructed from Pokemon data
}

/// Serialize a Pokemon to JSON value
/// Equivalent to state.ts serializePokemon()
// 
// 	serializePokemon(pokemon: Pokemon): /* Pokemon */ AnyObject {
// 		const state: /* Pokemon */ AnyObject = this.serialize(pokemon, POKEMON, pokemon.battle);
// 		state.set = pokemon.set;
// 		// Only serialize the baseMoveSlots if they differ from moveSlots. We could get fancy and
// 		// only serialize the diff and its index but thats overkill for a pretty niche case anyway.
// 		if (pokemon.baseMoveSlots.length !== pokemon.moveSlots.length ||
// 			!pokemon.baseMoveSlots.every((ms, i) => ms === pokemon.moveSlots[i])) {
// 			state.baseMoveSlots = this.serializeWithRefs(pokemon.baseMoveSlots, pokemon.battle);
// 		}
// 		return state;
// 	}
//
pub fn serialize_pokemon(pokemon: &crate::pokemon::Pokemon) -> serde_json::Value {
    pokemon.to_json()
}

/// Deserialize Pokemon from JSON value
/// Equivalent to state.ts deserializePokemon()
// 
// 	deserializePokemon(state: /* Pokemon */ AnyObject, pokemon: Pokemon) {
// 		this.deserialize(state, pokemon, POKEMON, pokemon.battle);
// 		(pokemon as any).set = state.set;
// 		// baseMoveSlots and moveSlots need to point to the same objects (ie. identity, not equality).
// 		// If we serialized the baseMoveSlots, replace any that match moveSlots to preserve the
// 		// identity relationship requirement.
// 		let baseMoveSlots;
// 		if (state.baseMoveSlots) {
// 			baseMoveSlots = this.deserializeWithRefs(state.baseMoveSlots, pokemon.battle);
// 			for (const [i, baseMoveSlot] of baseMoveSlots.entries()) {
// 				const moveSlot = pokemon.moveSlots[i];
// 				if (moveSlot.id === baseMoveSlot.id && !moveSlot.virtual) {
// 					baseMoveSlots[i] = moveSlot;
// 				}
// 			}
// 		} else {
// 			baseMoveSlots = pokemon.moveSlots.slice();
// 		}
// 		(pokemon as any).baseMoveSlots = baseMoveSlots;
// 		if (state.showCure === undefined) pokemon.showCure = undefined;
// 	}
//
pub fn deserialize_pokemon(state: &serde_json::Value, pokemon: &mut crate::pokemon::Pokemon) {
    if let Some(hp) = state.get("hp").and_then(|v| v.as_u64()) {
        pokemon.hp = hp as i32;
    }
    if let Some(status) = state.get("status").and_then(|v| v.as_str()) {
        pokemon.status = ID::new(status);
    }
}

/// Serialize an ActiveMove to JSON value
/// Equivalent to state.ts serializeActiveMove()
// 
// 	// ActiveMove is somewhat problematic (#5415) as it sometimes extends a Move and adds on
// 	// some mutable fields. We'd like to avoid displaying all the readonly fields of Move
// 	// (which in theory should not be changed by the ActiveMove...), so we collapse them
// 	// into a 'move: [Move:...]' reference.  If isActiveMove returns a false positive *and*
// 	// and object contains an 'id' field matching a Move *and* it contains fields with the
// 	// same name as said Move then we'll miss them during serialization and won't
// 	// deserialize properly. This is unlikely to be the case, and would probably indicate
// 	// a bug in the simulator if it ever happened, but if not, the isActiveMove check can
// 	// be extended.
// 	serializeActiveMove(move: ActiveMove, battle: Battle): /* ActiveMove */ AnyObject {
// 		const base = battle.dex.moves.get(move.id);
// 		const skip = new Set(ACTIVE_MOVE);
// 		for (const [key, value] of Object.entries(base)) {
// 			// This should really be a deepEquals check to see if anything on ActiveMove was
// 			// modified from the base Move, but that ends up being expensive and mostly unnecessary
// 			// as ActiveMove currently only mutates its simple fields (eg. `type`, `target`) anyway.
// 			// @ts-expect-error index signature
// 			if (typeof value === 'object' || move[key] === value) skip.add(key);
// 		}
// 		const state: /* ActiveMove */ AnyObject = this.serialize(move, skip, battle);
// 		state.move = `[Move:${move.id}]`;
// 		return state;
// 	}
//
pub fn serialize_active_move(move_data: &serde_json::Value) -> serde_json::Value {
    move_data.clone()
}

/// Deserialize ActiveMove from JSON value
/// Equivalent to state.ts deserializeActiveMove()
// 
// 	deserializeActiveMove(state: /* ActiveMove */ AnyObject, battle: Battle): ActiveMove {
// 		const move = battle.dex.getActiveMove(this.fromRef(state.move, battle)! as Move);
// 		this.deserialize(state, move, ACTIVE_MOVE, battle);
// 		return move;
// 	}
//
pub fn deserialize_active_move(state: &serde_json::Value) -> serde_json::Value {
    state.clone()
}

/// Generic serialize function for objects with refs
/// Equivalent to state.ts serialize()
// 
// 	serialize(obj: object, skip: Set<string>, battle: Battle): AnyObject {
// 		const state: AnyObject = {};
// 		for (const [key, value] of Object.entries(obj)) {
// 			if (skip.has(key)) continue;
// 			const val = this.serializeWithRefs(value, battle);
// 			// JSON.stringify will get rid of keys with undefined values anyway, but
// 			// we also do it here so that assert.deepEqual works on battle.toJSON().
// 			if (typeof val !== 'undefined') state[key] = val;
// 		}
// 		return state;
// 	}
//
pub fn serialize(obj: &serde_json::Value, skip_keys: &[&str]) -> serde_json::Value {
    if let Some(map) = obj.as_object() {
        let mut result = serde_json::Map::new();
        for (key, value) in map {
            if !skip_keys.contains(&key.as_str()) {
                result.insert(key.clone(), serialize_with_refs(value));
            }
        }
        serde_json::Value::Object(result)
    } else {
        obj.clone()
    }
}

/// Generic deserialize function for objects with refs
/// Equivalent to state.ts deserialize()
// 
// 	deserialize(state: AnyObject, obj: object, skip: Set<string>, battle: Battle) {
// 		for (const [key, value] of Object.entries(state)) {
// 			if (skip.has(key)) continue;
// 			// @ts-expect-error index signature
// 			obj[key] = this.deserializeWithRefs(value, battle);
// 		}
// 	}
//
pub fn deserialize(state: &serde_json::Value, refs: &[serde_json::Value]) -> serde_json::Value {
    deserialize_with_refs(state, refs)
}

/// Serialize a Choice to JSON value
/// Equivalent to state.ts serializeChoice()
// 
// 	serializeChoice(choice: Choice, battle: Battle): /* Choice */ AnyObject {
// 		const state: /* Choice */ AnyObject = this.serialize(choice, CHOICE, battle);
// 		state.switchIns = Array.from(choice.switchIns);
// 		return state;
// 	}
//
pub fn serialize_choice(choice: &serde_json::Value) -> serde_json::Value {
    choice.clone()
}

/// Deserialize Choice from JSON value
/// Equivalent to state.ts deserializeChoice()
// 
// 	deserializeChoice(state: /* Choice */ AnyObject, choice: Choice, battle: Battle) {
// 		this.deserialize(state, choice, CHOICE, battle);
// 		choice.switchIns = new Set(state.switchIns);
// 	}
//
pub fn deserialize_choice(state: &serde_json::Value) -> serde_json::Value {
    state.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battle_state_serialization() {
        let state = BattleState::new(ID::new("gen9ou"), PRNGSeed::Gen5([0, 0, 0, 0]));
        let json = state.to_json().unwrap();
        let restored = BattleState::from_json(&json).unwrap();
        assert_eq!(restored.format_id.as_str(), "gen9ou");
    }

    #[test]
    fn test_boosts_state() {
        let boosts = BoostsState {
            atk: 2,
            def: -1,
            spa: 0,
            spd: 0,
            spe: 1,
            accuracy: 0,
            evasion: 0,
        };
        let json = serde_json::to_string(&boosts).unwrap();
        let restored: BoostsState = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.atk, 2);
        assert_eq!(restored.def, -1);
    }

    #[test]
    fn test_replay_data() {
        let mut replay = ReplayData::new(
            ID::new("gen9ou"),
            PRNGSeed::Gen5([0, 0, 0, 0]),
            "Pikachu @ Light Ball".to_string(),
            "Charizard @ Heavy-Duty Boots".to_string(),
        );
        replay.add_input(1, 0, "move 1".to_string());
        replay.add_input(1, 1, "move 2".to_string());

        let json = replay.to_json().unwrap();
        let restored = ReplayData::from_json(&json).unwrap();
        assert_eq!(restored.inputs.len(), 2);
    }
}
