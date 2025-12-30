//! Item Callback Handlers
//!
//! This module provides dispatch functions for item callbacks.
//! Individual item implementations will be added as needed.

use crate::battle::Battle;
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
// DISPATCH FUNCTIONS
//
// These functions route item events to specific item implementations.
// They return EventResult directly, with EventResult::Continue for no match.
// =========================================================================

/// Dispatch onAfterBoost callbacks
pub fn dispatch_on_after_boost(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
    boost: Option<&crate::dex_data::BoostsTable>,
) -> EventResult {
    use crate::dex_data::ID;
    match ID::from(item_id).as_str() {
        "adrenalineorb" => {
            if let Some(boost_table) = boost {
                adrenalineorb::on_after_boost(battle, pokemon_pos, boost_table)
            } else {
                EventResult::Continue
            }
        }
        "ejectpack" => {
            if let Some(boost_table) = boost {
                ejectpack::on_after_boost(battle, pokemon_pos, boost_table)
            } else {
                EventResult::Continue
            }
        }
        _ => EventResult::Continue,
    }
}

/// Dispatch onAfterMoveSecondary callbacks
pub fn dispatch_on_after_move_secondary(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onAfterMoveSecondaryPriority callbacks
pub fn dispatch_on_after_move_secondary_priority(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onAfterMoveSecondarySelf callbacks
pub fn dispatch_on_after_move_secondary_self(
    _battle: &mut Battle,
    _item_id: &str,
    _source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
    _move_id: &str,
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onAfterMoveSecondarySelfPriority callbacks
pub fn dispatch_on_after_move_secondary_self_priority(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onAfterSetStatus callbacks
pub fn dispatch_on_after_set_status(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onAfterSetStatusPriority callbacks
pub fn dispatch_on_after_set_status_priority(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onAfterSubDamage callbacks
pub fn dispatch_on_after_sub_damage(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onAnyAfterMega callbacks
pub fn dispatch_on_any_after_mega(
    battle: &mut Battle,
    item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;
    match ID::from(item_id).as_str() {
        "ejectpack" => ejectpack::on_any_after_mega(battle),
        "mirrorherb" => mirrorherb::on_any_after_mega(battle),
        "whiteherb" => whiteherb::on_any_after_mega(battle),
        _ => EventResult::Continue,
    }
}

/// Dispatch onAnyAfterMove callbacks
pub fn dispatch_on_any_after_move(
    battle: &mut Battle,
    item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;
    match ID::from(item_id).as_str() {
        "ejectpack" => ejectpack::on_any_after_move(battle),
        "mirrorherb" => mirrorherb::on_any_after_move(battle),
        "whiteherb" => whiteherb::on_any_after_move(battle),
        _ => EventResult::Continue,
    }
}

/// Dispatch onAnyAfterTerastallization callbacks
pub fn dispatch_on_any_after_terastallization(
    battle: &mut Battle,
    item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;
    match ID::from(item_id).as_str() {
        "mirrorherb" => mirrorherb::on_any_after_terastallization(battle),
        _ => EventResult::Continue,
    }
}

/// Dispatch onAnyPseudoWeatherChange callbacks
pub fn dispatch_on_any_pseudo_weather_change(
    battle: &mut Battle,
    item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;
    match ID::from(item_id).as_str() {
        "roomservice" => roomservice::on_any_pseudo_weather_change(battle),
        _ => EventResult::Continue,
    }
}

/// Dispatch onAnySwitchIn callbacks
pub fn dispatch_on_any_switch_in(
    battle: &mut Battle,
    item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;
    match ID::from(item_id).as_str() {
        "ejectpack" => ejectpack::on_any_switch_in(battle),
        "mirrorherb" => mirrorherb::on_any_switch_in(battle),
        "whiteherb" => whiteherb::on_any_switch_in(battle),
        _ => EventResult::Continue,
    }
}

/// Dispatch onAnySwitchInPriority callbacks
pub fn dispatch_on_any_switch_in_priority(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onAttract callbacks
pub fn dispatch_on_attract(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onAttractPriority callbacks
pub fn dispatch_on_attract_priority(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onBasePower callbacks
pub fn dispatch_on_base_power(
    _battle: &mut Battle,
    _item_id: &str,
    _base_power: i32,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
    _move_id: &str,
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onBasePowerPriority callbacks
pub fn dispatch_on_base_power_priority(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onChargeMove callbacks
pub fn dispatch_on_charge_move(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;

    // Get move_id and target from active_move and current_event
    let (move_id, target_pos) = {
        let move_id = battle.active_move.as_ref().map(|m| m.id.clone()).unwrap_or_default();
        let target_pos = battle.current_event.as_ref().and_then(|e| e.target);
        (move_id, target_pos)
    };

    match ID::from(item_id).as_str() {
        "powerherb" => powerherb::on_charge_move(battle, pokemon_pos, target_pos, move_id.as_str()),
        _ => EventResult::Continue,
    }
}

/// Dispatch onDamage callbacks
pub fn dispatch_on_damage(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onDamagePriority callbacks
pub fn dispatch_on_damage_priority(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onDamagingHit callbacks
pub fn dispatch_on_damaging_hit(
    _battle: &mut Battle,
    _item_id: &str,
    _damage: i32,
    _target_pos: (usize, usize),
    _source_pos: (usize, usize),
    _move_id: &str,
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onDamagingHitOrder callbacks
pub fn dispatch_on_damaging_hit_order(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onDisableMove callbacks
pub fn dispatch_on_disable_move(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onDrive callbacks
pub fn dispatch_on_drive(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onEat callbacks
pub fn dispatch_on_eat(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onEffectiveness callbacks
pub fn dispatch_on_effectiveness(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;
    match ID::from(item_id).as_str() {
        "ironball" => ironball::on_effectiveness(battle, Some(pokemon_pos)),
        _ => EventResult::Continue,
    }
}

/// Dispatch onEnd callbacks
pub fn dispatch_on_end(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;
    match ID::from(item_id).as_str() {
        "ejectpack" => ejectpack::on_end(battle, pokemon_pos),
        "mirrorherb" => mirrorherb::on_end(battle, pokemon_pos),
        "utilityumbrella" => utilityumbrella::on_end(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onFoeAfterBoost callbacks
pub fn dispatch_on_foe_after_boost(
    battle: &mut Battle,
    item_id: &str,
    _pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    effect_id: Option<&str>,
    boost: Option<&crate::dex_data::BoostsTable>,
) -> EventResult {
    use crate::dex_data::ID;
    match ID::from(item_id).as_str() {
        "mirrorherb" => {
            if let Some(boost_table) = boost {
                mirrorherb::on_foe_after_boost(battle, target_pos, source_pos, effect_id, boost_table)
            } else {
                EventResult::Continue
            }
        }
        _ => EventResult::Continue,
    }
}

/// Dispatch onFractionalPriority callbacks
pub fn dispatch_on_fractional_priority(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
    priority: f64,
) -> EventResult {
    use crate::dex_data::ID;
    match ID::from(item_id).as_str() {
        "custapberry" => custapberry::on_fractional_priority(battle, pokemon_pos, priority),
        "quickclaw" => quickclaw::on_fractional_priority(battle, pokemon_pos, priority),
        _ => EventResult::Continue,
    }
}

/// Dispatch onFractionalPriorityPriority callbacks
pub fn dispatch_on_fractional_priority_priority(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onHit callbacks
pub fn dispatch_on_hit(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onImmunity callbacks
pub fn dispatch_on_immunity(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
    immunity_type: Option<&str>,
) -> EventResult {
    use crate::dex_data::ID;
    match ID::from(item_id).as_str() {
        "safetygoggles" => {
            if let Some(type_str) = immunity_type {
                safetygoggles::on_immunity(battle, pokemon_pos, type_str)
            } else {
                EventResult::Continue
            }
        }
        _ => EventResult::Continue,
    }
}

/// Dispatch onMaybeTrapPokemon callbacks
pub fn dispatch_on_maybe_trap_pokemon(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onMaybeTrapPokemonPriority callbacks
pub fn dispatch_on_maybe_trap_pokemon_priority(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onMemory callbacks
pub fn dispatch_on_memory(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onModifyAccuracy callbacks
pub fn dispatch_on_modify_accuracy(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onModifyAccuracyPriority callbacks
pub fn dispatch_on_modify_accuracy_priority(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onModifyAtk callbacks
pub fn dispatch_on_modify_atk(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onModifyAtkPriority callbacks
pub fn dispatch_on_modify_atk_priority(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onModifyCritRatio callbacks
pub fn dispatch_on_modify_crit_ratio(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
    crit_ratio: i32,
) -> EventResult {
    use crate::dex_data::ID;
    match ID::from(item_id).as_str() {
        "leek" => leek::on_modify_crit_ratio(battle, pokemon_pos, crit_ratio),
        "luckypunch" => luckypunch::on_modify_crit_ratio(battle, crit_ratio),
        "razorclaw" => razorclaw::on_modify_crit_ratio(battle, crit_ratio),
        "scopelens" => scopelens::on_modify_crit_ratio(battle, crit_ratio),
        "stick" => stick::on_modify_crit_ratio(battle, pokemon_pos, crit_ratio),
        _ => EventResult::Continue,
    }
}

/// Dispatch onModifyDamage callbacks
pub fn dispatch_on_modify_damage(
    _battle: &mut Battle,
    _item_id: &str,
    _damage: i32,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
    _move_id: &str,
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onModifyDef callbacks
pub fn dispatch_on_modify_def(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onModifyDefPriority callbacks
pub fn dispatch_on_modify_def_priority(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onModifyMove callbacks
pub fn dispatch_on_modify_move(
    _battle: &mut Battle,
    _item_id: &str,
    _move_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onModifyMovePriority callbacks
pub fn dispatch_on_modify_move_priority(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onModifySecondaries callbacks
pub fn dispatch_on_modify_secondaries(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
    secondaries: Option<&mut Vec<crate::battle_actions::SecondaryEffect>>,
) -> EventResult {
    use crate::dex_data::ID;
    match ID::from(item_id).as_str() {
        "covertcloak" => {
            if let Some(sec) = secondaries {
                covertcloak::on_modify_secondaries(battle, pokemon_pos, sec)
            } else {
                EventResult::Continue
            }
        }
        _ => EventResult::Continue,
    }
}

/// Dispatch onModifySpA callbacks
pub fn dispatch_on_modify_sp_a(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onModifySpAPriority callbacks
pub fn dispatch_on_modify_sp_a_priority(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onModifySpD callbacks
pub fn dispatch_on_modify_sp_d(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onModifySpDPriority callbacks
pub fn dispatch_on_modify_sp_d_priority(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onModifySpe callbacks
pub fn dispatch_on_modify_spe(
    battle: &mut Battle,
    item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;
    match ID::from(item_id).as_str() {
        "ironball" => ironball::on_modify_spe(battle),
        _ => EventResult::Continue,
    }
}

/// Dispatch onModifyWeight callbacks
pub fn dispatch_on_modify_weight(
    battle: &mut Battle,
    item_id: &str,
    _pokemon_pos: (usize, usize),
    weighthg: i32,
) -> EventResult {
    use crate::dex_data::ID;
    match ID::from(item_id).as_str() {
        "floatstone" => floatstone::on_modify_weight(battle, weighthg),
        _ => EventResult::Continue,
    }
}

/// Dispatch onNegateImmunity callbacks
pub fn dispatch_on_negate_immunity(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onPlate callbacks
pub fn dispatch_on_plate(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onResidual callbacks
pub fn dispatch_on_residual(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;
    match ID::from(item_id).as_str() {
        "ejectpack" => ejectpack::on_residual(battle, pokemon_pos),
        "mirrorherb" => mirrorherb::on_residual(battle, pokemon_pos),
        "whiteherb" => whiteherb::on_residual(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onResidualOrder callbacks
pub fn dispatch_on_residual_order(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onResidualSubOrder callbacks
pub fn dispatch_on_residual_sub_order(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onSetAbility callbacks
pub fn dispatch_on_set_ability(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onSourceModifyAccuracy callbacks
pub fn dispatch_on_source_modify_accuracy(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onSourceModifyAccuracyPriority callbacks
pub fn dispatch_on_source_modify_accuracy_priority(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onSourceModifyDamage callbacks
pub fn dispatch_on_source_modify_damage(
    _battle: &mut Battle,
    _item_id: &str,
    _damage: i32,
    _source_pos: (usize, usize),
    _target_pos: (usize, usize),
    _move_id: &str,
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onSourceTryPrimaryHit callbacks
pub fn dispatch_on_source_try_primary_hit(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onStart callbacks
pub fn dispatch_on_start(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;
    match ID::from(item_id).as_str() {
        "boosterenergy" => boosterenergy::on_start(battle, Some(pokemon_pos)),
        "roomservice" => roomservice::on_start(battle, Some(pokemon_pos)),
        "utilityumbrella" => utilityumbrella::on_start(battle, Some(pokemon_pos)),
        _ => EventResult::Continue,
    }
}

/// Dispatch onSwitchIn callbacks
pub fn dispatch_on_switch_in(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;
    match ID::from(item_id).as_str() {
        "blueorb" => blueorb::on_switch_in(battle, pokemon_pos),
        "redorb" => redorb::on_switch_in(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onSwitchInPriority callbacks
pub fn dispatch_on_switch_in_priority(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onTakeItem callbacks
pub fn dispatch_on_take_item(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;
    match ID::from(item_id).as_str() {
        "boosterenergy" => boosterenergy::on_take_item(battle, Some(pokemon_pos), pokemon_pos, source_pos),
        "blueorb" => blueorb::on_take_item(battle, Some(pokemon_pos), pokemon_pos, source_pos),
        "redorb" => redorb::on_take_item(battle, Some(pokemon_pos), pokemon_pos, source_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onTerrainChange callbacks
pub fn dispatch_on_terrain_change(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onTrapPokemon callbacks
pub fn dispatch_on_trap_pokemon(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onTrapPokemonPriority callbacks
pub fn dispatch_on_trap_pokemon_priority(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onTryBoost callbacks
pub fn dispatch_on_try_boost(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
    boost: Option<&mut crate::dex_data::BoostsTable>,
) -> EventResult {
    use crate::dex_data::ID;
    match ID::from(item_id).as_str() {
        "clearamulet" => {
            if let Some(boost_table) = boost {
                clearamulet::on_try_boost(battle, pokemon_pos, boost_table)
            } else {
                EventResult::Continue
            }
        }
        _ => EventResult::Continue,
    }
}

/// Dispatch onTryBoostPriority callbacks
pub fn dispatch_on_try_boost_priority(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onTryEatItem callbacks
pub fn dispatch_on_try_eat_item(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onTryHeal callbacks
pub fn dispatch_on_try_heal(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onTryHealPriority callbacks
pub fn dispatch_on_try_heal_priority(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onTryHit callbacks
pub fn dispatch_on_try_hit(
    battle: &mut Battle,
    item_id: &str,
    target_pos: (usize, usize),
    source_pos: (usize, usize),
    _move_id: &str,
) -> EventResult {
    use crate::dex_data::ID;
    match ID::from(item_id).as_str() {
        "safetygoggles" => safetygoggles::on_try_hit(battle, target_pos, source_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onUpdate callbacks
pub fn dispatch_on_update(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;
    match ID::from(item_id).as_str() {
        "boosterenergy" => boosterenergy::on_update(battle, pokemon_pos),
        "utilityumbrella" => utilityumbrella::on_update(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onUse callbacks
pub fn dispatch_on_use(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;
    match ID::from(item_id).as_str() {
        "ejectpack" => ejectpack::on_use(battle, pokemon_pos),
        "mirrorherb" => mirrorherb::on_use(battle, pokemon_pos),
        "whiteherb" => whiteherb::on_use(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onUseItem callbacks
pub fn dispatch_on_use_item(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;
    match ID::from(item_id).as_str() {
        "ejectpack" => ejectpack::on_use_item(battle, item_id, pokemon_pos),
        _ => EventResult::Continue,
    }
}
