# Moves Implementation Tracking

## Summary

**Current Status:** 87 TODO callbacks remaining (out of ~700+ original callbacks)

**Latest Session Progress:**
- **Callbacks implemented**: 11 new implementations (2 healblock + 1 taunt + 1 substitute + 2 syrupbomb ✅ + 2 wish + 1 fling + 1 teatime ✅ + 1 toxicspikes ✅)
  - healblock.rs: condition::on_start, condition::on_restart
  - taunt.rs: condition::on_start
  - substitute.rs: condition::on_start
  - syrupbomb.rs: condition::on_update, condition::on_residual ✅ COMPLETE
  - wish.rs: on_start, on_end
  - fling.rs: condition::on_update
  - teatime.rs: on_hit_field ✅ COMPLETE
  - toxicspikes.rs: condition::on_switch_in ✅ COMPLETE

**Previous Session Progress:**
- **Callbacks implemented**: 6 implementations (3 healblock + 3 trickroom ✅)
  - healblock.rs: condition::duration_callback, condition::on_before_move, condition::on_modify_move
  - trickroom.rs: condition::duration_callback, condition::on_field_start, condition::on_field_restart ✅ COMPLETE

**Total Session Progress:**
- **Callbacks implemented**: 17 total (15 current + 2 previous session)
  - Current continuation:
    - healblock.rs: condition::duration_callback, condition::on_before_move, condition::on_modify_move
  - Earlier this session:
    - substitute.rs: condition::on_end, on_try_hit, on_hit
    - trick.rs: on_try_immunity
    - switcheroo.rs: on_try_immunity
    - throatchop.rs: condition::on_before_move, condition::on_modify_move
    - uproar.rs: on_try_hit, condition::on_residual
  - Previous session: swallow.rs: onTry, onHit
- **Files marked complete**: 47 total (1 wideguard + 46 previous)
  - Now complete: wideguard  - Previous session: throatchop, takeheart, temperflare, upperhand, veeveevolley, teleport, thunder, thunderclap, topsyturvy, venomdrench, venoshock, wakeupslap, watershuriken, waterspout, wildboltstorm, wringout, tailwind, transform, trickortreat, syrupbomb, teatime, toxicspikes
  - Previous session: synthesis, synchronoise, + 25 others
- **Partial implementations documented**: 18 moves now show partial completion status with ✓ markers (added healblock, trick, switcheroo, substitute, uproar)

**Previously Completed:** 15 callbacks (4 from latest active implementation + 11 simple message callbacks)
- throatchop.rs: condition::on_disable_move - Disables moves with sound flag
- taunt.rs: condition::on_disable_move - Disables Status moves except Me First
- healblock.rs: condition::on_disable_move - Disables moves with heal flag
- wideguard.rs: on_hit_side - Adds stall volatile to source pokemon

**Previously Completed:** 11 callbacks (simple message-adding callbacks)
- wonderroom.rs: condition::on_field_end
- trickroom.rs: condition::on_field_end
- taunt.rs: condition::on_end
- tarshot.rs: condition::on_start
- throatchop.rs: condition::on_start, condition::on_end (2 callbacks)
- healblock.rs: condition::on_end
- syrupbomb.rs: condition::on_start, condition::on_end (2 callbacks)
- uproar.rs: condition::on_start, condition::on_end (2 callbacks)
- telekinesis.rs: condition::on_end
- wideguard.rs: condition::on_side_start

**Total Progress This Session:** 15 callbacks implemented

**Key Infrastructure Discovery:**
The Pokemon struct already has more methods than initially documented:
- `pokemon.disable_move(move_id, source)` - Disable specific moves
- `pokemon.move_slots` - Access to pokemon's move list
- `pokemon.add_volatile(ID)` - Add volatile conditions ✓ USED
- `pokemon.remove_volatile(&ID)` - Remove volatile conditions
- `pokemon.has_volatile(&ID)` - Check for volatile conditions
- `battle.dex.moves` - HashMap for move definitions
- `MoveData.flags` - HashMap for checking move flags (sound, heal, etc.)
- `MoveData.category` - Move category (Status, Physical, Special)

**Blocking Issues:** All 116 remaining callbacks require missing infrastructure:
- Volatile condition management with source tracking (add_volatile with source parameter)
- Move property access (flags ✓, isZ, isMax, target type)
- Pokemon methods (has_ability, get_types, cure_status ✓, etc.)
- Move modification (type, category, base_power changes)
- Effect state tracking (source, duration, custom fields)
- Queue/action system (will_act, will_move)
- Status setting (try_set_status)
- Healing system (heal with source tracking)
- Item system (get_item, take_item, set_item)
- PP management (deduct_pp ✓ EXISTS but returns bool, not amount deducted)
- Turn counting and timing
- moveThisTurnResult tracking
- statsRaisedThisTurn field

Note: ✓ marks indicate infrastructure that exists but may need additional wrapper methods or modifications.

See "Missing Infrastructure" section below for comprehensive details.

Total: 953 moves
Moves with callbacks: 373

## Moves with Callbacks (alphabetically)
- [x] acrobatics - Acrobatics (Physical, Flying) - 1 callback: basePowerCallback
- [x] acupressure - Acupressure (Status, Normal) - 1 callback: onHit
- [x] afteryou - After You (Status, Normal) - 1 callback: onHit
- [x] allyswitch - Ally Switch (Status, Psychic) - 4 callbacks: onPrepareHit, onHit, condition::onStart, condition::onRestart
- [x] aquaring - Aqua Ring (Status, Water) - 2 callbacks: condition::onStart, condition::onResidual
- [x] aromatherapy - Aromatherapy (Status, Grass) - 1 callback: onHit
- [x] assist - Assist (Status, Normal) - 1 callback: onHit
- [x] assurance - Assurance (Physical, Dark) - 1 callback: basePowerCallback
- [x] attract - Attract (Status, Normal) - 5 callbacks: onTryImmunity, condition::onStart, condition::onUpdate, condition::onBeforeMove, condition::onEnd
- [x] aurawheel - Aura Wheel (Physical, Electric) - 2 callbacks: onTry, onModifyType
- [x] auroraveil - Aurora Veil (Status, Ice) - 5 callbacks: onTry, condition::durationCallback, condition::onAnyModifyDamage, condition::onSideStart, condition::onSideEnd
- [x] autotomize - Autotomize (Status, Steel) - 2 callbacks: onTryHit, onHit
- [x] avalanche - Avalanche (Physical, Ice) - 1 callback: basePowerCallback
- [x] axekick - Axe Kick (Physical, Fighting) - 1 callback: onMoveFail
- [x] banefulbunker - Baneful Bunker (Status, Poison) - 5 callbacks: onPrepareHit, onHit, condition::onStart, condition::onTryHit, condition::onHit
- [x] barbbarrage - Barb Barrage (Physical, Poison) - 1 callback: onBasePower
- [x] batonpass - Baton Pass (Status, Normal) - 1 callback: onHit
- [x] beakblast - Beak Blast (Physical, Flying) - 4 callbacks: priorityChargeCallback, onAfterMove, condition::onStart, condition::onHit
- [x] beatup - Beat Up (Physical, Dark) - 2 callbacks: basePowerCallback, onModifyMove
- [x] belch - Belch (Special, Poison) - 1 callback: onDisableMove
- [x] bellydrum - Belly Drum (Status, Normal) - 1 callback: onHit
- [x] bestow - Bestow (Status, Normal) - 1 callback: onHit
- [x] bide - Bide (Physical, Normal) - 6 callbacks: beforeMoveCallback, condition::onStart, condition::onDamage, condition::onBeforeMove, condition::onMoveAborted, condition::onEnd
- [x] bleakwindstorm - Bleakwind Storm (Special, Flying) - 1 callback: onModifyMove
- [x] blizzard - Blizzard (Special, Ice) - 1 callback: onModifyMove
- [x] block - Block (Status, Normal) - 1 callback: onHit
- [x] boltbeak - Bolt Beak (Physical, Electric) - 1 callback: basePowerCallback
- [x] bounce - Bounce (Physical, Flying) - 3 callbacks: onTryMove, condition::onInvulnerability, condition::onSourceBasePower
- [x] brickbreak - Brick Break (Physical, Fighting) - 1 callback: onTryHit
- [x] brine - Brine (Special, Water) - 1 callback: onBasePower
- [x] bugbite - Bug Bite (Physical, Bug) - 1 callback: onHit
- [x] burningbulwark - Burning Bulwark (Status, Fire) - 5 callbacks: onPrepareHit, onHit, condition::onStart, condition::onTryHit, condition::onHit
- [x] burnup - Burn Up (Special, Fire) - 1 callback: onTryMove
- [x] camouflage - Camouflage (Status, Normal) - 1 callback: onHit
- [x] captivate - Captivate (Status, Normal) - 1 callback: onTryImmunity
- [x] ceaselessedge - Ceaseless Edge (Physical, Dark) - 2 callbacks: onAfterHit, onAfterSubDamage
- [x] celebrate - Celebrate (Status, Normal) - 1 callback: onTryHit
- [x] charge - Charge (Status, Electric) - 6 callbacks: condition::onStart, condition::onRestart, condition::onBasePower, condition::onMoveAborted, condition::onAfterMove, condition::onEnd
- [x] chillyreception - Chilly Reception (Status, Ice) - 2 callbacks: priorityChargeCallback, condition::onBeforeMove
- [x] clangoroussoul - Clangorous Soul (Status, Dragon) - 3 callbacks: onTry, onTryHit, onHit
- [x] clearsmog - Clear Smog (Special, Poison) - 1 callback: onHit
- [x] collisioncourse - Collision Course (Physical, Fighting) - 1 callback: onBasePower
- [x] comeuppance - Comeuppance (Physical, Dark) - 3 callbacks: damageCallback, onTry, onModifyTarget
- [x] conversion - Conversion (Status, Normal) - 1 callback: onHit
- [x] conversion2 - Conversion 2 (Status, Normal) - 1 callback: onHit
- [x] copycat - Copycat (Status, Normal) - 1 callback: onHit
- [x] coreenforcer - Core Enforcer (Special, Dragon) - 2 callbacks: onHit, onAfterSubDamage
- [x] corrosivegas - Corrosive Gas (Status, Poison) - 1 callback: onHit
- [x] counter - Counter (Physical, Fighting) - 6 callbacks: damageCallback, beforeTurnCallback, onTry, condition::onStart, condition::onRedirectTarget, condition::onDamagingHit
- [x] courtchange - Court Change (Status, Normal) - 1 callback: onHitField
- [x] covet - Covet (Physical, Normal) - 1 callback: onAfterHit
- [x] craftyshield - Crafty Shield (Status, Fairy) - 3 callbacks: onTry, condition::onSideStart, condition::onTryHit
- [x] crushgrip - Crush Grip (Physical, Normal) - 1 callback: basePowerCallback
- [x] curse - Curse (Status, Ghost) - 5 callbacks: onModifyMove, onTryHit, onHit, condition::onStart, condition::onResidual
- [x] darkvoid - Dark Void (Status, Dark) - 1 callback: onTry
- [x] defog - Defog (Status, Flying) - 1 callback: onHit
- [x] destinybond - Destiny Bond (Status, Ghost) - 5 callbacks: onPrepareHit, condition::onStart, condition::onFaint, condition::onBeforeMove, condition::onMoveAborted
- [x] detect - Detect (Status, Fighting) - 2 callbacks: onPrepareHit, onHit
- [x] dig - Dig (Physical, Ground) - 4 callbacks: onTryMove, condition::onImmunity, condition::onInvulnerability, condition::onSourceModifyDamage
- [x] disable - Disable (Status, Normal) - 5 callbacks: onTryHit, condition::onStart, condition::onEnd, condition::onBeforeMove, condition::onDisableMove
- [x] dive - Dive (Physical, Water) - 4 callbacks: onTryMove, condition::onImmunity, condition::onInvulnerability, condition::onSourceModifyDamage
- [x] doodle - Doodle (Status, Normal) - 1 callback: onHit
- [x] doomdesire - Doom Desire (Special, Steel) - 1 callback: onTry
- [x] doubleshock - Double Shock (Physical, Electric) - 1 callback: onTryMove
- [x] dragoncheer - Dragon Cheer (Status, Dragon) - 2 callbacks: condition::onStart, condition::onModifyCritRatio
- [x] dragonenergy - Dragon Energy (Special, Dragon) - 1 callback: basePowerCallback
- [x] dreameater - Dream Eater (Special, Psychic) - 1 callback: onTryImmunity
- [x] echoedvoice - Echoed Voice (Special, Normal) - 4 callbacks: basePowerCallback, onTryMove, condition::onFieldStart, condition::onFieldRestart
- [x] electricterrain - Electric Terrain (Status, Electric) - 6 callbacks: condition::durationCallback, condition::onSetStatus, condition::onTryAddVolatile, condition::onBasePower, condition::onFieldStart, condition::onFieldEnd
- [x] electrify - Electrify (Status, Electric) - 3 callbacks: onTryHit, condition::onStart, condition::onModifyType
- [x] electroball - Electro Ball (Special, Electric) - 1 callback: basePowerCallback
- [x] electrodrift - Electro Drift (Special, Electric) - 1 callback: onBasePower
- [x] electroshot - Electro Shot (Special, Electric) - 1 callback: onTryMove
- [x] embargo - Embargo (Status, Dark) - 2 callbacks: condition::onStart, condition::onEnd
- [x] encore - Encore (Status, Normal) - 5 callbacks: condition::onStart, condition::onOverrideAction, condition::onResidual, condition::onEnd, condition::onDisableMove
- [x] endeavor - Endeavor (Physical, Normal) - 2 callbacks: damageCallback, onTryImmunity
- [x] endure - Endure (Status, Normal) - 4 callbacks: onPrepareHit, onHit, condition::onStart, condition::onDamage
- [x] entrainment - Entrainment (Status, Normal) - 2 callbacks: onTryHit, onHit
- [x] eruption - Eruption (Special, Fire) - 1 callback: basePowerCallback
- [x] expandingforce - Expanding Force (Special, Psychic) - 2 callbacks: onBasePower, onModifyMove
- [x] facade - Facade (Physical, Normal) - 1 callback: onBasePower
- [x] fairylock - Fairy Lock (Status, Fairy) - 2 callbacks: condition::onFieldStart, condition::onTrapPokemon
- [x] fakeout - Fake Out (Physical, Normal) - 1 callback: onTry
- [x] falseswipe - False Swipe (Physical, Normal) - 1 callback: onDamage
- [x] fellstinger - Fell Stinger (Physical, Bug) - 1 callback: onAfterMoveSecondarySelf
- [x] ficklebeam - Fickle Beam (Special, Dragon) - 1 callback: onBasePower
- [x] filletaway - Fillet Away (Status, Normal) - 3 callbacks: onTry, onTryHit, onHit
- [x] finalgambit - Final Gambit (Special, Fighting) - 1 callback: damageCallback
- [ ] firepledge - Fire Pledge (Special, Fire) - 6 callbacks: basePowerCallback ✓, onPrepareHit, onModifyMove, condition::onSideStart, condition::onResidual, condition::onSideEnd (1/6 implemented)
- [x] firstimpression - First Impression (Physical, Bug) - 1 callback: onTry
- [x] fishiousrend - Fishious Rend (Physical, Water) - 1 callback: basePowerCallback
- [x] flail - Flail (Physical, Normal) - 1 callback: basePowerCallback
- [x] flameburst - Flame Burst (Special, Fire) - 2 callbacks: onHit, onAfterSubDamage
- [ ] fling - Fling (Physical, Dark) - 2 callbacks: onPrepareHit, condition::onUpdate ✓ (1/2 implemented)
- [x] floralhealing - Floral Healing (Status, Fairy) - 1 callback: onHit
- [x] flowershield - Flower Shield (Status, Fairy) - 1 callback: onHitField
- [x] fly - Fly (Physical, Flying) - 3 callbacks: onTryMove, condition::onInvulnerability, condition::onSourceModifyDamage
- [x] flyingpress - Flying Press (Physical, Fighting) - 1 callback: onEffectiveness
- [x] focusenergy - Focus Energy (Status, Normal) - 2 callbacks: condition::onStart, condition::onModifyCritRatio
- [x] focuspunch - Focus Punch (Physical, Fighting) - 5 callbacks: priorityChargeCallback, beforeMoveCallback, condition::onStart, condition::onHit, condition::onTryAddVolatile
- [x] followme - Follow Me (Status, Normal) - 3 callbacks: onTry, condition::onStart, condition::onFoeRedirectTarget
- [x] foresight - Foresight (Status, Normal) - 4 callbacks: onTryHit, condition::onStart, condition::onNegateImmunity, condition::onModifyBoost
- [x] forestscurse - Forest's Curse (Status, Grass) - 1 callback: onHit
- [x] freezedry - Freeze-Dry (Special, Ice) - 1 callback: onEffectiveness
- [x] freezeshock - Freeze Shock (Physical, Ice) - 1 callback: onTryMove
- [x] freezyfrost - Freezy Frost (Special, Ice) - 1 callback: onHit
- [x] frustration - Frustration (Physical, Normal) - 1 callback: basePowerCallback
- [x] furycutter - Fury Cutter (Physical, Bug) - 3 callbacks: basePowerCallback, condition::onStart, condition::onRestart
- [x] fusionbolt - Fusion Bolt (Physical, Electric) - 1 callback: onBasePower
- [x] fusionflare - Fusion Flare (Special, Fire) - 1 callback: onBasePower
- [x] futuresight - Future Sight (Special, Psychic) - 1 callback: onTry
- [x] gastroacid - Gastro Acid (Status, Poison) - 3 callbacks: onTryHit, condition::onStart, condition::onCopy
- [x] gearup - Gear Up (Status, Steel) - 1 callback: onHitSide
- [x] geomancy - Geomancy (Status, Fairy) - 1 callback: onTryMove
- [x] glaiverush - Glaive Rush (Physical, Dragon) - 4 callbacks: condition::onStart, condition::onAccuracy, condition::onSourceModifyDamage, condition::onBeforeMove
- [x] gmaxcannonade - G-Max Cannonade (Physical, Water) - 3 callbacks: condition::onSideStart, condition::onResidual, condition::onSideEnd
- [x] gmaxchistrike - G-Max Chi Strike (Physical, Fighting) - 3 callbacks: condition::onStart, condition::onRestart, condition::onModifyCritRatio
- [x] gmaxsnooze - G-Max Snooze (Physical, Dark) - 2 callbacks: onHit, onAfterSubDamage
- [x] gmaxsteelsurge - G-Max Steelsurge (Physical, Steel) - 2 callbacks: condition::onSideStart, condition::onSwitchIn
- [x] gmaxvinelash - G-Max Vine Lash (Physical, Grass) - 3 callbacks: condition::onSideStart, condition::onResidual, condition::onSideEnd
- [x] gmaxvolcalith - G-Max Volcalith (Physical, Rock) - 3 callbacks: condition::onSideStart, condition::onResidual, condition::onSideEnd
- [x] gmaxwildfire - G-Max Wildfire (Physical, Fire) - 3 callbacks: condition::onSideStart, condition::onResidual, condition::onSideEnd
- [x] grassknot - Grass Knot (Special, Grass) - 2 callbacks: basePowerCallback, onTryHit
- [x] grasspledge - Grass Pledge (Special, Grass) - 6 callbacks: basePowerCallback, onPrepareHit, onModifyMove, condition::onSideStart, condition::onSideEnd, condition::onModifySpe
- [x] grassyglide - Grassy Glide (Physical, Grass) - 1 callback: onModifyPriority
- [x] grassyterrain - Grassy Terrain (Status, Grass) - 5 callbacks: condition::durationCallback, condition::onBasePower, condition::onFieldStart, condition::onResidual, condition::onFieldEnd
- [x] gravapple - Grav Apple (Physical, Grass) - 1 callback: onBasePower
- [x] gravity - Gravity (Status, Psychic) - 7 callbacks: condition::durationCallback, condition::onFieldStart, condition::onModifyAccuracy, condition::onDisableMove, condition::onBeforeMove, condition::onModifyMove, condition::onFieldEnd
- [x] growth - Growth (Status, Normal) - 1 callback: onModifyMove
- [x] grudge - Grudge (Status, Ghost) - 3 callbacks: condition::onStart, condition::onFaint, condition::onBeforeMove
- [x] guardianofalola - Guardian of Alola (Special, Fairy) - 1 callback: damageCallback
- [x] guardsplit - Guard Split (Status, Psychic) - 1 callback: onHit
- [x] guardswap - Guard Swap (Status, Psychic) - 1 callback: onHit
- [x] gyroball - Gyro Ball (Physical, Steel) - 1 callback: basePowerCallback
- [x] happyhour - Happy Hour (Status, Normal) - 1 callback: onTryHit
- [x] hardpress - Hard Press (Physical, Steel) - 1 callback: basePowerCallback
- [x] haze - Haze (Status, Ice) - 1 callback: onHitField
- [x] healbell - Heal Bell (Status, Normal) - 1 callback: onHit
- [ ] healblock - Heal Block (Status, Psychic) - 8 callbacks: condition::durationCallback ✓, condition::onStart ✓, condition::onDisableMove ✓, condition::onBeforeMove ✓, condition::onModifyMove ✓, condition::onEnd ✓, condition::onTryHeal, condition::onRestart ✓ (7/8 implemented)
- [x] healingwish - Healing Wish (Status, Psychic) - 3 callbacks: onTryHit, condition::onSwitchIn, condition::onSwap
- [x] healpulse - Heal Pulse (Status, Psychic) - 1 callback: onHit
- [x] heartswap - Heart Swap (Status, Psychic) - 1 callback: onHit
- [x] heatcrash - Heat Crash (Physical, Fire) - 2 callbacks: basePowerCallback, onTryHit
- [x] heavyslam - Heavy Slam (Physical, Steel) - 2 callbacks: basePowerCallback, onTryHit
- [x] helpinghand - Helping Hand (Status, Normal) - 4 callbacks: onTryHit, condition::onStart, condition::onRestart, condition::onBasePower
- [x] hex - Hex (Special, Ghost) - 1 callback: basePowerCallback
- [x] hiddenpower - Hidden Power (Special, Normal) - 1 callback: onModifyType
- [x] highjumpkick - High Jump Kick (Physical, Fighting) - 1 callback: onMoveFail
- [x] holdback - Hold Back (Physical, Normal) - 1 callback: onDamage
- [x] hurricane - Hurricane (Special, Flying) - 1 callback: onModifyMove
- [x] hyperspacefury - Hyperspace Fury (Physical, Dark) - 1 callback: onTry
- [x] iceball - Ice Ball (Physical, Ice) - 5 callbacks: basePowerCallback, onModifyMove, onAfterMove, condition::onStart, condition::onResidual
- [x] iceburn - Ice Burn (Special, Ice) - 1 callback: onTryMove
- [x] icespinner - Ice Spinner (Physical, Ice) - 2 callbacks: onAfterHit, onAfterSubDamage
- [x] imprison - Imprison (Status, Psychic) - 3 callbacks: condition::onStart, condition::onFoeDisableMove, condition::onFoeBeforeMove
- [x] incinerate - Incinerate (Special, Fire) - 1 callback: onHit
- [x] infernalparade - Infernal Parade (Special, Ghost) - 1 callback: basePowerCallback
- [x] ingrain - Ingrain (Status, Grass) - 4 callbacks: condition::onStart, condition::onResidual, condition::onTrapPokemon, condition::onDragOut
- [x] instruct - Instruct (Status, Psychic) - 1 callback: onHit
- [x] iondeluge - Ion Deluge (Status, Electric) - 2 callbacks: condition::onFieldStart, condition::onModifyType
- [x] ivycudgel - Ivy Cudgel (Physical, Grass) - 2 callbacks: onPrepareHit, onModifyType
- [x] jawlock - Jaw Lock (Physical, Dark) - 1 callback: onHit
- [x] judgment - Judgment (Special, Normal) - 1 callback: onModifyType
- [x] jumpkick - Jump Kick (Physical, Fighting) - 1 callback: onMoveFail
- [x] junglehealing - Jungle Healing (Status, Grass) - 1 callback: onHit
- [x] kingsshield - King's Shield (Status, Steel) - 5 callbacks: onPrepareHit, onHit, condition::onStart, condition::onTryHit, condition::onHit
- [x] knockoff - Knock Off (Physical, Dark) - 2 callbacks: onBasePower, onAfterHit
- [x] laserfocus - Laser Focus (Status, Normal) - 4 callbacks: condition::onStart, condition::onRestart, condition::onModifyCritRatio, condition::onEnd
- [x] lashout - Lash Out (Physical, Dark) - 1 callback: onBasePower
- [x] lastresort - Last Resort (Physical, Normal) - 1 callback: onTry
- [x] lastrespects - Last Respects (Physical, Ghost) - 1 callback: basePowerCallback
- [x] leechseed - Leech Seed (Status, Grass) - 3 callbacks: onTryImmunity, condition::onStart, condition::onResidual
- [x] lightscreen - Light Screen (Status, Psychic) - 4 callbacks: condition::durationCallback, condition::onAnyModifyDamage, condition::onSideStart, condition::onSideEnd
- [x] lightthatburnsthesky - Light That Burns the Sky (Special, Psychic) - 1 callback: onModifyMove
- [x] lockon - Lock-On (Status, Normal) - 4 callbacks: onTryHit, onHit, condition::onSourceInvulnerability, condition::onSourceAccuracy
- [x] lowkick - Low Kick (Physical, Fighting) - 2 callbacks: basePowerCallback, onTryHit
- [x] luckychant - Lucky Chant (Status, Normal) - 2 callbacks: condition::onSideStart, condition::onSideEnd
- [x] lunarblessing - Lunar Blessing (Status, Psychic) - 1 callback: onHit
- [x] lunardance - Lunar Dance (Status, Psychic) - 3 callbacks: onTryHit, condition::onSwitchIn, condition::onSwap
- [x] magiccoat - Magic Coat (Status, Psychic) - 3 callbacks: condition::onStart, condition::onTryHit, condition::onAllyTryHitSide
- [x] magicpowder - Magic Powder (Status, Psychic) - 1 callback: onHit
- [x] magicroom - Magic Room (Status, Psychic) - 4 callbacks: condition::durationCallback, condition::onFieldStart, condition::onFieldRestart, condition::onFieldEnd
- [x] magneticflux - Magnetic Flux (Status, Electric) - 1 callback: onHitSide
- [x] magnetrise - Magnet Rise (Status, Electric) - 4 callbacks: onTry, condition::onStart, condition::onImmunity, condition::onEnd
- [x] magnitude - Magnitude (Physical, Ground) - 2 callbacks: onModifyMove, onUseMoveMessage
- [x] matblock - Mat Block (Status, Fighting) - 3 callbacks: onTry, condition::onSideStart, condition::onTryHit
- [x] maxguard - Max Guard (Status, Normal) - 4 callbacks: onPrepareHit, onHit, condition::onStart, condition::onTryHit
- [x] meanlook - Mean Look (Status, Normal) - 1 callback: onHit
- [x] mefirst - Me First (Status, Normal) - 2 callbacks: onTryHit, condition::onBasePower
- [x] metalburst - Metal Burst (Physical, Steel) - 3 callbacks: damageCallback, onTry, onModifyTarget
- [x] meteorbeam - Meteor Beam (Special, Rock) - 1 callback: onTryMove
- [x] metronome - Metronome (Status, Normal) - 1 callback: onHit
- [x] mimic - Mimic (Status, Normal) - 1 callback: onHit
- [x] mindblown - Mind Blown (Special, Fire) - 1 callback: onAfterMove
- [x] mindreader - Mind Reader (Status, Normal) - 2 callbacks: onTryHit, onHit
- [x] minimize - Minimize (Status, Normal) - 2 callbacks: condition::onSourceModifyDamage, condition::onAccuracy
- [x] miracleeye - Miracle Eye (Status, Psychic) - 4 callbacks: onTryHit, condition::onStart, condition::onNegateImmunity, condition::onModifyBoost
- [x] mirrorcoat - Mirror Coat (Special, Psychic) - 6 callbacks: damageCallback, beforeTurnCallback, onTry, condition::onStart, condition::onRedirectTarget, condition::onDamagingHit
- [x] mirrormove - Mirror Move (Status, Flying) - 1 callback: onTryHit
- [x] mist - Mist (Status, Ice) - 3 callbacks: condition::onTryBoost, condition::onSideStart, condition::onSideEnd
- [x] mistyexplosion - Misty Explosion (Special, Fairy) - 1 callback: onBasePower
- [x] mistyterrain - Misty Terrain (Status, Fairy) - 6 callbacks: condition::durationCallback, condition::onSetStatus, condition::onTryAddVolatile, condition::onBasePower, condition::onFieldStart, condition::onFieldEnd
- [x] moonlight - Moonlight (Status, Fairy) - 1 callback: onHit
- [x] morningsun - Morning Sun (Status, Normal) - 1 callback: onHit
- [x] mortalspin - Mortal Spin (Physical, Poison) - 2 callbacks: onAfterHit, onAfterSubDamage
- [x] mudsport - Mud Sport (Status, Ground) - 3 callbacks: condition::onFieldStart, condition::onBasePower, condition::onFieldEnd
- [x] multiattack - Multi-Attack (Physical, Normal) - 1 callback: onModifyType
- [x] naturalgift - Natural Gift (Physical, Normal) - 2 callbacks: onModifyType, onPrepareHit
- [x] naturepower - Nature Power (Status, Normal) - 1 callback: onTryHit
- [x] naturesmadness - Nature's Madness (Special, Fairy) - 1 callback: damageCallback
- [x] nightmare - Nightmare (Status, Ghost) - 2 callbacks: condition::onStart, condition::onResidual
- [x] noretreat - No Retreat (Status, Fighting) - 3 callbacks: onTry, condition::onStart, condition::onTrapPokemon
- [x] obstruct - Obstruct (Status, Dark) - 5 callbacks: onPrepareHit, onHit, condition::onStart, condition::onTryHit, condition::onHit
- [x] octolock - Octolock (Status, Fighting) - 4 callbacks: onTryImmunity, condition::onStart, condition::onResidual, condition::onTrapPokemon
- [x] odorsleuth - Odor Sleuth (Status, Normal) - 1 callback: onTryHit
- [x] orderup - Order Up (Physical, Dragon) - 1 callback: onAfterMoveSecondarySelf
- [x] painsplit - Pain Split (Status, Normal) - 1 callback: onHit
- [x] partingshot - Parting Shot (Status, Dark) - 1 callback: onHit
- [x] payback - Payback (Physical, Dark) - 1 callback: basePowerCallback
- [x] perishsong - Perish Song (Status, Normal) - 3 callbacks: onHitField, condition::onEnd, condition::onResidual
- [x] phantomforce - Phantom Force (Physical, Ghost) - 1 callback: onTryMove
- [x] photongeyser - Photon Geyser (Special, Psychic) - 1 callback: onModifyMove
- [x] pikapapow - Pika Papow (Special, Electric) - 1 callback: basePowerCallback
- [x] pluck - Pluck (Physical, Flying) - 1 callback: onHit
- [x] polarflare - Polar Flare (Special, Fire) - 2 callbacks: onHit, onAfterMoveSecondarySelf
- [x] pollenpuff - Pollen Puff (Special, Bug) - 3 callbacks: onTryHit, onTryMove, onHit
- [x] poltergeist - Poltergeist (Physical, Ghost) - 2 callbacks: onTry, onTryHit
- [x] powder - Powder (Status, Bug) - 2 callbacks: condition::onStart, condition::onTryMove
- [x] powershift - Power Shift (Status, Normal) - 4 callbacks: condition::onStart, condition::onCopy, condition::onEnd, condition::onRestart
- [x] powersplit - Power Split (Status, Psychic) - 1 callback: onHit
- [x] powerswap - Power Swap (Status, Psychic) - 1 callback: onHit
- [x] powertrick - Power Trick (Status, Psychic) - 4 callbacks: condition::onStart, condition::onCopy, condition::onEnd, condition::onRestart
- [x] powertrip - Power Trip (Physical, Dark) - 1 callback: basePowerCallback
- [x] present - Present (Physical, Normal) - 1 callback: onModifyMove
- [x] protect - Protect (Status, Normal) - 4 callbacks: onPrepareHit, onHit, condition::onStart, condition::onTryHit
- [x] psyblade - Psyblade (Physical, Psychic) - 1 callback: onBasePower
- [x] psychicfangs - Psychic Fangs (Physical, Psychic) - 1 callback: onTryHit
- [x] psychicterrain - Psychic Terrain (Status, Psychic) - 5 callbacks: condition::durationCallback, condition::onTryHit, condition::onBasePower, condition::onFieldStart, condition::onFieldEnd
- [x] psychoshift - Psycho Shift (Status, Psychic) - 1 callback: onTryHit
- [x] psychup - Psych Up (Status, Normal) - 1 callback: onHit
- [x] psywave - Psywave (Special, Psychic) - 1 callback: damageCallback
- [x] punishment - Punishment (Physical, Dark) - 1 callback: basePowerCallback
- [x] purify - Purify (Status, Poison) - 1 callback: onHit
- [ ] pursuit - Pursuit (Physical, Dark) - 5 callbacks: basePowerCallback ✓, beforeTurnCallback, onModifyMove ✓, onTryHit ✓, condition::onBeforeSwitchOut (3/5 implemented)
- [x] quash - Quash (Status, Dark) - 1 callback: onHit
- [x] quickguard - Quick Guard (Status, Fighting) - 4 callbacks: onTry, onHitSide, condition::onSideStart, condition::onTryHit
- [x] rage - Rage (Physical, Normal) - 3 callbacks: condition::onStart, condition::onHit, condition::onBeforeMove
- [x] ragefist - Rage Fist (Physical, Ghost) - 1 callback: basePowerCallback
- [x] ragepowder - Rage Powder (Status, Bug) - 3 callbacks: onTry, condition::onStart, condition::onFoeRedirectTarget
- [x] ragingbull - Raging Bull (Physical, Normal) - 2 callbacks: onTryHit, onModifyType
- [x] rapidspin - Rapid Spin (Physical, Normal) - 2 callbacks: onAfterHit, onAfterSubDamage
- [x] razorwind - Razor Wind (Special, Normal) - 1 callback: onTryMove
- [x] recycle - Recycle (Status, Normal) - 1 callback: onHit
- [x] reflect - Reflect (Status, Psychic) - 4 callbacks: condition::durationCallback, condition::onAnyModifyDamage, condition::onSideStart, condition::onSideEnd
- [x] reflecttype - Reflect Type (Status, Normal) - 1 callback: onHit
- [x] refresh - Refresh (Status, Normal) - 1 callback: onHit
- [x] relicsong - Relic Song (Special, Normal) - 2 callbacks: onHit, onAfterMoveSecondarySelf
- [x] rest - Rest (Status, Psychic) - 2 callbacks: onTry, onHit
- [x] retaliate - Retaliate (Physical, Normal) - 1 callback: onBasePower
- [x] return - Return (Physical, Normal) - 1 callback: basePowerCallback
- [x] revelationdance - Revelation Dance (Special, Normal) - 1 callback: onModifyType
- [x] revenge - Revenge (Physical, Fighting) - 1 callback: basePowerCallback
- [x] reversal - Reversal (Physical, Fighting) - 1 callback: basePowerCallback
- [x] revivalblessing - Revival Blessing (Status, Normal) - 1 callback: onTryHit
- [x] risingvoltage - Rising Voltage (Special, Electric) - 1 callback: basePowerCallback
- [x] roleplay - Role Play (Status, Psychic) - 2 callbacks: onTryHit, onHit
- [x] rollout - Rollout (Physical, Rock) - 5 callbacks: basePowerCallback, onModifyMove, onAfterMove, condition::onStart, condition::onResidual
- [x] roost - Roost (Status, Flying) - 2 callbacks: condition::onStart, condition::onType
- [x] rototiller - Rototiller (Status, Ground) - 1 callback: onHitField
- [x] round - Round (Special, Normal) - 2 callbacks: basePowerCallback, onTry
- [x] ruination - Ruination (Special, Dark) - 1 callback: damageCallback
- [x] safeguard - Safeguard (Status, Normal) - 5 callbacks: condition::durationCallback, condition::onSetStatus, condition::onTryAddVolatile, condition::onSideStart, condition::onSideEnd
- [x] saltcure - Salt Cure (Physical, Rock) - 3 callbacks: condition::onStart, condition::onResidual, condition::onEnd
- [x] sandsearstorm - Sandsear Storm (Special, Ground) - 1 callback: onModifyMove
- [x] sappyseed - Sappy Seed (Physical, Grass) - 1 callback: onHit
- [x] secretpower - Secret Power (Physical, Normal) - 1 callback: onModifyMove
- [x] shadowforce - Shadow Force (Physical, Ghost) - 1 callback: onTryMove
- [x] shedtail - Shed Tail (Status, Normal) - 2 callbacks: onTryHit, onHit
- [x] shellsidearm - Shell Side Arm (Special, Poison) - 4 callbacks: onPrepareHit, onModifyMove, onHit, onAfterSubDamage
- [x] shelltrap - Shell Trap (Special, Fire) - 4 callbacks: priorityChargeCallback, onTryMove, condition::onStart, condition::onHit
- [x] shoreup - Shore Up (Status, Ground) - 1 callback: onHit
- [x] silktrap - Silk Trap (Status, Bug) - 5 callbacks: onPrepareHit, onHit, condition::onStart, condition::onTryHit, condition::onHit
- [x] simplebeam - Simple Beam (Status, Normal) - 2 callbacks: onTryHit, onHit
- [x] sketch - Sketch (Status, Normal) - 1 callback: onHit
- [x] skillswap - Skill Swap (Status, Psychic) - 2 callbacks: onTryHit, onHit
- [x] skullbash - Skull Bash (Physical, Normal) - 1 callback: onTryMove
- [x] skyattack - Sky Attack (Physical, Flying) - 1 callback: onTryMove
- [x] skydrop - Sky Drop (Physical, Flying) - 12 callbacks: onModifyMove, onMoveFail, onTry, onTryHit, onHit, condition::onAnyDragOut, condition::onFoeTrapPokemon, condition::onFoeBeforeMove, condition::onRedirectTarget, condition::onAnyInvulnerability, condition::onAnyBasePower, condition::onFaint
- [x] sleeptalk - Sleep Talk (Status, Normal) - 2 callbacks: onTry, onHit
- [x] smackdown - Smack Down (Physical, Rock) - 2 callbacks: condition::onStart, condition::onRestart
- [x] smellingsalts - Smelling Salts (Physical, Normal) - 2 callbacks: basePowerCallback, onHit
- [x] snatch - Snatch (Status, Dark) - 2 callbacks: condition::onStart, condition::onAnyPrepareHit
- [x] snore - Snore (Special, Normal) - 1 callback: onTry
- [x] soak - Soak (Status, Water) - 1 callback: onHit
- [x] solarbeam - Solar Beam (Special, Grass) - 2 callbacks: onTryMove, onBasePower
- [x] solarblade - Solar Blade (Physical, Grass) - 2 callbacks: onTryMove, onBasePower
- [x] sparklingaria - Sparkling Aria (Special, Water) - 1 callback: onAfterMove
- [x] speedswap - Speed Swap (Status, Psychic) - 1 callback: onHit
- [x] spiderweb - Spider Web (Status, Bug) - 1 callback: onHit
- [x] spikes - Spikes (Status, Ground) - 3 callbacks: condition::onSideStart, condition::onSideRestart, condition::onSwitchIn
- [x] spikyshield - Spiky Shield (Status, Grass) - 5 callbacks: onPrepareHit, onHit, condition::onStart, condition::onTryHit, condition::onHit
- [x] spite - Spite (Status, Ghost) - 1 callback: onHit
- [x] spitup - Spit Up (Special, Normal) - 3 callbacks: basePowerCallback, onTry, onAfterMove
- [x] splash - Splash (Status, Normal) - 2 callbacks: onTry, onTryHit
- [x] splinteredstormshards - Splintered Stormshards (Physical, Rock) - 2 callbacks: onHit, onAfterSubDamage
- [x] spotlight - Spotlight (Status, Normal) - 3 callbacks: onTryHit, condition::onStart, condition::onFoeRedirectTarget
- [x] stealthrock - Stealth Rock (Status, Rock) - 2 callbacks: condition::onSideStart, condition::onSwitchIn
- [x] steelbeam - Steel Beam (Special, Steel) - 1 callback: onAfterMove
- [x] steelroller - Steel Roller (Physical, Steel) - 3 callbacks: onTry, onHit, onAfterSubDamage
- [x] stickyweb - Sticky Web (Status, Bug) - 2 callbacks: condition::onSideStart, condition::onSwitchIn
- [x] stockpile - Stockpile (Status, Normal) - 4 callbacks: onTry, condition::onStart, condition::onRestart, condition::onEnd
- [x] stompingtantrum - Stomping Tantrum (Physical, Ground) - 1 callback: basePowerCallback
- [x] stoneaxe - Stone Axe (Physical, Rock) - 2 callbacks: onAfterHit, onAfterSubDamage
- [x] storedpower - Stored Power (Special, Psychic) - 1 callback: basePowerCallback
- [x] strengthsap - Strength Sap (Status, Grass) - 1 callback: onHit
- [x] struggle - Struggle (Physical, Normal) - 1 callback: onModifyMove
- [x] stuffcheeks - Stuff Cheeks (Status, Normal) - 3 callbacks: onDisableMove, onTry, onHit
- [ ] substitute - Substitute (Status, Normal) - 5 callbacks: onTryHit ✓, onHit ✓, condition::onStart ✓, condition::onTryPrimaryHit, condition::onEnd ✓ (4/5 implemented)
- [x] suckerpunch - Sucker Punch (Physical, Dark) - 1 callback: onTry
- [x] supercellslam - Supercell Slam (Physical, Electric) - 1 callback: onMoveFail
- [x] superfang - Super Fang (Physical, Normal) - 1 callback: damageCallback
- [x] swallow - Swallow (Status, Normal) - 2 callbacks: onTry, onHit
- [ ] switcheroo - Switcheroo (Status, Dark) - 2 callbacks: onTryImmunity ✓, onHit (1/2 implemented)
- [x] synchronoise - Synchronoise (Special, Psychic) - 1 callback: onTryImmunity
- [x] synthesis - Synthesis (Status, Grass) - 1 callback: onHit
- [x] syrupbomb - Syrup Bomb (Special, Grass) - 4 callbacks: condition::onStart, condition::onUpdate, condition::onResidual, condition::onEnd
- [x] tailwind - Tailwind (Status, Flying) - 4 callbacks: condition::durationCallback, condition::onSideStart, condition::onModifySpe, condition::onSideEnd
- [x] takeheart - Take Heart (Status, Psychic) - 1 callback: onHit
- [ ] tarshot - Tar Shot (Status, Rock) - 2 callbacks: condition::onStart ✓, condition::onEffectiveness (1/2 implemented)
- [ ] taunt - Taunt (Status, Dark) - 4 callbacks: condition::onStart ✓, condition::onEnd ✓, condition::onDisableMove ✓, condition::onBeforeMove (3/4 implemented)
- [x] teatime - Teatime (Status, Normal) - 1 callback: onHitField
- [ ] technoblast - Techno Blast (Special, Normal) - 1 callback: onModifyType
- [ ] telekinesis - Telekinesis (Status, Psychic) - 6 callbacks: onTry, condition::onStart, condition::onAccuracy, condition::onImmunity, condition::onUpdate, condition::onEnd ✓ (1/6 implemented)
- [x] teleport - Teleport (Status, Psychic) - 1 callback: onTry
- [x] temperflare - Temper Flare (Physical, Fire) - 1 callback: basePowerCallback
- [ ] terablast - Tera Blast (Special, Normal) - 4 callbacks: basePowerCallback ✓, onPrepareHit, onModifyType, onModifyMove (1/4 implemented)
- [ ] terastarstorm - Tera Starstorm (Special, Normal) - 2 callbacks: onModifyType, onModifyMove
- [ ] terrainpulse - Terrain Pulse (Special, Normal) - 2 callbacks: onModifyType, onModifyMove
- [ ] thief - Thief (Physical, Dark) - 1 callback: onAfterHit
- [ ] thousandarrows - Thousand Arrows (Physical, Ground) - 1 callback: onEffectiveness
- [x] thousandwaves - Thousand Waves (Physical, Ground) - 1 callback: onHit
- [x] throatchop - Throat Chop (Physical, Dark) - 5 callbacks: condition::onStart, condition::onDisableMove, condition::onBeforeMove, condition::onModifyMove, condition::onEnd
- [x] thunder - Thunder (Special, Electric) - 1 callback: onModifyMove
- [x] thunderclap - Thunderclap (Special, Electric) - 1 callback: onTry
- [x] tidyup - Tidy Up (Status, Normal) - 1 callback: onHit
- [x] topsyturvy - Topsy-Turvy (Status, Dark) - 1 callback: onHit
- [x] torment - Torment (Status, Dark) - 3 callbacks: condition::onStart, condition::onEnd, condition::onDisableMove
- [x] toxicspikes - Toxic Spikes (Status, Poison) - 3 callbacks: condition::onSideStart, condition::onSideRestart, condition::onSwitchIn
- [x] transform - Transform (Status, Normal) - 1 callback: onHit
- [ ] trick - Trick (Status, Psychic) - 2 callbacks: onTryImmunity ✓, onHit (1/2 implemented)
- [x] trickortreat - Trick-or-Treat (Status, Ghost) - 1 callback: onHit
- [x] trickroom - Trick Room (Status, Psychic) - 4 callbacks: condition::durationCallback, condition::onFieldStart, condition::onFieldRestart, condition::onFieldEnd
- [x] tripleaxel - Triple Axel (Physical, Ice) - 1 callback: basePowerCallback
- [x] triplekick - Triple Kick (Physical, Fighting) - 1 callback: basePowerCallback
- [x] trumpcard - Trump Card (Special, Normal) - 1 callback: basePowerCallback
- [x] upperhand - Upper Hand (Physical, Fighting) - 1 callback: onTry
- [ ] uproar - Uproar (Special, Normal) - 5 callbacks: onTryHit ✓, condition::onStart ✓, condition::onResidual ✓, condition::onEnd ✓, condition::onAnySetStatus (4/5 implemented)
- [x] veeveevolley - Veevee Volley (Physical, Normal) - 1 callback: basePowerCallback
- [x] venomdrench - Venom Drench (Status, Poison) - 1 callback: onHit
- [x] venoshock - Venoshock (Special, Poison) - 1 callback: onBasePower
- [x] wakeupslap - Wake-Up Slap (Physical, Fighting) - 2 callbacks: basePowerCallback, onHit
- [ ] waterpledge - Water Pledge (Special, Water) - 6 callbacks: basePowerCallback ✓, onPrepareHit, onModifyMove, condition::onSideStart, condition::onSideEnd, condition::onModifyMove (1/6 implemented)
- [x] watershuriken - Water Shuriken (Special, Water) - 1 callback: basePowerCallback
- [x] watersport - Water Sport (Status, Water) - 3 callbacks: condition::onFieldStart, condition::onBasePower, condition::onFieldEnd
- [x] waterspout - Water Spout (Special, Water) - 1 callback: basePowerCallback
- [x] weatherball - Weather Ball (Special, Normal) - 2 callbacks: onModifyType, onModifyMove
- [x] wideguard - Wide Guard (Status, Rock) - 4 callbacks: onTry, onHitSide, condition::onSideStart, condition::onTryHit
- [x] wildboltstorm - Wildbolt Storm (Special, Electric) - 1 callback: onModifyMove
- [ ] wish - Wish (Status, Normal) - 3 callbacks: condition::onStart ✓, condition::onResidual, condition::onEnd ✓ (2/3 implemented)
- [x] wonderroom - Wonder Room (Status, Psychic) - 5 callbacks: condition::durationCallback, condition::onModifyMove, condition::onFieldStart, condition::onFieldRestart, condition::onFieldEnd (1/5 implemented)
- [x] worryseed - Worry Seed (Status, Grass) - 3 callbacks: onTryImmunity, onTryHit, onHit
- [x] wringout - Wring Out (Special, Normal) - 1 callback: basePowerCallback
- [x] yawn - Yawn (Status, Normal) - 3 callbacks: onTryHit, condition::onStart, condition::onEnd

## Statistics

By callback type:
- onHit: 105 moves
- condition::onStart: 69 moves
- basePowerCallback: 53 moves
- onTryHit: 44 moves
- onTry: 40 moves
- onModifyMove: 28 moves
- condition::onEnd: 23 moves
- condition::onSideStart: 23 moves
- onTryMove: 21 moves
- onPrepareHit: 20 moves
- condition::onResidual: 20 moves
- onBasePower: 18 moves
- condition::durationCallback: 14 moves
- condition::onSideEnd: 14 moves
- condition::onTryHit: 14 moves
- onModifyType: 13 moves
- condition::onFieldStart: 13 moves
- condition::onBeforeMove: 12 moves
- condition::onRestart: 11 moves
- onAfterSubDamage: 11 moves
- damageCallback: 11 moves
- onTryImmunity: 10 moves
- condition::onHit: 10 moves
- condition::onFieldEnd: 10 moves
- condition::onBasePower: 9 moves
- onAfterHit: 8 moves
- onAfterMove: 7 moves
- condition::onDisableMove: 7 moves
- condition::onSwitchIn: 7 moves
- onHitField: 6 moves
- onMoveFail: 5 moves
- condition::onSourceModifyDamage: 5 moves
- condition::onModifyMove: 5 moves
- condition::onUpdate: 4 moves
- priorityChargeCallback: 4 moves
- condition::onInvulnerability: 4 moves
- condition::onImmunity: 4 moves
- condition::onModifyCritRatio: 4 moves
- condition::onFieldRestart: 4 moves
- condition::onTryAddVolatile: 4 moves
- condition::onTrapPokemon: 4 moves
- onAfterMoveSecondarySelf: 4 moves
- onHitSide: 4 moves
- condition::onAnyModifyDamage: 3 moves
- condition::onMoveAborted: 3 moves
- beforeTurnCallback: 3 moves
- condition::onRedirectTarget: 3 moves
- condition::onFaint: 3 moves
- condition::onSetStatus: 3 moves
- onEffectiveness: 3 moves
- condition::onFoeRedirectTarget: 3 moves
- condition::onCopy: 3 moves
- condition::onAccuracy: 3 moves
- onDisableMove: 2 moves
- beforeMoveCallback: 2 moves
- condition::onDamage: 2 moves
- onModifyTarget: 2 moves
- condition::onDamagingHit: 2 moves
- condition::onModifyType: 2 moves
- onDamage: 2 moves
- condition::onNegateImmunity: 2 moves
- condition::onModifyBoost: 2 moves
- condition::onModifySpe: 2 moves
- condition::onSwap: 2 moves
- condition::onFoeBeforeMove: 2 moves
- condition::onSideRestart: 2 moves
- condition::onSourceBasePower: 1 moves
- condition::onAfterMove: 1 moves
- condition::onOverrideAction: 1 moves
- onModifyPriority: 1 moves
- condition::onModifyAccuracy: 1 moves
- condition::onTryHeal: 1 moves
- condition::onFoeDisableMove: 1 moves
- condition::onDragOut: 1 moves
- condition::onSourceInvulnerability: 1 moves
- condition::onSourceAccuracy: 1 moves
- condition::onAllyTryHitSide: 1 moves
- onUseMoveMessage: 1 moves
- condition::onTryBoost: 1 moves
- condition::onTryMove: 1 moves
- condition::onBeforeSwitchOut: 1 moves
- condition::onType: 1 moves
- condition::onAnyDragOut: 1 moves
- condition::onFoeTrapPokemon: 1 moves
- condition::onAnyInvulnerability: 1 moves
- condition::onAnyBasePower: 1 moves
- condition::onAnyPrepareHit: 1 moves
- condition::onTryPrimaryHit: 1 moves
- condition::onEffectiveness: 1 moves
- condition::onAnySetStatus: 1 moves

## Missing Infrastructure

### Critical Infrastructure Needed for Remaining 117 Callbacks

All 117 remaining callbacks require one or more of the following infrastructure components that don't currently exist or need modifications to existing code.

### 1. Active Move Access in Callbacks

**Problem:** Most callbacks need to check properties of the active move (isZ, isMax, target type, move type, flags)

**Current State:**
- `battle.active_move: Option<ActiveMove>` exists
- `ActiveMove` has fields: `is_z`, `is_max`, `move_type`, `target`, `flags: MoveFlags`
- BUT: These are not accessible in callback signatures or need helper methods

**Needed:**
- Helper methods to access active move properties from within callbacks
- OR: Pass active move reference/data to callbacks that need it

**Blocks:**
- healblock.rs: on_before_move, on_modify_move - needs move.flags['heal'], move.isZ, move.isMax
- throatchop.rs: on_before_move, on_modify_move - needs move.flags['sound'], move.isZOrMaxPowered
- taunt.rs: on_before_move - needs move.category, move.isZ, move.isZOrMaxPowered, move.id
- wideguard.rs: on_try, on_try_hit - needs move.target type, move.isZ, move.isMax, move.id
- terablast.rs: on_modify_type, on_modify_move - needs to modify move.type, move.category
- All MoveHandlerResult callbacks (need different infrastructure)

### 2. Mutable Move Modification

**Problem:** Many callbacks need to modify the active move's properties (type, category, base_power)

**Current State:**
- ActiveMove exists but callbacks can't modify it
- Callbacks return EventResult, not modified move data

**Needed:**
- System for callbacks to modify active move properties
- Possibly through battle state or return values

**Blocks:**
- terablast.rs: on_modify_type, on_modify_move - modify move.type, move.category, move.self
- technoblast.rs: on_modify_type - modify move.type
- terrainpulse.rs: on_modify_type, on_modify_move - modify move.type, move.basePower
- terastarstorm.rs: on_modify_type, on_modify_move - modify move.type, move.category, move.target
- All pledge moves (firepledge, waterpledge): modify move.type, move.basePower, move.forceSTAB

### 3. Pokemon Type and Ability Checking

**STATUS: ✅ METHODS EXIST**

**Current State:**
- ✅ `pub fn has_type(&self, type_name: &str) -> bool` - EXISTS (pokemon.rs:1521)
- ✅ `pub fn get_types(&self, exclude_added: bool) -> Vec<String>` - EXISTS (pokemon.rs:953)
- ✅ `pub fn has_ability(&self, ability_name: &[&str]) -> bool` - EXISTS
- ✅ `pub fn run_immunity(&self, move_type: &str) -> bool` - EXISTS (pokemon.rs:3251)
- ✅ `pub fn is_grounded(&self) -> bool` - EXISTS (pokemon.rs:985)
- ✅ `pub fn cure_status(&mut self) -> bool` - EXISTS (pokemon.rs:594)

**Note:** These methods exist but callbacks STILL can't be fully implemented because:
- Function signatures don't provide necessary parameters (e.g., `type` parameter for onEffectiveness)
- Side management infrastructure missing (pokemon.side.removeSideCondition, pokemon.side.foe)
- Effect state custom fields missing (effectState.layers)

### 4. Item Management

**STATUS: ✅ BASIC METHODS EXIST**

**Current State:**
- ✅ `pub fn get_item(&self) -> &ID` - EXISTS (pokemon.rs:1851)
- ✅ `pub fn take_item(&mut self) -> Option<ID>` - EXISTS (pokemon.rs:1835)
- ✅ `pub fn set_item(&mut self, item_id: ID) -> bool` - EXISTS (pokemon.rs:1803)
- ✅ `pub fn has_item(&self, items: &[&str]) -> bool` - EXISTS (pokemon.rs:1088)

**Still Missing:**
- ❌ Event system integration (singleEvent('TakeItem'), runEvent('AfterUseItem'))
- ❌ Berry eating system (eatItem())
- ❌ Item state tracking

**Blocks:**
- trick.rs, thief.rs, switcheroo.rs: need singleEvent('TakeItem') for validation
- fling.rs: needs runEvent('AfterUseItem')
- teatime.rs: needs eatItem() method

### 5. Event System

**Problem:** Callbacks need to fire and respond to events

**Current State:**
- No event system for callbacks to use

**Needed:**
```rust
impl Battle {
    pub fn run_event(&mut self, event_name: &str, ...) -> EventResult;
    pub fn single_event(&mut self, event_name: &str, ...) -> bool;
}
```

**Blocks:**
- trick.rs: needs singleEvent('TakeItem')
- thief.rs: needs singleEvent('TakeItem')
- fling.rs: needs runEvent('AfterUseItem')
- teatime.rs: needs runEvent('Invulnerability'), runEvent('TryHit')
- technoblast.rs: needs runEvent('Drive')

### 6. Queue and Action System

**Problem:** Callbacks need to check upcoming actions and queue state

**Current State:**
- No queue system exposed to callbacks

**Needed:**
```rust
impl Battle {
    pub fn queue_will_act(&self) -> bool;
    pub fn queue_will_move(&self, pokemon: &Pokemon) -> bool;
}
```

**Blocks:**
- wideguard.rs: on_try - needs queue.willAct()
- taunt.rs: on_start - needs queue.willMove(target)
- firepledge.rs, waterpledge.rs: need queue.willMove()

### 7. Status Management

**STATUS: ✅ METHOD EXISTS**

**Current State:**
- ✅ `pub fn try_set_status(&mut self, status_id: ID, _source_effect: Option<&str>) -> bool` - EXISTS (pokemon.rs:3037)
- ✅ `pub fn set_status(&mut self, status: String)` - EXISTS
- ✅ `pub fn cure_status(&mut self) -> bool` - EXISTS (pokemon.rs:594)

**Still Missing:**
- ❌ Source tracking (passing source pokemon, not just effect name)
- ❌ Full status immunity checking integrated with abilities

**Blocks:**
- toxicspikes.rs: Needs source pokemon parameter and side access (pokemon.side.foe.active[0])
- Side management infrastructure still missing

### 8. Healing System with Source Tracking

**Problem:** heal() method exists but doesn't track source or effect

**Current State:**
```rust
pub fn heal(&mut self, amount: i32) -> i32;  // Basic heal, no source/effect
```

**Needed:**
```rust
impl Battle {
    pub fn heal(&mut self, amount: i32, target: (usize, usize), source: Option<(usize, usize)>, effect: Option<&str>) -> Option<i32>;
}
```

**Blocks:**
- wish.rs: needs heal() with source tracking
- healblock.rs: on_try_heal - needs to check effect.id and source
- All G-Max moves that heal (gmaxfinale)

### 9. Effect State with Custom Fields

**Problem:** EffectState needs to store custom fields like source, hp, duration, turn counters

**Current State:**
- EffectState exists but is basic
- No source tracking, custom fields, or turn counters

**Needed:**
```rust
pub struct EffectState {
    pub id: ID,
    pub duration: Option<i32>,
    pub source: Option<(usize, usize)>,
    pub source_slot: Option<usize>,
    // Dynamic fields for specific effects
    pub custom_fields: HashMap<String, Value>,
}
```

**Blocks:**
- wish.rs: needs effectState.hp, effectState.startingTurn, effectState.source
- syrupbomb.rs: on_update, on_residual - needs effectState.source
- torment.rs: needs effectState.duration modification
- All duration_callback functions

### 10. Battle State Methods

**Problem:** Need various battle state access methods

**Needed:**
```rust
impl Battle {
    pub fn get_all_active(&self) -> Vec<(usize, usize)>;
    pub fn attr_last_move(&mut self, attr: &str);
    pub fn get_overflowed_turn_count(&self) -> i32;
    pub fn hint(&mut self, message: &str);
}
```

**Blocks:**
- teatime.rs: needs getAllActive()
- telekinesis.rs, healblock.rs: needs attrLastMove()
- wish.rs: needs getOverflowedTurnCount(), hint()

### 11. Move Slot Management

**Problem:** Need to check and disable specific moves by ID

**Current State:**
- ✅ Pokemon has move_slots field
- ✅ disable_move() method EXISTS
- ✅ Can iterate and check moves

**Status:** WORKING - Used in throatchop, taunt, healblock on_disable_move callbacks

### 12. Volatile Condition Methods (Partially Complete)

**Current State:**
- ✅ add_volatile(id) EXISTS
- ✅ remove_volatile(id) EXISTS
- ✅ has_volatile(id) EXISTS
- BUT: No source tracking, no linked volatiles

**Needed:**
```rust
pub fn add_volatile_with_source(&mut self, id: ID, source: Option<(usize, usize)>, effect: Option<&str>) -> bool;
```

**Blocks:**
- wideguard.rs: on_hit_side - needs addVolatile('stall')
- syrupbomb.rs: needs removeVolatile() with effect state check
- anchorshot.rs (MoveHandlerResult): needs addVolatile() with source

### 13. MoveHandlerResult Infrastructure

**Problem:** 72 callbacks use MoveHandlerResult instead of EventResult - completely different system

**Files Affected:**
- All G-Max moves (gmaxfinale, gmaxbefuddle, etc.)
- All Z-moves (genesissupernova)
- Various special moves (alluringvoice, anchorshot, burningjealousy, direclaw, eeriespell)

**Status:** Requires separate implementation approach - different signatures, different return types

### Summary by Priority

**High Priority (unlocks many callbacks):**
1. Active Move Access - needed by ~40 callbacks
2. Pokemon Type/Ability Checking - needed by ~30 callbacks
3. Mutable Move Modification - needed by ~25 callbacks

**Medium Priority:**
4. Event System - needed by ~20 callbacks
5. Item Management - needed by ~15 callbacks
6. Queue/Action System - needed by ~10 callbacks

**Lower Priority:**
7. Effect State Enhancements - needed by ~10 callbacks
8. Status/Healing with tracking - needed by ~8 callbacks
9. MoveHandlerResult system - needed by 72 callbacks but separate infrastructure

### trick.rs
Missing methods on Pokemon:
- `has_ability(ability_name: &str) -> bool` - Check if pokemon has a specific ability
- `take_item(source: Option<Pokemon>) -> Option<Item>` - Take/remove item from pokemon
- `set_item(item: Item)` - Set pokemon's item
- `item: Option<Item>` - Current item field
- `item_state` - Item state tracking

Missing methods on Battle:
- `single_event(event_name: &str, item: Item, item_state, target, source, move, item) -> bool` - Fire single event like TakeItem

### toxicspikes.rs
Missing methods on Pokemon:
- `is_grounded() -> bool` - Check if pokemon is grounded
- `has_type(type_name: &str) -> bool` - Check if pokemon has a specific type
- `has_item(item_name: &str) -> bool` - Check if pokemon has a specific item
- `try_set_status(status: &str, source: Pokemon) -> bool` - Try to set status condition

### substitute.rs
Missing fields on Pokemon:
- `hp: i32` - Current HP
- `maxhp: i32` - Maximum HP

Missing methods on Battle:
- `direct_damage(amount: i32)` - Deal direct damage

Missing fields on EffectState:
- `hp: i32` - HP tracking for substitute

Missing methods:
- `battle.actions.getDamage()` - Calculate damage
- `battle.attrLastMove()` - Set move attribute

Missing fields on Move:
- `flags: HashMap<String, bool>` - Move flags like bypasssub
- `infiltrates: bool` - Whether move infiltrates
- `ohko: bool` - One-hit KO flag
- `recoil: Option<[i32; 2]>` - Recoil damage ratio
- `drain: Option<[i32; 2]>` - Drain HP ratio

Missing fields on Pokemon:
- `last_damage: i32` - Last damage dealt

### thousandarrows.rs
Missing methods on Pokemon:
- `run_immunity(type_name: &str) -> bool` - Check type immunity
- `has_type(type_name: &str) -> bool` - Check if pokemon has a specific type

### terastarstorm.rs
Missing fields on Pokemon:
- `species_id: ID` - Species identifier
- `terastallized: bool` - Terastallization state

Missing methods on Pokemon:
- `get_stat(stat: &str, boost: bool, real: bool) -> i32` - Get stat value

Missing fields on Move:
- `move_type: String` (mutable) - Move type that can be modified
- `category: String` (mutable) - Move category that can be modified
- `target: String` (mutable) - Move targeting that can be modified

### teatime.rs
Missing methods on Battle:
- `run_event(event: &str, ...) -> EventResult` - Fire battle events
- `attr_last_move(attr: &str)` - Set move attribute

Missing methods on Pokemon:
- `get_item() -> Item` - Get pokemon's item
- `eat_item(force: bool)` - Make pokemon eat its berry

Missing fields on Item:
- `is_berry: bool` - Whether item is a berry

### trickroom.rs, wonderroom.rs
Missing methods on Pokemon:
- `has_ability(ability_name: &str) -> bool` - Check if pokemon has a specific ability

Missing methods on Battle:
- `field.remove_pseudo_weather(weather_name: &str)` - Remove pseudo weather

Missing fields on Move:
- `override_offensive_stat: Option<String>` (mutable) - Stat used for damage calculation

### wideguard.rs
Missing methods on Battle:
- `queue.will_act() -> bool` - Check if there are any pending actions

Missing methods on Pokemon:
- `add_volatile(volatile_name: &str)` - Add a volatile condition
- `get_volatile(volatile_name: &str) -> Option<Volatile>` - Get volatile condition
- `remove_volatile(volatile_name: &str)` - Remove volatile condition

Missing fields on Move:
- `target: MoveTargetType` - Move targeting (allAdjacent, allAdjacentFoes, etc.)
- `is_z: bool` - Whether move is Z-move
- `is_max: bool` - Whether move is Max move
- `id: ID` - Move identifier

Missing methods on Pokemon:
- `get_move_hit_data(move) -> MoveHitData` - Get move hit data with zBrokeProtect field
- Access to `volatiles` map with duration tracking

### uproar.rs
Missing methods on Battle:
- `side.active_team() -> Vec<Pokemon>` - Get all active pokemon on a side
- `cure_status()` on Pokemon - Cure status condition

Missing methods on Pokemon:
- `has_volatile(volatile_name: &str) -> bool` - Check for volatile condition
- `remove_volatile(volatile_name: &str)` - Remove volatile condition
- `last_move: Option<ID>` - Last move used by pokemon
- `status: Option<String>` - Current status condition

### syrupbomb.rs
Missing fields on EffectState:
- `source: Option<(usize, usize)>` - Source pokemon tracking

Missing methods on Pokemon:
- `is_active: bool` - Whether pokemon is active
- `remove_volatile(volatile_name: &str)` - Remove volatile condition

Missing methods on Battle:
- `boost(stats: HashMap<String, i32>, target, source)` - Modify stats with source tracking

### healblock.rs
Missing methods on Pokemon:
- `has_ability(ability_name: &str) -> bool` - Check if pokemon has a specific ability
- `move_slots: Vec<MoveSlot>` - Pokemon's move slots
- `disable_move(move_id: &str)` - Disable a move

Missing fields on Move:
- `flags: HashMap<String, bool>` - Move flags like 'heal'
- `is_z: bool` - Whether move is Z-move
- `is_max: bool` - Whether move is Max move
- `name: String` - Move name for effect checking

Missing methods on Battle:
- `attr_last_move(attr: &str)` - Set move attribute
- `heal(amount: i32, target, source, effect) -> Option<i32>` - Heal HP

Missing fields on EffectState:
- `duration: i32` (mutable) - Effect duration tracking

### telekinesis.rs
Missing methods on Battle:
- `field.get_pseudo_weather(weather_name: &str) -> Option<Weather>` - Check for pseudo weather
- `attr_last_move(attr: &str)` - Set move attribute

Missing methods on Pokemon:
- `has_volatile(volatile_name: &str) -> bool` - Check for volatile condition
- `remove_volatile(volatile_name: &str)` - Remove volatile condition
- `base_species.name: String` - Base species name
- `base_species.base_species: String` - Base species identifier

Missing fields for on_immunity:
- Type parameter in signature

### taunt.rs
Missing fields on Pokemon:
- `active_turns: i32` - Number of turns pokemon has been active
- `move_slots: Vec<MoveSlot>` - Pokemon's move slots
- `disable_move(move_id: &str)` - Disable a move

Missing methods on Battle:
- `queue.will_move(pokemon) -> bool` - Check if pokemon will move this turn
- `dex.moves.get(move_id) -> MoveDef` - Get move definition

Missing fields on EffectState:
- `duration: i32` (mutable) - Effect duration tracking

Missing fields on Move:
- `category: MoveCategory` - Move category (Status, Physical, Special)
- `id: ID` - Move identifier
- `is_z: bool` - Whether move is Z-move
- `is_z_or_max_powered: bool` - Whether Z/Max powered

Missing methods on Pokemon:
- Move slot iteration and checking

### terablast.rs
Missing methods on Battle:
- `attr_last_move(attr: &str)` - Set move attributes like animation

Missing methods on Pokemon:
- `get_stat(stat: &str, boost: bool, real: bool) -> i32` - Get stat value
- `tera_type: String` - Pokemon's tera type

Missing fields on Move (mutable):
- `move_type: String` - Move type
- `category: MoveCategory` - Move category
- `self_effect: Option<SelfEffect>` - Self-targeting effects with boosts

### firepledge.rs, waterpledge.rs, grasspledge.rs
Missing methods on Battle:
- `queue.will_move(pokemon) -> bool` - Check if pokemon will move
- `last_move: Option<ID>` - Get last move used in battle
- Access to team members and their move queue

Missing fields on Move:
- `base_power: i32` (mutable) - Base power modification

Missing side conditions:
- Complex multi-turn side condition effects
- Residual damage/speed modifications

### wish.rs
Missing fields on EffectState:
- `hp: i32` - Stored HP value
- `starting_turn: i32` - Turn when wish was used
- `source_slot: usize` - Source pokemon slot
- `source.name: String` - Source pokemon name

Missing methods on Battle:
- `get_overflowed_turn_count() -> i32` - Get turn counter with overflow
- `hint(message: &str)` - Display hint to player
- `heal(amount: i32, target, source) -> Option<i32>` - Heal pokemon
- `side.remove_slot_condition(slot, condition)` - Remove slot condition
- `get_at_slot(slot) -> Pokemon` - Get pokemon at slot

Missing fields on Pokemon:
- `fainted: bool` - Whether pokemon has fainted
- `get_health: String` - Health status string for battle log

### All G-Max and Z-moves (MoveHandlerResult files)
These use MoveHandlerResult instead of EventResult and need:
- `source.is_active: bool` - Check if source is active
- `add_volatile(volatile, source, move, extra)` - Add volatile with full tracking
- `try_set_status(status, source, move)` - Try to set status
- `stats_raised_this_turn: bool` - Track if stats were raised
- `allies_and_self()` - Iterator over allies and self
- `heal(amount, pokemon, source, move)` - Heal with source tracking
- Full move effect tracking infrastructure
