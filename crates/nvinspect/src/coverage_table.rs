//! What nv-rs covers of the game, by record type, file kind and NIF block
//! type: the hand-kept half of `nvinspect coverage` (the other half, what
//! the game has, is counted from its files).
//!
//! Keep it up to date when a module starts reading something: the
//! `coverage` tests fail when a form type the exe knows has no entry.

use esm::FourCC;

/// How far nv-rs has got with something.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// Everything the game does with it is done (as far as is known).
    Done,
    /// Some of it is read and used.
    Partial,
    /// Nothing reads it yet.
    NotStarted,
    /// Nothing to do: file structure, or the exe knows the type but the
    /// shipped game neither has nor uses any.
    NotApplicable,
}

impl Status {
    pub const ALL: [Status; 4] = [
        Status::Done,
        Status::Partial,
        Status::NotStarted,
        Status::NotApplicable,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Status::Done => "done",
            Status::Partial => "partial",
            Status::NotStarted => "not started",
            Status::NotApplicable => "n/a",
        }
    }
}

/// The exe's form type table (`01187000`: 12-byte entries {number,
/// signature, 0}), in form type order: index = the form type number the
/// engine uses (`RemoveAllTypedItems`, the form factory and saves).
pub const EXE_FORM_TYPES: [&str; 121] = [
    "NONE", "TES4", "GRUP", "GMST", "TXST", "MICN", "GLOB", "CLAS", "FACT", "HDPT", "HAIR", "EYES",
    "RACE", "SOUN", "ASPC", "SKIL", "MGEF", "SCPT", "LTEX", "ENCH", "SPEL", "ACTI", "TACT", "TERM",
    "ARMO", "BOOK", "CLOT", "CONT", "DOOR", "INGR", "LIGH", "MISC", "STAT", "SCOL", "MSTT", "PWAT",
    "GRAS", "TREE", "FLOR", "FURN", "WEAP", "AMMO", "NPC_", "CREA", "LVLC", "LVLN", "KEYM", "ALCH",
    "IDLM", "NOTE", "COBJ", "PROJ", "LVLI", "WTHR", "CLMT", "REGN", "NAVI", "CELL", "REFR", "ACHR",
    "ACRE", "PMIS", "PGRE", "PBEA", "PFLA", "WRLD", "LAND", "NAVM", "TLOD", "DIAL", "INFO", "QUST",
    "IDLE", "PACK", "CSTY", "LSCR", "LVSP", "ANIO", "WATR", "EFSH", "TOFT", "EXPL", "DEBR", "IMGS",
    "IMAD", "FLST", "PERK", "BPTD", "ADDN", "AVIF", "RADS", "CAMS", "CPTH", "VTYP", "IPCT", "IPDS",
    "ARMA", "ECZN", "MESG", "RGDL", "DOBJ", "LGTM", "MUSC", "IMOD", "REPU", "PCBE", "RCPE", "RCCT",
    "CHIP", "CSNO", "LSCT", "MSET", "ALOC", "CHAL", "AMEF", "CCRD", "CMNY", "CDCK", "DEHY", "HUNG",
    "SLPD",
];

/// The form type number of a record type, from the exe's table.
pub fn form_type_number(kind: FourCC) -> Option<usize> {
    EXE_FORM_TYPES
        .iter()
        .position(|s| s.as_bytes() == kind.as_bytes())
}

/// One record type: what it is, how far nv-rs has got, the subrecords nv-rs
/// reads (space separated), where, and what's missing.
#[derive(Debug, Clone, Copy)]
pub struct RecordEntry {
    pub kind: &'static str,
    pub what: &'static str,
    pub status: Status,
    pub read: &'static str,
    pub modules: &'static str,
    pub note: &'static str,
}

const fn r(
    kind: &'static str,
    what: &'static str,
    status: Status,
    read: &'static str,
    modules: &'static str,
    note: &'static str,
) -> RecordEntry {
    RecordEntry {
        kind,
        what,
        status,
        read,
        modules,
        note,
    }
}

use Status::{Done, NotApplicable as NA, NotStarted as No, Partial};

// Kept by hand from the source; checked against a scan of the
// four-character codes each module names (`coverage\raw\code_fourcc.txt`).
// "read" lists subrecords some nv-rs code decodes for this record type.
pub const RECORDS: &[RecordEntry] = &[
    r("TES4", "plugin header", Done, "HEDR CNAM SNAM MAST DATA", "esm::plugin", ""),
    r("GRUP", "group (file structure)", Done, "", "esm::plugin", ""),
    r("GMST", "game setting", Done, "DATA", "script_functions (GetGameSetting), each rule's own settings", "which settings each system uses: see settings.md"),
    r("TXST", "texture set", Partial, "TX00 TX01 DODT", "world::land, world::impacts", "TX02-TX05 (glow, height, environment, mask) and DNAM flags; texture sets swapped onto models (MODS/MO2S/...) and decals' TXST on placed objects"),
    r("MICN", "menu icon", No, "", "", "ICON (menu icons)"),
    r("GLOB", "global variable", Done, "FNAM FLTV", "world::scripting, world::leveled", ""),
    r("CLAS", "class", No, "", "(only an NPC's CNAM is compared, GetIsClass)", "ATTR, DATA (tag skills, training, services), DESC"),
    r("FACT", "faction", Partial, "DATA XNAM WMI1 FULL", "world::factions, world::crime, world::music", "ranks and titles (RNAM MNAM FNAM), CNAM"),
    r("HDPT", "head part", Partial, "MODL", "world::actor", "HNAM extra parts, DATA playable flag"),
    r("HAIR", "hair", Partial, "MODL ICON", "world::actor", "DATA flags (playable, not male/female, fixed colour)"),
    r("EYES", "eyes", Partial, "ICON", "world::actor", "DATA flags (playable)"),
    r("RACE", "race", Partial, "NAM0 NAM1 MNAM FNAM INDX MODL ICON DATA", "world::actor, world::script_functions (IsChild)", "DATA skill bonuses, heights and weights; ATTR; default hair/eyes (DNAM CNAM ENAM HNAM); FaceGen defaults (FGGS FGGA FGTS); older/younger races (ONAM YNAM); voices (VTCK); PNAM/UNAM FaceGen clamps"),
    r("SOUN", "sound", Partial, "FNAM SNDD", "world::sound, world::weapon_fx", "SNDX, reverb/random (GNAM HNAM RNAM); distances, static attenuation and curve only for firing and swing sounds (world::weapon_fx)"),
    r("ASPC", "acoustic space", Partial, "SNAM RDAT", "world::sound, world::music", "INAM (reverb), WNAM (walla), ANAM; placed acoustic spaces"),
    r("SKIL", "skill (Oblivion-era; unused)", NA, "", "", "none in the data (New Vegas uses AVIF)"),
    r("MGEF", "base effect", Partial, "DATA FULL", "world::items, world::magic", "DESC; DATA's visuals (effect shader, light, hit/cast sounds, projectile, explosion); many archetypes (see exe_systems.md)"),
    r("SCPT", "script", Partial, "SCTX SCDA SCVR SLSD SCRO", "script, world::scripting", "SCHR (script type), SCRV; scripts run from their source text, not the compiled SCDA"),
    r("LTEX", "landscape texture", Partial, "TNAM GNAM", "world::land, world::grass", "HNAM (Havok material and friction: footsteps, impacts), SNAM (specular exponent)"),
    r("ENCH", "object effect (enchantment)", No, "", "", "ENIT EFID EFIT CTDA: weapons' and armour's own effects (EITM) aren't applied"),
    r("SPEL", "actor effect (spell, ability, addiction)", Partial, "SPIT EFID EFIT CTDA FULL", "world::magic, world::items", "abilities on races and actors (SPLO) aren't given"),
    r("ACTI", "activator", Partial, "MODL SCRI FULL", "world::placement, world::scripting", "SNAM (looping sound), VNAM (activation sound), RNAM (radio station), WNAM (water), INAM (radio template), XATO, destruction (DEST DSTD DSTF DMDL)"),
    r("TACT", "talking activator", Partial, "MODL SCRI FULL", "world::placement, world::scripting", "SNAM, VNAM (voice type), INAM (radio template): talking activators don't speak"),
    r("TERM", "terminal", Partial, "DESC DNAM PNAM ITXT RNAM ANAM INAM TNAM SCTX CTDA SCRI MODL FULL", "world::terminal, world::hacking", "SNAM (sound)"),
    r("ARMO", "armour and clothing", Partial, "BMDT MODL MOD2 MOD3 DATA DNAM FULL ICON YNAM ZNAM", "world::actor, world::items, world::combat, world::impacts, world::sound", "first-person and ground models (MOD4, MOD2 in 1st person), the female icon (ICO2), BIPL/REPL lists, ETYP, EITM effects, SNAM sounds, TNAM, BMCT"),
    r("BOOK", "book", Partial, "DATA FULL ICON YNAM ZNAM", "world::items, world::sound", "DESC (the text shown when read), the book menu"),
    r("CLOT", "clothing (Oblivion-era; unused)", NA, "", "", "none in the data"),
    r("CONT", "container", Partial, "CNTO MODL SCRI FULL DATA SNAM QNAM", "world::scripting, world::placement, world::items (weight), world::sound", "DATA flags (respawns), RNAM, COED, destruction"),
    r("DOOR", "door", Partial, "MODL SCRI SNAM ANAM FULL", "world::placement, world::sound", "FNAM flags (automatic, hidden, minimal use, sliding), BNAM loop sound; the leaf's swing isn't drawn"),
    r("INGR", "ingredient", Partial, "DATA", "world::items (weight; counted as food by sandboxes)", "ENIT EFID EFIT: one unused record in the data"),
    r("LIGH", "light", Partial, "DATA FNAM MODL", "world::placement", "flicker/pulse and spot flags drawn as steady point lights; SNAM (sound); shadows"),
    r("MISC", "misc item", Partial, "DATA MODL FULL SCRI ICON YNAM ZNAM", "world::items, world::placement, world::sound", "RNAM"),
    r("STAT", "static", Partial, "MODL", "world::placement, preview", "BRUS (passthrough sound), RNAM (looping sound), MODS texture swaps"),
    r("SCOL", "static collection", Done, "ONAM DATA", "world::placement", ""),
    r("MSTT", "movable static", Partial, "MODL", "world::placement", "SNAM (sound), DATA flags, destruction; its Havok bodies aren't simulated"),
    r("PWAT", "placeable water", Done, "MODL DNAM", "world::water", ""),
    r("GRAS", "grass", Partial, "DATA MODL", "world::grass, cellview::grass", "billboard grass, point-lit and shadowed variants"),
    r("TREE", "tree (SpeedTree)", Partial, "EDID OBND MODL ICON SNAM CNAM BNAM", "world::tree, cellview::tree, speedtree", "billboards (none drawn in the recording), the one trunk collision capsule (WhiteOak01)"),
    r("FLOR", "flora (Oblivion-era; unused)", NA, "", "", "none in the data"),
    r("FURN", "furniture", Partial, "MNAM MODL SCRI FULL", "world::furniture", "the player's seat, sit states and animations"),
    r("WEAP", "weapon", Partial, "DATA DNAM CRDT NAM0 SNAM VATS MODL INAM ETYP FULL SCRI ICON YNAM ZNAM", "world::combat, world::items, world::vats, world::impacts, world::actor, world::sound", "weapon mods (WMI1-3, MWD1-7, WNM1-7), first-person model (WNAM), sounds (NAM6-9, UNAM; SNAM XNAM TNAM WMS1-2 in world::weapon_fx), shell casing and impacts' models, EITM effects, REPL/BIPL lists, EAMT"),
    r("AMMO", "ammunition", Partial, "DATA DAT2 RCIL FULL ICON YNAM ZNAM", "world::items, world::combat, world::sound", "ONAM/QNAM short names, models of casings"),
    r("NPC_", "non-player character", Partial, "ACBS AIDT CNTO DATA DNAM ENAM FGGA FGGS HCLR HNAM NAM6 PKID PNAM RNAM SNAM TPLT VTCK ZNAM CNAM SCRI FULL OBND", "world::actor, world::combat, world::factions, world::ai, world::dialogue, world::scripting", "FGTS (skin tone), LNAM (hair length), NAM4 (impact material), NAM5, NAM7 (weight), SPLO (abilities), EITM, EAMT, INAM (death item), COED, destruction"),
    r("CREA", "creature", Partial, "ACBS AIDT CNTO DATA MODL NIFZ PNAM RNAM SNAM TPLT ZNAM VTCK PKID SCRI FULL OBND CSCR CSDT CSDI CSDC TNAM", "world::actor, world::combat, world::combat_ai, world::body_parts, world::impacts, world::movement", "KFFZ (animation list), NIFT, NAM4/NAM5, LNAM, WNAM (foot weight), BNAM (base scale), CNAM, SPLO, EITM, EAMT, INAM, destruction"),
    r("LVLC", "leveled creature list", Done, "LVLD LVLF LVLO", "world::leveled, world::actor", ""),
    r("LVLN", "leveled NPC list", Done, "LVLD LVLF LVLO", "world::leveled, world::actor", ""),
    r("KEYM", "key", Partial, "DATA FULL MODL YNAM ZNAM", "world::items, world::locks, world::sound", ""),
    r("ALCH", "ingestible (food, drink, chems)", Partial, "DATA ENIT EFID EFIT CTDA FULL MODL ETYP ICON YNAM ZNAM", "world::items, world::magic, world::barter, world::sound", ""),
    r("IDLM", "idle marker", Partial, "IDLF IDLA IDLT", "world::idles, world::sandbox", "IDLC; the IDLT timer"),
    r("NOTE", "note", Partial, "DATA TNAM FULL", "world::terminal, world::scripting", "voice holotapes (SNAM), image notes (XNAM), ONAM quests, the Pip-Boy notes page"),
    r("COBJ", "constructible object (unused)", NA, "", "", "none in the data (New Vegas uses RCPE)"),
    r("PROJ", "projectile", Partial, "DATA NAM1", "world::combat, world::combat_ai, world::weapon_fx", "projectiles flying (non-hitscan), tracers on screen, muzzle flash light and NAM2, sounds in flight, explosions, VNAM sound level, destruction"),
    r("LVLI", "leveled item list", Partial, "LVLD LVLF LVLG LVLO", "world::leveled", "COED (owner, condition of what's given)"),
    r("WTHR", "weather", Partial, "NAM0 FNAM DATA PNAM ONAM ANAM BNAM CNAM DNAM *IAD", "world::weather", "SNAM (weather sounds), INAM, LNAM; rain/lightning effects"),
    r("CLMT", "climate", Done, "WLST TNAM FNAM GNAM MODL", "world::weather", ""),
    r("REGN", "region", Partial, "RPLI RPLD WNAM RDAT RDWT RDSI RDSB", "world::region, world::weather, world::music", "RDSD (region sounds), RDMP (map names), RDOT (objects), RDID, RCLR, ICON"),
    r("NAVI", "navmesh info map", No, "", "", "NVMI NVCI NVER: the navmesh connection index (paths across cells are joined by NVEX instead)"),
    r("CELL", "cell", Partial, "DATA XCLL XCLC XCLW XCWT XCIM XCAS XCCM XCLR XEZN XOWN LTMP LNAM FULL", "world::cell, world::exterior, world::water, world::music, world::crime", "XNAM (water noise texture); XCMO (music type) is read by nothing in the game either"),
    r("REFR", "placed object", Partial, "NAME DATA XSCL XESP XEMI XTEL XLKR XLOC XOWN XRNK XPRM XTRI XACT XRDS XSED XMRK FNAM FULL TNAM XIBS XCNT ONAM MMRK CNAM", "world::placement, world::map, world::locks, world::crime, world::music, world::tree", "XMBO/XMBP (multibounds), XOCP/XORD (occlusion), XRGD (ragdoll pose), XPWR (reflected by water), XRAD (radiation), XRDO (radio), XAPD/XAPR (activate parents), XNDP/XPOD (portals), patrol data (XPRD XPPA INAM SCHR SCTX TNAM), XHLP (health), XSRF/XSRD, XAMT/XAMC, XCHG, XTRG, XATO, audio marker BNAM/MNAM/NNAM"),
    r("ACHR", "placed NPC", Partial, "NAME DATA XESP XLKR XMRC XSCL XEZN XIBS", "world::placement, world::ai, world::barter", "XRGD/XRGB (ragdoll pose: placed dead), XLCM (level modifier), XAPD/XATO"),
    r("ACRE", "placed creature", Partial, "NAME DATA XESP XLKR XMRC XSCL XEZN XOWN", "world::placement, world::ai", "XRGD/XRGB, XLCM, XAPD/XATO"),
    r("PMIS", "placed missile", NA, "", "", "none in the data"),
    r("PGRE", "placed grenade", Partial, "NAME DATA XESP XSCL XOWN", "world::placement", "XPWR, XRGD; placed as a static model, not a live grenade"),
    r("PBEA", "placed beam", NA, "", "", "none in the data"),
    r("PFLA", "placed flame", NA, "", "", "none in the data"),
    r("WRLD", "worldspace", Partial, "CNAM DNAM NAM2 NAM3 NAM4 INAM PNAM WNAM DATA", "world::exterior, world::water, world::weather", "MNAM/ONAM (world map), NAM0/NAM9 (bounds), XNAM (water noise), ZNAM (music), NNAM (canopy shadow), OFST, XEZN, ICON"),
    r("LAND", "terrain", Done, "DATA VHGT VNML VCLR BTXT ATXT VTXT", "world::land", ""),
    r("NAVM", "navmesh", Partial, "NVVX NVTR NVEX", "world::ai", "NVER, DATA, NVCA (cover), NVDP (door portals), NVGD (grid)"),
    r("TLOD", "tree LOD (unused)", NA, "", "", "none in the data"),
    r("DIAL", "dialogue topic", Partial, "DATA PNAM QSTI TDUM FULL", "world::dialogue", "INFC/INFX (line order across plugins)"),
    r("INFO", "dialogue line", Partial, "DATA QSTI TRDT NAM1 CTDA TCLT NAME RNAM KNAM SCTX NEXT PNAM", "world::dialogue", "ANAM (speaker), SNAM (sound), TCLF/TCFU, LNAM, DNAM, NAM2/NAM3 (notes), the dialogue camera"),
    r("QUST", "quest", Partial, "DATA FULL INDX QSDT CTDA CNAM QOBJ NNAM QSTA SCRI SCTX NAM0", "world::quest, world::quest_targets, world::scripting", "Pip-Boy map quest markers"),
    r("IDLE", "idle animation", Partial, "ANAM DATA CTDA MODL", "world::idles", "upper-body idles play as whole-body"),
    r("PACK", "AI package", Partial, "PKDT PLDT PLD2 PSDT PTDT PTD2 CTDA PKDD PKW3 PKPT PKE2 PKFD POBA POEA POCA INAM TNAM SCHR SCDA SCTX SLSD SCVR SCRO SCRV", "world::ai, world::ai::actions, world::ai::data, world::ai::flee, world::ai::guard, world::sandbox, world::movement, world::social", "use weapon, use item at, ambush wait and patrol procedures; guard intruder scan/warnings; PKAM/PKED are markers the loader skips; PKDD's f32 at 12 and u32 at 20, IDLA/IDLC/IDLF/IDLT, PUID, CNAM"),
    r("CSTY", "combat style", Done, "CSTD CSAD CSSD", "world::combat_ai", ""),
    r("LSCR", "load screen", No, "", "", "ICON, DESC, LNAM, WMI1: no loading screens"),
    r("LVSP", "leveled spell (unused)", NA, "", "", "none in the data"),
    r("ANIO", "animated object", No, "", "", "MODL, DATA (idle): furniture and marker animations"),
    r("WATR", "water type", Partial, "ANAM NNAM DNAM FNAM SNAM XNAM GNAM DATA", "world::water", "underwater, wading ripples, MNAM, sounds"),
    r("EFSH", "effect shader", No, "", "", "DATA ICON ICO2 NAM7: glows, fire, membrane shaders on actors and objects"),
    r("TOFT", "(unused)", NA, "", "", "none in the data"),
    r("EXPL", "explosion", No, "", "", "DATA MODL EITM MNAM: no explosions"),
    r("DEBR", "debris", No, "", "", "DATA MODT"),
    r("IMGS", "image space", Done, "DNAM", "world::image_space", ""),
    r("IMAD", "image space modifier", Partial, "DNAM *IAD TNAM NAM3 BNAM VNAM", "world::modifier", "radial blur, depth of field, motion blur (RNAM SNAM UNAM NAM1 NAM2 NAM4 WNAM XNAM YNAM), sounds (RDSD RDSI)"),
    r("FLST", "form list", Done, "LNAM", "world::script_functions, world::perks, world::combat", ""),
    r("PERK", "perk", Partial, "DATA DESC FULL PRKE PRKC CTDA EPFT EPFD PRKF ICON", "world::perks, world::chargen", "entry points whose mechanic isn't here (mines, terminal lockouts, addiction rolls, activation prompts, explosions, knockdowns, thrown weapons, unarmed specials; see exe_systems.md), EPF2/EPF3 (activate choices), perk scripts (SCHR SCTX)"),
    r("BPTD", "body part data", Partial, "BPTN BPNN BPNT BPNI BPND NAM1 NAM4 NAM5 RAGA MODL", "world::body_parts", "dismembering and exploding limbs"),
    r("ADDN", "addon node", No, "", "", "DATA DNAM MODL SNAM: particle add-ons on models"),
    r("AVIF", "actor value information", Partial, "FULL DESC ICON", "world::chargen", "ANAM"),
    r("RADS", "radiation stage", Done, "DATA", "world::living::needs", ""),
    r("CAMS", "camera shot", No, "", "", "DATA MNAM MODL: kill cam, V.A.T.S. cameras"),
    r("CPTH", "camera path", No, "", "", "CTDA ANAM DATA SNAM"),
    r("VTYP", "voice type", Partial, "", "world::dialogue (the editor ID names the voice folder)", "DNAM flags (allow default dialogue, female)"),
    r("IPCT", "impact", Partial, "DATA DNAM DODT MODL SNAM NAM1", "world::impacts", "effects aren't drawn yet"),
    r("IPDS", "impact data set", Done, "DATA", "world::impacts", ""),
    r("ARMA", "armour addon", No, "", "", "BMDT DATA DNAM MODL MOD2-4 ETYP"),
    r("ECZN", "encounter zone", Partial, "DATA", "world::crime", "minimum level and leveled lists' area level, never-resets flag"),
    r("MESG", "message", Partial, "DESC FULL DNAM ITXT CTDA", "world::scripting", "INAM (icon), TNAM (display time)"),
    r("RGDL", "ragdoll", No, "", "", "NVER DATA XNAM TNAM RAFD RAFB RAPS ANAM: feedback and pose matching"),
    r("DOBJ", "default object manager", No, "", "", "DATA: the engine's default forms (sounds, effects)"),
    r("LGTM", "lighting template", Done, "DATA", "world::cell", ""),
    r("MUSC", "music type", Done, "FNAM ANAM", "world::music", ""),
    r("IMOD", "item mod", Partial, "DATA FULL", "world::items", "mods attached to weapons and their effects, DESC"),
    r("REPU", "reputation", Done, "DATA FULL", "world::reputation", ""),
    r("PCBE", "(unused)", NA, "", "", "none in the data"),
    r("RCPE", "recipe", No, "", "", "DATA RCIL RCQY RCOD CTDA: crafting (workbench, campfire, reloading bench)"),
    r("RCCT", "recipe category", No, "", "", "DATA FULL"),
    r("CHIP", "casino chip", No, "", "(recognised as an item)", "the casino games"),
    r("CSNO", "casino", No, "", "", "DATA, models: blackjack, roulette, slots"),
    r("LSCT", "load screen type", No, "", "", "DATA"),
    r("MSET", "media set (music)", Done, "NAM1 NAM2 NAM3 NAM4 NAM5 NAM6 NAM7 NAM8 NAM9 NAM0 ANAM BNAM CNAM JNAM KNAM LNAM MNAM NNAM ONAM PNAM DNAM ENAM FNAM GNAM HNAM INAM", "world::music", "DATA isn't read (whether the game uses it isn't traced)"),
    r("ALOC", "media location controller", Done, "NAM1 NAM4 NAM5 NAM6 GNAM LNAM HNAM YNAM ZNAM XNAM RNAM", "world::music", "NAM2 NAM3 NAM7 FNAM aren't read (whether the game uses them isn't traced)"),
    r("CHAL", "challenge", No, "", "", "DATA SCRI DESC SNAM XNAM: challenges and their counters"),
    r("AMEF", "ammo effect", Done, "DATA", "world::combat", ""),
    r("CCRD", "Caravan card", Partial, "DATA", "world::items", "INTV, TX00/TX01: the Caravan game"),
    r("CMNY", "Caravan money", Partial, "DATA", "world::items", ""),
    r("CDCK", "Caravan deck", No, "", "", "CARD DATA: the Caravan game"),
    r("DEHY", "dehydration stage", Done, "DATA", "world::living::needs", ""),
    r("HUNG", "hunger stage", Done, "DATA", "world::living::needs", ""),
    r("SLPD", "sleep deprivation stage", Done, "DATA", "world::living::needs", ""),
];

/// The entry for a record type.
pub fn record(kind: FourCC) -> Option<&'static RecordEntry> {
    RECORDS
        .iter()
        .find(|e| e.kind.as_bytes() == kind.as_bytes())
}

/// Whether nv-rs reads a subrecord of a record type. The editor ID is
/// read for every record (`esm`, finding records by editor ID).
pub fn reads_subrecord(kind: FourCC, sub: FourCC) -> bool {
    if sub.as_bytes() == b"EDID" {
        return record(kind).is_some();
    }
    record(kind).is_some_and(|e| {
        e.read.split_whitespace().any(|s| {
            let s = s.as_bytes();
            // "*IAD": any first byte (image space modifier tracks are
            // numbered by it).
            s == sub.as_bytes() || (s[0] == b'*' && s[1..] == sub.as_bytes()[1..])
        })
    })
}

/// A subrecord's code for printing: a first byte that isn't a letter (the
/// image space modifier tracks `\0IAD`..) as `[00]`.
pub fn sig_text(sub: FourCC) -> String {
    sub.as_bytes()
        .iter()
        .map(|&b| {
            if b.is_ascii_graphic() {
                char::from(b).to_string()
            } else {
                format!("[{b:02X}]")
            }
        })
        .collect()
}

/// One file kind (by extension).
#[derive(Debug, Clone, Copy)]
pub struct FileKind {
    pub ext: &'static str,
    pub what: &'static str,
    pub status: Status,
    pub reader: &'static str,
    pub note: &'static str,
}

const fn f(
    ext: &'static str,
    what: &'static str,
    status: Status,
    reader: &'static str,
    note: &'static str,
) -> FileKind {
    FileKind {
        ext,
        what,
        status,
        reader,
        note,
    }
}

pub const FILE_KINDS: &[FileKind] = &[
    f("bik", "Bink video (the intro)", No, "", "FNVIntro.bik isn't played (a Bink decoder)"),
    f("ctl", "FaceGen controls (facegen\\si.ctl)", No, "", "the face sliders' mapping onto FaceGen morphs (race menu, tints)"),
    f("dat", "LIPSinc lip-sync generator data (lsdata\\, XOR 0x85 text: TIMIT/CMU dictionary)", No, "", "only for making lip sync from a voice file; whether the game does that at run time isn't traced"),
    f("dds", "texture", Done, "dds", "every texture in the data decodes (DXT1/3/5, A8R8G8B8, A4R4G4B4, R8G8B8)"),
    f("dlodsettings", "distant LOD settings per worldspace (levels, cell bounds)", No, "", "the LOD quadtree (levels 8, 16, 32) isn't read"),
    f("egm", "FaceGen shape morphs", Done, "nif::egm", ""),
    f("egt", "FaceGen texture morphs", No, "", "skin tints for faces without a shipped facemods texture"),
    f("fnt", "font", Done, "ui::font", ""),
    f("kf", "animation", Partial, "nif::anim", "text keys (hit keys, sounds), animation notes, blending; 5 files in the older 20.0.0.4 format aren't read"),
    f("lip", "lip sync", Done, "world::lip", ""),
    f("mp3", "music", Done, "mp3", "the radio's songs (sound\\songs) aren't played"),
    f("nif", "model", Partial, "nif, preview, cellview", "see the block types below: particles, lights, cameras, most controllers aren't decoded"),
    f("ogg", "voice and some sounds (Vorbis)", Partial, "viewer (through Bevy's audio)", "no decoder in nv-rs's own crates; played by the viewer"),
    f("psa", "pose array (death poses)", No, "", "bhkPoseArray: the death poses aren't used"),
    f("psd", "Photoshop file (leftover)", NA, "", "not loaded by the game"),
    f("sdp", "compiled shader package", Done, "shaders", "every package reads; the viewer ports shaders by hand (shaders.md)"),
    f("spt", "SpeedTree tree", Partial, "speedtree", "fronds (off in every shipped tree), stored leaves (none ship)"),
    f("tai", "texture atlas index", Done, "ui", ""),
    f("tex", "font texture", Done, "ui::font", ""),
    f("tri", "FaceGen morph targets", Done, "nif::tri", ""),
    f("txt", "text: the hacking word list (menus\\falloutdict.txt), the menu sources (master_menu_file.txt), placeholders", Partial, "world::hacking", "only falloutdict.txt (the hacking game's words) is read"),
    f("wav", "sound (PCM)", Partial, "cellview::sound", "16-bit PCM read; no 3D placement, volumes or most sound kinds"),
    f("xml", "menu", Partial, "ui", "every menu file is worked out as the game does; only the HUD is drawn"),
];

pub fn file_kind(ext: &str) -> Option<&'static FileKind> {
    FILE_KINDS.iter().find(|k| k.ext.eq_ignore_ascii_case(ext))
}

/// NIF block types nv-rs decodes, and where.
pub const NIF_BLOCKS: &[(&str, &str)] = &[
    // Scene graph (nif::blocks)
    ("NiNode", "nif::blocks (node)"),
    ("BSFadeNode", "nif::blocks (node)"),
    (
        "BSMultiBoundNode",
        "nif::blocks (node; its multibound isn't)",
    ),
    ("BSOrderedNode", "nif::blocks (node)"),
    ("BSValueNode", "nif::blocks (node)"),
    ("BSBlastNode", "nif::blocks (node)"),
    ("BSDamageStage", "nif::blocks (node)"),
    ("BSDebrisNode", "nif::blocks (node)"),
    ("NiBillboardNode", "nif::blocks (node, facing the camera)"),
    ("NiSwitchNode", "nif::blocks (node, first child)"),
    ("NiLODNode", "nif::blocks (node, nearest level)"),
    ("NiBone", "nif::blocks (node)"),
    ("BSTreeNode", "nif::blocks (node)"),
    ("RootCollisionNode", "nif::blocks (node)"),
    ("NiTriShape", "nif::blocks (geometry)"),
    ("NiTriStrips", "nif::blocks (geometry)"),
    ("BSSegmentedTriShape", "nif::blocks, nif::segments"),
    ("BSLODTriShape", "nif::blocks (geometry)"),
    ("NiTriShapeData", "nif::blocks"),
    ("NiTriStripsData", "nif::blocks"),
    // Properties
    ("BSShaderPPLightingProperty", "nif::blocks (shader)"),
    ("Lighting30ShaderProperty", "nif::blocks (shader)"),
    ("BSShaderNoLightingProperty", "nif::blocks (shader)"),
    ("SkyShaderProperty", "nif::blocks (shader)"),
    ("TileShaderProperty", "nif::blocks (shader)"),
    ("TallGrassShaderProperty", "nif::blocks (shader)"),
    (
        "WaterShaderProperty",
        "cellview::water (recognised by type; fields from the WATR record)",
    ),
    ("BSShaderTextureSet", "nif::blocks"),
    ("NiTexturingProperty", "nif::blocks"),
    ("NiSourceTexture", "nif::blocks"),
    ("NiMaterialProperty", "nif::blocks"),
    ("NiAlphaProperty", "nif::blocks"),
    ("NiStencilProperty", "nif::blocks"),
    ("NiZBufferProperty", "nif::blocks"),
    // Skinning and extra data (nif::skin)
    ("NiSkinInstance", "nif::skin"),
    ("BSDismemberSkinInstance", "nif::skin"),
    ("NiSkinData", "nif::skin"),
    ("NiSkinPartition", "nif::skin"),
    ("NiStringExtraData", "nif::skin (Prn bone names)"),
    ("BSFurnitureMarker", "nif::skin (furniture markers)"),
    ("BSBound", "nif::skin (bounds)"),
    // Animation (nif::anim)
    ("NiControllerSequence", "nif::anim"),
    ("NiTransformInterpolator", "nif::anim"),
    ("NiTransformData", "nif::anim"),
    ("NiKeyframeData", "nif::anim"),
    ("NiBSplineCompTransformInterpolator", "nif::anim"),
    ("NiBSplineData", "nif::anim"),
    ("NiBSplineBasisData", "nif::anim"),
    ("NiFloatInterpolator", "nif::anim (material keys)"),
    ("NiFloatData", "nif::anim (material keys)"),
    ("NiPoint3Interpolator", "nif::anim (material keys)"),
    ("NiPosData", "nif::anim (material keys)"),
    ("NiAlphaController", "nif::anim (material keys)"),
    ("NiMaterialColorController", "nif::anim (material keys)"),
    (
        "BSMaterialEmittanceMultController",
        "nif::anim (material keys)",
    ),
    // Particles (nif::particles, run by world::particles)
    ("NiParticleSystem", "nif::particles"),
    ("NiPSysData", "nif::particles"),
    (
        "BSStripParticleSystem",
        "nif::particles (read; strips not drawn)",
    ),
    ("BSStripPSysData", "nif::particles (read; strips not drawn)"),
    ("BSMasterParticleSystem", "nif::particles (read as a node)"),
    ("NiPSysAgeDeathModifier", "nif::particles"),
    ("NiPSysBoundUpdateModifier", "nif::particles (culling only)"),
    ("NiPSysPositionModifier", "nif::particles"),
    ("NiPSysBoxEmitter", "nif::particles"),
    ("NiPSysCylinderEmitter", "nif::particles"),
    ("NiPSysSphereEmitter", "nif::particles"),
    ("NiPSysMeshEmitter", "nif::particles"),
    ("NiPSysSpawnModifier", "nif::particles"),
    ("NiPSysGravityModifier", "nif::particles"),
    ("NiPSysDragModifier", "nif::particles"),
    ("NiPSysBombModifier", "nif::particles"),
    ("NiPSysGrowFadeModifier", "nif::particles"),
    ("NiPSysRotationModifier", "nif::particles"),
    ("NiPSysColorModifier", "nif::particles"),
    ("NiColorData", "nif::particles"),
    ("BSPSysSimpleColorModifier", "nif::particles"),
    ("NiPSysColliderManager", "nif::particles"),
    ("NiPSysPlanarCollider", "nif::particles"),
    ("NiPSysSphericalCollider", "nif::particles"),
    ("BSWindModifier", "nif::particles"),
    ("BSParentVelocityModifier", "nif::particles (read; not run)"),
    (
        "BSPSysStripUpdateModifier",
        "nif::particles (read; not run)",
    ),
    ("NiPSysEmitterCtlr", "nif::particles"),
    (
        "BSPSysMultiTargetEmitterCtlr",
        "nif::particles (read; not run)",
    ),
    ("NiPSysUpdateCtlr", "nif::particles"),
    ("NiPSysResetOnLoopCtlr", "nif::particles"),
    ("NiPSysModifierActiveCtlr", "nif::particles"),
    ("NiPSysEmitterSpeedCtlr", "nif::particles"),
    ("NiPSysEmitterInitialRadiusCtlr", "nif::particles"),
    ("NiPSysEmitterLifeSpanCtlr", "nif::particles"),
    ("NiPSysEmitterDeclinationCtlr", "nif::particles"),
    ("NiPSysEmitterDeclinationVarCtlr", "nif::particles"),
    ("NiPSysEmitterPlanarAngleCtlr", "nif::particles"),
    ("NiPSysEmitterPlanarAngleVarCtlr", "nif::particles"),
    ("NiPSysGravityStrengthCtlr", "nif::particles"),
    ("NiPSysInitialRotAngleCtlr", "nif::particles"),
    ("NiPSysInitialRotSpeedCtlr", "nif::particles"),
    ("NiPSysInitialRotSpeedVarCtlr", "nif::particles"),
    ("NiBoolInterpolator", "nif::particles"),
    ("NiBoolTimelineInterpolator", "nif::particles"),
    ("NiBoolData", "nif::particles"),
    // Collision (nif::collision, nif::ragdoll)
    ("bhkCollisionObject", "nif::collision"),
    ("bhkRigidBody", "nif::collision, nif::ragdoll"),
    ("bhkRigidBodyT", "nif::collision"),
    ("bhkMoppBvTreeShape", "nif::collision"),
    ("bhkPackedNiTriStripsShape", "nif::collision"),
    ("hkPackedNiTriStripsData", "nif::collision"),
    ("bhkNiTriStripsShape", "nif::collision"),
    ("bhkBoxShape", "nif::collision"),
    ("bhkSphereShape", "nif::collision"),
    ("bhkCapsuleShape", "nif::collision, nif::ragdoll"),
    ("bhkConvexVerticesShape", "nif::collision"),
    ("bhkConvexTransformShape", "nif::collision"),
    ("bhkTransformShape", "nif::collision"),
    ("bhkListShape", "nif::collision"),
    ("bhkBlendCollisionObject", "nif::ragdoll"),
    ("bhkRagdollConstraint", "nif::ragdoll"),
    ("bhkLimitedHingeConstraint", "nif::ragdoll"),
    ("bhkMalleableConstraint", "nif::ragdoll"),
    ("bhkBallAndSocketConstraint", "nif::ragdoll"),
];

pub fn nif_block(name: &str) -> Option<&'static str> {
    NIF_BLOCKS.iter().find(|b| b.0 == name).map(|b| b.1)
}
