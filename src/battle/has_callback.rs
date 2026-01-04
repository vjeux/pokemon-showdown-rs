// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Battle {

    /// Check if an effect has a callback for a specific event
    /// This is a Rust helper to replicate JavaScript's getCallback() check
    /// without actually executing the callback
    ///
    /// Returns true if the effect has a handler for the event, false otherwise
    pub fn has_callback(&self, effect_id: &ID, event_id: &str) -> bool {
        let effect_str = effect_id.as_str();

        eprintln!("[HAS_CALLBACK] effect_id='{}', event_id='{}'", effect_str, event_id);

        // IMPORTANT: Check conditions (status, volatile, weather, terrain) FIRST
        // This is critical because some IDs like "stall" exist as both abilities AND conditions
        // When checking a volatile, we want to find the CONDITION, not the ability
        // In JavaScript, the volatile already has its callback attached, so there's no ambiguity
        let condition_check = crate::data::conditions::get_condition(effect_id);
        eprintln!("[HAS_CALLBACK] Checking conditions: found={}", condition_check.is_some());
        if condition_check.is_some() {
            eprintln!("[HAS_CALLBACK] Found as condition, calling condition_has_callback");
            return self.condition_has_callback(effect_str, event_id);
        }

        // Check if this is a move-embedded condition
        // JavaScript's dex.conditions.getByID() checks moves for embedded conditions:
        // } else if (this.dex.data.Moves.hasOwnProperty(id) && (found = this.dex.data.Moves[id]).condition ...
        //     condition = new Condition({ name: found.name || id, ...found.condition });
        if let Some(move_data) = self.dex.moves().get(effect_str) {
            if move_data.condition.is_some() {
                // This is a move with an embedded condition (like King's Shield)
                // Treat it as a condition for callback purposes
                eprintln!("[HAS_CALLBACK] Found as move-embedded condition");
                return self.condition_has_callback(effect_str, event_id);
            }
        }

        // Check abilities
        if self.dex.abilities().get(effect_str).is_some() {
            eprintln!("[HAS_CALLBACK] Found as ability");
            return self.ability_has_callback(effect_str, event_id);
        }

        // Check items
        if self.dex.items().get(effect_str).is_some() {
            eprintln!("[HAS_CALLBACK] Found as item");
            return self.item_has_callback(effect_str, event_id);
        }

        // Check moves
        if self.dex.moves().get(effect_str).is_some() {
            eprintln!("[HAS_CALLBACK] Found as move");
            return self.move_has_callback(effect_str, event_id);
        }

        // Check species - species can have callbacks like onSwitchIn for form changes
        if self.dex.species().get(effect_str).is_some() {
            eprintln!("[HAS_CALLBACK] Found as species");
            return self.species_has_callback(effect_str, event_id);
        }

        eprintln!("[HAS_CALLBACK] Not found, returning false");
        false
    }

    /// Check if a an ability has a callback for an event
    pub fn ability_has_callback(&self, ability_id: &str, event_id: &str) -> bool {
        // Gen 5+ special case: abilities use onStart during SwitchIn events
        // This matches JavaScript getCallback() behavior
        // JavaScript: if (callback === undefined && target instanceof Pokemon && this.gen >= 5 && callbackName === 'onSwitchIn' &&
        //             !(effect as any).onAnySwitchIn && (['Ability', 'Item'].includes(effect.effectType) ...)) {
        //             callback = (effect as any).onStart;
        // }
        if self.gen >= 5 && event_id == "onSwitchIn" {
            // Check if ability has onAnySwitchIn - if yes, use normal SwitchIn logic
            // This recursive call is safe because event_id != "onSwitchIn" so won't trigger special case
            let has_any_switch_in = self.ability_has_callback(ability_id, "onAnySwitchIn");

            // If ability doesn't have onAnySwitchIn, check for onStart instead
            // This recursive call is safe because event_id != "onSwitchIn" so won't trigger special case
            if !has_any_switch_in && self.ability_has_callback(ability_id, "onStart") {
                return true;
            }
        }

        match event_id {
            "AllyBasePower" | "onAllyBasePower" => matches!(
                ability_id,
                "battery" | "powerspot" | "steelyspirit"
            ),
            "AnyBasePower" | "onAnyBasePower" => matches!(
                ability_id,
                "darkaura" | "fairyaura"
            ),
            "BasePower" | "onBasePower" => matches!(
                ability_id,
                "aerilate" | "analytic" | "flareboost" | "galvanize" | "ironfist" |
                "megalauncher" | "normalize" | "pixilate" | "punkrock" | "reckless" |
                "refrigerate" | "rivalry" | "sandforce" | "sharpness" | "sheerforce" |
                "strongjaw" | "supremeoverlord" | "technician" | "toughclaws" | "toxicboost"
            ),
            "SourceBasePower" | "onSourceBasePower" => matches!(
                ability_id,
                "dryskin"
            ),
            "onAfterBoost" => matches!(
                ability_id,
                "rattled"
            ),
            "onAfterEachBoost" => matches!(
                ability_id,
                "competitive" | "defiant"
            ),
            "onAfterMoveSecondary" => matches!(
                ability_id,
                "angershell" | "berserk" | "colorchange" | "pickpocket"
            ),
            "onAfterMoveSecondarySelf" => matches!(
                ability_id,
                "magician"
            ),
            "onAfterSetStatus" => matches!(
                ability_id,
                "synchronize"
            ),
            "onAfterTerastallization" => matches!(
                ability_id,
                "teraformzero"
            ),
            "onAfterUseItem" => matches!(
                ability_id,
                "unburden"
            ),
            "onAllyAfterUseItem" => matches!(
                ability_id,
                "symbiosis"
            ),
            "onAllyFaint" => matches!(
                ability_id,
                "powerofalchemy" | "receiver"
            ),
            "onAllyModifyAtk" => matches!(
                ability_id,
                "flowergift"
            ),
            "onAllyModifySpD" => matches!(
                ability_id,
                "flowergift"
            ),
            "onAllySetStatus" => matches!(
                ability_id,
                "flowerveil" | "pastelveil" | "sweetveil"
            ),
            "onAllyTryAddVolatile" => matches!(
                ability_id,
                "aromaveil" | "flowerveil" | "sweetveil"
            ),
            "onAllyTryBoost" => matches!(
                ability_id,
                "flowerveil"
            ),
            "onAllyTryHitSide" => matches!(
                ability_id,
                "magicbounce" | "rebound" | "sapsipper" | "soundproof"
            ),
            "onAnyAccuracy" => matches!(
                ability_id,
                "noguard"
            ),
            "onAnyAfterMega" => matches!(
                ability_id,
                "opportunist"
            ),
            "onAnyAfterMove" => matches!(
                ability_id,
                "opportunist" | "terashell"
            ),
            "onAnyAfterSetStatus" => matches!(
                ability_id,
                "poisonpuppeteer"
            ),
            "onAnyAfterTerastallization" => matches!(
                ability_id,
                "opportunist"
            ),
            "onAnyBeforeMove" => matches!(
                ability_id,
                "terashell"
            ),
            "onAnyDamage" => matches!(
                ability_id,
                "damp"
            ),
            "onAnyFaint" => matches!(
                ability_id,
                "soulheart"
            ),
            "onAnyInvulnerability" => matches!(
                ability_id,
                "noguard"
            ),
            "onAnyModifyAccuracy" => matches!(
                ability_id,
                "victorystar"
            ),
            "onAnyModifyAtk" => matches!(
                ability_id,
                "tabletsofruin"
            ),
            "onAnyModifyBoost" => matches!(
                ability_id,
                "unaware"
            ),
            "onAnyModifyDamage" => matches!(
                ability_id,
                "friendguard"
            ),
            "onAnyModifyDef" => matches!(
                ability_id,
                "swordofruin"
            ),
            "onAnyModifySpA" => matches!(
                ability_id,
                "vesselofruin"
            ),
            "onAnyModifySpD" => matches!(
                ability_id,
                "beadsofruin"
            ),
            "onAnyRedirectTarget" => matches!(
                ability_id,
                "lightningrod" | "stormdrain"
            ),
            "onAnySetWeather" => matches!(
                ability_id,
                "deltastream" | "desolateland" | "primordialsea"
            ),
            "onAnySwitchIn" => matches!(
                ability_id,
                "commander" | "opportunist" | "pastelveil"
            ),
            "onAnyTryMove" => matches!(
                ability_id,
                "damp"
            ),
            "onAnyTryPrimaryHit" => matches!(
                ability_id,
                "aurabreak"
            ),
            "onBeforeMove" => matches!(
                ability_id,
                "gorillatactics" | "truant"
            ),
            "onBeforeSwitchIn" => matches!(
                ability_id,
                "illusion"
            ),
            "onChangeBoost" => matches!(
                ability_id,
                "contrary" | "ripen" | "simple"
            ),
            "onCheckShow" => matches!(
                ability_id,
                "naturalcure"
            ),
            "onCriticalHit" => matches!(
                ability_id,
                "disguise" | "iceface"
            ),
            "onDamage" => matches!(
                ability_id,
                "angershell" | "berserk" | "disguise" | "gluttony" | "heatproof" |
                "iceface" | "magicguard" | "mountaineer" | "poisonheal" | "rockhead" |
                "sturdy"
            ),
            "onDamagingHit" => matches!(
                ability_id,
                "aftermath" | "cottondown" | "cursedbody" | "cutecharm" | "effectspore" |
                "electromorphosis" | "flamebody" | "gooey" | "gulpmissile" | "illusion" |
                "innardsout" | "ironbarbs" | "justified" | "lingeringaroma" | "mummy" |
                "perishbody" | "poisonpoint" | "rattled" | "roughskin" | "sandspit" |
                "seedsower" | "stamina" | "static" | "steamengine" | "tanglinghair" |
                "thermalexchange" | "toxicdebris" | "wanderingspirit" | "watercompaction" | "weakarmor" |
                "windpower"
            ),
            "onDeductPP" => matches!(
                ability_id,
                "pressure"
            ),
            "onDisableMove" => matches!(
                ability_id,
                "gorillatactics"
            ),
            "onDragOut" => matches!(
                ability_id,
                "guarddog" | "suctioncups"
            ),
            "onEatItem" => matches!(
                ability_id,
                "cheekpouch" | "cudchew" | "ripen"
            ),
            "onEffectiveness" => matches!(
                ability_id,
                "disguise" | "iceface"
            ),
            "onEmergencyExit" => matches!(
                ability_id,
                "emergencyexit" | "wimpout"
            ),
            "onEnd" => matches!(
                ability_id,
                "airlock" | "asoneglastrier" | "asonespectrier" | "cloudnine" | "deltastream" |
                "desolateland" | "flashfire" | "gorillatactics" | "illusion" | "neutralizinggas" |
                "opportunist" | "primordialsea" | "protosynthesis" | "quarkdrive" | "supremeoverlord" |
                "unburden" | "unnerve" | "zenmode"
            ),
            "onFaint" => matches!(
                ability_id,
                "illusion"
            ),
            "onFlinch" => matches!(
                ability_id,
                "steadfast"
            ),
            "onFoeAfterBoost" => matches!(
                ability_id,
                "opportunist"
            ),
            "onFoeMaybeTrapPokemon" => matches!(
                ability_id,
                "arenatrap" | "magnetpull" | "shadowtag"
            ),
            "onFoeTrapPokemon" => matches!(
                ability_id,
                "arenatrap" | "magnetpull" | "shadowtag"
            ),
            "onFoeTryEatItem" => matches!(
                ability_id,
                "asoneglastrier" | "asonespectrier" | "unnerve"
            ),
            "onFoeTryMove" => matches!(
                ability_id,
                "armortail" | "dazzling" | "queenlymajesty"
            ),
            "onHit" => matches!(
                ability_id,
                "angerpoint" | "owntempo"
            ),
            "onImmunity" => matches!(
                ability_id,
                "icebody" | "magmaarmor" | "oblivious" | "overcoat" | "sandforce" |
                "sandrush" | "sandveil" | "snowcloak"
            ),
            "onModifyAccuracy" => matches!(
                ability_id,
                "sandveil" | "snowcloak" | "tangledfeet" | "wonderskin"
            ),
            "onModifyAtk" => matches!(
                ability_id,
                "blaze" | "defeatist" | "dragonsmaw" | "gorillatactics" | "guts" |
                "hugepower" | "hustle" | "orichalcumpulse" | "overgrow" | "purepower" |
                "rockypayload" | "slowstart" | "stakeout" | "steelworker" | "swarm" |
                "torrent" | "transistor" | "waterbubble"
            ),
            "onModifyCritRatio" => matches!(
                ability_id,
                "merciless" | "superluck"
            ),
            "onModifyDamage" => matches!(
                ability_id,
                "neuroforce" | "sniper" | "tintedlens"
            ),
            "onModifyDef" => matches!(
                ability_id,
                "furcoat" | "grasspelt" | "marvelscale"
            ),
            "onModifyMove" => matches!(
                ability_id,
                "battlebond" | "gorillatactics" | "illuminate" | "infiltrator" | "keeneye" |
                "longreach" | "mindseye" | "moldbreaker" | "myceliummight" | "propellertail" |
                "scrappy" | "serenegrace" | "sheerforce" | "skilllink" | "stalwart" |
                "stancechange" | "stench" | "teravolt" | "turboblaze" | "unseenfist"
            ),
            "onModifySTAB" => matches!(
                ability_id,
                "adaptability"
            ),
            "onModifySecondaries" => matches!(
                ability_id,
                "shielddust"
            ),
            "onModifySpA" => matches!(
                ability_id,
                "blaze" | "defeatist" | "dragonsmaw" | "hadronengine" | "minus" |
                "overgrow" | "plus" | "rockypayload" | "solarpower" | "stakeout" |
                "steelworker" | "swarm" | "torrent" | "transistor" | "waterbubble"
            ),
            "onModifySpe" => matches!(
                ability_id,
                "chlorophyll" | "quickfeet" | "sandrush" | "slowstart" | "slushrush" |
                "surgesurfer" | "swiftswim"
            ),
            "ModifyType" | "onModifyType" => matches!(
                ability_id,
                "aerilate" | "galvanize" | "liquidvoice" | "normalize" | "pixilate" |
                "refrigerate"
            ),
            "onModifyWeight" => matches!(
                ability_id,
                "heavymetal" | "lightmetal"
            ),
            "onPrepareHit" => matches!(
                ability_id,
                "libero" | "parentalbond" | "protean"
            ),
            "onResidual" => matches!(
                ability_id,
                "baddreams" | "cudchew" | "harvest" | "healer" | "hungerswitch" |
                "hydration" | "moody" | "opportunist" | "pickup" | "powerconstruct" |
                "schooling" | "shedskin" | "shieldsdown" | "slowstart" | "speedboost" |
                "zenmode"
            ),
            "onSetStatus" => matches!(
                ability_id,
                "comatose" | "immunity" | "insomnia" | "leafguard" | "limber" |
                "pastelveil" | "purifyingsalt" | "shieldsdown" | "thermalexchange" | "vitalspirit" |
                "waterbubble" | "waterveil"
            ),
            "onSideConditionStart" => matches!(
                ability_id,
                "windpower" | "windrider"
            ),
            "onSourceAfterFaint" => matches!(
                ability_id,
                "asoneglastrier" | "asonespectrier" | "battlebond" | "beastboost" | "chillingneigh" |
                "grimneigh" | "moxie"
            ),
            "onSourceDamagingHit" => matches!(
                ability_id,
                "poisontouch" | "toxicchain"
            ),
            "onSourceModifyAccuracy" => matches!(
                ability_id,
                "compoundeyes" | "hustle"
            ),
            "onSourceModifyAtk" => matches!(
                ability_id,
                "heatproof" | "purifyingsalt" | "thickfat" | "waterbubble"
            ),
            "onSourceModifyDamage" => matches!(
                ability_id,
                "filter" | "fluffy" | "icescales" | "multiscale" | "prismarmor" |
                "punkrock" | "ripen" | "shadowshield" | "solidrock"
            ),
            "onSourceModifySecondaries" => matches!(
                ability_id,
                "parentalbond"
            ),
            "onSourceModifySpA" => matches!(
                ability_id,
                "heatproof" | "purifyingsalt" | "thickfat" | "waterbubble"
            ),
            "onSourceTryHeal" => matches!(
                ability_id,
                "liquidooze"
            ),
            "onSourceTryPrimaryHit" => matches!(
                ability_id,
                "gulpmissile"
            ),
            "onStart" => matches!(
                ability_id,
                "airlock" | "anticipation" | "asoneglastrier" | "asonespectrier" | "aurabreak" |
                "beadsofruin" | "cloudnine" | "comatose" | "commander" | "costar" |
                "curiousmedicine" | "darkaura" | "dauntlessshield" | "deltastream" | "desolateland" |
                "download" | "drizzle" | "drought" | "electricsurge" | "embodyaspectcornerstone" |
                "embodyaspecthearthflame" | "embodyaspectteal" | "embodyaspectwellspring" | "fairyaura" | "flowergift" |
                "forecast" | "forewarn" | "frisk" | "gluttony" | "gorillatactics" |
                "grassysurge" | "hadronengine" | "hospitality" | "iceface" | "intimidate" |
                "intrepidsword" | "klutz" | "mimicry" | "mistysurge" | "moldbreaker" |
                "orichalcumpulse" | "pastelveil" | "pressure" | "primordialsea" | "protosynthesis" |
                "psychicsurge" | "quarkdrive" | "sandstream" | "schooling" | "screencleaner" |
                "shieldsdown" | "slowstart" | "snowwarning" | "supersweetsyrup" | "supremeoverlord" |
                "swordofruin" | "tabletsofruin" | "teravolt" | "trace" | "truant" |
                "turboblaze" | "unnerve" | "vesselofruin" | "windrider"
            ),
            "onSwitchIn" => matches!(
                ability_id,
                "airlock" | "cloudnine" | "imposter" | "neutralizinggas" | "terashift" |
                "zerotohero"
            ),
            "onSwitchOut" => matches!(
                ability_id,
                "naturalcure" | "regenerator" | "zerotohero"
            ),
            "onTakeItem" => matches!(
                ability_id,
                "stickyhold" | "unburden"
            ),
            "onTerrainChange" => matches!(
                ability_id,
                "mimicry" | "quarkdrive"
            ),
            "onTryAddVolatile" => matches!(
                ability_id,
                "innerfocus" | "insomnia" | "leafguard" | "owntempo" | "purifyingsalt" |
                "shieldsdown" | "vitalspirit"
            ),
            "onTryBoost" => matches!(
                ability_id,
                "bigpecks" | "clearbody" | "fullmetalbody" | "guarddog" | "hypercutter" |
                "illuminate" | "innerfocus" | "keeneye" | "mindseye" | "mirrorarmor" |
                "oblivious" | "owntempo" | "scrappy" | "whitesmoke"
            ),
            "onTryEatItem" => matches!(
                ability_id,
                "angershell" | "berserk" | "ripen"
            ),
            "onTryHeal" => matches!(
                ability_id,
                "ripen"
            ),
            "onTryHit" => matches!(
                ability_id,
                "bulletproof" | "dryskin" | "eartheater" | "flashfire" | "goodasgold" |
                "lightningrod" | "magicbounce" | "motordrive" | "mountaineer" | "oblivious" |
                "overcoat" | "rebound" | "sapsipper" | "soundproof" | "stormdrain" |
                "sturdy" | "telepathy" | "voltabsorb" | "waterabsorb" | "wellbakedbody" |
                "windrider" | "wonderguard"
            ),
            "onUpdate" => matches!(
                ability_id,
                "commander" | "disguise" | "iceface" | "immunity" | "insomnia" |
                "limber" | "magmaarmor" | "oblivious" | "owntempo" | "pastelveil" |
                "thermalexchange" | "trace" | "vitalspirit" | "waterbubble" | "waterveil"
            ),
            "onWeather" => matches!(
                ability_id,
                "dryskin" | "icebody" | "raindish" | "solarpower"
            ),
            "onWeatherChange" => matches!(
                ability_id,
                "flowergift" | "forecast" | "iceface" | "protosynthesis"
            ),
            _ => false,
        }
    }

    /// Check if a an item has a callback for an event
    pub fn item_has_callback(&self, item_id: &str, event_id: &str) -> bool {
        // Gen 5+ special case: items use onStart during SwitchIn events
        // This matches JavaScript getCallback() behavior (same logic as abilities)
        // Items don't have onAnySwitchIn callbacks, so always check onStart if gen >= 5
        if self.gen >= 5 && event_id == "onSwitchIn" {
            // This recursive call is safe because event_id != "onSwitchIn" so won't trigger special case
            if self.item_has_callback(item_id, "onStart") {
                return true;
            }
        }

        match event_id {
            "onAfterBoost" => matches!(
                item_id,
                "adrenalineorb" | "ejectpack"
            ),
            "onAnyAfterMega" => matches!(
                item_id,
                "ejectpack" | "mirrorherb" | "whiteherb"
            ),
            "onAnyAfterMove" => matches!(
                item_id,
                "ejectpack" | "mirrorherb" | "whiteherb"
            ),
            "onAnyAfterTerastallization" => matches!(
                item_id,
                "mirrorherb"
            ),
            "onAnyPseudoWeatherChange" => matches!(
                item_id,
                "roomservice"
            ),
            "onAnySwitchIn" => matches!(
                item_id,
                "ejectpack" | "mirrorherb" | "whiteherb"
            ),
            "onChargeMove" => matches!(
                item_id,
                "powerherb"
            ),
            "onEffectiveness" => matches!(
                item_id,
                "ironball"
            ),
            "onEnd" => matches!(
                item_id,
                "ejectpack" | "mirrorherb" | "utilityumbrella"
            ),
            "onFoeAfterBoost" => matches!(
                item_id,
                "mirrorherb"
            ),
            "onImmunity" => matches!(
                item_id,
                "safetygoggles"
            ),
            "onModifyCritRatio" => matches!(
                item_id,
                "leek" | "luckypunch" | "razorclaw" | "scopelens" | "stick"
            ),
            "onModifyAtk" => matches!(
                item_id,
                "choiceband"
            ),
            "onModifySecondaries" => matches!(
                item_id,
                "covertcloak"
            ),
            "onModifySpe" => matches!(
                item_id,
                "ironball"
            ),
            "onModifyWeight" => matches!(
                item_id,
                "floatstone"
            ),
            "onResidual" => matches!(
                item_id,
                "ejectpack" | "leftovers" | "mirrorherb" | "toxicorb" | "whiteherb"
            ),
            "onStart" => matches!(
                item_id,
                "boosterenergy" | "roomservice" | "utilityumbrella"
            ),
            "onSwitchIn" => matches!(
                item_id,
                "blueorb" | "redorb"
            ),
            "onTakeItem" => matches!(
                item_id,
                "blueorb" | "boosterenergy" | "redorb"
            ),
            "onTryBoost" => matches!(
                item_id,
                "clearamulet"
            ),
            "onTryHit" => matches!(
                item_id,
                "safetygoggles"
            ),
            "onUpdate" => matches!(
                item_id,
                "boosterenergy" | "utilityumbrella"
            ),
            "onUse" => matches!(
                item_id,
                "ejectpack" | "mirrorherb" | "whiteherb"
            ),
            "onUseItem" => matches!(
                item_id,
                "ejectpack"
            ),
            "onModifyDamage" => matches!(
                item_id,
                "lifeorb"
            ),
            "onAfterMoveSecondarySelf" => matches!(
                item_id,
                "lifeorb"
            ),
            _ => false,
        }
    }

    /// Check if a a move has a callback for an event
    pub fn move_has_callback(&self, move_id: &str, event_id: &str) -> bool {
        match event_id {
            "BasePower" => matches!(
                move_id,
                "barbbarrage" | "brine" | "collisioncourse" | "electrodrift" | "expandingforce" |
                "facade" | "ficklebeam" | "fusionbolt" | "fusionflare" | "gravapple" |
                "knockoff" | "lashout" | "mistyexplosion" | "psyblade" | "retaliate" |
                "solarbeam" | "solarblade" | "venoshock"
            ),
            "onAfterHit" => matches!(
                move_id,
                "ceaselessedge" | "covet" | "icespinner" | "knockoff" | "mortalspin" |
                "rapidspin" | "stoneaxe" | "thief"
            ),
            "onAfterMove" => matches!(
                move_id,
                "beakblast" | "iceball" | "mindblown" | "rollout" | "sparklingaria" |
                "spitup" | "steelbeam"
            ),
            "onAfterMoveSecondarySelf" => matches!(
                move_id,
                "fellstinger" | "orderup" | "polarflare" | "relicsong"
            ),
            "onAfterSubDamage" => matches!(
                move_id,
                "ceaselessedge" | "coreenforcer" | "flameburst" | "gmaxsnooze" | "icespinner" |
                "mortalspin" | "rapidspin" | "shellsidearm" | "splinteredstormshards" | "steelroller" |
                "stoneaxe"
            ),
            "onDamage" => matches!(
                move_id,
                "falseswipe" | "holdback"
            ),
            "onDisableMove" => matches!(
                move_id,
                "belch" | "stuffcheeks"
            ),
            "onEffectiveness" => matches!(
                move_id,
                "flyingpress" | "freezedry" | "thousandarrows"
            ),
            "onHit" => matches!(
                move_id,
                "acupressure" | "afteryou" | "alluringvoice" | "allyswitch" | "anchorshot" |
                "aromatherapy" | "assist" | "autotomize" | "banefulbunker" | "batonpass" |
                "bellydrum" | "bestow" | "block" | "bugbite" | "burningbulwark" |
                "burningjealousy" | "camouflage" | "clangoroussoul" | "clearsmog" | "conversion" |
                "conversion2" | "copycat" | "coreenforcer" | "corrosivegas" | "curse" |
                "defog" | "detect" | "direclaw" | "doodle" | "eeriespell" |
                "endure" | "entrainment" | "filletaway" | "flameburst" | "floralhealing" |
                "forestscurse" | "freezyfrost" | "genesissupernova" | "gmaxbefuddle" | "gmaxcentiferno" |
                "gmaxcuddle" | "gmaxdepletion" | "gmaxfinale" | "gmaxfoamburst" | "gmaxgoldrush" |
                "gmaxmalodor" | "gmaxmeltdown" | "gmaxreplenish" | "gmaxsandblast" | "gmaxsmite" |
                "gmaxsnooze" | "gmaxstonesurge" | "gmaxstunshock" | "gmaxsweetness" | "gmaxtartness" |
                "gmaxterror" | "gmaxvoltcrash" | "gmaxwindrage" | "guardsplit" | "guardswap" |
                "healbell" | "healpulse" | "heartswap" | "incinerate" | "instruct" |
                "jawlock" | "junglehealing" | "kingsshield" | "lockon" | "lunarblessing" |
                "magicpowder" | "maxairstream" | "maxdarkness" | "maxflare" | "maxflutterby" |
                "maxgeyser" | "maxguard" | "maxhailstorm" | "maxknuckle" | "maxlightning" |
                "maxmindstorm" | "maxooze" | "maxovergrowth" | "maxphantasm" | "maxquake" |
                "maxrockfall" | "maxstarfall" | "maxsteelspike" | "maxstrike" | "maxwyrmwind" |
                "meanlook" | "metronome" | "mimic" | "mindreader" | "moonlight" |
                "morningsun" | "obstruct" | "painsplit" | "partingshot" | "pluck" |
                "polarflare" | "pollenpuff" | "powersplit" | "powerswap" | "protect" |
                "psychup" | "purify" | "quash" | "recycle" | "reflecttype" |
                "refresh" | "relicsong" | "rest" | "roleplay" | "sappyseed" |
                "shedtail" | "shellsidearm" | "shoreup" | "silktrap" | "simplebeam" |
                "sketch" | "skillswap" | "skydrop" | "sleeptalk" | "smellingsalts" |
                "soak" | "speedswap" | "spiderweb" | "spikyshield" | "spiritshackle" |
                "spite" | "splinteredstormshards" | "steelroller" | "strengthsap" | "stuffcheeks" |
                "substitute" | "swallow" | "switcheroo" | "synthesis" | "takeheart" |
                "thousandwaves" | "tidyup" | "topsyturvy" | "transform" | "trick" |
                "trickortreat" | "venomdrench" | "wakeupslap" | "worryseed"
            ),
            "onHitField" => matches!(
                move_id,
                "courtchange" | "flowershield" | "haze" | "perishsong" | "rototiller" |
                "teatime"
            ),
            "onHitSide" => matches!(
                move_id,
                "gearup" | "magneticflux" | "quickguard" | "wideguard"
            ),
            "onModifyMove" => matches!(
                move_id,
                "beatup" | "bleakwindstorm" | "blizzard" | "curse" | "expandingforce" |
                "firepledge" | "grasspledge" | "growth" | "hurricane" | "iceball" |
                "lightthatburnsthesky" | "magnitude" | "photongeyser" | "present" | "pursuit" |
                "rollout" | "sandsearstorm" | "secretpower" | "shellsidearm" | "skydrop" |
                "struggle" | "terablast" | "terastarstorm" | "terrainpulse" | "thunder" |
                "waterpledge" | "weatherball" | "wildboltstorm"
            ),
            "onModifyTarget" => matches!(
                move_id,
                "comeuppance" | "metalburst"
            ),
            "onModifyType" => matches!(
                move_id,
                "aurawheel" | "hiddenpower" | "ivycudgel" | "judgment" | "multiattack" |
                "naturalgift" | "ragingbull" | "revelationdance" | "technoblast" | "terablast" |
                "terastarstorm" | "terrainpulse" | "weatherball"
            ),
            "onMoveFail" => matches!(
                move_id,
                "axekick" | "highjumpkick" | "jumpkick" | "skydrop" | "supercellslam"
            ),
            "onPrepareHit" => matches!(
                move_id,
                "allyswitch" | "banefulbunker" | "burningbulwark" | "destinybond" | "detect" |
                "endure" | "firepledge" | "fling" | "grasspledge" | "ivycudgel" |
                "kingsshield" | "maxguard" | "naturalgift" | "obstruct" | "protect" |
                "shellsidearm" | "silktrap" | "spikyshield" | "terablast" | "waterpledge"
            ),
            "onTry" => matches!(
                move_id,
                "aurawheel" | "auroraveil" | "clangoroussoul" | "comeuppance" | "counter" |
                "craftyshield" | "darkvoid" | "doomdesire" | "fakeout" | "filletaway" |
                "firstimpression" | "followme" | "futuresight" | "hyperspacefury" | "lastresort" |
                "magnetrise" | "matblock" | "metalburst" | "mirrorcoat" | "noretreat" |
                "poltergeist" | "quickguard" | "ragepowder" | "rest" | "round" |
                "skydrop" | "sleeptalk" | "snore" | "spitup" | "splash" |
                "steelroller" | "stockpile" | "stuffcheeks" | "suckerpunch" | "swallow" |
                "telekinesis" | "teleport" | "thunderclap" | "upperhand" | "wideguard"
            ),
            "onTryHit" => matches!(
                move_id,
                "autotomize" | "brickbreak" | "celebrate" | "clangoroussoul" | "curse" |
                "disable" | "electrify" | "entrainment" | "filletaway" | "foresight" |
                "gastroacid" | "grassknot" | "happyhour" | "healingwish" | "heatcrash" |
                "heavyslam" | "helpinghand" | "lockon" | "lowkick" | "lunardance" |
                "mefirst" | "mindreader" | "miracleeye" | "mirrormove" | "naturepower" |
                "odorsleuth" | "pollenpuff" | "poltergeist" | "psychicfangs" | "psychoshift" |
                "pursuit" | "ragingbull" | "revivalblessing" | "roleplay" | "shedtail" |
                "simplebeam" | "skillswap" | "skydrop" | "splash" | "spotlight" |
                "substitute" | "uproar" | "worryseed" | "yawn"
            ),
            "onTryImmunity" => matches!(
                move_id,
                "attract" | "captivate" | "dreameater" | "endeavor" | "leechseed" |
                "octolock" | "switcheroo" | "synchronoise" | "trick" | "worryseed"
            ),
            "onTryMove" => matches!(
                move_id,
                "bounce" | "burnup" | "dig" | "dive" | "doubleshock" |
                "echoedvoice" | "electroshot" | "fly" | "freezeshock" | "geomancy" |
                "iceburn" | "meteorbeam" | "phantomforce" | "pollenpuff" | "razorwind" |
                "shadowforce" | "shelltrap" | "skullbash" | "skyattack" | "solarbeam" |
                "solarblade"
            ),
            "onUseMoveMessage" => matches!(
                move_id,
                "magnitude"
            ),
            _ => false,
        }
    }

    /// Check if a condition has a callback for an event
    ///
    /// In JavaScript, this is done by checking if effect[callbackName] exists.
    /// In Rust, we check against the dispatcher implementations to see which
    /// conditions actually have which callbacks.
    ///
    /// This prevents false positives where we'd add handlers for callbacks
    /// that don't exist, which would cause incorrect speed_sort shuffling.
    pub fn condition_has_callback(&self, condition_id: &str, event_id: &str) -> bool {
        // Normalize event name to "onEventName" format for lookup
        let normalized = if event_id.starts_with("on") {
            event_id.to_string()
        } else {
            format!("on{}", event_id)
        };

        eprintln!("[CONDITION_HAS_CALLBACK] condition_id='{}', event_id='{}', normalized='{}'",
            condition_id, event_id, normalized);

        // Special case: conditions don't have onAnySwitchIn
        if normalized == "onAnySwitchIn" {
            return false;
        }

        // Look up the condition in dex data and check its extra field for callback boolean
        let id = ID::from(condition_id);
        let result = if let Some(condition_data) = self.dex.conditions.get(&id) {
            // Check if the callback key exists and is true
            condition_data.extra.get(&normalized)
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
        } else {
            // If not found in dex, return false
            false
        };

        eprintln!("[CONDITION_HAS_CALLBACK] Returning {} for condition='{}', normalized='{}'",
            result, condition_id, normalized);
        result
    }

    /// Check if a species has a callback for an event
    pub fn species_has_callback(&self, _species_id: &str, event_id: &str) -> bool {
        // Species don't have onAnySwitchIn
        if event_id == "onAnySwitchIn" {
            return false;
        }

        // For other events, conservatively return false by default
        false
    }
}
