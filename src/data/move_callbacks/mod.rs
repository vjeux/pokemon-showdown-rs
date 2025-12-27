//! Move Callback Handlers
//!
//! This module exports all move callback implementations.
//! Each move with callbacks is in its own file.

use crate::battle::Battle;
use crate::event::EventResult;

// Common types
mod common;
pub use common::*;

// Individual move modules
pub mod acrobatics;
pub mod acupressure;
pub mod afteryou;
pub mod allyswitch;
pub mod aquaring;
pub mod aromatherapy;
pub mod assist;
pub mod assurance;
pub mod attract;
pub mod aurawheel;
pub mod auroraveil;
pub mod autotomize;
pub mod avalanche;
pub mod axekick;
pub mod banefulbunker;
pub mod barbbarrage;
pub mod batonpass;
pub mod beakblast;
pub mod beatup;
pub mod belch;
pub mod bellydrum;
pub mod bestow;
pub mod bide;
pub mod bleakwindstorm;
pub mod blizzard;
pub mod block;
pub mod boltbeak;
pub mod bounce;
pub mod brickbreak;
pub mod brine;
pub mod bugbite;
pub mod burningbulwark;
pub mod burnup;
pub mod camouflage;
pub mod captivate;
pub mod ceaselessedge;
pub mod celebrate;
pub mod charge;
pub mod chillyreception;
pub mod clangoroussoul;
pub mod clearsmog;
pub mod collisioncourse;
pub mod comeuppance;
pub mod conversion;
pub mod conversion2;
pub mod copycat;
pub mod coreenforcer;
pub mod corrosivegas;
pub mod counter;
pub mod courtchange;
pub mod covet;
pub mod craftyshield;
pub mod crushgrip;
pub mod curse;
pub mod darkvoid;
pub mod defog;
pub mod destinybond;
pub mod detect;
pub mod dig;
pub mod disable;
pub mod dive;
pub mod doodle;
pub mod doomdesire;
pub mod doubleshock;
pub mod dragoncheer;
pub mod dragonenergy;
pub mod dreameater;
pub mod echoedvoice;
pub mod electricterrain;
pub mod electrify;
pub mod electroball;
pub mod electrodrift;
pub mod electroshot;
pub mod embargo;
pub mod encore;
pub mod endeavor;
pub mod endure;
pub mod entrainment;
pub mod eruption;
pub mod expandingforce;
pub mod facade;
pub mod fairylock;
pub mod fakeout;
pub mod falseswipe;
pub mod fellstinger;
pub mod ficklebeam;
pub mod filletaway;
pub mod finalgambit;
pub mod firepledge;
pub mod firstimpression;
pub mod fishiousrend;
pub mod flail;
pub mod flameburst;
pub mod fling;
pub mod floralhealing;
pub mod flowershield;
pub mod fly;
pub mod flyingpress;
pub mod focusenergy;
pub mod focuspunch;
pub mod followme;
pub mod foresight;
pub mod forestscurse;
pub mod freezedry;
pub mod freezeshock;
pub mod freezyfrost;
pub mod frustration;
pub mod furycutter;
pub mod fusionbolt;
pub mod fusionflare;
pub mod futuresight;
pub mod gastroacid;
pub mod gearup;
pub mod geomancy;
pub mod glaiverush;
pub mod gmaxcannonade;
pub mod gmaxchistrike;
pub mod gmaxsnooze;
pub mod gmaxsteelsurge;
pub mod gmaxvinelash;
pub mod gmaxvolcalith;
pub mod gmaxwildfire;
pub mod grassknot;
pub mod grasspledge;
pub mod grassyglide;
pub mod grassyterrain;
pub mod gravapple;
pub mod gravity;
pub mod growth;
pub mod grudge;
pub mod guardianofalola;
pub mod guardsplit;
pub mod guardswap;
pub mod gyroball;
pub mod happyhour;
pub mod hardpress;
pub mod haze;
pub mod healbell;
pub mod healblock;
pub mod healingwish;
pub mod healpulse;
pub mod heartswap;
pub mod heatcrash;
pub mod heavyslam;
pub mod helpinghand;
pub mod hex;
pub mod hiddenpower;
pub mod highjumpkick;
pub mod holdback;
pub mod hurricane;
pub mod hyperspacefury;
pub mod iceball;
pub mod iceburn;
pub mod icespinner;
pub mod imprison;
pub mod incinerate;
pub mod infernalparade;
pub mod ingrain;
pub mod instruct;
pub mod iondeluge;
pub mod ivycudgel;
pub mod jawlock;
pub mod judgment;
pub mod jumpkick;
pub mod junglehealing;
pub mod kingsshield;
pub mod knockoff;
pub mod laserfocus;
pub mod lashout;
pub mod lastresort;
pub mod lastrespects;
pub mod leechseed;
pub mod lightscreen;
pub mod lightthatburnsthesky;
pub mod lockon;
pub mod lowkick;
pub mod luckychant;
pub mod lunarblessing;
pub mod lunardance;
pub mod magiccoat;
pub mod magicpowder;
pub mod magicroom;
pub mod magneticflux;
pub mod magnetrise;
pub mod magnitude;
pub mod matblock;
pub mod maxguard;
pub mod meanlook;
pub mod mefirst;
pub mod metalburst;
pub mod meteorbeam;
pub mod metronome;
pub mod mimic;
pub mod mindblown;
pub mod mindreader;
pub mod minimize;
pub mod miracleeye;
pub mod mirrorcoat;
pub mod mirrormove;
pub mod mist;
pub mod mistyexplosion;
pub mod mistyterrain;
pub mod moonlight;
pub mod morningsun;
pub mod mortalspin;
pub mod mudsport;
pub mod multiattack;
pub mod naturalgift;
pub mod naturepower;
pub mod naturesmadness;
pub mod nightmare;
pub mod noretreat;
pub mod obstruct;
pub mod octolock;
pub mod odorsleuth;
pub mod orderup;
pub mod painsplit;
pub mod partingshot;
pub mod payback;
pub mod perishsong;
pub mod phantomforce;
pub mod photongeyser;
pub mod pikapapow;
pub mod pluck;
pub mod polarflare;
pub mod pollenpuff;
pub mod poltergeist;
pub mod powder;
pub mod powershift;
pub mod powersplit;
pub mod powerswap;
pub mod powertrick;
pub mod powertrip;
pub mod present;
pub mod protect;
pub mod psyblade;
pub mod psychicfangs;
pub mod psychicterrain;
pub mod psychoshift;
pub mod psychup;
pub mod psywave;
pub mod punishment;
pub mod purify;
pub mod pursuit;
pub mod quash;
pub mod quickguard;
pub mod rage;
pub mod ragefist;
pub mod ragepowder;
pub mod ragingbull;
pub mod rapidspin;
pub mod razorwind;
pub mod recycle;
pub mod reflect;
pub mod reflecttype;
pub mod refresh;
pub mod relicsong;
pub mod rest;
pub mod retaliate;
pub mod r#return;
pub mod revelationdance;
pub mod revenge;
pub mod reversal;
pub mod revivalblessing;
pub mod risingvoltage;
pub mod roleplay;
pub mod rollout;
pub mod roost;
pub mod rototiller;
pub mod round;
pub mod ruination;
pub mod safeguard;
pub mod saltcure;
pub mod sandsearstorm;
pub mod sappyseed;
pub mod secretpower;
pub mod shadowforce;
pub mod shedtail;
pub mod shellsidearm;
pub mod shelltrap;
pub mod shoreup;
pub mod silktrap;
pub mod simplebeam;
pub mod sketch;
pub mod skillswap;
pub mod skullbash;
pub mod skyattack;
pub mod skydrop;
pub mod sleeptalk;
pub mod smackdown;
pub mod smellingsalts;
pub mod snatch;
pub mod snore;
pub mod soak;
pub mod solarbeam;
pub mod solarblade;
pub mod sparklingaria;
pub mod speedswap;
pub mod spiderweb;
pub mod spikes;
pub mod spikyshield;
pub mod spite;
pub mod spitup;
pub mod splash;
pub mod splinteredstormshards;
pub mod spotlight;
pub mod stealthrock;
pub mod steelbeam;
pub mod steelroller;
pub mod stickyweb;
pub mod stockpile;
pub mod stompingtantrum;
pub mod stoneaxe;
pub mod storedpower;
pub mod strengthsap;
pub mod struggle;
pub mod stuffcheeks;
pub mod substitute;
pub mod suckerpunch;
pub mod supercellslam;
pub mod superfang;
pub mod swallow;
pub mod switcheroo;
pub mod synchronoise;
pub mod synthesis;
pub mod syrupbomb;
pub mod tailwind;
pub mod takeheart;
pub mod tarshot;
pub mod taunt;
pub mod teatime;
pub mod technoblast;
pub mod telekinesis;
pub mod teleport;
pub mod temperflare;
pub mod terablast;
pub mod terastarstorm;
pub mod terrainpulse;
pub mod thief;
pub mod thousandarrows;
pub mod thousandwaves;
pub mod throatchop;
pub mod thunder;
pub mod thunderclap;
pub mod tidyup;
pub mod topsyturvy;
pub mod torment;
pub mod toxicspikes;
pub mod transform;
pub mod trick;
pub mod trickortreat;
pub mod trickroom;
pub mod tripleaxel;
pub mod triplekick;
pub mod trumpcard;
pub mod upperhand;
pub mod uproar;
pub mod veeveevolley;
pub mod venomdrench;
pub mod venoshock;
pub mod wakeupslap;
pub mod waterpledge;
pub mod watershuriken;
pub mod watersport;
pub mod waterspout;
pub mod weatherball;
pub mod wideguard;
pub mod wildboltstorm;
pub mod wish;
pub mod wonderroom;
pub mod worryseed;
pub mod wringout;
pub mod yawn;

// Dispatch functions
/// Dispatch basePowerCallback callbacks
pub fn dispatch_base_power_callback(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch beforeMoveCallback callbacks
pub fn dispatch_before_move_callback(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch beforeTurnCallback callbacks
pub fn dispatch_before_turn_callback(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch damageCallback callbacks
pub fn dispatch_damage_callback(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onAfterHit callbacks
pub fn dispatch_on_after_hit(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
    _target_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onAfterMove callbacks
pub fn dispatch_on_after_move(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onAfterMoveSecondarySelf callbacks
pub fn dispatch_on_after_move_secondary_self(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onAfterSubDamage callbacks
pub fn dispatch_on_after_sub_damage(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
    _target_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onBasePower callbacks
pub fn dispatch_on_base_power(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onDamage callbacks
pub fn dispatch_on_damage(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
    _target_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onDisableMove callbacks
pub fn dispatch_on_disable_move(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onEffectiveness callbacks
pub fn dispatch_on_effectiveness(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onHit callbacks
pub fn dispatch_on_hit(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
    _target_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onHitField callbacks
pub fn dispatch_on_hit_field(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onHitSide callbacks
pub fn dispatch_on_hit_side(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onModifyMove callbacks
pub fn dispatch_on_modify_move(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onModifyPriority callbacks
pub fn dispatch_on_modify_priority(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onModifyTarget callbacks
pub fn dispatch_on_modify_target(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onModifyType callbacks
pub fn dispatch_on_modify_type(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onMoveFail callbacks
pub fn dispatch_on_move_fail(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onPrepareHit callbacks
pub fn dispatch_on_prepare_hit(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onTry callbacks
pub fn dispatch_on_try(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onTryHit callbacks
pub fn dispatch_on_try_hit(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
    _target_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onTryImmunity callbacks
pub fn dispatch_on_try_immunity(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onTryMove callbacks
pub fn dispatch_on_try_move(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onUseMoveMessage callbacks
pub fn dispatch_on_use_move_message(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch priorityChargeCallback callbacks
pub fn dispatch_priority_charge_callback(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

// Condition dispatch functions
/// Dispatch condition durationCallback callbacks
pub fn dispatch_condition_duration_callback(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onAccuracy callbacks
pub fn dispatch_condition_on_accuracy(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onAfterMove callbacks
pub fn dispatch_condition_on_after_move(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onAllyTryHitSide callbacks
pub fn dispatch_condition_on_ally_try_hit_side(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onAnyBasePower callbacks
pub fn dispatch_condition_on_any_base_power(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onAnyDragOut callbacks
pub fn dispatch_condition_on_any_drag_out(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onAnyInvulnerability callbacks
pub fn dispatch_condition_on_any_invulnerability(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onAnyModifyDamage callbacks
pub fn dispatch_condition_on_any_modify_damage(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
    _target_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onAnyPrepareHit callbacks
pub fn dispatch_condition_on_any_prepare_hit(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onAnySetStatus callbacks
pub fn dispatch_condition_on_any_set_status(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onBasePower callbacks
pub fn dispatch_condition_on_base_power(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onBeforeMove callbacks
pub fn dispatch_condition_on_before_move(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onBeforeSwitchOut callbacks
pub fn dispatch_condition_on_before_switch_out(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onCopy callbacks
pub fn dispatch_condition_on_copy(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onDamage callbacks
pub fn dispatch_condition_on_damage(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
    _target_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onDamagingHit callbacks
pub fn dispatch_condition_on_damaging_hit(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onDisableMove callbacks
pub fn dispatch_condition_on_disable_move(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onDragOut callbacks
pub fn dispatch_condition_on_drag_out(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onEffectiveness callbacks
pub fn dispatch_condition_on_effectiveness(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onEnd callbacks
pub fn dispatch_condition_on_end(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onFaint callbacks
pub fn dispatch_condition_on_faint(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onFieldEnd callbacks
pub fn dispatch_condition_on_field_end(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onFieldRestart callbacks
pub fn dispatch_condition_on_field_restart(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onFieldStart callbacks
pub fn dispatch_condition_on_field_start(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onFoeBeforeMove callbacks
pub fn dispatch_condition_on_foe_before_move(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onFoeDisableMove callbacks
pub fn dispatch_condition_on_foe_disable_move(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onFoeRedirectTarget callbacks
pub fn dispatch_condition_on_foe_redirect_target(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onFoeTrapPokemon callbacks
pub fn dispatch_condition_on_foe_trap_pokemon(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onHit callbacks
pub fn dispatch_condition_on_hit(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
    _target_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onImmunity callbacks
pub fn dispatch_condition_on_immunity(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onInvulnerability callbacks
pub fn dispatch_condition_on_invulnerability(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onModifyAccuracy callbacks
pub fn dispatch_condition_on_modify_accuracy(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onModifyBoost callbacks
pub fn dispatch_condition_on_modify_boost(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onModifyCritRatio callbacks
pub fn dispatch_condition_on_modify_crit_ratio(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onModifyMove callbacks
pub fn dispatch_condition_on_modify_move(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onModifySpe callbacks
pub fn dispatch_condition_on_modify_spe(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onModifyType callbacks
pub fn dispatch_condition_on_modify_type(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onMoveAborted callbacks
pub fn dispatch_condition_on_move_aborted(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onNegateImmunity callbacks
pub fn dispatch_condition_on_negate_immunity(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onOverrideAction callbacks
pub fn dispatch_condition_on_override_action(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onRedirectTarget callbacks
pub fn dispatch_condition_on_redirect_target(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onResidual callbacks
pub fn dispatch_condition_on_residual(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onRestart callbacks
pub fn dispatch_condition_on_restart(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onSetStatus callbacks
pub fn dispatch_condition_on_set_status(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onSideEnd callbacks
pub fn dispatch_condition_on_side_end(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onSideRestart callbacks
pub fn dispatch_condition_on_side_restart(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onSideStart callbacks
pub fn dispatch_condition_on_side_start(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onSourceAccuracy callbacks
pub fn dispatch_condition_on_source_accuracy(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onSourceBasePower callbacks
pub fn dispatch_condition_on_source_base_power(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onSourceInvulnerability callbacks
pub fn dispatch_condition_on_source_invulnerability(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onSourceModifyDamage callbacks
pub fn dispatch_condition_on_source_modify_damage(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
    _target_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onStart callbacks
pub fn dispatch_condition_on_start(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onSwap callbacks
pub fn dispatch_condition_on_swap(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onSwitchIn callbacks
pub fn dispatch_condition_on_switch_in(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onTrapPokemon callbacks
pub fn dispatch_condition_on_trap_pokemon(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onTryAddVolatile callbacks
pub fn dispatch_condition_on_try_add_volatile(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onTryBoost callbacks
pub fn dispatch_condition_on_try_boost(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onTryHeal callbacks
pub fn dispatch_condition_on_try_heal(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onTryHit callbacks
pub fn dispatch_condition_on_try_hit(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
    _target_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onTryMove callbacks
pub fn dispatch_condition_on_try_move(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onTryPrimaryHit callbacks
pub fn dispatch_condition_on_try_primary_hit(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onType callbacks
pub fn dispatch_condition_on_type(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch condition onUpdate callbacks
pub fn dispatch_condition_on_update(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}
