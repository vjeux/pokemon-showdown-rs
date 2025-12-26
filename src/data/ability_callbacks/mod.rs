//! Ability Callback Handlers
//!
//! This module exports all ability callback implementations.
//! Each ability is in its own file.

// Common types
mod common;
pub use common::*;

// Individual ability modules
pub mod adaptability;
pub mod aerilate;
pub mod aftermath;
pub mod airlock;
pub mod analytic;
pub mod angerpoint;
pub mod angershell;
pub mod anticipation;
pub mod arenatrap;
pub mod armortail;
pub mod aromaveil;
pub mod asoneglastrier;
pub mod asonespectrier;
pub mod aurabreak;
pub mod baddreams;
pub mod ballfetch;
pub mod battery;
pub mod battlearmor;
pub mod battlebond;
pub mod beadsofruin;
pub mod beastboost;
pub mod berserk;
pub mod bigpecks;
pub mod blaze;
pub mod bulletproof;
pub mod cheekpouch;
pub mod chillingneigh;
pub mod chlorophyll;
pub mod clearbody;
pub mod cloudnine;
pub mod colorchange;
pub mod comatose;
pub mod commander;
pub mod competitive;
pub mod compoundeyes;
pub mod contrary;
pub mod corrosion;
pub mod costar;
pub mod cottondown;
pub mod cudchew;
pub mod curiousmedicine;
pub mod cursedbody;
pub mod cutecharm;
pub mod damp;
pub mod dancer;
pub mod darkaura;
pub mod dauntlessshield;
pub mod dazzling;
pub mod defeatist;
pub mod defiant;
pub mod deltastream;
pub mod desolateland;
pub mod disguise;
pub mod download;
pub mod dragonsmaw;
pub mod drizzle;
pub mod drought;
pub mod dryskin;
pub mod earlybird;
pub mod eartheater;
pub mod effectspore;
pub mod electricsurge;
pub mod electromorphosis;
pub mod embodyaspectcornerstone;
pub mod embodyaspecthearthflame;
pub mod embodyaspectteal;
pub mod embodyaspectwellspring;
pub mod emergencyexit;
pub mod fairyaura;
pub mod filter;
pub mod flamebody;
pub mod flareboost;
pub mod flashfire;
pub mod flowergift;
pub mod flowerveil;
pub mod fluffy;
pub mod forecast;
pub mod forewarn;
pub mod friendguard;
pub mod frisk;
pub mod fullmetalbody;
pub mod furcoat;
pub mod galewings;
pub mod galvanize;
pub mod gluttony;
pub mod goodasgold;
pub mod gooey;
pub mod gorillatactics;
pub mod grasspelt;
pub mod grassysurge;
pub mod grimneigh;
pub mod guarddog;
pub mod gulpmissile;
pub mod guts;
pub mod hadronengine;
pub mod harvest;
pub mod healer;
pub mod heatproof;
pub mod heavymetal;
pub mod honeygather;
pub mod hospitality;
pub mod hugepower;
pub mod hungerswitch;
pub mod hustle;
pub mod hydration;
pub mod hypercutter;
pub mod icebody;
pub mod iceface;
pub mod icescales;
pub mod illuminate;
pub mod illusion;
pub mod immunity;
pub mod imposter;
pub mod infiltrator;
pub mod innardsout;
pub mod innerfocus;
pub mod insomnia;
pub mod intimidate;
pub mod intrepidsword;
pub mod ironbarbs;
pub mod ironfist;
pub mod justified;
pub mod keeneye;
pub mod klutz;
pub mod leafguard;
pub mod levitate;
pub mod libero;
pub mod lightmetal;
pub mod lightningrod;
pub mod limber;
pub mod lingeringaroma;
pub mod liquidooze;
pub mod liquidvoice;
pub mod longreach;
pub mod magicbounce;
pub mod magicguard;
pub mod magician;
pub mod magmaarmor;
pub mod magnetpull;
pub mod marvelscale;
pub mod megalauncher;
pub mod merciless;
pub mod mimicry;
pub mod mindseye;
pub mod minus;
pub mod mirrorarmor;
pub mod mistysurge;
pub mod moldbreaker;
pub mod moody;
pub mod motordrive;
pub mod mountaineer;
pub mod moxie;
pub mod multiscale;
pub mod multitype;
pub mod mummy;
pub mod myceliummight;
pub mod naturalcure;
pub mod neuroforce;
pub mod neutralizinggas;
pub mod noability;
pub mod noguard;
pub mod normalize;
pub mod oblivious;
pub mod opportunist;
pub mod orichalcumpulse;
pub mod overcoat;
pub mod overgrow;
pub mod owntempo;
pub mod parentalbond;
pub mod pastelveil;
pub mod perishbody;
pub mod persistent;
pub mod pickpocket;
pub mod pickup;
pub mod pixilate;
pub mod plus;
pub mod poisonheal;
pub mod poisonpoint;
pub mod poisonpuppeteer;
pub mod poisontouch;
pub mod powerconstruct;
pub mod powerofalchemy;
pub mod powerspot;
pub mod prankster;
pub mod pressure;
pub mod primordialsea;
pub mod prismarmor;
pub mod propellertail;
pub mod protean;
pub mod protosynthesis;
pub mod psychicsurge;
pub mod punkrock;
pub mod purepower;
pub mod purifyingsalt;
pub mod quarkdrive;
pub mod queenlymajesty;
pub mod quickdraw;
pub mod quickfeet;
pub mod raindish;
pub mod rattled;
pub mod rebound;
pub mod receiver;
pub mod reckless;
pub mod refrigerate;
pub mod regenerator;
pub mod ripen;
pub mod rivalry;
pub mod rkssystem;
pub mod rockhead;
pub mod rockypayload;
pub mod roughskin;
pub mod runaway;
pub mod sandforce;
pub mod sandrush;
pub mod sandspit;
pub mod sandstream;
pub mod sandveil;
pub mod sapsipper;
pub mod schooling;
pub mod scrappy;
pub mod screencleaner;
pub mod seedsower;
pub mod serenegrace;
pub mod shadowshield;
pub mod shadowtag;
pub mod sharpness;
pub mod shedskin;
pub mod sheerforce;
pub mod shellarmor;
pub mod shielddust;
pub mod shieldsdown;
pub mod simple;
pub mod skilllink;
pub mod slowstart;
pub mod slushrush;
pub mod sniper;
pub mod snowcloak;
pub mod snowwarning;
pub mod solarpower;
pub mod solidrock;
pub mod soulheart;
pub mod soundproof;
pub mod speedboost;
pub mod stakeout;
pub mod stall;
pub mod stalwart;
pub mod stamina;
pub mod stancechange;
pub mod r#static;
pub mod steadfast;
pub mod steamengine;
pub mod steelworker;
pub mod steelyspirit;
pub mod stench;
pub mod stickyhold;
pub mod stormdrain;
pub mod strongjaw;
pub mod sturdy;
pub mod suctioncups;
pub mod superluck;
pub mod supersweetsyrup;
pub mod supremeoverlord;
pub mod surgesurfer;
pub mod swarm;
pub mod sweetveil;
pub mod swiftswim;
pub mod swordofruin;
pub mod symbiosis;
pub mod synchronize;
pub mod tabletsofruin;
pub mod tangledfeet;
pub mod tanglinghair;
pub mod technician;
pub mod telepathy;
pub mod teraformzero;
pub mod terashell;
pub mod terashift;
pub mod teravolt;
pub mod thermalexchange;
pub mod thickfat;
pub mod tintedlens;
pub mod torrent;
pub mod toughclaws;
pub mod toxicboost;
pub mod toxicchain;
pub mod toxicdebris;
pub mod trace;
pub mod transistor;
pub mod triage;
pub mod truant;
pub mod turboblaze;
pub mod unaware;
pub mod unburden;
pub mod unnerve;
pub mod unseenfist;
pub mod vesselofruin;
pub mod victorystar;
pub mod vitalspirit;
pub mod voltabsorb;
pub mod wanderingspirit;
pub mod waterabsorb;
pub mod waterbubble;
pub mod watercompaction;
pub mod waterveil;
pub mod weakarmor;
pub mod wellbakedbody;
pub mod whitesmoke;
pub mod wimpout;
pub mod windpower;
pub mod windrider;
pub mod wonderguard;
pub mod wonderskin;
pub mod zenmode;
pub mod zerotohero;

use crate::battle::Battle;
use crate::event::EventResult;
use crate::dex_data::ID;

// ===========================================
// Ability Callback Dispatch Functions
// ===========================================
// These functions route ability events to their specific callback implementations.
// Each function matches on ability_id and calls the corresponding module's callback.
//
// Note: Only abilities with properly implemented callbacks are included.
// TODO: Add more abilities as their callbacks are implemented.

/// Dispatch onModifySTAB callbacks (read-only)
pub fn dispatch_on_modify_stab(
    battle: &Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> Option<EventResult> {
    match ability_id {
        "adaptability" => Some(EventResult::Modify(2.0 / 1.5)), // 1.5x STAB -> 2x STAB
        // TODO: Add other STAB-modifying abilities when implemented
        _ => None,
    }
}

/// Dispatch onSwitchIn callbacks (mutates battle)
pub fn dispatch_on_switch_in(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> Option<EventResult> {
    match ability_id {
        "intimidate" => {
            // Lower foe's Attack
            let (side_idx, poke_idx) = pokemon_pos;
            let foe_side = if side_idx == 0 { 1 } else { 0 };
            let mut targets = Vec::new();
            if let Some(foe) = battle.sides.get(foe_side) {
                for slot in 0..foe.active.len() {
                    if foe.active[slot].is_some() {
                        targets.push((foe_side, slot));
                    }
                }
            }
            for target_pos in targets {
                battle.boost(&[("atk", -1)], target_pos, Some(pokemon_pos), Some("Intimidate"));
            }
            Some(EventResult::Stop)
        }
        "drizzle" => {
            battle.field.set_weather(ID::new("rain"), None);
            Some(EventResult::Stop)
        }
        "drought" => {
            battle.field.set_weather(ID::new("sunnyday"), None);
            Some(EventResult::Stop)
        }
        "sandstream" => {
            battle.field.set_weather(ID::new("sandstorm"), None);
            Some(EventResult::Stop)
        }
        "snowwarning" => {
            battle.field.set_weather(ID::new("snow"), None);
            Some(EventResult::Stop)
        }
        // TODO: Add electricsurge, grassysurge, mistysurge, psychicsurge (terrain setters)
        // TODO: Add download, trace, imposter, etc.
        _ => None,
    }
}

/// Dispatch onSourceModifyDamage callbacks (read-only)
/// For defender's abilities that reduce incoming damage
pub fn dispatch_on_source_modify_damage(
    battle: &Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> Option<EventResult> {
    match ability_id {
        "multiscale" => {
            // Halve damage when at full HP
            let (side_idx, poke_idx) = pokemon_pos;
            if let Some(side) = battle.sides.get(side_idx) {
                if let Some(pokemon) = side.pokemon.get(poke_idx) {
                    if pokemon.hp == pokemon.maxhp {
                        return Some(EventResult::Modify(0.5));
                    }
                }
            }
            None
        }
        "filter" | "solidrock" | "prismarmor" => {
            // Reduce super effective damage to 0.75x
            // TODO: Check if move is super effective
            // For now, cannot check type effectiveness in event context
            None
        }
        // TODO: Add fluffy, icescales, punkrock, etc.
        _ => None,
    }
}

/// Dispatch onModifyDamage callbacks (read-only)
/// For attacker's abilities that modify outgoing damage
pub fn dispatch_on_modify_damage(
    battle: &Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> Option<EventResult> {
    match ability_id {
        // TODO: Add abilities that modify damage based on conditions
        // Examples: sheerforce, ironfist, megalauncher, toughclaws, etc.
        _ => None,
    }
}

/// Dispatch onResidual callbacks (mutates battle)
/// For end-of-turn ability effects
pub fn dispatch_on_residual(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> Option<EventResult> {
    let (side_idx, poke_idx) = pokemon_pos;

    match ability_id {
        "poisonheal" => {
            if let Some(side) = battle.sides.get(side_idx) {
                if let Some(pokemon) = side.pokemon.get(poke_idx) {
                    if pokemon.status.as_str() == "tox" || pokemon.status.as_str() == "psn" {
                        let heal = pokemon.maxhp / 8;
                        battle.heal(heal as i32, Some(pokemon_pos), None, Some(&ID::new("Poison Heal")));
                        return Some(EventResult::Stop);
                    }
                }
            }
            None
        }
        "hydration" => {
            if let Some(side) = battle.sides.get(side_idx) {
                if let Some(pokemon) = side.pokemon.get(poke_idx) {
                    if !pokemon.status.is_empty() {
                        let weather = battle.field.weather.as_str();
                        if weather == "raindance" || weather == "primordialsea" {
                            battle.add("-activate", &[
                                crate::battle::Arg::String(pokemon.name.clone()),
                                crate::battle::Arg::Str("ability: Hydration")
                            ]);
                            battle.sides[side_idx].pokemon[poke_idx].cure_status();
                            return Some(EventResult::Stop);
                        }
                    }
                }
            }
            None
        }
        "baddreams" => {
            if let Some(side) = battle.sides.get(side_idx) {
                if let Some(pokemon) = side.pokemon.get(poke_idx) {
                    if pokemon.hp == 0 {
                        return Some(EventResult::Continue);
                    }
                    // Damage sleeping foes
                    let foe_side = if side_idx == 0 { 1 } else { 0 };
                    let mut foes_to_damage: Vec<(usize, u32)> = Vec::new();

                    if let Some(foe_side_ref) = battle.sides.get(foe_side) {
                        for foe in foe_side_ref.pokemon.iter().filter(|p| p.is_active && !p.fainted) {
                            if foe.status.as_str() == "slp" || foe.ability.as_str() == "comatose" {
                                let damage = foe.maxhp / 8;
                                foes_to_damage.push((foe.position, damage));
                            }
                        }
                    }

                    for (foe_pos, dmg) in foes_to_damage {
                        battle.damage(dmg as i32, Some((foe_side, foe_pos)), Some(pokemon_pos), None, false);
                    }
                    return Some(EventResult::Stop);
                }
            }
            None
        }
        "healer" => {
            // TODO: Implement healer - 30% chance to cure status of adjacent allies
            None
        }
        "raindish" => {
            // TODO: Heal 1/16 in rain
            None
        }
        "dryskin" => {
            // TODO: Heal 1/8 in rain, lose 1/8 in sun
            None
        }
        "icebody" => {
            // TODO: Heal 1/16 in snow
            None
        }
        "shedskin" => {
            // TODO: 33% chance to cure status
            None
        }
        // TODO: Add speedboost, moody, etc.
        _ => None,
    }
}

/// Dispatch onModifyAtk callbacks (read-only)
/// For abilities that modify Attack stat
pub fn dispatch_on_modify_atk(
    battle: &Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> Option<EventResult> {
    match ability_id {
        "hugepower" | "purepower" => Some(EventResult::Modify(2.0)),
        "gorillatactics" => Some(EventResult::Modify(1.5)),
        "defeatist" => {
            // Halve Attack when HP < 50%
            let (side_idx, poke_idx) = pokemon_pos;
            if let Some(side) = battle.sides.get(side_idx) {
                if let Some(pokemon) = side.pokemon.get(poke_idx) {
                    if pokemon.hp < pokemon.maxhp / 2 {
                        return Some(EventResult::Modify(0.5));
                    }
                }
            }
            None
        }
        // TODO: Add guts, hustle, flowergift, slowstart, etc.
        _ => None,
    }
}

/// Dispatch onModifySpA callbacks (read-only)
/// For abilities that modify Special Attack stat
pub fn dispatch_on_modify_sp_a(
    battle: &Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> Option<EventResult> {
    match ability_id {
        "defeatist" => {
            // Halve SpA when HP < 50%
            let (side_idx, poke_idx) = pokemon_pos;
            if let Some(side) = battle.sides.get(side_idx) {
                if let Some(pokemon) = side.pokemon.get(poke_idx) {
                    if pokemon.hp < pokemon.maxhp / 2 {
                        return Some(EventResult::Modify(0.5));
                    }
                }
            }
            None
        }
        // TODO: Add solarpower, flowergift, etc.
        _ => None,
    }
}

/// Dispatch onModifyDef callbacks (read-only)
/// For abilities that modify Defense stat
pub fn dispatch_on_modify_def(
    battle: &Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> Option<EventResult> {
    match ability_id {
        "furcoat" => Some(EventResult::Modify(2.0)),
        "marvelscale" => {
            // 1.5x Defense when statused
            let (side_idx, poke_idx) = pokemon_pos;
            if let Some(side) = battle.sides.get(side_idx) {
                if let Some(pokemon) = side.pokemon.get(poke_idx) {
                    if !pokemon.status.is_empty() {
                        return Some(EventResult::Modify(1.5));
                    }
                }
            }
            None
        }
        // TODO: Add grasspelt, flowergift, etc.
        _ => None,
    }
}

/// Dispatch onModifySpD callbacks (read-only)
/// For abilities that modify Special Defense stat
pub fn dispatch_on_modify_sp_d(
    battle: &Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> Option<EventResult> {
    match ability_id {
        // TODO: Add flowergift, etc.
        _ => None,
    }
}

/// Dispatch onModifySpe callbacks (read-only)
/// For abilities that modify Speed stat
pub fn dispatch_on_modify_spe(
    battle: &Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> Option<EventResult> {
    match ability_id {
        "quickfeet" => {
            // 1.5x Speed when statused
            let (side_idx, poke_idx) = pokemon_pos;
            if let Some(side) = battle.sides.get(side_idx) {
                if let Some(pokemon) = side.pokemon.get(poke_idx) {
                    if !pokemon.status.is_empty() {
                        return Some(EventResult::Modify(1.5));
                    }
                }
            }
            None
        }
        "slowstart" => {
            // TODO: Halve Speed for first 5 turns
            None
        }
        "unburden" => {
            // TODO: 2x Speed after using/losing item
            None
        }
        // TODO: Add chlorophyll, swiftswim, sandrush, slushrush, surgesurfer, etc.
        _ => None,
    }
}

/// Dispatch onBasePower callbacks (read-only)
/// For abilities that modify move base power
pub fn dispatch_on_base_power(
    battle: &Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> Option<EventResult> {
    match ability_id {
        // TODO: Add ironfist, megalauncher, toughclaws, technician, sheerforce, etc.
        // TODO: Add type-boosting abilities (blaze, torrent, overgrow, swarm)
        _ => None,
    }
}

/// Dispatch onModifyAccuracy callbacks (read-only)
/// For abilities that modify move accuracy
pub fn dispatch_on_modify_accuracy(
    battle: &Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> Option<EventResult> {
    match ability_id {
        "compoundeyes" => Some(EventResult::Modify(1.3)),
        "hustle" => {
            // TODO: Check if move is physical - if so, 0.8x accuracy
            None
        }
        // TODO: Add victorystar, tangledfeet, etc.
        _ => None,
    }
}

/// Dispatch onModifyCritRatio callbacks (read-only)
/// For abilities that modify critical hit ratio
pub fn dispatch_on_modify_crit_ratio(
    battle: &Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> Option<EventResult> {
    match ability_id {
        "superluck" => Some(EventResult::Modify(1.0)), // +1 stage
        // TODO: Properly implement crit ratio modification (this is a placeholder)
        _ => None,
    }
}