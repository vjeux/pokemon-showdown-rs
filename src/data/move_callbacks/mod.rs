//! Move Callback Handlers
//!
//! This module exports all move callback implementations.
//! Each move with callbacks is in its own file.

use crate::battle::Battle;
use crate::battle_actions::ActiveMove;
use crate::event::EventResult;

// Individual move modules
pub mod acrobatics;
pub mod acupressure;
pub mod afteryou;
pub mod alluringvoice;
pub mod allyswitch;
pub mod anchorshot;
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
pub mod burningjealousy;
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
pub mod defensecurl;
pub mod defog;
pub mod destinybond;
pub mod detect;
pub mod dig;
pub mod direclaw;
pub mod disable;
pub mod dive;
pub mod doodle;
pub mod doomdesire;
pub mod doubleshock;
pub mod dragoncheer;
pub mod dragonenergy;
pub mod dreameater;
pub mod echoedvoice;
pub mod eeriespell;
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
pub mod gmaxbefuddle;
pub mod gmaxcannonade;
pub mod gmaxcentiferno;
pub mod gmaxchistrike;
pub mod gmaxcuddle;
pub mod gmaxdepletion;
pub mod gmaxfinale;
pub mod gmaxfoamburst;
pub mod gmaxgoldrush;
pub mod gmaxmalodor;
pub mod gmaxmeltdown;
pub mod gmaxreplenish;
pub mod gmaxsandblast;
pub mod gmaxsmite;
pub mod gmaxsnooze;
pub mod gmaxsteelsurge;
pub mod gmaxstonesurge;
pub mod gmaxstunshock;
pub mod gmaxtartness;
pub mod gmaxsweetness;
pub mod gmaxterror;
pub mod gmaxvinelash;
pub mod gmaxvolcalith;
pub mod gmaxvoltcrash;
pub mod gmaxwildfire;
pub mod gmaxwindrage;
pub mod genesissupernova;
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
pub mod maxairstream;
pub mod maxdarkness;
pub mod maxflare;
pub mod maxflutterby;
pub mod maxgeyser;
pub mod maxhailstorm;
pub mod maxknuckle;
pub mod maxlightning;
pub mod maxmindstorm;
pub mod maxooze;
pub mod maxovergrowth;
pub mod maxphantasm;
pub mod maxquake;
pub mod maxrockfall;
pub mod maxsteelspike;
pub mod maxstarfall;
pub mod maxstrike;
pub mod maxwyrmwind;
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
pub mod spiritshackle;
pub mod speedswap;
pub mod spiderweb;
pub mod spikes;
pub mod spikyshield;
pub mod spite;
pub mod spitup;
pub mod sparklyswirl;
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
pub mod triattack;
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
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "acrobatics" => acrobatics::base_power_callback(battle, pokemon_pos, target_pos),
        "assurance" => assurance::base_power_callback(battle, pokemon_pos, target_pos),
        "avalanche" => avalanche::base_power_callback(battle, pokemon_pos, target_pos),
        "beatup" => beatup::base_power_callback(battle, pokemon_pos, target_pos),
        "boltbeak" => boltbeak::base_power_callback(battle, pokemon_pos, target_pos),
        "crushgrip" => crushgrip::base_power_callback(battle, pokemon_pos, target_pos),
        "dragonenergy" => dragonenergy::base_power_callback(battle, pokemon_pos, target_pos),
        "echoedvoice" => echoedvoice::base_power_callback(battle, pokemon_pos, target_pos),
        "electroball" => electroball::base_power_callback(battle, pokemon_pos, target_pos),
        "eruption" => eruption::base_power_callback(battle, pokemon_pos, target_pos),
        "firepledge" => firepledge::base_power_callback(battle, pokemon_pos, target_pos),
        "fishiousrend" => fishiousrend::base_power_callback(battle, pokemon_pos, target_pos),
        "flail" => flail::base_power_callback(battle, pokemon_pos, target_pos),
        "frustration" => frustration::base_power_callback(battle, pokemon_pos, target_pos),
        "furycutter" => furycutter::base_power_callback(battle, pokemon_pos, target_pos),
        "grassknot" => grassknot::base_power_callback(battle, pokemon_pos, target_pos),
        "grasspledge" => grasspledge::base_power_callback(battle, pokemon_pos, target_pos),
        "gyroball" => gyroball::base_power_callback(battle, pokemon_pos, target_pos),
        "hardpress" => hardpress::base_power_callback(battle, pokemon_pos, target_pos),
        "heatcrash" => heatcrash::base_power_callback(battle, pokemon_pos, target_pos),
        "heavyslam" => heavyslam::base_power_callback(battle, pokemon_pos, target_pos),
        "hex" => hex::base_power_callback(battle, pokemon_pos, target_pos),
        "iceball" => iceball::base_power_callback(battle, pokemon_pos, target_pos),
        "infernalparade" => infernalparade::base_power_callback(battle, pokemon_pos, target_pos),
        "lastrespects" => lastrespects::base_power_callback(battle, pokemon_pos, target_pos),
        "lowkick" => lowkick::base_power_callback(battle, pokemon_pos, target_pos),
        "payback" => payback::base_power_callback(battle, pokemon_pos, target_pos),
        "pikapapow" => pikapapow::base_power_callback(battle, pokemon_pos, target_pos),
        "powertrip" => powertrip::base_power_callback(battle, pokemon_pos, target_pos),
        "punishment" => punishment::base_power_callback(battle, pokemon_pos, target_pos),
        "pursuit" => pursuit::base_power_callback(battle, pokemon_pos, target_pos),
        "ragefist" => ragefist::base_power_callback(battle, pokemon_pos, target_pos),
        "return" => r#return::base_power_callback(battle, pokemon_pos, target_pos),
        "revenge" => revenge::base_power_callback(battle, pokemon_pos, target_pos),
        "reversal" => reversal::base_power_callback(battle, pokemon_pos, target_pos),
        "risingvoltage" => risingvoltage::base_power_callback(battle, pokemon_pos, target_pos),
        "rollout" => rollout::base_power_callback(battle, pokemon_pos, target_pos),
        "round" => round::base_power_callback(battle, pokemon_pos, target_pos),
        "smellingsalts" => smellingsalts::base_power_callback(battle, pokemon_pos, target_pos),
        "spitup" => spitup::base_power_callback(battle, pokemon_pos, target_pos),
        "stompingtantrum" => stompingtantrum::base_power_callback(battle, pokemon_pos, target_pos),
        "storedpower" => storedpower::base_power_callback(battle, pokemon_pos, target_pos),
        "temperflare" => temperflare::base_power_callback(battle, pokemon_pos, target_pos),
        "terablast" => terablast::base_power_callback(battle, pokemon_pos, target_pos),
        "tripleaxel" => tripleaxel::base_power_callback(battle, pokemon_pos, target_pos),
        "triplekick" => triplekick::base_power_callback(battle, pokemon_pos, target_pos),
        "trumpcard" => trumpcard::base_power_callback(battle, pokemon_pos, target_pos),
        "veeveevolley" => veeveevolley::base_power_callback(battle, pokemon_pos, target_pos),
        "wakeupslap" => wakeupslap::base_power_callback(battle, pokemon_pos, target_pos),
        "waterpledge" => waterpledge::base_power_callback(battle, pokemon_pos, target_pos),
        "watershuriken" => watershuriken::base_power_callback(battle, pokemon_pos, target_pos),
        "waterspout" => waterspout::base_power_callback(battle, pokemon_pos, target_pos),
        "wringout" => wringout::base_power_callback(battle, pokemon_pos, target_pos),
        _ => EventResult::Continue,
    }
}
/// Dispatch beforeMoveCallback callbacks
pub fn dispatch_before_move_callback(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    pokemon_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "bide" => bide::before_move_callback(battle, pokemon_pos),
        "focuspunch" => focuspunch::before_move_callback(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}
//   beforeTurnCallback()
//   beforeTurnCallback(pokemon)
//   beforeTurnCallback(pokemon, target)
//   beforeTurnCallback(source, target)

/// Dispatch beforeTurnCallback callbacks
pub fn dispatch_before_turn_callback(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    pokemon_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "counter" => counter::before_turn_callback(battle, pokemon_pos),
        "mirrorcoat" => mirrorcoat::before_turn_callback(battle, pokemon_pos),
        "pursuit" => pursuit::before_turn_callback(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}
//   damageCallback(pokemon)
//   damageCallback(pokemon, target)

/// Dispatch damageCallback callbacks
pub fn dispatch_damage_callback(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "comeuppance" => comeuppance::damage_callback(battle, pokemon_pos, target_pos),
        "counter" => counter::damage_callback(battle, pokemon_pos, target_pos),
        "endeavor" => endeavor::damage_callback(battle, pokemon_pos, target_pos),
        "finalgambit" => finalgambit::damage_callback(battle, pokemon_pos, target_pos),
        "guardianofalola" => guardianofalola::damage_callback(battle, pokemon_pos, target_pos),
        "metalburst" => metalburst::damage_callback(battle, pokemon_pos, target_pos),
        "mirrorcoat" => mirrorcoat::damage_callback(battle, pokemon_pos, target_pos),
        "naturesmadness" => naturesmadness::damage_callback(battle, pokemon_pos, target_pos),
        "psywave" => psywave::damage_callback(battle, pokemon_pos, target_pos),
        "ruination" => ruination::damage_callback(battle, pokemon_pos, target_pos),
        "superfang" => superfang::damage_callback(battle, pokemon_pos, target_pos),
        _ => EventResult::Continue,
    }
}
//   onAfterHit()
//   onAfterHit(target, pokemon)
//   onAfterHit(target, pokemon, move)
//   onAfterHit(target, source)
//   onAfterHit(target, source, move)

/// Dispatch onAfterHit callbacks
pub fn dispatch_on_after_hit(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    target_pos: (usize, usize),  // JavaScript: onAfterHit(target, source) - target first
    source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "ceaselessedge" => ceaselessedge::on_after_hit(battle, target_pos, source_pos),
        "covet" => covet::on_after_hit(battle, target_pos, source_pos),
        "icespinner" => icespinner::on_after_hit(battle, target_pos, source_pos),
        "knockoff" => knockoff::on_after_hit(battle, target_pos, source_pos),
        "mortalspin" => mortalspin::on_after_hit(battle, target_pos, source_pos),
        "rapidspin" => rapidspin::on_after_hit(battle, target_pos, source_pos),
        "stoneaxe" => stoneaxe::on_after_hit(battle, target_pos, source_pos),
        "thief" => thief::on_after_hit(battle, target_pos, source_pos),
        _ => EventResult::Continue,
    }
}
//   onAfterMove()
//   onAfterMove(pokemon)
//   onAfterMove(pokemon, attacker, move)
//   onAfterMove(pokemon, target, move)
//   onAfterMove(source)
//   onAfterMove(source, target, move)

/// Dispatch onAfterMove callbacks
pub fn dispatch_on_after_move(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "beakblast" => beakblast::on_after_move(battle, source_pos, target_pos),
        "iceball" => iceball::on_after_move(battle, source_pos, target_pos),
        "mindblown" => mindblown::on_after_move(battle, source_pos, target_pos),
        "rollout" => rollout::on_after_move(battle, source_pos, target_pos),
        "sparklingaria" => sparklingaria::on_after_move(battle, source_pos, target_pos),
        "spitup" => spitup::on_after_move(battle, source_pos, target_pos),
        "steelbeam" => steelbeam::on_after_move(battle, source_pos, target_pos),
        _ => EventResult::Continue,
    }
}
//   onAfterMoveSecondarySelf()
//   onAfterMoveSecondarySelf(pokemon, source, move)
//   onAfterMoveSecondarySelf(pokemon, target, move)
//   onAfterMoveSecondarySelf(source)
//   onAfterMoveSecondarySelf(source, target, move)
//   onAfterMoveSecondarySelf(target, source, move)

/// Dispatch onAfterMoveSecondarySelf callbacks
pub fn dispatch_on_after_move_secondary_self(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "fellstinger" => {
            fellstinger::on_after_move_secondary_self(battle, pokemon_pos, target_pos, active_move)
        }
        "orderup" => {
            orderup::on_after_move_secondary_self(battle, pokemon_pos, target_pos, active_move)
        }
        "polarflare" => {
            polarflare::on_after_move_secondary_self(battle, pokemon_pos, target_pos, active_move)
        }
        "relicsong" => {
            relicsong::on_after_move_secondary_self(battle, pokemon_pos, target_pos, active_move)
        }
        _ => EventResult::Continue,
    }
}
//   onAfterSubDamage()
//   onAfterSubDamage(damage, target)
//   onAfterSubDamage(damage, target, pokemon)
//   onAfterSubDamage(damage, target, pokemon, move)
//   onAfterSubDamage(damage, target, source)
//   onAfterSubDamage(damage, target, source, effect)
//   onAfterSubDamage(damage, target, source, move)

/// Dispatch onAfterSubDamage callbacks
pub fn dispatch_on_after_sub_damage(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    pokemon_pos: (usize, usize),
    damage: i32,
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "ceaselessedge" => ceaselessedge::on_after_sub_damage(
            battle,
            damage,
            target_pos,
            Some(pokemon_pos),
            active_move,
        ),
        "coreenforcer" => coreenforcer::on_after_sub_damage(battle, damage, target_pos),
        "flameburst" => {
            flameburst::on_after_sub_damage(battle, damage, target_pos, Some(pokemon_pos), active_move)
        }
        "gmaxsnooze" => gmaxsnooze::on_after_sub_damage(battle, damage, target_pos),
        "icespinner" => {
            icespinner::on_after_sub_damage(battle, damage, target_pos, Some(pokemon_pos))
        }
        "mortalspin" => {
            if let Some(target) = target_pos {
                mortalspin::on_after_sub_damage(battle, damage, target, pokemon_pos)
            } else {
                EventResult::Continue
            }
        }
        "rapidspin" => {
            rapidspin::on_after_sub_damage(battle, damage, target_pos, pokemon_pos, active_move)
        }
        "shellsidearm" => {
            if let Some(target) = target_pos {
                shellsidearm::on_after_sub_damage(battle, damage, target, pokemon_pos, active_move)
            } else {
                EventResult::Continue
            }
        }
        "splinteredstormshards" => splinteredstormshards::on_after_sub_damage(battle),
        "steelroller" => steelroller::on_after_sub_damage(battle),
        "stoneaxe" => {
            stoneaxe::on_after_sub_damage(battle, damage, target_pos, Some(pokemon_pos), active_move)
        }
        _ => EventResult::Continue,
    }
}
//   onBasePower()
//   onBasePower(basePower)
//   onBasePower(basePower, attacker, defender, move)
//   onBasePower(basePower, pokemon)
//   onBasePower(basePower, pokemon, target)
//   onBasePower(basePower, pokemon, target, move)
//   onBasePower(basePower, source)
//   onBasePower(basePower, source, target)
//   onBasePower(basePower, source, target, move)
//   onBasePower(basePower, user, target)
//   onBasePower(basePower, user, target, move)
//   onBasePower(relayVar, source, target, move)

/// Dispatch onBasePower callbacks
/// JavaScript: onBasePower(basePower, attacker, defender, move) - ATTACKER=SOURCE, DEFENDER=TARGET
pub fn dispatch_on_base_power(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    base_power: i32,
    target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "barbbarrage" => barbbarrage::on_base_power(battle, base_power, target_pos, source_pos),
        "brine" => brine::on_base_power(battle, base_power, target_pos, source_pos),
        "collisioncourse" => {
            collisioncourse::on_base_power(battle, base_power, target_pos, source_pos)
        }
        "electrodrift" => electrodrift::on_base_power(battle, base_power, target_pos, source_pos),
        "expandingforce" => {
            expandingforce::on_base_power(battle, base_power, target_pos, source_pos)
        }
        "facade" => facade::on_base_power(battle, base_power, target_pos, source_pos),
        "ficklebeam" => ficklebeam::on_base_power(battle, base_power, target_pos, source_pos),
        "fusionbolt" => fusionbolt::on_base_power(battle, base_power, target_pos, source_pos),
        "fusionflare" => fusionflare::on_base_power(battle, base_power, target_pos, source_pos),
        "gravapple" => gravapple::on_base_power(battle, base_power, target_pos, source_pos),
        "knockoff" => knockoff::on_base_power(battle, base_power, target_pos, source_pos),
        "lashout" => lashout::on_base_power(battle, base_power, target_pos, source_pos),
        "mistyexplosion" => {
            mistyexplosion::on_base_power(battle, base_power, target_pos, source_pos)
        }
        "psyblade" => psyblade::on_base_power(battle, base_power, target_pos, source_pos),
        "retaliate" => retaliate::on_base_power(battle, base_power, target_pos, source_pos),
        "solarbeam" => solarbeam::on_base_power(battle, base_power, target_pos, source_pos),
        "solarblade" => solarblade::on_base_power(battle, base_power, target_pos, source_pos),
        "venoshock" => venoshock::on_base_power(battle, base_power, target_pos, source_pos),
        _ => EventResult::Continue,
    }
}
//   onDamage()
//   onDamage(damage, pokemon, source, effect)
//   onDamage(damage, target, source, effect)
//   onDamage(damage, target, source, move)
//   onDamage(item, pokemon)

/// Dispatch onDamage callbacks
pub fn dispatch_on_damage(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    damage: i32,
    target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
    effect_id: Option<&str>,
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "falseswipe" => falseswipe::on_damage(battle, damage, target_pos, source_pos, effect_id),
        "holdback" => holdback::on_damage(battle, damage, target_pos, source_pos, effect_id),
        _ => EventResult::Continue,
    }
}
//   onDisableMove(pokemon)
//   onDisableMove(target)

/// Dispatch onDisableMove callbacks
// TODO: Verify move parameter type matches JavaScript's ActiveMove usage
pub fn dispatch_on_disable_move(
    battle: &mut Battle,
    move_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match move_id {
        "belch" => belch::on_disable_move(battle, pokemon_pos),
        "stuffcheeks" => stuffcheeks::on_disable_move(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}
//   onEffectiveness()
//   onEffectiveness(typeMod, target, type)
//   onEffectiveness(typeMod, target, type, move)

/// Dispatch onEffectiveness callbacks
pub fn dispatch_on_effectiveness(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    type_mod: i32,
    target_type: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // Type mod and target_type parameters added to support moves that need them
    // Currently passing placeholder/default values from callers until event system is updated
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "flyingpress" => flyingpress::on_effectiveness(battle, type_mod, target_type),
        "freezedry" => freezedry::on_effectiveness(battle, type_mod, target_type),
        "thousandarrows" => thousandarrows::on_effectiveness(battle, type_mod, target_type, Some(pokemon_pos)),
        _ => EventResult::Continue,
    }
}

/// Dispatch self.onHit callbacks
/// JavaScript: self.onHit(source) - SELF-TARGETING, source is the move user
/// In JavaScript, self callbacks are in the self: { } object
/// They target the move user, not the move target
pub fn dispatch_self_on_hit(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
    source_effect: Option<&crate::battle::Effect>,
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "batonpass" => batonpass::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "burnup" => burnup::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "doubleshock" => doubleshock::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "gmaxbefuddle" => gmaxbefuddle::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "gmaxcannonade" => gmaxcannonade::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "gmaxcentiferno" => gmaxcentiferno::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "gmaxchistrike" => gmaxchistrike::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "gmaxcuddle" => gmaxcuddle::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "gmaxdepletion" => gmaxdepletion::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "gmaxfinale" => gmaxfinale::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "gmaxfoamburst" => gmaxfoamburst::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "gmaxgoldrush" => gmaxgoldrush::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "gmaxmalodor" => gmaxmalodor::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "gmaxmeltdown" => gmaxmeltdown::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "gmaxreplenish" => gmaxreplenish::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "gmaxsandblast" => gmaxsandblast::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "gmaxsmite" => gmaxsmite::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "gmaxsteelsurge" => gmaxsteelsurge::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "gmaxstonesurge" => gmaxstonesurge::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "gmaxstunshock" => gmaxstunshock::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "gmaxsweetness" => gmaxsweetness::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "gmaxtartness" => gmaxtartness::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "gmaxterror" => gmaxterror::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "gmaxvinelash" => gmaxvinelash::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "gmaxvolcalith" => gmaxvolcalith::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "gmaxvoltcrash" => gmaxvoltcrash::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "gmaxwildfire" => gmaxwildfire::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "gmaxwindrage" => gmaxwindrage::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "maxairstream" => maxairstream::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "maxdarkness" => maxdarkness::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "maxflare" => maxflare::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "maxflutterby" => maxflutterby::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "maxgeyser" => maxgeyser::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "maxhailstorm" => maxhailstorm::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "maxknuckle" => maxknuckle::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "maxlightning" => maxlightning::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "maxmindstorm" => maxmindstorm::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "maxooze" => maxooze::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "maxovergrowth" => maxovergrowth::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "maxphantasm" => maxphantasm::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "maxquake" => maxquake::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "maxrockfall" => maxrockfall::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "maxstarfall" => maxstarfall::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "maxsteelspike" => maxsteelspike::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "maxstrike" => maxstrike::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "maxwyrmwind" => maxwyrmwind::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "psychoshift" => psychoshift::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "shedtail" => shedtail::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        "sparklyswirl" => sparklyswirl::self_callbacks::on_hit(battle, target_pos, source_pos, source_effect),
        _ => EventResult::Continue,
    }
}
//   onHit()
//   onHit(pokemon)
//   onHit(pokemon, qwerty, move)
//   onHit(pokemon, source)
//   onHit(pokemon, source, move)
//   onHit(source)
//   onHit(source, target, effect)
//   onHit(t, source, m)
//   onHit(target)
//   onHit(target, pokemon)
//   onHit(target, pokemon, move)
//   onHit(target, source)
//   onHit(target, source, effect)
//   onHit(target, source, m)
//   onHit(target, source, move)

/// Check if a move has a secondary.onHit callback
/// This is used to determine if we should call the onHit dispatch for secondary effects
/// In JavaScript, the secondary object can have its own onHit callback
// TODO: Verify move parameter type matches JavaScript's ActiveMove usage
pub fn has_secondary_on_hit(move_id: &str) -> bool {
    // List of moves that have secondary.onHit callbacks
    // These are moves where the onHit is defined inside the secondary object
    // and we have the callback implemented in Rust
    match move_id {
        "direclaw" => true,
        "triattack" => true,
        "barbbarrage" => true,
        "psychicnoise" => true,
        "relicsong" => true,
        "shellsidearm" => true,
        "spiritshackle" => true,
        "anchorshot" => true,
        "clangoroussoulblaze" => true,
        "syrupbomb" => true,
        "throatchop" => true,
        "axekick" => true,
        "doubleironbash" => true,
        "freezingglare" => true,
        "genesissupernova" => true,
        "upperhand" => true,
        "alluringvoice" => true,
        "burningjealousy" => true,
        "eeriespell" => true,
        _ => false,
    }
}

/// Dispatch onHit callbacks
/// JavaScript: onHit(target, source, move) - TARGET FIRST
pub fn dispatch_on_hit(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "acupressure" => acupressure::on_hit(battle, target_pos, source_pos),
        "afteryou" => afteryou::on_hit(battle, target_pos, source_pos),
        "alluringvoice" => alluringvoice::on_hit(battle, target_pos, source_pos),
        "allyswitch" => allyswitch::on_hit(battle, target_pos, source_pos),
        "anchorshot" => anchorshot::on_hit(battle, target_pos, source_pos),
        "aromatherapy" => aromatherapy::on_hit(battle, target_pos, source_pos),
        "assist" => assist::on_hit(battle, target_pos, source_pos),
        "autotomize" => autotomize::on_hit(battle, target_pos, source_pos),
        "banefulbunker" => banefulbunker::on_hit(battle, target_pos, source_pos),
        "batonpass" => batonpass::on_hit(battle, target_pos, source_pos),
        "bellydrum" => bellydrum::on_hit(battle, target_pos, source_pos),
        "bestow" => bestow::on_hit(battle, target_pos, source_pos),
        "block" => block::on_hit(battle, target_pos, source_pos),
        "bugbite" => bugbite::on_hit(battle, target_pos, source_pos),
        "burningbulwark" => burningbulwark::on_hit(battle, target_pos, source_pos),
        "burningjealousy" => burningjealousy::on_hit(battle, target_pos, source_pos),
        "camouflage" => camouflage::on_hit(battle, target_pos, source_pos),
        "clangoroussoul" => clangoroussoul::on_hit(battle, target_pos, source_pos),
        "clearsmog" => clearsmog::on_hit(battle, target_pos, source_pos),
        "conversion" => conversion::on_hit(battle, target_pos, source_pos),
        "conversion2" => conversion2::on_hit(battle, target_pos, source_pos),
        "copycat" => copycat::on_hit(battle, target_pos, source_pos),
        "coreenforcer" => coreenforcer::on_hit(battle, target_pos, source_pos),
        "corrosivegas" => corrosivegas::on_hit(battle, target_pos, source_pos),
        "curse" => curse::on_hit(battle, target_pos, source_pos),
        "defog" => defog::on_hit(battle, target_pos, source_pos),
        "detect" => detect::on_hit(battle, target_pos, source_pos),
        "direclaw" => direclaw::on_hit(battle, target_pos, source_pos),
        "doodle" => doodle::on_hit(battle, target_pos, source_pos),
        "eeriespell" => eeriespell::on_hit(battle, target_pos, source_pos),
        "endure" => endure::on_hit(battle, target_pos, source_pos),
        "entrainment" => entrainment::on_hit(battle, target_pos, source_pos),
        "filletaway" => filletaway::on_hit(battle, target_pos, source_pos),
        "flameburst" => flameburst::on_hit(battle, target_pos, source_pos),
        "floralhealing" => floralhealing::on_hit(battle, target_pos, source_pos),
        "forestscurse" => forestscurse::on_hit(battle, target_pos, source_pos),
        "freezyfrost" => freezyfrost::on_hit(battle, target_pos, source_pos),
        "genesissupernova" => genesissupernova::on_hit(battle, target_pos, source_pos),
        "gmaxsnooze" => gmaxsnooze::on_hit(battle, target_pos, source_pos),
        "guardsplit" => guardsplit::on_hit(battle, target_pos, source_pos),
        "guardswap" => guardswap::on_hit(battle, target_pos, source_pos),
        "healbell" => healbell::on_hit(battle, target_pos, source_pos),
        "healpulse" => healpulse::on_hit(battle, target_pos, source_pos),
        "heartswap" => heartswap::on_hit(battle, target_pos, source_pos),
        "incinerate" => incinerate::on_hit(battle, target_pos, source_pos),
        "instruct" => instruct::on_hit(battle, target_pos, source_pos),
        "jawlock" => jawlock::on_hit(battle, target_pos, source_pos),
        "junglehealing" => junglehealing::on_hit(battle, target_pos, source_pos),
        "kingsshield" => kingsshield::on_hit(battle, target_pos, source_pos),
        "lockon" => lockon::on_hit(battle, target_pos, source_pos),
        "lunarblessing" => lunarblessing::on_hit(battle, target_pos, source_pos),
        "magicpowder" => magicpowder::on_hit(battle, target_pos, source_pos),
        "maxguard" => maxguard::on_hit(battle, target_pos, source_pos),
        "meanlook" => meanlook::on_hit(battle, target_pos, source_pos),
        "metronome" => metronome::on_hit(battle, target_pos, source_pos),
        "mimic" => mimic::on_hit(battle, target_pos, source_pos),
        "mindreader" => mindreader::on_hit(battle, target_pos, source_pos),
        "moonlight" => moonlight::on_hit(battle, target_pos, source_pos),
        "morningsun" => morningsun::on_hit(battle, target_pos, source_pos),
        "obstruct" => obstruct::on_hit(battle, target_pos, source_pos),
        "painsplit" => painsplit::on_hit(battle, target_pos, source_pos),
        "partingshot" => partingshot::on_hit(battle, target_pos, source_pos),
        "pluck" => pluck::on_hit(battle, target_pos, source_pos),
        "polarflare" => polarflare::on_hit(battle, target_pos, source_pos),
        "pollenpuff" => pollenpuff::on_hit(battle, target_pos, source_pos),
        "powersplit" => powersplit::on_hit(battle, target_pos, source_pos),
        "powerswap" => powerswap::on_hit(battle, target_pos, source_pos),
        "protect" => protect::on_hit(battle, target_pos, source_pos),
        "psychup" => psychup::on_hit(battle, target_pos, source_pos),
        "purify" => purify::on_hit(battle, target_pos, source_pos),
        "quash" => quash::on_hit(battle, target_pos, source_pos),
        "recycle" => recycle::on_hit(battle, target_pos, source_pos),
        "reflecttype" => reflecttype::on_hit(battle, target_pos, source_pos),
        "refresh" => refresh::on_hit(battle, target_pos, source_pos),
        "relicsong" => relicsong::on_hit(battle, target_pos, source_pos),
        "rest" => rest::on_hit(battle, target_pos, source_pos),
        "roleplay" => roleplay::on_hit(battle, target_pos, source_pos),
        "sappyseed" => sappyseed::on_hit(battle, target_pos, source_pos),
        "shedtail" => shedtail::on_hit(battle, target_pos, source_pos),
        "shellsidearm" => shellsidearm::on_hit(battle, target_pos, source_pos),
        "shoreup" => shoreup::on_hit(battle, target_pos, source_pos),
        "silktrap" => silktrap::on_hit(battle, target_pos, source_pos),
        "simplebeam" => simplebeam::on_hit(battle, target_pos, source_pos),
        "sketch" => sketch::on_hit(battle, target_pos, source_pos),
        "skillswap" => skillswap::on_hit(battle, target_pos, source_pos),
        "skydrop" => skydrop::on_hit(battle, target_pos, source_pos),
        "sleeptalk" => sleeptalk::on_hit(battle, target_pos, source_pos),
        "smellingsalts" => smellingsalts::on_hit(battle, target_pos, source_pos),
        "soak" => soak::on_hit(battle, target_pos, source_pos),
        "speedswap" => speedswap::on_hit(battle, target_pos, source_pos),
        "spiritshackle" => spiritshackle::on_hit(battle, target_pos, source_pos),
        "spiderweb" => spiderweb::on_hit(battle, target_pos, source_pos),
        "spikyshield" => spikyshield::on_hit(battle, target_pos, source_pos),
        "spite" => spite::on_hit(battle, target_pos, source_pos),
        "splinteredstormshards" => splinteredstormshards::on_hit(battle, target_pos, source_pos),
        "steelroller" => steelroller::on_hit(battle, target_pos, source_pos),
        "strengthsap" => strengthsap::on_hit(battle, target_pos, source_pos),
        "stuffcheeks" => stuffcheeks::on_hit(battle, target_pos, source_pos),
        "substitute" => substitute::on_hit(battle, target_pos, source_pos),
        "swallow" => swallow::on_hit(battle, target_pos, source_pos),
        "switcheroo" => switcheroo::on_hit(battle, target_pos, source_pos),
        "synthesis" => synthesis::on_hit(battle, target_pos, source_pos),
        "takeheart" => takeheart::on_hit(battle, target_pos, source_pos),
        "thousandwaves" => thousandwaves::on_hit(battle, target_pos, source_pos),
        "throatchop" => throatchop::on_hit(battle, target_pos, source_pos),
        "tidyup" => tidyup::on_hit(battle, target_pos, source_pos),
        "topsyturvy" => topsyturvy::on_hit(battle, target_pos, source_pos),
        "transform" => transform::on_hit(battle, target_pos, source_pos),
        "triattack" => triattack::on_hit(battle, target_pos, source_pos),
        "trick" => trick::on_hit(battle, target_pos, source_pos),
        "trickortreat" => trickortreat::on_hit(battle, target_pos, source_pos),
        "venomdrench" => venomdrench::on_hit(battle, target_pos, source_pos),
        "wakeupslap" => wakeupslap::on_hit(battle, target_pos, source_pos),
        "worryseed" => worryseed::on_hit(battle, target_pos, source_pos),
        _ => EventResult::Continue,
    }
}
//   onHitField()
//   onHitField(t, source, move)
//   onHitField(target, source)
//   onHitField(target, source, move)

/// Dispatch onHitField callbacks
pub fn dispatch_on_hit_field(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "courtchange" => courtchange::on_hit_field(battle, target_pos, Some(pokemon_pos)),
        "flowershield" => flowershield::on_hit_field(battle, Some(pokemon_pos), active_move),
        "haze" => haze::on_hit_field(battle),
        "perishsong" => perishsong::on_hit_field(battle, target_pos, Some(pokemon_pos), active_move),
        "rototiller" => rototiller::on_hit_field(battle, target_pos, Some(pokemon_pos)),
        "teatime" => teatime::on_hit_field(battle, target_pos, Some(pokemon_pos), active_move),
        _ => EventResult::Continue,
    }
}
//   onHitSide(side, source)
//   onHitSide(side, source, move)

/// Dispatch onHitSide callbacks
pub fn dispatch_on_hit_side(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    pokemon_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "gearup" => gearup::on_hit_side(battle, Some(pokemon_pos), active_move),
        "magneticflux" => magneticflux::on_hit_side(battle, Some(pokemon_pos), active_move),
        "quickguard" => quickguard::on_hit_side(battle, Some(pokemon_pos)),
        "wideguard" => wideguard::on_hit_side(battle, Some(pokemon_pos)),
        _ => EventResult::Continue,
    }
}
//   onModifyMove()
//   onModifyMove(move)
//   onModifyMove(move, attacker)
//   onModifyMove(move, attacker, defender)
//   onModifyMove(move, pokemon)
//   onModifyMove(move, pokemon, target)
//   onModifyMove(move, source)
//   onModifyMove(move, source, target)

/// Dispatch onModifyMove callbacks
pub fn dispatch_on_modify_move(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "beatup" => beatup::on_modify_move(battle, pokemon_pos, target_pos),
        "bleakwindstorm" => bleakwindstorm::on_modify_move(battle, pokemon_pos, target_pos),
        "blizzard" => blizzard::on_modify_move(battle, pokemon_pos, target_pos),
        "curse" => curse::on_modify_move(battle, pokemon_pos, target_pos),
        "expandingforce" => expandingforce::on_modify_move(battle, pokemon_pos, target_pos),
        "firepledge" => firepledge::on_modify_move(battle, pokemon_pos, target_pos),
        "grasspledge" => grasspledge::on_modify_move(battle, pokemon_pos, target_pos),
        "growth" => growth::on_modify_move(battle, pokemon_pos, target_pos),
        "hurricane" => hurricane::on_modify_move(battle, pokemon_pos, target_pos),
        "iceball" => iceball::on_modify_move(battle, pokemon_pos, target_pos),
        "lightthatburnsthesky" => {
            lightthatburnsthesky::on_modify_move(battle, pokemon_pos, target_pos)
        }
        "magnitude" => magnitude::on_modify_move(battle, pokemon_pos, target_pos),
        "photongeyser" => photongeyser::on_modify_move(battle, pokemon_pos, target_pos),
        "present" => present::on_modify_move(battle, pokemon_pos, target_pos),
        "pursuit" => pursuit::on_modify_move(battle, pokemon_pos, target_pos),
        "rollout" => rollout::on_modify_move(battle, pokemon_pos, target_pos),
        "sandsearstorm" => sandsearstorm::on_modify_move(battle, pokemon_pos, target_pos),
        "secretpower" => secretpower::on_modify_move(battle, pokemon_pos, target_pos),
        "shellsidearm" => shellsidearm::on_modify_move(battle, pokemon_pos, target_pos),
        "skydrop" => skydrop::on_modify_move(battle, pokemon_pos, target_pos),
        "struggle" => struggle::on_modify_move(battle, pokemon_pos, target_pos),
        "terablast" => terablast::on_modify_move(battle, pokemon_pos, target_pos),
        "terastarstorm" => terastarstorm::on_modify_move(battle, pokemon_pos, target_pos),
        "terrainpulse" => terrainpulse::on_modify_move(battle, pokemon_pos, target_pos),
        "thunder" => thunder::on_modify_move(battle, pokemon_pos, target_pos),
        "waterpledge" => waterpledge::on_modify_move(battle, pokemon_pos, target_pos),
        "weatherball" => weatherball::on_modify_move(battle, pokemon_pos, target_pos),
        "wildboltstorm" => wildboltstorm::on_modify_move(battle, pokemon_pos, target_pos),
        _ => EventResult::Continue,
    }
}
//   onModifyPriority(priority, pokemon)
//   onModifyPriority(priority, pokemon, target)
//   onModifyPriority(priority, pokemon, target, move)
//   onModifyPriority(priority, source)
//   onModifyPriority(priority, source, target, move)
//   onModifyPriority(relayVar, source, target, move)

/// Dispatch onModifyPriority callbacks
pub fn dispatch_on_modify_priority(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    pokemon_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "grassyglide" => grassyglide::on_modify_priority(battle, Some(pokemon_pos), None, active_move),
        _ => EventResult::Continue,
    }
}
//   onModifyTarget(targetRelayVar, source, target, move)

/// Dispatch onModifyTarget callbacks
pub fn dispatch_on_modify_target(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    pokemon_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "comeuppance" => comeuppance::on_modify_target(battle, Some(pokemon_pos), None, active_move),
        "metalburst" => metalburst::on_modify_target(battle, Some(pokemon_pos), None, active_move),
        _ => EventResult::Continue,
    }
}
//   onModifyType(move)
//   onModifyType(move, pokemon)
//   onModifyType(move, pokemon, target)

/// Dispatch onModifyType callbacks
pub fn dispatch_on_modify_type(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "aurawheel" => aurawheel::on_modify_type(battle, active_move, pokemon_pos),
        "hiddenpower" => hiddenpower::on_modify_type(battle, active_move, pokemon_pos),
        "ivycudgel" => ivycudgel::on_modify_type(battle, active_move, pokemon_pos),
        "judgment" => judgment::on_modify_type(battle, active_move, pokemon_pos),
        "multiattack" => multiattack::on_modify_type(battle, active_move, pokemon_pos),
        "naturalgift" => naturalgift::on_modify_type(battle, active_move, pokemon_pos),
        "ragingbull" => ragingbull::on_modify_type(battle, active_move, pokemon_pos),
        "revelationdance" => revelationdance::on_modify_type(battle, pokemon_pos),
        "technoblast" => technoblast::on_modify_type(battle, active_move, pokemon_pos),
        "terablast" => terablast::on_modify_type(battle, active_move, pokemon_pos, None),
        "terastarstorm" => terastarstorm::on_modify_type(battle, active_move, pokemon_pos),
        "terrainpulse" => terrainpulse::on_modify_type(battle, active_move, pokemon_pos),
        "weatherball" => weatherball::on_modify_type(battle, active_move, pokemon_pos),
        _ => EventResult::Continue,
    }
}
//   onMoveFail()
//   onMoveFail(target, source)
//   onMoveFail(target, source, move)

/// Dispatch onMoveFail callbacks
pub fn dispatch_on_move_fail(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    pokemon_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "axekick" => axekick::on_move_fail(battle, None, Some(pokemon_pos), active_move),
        "highjumpkick" => highjumpkick::on_move_fail(battle, None, Some(pokemon_pos), active_move),
        "jumpkick" => jumpkick::on_move_fail(battle, None, Some(pokemon_pos), active_move),
        "skydrop" => skydrop::on_move_fail(battle, None, Some(pokemon_pos)),
        "supercellslam" => supercellslam::on_move_fail(battle, None, Some(pokemon_pos), active_move),
        _ => EventResult::Continue,
    }
}
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
    active_move: Option<&ActiveMove>,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "allyswitch" => allyswitch::on_prepare_hit(battle, pokemon_pos, target_pos),
        "banefulbunker" => banefulbunker::on_prepare_hit(battle, pokemon_pos, target_pos),
        "burningbulwark" => burningbulwark::on_prepare_hit(battle, pokemon_pos, target_pos),
        "destinybond" => destinybond::on_prepare_hit(battle, pokemon_pos, target_pos),
        "detect" => detect::on_prepare_hit(battle, pokemon_pos, target_pos),
        "endure" => endure::on_prepare_hit(battle, pokemon_pos, target_pos),
        "firepledge" => firepledge::on_prepare_hit(battle, pokemon_pos, target_pos),
        "fling" => fling::on_prepare_hit(battle, pokemon_pos, target_pos),
        "grasspledge" => grasspledge::on_prepare_hit(battle, pokemon_pos, target_pos),
        "ivycudgel" => ivycudgel::on_prepare_hit(battle, pokemon_pos, target_pos),
        "kingsshield" => kingsshield::on_prepare_hit(battle, pokemon_pos, target_pos),
        "maxguard" => maxguard::on_prepare_hit(battle, pokemon_pos, target_pos),
        "naturalgift" => naturalgift::on_prepare_hit(battle, pokemon_pos, target_pos),
        "obstruct" => obstruct::on_prepare_hit(battle, pokemon_pos, target_pos),
        "protect" => protect::on_prepare_hit(battle, pokemon_pos, target_pos),
        "shellsidearm" => shellsidearm::on_prepare_hit(battle, pokemon_pos, target_pos),
        "silktrap" => silktrap::on_prepare_hit(battle, pokemon_pos, target_pos),
        "spikyshield" => spikyshield::on_prepare_hit(battle, pokemon_pos, target_pos),
        "terablast" => terablast::on_prepare_hit(battle, pokemon_pos, target_pos),
        "waterpledge" => waterpledge::on_prepare_hit(battle, pokemon_pos, target_pos),
        _ => EventResult::Continue,
    }
}
//   onTry()
//   onTry(pokemon)
//   onTry(pokemon, target)
//   onTry(pokemon, target, move)
//   onTry(source)
//   onTry(source, target)
//   onTry(source, target, move)

/// Dispatch onTry callbacks
/// Dispatch onTry callbacks
/// JavaScript: onTry(target, source) - TARGET FIRST
pub fn dispatch_on_try(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "aurawheel" => aurawheel::on_try(battle, target_pos, source_pos),
        "auroraveil" => auroraveil::on_try(battle, target_pos, source_pos),
        "clangoroussoul" => clangoroussoul::on_try(battle, target_pos, source_pos),
        "comeuppance" => comeuppance::on_try(battle, target_pos, source_pos),
        "counter" => counter::on_try(battle, target_pos, source_pos),
        "craftyshield" => craftyshield::on_try(battle, target_pos, source_pos),
        "darkvoid" => darkvoid::on_try(battle, target_pos, source_pos),
        "doomdesire" => doomdesire::on_try(battle, target_pos, source_pos),
        "fakeout" => fakeout::on_try(battle, target_pos, source_pos),
        "filletaway" => filletaway::on_try(battle, target_pos, source_pos),
        "firstimpression" => firstimpression::on_try(battle, target_pos, source_pos),
        "followme" => followme::on_try(battle, target_pos, source_pos),
        "futuresight" => futuresight::on_try(battle, target_pos, source_pos),
        "hyperspacefury" => hyperspacefury::on_try(battle, target_pos, source_pos),
        "lastresort" => lastresort::on_try(battle, target_pos, source_pos),
        "magnetrise" => magnetrise::on_try(battle, target_pos, source_pos),
        "matblock" => matblock::on_try(battle, target_pos, source_pos),
        "metalburst" => metalburst::on_try(battle, target_pos, source_pos),
        "mirrorcoat" => mirrorcoat::on_try(battle, target_pos, source_pos),
        "noretreat" => noretreat::on_try(battle, target_pos, source_pos),
        "poltergeist" => poltergeist::on_try(battle, target_pos, source_pos),
        "quickguard" => quickguard::on_try(battle, target_pos, source_pos),
        "ragepowder" => ragepowder::on_try(battle, target_pos, source_pos),
        "rest" => rest::on_try(battle, target_pos, source_pos),
        "round" => round::on_try(battle, target_pos, source_pos),
        "skydrop" => skydrop::on_try(battle, target_pos, source_pos),
        "sleeptalk" => sleeptalk::on_try(battle, target_pos, source_pos),
        "snore" => snore::on_try(battle, target_pos, source_pos),
        "spitup" => spitup::on_try(battle, target_pos, source_pos),
        "splash" => splash::on_try(battle, target_pos, source_pos),
        "steelroller" => steelroller::on_try(battle, target_pos, source_pos),
        "stockpile" => stockpile::on_try(battle, target_pos, source_pos),
        "stuffcheeks" => stuffcheeks::on_try(battle, target_pos, source_pos),
        "suckerpunch" => suckerpunch::on_try(battle, target_pos, source_pos),
        "swallow" => swallow::on_try(battle, target_pos, source_pos),
        "telekinesis" => telekinesis::on_try(battle, target_pos, source_pos),
        "teleport" => teleport::on_try(battle, target_pos, source_pos),
        "thunderclap" => thunderclap::on_try(battle, target_pos, source_pos),
        "upperhand" => upperhand::on_try(battle, target_pos, source_pos),
        "wideguard" => wideguard::on_try(battle, target_pos, source_pos),
        _ => EventResult::Continue,
    }
}
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
/// JavaScript: onTryHit(target, source, move) - TARGET FIRST
pub fn dispatch_on_try_hit(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    target_pos: (usize, usize),
    source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "autotomize" => autotomize::on_try_hit(battle, target_pos, source_pos),
        "brickbreak" => brickbreak::on_try_hit(battle, target_pos, source_pos),
        "celebrate" => celebrate::on_try_hit(battle, target_pos, source_pos),
        "clangoroussoul" => clangoroussoul::on_try_hit(battle, target_pos, source_pos),
        "curse" => curse::on_try_hit(battle, target_pos, source_pos),
        "disable" => disable::on_try_hit(battle, target_pos, source_pos),
        "electrify" => electrify::on_try_hit(battle, target_pos, source_pos),
        "entrainment" => entrainment::on_try_hit(battle, target_pos, source_pos),
        "filletaway" => filletaway::on_try_hit(battle, target_pos, source_pos),
        "foresight" => foresight::on_try_hit(battle, target_pos, source_pos),
        "gastroacid" => gastroacid::on_try_hit(battle, target_pos, source_pos),
        "grassknot" => grassknot::on_try_hit(battle, target_pos, source_pos),
        "happyhour" => happyhour::on_try_hit(battle, target_pos, source_pos),
        "healingwish" => healingwish::on_try_hit(battle, target_pos, source_pos),
        "heatcrash" => heatcrash::on_try_hit(battle, target_pos, source_pos),
        "heavyslam" => heavyslam::on_try_hit(battle, target_pos, source_pos),
        "helpinghand" => helpinghand::on_try_hit(battle, target_pos, source_pos),
        "lockon" => lockon::on_try_hit(battle, target_pos, source_pos),
        "lowkick" => lowkick::on_try_hit(battle, target_pos, source_pos),
        "lunardance" => lunardance::on_try_hit(battle, target_pos, source_pos),
        "mefirst" => mefirst::on_try_hit(battle, target_pos, source_pos),
        "mindreader" => mindreader::on_try_hit(battle, target_pos, source_pos),
        "miracleeye" => miracleeye::on_try_hit(battle, target_pos, source_pos),
        "mirrormove" => mirrormove::on_try_hit(battle, target_pos, source_pos),
        "naturepower" => naturepower::on_try_hit(battle, target_pos, source_pos),
        "odorsleuth" => odorsleuth::on_try_hit(battle, target_pos, source_pos),
        "pollenpuff" => pollenpuff::on_try_hit(battle, target_pos, source_pos),
        "poltergeist" => poltergeist::on_try_hit(battle, target_pos, source_pos),
        "psychicfangs" => psychicfangs::on_try_hit(battle, target_pos, source_pos),
        "psychoshift" => psychoshift::on_try_hit(battle, target_pos, source_pos),
        "pursuit" => pursuit::on_try_hit(battle, target_pos, source_pos),
        "ragingbull" => ragingbull::on_try_hit(battle, target_pos, source_pos),
        "revivalblessing" => revivalblessing::on_try_hit(battle, target_pos, source_pos),
        "roleplay" => roleplay::on_try_hit(battle, target_pos, source_pos),
        "shedtail" => shedtail::on_try_hit(battle, target_pos, source_pos),
        "simplebeam" => simplebeam::on_try_hit(battle, target_pos, source_pos),
        "skillswap" => skillswap::on_try_hit(battle, target_pos, source_pos),
        "skydrop" => skydrop::on_try_hit(battle, target_pos, source_pos),
        "splash" => splash::on_try_hit(battle, target_pos, source_pos),
        "spotlight" => spotlight::on_try_hit(battle, target_pos, source_pos),
        "substitute" => substitute::on_try_hit(battle, target_pos, source_pos),
        "uproar" => uproar::on_try_hit(battle, target_pos, source_pos),
        "worryseed" => worryseed::on_try_hit(battle, target_pos, source_pos),
        "yawn" => yawn::on_try_hit(battle, target_pos, source_pos),
        _ => EventResult::Continue,
    }
}
//   onTryImmunity(pokemon, source)
//   onTryImmunity(target)
//   onTryImmunity(target, pokemon)
//   onTryImmunity(target, source)

/// Dispatch onTryImmunity callbacks
pub fn dispatch_on_try_immunity(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    pokemon_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "attract" => attract::on_try_immunity(battle, Some(pokemon_pos), source_pos),
        "captivate" => captivate::on_try_immunity(battle, Some(pokemon_pos), source_pos),
        "dreameater" => dreameater::on_try_immunity(battle, Some(pokemon_pos)),
        "endeavor" => endeavor::on_try_immunity(battle, Some(pokemon_pos), source_pos),
        "leechseed" => leechseed::on_try_immunity(battle, Some(pokemon_pos)),
        "octolock" => octolock::on_try_immunity(battle, Some(pokemon_pos)),
        "switcheroo" => switcheroo::on_try_immunity(battle, Some(pokemon_pos)),
        "synchronoise" => synchronoise::on_try_immunity(battle, Some(pokemon_pos), source_pos),
        "trick" => trick::on_try_immunity(battle, Some(pokemon_pos)),
        "worryseed" => worryseed::on_try_immunity(battle, Some(pokemon_pos)),
        _ => EventResult::Continue,
    }
}
//   onTryMove()
//   onTryMove(attacker, defender, move)
//   onTryMove(pokemon)
//   onTryMove(pokemon, target, move)
//   onTryMove(source, target)
//   onTryMove(source, target, move)
//   onTryMove(target, source, move)

/// Dispatch onTryMove callbacks
pub fn dispatch_on_try_move(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "bounce" => bounce::on_try_move(battle, source_pos, target_pos),
        "burnup" => burnup::on_try_move(battle, source_pos, target_pos),
        "dig" => dig::on_try_move(battle, source_pos, target_pos),
        "dive" => dive::on_try_move(battle, source_pos, target_pos),
        "doubleshock" => doubleshock::on_try_move(battle, source_pos, target_pos),
        "echoedvoice" => echoedvoice::on_try_move(battle, source_pos, target_pos),
        "electroshot" => electroshot::on_try_move(battle, source_pos, target_pos),
        "fly" => fly::on_try_move(battle, source_pos, target_pos),
        "freezeshock" => freezeshock::on_try_move(battle, source_pos, target_pos),
        "geomancy" => geomancy::on_try_move(battle, source_pos, target_pos),
        "iceburn" => iceburn::on_try_move(battle, source_pos, target_pos),
        "meteorbeam" => meteorbeam::on_try_move(battle, source_pos, target_pos),
        "phantomforce" => phantomforce::on_try_move(battle, source_pos, target_pos),
        "pollenpuff" => pollenpuff::on_try_move(battle, source_pos, target_pos),
        "razorwind" => razorwind::on_try_move(battle, source_pos, target_pos),
        "shadowforce" => shadowforce::on_try_move(battle, source_pos, target_pos),
        "shelltrap" => shelltrap::on_try_move(battle, source_pos, target_pos),
        "skullbash" => skullbash::on_try_move(battle, source_pos, target_pos),
        "skyattack" => skyattack::on_try_move(battle, source_pos, target_pos),
        "solarbeam" => solarbeam::on_try_move(battle, source_pos, target_pos),
        "solarblade" => solarblade::on_try_move(battle, source_pos, target_pos),
        _ => EventResult::Continue,
    }
}
//   onUseMoveMessage(pokemon, target, move)

/// Dispatch onUseMoveMessage callbacks
pub fn dispatch_on_use_move_message(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    pokemon_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "magnitude" => magnitude::on_use_move_message(battle, pokemon_pos, None, active_move),
        _ => EventResult::Continue,
    }
}
//   priorityChargeCallback()
//   priorityChargeCallback(pokemon)
//   priorityChargeCallback(source)

/// Dispatch priorityChargeCallback callbacks
pub fn dispatch_priority_charge_callback(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    pokemon_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "beakblast" => beakblast::priority_charge_callback(battle, pokemon_pos),
        "chillyreception" => chillyreception::priority_charge_callback(battle, pokemon_pos),
        "focuspunch" => focuspunch::priority_charge_callback(battle, pokemon_pos),
        "shelltrap" => shelltrap::priority_charge_callback(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Check if a move has a beforeTurnCallback
pub fn has_before_turn_callback(active_move: Option<&ActiveMove>) -> bool {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    matches!(move_id, "counter" | "mirrorcoat" | "pursuit")
}

/// Check if a move has a priorityChargeCallback
pub fn has_priority_charge_callback(active_move: Option<&ActiveMove>) -> bool {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    matches!(move_id, "beakblast" | "chillyreception" | "focuspunch" | "shelltrap")
}

/// Check if a move has a beforeMoveCallback
pub fn has_before_move_callback(active_move: Option<&ActiveMove>) -> bool {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    matches!(move_id, "bide" | "focuspunch")
}

// Condition dispatch functions
/// Dispatch condition durationCallback callbacks
/// Takes condition_id directly to support fallback from condition_callbacks
pub fn dispatch_condition_duration_callback(
    battle: &mut Battle,
    condition_id: &str,
    source_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "auroraveil" => {
            auroraveil::condition::duration_callback(battle, None, Some(source_pos), Some(condition_id))
        }
        "electricterrain" => electricterrain::condition::duration_callback(
            battle,
            None,
            Some(source_pos),
            Some(condition_id),
        ),
        "grassyterrain" => grassyterrain::condition::duration_callback(
            battle,
            None,
            Some(source_pos),
            Some(condition_id),
        ),
        "gravity" => {
            gravity::condition::duration_callback(battle, None, Some(source_pos), Some(condition_id))
        }
        "healblock" => {
            healblock::condition::duration_callback(battle, None, Some(source_pos), Some(condition_id))
        }
        "lightscreen" => {
            lightscreen::condition::duration_callback(battle, None, Some(source_pos), Some(condition_id))
        }
        "magicroom" => {
            magicroom::condition::duration_callback(battle, None, Some(source_pos), Some(condition_id))
        }
        "mistyterrain" => mistyterrain::condition::duration_callback(
            battle,
            None,
            Some(source_pos),
            Some(condition_id),
        ),
        "psychicterrain" => psychicterrain::condition::duration_callback(
            battle,
            None,
            Some(source_pos),
            Some(condition_id),
        ),
        "reflect" => {
            reflect::condition::duration_callback(battle, None, Some(source_pos), Some(condition_id))
        }
        "safeguard" => {
            safeguard::condition::duration_callback(battle, None, Some(source_pos), Some(condition_id))
        }
        "tailwind" => {
            tailwind::condition::duration_callback(battle, None, Some(source_pos), Some(condition_id))
        }
        "trickroom" => {
            trickroom::condition::duration_callback(battle, None, Some(source_pos), Some(condition_id))
        }
        "wonderroom" => {
            wonderroom::condition::duration_callback(battle, None, Some(source_pos), Some(condition_id))
        }
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onAccuracy callbacks
pub fn dispatch_condition_on_accuracy(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    accuracy: i32,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "glaiverush" => {
            glaiverush::condition::on_accuracy(battle, accuracy, target_pos, source_pos, active_move)
        }
        "minimize" => {
            minimize::condition::on_accuracy(battle, accuracy, target_pos, source_pos, active_move)
        }
        "telekinesis" => {
            telekinesis::condition::on_accuracy(battle, accuracy, target_pos, source_pos, active_move)
        }
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onAfterMove callbacks
pub fn dispatch_condition_on_after_move(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "charge" => charge::condition::on_after_move(battle, source_pos, target_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onAllyTryHitSide callbacks
pub fn dispatch_condition_on_ally_try_hit_side(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "magiccoat" => {
            magiccoat::condition::on_ally_try_hit_side(battle, None, Some(source_pos), active_move)
        }
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onAnyBasePower callbacks
pub fn dispatch_condition_on_any_base_power(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "skydrop" => {
            skydrop::condition::on_any_base_power(battle, 0, None, Some(source_pos), active_move)
        }
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onAnyDragOut callbacks
pub fn dispatch_condition_on_any_drag_out(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "skydrop" => skydrop::condition::on_any_drag_out(battle, source_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onAnyInvulnerability callbacks
pub fn dispatch_condition_on_any_invulnerability(
    battle: &mut Battle,
    condition_id: &str,
    target_pos: (usize, usize),
    source_pos: (usize, usize),
    attacking_active_move: Option<&ActiveMove>,
) -> EventResult {
    let attacking_move_id = attacking_active_move.map(|m| m.id.as_str()).unwrap_or("");
    eprintln!("[DISPATCH_COND_ANY_INVULN] condition_id='{}', attacking_move_id='{}', target_pos={:?}, source_pos={:?}",
        condition_id, attacking_move_id, target_pos, source_pos);
    let result = match condition_id {
        "skydrop" => {
            eprintln!("[DISPATCH_COND_ANY_INVULN] Matched skydrop, calling callback with attacking_move_id={}", attacking_move_id);
            skydrop::condition::on_any_invulnerability(battle, Some(target_pos), Some(source_pos), attacking_active_move)
        }
        _ => {
            eprintln!("[DISPATCH_COND_ANY_INVULN] No match, returning Continue");
            EventResult::Continue
        }
    };
    eprintln!("[DISPATCH_COND_ANY_INVULN] Returning {:?}", result);
    result
}

/// Dispatch condition onAnyModifyDamage callbacks
pub fn dispatch_condition_on_any_modify_damage(
    battle: &mut Battle,
    condition_id: &str,
    source_pos: (usize, usize),
    target_pos: (usize, usize),
    active_move: Option<&ActiveMove>,
) -> EventResult {
    match condition_id {
        "auroraveil" => auroraveil::condition::on_any_modify_damage(
            battle,
            0,
            Some(source_pos),
            Some(target_pos),
            active_move,
        ),
        "lightscreen" => lightscreen::condition::on_any_modify_damage(
            battle,
            0,
            Some(source_pos),
            Some(target_pos),
            active_move,
        ),
        "reflect" => reflect::condition::on_any_modify_damage(
            battle,
            0,
            Some(source_pos),
            Some(target_pos),
            active_move,
        ),
        // Item-embedded conditions
        "metronome" => crate::data::item_callbacks::metronome::condition::on_modify_damage(
            battle,
            0,
            Some(source_pos),
            Some(target_pos),
            active_move,
        ),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onAnyPrepareHit callbacks
pub fn dispatch_condition_on_any_prepare_hit(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "snatch" => snatch::condition::on_any_prepare_hit(battle, Some(source_pos), None, active_move),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onAnySetStatus callbacks
pub fn dispatch_condition_on_any_set_status(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "uproar" => uproar::condition::on_any_set_status(battle, None, source_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onBasePower callbacks
pub fn dispatch_condition_on_base_power(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "charge" => charge::condition::on_base_power(battle, 0, source_pos, target_pos),
        "electricterrain" => {
            electricterrain::condition::on_base_power(battle, 0, source_pos, target_pos)
        }
        "grassyterrain" => {
            grassyterrain::condition::on_base_power(battle, 0, source_pos, target_pos)
        }
        "helpinghand" => helpinghand::condition::on_base_power(battle, 0, source_pos, target_pos),
        "mefirst" => mefirst::condition::on_base_power(battle, 0, source_pos, target_pos),
        "mistyterrain" => mistyterrain::condition::on_base_power(battle, 0, source_pos, target_pos),
        "mudsport" => mudsport::condition::on_base_power(battle, 0, source_pos, target_pos),
        "psychicterrain" => {
            psychicterrain::condition::on_base_power(battle, 0, source_pos, target_pos)
        }
        "watersport" => watersport::condition::on_base_power(battle, 0, source_pos, target_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onBeforeMove callbacks
/// Takes condition_id directly to support fallback from condition_callbacks
pub fn dispatch_condition_on_before_move(
    battle: &mut Battle,
    condition_id: &str,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "attract" => attract::condition::on_before_move(battle, source_pos, None, active_move),
        "bide" => bide::condition::on_before_move(battle, source_pos, None, active_move),
        "chillyreception" => {
            chillyreception::condition::on_before_move(battle, Some(source_pos), None, active_move)
        }
        "destinybond" => destinybond::condition::on_before_move(battle, source_pos, None, active_move),
        "disable" => disable::condition::on_before_move(battle, active_move),
        "glaiverush" => glaiverush::condition::on_before_move(battle, source_pos),
        "gravity" => gravity::condition::on_before_move(battle, source_pos, None, active_move),
        "grudge" => grudge::condition::on_before_move(battle, source_pos),
        "healblock" => healblock::condition::on_before_move(battle, source_pos, None, active_move),
        "rage" => rage::condition::on_before_move(battle, source_pos),
        "taunt" => taunt::condition::on_before_move(battle, source_pos, active_move),
        "throatchop" => throatchop::condition::on_before_move(battle, source_pos, None, active_move),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onBeforeSwitchOut callbacks
/// Takes condition_id directly to support fallback from condition_callbacks
pub fn dispatch_condition_on_before_switch_out(
    battle: &mut Battle,
    condition_id: &str,
    source_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "pursuit" => pursuit::condition::on_before_switch_out(battle, source_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onCopy callbacks
pub fn dispatch_condition_on_copy(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "gastroacid" => gastroacid::condition::on_copy(battle, source_pos),
        "powershift" => powershift::condition::on_copy(battle, source_pos),
        "powertrick" => powertrick::condition::on_copy(battle, source_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onDamage callbacks
pub fn dispatch_condition_on_damage(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
    target_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "bide" => bide::condition::on_damage(battle, 0, target_pos, Some(source_pos), None),
        "endure" => endure::condition::on_damage(battle, 0, target_pos, Some(source_pos), None),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onDamagingHit callbacks
pub fn dispatch_condition_on_damaging_hit(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "counter" => {
            counter::condition::on_damaging_hit(battle, 0, None, Some(source_pos), active_move)
        }
        "mirrorcoat" => {
            mirrorcoat::condition::on_damaging_hit(battle, 0, None, Some(source_pos), active_move)
        }
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onDisableMove callbacks
/// Takes condition_id directly to support fallback from condition_callbacks
pub fn dispatch_condition_on_disable_move(
    battle: &mut Battle,
    condition_id: &str,
    source_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "disable" => disable::condition::on_disable_move(battle, source_pos),
        "encore" => encore::condition::on_disable_move(battle, source_pos),
        "gravity" => gravity::condition::on_disable_move(battle, source_pos),
        "healblock" => healblock::condition::on_disable_move(battle, source_pos),
        "taunt" => taunt::condition::on_disable_move(battle, source_pos),
        "throatchop" => throatchop::condition::on_disable_move(battle, source_pos),
        "torment" => torment::condition::on_disable_move(battle, source_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onDragOut callbacks
pub fn dispatch_condition_on_drag_out(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "ingrain" => ingrain::condition::on_drag_out(battle, source_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onEffectiveness callbacks
pub fn dispatch_condition_on_effectiveness(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    type_mod: i32,
    target_type: &str,
    source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "tarshot" => tarshot::condition::on_effectiveness(battle, type_mod, target_type, Some(source_pos)),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onEnd callbacks
/// Takes condition_id directly to support fallback from condition_callbacks
pub fn dispatch_condition_on_end(
    battle: &mut Battle,
    condition_id: &str,
    source_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "attract" => attract::condition::on_end(battle, source_pos),
        "bide" => bide::condition::on_end(battle, source_pos),
        "charge" => charge::condition::on_end(battle, source_pos),
        "disable" => disable::condition::on_end(battle, source_pos),
        "embargo" => embargo::condition::on_end(battle, source_pos),
        "encore" => encore::condition::on_end(battle, Some(source_pos)),
        "healblock" => healblock::condition::on_end(battle, source_pos),
        "laserfocus" => laserfocus::condition::on_end(battle, source_pos),
        "magnetrise" => magnetrise::condition::on_end(battle, Some(source_pos)),
        "perishsong" => perishsong::condition::on_end(battle, Some(source_pos)),
        "powershift" => powershift::condition::on_end(battle, source_pos),
        "powertrick" => powertrick::condition::on_end(battle, source_pos),
        "saltcure" => saltcure::condition::on_end(battle, source_pos),
        "stockpile" => stockpile::condition::on_end(battle, Some(source_pos)),
        "substitute" => substitute::condition::on_end(battle, Some(source_pos)),
        "syrupbomb" => syrupbomb::condition::on_end(battle, source_pos),
        "taunt" => taunt::condition::on_end(battle, Some(source_pos)),
        "telekinesis" => telekinesis::condition::on_end(battle, Some(source_pos)),
        "throatchop" => throatchop::condition::on_end(battle, Some(source_pos)),
        "torment" => torment::condition::on_end(battle, source_pos),
        "uproar" => uproar::condition::on_end(battle, Some(source_pos)),
        "wish" => wish::condition::on_end(battle, Some(source_pos)),
        "yawn" => yawn::condition::on_end(battle, Some(source_pos)),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onFaint callbacks (with target, source, effect)
/// Takes condition_id directly to support fallback from condition_callbacks
pub fn dispatch_condition_on_faint(
    battle: &mut Battle,
    condition_id: &str,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    effect_id: Option<&str>,
) -> EventResult {
    match condition_id {
        "destinybond" => destinybond::condition::on_faint(battle, target_pos, source_pos, effect_id),
        "grudge" => grudge::condition::on_faint(battle, target_pos, source_pos, effect_id),
        "skydrop" => skydrop::condition::on_faint(battle, target_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onFieldEnd callbacks
/// Takes condition_id directly to support fallback from condition_callbacks
pub fn dispatch_condition_on_field_end(
    battle: &mut Battle,
    condition_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "electricterrain" => electricterrain::condition::on_field_end(battle),
        "grassyterrain" => grassyterrain::condition::on_field_end(battle),
        "gravity" => gravity::condition::on_field_end(battle),
        "magicroom" => magicroom::condition::on_field_end(battle),
        "mistyterrain" => mistyterrain::condition::on_field_end(battle),
        "mudsport" => mudsport::condition::on_field_end(battle),
        "psychicterrain" => psychicterrain::condition::on_field_end(battle),
        "trickroom" => trickroom::condition::on_field_end(battle),
        "watersport" => watersport::condition::on_field_end(battle),
        "wonderroom" => wonderroom::condition::on_field_end(battle),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onFieldRestart callbacks
/// Takes condition_id directly to support fallback from condition_callbacks
pub fn dispatch_condition_on_field_restart(
    battle: &mut Battle,
    condition_id: &str,
    _source_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "echoedvoice" => echoedvoice::condition::on_field_restart(battle),
        "magicroom" => magicroom::condition::on_field_restart(battle, None, None),
        "trickroom" => trickroom::condition::on_field_restart(battle, None, None),
        "wonderroom" => wonderroom::condition::on_field_restart(battle, None, None),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onFieldStart callbacks
/// Takes condition_id directly to support fallback from condition_callbacks
pub fn dispatch_condition_on_field_start(
    battle: &mut Battle,
    condition_id: &str,
    source_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "echoedvoice" => echoedvoice::condition::on_field_start(battle),
        "electricterrain" => {
            electricterrain::condition::on_field_start(battle, None, Some(source_pos), None)
        }
        "fairylock" => fairylock::condition::on_field_start(battle, Some(source_pos)),
        "grassyterrain" => {
            grassyterrain::condition::on_field_start(battle, None, Some(source_pos), None)
        }
        "gravity" => gravity::condition::on_field_start(battle, None, Some(source_pos)),
        "iondeluge" => iondeluge::condition::on_field_start(battle, None, None),
        "magicroom" => magicroom::condition::on_field_start(battle, None, None),
        "mistyterrain" => {
            mistyterrain::condition::on_field_start(battle, None, Some(source_pos), None)
        }
        "mudsport" => mudsport::condition::on_field_start(battle, None, None),
        "psychicterrain" => {
            psychicterrain::condition::on_field_start(battle, None, Some(source_pos), None)
        }
        "trickroom" => trickroom::condition::on_field_start(battle, None, None),
        "watersport" => watersport::condition::on_field_start(battle, None, None),
        "wonderroom" => wonderroom::condition::on_field_start(battle, None, None),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onFoeBeforeMove callbacks
pub fn dispatch_condition_on_foe_before_move(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    _source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "imprison" => imprison::condition::on_foe_before_move(battle, active_move),
        "skydrop" => skydrop::condition::on_foe_before_move(battle, active_move),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onFoeDisableMove callbacks
pub fn dispatch_condition_on_foe_disable_move(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "imprison" => imprison::condition::on_foe_disable_move(battle, source_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onFoeRedirectTarget callbacks
pub fn dispatch_condition_on_foe_redirect_target(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "followme" => {
            followme::condition::on_foe_redirect_target(battle, None, Some(source_pos), active_move)
        }
        "ragepowder" => {
            ragepowder::condition::on_foe_redirect_target(battle, None, Some(source_pos), active_move)
        }
        "spotlight" => {
            spotlight::condition::on_foe_redirect_target(battle, None, Some(source_pos), active_move)
        }
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onFoeTrapPokemon callbacks
pub fn dispatch_condition_on_foe_trap_pokemon(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    _source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "skydrop" => skydrop::condition::on_foe_trap_pokemon(battle),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onHit callbacks
/// Called when a Pokemon with a volatile condition is hit by an attack.
/// The condition_id is the volatile (e.g., "rage"), not the attacking move.
///
/// Arguments follow JavaScript onHit(target, source, move) convention:
/// - target_pos: Pokemon with the volatile (being hit)
/// - source_pos: Pokemon doing the hitting (attacker)
pub fn dispatch_condition_on_hit(
    battle: &mut Battle,
    condition_id: &str,
    source_pos: (usize, usize),   // attacker
    target_pos: (usize, usize),   // Pokemon with volatile (target being hit)
) -> EventResult {
    // Pass target first (Pokemon with volatile), source second (attacker)
    // to match JS onHit(target, source, move) signature
    match condition_id {
        "banefulbunker" => banefulbunker::condition::on_hit(battle, target_pos, Some(source_pos)),
        "beakblast" => beakblast::condition::on_hit(battle, target_pos, Some(source_pos)),
        "burningbulwark" => burningbulwark::condition::on_hit(battle, target_pos, Some(source_pos)),
        "focuspunch" => focuspunch::condition::on_hit(battle, target_pos, Some(source_pos)),
        "kingsshield" => kingsshield::condition::on_hit(battle, target_pos, Some(source_pos)),
        "obstruct" => obstruct::condition::on_hit(battle, target_pos, Some(source_pos)),
        "rage" => rage::condition::on_hit(battle, target_pos, Some(source_pos)),
        "shelltrap" => shelltrap::condition::on_hit(battle, target_pos, Some(source_pos)),
        "silktrap" => silktrap::condition::on_hit(battle, target_pos, Some(source_pos)),
        "spikyshield" => spikyshield::condition::on_hit(battle, target_pos, Some(source_pos)),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onImmunity callbacks
/// Takes condition_id directly to support fallback from condition_callbacks
pub fn dispatch_condition_on_immunity(
    battle: &mut Battle,
    condition_id: &str,
    immunity_type: &str,
    source_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "dig" => dig::condition::on_immunity(battle, source_pos),
        "dive" => dive::condition::on_immunity(battle, source_pos),
        "magnetrise" => magnetrise::condition::on_immunity(battle, source_pos),
        "telekinesis" => telekinesis::condition::on_immunity(battle, immunity_type),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onInvulnerability callbacks
pub fn dispatch_condition_on_invulnerability(
    battle: &mut Battle,
    condition_id: &str,
    source_pos: (usize, usize),
    active_move: Option<&ActiveMove>,
) -> EventResult {
    // Match on condition_id (the volatile that has the callback), not the attacking move
    match condition_id {
        "bounce" => bounce::condition::on_invulnerability(battle, None, Some(source_pos), active_move),
        "commanding" => crate::data::condition_callbacks::commanding::on_invulnerability(battle, None, Some(source_pos), active_move),
        "dig" => dig::condition::on_invulnerability(battle, None, Some(source_pos), active_move),
        "dive" => dive::condition::on_invulnerability(battle, None, Some(source_pos), active_move),
        "fly" => fly::condition::on_invulnerability(battle, None, Some(source_pos), active_move),
        "phantomforce" => crate::data::condition_callbacks::phantomforce::on_invulnerability(battle, None, Some(source_pos), active_move),
        "shadowforce" => crate::data::condition_callbacks::shadowforce::on_invulnerability(battle, None, Some(source_pos), active_move),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onModifyAccuracy callbacks
pub fn dispatch_condition_on_modify_accuracy(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    accuracy: i32,
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "gravity" => gravity::condition::on_modify_accuracy(battle, accuracy),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onModifyBoost callbacks
pub fn dispatch_condition_on_modify_boost(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    _source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "foresight" => foresight::condition::on_modify_boost(battle),
        "miracleeye" => miracleeye::condition::on_modify_boost(battle),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onModifyCritRatio callbacks
/// Takes condition_id directly to support volatiles like gmaxchistrike
pub fn dispatch_condition_on_modify_crit_ratio(
    battle: &mut Battle,
    condition_id: &str,
    crit_ratio: i32,
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    match condition_id {
        "dragoncheer" => {
            dragoncheer::condition::on_modify_crit_ratio(battle, crit_ratio, source_pos)
        }
        "focusenergy" => focusenergy::condition::on_modify_crit_ratio(battle, crit_ratio),
        "gmaxchistrike" => gmaxchistrike::condition::on_modify_crit_ratio(battle, crit_ratio),
        "laserfocus" => laserfocus::condition::on_modify_crit_ratio(battle),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onModifyMove callbacks
/// Takes condition_id directly to support fallback from condition_callbacks
pub fn dispatch_condition_on_modify_move(
    battle: &mut Battle,
    condition_id: &str,
    source_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "gravity" => gravity::condition::on_modify_move(battle, source_pos, None),
        "healblock" => healblock::condition::on_modify_move(battle, source_pos, None),
        "throatchop" => throatchop::condition::on_modify_move(battle, source_pos, None),
        "waterpledge" => waterpledge::condition::on_modify_move(battle, source_pos, None),
        "wonderroom" => wonderroom::condition::on_modify_move(battle, source_pos, None),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onModifySpe callbacks
/// Takes condition_id directly to support fallback from condition_callbacks
pub fn dispatch_condition_on_modify_spe(
    battle: &mut Battle,
    condition_id: &str,
    _spe: i32,
    source_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "grasspledge" => grasspledge::condition::on_modify_spe(battle, source_pos),
        "tailwind" => tailwind::condition::on_modify_spe(battle, source_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onModifyType callbacks
pub fn dispatch_condition_on_modify_type(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    _source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "electrify" => electrify::condition::on_modify_type(battle, active_move),
        "iondeluge" => iondeluge::condition::on_modify_type(battle, active_move),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onMoveAborted callbacks
pub fn dispatch_condition_on_move_aborted(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "bide" => bide::condition::on_move_aborted(battle, source_pos),
        "charge" => charge::condition::on_move_aborted(battle, source_pos, None, active_move),
        "destinybond" => destinybond::condition::on_move_aborted(battle, source_pos, None, active_move),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onNegateImmunity callbacks
pub fn dispatch_condition_on_negate_immunity(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "foresight" => foresight::condition::on_negate_immunity(battle, source_pos),
        "miracleeye" => miracleeye::condition::on_negate_immunity(battle, source_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onOverrideAction callbacks
pub fn dispatch_condition_on_override_action(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "encore" => encore::condition::on_override_action(battle, source_pos, None, active_move),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onRedirectTarget callbacks
pub fn dispatch_condition_on_redirect_target(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "counter" => {
            counter::condition::on_redirect_target(battle, None, Some(source_pos), active_move)
        }
        "mirrorcoat" => {
            mirrorcoat::condition::on_redirect_target(battle, None, Some(source_pos), active_move)
        }
        "skydrop" => skydrop::condition::on_redirect_target(battle, None, Some(source_pos)),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onResidual callbacks
/// Takes condition_id directly to support fallback from condition_callbacks
pub fn dispatch_condition_on_residual(
    battle: &mut Battle,
    condition_id: &str,
    source_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "aquaring" => aquaring::condition::on_residual(battle, source_pos),
        "curse" => curse::condition::on_residual(battle, source_pos),
        "encore" => encore::condition::on_residual(battle, Some(source_pos)),
        "firepledge" => firepledge::condition::on_residual(battle, source_pos),
        "gmaxcannonade" => gmaxcannonade::condition::on_residual(battle, Some(source_pos)),
        "gmaxvinelash" => gmaxvinelash::condition::on_residual(battle, Some(source_pos)),
        "gmaxvolcalith" => gmaxvolcalith::condition::on_residual(battle, Some(source_pos)),
        "gmaxwildfire" => gmaxwildfire::condition::on_residual(battle, Some(source_pos)),
        "grassyterrain" => grassyterrain::condition::on_residual(battle, source_pos),
        "iceball" => iceball::condition::on_residual(battle, Some(source_pos)),
        "ingrain" => ingrain::condition::on_residual(battle, source_pos),
        "leechseed" => leechseed::condition::on_residual(battle, source_pos),
        "nightmare" => nightmare::condition::on_residual(battle, source_pos),
        "octolock" => octolock::condition::on_residual(battle, source_pos),
        "perishsong" => perishsong::condition::on_residual(battle, source_pos),
        "rollout" => rollout::condition::on_residual(battle, Some(source_pos)),
        "saltcure" => saltcure::condition::on_residual(battle, source_pos),
        "syrupbomb" => syrupbomb::condition::on_residual(battle, source_pos),
        "uproar" => uproar::condition::on_residual(battle, Some(source_pos)),
        "wish" => wish::condition::on_residual(battle),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onRestart callbacks
/// Takes condition_id directly to support fallback from condition_callbacks
pub fn dispatch_condition_on_restart(
    battle: &mut Battle,
    condition_id: &str,
    source_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "allyswitch" => allyswitch::condition::on_restart(battle, source_pos),
        "charge" => charge::condition::on_restart(battle, source_pos, None, None),
        "defensecurl" => defensecurl::condition::on_restart(battle, source_pos),
        "furycutter" => furycutter::condition::on_restart(battle),
        "gmaxchistrike" => {
            gmaxchistrike::condition::on_restart(battle, None, Some(source_pos), None)
        }
        "healblock" => healblock::condition::on_restart(battle, None, Some(source_pos), None),
        "helpinghand" => helpinghand::condition::on_restart(battle, None, Some(source_pos)),
        "laserfocus" => laserfocus::condition::on_restart(battle, source_pos),
        "minimize" => minimize::condition::on_restart(battle, source_pos),
        "powershift" => powershift::condition::on_restart(battle, source_pos),
        "powertrick" => powertrick::condition::on_restart(battle, source_pos),
        "smackdown" => smackdown::condition::on_restart(battle, source_pos),
        "stockpile" => stockpile::condition::on_restart(battle, Some(source_pos)),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onFieldResidual callbacks
/// Takes condition_id directly to support fallback from condition_callbacks
pub fn dispatch_condition_on_field_residual(
    battle: &mut Battle,
    condition_id: &str,
    source_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "grassyterrain" => grassyterrain::condition::on_field_residual(battle, source_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onSetStatus callbacks
pub fn dispatch_condition_on_set_status(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "electricterrain" => {
            electricterrain::condition::on_set_status(battle, None, None, Some(source_pos), None)
        }
        "mistyterrain" => {
            mistyterrain::condition::on_set_status(battle, None, None, Some(source_pos), None)
        }
        "safeguard" => {
            safeguard::condition::on_set_status(battle, None, None, Some(source_pos), None)
        }
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onSideEnd callbacks
pub fn dispatch_condition_on_side_end(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    _source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "auroraveil" => auroraveil::condition::on_side_end(battle),
        "firepledge" => firepledge::condition::on_side_end(battle),
        "gmaxcannonade" => gmaxcannonade::condition::on_side_end(battle),
        "gmaxvinelash" => gmaxvinelash::condition::on_side_end(battle),
        "gmaxvolcalith" => gmaxvolcalith::condition::on_side_end(battle),
        "gmaxwildfire" => gmaxwildfire::condition::on_side_end(battle),
        "grasspledge" => grasspledge::condition::on_side_end(battle),
        "lightscreen" => lightscreen::condition::on_side_end(battle),
        "luckychant" => luckychant::condition::on_side_end(battle),
        "mist" => mist::condition::on_side_end(battle),
        "reflect" => reflect::condition::on_side_end(battle),
        "safeguard" => safeguard::condition::on_side_end(battle),
        "tailwind" => tailwind::condition::on_side_end(battle),
        "waterpledge" => waterpledge::condition::on_side_end(battle),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onSideRestart callbacks
pub fn dispatch_condition_on_side_restart(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    _source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "spikes" => spikes::condition::on_side_restart(battle),
        "toxicspikes" => toxicspikes::condition::on_side_restart(battle),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onSideStart callbacks
pub fn dispatch_condition_on_side_start(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "auroraveil" => auroraveil::condition::on_side_start(battle),
        "craftyshield" => craftyshield::condition::on_side_start(battle, None, Some(source_pos)),
        "firepledge" => firepledge::condition::on_side_start(battle),
        "gmaxcannonade" => gmaxcannonade::condition::on_side_start(battle),
        "gmaxsteelsurge" => gmaxsteelsurge::condition::on_side_start(battle),
        "gmaxvinelash" => gmaxvinelash::condition::on_side_start(battle),
        "gmaxvolcalith" => gmaxvolcalith::condition::on_side_start(battle),
        "gmaxwildfire" => gmaxwildfire::condition::on_side_start(battle),
        "grasspledge" => grasspledge::condition::on_side_start(battle),
        "lightscreen" => lightscreen::condition::on_side_start(battle),
        "luckychant" => luckychant::condition::on_side_start(battle),
        "matblock" => matblock::condition::on_side_start(battle, None, Some(source_pos)),
        "mist" => mist::condition::on_side_start(battle),
        "quickguard" => quickguard::condition::on_side_start(battle, None, Some(source_pos)),
        "reflect" => reflect::condition::on_side_start(battle),
        "safeguard" => safeguard::condition::on_side_start(battle, Some(source_pos)),
        "spikes" => spikes::condition::on_side_start(battle),
        "stealthrock" => stealthrock::condition::on_side_start(battle),
        "stickyweb" => stickyweb::condition::on_side_start(battle),
        "tailwind" => tailwind::condition::on_side_start(battle, Some(source_pos)),
        "toxicspikes" => toxicspikes::condition::on_side_start(battle),
        "waterpledge" => waterpledge::condition::on_side_start(battle),
        "wideguard" => wideguard::condition::on_side_start(battle, None, Some(source_pos)),
        _ => EventResult::Continue,
    }
}

/// Dispatch side condition onSideStart callbacks by condition_id
/// Used by single_event_side when the condition_id is known
pub fn dispatch_side_condition_on_side_start_by_id(
    battle: &mut Battle,
    condition_id: &str,
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    match condition_id {
        "auroraveil" => auroraveil::condition::on_side_start(battle),
        "craftyshield" => craftyshield::condition::on_side_start(battle, None, source_pos),
        "firepledge" => firepledge::condition::on_side_start(battle),
        "gmaxcannonade" => gmaxcannonade::condition::on_side_start(battle),
        "gmaxsteelsurge" => gmaxsteelsurge::condition::on_side_start(battle),
        "gmaxvinelash" => gmaxvinelash::condition::on_side_start(battle),
        "gmaxvolcalith" => gmaxvolcalith::condition::on_side_start(battle),
        "gmaxwildfire" => gmaxwildfire::condition::on_side_start(battle),
        "grasspledge" => grasspledge::condition::on_side_start(battle),
        "lightscreen" => lightscreen::condition::on_side_start(battle),
        "luckychant" => luckychant::condition::on_side_start(battle),
        "matblock" => matblock::condition::on_side_start(battle, None, source_pos),
        "mist" => mist::condition::on_side_start(battle),
        "quickguard" => quickguard::condition::on_side_start(battle, None, source_pos),
        "reflect" => reflect::condition::on_side_start(battle),
        "safeguard" => safeguard::condition::on_side_start(battle, source_pos),
        "spikes" => spikes::condition::on_side_start(battle),
        "stealthrock" => stealthrock::condition::on_side_start(battle),
        "stickyweb" => stickyweb::condition::on_side_start(battle),
        "tailwind" => tailwind::condition::on_side_start(battle, source_pos),
        "toxicspikes" => toxicspikes::condition::on_side_start(battle),
        "waterpledge" => waterpledge::condition::on_side_start(battle),
        "wideguard" => wideguard::condition::on_side_start(battle, None, source_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch side condition onSideRestart callbacks by condition_id
/// Used by single_event_side when the condition_id is known
pub fn dispatch_side_condition_on_side_restart_by_id(
    battle: &mut Battle,
    condition_id: &str,
) -> EventResult {
    match condition_id {
        "spikes" => spikes::condition::on_side_restart(battle),
        "toxicspikes" => toxicspikes::condition::on_side_restart(battle),
        _ => EventResult::Continue,
    }
}

/// Dispatch side condition onSideEnd callbacks by condition_id
/// Used by single_event_side when the condition_id is known
pub fn dispatch_side_condition_on_side_end_by_id(
    battle: &mut Battle,
    condition_id: &str,
) -> EventResult {
    match condition_id {
        "auroraveil" => auroraveil::condition::on_side_end(battle),
        "firepledge" => firepledge::condition::on_side_end(battle),
        "gmaxcannonade" => gmaxcannonade::condition::on_side_end(battle),
        "gmaxvinelash" => gmaxvinelash::condition::on_side_end(battle),
        "gmaxvolcalith" => gmaxvolcalith::condition::on_side_end(battle),
        "gmaxwildfire" => gmaxwildfire::condition::on_side_end(battle),
        "grasspledge" => grasspledge::condition::on_side_end(battle),
        "lightscreen" => lightscreen::condition::on_side_end(battle),
        "luckychant" => luckychant::condition::on_side_end(battle),
        "mist" => mist::condition::on_side_end(battle),
        "reflect" => reflect::condition::on_side_end(battle),
        "safeguard" => safeguard::condition::on_side_end(battle),
        "tailwind" => tailwind::condition::on_side_end(battle),
        "waterpledge" => waterpledge::condition::on_side_end(battle),
        _ => EventResult::Continue,
    }
}

/// Dispatch side condition onResidual callbacks by condition_id
/// Used by single_event_side when the condition_id is known
pub fn dispatch_side_condition_on_residual_by_id(
    battle: &mut Battle,
    condition_id: &str,
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    match condition_id {
        "gmaxcannonade" => gmaxcannonade::condition::on_residual(battle, target_pos),
        "gmaxvinelash" => gmaxvinelash::condition::on_residual(battle, target_pos),
        "gmaxvolcalith" => gmaxvolcalith::condition::on_residual(battle, target_pos),
        "gmaxwildfire" => gmaxwildfire::condition::on_residual(battle, target_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onSourceAccuracy callbacks
pub fn dispatch_condition_on_source_accuracy(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "lockon" => {
            lockon::condition::on_source_accuracy(battle, 0, None, Some(source_pos), active_move)
        }
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onSourceBasePower callbacks
pub fn dispatch_condition_on_source_base_power(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "bounce" => {
            bounce::condition::on_source_base_power(battle, 0, None, Some(source_pos), active_move)
        }
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onSourceInvulnerability callbacks
pub fn dispatch_condition_on_source_invulnerability(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "lockon" => {
            lockon::condition::on_source_invulnerability(battle, None, Some(source_pos), active_move)
        }
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onSourceModifyDamage callbacks
/// These are callbacks on conditions/volatiles embedded in moves (like glaiverush)
pub fn dispatch_condition_on_source_modify_damage(
    battle: &mut Battle,
    condition_id: &str,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
    target_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "dig" => dig::condition::on_source_modify_damage(
            battle,
            0,
            Some(source_pos),
            Some(target_pos),
            active_move,
        ),
        "dive" => dive::condition::on_source_modify_damage(
            battle,
            0,
            Some(source_pos),
            Some(target_pos),
            active_move,
        ),
        "fly" => fly::condition::on_source_modify_damage(
            battle,
            0,
            Some(source_pos),
            Some(target_pos),
            active_move,
        ),
        "glaiverush" => glaiverush::condition::on_source_modify_damage(battle),
        "minimize" => minimize::condition::on_source_modify_damage(
            battle,
            0,
            Some(source_pos),
            Some(target_pos),
            active_move,
        ),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onStart callbacks
/// Takes condition_id directly to support fallback from condition_callbacks
pub fn dispatch_condition_on_start(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
    effect: Option<&crate::battle::Effect>,
) -> EventResult {
    match condition_id {
        "allyswitch" => allyswitch::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "aquaring" => aquaring::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "attract" => attract::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "banefulbunker" => banefulbunker::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "beakblast" => beakblast::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "bide" => bide::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "burningbulwark" => burningbulwark::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "charge" => charge::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "counter" => counter::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "curse" => curse::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "destinybond" => destinybond::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "disable" => disable::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "dragoncheer" => dragoncheer::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "electrify" => electrify::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "embargo" => embargo::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "encore" => encore::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "endure" => endure::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "focusenergy" => focusenergy::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "focuspunch" => focuspunch::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "followme" => followme::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "foresight" => foresight::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "furycutter" => furycutter::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "gastroacid" => gastroacid::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "glaiverush" => glaiverush::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "gmaxchistrike" => gmaxchistrike::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "grudge" => grudge::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "healblock" => healblock::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "helpinghand" => helpinghand::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "iceball" => iceball::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "imprison" => imprison::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "ingrain" => ingrain::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "kingsshield" => kingsshield::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "laserfocus" => laserfocus::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "leechseed" => leechseed::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "magiccoat" => magiccoat::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "magnetrise" => magnetrise::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "maxguard" => maxguard::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "metronome" => crate::data::item_callbacks::metronome::condition::on_start(battle, pokemon_pos),
        "miracleeye" => miracleeye::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "mirrorcoat" => mirrorcoat::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "nightmare" => nightmare::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "noretreat" => noretreat::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "obstruct" => obstruct::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "octolock" => octolock::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "powder" => powder::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "powershift" => powershift::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "powertrick" => powertrick::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "protect" => protect::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "rage" => rage::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "ragepowder" => ragepowder::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "rollout" => rollout::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "roost" => roost::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "saltcure" => saltcure::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "shelltrap" => shelltrap::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "silktrap" => silktrap::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "smackdown" => smackdown::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "snatch" => snatch::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "spikyshield" => spikyshield::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "spotlight" => spotlight::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "stockpile" => stockpile::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "substitute" => substitute::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "syrupbomb" => syrupbomb::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "tarshot" => tarshot::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "taunt" => taunt::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "telekinesis" => telekinesis::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "throatchop" => throatchop::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "torment" => torment::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "uproar" => uproar::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "wish" => wish::condition::on_start(battle, pokemon_pos, source_pos, effect),
        "yawn" => yawn::condition::on_start(battle, pokemon_pos, source_pos, effect),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onSwap callbacks
pub fn dispatch_condition_on_swap(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "healingwish" => healingwish::condition::on_swap(battle, Some(source_pos)),
        "lunardance" => lunardance::condition::on_swap(battle, Some(source_pos)),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onSwap callbacks by condition ID
/// Takes condition_id directly to support routing from handle_condition_event
pub fn dispatch_condition_on_swap_by_id(
    battle: &mut Battle,
    condition_id: &str,
    source_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "healingwish" => healingwish::condition::on_swap(battle, Some(source_pos)),
        "lunardance" => lunardance::condition::on_swap(battle, Some(source_pos)),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onSwitchIn callbacks
/// Takes condition_id directly to support fallback from condition_callbacks
pub fn dispatch_condition_on_switch_in(
    battle: &mut Battle,
    condition_id: &str,
    source_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "gmaxsteelsurge" => gmaxsteelsurge::condition::on_switch_in(battle, source_pos),
        "healingwish" => healingwish::condition::on_switch_in(battle, Some(source_pos)),
        "lunardance" => lunardance::condition::on_switch_in(battle, Some(source_pos)),
        "spikes" => spikes::condition::on_switch_in(battle, source_pos),
        "stealthrock" => stealthrock::condition::on_switch_in(battle, source_pos),
        "stickyweb" => stickyweb::condition::on_switch_in(battle, source_pos),
        "toxicspikes" => toxicspikes::condition::on_switch_in(battle, source_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onTrapPokemon callbacks
/// Takes condition_id directly to support fallback from condition_callbacks
pub fn dispatch_condition_on_trap_pokemon(
    battle: &mut Battle,
    condition_id: &str,
    source_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "fairylock" => fairylock::condition::on_trap_pokemon(battle, source_pos),
        "ingrain" => ingrain::condition::on_trap_pokemon(battle, source_pos),
        "noretreat" => noretreat::condition::on_trap_pokemon(battle, source_pos),
        "octolock" => octolock::condition::on_trap_pokemon(battle, source_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onTryAddVolatile callbacks
pub fn dispatch_condition_on_try_add_volatile(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    status: Option<&str>,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    effect_id: Option<&str>,
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "electricterrain" => {
            electricterrain::condition::on_try_add_volatile(battle, status, target_pos)
        }
        "focuspunch" => focuspunch::condition::on_try_add_volatile(battle, status, target_pos.unwrap_or((0,0))),
        "mistyterrain" => {
            mistyterrain::condition::on_try_add_volatile(battle, status, target_pos, source_pos, effect_id)
        }
        "safeguard" => {
            safeguard::condition::on_try_add_volatile(battle, status, target_pos, source_pos, effect_id)
        }
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onTryBoost callbacks
pub fn dispatch_condition_on_try_boost(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "mist" => mist::condition::on_try_boost(battle, Some(source_pos), None, None),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onTryHeal callbacks
pub fn dispatch_condition_on_try_heal(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "healblock" => healblock::condition::on_try_heal(battle, 0, Some(source_pos), None, None),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onTryHit callbacks
pub fn dispatch_condition_on_try_hit(
    battle: &mut Battle,
    _active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
    target_pos: (usize, usize),
    condition_id_param: Option<&str>,
) -> EventResult {
    let condition_id = condition_id_param.unwrap_or(""); match condition_id {
        "banefulbunker" => banefulbunker::condition::on_try_hit(battle, source_pos, target_pos, condition_id_param),
        "burningbulwark" => burningbulwark::condition::on_try_hit(battle, source_pos, target_pos, condition_id_param),
        "craftyshield" => craftyshield::condition::on_try_hit(battle, source_pos, target_pos, condition_id_param),
        "kingsshield" => kingsshield::condition::on_try_hit(battle, source_pos, target_pos, condition_id_param),
        "magiccoat" => magiccoat::condition::on_try_hit(battle, source_pos, target_pos, condition_id_param),
        "matblock" => matblock::condition::on_try_hit(battle, source_pos, target_pos, condition_id_param),
        "maxguard" => maxguard::condition::on_try_hit(battle, source_pos, target_pos, condition_id_param),
        "obstruct" => obstruct::condition::on_try_hit(battle, source_pos, target_pos, condition_id_param),
        "protect" => protect::condition::on_try_hit(battle, source_pos, target_pos, condition_id_param),
        "psychicterrain" => psychicterrain::condition::on_try_hit(battle, source_pos, target_pos, condition_id_param),
        "quickguard" => quickguard::condition::on_try_hit(battle, source_pos, target_pos, condition_id_param),
        "silktrap" => silktrap::condition::on_try_hit(battle, source_pos, target_pos, condition_id_param),
        "spikyshield" => spikyshield::condition::on_try_hit(battle, source_pos, target_pos, condition_id_param),
        "wideguard" => wideguard::condition::on_try_hit(battle, source_pos, target_pos, condition_id_param),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onTryMove callbacks
pub fn dispatch_condition_on_try_move(
    battle: &mut Battle,
    _active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
    condition_id_param: Option<&str>,
) -> EventResult {
    let condition_id = condition_id_param.unwrap_or(""); match condition_id {
        "metronome" => crate::data::item_callbacks::metronome::condition::on_try_move(
            battle,
            source_pos,
            target_pos,
            _active_move,
        ),
        "powder" => powder::condition::on_try_move(battle, source_pos, target_pos, condition_id_param),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onTryPrimaryHit callbacks
pub fn dispatch_condition_on_try_primary_hit(
    battle: &mut Battle,
    condition_id: &str,
    target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
    active_move: Option<&ActiveMove>,
) -> EventResult {
    match condition_id {
        "substitute" => {
            substitute::condition::on_try_primary_hit(
                battle,
                Some(target_pos),
                source_pos,
                active_move,
            )
        }
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onType callbacks
pub fn dispatch_condition_on_type(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
    types: Option<&[String]>,
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "roost" => roost::condition::on_type(battle, Some(source_pos), types),
        _ => EventResult::Continue,
    }
}

/// Dispatch condition onUpdate callbacks
pub fn dispatch_condition_on_update(
    battle: &mut Battle,
    active_move: Option<&ActiveMove>,
    source_pos: (usize, usize),
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or(""); match move_id {
        "attract" => attract::condition::on_update(battle, source_pos),
        "fling" => fling::condition::on_update(battle, source_pos),
        "syrupbomb" => syrupbomb::condition::on_update(battle, source_pos),
        "telekinesis" => telekinesis::condition::on_update(battle, source_pos),
        _ => EventResult::Continue,
    }
}
