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

    /// Check if an ability has a callback for an event
    fn ability_has_callback(&self, ability_id: &str, event_id: &str) -> bool {
        // Special case: onAnySwitchIn is only for abilities that trigger when ANY Pokemon switches in
        // Examples: Intimidate, Trace, Download, etc.
        if event_id == "onAnySwitchIn" {
            // Only specific abilities have onAnySwitchIn
            return matches!(ability_id, "intimidate" | "trace" | "download" | "frisk" | "forewarn" | "anticipation");
        }

        // Check for BasePower event
        if event_id == "BasePower" {
            // Abilities with onBasePower callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(
                ability_id,
                "aerilate" | "analytic" | "flareboost" | "galvanize" | "ironfist"
                | "megalauncher" | "normalize" | "pixilate" | "punkrock" | "reckless"
                | "refrigerate" | "rivalry" | "sandforce" | "sharpness" | "sheerforce"
                | "strongjaw" | "supremeoverlord" | "technician" | "toughclaws" | "toxicboost"
            );
        }

        // Check for onSourceBasePower event (when this Pokemon is being hit)
        if event_id == "onSourceBasePower" {
            // Abilities with onSourceBasePower callbacks
            return matches!(ability_id, "dryskin");
        }

        // Check for onAllyBasePower event (when an ally is attacking)
        if event_id == "onAllyBasePower" {
            // Abilities with onAllyBasePower callbacks
            return matches!(ability_id, "battery" | "powerspot" | "steelyspirit");
        }

        // Check for onResidual event (end-of-turn effects)
        if event_id == "onResidual" {
            // Abilities with onResidual callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(
                ability_id,
                "baddreams" | "cudchew" | "harvest" | "healer" | "hungerswitch"
                | "hydration" | "moody" | "opportunist" | "pickup" | "powerconstruct"
                | "schooling" | "shedskin" | "shieldsdown" | "slowstart"
                | "speedboost" | "zenmode"
            );
        }

        // Check for onSwitchIn event (when Pokemon switches in)
        if event_id == "onSwitchIn" {
            // Abilities with onSwitchIn callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(
                ability_id,
                "airlock" | "cloudnine" | "imposter" | "neutralizinggas" | "terashift" | "zerotohero"
            );
        }

        // Check for onSwitchOut event (when Pokemon switches out)
        if event_id == "onSwitchOut" {
            // Abilities with onSwitchOut callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "naturalcure" | "regenerator" | "zerotohero");
        }

        // Check for onDamage event (damage calculation effects)
        if event_id == "onDamage" {
            // Abilities with onDamage callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(
                ability_id,
                "angershell" | "berserk" | "disguise" | "gluttony" | "heatproof"
                | "iceface" | "magicguard" | "mountaineer" | "poisonheal" | "rockhead" | "sturdy"
            );
        }

        // Check for onStart event (ability/effect activation)
        if event_id == "onStart" {
            // Abilities with onStart callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(
                ability_id,
                "airlock" | "anticipation" | "asoneglastrier" | "asonespectrier" | "aurabreak"
                | "beadsofruin" | "cloudnine" | "comatose" | "commander" | "costar"
                | "curiousmedicine" | "darkaura" | "dauntlessshield" | "deltastream" | "desolateland"
                | "download" | "drizzle" | "drought" | "electricsurge" | "embodyaspectcornerstone"
                | "embodyaspecthearthflame" | "embodyaspectteal" | "embodyaspectwellspring" | "fairyaura"
                | "flowergift" | "forecast" | "forewarn" | "frisk" | "gluttony"
                | "gorillatactics" | "grassysurge" | "hadronengine" | "hospitality" | "iceface"
                | "intimidate" | "intrepidsword" | "klutz" | "mimicry" | "mistysurge"
                | "moldbreaker" | "orichalcumpulse" | "pastelveil" | "pressure" | "primordialsea"
                | "protosynthesis" | "psychicsurge" | "quarkdrive" | "sandstream" | "schooling"
                | "screencleaner" | "shieldsdown" | "slowstart" | "snowwarning" | "supersweetsyrup"
                | "supremeoverlord" | "swordofruin" | "tabletsofruin" | "teravolt" | "trace"
                | "truant" | "turboblaze" | "unnerve" | "vesselofruin" | "windrider"
            );
        }

        // Check for onEnd event (ability/effect deactivation)
        if event_id == "onEnd" {
            // Abilities with onEnd callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(
                ability_id,
                "airlock" | "asoneglastrier" | "asonespectrier" | "cloudnine" | "deltastream"
                | "desolateland" | "flashfire" | "gorillatactics" | "illusion" | "neutralizinggas"
                | "opportunist" | "primordialsea" | "protosynthesis" | "quarkdrive" | "supremeoverlord"
                | "unburden" | "unnerve" | "zenmode"
            );
        }

        // Check for onUpdate event (turn-based updates)
        if event_id == "onUpdate" {
            // Abilities with onUpdate callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(
                ability_id,
                "commander" | "disguise" | "iceface" | "immunity" | "insomnia"
                | "limber" | "magmaarmor" | "oblivious" | "owntempo" | "pastelveil"
                | "thermalexchange" | "trace" | "vitalspirit" | "waterbubble" | "waterveil"
            );
        }

        // Check for onModifyCritRatio event (critical hit ratio modification)
        if event_id == "onModifyCritRatio" {
            // Abilities with onModifyCritRatio callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "merciless" | "superluck");
        }

        // Check for onCriticalHit event (critical hit processing)
        if event_id == "onCriticalHit" {
            // Abilities with onCriticalHit callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "disguise" | "iceface");
        }

        // Check for onTryHit event (move hit checking)
        if event_id == "onTryHit" {
            // Abilities with onTryHit callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(
                ability_id,
                "bulletproof" | "dryskin" | "eartheater" | "flashfire" | "goodasgold"
                | "lightningrod" | "magicbounce" | "motordrive" | "mountaineer" | "oblivious"
                | "overcoat" | "rebound" | "sapsipper" | "soundproof" | "stormdrain"
                | "sturdy" | "telepathy" | "voltabsorb" | "waterabsorb" | "wellbakedbody"
                | "windrider" | "wonderguard"
            );
        }

        // Check for onDamagingHit event (contact move effects)
        if event_id == "onDamagingHit" {
            // Abilities with onDamagingHit callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(
                ability_id,
                "aftermath" | "cottondown" | "cursedbody" | "cutecharm" | "effectspore"
                | "electromorphosis" | "flamebody" | "gooey" | "gulpmissile" | "illusion"
                | "innardsout" | "ironbarbs" | "justified" | "lingeringaroma" | "mummy"
                | "perishbody" | "poisonpoint" | "rattled" | "roughskin" | "sandspit"
                | "seedsower" | "stamina" | "static" | "steamengine" | "tanglinghair"
                | "thermalexchange" | "toxicdebris" | "wanderingspirit" | "watercompaction" | "weakarmor"
                | "windpower"
            );
        }

        // Check for onModifyMove event (move modification)
        if event_id == "onModifyMove" {
            // Abilities with onModifyMove callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(
                ability_id,
                "battlebond" | "gorillatactics" | "illuminate" | "infiltrator" | "keeneye"
                | "longreach" | "mindseye" | "moldbreaker" | "myceliummight" | "propellertail"
                | "scrappy" | "serenegrace" | "sheerforce" | "skilllink" | "stalwart"
                | "stancechange" | "stench" | "teravolt" | "turboblaze" | "unseenfist"
            );
        }

        // Check for onAfterBoost event (after stat changes)
        if event_id == "onAfterBoost" {
            // Abilities with onAfterBoost callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "rattled");
        }

        // Check for onAfterEachBoost event (after each individual stat change)
        if event_id == "onAfterEachBoost" {
            // Abilities with onAfterEachBoost callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "competitive" | "defiant");
        }

        // Check for onTryBoost event (stat boost prevention)
        if event_id == "onTryBoost" {
            // Abilities with onTryBoost callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(
                ability_id,
                "bigpecks" | "clearbody" | "fullmetalbody" | "guarddog" | "hypercutter"
                | "illuminate" | "innerfocus" | "keeneye" | "mindseye" | "mirrorarmor"
                | "oblivious" | "owntempo" | "scrappy" | "whitesmoke"
            );
        }

        // Check for onTryHeal event (healing prevention/modification)
        if event_id == "onTryHeal" {
            // Abilities with onTryHeal callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "ripen");
        }

        // Check for onDisableMove event (move disabling)
        if event_id == "onDisableMove" {
            // Abilities with onDisableMove callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "gorillatactics");
        }

        // Check for onModifyPriority event (priority modification)
        if event_id == "onModifyPriority" {
            // Abilities with onModifyPriority callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "galewings" | "prankster" | "triage");
        }

        // Check for onDragOut event (forced switch prevention)
        if event_id == "onDragOut" {
            // Abilities with onDragOut callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "guarddog" | "suctioncups");
        }

        // Check for onHit event (when move hits successfully)
        if event_id == "onHit" {
            // Abilities with onHit callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "angerpoint" | "owntempo");
        }

        // Check for onFaint event (when Pokemon faints)
        if event_id == "onFaint" {
            // Abilities with onFaint callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "illusion");
        }

        // Check for onAfterUseItem event (after using/consuming an item)
        if event_id == "onAfterUseItem" {
            // Abilities with onAfterUseItem callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "unburden");
        }

        // Check for onEatItem event (when Pokemon eats a berry/item)
        if event_id == "onEatItem" {
            // Abilities with onEatItem callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "cheekpouch" | "cudchew" | "ripen");
        }

        // Check for onAnyFaint event (when ANY Pokemon faints)
        if event_id == "onAnyFaint" {
            // Abilities with onAnyFaint callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "soulheart");
        }

        // Check for onAllyFaint event (when an ally faints)
        if event_id == "onAllyFaint" {
            // Abilities with onAllyFaint callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "powerofalchemy" | "receiver");
        }

        // Check for onSourceAfterFaint event (when source defeats a Pokemon)
        if event_id == "onSourceAfterFaint" {
            // Abilities with onSourceAfterFaint callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(
                ability_id,
                "asoneglastrier" | "asonespectrier" | "battlebond" | "beastboost"
                | "chillingneigh" | "grimneigh" | "moxie"
            );
        }

        // Check for onSourceDamagingHit event (when source damages target with attack)
        if event_id == "onSourceDamagingHit" {
            // Abilities with onSourceDamagingHit callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "poisontouch" | "toxicchain");
        }

        // Check for onSourceModifyAtk event (when source's Attack is being modified)
        if event_id == "onSourceModifyAtk" {
            // Abilities with onSourceModifyAtk callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(
                ability_id,
                "heatproof" | "purifyingsalt" | "thickfat" | "waterbubble"
            );
        }

        // Check for onSourceModifySpA event (when source's Sp. Atk is being modified)
        if event_id == "onSourceModifySpA" {
            // Abilities with onSourceModifySpA callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(
                ability_id,
                "heatproof" | "purifyingsalt" | "thickfat" | "waterbubble"
            );
        }

        // Check for onSourceModifyDamage event (when source's damage is being modified)
        if event_id == "onSourceModifyDamage" {
            // Abilities with onSourceModifyDamage callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(
                ability_id,
                "filter" | "fluffy" | "icescales" | "multiscale" | "prismarmor"
                | "punkrock" | "ripen" | "shadowshield" | "solidrock"
            );
        }

        // Check for onSourceTryHeal event (when source is trying to heal)
        if event_id == "onSourceTryHeal" {
            // Abilities with onSourceTryHeal callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "liquidooze");
        }

        // Check for onSourceModifyAccuracy event (when source's accuracy is being modified)
        if event_id == "onSourceModifyAccuracy" {
            // Abilities with onSourceModifyAccuracy callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "compoundeyes" | "hustle");
        }

        // Check for onAnyBasePower event (when ANY Pokemon uses a move)
        if event_id == "onAnyBasePower" {
            // Abilities with onAnyBasePower callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "darkaura" | "fairyaura");
        }

        // Check for onAnyDamage event (when ANY Pokemon takes damage)
        if event_id == "onAnyDamage" {
            // Abilities with onAnyDamage callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "damp");
        }

        // Check for onAnyInvulnerability event (when ANY Pokemon checks invulnerability)
        if event_id == "onAnyInvulnerability" {
            // Abilities with onAnyInvulnerability callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "noguard");
        }

        // Check for onAnyRedirectTarget event (when ANY move target is being determined)
        if event_id == "onAnyRedirectTarget" {
            // Abilities with onAnyRedirectTarget callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "lightningrod" | "stormdrain");
        }

        // Check for onAnySetWeather event (when ANY weather is being set)
        if event_id == "onAnySetWeather" {
            // Abilities with onAnySetWeather callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "deltastream" | "desolateland" | "primordialsea");
        }

        // Check for onImmunity event (immunity to effects/weather)
        if event_id == "onImmunity" {
            // Abilities with onImmunity callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(
                ability_id,
                "icebody" | "magmaarmor" | "oblivious" | "overcoat" | "sandforce"
                | "sandrush" | "sandveil" | "snowcloak"
            );
        }

        // Check for onSetStatus event (when status condition is set)
        if event_id == "onSetStatus" {
            // Abilities with onSetStatus callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(
                ability_id,
                "comatose" | "immunity" | "insomnia" | "leafguard" | "limber"
                | "pastelveil" | "purifyingsalt" | "shieldsdown" | "thermalexchange"
                | "vitalspirit" | "waterbubble" | "waterveil"
            );
        }

        // Check for onTryAddVolatile event (when volatile condition is added)
        if event_id == "onTryAddVolatile" {
            // Abilities with onTryAddVolatile callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(
                ability_id,
                "innerfocus" | "insomnia" | "leafguard" | "owntempo"
                | "purifyingsalt" | "shieldsdown" | "vitalspirit"
            );
        }

        // Check for onFoeTryMove event (when foe attempts to use a move)
        if event_id == "onFoeTryMove" {
            // Abilities with onFoeTryMove callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "armortail" | "dazzling" | "queenlymajesty");
        }

        // Check for onFoeTrapPokemon event (when foe tries to switch out)
        if event_id == "onFoeTrapPokemon" {
            // Abilities with onFoeTrapPokemon callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "arenatrap" | "magnetpull" | "shadowtag");
        }

        // Check for onFoeMaybeTrapPokemon event (when foe might be trapped)
        if event_id == "onFoeMaybeTrapPokemon" {
            // Abilities with onFoeMaybeTrapPokemon callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "arenatrap" | "magnetpull" | "shadowtag");
        }

        // Check for onFoeTryEatItem event (when foe tries to eat berry/item)
        if event_id == "onFoeTryEatItem" {
            // Abilities with onFoeTryEatItem callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "asoneglastrier" | "asonespectrier" | "unnerve");
        }

        // Check for onFoeAfterBoost event (when foe's stats are boosted)
        if event_id == "onFoeAfterBoost" {
            // Abilities with onFoeAfterBoost callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "opportunist");
        }

        // Check for onAllyTryBoost event (when ally's stats are being boosted)
        if event_id == "onAllyTryBoost" {
            // Abilities with onAllyTryBoost callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "flowerveil");
        }

        // Check for onAllySetStatus event (when ally's status is being set)
        if event_id == "onAllySetStatus" {
            // Abilities with onAllySetStatus callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "flowerveil" | "pastelveil" | "sweetveil");
        }

        // Check for onAllyTryAddVolatile event (when ally's volatile is being added)
        if event_id == "onAllyTryAddVolatile" {
            // Abilities with onAllyTryAddVolatile callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "aromaveil" | "flowerveil" | "sweetveil");
        }

        // Check for onAllyModifyAtk event (when ally's Attack is calculated)
        if event_id == "onAllyModifyAtk" {
            // Abilities with onAllyModifyAtk callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "flowergift");
        }

        // Check for onAllyModifySpD event (when ally's Sp. Def is calculated)
        if event_id == "onAllyModifySpD" {
            // Abilities with onAllyModifySpD callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "flowergift");
        }

        // Check for onAllyTryHitSide event (when move targets ally's side)
        if event_id == "onAllyTryHitSide" {
            // Abilities with onAllyTryHitSide callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "magicbounce" | "rebound" | "sapsipper" | "soundproof");
        }

        // Check for onAllyAfterUseItem event (when ally uses/consumes an item)
        if event_id == "onAllyAfterUseItem" {
            // Abilities with onAllyAfterUseItem callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "symbiosis");
        }

        // Check for onModifyAtk event (when Attack stat is calculated)
        if event_id == "onModifyAtk" {
            // Abilities with onModifyAtk callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(
                ability_id,
                "blaze" | "defeatist" | "dragonsmaw" | "gorillatactics" | "guts"
                | "hugepower" | "hustle" | "orichalcumpulse" | "overgrow" | "purepower"
                | "rockypayload" | "slowstart" | "stakeout" | "steelworker" | "swarm"
                | "torrent" | "transistor" | "waterbubble"
            );
        }

        // Check for onModifyDef event (when Defense stat is calculated)
        if event_id == "onModifyDef" {
            // Abilities with onModifyDef callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "furcoat" | "grasspelt" | "marvelscale");
        }

        // Check for onModifySpA event (when Sp. Atk stat is calculated)
        if event_id == "onModifySpA" {
            // Abilities with onModifySpA callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(
                ability_id,
                "blaze" | "defeatist" | "dragonsmaw" | "hadronengine" | "minus"
                | "overgrow" | "plus" | "rockypayload" | "solarpower" | "stakeout"
                | "steelworker" | "swarm" | "torrent" | "transistor" | "waterbubble"
            );
        }

        // Check for onModifySpe event (when Speed stat is calculated)
        if event_id == "onModifySpe" {
            // Abilities with onModifySpe callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(
                ability_id,
                "chlorophyll" | "quickfeet" | "sandrush" | "slowstart" | "slushrush"
                | "surgesurfer" | "swiftswim"
            );
        }

        // Check for onModifyDamage event (when damage is being calculated)
        if event_id == "onModifyDamage" {
            // Abilities with onModifyDamage callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "neuroforce" | "sniper" | "tintedlens");
        }

        // Check for onModifyAccuracy event (when accuracy is being calculated)
        if event_id == "onModifyAccuracy" {
            // Abilities with onModifyAccuracy callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "sandveil" | "snowcloak" | "tangledfeet" | "wonderskin");
        }

        // Check for onEffectiveness event (when type effectiveness is calculated)
        if event_id == "onEffectiveness" {
            // Abilities with onEffectiveness callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "disguise" | "iceface");
        }

        // Check for onBeforeMove event (before a move is used)
        if event_id == "onBeforeMove" {
            // Abilities with onBeforeMove callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "gorillatactics" | "truant");
        }

        // Check for onPrepareHit event (when preparing to execute a move)
        if event_id == "onPrepareHit" {
            // Abilities with onPrepareHit callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "libero" | "parentalbond" | "protean");
        }

        // Check for onModifyType event (when move type is being modified)
        if event_id == "onModifyType" {
            // Abilities with onModifyType callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(
                ability_id,
                "aerilate" | "galvanize" | "liquidvoice" | "normalize" | "pixilate"
                | "refrigerate"
            );
        }

        // Check for onModifyWeight event (when weight is being calculated)
        if event_id == "onModifyWeight" {
            // Abilities with onModifyWeight callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "heavymetal" | "lightmetal");
        }

        // Check for onModifySTAB event (when STAB multiplier is calculated)
        if event_id == "onModifySTAB" {
            // Abilities with onModifySTAB callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "adaptability");
        }

        // Check for onAfterMoveSecondary event (after secondary effects are processed)
        if event_id == "onAfterMoveSecondary" {
            // Abilities with onAfterMoveSecondary callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(
                ability_id,
                "angershell" | "berserk" | "colorchange" | "pickpocket"
            );
        }

        // Check for onAfterMoveSecondarySelf event (after secondary effects on self)
        if event_id == "onAfterMoveSecondarySelf" {
            // Abilities with onAfterMoveSecondarySelf callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "magician");
        }

        // Check for onModifySecondaries event (when secondary effect chances are modified)
        if event_id == "onModifySecondaries" {
            // Abilities with onModifySecondaries callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "shielddust");
        }

        // Check for onSourceModifySecondaries event (when source's secondary effect chances are modified)
        if event_id == "onSourceModifySecondaries" {
            // Abilities with onSourceModifySecondaries callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "parentalbond");
        }

        // Check for onFlinch event (when Pokemon flinches)
        if event_id == "onFlinch" {
            // Abilities with onFlinch callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "steadfast");
        }

        // Check for onTryEatItem event (when trying to eat a berry/item)
        if event_id == "onTryEatItem" {
            // Abilities with onTryEatItem callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "angershell" | "berserk" | "ripen");
        }

        // Check for onAnyModifyAtk event (when ANY Pokemon's Attack is modified)
        if event_id == "onAnyModifyAtk" {
            // Abilities with onAnyModifyAtk callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "tabletsofruin");
        }

        // Check for onAnyModifyDef event (when ANY Pokemon's Defense is modified)
        if event_id == "onAnyModifyDef" {
            // Abilities with onAnyModifyDef callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "swordofruin");
        }

        // Check for onAnyModifySpA event (when ANY Pokemon's Sp. Atk is modified)
        if event_id == "onAnyModifySpA" {
            // Abilities with onAnyModifySpA callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "vesselofruin");
        }

        // Check for onAnyModifySpD event (when ANY Pokemon's Sp. Def is modified)
        if event_id == "onAnyModifySpD" {
            // Abilities with onAnyModifySpD callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "beadsofruin");
        }

        // Check for onAnyModifyDamage event (when ANY Pokemon's damage is modified)
        if event_id == "onAnyModifyDamage" {
            // Abilities with onAnyModifyDamage callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "friendguard");
        }

        // Check for onAnyAccuracy event (when ANY accuracy check is made)
        if event_id == "onAnyAccuracy" {
            // Abilities with onAnyAccuracy callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "noguard");
        }

        // Check for onAnyModifyAccuracy event (when ANY Pokemon's accuracy is modified)
        if event_id == "onAnyModifyAccuracy" {
            // Abilities with onAnyModifyAccuracy callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "victorystar");
        }

        // Check for onAnyTryMove event (when ANY Pokemon tries to use a move)
        if event_id == "onAnyTryMove" {
            // Abilities with onAnyTryMove callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "damp");
        }

        // Check for onAfterSetStatus event (after status condition is set)
        if event_id == "onAfterSetStatus" {
            // Abilities with onAfterSetStatus callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "synchronize");
        }

        // Check for onFractionalPriority event (for fractional priority adjustments)
        if event_id == "onFractionalPriority" {
            // Abilities with onFractionalPriority callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "myceliummight" | "quickdraw");
        }

        // Check for onChangeBoost event (when stat boost values are being changed)
        if event_id == "onChangeBoost" {
            // Abilities with onChangeBoost callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "contrary" | "ripen" | "simple");
        }

        // Check for onSourceTryPrimaryHit event (when source tries to hit with primary damage)
        if event_id == "onSourceTryPrimaryHit" {
            // Abilities with onSourceTryPrimaryHit callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "gulpmissile");
        }

        // Check for onDeductPP event (when PP is being deducted)
        if event_id == "onDeductPP" {
            // Abilities with onDeductPP callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "pressure");
        }

        // Check for onBeforeSwitchIn event (before Pokemon switches in)
        if event_id == "onBeforeSwitchIn" {
            // Abilities with onBeforeSwitchIn callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "illusion");
        }

        // Check for onAnyBeforeMove event (before ANY Pokemon uses a move)
        if event_id == "onAnyBeforeMove" {
            // Abilities with onAnyBeforeMove callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "terashell");
        }

        // Check for onAnyModifyBoost event (when ANY Pokemon's stat boosts are modified)
        if event_id == "onAnyModifyBoost" {
            // Abilities with onAnyModifyBoost callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "unaware");
        }

        // Check for onTakeItem event (when item is being taken)
        if event_id == "onTakeItem" {
            // Abilities with onTakeItem callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "stickyhold" | "unburden");
        }

        // Check for onAfterTerastallization event (after Pokemon Terastallizes)
        if event_id == "onAfterTerastallization" {
            // Abilities with onAfterTerastallization callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "teraformzero");
        }

        // Check for onAnyAfterSetStatus event (after ANY Pokemon gets a status)
        if event_id == "onAnyAfterSetStatus" {
            // Abilities with onAnyAfterSetStatus callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "poisonpuppeteer");
        }

        // Check for onAnyTryPrimaryHit event (when ANY move tries primary hit)
        if event_id == "onAnyTryPrimaryHit" {
            // Abilities with onAnyTryPrimaryHit callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "aurabreak");
        }

        // Check for onWeather event (weather damage/effects at end of turn)
        if event_id == "onWeather" {
            // Abilities with onWeather callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "dryskin" | "icebody" | "raindish" | "solarpower");
        }

        // Check for onAnyAfterMega event (after ANY Pokemon Mega Evolves)
        if event_id == "onAnyAfterMega" {
            // Abilities with onAnyAfterMega callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "opportunist");
        }

        // Check for onAnyAfterMove event (after ANY Pokemon uses a move)
        if event_id == "onAnyAfterMove" {
            // Abilities with onAnyAfterMove callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "opportunist" | "terashell");
        }

        // Check for onAnyAfterTerastallization event (after ANY Pokemon Terastallizes)
        if event_id == "onAnyAfterTerastallization" {
            // Abilities with onAnyAfterTerastallization callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "opportunist");
        }

        // Check for onCheckShow event (when checking if ability should be shown)
        if event_id == "onCheckShow" {
            // Abilities with onCheckShow callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "naturalcure");
        }

        // Check for onEmergencyExit event (when Pokemon should emergency exit)
        if event_id == "onEmergencyExit" {
            // Abilities with onEmergencyExit callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "emergencyexit" | "wimpout");
        }

        // Check for onSideConditionStart event (when a side condition starts)
        if event_id == "onSideConditionStart" {
            // Abilities with onSideConditionStart callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "windpower" | "windrider");
        }

        // Check for onTerrainChange event (when terrain changes)
        if event_id == "onTerrainChange" {
            // Abilities with onTerrainChange callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(ability_id, "mimicry" | "quarkdrive");
        }

        // Check for onWeatherChange event (when weather changes)
        if event_id == "onWeatherChange" {
            // Abilities with onWeatherChange callbacks (from ability_callbacks/mod.rs dispatcher)
            return matches!(
                ability_id,
                "flowergift" | "forecast" | "iceface" | "protosynthesis"
            );
        }

        // For other events, conservatively return false by default
        // TODO: Implement proper callback checking for other events
        // For now, this prevents collecting non-existent handlers
        false
    }

    /// Check if an item has a callback for an event
    fn item_has_callback(&self, _item_id: &str, event_id: &str) -> bool {
        // Check for onResidual event
        if event_id == "onResidual" {
            // Items with onResidual callbacks (from item_callbacks/mod.rs dispatcher)
            return matches!(_item_id, "ejectpack" | "mirrorherb" | "whiteherb");
        }

        // Check for onSwitchIn event (when Pokemon switches in)
        if event_id == "onSwitchIn" {
            // Items with onSwitchIn callbacks (from item_callbacks/mod.rs dispatcher)
            return matches!(_item_id, "blueorb" | "redorb");
        }

        // Check for onUpdate event (turn-based updates)
        if event_id == "onUpdate" {
            // Items with onUpdate callbacks (from item_callbacks/mod.rs dispatcher)
            return matches!(_item_id, "boosterenergy" | "utilityumbrella");
        }

        // Check for onTryHit event (move hit checking)
        if event_id == "onTryHit" {
            // Items with onTryHit callbacks (from item_callbacks/mod.rs dispatcher)
            return matches!(_item_id, "safetygoggles");
        }

        // Check for onAfterBoost event (after stat changes)
        if event_id == "onAfterBoost" {
            // Items with onAfterBoost callbacks (from item_callbacks/mod.rs dispatcher)
            return matches!(_item_id, "adrenalineorb" | "ejectpack");
        }

        // Check for onTryBoost event (stat boost prevention)
        if event_id == "onTryBoost" {
            // Items with onTryBoost callbacks (from item_callbacks/mod.rs dispatcher)
            return matches!(_item_id, "clearamulet");
        }

        // Check for onModifyCritRatio event (when critical hit ratio is modified)
        if event_id == "onModifyCritRatio" {
            // Items with onModifyCritRatio callbacks (from item_callbacks/mod.rs dispatcher)
            return matches!(_item_id, "leek" | "luckypunch" | "razorclaw" | "scopelens" | "stick");
        }

        // Check for onEnd event (when effect ends)
        if event_id == "onEnd" {
            // Items with onEnd callbacks (from item_callbacks/mod.rs dispatcher)
            return matches!(_item_id, "ejectpack" | "mirrorherb" | "utilityumbrella");
        }

        // Check for onFractionalPriority event (fractional priority adjustments)
        if event_id == "onFractionalPriority" {
            // Items with onFractionalPriority callbacks (from item_callbacks/mod.rs dispatcher)
            return matches!(_item_id, "custapberry" | "quickclaw");
        }

        // Check for onModifySpe event (when Speed stat is modified)
        if event_id == "onModifySpe" {
            // Items with onModifySpe callbacks (from item_callbacks/mod.rs dispatcher)
            return matches!(_item_id, "ironball");
        }

        // Check for onModifyWeight event (when weight is modified)
        if event_id == "onModifyWeight" {
            // Items with onModifyWeight callbacks (from item_callbacks/mod.rs dispatcher)
            return matches!(_item_id, "floatstone");
        }

        // Check for onStart event (when effect starts)
        if event_id == "onStart" {
            // Items with onStart callbacks (from item_callbacks/mod.rs dispatcher)
            return matches!(_item_id, "boosterenergy" | "roomservice" | "utilityumbrella");
        }

        // Check for onTakeItem event (when item is taken)
        if event_id == "onTakeItem" {
            // Items with onTakeItem callbacks (from item_callbacks/mod.rs dispatcher)
            return matches!(_item_id, "blueorb" | "boosterenergy" | "redorb");
        }

        // Check for onAnyAfterMega event (after ANY Pokemon Mega Evolves)
        if event_id == "onAnyAfterMega" {
            // Items with onAnyAfterMega callbacks (from item_callbacks/mod.rs dispatcher)
            return matches!(_item_id, "ejectpack" | "mirrorherb" | "whiteherb");
        }

        // Check for onAnyAfterMove event (after ANY Pokemon uses a move)
        if event_id == "onAnyAfterMove" {
            // Items with onAnyAfterMove callbacks (from item_callbacks/mod.rs dispatcher)
            return matches!(_item_id, "ejectpack" | "mirrorherb" | "whiteherb");
        }

        // Check for onAnyAfterTerastallization event (after ANY Pokemon Terastallizes)
        if event_id == "onAnyAfterTerastallization" {
            // Items with onAnyAfterTerastallization callbacks (from item_callbacks/mod.rs dispatcher)
            return matches!(_item_id, "mirrorherb");
        }

        // Check for onAnyPseudoWeatherChange event (when pseudo-weather changes)
        if event_id == "onAnyPseudoWeatherChange" {
            // Items with onAnyPseudoWeatherChange callbacks (from item_callbacks/mod.rs dispatcher)
            return matches!(_item_id, "roomservice");
        }

        // Check for onAnySwitchIn event (when ANY Pokemon switches in)
        if event_id == "onAnySwitchIn" {
            // Items with onAnySwitchIn callbacks (from item_callbacks/mod.rs dispatcher)
            return matches!(_item_id, "ejectpack");
        }

        // Check for onEffectiveness event (type effectiveness modification)
        if event_id == "onEffectiveness" {
            // Items with onEffectiveness callbacks (from item_callbacks/mod.rs dispatcher)
            return matches!(_item_id, "ironball");
        }

        // Check for onFoeAfterBoost event (when foe's stats are boosted)
        if event_id == "onFoeAfterBoost" {
            // Items with onFoeAfterBoost callbacks (from item_callbacks/mod.rs dispatcher)
            return matches!(_item_id, "mirrorherb");
        }

        // Check for onImmunity event (immunity to effects)
        if event_id == "onImmunity" {
            // Items with onImmunity callbacks (from item_callbacks/mod.rs dispatcher)
            return matches!(_item_id, "safetygoggles");
        }

        // Check for onModifySecondaries event (modify secondary effect chances)
        if event_id == "onModifySecondaries" {
            // Items with onModifySecondaries callbacks (from item_callbacks/mod.rs dispatcher)
            return matches!(_item_id, "covertcloak");
        }

        // Check for onUse event (when item is used/activated)
        if event_id == "onUse" {
            // Items with onUse callbacks (from item_callbacks/mod.rs dispatcher)
            return matches!(_item_id, "ejectpack" | "mirrorherb" | "whiteherb");
        }

        // Check for onUseItem event (when item is consumed)
        if event_id == "onUseItem" {
            // Items with onUseItem callbacks (from item_callbacks/mod.rs dispatcher)
            return matches!(_item_id, "ejectpack");
        }

        // For other events, conservatively return false by default
        // TODO: Implement proper callback checking by consulting item data
        false
    }

    /// Check if a move has a callback for an event
    fn move_has_callback(&self, move_id: &str, event_id: &str) -> bool {
        // Moves don't have onAnySwitchIn
        if event_id == "onAnySwitchIn" {
            return false;
        }

        // For BasePower event, check if move is in the dispatcher
        if event_id == "BasePower" {
            return matches!(
                move_id,
                "barbbarrage"
                    | "brine"
                    | "collisioncourse"
                    | "electrodrift"
                    | "expandingforce"
                    | "facade"
                    | "ficklebeam"
                    | "fusionbolt"
                    | "fusionflare"
                    | "gravapple"
                    | "knockoff"
                    | "lashout"
                    | "mistyexplosion"
                    | "psyblade"
                    | "retaliate"
                    | "solarbeam"
                    | "solarblade"
                    | "venoshock"
            );
        }

        // Check for onDamage event
        if event_id == "onDamage" {
            return matches!(move_id, "falseswipe" | "holdback");
        }

        // Check for onEffectiveness event
        if event_id == "onEffectiveness" {
            return matches!(move_id, "flyingpress" | "freezedry" | "thousandarrows");
        }

        // Check for onModifyType event
        if event_id == "onModifyType" {
            return matches!(
                move_id,
                "aurawheel" | "hiddenpower" | "ivycudgel" | "judgment" | "multiattack"
                | "naturalgift" | "ragingbull" | "revelationdance" | "technoblast"
                | "terablast" | "terastarstorm" | "terrainpulse" | "weatherball"
            );
        }

        // Check for onTryHit event
        if event_id == "onTryHit" {
            return matches!(
                move_id,
                "autotomize" | "brickbreak" | "celebrate" | "clangoroussoul" | "curse"
                | "disable" | "electrify" | "entrainment" | "filletaway" | "foresight"
                | "gastroacid" | "grassknot" | "happyhour" | "healingwish" | "heatcrash"
                | "heavyslam" | "helpinghand" | "lockon" | "lowkick" | "lunardance"
                | "mefirst" | "mindreader" | "miracleeye" | "mirrormove" | "naturepower"
                | "odorsleuth" | "pollenpuff" | "poltergeist" | "psychicfangs" | "psychoshift"
                | "pursuit" | "ragingbull" | "revivalblessing" | "roleplay" | "shedtail"
                | "simplebeam" | "skillswap" | "skydrop" | "splash" | "spotlight"
                | "substitute" | "uproar" | "worryseed" | "yawn"
            );
        }

        // Check for onAfterHit event (after move hits successfully)
        if event_id == "onAfterHit" {
            // Moves with onAfterHit callbacks (from move_callbacks/mod.rs dispatcher)
            return matches!(
                move_id,
                "beakblast" | "ceaselessedge" | "covet" | "iceball" | "icespinner"
                | "knockoff" | "mindblown" | "mortalspin" | "rapidspin" | "rollout"
                | "stoneaxe" | "thief"
            );
        }

        // Check for onDisableMove event (move disabling)
        if event_id == "onDisableMove" {
            // Moves with onDisableMove callbacks (from move_callbacks/mod.rs dispatcher)
            return matches!(move_id, "belch" | "stuffcheeks");
        }

        // Check for onAfterMove event (after move execution)
        if event_id == "onAfterMove" {
            // Moves with onAfterMove callbacks (from move_callbacks/mod.rs dispatcher)
            return matches!(
                move_id,
                "beakblast" | "iceball" | "mindblown" | "rollout" | "sparklingaria"
                | "spitup" | "steelbeam"
            );
        }

        // Check for onAfterMoveSecondarySelf event (after secondary effects on self)
        if event_id == "onAfterMoveSecondarySelf" {
            // Moves with onAfterMoveSecondarySelf callbacks (from move_callbacks/mod.rs dispatcher)
            return matches!(move_id, "fellstinger" | "orderup" | "polarflare" | "relicsong");
        }

        // Check for onAfterSubDamage event (after substitute damage)
        if event_id == "onAfterSubDamage" {
            // Moves with onAfterSubDamage callbacks (from move_callbacks/mod.rs dispatcher)
            return matches!(
                move_id,
                "ceaselessedge" | "coreenforcer" | "flameburst" | "gmaxsnooze" | "icespinner"
                | "mortalspin" | "rapidspin" | "shellsidearm" | "splinteredstormshards"
                | "steelroller" | "stoneaxe"
            );
        }

        // Check for onHit event (when move hits)
        if event_id == "onHit" {
            // Moves with onHit callbacks (from move_callbacks/mod.rs dispatcher)
            return matches!(
                move_id,
                "acupressure" | "afteryou" | "alluringvoice" | "allyswitch" | "anchorshot"
                | "aromatherapy" | "assist" | "autotomize" | "banefulbunker" | "batonpass"
                | "bellydrum" | "bestow" | "block" | "bugbite" | "burningbulwark"
                | "burningjealousy" | "camouflage" | "clangoroussoul" | "clearsmog" | "conversion"
                | "conversion2" | "copycat" | "coreenforcer" | "corrosivegas" | "curse"
                | "defog" | "detect" | "direclaw" | "doodle" | "eeriespell"
                | "endure" | "entrainment" | "filletaway" | "flameburst" | "floralhealing"
                | "forestscurse" | "freezyfrost" | "genesissupernova" | "gmaxbefuddle" | "gmaxcentiferno"
                | "gmaxcuddle" | "gmaxdepletion" | "gmaxfinale" | "gmaxfoamburst" | "gmaxgoldrush"
                | "gmaxmalodor" | "gmaxmeltdown" | "gmaxreplenish" | "gmaxsandblast" | "gmaxsmite"
                | "gmaxsnooze" | "gmaxstonesurge" | "gmaxstunshock" | "gmaxsweetness" | "gmaxtartness"
                | "gmaxterror" | "gmaxvoltcrash" | "gmaxwindrage" | "guardsplit" | "guardswap"
                | "healbell" | "healpulse" | "heartswap" | "incinerate" | "instruct"
                | "jawlock" | "junglehealing" | "kingsshield" | "lockon" | "lunarblessing"
                | "magicpowder" | "maxairstream" | "maxdarkness" | "maxflare" | "maxflutterby"
                | "maxgeyser" | "maxguard" | "maxhailstorm" | "maxknuckle" | "maxlightning"
                | "maxmindstorm" | "maxooze" | "maxovergrowth" | "maxphantasm" | "maxquake"
                | "maxrockfall" | "maxstarfall" | "maxsteelspike" | "maxstrike" | "maxwyrmwind"
                | "meanlook" | "metronome" | "mimic" | "mindreader" | "moonlight"
                | "morningsun" | "obstruct" | "painsplit" | "partingshot" | "pluck"
                | "polarflare" | "pollenpuff" | "powersplit" | "powerswap" | "protect"
                | "psychup" | "purify" | "quash" | "recycle" | "reflecttype"
                | "refresh" | "relicsong" | "rest" | "roleplay" | "sappyseed"
                | "shedtail" | "shellsidearm" | "shoreup" | "silktrap" | "simplebeam"
                | "sketch" | "skillswap" | "skydrop" | "sleeptalk" | "smellingsalts"
                | "soak" | "speedswap" | "spiderweb" | "spikyshield" | "spiritshackle"
                | "spite" | "splinteredstormshards" | "steelroller" | "strengthsap" | "stuffcheeks"
                | "substitute" | "swallow" | "switcheroo" | "synthesis" | "takeheart"
                | "thousandwaves" | "tidyup" | "topsyturvy" | "transform" | "trick"
                | "trickortreat" | "venomdrench" | "wakeupslap" | "worryseed"
            );
        }

        // Check for onHitField event (when move hits the field)
        if event_id == "onHitField" {
            // Moves with onHitField callbacks (from move_callbacks/mod.rs dispatcher)
            return matches!(
                move_id,
                "courtchange" | "flowershield" | "haze" | "perishsong" | "rototiller"
                | "teatime"
            );
        }

        // Check for onHitSide event (when move hits a side)
        if event_id == "onHitSide" {
            // Moves with onHitSide callbacks (from move_callbacks/mod.rs dispatcher)
            return matches!(move_id, "gearup" | "magneticflux" | "quickguard" | "wideguard");
        }

        // Check for onModifyMove event (move modification)
        if event_id == "onModifyMove" {
            // Moves with onModifyMove callbacks (from move_callbacks/mod.rs dispatcher)
            return matches!(
                move_id,
                "beatup" | "bleakwindstorm" | "blizzard" | "curse" | "expandingforce"
                | "firepledge" | "grasspledge" | "growth" | "hurricane" | "iceball"
                | "lightthatburnsthesky" | "magnitude" | "photongeyser" | "present" | "pursuit"
                | "rollout" | "sandsearstorm" | "secretpower" | "shellsidearm" | "skydrop"
                | "struggle" | "terablast" | "terastarstorm" | "terrainpulse" | "thunder"
                | "waterpledge" | "weatherball" | "wildboltstorm"
            );
        }

        // Check for onModifyPriority event (priority modification)
        if event_id == "onModifyPriority" {
            // Moves with onModifyPriority callbacks (from move_callbacks/mod.rs dispatcher)
            return matches!(move_id, "grassyglide");
        }

        // Check for onModifyTarget event (target modification)
        if event_id == "onModifyTarget" {
            // Moves with onModifyTarget callbacks (from move_callbacks/mod.rs dispatcher)
            return matches!(move_id, "comeuppance" | "metalburst");
        }

        // Check for onMoveFail event (when move fails)
        if event_id == "onMoveFail" {
            // Moves with onMoveFail callbacks (from move_callbacks/mod.rs dispatcher)
            return matches!(
                move_id,
                "axekick" | "highjumpkick" | "jumpkick" | "skydrop" | "supercellslam"
            );
        }

        // Check for onPrepareHit event (preparing to hit)
        if event_id == "onPrepareHit" {
            // Moves with onPrepareHit callbacks (from move_callbacks/mod.rs dispatcher)
            return matches!(
                move_id,
                "allyswitch" | "banefulbunker" | "burningbulwark" | "destinybond" | "detect"
                | "endure" | "firepledge" | "fling" | "grasspledge" | "ivycudgel"
                | "kingsshield" | "maxguard" | "naturalgift" | "obstruct" | "protect"
                | "shellsidearm" | "silktrap" | "spikyshield" | "terablast" | "waterpledge"
            );
        }

        // Check for onTry event (move attempt)
        if event_id == "onTry" {
            // Moves with onTry callbacks (from move_callbacks/mod.rs dispatcher)
            return matches!(
                move_id,
                "aurawheel" | "auroraveil" | "clangoroussoul" | "comeuppance" | "counter"
                | "craftyshield" | "darkvoid" | "doomdesire" | "fakeout" | "filletaway"
                | "firstimpression" | "followme" | "futuresight" | "hyperspacefury" | "lastresort"
                | "magnetrise" | "matblock" | "metalburst" | "mirrorcoat" | "noretreat"
                | "poltergeist" | "quickguard" | "ragepowder" | "rest" | "round"
                | "skydrop" | "sleeptalk" | "snore" | "spitup" | "splash"
                | "steelroller" | "stockpile" | "stuffcheeks" | "suckerpunch" | "swallow"
                | "telekinesis" | "teleport" | "thunderclap" | "upperhand" | "wideguard"
            );
        }

        // Check for onTryImmunity event (immunity checking)
        if event_id == "onTryImmunity" {
            // Moves with onTryImmunity callbacks (from move_callbacks/mod.rs dispatcher)
            return matches!(
                move_id,
                "attract" | "captivate" | "dreameater" | "endeavor" | "leechseed"
                | "octolock" | "switcheroo" | "synchronoise" | "trick" | "worryseed"
            );
        }

        // Check for onTryMove event (move execution attempt)
        if event_id == "onTryMove" {
            // Moves with onTryMove callbacks (from move_callbacks/mod.rs dispatcher)
            return matches!(
                move_id,
                "bounce" | "burnup" | "dig" | "dive" | "doubleshock"
                | "echoedvoice" | "electroshot" | "fly" | "freezeshock" | "geomancy"
                | "iceburn" | "meteorbeam" | "phantomforce" | "pollenpuff" | "razorwind"
                | "shadowforce" | "shelltrap" | "skullbash" | "skyattack" | "solarbeam"
                | "solarblade"
            );
        }

        // Check for onUseMoveMessage event (move usage message)
        if event_id == "onUseMoveMessage" {
            // Moves with onUseMoveMessage callbacks (from move_callbacks/mod.rs dispatcher)
            return matches!(move_id, "magnitude");
        }

        // For other events, conservatively return false by default
        // TODO: Implement proper callback checking for other events
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
