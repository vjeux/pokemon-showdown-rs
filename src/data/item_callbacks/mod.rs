//! Item Callback Handlers
//!
//! This module exports all item callback implementations.
//! Each item with callbacks is in its own file.

// Common types
mod common;
pub use common::*;

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
pub mod aloraichiumz;
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
pub mod belueberry;
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
pub mod blukberry;
pub mod boosterenergy;
pub mod brightpowder;
pub mod buggem;
pub mod buginiumz;
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
pub mod cornnberry;
pub mod covertcloak;
pub mod crabominite;
pub mod crucibellite;
pub mod custapberry;
pub mod darkgem;
pub mod darkiniumz;
pub mod darkmemory;
pub mod darkranite;
pub mod decidiumz;
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
pub mod dragoniumz;
pub mod dragonmemory;
pub mod drampanite;
pub mod dreadplate;
pub mod durinberry;
pub mod earthplate;
pub mod eelektrossite;
pub mod eeviumz;
pub mod ejectbutton;
pub mod ejectpack;
pub mod electricgem;
pub mod electricmemory;
pub mod electricseed;
pub mod electriumz;
pub mod emboarite;
pub mod enigmaberry;
pub mod eviolite;
pub mod excadrite;
pub mod expertbelt;
pub mod fairiumz;
pub mod fairyfeather;
pub mod fairygem;
pub mod fairymemory;
pub mod falinksite;
pub mod feraligite;
pub mod fightinggem;
pub mod fightingmemory;
pub mod fightiniumz;
pub mod figyberry;
pub mod firegem;
pub mod firememory;
pub mod firiumz;
pub mod fistplate;
pub mod flameorb;
pub mod flameplate;
pub mod floatstone;
pub mod floettite;
pub mod flyinggem;
pub mod flyingmemory;
pub mod flyiniumz;
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
pub mod ghostiumz;
pub mod ghostmemory;
pub mod glalitite;
pub mod glimmoranite;
pub mod goldberry;
pub mod golisopite;
pub mod golurkite;
pub mod grassgem;
pub mod grassiumz;
pub mod grassmemory;
pub mod grassyseed;
pub mod greninjite;
pub mod grepaberry;
pub mod griseouscore;
pub mod griseousorb;
pub mod groundgem;
pub mod groundiumz;
pub mod groundmemory;
pub mod gyaradosite;
pub mod habanberry;
pub mod hardstone;
pub mod hawluchanite;
pub mod hearthflamemask;
pub mod heatranite;
pub mod heracronite;
pub mod hondewberry;
pub mod houndoominite;
pub mod iapapaberry;
pub mod iceberry;
pub mod icegem;
pub mod icememory;
pub mod icicleplate;
pub mod iciumz;
pub mod inciniumz;
pub mod insectplate;
pub mod ironball;
pub mod ironplate;
pub mod jabocaberry;
pub mod kangaskhanite;
pub mod kasibberry;
pub mod kebiaberry;
pub mod keeberry;
pub mod kelpsyberry;
pub mod kingsrock;
pub mod kommoniumz;
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
pub mod lunaliumz;
pub mod lustrousglobe;
pub mod lustrousorb;
pub mod lycaniumz;
pub mod machobrace;
pub mod magearnite;
pub mod magnet;
pub mod magoberry;
pub mod magostberry;
pub mod mail;
pub mod malamarite;
pub mod manectite;
pub mod marangaberry;
pub mod marshadiumz;
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
pub mod mewniumz;
pub mod mewtwonitex;
pub mod mewtwonitey;
pub mod micleberry;
pub mod mimikiumz;
pub mod mindplate;
pub mod mintberry;
pub mod miracleberry;
pub mod miracleseed;
pub mod mirrorherb;
pub mod mistyseed;
pub mod muscleband;
pub mod mysteryberry;
pub mod mysticwater;
pub mod nanabberry;
pub mod nevermeltice;
pub mod nomelberry;
pub mod normalgem;
pub mod normaliumz;
pub mod occaberry;
pub mod oddincense;
pub mod oranberry;
pub mod pamtreberry;
pub mod passhoberry;
pub mod payapaberry;
pub mod pechaberry;
pub mod persimberry;
pub mod petayaberry;
pub mod pidgeotite;
pub mod pikaniumz;
pub mod pikashuniumz;
pub mod pinapberry;
pub mod pinkbow;
pub mod pinsirite;
pub mod pixieplate;
pub mod poisonbarb;
pub mod poisongem;
pub mod poisoniumz;
pub mod poisonmemory;
pub mod polkadotbow;
pub mod pomegberry;
pub mod poweranklet;
pub mod powerband;
pub mod powerbelt;
pub mod powerbracer;
pub mod powerherb;
pub mod powerlens;
pub mod powerweight;
pub mod primariumz;
pub mod przcureberry;
pub mod psncureberry;
pub mod psychicgem;
pub mod psychicmemory;
pub mod psychicseed;
pub mod psychiumz;
pub mod punchingglove;
pub mod pyroarite;
pub mod qualotberry;
pub mod quickclaw;
pub mod quickpowder;
pub mod rabutaberry;
pub mod raichunitex;
pub mod raichunitey;
pub mod rawstberry;
pub mod razorclaw;
pub mod razorfang;
pub mod razzberry;
pub mod redcard;
pub mod redorb;
pub mod rindoberry;
pub mod ringtarget;
pub mod rockgem;
pub mod rockincense;
pub mod rockiumz;
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
pub mod snorliumz;
pub mod snowball;
pub mod softsand;
pub mod solganiumz;
pub mod souldew;
pub mod spelltag;
pub mod spelonberry;
pub mod splashplate;
pub mod spookyplate;
pub mod staraptite;
pub mod starfberry;
pub mod starminite;
pub mod steelgem;
pub mod steeliumz;
pub mod steelixite;
pub mod steelmemory;
pub mod stick;
pub mod stickybarb;
pub mod stoneplate;
pub mod swampertite;
pub mod tamatoberry;
pub mod tangaberry;
pub mod tapuniumz;
pub mod tatsugirinite;
pub mod thickclub;
pub mod throatspray;
pub mod toxicorb;
pub mod toxicplate;
pub mod twistedspoon;
pub mod tyranitarite;
pub mod ultranecroziumz;
pub mod utilityumbrella;
pub mod venusaurite;
pub mod victreebelite;
pub mod vilevial;
pub mod wacanberry;
pub mod watergem;
pub mod wateriumz;
pub mod watermemory;
pub mod watmelberry;
pub mod waveincense;
pub mod weaknesspolicy;
pub mod wellspringmask;
pub mod wepearberry;
pub mod whiteherb;
pub mod widelens;
pub mod wikiberry;
pub mod wiseglasses;
pub mod yacheberry;
pub mod zapplate;
pub mod zeraorite;
pub mod zoomlens;
pub mod zygardite;
use crate::battle::Battle;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

// ===========================================
// Item Callback Dispatch Functions
// ===========================================
// These functions route item events to their specific callback implementations.
// Each function matches on item_id and calls the corresponding module's callback.
//
// Note: Only items with properly implemented callbacks are included.
// TODO: Add more items as their callbacks are implemented.

/// Dispatch onModifyAtk callbacks (read-only)
pub fn dispatch_on_modify_atk(
    battle: &Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> Option<EventResult> {
    match item_id {
        "choiceband" => Some(choiceband::on_modify_atk(battle, pokemon_pos)),
        // TODO: Add lightball, thickclub when implemented
        _ => None,
    }
}

/// Dispatch onModifySpA callbacks (read-only)
pub fn dispatch_on_modify_sp_a(
    battle: &Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> Option<EventResult> {
    match item_id {
        "choicespecs" => Some(choicespecs::on_modify_sp_a(battle, pokemon_pos)),
        // TODO: Add deepseatooth, lightball when implemented
        _ => None,
    }
}

/// Dispatch onModifyDef callbacks (read-only)
pub fn dispatch_on_modify_def(
    battle: &Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> Option<EventResult> {
    match item_id {
        "eviolite" => Some(eviolite::on_modify_def(battle, pokemon_pos)),
        // TODO: Add metalpowder when implemented
        _ => None,
    }
}

/// Dispatch onModifySpD callbacks (read-only)
pub fn dispatch_on_modify_sp_d(
    battle: &Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> Option<EventResult> {
    match item_id {
        "assaultvest" => Some(assaultvest::on_modify_sp_d(battle, pokemon_pos)),
        "eviolite" => Some(eviolite::on_modify_sp_d(battle, pokemon_pos)),
        // TODO: Add deepseascale when implemented
        _ => None,
    }
}

/// Dispatch onModifySpe callbacks (read-only)
pub fn dispatch_on_modify_spe(
    battle: &Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> Option<EventResult> {
    match item_id {
        // TODO: Add choicescarf, ironball, machobrace, power items, quickpowder when implemented
        _ => None,
    }
}

/// Dispatch onModifyDamage callbacks (read-only)
pub fn dispatch_on_modify_damage(
    battle: &Battle,
    item_id: &str,
    target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
) -> Option<EventResult> {
    match item_id {
        "lifeorb" => Some(lifeorb::on_modify_damage(battle, target_pos, source_pos)),
        // TODO: Add expertbelt, metronome when implemented
        _ => None,
    }
}

/// Dispatch onSourceModifyDamage callbacks (read-only)
pub fn dispatch_on_source_modify_damage(
    battle: &Battle,
    item_id: &str,
    target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
) -> Option<EventResult> {
    match item_id {
        // TODO: Add type-resist berries when implemented
        _ => None,
    }
}

/// Dispatch onBasePower callbacks (read-only)
pub fn dispatch_on_base_power(
    battle: &Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> Option<EventResult> {
    match item_id {
        // TODO: Add type-boost items (blackbelt, charcoal, etc.) when implemented
        // TODO: Add plates when implemented
        // TODO: Add special items (adamantorb, etc.) when implemented
        _ => None,
    }
}

/// Dispatch onModifyCritRatio callbacks (read-only)
pub fn dispatch_on_modify_crit_ratio(
    battle: &Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> Option<EventResult> {
    match item_id {
        // TODO: Add leek, luckypunch, razorclaw, scopelens, stick when implemented
        _ => None,
    }
}

/// Dispatch onModifyAccuracy callbacks (read-only)
pub fn dispatch_on_modify_accuracy(
    battle: &Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> Option<EventResult> {
    match item_id {
        // TODO: Add brightpowder, laxincense, widelens, zoomlens when implemented
        _ => None,
    }
}

/// Dispatch onAfterMoveSecondarySelf callbacks (mutates battle)
pub fn dispatch_on_after_move_secondary_self(
    battle: &mut Battle,
    item_id: &str,
    source_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> Option<EventResult> {
    match item_id {
        "lifeorb" => Some(lifeorb::on_after_move_secondary_self(battle, source_pos, target_pos)),
        // TODO: Add shellbell, throatspray when implemented
        _ => None,
    }
}

/// Dispatch onResidual callbacks (mutates battle)
pub fn dispatch_on_residual(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> Option<EventResult> {
    match item_id {
        "blacksludge" => Some(blacksludge::on_residual(battle, pokemon_pos)),
        "leftovers" => Some(leftovers::on_residual(battle, pokemon_pos)),
        // TODO: Add flameorb, stickybarb, toxicorb when implemented
        _ => None,
    }
}

/// Dispatch onUpdate callbacks (mutates battle)
pub fn dispatch_on_update(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> Option<EventResult> {
    match item_id {
        // TODO: Add healing berries (aguavberry, aspearberry, etc.) when implemented
        // TODO: Add berryjuice, mentalherb, whiteherb when implemented
        _ => None,
    }
}

/// Dispatch onStart callbacks (mutates battle)
pub fn dispatch_on_start(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> Option<EventResult> {
    match item_id {
        // TODO: Add airballoon, boosterenergy, choice items, seeds, metronome when implemented
        _ => None,
    }
}

/// Dispatch onDamagingHit callbacks (mutates battle)
pub fn dispatch_on_damaging_hit(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
    source_pos: (usize, usize),
) -> Option<EventResult> {
    match item_id {
        // TODO: Add absorbbulb, airballoon, cellbattery, ejectbutton, etc. when implemented
        _ => None,
    }
}

/// Dispatch onTakeItem callbacks (read-only)
pub fn dispatch_on_take_item(
    battle: &Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> Option<EventResult> {
    match item_id {
        // TODO: Add mega stones, Z-crystals, memories, drives when implemented
        _ => None,
    }
}

/// Dispatch onEat callbacks (mutates battle)
pub fn dispatch_on_eat(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
) -> Option<EventResult> {
    match item_id {
        // TODO: Add berries with onEat when implemented
        _ => None,
    }
}

/// Dispatch onSourceTryPrimaryHit callbacks (mutates battle)
pub fn dispatch_on_source_try_primary_hit(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
    source_pos: (usize, usize),
) -> Option<EventResult> {
    match item_id {
        // TODO: Add type gems when implemented
        _ => None,
    }
}
