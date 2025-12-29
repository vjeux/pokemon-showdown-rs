# Items Implementation Tracking

Total: 583 items
Items with callbacks: 326

## Missing Infrastructure for Item Callbacks

### ✅ COMPLETED: Mega Stones (onTakeItem)
- ✅ Access to item data's `megaEvolves` property - IMPLEMENTED
- ✅ Access to pokemon's `baseSpecies.baseSpecies` property - IMPLEMENTED
- Items affected: abomasite, absolite, absolitez, aerodactylite, aggronite, alakazite, altarianite, ampharosite, audinite, banettite, barbaracite, baxcalibrite, beedrillite (and all other mega stones)
- JS code pattern: `if (item.megaEvolves === source.baseSpecies.baseSpecies) return false; return true;`
- **Status:** All 86 mega stone items have been implemented and are working correctly.

## Items with Callbacks (alphabetically)
- [x] abilityshield - Ability Shield (Gen 9) - 1 callback: onSetAbility
- [x] abomasite - Abomasite (Gen 6) - 1 callback: onTakeItem
- [x] absolite - Absolite (Gen 6) - 1 callback: onTakeItem
- [x] absolitez - Absolite Z (Gen 9) - 1 callback: onTakeItem
- [x] absorbbulb - Absorb Bulb (Gen 5) - 1 callback: onDamagingHit
- [x] adamantcrystal - Adamant Crystal (Gen 8) - 2 callbacks: onBasePower, onTakeItem
- [x] adamantorb - Adamant Orb (Gen 4) - 1 callback: onBasePower
- [x] adrenalineorb - Adrenaline Orb (Gen 7) - 1 callback: onAfterBoost
- [x] aerodactylite - Aerodactylite (Gen 6) - 1 callback: onTakeItem
- [x] aggronite - Aggronite (Gen 6) - 1 callback: onTakeItem
- [x] aguavberry - Aguav Berry (Gen 3) - 3 callbacks: onUpdate, onTryEatItem, onEat
- [x] airballoon - Air Balloon (Gen 5) - 3 callbacks: onStart, onDamagingHit, onAfterSubDamage
- [x] alakazite - Alakazite (Gen 6) - 1 callback: onTakeItem
- [x] altarianite - Altarianite (Gen 6) - 1 callback: onTakeItem
- [x] ampharosite - Ampharosite (Gen 6) - 1 callback: onTakeItem
- [x] apicotberry - Apicot Berry (Gen 3) - 2 callbacks: onUpdate, onEat
- [x] aspearberry - Aspear Berry (Gen 3) - 2 callbacks: onUpdate, onEat
- [x] assaultvest - Assault Vest (Gen 6) - 2 callbacks: onModifySpD, onDisableMove
- [x] audinite - Audinite (Gen 6) - 1 callback: onTakeItem
- [x] babiriberry - Babiri Berry (Gen 4) - 2 callbacks: onSourceModifyDamage, onEat
- [x] banettite - Banettite (Gen 6) - 1 callback: onTakeItem
- [x] barbaracite - Barbaracite (Gen 9) - 1 callback: onTakeItem
- [x] baxcalibrite - Baxcalibrite (Gen 9) - 1 callback: onTakeItem
- [x] beedrillite - Beedrillite (Gen 6) - 1 callback: onTakeItem
- [x] berry - Berry (Gen 2) - 3 callbacks: onResidual, onTryEatItem, onEat
- [x] berryjuice - Berry Juice (Gen 2) - 1 callback: onUpdate
- [x] berserkgene - Berserk Gene (Gen 2) - 1 callback: onUpdate
- [x] bigroot - Big Root (Gen 4) - 1 callback: onTryHeal
- [x] bitterberry - Bitter Berry (Gen 2) - 2 callbacks: onUpdate, onEat
- [x] blackbelt - Black Belt (Gen 2) - 1 callback: onBasePower
- [x] blackglasses - Black Glasses (Gen 2) - 1 callback: onBasePower
- [x] blacksludge - Black Sludge (Gen 4) - 1 callback: onResidual
- [x] blastoisinite - Blastoisinite (Gen 6) - 1 callback: onTakeItem
- [x] blazikenite - Blazikenite (Gen 6) - 1 callback: onTakeItem
- [ ] blueorb - Blue Orb (Gen 6) - 2 callbacks: onSwitchIn, onTakeItem
- [x] boosterenergy - Booster Energy (Gen 9) - 3 callbacks: onStart, onUpdate, onTakeItem
- [x] brightpowder - Bright Powder (Gen 2) - 1 callback: onModifyAccuracy
- [x] buggem - Bug Gem (Gen 5) - 1 callback: onSourceTryPrimaryHit
- [x] bugmemory - Bug Memory (Gen 7) - 1 callback: onTakeItem
- [x] burndrive - Burn Drive (Gen 5) - 1 callback: onTakeItem
- [x] burntberry - Burnt Berry (Gen 2) - 2 callbacks: onUpdate, onEat
- [x] cameruptite - Cameruptite (Gen 6) - 1 callback: onTakeItem
- [x] cellbattery - Cell Battery (Gen 5) - 1 callback: onDamagingHit
- [x] chandelurite - Chandelurite (Gen 9) - 1 callback: onTakeItem
- [x] charcoal - Charcoal (Gen 2) - 1 callback: onBasePower
- [x] charizarditex - Charizardite X (Gen 6) - 1 callback: onTakeItem
- [x] charizarditey - Charizardite Y (Gen 6) - 1 callback: onTakeItem
- [x] chartiberry - Charti Berry (Gen 4) - 2 callbacks: onSourceModifyDamage, onEat
- [x] cheriberry - Cheri Berry (Gen 3) - 2 callbacks: onUpdate, onEat
- [x] chesnaughtite - Chesnaughtite (Gen 9) - 1 callback: onTakeItem
- [x] chestoberry - Chesto Berry (Gen 3) - 2 callbacks: onUpdate, onEat
- [x] chilanberry - Chilan Berry (Gen 4) - 2 callbacks: onSourceModifyDamage, onEat
- [x] chilldrive - Chill Drive (Gen 5) - 1 callback: onTakeItem
- [x] chimechite - Chimechite (Gen 9) - 1 callback: onTakeItem
- [x] choiceband - Choice Band (Gen 3) - 3 callbacks: onStart, onModifyMove, onModifyAtk
- [x] choicescarf - Choice Scarf (Gen 4) - 3 callbacks: onStart, onModifyMove, onModifySpe
- [x] choicespecs - Choice Specs (Gen 4) - 3 callbacks: onStart, onModifyMove, onModifySpA
- [x] chopleberry - Chople Berry (Gen 4) - 2 callbacks: onSourceModifyDamage, onEat
- [x] clearamulet - Clear Amulet (Gen 9) - 1 callback: onTryBoost
- [x] clefablite - Clefablite (Gen 9) - 1 callback: onTakeItem
- [x] cobaberry - Coba Berry (Gen 4) - 2 callbacks: onSourceModifyDamage, onEat
- [x] colburberry - Colbur Berry (Gen 4) - 2 callbacks: onSourceModifyDamage, onEat
- [x] cornerstonemask - Cornerstone Mask (Gen 9) - 2 callbacks: onBasePower, onTakeItem
- [x] covertcloak - Covert Cloak (Gen 9) - 1 callback: onModifySecondaries
- [x] crabominite - Crabominite (Gen 9) - 1 callback: onTakeItem
- [x] crucibellite - Crucibellite (Gen 6) - 1 callback: onTakeItem
- [x] custapberry - Custap Berry (Gen 4) - 2 callbacks: onFractionalPriority, onEat
- [x] darkgem - Dark Gem (Gen 5) - 1 callback: onSourceTryPrimaryHit
- [x] darkmemory - Dark Memory (Gen 7) - 1 callback: onTakeItem
- [x] darkranite - Darkranite (Gen 9) - 1 callback: onTakeItem
- [x] deepseascale - Deep Sea Scale (Gen 3) - 1 callback: onModifySpD
- [x] deepseatooth - Deep Sea Tooth (Gen 3) - 1 callback: onModifySpA
- [x] delphoxite - Delphoxite (Gen 9) - 1 callback: onTakeItem
- [x] destinyknot - Destiny Knot (Gen 4) - 1 callback: onAttract
- [x] diancite - Diancite (Gen 6) - 1 callback: onTakeItem
- [x] dousedrive - Douse Drive (Gen 5) - 1 callback: onTakeItem
- [x] dracoplate - Draco Plate (Gen 4) - 2 callbacks: onBasePower, onTakeItem
- [x] dragalgite - Dragalgite (Gen 9) - 1 callback: onTakeItem
- [x] dragonfang - Dragon Fang (Gen 2) - 1 callback: onBasePower
- [x] dragongem - Dragon Gem (Gen 5) - 1 callback: onSourceTryPrimaryHit
- [x] dragoninite - Dragoninite (Gen 9) - 1 callback: onTakeItem
- [x] dragonmemory - Dragon Memory (Gen 7) - 1 callback: onTakeItem
- [x] drampanite - Drampanite (Gen 9) - 1 callback: onTakeItem
- [x] dreadplate - Dread Plate (Gen 4) - 2 callbacks: onBasePower, onTakeItem
- [x] earthplate - Earth Plate (Gen 4) - 2 callbacks: onBasePower, onTakeItem
- [x] eelektrossite - Eelektrossite (Gen 9) - 1 callback: onTakeItem
- [x] ejectbutton - Eject Button (Gen 5) - 1 callback: onAfterMoveSecondary
- [x] ejectpack - Eject Pack (Gen 8) - 8 callbacks: onAfterBoost, onAnySwitchIn, onAnyAfterMega, onAnyAfterMove, onResidual, onUseItem, onUse, onEnd
- [x] electricgem - Electric Gem (Gen 5) - 1 callback: onSourceTryPrimaryHit
- [x] electricmemory - Electric Memory (Gen 7) - 1 callback: onTakeItem
- [x] electricseed - Electric Seed (Gen 7) - 2 callbacks: onStart, onTerrainChange
- [x] emboarite - Emboarite (Gen 9) - 1 callback: onTakeItem
- [x] enigmaberry - Enigma Berry (Gen 3) - 3 callbacks: onHit, onTryEatItem, onEat
- [x] eviolite - Eviolite (Gen 5) - 2 callbacks: onModifyDef, onModifySpD
- [x] excadrite - Excadrite (Gen 9) - 1 callback: onTakeItem
- [x] expertbelt - Expert Belt (Gen 4) - 1 callback: onModifyDamage
- [x] fairyfeather - Fairy Feather (Gen 9) - 1 callback: onBasePower
- [x] fairygem - Fairy Gem (Gen 6) - 1 callback: onSourceTryPrimaryHit
- [x] fairymemory - Fairy Memory (Gen 7) - 1 callback: onTakeItem
- [x] falinksite - Falinksite (Gen 9) - 1 callback: onTakeItem
- [x] feraligite - Feraligite (Gen 9) - 1 callback: onTakeItem
- [x] fightinggem - Fighting Gem (Gen 5) - 1 callback: onSourceTryPrimaryHit
- [x] fightingmemory - Fighting Memory (Gen 7) - 1 callback: onTakeItem
- [x] figyberry - Figy Berry (Gen 3) - 3 callbacks: onUpdate, onTryEatItem, onEat
- [x] firegem - Fire Gem (Gen 5) - 1 callback: onSourceTryPrimaryHit
- [x] firememory - Fire Memory (Gen 7) - 1 callback: onTakeItem
- [x] fistplate - Fist Plate (Gen 4) - 2 callbacks: onBasePower, onTakeItem
- [x] flameorb - Flame Orb (Gen 4) - 1 callback: onResidual
- [x] flameplate - Flame Plate (Gen 4) - 2 callbacks: onBasePower, onTakeItem
- [x] floatstone - Float Stone (Gen 5) - 1 callback: onModifyWeight
- [x] floettite - Floettite (Gen 9) - 1 callback: onTakeItem
- [x] flyinggem - Flying Gem (Gen 5) - 1 callback: onSourceTryPrimaryHit
- [x] flyingmemory - Flying Memory (Gen 7) - 1 callback: onTakeItem
- [x] focusband - Focus Band (Gen 2) - 1 callback: onDamage
- [x] focussash - Focus Sash (Gen 4) - 1 callback: onDamage
- [x] froslassite - Froslassite (Gen 9) - 1 callback: onTakeItem
- [x] galladite - Galladite (Gen 6) - 1 callback: onTakeItem
- [x] ganlonberry - Ganlon Berry (Gen 3) - 2 callbacks: onUpdate, onEat
- [x] garchompite - Garchompite (Gen 6) - 1 callback: onTakeItem
- [x] garchompitez - Garchompite Z (Gen 9) - 1 callback: onTakeItem
- [x] gardevoirite - Gardevoirite (Gen 6) - 1 callback: onTakeItem
- [x] gengarite - Gengarite (Gen 6) - 1 callback: onTakeItem
- [x] ghostgem - Ghost Gem (Gen 5) - 1 callback: onSourceTryPrimaryHit
- [x] ghostmemory - Ghost Memory (Gen 7) - 1 callback: onTakeItem
- [x] glalitite - Glalitite (Gen 6) - 1 callback: onTakeItem
- [x] glimmoranite - Glimmoranite (Gen 9) - 1 callback: onTakeItem
- [x] goldberry - Gold Berry (Gen 2) - 3 callbacks: onResidual, onTryEatItem, onEat
- [x] golisopite - Golisopite (Gen 9) - 1 callback: onTakeItem
- [x] golurkite - Golurkite (Gen 9) - 1 callback: onTakeItem
- [x] grassgem - Grass Gem (Gen 5) - 1 callback: onSourceTryPrimaryHit
- [x] grassmemory - Grass Memory (Gen 7) - 1 callback: onTakeItem
- [x] grassyseed - Grassy Seed (Gen 7) - 2 callbacks: onStart, onTerrainChange
- [x] greninjite - Greninjite (Gen 9) - 1 callback: onTakeItem
- [x] griseouscore - Griseous Core (Gen 8) - 2 callbacks: onBasePower, onTakeItem
- [x] griseousorb - Griseous Orb (Gen 4) - 1 callback: onBasePower
- [x] groundgem - Ground Gem (Gen 5) - 1 callback: onSourceTryPrimaryHit
- [x] groundmemory - Ground Memory (Gen 7) - 1 callback: onTakeItem
- [x] gyaradosite - Gyaradosite (Gen 6) - 1 callback: onTakeItem
- [x] habanberry - Haban Berry (Gen 4) - 2 callbacks: onSourceModifyDamage, onEat
- [x] hardstone - Hard Stone (Gen 2) - 1 callback: onBasePower
- [x] hawluchanite - Hawluchanite (Gen 9) - 1 callback: onTakeItem
- [x] hearthflamemask - Hearthflame Mask (Gen 9) - 2 callbacks: onBasePower, onTakeItem
- [x] heatranite - Heatranite (Gen 9) - 1 callback: onTakeItem
- [x] heracronite - Heracronite (Gen 6) - 1 callback: onTakeItem
- [x] houndoominite - Houndoominite (Gen 6) - 1 callback: onTakeItem
- [x] iapapaberry - Iapapa Berry (Gen 3) - 3 callbacks: onUpdate, onTryEatItem, onEat
- [x] iceberry - Ice Berry (Gen 2) - 2 callbacks: onUpdate, onEat
- [x] icegem - Ice Gem (Gen 5) - 1 callback: onSourceTryPrimaryHit
- [x] icememory - Ice Memory (Gen 7) - 1 callback: onTakeItem
- [x] icicleplate - Icicle Plate (Gen 4) - 2 callbacks: onBasePower, onTakeItem
- [x] insectplate - Insect Plate (Gen 4) - 2 callbacks: onBasePower, onTakeItem
- [x] ironball - Iron Ball (Gen 4) - 2 callbacks: onEffectiveness, onModifySpe
- [x] ironplate - Iron Plate (Gen 4) - 2 callbacks: onBasePower, onTakeItem
- [x] jabocaberry - Jaboca Berry (Gen 4) - 2 callbacks: onDamagingHit, onEat
- [x] kangaskhanite - Kangaskhanite (Gen 6) - 1 callback: onTakeItem
- [x] kasibberry - Kasib Berry (Gen 4) - 2 callbacks: onSourceModifyDamage, onEat
- [x] kebiaberry - Kebia Berry (Gen 4) - 2 callbacks: onSourceModifyDamage, onEat
- [x] keeberry - Kee Berry (Gen 6) - 2 callbacks: onAfterMoveSecondary, onEat
- [x] kingsrock - King's Rock (Gen 2) - 1 callback: onModifyMove
- [x] lansatberry - Lansat Berry (Gen 3) - 2 callbacks: onUpdate, onEat
- [x] latiasite - Latiasite (Gen 6) - 1 callback: onTakeItem
- [x] latiosite - Latiosite (Gen 6) - 1 callback: onTakeItem
- [x] laxincense - Lax Incense (Gen 3) - 1 callback: onModifyAccuracy
- [x] leek - Leek (Gen 8) - 1 callback: onModifyCritRatio
- [x] leftovers - Leftovers (Gen 2) - 1 callback: onResidual
- [x] leppaberry - Leppa Berry (Gen 3) - 2 callbacks: onUpdate, onEat
- [x] liechiberry - Liechi Berry (Gen 3) - 2 callbacks: onUpdate, onEat
- [x] lifeorb - Life Orb (Gen 4) - 2 callbacks: onModifyDamage, onAfterMoveSecondarySelf
- [x] lightball - Light Ball (Gen 2) - 2 callbacks: onModifyAtk, onModifySpA
- [x] loadeddice - Loaded Dice (Gen 9) - 1 callback: onModifyMove
- [x] lopunnite - Lopunnite (Gen 6) - 1 callback: onTakeItem
- [x] lucarionite - Lucarionite (Gen 6) - 1 callback: onTakeItem
- [x] lucarionitez - Lucarionite Z (Gen 9) - 1 callback: onTakeItem
- [x] luckypunch - Lucky Punch (Gen 2) - 1 callback: onModifyCritRatio
- [x] lumberry - Lum Berry (Gen 3) - 3 callbacks: onAfterSetStatus, onUpdate, onEat
- [x] luminousmoss - Luminous Moss (Gen 6) - 1 callback: onDamagingHit
- [x] lustrousglobe - Lustrous Globe (Gen 8) - 2 callbacks: onBasePower, onTakeItem
- [x] lustrousorb - Lustrous Orb (Gen 4) - 1 callback: onBasePower
- [x] machobrace - Macho Brace (Gen 3) - 1 callback: onModifySpe
- [x] magearnite - Magearnite (Gen 9) - 1 callback: onTakeItem
- [x] magnet - Magnet (Gen 2) - 1 callback: onBasePower
- [x] magoberry - Mago Berry (Gen 3) - 3 callbacks: onUpdate, onTryEatItem, onEat
- [x] mail - Mail (Gen 2) - 1 callback: onTakeItem
- [x] malamarite - Malamarite (Gen 9) - 1 callback: onTakeItem
- [x] manectite - Manectite (Gen 6) - 1 callback: onTakeItem
- [x] marangaberry - Maranga Berry (Gen 6) - 2 callbacks: onAfterMoveSecondary, onEat
- [x] mawilite - Mawilite (Gen 6) - 1 callback: onTakeItem
- [x] meadowplate - Meadow Plate (Gen 4) - 2 callbacks: onBasePower, onTakeItem
- [x] medichamite - Medichamite (Gen 6) - 1 callback: onTakeItem
- [x] meganiumite - Meganiumite (Gen 9) - 1 callback: onTakeItem
- [x] mentalherb - Mental Herb (Gen 3) - 1 callback: onUpdate
- [x] meowsticite - Meowsticite (Gen 9) - 1 callback: onTakeItem
- [x] metagrossite - Metagrossite (Gen 6) - 1 callback: onTakeItem
- [x] metalcoat - Metal Coat (Gen 2) - 1 callback: onBasePower
- [x] metalpowder - Metal Powder (Gen 2) - 1 callback: onModifyDef
- [x] metronome - Metronome (Gen 4) - 1 callback: onStart
- [x] mewtwonitex - Mewtwonite X (Gen 6) - 1 callback: onTakeItem
- [x] mewtwonitey - Mewtwonite Y (Gen 6) - 1 callback: onTakeItem
- [x] micleberry - Micle Berry (Gen 4) - 2 callbacks: onResidual, onEat
- [x] mindplate - Mind Plate (Gen 4) - 2 callbacks: onBasePower, onTakeItem
- [x] mintberry - Mint Berry (Gen 2) - 2 callbacks: onUpdate, onEat
- [x] miracleberry - Miracle Berry (Gen 2) - 2 callbacks: onUpdate, onEat
- [x] miracleseed - Miracle Seed (Gen 2) - 1 callback: onBasePower
- [x] mirrorherb - Mirror Herb (Gen 9) - 8 callbacks: onFoeAfterBoost, onAnySwitchIn, onAnyAfterMega, onAnyAfterTerastallization, onAnyAfterMove, onResidual, onUse, onEnd
- [x] mistyseed - Misty Seed (Gen 7) - 2 callbacks: onStart, onTerrainChange
- [x] muscleband - Muscle Band (Gen 4) - 1 callback: onBasePower
- [x] mysteryberry - Mystery Berry (Gen 2) - 2 callbacks: onUpdate, onEat
- [x] mysticwater - Mystic Water (Gen 2) - 1 callback: onBasePower
- [x] nevermeltice - Never-Melt Ice (Gen 2) - 1 callback: onBasePower
- [x] normalgem - Normal Gem (Gen 5) - 1 callback: onSourceTryPrimaryHit
- [x] occaberry - Occa Berry (Gen 4) - 2 callbacks: onSourceModifyDamage, onEat
- [x] oddincense - Odd Incense (Gen 4) - 1 callback: onBasePower
- [x] oranberry - Oran Berry (Gen 3) - 3 callbacks: onUpdate, onTryEatItem, onEat
- [x] passhoberry - Passho Berry (Gen 4) - 2 callbacks: onSourceModifyDamage, onEat
- [x] payapaberry - Payapa Berry (Gen 4) - 2 callbacks: onSourceModifyDamage, onEat
- [x] pechaberry - Pecha Berry (Gen 3) - 2 callbacks: onUpdate, onEat
- [x] persimberry - Persim Berry (Gen 3) - 2 callbacks: onUpdate, onEat
- [x] petayaberry - Petaya Berry (Gen 3) - 2 callbacks: onUpdate, onEat
- [x] pidgeotite - Pidgeotite (Gen 6) - 1 callback: onTakeItem
- [x] pinkbow - Pink Bow (Gen 2) - 1 callback: onBasePower
- [x] pinsirite - Pinsirite (Gen 6) - 1 callback: onTakeItem
- [x] pixieplate - Pixie Plate (Gen 6) - 2 callbacks: onBasePower, onTakeItem
- [x] poisonbarb - Poison Barb (Gen 2) - 1 callback: onBasePower
- [x] poisongem - Poison Gem (Gen 5) - 1 callback: onSourceTryPrimaryHit
- [x] poisonmemory - Poison Memory (Gen 7) - 1 callback: onTakeItem
- [x] polkadotbow - Polkadot Bow (Gen 2) - 1 callback: onBasePower
- [x] poweranklet - Power Anklet (Gen 4) - 1 callback: onModifySpe
- [x] powerband - Power Band (Gen 4) - 1 callback: onModifySpe
- [x] powerbelt - Power Belt (Gen 4) - 1 callback: onModifySpe
- [x] powerbracer - Power Bracer (Gen 4) - 1 callback: onModifySpe
- [x] powerherb - Power Herb (Gen 4) - 1 callback: onChargeMove
- [x] powerlens - Power Lens (Gen 4) - 1 callback: onModifySpe
- [x] powerweight - Power Weight (Gen 4) - 1 callback: onModifySpe
- [x] przcureberry - PRZ Cure Berry (Gen 2) - 2 callbacks: onUpdate, onEat
- [x] psncureberry - PSN Cure Berry (Gen 2) - 2 callbacks: onUpdate, onEat
- [x] psychicgem - Psychic Gem (Gen 5) - 1 callback: onSourceTryPrimaryHit
- [x] psychicmemory - Psychic Memory (Gen 7) - 1 callback: onTakeItem
- [x] psychicseed - Psychic Seed (Gen 7) - 2 callbacks: onStart, onTerrainChange
- [x] punchingglove - Punching Glove (Gen 9) - 2 callbacks: onBasePower, onModifyMove
- [x] pyroarite - Pyroarite (Gen 9) - 1 callback: onTakeItem
- [x] quickclaw - Quick Claw (Gen 2) - 1 callback: onFractionalPriority
- [x] quickpowder - Quick Powder (Gen 4) - 1 callback: onModifySpe
- [x] raichunitex - Raichunite X (Gen 9) - 1 callback: onTakeItem
- [x] raichunitey - Raichunite Y (Gen 9) - 1 callback: onTakeItem
- [x] rawstberry - Rawst Berry (Gen 3) - 2 callbacks: onUpdate, onEat
- [x] razorclaw - Razor Claw (Gen 4) - 1 callback: onModifyCritRatio
- [x] razorfang - Razor Fang (Gen 4) - 1 callback: onModifyMove
- [x] redcard - Red Card (Gen 5) - 1 callback: onAfterMoveSecondary
- [ ] redorb - Red Orb (Gen 6) - 2 callbacks: onSwitchIn, onTakeItem
- [x] rindoberry - Rindo Berry (Gen 4) - 2 callbacks: onSourceModifyDamage, onEat
- [x] rockgem - Rock Gem (Gen 5) - 1 callback: onSourceTryPrimaryHit
- [x] rockincense - Rock Incense (Gen 4) - 1 callback: onBasePower
- [x] rockmemory - Rock Memory (Gen 7) - 1 callback: onTakeItem
- [x] rockyhelmet - Rocky Helmet (Gen 5) - 1 callback: onDamagingHit
- [x] roomservice - Room Service (Gen 8) - 2 callbacks: onStart, onAnyPseudoWeatherChange
- [x] roseincense - Rose Incense (Gen 4) - 1 callback: onBasePower
- [x] roseliberry - Roseli Berry (Gen 6) - 2 callbacks: onSourceModifyDamage, onEat
- [x] rowapberry - Rowap Berry (Gen 4) - 2 callbacks: onDamagingHit, onEat
- [x] rustedshield - Rusted Shield (Gen 8) - 1 callback: onTakeItem
- [x] rustedsword - Rusted Sword (Gen 8) - 1 callback: onTakeItem
- [x] sablenite - Sablenite (Gen 6) - 1 callback: onTakeItem
- [x] safetygoggles - Safety Goggles (Gen 6) - 2 callbacks: onImmunity, onTryHit
- [x] salacberry - Salac Berry (Gen 3) - 2 callbacks: onUpdate, onEat
- [x] salamencite - Salamencite (Gen 6) - 1 callback: onTakeItem
- [x] sceptilite - Sceptilite (Gen 6) - 1 callback: onTakeItem
- [x] scizorite - Scizorite (Gen 6) - 1 callback: onTakeItem
- [x] scolipite - Scolipite (Gen 9) - 1 callback: onTakeItem
- [x] scopelens - Scope Lens (Gen 2) - 1 callback: onModifyCritRatio
- [x] scovillainite - Scovillainite (Gen 9) - 1 callback: onTakeItem
- [x] scraftinite - Scraftinite (Gen 9) - 1 callback: onTakeItem
- [x] seaincense - Sea Incense (Gen 3) - 1 callback: onBasePower
- [x] sharpbeak - Sharp Beak (Gen 2) - 1 callback: onBasePower
- [x] sharpedonite - Sharpedonite (Gen 6) - 1 callback: onTakeItem
- [x] shedshell - Shed Shell (Gen 4) - 2 callbacks: onTrapPokemon, onMaybeTrapPokemon
- [x] shellbell - Shell Bell (Gen 3) - 1 callback: onAfterMoveSecondarySelf
- [x] shockdrive - Shock Drive (Gen 5) - 1 callback: onTakeItem
- [x] shucaberry - Shuca Berry (Gen 4) - 2 callbacks: onSourceModifyDamage, onEat
- [x] silkscarf - Silk Scarf (Gen 3) - 1 callback: onBasePower
- [x] silverpowder - Silver Powder (Gen 2) - 1 callback: onBasePower
- [x] sitrusberry - Sitrus Berry (Gen 3) - 3 callbacks: onUpdate, onTryEatItem, onEat
- [x] skarmorite - Skarmorite (Gen 9) - 1 callback: onTakeItem
- [x] skyplate - Sky Plate (Gen 4) - 2 callbacks: onBasePower, onTakeItem
- [x] slowbronite - Slowbronite (Gen 6) - 1 callback: onTakeItem
- [x] snowball - Snowball (Gen 6) - 1 callback: onDamagingHit
- [x] softsand - Soft Sand (Gen 2) - 1 callback: onBasePower
- [x] souldew - Soul Dew (Gen 3) - 1 callback: onBasePower
- [x] spelltag - Spell Tag (Gen 2) - 1 callback: onBasePower
- [x] splashplate - Splash Plate (Gen 4) - 2 callbacks: onBasePower, onTakeItem
- [x] spookyplate - Spooky Plate (Gen 4) - 2 callbacks: onBasePower, onTakeItem
- [x] staraptite - Staraptite (Gen 9) - 1 callback: onTakeItem
- [x] starfberry - Starf Berry (Gen 3) - 2 callbacks: onUpdate, onEat
- [x] starminite - Starminite (Gen 9) - 1 callback: onTakeItem
- [x] steelgem - Steel Gem (Gen 5) - 1 callback: onSourceTryPrimaryHit
- [x] steelixite - Steelixite (Gen 6) - 1 callback: onTakeItem
- [x] steelmemory - Steel Memory (Gen 7) - 1 callback: onTakeItem
- [x] stick - Stick (Gen 2) - 1 callback: onModifyCritRatio
- [x] stickybarb - Sticky Barb (Gen 4) - 2 callbacks: onResidual, onHit
- [x] stoneplate - Stone Plate (Gen 4) - 2 callbacks: onBasePower, onTakeItem
- [x] swampertite - Swampertite (Gen 6) - 1 callback: onTakeItem
- [x] tangaberry - Tanga Berry (Gen 4) - 2 callbacks: onSourceModifyDamage, onEat
- [x] tatsugirinite - Tatsugirinite (Gen 9) - 1 callback: onTakeItem
- [x] thickclub - Thick Club (Gen 2) - 1 callback: onModifyAtk
- [x] throatspray - Throat Spray (Gen 8) - 1 callback: onAfterMoveSecondarySelf
- [x] toxicorb - Toxic Orb (Gen 4) - 1 callback: onResidual
- [x] toxicplate - Toxic Plate (Gen 4) - 2 callbacks: onBasePower, onTakeItem
- [x] twistedspoon - Twisted Spoon (Gen 2) - 1 callback: onBasePower
- [x] tyranitarite - Tyranitarite (Gen 6) - 1 callback: onTakeItem
- [x] utilityumbrella - Utility Umbrella (Gen 8) - 3 callbacks: onStart, onUpdate, onEnd
- [x] venusaurite - Venusaurite (Gen 6) - 1 callback: onTakeItem
- [x] victreebelite - Victreebelite (Gen 9) - 1 callback: onTakeItem
- [x] vilevial - Vile Vial (Gen 8) - 2 callbacks: onBasePower, onTakeItem
- [x] wacanberry - Wacan Berry (Gen 4) - 2 callbacks: onSourceModifyDamage, onEat
- [x] watergem - Water Gem (Gen 5) - 1 callback: onSourceTryPrimaryHit
- [x] watermemory - Water Memory (Gen 7) - 1 callback: onTakeItem
- [x] waveincense - Wave Incense (Gen 4) - 1 callback: onBasePower
- [x] weaknesspolicy - Weakness Policy (Gen 6) - 1 callback: onDamagingHit
- [x] wellspringmask - Wellspring Mask (Gen 9) - 2 callbacks: onBasePower, onTakeItem
- [x] whiteherb - White Herb (Gen 3) - 6 callbacks: onStart, onAnySwitchIn, onAnyAfterMega, onAnyAfterMove, onResidual, onUse
- [x] widelens - Wide Lens (Gen 4) - 1 callback: onSourceModifyAccuracy
- [x] wikiberry - Wiki Berry (Gen 3) - 3 callbacks: onUpdate, onTryEatItem, onEat
- [x] wiseglasses - Wise Glasses (Gen 4) - 1 callback: onBasePower
- [x] yacheberry - Yache Berry (Gen 4) - 2 callbacks: onSourceModifyDamage, onEat
- [x] zapplate - Zap Plate (Gen 4) - 2 callbacks: onBasePower, onTakeItem
- [x] zeraorite - Zeraorite (Gen 9) - 1 callback: onTakeItem
- [x] zoomlens - Zoom Lens (Gen 4) - 1 callback: onSourceModifyAccuracy
- [x] zygardite - Zygardite (Gen 9) - 1 callback: onTakeItem

## Statistics

By callback type:
- onTakeItem: 144 items
- onEat: 57 items
- onBasePower: 56 items
- onUpdate: 35 items
- onSourceModifyDamage: 18 items
- onSourceTryPrimaryHit: 18 items
- onStart: 13 items
- onResidual: 11 items
- onTryEatItem: 10 items
- onModifySpe: 10 items
- onDamagingHit: 9 items
- onModifyMove: 7 items
- onModifyCritRatio: 5 items
- onAfterMoveSecondary: 4 items
- onTerrainChange: 4 items
- onModifySpD: 3 items
- onModifyAtk: 3 items
- onModifySpA: 3 items
- onAnySwitchIn: 3 items
- onAnyAfterMega: 3 items
- onAnyAfterMove: 3 items
- onUse: 3 items
- onEnd: 3 items
- onAfterMoveSecondarySelf: 3 items
- onAfterBoost: 2 items
- onSwitchIn: 2 items
- onModifyAccuracy: 2 items
- onFractionalPriority: 2 items
- onHit: 2 items
- onModifyDef: 2 items
- onModifyDamage: 2 items
- onDamage: 2 items
- onSourceModifyAccuracy: 2 items
- onSetAbility: 1 items
- onAfterSubDamage: 1 items
- onDisableMove: 1 items
- onTryHeal: 1 items
- onTryBoost: 1 items
- onModifySecondaries: 1 items
- onAttract: 1 items
- onUseItem: 1 items
- onModifyWeight: 1 items
- onEffectiveness: 1 items
- onAfterSetStatus: 1 items
- onFoeAfterBoost: 1 items
- onAnyAfterTerastallization: 1 items
- onChargeMove: 1 items
- onAnyPseudoWeatherChange: 1 items
- onImmunity: 1 items
- onTryHit: 1 items
- onTrapPokemon: 1 items
- onMaybeTrapPokemon: 1 items

## Items by Generation
- Gen 2: 42 items
- Gen 3: 34 items
- Gen 4: 72 items
- Gen 5: 29 items
- Gen 6: 60 items
- Gen 7: 22 items
- Gen 8: 11 items
- Gen 9: 56 items

<<<<<<< HEAD
## Missing Infrastructure for J-L Items

### Mega Evolution Stones (kangaskhanite, latiasite, latiosite, lopunnite, lucarionite, lucarionitez)
- Missing Fields in ItemData: mega_evolves, mega_stone, item_user
- onTakeItem needs to check if item.megaEvolves === source.baseSpecies.baseSpecies
- Requires access to Pokemon base species and should return EventResult::False to prevent taking
## Missing Infrastructure for T-V Items

### Type-Modifier Berries (tangaberry, etc)
- Missing Methods: Pokemon.getMoveHitData(move).typeMod
- Need to check type effectiveness > 0
- Need to check volatiles['substitute']
- Need to check move.flags['bypasssub']
- Need to check move.infiltrates
- Need to access battle.gen
- Need Pokemon.eatItem() method
- Need battle.debug() method
- Need battle.add() method for event messages
- Pattern: All resist berries use same logic, just different types

### Mega Evolution Stones (tatsugirinite, tyranitarite, venusaurite, victreebelite)
- Missing Fields in ItemData: mega_evolves field
- onTakeItem needs to check if item.megaEvolves === source.baseSpecies.baseSpecies
- Some use megaEvolves.includes() for multiple forms (tatsugirinite)
- Requires access to item data at runtime

### Status/Effect Items (throatspray)
- Missing: Move flags infrastructure (move.flags['sound'])
- Missing: Pokemon.useItem() method (different from eatItem)
- Need to trigger item effect when sound move is used

### Status Orbs (toxicorb)
- Missing: Pokemon.trySetStatus(status, source) method
- Need to apply status at residual phase
- Status IDs: 'tox' for toxic poison

### Weather Items (utilityumbrella)
- Missing: Pokemon.ignoringItem() method
- Missing: field.effectiveWeather() method
- Missing: battle.runEvent() for WeatherChange
- Missing: effectState.inactive tracking
- Need to track weather changes and update effects

## Completed T-V Items
- [x] stoneplate - Stone Plate (Gen 4) - 2/2 callbacks implemented
- [x] thickclub - Thick Club (Gen 2) - 1/1 callback implemented
- [x] toxicplate - Toxic Plate (Gen 4) - 2/2 callbacks implemented
- [x] twistedspoon - Twisted Spoon (Gen 2) - 1/1 callback implemented
- [x] vilevial - Vile Vial (Gen 8) - 2/2 callbacks implemented

## Completed T-V Items
- [x] tangaberry - Tanga Berry (Gen 4) - COMPLETED
- [x] tatsugirinite - Tatsugirinite (Gen 9) - COMPLETED
- [x] throatspray - Throat Spray (Gen 8) - COMPLETED
- [x] toxicorb - Toxic Orb (Gen 4) - COMPLETED
- [x] tyranitarite - Tyranitarite (Gen 6) - COMPLETED
- [x] venusaurite - Venusaurite (Gen 6) - COMPLETED
- [x] victreebelite - Victreebelite (Gen 9) - COMPLETED


### Type-resist Berries (kasibberry, kebiaberry, etc.) 
- Requires `getMoveHitData()` method on Pokemon  
- Requires checking substitute volatiles
- Requires `chainModify` for damage reduction
- Requires `this.add()` for messages

### Stat-boost on-eat Berries (keeberry, lansatberry, liechiberry)
- Requires checking move properties (e.g. move.heal for present)
- onEat needs to call `this.boost()`

### Accuracy/Evasion Items (laxincense)
- onModifyAccuracy signature doesn't pass accuracy parameter
- Needs signature update to receive accuracy value

### Move modification Items (loadeddice)
- Needs ability to modify active move properties
- Specifically needs to delete/modify multiaccuracy field

### Species-specific Items needing base species (leek, luckypunch)
- Currently can access pokemon.base_species (ID)
- Need to look up species data and check baseSpecies field  
- Works but is verbose - IMPLEMENTED for lightball

### PP restoration (leppaberry)
- Requires move PP manipulation  
- Needs deduct PP, restore PP methods

### Status cure berries (lumberry)
- Requires status curing infrastructure
- Needs `pokemon.cureStatus()` method

### Orbs for specific Pokemon (lustrousglobe, lustrousorb)
- Similar to lightball - check base species
- Modify SpA for specific Pokemon (Palkia/Dialga)

### Side effects items (luminousmoss)
- Boost stats when hit by specific type
- Similar pattern to other boost items

### Flinch items (kingsrock)
- Add secondary effect (flinch) to moves
- Requires modifying move secondaries

## Missing Infrastructure for Item Callbacks

### Megastone Items (onTakeItem)
**Issue:** ItemData structure in src/dex.rs is missing  and  fields that are present in data/items.json

**Affects items:**
- All megastone items (raichunitex, raichunitey, sablenite, salamencite, sceptilite, scizorite, scolipite, scovillainite, scraftinite, sharpedonite, skarmorite, slowbronite, staraptite, starminite, steelixite, and many others)

**JS Code Pattern:**
```javascript
onTakeItem(item, source) {
    if (item.megaEvolves === source.baseSpecies.baseSpecies) return false;
    return true;
}
```

**Required Fix:**
Add to ItemData in src/dex.rs:
```rust
#[serde(rename = "megaEvolves", default)]
pub mega_evolves: Option<String>,  // or Vec<String> for some items
#[serde(rename = "megaStone", default)]
pub mega_stone: Option<String>,
```

### onModifyCritRatio Callbacks
**Issue:** Callback signatures are missing required parameters (critRatio value and user/pokemon position)

**Affects items:**
- razorclaw, scopelens, leek, luckypunch, stick

**Current Signature:**
```rust
pub fn on_modify_crit_ratio(battle: &mut Battle) -> EventResult
```

**Required Signature:**
```rust
pub fn on_modify_crit_ratio(battle: &mut Battle, crit_ratio: i32, pokemon_pos: (usize, usize)) -> EventResult
```

**JS Code Pattern:**
```javascript
onModifyCritRatio(critRatio) {
    return critRatio + 1;
}
```

## A-B Items Implementation Progress

### Implemented (8 items, 8 callbacks)
- [x] abilityshield - on_set_ability
- [x] absorbbulb - on_damaging_hit
- [x] adamantorb - on_base_power
- [x] blackbelt - on_base_power  
- [x] blackglasses - on_base_power
- [x] blacksludge - on_residual
- [x] brightpowder - on_modify_accuracy
- [x] buggem - on_source_try_primary_hit

### Blocked by Missing Infrastructure (33 items, 49 callbacks remaining)

#### Mega Stones (need ItemData.megaEvolves field)
- abomasite, absolite, absolitez, aerodactylite, aggronite, alakazite
- altarianite, ampharosite, audinite, banettite, barbaracite, baxcalibrite  
- beedrillite, blastoisinite, blazikenite

#### Memory/Drive Items (need onTakeItem for type-locked Pokemon)
- bugmemory, burndrive

#### Berries (need complex berry eating infrastructure)
- aguavberry (onUpdate, onTryEatItem, onEat)
- apicotberry (onUpdate, onEat) 
- aspearberry (onUpdate, onEat)
- berry (onResidual, onTryEatItem, onEat)
- bitterberry (onUpdate, onEat)
- burntberry (onUpdate, onEat)
- babiriberry (onSourceModifyDamage, onEat)

#### Items Needing Field/Pseudo-Weather
- airballoon (needs field.getPseudoWeather for gravity check)

#### Items Needing Special Infrastructure  
- adamantcrystal (needs onTakeItem for Dialga Origin check)
- adrenalineorb (needs boost event detection)
- assaultvest (needs move category disabling)
- berryjuice (needs HP threshold and healing)
- berserkgene (needs stat boost and item consumption)
- bigroot (needs heal amount modification)
- blueorb (needs primal reversion check)
- boosterenergy (needs paradox form detection)

### Missing Infrastructure Summary for A-B Items

1. **ItemData fields**
   - `mega_evolves: Option<String>` - for mega stone compatibility

2. **Field methods**
   - `field.get_pseudo_weather(id)` - EXISTS but signature may differ
   - Returns Option<&EffectState>

3. **Battle methods**  
   - `battle.clear_effect_state(state)` - for clearing item states
   - `battle.run_event("AfterUseItem", ...)` - EXISTS but need to verify usage

4. **Berry mechanics**
   - HP threshold detection (1/4, 1/2, 1/3 maxhp, etc.)
   - `pokemon.eat_item()` or similar consumption
   - `onTryEatItem` callback pattern
   - Nature-based confusion damage for berries

5. **Boost detection**
   - `onAfterBoost` event needs stat change info
   - Detect if stats were lowered (for adrenalineorb)

6. **Move category disabling**
   - `onDisableMove` needs access to move category
   - Prevent Status moves (for assaultvest)

7. **Heal modification**
   - Modify heal amounts (bigroot multiplies draining/healing by 1.3x)

8. **Status application**
   - `pokemon.try_set_status()` or similar
   - For berserkgene confusion

9. **Species form checks**
   - Origin forme detection (adamantcrystal for Dialga)
   - Primal forme detection (blueorb for Kyogre)
   - Paradox forme detection (boosterenergy)

## Missing Infrastructure for Items C-D

### ItemData missing fields
The following fields need to be added to `ItemData` in `src/dex.rs`:
- `mega_evolves: Option<String>` - For mega stones (cameruptite, chandelurite, charizarditex, charizarditey, etc.) ✓ ADDED
- `mega_stone: Option<String>` - The species this stone evolves into ✓ ADDED
- `item_user: Option<Vec<String>>` - Species that can use this item ✓ ADDED
- `boosts: Option<HashMap<String, i32>>` - Stat boosts for one-time use items ✓ ADDED

### Methods needed
- Access to item data from item position or ID in callbacks ✓ EXISTS (battle.dex.get_item_by_id)
- Access to species data from pokemon's base_species ✓ EXISTS (battle.dex.get_species_by_id)
- Methods to check if pokemon can use items (for mega stones) ✓ IMPLEMENTED in callbacks
- **MISSING: Battle-level useItem method that applies item effects and boosts**
  - JavaScript `target.useItem()` does more than just remove the item
  - It triggers item effects, applies boosts, and runs events
  - Need to implement this in battle.rs or battle_actions.rs
  - For now, implementing manual boost application in item callbacks
=======
## R-T Items Implementation Progress (Files 201-225)

### Blocked by Missing Infrastructure (25 items, 35 callbacks)

#### Type-Resist Berries (need getMoveHitData().typeMod and chainModify return value)
- roseliberry - onSourceModifyDamage, onEat
- shucaberry - onSourceModifyDamage, onEat
- tangaberry - onSourceModifyDamage, onEat

**Missing:**
- `pokemon.get_move_hit_data(move).type_mod` - Returns type effectiveness multiplier
- `battle.chain_modify(0.5)` must return modified damage value for onSourceModifyDamage
- Proper substitute check logic: `target.volatiles['substitute'] && !move.flags['bypasssub'] && !(move.infiltrates && this.gen >= 6)`

#### Recoil/Counter Berries (need complex damage tracking)
- rowapberry - onDamagingHit

**Missing:**
- `source.base_maxhp` field access
- `pokemon.has_ability('ripen')` with ternary division logic
- `battle.damage()` call from item callback

#### Safety Items (need immunity system overhaul)
- safetygoggles - onImmunity, onTryHit

**Missing:**
- `on_immunity(type_str)` signature with type parameter
- Return `false` to grant immunity (current system unclear)
- `on_try_hit` return `null` to block move
- `battle.dex.get_immunity('powder', pokemon)` check

#### Stat-Boost Berries (need eat_item + boost infrastructure)
- salacberry - onUpdate, onEat (boost spe: 1)
- starfberry - onUpdate, onEat (random stat boost +2)

**Missing:**
- `pokemon.eat_item()` callable from onUpdate
- `pokemon.has_ability('gluttony') && pokemon.ability_state.gluttony` check
- HP threshold detection (hp <= maxhp/4 or hp <= maxhp/2 with gluttony)
- `battle.boost({spe: 1})` syntax
- Random stat selection logic for starfberry

#### Healing Berries (need heal + TryHeal event)
- sitrusberry - onUpdate, onTryEatItem, onEat

**Missing:**
- `battle.run_event('TryHeal', pokemon, None, effect, amount)` - returns bool
- `battle.heal(pokemon.base_maxhp / 4)` from onEat
- HP threshold: hp <= maxhp/2

#### Damage-Based Healing (need move.totalDamage tracking)
- shellbell - onAfterMoveSecondarySelf

**Missing:**
- `move.total_damage` field in ActiveMove
- `pokemon.force_switch_flag` check
- `battle.heal(amount, pokemon)` signature

#### Damage-Triggered Boost Items (need item boosts field)
- snowball - onDamagingHit, boosts field

**Missing:**
- `ItemData.boosts` field (e.g., `{atk: 1}`)
- Auto-apply boosts when `pokemon.use_item()` is called
- Integration with item consumption

#### Species-Locked Items (need baseSpecies.num)
- souldew - onBasePower

**Missing:**
- `pokemon.base_species.num` field (species dex number)
- Check if num == 380 (Latias) or 381 (Latios)
- Type check for Psychic or Dragon moves
- `battle.chain_modify_fraction(4915, 4096)` for 1.2x boost

#### Crit Ratio Items (need parameter in callback)
- scopelens - onModifyCritRatio

**Missing:**
- Callback signature: `on_modify_crit_ratio(battle, crit_ratio: i32, pokemon_pos) -> EventResult`
- Return `EventResult::Number(crit_ratio + 1)`

#### Type Gems (need addVolatile)
- steelgem - onSourceTryPrimaryHit

**Missing:**
- `source.add_volatile('gem')` to apply gem boost
- Check `move.category === 'Status'` to skip
- `source.use_item()` returns bool

#### Mega Stones (need ItemData.megaEvolves field) - 12 items
- salamencite, sceptilite, scizorite, scolipite (Gen 9 Fan), scovillainite (Gen 9 Fan)
- scraftinite (Gen 9 Fan), sharpedonite, skarmorite (Gen 9 Fan), slowbronite
- staraptite (Gen 9 Fan), starminite (Gen 9 Fan), steelixite

**All have same pattern:**
- onTakeItem callback
- Check `item.mega_evolves === source.base_species.base_species`
- Return `false` to prevent taking, `true` to allow

**Missing:**
- `ItemData.mega_evolves: Option<String>` field
- `pokemon.base_species.base_species` field

### Missing Infrastructure Summary for R-T Items

1. **Damage Modification System**
   - `battle.chain_modify()` must return the modified value for use in onSourceModifyDamage
   - Currently returns i32 but event system doesn't use the return value
   - JS: `return this.chainModify(0.5)` actually returns the modified damage

2. **Type Effectiveness Access**
   - `pokemon.get_move_hit_data(move).type_mod` to check super-effective hits
   - Substitute bypass logic needs move.flags and move.infiltrates

3. **Immunity System**
   - `on_immunity(type: &str)` signature with type parameter
   - Return semantics: `false` = immune, `true` or `Continue` = not immune
   - Weather immunity: 'sandstorm', 'hail', 'powder'

4. **Move Blocking**
   - `on_try_hit` return `null` or `EventResult::Null` to completely block move
   - Different from `EventResult::Stop`

5. **Berry Consumption**
   - `pokemon.eat_item()` callable from onUpdate/onResidual
   - Gluttony ability detection with ability_state
   - HP threshold checks (1/4, 1/2, 1/3 maxhp)

6. **Healing System**
   - `battle.run_event('TryHeal', ...)` returns bool
   - `battle.heal(amount)` from item callbacks
   - `battle.heal(amount, pokemon)` with target parameter

7. **Item-Based Boosts**
   - `ItemData.boosts` field for auto-apply boosts
   - Applied when item is consumed via use_item()

8. **Species Data Access**
   - `pokemon.base_species.num` - species dex number
   - `pokemon.base_species.base_species` - base forme name
   - Used for species-locked items

9. **Active Move Tracking**
   - `move.total_damage` field to track cumulative damage dealt
   - Used for Shell Bell healing calculation

10. **Volatile Application**
    - `pokemon.add_volatile(volatile_id)` method
    - Used for type gems to trigger 1.3x power boost

11. **Mega Evolution**
    - `ItemData.mega_evolves` field
    - Mega stone locking mechanism via onTakeItem
