use crate::*;

impl Battle {

    /// Check if an effect has a callback for a specific event
    /// This is a Rust helper to replicate JavaScript's getCallback() check
    /// without actually executing the callback
    ///
    /// Returns true if the effect has a handler for the event, false otherwise
    pub fn has_callback(&self, effect_id: &ID, event_id: &str) -> bool {
        let effect_str = effect_id.as_str();

        // Check abilities
        if self.dex.abilities().get(effect_str).is_some() {
            return self.ability_has_callback(effect_str, event_id);
        }

        // Check items
        if self.dex.items().get(effect_str).is_some() {
            return self.item_has_callback(effect_str, event_id);
        }

        // Check moves
        if self.dex.moves().get(effect_str).is_some() {
            return self.move_has_callback(effect_str, event_id);
        }

        // Check conditions (status, volatile, weather, terrain)
        if crate::data::conditions::get_condition(effect_id).is_some() {
            return self.condition_has_callback(effect_str, event_id);
        }

        // Check species - species can have callbacks like onSwitchIn for form changes
        if self.dex.species().get(effect_str).is_some() {
            return self.species_has_callback(effect_str, event_id);
        }

        false
    }

    /// Check if a an ability has a callback for an event
    fn ability_has_callback(&self, ability_id: &str, event_id: &str) -> bool {
        // Check for AllyBasePower event
        if event_id == "AllyBasePower" {
            return matches!(
                ability_id,
                "battery" | "powerspot" | "steelyspirit"
            );
        }

        // Check for AnyBasePower event
        if event_id == "AnyBasePower" {
            return matches!(
                ability_id,
                "darkaura" | "fairyaura"
            );
        }

        // Check for BasePower event
        if event_id == "BasePower" {
            return matches!(
                ability_id,
                "aerilate" | "analytic" | "flareboost" | "galvanize" | "ironfist" |
                "megalauncher" | "normalize" | "pixilate" | "punkrock" | "reckless" |
                "refrigerate" | "rivalry" | "sandforce" | "sharpness" | "sheerforce" |
                "strongjaw" | "supremeoverlord" | "technician" | "toughclaws" | "toxicboost"
            );
        }

        // Check for SourceBasePower event
        if event_id == "SourceBasePower" {
            return matches!(
                ability_id,
                "dryskin"
            );
        }

        // Check for onAfterBoost event
        if event_id == "onAfterBoost" {
            return matches!(
                ability_id,
                "rattled"
            );
        }

        // Check for onAfterEachBoost event
        if event_id == "onAfterEachBoost" {
            return matches!(
                ability_id,
                "competitive" | "defiant"
            );
        }

        // Check for onAfterMoveSecondary event
        if event_id == "onAfterMoveSecondary" {
            return matches!(
                ability_id,
                "angershell" | "berserk" | "colorchange" | "pickpocket"
            );
        }

        // Check for onAfterMoveSecondarySelf event
        if event_id == "onAfterMoveSecondarySelf" {
            return matches!(
                ability_id,
                "magician"
            );
        }

        // Check for onAfterSetStatus event
        if event_id == "onAfterSetStatus" {
            return matches!(
                ability_id,
                "synchronize"
            );
        }

        // Check for onAfterTerastallization event
        if event_id == "onAfterTerastallization" {
            return matches!(
                ability_id,
                "teraformzero"
            );
        }

        // Check for onAfterUseItem event
        if event_id == "onAfterUseItem" {
            return matches!(
                ability_id,
                "unburden"
            );
        }

        // Check for onAllyAfterUseItem event
        if event_id == "onAllyAfterUseItem" {
            return matches!(
                ability_id,
                "symbiosis"
            );
        }

        // Check for onAllyFaint event
        if event_id == "onAllyFaint" {
            return matches!(
                ability_id,
                "powerofalchemy" | "receiver"
            );
        }

        // Check for onAllyModifyAtk event
        if event_id == "onAllyModifyAtk" {
            return matches!(
                ability_id,
                "flowergift"
            );
        }

        // Check for onAllyModifySpD event
        if event_id == "onAllyModifySpD" {
            return matches!(
                ability_id,
                "flowergift"
            );
        }

        // Check for onAllySetStatus event
        if event_id == "onAllySetStatus" {
            return matches!(
                ability_id,
                "flowerveil" | "pastelveil" | "sweetveil"
            );
        }

        // Check for onAllyTryAddVolatile event
        if event_id == "onAllyTryAddVolatile" {
            return matches!(
                ability_id,
                "aromaveil" | "flowerveil" | "sweetveil"
            );
        }

        // Check for onAllyTryBoost event
        if event_id == "onAllyTryBoost" {
            return matches!(
                ability_id,
                "flowerveil"
            );
        }

        // Check for onAllyTryHitSide event
        if event_id == "onAllyTryHitSide" {
            return matches!(
                ability_id,
                "magicbounce" | "rebound" | "sapsipper" | "soundproof"
            );
        }

        // Check for onAnyAccuracy event
        if event_id == "onAnyAccuracy" {
            return matches!(
                ability_id,
                "noguard"
            );
        }

        // Check for onAnyAfterMega event
        if event_id == "onAnyAfterMega" {
            return matches!(
                ability_id,
                "opportunist"
            );
        }

        // Check for onAnyAfterMove event
        if event_id == "onAnyAfterMove" {
            return matches!(
                ability_id,
                "opportunist" | "terashell"
            );
        }

        // Check for onAnyAfterSetStatus event
        if event_id == "onAnyAfterSetStatus" {
            return matches!(
                ability_id,
                "poisonpuppeteer"
            );
        }

        // Check for onAnyAfterTerastallization event
        if event_id == "onAnyAfterTerastallization" {
            return matches!(
                ability_id,
                "opportunist"
            );
        }

        // Check for onAnyBeforeMove event
        if event_id == "onAnyBeforeMove" {
            return matches!(
                ability_id,
                "terashell"
            );
        }

        // Check for onAnyDamage event
        if event_id == "onAnyDamage" {
            return matches!(
                ability_id,
                "damp"
            );
        }

        // Check for onAnyFaint event
        if event_id == "onAnyFaint" {
            return matches!(
                ability_id,
                "soulheart"
            );
        }

        // Check for onAnyInvulnerability event
        if event_id == "onAnyInvulnerability" {
            return matches!(
                ability_id,
                "noguard"
            );
        }

        // Check for onAnyModifyAccuracy event
        if event_id == "onAnyModifyAccuracy" {
            return matches!(
                ability_id,
                "victorystar"
            );
        }

        // Check for onAnyModifyAtk event
        if event_id == "onAnyModifyAtk" {
            return matches!(
                ability_id,
                "tabletsofruin"
            );
        }

        // Check for onAnyModifyBoost event
        if event_id == "onAnyModifyBoost" {
            return matches!(
                ability_id,
                "unaware"
            );
        }

        // Check for onAnyModifyDamage event
        if event_id == "onAnyModifyDamage" {
            return matches!(
                ability_id,
                "friendguard"
            );
        }

        // Check for onAnyModifyDef event
        if event_id == "onAnyModifyDef" {
            return matches!(
                ability_id,
                "swordofruin"
            );
        }

        // Check for onAnyModifySpA event
        if event_id == "onAnyModifySpA" {
            return matches!(
                ability_id,
                "vesselofruin"
            );
        }

        // Check for onAnyModifySpD event
        if event_id == "onAnyModifySpD" {
            return matches!(
                ability_id,
                "beadsofruin"
            );
        }

        // Check for onAnyRedirectTarget event
        if event_id == "onAnyRedirectTarget" {
            return matches!(
                ability_id,
                "lightningrod" | "stormdrain"
            );
        }

        // Check for onAnySetWeather event
        if event_id == "onAnySetWeather" {
            return matches!(
                ability_id,
                "deltastream" | "desolateland" | "primordialsea"
            );
        }

        // Check for onAnySwitchIn event
        if event_id == "onAnySwitchIn" {
            return matches!(
                ability_id,
                "commander" | "opportunist" | "pastelveil"
            );
        }

        // Check for onAnyTryMove event
        if event_id == "onAnyTryMove" {
            return matches!(
                ability_id,
                "damp"
            );
        }

        // Check for onAnyTryPrimaryHit event
        if event_id == "onAnyTryPrimaryHit" {
            return matches!(
                ability_id,
                "aurabreak"
            );
        }

        // Check for onBeforeMove event
        if event_id == "onBeforeMove" {
            return matches!(
                ability_id,
                "gorillatactics" | "truant"
            );
        }

        // Check for onBeforeSwitchIn event
        if event_id == "onBeforeSwitchIn" {
            return matches!(
                ability_id,
                "illusion"
            );
        }

        // Check for onChangeBoost event
        if event_id == "onChangeBoost" {
            return matches!(
                ability_id,
                "contrary" | "ripen" | "simple"
            );
        }

        // Check for onCheckShow event
        if event_id == "onCheckShow" {
            return matches!(
                ability_id,
                "naturalcure"
            );
        }

        // Check for onCriticalHit event
        if event_id == "onCriticalHit" {
            return matches!(
                ability_id,
                "disguise" | "iceface"
            );
        }

        // Check for onDamage event
        if event_id == "onDamage" {
            return matches!(
                ability_id,
                "angershell" | "berserk" | "disguise" | "gluttony" | "heatproof" |
                "iceface" | "magicguard" | "mountaineer" | "poisonheal" | "rockhead" |
                "sturdy"
            );
        }

        // Check for onDamagingHit event
        if event_id == "onDamagingHit" {
            return matches!(
                ability_id,
                "aftermath" | "cottondown" | "cursedbody" | "cutecharm" | "effectspore" |
                "electromorphosis" | "flamebody" | "gooey" | "gulpmissile" | "illusion" |
                "innardsout" | "ironbarbs" | "justified" | "lingeringaroma" | "mummy" |
                "perishbody" | "poisonpoint" | "rattled" | "roughskin" | "sandspit" |
                "seedsower" | "stamina" | "static" | "steamengine" | "tanglinghair" |
                "thermalexchange" | "toxicdebris" | "wanderingspirit" | "watercompaction" | "weakarmor" |
                "windpower"
            );
        }

        // Check for onDeductPP event
        if event_id == "onDeductPP" {
            return matches!(
                ability_id,
                "pressure"
            );
        }

        // Check for onDisableMove event
        if event_id == "onDisableMove" {
            return matches!(
                ability_id,
                "gorillatactics"
            );
        }

        // Check for onDragOut event
        if event_id == "onDragOut" {
            return matches!(
                ability_id,
                "guarddog" | "suctioncups"
            );
        }

        // Check for onEatItem event
        if event_id == "onEatItem" {
            return matches!(
                ability_id,
                "cheekpouch" | "cudchew" | "ripen"
            );
        }

        // Check for onEffectiveness event
        if event_id == "onEffectiveness" {
            return matches!(
                ability_id,
                "disguise" | "iceface"
            );
        }

        // Check for onEmergencyExit event
        if event_id == "onEmergencyExit" {
            return matches!(
                ability_id,
                "emergencyexit" | "wimpout"
            );
        }

        // Check for onEnd event
        if event_id == "onEnd" {
            return matches!(
                ability_id,
                "airlock" | "asoneglastrier" | "asonespectrier" | "cloudnine" | "deltastream" |
                "desolateland" | "flashfire" | "gorillatactics" | "illusion" | "neutralizinggas" |
                "opportunist" | "primordialsea" | "protosynthesis" | "quarkdrive" | "supremeoverlord" |
                "unburden" | "unnerve" | "zenmode"
            );
        }

        // Check for onFaint event
        if event_id == "onFaint" {
            return matches!(
                ability_id,
                "illusion"
            );
        }

        // Check for onFlinch event
        if event_id == "onFlinch" {
            return matches!(
                ability_id,
                "steadfast"
            );
        }

        // Check for onFoeAfterBoost event
        if event_id == "onFoeAfterBoost" {
            return matches!(
                ability_id,
                "opportunist"
            );
        }

        // Check for onFoeMaybeTrapPokemon event
        if event_id == "onFoeMaybeTrapPokemon" {
            return matches!(
                ability_id,
                "arenatrap" | "magnetpull" | "shadowtag"
            );
        }

        // Check for onFoeTrapPokemon event
        if event_id == "onFoeTrapPokemon" {
            return matches!(
                ability_id,
                "arenatrap" | "magnetpull" | "shadowtag"
            );
        }

        // Check for onFoeTryEatItem event
        if event_id == "onFoeTryEatItem" {
            return matches!(
                ability_id,
                "asoneglastrier" | "asonespectrier" | "unnerve"
            );
        }

        // Check for onFoeTryMove event
        if event_id == "onFoeTryMove" {
            return matches!(
                ability_id,
                "armortail" | "dazzling" | "queenlymajesty"
            );
        }

        // Check for onHit event
        if event_id == "onHit" {
            return matches!(
                ability_id,
                "angerpoint" | "owntempo"
            );
        }

        // Check for onImmunity event
        if event_id == "onImmunity" {
            return matches!(
                ability_id,
                "icebody" | "magmaarmor" | "oblivious" | "overcoat" | "sandforce" |
                "sandrush" | "sandveil" | "snowcloak"
            );
        }

        // Check for onModifyAccuracy event
        if event_id == "onModifyAccuracy" {
            return matches!(
                ability_id,
                "sandveil" | "snowcloak" | "tangledfeet" | "wonderskin"
            );
        }

        // Check for onModifyAtk event
        if event_id == "onModifyAtk" {
            return matches!(
                ability_id,
                "blaze" | "defeatist" | "dragonsmaw" | "gorillatactics" | "guts" |
                "hugepower" | "hustle" | "orichalcumpulse" | "overgrow" | "purepower" |
                "rockypayload" | "slowstart" | "stakeout" | "steelworker" | "swarm" |
                "torrent" | "transistor" | "waterbubble"
            );
        }

        // Check for onModifyCritRatio event
        if event_id == "onModifyCritRatio" {
            return matches!(
                ability_id,
                "merciless" | "superluck"
            );
        }

        // Check for onModifyDamage event
        if event_id == "onModifyDamage" {
            return matches!(
                ability_id,
                "neuroforce" | "sniper" | "tintedlens"
            );
        }

        // Check for onModifyDef event
        if event_id == "onModifyDef" {
            return matches!(
                ability_id,
                "furcoat" | "grasspelt" | "marvelscale"
            );
        }

        // Check for onModifyMove event
        if event_id == "onModifyMove" {
            return matches!(
                ability_id,
                "battlebond" | "gorillatactics" | "illuminate" | "infiltrator" | "keeneye" |
                "longreach" | "mindseye" | "moldbreaker" | "myceliummight" | "propellertail" |
                "scrappy" | "serenegrace" | "sheerforce" | "skilllink" | "stalwart" |
                "stancechange" | "stench" | "teravolt" | "turboblaze" | "unseenfist"
            );
        }

        // Check for onModifySTAB event
        if event_id == "onModifySTAB" {
            return matches!(
                ability_id,
                "adaptability"
            );
        }

        // Check for onModifySecondaries event
        if event_id == "onModifySecondaries" {
            return matches!(
                ability_id,
                "shielddust"
            );
        }

        // Check for onModifySpA event
        if event_id == "onModifySpA" {
            return matches!(
                ability_id,
                "blaze" | "defeatist" | "dragonsmaw" | "hadronengine" | "minus" |
                "overgrow" | "plus" | "rockypayload" | "solarpower" | "stakeout" |
                "steelworker" | "swarm" | "torrent" | "transistor" | "waterbubble"
            );
        }

        // Check for onModifySpe event
        if event_id == "onModifySpe" {
            return matches!(
                ability_id,
                "chlorophyll" | "quickfeet" | "sandrush" | "slowstart" | "slushrush" |
                "surgesurfer" | "swiftswim"
            );
        }

        // Check for onModifyType event
        if event_id == "onModifyType" {
            return matches!(
                ability_id,
                "aerilate" | "galvanize" | "liquidvoice" | "normalize" | "pixilate" |
                "refrigerate"
            );
        }

        // Check for onModifyWeight event
        if event_id == "onModifyWeight" {
            return matches!(
                ability_id,
                "heavymetal" | "lightmetal"
            );
        }

        // Check for onPrepareHit event
        if event_id == "onPrepareHit" {
            return matches!(
                ability_id,
                "libero" | "parentalbond" | "protean"
            );
        }

        // Check for onResidual event
        if event_id == "onResidual" {
            return matches!(
                ability_id,
                "baddreams" | "cudchew" | "harvest" | "healer" | "hungerswitch" |
                "hydration" | "moody" | "opportunist" | "pickup" | "powerconstruct" |
                "schooling" | "shedskin" | "shieldsdown" | "slowstart" | "speedboost" |
                "zenmode"
            );
        }

        // Check for onSetStatus event
        if event_id == "onSetStatus" {
            return matches!(
                ability_id,
                "comatose" | "immunity" | "insomnia" | "leafguard" | "limber" |
                "pastelveil" | "purifyingsalt" | "shieldsdown" | "thermalexchange" | "vitalspirit" |
                "waterbubble" | "waterveil"
            );
        }

        // Check for onSideConditionStart event
        if event_id == "onSideConditionStart" {
            return matches!(
                ability_id,
                "windpower" | "windrider"
            );
        }

        // Check for onSourceAfterFaint event
        if event_id == "onSourceAfterFaint" {
            return matches!(
                ability_id,
                "asoneglastrier" | "asonespectrier" | "battlebond" | "beastboost" | "chillingneigh" |
                "grimneigh" | "moxie"
            );
        }

        // Check for onSourceDamagingHit event
        if event_id == "onSourceDamagingHit" {
            return matches!(
                ability_id,
                "poisontouch" | "toxicchain"
            );
        }

        // Check for onSourceModifyAccuracy event
        if event_id == "onSourceModifyAccuracy" {
            return matches!(
                ability_id,
                "compoundeyes" | "hustle"
            );
        }

        // Check for onSourceModifyAtk event
        if event_id == "onSourceModifyAtk" {
            return matches!(
                ability_id,
                "heatproof" | "purifyingsalt" | "thickfat" | "waterbubble"
            );
        }

        // Check for onSourceModifyDamage event
        if event_id == "onSourceModifyDamage" {
            return matches!(
                ability_id,
                "filter" | "fluffy" | "icescales" | "multiscale" | "prismarmor" |
                "punkrock" | "ripen" | "shadowshield" | "solidrock"
            );
        }

        // Check for onSourceModifySecondaries event
        if event_id == "onSourceModifySecondaries" {
            return matches!(
                ability_id,
                "parentalbond"
            );
        }

        // Check for onSourceModifySpA event
        if event_id == "onSourceModifySpA" {
            return matches!(
                ability_id,
                "heatproof" | "purifyingsalt" | "thickfat" | "waterbubble"
            );
        }

        // Check for onSourceTryHeal event
        if event_id == "onSourceTryHeal" {
            return matches!(
                ability_id,
                "liquidooze"
            );
        }

        // Check for onSourceTryPrimaryHit event
        if event_id == "onSourceTryPrimaryHit" {
            return matches!(
                ability_id,
                "gulpmissile"
            );
        }

        // Check for onStart event
        if event_id == "onStart" {
            return matches!(
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
            );
        }

        // Check for onSwitchIn event
        if event_id == "onSwitchIn" {
            return matches!(
                ability_id,
                "airlock" | "cloudnine" | "imposter" | "neutralizinggas" | "terashift" |
                "zerotohero"
            );
        }

        // Check for onSwitchOut event
        if event_id == "onSwitchOut" {
            return matches!(
                ability_id,
                "naturalcure" | "regenerator" | "zerotohero"
            );
        }

        // Check for onTakeItem event
        if event_id == "onTakeItem" {
            return matches!(
                ability_id,
                "stickyhold" | "unburden"
            );
        }

        // Check for onTerrainChange event
        if event_id == "onTerrainChange" {
            return matches!(
                ability_id,
                "mimicry" | "quarkdrive"
            );
        }

        // Check for onTryAddVolatile event
        if event_id == "onTryAddVolatile" {
            return matches!(
                ability_id,
                "innerfocus" | "insomnia" | "leafguard" | "owntempo" | "purifyingsalt" |
                "shieldsdown" | "vitalspirit"
            );
        }

        // Check for onTryBoost event
        if event_id == "onTryBoost" {
            return matches!(
                ability_id,
                "bigpecks" | "clearbody" | "fullmetalbody" | "guarddog" | "hypercutter" |
                "illuminate" | "innerfocus" | "keeneye" | "mindseye" | "mirrorarmor" |
                "oblivious" | "owntempo" | "scrappy" | "whitesmoke"
            );
        }

        // Check for onTryEatItem event
        if event_id == "onTryEatItem" {
            return matches!(
                ability_id,
                "angershell" | "berserk" | "ripen"
            );
        }

        // Check for onTryHeal event
        if event_id == "onTryHeal" {
            return matches!(
                ability_id,
                "ripen"
            );
        }

        // Check for onTryHit event
        if event_id == "onTryHit" {
            return matches!(
                ability_id,
                "bulletproof" | "dryskin" | "eartheater" | "flashfire" | "goodasgold" |
                "lightningrod" | "magicbounce" | "motordrive" | "mountaineer" | "oblivious" |
                "overcoat" | "rebound" | "sapsipper" | "soundproof" | "stormdrain" |
                "sturdy" | "telepathy" | "voltabsorb" | "waterabsorb" | "wellbakedbody" |
                "windrider" | "wonderguard"
            );
        }

        // Check for onUpdate event
        if event_id == "onUpdate" {
            return matches!(
                ability_id,
                "commander" | "disguise" | "iceface" | "immunity" | "insomnia" |
                "limber" | "magmaarmor" | "oblivious" | "owntempo" | "pastelveil" |
                "thermalexchange" | "trace" | "vitalspirit" | "waterbubble" | "waterveil"
            );
        }

        // Check for onWeather event
        if event_id == "onWeather" {
            return matches!(
                ability_id,
                "dryskin" | "icebody" | "raindish" | "solarpower"
            );
        }

        // Check for onWeatherChange event
        if event_id == "onWeatherChange" {
            return matches!(
                ability_id,
                "flowergift" | "forecast" | "iceface" | "protosynthesis"
            );
        }

        false
    }

    /// Check if a an item has a callback for an event
    fn item_has_callback(&self, item_id: &str, event_id: &str) -> bool {
        // Check for onAfterBoost event
        if event_id == "onAfterBoost" {
            return matches!(
                item_id,
                "adrenalineorb" | "ejectpack"
            );
        }

        // Check for onAnyAfterMega event
        if event_id == "onAnyAfterMega" {
            return matches!(
                item_id,
                "ejectpack" | "mirrorherb" | "whiteherb"
            );
        }

        // Check for onAnyAfterMove event
        if event_id == "onAnyAfterMove" {
            return matches!(
                item_id,
                "ejectpack" | "mirrorherb" | "whiteherb"
            );
        }

        // Check for onAnyAfterTerastallization event
        if event_id == "onAnyAfterTerastallization" {
            return matches!(
                item_id,
                "mirrorherb"
            );
        }

        // Check for onAnyPseudoWeatherChange event
        if event_id == "onAnyPseudoWeatherChange" {
            return matches!(
                item_id,
                "roomservice"
            );
        }

        // Check for onAnySwitchIn event
        if event_id == "onAnySwitchIn" {
            return matches!(
                item_id,
                "ejectpack" | "mirrorherb" | "whiteherb"
            );
        }

        // Check for onChargeMove event
        if event_id == "onChargeMove" {
            return matches!(
                item_id,
                "powerherb"
            );
        }

        // Check for onEffectiveness event
        if event_id == "onEffectiveness" {
            return matches!(
                item_id,
                "ironball"
            );
        }

        // Check for onEnd event
        if event_id == "onEnd" {
            return matches!(
                item_id,
                "ejectpack" | "mirrorherb" | "utilityumbrella"
            );
        }

        // Check for onFoeAfterBoost event
        if event_id == "onFoeAfterBoost" {
            return matches!(
                item_id,
                "mirrorherb"
            );
        }

        // Check for onImmunity event
        if event_id == "onImmunity" {
            return matches!(
                item_id,
                "safetygoggles"
            );
        }

        // Check for onModifyCritRatio event
        if event_id == "onModifyCritRatio" {
            return matches!(
                item_id,
                "leek" | "luckypunch" | "razorclaw" | "scopelens" | "stick"
            );
        }

        // Check for onModifySecondaries event
        if event_id == "onModifySecondaries" {
            return matches!(
                item_id,
                "covertcloak"
            );
        }

        // Check for onModifySpe event
        if event_id == "onModifySpe" {
            return matches!(
                item_id,
                "ironball"
            );
        }

        // Check for onModifyWeight event
        if event_id == "onModifyWeight" {
            return matches!(
                item_id,
                "floatstone"
            );
        }

        // Check for onResidual event
        if event_id == "onResidual" {
            return matches!(
                item_id,
                "ejectpack" | "mirrorherb" | "whiteherb"
            );
        }

        // Check for onStart event
        if event_id == "onStart" {
            return matches!(
                item_id,
                "boosterenergy" | "roomservice" | "utilityumbrella"
            );
        }

        // Check for onSwitchIn event
        if event_id == "onSwitchIn" {
            return matches!(
                item_id,
                "blueorb" | "redorb"
            );
        }

        // Check for onTakeItem event
        if event_id == "onTakeItem" {
            return matches!(
                item_id,
                "blueorb" | "boosterenergy" | "redorb"
            );
        }

        // Check for onTryBoost event
        if event_id == "onTryBoost" {
            return matches!(
                item_id,
                "clearamulet"
            );
        }

        // Check for onTryHit event
        if event_id == "onTryHit" {
            return matches!(
                item_id,
                "safetygoggles"
            );
        }

        // Check for onUpdate event
        if event_id == "onUpdate" {
            return matches!(
                item_id,
                "boosterenergy" | "utilityumbrella"
            );
        }

        // Check for onUse event
        if event_id == "onUse" {
            return matches!(
                item_id,
                "ejectpack" | "mirrorherb" | "whiteherb"
            );
        }

        // Check for onUseItem event
        if event_id == "onUseItem" {
            return matches!(
                item_id,
                "ejectpack"
            );
        }

        false
    }

    /// Check if a a move has a callback for an event
    fn move_has_callback(&self, move_id: &str, event_id: &str) -> bool {
        // Check for BasePower event
        if event_id == "BasePower" {
            return matches!(
                move_id,
                "barbbarrage" | "brine" | "collisioncourse" | "electrodrift" | "expandingforce" |
                "facade" | "ficklebeam" | "fusionbolt" | "fusionflare" | "gravapple" |
                "knockoff" | "lashout" | "mistyexplosion" | "psyblade" | "retaliate" |
                "solarbeam" | "solarblade" | "venoshock"
            );
        }

        // Check for onAfterHit event
        if event_id == "onAfterHit" {
            return matches!(
                move_id,
                "ceaselessedge" | "covet" | "icespinner" | "knockoff" | "mortalspin" |
                "rapidspin" | "stoneaxe" | "thief"
            );
        }

        // Check for onAfterMove event
        if event_id == "onAfterMove" {
            return matches!(
                move_id,
                "beakblast" | "iceball" | "mindblown" | "rollout" | "sparklingaria" |
                "spitup" | "steelbeam"
            );
        }

        // Check for onAfterMoveSecondarySelf event
        if event_id == "onAfterMoveSecondarySelf" {
            return matches!(
                move_id,
                "fellstinger" | "orderup" | "polarflare" | "relicsong"
            );
        }

        // Check for onAfterSubDamage event
        if event_id == "onAfterSubDamage" {
            return matches!(
                move_id,
                "ceaselessedge" | "coreenforcer" | "flameburst" | "gmaxsnooze" | "icespinner" |
                "mortalspin" | "rapidspin" | "shellsidearm" | "splinteredstormshards" | "steelroller" |
                "stoneaxe"
            );
        }

        // Check for onDamage event
        if event_id == "onDamage" {
            return matches!(
                move_id,
                "falseswipe" | "holdback"
            );
        }

        // Check for onDisableMove event
        if event_id == "onDisableMove" {
            return matches!(
                move_id,
                "belch" | "stuffcheeks"
            );
        }

        // Check for onEffectiveness event
        if event_id == "onEffectiveness" {
            return matches!(
                move_id,
                "flyingpress" | "freezedry" | "thousandarrows"
            );
        }

        // Check for onHit event
        if event_id == "onHit" {
            return matches!(
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
            );
        }

        // Check for onHitField event
        if event_id == "onHitField" {
            return matches!(
                move_id,
                "courtchange" | "flowershield" | "haze" | "perishsong" | "rototiller" |
                "teatime"
            );
        }

        // Check for onHitSide event
        if event_id == "onHitSide" {
            return matches!(
                move_id,
                "gearup" | "magneticflux" | "quickguard" | "wideguard"
            );
        }

        // Check for onModifyMove event
        if event_id == "onModifyMove" {
            return matches!(
                move_id,
                "beatup" | "bleakwindstorm" | "blizzard" | "curse" | "expandingforce" |
                "firepledge" | "grasspledge" | "growth" | "hurricane" | "iceball" |
                "lightthatburnsthesky" | "magnitude" | "photongeyser" | "present" | "pursuit" |
                "rollout" | "sandsearstorm" | "secretpower" | "shellsidearm" | "skydrop" |
                "struggle" | "terablast" | "terastarstorm" | "terrainpulse" | "thunder" |
                "waterpledge" | "weatherball" | "wildboltstorm"
            );
        }

        // Check for onModifyTarget event
        if event_id == "onModifyTarget" {
            return matches!(
                move_id,
                "comeuppance" | "metalburst"
            );
        }

        // Check for onModifyType event
        if event_id == "onModifyType" {
            return matches!(
                move_id,
                "aurawheel" | "hiddenpower" | "ivycudgel" | "judgment" | "multiattack" |
                "naturalgift" | "ragingbull" | "revelationdance" | "technoblast" | "terablast" |
                "terastarstorm" | "terrainpulse" | "weatherball"
            );
        }

        // Check for onMoveFail event
        if event_id == "onMoveFail" {
            return matches!(
                move_id,
                "axekick" | "highjumpkick" | "jumpkick" | "skydrop" | "supercellslam"
            );
        }

        // Check for onPrepareHit event
        if event_id == "onPrepareHit" {
            return matches!(
                move_id,
                "allyswitch" | "banefulbunker" | "burningbulwark" | "destinybond" | "detect" |
                "endure" | "firepledge" | "fling" | "grasspledge" | "ivycudgel" |
                "kingsshield" | "maxguard" | "naturalgift" | "obstruct" | "protect" |
                "shellsidearm" | "silktrap" | "spikyshield" | "terablast" | "waterpledge"
            );
        }

        // Check for onTry event
        if event_id == "onTry" {
            return matches!(
                move_id,
                "aurawheel" | "auroraveil" | "clangoroussoul" | "comeuppance" | "counter" |
                "craftyshield" | "darkvoid" | "doomdesire" | "fakeout" | "filletaway" |
                "firstimpression" | "followme" | "futuresight" | "hyperspacefury" | "lastresort" |
                "magnetrise" | "matblock" | "metalburst" | "mirrorcoat" | "noretreat" |
                "poltergeist" | "quickguard" | "ragepowder" | "rest" | "round" |
                "skydrop" | "sleeptalk" | "snore" | "spitup" | "splash" |
                "steelroller" | "stockpile" | "stuffcheeks" | "suckerpunch" | "swallow" |
                "telekinesis" | "teleport" | "thunderclap" | "upperhand" | "wideguard"
            );
        }

        // Check for onTryHit event
        if event_id == "onTryHit" {
            return matches!(
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
            );
        }

        // Check for onTryImmunity event
        if event_id == "onTryImmunity" {
            return matches!(
                move_id,
                "attract" | "captivate" | "dreameater" | "endeavor" | "leechseed" |
                "octolock" | "switcheroo" | "synchronoise" | "trick" | "worryseed"
            );
        }

        // Check for onTryMove event
        if event_id == "onTryMove" {
            return matches!(
                move_id,
                "bounce" | "burnup" | "dig" | "dive" | "doubleshock" |
                "echoedvoice" | "electroshot" | "fly" | "freezeshock" | "geomancy" |
                "iceburn" | "meteorbeam" | "phantomforce" | "pollenpuff" | "razorwind" |
                "shadowforce" | "shelltrap" | "skullbash" | "skyattack" | "solarbeam" |
                "solarblade"
            );
        }

        // Check for onUseMoveMessage event
        if event_id == "onUseMoveMessage" {
            return matches!(
                move_id,
                "magnitude"
            );
        }

        false
    }

    /// Check if a condition has a callback for an event
    fn condition_has_callback(&self, _condition_id: &str, event_id: &str) -> bool {
        // Conditions don't have onAnySwitchIn
        if event_id == "onAnySwitchIn" {
            return false;
        }

        // For other events, conservatively return false by default
        false
    }

    /// Check if a species has a callback for an event
    fn species_has_callback(&self, _species_id: &str, event_id: &str) -> bool {
        // Species don't have onAnySwitchIn
        if event_id == "onAnySwitchIn" {
            return false;
        }

        // For other events, conservatively return false by default
        false
    }
}
