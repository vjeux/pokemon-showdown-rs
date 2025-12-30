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

        // For other events, conservatively return false by default
        // TODO: Implement proper callback checking for other events
        // For now, this prevents collecting non-existent handlers
        false
    }

    /// Check if an item has a callback for an event
    fn item_has_callback(&self, _item_id: &str, event_id: &str) -> bool {
        // Items don't have onAnySwitchIn
        if event_id == "onAnySwitchIn" {
            return false;
        }

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
