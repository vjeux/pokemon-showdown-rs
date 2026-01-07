//! Ability Callback Handlers
//!
//! This module exports all ability callback implementations.
//! Each ability with callbacks is in its own file.

use crate::battle::Battle;
use crate::event::EventResult;

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
pub mod battery;
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
pub mod costar;
pub mod cottondown;
pub mod cudchew;
pub mod curiousmedicine;
pub mod cursedbody;
pub mod cutecharm;
pub mod damp;
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
pub mod mummy;
pub mod myceliummight;
pub mod naturalcure;
pub mod neuroforce;
pub mod neutralizinggas;
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
pub mod rockhead;
pub mod rockypayload;
pub mod roughskin;
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

// Dispatch functions
/// Dispatch onAfterBoost callbacks
pub fn dispatch_on_after_boost(
    battle: &mut Battle,
    ability_id: &str,
    boost: &crate::dex_data::BoostsTable,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    match ability_id {
        "rattled" => rattled::on_after_boost(battle, boost, target_pos, source_pos, effect_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onAfterEachBoost callbacks
pub fn dispatch_on_after_each_boost(
    battle: &mut Battle,
    ability_id: &str,
    boost: &crate::dex_data::BoostsTable,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    match ability_id {
        "competitive" => competitive::on_after_each_boost(battle, boost, target_pos, source_pos, effect_id),
        "defiant" => defiant::on_after_each_boost(battle, boost, target_pos, source_pos, effect_id),
        _ => EventResult::Continue,
    }
}

/// Dispatch onAfterMoveSecondary callbacks
pub fn dispatch_on_after_move_secondary(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str,
) -> EventResult {
    match ability_id {
        "angershell" => angershell::on_after_move_secondary(battle, target_pos, source_pos, move_id),
        "berserk" => berserk::on_after_move_secondary(battle, target_pos, source_pos, move_id),
        "colorchange" => colorchange::on_after_move_secondary(battle, target_pos, source_pos, move_id),
        "pickpocket" => pickpocket::on_after_move_secondary(battle, target_pos, source_pos, move_id),
        _ => EventResult::Continue,
    }
}

/// Dispatch onAfterMoveSecondarySelf callbacks
pub fn dispatch_on_after_move_secondary_self(
    battle: &mut Battle,
    ability_id: &str,
    source_pos: (usize, usize), target_pos: (usize, usize), move_id: &str,
) -> EventResult {
    match ability_id {
        "magician" => magician::on_after_move_secondary_self(battle, source_pos, target_pos, move_id),
        _ => EventResult::Continue,
    }
}

/// Dispatch onAfterSetStatus callbacks
pub fn dispatch_on_after_set_status(
    battle: &mut Battle,
    ability_id: &str,
    status: Option<&str>, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    match ability_id {
        "synchronize" => synchronize::on_after_set_status(battle, status, target_pos, source_pos, effect_id),
        _ => EventResult::Continue,
    }
}

/// Dispatch onAfterTerastallization callbacks
pub fn dispatch_on_after_terastallization(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "teraformzero" => teraformzero::on_after_terastallization(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onAfterUseItem callbacks
pub fn dispatch_on_after_use_item(
    battle: &mut Battle,
    ability_id: &str,
    item_id: Option<&str>, pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "unburden" => unburden::on_after_use_item(battle, item_id, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onAllyAfterUseItem callbacks
pub fn dispatch_on_ally_after_use_item(
    battle: &mut Battle,
    ability_id: &str,
    item_id: Option<&str>, pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "symbiosis" => symbiosis::on_ally_after_use_item(battle, item_id, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onAllyBasePower callbacks
pub fn dispatch_on_ally_base_power(
    battle: &mut Battle,
    ability_id: &str,
    base_power: i32, attacker_pos: Option<(usize, usize)>, defender_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    match ability_id {
        "battery" => battery::on_ally_base_power(battle, base_power, attacker_pos, defender_pos, move_id),
        "powerspot" => powerspot::on_ally_base_power(battle, base_power, attacker_pos, defender_pos, move_id),
        "steelyspirit" => steelyspirit::on_ally_base_power(battle, base_power, attacker_pos, defender_pos, move_id),
        _ => EventResult::Continue,
    }
}

/// Dispatch onAllyFaint callbacks
pub fn dispatch_on_ally_faint(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    match ability_id {
        "powerofalchemy" => powerofalchemy::on_ally_faint(battle, target_pos),
        "receiver" => receiver::on_ally_faint(battle, target_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onAllyModifyAtk callbacks
pub fn dispatch_on_ally_modify_atk(
    battle: &mut Battle,
    ability_id: &str,
    atk: i32, pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "flowergift" => flowergift::on_ally_modify_atk(battle, atk, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onAllyModifySpD callbacks
pub fn dispatch_on_ally_modify_sp_d(
    battle: &mut Battle,
    ability_id: &str,
    spd: i32, pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "flowergift" => flowergift::on_ally_modify_sp_d(battle, spd, pokemon_pos),
        _ => EventResult::Continue,
    }
}
/// Dispatch onAllySetStatus callbacks
pub fn dispatch_on_ally_set_status(
    battle: &mut Battle,
    ability_id: &str,
    status_id: &str, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    match ability_id {
        "flowerveil" => flowerveil::on_ally_set_status(battle, status_id, target_pos, source_pos, effect_id),
        "pastelveil" => pastelveil::on_ally_set_status(battle, status_id, target_pos, source_pos, effect_id),
        "sweetveil" => sweetveil::on_ally_set_status(battle, status_id, target_pos, source_pos, effect_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onAllyTryAddVolatile callbacks
pub fn dispatch_on_ally_try_add_volatile(
    battle: &mut Battle,
    ability_id: &str,
    status: Option<&str>, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    match ability_id {
        "aromaveil" => aromaveil::on_ally_try_add_volatile(battle, status, target_pos, source_pos, effect_id),
        "flowerveil" => flowerveil::on_ally_try_add_volatile(battle, status, target_pos, source_pos, effect_id),
        "sweetveil" => sweetveil::on_ally_try_add_volatile(battle, status, target_pos, source_pos, effect_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onAllyTryBoost callbacks
/// Note: JavaScript signature is onAllyTryBoost(boost, target, source, effect)
/// In Rust, the boost parameter is accessed through battle.current_event.relay_var
/// due to borrow checker limitations (callback needs &mut Battle)
pub fn dispatch_on_ally_try_boost(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    match ability_id {
        "flowerveil" => flowerveil::on_ally_try_boost(battle, target_pos, source_pos, effect_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onAllyTryHitSide callbacks
pub fn dispatch_on_ally_try_hit_side(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    match ability_id {
        "magicbounce" => magicbounce::on_ally_try_hit_side(battle, target_pos, source_pos, move_id),
        "rebound" => rebound::on_ally_try_hit_side(battle, target_pos, source_pos, move_id),
        "sapsipper" => sapsipper::on_ally_try_hit_side(battle, target_pos, source_pos, move_id),
        "soundproof" => soundproof::on_ally_try_hit_side(battle, target_pos, source_pos, move_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onAnyAccuracy callbacks
pub fn dispatch_on_any_accuracy(
    battle: &mut Battle,
    ability_id: &str,
    accuracy: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    match ability_id {
        "noguard" => noguard::on_any_accuracy(battle, accuracy, target_pos, source_pos, move_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onAnyAfterMega callbacks
pub fn dispatch_on_any_after_mega(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    match ability_id {
        "opportunist" => opportunist::on_any_after_mega(battle),
        _ => EventResult::Continue,
    }
}
/// Dispatch onAnyAfterMove callbacks
pub fn dispatch_on_any_after_move(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    match ability_id {
        "opportunist" => opportunist::on_any_after_move(battle),
        "terashell" => terashell::on_any_after_move(battle),
        _ => EventResult::Continue,
    }
}
/// Dispatch onAnyAfterSetStatus callbacks
pub fn dispatch_on_any_after_set_status(
    battle: &mut Battle,
    ability_id: &str,
    status: Option<&str>, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    match ability_id {
        "poisonpuppeteer" => poisonpuppeteer::on_any_after_set_status(battle, status, target_pos, source_pos, effect_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onAnyAfterTerastallization callbacks
pub fn dispatch_on_any_after_terastallization(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    match ability_id {
        "opportunist" => opportunist::on_any_after_terastallization(battle),
        _ => EventResult::Continue,
    }
}
/// Dispatch onAnyBasePower callbacks
pub fn dispatch_on_any_base_power(
    battle: &mut Battle,
    ability_id: &str,
    base_power: i32, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    match ability_id {
        "darkaura" => darkaura::on_any_base_power(battle, base_power, source_pos, target_pos, move_id),
        "fairyaura" => fairyaura::on_any_base_power(battle, base_power, source_pos, target_pos, move_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onAnyBeforeMove callbacks
pub fn dispatch_on_any_before_move(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    match ability_id {
        "terashell" => terashell::on_any_before_move(battle),
        _ => EventResult::Continue,
    }
}
/// Dispatch onAnyDamage callbacks
pub fn dispatch_on_any_damage(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    match ability_id {
        "damp" => damp::on_any_damage(battle, damage, target_pos, source_pos, effect_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onAnyFaint callbacks
pub fn dispatch_on_any_faint(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    match ability_id {
        "soulheart" => soulheart::on_any_faint(battle),
        _ => EventResult::Continue,
    }
}
/// Dispatch onAnyInvulnerability callbacks
pub fn dispatch_on_any_invulnerability(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    match ability_id {
        "noguard" => noguard::on_any_invulnerability(battle, target_pos, source_pos, move_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onAnyModifyAccuracy callbacks
pub fn dispatch_on_any_modify_accuracy(
    battle: &mut Battle,
    ability_id: &str,
    accuracy: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>,
) -> EventResult {
    match ability_id {
        "victorystar" => victorystar::on_any_modify_accuracy(battle, accuracy, target_pos, source_pos),
        _ => EventResult::Continue,
    }
}
/// Dispatch onAnyModifyAtk callbacks
pub fn dispatch_on_any_modify_atk(
    battle: &mut Battle,
    ability_id: &str,
    atk: i32, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    match ability_id {
        "tabletsofruin" => tabletsofruin::on_any_modify_atk(battle, atk, source_pos, target_pos, move_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onAnyModifyBoost callbacks
/// Note: JavaScript signature is onAnyModifyBoost(boosts, pokemon)
/// In Rust, boosts are accessed through battle.current_event.relay_var
pub fn dispatch_on_any_modify_boost(
    battle: &mut Battle,
    ability_id: &str,
    boosts: &str, pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "unaware" => unaware::on_any_modify_boost(battle, boosts, pokemon_pos),
        _ => EventResult::Continue,
    }
}
/// Dispatch onAnyModifyDamage callbacks
pub fn dispatch_on_any_modify_damage(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    match ability_id {
        "friendguard" => friendguard::on_any_modify_damage(battle, damage, source_pos, target_pos, move_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onAnyModifyDef callbacks
pub fn dispatch_on_any_modify_def(
    battle: &mut Battle,
    ability_id: &str,
    def: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    match ability_id {
        "swordofruin" => swordofruin::on_any_modify_def(battle, def, target_pos, source_pos, move_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onAnyModifySpA callbacks
pub fn dispatch_on_any_modify_sp_a(
    battle: &mut Battle,
    ability_id: &str,
    spa: i32, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    match ability_id {
        "vesselofruin" => vesselofruin::on_any_modify_sp_a(battle, spa, source_pos, target_pos, move_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onAnyModifySpD callbacks
pub fn dispatch_on_any_modify_sp_d(
    battle: &mut Battle,
    ability_id: &str,
    spd: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    match ability_id {
        "beadsofruin" => beadsofruin::on_any_modify_sp_d(battle, spd, target_pos, source_pos, move_id),
        _ => EventResult::Continue,
    }
}

/// Dispatch onAnyRedirectTarget callbacks
pub fn dispatch_on_any_redirect_target(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, source2_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    match ability_id {
        "lightningrod" => lightningrod::on_any_redirect_target(battle, target_pos, source_pos, source2_pos, move_id),
        "stormdrain" => stormdrain::on_any_redirect_target(battle, target_pos, source_pos, source2_pos, move_id),
        _ => EventResult::Continue,
    }
}

/// Dispatch onAnySetWeather callbacks
pub fn dispatch_on_any_set_weather(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, weather_id: &str,
) -> EventResult {
    match ability_id {
        "deltastream" => deltastream::on_any_set_weather(battle, target_pos, source_pos, weather_id),
        "desolateland" => desolateland::on_any_set_weather(battle, target_pos, source_pos, weather_id),
        "primordialsea" => primordialsea::on_any_set_weather(battle, target_pos, source_pos, weather_id),
        _ => EventResult::Continue,
    }
}

/// Dispatch onAnySwitchIn callbacks
pub fn dispatch_on_any_switch_in(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    match ability_id {
        "commander" => commander::on_any_switch_in(battle),
        "opportunist" => opportunist::on_any_switch_in(battle),
        "pastelveil" => pastelveil::on_any_switch_in(battle),
        _ => EventResult::Continue,
    }
}

/// Dispatch onAnyTryMove callbacks
pub fn dispatch_on_any_try_move(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    match ability_id {
        "damp" => damp::on_any_try_move(battle, target_pos, source_pos, effect_id),
        _ => EventResult::Continue,
    }
}

/// Dispatch onAnyTryPrimaryHit callbacks
pub fn dispatch_on_any_try_primary_hit(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    match ability_id {
        "aurabreak" => aurabreak::on_any_try_primary_hit(battle, target_pos, source_pos, move_id),
        _ => EventResult::Continue,
    }
}

/// Dispatch onBasePower callbacks
pub fn dispatch_on_base_power(
    battle: &mut Battle,
    ability_id: &str,
    base_power: i32, attacker_pos: (usize, usize), defender_pos: (usize, usize), move_id: &str,
) -> EventResult {
    match ability_id {
        "aerilate" => aerilate::on_base_power(battle, base_power, attacker_pos, defender_pos, move_id),
        "analytic" => analytic::on_base_power(battle, base_power, attacker_pos, defender_pos, move_id),
        "flareboost" => flareboost::on_base_power(battle, base_power, attacker_pos, defender_pos, move_id),
        "galvanize" => galvanize::on_base_power(battle, base_power, attacker_pos, defender_pos, move_id),
        "ironfist" => ironfist::on_base_power(battle, base_power, attacker_pos, defender_pos, move_id),
        "megalauncher" => megalauncher::on_base_power(battle, base_power, attacker_pos, defender_pos, move_id),
        "normalize" => normalize::on_base_power(battle, base_power, attacker_pos, defender_pos, move_id),
        "pixilate" => pixilate::on_base_power(battle, base_power, attacker_pos, defender_pos, move_id),
        "punkrock" => punkrock::on_base_power(battle, base_power, attacker_pos, defender_pos, move_id),
        "reckless" => reckless::on_base_power(battle, base_power, attacker_pos, defender_pos, move_id),
        "refrigerate" => refrigerate::on_base_power(battle, base_power, attacker_pos, defender_pos, move_id),
        "rivalry" => rivalry::on_base_power(battle, base_power, attacker_pos, defender_pos, move_id),
        "sandforce" => sandforce::on_base_power(battle, base_power, attacker_pos, defender_pos, move_id),
        "sharpness" => sharpness::on_base_power(battle, base_power, attacker_pos, defender_pos, move_id),
        "sheerforce" => sheerforce::on_base_power(battle, base_power, attacker_pos, defender_pos, move_id),
        "strongjaw" => strongjaw::on_base_power(battle, base_power, attacker_pos, defender_pos, move_id),
        "supremeoverlord" => supremeoverlord::on_base_power(battle, base_power, attacker_pos, defender_pos, move_id),
        "technician" => technician::on_base_power(battle, base_power, attacker_pos, defender_pos, move_id),
        "toughclaws" => toughclaws::on_base_power(battle, base_power, attacker_pos, defender_pos, move_id),
        "toxicboost" => toxicboost::on_base_power(battle, base_power, attacker_pos, defender_pos, move_id),
        _ => EventResult::Continue,
    }
}

/// Dispatch onBeforeMove callbacks
pub fn dispatch_on_before_move(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    match ability_id {
        "gorillatactics" => gorillatactics::on_before_move(battle, pokemon_pos, target_pos, move_id),
        "truant" => truant::on_before_move(battle, pokemon_pos, target_pos, move_id),
        _ => EventResult::Continue,
    }
}

/// Dispatch onBeforeSwitchIn callbacks
pub fn dispatch_on_before_switch_in(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "illusion" => illusion::on_before_switch_in(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onChangeBoost callbacks
/// Note: boost parameter accessed via battle.current_event.relay_var (not passed as parameter)
pub fn dispatch_on_change_boost(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    match ability_id {
        "contrary" => contrary::on_change_boost(battle, target_pos, source_pos, effect_id),
        "ripen" => ripen::on_change_boost(battle, target_pos, source_pos, effect_id),
        "simple" => simple::on_change_boost(battle, target_pos, source_pos, effect_id),
        _ => EventResult::Continue,
    }
}

/// Dispatch onCheckShow callbacks
pub fn dispatch_on_check_show(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "naturalcure" => naturalcure::on_check_show(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onCriticalHit callbacks
pub fn dispatch_on_critical_hit(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    match ability_id {
        "disguise" => disguise::on_critical_hit(battle, target_pos, source_pos, move_id),
        "iceface" => iceface::on_critical_hit(battle, target_pos, source_pos, move_id),
        _ => EventResult::Continue,
    }
}

/// Dispatch onDamage callbacks
pub fn dispatch_on_damage(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    match ability_id {
        "angershell" => angershell::on_damage(battle, damage, target_pos, source_pos, effect_id),
        "berserk" => berserk::on_damage(battle, damage, target_pos, source_pos, effect_id),
        "disguise" => disguise::on_damage(battle, damage, target_pos, source_pos, effect_id),
        "gluttony" => gluttony::on_damage(battle, damage, target_pos, source_pos, effect_id),
        "heatproof" => heatproof::on_damage(battle, damage, target_pos, source_pos, effect_id),
        "iceface" => iceface::on_damage(battle, damage, target_pos, source_pos, effect_id),
        "magicguard" => magicguard::on_damage(battle, damage, target_pos, source_pos, effect_id),
        "mountaineer" => mountaineer::on_damage(battle, damage, target_pos, source_pos, effect_id),
        "poisonheal" => poisonheal::on_damage(battle, damage, target_pos, source_pos, effect_id),
        "rockhead" => rockhead::on_damage(battle, damage, target_pos, source_pos, effect_id),
        "sturdy" => sturdy::on_damage(battle, damage, target_pos, source_pos, effect_id),
        _ => EventResult::Continue,
    }
}

/// Dispatch onDamagingHit callbacks
pub fn dispatch_on_damaging_hit(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    match ability_id {
        "aftermath" => aftermath::on_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        "cottondown" => cottondown::on_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        "cursedbody" => cursedbody::on_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        "cutecharm" => cutecharm::on_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        "effectspore" => effectspore::on_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        "electromorphosis" => electromorphosis::on_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        "flamebody" => flamebody::on_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        "gooey" => gooey::on_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        "gulpmissile" => gulpmissile::on_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        "illusion" => illusion::on_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        "innardsout" => innardsout::on_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        "ironbarbs" => ironbarbs::on_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        "justified" => justified::on_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        "lingeringaroma" => lingeringaroma::on_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        "mummy" => mummy::on_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        "perishbody" => perishbody::on_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        "poisonpoint" => poisonpoint::on_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        "rattled" => rattled::on_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        "roughskin" => roughskin::on_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        "sandspit" => sandspit::on_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        "seedsower" => seedsower::on_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        "stamina" => stamina::on_damaging_hit(battle, damage, target_pos, source_pos, Some(move_id)),
        "static" => r#static::on_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        "steamengine" => steamengine::on_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        "tanglinghair" => tanglinghair::on_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        "thermalexchange" => thermalexchange::on_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        "toxicdebris" => toxicdebris::on_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        "wanderingspirit" => wanderingspirit::on_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        "watercompaction" => watercompaction::on_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        "weakarmor" => weakarmor::on_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        "windpower" => windpower::on_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        _ => EventResult::Continue,
    }
}

/// Dispatch onDeductPP callbacks
pub fn dispatch_on_deduct_p_p(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>,
) -> EventResult {
    match ability_id {
        "pressure" => pressure::on_deduct_p_p(battle, target_pos, source_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onDisableMove callbacks
pub fn dispatch_on_disable_move(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "gorillatactics" => gorillatactics::on_disable_move(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onDragOut callbacks
pub fn dispatch_on_drag_out(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    match ability_id {
        "guarddog" => guarddog::on_drag_out(battle, pokemon_pos, source_pos, move_id),
        "suctioncups" => suctioncups::on_drag_out(battle, pokemon_pos, source_pos, move_id),
        _ => EventResult::Continue,
    }
}

/// Dispatch onEatItem callbacks
pub fn dispatch_on_eat_item(
    battle: &mut Battle,
    ability_id: &str,
    item_id: Option<&str>, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    match ability_id {
        "cheekpouch" => cheekpouch::on_eat_item(battle, item_id, pokemon_pos, source_pos, effect_id),
        "cudchew" => cudchew::on_eat_item(battle, item_id, pokemon_pos, source_pos, effect_id),
        "ripen" => ripen::on_eat_item(battle, item_id, pokemon_pos, source_pos, effect_id),
        _ => EventResult::Continue,
    }
}

/// Dispatch onEffectiveness callbacks
pub fn dispatch_on_effectiveness(
    battle: &mut Battle,
    ability_id: &str,
    type_mod: i32, target_pos: (usize, usize), type_str: &str, move_id: &str,
) -> EventResult {
    match ability_id {
        "disguise" => disguise::on_effectiveness(battle, type_mod, target_pos, type_str, move_id),
        "iceface" => iceface::on_effectiveness(battle, type_mod, target_pos, type_str, move_id),
        _ => EventResult::Continue,
    }
}

/// Dispatch onEmergencyExit callbacks
pub fn dispatch_on_emergency_exit(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    match ability_id {
        "emergencyexit" => emergencyexit::on_emergency_exit(battle, target_pos),
        "wimpout" => wimpout::on_emergency_exit(battle, target_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onEnd callbacks
pub fn dispatch_on_end(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "airlock" => airlock::on_end(battle, pokemon_pos),
        "asoneglastrier" => asoneglastrier::on_end(battle, pokemon_pos),
        "asonespectrier" => asonespectrier::on_end(battle, pokemon_pos),
        "cloudnine" => cloudnine::on_end(battle, pokemon_pos),
        "deltastream" => deltastream::on_end(battle, pokemon_pos),
        "desolateland" => desolateland::on_end(battle, pokemon_pos),
        "flashfire" => flashfire::on_end(battle, pokemon_pos),
        "gorillatactics" => gorillatactics::on_end(battle, pokemon_pos),
        "illusion" => illusion::on_end(battle, pokemon_pos),
        "neutralizinggas" => neutralizinggas::on_end(battle, pokemon_pos),
        "opportunist" => opportunist::on_end(battle, pokemon_pos),
        "primordialsea" => primordialsea::on_end(battle, pokemon_pos),
        "protosynthesis" => protosynthesis::on_end(battle, pokemon_pos),
        "quarkdrive" => quarkdrive::on_end(battle, pokemon_pos),
        "supremeoverlord" => supremeoverlord::on_end(battle, pokemon_pos),
        "unburden" => unburden::on_end(battle, pokemon_pos),
        "unnerve" => unnerve::on_end(battle, pokemon_pos),
        "zenmode" => zenmode::on_end(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}
/// Dispatch onFaint callbacks
pub fn dispatch_on_faint(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
    effect_id: Option<&str>,
) -> EventResult {
    match ability_id {
        "illusion" => illusion::on_faint(battle, pokemon_pos, source_pos, effect_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onFlinch callbacks
pub fn dispatch_on_flinch(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "steadfast" => steadfast::on_flinch(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}
/// Dispatch onFoeAfterBoost callbacks
pub fn dispatch_on_foe_after_boost(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    match ability_id {
        "opportunist" => opportunist::on_foe_after_boost(battle, target_pos, source_pos, effect_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onFoeMaybeTrapPokemon callbacks
pub fn dispatch_on_foe_maybe_trap_pokemon(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>,
) -> EventResult {
    match ability_id {
        "arenatrap" => arenatrap::on_foe_maybe_trap_pokemon(battle, pokemon_pos, source_pos),
        "magnetpull" => magnetpull::on_foe_maybe_trap_pokemon(battle, pokemon_pos, source_pos),
        "shadowtag" => shadowtag::on_foe_maybe_trap_pokemon(battle, pokemon_pos, source_pos),
        _ => EventResult::Continue,
    }
}
/// Dispatch onFoeTrapPokemon callbacks
pub fn dispatch_on_foe_trap_pokemon(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "arenatrap" => arenatrap::on_foe_trap_pokemon(battle, pokemon_pos),
        "magnetpull" => magnetpull::on_foe_trap_pokemon(battle, pokemon_pos),
        "shadowtag" => shadowtag::on_foe_trap_pokemon(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}
/// Dispatch onFoeTryEatItem callbacks
pub fn dispatch_on_foe_try_eat_item(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    match ability_id {
        "asoneglastrier" => asoneglastrier::on_foe_try_eat_item(battle),
        "asonespectrier" => asonespectrier::on_foe_try_eat_item(battle),
        "unnerve" => unnerve::on_foe_try_eat_item(battle),
        _ => EventResult::Continue,
    }
}
/// Dispatch onFoeTryMove callbacks
pub fn dispatch_on_foe_try_move(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    match ability_id {
        "armortail" => armortail::on_foe_try_move(battle, target_pos, source_pos, move_id),
        "dazzling" => dazzling::on_foe_try_move(battle, target_pos, source_pos, move_id),
        "queenlymajesty" => queenlymajesty::on_foe_try_move(battle, target_pos, source_pos, move_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onFractionalPriority callbacks
pub fn dispatch_on_fractional_priority(
    battle: &mut Battle,
    ability_id: &str,
    priority: i32,
    pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    match ability_id {
        "myceliummight" => myceliummight::on_fractional_priority(battle, priority, pokemon_pos, target_pos, move_id),
        "quickdraw" => quickdraw::on_fractional_priority(battle, priority, pokemon_pos, target_pos, move_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onHit callbacks
pub fn dispatch_on_hit(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize), source_pos: (usize, usize), move_id: &str,
) -> EventResult {
    match ability_id {
        "angerpoint" => angerpoint::on_hit(battle, pokemon_pos, source_pos, move_id),
        "owntempo" => owntempo::on_hit(battle, pokemon_pos, source_pos, move_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onImmunity callbacks
pub fn dispatch_on_immunity(
    battle: &mut Battle,
    ability_id: &str,
    type_or_status: &str, pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "icebody" => icebody::on_immunity(battle, type_or_status, pokemon_pos),
        "magmaarmor" => magmaarmor::on_immunity(battle, type_or_status, pokemon_pos),
        "oblivious" => oblivious::on_immunity(battle, type_or_status, pokemon_pos),
        "overcoat" => overcoat::on_immunity(battle, type_or_status, pokemon_pos),
        "sandforce" => sandforce::on_immunity(battle, type_or_status, pokemon_pos),
        "sandrush" => sandrush::on_immunity(battle, type_or_status, pokemon_pos),
        "sandveil" => sandveil::on_immunity(battle, type_or_status, pokemon_pos),
        "snowcloak" => snowcloak::on_immunity(battle, type_or_status, pokemon_pos),
        _ => EventResult::Continue,
    }
}
/// Dispatch onModifyAccuracy callbacks
pub fn dispatch_on_modify_accuracy(
    battle: &mut Battle,
    ability_id: &str,
    accuracy: i32, target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str,
) -> EventResult {
    match ability_id {
        "sandveil" => sandveil::on_modify_accuracy(battle, accuracy, target_pos, source_pos, move_id),
        "snowcloak" => snowcloak::on_modify_accuracy(battle, accuracy, target_pos, source_pos, move_id),
        "tangledfeet" => tangledfeet::on_modify_accuracy(battle, accuracy, target_pos, source_pos, move_id),
        "wonderskin" => wonderskin::on_modify_accuracy(battle, accuracy, target_pos, source_pos, move_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onModifyAtk callbacks
pub fn dispatch_on_modify_atk(
    battle: &mut Battle,
    ability_id: &str,
    atk: i32, attacker_pos: (usize, usize), defender_pos: (usize, usize), move_id: &str,
) -> EventResult {
    match ability_id {
        "blaze" => blaze::on_modify_atk(battle, atk, attacker_pos, defender_pos, move_id),
        "defeatist" => defeatist::on_modify_atk(battle, atk, attacker_pos, defender_pos, move_id),
        "dragonsmaw" => dragonsmaw::on_modify_atk(battle, atk, attacker_pos, defender_pos, move_id),
        "gorillatactics" => gorillatactics::on_modify_atk(battle, atk, attacker_pos, defender_pos, move_id),
        "guts" => guts::on_modify_atk(battle, atk, attacker_pos, defender_pos, move_id),
        "hugepower" => hugepower::on_modify_atk(battle, atk, attacker_pos, defender_pos, move_id),
        "hustle" => hustle::on_modify_atk(battle, atk, attacker_pos, defender_pos, move_id),
        "orichalcumpulse" => orichalcumpulse::on_modify_atk(battle, atk, attacker_pos, defender_pos, move_id),
        "overgrow" => overgrow::on_modify_atk(battle, atk, attacker_pos, defender_pos, move_id),
        "purepower" => purepower::on_modify_atk(battle, atk, attacker_pos, defender_pos, move_id),
        "rockypayload" => rockypayload::on_modify_atk(battle, atk, attacker_pos, defender_pos, move_id),
        "slowstart" => slowstart::on_modify_atk(battle, atk, attacker_pos, defender_pos, move_id),
        "stakeout" => stakeout::on_modify_atk(battle, atk, attacker_pos, defender_pos, move_id),
        "steelworker" => steelworker::on_modify_atk(battle, atk, attacker_pos, defender_pos, move_id),
        "swarm" => swarm::on_modify_atk(battle, atk, attacker_pos, defender_pos, move_id),
        "torrent" => torrent::on_modify_atk(battle, atk, attacker_pos, defender_pos, move_id),
        "transistor" => transistor::on_modify_atk(battle, atk, attacker_pos, defender_pos, move_id),
        "waterbubble" => waterbubble::on_modify_atk(battle, atk, attacker_pos, defender_pos, move_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onModifyCritRatio callbacks
pub fn dispatch_on_modify_crit_ratio(
    battle: &mut Battle,
    ability_id: &str,
    crit_ratio: i32, source_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    match ability_id {
        "merciless" => merciless::on_modify_crit_ratio(battle, crit_ratio, source_pos, target_pos, move_id),
        "superluck" => superluck::on_modify_crit_ratio(battle, crit_ratio, source_pos, target_pos, move_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onModifyDamage callbacks
pub fn dispatch_on_modify_damage(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, source_pos: (usize, usize), target_pos: (usize, usize), move_id: &str,
) -> EventResult {
    match ability_id {
        "neuroforce" => neuroforce::on_modify_damage(battle, damage, source_pos, target_pos, move_id),
        "sniper" => sniper::on_modify_damage(battle, damage, source_pos, target_pos, move_id),
        "tintedlens" => tintedlens::on_modify_damage(battle, damage, source_pos, target_pos, move_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onModifyDef callbacks
pub fn dispatch_on_modify_def(
    battle: &mut Battle,
    ability_id: &str,
    def: i32, defender_pos: (usize, usize), attacker_pos: (usize, usize), move_id: &str,
) -> EventResult {
    match ability_id {
        "furcoat" => furcoat::on_modify_def(battle, def, defender_pos, attacker_pos, move_id),
        "grasspelt" => grasspelt::on_modify_def(battle, def, defender_pos, attacker_pos, move_id),
        "marvelscale" => marvelscale::on_modify_def(battle, def, defender_pos, attacker_pos, move_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onModifyMove callbacks
pub fn dispatch_on_modify_move(
    battle: &mut Battle,
    ability_id: &str,
    move_id: &str,
    source_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    match ability_id {
        "battlebond" => battlebond::on_modify_move(battle, move_id, source_pos, target_pos),
        "gorillatactics" => gorillatactics::on_modify_move(battle, move_id, source_pos, target_pos),
        "illuminate" => illuminate::on_modify_move(battle, move_id, source_pos, target_pos),
        "infiltrator" => infiltrator::on_modify_move(battle, move_id, source_pos, target_pos),
        "keeneye" => keeneye::on_modify_move(battle, move_id, source_pos, target_pos),
        "longreach" => longreach::on_modify_move(battle, move_id, source_pos, target_pos),
        "mindseye" => mindseye::on_modify_move(battle, move_id, source_pos, target_pos),
        "moldbreaker" => moldbreaker::on_modify_move(battle, move_id, source_pos, target_pos),
        "myceliummight" => myceliummight::on_modify_move(battle, move_id, source_pos, target_pos),
        "propellertail" => propellertail::on_modify_move(battle, move_id, source_pos, target_pos),
        "scrappy" => scrappy::on_modify_move(battle, move_id, source_pos, target_pos),
        "serenegrace" => serenegrace::on_modify_move(battle, move_id, source_pos, target_pos),
        "sheerforce" => sheerforce::on_modify_move(battle, move_id, source_pos, target_pos),
        "skilllink" => skilllink::on_modify_move(battle, move_id, source_pos, target_pos),
        "stalwart" => stalwart::on_modify_move(battle, move_id, source_pos, target_pos),
        "stancechange" => stancechange::on_modify_move(battle, move_id, source_pos, target_pos),
        "stench" => stench::on_modify_move(battle, move_id, source_pos, target_pos),
        "teravolt" => teravolt::on_modify_move(battle, move_id, source_pos, target_pos),
        "turboblaze" => turboblaze::on_modify_move(battle, move_id, source_pos, target_pos),
        "unseenfist" => unseenfist::on_modify_move(battle, move_id, source_pos, target_pos),
        _ => EventResult::Continue,
    }
}
/// Dispatch onModifyPriority callbacks
pub fn dispatch_on_modify_priority(
    battle: &mut Battle,
    ability_id: &str,
    priority: i32, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    match ability_id {
        "galewings" => galewings::on_modify_priority(battle, priority, pokemon_pos, target_pos, move_id),
        "prankster" => prankster::on_modify_priority(battle, priority, pokemon_pos, target_pos, move_id),
        "triage" => triage::on_modify_priority(battle, priority, pokemon_pos, target_pos, move_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onModifySTAB callbacks
pub fn dispatch_on_modify_s_t_a_b(
    battle: &mut Battle,
    ability_id: &str,
    stab: f64,
    source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    match ability_id {
        "adaptability" => adaptability::on_modify_s_t_a_b(battle, stab, source_pos, target_pos, move_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onModifySecondaries callbacks
pub fn dispatch_on_modify_secondaries(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    match ability_id {
        "shielddust" => shielddust::on_modify_secondaries(battle),
        _ => EventResult::Continue,
    }
}
/// Dispatch onModifySpA callbacks
pub fn dispatch_on_modify_sp_a(
    battle: &mut Battle,
    ability_id: &str,
    spa: i32, attacker_pos: (usize, usize), defender_pos: (usize, usize), move_id: &str,
) -> EventResult {
    match ability_id {
        "blaze" => blaze::on_modify_sp_a(battle, spa, attacker_pos, defender_pos, move_id),
        "defeatist" => defeatist::on_modify_sp_a(battle, spa, attacker_pos, defender_pos, move_id),
        "dragonsmaw" => dragonsmaw::on_modify_sp_a(battle, spa, attacker_pos, defender_pos, move_id),
        "hadronengine" => hadronengine::on_modify_sp_a(battle, spa, attacker_pos, defender_pos, move_id),
        "minus" => minus::on_modify_sp_a(battle, spa, attacker_pos, defender_pos, move_id),
        "overgrow" => overgrow::on_modify_sp_a(battle, spa, attacker_pos, defender_pos, move_id),
        "plus" => plus::on_modify_sp_a(battle, spa, attacker_pos, defender_pos, move_id),
        "rockypayload" => rockypayload::on_modify_sp_a(battle, spa, attacker_pos, defender_pos, move_id),
        "solarpower" => solarpower::on_modify_sp_a(battle, spa, attacker_pos, defender_pos, move_id),
        "stakeout" => stakeout::on_modify_sp_a(battle, spa, attacker_pos, defender_pos, move_id),
        "steelworker" => steelworker::on_modify_sp_a(battle, spa, attacker_pos, defender_pos, move_id),
        "swarm" => swarm::on_modify_sp_a(battle, spa, attacker_pos, defender_pos, move_id),
        "torrent" => torrent::on_modify_sp_a(battle, spa, attacker_pos, defender_pos, move_id),
        "transistor" => transistor::on_modify_sp_a(battle, spa, attacker_pos, defender_pos, move_id),
        "waterbubble" => waterbubble::on_modify_sp_a(battle, spa, attacker_pos, defender_pos, move_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onModifySpe callbacks
pub fn dispatch_on_modify_spe(
    battle: &mut Battle,
    ability_id: &str,
    spe: i32, pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "chlorophyll" => chlorophyll::on_modify_spe(battle, spe, pokemon_pos),
        "quickfeet" => quickfeet::on_modify_spe(battle, spe, pokemon_pos),
        "sandrush" => sandrush::on_modify_spe(battle, spe, pokemon_pos),
        "slowstart" => slowstart::on_modify_spe(battle, spe, pokemon_pos),
        "slushrush" => slushrush::on_modify_spe(battle, spe, pokemon_pos),
        "surgesurfer" => surgesurfer::on_modify_spe(battle, spe, pokemon_pos),
        "swiftswim" => swiftswim::on_modify_spe(battle, spe, pokemon_pos),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures:
//   onModifyType(move)
//   onModifyType(move, pokemon)
//   onModifyType(move, pokemon, target)

/// Dispatch onModifyType callbacks
pub fn dispatch_on_modify_type(
    battle: &mut Battle,
    ability_id: &str,
    move_id: &str, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>,
) -> EventResult {
    match ability_id {
        "aerilate" => aerilate::on_modify_type(battle, move_id, pokemon_pos, target_pos),
        "galvanize" => galvanize::on_modify_type(battle, move_id, pokemon_pos, target_pos),
        "liquidvoice" => liquidvoice::on_modify_type(battle, move_id, pokemon_pos, target_pos),
        "normalize" => normalize::on_modify_type(battle, move_id, pokemon_pos, target_pos),
        "pixilate" => pixilate::on_modify_type(battle, move_id, pokemon_pos, target_pos),
        "refrigerate" => refrigerate::on_modify_type(battle, move_id, pokemon_pos, target_pos),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures:
//   onModifyWeight(weighthg)
//   onModifyWeight(weighthg, pokemon)

/// Dispatch onModifyWeight callbacks
pub fn dispatch_on_modify_weight(
    battle: &mut Battle,
    ability_id: &str,
    weight: i32, pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "heavymetal" => heavymetal::on_modify_weight(battle, weight, pokemon_pos),
        "lightmetal" => lightmetal::on_modify_weight(battle, weight, pokemon_pos),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures:
//   onPrepareHit()
//   onPrepareHit(pokemon)
//   onPrepareHit(pokemon, source)
//   onPrepareHit(source, target)
//   onPrepareHit(source, target, move)
//   onPrepareHit(t, s, m)
//   onPrepareHit(target, pokemon, move)
//   onPrepareHit(target, source)
//   onPrepareHit(target, source, move)

/// Dispatch onPrepareHit callbacks
pub fn dispatch_on_prepare_hit(
    battle: &mut Battle,
    ability_id: &str,
    source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    match ability_id {
        "libero" => libero::on_prepare_hit(battle, source_pos, target_pos, move_id),
        "parentalbond" => parentalbond::on_prepare_hit(battle, source_pos, target_pos, move_id),
        "protean" => protean::on_prepare_hit(battle, source_pos, target_pos, move_id),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures:
//   onResidual()
//   onResidual(pokemon)
//   onResidual(pokemon, s, effect)
//   onResidual(pokemon, source)
//   onResidual(pokemon, source, effect)
//   onResidual(source)
//   onResidual(source, target, effect)
//   onResidual(target)
//   onResidual(target, source, effect)

/// Dispatch onResidual callbacks
pub fn dispatch_on_residual(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "baddreams" => baddreams::on_residual(battle, pokemon_pos),
        "cudchew" => cudchew::on_residual(battle, pokemon_pos),
        "harvest" => harvest::on_residual(battle, pokemon_pos),
        "healer" => healer::on_residual(battle, pokemon_pos),
        "hungerswitch" => hungerswitch::on_residual(battle, pokemon_pos),
        "hydration" => hydration::on_residual(battle, pokemon_pos),
        "moody" => moody::on_residual(battle, pokemon_pos),
        "opportunist" => opportunist::on_residual(battle, pokemon_pos),
        "pickup" => pickup::on_residual(battle, pokemon_pos),
        "powerconstruct" => powerconstruct::on_residual(battle, pokemon_pos),
        "schooling" => schooling::on_residual(battle, pokemon_pos),
        "shedskin" => shedskin::on_residual(battle, pokemon_pos),
        "shieldsdown" => shieldsdown::on_residual(battle, pokemon_pos),
        "slowstart" => slowstart::on_residual(battle, pokemon_pos),
        "speedboost" => speedboost::on_residual(battle, pokemon_pos),
        "zenmode" => zenmode::on_residual(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures:
//   onSetStatus(status, target, source)
//   onSetStatus(status, target, source, effect)

/// Dispatch onSetStatus callbacks
pub fn dispatch_on_set_status(
    battle: &mut Battle,
    ability_id: &str,
    status_id: &str, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    match ability_id {
        "comatose" => comatose::on_set_status(battle, status_id, target_pos, source_pos, effect_id),
        "immunity" => immunity::on_set_status(battle, status_id, target_pos, source_pos, effect_id),
        "insomnia" => insomnia::on_set_status(battle, status_id, target_pos, source_pos, effect_id),
        "leafguard" => leafguard::on_set_status(battle, status_id, target_pos, source_pos, effect_id),
        "limber" => limber::on_set_status(battle, status_id, target_pos, source_pos, effect_id),
        "pastelveil" => pastelveil::on_set_status(battle, status_id, target_pos, source_pos, effect_id),
        "purifyingsalt" => purifyingsalt::on_set_status(battle, status_id, target_pos, source_pos, effect_id),
        "shieldsdown" => shieldsdown::on_set_status(battle, status_id, target_pos, source_pos, effect_id),
        "thermalexchange" => thermalexchange::on_set_status(battle, status_id, target_pos, source_pos, effect_id),
        "vitalspirit" => vitalspirit::on_set_status(battle, status_id, target_pos, source_pos, effect_id),
        "waterbubble" => waterbubble::on_set_status(battle, status_id, target_pos, source_pos, effect_id),
        "waterveil" => waterveil::on_set_status(battle, status_id, target_pos, source_pos, effect_id),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures:
//   onSideConditionStart(side, source, sideCondition)
//   onSideConditionStart(target, source, sideCondition)

/// Dispatch onSideConditionStart callbacks
/// First param: pokemon_pos = Pokemon with the ability (this.effectState.target in JS)
/// Second param: side_idx = Event target (the side parameter in JS)
pub fn dispatch_on_side_condition_start(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
    side_idx: usize,
    side_condition_id: &str,
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    match ability_id {
        "windpower" => windpower::on_side_condition_start(battle, pokemon_pos, side_idx, side_condition_id, source_pos),
        "windrider" => windrider::on_side_condition_start(battle, pokemon_pos, side_idx, side_condition_id, source_pos),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures:
//   onSourceAfterFaint()
//   onSourceAfterFaint(length, target, source, effect)

/// Dispatch onSourceAfterFaint callbacks
pub fn dispatch_on_source_after_faint(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    match ability_id {
        "asoneglastrier" => asoneglastrier::on_source_after_faint(battle, target_pos, source_pos, effect_id),
        "asonespectrier" => asonespectrier::on_source_after_faint(battle, target_pos, source_pos, effect_id),
        "battlebond" => battlebond::on_source_after_faint(battle, target_pos, source_pos, effect_id),
        "beastboost" => beastboost::on_source_after_faint(battle, target_pos, source_pos, effect_id),
        "chillingneigh" => chillingneigh::on_source_after_faint(battle, target_pos, source_pos, effect_id),
        "grimneigh" => grimneigh::on_source_after_faint(battle, target_pos, source_pos, effect_id),
        "moxie" => moxie::on_source_after_faint(battle, target_pos, source_pos, effect_id),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures:
//   onSourceBasePower(basePower, attacker, defender, move)
//   onSourceBasePower(basePower, target, source, move)

/// Dispatch onSourceBasePower callbacks
pub fn dispatch_on_source_base_power(
    battle: &mut Battle,
    ability_id: &str,
    base_power: i32, move_id: &str,
) -> EventResult {
    match ability_id {
        "dryskin" => dryskin::on_source_base_power(battle, base_power, move_id),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures:
//   onSourceDamagingHit(damage, target, source, move)

/// Dispatch onSourceDamagingHit callbacks
pub fn dispatch_on_source_damaging_hit(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    match ability_id {
        "poisontouch" => poisontouch::on_source_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        "toxicchain" => toxicchain::on_source_damaging_hit(battle, damage, target_pos, source_pos, move_id),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures:
//   onSourceModifyAccuracy(accuracy)
//   onSourceModifyAccuracy(accuracy, target)
//   onSourceModifyAccuracy(accuracy, target, source)
//   onSourceModifyAccuracy(accuracy, target, source, move)

/// Dispatch onSourceModifyAccuracy callbacks
pub fn dispatch_on_source_modify_accuracy(
    battle: &mut Battle,
    ability_id: &str,
    accuracy: i32, target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str,
) -> EventResult {
    match ability_id {
        "compoundeyes" => compoundeyes::on_source_modify_accuracy(battle, accuracy, target_pos, source_pos, move_id),
        "hustle" => hustle::on_source_modify_accuracy(battle, accuracy, target_pos, source_pos, move_id),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures:
//   onSourceModifyAtk()
//   onSourceModifyAtk(atk, attacker, defender, move)

/// Dispatch onSourceModifyAtk callbacks
pub fn dispatch_on_source_modify_atk(
    battle: &mut Battle,
    ability_id: &str,
    move_id: &str,
) -> EventResult {
    match ability_id {
        "heatproof" => heatproof::on_source_modify_atk(battle, move_id),
        "purifyingsalt" => purifyingsalt::on_source_modify_atk(battle, move_id),
        "thickfat" => thickfat::on_source_modify_atk(battle, move_id),
        "waterbubble" => waterbubble::on_source_modify_atk(battle, move_id),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures:
//   onSourceModifyDamage()
//   onSourceModifyDamage(damage, source, target, move)

/// Dispatch onSourceModifyDamage callbacks
pub fn dispatch_on_source_modify_damage(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, source_pos: (usize, usize), target_pos: (usize, usize), move_id: &str,
) -> EventResult {
    match ability_id {
        "filter" => filter::on_source_modify_damage(battle, damage, source_pos, target_pos, move_id),
        "fluffy" => fluffy::on_source_modify_damage(battle, damage, source_pos, target_pos, move_id),
        "icescales" => icescales::on_source_modify_damage(battle, damage, source_pos, target_pos, move_id),
        "multiscale" => multiscale::on_source_modify_damage(battle, damage, source_pos, target_pos, move_id),
        "prismarmor" => prismarmor::on_source_modify_damage(battle, damage, source_pos, target_pos, move_id),
        "punkrock" => punkrock::on_source_modify_damage(battle, damage, source_pos, target_pos, move_id),
        "ripen" => ripen::on_source_modify_damage(battle, damage, source_pos, target_pos, move_id),
        "shadowshield" => shadowshield::on_source_modify_damage(battle, damage, source_pos, target_pos, move_id),
        "solidrock" => solidrock::on_source_modify_damage(battle, damage, source_pos, target_pos, move_id),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures:
//   onSourceModifySecondaries(secondaries, target, source, move)

/// Dispatch onSourceModifySecondaries callbacks
pub fn dispatch_on_source_modify_secondaries(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    match ability_id {
        "parentalbond" => parentalbond::on_source_modify_secondaries(battle, target_pos, source_pos, move_id),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures:
//   onSourceModifySpA()
//   onSourceModifySpA(atk, attacker, defender, move)
//   onSourceModifySpA(spa, attacker, defender, move)

/// Dispatch onSourceModifySpA callbacks
pub fn dispatch_on_source_modify_sp_a(
    battle: &mut Battle,
    ability_id: &str,
    move_id: &str,
) -> EventResult {
    match ability_id {
        "heatproof" => heatproof::on_source_modify_sp_a(battle, move_id),
        "purifyingsalt" => purifyingsalt::on_source_modify_sp_a(battle, move_id),
        "thickfat" => thickfat::on_source_modify_sp_a(battle, move_id),
        "waterbubble" => waterbubble::on_source_modify_sp_a(battle, move_id),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures:
//   onSourceTryHeal(damage, target, source, effect)

/// Dispatch onSourceTryHeal callbacks
pub fn dispatch_on_source_try_heal(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    match ability_id {
        "liquidooze" => liquidooze::on_source_try_heal(battle, damage, target_pos, source_pos, effect_id),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures:
//   onSourceTryPrimaryHit(target, source, effect)
//   onSourceTryPrimaryHit(target, source, move)

/// Dispatch onSourceTryPrimaryHit callbacks
pub fn dispatch_on_source_try_primary_hit(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    match ability_id {
        "gulpmissile" => gulpmissile::on_source_try_primary_hit(battle, target_pos, source_pos, effect_id),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures:
//   onStart()
//   onStart(attacker, defender, effect)
//   onStart(pokemon)
//   onStart(pokemon, source)
//   onStart(pokemon, source, effect)
//   onStart(pokemon, source, sourceEffect)
//   onStart(side, source, sideCondition)
//   onStart(source)
//   onStart(target)
//   onStart(target, source)
//   onStart(target, source, effect)
//   onStart(target, source, move)
//   onStart(target, source, sideCondition)
//   onStart(target, source, sourceEffect)
//   onStart(this, pokemon)

/// Dispatch onStart callbacks
pub fn dispatch_on_start(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "airlock" => airlock::on_start(battle, pokemon_pos),
        "anticipation" => anticipation::on_start(battle, pokemon_pos),
        "asoneglastrier" => asoneglastrier::on_start(battle, pokemon_pos),
        "asonespectrier" => asonespectrier::on_start(battle, pokemon_pos),
        "aurabreak" => aurabreak::on_start(battle, pokemon_pos),
        "beadsofruin" => beadsofruin::on_start(battle, pokemon_pos),
        "cloudnine" => cloudnine::on_start(battle, pokemon_pos),
        "comatose" => comatose::on_start(battle, pokemon_pos),
        "commander" => commander::on_start(battle, pokemon_pos),
        "costar" => costar::on_start(battle, pokemon_pos),
        "curiousmedicine" => curiousmedicine::on_start(battle, pokemon_pos),
        "darkaura" => darkaura::on_start(battle, pokemon_pos),
        "dauntlessshield" => dauntlessshield::on_start(battle, pokemon_pos),
        "deltastream" => deltastream::on_start(battle, pokemon_pos),
        "desolateland" => desolateland::on_start(battle, pokemon_pos),
        "download" => download::on_start(battle, pokemon_pos),
        "drizzle" => drizzle::on_start(battle, pokemon_pos),
        "drought" => drought::on_start(battle, pokemon_pos),
        "electricsurge" => electricsurge::on_start(battle, pokemon_pos),
        "embodyaspectcornerstone" => embodyaspectcornerstone::on_start(battle, pokemon_pos),
        "embodyaspecthearthflame" => embodyaspecthearthflame::on_start(battle, pokemon_pos),
        "embodyaspectteal" => embodyaspectteal::on_start(battle, pokemon_pos),
        "embodyaspectwellspring" => embodyaspectwellspring::on_start(battle, pokemon_pos),
        "fairyaura" => fairyaura::on_start(battle, pokemon_pos),
        "flowergift" => flowergift::on_start(battle, pokemon_pos),
        "forecast" => forecast::on_start(battle, pokemon_pos),
        "forewarn" => forewarn::on_start(battle, pokemon_pos),
        "frisk" => frisk::on_start(battle, pokemon_pos),
        "gluttony" => gluttony::on_start(battle, pokemon_pos),
        "gorillatactics" => gorillatactics::on_start(battle, pokemon_pos),
        "grassysurge" => grassysurge::on_start(battle, pokemon_pos),
        "hadronengine" => hadronengine::on_start(battle, pokemon_pos),
        "hospitality" => hospitality::on_start(battle, pokemon_pos),
        "iceface" => iceface::on_start(battle, pokemon_pos),
        "intimidate" => intimidate::on_start(battle, pokemon_pos),
        "intrepidsword" => intrepidsword::on_start(battle, pokemon_pos),
        "klutz" => klutz::on_start(battle, pokemon_pos),
        "mimicry" => mimicry::on_start(battle, pokemon_pos),
        "mistysurge" => mistysurge::on_start(battle, pokemon_pos),
        "moldbreaker" => moldbreaker::on_start(battle, pokemon_pos),
        "orichalcumpulse" => orichalcumpulse::on_start(battle, pokemon_pos),
        "pastelveil" => pastelveil::on_start(battle, pokemon_pos),
        "pressure" => pressure::on_start(battle, pokemon_pos),
        "primordialsea" => primordialsea::on_start(battle, pokemon_pos),
        "protosynthesis" => protosynthesis::on_start(battle, pokemon_pos),
        "psychicsurge" => psychicsurge::on_start(battle, pokemon_pos),
        "quarkdrive" => quarkdrive::on_start(battle, pokemon_pos),
        "sandstream" => sandstream::on_start(battle, pokemon_pos),
        "schooling" => schooling::on_start(battle, pokemon_pos),
        "screencleaner" => screencleaner::on_start(battle, pokemon_pos),
        "shieldsdown" => shieldsdown::on_start(battle, pokemon_pos),
        "slowstart" => slowstart::on_start(battle, pokemon_pos),
        "snowwarning" => snowwarning::on_start(battle, pokemon_pos),
        "supersweetsyrup" => supersweetsyrup::on_start(battle, pokemon_pos),
        "supremeoverlord" => supremeoverlord::on_start(battle, pokemon_pos),
        "swordofruin" => swordofruin::on_start(battle, pokemon_pos),
        "tabletsofruin" => tabletsofruin::on_start(battle, pokemon_pos),
        "teravolt" => teravolt::on_start(battle, pokemon_pos),
        "trace" => trace::on_start(battle, pokemon_pos),
        "truant" => truant::on_start(battle, pokemon_pos),
        "turboblaze" => turboblaze::on_start(battle, pokemon_pos),
        "unnerve" => unnerve::on_start(battle, pokemon_pos),
        "vesselofruin" => vesselofruin::on_start(battle, pokemon_pos),
        "windrider" => windrider::on_start(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures:
//   onSwitchIn()
//   onSwitchIn(pokemon)
//   onSwitchIn(target)

/// Dispatch onSwitchIn callbacks
pub fn dispatch_on_switch_in(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "airlock" => airlock::on_switch_in(battle, pokemon_pos),
        "cloudnine" => cloudnine::on_switch_in(battle, pokemon_pos),
        "imposter" => imposter::on_switch_in(battle, pokemon_pos),
        "neutralizinggas" => neutralizinggas::on_switch_in(battle, pokemon_pos),
        "terashift" => terashift::on_switch_in(battle, pokemon_pos),
        "zerotohero" => zerotohero::on_switch_in(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures:
//   onSwitchOut()
//   onSwitchOut(pokemon)

/// Dispatch onSwitchOut callbacks
pub fn dispatch_on_switch_out(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "naturalcure" => naturalcure::on_switch_out(battle, pokemon_pos),
        "regenerator" => regenerator::on_switch_out(battle, pokemon_pos),
        "zerotohero" => zerotohero::on_switch_out(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures:
//   onTakeItem(item)
//   onTakeItem(item, pokemon)
//   onTakeItem(item, pokemon, source)
//   onTakeItem(item, source)

/// Dispatch onTakeItem callbacks
pub fn dispatch_on_take_item(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>,
) -> EventResult {
    match ability_id {
        "stickyhold" => stickyhold::on_take_item(battle, pokemon_pos, source_pos),
        "unburden" => unburden::on_take_item(battle, pokemon_pos, source_pos),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures:
//   onTerrainChange(pokemon)
//   onTerrainChange(target, source, sourceEffect)

/// Dispatch onTerrainChange callbacks
pub fn dispatch_on_terrain_change(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "mimicry" => mimicry::on_terrain_change(battle, pokemon_pos),
        "quarkdrive" => quarkdrive::on_terrain_change(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures:
//   onTryAddVolatile(status, pokemon)
//   onTryAddVolatile(status, target)
//   onTryAddVolatile(status, target, source, effect)

/// Dispatch onTryAddVolatile callbacks
pub fn dispatch_on_try_add_volatile(
    battle: &mut Battle,
    ability_id: &str,
    status_id: &str, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    match ability_id {
        "innerfocus" => innerfocus::on_try_add_volatile(battle, status_id, target_pos, source_pos, effect_id),
        "insomnia" => insomnia::on_try_add_volatile(battle, status_id, target_pos, source_pos, effect_id),
        "leafguard" => leafguard::on_try_add_volatile(battle, status_id, target_pos, source_pos, effect_id),
        "owntempo" => owntempo::on_try_add_volatile(battle, status_id, target_pos, source_pos, effect_id),
        "purifyingsalt" => purifyingsalt::on_try_add_volatile(battle, status_id, target_pos, source_pos, effect_id),
        "shieldsdown" => shieldsdown::on_try_add_volatile(battle, status_id, target_pos, source_pos, effect_id),
        "vitalspirit" => vitalspirit::on_try_add_volatile(battle, status_id, target_pos, source_pos, effect_id),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures:
//   onTryBoost()
//   onTryBoost(boost, target, source, effect)

/// Dispatch onTryBoost callbacks
pub fn dispatch_on_try_boost(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: (usize, usize),
    boost: Option<&mut crate::dex_data::BoostsTable>,
) -> EventResult {
    match ability_id {
        "bigpecks" => bigpecks::on_try_boost(battle, target_pos, boost),
        "clearbody" => clearbody::on_try_boost(battle, target_pos, boost),
        "fullmetalbody" => fullmetalbody::on_try_boost(battle, target_pos, boost),
        "guarddog" => guarddog::on_try_boost(battle, target_pos, boost),
        "hypercutter" => hypercutter::on_try_boost(battle, target_pos, boost),
        "illuminate" => illuminate::on_try_boost(battle, target_pos, boost),
        "innerfocus" => innerfocus::on_try_boost(battle, target_pos, boost),
        "keeneye" => keeneye::on_try_boost(battle, target_pos, boost),
        "mindseye" => mindseye::on_try_boost(battle, target_pos, boost),
        "mirrorarmor" => mirrorarmor::on_try_boost(battle, target_pos, boost),
        "oblivious" => oblivious::on_try_boost(battle, target_pos, boost),
        "owntempo" => owntempo::on_try_boost(battle, target_pos, boost),
        "scrappy" => scrappy::on_try_boost(battle, target_pos, boost),
        "whitesmoke" => whitesmoke::on_try_boost(battle, target_pos, boost),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures:
//   onTryEatItem(item)
//   onTryEatItem(item, pokemon)

/// Dispatch onTryEatItem callbacks
pub fn dispatch_on_try_eat_item(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    match ability_id {
        "angershell" => angershell::on_try_eat_item(battle),
        "berserk" => berserk::on_try_eat_item(battle),
        "ripen" => ripen::on_try_eat_item(battle, (0, 0)),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures:
//   onTryHeal(damage, pokemon, source, effect)
//   onTryHeal(damage, target, source, effect)

/// Dispatch onTryHeal callbacks
pub fn dispatch_on_try_heal(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    match ability_id {
        "ripen" => ripen::on_try_heal(battle, damage, target_pos, source_pos, effect_id),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures:
//   onTryHit()
//   onTryHit(pokemon)
//   onTryHit(pokemon, source, move)
//   onTryHit(pokemon, target, move)
//   onTryHit(source)
//   onTryHit(source, target)
//   onTryHit(source, target, move)
//   onTryHit(target)
//   onTryHit(target, pokemon)
//   onTryHit(target, pokemon, move)
//   onTryHit(target, source)
//   onTryHit(target, source, effect)
//   onTryHit(target, source, move)

/// Dispatch onTryHit callbacks
pub fn dispatch_on_try_hit(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str,
) -> EventResult {
    match ability_id {
        "bulletproof" => bulletproof::on_try_hit(battle, target_pos, source_pos, move_id),
        "dryskin" => dryskin::on_try_hit(battle, target_pos, source_pos, move_id),
        "eartheater" => eartheater::on_try_hit(battle, target_pos, source_pos, move_id),
        "flashfire" => flashfire::on_try_hit(battle, target_pos, source_pos, move_id),
        "goodasgold" => goodasgold::on_try_hit(battle, target_pos, source_pos, move_id),
        "lightningrod" => lightningrod::on_try_hit(battle, target_pos, source_pos, move_id),
        "magicbounce" => magicbounce::on_try_hit(battle, target_pos, source_pos, move_id),
        "motordrive" => motordrive::on_try_hit(battle, target_pos, source_pos, move_id),
        "mountaineer" => mountaineer::on_try_hit(battle, target_pos, source_pos, move_id),
        "oblivious" => oblivious::on_try_hit(battle, target_pos, source_pos, move_id),
        "overcoat" => overcoat::on_try_hit(battle, target_pos, source_pos, move_id),
        "rebound" => rebound::on_try_hit(battle, target_pos, source_pos, move_id),
        "sapsipper" => sapsipper::on_try_hit(battle, target_pos, source_pos, move_id),
        "soundproof" => soundproof::on_try_hit(battle, target_pos, source_pos, move_id),
        "stormdrain" => stormdrain::on_try_hit(battle, target_pos, source_pos, move_id),
        "sturdy" => sturdy::on_try_hit(battle, target_pos, source_pos, move_id),
        "telepathy" => telepathy::on_try_hit(battle, target_pos, source_pos, move_id),
        "voltabsorb" => voltabsorb::on_try_hit(battle, target_pos, source_pos, move_id),
        "waterabsorb" => waterabsorb::on_try_hit(battle, target_pos, source_pos, move_id),
        "wellbakedbody" => wellbakedbody::on_try_hit(battle, target_pos, source_pos, move_id),
        "windrider" => windrider::on_try_hit(battle, target_pos, source_pos, move_id),
        "wonderguard" => wonderguard::on_try_hit(battle, target_pos, source_pos, move_id),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures:
//   onUpdate()
//   onUpdate(pokemon)

/// Dispatch onUpdate callbacks
pub fn dispatch_on_update(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "commander" => commander::on_update(battle, pokemon_pos),
        "disguise" => disguise::on_update(battle, pokemon_pos),
        "iceface" => iceface::on_update(battle, pokemon_pos),
        "immunity" => immunity::on_update(battle, pokemon_pos),
        "insomnia" => insomnia::on_update(battle, pokemon_pos),
        "limber" => limber::on_update(battle, pokemon_pos),
        "magmaarmor" => magmaarmor::on_update(battle, pokemon_pos),
        "oblivious" => oblivious::on_update(battle, pokemon_pos),
        "owntempo" => owntempo::on_update(battle, pokemon_pos),
        "pastelveil" => pastelveil::on_update(battle, pokemon_pos),
        "thermalexchange" => thermalexchange::on_update(battle, pokemon_pos),
        "trace" => trace::on_update(battle, pokemon_pos),
        "vitalspirit" => vitalspirit::on_update(battle, pokemon_pos),
        "waterbubble" => waterbubble::on_update(battle, pokemon_pos),
        "waterveil" => waterveil::on_update(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures:
//   onWeather()
//   onWeather(target)
//   onWeather(target, source, effect)

/// Dispatch onWeather callbacks
pub fn dispatch_on_weather(
    battle: &mut Battle,
    ability_id: &str,
    weather_id: &str, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>,
) -> EventResult {
    match ability_id {
        "dryskin" => dryskin::on_weather(battle, weather_id, pokemon_pos, source_pos),
        "icebody" => icebody::on_weather(battle, weather_id, pokemon_pos, source_pos),
        "raindish" => raindish::on_weather(battle, weather_id, pokemon_pos, source_pos),
        "solarpower" => solarpower::on_weather(battle, weather_id, pokemon_pos, source_pos),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures:
//   onWeatherChange(pokemon)
//   onWeatherChange(pokemon, source, sourceEffect)
//   onWeatherChange(target, source, sourceEffect)

/// Dispatch onWeatherChange callbacks
pub fn dispatch_on_weather_change(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "flowergift" => flowergift::on_weather_change(battle, pokemon_pos),
        "forecast" => forecast::on_weather_change(battle, pokemon_pos),
        "iceface" => iceface::on_weather_change(battle, pokemon_pos, None),
        "protosynthesis" => protosynthesis::on_weather_change(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

// Priority/Order/SubOrder variant dispatchers (aliases)
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND
/// Dispatch onAfterBoostPriority callbacks (alias for onAfterBoost)
pub fn dispatch_on_after_boost_priority(
    battle: &mut Battle,
    ability_id: &str,
    boost: &crate::dex_data::BoostsTable,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_after_boost(battle, ability_id, boost, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAfterBoostOrder callbacks (alias for onAfterBoost)
pub fn dispatch_on_after_boost_order(
    battle: &mut Battle,
    ability_id: &str,
    boost: &crate::dex_data::BoostsTable,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_after_boost(battle, ability_id, boost, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAfterBoostSubOrder callbacks (alias for onAfterBoost)
pub fn dispatch_on_after_boost_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    boost: &crate::dex_data::BoostsTable,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_after_boost(battle, ability_id, boost, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAfterEachBoostPriority callbacks (alias for onAfterEachBoost)
pub fn dispatch_on_after_each_boost_priority(
    battle: &mut Battle,
    ability_id: &str,
    boost: &crate::dex_data::BoostsTable,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_after_each_boost(battle, ability_id, boost, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAfterEachBoostOrder callbacks (alias for onAfterEachBoost)
pub fn dispatch_on_after_each_boost_order(
    battle: &mut Battle,
    ability_id: &str,
    boost: &crate::dex_data::BoostsTable,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_after_each_boost(battle, ability_id, boost, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAfterEachBoostSubOrder callbacks (alias for onAfterEachBoost)
pub fn dispatch_on_after_each_boost_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    boost: &crate::dex_data::BoostsTable,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_after_each_boost(battle, ability_id, boost, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAfterMoveSecondaryPriority callbacks (alias for onAfterMoveSecondary)
pub fn dispatch_on_after_move_secondary_priority(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_after_move_secondary(battle, ability_id, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAfterMoveSecondaryOrder callbacks (alias for onAfterMoveSecondary)
pub fn dispatch_on_after_move_secondary_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_after_move_secondary(battle, ability_id, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAfterMoveSecondarySubOrder callbacks (alias for onAfterMoveSecondary)
pub fn dispatch_on_after_move_secondary_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_after_move_secondary(battle, ability_id, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAfterMoveSecondarySelfPriority callbacks (alias for onAfterMoveSecondarySelf)
pub fn dispatch_on_after_move_secondary_self_priority(
    battle: &mut Battle,
    ability_id: &str,
    source_pos: (usize, usize), target_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_after_move_secondary_self(battle, ability_id, source_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAfterMoveSecondarySelfOrder callbacks (alias for onAfterMoveSecondarySelf)
pub fn dispatch_on_after_move_secondary_self_order(
    battle: &mut Battle,
    ability_id: &str,
    source_pos: (usize, usize), target_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_after_move_secondary_self(battle, ability_id, source_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAfterMoveSecondarySelfSubOrder callbacks (alias for onAfterMoveSecondarySelf)
pub fn dispatch_on_after_move_secondary_self_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    source_pos: (usize, usize), target_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_after_move_secondary_self(battle, ability_id, source_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAfterSetStatusPriority callbacks (alias for onAfterSetStatus)
pub fn dispatch_on_after_set_status_priority(
    battle: &mut Battle,
    ability_id: &str,
    status: Option<&str>, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_after_set_status(battle, ability_id, status, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAfterSetStatusOrder callbacks (alias for onAfterSetStatus)
pub fn dispatch_on_after_set_status_order(
    battle: &mut Battle,
    ability_id: &str,
    status: Option<&str>, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_after_set_status(battle, ability_id, status, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAfterSetStatusSubOrder callbacks (alias for onAfterSetStatus)
pub fn dispatch_on_after_set_status_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    status: Option<&str>, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_after_set_status(battle, ability_id, status, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAfterTerastallizationPriority callbacks (alias for onAfterTerastallization)
pub fn dispatch_on_after_terastallization_priority(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_after_terastallization(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAfterTerastallizationOrder callbacks (alias for onAfterTerastallization)
pub fn dispatch_on_after_terastallization_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_after_terastallization(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAfterTerastallizationSubOrder callbacks (alias for onAfterTerastallization)
pub fn dispatch_on_after_terastallization_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_after_terastallization(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAfterUseItemPriority callbacks (alias for onAfterUseItem)
pub fn dispatch_on_after_use_item_priority(
    battle: &mut Battle,
    ability_id: &str,
    item_id: Option<&str>, pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_after_use_item(battle, ability_id, item_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAfterUseItemOrder callbacks (alias for onAfterUseItem)
pub fn dispatch_on_after_use_item_order(
    battle: &mut Battle,
    ability_id: &str,
    item_id: Option<&str>, pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_after_use_item(battle, ability_id, item_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAfterUseItemSubOrder callbacks (alias for onAfterUseItem)
pub fn dispatch_on_after_use_item_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    item_id: Option<&str>, pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_after_use_item(battle, ability_id, item_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAllyAfterUseItemPriority callbacks (alias for onAllyAfterUseItem)
pub fn dispatch_on_ally_after_use_item_priority(
    battle: &mut Battle,
    ability_id: &str,
    item_id: Option<&str>, pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_ally_after_use_item(battle, ability_id, item_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAllyAfterUseItemOrder callbacks (alias for onAllyAfterUseItem)
pub fn dispatch_on_ally_after_use_item_order(
    battle: &mut Battle,
    ability_id: &str,
    item_id: Option<&str>, pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_ally_after_use_item(battle, ability_id, item_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAllyAfterUseItemSubOrder callbacks (alias for onAllyAfterUseItem)
pub fn dispatch_on_ally_after_use_item_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    item_id: Option<&str>, pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_ally_after_use_item(battle, ability_id, item_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAllyBasePowerPriority callbacks (alias for onAllyBasePower)
pub fn dispatch_on_ally_base_power_priority(
    battle: &mut Battle,
    ability_id: &str,
    base_power: i32, attacker_pos: Option<(usize, usize)>, defender_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_ally_base_power(battle, ability_id, base_power, attacker_pos, defender_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAllyBasePowerOrder callbacks (alias for onAllyBasePower)
pub fn dispatch_on_ally_base_power_order(
    battle: &mut Battle,
    ability_id: &str,
    base_power: i32, attacker_pos: Option<(usize, usize)>, defender_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_ally_base_power(battle, ability_id, base_power, attacker_pos, defender_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAllyBasePowerSubOrder callbacks (alias for onAllyBasePower)
pub fn dispatch_on_ally_base_power_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    base_power: i32, attacker_pos: Option<(usize, usize)>, defender_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_ally_base_power(battle, ability_id, base_power, attacker_pos, defender_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAllyFaintPriority callbacks (alias for onAllyFaint)
pub fn dispatch_on_ally_faint_priority(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    dispatch_on_ally_faint(battle, ability_id, target_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAllyFaintOrder callbacks (alias for onAllyFaint)
pub fn dispatch_on_ally_faint_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    dispatch_on_ally_faint(battle, ability_id, target_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAllyFaintSubOrder callbacks (alias for onAllyFaint)
pub fn dispatch_on_ally_faint_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    dispatch_on_ally_faint(battle, ability_id, target_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAllyModifyAtkPriority callbacks (alias for onAllyModifyAtk)
pub fn dispatch_on_ally_modify_atk_priority(
    battle: &mut Battle,
    ability_id: &str,
    atk: i32, pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_ally_modify_atk(battle, ability_id, atk, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAllyModifyAtkOrder callbacks (alias for onAllyModifyAtk)
pub fn dispatch_on_ally_modify_atk_order(
    battle: &mut Battle,
    ability_id: &str,
    atk: i32, pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_ally_modify_atk(battle, ability_id, atk, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAllyModifyAtkSubOrder callbacks (alias for onAllyModifyAtk)
pub fn dispatch_on_ally_modify_atk_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    atk: i32, pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_ally_modify_atk(battle, ability_id, atk, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAllyModifySpDPriority callbacks (alias for onAllyModifySpD)
pub fn dispatch_on_ally_modify_sp_d_priority(
    battle: &mut Battle,
    ability_id: &str,
    spd: i32, pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_ally_modify_sp_d(battle, ability_id, spd, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAllyModifySpDOrder callbacks (alias for onAllyModifySpD)
pub fn dispatch_on_ally_modify_sp_d_order(
    battle: &mut Battle,
    ability_id: &str,
    spd: i32, pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_ally_modify_sp_d(battle, ability_id, spd, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAllyModifySpDSubOrder callbacks (alias for onAllyModifySpD)
pub fn dispatch_on_ally_modify_sp_d_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    spd: i32, pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_ally_modify_sp_d(battle, ability_id, spd, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAllySetStatusPriority callbacks (alias for onAllySetStatus)
pub fn dispatch_on_ally_set_status_priority(
    battle: &mut Battle,
    ability_id: &str,
    status_id: &str, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_ally_set_status(battle, ability_id, status_id, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAllySetStatusOrder callbacks (alias for onAllySetStatus)
pub fn dispatch_on_ally_set_status_order(
    battle: &mut Battle,
    ability_id: &str,
    status_id: &str, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_ally_set_status(battle, ability_id, status_id, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAllySetStatusSubOrder callbacks (alias for onAllySetStatus)
pub fn dispatch_on_ally_set_status_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    status_id: &str, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_ally_set_status(battle, ability_id, status_id, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAllyTryAddVolatilePriority callbacks (alias for onAllyTryAddVolatile)
pub fn dispatch_on_ally_try_add_volatile_priority(
    battle: &mut Battle,
    ability_id: &str,
    status: Option<&str>, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_ally_try_add_volatile(battle, ability_id, status, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAllyTryAddVolatileOrder callbacks (alias for onAllyTryAddVolatile)
pub fn dispatch_on_ally_try_add_volatile_order(
    battle: &mut Battle,
    ability_id: &str,
    status: Option<&str>, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_ally_try_add_volatile(battle, ability_id, status, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAllyTryAddVolatileSubOrder callbacks (alias for onAllyTryAddVolatile)
pub fn dispatch_on_ally_try_add_volatile_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    status: Option<&str>, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_ally_try_add_volatile(battle, ability_id, status, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAllyTryBoostPriority callbacks (alias for onAllyTryBoost)
pub fn dispatch_on_ally_try_boost_priority(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_ally_try_boost(battle, ability_id, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAllyTryBoostOrder callbacks (alias for onAllyTryBoost)
pub fn dispatch_on_ally_try_boost_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_ally_try_boost(battle, ability_id, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAllyTryBoostSubOrder callbacks (alias for onAllyTryBoost)
pub fn dispatch_on_ally_try_boost_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_ally_try_boost(battle, ability_id, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAllyTryHitSidePriority callbacks (alias for onAllyTryHitSide)
pub fn dispatch_on_ally_try_hit_side_priority(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_ally_try_hit_side(battle, ability_id, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAllyTryHitSideOrder callbacks (alias for onAllyTryHitSide)
pub fn dispatch_on_ally_try_hit_side_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_ally_try_hit_side(battle, ability_id, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAllyTryHitSideSubOrder callbacks (alias for onAllyTryHitSide)
pub fn dispatch_on_ally_try_hit_side_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_ally_try_hit_side(battle, ability_id, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyAccuracyPriority callbacks (alias for onAnyAccuracy)
pub fn dispatch_on_any_accuracy_priority(
    battle: &mut Battle,
    ability_id: &str,
    accuracy: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_any_accuracy(battle, ability_id, accuracy, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyAccuracyOrder callbacks (alias for onAnyAccuracy)
pub fn dispatch_on_any_accuracy_order(
    battle: &mut Battle,
    ability_id: &str,
    accuracy: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_any_accuracy(battle, ability_id, accuracy, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyAccuracySubOrder callbacks (alias for onAnyAccuracy)
pub fn dispatch_on_any_accuracy_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    accuracy: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_any_accuracy(battle, ability_id, accuracy, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyAfterMegaPriority callbacks (alias for onAnyAfterMega)
pub fn dispatch_on_any_after_mega_priority(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    dispatch_on_any_after_mega(battle, ability_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyAfterMegaOrder callbacks (alias for onAnyAfterMega)
pub fn dispatch_on_any_after_mega_order(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    dispatch_on_any_after_mega(battle, ability_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyAfterMegaSubOrder callbacks (alias for onAnyAfterMega)
pub fn dispatch_on_any_after_mega_sub_order(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    dispatch_on_any_after_mega(battle, ability_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyAfterMovePriority callbacks (alias for onAnyAfterMove)
pub fn dispatch_on_any_after_move_priority(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    dispatch_on_any_after_move(battle, ability_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyAfterMoveOrder callbacks (alias for onAnyAfterMove)
pub fn dispatch_on_any_after_move_order(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    dispatch_on_any_after_move(battle, ability_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyAfterMoveSubOrder callbacks (alias for onAnyAfterMove)
pub fn dispatch_on_any_after_move_sub_order(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    dispatch_on_any_after_move(battle, ability_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyAfterSetStatusPriority callbacks (alias for onAnyAfterSetStatus)
pub fn dispatch_on_any_after_set_status_priority(
    battle: &mut Battle,
    ability_id: &str,
    status: Option<&str>, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_any_after_set_status(battle, ability_id, status, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyAfterSetStatusOrder callbacks (alias for onAnyAfterSetStatus)
pub fn dispatch_on_any_after_set_status_order(
    battle: &mut Battle,
    ability_id: &str,
    status: Option<&str>, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_any_after_set_status(battle, ability_id, status, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyAfterSetStatusSubOrder callbacks (alias for onAnyAfterSetStatus)
pub fn dispatch_on_any_after_set_status_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    status: Option<&str>, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_any_after_set_status(battle, ability_id, status, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyAfterTerastallizationPriority callbacks (alias for onAnyAfterTerastallization)
pub fn dispatch_on_any_after_terastallization_priority(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    dispatch_on_any_after_terastallization(battle, ability_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyAfterTerastallizationOrder callbacks (alias for onAnyAfterTerastallization)
pub fn dispatch_on_any_after_terastallization_order(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    dispatch_on_any_after_terastallization(battle, ability_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyAfterTerastallizationSubOrder callbacks (alias for onAnyAfterTerastallization)
pub fn dispatch_on_any_after_terastallization_sub_order(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    dispatch_on_any_after_terastallization(battle, ability_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyBasePowerPriority callbacks (alias for onAnyBasePower)
pub fn dispatch_on_any_base_power_priority(
    battle: &mut Battle,
    ability_id: &str,
    base_power: i32, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_any_base_power(battle, ability_id, base_power, source_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyBasePowerOrder callbacks (alias for onAnyBasePower)
pub fn dispatch_on_any_base_power_order(
    battle: &mut Battle,
    ability_id: &str,
    base_power: i32, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_any_base_power(battle, ability_id, base_power, source_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyBasePowerSubOrder callbacks (alias for onAnyBasePower)
pub fn dispatch_on_any_base_power_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    base_power: i32, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_any_base_power(battle, ability_id, base_power, source_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyBeforeMovePriority callbacks (alias for onAnyBeforeMove)
pub fn dispatch_on_any_before_move_priority(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    dispatch_on_any_before_move(battle, ability_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyBeforeMoveOrder callbacks (alias for onAnyBeforeMove)
pub fn dispatch_on_any_before_move_order(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    dispatch_on_any_before_move(battle, ability_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyBeforeMoveSubOrder callbacks (alias for onAnyBeforeMove)
pub fn dispatch_on_any_before_move_sub_order(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    dispatch_on_any_before_move(battle, ability_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyDamagePriority callbacks (alias for onAnyDamage)
pub fn dispatch_on_any_damage_priority(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_any_damage(battle, ability_id, damage, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyDamageOrder callbacks (alias for onAnyDamage)
pub fn dispatch_on_any_damage_order(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_any_damage(battle, ability_id, damage, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyDamageSubOrder callbacks (alias for onAnyDamage)
pub fn dispatch_on_any_damage_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_any_damage(battle, ability_id, damage, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyFaintPriority callbacks (alias for onAnyFaint)
pub fn dispatch_on_any_faint_priority(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    dispatch_on_any_faint(battle, ability_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyFaintOrder callbacks (alias for onAnyFaint)
pub fn dispatch_on_any_faint_order(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    dispatch_on_any_faint(battle, ability_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyFaintSubOrder callbacks (alias for onAnyFaint)
pub fn dispatch_on_any_faint_sub_order(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    dispatch_on_any_faint(battle, ability_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyInvulnerabilityPriority callbacks (alias for onAnyInvulnerability)
pub fn dispatch_on_any_invulnerability_priority(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_any_invulnerability(battle, ability_id, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyInvulnerabilityOrder callbacks (alias for onAnyInvulnerability)
pub fn dispatch_on_any_invulnerability_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_any_invulnerability(battle, ability_id, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyInvulnerabilitySubOrder callbacks (alias for onAnyInvulnerability)
pub fn dispatch_on_any_invulnerability_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_any_invulnerability(battle, ability_id, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyModifyAccuracyPriority callbacks (alias for onAnyModifyAccuracy)
pub fn dispatch_on_any_modify_accuracy_priority(
    battle: &mut Battle,
    ability_id: &str,
    accuracy: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>,
) -> EventResult {
    dispatch_on_any_modify_accuracy(battle, ability_id, accuracy, target_pos, source_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyModifyAccuracyOrder callbacks (alias for onAnyModifyAccuracy)
pub fn dispatch_on_any_modify_accuracy_order(
    battle: &mut Battle,
    ability_id: &str,
    accuracy: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>,
) -> EventResult {
    dispatch_on_any_modify_accuracy(battle, ability_id, accuracy, target_pos, source_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyModifyAccuracySubOrder callbacks (alias for onAnyModifyAccuracy)
pub fn dispatch_on_any_modify_accuracy_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    accuracy: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>,
) -> EventResult {
    dispatch_on_any_modify_accuracy(battle, ability_id, accuracy, target_pos, source_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyModifyAtkPriority callbacks (alias for onAnyModifyAtk)
pub fn dispatch_on_any_modify_atk_priority(
    battle: &mut Battle,
    ability_id: &str,
    atk: i32, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_any_modify_atk(battle, ability_id, atk, source_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyModifyAtkOrder callbacks (alias for onAnyModifyAtk)
pub fn dispatch_on_any_modify_atk_order(
    battle: &mut Battle,
    ability_id: &str,
    atk: i32, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_any_modify_atk(battle, ability_id, atk, source_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyModifyAtkSubOrder callbacks (alias for onAnyModifyAtk)
pub fn dispatch_on_any_modify_atk_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    atk: i32, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_any_modify_atk(battle, ability_id, atk, source_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyModifyBoostPriority callbacks (alias for onAnyModifyBoost)
pub fn dispatch_on_any_modify_boost_priority(
    battle: &mut Battle,
    ability_id: &str,
    boosts: &str, pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_any_modify_boost(battle, ability_id, boosts, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyModifyBoostOrder callbacks (alias for onAnyModifyBoost)
pub fn dispatch_on_any_modify_boost_order(
    battle: &mut Battle,
    ability_id: &str,
    boosts: &str, pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_any_modify_boost(battle, ability_id, boosts, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyModifyBoostSubOrder callbacks (alias for onAnyModifyBoost)
pub fn dispatch_on_any_modify_boost_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    boosts: &str, pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_any_modify_boost(battle, ability_id, boosts, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyModifyDamagePriority callbacks (alias for onAnyModifyDamage)
pub fn dispatch_on_any_modify_damage_priority(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_any_modify_damage(battle, ability_id, damage, source_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyModifyDamageOrder callbacks (alias for onAnyModifyDamage)
pub fn dispatch_on_any_modify_damage_order(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_any_modify_damage(battle, ability_id, damage, source_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyModifyDamageSubOrder callbacks (alias for onAnyModifyDamage)
pub fn dispatch_on_any_modify_damage_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_any_modify_damage(battle, ability_id, damage, source_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyModifyDefPriority callbacks (alias for onAnyModifyDef)
pub fn dispatch_on_any_modify_def_priority(
    battle: &mut Battle,
    ability_id: &str,
    def: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_any_modify_def(battle, ability_id, def, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyModifyDefOrder callbacks (alias for onAnyModifyDef)
pub fn dispatch_on_any_modify_def_order(
    battle: &mut Battle,
    ability_id: &str,
    def: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_any_modify_def(battle, ability_id, def, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyModifyDefSubOrder callbacks (alias for onAnyModifyDef)
pub fn dispatch_on_any_modify_def_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    def: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_any_modify_def(battle, ability_id, def, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyModifySpAPriority callbacks (alias for onAnyModifySpA)
pub fn dispatch_on_any_modify_sp_a_priority(
    battle: &mut Battle,
    ability_id: &str,
    spa: i32, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_any_modify_sp_a(battle, ability_id, spa, source_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyModifySpAOrder callbacks (alias for onAnyModifySpA)
pub fn dispatch_on_any_modify_sp_a_order(
    battle: &mut Battle,
    ability_id: &str,
    spa: i32, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_any_modify_sp_a(battle, ability_id, spa, source_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyModifySpASubOrder callbacks (alias for onAnyModifySpA)
pub fn dispatch_on_any_modify_sp_a_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    spa: i32, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_any_modify_sp_a(battle, ability_id, spa, source_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyModifySpDPriority callbacks (alias for onAnyModifySpD)
pub fn dispatch_on_any_modify_sp_d_priority(
    battle: &mut Battle,
    ability_id: &str,
    spd: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_any_modify_sp_d(battle, ability_id, spd, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyModifySpDOrder callbacks (alias for onAnyModifySpD)
pub fn dispatch_on_any_modify_sp_d_order(
    battle: &mut Battle,
    ability_id: &str,
    spd: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_any_modify_sp_d(battle, ability_id, spd, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyModifySpDSubOrder callbacks (alias for onAnyModifySpD)
pub fn dispatch_on_any_modify_sp_d_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    spd: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_any_modify_sp_d(battle, ability_id, spd, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyRedirectTargetPriority callbacks (alias for onAnyRedirectTarget)
pub fn dispatch_on_any_redirect_target_priority(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, source2_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_any_redirect_target(battle, ability_id, target_pos, source_pos, source2_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyRedirectTargetOrder callbacks (alias for onAnyRedirectTarget)
pub fn dispatch_on_any_redirect_target_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, source2_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_any_redirect_target(battle, ability_id, target_pos, source_pos, source2_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyRedirectTargetSubOrder callbacks (alias for onAnyRedirectTarget)
pub fn dispatch_on_any_redirect_target_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, source2_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_any_redirect_target(battle, ability_id, target_pos, source_pos, source2_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnySetWeatherPriority callbacks (alias for onAnySetWeather)
pub fn dispatch_on_any_set_weather_priority(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, weather_id: &str,
) -> EventResult {
    dispatch_on_any_set_weather(battle, ability_id, target_pos, source_pos, weather_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnySetWeatherOrder callbacks (alias for onAnySetWeather)
pub fn dispatch_on_any_set_weather_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, weather_id: &str,
) -> EventResult {
    dispatch_on_any_set_weather(battle, ability_id, target_pos, source_pos, weather_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnySetWeatherSubOrder callbacks (alias for onAnySetWeather)
pub fn dispatch_on_any_set_weather_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, weather_id: &str,
) -> EventResult {
    dispatch_on_any_set_weather(battle, ability_id, target_pos, source_pos, weather_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnySwitchInPriority callbacks (alias for onAnySwitchIn)
pub fn dispatch_on_any_switch_in_priority(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    dispatch_on_any_switch_in(battle, ability_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnySwitchInOrder callbacks (alias for onAnySwitchIn)
pub fn dispatch_on_any_switch_in_order(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    dispatch_on_any_switch_in(battle, ability_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnySwitchInSubOrder callbacks (alias for onAnySwitchIn)
pub fn dispatch_on_any_switch_in_sub_order(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    dispatch_on_any_switch_in(battle, ability_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyTryMovePriority callbacks (alias for onAnyTryMove)
pub fn dispatch_on_any_try_move_priority(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_any_try_move(battle, ability_id, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyTryMoveOrder callbacks (alias for onAnyTryMove)
pub fn dispatch_on_any_try_move_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_any_try_move(battle, ability_id, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyTryMoveSubOrder callbacks (alias for onAnyTryMove)
pub fn dispatch_on_any_try_move_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_any_try_move(battle, ability_id, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyTryPrimaryHitPriority callbacks (alias for onAnyTryPrimaryHit)
pub fn dispatch_on_any_try_primary_hit_priority(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_any_try_primary_hit(battle, ability_id, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyTryPrimaryHitOrder callbacks (alias for onAnyTryPrimaryHit)
pub fn dispatch_on_any_try_primary_hit_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_any_try_primary_hit(battle, ability_id, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onAnyTryPrimaryHitSubOrder callbacks (alias for onAnyTryPrimaryHit)
pub fn dispatch_on_any_try_primary_hit_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_any_try_primary_hit(battle, ability_id, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onBasePowerPriority callbacks (alias for onBasePower)
pub fn dispatch_on_base_power_priority(
    battle: &mut Battle,
    ability_id: &str,
    base_power: i32, attacker_pos: (usize, usize), defender_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_base_power(battle, ability_id, base_power, attacker_pos, defender_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onBasePowerOrder callbacks (alias for onBasePower)
pub fn dispatch_on_base_power_order(
    battle: &mut Battle,
    ability_id: &str,
    base_power: i32, attacker_pos: (usize, usize), defender_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_base_power(battle, ability_id, base_power, attacker_pos, defender_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onBasePowerSubOrder callbacks (alias for onBasePower)
pub fn dispatch_on_base_power_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    base_power: i32, attacker_pos: (usize, usize), defender_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_base_power(battle, ability_id, base_power, attacker_pos, defender_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onBeforeMovePriority callbacks (alias for onBeforeMove)
pub fn dispatch_on_before_move_priority(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_before_move(battle, ability_id, pokemon_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onBeforeMoveOrder callbacks (alias for onBeforeMove)
pub fn dispatch_on_before_move_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_before_move(battle, ability_id, pokemon_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onBeforeMoveSubOrder callbacks (alias for onBeforeMove)
pub fn dispatch_on_before_move_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_before_move(battle, ability_id, pokemon_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onBeforeSwitchInPriority callbacks (alias for onBeforeSwitchIn)
pub fn dispatch_on_before_switch_in_priority(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_before_switch_in(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onBeforeSwitchInOrder callbacks (alias for onBeforeSwitchIn)
pub fn dispatch_on_before_switch_in_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_before_switch_in(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onBeforeSwitchInSubOrder callbacks (alias for onBeforeSwitchIn)
pub fn dispatch_on_before_switch_in_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_before_switch_in(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onChangeBoostPriority callbacks (alias for onChangeBoost)
pub fn dispatch_on_change_boost_priority(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_change_boost(battle, ability_id, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onChangeBoostOrder callbacks (alias for onChangeBoost)
pub fn dispatch_on_change_boost_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_change_boost(battle, ability_id, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onChangeBoostSubOrder callbacks (alias for onChangeBoost)
pub fn dispatch_on_change_boost_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_change_boost(battle, ability_id, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onCheckShowPriority callbacks (alias for onCheckShow)
pub fn dispatch_on_check_show_priority(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_check_show(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onCheckShowOrder callbacks (alias for onCheckShow)
pub fn dispatch_on_check_show_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_check_show(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onCheckShowSubOrder callbacks (alias for onCheckShow)
pub fn dispatch_on_check_show_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_check_show(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onCriticalHitPriority callbacks (alias for onCriticalHit)
pub fn dispatch_on_critical_hit_priority(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_critical_hit(battle, ability_id, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onCriticalHitOrder callbacks (alias for onCriticalHit)
pub fn dispatch_on_critical_hit_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_critical_hit(battle, ability_id, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onCriticalHitSubOrder callbacks (alias for onCriticalHit)
pub fn dispatch_on_critical_hit_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_critical_hit(battle, ability_id, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onDamagePriority callbacks (alias for onDamage)
pub fn dispatch_on_damage_priority(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_damage(battle, ability_id, damage, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onDamageOrder callbacks (alias for onDamage)
pub fn dispatch_on_damage_order(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_damage(battle, ability_id, damage, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onDamageSubOrder callbacks (alias for onDamage)
pub fn dispatch_on_damage_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_damage(battle, ability_id, damage, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onDamagingHitPriority callbacks (alias for onDamagingHit)
pub fn dispatch_on_damaging_hit_priority(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_damaging_hit(battle, ability_id, damage, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onDamagingHitOrder callbacks (alias for onDamagingHit)
pub fn dispatch_on_damaging_hit_order(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_damaging_hit(battle, ability_id, damage, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onDamagingHitSubOrder callbacks (alias for onDamagingHit)
pub fn dispatch_on_damaging_hit_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_damaging_hit(battle, ability_id, damage, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onDeductPPPriority callbacks (alias for onDeductPP)
pub fn dispatch_on_deduct_p_p_priority(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>,
) -> EventResult {
    dispatch_on_deduct_p_p(battle, ability_id, target_pos, source_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onDeductPPOrder callbacks (alias for onDeductPP)
pub fn dispatch_on_deduct_p_p_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>,
) -> EventResult {
    dispatch_on_deduct_p_p(battle, ability_id, target_pos, source_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onDeductPPSubOrder callbacks (alias for onDeductPP)
pub fn dispatch_on_deduct_p_p_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>,
) -> EventResult {
    dispatch_on_deduct_p_p(battle, ability_id, target_pos, source_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onDisableMovePriority callbacks (alias for onDisableMove)
pub fn dispatch_on_disable_move_priority(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_disable_move(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onDisableMoveOrder callbacks (alias for onDisableMove)
pub fn dispatch_on_disable_move_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_disable_move(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onDisableMoveSubOrder callbacks (alias for onDisableMove)
pub fn dispatch_on_disable_move_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_disable_move(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onDragOutPriority callbacks (alias for onDragOut)
pub fn dispatch_on_drag_out_priority(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_drag_out(battle, ability_id, pokemon_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onDragOutOrder callbacks (alias for onDragOut)
pub fn dispatch_on_drag_out_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_drag_out(battle, ability_id, pokemon_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onDragOutSubOrder callbacks (alias for onDragOut)
pub fn dispatch_on_drag_out_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_drag_out(battle, ability_id, pokemon_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onEatItemPriority callbacks (alias for onEatItem)
pub fn dispatch_on_eat_item_priority(
    battle: &mut Battle,
    ability_id: &str,
    item_id: Option<&str>, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_eat_item(battle, ability_id, item_id, pokemon_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onEatItemOrder callbacks (alias for onEatItem)
pub fn dispatch_on_eat_item_order(
    battle: &mut Battle,
    ability_id: &str,
    item_id: Option<&str>, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_eat_item(battle, ability_id, item_id, pokemon_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onEatItemSubOrder callbacks (alias for onEatItem)
pub fn dispatch_on_eat_item_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    item_id: Option<&str>, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_eat_item(battle, ability_id, item_id, pokemon_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onEffectivenessPriority callbacks (alias for onEffectiveness)
pub fn dispatch_on_effectiveness_priority(
    battle: &mut Battle,
    ability_id: &str,
    type_mod: i32, target_pos: (usize, usize), type_str: &str, move_id: &str,
) -> EventResult {
    dispatch_on_effectiveness(battle, ability_id, type_mod, target_pos, type_str, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onEffectivenessOrder callbacks (alias for onEffectiveness)
pub fn dispatch_on_effectiveness_order(
    battle: &mut Battle,
    ability_id: &str,
    type_mod: i32, target_pos: (usize, usize), type_str: &str, move_id: &str,
) -> EventResult {
    dispatch_on_effectiveness(battle, ability_id, type_mod, target_pos, type_str, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onEffectivenessSubOrder callbacks (alias for onEffectiveness)
pub fn dispatch_on_effectiveness_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    type_mod: i32, target_pos: (usize, usize), type_str: &str, move_id: &str,
) -> EventResult {
    dispatch_on_effectiveness(battle, ability_id, type_mod, target_pos, type_str, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onEmergencyExitPriority callbacks (alias for onEmergencyExit)
pub fn dispatch_on_emergency_exit_priority(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    dispatch_on_emergency_exit(battle, ability_id, target_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onEmergencyExitOrder callbacks (alias for onEmergencyExit)
pub fn dispatch_on_emergency_exit_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    dispatch_on_emergency_exit(battle, ability_id, target_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onEmergencyExitSubOrder callbacks (alias for onEmergencyExit)
pub fn dispatch_on_emergency_exit_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    dispatch_on_emergency_exit(battle, ability_id, target_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onEndPriority callbacks (alias for onEnd)
pub fn dispatch_on_end_priority(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_end(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onEndOrder callbacks (alias for onEnd)
pub fn dispatch_on_end_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_end(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onEndSubOrder callbacks (alias for onEnd)
pub fn dispatch_on_end_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_end(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onFaintPriority callbacks (alias for onFaint)
pub fn dispatch_on_faint_priority(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
    effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_faint(battle, ability_id, pokemon_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onFaintOrder callbacks (alias for onFaint)
pub fn dispatch_on_faint_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
    effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_faint(battle, ability_id, pokemon_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onFaintSubOrder callbacks (alias for onFaint)
pub fn dispatch_on_faint_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
    effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_faint(battle, ability_id, pokemon_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onFlinchPriority callbacks (alias for onFlinch)
pub fn dispatch_on_flinch_priority(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_flinch(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onFlinchOrder callbacks (alias for onFlinch)
pub fn dispatch_on_flinch_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_flinch(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onFlinchSubOrder callbacks (alias for onFlinch)
pub fn dispatch_on_flinch_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_flinch(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onFoeAfterBoostPriority callbacks (alias for onFoeAfterBoost)
pub fn dispatch_on_foe_after_boost_priority(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_foe_after_boost(battle, ability_id, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onFoeAfterBoostOrder callbacks (alias for onFoeAfterBoost)
pub fn dispatch_on_foe_after_boost_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_foe_after_boost(battle, ability_id, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onFoeAfterBoostSubOrder callbacks (alias for onFoeAfterBoost)
pub fn dispatch_on_foe_after_boost_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_foe_after_boost(battle, ability_id, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onFoeMaybeTrapPokemonPriority callbacks (alias for onFoeMaybeTrapPokemon)
pub fn dispatch_on_foe_maybe_trap_pokemon_priority(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>,
) -> EventResult {
    dispatch_on_foe_maybe_trap_pokemon(battle, ability_id, pokemon_pos, source_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onFoeMaybeTrapPokemonOrder callbacks (alias for onFoeMaybeTrapPokemon)
pub fn dispatch_on_foe_maybe_trap_pokemon_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>,
) -> EventResult {
    dispatch_on_foe_maybe_trap_pokemon(battle, ability_id, pokemon_pos, source_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onFoeMaybeTrapPokemonSubOrder callbacks (alias for onFoeMaybeTrapPokemon)
pub fn dispatch_on_foe_maybe_trap_pokemon_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>,
) -> EventResult {
    dispatch_on_foe_maybe_trap_pokemon(battle, ability_id, pokemon_pos, source_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onFoeTrapPokemonPriority callbacks (alias for onFoeTrapPokemon)
pub fn dispatch_on_foe_trap_pokemon_priority(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_foe_trap_pokemon(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onFoeTrapPokemonOrder callbacks (alias for onFoeTrapPokemon)
pub fn dispatch_on_foe_trap_pokemon_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_foe_trap_pokemon(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onFoeTrapPokemonSubOrder callbacks (alias for onFoeTrapPokemon)
pub fn dispatch_on_foe_trap_pokemon_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_foe_trap_pokemon(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onFoeTryEatItemPriority callbacks (alias for onFoeTryEatItem)
pub fn dispatch_on_foe_try_eat_item_priority(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    dispatch_on_foe_try_eat_item(battle, ability_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onFoeTryEatItemOrder callbacks (alias for onFoeTryEatItem)
pub fn dispatch_on_foe_try_eat_item_order(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    dispatch_on_foe_try_eat_item(battle, ability_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onFoeTryEatItemSubOrder callbacks (alias for onFoeTryEatItem)
pub fn dispatch_on_foe_try_eat_item_sub_order(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    dispatch_on_foe_try_eat_item(battle, ability_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onFoeTryMovePriority callbacks (alias for onFoeTryMove)
pub fn dispatch_on_foe_try_move_priority(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_foe_try_move(battle, ability_id, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onFoeTryMoveOrder callbacks (alias for onFoeTryMove)
pub fn dispatch_on_foe_try_move_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_foe_try_move(battle, ability_id, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onFoeTryMoveSubOrder callbacks (alias for onFoeTryMove)
pub fn dispatch_on_foe_try_move_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_foe_try_move(battle, ability_id, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onFractionalPriorityPriority callbacks (alias for onFractionalPriority)
pub fn dispatch_on_fractional_priority_priority(
    battle: &mut Battle,
    ability_id: &str,
    priority: i32,
    pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_fractional_priority(battle, ability_id, priority, pokemon_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onFractionalPriorityOrder callbacks (alias for onFractionalPriority)
pub fn dispatch_on_fractional_priority_order(
    battle: &mut Battle,
    ability_id: &str,
    priority: i32,
    pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_fractional_priority(battle, ability_id, priority, pokemon_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onFractionalPrioritySubOrder callbacks (alias for onFractionalPriority)
pub fn dispatch_on_fractional_priority_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    priority: i32,
    pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_fractional_priority(battle, ability_id, priority, pokemon_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onHitPriority callbacks (alias for onHit)
pub fn dispatch_on_hit_priority(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize), source_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_hit(battle, ability_id, pokemon_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onHitOrder callbacks (alias for onHit)
pub fn dispatch_on_hit_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize), source_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_hit(battle, ability_id, pokemon_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onHitSubOrder callbacks (alias for onHit)
pub fn dispatch_on_hit_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize), source_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_hit(battle, ability_id, pokemon_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onImmunityPriority callbacks (alias for onImmunity)
pub fn dispatch_on_immunity_priority(
    battle: &mut Battle,
    ability_id: &str,
    type_or_status: &str, pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_immunity(battle, ability_id, type_or_status, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onImmunityOrder callbacks (alias for onImmunity)
pub fn dispatch_on_immunity_order(
    battle: &mut Battle,
    ability_id: &str,
    type_or_status: &str, pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_immunity(battle, ability_id, type_or_status, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onImmunitySubOrder callbacks (alias for onImmunity)
pub fn dispatch_on_immunity_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    type_or_status: &str, pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_immunity(battle, ability_id, type_or_status, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifyAccuracyPriority callbacks (alias for onModifyAccuracy)
pub fn dispatch_on_modify_accuracy_priority(
    battle: &mut Battle,
    ability_id: &str,
    accuracy: i32, target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_modify_accuracy(battle, ability_id, accuracy, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifyAccuracyOrder callbacks (alias for onModifyAccuracy)
pub fn dispatch_on_modify_accuracy_order(
    battle: &mut Battle,
    ability_id: &str,
    accuracy: i32, target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_modify_accuracy(battle, ability_id, accuracy, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifyAccuracySubOrder callbacks (alias for onModifyAccuracy)
pub fn dispatch_on_modify_accuracy_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    accuracy: i32, target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_modify_accuracy(battle, ability_id, accuracy, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifyAtkPriority callbacks (alias for onModifyAtk)
pub fn dispatch_on_modify_atk_priority(
    battle: &mut Battle,
    ability_id: &str,
    atk: i32, attacker_pos: (usize, usize), defender_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_modify_atk(battle, ability_id, atk, attacker_pos, defender_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifyAtkOrder callbacks (alias for onModifyAtk)
pub fn dispatch_on_modify_atk_order(
    battle: &mut Battle,
    ability_id: &str,
    atk: i32, attacker_pos: (usize, usize), defender_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_modify_atk(battle, ability_id, atk, attacker_pos, defender_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifyAtkSubOrder callbacks (alias for onModifyAtk)
pub fn dispatch_on_modify_atk_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    atk: i32, attacker_pos: (usize, usize), defender_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_modify_atk(battle, ability_id, atk, attacker_pos, defender_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifyCritRatioPriority callbacks (alias for onModifyCritRatio)
pub fn dispatch_on_modify_crit_ratio_priority(
    battle: &mut Battle,
    ability_id: &str,
    crit_ratio: i32, source_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_modify_crit_ratio(battle, ability_id, crit_ratio, source_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifyCritRatioOrder callbacks (alias for onModifyCritRatio)
pub fn dispatch_on_modify_crit_ratio_order(
    battle: &mut Battle,
    ability_id: &str,
    crit_ratio: i32, source_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_modify_crit_ratio(battle, ability_id, crit_ratio, source_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifyCritRatioSubOrder callbacks (alias for onModifyCritRatio)
pub fn dispatch_on_modify_crit_ratio_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    crit_ratio: i32, source_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_modify_crit_ratio(battle, ability_id, crit_ratio, source_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifyDamagePriority callbacks (alias for onModifyDamage)
pub fn dispatch_on_modify_damage_priority(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, source_pos: (usize, usize), target_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_modify_damage(battle, ability_id, damage, source_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifyDamageOrder callbacks (alias for onModifyDamage)
pub fn dispatch_on_modify_damage_order(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, source_pos: (usize, usize), target_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_modify_damage(battle, ability_id, damage, source_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifyDamageSubOrder callbacks (alias for onModifyDamage)
pub fn dispatch_on_modify_damage_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, source_pos: (usize, usize), target_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_modify_damage(battle, ability_id, damage, source_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifyDefPriority callbacks (alias for onModifyDef)
pub fn dispatch_on_modify_def_priority(
    battle: &mut Battle,
    ability_id: &str,
    def: i32, defender_pos: (usize, usize), attacker_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_modify_def(battle, ability_id, def, defender_pos, attacker_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifyDefOrder callbacks (alias for onModifyDef)
pub fn dispatch_on_modify_def_order(
    battle: &mut Battle,
    ability_id: &str,
    def: i32, defender_pos: (usize, usize), attacker_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_modify_def(battle, ability_id, def, defender_pos, attacker_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifyDefSubOrder callbacks (alias for onModifyDef)
pub fn dispatch_on_modify_def_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    def: i32, defender_pos: (usize, usize), attacker_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_modify_def(battle, ability_id, def, defender_pos, attacker_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifyMovePriority callbacks (alias for onModifyMove)
pub fn dispatch_on_modify_move_priority(
    battle: &mut Battle,
    ability_id: &str,
    move_id: &str,
    source_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    dispatch_on_modify_move(battle, ability_id, move_id, source_pos, target_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifyMoveOrder callbacks (alias for onModifyMove)
pub fn dispatch_on_modify_move_order(
    battle: &mut Battle,
    ability_id: &str,
    move_id: &str,
    source_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    dispatch_on_modify_move(battle, ability_id, move_id, source_pos, target_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifyMoveSubOrder callbacks (alias for onModifyMove)
pub fn dispatch_on_modify_move_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    move_id: &str,
    source_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    dispatch_on_modify_move(battle, ability_id, move_id, source_pos, target_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifyPriorityPriority callbacks (alias for onModifyPriority)
pub fn dispatch_on_modify_priority_priority(
    battle: &mut Battle,
    ability_id: &str,
    priority: i32, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_modify_priority(battle, ability_id, priority, pokemon_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifyPriorityOrder callbacks (alias for onModifyPriority)
pub fn dispatch_on_modify_priority_order(
    battle: &mut Battle,
    ability_id: &str,
    priority: i32, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_modify_priority(battle, ability_id, priority, pokemon_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifyPrioritySubOrder callbacks (alias for onModifyPriority)
pub fn dispatch_on_modify_priority_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    priority: i32, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_modify_priority(battle, ability_id, priority, pokemon_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifySTABPriority callbacks (alias for onModifySTAB)
pub fn dispatch_on_modify_s_t_a_b_priority(
    battle: &mut Battle,
    ability_id: &str,
    stab: f64,
    source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_modify_s_t_a_b(battle, ability_id, stab, source_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifySTABOrder callbacks (alias for onModifySTAB)
pub fn dispatch_on_modify_s_t_a_b_order(
    battle: &mut Battle,
    ability_id: &str,
    stab: f64,
    source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_modify_s_t_a_b(battle, ability_id, stab, source_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifySTABSubOrder callbacks (alias for onModifySTAB)
pub fn dispatch_on_modify_s_t_a_b_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    stab: f64,
    source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_modify_s_t_a_b(battle, ability_id, stab, source_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifySecondariesPriority callbacks (alias for onModifySecondaries)
pub fn dispatch_on_modify_secondaries_priority(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    dispatch_on_modify_secondaries(battle, ability_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifySecondariesOrder callbacks (alias for onModifySecondaries)
pub fn dispatch_on_modify_secondaries_order(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    dispatch_on_modify_secondaries(battle, ability_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifySecondariesSubOrder callbacks (alias for onModifySecondaries)
pub fn dispatch_on_modify_secondaries_sub_order(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    dispatch_on_modify_secondaries(battle, ability_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifySpAPriority callbacks (alias for onModifySpA)
pub fn dispatch_on_modify_sp_a_priority(
    battle: &mut Battle,
    ability_id: &str,
    spa: i32, attacker_pos: (usize, usize), defender_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_modify_sp_a(battle, ability_id, spa, attacker_pos, defender_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifySpAOrder callbacks (alias for onModifySpA)
pub fn dispatch_on_modify_sp_a_order(
    battle: &mut Battle,
    ability_id: &str,
    spa: i32, attacker_pos: (usize, usize), defender_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_modify_sp_a(battle, ability_id, spa, attacker_pos, defender_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifySpASubOrder callbacks (alias for onModifySpA)
pub fn dispatch_on_modify_sp_a_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    spa: i32, attacker_pos: (usize, usize), defender_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_modify_sp_a(battle, ability_id, spa, attacker_pos, defender_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifySpePriority callbacks (alias for onModifySpe)
pub fn dispatch_on_modify_spe_priority(
    battle: &mut Battle,
    ability_id: &str,
    spe: i32, pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_modify_spe(battle, ability_id, spe, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifySpeOrder callbacks (alias for onModifySpe)
pub fn dispatch_on_modify_spe_order(
    battle: &mut Battle,
    ability_id: &str,
    spe: i32, pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_modify_spe(battle, ability_id, spe, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifySpeSubOrder callbacks (alias for onModifySpe)
pub fn dispatch_on_modify_spe_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    spe: i32, pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_modify_spe(battle, ability_id, spe, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifyTypePriority callbacks (alias for onModifyType)
pub fn dispatch_on_modify_type_priority(
    battle: &mut Battle,
    ability_id: &str,
    move_id: &str, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>,
) -> EventResult {
    dispatch_on_modify_type(battle, ability_id, move_id, pokemon_pos, target_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifyTypeOrder callbacks (alias for onModifyType)
pub fn dispatch_on_modify_type_order(
    battle: &mut Battle,
    ability_id: &str,
    move_id: &str, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>,
) -> EventResult {
    dispatch_on_modify_type(battle, ability_id, move_id, pokemon_pos, target_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifyTypeSubOrder callbacks (alias for onModifyType)
pub fn dispatch_on_modify_type_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    move_id: &str, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>,
) -> EventResult {
    dispatch_on_modify_type(battle, ability_id, move_id, pokemon_pos, target_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifyWeightPriority callbacks (alias for onModifyWeight)
pub fn dispatch_on_modify_weight_priority(
    battle: &mut Battle,
    ability_id: &str,
    weight: i32, pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_modify_weight(battle, ability_id, weight, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifyWeightOrder callbacks (alias for onModifyWeight)
pub fn dispatch_on_modify_weight_order(
    battle: &mut Battle,
    ability_id: &str,
    weight: i32, pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_modify_weight(battle, ability_id, weight, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onModifyWeightSubOrder callbacks (alias for onModifyWeight)
pub fn dispatch_on_modify_weight_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    weight: i32, pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_modify_weight(battle, ability_id, weight, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onPrepareHitPriority callbacks (alias for onPrepareHit)
pub fn dispatch_on_prepare_hit_priority(
    battle: &mut Battle,
    ability_id: &str,
    source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_prepare_hit(battle, ability_id, source_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onPrepareHitOrder callbacks (alias for onPrepareHit)
pub fn dispatch_on_prepare_hit_order(
    battle: &mut Battle,
    ability_id: &str,
    source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_prepare_hit(battle, ability_id, source_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onPrepareHitSubOrder callbacks (alias for onPrepareHit)
pub fn dispatch_on_prepare_hit_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_prepare_hit(battle, ability_id, source_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onResidualPriority callbacks (alias for onResidual)
pub fn dispatch_on_residual_priority(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_residual(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onResidualOrder callbacks (alias for onResidual)
pub fn dispatch_on_residual_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_residual(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onResidualSubOrder callbacks (alias for onResidual)
pub fn dispatch_on_residual_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_residual(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSetStatusPriority callbacks (alias for onSetStatus)
pub fn dispatch_on_set_status_priority(
    battle: &mut Battle,
    ability_id: &str,
    status_id: &str, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_set_status(battle, ability_id, status_id, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSetStatusOrder callbacks (alias for onSetStatus)
pub fn dispatch_on_set_status_order(
    battle: &mut Battle,
    ability_id: &str,
    status_id: &str, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_set_status(battle, ability_id, status_id, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSetStatusSubOrder callbacks (alias for onSetStatus)
pub fn dispatch_on_set_status_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    status_id: &str, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_set_status(battle, ability_id, status_id, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSideConditionStartPriority callbacks (alias for onSideConditionStart)
pub fn dispatch_on_side_condition_start_priority(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
    side_idx: usize,
    side_condition_id: &str,
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    dispatch_on_side_condition_start(battle, ability_id, pokemon_pos, side_idx, side_condition_id, source_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSideConditionStartOrder callbacks (alias for onSideConditionStart)
pub fn dispatch_on_side_condition_start_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
    side_idx: usize,
    side_condition_id: &str,
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    dispatch_on_side_condition_start(battle, ability_id, pokemon_pos, side_idx, side_condition_id, source_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSideConditionStartSubOrder callbacks (alias for onSideConditionStart)
pub fn dispatch_on_side_condition_start_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
    side_idx: usize,
    side_condition_id: &str,
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    dispatch_on_side_condition_start(battle, ability_id, pokemon_pos, side_idx, side_condition_id, source_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSourceAfterFaintPriority callbacks (alias for onSourceAfterFaint)
pub fn dispatch_on_source_after_faint_priority(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_source_after_faint(battle, ability_id, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSourceAfterFaintOrder callbacks (alias for onSourceAfterFaint)
pub fn dispatch_on_source_after_faint_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_source_after_faint(battle, ability_id, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSourceAfterFaintSubOrder callbacks (alias for onSourceAfterFaint)
pub fn dispatch_on_source_after_faint_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_source_after_faint(battle, ability_id, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSourceBasePowerPriority callbacks (alias for onSourceBasePower)
pub fn dispatch_on_source_base_power_priority(
    battle: &mut Battle,
    ability_id: &str,
    base_power: i32, move_id: &str,
) -> EventResult {
    dispatch_on_source_base_power(battle, ability_id, base_power, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSourceBasePowerOrder callbacks (alias for onSourceBasePower)
pub fn dispatch_on_source_base_power_order(
    battle: &mut Battle,
    ability_id: &str,
    base_power: i32, move_id: &str,
) -> EventResult {
    dispatch_on_source_base_power(battle, ability_id, base_power, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSourceBasePowerSubOrder callbacks (alias for onSourceBasePower)
pub fn dispatch_on_source_base_power_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    base_power: i32, move_id: &str,
) -> EventResult {
    dispatch_on_source_base_power(battle, ability_id, base_power, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSourceDamagingHitPriority callbacks (alias for onSourceDamagingHit)
pub fn dispatch_on_source_damaging_hit_priority(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_source_damaging_hit(battle, ability_id, damage, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSourceDamagingHitOrder callbacks (alias for onSourceDamagingHit)
pub fn dispatch_on_source_damaging_hit_order(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_source_damaging_hit(battle, ability_id, damage, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSourceDamagingHitSubOrder callbacks (alias for onSourceDamagingHit)
pub fn dispatch_on_source_damaging_hit_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_source_damaging_hit(battle, ability_id, damage, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSourceModifyAccuracyPriority callbacks (alias for onSourceModifyAccuracy)
pub fn dispatch_on_source_modify_accuracy_priority(
    battle: &mut Battle,
    ability_id: &str,
    accuracy: i32, target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_source_modify_accuracy(battle, ability_id, accuracy, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSourceModifyAccuracyOrder callbacks (alias for onSourceModifyAccuracy)
pub fn dispatch_on_source_modify_accuracy_order(
    battle: &mut Battle,
    ability_id: &str,
    accuracy: i32, target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_source_modify_accuracy(battle, ability_id, accuracy, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSourceModifyAccuracySubOrder callbacks (alias for onSourceModifyAccuracy)
pub fn dispatch_on_source_modify_accuracy_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    accuracy: i32, target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_source_modify_accuracy(battle, ability_id, accuracy, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSourceModifyAtkPriority callbacks (alias for onSourceModifyAtk)
pub fn dispatch_on_source_modify_atk_priority(
    battle: &mut Battle,
    ability_id: &str,
    move_id: &str,
) -> EventResult {
    dispatch_on_source_modify_atk(battle, ability_id, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSourceModifyAtkOrder callbacks (alias for onSourceModifyAtk)
pub fn dispatch_on_source_modify_atk_order(
    battle: &mut Battle,
    ability_id: &str,
    move_id: &str,
) -> EventResult {
    dispatch_on_source_modify_atk(battle, ability_id, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSourceModifyAtkSubOrder callbacks (alias for onSourceModifyAtk)
pub fn dispatch_on_source_modify_atk_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    move_id: &str,
) -> EventResult {
    dispatch_on_source_modify_atk(battle, ability_id, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSourceModifyDamagePriority callbacks (alias for onSourceModifyDamage)
pub fn dispatch_on_source_modify_damage_priority(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, source_pos: (usize, usize), target_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_source_modify_damage(battle, ability_id, damage, source_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSourceModifyDamageOrder callbacks (alias for onSourceModifyDamage)
pub fn dispatch_on_source_modify_damage_order(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, source_pos: (usize, usize), target_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_source_modify_damage(battle, ability_id, damage, source_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSourceModifyDamageSubOrder callbacks (alias for onSourceModifyDamage)
pub fn dispatch_on_source_modify_damage_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, source_pos: (usize, usize), target_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_source_modify_damage(battle, ability_id, damage, source_pos, target_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSourceModifySecondariesPriority callbacks (alias for onSourceModifySecondaries)
pub fn dispatch_on_source_modify_secondaries_priority(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_source_modify_secondaries(battle, ability_id, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSourceModifySecondariesOrder callbacks (alias for onSourceModifySecondaries)
pub fn dispatch_on_source_modify_secondaries_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_source_modify_secondaries(battle, ability_id, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSourceModifySecondariesSubOrder callbacks (alias for onSourceModifySecondaries)
pub fn dispatch_on_source_modify_secondaries_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str,
) -> EventResult {
    dispatch_on_source_modify_secondaries(battle, ability_id, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSourceModifySpAPriority callbacks (alias for onSourceModifySpA)
pub fn dispatch_on_source_modify_sp_a_priority(
    battle: &mut Battle,
    ability_id: &str,
    move_id: &str,
) -> EventResult {
    dispatch_on_source_modify_sp_a(battle, ability_id, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSourceModifySpAOrder callbacks (alias for onSourceModifySpA)
pub fn dispatch_on_source_modify_sp_a_order(
    battle: &mut Battle,
    ability_id: &str,
    move_id: &str,
) -> EventResult {
    dispatch_on_source_modify_sp_a(battle, ability_id, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSourceModifySpASubOrder callbacks (alias for onSourceModifySpA)
pub fn dispatch_on_source_modify_sp_a_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    move_id: &str,
) -> EventResult {
    dispatch_on_source_modify_sp_a(battle, ability_id, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSourceTryHealPriority callbacks (alias for onSourceTryHeal)
pub fn dispatch_on_source_try_heal_priority(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_source_try_heal(battle, ability_id, damage, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSourceTryHealOrder callbacks (alias for onSourceTryHeal)
pub fn dispatch_on_source_try_heal_order(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_source_try_heal(battle, ability_id, damage, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSourceTryHealSubOrder callbacks (alias for onSourceTryHeal)
pub fn dispatch_on_source_try_heal_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_source_try_heal(battle, ability_id, damage, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSourceTryPrimaryHitPriority callbacks (alias for onSourceTryPrimaryHit)
pub fn dispatch_on_source_try_primary_hit_priority(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_source_try_primary_hit(battle, ability_id, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSourceTryPrimaryHitOrder callbacks (alias for onSourceTryPrimaryHit)
pub fn dispatch_on_source_try_primary_hit_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_source_try_primary_hit(battle, ability_id, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSourceTryPrimaryHitSubOrder callbacks (alias for onSourceTryPrimaryHit)
pub fn dispatch_on_source_try_primary_hit_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_source_try_primary_hit(battle, ability_id, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onStartPriority callbacks (alias for onStart)
pub fn dispatch_on_start_priority(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_start(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onStartOrder callbacks (alias for onStart)
pub fn dispatch_on_start_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_start(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onStartSubOrder callbacks (alias for onStart)
pub fn dispatch_on_start_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_start(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSwitchInPriority callbacks (alias for onSwitchIn)
pub fn dispatch_on_switch_in_priority(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_switch_in(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSwitchInOrder callbacks (alias for onSwitchIn)
pub fn dispatch_on_switch_in_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_switch_in(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSwitchInSubOrder callbacks (alias for onSwitchIn)
pub fn dispatch_on_switch_in_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_switch_in(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSwitchOutPriority callbacks (alias for onSwitchOut)
pub fn dispatch_on_switch_out_priority(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_switch_out(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSwitchOutOrder callbacks (alias for onSwitchOut)
pub fn dispatch_on_switch_out_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_switch_out(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onSwitchOutSubOrder callbacks (alias for onSwitchOut)
pub fn dispatch_on_switch_out_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_switch_out(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onTakeItemPriority callbacks (alias for onTakeItem)
pub fn dispatch_on_take_item_priority(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>,
) -> EventResult {
    dispatch_on_take_item(battle, ability_id, pokemon_pos, source_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onTakeItemOrder callbacks (alias for onTakeItem)
pub fn dispatch_on_take_item_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>,
) -> EventResult {
    dispatch_on_take_item(battle, ability_id, pokemon_pos, source_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onTakeItemSubOrder callbacks (alias for onTakeItem)
pub fn dispatch_on_take_item_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>,
) -> EventResult {
    dispatch_on_take_item(battle, ability_id, pokemon_pos, source_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onTerrainChangePriority callbacks (alias for onTerrainChange)
pub fn dispatch_on_terrain_change_priority(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_terrain_change(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onTerrainChangeOrder callbacks (alias for onTerrainChange)
pub fn dispatch_on_terrain_change_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_terrain_change(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onTerrainChangeSubOrder callbacks (alias for onTerrainChange)
pub fn dispatch_on_terrain_change_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_terrain_change(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onTryAddVolatilePriority callbacks (alias for onTryAddVolatile)
pub fn dispatch_on_try_add_volatile_priority(
    battle: &mut Battle,
    ability_id: &str,
    status_id: &str, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_try_add_volatile(battle, ability_id, status_id, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onTryAddVolatileOrder callbacks (alias for onTryAddVolatile)
pub fn dispatch_on_try_add_volatile_order(
    battle: &mut Battle,
    ability_id: &str,
    status_id: &str, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_try_add_volatile(battle, ability_id, status_id, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onTryAddVolatileSubOrder callbacks (alias for onTryAddVolatile)
pub fn dispatch_on_try_add_volatile_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    status_id: &str, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_try_add_volatile(battle, ability_id, status_id, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onTryBoostPriority callbacks (alias for onTryBoost)
pub fn dispatch_on_try_boost_priority(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: (usize, usize),
    boost: Option<&mut crate::dex_data::BoostsTable>,
) -> EventResult {
    dispatch_on_try_boost(battle, ability_id, target_pos, boost)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onTryBoostOrder callbacks (alias for onTryBoost)
pub fn dispatch_on_try_boost_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: (usize, usize),
    boost: Option<&mut crate::dex_data::BoostsTable>,
) -> EventResult {
    dispatch_on_try_boost(battle, ability_id, target_pos, boost)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onTryBoostSubOrder callbacks (alias for onTryBoost)
pub fn dispatch_on_try_boost_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: (usize, usize),
    boost: Option<&mut crate::dex_data::BoostsTable>,
) -> EventResult {
    dispatch_on_try_boost(battle, ability_id, target_pos, boost)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onTryEatItemPriority callbacks (alias for onTryEatItem)
pub fn dispatch_on_try_eat_item_priority(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    dispatch_on_try_eat_item(battle, ability_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onTryEatItemOrder callbacks (alias for onTryEatItem)
pub fn dispatch_on_try_eat_item_order(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    dispatch_on_try_eat_item(battle, ability_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onTryEatItemSubOrder callbacks (alias for onTryEatItem)
pub fn dispatch_on_try_eat_item_sub_order(
    battle: &mut Battle,
    ability_id: &str,
) -> EventResult {
    dispatch_on_try_eat_item(battle, ability_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onTryHealPriority callbacks (alias for onTryHeal)
pub fn dispatch_on_try_heal_priority(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_try_heal(battle, ability_id, damage, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onTryHealOrder callbacks (alias for onTryHeal)
pub fn dispatch_on_try_heal_order(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_try_heal(battle, ability_id, damage, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onTryHealSubOrder callbacks (alias for onTryHeal)
pub fn dispatch_on_try_heal_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>,
) -> EventResult {
    dispatch_on_try_heal(battle, ability_id, damage, target_pos, source_pos, effect_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onTryHitPriority callbacks (alias for onTryHit)
pub fn dispatch_on_try_hit_priority(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_try_hit(battle, ability_id, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onTryHitOrder callbacks (alias for onTryHit)
pub fn dispatch_on_try_hit_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_try_hit(battle, ability_id, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onTryHitSubOrder callbacks (alias for onTryHit)
pub fn dispatch_on_try_hit_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str,
) -> EventResult {
    dispatch_on_try_hit(battle, ability_id, target_pos, source_pos, move_id)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onUpdatePriority callbacks (alias for onUpdate)
pub fn dispatch_on_update_priority(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_update(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onUpdateOrder callbacks (alias for onUpdate)
pub fn dispatch_on_update_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_update(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onUpdateSubOrder callbacks (alias for onUpdate)
pub fn dispatch_on_update_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_update(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onWeatherPriority callbacks (alias for onWeather)
pub fn dispatch_on_weather_priority(
    battle: &mut Battle,
    ability_id: &str,
    weather_id: &str, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>,
) -> EventResult {
    dispatch_on_weather(battle, ability_id, weather_id, pokemon_pos, source_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onWeatherOrder callbacks (alias for onWeather)
pub fn dispatch_on_weather_order(
    battle: &mut Battle,
    ability_id: &str,
    weather_id: &str, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>,
) -> EventResult {
    dispatch_on_weather(battle, ability_id, weather_id, pokemon_pos, source_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onWeatherSubOrder callbacks (alias for onWeather)
pub fn dispatch_on_weather_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    weather_id: &str, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>,
) -> EventResult {
    dispatch_on_weather(battle, ability_id, weather_id, pokemon_pos, source_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onWeatherChangePriority callbacks (alias for onWeatherChange)
pub fn dispatch_on_weather_change_priority(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_weather_change(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onWeatherChangeOrder callbacks (alias for onWeatherChange)
pub fn dispatch_on_weather_change_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_weather_change(battle, ability_id, pokemon_pos)
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch onWeatherChangeSubOrder callbacks (alias for onWeatherChange)
pub fn dispatch_on_weather_change_sub_order(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    dispatch_on_weather_change(battle, ability_id, pokemon_pos)
}

// Condition dispatch functions
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND
/// Dispatch condition onEnd callbacks
pub fn dispatch_condition_on_end(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "flashfire" => flashfire::condition::on_end(battle, pokemon_pos),
        "protosynthesis" => protosynthesis::condition::on_end(battle, pokemon_pos),
        "quarkdrive" => quarkdrive::condition::on_end(battle, pokemon_pos),
        "zenmode" => zenmode::condition::on_end(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch condition onModifyAtk callbacks
pub fn dispatch_condition_on_modify_atk(
    battle: &mut Battle,
    ability_id: &str,
    atk: i32, attacker_pos: (usize, usize), defender_pos: (usize, usize), move_id: &str,
) -> EventResult {
    match ability_id {
        "flashfire" => flashfire::condition::on_modify_atk(battle, atk, attacker_pos, defender_pos, move_id),
        "protosynthesis" => protosynthesis::condition::on_modify_atk(battle, atk, attacker_pos, defender_pos, move_id),
        "quarkdrive" => quarkdrive::condition::on_modify_atk(battle, atk, attacker_pos, defender_pos, move_id),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch condition onModifyDef callbacks
pub fn dispatch_condition_on_modify_def(
    battle: &mut Battle,
    ability_id: &str,
    def: i32, defender_pos: (usize, usize), attacker_pos: (usize, usize), move_id: &str,
) -> EventResult {
    match ability_id {
        "protosynthesis" => protosynthesis::condition::on_modify_def(battle, def, defender_pos, attacker_pos, move_id),
        "quarkdrive" => quarkdrive::condition::on_modify_def(battle, def, defender_pos, attacker_pos, move_id),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch condition onModifySpA callbacks
pub fn dispatch_condition_on_modify_sp_a(
    battle: &mut Battle,
    ability_id: &str,
    spa: i32, attacker_pos: (usize, usize), defender_pos: (usize, usize), move_id: &str,
) -> EventResult {
    match ability_id {
        "flashfire" => flashfire::condition::on_modify_sp_a(battle, spa, attacker_pos, defender_pos, move_id),
        "protosynthesis" => protosynthesis::condition::on_modify_sp_a(battle, spa, attacker_pos, defender_pos, move_id),
        "quarkdrive" => quarkdrive::condition::on_modify_sp_a(battle, spa, attacker_pos, defender_pos, move_id),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch condition onModifySpD callbacks
pub fn dispatch_condition_on_modify_sp_d(
    battle: &mut Battle,
    ability_id: &str,
    spd: i32, defender_pos: (usize, usize), attacker_pos: (usize, usize), move_id: &str,
) -> EventResult {
    match ability_id {
        "protosynthesis" => protosynthesis::condition::on_modify_sp_d(battle, spd, defender_pos, attacker_pos, move_id),
        "quarkdrive" => quarkdrive::condition::on_modify_sp_d(battle, spd, defender_pos, attacker_pos, move_id),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch condition onModifySpe callbacks
pub fn dispatch_condition_on_modify_spe(
    battle: &mut Battle,
    ability_id: &str,
    spe: i32, pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "protosynthesis" => protosynthesis::condition::on_modify_spe(battle, spe, pokemon_pos),
        "quarkdrive" => quarkdrive::condition::on_modify_spe(battle, spe, pokemon_pos),
        "unburden" => unburden::condition::on_modify_spe(battle, spe, pokemon_pos),
        _ => EventResult::Continue,
    }
}
// TODO: verify that the list of calls in JavaScript matches the Rust equivalent
// JavaScript signatures: NONE FOUND

/// Dispatch condition onStart callbacks
pub fn dispatch_condition_on_start(
    battle: &mut Battle,
    ability_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "flashfire" => flashfire::condition::on_start(battle, pokemon_pos),
        "protosynthesis" => protosynthesis::condition::on_start(battle, pokemon_pos),
        "quarkdrive" => quarkdrive::condition::on_start(battle, pokemon_pos),
        "zenmode" => zenmode::condition::on_start(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}
