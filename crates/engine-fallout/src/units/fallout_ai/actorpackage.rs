//! `fallout/ai/actorpackage.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008c6060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores 0 in the word at `this + 4`. The engine map names it
/// `ActorPackage::NullInstanceData` (Xbox PDB), but the body is shared by
/// identical-code folding (constructors such as `00487420` call it on their
/// own objects, as do `00475350`, `0048e7a0`, ...), so it is named by
/// address.
pub fn fn_008c6060(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 4, 0);
}

// Translated from 008c5ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorPackage::Clear` (engine map name; the object is the four word record `008c5e20`
/// builds). Lets go of the package at `+0` and the object at `+4` and restores the start values.
///
/// - The package at `+0`, when set: the pointer in the word at `+0x208` of the object at
///   `0x011dea3c` is cleared when it is this package; the package's count is lowered
///   (`008c6030`) and when it reaches 0 and `00674d40` agrees the package is handed to
///   `0085a2e0` of the object at `0x011de45c` if `0047c850` of it answers true, otherwise
///   destroyed through virtual slot `+0x10` (argument 1). The word is cleared in either case.
/// - The object at `+4`, when set, is destroyed through virtual slot `+0` (argument 1) and the
///   word cleared.
/// - The float at `+0x10` becomes the float at `0x01012054`, the word at `+0xc` 0xffffffff and
///   the words at `+0x14` and `+8` 0.
pub fn actorpackage_clear(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let package = e.mem.u32(base);
    if package != 0 {
        let globals = e.global::<u32>(0x011d_ea3c);
        if package == e.mem.u32(globals + 0x208) {
            let globals = e.global::<u32>(0x011d_ea3c);
            e.mem.set_u32(globals + 0x208, 0);
        }
        let package = e.mem.u32(base);
        let remaining = e.call(0x008c_6030, &args![package]).u32();
        if remaining == 0 && e.call(0x0067_4d40, &args![package]).bool() {
            let package = e.mem.u32(base);
            e.mem.set_u32(base, 0);
            let manager = e.global::<u32>(0x011d_e45c);
            if e.call(0x0047_c850, &args![manager]).bool() {
                let manager = e.global::<u32>(0x011d_e45c);
                e.call(0x0085_a2e0, &args![manager, package]);
            } else {
                e.vcall(package, 0x10, &args![1u32]);
            }
        } else {
            e.mem.set_u32(base, 0);
        }
    }
    let object = e.mem.u32(base + 4);
    if object != 0 {
        e.vcall(object, 0, &args![1u32]);
        e.mem.set_u32(base + 4, 0);
    }
    let value = x87_quiet(e.global::<f32>(0x0101_2054));
    e.mem.set_f32(base + 0x10, value);
    e.mem.set_u32(base + 0xc, 0xffff_ffff);
    e.mem.set_u32(base + 0x14, 0);
    e.mem.set_u32(base + 8, 0);
}

layout! {
    /// `ActorPackage` (Xbox PDB, 0x18 bytes), the fields `MoveTo` uses; the PC offsets are
    /// the ones the code reads and writes (`+0`, `+4`, `+0xc`, `+0x10`).
    pub struct ActorPackage: 0x18 {
        /// `pPackage` (Xbox PDB): `TESPackage*`.
        0x00 pPackage: u32,
        /// `pData` (Xbox PDB): `ActorPackageData*`.
        0x04 pData: u32,
        /// `iCurrentProcedureIndex` (Xbox PDB).
        0x0C iCurrentProcedureIndex: i32,
        /// `fPackageStartTime` (Xbox PDB).
        0x10 fPackageStartTime: f32,
    }
}

// Translated from 008c6130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorPackage::MoveTo` (Xbox PDB; `thiscall`, `RET 4`): `this` is the package that is
/// moved out, `destination` the one that takes it (the word on the stack). Copies `+0`,
/// `+4`, `+0xc` and the float at `+0x10` (`FLD`/`FSTP`, which quiets a signalling NaN) to
/// `destination`, then resets `this` to the empty values: `+0` and `+4` to 0, `+0xc` to
/// 0xffffffff and the float at `+0x10` to the float at `0x01012054` (read through memory,
/// the same load and store). `+8` and `+0x14` are not touched.
pub fn actor_package_move_to(
    e: &mut Engine,
    this: Ptr<ActorPackage>,
    destination: Ptr<ActorPackage>,
) {
    let package = e.get(this, ActorPackage::pPackage);
    e.set(destination, ActorPackage::pPackage, package);
    let data = e.get(this, ActorPackage::pData);
    e.set(destination, ActorPackage::pData, data);
    let procedure = e.get(this, ActorPackage::iCurrentProcedureIndex);
    e.set(destination, ActorPackage::iCurrentProcedureIndex, procedure);
    let start_time = x87_quiet(e.get(this, ActorPackage::fPackageStartTime));
    e.set(destination, ActorPackage::fPackageStartTime, start_time);
    e.set(this, ActorPackage::pPackage, 0);
    e.set(this, ActorPackage::pData, 0);
    e.set(this, ActorPackage::iCurrentProcedureIndex, -1);
    let empty_start_time = x87_quiet(e.global::<f32>(0x0101_2054));
    e.set(this, ActorPackage::fPackageStartTime, empty_start_time);
}

/// Interlocked decrement of the word at the address (`004019a0`, cdecl;
/// the engine map's name `WaitForSingleObjectEx` belongs to folded code).
const INTERLOCKED_DECREMENT: u32 = 0x0040_19a0;
/// Interlocked increment of the word at the address (`0040b460`, cdecl).
const INTERLOCKED_INCREMENT: u32 = 0x0040_b460;
/// `TESPackage`'s reference count (+0x7C).
const PACKAGE_REF_COUNT: u32 = 0x7c;
/// The `Calendar` singleton (`011de7b8`).
const CALENDAR: u32 = 0x011d_e7b8;
/// `Calendar::GetHour` (`00867da0`, a `float` in `ST0`).
const CALENDAR_GET_HOUR: u32 = 0x0086_7da0;
/// The package's type byte (`0041ca90`, `fallout shared/extradatalist.cpp`
/// by the engine map).
const PACKAGE_TYPE: u32 = 0x0041_ca90;
/// `ActorPackageData::CreateActorPackageData` (`008c6ca0`, cdecl, the
/// package type).
const CREATE_ACTOR_PACKAGE_DATA: u32 = 0x008c_6ca0;
/// `BGSSaveGameBuffer::SaveFormID_ov2` (`00865df0`, the buffer, the form and
/// a flag word).
const SAVE_FORM_ID: u32 = 0x0086_5df0;
/// Writes bytes into the save buffer (`00865e50`, the buffer, the address,
/// the size and a flag word).
const SAVE_DATA: u32 = 0x0086_5e50;
/// A `TESForm` getter (`0084e3a0`) whose result `00469860` checks.
const FORM_VALUE_0084E3A0: u32 = 0x0084_e3a0;
/// A `TESDataHandler` check (`00469860`, the data handler and the value).
const DATA_HANDLER_CHECK_00469860: u32 = 0x0046_9860;
/// The `TESDataHandler` singleton pointer (`011c3f2c`).
const DATA_HANDLER: u32 = 0x011c_3f2c;

// Translated from 008c6030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::DecRefCount` (Xbox PDB): interlocked decrement of the
/// package's reference count (+0x7C); returns the new count.
pub fn tes_package_dec_ref_count(e: &mut Engine, this: Ptr) -> u32 {
    let count = this.addr().wrapping_add(PACKAGE_REF_COUNT);
    e.call(INTERLOCKED_DECREMENT, &args![count]).u32()
}

// Translated from 008c6100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::IncRefCount` (Xbox PDB): interlocked increment of the
/// package's reference count (+0x7C); returns the new count.
pub fn tes_package_inc_ref_count(e: &mut Engine, this: Ptr) -> u32 {
    let count = this.addr().wrapping_add(PACKAGE_REF_COUNT);
    e.call(INTERLOCKED_INCREMENT, &args![count]).u32()
}

// Translated from 008c6080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorPackage::InitFromPackage` (Xbox PDB): nothing when `package` is
/// already this one's; otherwise clears this one, and for a package that is
/// not null takes a reference on it, stores the calendar's hour as the start
/// time, the package, new data for the package's type
/// (`ActorPackageData::CreateActorPackageData`), procedure index 0 and
/// `target` at +8.
pub fn actor_package_init_from_package(e: &mut Engine, this: Ptr, package: Ptr, target: u32) {
    let base = this.addr();
    if package.addr() == e.mem.u32(base) {
        return;
    }
    actorpackage_clear(e, this);
    if package.is_null() {
        return;
    }
    tes_package_inc_ref_count(e, package);
    let hour = e.call(CALENDAR_GET_HOUR, &args![CALENDAR]).f64();
    e.mem.set_u32(base + 0x10, x87_store32(hour));
    e.mem.set_u32(base, package.addr());
    let kind = e.call(PACKAGE_TYPE, &args![package]).u32();
    let data = e.call(CREATE_ACTOR_PACKAGE_DATA, &args![kind]).u32();
    e.mem.set_u32(base + 4, data);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 8, target);
}

// Translated from 008c65d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorPackage::SaveGame` (Xbox PDB): writes the package's form id;
/// for a package, when the data handler check (`00469860` on `0084e3a0`'s
/// value) passes, its type byte (0xFF when the package's virtual `+0x15C`
/// refuses the buffer's version) and, unless 0xFF, the package's own data
/// (virtual `+0x54`); then the data's type byte (its virtual `+8`, 0xFF
/// without data) and, unless 0xFF, the data's own save (virtual `+0xC`), the
/// words at +0xC, +0x10 and +0x14 and the form id of the target at +8.
pub fn actor_package_save_game(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let base = this.addr();
    let version = e.vcall(buffer.addr(), 8, &args![]).u32();
    let package = e.mem.u32(base);
    e.call(SAVE_FORM_ID, &args![buffer, package, 0u32]);
    if e.mem.u32(base) == 0 {
        return;
    }
    e.with_stack(2, |e, bytes| {
        let package_type = bytes.addr();
        let data_type = bytes.addr() + 1;
        let package = e.mem.u32(base);
        let value = e.call(FORM_VALUE_0084E3A0, &args![package]).u32();
        let data_handler = e.global::<u32>(DATA_HANDLER);
        if e.call(DATA_HANDLER_CHECK_00469860, &args![data_handler, value])
            .bool()
        {
            let package = e.mem.u32(base);
            let kind = e.call(PACKAGE_TYPE, &args![package]).u8();
            e.mem.set_u8(package_type, kind);
            let package = e.mem.u32(base);
            if !e.vcall(package, 0x15c, &args![version]).bool() {
                e.mem.set_u8(package_type, 0xff);
            }
            e.call(SAVE_DATA, &args![buffer, package_type, 1u32, 0u32]);
            if e.mem.u8(package_type) as i8 != -1 {
                let package = e.mem.u32(base);
                e.vcall(package, 0x54, &args![buffer]);
            }
        }
        let data = e.mem.u32(base + 4);
        let kind = if data != 0 {
            e.vcall(data, 8, &args![]).u32()
        } else {
            0xffff_ffff
        };
        e.mem.set_u8(data_type, kind as u8);
        e.call(SAVE_DATA, &args![buffer, data_type, 1u32, 0u32]);
        if e.mem.u8(data_type) as i8 != -1 {
            let data = e.mem.u32(base + 4);
            e.vcall(data, 0xc, &args![buffer]);
        }
    });
    e.call(SAVE_DATA, &args![buffer, base + 0xc, 4u32, 0u32]);
    e.call(SAVE_DATA, &args![buffer, base + 0x10, 4u32, 0u32]);
    e.call(SAVE_DATA, &args![buffer, base + 0x14, 4u32, 0u32]);
    let target = e.mem.u32(base + 8);
    e.call(SAVE_FORM_ID, &args![buffer, target, 0u32]);
}

/// The mask the compiler folded to 0 in the `[stack word] & 0` tests of these functions: the
/// blocks it guards never run in this build, but they are translated as the exe has them.
/// A `static` (not a `const`) so the test reads as the exe's `AND reg,0`.
static FOLDED_MASK: u32 = 0;
/// The save/load game object the older save and load functions use (the pointer at
/// `011de45c`, as [`actorpackage_clear`] also uses it).
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
/// Its members (`thiscall` on that object, unit `fallout shared/tessaveloadgame.cpp`):
/// `TESSaveLoadGame::SaveNumericID` (`00857a10`, address and size), the byte writer
/// (`008579b0`, address and size), `TESSaveLoadGame::LoadNumericID` (`00857aa0`, address and
/// size; a byte answer), the byte reader (`008579e0`, address and size), `00857bd0` (one
/// word: adds it to the word at +0x14 of the object) and `0085a240` (an id and a type word;
/// answers an object).
const SAVE_NUMERIC_ID: u32 = 0x0085_7a10;
const SAVE_BYTES: u32 = 0x0085_79b0;
const LOAD_NUMERIC_ID: u32 = 0x0085_7aa0;
const LOAD_BYTES: u32 = 0x0085_79e0;
const ADVANCE_POSITION: u32 = 0x0085_7bd0;
const LOAD_PACKAGE_OF_TYPE: u32 = 0x0085_a240;
/// `BGSLoadGameBuffer::LoadFormID_ov2` (`008648e0`, `thiscall` on the buffer, the address of
/// the id; a byte answer) and the buffer's byte reader (`00864980`, `thiscall`, address and
/// size; unit `bgsloadgamebuffer.cpp`).
const LOAD_FORM_ID: u32 = 0x0086_48e0;
const BUFFER_LOAD_BYTES: u32 = 0x0086_4980;
/// The `BGSSaveLoadGame` object at `011ddf38` with `AddChangedFormID` (`0084aa30`,
/// `thiscall`, two ids) and `0084aa90` (`thiscall`, one word, unit `bgssaveloadgame.cpp`):
/// what [`actor_package_init_load_game_ov2`] maps a package word through.
const CHANGED_FORMS: u32 = 0x011d_df38;
const ADD_CHANGED_FORM_ID: u32 = 0x0084_aa30;
const CHANGED_FORM_LOOKUP: u32 = 0x0084_aa90;
/// `TESPackage::CreatePackage` (`00670b90`, `cdecl`, the type).
const CREATE_PACKAGE: u32 = 0x0067_0b90;
/// The form lookup by id (`004839c0`, `cdecl`, unit `tesform.cpp`) and `__RTDynamicCast`
/// (`00ec43fb`, `cdecl`: object, offset, source and target type descriptors, reference
/// flag) with the type descriptors of `TESForm` (`01183028`), `TESPackage` (`011846a0`) and
/// `TESObjectREFR` (`011841cc`).
const FORM_LOOKUP: u32 = 0x0048_39c0;
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_PACKAGE: u32 = 0x0118_46a0;
const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
/// `009611e0` (`thiscall` on a package; unit `playercharacter.cpp` by the engine map; its
/// answer goes to `Procedure::GetProcedureCount`), `Procedure::GetProcedureCount`
/// (`0096a820`, `cdecl`) and `TESPackage::CalculateProcedureType` (`006777b0`, `thiscall`,
/// one word).
const PACKAGE_PROCEDURE: u32 = 0x0096_11e0;
const PROCEDURE_COUNT: u32 = 0x0096_a820;
const CALCULATE_PROCEDURE_TYPE: u32 = 0x0067_77b0;
/// `TESObjectREFR::SetTargeted` (`00564db0`, `thiscall`, one word).
const SET_TARGETED: u32 = 0x0056_4db0;
/// `0041fd00` (`thiscall`, one word; the engine map names it
/// `NonActorMagicCaster::SetCurrentSpell`, folded code): stores the word at +0xc.
const SET_WORD_C: u32 = 0x0041_fd00;
/// The AI log (`005b5e40`, `cdecl`, a format and its arguments) and `SaveGameWarning`
/// (`00845cc0`, `cdecl`, likewise).
const AI_LOG: u32 = 0x005b_5e40;
const SAVE_GAME_WARNING: u32 = 0x0084_5cc0;
/// `"AI: Couldn't find a plugin package while loading and will try to choose a new one."`,
/// `"AI: Couldn't find package %08X while loading and will try to choose a new one."` and
/// `"Package '%s' (%08X) changed type from %s to %s, so the actor package data was
/// replaced"`.
const FORMAT_NO_PLUGIN_PACKAGE: u32 = 0x0108_5458;
const FORMAT_NO_PACKAGE: u32 = 0x0108_54b0;
const FORMAT_TYPE_CHANGED: u32 = 0x0108_5500;
/// The package type names, one string pointer per type (`0119bcb0`, `.data`).
const PACKAGE_TYPE_NAMES: u32 = 0x0119_bcb0;

/// The form for `id` cast to the type at `target` (`004839c0`, then `__RTDynamicCast` from
/// `TESForm`).
fn cast_form(e: &mut Engine, id: u32, target: u32) -> u32 {
    let form = e.call(FORM_LOOKUP, &args![id]).u32();
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, TYPE_TES_FORM, target, 0u32],
    )
    .u32()
}

/// Whether the data handler accepts the value (`00469860` on the data handler).
fn data_handler_check(e: &mut Engine, value: u32) -> bool {
    let data_handler = e.global::<u32>(DATA_HANDLER);
    e.call(DATA_HANDLER_CHECK_00469860, &args![data_handler, value])
        .bool()
}

/// Lowers the procedure index (+0xc) below the package's procedure count when it has reached
/// it (signed).
fn clamp_procedure_index(e: &mut Engine, base: u32) {
    let package = e.mem.u32(base);
    let procedure = e.call(PACKAGE_PROCEDURE, &args![package]).u32();
    let count = e.call(PROCEDURE_COUNT, &args![procedure]).u32() as i32;
    if e.mem.u32(base + 0xc) as i32 >= count {
        e.mem.set_u32(base + 0xc, count.wrapping_sub(1) as u32);
    }
}

// Translated from 008c61a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word; `RET 4`): the size of what
/// [`fn_008c6250`] writes (the exe loads it into `AX` over the same value in `EAX`): 8, or 12 with a package (whose form id `0084e3a0` reads).
/// The block under the folded mask (the data handler check of that id, the package's type
/// byte `0041ca90`, one more byte and the package's virtual `+0x14c`) never runs; it is
/// translated as the exe has it.
pub fn fn_008c61a0(e: &mut Engine, this: Ptr, word: u32) -> u32 {
    let base = this.addr();
    let mut size = 4u32;
    if e.mem.u32(base) != 0 {
        let package = e.mem.u32(base);
        let id = e.call(FORM_VALUE_0084E3A0, &args![package]).u32();
        if word & FOLDED_MASK != 0 && data_handler_check(e, id) {
            let package = e.mem.u32(base);
            e.call(PACKAGE_TYPE, &args![package]);
            size += 1;
            if word & FOLDED_MASK == 0 {
                let package = e.mem.u32(base);
                let extra = e.vcall(package, 0x14c, &args![]).u16();
                size = size.wrapping_add(u32::from(extra));
            }
        }
        size += 4;
    }
    size += 4;
    size
}

// Translated from 008c6250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word; `RET 4`): writes the package through the
/// save/load game object at `011de45c`: the package's form id (0 without a package) as a
/// numeric id (4 bytes, `00857a10`), then with an id the procedure index (+0xc, 4 bytes,
/// `008579b0`), and always the start time (+0x10, 4 bytes). The block under the folded mask
/// (the data handler check, the type byte and the package's virtual `+0x150`) never runs; it
/// is translated as the exe has it.
pub fn fn_008c6250(e: &mut Engine, this: Ptr, word: u32) {
    let base = this.addr();
    e.with_stack(8, |e, locals| {
        let id_at = locals.addr();
        let kind_at = locals.addr() + 4;
        e.mem.set_u32(id_at, 0);
        if e.mem.u32(base) != 0 {
            let package = e.mem.u32(base);
            let id = e.call(FORM_VALUE_0084E3A0, &args![package]).u32();
            e.mem.set_u32(id_at, id);
        }
        let save_load = e.global::<u32>(SAVE_LOAD_GAME);
        e.call(SAVE_NUMERIC_ID, &args![save_load, id_at, 4u32]);
        if e.mem.u32(id_at) == 0 {
            return;
        }
        let id = e.mem.u32(id_at);
        if word & FOLDED_MASK != 0 && data_handler_check(e, id) {
            let package = e.mem.u32(base);
            let kind = e.call(PACKAGE_TYPE, &args![package]).u8();
            e.mem.set_u8(kind_at, kind);
            let save_load = e.global::<u32>(SAVE_LOAD_GAME);
            e.call(SAVE_BYTES, &args![save_load, kind_at, 1u32]);
            if word & FOLDED_MASK == 0 {
                let package = e.mem.u32(base);
                e.vcall(package, 0x150, &args![]);
            }
        }
        let save_load = e.global::<u32>(SAVE_LOAD_GAME);
        e.call(SAVE_BYTES, &args![save_load, base + 0xc, 4u32]);
    });
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(SAVE_BYTES, &args![save_load, base + 0x10, 4u32]);
}

// Translated from 008c6320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorPackage::LoadGame` (Xbox PDB; `thiscall`, two words, the second not read; `RET 8`):
/// the read counterpart of [`fn_008c6250`] through the save/load game object at `011de45c`.
/// Reads the numeric id (`00857aa0`, its byte answer kept), clears the package
/// ([`actorpackage_clear`]); with an id, takes the package (`004839c0` cast to `TESPackage`)
/// through [`actor_package_init_from_package`], has it work out its procedure type for the
/// target (`006777b0` with +8) or, without one, marks the load as failed, and reads the
/// procedure index (+0xc, 4 bytes). Without an id but a set answer it moves the object's
/// position on by 4 (`00857bd0`), sets +0xc to 0 (`0041fd00`) and marks the load as failed.
/// A failed load logs (`005b5e40`, the id when there is one) and sets +0xc to -1. The start
/// time (+0x10, 4 bytes) is read last. The block under the folded mask (a type byte and
/// `0085a240`'s package with its virtual `+0x154`) never runs; it is translated as the exe
/// has it.
pub fn actor_package_load_game(e: &mut Engine, this: Ptr, word: u32, _unused_2: u32) {
    let base = this.addr();
    e.with_stack(8, |e, locals| {
        let id_at = locals.addr();
        let kind_at = locals.addr() + 4;
        let save_load = e.global::<u32>(SAVE_LOAD_GAME);
        let answer = e.call(LOAD_NUMERIC_ID, &args![save_load, id_at, 4u32]).u8();
        actorpackage_clear(e, this);
        let mut failed = false;
        let id = e.mem.u32(id_at);
        if id != 0 {
            if word & FOLDED_MASK != 0 && data_handler_check(e, id) {
                let save_load = e.global::<u32>(SAVE_LOAD_GAME);
                e.call(LOAD_BYTES, &args![save_load, kind_at, 1u32]);
                if word & FOLDED_MASK == 0 {
                    let kind = u32::from(e.mem.u8(kind_at));
                    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
                    let package = e
                        .call(LOAD_PACKAGE_OF_TYPE, &args![save_load, id, kind])
                        .u32();
                    e.vcall(package, 0x154, &args![]);
                    actor_package_init_from_package(e, this, Ptr::new(package), 0);
                    if e.call(PACKAGE_PROCEDURE, &args![package]).u32() == 0xffff_ffff {
                        let target = e.mem.u32(base + 8);
                        e.call(CALCULATE_PROCEDURE_TYPE, &args![package, target]);
                    }
                } else {
                    actor_package_init_from_package(e, this, Ptr::new(id), 0);
                }
            } else {
                let package = cast_form(e, id, TYPE_TES_PACKAGE);
                actor_package_init_from_package(e, this, Ptr::new(package), 0);
                if e.mem.u32(base) != 0 {
                    let target = e.mem.u32(base + 8);
                    let package = e.mem.u32(base);
                    e.call(CALCULATE_PROCEDURE_TYPE, &args![package, target]);
                } else {
                    failed = true;
                }
            }
            let save_load = e.global::<u32>(SAVE_LOAD_GAME);
            e.call(LOAD_BYTES, &args![save_load, base + 0xc, 4u32]);
        } else if answer != 0 {
            let save_load = e.global::<u32>(SAVE_LOAD_GAME);
            e.call(ADVANCE_POSITION, &args![save_load, 4u32]);
            e.call(SET_WORD_C, &args![base, 0u32]);
            failed = true;
        }
        if failed {
            let id = e.mem.u32(id_at);
            if id != 0 {
                e.call(AI_LOG, &args![FORMAT_NO_PACKAGE, id]);
            } else {
                e.call(AI_LOG, &args![FORMAT_NO_PLUGIN_PACKAGE]);
            }
            e.call(SET_WORD_C, &args![base, 0xffff_ffffu32]);
        }
    });
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(LOAD_BYTES, &args![save_load, base + 0x10, 4u32]);
}

// Translated from 008c64d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorPackage::InitLoadGame` (Xbox PDB; `thiscall`, two words, the second not read;
/// `RET 8`): with a package, lowers the procedure index (+0xc) to the package's procedure
/// count minus one when it has reached the count (`009611e0`, `Procedure::GetProcedureCount`,
/// signed). The blocks under the folded mask (taking the package again through `004839c0`
/// and [`actor_package_init_from_package`], or the package's virtual `+0x158` after the
/// data handler check) never run; they are translated as the exe has them.
pub fn actor_package_init_load_game(e: &mut Engine, this: Ptr, word: u32, _unused_2: u32) {
    let base = this.addr();
    if e.mem.u32(base) == 0 {
        return;
    }
    if word & FOLDED_MASK != 0 {
        if word & FOLDED_MASK != 0 {
            let package = e.mem.u32(base);
            let cast = cast_form(e, package, TYPE_TES_PACKAGE);
            actor_package_init_from_package(e, this, Ptr::new(cast), 0);
        } else {
            let package = e.mem.u32(base);
            let id = e.call(FORM_VALUE_0084E3A0, &args![package]).u32();
            if data_handler_check(e, id) {
                let package = e.mem.u32(base);
                e.vcall(package, 0x158, &args![]);
            }
        }
    }
    if e.mem.u32(base) != 0 {
        clamp_procedure_index(e, base);
    }
}

// Translated from 008c65a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word; `RET 4`): the only code is under the
/// folded mask (with a package, [`actorpackage_clear`]), so it never does anything in this
/// build; translated as the exe has it.
pub fn fn_008c65a0(e: &mut Engine, this: Ptr, word: u32) {
    if word & FOLDED_MASK != 0 && e.mem.u32(this.addr()) != 0 {
        actorpackage_clear(e, this);
    }
}

// Translated from 008c6730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorPackage::LoadGame_ov2` (Xbox PDB; `thiscall`, the load buffer; `RET 4`): reads the
/// package's form id (`008648e0`, its byte answer kept); with neither an id nor an answer
/// it does nothing. When the data handler accepts the id a type byte follows: -1 keeps the
/// id itself as the package and marks it for [`fn_008c6920`]; another type makes a package
/// (`TESPackage::CreatePackage`), records the id change (`AddChangedFormID` with the new
/// form id) and loads it (its virtual `+0x5c`). Otherwise the package is the id's form cast
/// to `TESPackage` (null for id 0). Then the data's type byte: -1 deletes the data (its
/// virtual `+0` with 1) and clears +4, another makes new data
/// (`ActorPackageData::CreateActorPackageData`) that loads itself (its virtual `+0x10`).
/// Then the words at +0xc, +0x10 and +0x14 (4 bytes each) and the target's form id (+8).
/// A marked package sets bit 0 of +0x14 ([`fn_008c6920`]). A set answer finally clears the
/// package ([`actorpackage_clear`]).
pub fn actor_package_load_game_ov2(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let base = this.addr();
    e.with_stack(8, |e, locals| {
        let id_at = locals.addr();
        let kind_at = locals.addr() + 4;
        let data_kind_at = locals.addr() + 5;
        e.mem.set_u32(id_at, 0);
        let answer = e.call(LOAD_FORM_ID, &args![buffer, id_at]).u8();
        if e.mem.u32(id_at) == 0 && answer == 0 {
            return;
        }
        let mut marked = false;
        let id = e.mem.u32(id_at);
        if data_handler_check(e, id) {
            e.mem.set_u8(kind_at, 0);
            e.call(BUFFER_LOAD_BYTES, &args![buffer, kind_at, 1u32]);
            let kind = e.mem.u8(kind_at) as i8;
            if kind != -1 {
                let package = e.call(CREATE_PACKAGE, &args![i32::from(kind)]).u32();
                e.mem.set_u32(base, package);
                let new_id = e.call(FORM_VALUE_0084E3A0, &args![package]).u32();
                let changed = e.global::<u32>(CHANGED_FORMS);
                let id = e.mem.u32(id_at);
                e.call(ADD_CHANGED_FORM_ID, &args![changed, id, new_id]);
                let package = e.mem.u32(base);
                e.vcall(package, 0x5c, &args![buffer]);
            } else {
                let id = e.mem.u32(id_at);
                e.mem.set_u32(base, id);
                marked = true;
            }
        } else {
            let id = e.mem.u32(id_at);
            let package = if id == 0 {
                0
            } else {
                cast_form(e, id, TYPE_TES_PACKAGE)
            };
            e.mem.set_u32(base, package);
        }
        e.mem.set_u8(data_kind_at, 0xff);
        e.call(BUFFER_LOAD_BYTES, &args![buffer, data_kind_at, 1u32]);
        let data_kind = e.mem.u8(data_kind_at) as i8;
        if data_kind != -1 {
            let data = e
                .call(CREATE_ACTOR_PACKAGE_DATA, &args![i32::from(data_kind)])
                .u32();
            e.mem.set_u32(base + 4, data);
            let data = e.mem.u32(base + 4);
            e.vcall(data, 0x10, &args![buffer]);
        } else {
            let data = e.mem.u32(base + 4);
            if data != 0 {
                e.vcall(data, 0, &args![1u32]);
            }
            e.mem.set_u32(base + 4, 0);
        }
        for offset in [0xcu32, 0x10, 0x14] {
            e.call(BUFFER_LOAD_BYTES, &args![buffer, base + offset, 4u32]);
        }
        e.call(LOAD_FORM_ID, &args![buffer, base + 8]);
        if marked {
            fn_008c6920(e, this, 0, 1);
        }
        if answer != 0 {
            actorpackage_clear(e, this);
        }
    });
}

// Translated from 008c6920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, a bit number and a byte; `RET 8`): sets (byte not
/// 0) or clears that bit (its low 5 bits, as `SHL` counts) of the word at +0x14.
pub fn fn_008c6920(e: &mut Engine, this: Ptr, bit: u32, set: u8) {
    let base = this.addr();
    let mask = 1u32 << (bit & 0x1f);
    let flags = e.mem.u32(base + 0x14);
    if set != 0 {
        e.mem.set_u32(base + 0x14, mask | flags);
    } else {
        e.mem.set_u32(base + 0x14, !mask & flags);
    }
}

// Translated from 008c6970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorPackage::InitLoadGame_ov2` (Xbox PDB; `thiscall`, the load buffer; `RET 4`): a
/// package marked by bit 0 of +0x14 ([`fn_008c6b00`]) is looked up again (`0084aa90` of the
/// `BGSSaveLoadGame` at `011ddf38`, then cast to `TESPackage`; null for 0); an unmarked one
/// accepted by the data handler gets its own `InitLoadGame` (virtual `+0x64`). With a
/// package: it works out its procedure type when it has none yet (`009611e0` answering -1,
/// `006777b0` with 0), takes a reference ([`tes_package_inc_ref_count`]) and the procedure
/// index is clamped below its procedure count. The data gets its virtual `+0x14`; the target
/// (+8) is resolved to a `TESObjectREFR` (null for 0) and, when there is one, marked as
/// targeted (`00564db0` with 1).
pub fn actor_package_init_load_game_ov2(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let base = this.addr();
    if fn_008c6b00(e, this, 0) as u8 != 0 {
        let changed = e.global::<u32>(CHANGED_FORMS);
        let package = e.mem.u32(base);
        let id = e.call(CHANGED_FORM_LOOKUP, &args![changed, package]).u32();
        let package = if id == 0 {
            0
        } else {
            cast_form(e, id, TYPE_TES_PACKAGE)
        };
        e.mem.set_u32(base, package);
    } else if e.mem.u32(base) != 0 {
        let package = e.mem.u32(base);
        let id = e.call(FORM_VALUE_0084E3A0, &args![package]).u32();
        if data_handler_check(e, id) {
            let package = e.mem.u32(base);
            e.vcall(package, 0x64, &args![buffer]);
        }
    }
    if e.mem.u32(base) != 0 {
        let package = e.mem.u32(base);
        if e.call(PACKAGE_PROCEDURE, &args![package]).u32() == 0xffff_ffff {
            let package = e.mem.u32(base);
            e.call(CALCULATE_PROCEDURE_TYPE, &args![package, 0u32]);
        }
        let package = e.mem.u32(base);
        tes_package_inc_ref_count(e, Ptr::new(package));
        clamp_procedure_index(e, base);
    }
    if e.mem.u32(base + 4) != 0 {
        let data = e.mem.u32(base + 4);
        e.vcall(data, 0x14, &args![buffer]);
    }
    let target = e.mem.u32(base + 8);
    let target = if target == 0 {
        0
    } else {
        cast_form(e, target, TYPE_TES_OBJECT_REFR)
    };
    e.mem.set_u32(base + 8, target);
    if e.mem.u32(base + 8) != 0 {
        let target = e.mem.u32(base + 8);
        e.call(SET_TARGETED, &args![target, 1u32]);
    }
}

// Translated from 008c6b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, a bit number; `RET 4`): whether that bit (its low 5
/// bits, as `SHL` counts) of the word at +0x14 is set, in `AL` (`SETNZ`); the upper bytes of
/// `EAX` keep the masked word's.
pub fn fn_008c6b00(e: &mut Engine, this: Ptr, bit: u32) -> u32 {
    let masked = (1u32 << (bit & 0x1f)) & e.mem.u32(this.addr() + 0x14);
    (masked & 0xffff_ff00) | u32::from(masked != 0)
}

// Translated from 008c6b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorPackage::FinishLoadGame` (Xbox PDB; `thiscall`, the load buffer; `RET 4`): a
/// package marked by bit 0 of +0x14 has the mark cleared ([`fn_008c6920`]); an unmarked one
/// accepted by the data handler gets its own `FinishLoadGame` (virtual `+0x84`). With data
/// and a package whose type (`0041ca90`) differs from the data's (its virtual `+8`), a
/// warning is written (`SaveGameWarning` with the package's name, its virtual `+0x130`, its
/// form id, and the old and new type names from the table at `0119bcb0`), the data deleted
/// (its virtual `+0` with 1) and replaced with new data for the package's type
/// (`ActorPackageData::CreateActorPackageData`).
pub fn actor_package_finish_load_game(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let base = this.addr();
    if fn_008c6b00(e, this, 0) as u8 != 0 {
        fn_008c6920(e, this, 0, 0);
    } else if e.mem.u32(base) != 0 {
        let package = e.mem.u32(base);
        let id = e.call(FORM_VALUE_0084E3A0, &args![package]).u32();
        if data_handler_check(e, id) {
            let package = e.mem.u32(base);
            e.vcall(package, 0x84, &args![buffer]);
        }
    }
    if e.mem.u32(base + 4) == 0 || e.mem.u32(base) == 0 {
        return;
    }
    let package = e.mem.u32(base);
    let package_kind = e.call(PACKAGE_TYPE, &args![package]).u32();
    let data = e.mem.u32(base + 4);
    let data_kind = e.vcall(data, 8, &args![]).u32();
    if package_kind == data_kind {
        return;
    }
    let package = e.mem.u32(base);
    let kind = e.call(PACKAGE_TYPE, &args![package]).u32();
    let new_name = e
        .mem
        .u32(PACKAGE_TYPE_NAMES.wrapping_add(kind.wrapping_mul(4)));
    let data = e.mem.u32(base + 4);
    let kind = e.vcall(data, 8, &args![]).u32();
    let old_name = e
        .mem
        .u32(PACKAGE_TYPE_NAMES.wrapping_add(kind.wrapping_mul(4)));
    let package = e.mem.u32(base);
    let id = e.call(FORM_VALUE_0084E3A0, &args![package]).u32();
    let package = e.mem.u32(base);
    let name = e.vcall(package, 0x130, &args![]).u32();
    e.call(
        SAVE_GAME_WARNING,
        &args![FORMAT_TYPE_CHANGED, name, id, old_name, new_name],
    );
    let data = e.mem.u32(base + 4);
    if data != 0 {
        e.vcall(data, 0, &args![1u32]);
    }
    let package = e.mem.u32(base);
    let kind = e.call(PACKAGE_TYPE, &args![package]).u32();
    let data = e.call(CREATE_ACTOR_PACKAGE_DATA, &args![kind]).u32();
    e.mem.set_u32(base + 4, data);
}

// Translated from 008c6c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorPackage::Revert` (Xbox PDB; `thiscall`, one word it does not read; `RET 4`):
/// [`actorpackage_clear`].
pub fn actor_package_revert(e: &mut Engine, this: Ptr, _unused_1: u32) {
    actorpackage_clear(e, this);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x008c_6130,
            actor_package_move_to(Ptr<ActorPackage>, Ptr<ActorPackage>)
        ),
        entry!(0x008c5ef0, actorpackage_clear(Ptr)),
        entry!(0x008c6060, fn_008c6060(Ptr)),
        entry!(0x008c6030, tes_package_dec_ref_count(Ptr) -> u32),
        entry!(0x008c6100, tes_package_inc_ref_count(Ptr) -> u32),
        entry!(0x008c6080, actor_package_init_from_package(Ptr, Ptr, u32)),
        entry!(0x008c65d0, actor_package_save_game(Ptr, Ptr)),
        entry!(0x008c_61a0, fn_008c61a0(Ptr, u32) -> u32),
        entry!(0x008c_6250, fn_008c6250(Ptr, u32)),
        entry!(0x008c_6320, actor_package_load_game(Ptr, u32, u32)),
        entry!(0x008c_64d0, actor_package_init_load_game(Ptr, u32, u32)),
        entry!(0x008c_65a0, fn_008c65a0(Ptr, u32)),
        entry!(0x008c_6730, actor_package_load_game_ov2(Ptr, Ptr)),
        entry!(0x008c_6920, fn_008c6920(Ptr, u32, u8)),
        entry!(0x008c_6970, actor_package_init_load_game_ov2(Ptr, Ptr)),
        entry!(0x008c_6b00, fn_008c6b00(Ptr, u32) -> u32),
        entry!(0x008c_6b20, actor_package_finish_load_game(Ptr, Ptr)),
        entry!(0x008c_6c80, actor_package_revert(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clears_only_the_word_at_4() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0xc);
        for offset in [0, 4, 8] {
            e.mem.set_u32(object + offset, 0xdead_0000 + offset);
        }
        e.call(0x008c_6060, &[object]);
        assert_eq!(e.mem.u32(object), 0xdead_0000);
        assert_eq!(e.mem.u32(object + 4), 0);
        assert_eq!(e.mem.u32(object + 8), 0xdead_0008);
    }
}

#[cfg(test)]
mod movement_path_tests {
    use super::*;

    const STATE: u32 = 0x011d_ea3c;
    const MANAGER: u32 = 0x011d_e45c;
    const OWNER: u32 = 0x0e00_0000;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    /// An engine with an owner object (the pending package of the owner at `+0x208` is
    /// `pending`), a package whose slot `+0x10` and a sub-object whose slot `+0` are doubles that
    /// log, and doubles for the callees; `count` is what `008c6030` answers, `agrees` what
    /// `00674d40` answers and `manager` what `0047c850` answers.
    fn setup(
        pending: u32,
        count: u32,
        agrees: bool,
        manager: bool,
    ) -> (Engine, Log, u32, u32, u32) {
        let mut e = Engine::new();
        let log: Log = Log::default();
        let record = e.mem.alloc(0x18);
        let package = e.mem.alloc(8);
        let object = e.mem.alloc(8);
        e.map(OWNER, 0x300);
        for global in [STATE, MANAGER, 0x0101_2054] {
            e.map(global, 4);
        }
        e.mem.set_u32(OWNER + 0x208, pending);
        e.set_global(STATE, OWNER);
        e.set_global(MANAGER, 0x0f00_0000u32);
        e.set_global(0x0101_2054, 0x3f80_0000u32);
        e.put_vtable(0x0090_1000, &[0x0090_0100, 0, 0, 0, 0x0090_0110]);
        e.mem.set_u32(package, 0x0090_1000);
        e.mem.set_u32(object, 0x0090_1000);
        for (address, eax) in [
            (0x008c_6030, count),
            (0x0067_4d40, u32::from(agrees)),
            (0x0047_c850, u32::from(manager)),
            (0x0085_a2e0, 0),
            (0x0090_0100, 0),
            (0x0090_0110, 0),
        ] {
            let log = log.clone();
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret {
                    eax,
                    ..Ret::default()
                }
            });
        }
        (e, log, record, package, object)
    }

    #[test]
    fn an_empty_record_is_only_reset() {
        let (mut e, log, record, _, _) = setup(0, 0, true, true);
        for offset in [0, 4, 8, 0x14] {
            e.mem.set_u32(record + offset, 0);
        }
        e.mem.set_u32(record + 8, 5);
        e.call(0x008c_5ef0, &[record]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(record + 8), 0);
        assert_eq!(e.mem.u32(record + 0xc), 0xffff_ffff);
        assert_eq!(e.mem.u32(record + 0x10), 0x3f80_0000);
        assert_eq!(e.mem.u32(record + 0x14), 0);
    }

    #[test]
    fn a_package_that_is_still_counted_is_only_let_go() {
        let (mut e, log, record, package, _) = setup(0x1234, 2, true, true);
        e.mem.set_u32(record, package);
        e.call(0x008c_5ef0, &[record]);
        assert_eq!(*log.borrow(), [(0x008c_6030, vec![package])]);
        assert_eq!(e.mem.u32(record), 0);
    }

    #[test]
    fn the_last_reference_hands_the_package_to_the_manager() {
        let (mut e, log, record, package, _) = setup(0, 0, true, true);
        e.mem.set_u32(OWNER + 0x208, package);
        e.mem.set_u32(record, package);
        e.call(0x008c_5ef0, &[record]);
        assert_eq!(
            *log.borrow(),
            [
                (0x008c_6030, vec![package]),
                (0x0067_4d40, vec![package]),
                (0x0047_c850, vec![0x0f00_0000]),
                (0x0085_a2e0, vec![0x0f00_0000, package]),
            ]
        );
        assert_eq!(e.mem.u32(record), 0);
        assert_eq!(e.mem.u32(OWNER + 0x208), 0);
    }

    #[test]
    fn without_a_manager_the_package_is_destroyed() {
        let (mut e, log, record, package, _) = setup(0x99, 0, true, false);
        e.mem.set_u32(record, package);
        e.call(0x008c_5ef0, &[record]);
        assert_eq!(
            log.borrow().last().unwrap(),
            &(0x0090_0110, vec![package, 1])
        );
        assert_eq!(e.mem.u32(OWNER + 0x208), 0x99);
        assert_eq!(e.mem.u32(record), 0);
    }

    #[test]
    fn a_package_that_disagrees_is_only_let_go() {
        let (mut e, log, record, package, _) = setup(0, 0, false, true);
        e.mem.set_u32(record, package);
        e.call(0x008c_5ef0, &[record]);
        assert_eq!(log.borrow().len(), 2);
        assert_eq!(e.mem.u32(record), 0);
    }

    #[test]
    fn the_sub_object_is_destroyed() {
        let (mut e, log, record, _, object) = setup(0, 0, true, true);
        e.mem.set_u32(record + 4, object);
        e.call(0x008c_5ef0, &[record]);
        assert_eq!(*log.borrow(), [(0x0090_0100, vec![object, 1])]);
        assert_eq!(e.mem.u32(record + 4), 0);
    }
}

#[cfg(test)]
mod move_to_tests {
    use super::*;

    const EMPTY_START_TIME: u32 = 0x0101_2054;

    fn fixture() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(EMPTY_START_TIME, 4);
        e.set_global(EMPTY_START_TIME, (-1.0f32).to_bits());
        let from = e.mem.alloc(0x18);
        let to = e.mem.alloc(0x18);
        (e, from, to)
    }

    #[test]
    fn moves_the_package_and_resets_the_source() {
        let (mut e, from, to) = fixture();
        e.mem.set_u32(from, 0x1111);
        e.mem.set_u32(from + 4, 0x2222);
        e.mem.set_u32(from + 8, 0x3333);
        e.mem.set_u32(from + 0xc, 5);
        e.mem.set_u32(from + 0x10, 12.5f32.to_bits());
        e.mem.set_u32(from + 0x14, 0x4444);
        e.mem.set_u32(to + 8, 0x7777);
        e.mem.set_u32(to + 0x14, 0x8888);
        e.call(0x008c_6130, &args![from, to]);
        assert_eq!(e.mem.u32(to), 0x1111);
        assert_eq!(e.mem.u32(to + 4), 0x2222);
        assert_eq!(e.mem.u32(to + 8), 0x7777);
        assert_eq!(e.mem.u32(to + 0xc), 5);
        assert_eq!(f32::from_bits(e.mem.u32(to + 0x10)), 12.5);
        assert_eq!(e.mem.u32(to + 0x14), 0x8888);
        assert_eq!(e.mem.u32(from), 0);
        assert_eq!(e.mem.u32(from + 4), 0);
        assert_eq!(e.mem.u32(from + 8), 0x3333);
        assert_eq!(e.mem.u32(from + 0xc), 0xffff_ffff);
        assert_eq!(e.mem.u32(from + 0x10), (-1.0f32).to_bits());
        assert_eq!(e.mem.u32(from + 0x14), 0x4444);
    }

    #[test]
    fn quiets_signalling_nans_in_the_start_time() {
        let (mut e, from, to) = fixture();
        e.set_global(EMPTY_START_TIME, 0x7fa0_0003u32);
        e.mem.set_u32(from + 0x10, 0xffa0_0001);
        e.call(0x008c_6130, &args![from, to]);
        assert_eq!(e.mem.u32(to + 0x10), 0xffe0_0001);
        assert_eq!(e.mem.u32(from + 0x10), 0x7fe0_0003);
    }

    fn calls_to_008c(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn package_ref_counts_go_through_the_interlocked_helpers() {
        let mut e = Engine::new();
        e.register(INTERLOCKED_DECREMENT, |_, _| 4u32.into_ret());
        e.register(INTERLOCKED_INCREMENT, |_, _| 6u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008c_6030, &args![0x1000u32]).u32(), 4);
        assert_eq!(e.call(0x008c_6100, &args![0x1000u32]).u32(), 6);
        assert_eq!(calls_to_008c(&e, INTERLOCKED_DECREMENT), vec![vec![0x107c]]);
        assert_eq!(calls_to_008c(&e, INTERLOCKED_INCREMENT), vec![vec![0x107c]]);
    }

    fn init_doubles(e: &mut Engine) {
        e.map(0x011d_ea3c, 4);
        let holder = e.mem.alloc(0x210);
        e.set_global(0x011d_ea3c, holder);
        e.map(0x0101_2054, 4);
        e.set_global(0x0101_2054, -1.0f32);
        e.register(INTERLOCKED_DECREMENT, |_, _| 1u32.into_ret());
        e.register(INTERLOCKED_INCREMENT, |_, _| 2u32.into_ret());
        e.register(CALENDAR_GET_HOUR, |_, _| 7.5f32.into_ret());
        e.register(PACKAGE_TYPE, |_, _| 0x12u32.into_ret());
        e.register(CREATE_ACTOR_PACKAGE_DATA, |_, _| 0x5150u32.into_ret());
    }

    #[test]
    fn init_from_package_takes_the_new_package() {
        let mut e = Engine::new();
        init_doubles(&mut e);
        let this = e.mem.alloc(0x18);
        let package = e.mem.alloc(0x80);
        e.call_log = Some(vec![]);
        e.call(0x008c_6080, &args![this, package, 0x77u32]);
        assert_eq!(e.mem.u32(this), package);
        assert_eq!(e.mem.u32(this + 4), 0x5150);
        assert_eq!(e.mem.u32(this + 8), 0x77);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.f32(this + 0x10), 7.5);
        assert_eq!(
            calls_to_008c(&e, INTERLOCKED_INCREMENT),
            vec![vec![package + 0x7c]]
        );
        assert_eq!(
            calls_to_008c(&e, CREATE_ACTOR_PACKAGE_DATA),
            vec![vec![0x12]]
        );
        // The same package again: nothing.
        e.call_log = Some(vec![]);
        e.call(0x008c_6080, &args![this, package, 0x99u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        assert_eq!(e.mem.u32(this + 8), 0x77);
    }

    #[test]
    fn init_from_a_null_package_only_clears() {
        let mut e = Engine::new();
        init_doubles(&mut e);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 8, 0x44);
        // With +0 already null a null package returns at once: start from
        // an old package.
        let old = e.mem.alloc(0x80);
        e.mem.set_u32(this, old);
        e.register(0x0067_4d40, |_, _| false.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008c_6080, &args![this, 0u32, 0x55u32]);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert!(calls_to_008c(&e, INTERLOCKED_INCREMENT).is_empty());
        assert!(calls_to_008c(&e, CALENDAR_GET_HOUR).is_empty());
    }

    /// A save buffer whose virtual `+8` answers the version, and doubles of
    /// the save helpers that record the bytes written.
    fn save_setup(e: &mut Engine, check: bool) -> u32 {
        e.put_vtable(0x0310_0000, &[0, 0, 0x0310_1000]);
        e.register(0x0310_1000, |_, _| 9u32.into_ret());
        let buffer = e.mem.alloc(4);
        e.mem.set_u32(buffer, 0x0310_0000);
        e.register(SAVE_FORM_ID, |_, _| Ret::default());
        e.register(SAVE_DATA, |_, _| Ret::default());
        e.register(FORM_VALUE_0084E3A0, |_, _| 0x33u32.into_ret());
        e.map(DATA_HANDLER, 4);
        e.set_global(DATA_HANDLER, 0x0400_0000u32);
        if check {
            e.register(DATA_HANDLER_CHECK_00469860, |_, _| true.into_ret());
        } else {
            e.register(DATA_HANDLER_CHECK_00469860, |_, _| false.into_ret());
        }
        e.register(PACKAGE_TYPE, |_, _| 0x05u32.into_ret());
        buffer
    }

    fn saved_bytes(e: &Engine) -> Vec<(u32, u32)> {
        calls_to_008c(e, SAVE_DATA)
            .iter()
            .map(|w| {
                (
                    w[2],
                    if w[2] == 1 {
                        u32::from(e.mem.u8(w[1]))
                    } else {
                        w[1]
                    },
                )
            })
            .collect()
    }

    #[test]
    fn save_game_without_a_package_writes_only_its_form_id() {
        let mut e = Engine::new();
        let buffer = save_setup(&mut e, true);
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        e.call(0x008c_65d0, &args![this, buffer]);
        assert_eq!(calls_to_008c(&e, SAVE_FORM_ID), vec![vec![buffer, 0, 0]]);
        assert!(calls_to_008c(&e, SAVE_DATA).is_empty());
    }

    #[test]
    fn save_game_writes_the_package_and_its_data() {
        let mut e = Engine::new();
        let buffer = save_setup(&mut e, true);
        // Package vtable: +0x54 saves, +0x15C accepts the version.
        let mut slots = vec![0u32; 0x58];
        slots[0x54 / 4] = 0x0310_2054;
        slots[0x15c / 4] = 0x0310_215c;
        e.put_vtable(0x0320_0000, &slots);
        e.register(0x0310_2054, |_, _| Ret::default());
        e.register(0x0310_215c, |_, a| (a[1] == 9).into_ret());
        let package = e.mem.alloc(0x80);
        e.mem.set_u32(package, 0x0320_0000);
        // Data vtable: +8 type 3, +0xC saves.
        e.put_vtable(0x0330_0000, &[0, 0, 0x0310_3008, 0x0310_300c]);
        e.register(0x0310_3008, |_, _| 3u32.into_ret());
        e.register(0x0310_300c, |_, _| Ret::default());
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, 0x0330_0000);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this, package);
        e.mem.set_u32(this + 4, data);
        e.mem.set_u32(this + 8, 0x0abc);
        e.call_log = Some(vec![]);
        e.call(0x008c_65d0, &args![this, buffer]);
        assert_eq!(
            calls_to_008c(&e, SAVE_FORM_ID),
            vec![vec![buffer, package, 0], vec![buffer, 0x0abc, 0]]
        );
        assert_eq!(
            calls_to_008c(&e, DATA_HANDLER_CHECK_00469860),
            vec![vec![0x0400_0000, 0x33]]
        );
        assert_eq!(
            saved_bytes(&e),
            vec![
                (1, 5),
                (1, 3),
                (4, this + 0xc),
                (4, this + 0x10),
                (4, this + 0x14)
            ]
        );
        assert_eq!(calls_to_008c(&e, 0x0310_2054), vec![vec![package, buffer]]);
        assert_eq!(calls_to_008c(&e, 0x0310_300c), vec![vec![data, buffer]]);
    }

    #[test]
    fn save_game_marks_refused_packages_and_missing_data_with_ff() {
        let mut e = Engine::new();
        let buffer = save_setup(&mut e, true);
        let mut slots = vec![0u32; 0x58];
        slots[0x54 / 4] = 0x0310_2054;
        slots[0x15c / 4] = 0x0310_215c;
        e.put_vtable(0x0320_0000, &slots);
        e.register(0x0310_2054, |_, _| Ret::default());
        e.register(0x0310_215c, |_, _| false.into_ret());
        let package = e.mem.alloc(0x80);
        e.mem.set_u32(package, 0x0320_0000);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this, package);
        e.call_log = Some(vec![]);
        e.call(0x008c_65d0, &args![this, buffer]);
        let bytes = saved_bytes(&e);
        assert_eq!(bytes[0], (1, 0xff));
        assert_eq!(bytes[1], (1, 0xff));
        assert!(calls_to_008c(&e, 0x0310_2054).is_empty());
        // The data handler check failing skips the package's byte entirely.
        let mut e = Engine::new();
        let buffer = save_setup(&mut e, false);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this, package);
        e.call_log = Some(vec![]);
        e.call(0x008c_65d0, &args![this, buffer]);
        assert_eq!(saved_bytes(&e)[0], (1, 0xff));
        assert_eq!(saved_bytes(&e).len(), 4);
    }
}

#[cfg(test)]
mod load_save_tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    type Queue = Rc<RefCell<VecDeque<u32>>>;

    const SAVE_LOAD_OBJECT: u32 = 0x0e10_0000;
    const CHANGED_OBJECT: u32 = 0x0e20_0000;
    const DATA_VTABLE: u32 = 0x0e30_0000;
    const PACKAGE_VTABLE: u32 = 0x0e30_1000;
    const DATA_DELETE: u32 = 0x0e40_0000;
    const DATA_LOAD: u32 = 0x0e40_0010;
    const DATA_INIT: u32 = 0x0e40_0014;
    const PACKAGE_LOAD: u32 = 0x0e40_005c;
    const PACKAGE_INIT: u32 = 0x0e40_0064;
    const PACKAGE_FINISH: u32 = 0x0e40_0084;
    const PACKAGE_NAME: u32 = 0x0e40_0130;

    /// An engine with the globals these functions read, the loaders reading from `queue`
    /// (each read takes the next word and writes its low bytes), and doubles for every
    /// callee outside the file. Packages are 0x80-byte blocks with their type byte at +4;
    /// data objects keep their type word at +4.
    fn engine(queue: &Queue) -> Engine {
        let mut e = Engine::new();
        for page in [
            0x011d_d000u32,
            0x011d_e000,
            0x011c_3000,
            0x0101_2000,
            0x0119_b000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_u32(SAVE_LOAD_GAME, SAVE_LOAD_OBJECT);
        e.mem.set_u32(CHANGED_FORMS, CHANGED_OBJECT);
        e.mem.set_u32(DATA_HANDLER, 0x0e50_0000);
        for i in 0..8u32 {
            e.mem.set_u32(PACKAGE_TYPE_NAMES + 4 * i, 0x0e60_0000 + i);
        }
        let mut data_slots = vec![0u32; 6];
        data_slots[0] = DATA_DELETE;
        data_slots[2] = 0x0e40_0008;
        data_slots[4] = DATA_LOAD;
        data_slots[5] = DATA_INIT;
        e.put_vtable(DATA_VTABLE, &data_slots);
        e.register(0x0e40_0008, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let mut package_slots = vec![0u32; 0x50];
        for slot in [0x5cu32, 0x64, 0x84, 0x130] {
            package_slots[(slot / 4) as usize] = 0x0e40_0000 + slot;
        }
        e.put_vtable(PACKAGE_VTABLE, &package_slots);
        for f in [
            DATA_DELETE,
            DATA_LOAD,
            DATA_INIT,
            PACKAGE_LOAD,
            PACKAGE_INIT,
            PACKAGE_FINISH,
            SAVE_NUMERIC_ID,
            SAVE_BYTES,
            ADVANCE_POSITION,
            CALCULATE_PROCEDURE_TYPE,
            SET_WORD_C,
            AI_LOG,
            SAVE_GAME_WARNING,
            SET_TARGETED,
            ADD_CHANGED_FORM_ID,
            INTERLOCKED_INCREMENT,
            INTERLOCKED_DECREMENT,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        e.register(PACKAGE_NAME, |_, _| 0x0e70_0000u32.into_ret());
        e.register(FORM_VALUE_0084E3A0, |_, a| (a[0] + 1).into_ret());
        e.register(DATA_HANDLER_CHECK_00469860, |_, _| 0u32.into_ret());
        e.register(PACKAGE_TYPE, |e, a| {
            u32::from(e.mem.u8(a[0] + 4)).into_ret()
        });
        e.register(CREATE_ACTOR_PACKAGE_DATA, |e, a| {
            let data = e.mem.alloc(8);
            e.mem.set_u32(data, DATA_VTABLE);
            e.mem.set_u32(data + 4, a[0]);
            data.into_ret()
        });
        e.register(CREATE_PACKAGE, |e, a| {
            let package = e.mem.alloc(0x80);
            e.mem.set_u32(package, PACKAGE_VTABLE);
            e.mem.set_u8(package + 4, a[0] as u8);
            package.into_ret()
        });
        e.register(CALENDAR_GET_HOUR, |_, _| Ret {
            st0: 6.0,
            ..Ret::default()
        });
        e.register(FORM_LOOKUP, |_, a| a[0].into_ret());
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(CHANGED_FORM_LOOKUP, |_, a| (a[1] + 0x100).into_ret());
        e.register(PACKAGE_PROCEDURE, |_, _| 0x777u32.into_ret());
        e.register(PROCEDURE_COUNT, |_, _| 3u32.into_ret());
        for loader in [LOAD_BYTES, BUFFER_LOAD_BYTES] {
            let queue = queue.clone();
            e.register_double(loader, move |e, a| {
                let value = queue.borrow_mut().pop_front().unwrap();
                e.mem.write(a[1], &value.to_le_bytes()[..a[2] as usize]);
                Ret::default()
            });
        }
        let ids = queue.clone();
        e.register_double(LOAD_NUMERIC_ID, move |e, a| {
            let value = ids.borrow_mut().pop_front().unwrap();
            e.mem.set_u32(a[1], value);
            ids.borrow_mut().pop_front().unwrap().into_ret()
        });
        let ids = queue.clone();
        e.register_double(LOAD_FORM_ID, move |e, a| {
            let value = ids.borrow_mut().pop_front().unwrap();
            e.mem.set_u32(a[1], value);
            ids.borrow_mut().pop_front().unwrap().into_ret()
        });
        e.call_log = Some(vec![]);
        e
    }

    fn queue(values: &[u32]) -> Queue {
        Rc::new(RefCell::new(values.iter().copied().collect()))
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w)
            .collect()
    }

    fn package(e: &mut Engine, kind: u8) -> u32 {
        let package = e.mem.alloc(0x80);
        e.mem.set_u32(package, PACKAGE_VTABLE);
        e.mem.set_u8(package + 4, kind);
        package
    }

    fn data(e: &mut Engine, kind: u32) -> u32 {
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, DATA_VTABLE);
        e.mem.set_u32(data + 4, kind);
        data
    }

    #[test]
    fn save_size_counts_the_package() {
        let q = queue(&[]);
        let mut e = engine(&q);
        let this = e.mem.alloc(0x18);
        assert_eq!(e.call(0x008c_61a0, &args![this, 0xffff_ffffu32]).u32(), 8);
        let p = package(&mut e, 1);
        e.mem.set_u32(this, p);
        assert_eq!(e.call(0x008c_61a0, &args![this, 0xffff_ffffu32]).u32(), 12);
        assert_eq!(calls(&e, FORM_VALUE_0084E3A0), vec![vec![p]]);
        assert!(calls(&e, PACKAGE_TYPE).is_empty());
    }

    #[test]
    fn save_writes_the_id_index_and_start_time() {
        let q = queue(&[]);
        let mut e = engine(&q);
        let this = e.mem.alloc(0x18);
        e.call(0x008c_6250, &args![this, 0xffff_ffffu32]);
        let saved: Vec<_> = calls(&e, SAVE_BYTES).iter().map(|w| w[1]).collect();
        assert_eq!(saved, vec![this + 0x10]);
        assert_eq!(calls(&e, SAVE_NUMERIC_ID)[0][0], SAVE_LOAD_OBJECT);
        let p = package(&mut e, 1);
        e.mem.set_u32(this, p);
        e.call_log = Some(vec![]);
        e.call(0x008c_6250, &args![this, 0xffff_ffffu32]);
        let saved: Vec<_> = calls(&e, SAVE_BYTES).iter().map(|w| (w[1], w[2])).collect();
        assert_eq!(saved, vec![(this + 0xc, 4), (this + 0x10, 4)]);
        assert!(calls(&e, DATA_HANDLER_CHECK_00469860).is_empty());
    }

    #[test]
    fn load_game_takes_the_package_or_logs() {
        // A package found: it is taken, its procedure type worked out, the index read.
        let p0 = 0x0e80_0000;
        let q = queue(&[p0, 1, 2, 0x4000_0000]);
        let mut e = engine(&q);
        e.map(p0, 0x80);
        e.mem.set_u8(p0 + 4, 3);
        let this = e.mem.alloc(0x18);
        e.call(0x008c_6320, &args![this, 0xffff_ffffu32, 0u32]);
        assert_eq!(e.mem.u32(this), p0);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        assert_eq!(e.mem.u32(this + 0x10), 0x4000_0000);
        assert_eq!(calls(&e, CALCULATE_PROCEDURE_TYPE), vec![vec![p0, 0]]);
        assert!(calls(&e, AI_LOG).is_empty());
        assert_eq!(calls(&e, CREATE_ACTOR_PACKAGE_DATA), vec![vec![3]]);
        // A package that does not cast: logged with its id, the index set to -1.
        let q = queue(&[0x1234, 1, 9, 0]);
        let mut e = engine(&q);
        e.register(RT_DYNAMIC_CAST, |_, _| 0u32.into_ret());
        let this = e.mem.alloc(0x18);
        e.call(0x008c_6320, &args![this, 0u32, 0u32]);
        assert_eq!(calls(&e, AI_LOG), vec![vec![FORMAT_NO_PACKAGE, 0x1234]]);
        assert_eq!(calls(&e, SET_WORD_C), vec![vec![this, 0xffff_ffff]]);
        assert_eq!(e.mem.u32(this + 0xc), 9);
        // No id but an answer: the position moves on, the index is 0 then -1.
        let q = queue(&[0, 1, 0]);
        let mut e = engine(&q);
        let this = e.mem.alloc(0x18);
        e.call(0x008c_6320, &args![this, 0u32, 0u32]);
        assert_eq!(calls(&e, ADVANCE_POSITION), vec![vec![SAVE_LOAD_OBJECT, 4]]);
        assert_eq!(calls(&e, AI_LOG), vec![vec![FORMAT_NO_PLUGIN_PACKAGE]]);
        assert_eq!(
            calls(&e, SET_WORD_C),
            vec![vec![this, 0], vec![this, 0xffff_ffff]]
        );
        // Neither: only the start time.
        let q = queue(&[0, 0, 0x55]);
        let mut e = engine(&q);
        let this = e.mem.alloc(0x18);
        e.call(0x008c_6320, &args![this, 0u32, 0u32]);
        assert!(calls(&e, AI_LOG).is_empty());
        assert_eq!(e.mem.u32(this + 0x10), 0x55);
    }

    #[test]
    fn init_load_game_clamps_the_procedure_index() {
        let q = queue(&[]);
        let mut e = engine(&q);
        let this = e.mem.alloc(0x18);
        e.call(0x008c_64d0, &args![this, 0u32, 0u32]);
        assert!(calls(&e, PROCEDURE_COUNT).is_empty());
        let p = package(&mut e, 1);
        e.mem.set_u32(this, p);
        e.mem.set_u32(this + 0xc, 5);
        e.call(0x008c_64d0, &args![this, 0u32, 0u32]);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        assert_eq!(calls(&e, PACKAGE_PROCEDURE), vec![vec![p]]);
        assert_eq!(calls(&e, PROCEDURE_COUNT), vec![vec![0x777]]);
        e.mem.set_u32(this + 0xc, 1);
        e.call(0x008c_64d0, &args![this, 0u32, 0u32]);
        assert_eq!(e.mem.u32(this + 0xc), 1);
        // A negative index stays (signed comparison).
        e.mem.set_u32(this + 0xc, 0xffff_ffff);
        e.call(0x008c_64d0, &args![this, 0u32, 0u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0xffff_ffff);
    }

    #[test]
    fn folded_block_does_nothing() {
        let q = queue(&[]);
        let mut e = engine(&q);
        let this = e.mem.alloc(0x18);
        let p = package(&mut e, 1);
        e.mem.set_u32(this, p);
        e.call(0x008c_65a0, &args![this, 0xffff_ffffu32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        assert_eq!(e.mem.u32(this), p);
    }

    #[test]
    fn load_game_ov2_makes_the_package_and_data() {
        // The data handler accepts the id: a new package of type 3, its id change recorded,
        // the old data deleted (type -1).
        let q = queue(&[0x0100_0001, 0, 3, 0xff, 7, 8, 9, 0x0abc, 0]);
        let mut e = engine(&q);
        e.register(DATA_HANDLER_CHECK_00469860, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0x18);
        let old = data(&mut e, 2);
        e.mem.set_u32(this + 4, old);
        e.call(0x008c_6730, &args![this, 0x0d00_0000u32]);
        let p = e.mem.u32(this);
        assert_eq!(calls(&e, CREATE_PACKAGE), vec![vec![3]]);
        assert_eq!(
            calls(&e, ADD_CHANGED_FORM_ID),
            vec![vec![CHANGED_OBJECT, 0x0100_0001, p + 1]]
        );
        assert_eq!(calls(&e, PACKAGE_LOAD), vec![vec![p, 0x0d00_0000]]);
        assert_eq!(calls(&e, DATA_DELETE), vec![vec![old, 1]]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(
            (
                e.mem.u32(this + 0xc),
                e.mem.u32(this + 0x10),
                e.mem.u32(this + 0x14)
            ),
            (7, 8, 9)
        );
        assert_eq!(e.mem.u32(this + 8), 0x0abc);
        // Rejected: the id is cast; data of type 2 is made and loads itself.
        let q = queue(&[0x0200_0002, 0, 2, 0, 0, 0, 0, 0]);
        let mut e = engine(&q);
        let this = e.mem.alloc(0x18);
        e.call(0x008c_6730, &args![this, 0x0d00_0000u32]);
        assert_eq!(e.mem.u32(this), 0x0200_0002);
        let d = e.mem.u32(this + 4);
        assert_eq!(e.mem.u32(d + 4), 2);
        assert_eq!(calls(&e, DATA_LOAD), vec![vec![d, 0x0d00_0000]]);
        // Type -1: the id stays as the package and bit 0 of +0x14 is set after the load.
        let q = queue(&[0x0300_0003, 0, 0xff, 0xff, 0, 0, 0x10, 0, 0]);
        let mut e = engine(&q);
        e.register(DATA_HANDLER_CHECK_00469860, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0x18);
        e.call(0x008c_6730, &args![this, 0x0d00_0000u32]);
        assert_eq!(e.mem.u32(this), 0x0300_0003);
        assert_eq!(e.mem.u32(this + 0x14), 0x11);
        // No id and no answer: nothing more is read.
        let q = queue(&[0, 0]);
        let mut e = engine(&q);
        let this = e.mem.alloc(0x18);
        e.call(0x008c_6730, &args![this, 0x0d00_0000u32]);
        assert!(q.borrow().is_empty());
        assert!(calls(&e, BUFFER_LOAD_BYTES).is_empty());
    }

    #[test]
    fn bit_helpers() {
        let q = queue(&[]);
        let mut e = engine(&q);
        let this = e.mem.alloc(0x18);
        e.call(0x008c_6920, &args![this, 0x21u32, 0x100u32]);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        e.call(0x008c_6920, &args![this, 0x21u32, 1u32]);
        assert_eq!(e.mem.u32(this + 0x14), 2);
        assert_eq!(e.call(0x008c_6b00, &args![this, 1u32]).u32(), 1);
        assert_eq!(e.call(0x008c_6b00, &args![this, 0u32]).u32(), 0);
        e.mem.set_u32(this + 0x14, 0x8000_0002);
        assert_eq!(e.call(0x008c_6b00, &args![this, 31u32]).u32(), 0x8000_0001);
        e.call(0x008c_6920, &args![this, 1u32, 0u32]);
        assert_eq!(e.mem.u32(this + 0x14), 0x8000_0000);
    }

    #[test]
    fn init_load_game_ov2_resolves_and_counts() {
        // Marked: looked up again; the package works out its procedure type, is counted, and
        // the index clamped; the data and the target follow.
        let q = queue(&[]);
        let mut e = engine(&q);
        e.register(PACKAGE_PROCEDURE, |_, _| 0xffff_ffffu32.into_ret());
        e.register(PROCEDURE_COUNT, |_, _| 2u32.into_ret());
        let this = e.mem.alloc(0x18);
        let p = package(&mut e, 1);
        e.mem.set_u32(this, p - 0x100);
        e.mem.set_u32(this + 0x14, 1);
        e.mem.set_u32(this + 0xc, 4);
        let d = data(&mut e, 1);
        e.mem.set_u32(this + 4, d);
        e.mem.set_u32(this + 8, 0x0f00_0000);
        e.call(0x008c_6970, &args![this, 0x0d00_0000u32]);
        assert_eq!(e.mem.u32(this), p);
        assert_eq!(calls(&e, CALCULATE_PROCEDURE_TYPE), vec![vec![p, 0]]);
        assert_eq!(calls(&e, INTERLOCKED_INCREMENT), vec![vec![p + 0x7c]]);
        assert_eq!(e.mem.u32(this + 0xc), 1);
        assert_eq!(calls(&e, DATA_INIT), vec![vec![d, 0x0d00_0000]]);
        assert_eq!(calls(&e, SET_TARGETED), vec![vec![0x0f00_0000, 1]]);
        // Unmarked and accepted: its own InitLoadGame; no target, no data.
        let q = queue(&[]);
        let mut e = engine(&q);
        e.register(DATA_HANDLER_CHECK_00469860, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0x18);
        let p = package(&mut e, 1);
        e.mem.set_u32(this, p);
        e.call(0x008c_6970, &args![this, 0x0d00_0000u32]);
        assert_eq!(calls(&e, PACKAGE_INIT), vec![vec![p, 0x0d00_0000]]);
        assert!(calls(&e, CALCULATE_PROCEDURE_TYPE).is_empty());
        assert!(calls(&e, SET_TARGETED).is_empty());
        assert!(calls(&e, CHANGED_FORM_LOOKUP).is_empty());
    }

    #[test]
    fn finish_load_game_replaces_data_of_another_type() {
        let q = queue(&[]);
        let mut e = engine(&q);
        e.register(DATA_HANDLER_CHECK_00469860, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0x18);
        let p = package(&mut e, 5);
        e.mem.set_u32(this, p);
        let old = data(&mut e, 2);
        e.mem.set_u32(this + 4, old);
        e.call(0x008c_6b20, &args![this, 0x0d00_0000u32]);
        assert_eq!(calls(&e, PACKAGE_FINISH), vec![vec![p, 0x0d00_0000]]);
        assert_eq!(
            calls(&e, SAVE_GAME_WARNING),
            vec![vec![
                FORMAT_TYPE_CHANGED,
                0x0e70_0000,
                p + 1,
                0x0e60_0002,
                0x0e60_0005
            ]]
        );
        assert_eq!(calls(&e, DATA_DELETE), vec![vec![old, 1]]);
        let new = e.mem.u32(this + 4);
        assert_eq!(e.mem.u32(new + 4), 5);
        // Same type: nothing replaced; a marked package has its mark cleared.
        e.call_log = Some(vec![]);
        e.mem.set_u32(this + 0x14, 3);
        e.call(0x008c_6b20, &args![this, 0x0d00_0000u32]);
        assert_eq!(e.mem.u32(this + 0x14), 2);
        assert!(calls(&e, PACKAGE_FINISH).is_empty());
        assert!(calls(&e, SAVE_GAME_WARNING).is_empty());
        assert_eq!(e.mem.u32(this + 4), new);
    }

    #[test]
    fn revert_clears() {
        let q = queue(&[]);
        let mut e = engine(&q);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 8, 7);
        e.mem.set_u32(this + 0xc, 7);
        e.call(0x008c_6c80, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0xffff_ffff);
    }
}
