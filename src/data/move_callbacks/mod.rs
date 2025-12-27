//! Move Callback Handlers
//!
//! This module exports all move callback implementations.
//! Each move with callbacks is in its own file.

use crate::battle::Battle;

// Common types
mod common;
pub use common::*;

// Individual move modules
pub mod acupressure;
pub mod afteryou;
pub mod allyswitch;
pub mod aquaring;
pub mod aromatherapy;
pub mod assist;
pub mod attract;
pub mod aurawheel;
pub mod auroraveil;
pub mod autotomize;
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
pub mod curse;
pub mod darkvoid;
pub mod defensecurl;
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
pub mod dreameater;
pub mod echoedvoice;
pub mod electricterrain;
pub mod electrify;
pub mod electrodrift;
pub mod electroshot;
pub mod embargo;
pub mod encore;
pub mod endeavor;
pub mod endure;
pub mod entrainment;
pub mod expandingforce;
pub mod facade;
pub mod fairylock;
pub mod fakeout;
pub mod falseswipe;
pub mod fellstinger;
pub mod ficklebeam;
pub mod filletaway;
pub mod firepledge;
pub mod firstimpression;
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
pub mod guardsplit;
pub mod guardswap;
pub mod happyhour;
pub mod haze;
pub mod healbell;
pub mod healblock;
pub mod healingwish;
pub mod healpulse;
pub mod heartswap;
pub mod heatcrash;
pub mod heavyslam;
pub mod helpinghand;
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
pub mod nightmare;
pub mod noretreat;
pub mod obstruct;
pub mod octolock;
pub mod odorsleuth;
pub mod orderup;
pub mod painsplit;
pub mod partingshot;
pub mod perishsong;
pub mod phantomforce;
pub mod photongeyser;
pub mod pluck;
pub mod polarflare;
pub mod pollenpuff;
pub mod poltergeist;
pub mod powder;
pub mod powershift;
pub mod powersplit;
pub mod powerswap;
pub mod powertrick;
pub mod present;
pub mod protect;
pub mod psyblade;
pub mod psychicfangs;
pub mod psychicterrain;
pub mod psychoshift;
pub mod psychup;
pub mod purify;
pub mod pursuit;
pub mod quash;
pub mod quickguard;
pub mod rage;
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
pub mod revelationdance;
pub mod revivalblessing;
pub mod roleplay;
pub mod rollout;
pub mod roost;
pub mod rototiller;
pub mod round;
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
pub mod stoneaxe;
pub mod strengthsap;
pub mod struggle;
pub mod stuffcheeks;
pub mod substitute;
pub mod suckerpunch;
pub mod supercellslam;
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
pub mod upperhand;
pub mod uproar;
pub mod venomdrench;
pub mod venoshock;
pub mod wakeupslap;
pub mod waterpledge;
pub mod watersport;
pub mod weatherball;
pub mod wideguard;
pub mod wildboltstorm;
pub mod wish;
pub mod wonderroom;
pub mod worryseed;
pub mod yawn;

// Dispatch functions
/// Dispatch onAfterHit callbacks
pub fn dispatch_on_after_hit(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
    _target_pos: (usize, usize),
) -> MoveHandlerResult {
    MoveHandlerResult::Undefined
}

/// Dispatch onAfterMove callbacks
pub fn dispatch_on_after_move(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> MoveHandlerResult {
    MoveHandlerResult::Undefined
}

/// Dispatch onAfterMoveSecondarySelf callbacks
pub fn dispatch_on_after_move_secondary_self(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> MoveHandlerResult {
    MoveHandlerResult::Undefined
}

/// Dispatch onAfterSubDamage callbacks
pub fn dispatch_on_after_sub_damage(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
    _target_pos: (usize, usize),
) -> MoveHandlerResult {
    MoveHandlerResult::Undefined
}

/// Dispatch onBasePower callbacks
pub fn dispatch_on_base_power(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> MoveHandlerResult {
    MoveHandlerResult::Undefined
}

/// Dispatch onDamage callbacks
pub fn dispatch_on_damage(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
    _target_pos: (usize, usize),
) -> MoveHandlerResult {
    MoveHandlerResult::Undefined
}

/// Dispatch onDisableMove callbacks
pub fn dispatch_on_disable_move(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> MoveHandlerResult {
    MoveHandlerResult::Undefined
}

/// Dispatch onEffectiveness callbacks
pub fn dispatch_on_effectiveness(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> MoveHandlerResult {
    MoveHandlerResult::Undefined
}

/// Dispatch onHit callbacks
pub fn dispatch_on_hit(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
    _target_pos: (usize, usize),
) -> MoveHandlerResult {
    MoveHandlerResult::Undefined
}

/// Dispatch onHitField callbacks
pub fn dispatch_on_hit_field(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> MoveHandlerResult {
    MoveHandlerResult::Undefined
}

/// Dispatch onHitSide callbacks
pub fn dispatch_on_hit_side(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> MoveHandlerResult {
    MoveHandlerResult::Undefined
}

/// Dispatch onModifyMove callbacks
pub fn dispatch_on_modify_move(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> MoveHandlerResult {
    MoveHandlerResult::Undefined
}

/// Dispatch onModifyPriority callbacks
pub fn dispatch_on_modify_priority(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> MoveHandlerResult {
    MoveHandlerResult::Undefined
}

/// Dispatch onModifyTarget callbacks
pub fn dispatch_on_modify_target(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> MoveHandlerResult {
    MoveHandlerResult::Undefined
}

/// Dispatch onModifyType callbacks
pub fn dispatch_on_modify_type(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> MoveHandlerResult {
    MoveHandlerResult::Undefined
}

/// Dispatch onMoveFail callbacks
pub fn dispatch_on_move_fail(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> MoveHandlerResult {
    MoveHandlerResult::Undefined
}

/// Dispatch onPrepareHit callbacks
pub fn dispatch_on_prepare_hit(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> MoveHandlerResult {
    MoveHandlerResult::Undefined
}

/// Dispatch onTry callbacks
pub fn dispatch_on_try(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> MoveHandlerResult {
    MoveHandlerResult::Undefined
}

/// Dispatch onTryHit callbacks
pub fn dispatch_on_try_hit(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
    _target_pos: (usize, usize),
) -> MoveHandlerResult {
    MoveHandlerResult::Undefined
}

/// Dispatch onTryImmunity callbacks
pub fn dispatch_on_try_immunity(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> MoveHandlerResult {
    MoveHandlerResult::Undefined
}

/// Dispatch onTryMove callbacks
pub fn dispatch_on_try_move(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> MoveHandlerResult {
    MoveHandlerResult::Undefined
}

/// Dispatch onUseMoveMessage callbacks
pub fn dispatch_on_use_move_message(
    _battle: &mut Battle,
    _move_id: &str,
    _source_pos: (usize, usize),
) -> MoveHandlerResult {
    MoveHandlerResult::Undefined
}
