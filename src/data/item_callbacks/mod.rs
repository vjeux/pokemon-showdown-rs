//! Item Callback Handlers
//!
//! This module provides dispatch functions for item callbacks.
//! Individual item implementations will be added as needed.

use crate::battle::Battle;
use crate::battle_actions::ActiveMove;
use crate::event::EventResult;

// Individual item modules
pub mod abilityshield;
pub mod abomasite;
pub mod absolite;
pub mod absolitez;
pub mod absorbbulb;
pub mod adamantcrystal;
pub mod adamantorb;
pub mod adrenalineorb;
pub mod aerodactylite;
pub mod aggronite;
pub mod aguavberry;
pub mod airballoon;
pub mod alakazite;
pub mod altarianite;
pub mod ampharosite;
pub mod apicotberry;
pub mod aspearberry;
pub mod assaultvest;
pub mod audinite;
pub mod babiriberry;
pub mod banettite;
pub mod barbaracite;
pub mod baxcalibrite;
pub mod beedrillite;
pub mod berry;
pub mod berryjuice;
pub mod berserkgene;
pub mod bigroot;
pub mod bitterberry;
pub mod blackbelt;
pub mod blackglasses;
pub mod blacksludge;
pub mod blastoisinite;
pub mod blazikenite;
pub mod blueorb;
pub mod boosterenergy;
pub mod brightpowder;
pub mod buggem;
pub mod bugmemory;
pub mod burndrive;
pub mod burntberry;
pub mod cameruptite;
pub mod cellbattery;
pub mod chandelurite;
pub mod charcoal;
pub mod charizarditex;
pub mod charizarditey;
pub mod chartiberry;
pub mod cheriberry;
pub mod chesnaughtite;
pub mod chestoberry;
pub mod chilanberry;
pub mod chilldrive;
pub mod chimechite;
pub mod choiceband;
pub mod choicescarf;
pub mod choicespecs;
pub mod chopleberry;
pub mod clearamulet;
pub mod clefablite;
pub mod cobaberry;
pub mod colburberry;
pub mod cornerstonemask;
pub mod covertcloak;
pub mod crabominite;
pub mod crucibellite;
pub mod custapberry;
pub mod darkgem;
pub mod darkmemory;
pub mod darkranite;
pub mod deepseascale;
pub mod deepseatooth;
pub mod delphoxite;
pub mod destinyknot;
pub mod diancite;
pub mod dousedrive;
pub mod dracoplate;
pub mod dragalgite;
pub mod dragonfang;
pub mod dragongem;
pub mod dragoninite;
pub mod dragonmemory;
pub mod drampanite;
pub mod dreadplate;
pub mod earthplate;
pub mod eelektrossite;
pub mod ejectbutton;
pub mod ejectpack;
pub mod electricgem;
pub mod electricmemory;
pub mod electricseed;
pub mod emboarite;
pub mod enigmaberry;
pub mod eviolite;
pub mod excadrite;
pub mod expertbelt;
pub mod fairyfeather;
pub mod fairygem;
pub mod fairymemory;
pub mod falinksite;
pub mod feraligite;
pub mod fightinggem;
pub mod fightingmemory;
pub mod figyberry;
pub mod firegem;
pub mod firememory;
pub mod fistplate;
pub mod flameorb;
pub mod flameplate;
pub mod floatstone;
pub mod floettite;
pub mod flyinggem;
pub mod flyingmemory;
pub mod focusband;
pub mod focussash;
pub mod froslassite;
pub mod fullincense;
pub mod galladite;
pub mod ganlonberry;
pub mod garchompite;
pub mod garchompitez;
pub mod gardevoirite;
pub mod gengarite;
pub mod ghostgem;
pub mod ghostmemory;
pub mod glalitite;
pub mod glimmoranite;
pub mod goldberry;
pub mod golisopite;
pub mod golurkite;
pub mod grassgem;
pub mod grassmemory;
pub mod grassyseed;
pub mod greninjite;
pub mod griseouscore;
pub mod griseousorb;
pub mod groundgem;
pub mod groundmemory;
pub mod gyaradosite;
pub mod habanberry;
pub mod hardstone;
pub mod hawluchanite;
pub mod hearthflamemask;
pub mod heatranite;
pub mod heracronite;
pub mod houndoominite;
pub mod iapapaberry;
pub mod iceberry;
pub mod icegem;
pub mod icememory;
pub mod icicleplate;
pub mod insectplate;
pub mod ironball;
pub mod ironplate;
pub mod jabocaberry;
pub mod kangaskhanite;
pub mod kasibberry;
pub mod kebiaberry;
pub mod keeberry;
pub mod kingsrock;
pub mod laggingtail;
pub mod lansatberry;
pub mod latiasite;
pub mod latiosite;
pub mod laxincense;
pub mod leek;
pub mod leftovers;
pub mod leppaberry;
pub mod liechiberry;
pub mod lifeorb;
pub mod lightball;
pub mod loadeddice;
pub mod lopunnite;
pub mod lucarionite;
pub mod lucarionitez;
pub mod luckypunch;
pub mod lumberry;
pub mod luminousmoss;
pub mod lustrousglobe;
pub mod lustrousorb;
pub mod machobrace;
pub mod magearnite;
pub mod magnet;
pub mod magoberry;
pub mod mail;
pub mod malamarite;
pub mod manectite;
pub mod marangaberry;
pub mod mawilite;
pub mod meadowplate;
pub mod medichamite;
pub mod meganiumite;
pub mod mentalherb;
pub mod meowsticite;
pub mod metagrossite;
pub mod metalcoat;
pub mod metalpowder;
pub mod metronome;
pub mod mewtwonitex;
pub mod mewtwonitey;
pub mod micleberry;
pub mod mindplate;
pub mod mintberry;
pub mod miracleberry;
pub mod miracleseed;
pub mod mirrorherb;
pub mod mistyseed;
pub mod muscleband;
pub mod mysteryberry;
pub mod mysticwater;
pub mod nevermeltice;
pub mod normalgem;
pub mod occaberry;
pub mod oddincense;
pub mod oranberry;
pub mod passhoberry;
pub mod payapaberry;
pub mod pechaberry;
pub mod persimberry;
pub mod petayaberry;
pub mod pidgeotite;
pub mod pinkbow;
pub mod pinsirite;
pub mod pixieplate;
pub mod poisonbarb;
pub mod poisongem;
pub mod poisonmemory;
pub mod polkadotbow;
pub mod poweranklet;
pub mod powerband;
pub mod powerbelt;
pub mod powerbracer;
pub mod powerherb;
pub mod powerlens;
pub mod powerweight;
pub mod przcureberry;
pub mod psncureberry;
pub mod psychicgem;
pub mod psychicmemory;
pub mod psychicseed;
pub mod punchingglove;
pub mod pyroarite;
pub mod quickclaw;
pub mod quickpowder;
pub mod raichunitex;
pub mod raichunitey;
pub mod rawstberry;
pub mod razorclaw;
pub mod razorfang;
pub mod redcard;
pub mod redorb;
pub mod rindoberry;
pub mod ringtarget;
pub mod rockgem;
pub mod rockincense;
pub mod rockmemory;
pub mod rockyhelmet;
pub mod roomservice;
pub mod roseincense;
pub mod roseliberry;
pub mod rowapberry;
pub mod rustedshield;
pub mod rustedsword;
pub mod sablenite;
pub mod safetygoggles;
pub mod salacberry;
pub mod salamencite;
pub mod sceptilite;
pub mod scizorite;
pub mod scolipite;
pub mod scopelens;
pub mod scovillainite;
pub mod scraftinite;
pub mod seaincense;
pub mod sharpbeak;
pub mod sharpedonite;
pub mod shedshell;
pub mod shellbell;
pub mod shockdrive;
pub mod shucaberry;
pub mod silkscarf;
pub mod silverpowder;
pub mod sitrusberry;
pub mod skarmorite;
pub mod skyplate;
pub mod slowbronite;
pub mod snowball;
pub mod softsand;
pub mod souldew;
pub mod spelltag;
pub mod splashplate;
pub mod spookyplate;
pub mod staraptite;
pub mod starfberry;
pub mod starminite;
pub mod steelgem;
pub mod steelixite;
pub mod steelmemory;
pub mod stick;
pub mod stickybarb;
pub mod stoneplate;
pub mod swampertite;
pub mod tangaberry;
pub mod tatsugirinite;
pub mod thickclub;
pub mod throatspray;
pub mod toxicorb;
pub mod toxicplate;
pub mod twistedspoon;
pub mod tyranitarite;
pub mod utilityumbrella;
pub mod venusaurite;
pub mod victreebelite;
pub mod vilevial;
pub mod wacanberry;
pub mod watergem;
pub mod watermemory;
pub mod waveincense;
pub mod weaknesspolicy;
pub mod wellspringmask;
pub mod whiteherb;
pub mod widelens;
pub mod wikiberry;
pub mod wiseglasses;
pub mod yacheberry;
pub mod zapplate;
pub mod zeraorite;
pub mod zoomlens;
pub mod zygardite;


// =========================================================================
// =========================================================================
// DISPATCH FUNCTIONS
//
// These functions route item events to specific item implementations.
// Auto-generated by scripts/generate-item-dispatchers.js
// =========================================================================
//   onAfterBoost(boost, pokemon)
//   onAfterBoost(boost, target, source, effect)

/// Dispatch onAfterBoost callbacks
pub fn dispatch_on_after_boost(
    battle: &mut Battle,
    item_id: &str,
    target_pos: (usize, usize),
    boost: &crate::dex_data::BoostsTable,
) -> EventResult {
    match item_id {
        "adrenalineorb" => adrenalineorb::on_after_boost(battle, target_pos, boost),
        "ejectpack" => ejectpack::on_after_boost(battle, target_pos, boost),
        _ => EventResult::Continue,
    }
}
//   onAfterMoveSecondary()
//   onAfterMoveSecondary(target)
//   onAfterMoveSecondary(target, source, move)

/// Dispatch onAfterMoveSecondary callbacks
pub fn dispatch_on_after_move_secondary(
    battle: &mut Battle,
    item_id: &str,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    active_move: Option<&ActiveMove>,
) -> EventResult {
    match item_id {
        "ejectbutton" => ejectbutton::on_after_move_secondary(battle, target_pos, source_pos, active_move),
        "keeberry" => keeberry::on_after_move_secondary(battle, target_pos, source_pos, active_move),
        "marangaberry" => marangaberry::on_after_move_secondary(battle, target_pos, source_pos, active_move),
        "redcard" => redcard::on_after_move_secondary(battle, target_pos, source_pos, active_move),
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
    item_id: &str,
    source_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
    active_move: Option<&ActiveMove>,
) -> EventResult {
    match item_id {
        "lifeorb" => lifeorb::on_after_move_secondary_self(battle, source_pos, target_pos, active_move),
        "shellbell" => shellbell::on_after_move_secondary_self(battle, source_pos, target_pos, active_move),
        "throatspray" => throatspray::on_after_move_secondary_self(battle, source_pos, target_pos, active_move),
        _ => EventResult::Continue,
    }
}
//   onAfterSetStatus(status, pokemon)
//   onAfterSetStatus(status, target, source, effect)

/// Dispatch onAfterSetStatus callbacks
pub fn dispatch_on_after_set_status(
    battle: &mut Battle,
    item_id: &str,
    target_pos: (usize, usize),
) -> EventResult {
    match item_id {
        "lumberry" => lumberry::on_after_set_status(battle, target_pos),
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
    item_id: &str,
    damage: i32,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    effect_id: Option<&str>,
) -> EventResult {
    match item_id {
        "airballoon" => airballoon::on_after_sub_damage(battle, damage, target_pos, source_pos, effect_id),
        _ => EventResult::Continue,
    }
}
//   onAnyAfterMega()

/// Dispatch onAnyAfterMega callbacks
pub fn dispatch_on_any_after_mega(
    battle: &mut Battle,
    item_id: &str,
) -> EventResult {
    match item_id {
        "ejectpack" => ejectpack::on_any_after_mega(battle),
        "mirrorherb" => mirrorherb::on_any_after_mega(battle),
        "whiteherb" => whiteherb::on_any_after_mega(battle),
        _ => EventResult::Continue,
    }
}
//   onAnyAfterMove()

/// Dispatch onAnyAfterMove callbacks
pub fn dispatch_on_any_after_move(
    battle: &mut Battle,
    item_id: &str,
) -> EventResult {
    match item_id {
        "ejectpack" => ejectpack::on_any_after_move(battle),
        "mirrorherb" => mirrorherb::on_any_after_move(battle),
        "whiteherb" => whiteherb::on_any_after_move(battle),
        _ => EventResult::Continue,
    }
}
//   onAnyAfterTerastallization()

/// Dispatch onAnyAfterTerastallization callbacks
pub fn dispatch_on_any_after_terastallization(
    battle: &mut Battle,
    item_id: &str,
) -> EventResult {
    match item_id {
        "mirrorherb" => mirrorherb::on_any_after_terastallization(battle),
        _ => EventResult::Continue,
    }
}
//   onAnyPseudoWeatherChange()

/// Dispatch onAnyPseudoWeatherChange callbacks
pub fn dispatch_on_any_pseudo_weather_change(
    battle: &mut Battle,
    item_id: &str,
) -> EventResult {
    match item_id {
        "roomservice" => roomservice::on_any_pseudo_weather_change(battle),
        _ => EventResult::Continue,
    }
}
//   onAnySwitchIn()
//   onAnySwitchIn(pokemon)

/// Dispatch onAnySwitchIn callbacks
pub fn dispatch_on_any_switch_in(
    battle: &mut Battle,
    item_id: &str,
) -> EventResult {
    match item_id {
        "ejectpack" => ejectpack::on_any_switch_in(battle),
        "mirrorherb" => mirrorherb::on_any_switch_in(battle),
        "whiteherb" => whiteherb::on_any_switch_in(battle),
        _ => EventResult::Continue,
    }
}
//   onAttract(target, source)

/// Dispatch onAttract callbacks
pub fn dispatch_on_attract(
    battle: &mut Battle,
    item_id: &str,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    match item_id {
        "destinyknot" => destinyknot::on_attract(battle, target_pos, source_pos),
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
pub fn dispatch_on_base_power(
    battle: &mut Battle,
    item_id: &str,
    base_power: i32,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    match item_id {
        "adamantcrystal" => adamantcrystal::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "adamantorb" => adamantorb::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "blackbelt" => blackbelt::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "blackglasses" => blackglasses::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "charcoal" => charcoal::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "cornerstonemask" => cornerstonemask::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "dracoplate" => dracoplate::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "dragonfang" => dragonfang::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "dreadplate" => dreadplate::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "earthplate" => earthplate::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "fairyfeather" => fairyfeather::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "fistplate" => fistplate::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "flameplate" => flameplate::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "griseouscore" => griseouscore::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "griseousorb" => griseousorb::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "hardstone" => hardstone::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "hearthflamemask" => hearthflamemask::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "icicleplate" => icicleplate::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "insectplate" => insectplate::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "ironplate" => ironplate::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "lustrousglobe" => lustrousglobe::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "lustrousorb" => lustrousorb::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "magnet" => magnet::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "meadowplate" => meadowplate::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "metalcoat" => metalcoat::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "mindplate" => mindplate::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "miracleseed" => miracleseed::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "muscleband" => muscleband::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "mysticwater" => mysticwater::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "nevermeltice" => nevermeltice::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "oddincense" => oddincense::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "pinkbow" => pinkbow::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "pixieplate" => pixieplate::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "poisonbarb" => poisonbarb::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "polkadotbow" => polkadotbow::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "punchingglove" => punchingglove::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "rockincense" => rockincense::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "roseincense" => roseincense::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "seaincense" => seaincense::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "sharpbeak" => sharpbeak::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "silkscarf" => silkscarf::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "silverpowder" => silverpowder::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "skyplate" => skyplate::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "softsand" => softsand::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "souldew" => souldew::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "spelltag" => spelltag::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "splashplate" => splashplate::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "spookyplate" => spookyplate::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "stoneplate" => stoneplate::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "toxicplate" => toxicplate::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "twistedspoon" => twistedspoon::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "vilevial" => vilevial::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "waveincense" => waveincense::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "wellspringmask" => wellspringmask::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "wiseglasses" => wiseglasses::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "zapplate" => zapplate::on_base_power(battle, base_power, pokemon_pos, target_pos),
        _ => EventResult::Continue,
    }
}
//   onChargeMove(pokemon, target, move)

/// Dispatch onChargeMove callbacks
pub fn dispatch_on_charge_move(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
    active_move: Option<&ActiveMove>,
) -> EventResult {
    match item_id {
        "powerherb" => powerherb::on_charge_move(battle, pokemon_pos, target_pos, active_move),
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
    item_id: &str,
    damage: i32,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    effect_id: Option<&str>,
) -> EventResult {
    match item_id {
        "focusband" => focusband::on_damage(battle, damage, target_pos, source_pos, effect_id),
        "focussash" => focussash::on_damage(battle, damage, target_pos, source_pos, effect_id),
        _ => EventResult::Continue,
    }
}
//   onDamagingHit()
//   onDamagingHit(damage, target, source, effect)
//   onDamagingHit(damage, target, source, move)

/// Dispatch onDamagingHit callbacks
pub fn dispatch_on_damaging_hit(
    battle: &mut Battle,
    item_id: &str,
    damage: i32,
    target_pos: (usize, usize),
    source_pos: (usize, usize),
) -> EventResult {
    match item_id {
        "absorbbulb" => absorbbulb::on_damaging_hit(battle, damage, target_pos, source_pos),
        "airballoon" => airballoon::on_damaging_hit(battle, damage, target_pos, source_pos),
        "cellbattery" => cellbattery::on_damaging_hit(battle, damage, target_pos, source_pos),
        "jabocaberry" => jabocaberry::on_damaging_hit(battle, damage, target_pos, source_pos),
        "luminousmoss" => luminousmoss::on_damaging_hit(battle, damage, target_pos, source_pos),
        "rockyhelmet" => rockyhelmet::on_damaging_hit(battle, damage, target_pos, source_pos),
        "rowapberry" => rowapberry::on_damaging_hit(battle, damage, target_pos, source_pos),
        "snowball" => snowball::on_damaging_hit(battle, damage, target_pos, source_pos),
        "weaknesspolicy" => weaknesspolicy::on_damaging_hit(battle, damage, target_pos, source_pos),
        _ => EventResult::Continue,
    }
}
//   onDisableMove(pokemon)
//   onDisableMove(target)

/// Dispatch onDisableMove callbacks
pub fn dispatch_on_disable_move(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match item_id {
        "assaultvest" => assaultvest::on_disable_move(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}
//   onEat()
//   onEat(pokemon)

/// Dispatch onEat callbacks
pub fn dispatch_on_eat(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match item_id {
        "aguavberry" => aguavberry::on_eat(battle, pokemon_pos),
        "apicotberry" => apicotberry::on_eat(battle, pokemon_pos),
        "aspearberry" => aspearberry::on_eat(battle, pokemon_pos),
        "babiriberry" => babiriberry::on_eat(battle, pokemon_pos),
        "berry" => berry::on_eat(battle, pokemon_pos),
        "bitterberry" => bitterberry::on_eat(battle, pokemon_pos),
        "burntberry" => burntberry::on_eat(battle, pokemon_pos),
        "chartiberry" => chartiberry::on_eat(battle, pokemon_pos),
        "cheriberry" => cheriberry::on_eat(battle, pokemon_pos),
        "chestoberry" => chestoberry::on_eat(battle, pokemon_pos),
        "chilanberry" => chilanberry::on_eat(battle, pokemon_pos),
        "chopleberry" => chopleberry::on_eat(battle, pokemon_pos),
        "cobaberry" => cobaberry::on_eat(battle, pokemon_pos),
        "colburberry" => colburberry::on_eat(battle, pokemon_pos),
        "custapberry" => custapberry::on_eat(battle, pokemon_pos),
        "enigmaberry" => enigmaberry::on_eat(battle, pokemon_pos),
        "figyberry" => figyberry::on_eat(battle, pokemon_pos),
        "ganlonberry" => ganlonberry::on_eat(battle, pokemon_pos),
        "goldberry" => goldberry::on_eat(battle, pokemon_pos),
        "habanberry" => habanberry::on_eat(battle, pokemon_pos),
        "iapapaberry" => iapapaberry::on_eat(battle, pokemon_pos),
        "iceberry" => iceberry::on_eat(battle, pokemon_pos),
        "jabocaberry" => jabocaberry::on_eat(battle, pokemon_pos),
        "kasibberry" => kasibberry::on_eat(battle, pokemon_pos),
        "kebiaberry" => kebiaberry::on_eat(battle, pokemon_pos),
        "keeberry" => keeberry::on_eat(battle, pokemon_pos),
        "lansatberry" => lansatberry::on_eat(battle, pokemon_pos),
        "leppaberry" => leppaberry::on_eat(battle, pokemon_pos),
        "liechiberry" => liechiberry::on_eat(battle, pokemon_pos),
        "lumberry" => lumberry::on_eat(battle, pokemon_pos),
        "magoberry" => magoberry::on_eat(battle, pokemon_pos),
        "marangaberry" => marangaberry::on_eat(battle, pokemon_pos),
        "micleberry" => micleberry::on_eat(battle, pokemon_pos),
        "mintberry" => mintberry::on_eat(battle, pokemon_pos),
        "miracleberry" => miracleberry::on_eat(battle, pokemon_pos),
        "mysteryberry" => mysteryberry::on_eat(battle, pokemon_pos),
        "occaberry" => occaberry::on_eat(battle, pokemon_pos),
        "oranberry" => oranberry::on_eat(battle, pokemon_pos),
        "passhoberry" => passhoberry::on_eat(battle, pokemon_pos),
        "payapaberry" => payapaberry::on_eat(battle, pokemon_pos),
        "pechaberry" => pechaberry::on_eat(battle, pokemon_pos),
        "persimberry" => persimberry::on_eat(battle, pokemon_pos),
        "petayaberry" => petayaberry::on_eat(battle, pokemon_pos),
        "przcureberry" => przcureberry::on_eat(battle, pokemon_pos),
        "psncureberry" => psncureberry::on_eat(battle, pokemon_pos),
        "rawstberry" => rawstberry::on_eat(battle, pokemon_pos),
        "rindoberry" => rindoberry::on_eat(battle, pokemon_pos),
        "roseliberry" => roseliberry::on_eat(battle, pokemon_pos),
        "rowapberry" => rowapberry::on_eat(battle, pokemon_pos),
        "salacberry" => salacberry::on_eat(battle, pokemon_pos),
        "shucaberry" => shucaberry::on_eat(battle, pokemon_pos),
        "sitrusberry" => sitrusberry::on_eat(battle, pokemon_pos),
        "starfberry" => starfberry::on_eat(battle, pokemon_pos),
        "tangaberry" => tangaberry::on_eat(battle, pokemon_pos),
        "wacanberry" => wacanberry::on_eat(battle, pokemon_pos),
        "wikiberry" => wikiberry::on_eat(battle, pokemon_pos),
        "yacheberry" => yacheberry::on_eat(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}
//   onEffectiveness()
//   onEffectiveness(typeMod, target, type)
//   onEffectiveness(typeMod, target, type, move)

/// Dispatch onEffectiveness callbacks
pub fn dispatch_on_effectiveness(
    battle: &mut Battle,
    item_id: &str,
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    match item_id {
        "ironball" => ironball::on_effectiveness(battle, target_pos),
        _ => EventResult::Continue,
    }
}
//   onEnd()
//   onEnd(pokemon)
//   onEnd(source)
//   onEnd(target)

/// Dispatch onEnd callbacks
pub fn dispatch_on_end(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match item_id {
        "ejectpack" => ejectpack::on_end(battle, pokemon_pos),
        "mirrorherb" => mirrorherb::on_end(battle, pokemon_pos),
        "utilityumbrella" => utilityumbrella::on_end(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}
//   onFoeAfterBoost(boost, target, source, effect)

/// Dispatch onFoeAfterBoost callbacks
pub fn dispatch_on_foe_after_boost(
    battle: &mut Battle,
    item_id: &str,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    effect_id: Option<&str>,
    boost: &crate::dex_data::BoostsTable,
) -> EventResult {
    match item_id {
        "mirrorherb" => mirrorherb::on_foe_after_boost(battle, target_pos, source_pos, effect_id, boost),
        _ => EventResult::Continue,
    }
}
//   onFractionalPriority()
//   onFractionalPriority(priority, pokemon)
//   onFractionalPriority(priority, pokemon, target, move)

/// Dispatch onFractionalPriority callbacks
pub fn dispatch_on_fractional_priority(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
    priority: f64,
) -> EventResult {
    match item_id {
        "custapberry" => custapberry::on_fractional_priority(battle, pokemon_pos, priority),
        "fullincense" => fullincense::on_fractional_priority(battle, pokemon_pos, priority),
        "laggingtail" => laggingtail::on_fractional_priority(battle, pokemon_pos, priority),
        "quickclaw" => quickclaw::on_fractional_priority(battle, pokemon_pos, priority),
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

/// Dispatch onHit callbacks
pub fn dispatch_on_hit(
    battle: &mut Battle,
    item_id: &str,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    active_move: Option<&ActiveMove>,
) -> EventResult {
    match item_id {
        "enigmaberry" => enigmaberry::on_hit(battle, target_pos, source_pos, active_move),
        "stickybarb" => stickybarb::on_hit(battle, target_pos, source_pos, active_move),
        _ => EventResult::Continue,
    }
}
//   onImmunity(type)
//   onImmunity(type, pokemon)

/// Dispatch onImmunity callbacks
pub fn dispatch_on_immunity(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
    immunity_type: &str,
) -> EventResult {
    match item_id {
        "safetygoggles" => safetygoggles::on_immunity(battle, pokemon_pos, immunity_type),
        _ => EventResult::Continue,
    }
}
//   onMaybeTrapPokemon(pokemon)

/// Dispatch onMaybeTrapPokemon callbacks
pub fn dispatch_on_maybe_trap_pokemon(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match item_id {
        "shedshell" => shedshell::on_maybe_trap_pokemon(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}
//   onModifyAccuracy(accuracy)
//   onModifyAccuracy(accuracy, target)
//   onModifyAccuracy(accuracy, target, source)
//   onModifyAccuracy(accuracy, target, source, move)

/// Dispatch onModifyAccuracy callbacks
pub fn dispatch_on_modify_accuracy(
    battle: &mut Battle,
    item_id: &str,
) -> EventResult {
    match item_id {
        "brightpowder" => brightpowder::on_modify_accuracy(battle),
        "laxincense" => laxincense::on_modify_accuracy(battle),
        _ => EventResult::Continue,
    }
}
//   onModifyAtk()
//   onModifyAtk(atk)
//   onModifyAtk(atk, attacker, defender)
//   onModifyAtk(atk, attacker, defender, move)
//   onModifyAtk(atk, pokemon)
//   onModifyAtk(atk, pokemon, defender, move)
//   onModifyAtk(atk, pokemon, target, move)
//   onModifyAtk(atk, source, target, move)
//   onModifyAtk(atk, user, target, move)

/// Dispatch onModifyAtk callbacks
pub fn dispatch_on_modify_atk(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match item_id {
        "choiceband" => choiceband::on_modify_atk(battle, pokemon_pos),
        "lightball" => lightball::on_modify_atk(battle, pokemon_pos),
        "thickclub" => thickclub::on_modify_atk(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}
//   onModifyCritRatio(critRatio)
//   onModifyCritRatio(critRatio, source)
//   onModifyCritRatio(critRatio, source, target)
//   onModifyCritRatio(critRatio, source, target, move)
//   onModifyCritRatio(critRatio, user)

/// Dispatch onModifyCritRatio callbacks
pub fn dispatch_on_modify_crit_ratio(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
    crit_ratio: i32,
) -> EventResult {
    match item_id {
        "leek" => leek::on_modify_crit_ratio(battle, pokemon_pos, crit_ratio),
        "luckypunch" => luckypunch::on_modify_crit_ratio(battle, pokemon_pos, crit_ratio),
        "razorclaw" => razorclaw::on_modify_crit_ratio(battle, pokemon_pos, crit_ratio),
        "scopelens" => scopelens::on_modify_crit_ratio(battle, pokemon_pos, crit_ratio),
        "stick" => stick::on_modify_crit_ratio(battle, pokemon_pos, crit_ratio),
        _ => EventResult::Continue,
    }
}
//   onModifyDamage()
//   onModifyDamage(damage, source, target, move)

/// Dispatch onModifyDamage callbacks
pub fn dispatch_on_modify_damage(
    battle: &mut Battle,
    item_id: &str,
    damage: i32,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    match item_id {
        "expertbelt" => expertbelt::on_modify_damage(battle, damage, pokemon_pos, target_pos),
        "lifeorb" => lifeorb::on_modify_damage(battle, damage, pokemon_pos, target_pos),
        _ => EventResult::Continue,
    }
}
//   onModifyDef()
//   onModifyDef(def)
//   onModifyDef(def, pokemon)
//   onModifyDef(def, target, source, move)
//   onModifyDef(pokemon)

/// Dispatch onModifyDef callbacks
pub fn dispatch_on_modify_def(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match item_id {
        "eviolite" => eviolite::on_modify_def(battle, pokemon_pos),
        "metalpowder" => metalpowder::on_modify_def(battle, pokemon_pos),
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
    item_id: &str,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    match item_id {
        "choiceband" => choiceband::on_modify_move(battle, pokemon_pos, target_pos),
        "choicescarf" => choicescarf::on_modify_move(battle, pokemon_pos, target_pos),
        "choicespecs" => choicespecs::on_modify_move(battle, pokemon_pos, target_pos),
        "kingsrock" => kingsrock::on_modify_move(battle, pokemon_pos, target_pos),
        "loadeddice" => loadeddice::on_modify_move(battle, pokemon_pos, target_pos),
        "punchingglove" => punchingglove::on_modify_move(battle, pokemon_pos, target_pos),
        "razorfang" => razorfang::on_modify_move(battle, pokemon_pos, target_pos),
        _ => EventResult::Continue,
    }
}
//   onModifySecondaries(secondaries)

/// Dispatch onModifySecondaries callbacks
pub fn dispatch_on_modify_secondaries(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
    secondaries: &Vec<crate::battle_actions::SecondaryEffect>,
) -> EventResult {
    match item_id {
        "covertcloak" => covertcloak::on_modify_secondaries(battle, pokemon_pos, secondaries),
        _ => EventResult::Continue,
    }
}
//   onModifySpA()
//   onModifySpA(atk, attacker, defender)
//   onModifySpA(atk, attacker, defender, move)
//   onModifySpA(atk, pokemon)
//   onModifySpA(atk, pokemon, defender, move)
//   onModifySpA(atk, pokemon, target, move)
//   onModifySpA(relayVar, source, target, move)
//   onModifySpA(spa)
//   onModifySpA(spa, attacker, defender, move)
//   onModifySpA(spa, pokemon)
//   onModifySpA(spa, pokemon, target, move)
//   onModifySpA(spa, user, target, move)

/// Dispatch onModifySpA callbacks
pub fn dispatch_on_modify_sp_a(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match item_id {
        "choicespecs" => choicespecs::on_modify_sp_a(battle, pokemon_pos),
        "deepseatooth" => deepseatooth::on_modify_sp_a(battle, pokemon_pos),
        "lightball" => lightball::on_modify_sp_a(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}
//   onModifySpD()
//   onModifySpD(spd)
//   onModifySpD(spd, pokemon)
//   onModifySpD(spd, target, source, move)

/// Dispatch onModifySpD callbacks
pub fn dispatch_on_modify_sp_d(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match item_id {
        "assaultvest" => assaultvest::on_modify_sp_d(battle, pokemon_pos),
        "deepseascale" => deepseascale::on_modify_sp_d(battle, pokemon_pos),
        "eviolite" => eviolite::on_modify_sp_d(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}
//   onModifySpe(spe)
//   onModifySpe(spe, pokemon)
//   onModifySpe(this, spe, pokemon)

/// Dispatch onModifySpe callbacks
pub fn dispatch_on_modify_spe(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match item_id {
        "choicescarf" => choicescarf::on_modify_spe(battle, pokemon_pos),
        "ironball" => ironball::on_modify_spe(battle, pokemon_pos),
        "machobrace" => machobrace::on_modify_spe(battle, pokemon_pos),
        "poweranklet" => poweranklet::on_modify_spe(battle, pokemon_pos),
        "powerband" => powerband::on_modify_spe(battle, pokemon_pos),
        "powerbelt" => powerbelt::on_modify_spe(battle, pokemon_pos),
        "powerbracer" => powerbracer::on_modify_spe(battle, pokemon_pos),
        "powerlens" => powerlens::on_modify_spe(battle, pokemon_pos),
        "powerweight" => powerweight::on_modify_spe(battle, pokemon_pos),
        "quickpowder" => quickpowder::on_modify_spe(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}
//   onModifyWeight(weighthg)
//   onModifyWeight(weighthg, pokemon)

/// Dispatch onModifyWeight callbacks
pub fn dispatch_on_modify_weight(
    battle: &mut Battle,
    item_id: &str,
    weighthg: i32,
) -> EventResult {
    match item_id {
        "floatstone" => floatstone::on_modify_weight(battle, weighthg),
        _ => EventResult::Continue,
    }
}
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
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match item_id {
        "berry" => berry::on_residual(battle, pokemon_pos),
        "blacksludge" => blacksludge::on_residual(battle, pokemon_pos),
        "ejectpack" => ejectpack::on_residual(battle, pokemon_pos),
        "flameorb" => flameorb::on_residual(battle, pokemon_pos),
        "goldberry" => goldberry::on_residual(battle, pokemon_pos),
        "leftovers" => leftovers::on_residual(battle, pokemon_pos),
        "micleberry" => micleberry::on_residual(battle, pokemon_pos),
        "mirrorherb" => mirrorherb::on_residual(battle, pokemon_pos),
        "stickybarb" => stickybarb::on_residual(battle, pokemon_pos),
        "toxicorb" => toxicorb::on_residual(battle, pokemon_pos),
        "whiteherb" => whiteherb::on_residual(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}
//   onSetAbility(ability, target, source, effect)

/// Dispatch onSetAbility callbacks
pub fn dispatch_on_set_ability(
    battle: &mut Battle,
    item_id: &str,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    effect_id: Option<&str>,
) -> EventResult {
    match item_id {
        "abilityshield" => abilityshield::on_set_ability(battle, target_pos, source_pos, effect_id),
        _ => EventResult::Continue,
    }
}
//   onSourceModifyAccuracy(accuracy)
//   onSourceModifyAccuracy(accuracy, target)
//   onSourceModifyAccuracy(accuracy, target, source)
//   onSourceModifyAccuracy(accuracy, target, source, move)

/// Dispatch onSourceModifyAccuracy callbacks
pub fn dispatch_on_source_modify_accuracy(
    battle: &mut Battle,
    item_id: &str,
    accuracy: i32,
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    match item_id {
        "widelens" => widelens::on_source_modify_accuracy(battle, accuracy, target_pos),
        "zoomlens" => zoomlens::on_source_modify_accuracy(battle, accuracy, target_pos),
        _ => EventResult::Continue,
    }
}
//   onSourceModifyDamage()
//   onSourceModifyDamage(damage, source, target, move)

/// Dispatch onSourceModifyDamage callbacks
pub fn dispatch_on_source_modify_damage(
    battle: &mut Battle,
    item_id: &str,
    damage: i32,
    source_pos: (usize, usize),
    target_pos: (usize, usize),
) -> EventResult {
    match item_id {
        "babiriberry" => babiriberry::on_source_modify_damage(battle, damage, source_pos, target_pos),
        "chartiberry" => chartiberry::on_source_modify_damage(battle, damage, source_pos, target_pos),
        "chilanberry" => chilanberry::on_source_modify_damage(battle, damage, source_pos, target_pos),
        "chopleberry" => chopleberry::on_source_modify_damage(battle, damage, source_pos, target_pos),
        "cobaberry" => cobaberry::on_source_modify_damage(battle, damage, source_pos, target_pos),
        "colburberry" => colburberry::on_source_modify_damage(battle, damage, source_pos, target_pos),
        "habanberry" => habanberry::on_source_modify_damage(battle, damage, source_pos, target_pos),
        "kasibberry" => kasibberry::on_source_modify_damage(battle, damage, source_pos, target_pos),
        "kebiaberry" => kebiaberry::on_source_modify_damage(battle, damage, source_pos, target_pos),
        "occaberry" => occaberry::on_source_modify_damage(battle, damage, source_pos, target_pos),
        "passhoberry" => passhoberry::on_source_modify_damage(battle, damage, source_pos, target_pos),
        "payapaberry" => payapaberry::on_source_modify_damage(battle, damage, source_pos, target_pos),
        "rindoberry" => rindoberry::on_source_modify_damage(battle, damage, source_pos, target_pos),
        "roseliberry" => roseliberry::on_source_modify_damage(battle, damage, source_pos, target_pos),
        "shucaberry" => shucaberry::on_source_modify_damage(battle, damage, source_pos, target_pos),
        "tangaberry" => tangaberry::on_source_modify_damage(battle, damage, source_pos, target_pos),
        "wacanberry" => wacanberry::on_source_modify_damage(battle, damage, source_pos, target_pos),
        "yacheberry" => yacheberry::on_source_modify_damage(battle, damage, source_pos, target_pos),
        _ => EventResult::Continue,
    }
}
//   onSourceTryPrimaryHit(target, source, effect)
//   onSourceTryPrimaryHit(target, source, move)

/// Dispatch onSourceTryPrimaryHit callbacks
pub fn dispatch_on_source_try_primary_hit(
    battle: &mut Battle,
    item_id: &str,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    active_move: Option<&ActiveMove>,
) -> EventResult {
    match item_id {
        "buggem" => buggem::on_source_try_primary_hit(battle, target_pos, source_pos, active_move),
        "darkgem" => darkgem::on_source_try_primary_hit(battle, target_pos, source_pos, active_move),
        "dragongem" => dragongem::on_source_try_primary_hit(battle, target_pos, source_pos, active_move),
        "electricgem" => electricgem::on_source_try_primary_hit(battle, target_pos, source_pos, active_move),
        "fairygem" => fairygem::on_source_try_primary_hit(battle, target_pos, source_pos, active_move),
        "fightinggem" => fightinggem::on_source_try_primary_hit(battle, target_pos, source_pos, active_move),
        "firegem" => firegem::on_source_try_primary_hit(battle, target_pos, source_pos, active_move),
        "flyinggem" => flyinggem::on_source_try_primary_hit(battle, target_pos, source_pos, active_move),
        "ghostgem" => ghostgem::on_source_try_primary_hit(battle, target_pos, source_pos, active_move),
        "grassgem" => grassgem::on_source_try_primary_hit(battle, target_pos, source_pos, active_move),
        "groundgem" => groundgem::on_source_try_primary_hit(battle, target_pos, source_pos, active_move),
        "icegem" => icegem::on_source_try_primary_hit(battle, target_pos, source_pos, active_move),
        "normalgem" => normalgem::on_source_try_primary_hit(battle, target_pos, source_pos, active_move),
        "poisongem" => poisongem::on_source_try_primary_hit(battle, target_pos, source_pos, active_move),
        "psychicgem" => psychicgem::on_source_try_primary_hit(battle, target_pos, source_pos, active_move),
        "rockgem" => rockgem::on_source_try_primary_hit(battle, target_pos, source_pos, active_move),
        "steelgem" => steelgem::on_source_try_primary_hit(battle, target_pos, source_pos, active_move),
        "watergem" => watergem::on_source_try_primary_hit(battle, target_pos, source_pos, active_move),
        _ => EventResult::Continue,
    }
}
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
    item_id: &str,
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    match item_id {
        "airballoon" => airballoon::on_start(battle, target_pos),
        "boosterenergy" => boosterenergy::on_start(battle, target_pos),
        "choiceband" => choiceband::on_start(battle, target_pos),
        "choicescarf" => choicescarf::on_start(battle, target_pos),
        "choicespecs" => choicespecs::on_start(battle, target_pos),
        "electricseed" => electricseed::on_start(battle, target_pos),
        "grassyseed" => grassyseed::on_start(battle, target_pos),
        "metronome" => metronome::on_start(battle, target_pos),
        "mistyseed" => mistyseed::on_start(battle, target_pos),
        "psychicseed" => psychicseed::on_start(battle, target_pos),
        "roomservice" => roomservice::on_start(battle, target_pos),
        "utilityumbrella" => utilityumbrella::on_start(battle, target_pos),
        "whiteherb" => whiteherb::on_start(battle, target_pos),
        _ => EventResult::Continue,
    }
}
//   onSwitchIn()
//   onSwitchIn(pokemon)
//   onSwitchIn(target)

/// Dispatch onSwitchIn callbacks
pub fn dispatch_on_switch_in(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match item_id {
        "blueorb" => blueorb::on_switch_in(battle, pokemon_pos),
        "redorb" => redorb::on_switch_in(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}
//   onTakeItem(item)
//   onTakeItem(item, pokemon)
//   onTakeItem(item, pokemon, source)
//   onTakeItem(item, source)

/// Dispatch onTakeItem callbacks
pub fn dispatch_on_take_item(
    battle: &mut Battle,
    item_id: &str,
    item_pos: Option<(usize, usize)>,
    pokemon_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    match item_id {
        "abomasite" => abomasite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "absolite" => absolite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "absolitez" => absolitez::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "adamantcrystal" => adamantcrystal::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "aerodactylite" => aerodactylite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "aggronite" => aggronite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "alakazite" => alakazite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "altarianite" => altarianite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "ampharosite" => ampharosite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "audinite" => audinite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "banettite" => banettite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "barbaracite" => barbaracite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "baxcalibrite" => baxcalibrite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "beedrillite" => beedrillite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "blastoisinite" => blastoisinite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "blazikenite" => blazikenite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "blueorb" => blueorb::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "boosterenergy" => boosterenergy::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "bugmemory" => bugmemory::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "burndrive" => burndrive::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "cameruptite" => cameruptite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "chandelurite" => chandelurite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "charizarditex" => charizarditex::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "charizarditey" => charizarditey::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "chesnaughtite" => chesnaughtite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "chilldrive" => chilldrive::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "chimechite" => chimechite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "clefablite" => clefablite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "cornerstonemask" => cornerstonemask::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "crabominite" => crabominite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "crucibellite" => crucibellite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "darkmemory" => darkmemory::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "darkranite" => darkranite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "delphoxite" => delphoxite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "diancite" => diancite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "dousedrive" => dousedrive::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "dracoplate" => dracoplate::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "dragalgite" => dragalgite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "dragoninite" => dragoninite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "dragonmemory" => dragonmemory::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "drampanite" => drampanite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "dreadplate" => dreadplate::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "earthplate" => earthplate::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "eelektrossite" => eelektrossite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "electricmemory" => electricmemory::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "emboarite" => emboarite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "excadrite" => excadrite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "fairymemory" => fairymemory::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "falinksite" => falinksite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "feraligite" => feraligite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "fightingmemory" => fightingmemory::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "firememory" => firememory::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "fistplate" => fistplate::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "flameplate" => flameplate::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "floettite" => floettite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "flyingmemory" => flyingmemory::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "froslassite" => froslassite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "galladite" => galladite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "garchompite" => garchompite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "garchompitez" => garchompitez::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "gardevoirite" => gardevoirite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "gengarite" => gengarite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "ghostmemory" => ghostmemory::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "glalitite" => glalitite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "glimmoranite" => glimmoranite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "golisopite" => golisopite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "golurkite" => golurkite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "grassmemory" => grassmemory::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "greninjite" => greninjite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "griseouscore" => griseouscore::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "groundmemory" => groundmemory::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "gyaradosite" => gyaradosite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "hawluchanite" => hawluchanite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "hearthflamemask" => hearthflamemask::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "heatranite" => heatranite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "heracronite" => heracronite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "houndoominite" => houndoominite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "icememory" => icememory::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "icicleplate" => icicleplate::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "insectplate" => insectplate::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "ironplate" => ironplate::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "kangaskhanite" => kangaskhanite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "latiasite" => latiasite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "latiosite" => latiosite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "lopunnite" => lopunnite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "lucarionite" => lucarionite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "lucarionitez" => lucarionitez::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "lustrousglobe" => lustrousglobe::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "magearnite" => magearnite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "mail" => mail::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "malamarite" => malamarite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "manectite" => manectite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "mawilite" => mawilite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "meadowplate" => meadowplate::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "medichamite" => medichamite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "meganiumite" => meganiumite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "meowsticite" => meowsticite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "metagrossite" => metagrossite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "mewtwonitex" => mewtwonitex::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "mewtwonitey" => mewtwonitey::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "mindplate" => mindplate::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "pidgeotite" => pidgeotite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "pinsirite" => pinsirite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "pixieplate" => pixieplate::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "poisonmemory" => poisonmemory::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "psychicmemory" => psychicmemory::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "pyroarite" => pyroarite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "raichunitex" => raichunitex::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "raichunitey" => raichunitey::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "redorb" => redorb::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "rockmemory" => rockmemory::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "rustedshield" => rustedshield::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "rustedsword" => rustedsword::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "sablenite" => sablenite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "salamencite" => salamencite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "sceptilite" => sceptilite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "scizorite" => scizorite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "scolipite" => scolipite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "scovillainite" => scovillainite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "scraftinite" => scraftinite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "sharpedonite" => sharpedonite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "shockdrive" => shockdrive::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "skarmorite" => skarmorite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "skyplate" => skyplate::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "slowbronite" => slowbronite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "splashplate" => splashplate::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "spookyplate" => spookyplate::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "staraptite" => staraptite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "starminite" => starminite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "steelixite" => steelixite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "steelmemory" => steelmemory::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "stoneplate" => stoneplate::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "swampertite" => swampertite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "tatsugirinite" => tatsugirinite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "toxicplate" => toxicplate::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "tyranitarite" => tyranitarite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "venusaurite" => venusaurite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "victreebelite" => victreebelite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "vilevial" => vilevial::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "watermemory" => watermemory::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "wellspringmask" => wellspringmask::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "zapplate" => zapplate::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "zeraorite" => zeraorite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        "zygardite" => zygardite::on_take_item(battle, item_pos, pokemon_pos, source_pos),
        _ => {
            // Z-crystals (items with zMove) cannot be removed
            // Check if this item is a Z-crystal by looking up its z_move field
            if let Some(item_data) = battle.dex.items().get_by_id(&item_id.into()) {
                if item_data.z_move.is_some() {
                    return EventResult::Boolean(false);
                }
            }
            EventResult::Continue
        }
    }
}
//   onTerrainChange(pokemon)
//   onTerrainChange(target, source, sourceEffect)

/// Dispatch onTerrainChange callbacks
pub fn dispatch_on_terrain_change(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match item_id {
        "electricseed" => electricseed::on_terrain_change(battle, pokemon_pos),
        "grassyseed" => grassyseed::on_terrain_change(battle, pokemon_pos),
        "mistyseed" => mistyseed::on_terrain_change(battle, pokemon_pos),
        "psychicseed" => psychicseed::on_terrain_change(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}
//   onTrapPokemon(pokemon)

/// Dispatch onTrapPokemon callbacks
pub fn dispatch_on_trap_pokemon(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match item_id {
        "shedshell" => shedshell::on_trap_pokemon(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}
//   onTryBoost()
//   onTryBoost(boost, target, source, effect)

/// Dispatch onTryBoost callbacks
pub fn dispatch_on_try_boost(
    battle: &mut Battle,
    item_id: &str,
    target_pos: (usize, usize),
    boost: &mut crate::dex_data::BoostsTable,
) -> EventResult {
    match item_id {
        "clearamulet" => clearamulet::on_try_boost(battle, target_pos, boost),
        _ => EventResult::Continue,
    }
}
//   onTryEatItem(item)
//   onTryEatItem(item, pokemon)

/// Dispatch onTryEatItem callbacks
pub fn dispatch_on_try_eat_item(
    battle: &mut Battle,
    item_id: &str,
    item: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match item_id {
        "aguavberry" => aguavberry::on_try_eat_item(battle, item, pokemon_pos),
        "berry" => berry::on_try_eat_item(battle, item, pokemon_pos),
        "enigmaberry" => enigmaberry::on_try_eat_item(battle, item, pokemon_pos),
        "figyberry" => figyberry::on_try_eat_item(battle, item, pokemon_pos),
        "goldberry" => goldberry::on_try_eat_item(battle, item, pokemon_pos),
        "iapapaberry" => iapapaberry::on_try_eat_item(battle, item, pokemon_pos),
        "magoberry" => magoberry::on_try_eat_item(battle, item, pokemon_pos),
        "oranberry" => oranberry::on_try_eat_item(battle, item, pokemon_pos),
        "sitrusberry" => sitrusberry::on_try_eat_item(battle, item, pokemon_pos),
        "wikiberry" => wikiberry::on_try_eat_item(battle, item, pokemon_pos),
        _ => EventResult::Continue,
    }
}
//   onTryHeal(damage, pokemon, source, effect)
//   onTryHeal(damage, target, source, effect)

/// Dispatch onTryHeal callbacks
pub fn dispatch_on_try_heal(
    battle: &mut Battle,
    item_id: &str,
    damage: i32,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    effect_id: Option<&str>,
) -> EventResult {
    match item_id {
        "bigroot" => bigroot::on_try_heal(battle, damage, target_pos, source_pos, effect_id),
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
pub fn dispatch_on_try_hit(
    battle: &mut Battle,
    item_id: &str,
    target_pos: (usize, usize),
    source_pos: (usize, usize),
) -> EventResult {
    match item_id {
        "safetygoggles" => safetygoggles::on_try_hit(battle, target_pos, source_pos),
        _ => EventResult::Continue,
    }
}
//   onUpdate()
//   onUpdate(pokemon)

/// Dispatch onUpdate callbacks
pub fn dispatch_on_update(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match item_id {
        "aguavberry" => aguavberry::on_update(battle, pokemon_pos),
        "apicotberry" => apicotberry::on_update(battle, pokemon_pos),
        "aspearberry" => aspearberry::on_update(battle, pokemon_pos),
        "berryjuice" => berryjuice::on_update(battle, pokemon_pos),
        "berserkgene" => berserkgene::on_update(battle, pokemon_pos),
        "bitterberry" => bitterberry::on_update(battle, pokemon_pos),
        "boosterenergy" => boosterenergy::on_update(battle, pokemon_pos),
        "burntberry" => burntberry::on_update(battle, pokemon_pos),
        "cheriberry" => cheriberry::on_update(battle, pokemon_pos),
        "chestoberry" => chestoberry::on_update(battle, pokemon_pos),
        "figyberry" => figyberry::on_update(battle, pokemon_pos),
        "ganlonberry" => ganlonberry::on_update(battle, pokemon_pos),
        "iapapaberry" => iapapaberry::on_update(battle, pokemon_pos),
        "iceberry" => iceberry::on_update(battle, pokemon_pos),
        "lansatberry" => lansatberry::on_update(battle, pokemon_pos),
        "leppaberry" => leppaberry::on_update(battle, pokemon_pos),
        "liechiberry" => liechiberry::on_update(battle, pokemon_pos),
        "lumberry" => lumberry::on_update(battle, pokemon_pos),
        "magoberry" => magoberry::on_update(battle, pokemon_pos),
        "mentalherb" => mentalherb::on_update(battle, pokemon_pos),
        "mintberry" => mintberry::on_update(battle, pokemon_pos),
        "miracleberry" => miracleberry::on_update(battle, pokemon_pos),
        "mysteryberry" => mysteryberry::on_update(battle, pokemon_pos),
        "oranberry" => oranberry::on_update(battle, pokemon_pos),
        "pechaberry" => pechaberry::on_update(battle, pokemon_pos),
        "persimberry" => persimberry::on_update(battle, pokemon_pos),
        "petayaberry" => petayaberry::on_update(battle, pokemon_pos),
        "przcureberry" => przcureberry::on_update(battle, pokemon_pos),
        "psncureberry" => psncureberry::on_update(battle, pokemon_pos),
        "rawstberry" => rawstberry::on_update(battle, pokemon_pos),
        "salacberry" => salacberry::on_update(battle, pokemon_pos),
        "sitrusberry" => sitrusberry::on_update(battle, pokemon_pos),
        "starfberry" => starfberry::on_update(battle, pokemon_pos),
        "utilityumbrella" => utilityumbrella::on_update(battle, pokemon_pos),
        "wikiberry" => wikiberry::on_update(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}
//   onUse(pokemon)

/// Dispatch onUse callbacks
pub fn dispatch_on_use(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match item_id {
        "ejectpack" => ejectpack::on_use(battle, pokemon_pos),
        "mirrorherb" => mirrorherb::on_use(battle, pokemon_pos),
        "whiteherb" => whiteherb::on_use(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}
//   onUseItem(item, pokemon)

/// Dispatch onUseItem callbacks
pub fn dispatch_on_use_item(
    battle: &mut Battle,
    item_id: &str,
    item: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match item_id {
        "ejectpack" => ejectpack::on_use_item(battle, item, pokemon_pos),
        _ => EventResult::Continue,
    }
}

