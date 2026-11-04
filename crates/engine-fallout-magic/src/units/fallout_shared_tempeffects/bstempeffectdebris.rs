//! `fallout shared/tempeffects/bstempeffectdebris.cpp` (Xbox PDB source unit), subsystem `fallout shared/tempeffects`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006834a0` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const MATCH_TARGET_006834A0: u32 = 0x0068_34a0;

// Translated from 00683480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (only caller `00682ff4`, in `~BSTempEffectDebris`): forwards
/// `this` and one word to `006834a0`, which returns whether `this` is non-null and the word at
/// `this+4` equals the argument (`SETZ DL`, result read from `AL`; 0 for a null `this`). The
/// wrapper returns that byte.
pub fn fn_00683480(e: &mut Engine, this: Ptr, value: u32) -> bool {
    e.call(MATCH_TARGET_006834A0, &args![this, value]).bool()
}

// Translated from 00682ac0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011d6a90 without reading the receiver. The map names this body `BSTempEffectDebris::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00682ac0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011d_6a90
}

/// The base constructor `00683760` (`this` only, plain `RET`).
const BASE_CONSTRUCTOR_00683760: u32 = 0x0068_3760;
/// The vtable stored at `+0` (RTTI class `BSList<BSTempEffectDebris*>`).
const VTABLE_006836F0: u32 = 0x0106_9948;

// Translated from 006836f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bstempeffectdebris.cpp`): runs `00683760` on `this`, then
/// stores the vtable `01069948` (RTTI class `BSList<BSTempEffectDebris*>`) at `+0`. Returns
/// `this`.
pub fn fn_006836f0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCTOR_00683760, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE_006836F0);
    this
}

// The functions below are `BSTempEffectDebris` (vtable `0106986c`) and the template code of
// the list of debris effects the unit keeps (the list object at `011d6a74`; vtables
// `01069958`, RTTI `BSPointerListBase<BSTempEffectDebris*>`, and `01069968`, RTTI
// `BSListBase<BSTempEffectDebris*>`). The list's head is at `+4`, its tail at `+8` and its count
// at `+0xc`; a node holds the next node at `+0`, the previous at `+4` and the item at `+8`.

/// The vtable of `BSTempEffectDebris` (RTTI).
const VTABLE_DEBRIS_0106986C: u32 = 0x0106_986c;
/// The vtables of the list classes (RTTI `BSPointerListBase<BSTempEffectDebris*>` and
/// `BSListBase<BSTempEffectDebris*>`).
const VTABLE_POINTER_LIST_01069958: u32 = 0x0106_9958;
const VTABLE_LIST_BASE_01069968: u32 = 0x0106_9968;
/// The list of debris effects (`.data`).
const DEBRIS_LIST_011D6A74: u32 = 0x011d_6a74;
/// `00559450` (thiscall, no stack words): the object a smart pointer holds.
const POINTER_GET_00559450: u32 = 0x0055_9450;
/// `0066b0d0` (thiscall, one word): assigns a smart pointer.
const POINTER_SET_0066B0D0: u32 = 0x0066_b0d0;
/// `0084e3a0` (thiscall, no stack words) and `0044ddc0` (thiscall, no stack words): the list
/// getters the unit calls on `this` and on the list (first node, count).
const GETTER_0084E3A0: u32 = 0x0084_e3a0;
const LIST_HEAD_0044DDC0: u32 = 0x0044_ddc0;
/// `006317a0` (thiscall, one word: a node): the address of a node's item.
const NODE_ITEM_006317A0: u32 = 0x0063_17a0;
/// `0069b080` (thiscall, no stack words; result in `AL`): whether the list is empty.
const LIST_EMPTY_0069B080: u32 = 0x0069_b080;
/// The model loader singleton (`.data`).
const MODEL_LOADER_011C3B3C: u32 = 0x011c_3b3c;

// Translated from 00619b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectDebris::GetIsForegroundEffect` (the map's name, from the vtable; thiscall):
/// the byte at `+0x22`.
pub fn bs_temp_effect_debris_get_is_foreground_effect(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x22)
}

// Translated from 00682ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectDebris`'s scalar deleting destructor (the map's name, from the vtable;
/// thiscall, one word, `RET 4`): the destructor ([`bs_temp_effect_debris_destructor`]), then
/// with flag bit 0 `00aa1460(this, 0x24)` (cdecl). Returns `this`.
pub fn bs_temp_effect_debris_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    bs_temp_effect_debris_destructor(e, this);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x24u32]);
    }
    this
}

// Translated from 00682c40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00682c40 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `BSTempEffectDebris` constructor (by its vtable `0106986c`; thiscall, eleven
/// words, `RET 0x2c`; the Xbox PDB's fields give the names of the words stored, the others are
/// named by the callee they go to). The base constructor `006827b0` (thiscall) with the cell
/// and the lifetime (`FLD`/`FSTP`); the vtable; `sp3D` at `+0x18` constructed (`00633c90`,
/// thiscall, with 0) and set to 0 (`0066b0d0`); `pFilename` at `+0x1c`, `bUseDebrisCounter` (the
/// last byte) at `+0x20`, 0 at `bForceDelete` (`+0x21`); then [`fn_00682d10`] with the next five
/// words, the second float (`FLD`/`FSTP`) and the byte before the last (zero-extended).
/// Returns `this`.
pub fn fn_00682c40(
    e: &mut Engine,
    this: Ptr,
    cell: u32,
    lifetime: f32,
    filename: u32,
    for_00683190: u32,
    for_00440460: u32,
    for_0043fa80: u32,
    for_0062b8d0: u32,
    for_0062ba80: u32,
    float_for_00440490: f32,
    from_cache: u32,
    use_debris_counter: u32,
) -> Ptr {
    let base = this.addr();
    e.call(
        0x0068_27b0,
        &args![base, cell, x87_quiet_bits(lifetime.to_bits())],
    );
    e.mem.set_u32(base, VTABLE_DEBRIS_0106986C);
    e.call(0x0063_3c90, &args![base + 0x18, 0u32]);
    e.call(POINTER_SET_0066B0D0, &args![base + 0x18, 0u32]);
    e.mem.set_u32(base + 0x1c, filename);
    e.mem.set_u8(base + 0x20, use_debris_counter as u8);
    e.mem.set_u8(base + 0x21, 0);
    fn_00682d10(
        e,
        this,
        for_00683190,
        for_00440460,
        for_0043fa80,
        for_0062b8d0,
        for_0062ba80,
        x87_quiet_bits(float_for_00440490.to_bits()),
        from_cache & 0xff,
    );
    this
}

/// The object `sp3D` holds (`00559450` on `this + 0x18`; Xbox PDB field name).
fn debris_node(e: &mut Engine, this: Ptr) -> u32 {
    e.call(POINTER_GET_00559450, &args![this.addr() + 0x18])
        .u32()
}

// Translated from 00682d10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00682d10 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: sets the debris up (thiscall, seven words, `RET 0x1c`; the parameters are named by
/// the callee they go to). `007e6580(this, 0)`; the model loader `011c3b3c` loads `pFilename`
/// (`+0x1c`; `00447080`, thiscall, six words: path, 0, 1, 0, 0, 0). With the last byte set
/// `sp3D` (`+0x18`) comes from the cache `011dea10` (`0045aba0(cache, model, this + 0x18)`) or
/// is set to 0 without a cache; otherwise it is a clone of the model (`00a5d2c0` with a stack
/// cloning process built by `004ad050(1.0)` and destroyed by `004ad270`). When the object and
/// `0084e3a0(this)` are set: slot `0xdc` of `005497a0(0084e3a0(this))` (when not null) with
/// the object and 1, then on the object `00440460`, `0043fa80`, `00440490` (the float,
/// `FLD`/`FSTP`), `00b55360(object, 5)` (cdecl), [`fn_00683190`] with the first word,
/// `00a5a040`, `00a59c60` with a stack point built by `0043d410(float at 011c3c08, 1, 0)`,
/// `0062b8d0(object, word, 1)` and `0062ba80(object, word, 1)` (cdecl); `00973fd0` on
/// `011e0e80` with `this`; with `bUseDebrisCounter` (`+0x20`) set it adds itself to the debris
/// list ([`fn_00683310`]) and trims it ([`fn_006833b0`]).
pub fn fn_00682d10(
    e: &mut Engine,
    this: Ptr,
    for_00683190: u32,
    for_00440460: u32,
    for_0043fa80: u32,
    for_0062b8d0: u32,
    for_0062ba80: u32,
    float_for_00440490: u32,
    from_cache: u32,
) {
    let base = this.addr();
    e.call(0x007e_6580, &args![base, 0u32]);
    let loader = e.global::<u32>(MODEL_LOADER_011C3B3C);
    let path = e.mem.u32(base + 0x1c);
    let model = e
        .call(
            0x0044_7080,
            &args![loader, path, 0u32, 1u32, 0u32, 0u32, 0u32],
        )
        .u32();
    if from_cache as u8 != 0 {
        let cache = e.global::<u32>(0x011d_ea10);
        if cache != 0 {
            e.call(0x0045_aba0, &args![cache, model, base + 0x18]);
        } else {
            e.call(POINTER_SET_0066B0D0, &args![base + 0x18, 0u32]);
        }
    } else {
        e.with_stack(0x1c, |e, cloning| {
            e.call(0x004a_d050, &args![cloning, 1.0f32.to_bits()]);
            let clone = e.call(0x00a5_d2c0, &args![model, cloning]).u32();
            e.call(POINTER_SET_0066B0D0, &args![base + 0x18, clone]);
            e.call(0x004a_d270, &args![cloning]);
        });
    }
    if debris_node(e, this) == 0 || e.call(GETTER_0084E3A0, &args![base]).u32() == 0 {
        return;
    }
    let holder = e.call(GETTER_0084E3A0, &args![base]).u32();
    let target = e.call(0x0054_97a0, &args![holder]).u32();
    if target != 0 {
        let node = debris_node(e, this);
        e.vcall(target, 0xdc, &args![node, 1u32]);
    }
    let node = debris_node(e, this);
    e.call(0x0044_0460, &args![node, for_00440460]);
    let node = debris_node(e, this);
    e.call(0x0043_fa80, &args![node, for_0043fa80]);
    let node = debris_node(e, this);
    e.call(
        0x0044_0490,
        &args![node, x87_quiet_bits(float_for_00440490)],
    );
    let node = debris_node(e, this);
    e.call(0x00b5_5360, &args![node, 5u32]);
    fn_00683190(e, this, for_00683190);
    let node = debris_node(e, this);
    e.call(0x00a5_a040, &args![node]);
    e.with_stack(0xc, |e, point| {
        let factor = x87_quiet_bits(e.global::<u32>(0x011c_3c08));
        e.call(0x0043_d410, &args![point, factor, 1u32, 0u32]);
        let node = debris_node(e, this);
        e.call(0x00a5_9c60, &args![node, point]);
    });
    let node = debris_node(e, this);
    e.call(0x0062_b8d0, &args![node, for_0062b8d0, 1u32]);
    let node = debris_node(e, this);
    e.call(0x0062_ba80, &args![node, for_0062ba80, 1u32]);
    e.call(0x0097_3fd0, &args![0x011e_0e80u32, base]);
    if e.mem.u8(base + 0x20) != 0 {
        fn_00683310(e, base);
        fn_006833b0(e);
    }
}

// Translated from 00682f40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00682f40 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// `BSTempEffectDebris::~BSTempEffectDebris` (the map's name; thiscall). Stores the vtable;
/// releases the model at `+0x1c` (`00445300` on the model loader, three words: model, 0, 1)
/// when set; when the object at `+0x18` and its `009611e0` are set, calls that one's slot
/// `0xe8` with the object. When [`fn_00683480`] on `011c3ea4` with 1 answers true it runs
/// `00c6a270(object, 1, 1, 0)` and `00c69ee0(object, 1, 0)` (cdecl), otherwise
/// `0087b140(004537b0(), object)`. With the byte at `+0x20` set it leaves the debris list
/// ([`fn_00683330`]); then sets the object to 0 (`0066b0d0`), destroys the smart pointer
/// (`0045cec0`) and runs the base destructor `00682860`.
pub fn bs_temp_effect_debris_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_DEBRIS_0106986C);
    let model = e.mem.u32(base + 0x1c);
    if model != 0 {
        let loader = e.global::<u32>(MODEL_LOADER_011C3B3C);
        e.call(0x0044_5300, &args![loader, model, 0u32, 1u32]);
    }
    if debris_node(e, this) != 0 {
        let node = debris_node(e, this);
        if e.call(0x0096_11e0, &args![node]).u32() != 0 {
            let node = debris_node(e, this);
            let owner = e.call(0x0096_11e0, &args![node]).u32();
            let node = debris_node(e, this);
            e.vcall(owner, 0xe8, &args![node]);
        }
    }
    if fn_00683480(e, Ptr::new(0x011c_3ea4), 1) {
        let node = debris_node(e, this);
        e.call(0x00c6_a270, &args![node, 1u32, 1u32, 0u32]);
        let node = debris_node(e, this);
        e.call(0x00c6_9ee0, &args![node, 1u32, 0u32]);
    } else {
        let node = debris_node(e, this);
        let queue = e.call(0x0045_37b0, &args![]).u32();
        e.call(0x0087_b140, &args![queue, node]);
    }
    if e.mem.u8(base + 0x20) != 0 {
        fn_00683330(e, base);
    }
    e.call(POINTER_SET_0066B0D0, &args![base + 0x18, 0u32]);
    e.call(0x0045_cec0, &args![base + 0x18]);
    e.call(0x0068_2860, &args![base]);
}

/// The fade threshold `00403e20` returns (a pointer to a float) when called on `011cfbfc`.
const FADE_SETTING_011CFBFC: u32 = 0x011c_fbfc;

// Translated from 006830a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectDebris::Update` (the map's name, from the vtable; thiscall, one float, `RET 4`;
/// the result in `AL`). The base update `006828a0` (with the float, `FLD`/`FSTP`); a set byte
/// at `+0x21` makes the result 0. While the result is set: the remaining time is `00488d50()`
/// minus `00621b00()` (x87, stored as a float); when it is not below the double at `01012060`
/// and not above the float `00403e20(011cfbfc)` points at (both compares false for NaN), the
/// node (slot `0xc` of slot `0x98`'s result, 0 for a null one) gets the fade alpha `remaining /
/// threshold` (`00b6bb30`, cdecl). Returns the result byte.
pub fn bs_temp_effect_debris_update(e: &mut Engine, this: Ptr, elapsed: f32) -> u8 {
    let base = this.addr();
    let mut alive = e
        .call(0x0068_28a0, &args![base, x87_quiet_bits(elapsed.to_bits())])
        .u32() as u8;
    if e.mem.u8(base + 0x21) != 0 {
        alive = 0;
    }
    if alive == 0 {
        return alive;
    }
    let now = e.call(0x0048_8d50, &args![base]).f64();
    let start = e.call(0x0062_1b00, &args![base]).f64();
    let remaining = x87_store32(x87_sub(now, start));
    let value = x87_load32(remaining);
    let lower = e.global::<f64>(0x0101_2060);
    // `FCOMP`, `TEST AH,1`: skipped when below or unordered.
    if !(value >= lower) {
        return alive;
    }
    let threshold = e.call(0x0040_3e20, &args![FADE_SETTING_011CFBFC]).u32();
    let limit = x87_m32(e.mem.u32(threshold));
    // `FCOMPP` with the threshold in ST0: skipped when it is below the value or unordered.
    if !(limit >= value) {
        return alive;
    }
    let holder = e.vcall(base, 0x98, &args![]).u32();
    let node = if holder != 0 {
        e.vcall(holder, 0xc, &args![]).u32()
    } else {
        0
    };
    if node != 0 {
        let threshold = e.call(0x0040_3e20, &args![FADE_SETTING_011CFBFC]).u32();
        let alpha = x87_store32(x87_div(
            x87_load32(remaining),
            x87_m32(e.mem.u32(threshold)),
        ));
        e.call(0x00b6_bb30, &args![node, x87_quiet_bits(alpha)]);
    }
    alive
}

// Translated from 00683190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (thiscall, one word: an object, `RET 4`). With the effect's slot `0x98` (`Get3D` in the Xbox PDB's vtable) result
/// (`holder`): the target is `0045bc00(found, 0)` when `00653270(011f9140, holder)` (cdecl)
/// finds one; otherwise `holder` itself, or, when `004a8b00(holder)` (cdecl) is 0 and the
/// holder's slot `0xc` result (`list`) is set and not empty (`0043b480`), its first entry
/// (`0043b4a0(list, 0)`), and the same once more for the entry at index 1 when the count is
/// above 1. With a target, `0084e3a0(this)` and that one's `004543c0`: when the object is
/// set and its slot `0x100` answers true, `00c6a3d0(target, 004a3a20(00931ed0(object,
/// local)), 1, 1)` (cdecl, four words; the decompiler hangs the two 1s on the wrong call);
/// then slot `0xd4` of the `004543c0` result with the target and `holder`.
pub fn fn_00683190(e: &mut Engine, this: Ptr, object: u32) {
    let base = this.addr();
    let holder = e.vcall(base, 0x98, &args![]).u32();
    let mut target = holder;
    if holder == 0 {
        return;
    }
    let found = e.call(0x0065_3270, &args![0x011f_9140u32, holder]).u32();
    if found != 0 {
        target = e.call(0x0045_bc00, &args![found, 0u32]).u32();
    } else {
        let list = e.vcall(holder, 0xc, &args![]).u32();
        let flagged = e.call(0x004a_8b00, &args![target]).u32();
        if flagged == 0
            && list != 0
            && e.call(0x0043_b480, &args![list]).u32() != 0
            && e.call(0x0043_b4a0, &args![list, 0u32]).u32() != 0
        {
            target = e.call(0x0043_b4a0, &args![list, 0u32]).u32();
            let flagged = e.call(0x004a_8b00, &args![target]).u32();
            if flagged == 0
                && list != 0
                && e.call(0x0043_b480, &args![list]).u32() > 1
                && e.call(0x0043_b4a0, &args![list, 1u32]).u32() != 0
            {
                target = e.call(0x0043_b4a0, &args![list, 1u32]).u32();
            }
        }
    }
    if target == 0 {
        return;
    }
    let outer = e.call(GETTER_0084E3A0, &args![base]).u32();
    if outer == 0 {
        return;
    }
    let receiver = e.call(0x0045_43c0, &args![outer]).u32();
    if receiver == 0 {
        return;
    }
    if object != 0 && e.vcall(object, 0x100, &args![]).u32() as u8 != 0 {
        e.with_stack(4, |e, local| {
            let made = e.call(0x0093_1ed0, &args![object, local]).u32();
            let value = e.call(0x004a_3a20, &args![made]).u32();
            e.call(0x00c6_a3d0, &args![target, value, 1u32, 1u32]);
        });
    }
    e.vcall(receiver, 0xd4, &args![target, holder]);
}

// Translated from 00683310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, one word): a set debris effect is added to the debris list (its argument
/// word passed by address to [`fn_006834e0`] on `011d6a74`).
pub fn fn_00683310(e: &mut Engine, debris: u32) {
    if debris == 0 {
        return;
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), debris);
        fn_006834e0(e, Ptr::new(DEBRIS_LIST_011D6A74), slot);
    });
}

// Translated from 00683330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, one word): removes a set debris effect from the debris list when the list is
/// not empty (`0069b080`): walks it from its first node (`0044ddc0`) with a node local, and at
/// the node whose item (`006317a0`) is the effect removes it ([`fn_00683550`]) and stops;
/// other nodes advance the local ([`fn_00683520`]).
pub fn fn_00683330(e: &mut Engine, debris: u32) {
    if debris == 0 {
        return;
    }
    if e.call(LIST_EMPTY_0069B080, &args![DEBRIS_LIST_011D6A74])
        .u32() as u8
        != 0
    {
        return;
    }
    let list = Ptr::new(DEBRIS_LIST_011D6A74);
    e.with_stack(4, |e, node| {
        let mut done = false;
        let first = e
            .call(LIST_HEAD_0044DDC0, &args![DEBRIS_LIST_011D6A74])
            .u32();
        e.mem.set_u32(node.addr(), first);
        while e.mem.u32(node.addr()) != 0 && !done {
            let current = e.mem.u32(node.addr());
            let item = e
                .call(NODE_ITEM_006317A0, &args![DEBRIS_LIST_011D6A74, current])
                .u32();
            if e.mem.u32(item) == debris {
                fn_00683550(e, list, node);
                done = true;
            } else {
                fn_00683520(e, list, node);
            }
        }
    });
}

// Translated from 006833b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): when the debris list is not empty and its count (`0084e3a0` on the
/// list) is above the limit `[0043d4d0(011cf3e4)]`, walks it from the first node and calls
/// `00683460(item, 1)` (thiscall) on each set item until `count - limit` items are done (a
/// signed count) or the nodes run out (`007b5300`, the next node).
pub fn fn_006833b0(e: &mut Engine) {
    if e.call(LIST_EMPTY_0069B080, &args![DEBRIS_LIST_011D6A74])
        .u32() as u8
        != 0
    {
        return;
    }
    let count = e.call(GETTER_0084E3A0, &args![DEBRIS_LIST_011D6A74]).u32();
    let limit_at = e.call(0x0043_d4d0, &args![0x011c_f3e4u32]).u32();
    if count <= e.mem.u32(limit_at) {
        return;
    }
    let count = e.call(GETTER_0084E3A0, &args![DEBRIS_LIST_011D6A74]).u32();
    let limit_at = e.call(0x0043_d4d0, &args![0x011c_f3e4u32]).u32();
    let mut excess = count.wrapping_sub(e.mem.u32(limit_at)) as i32;
    let mut node = e
        .call(LIST_HEAD_0044DDC0, &args![DEBRIS_LIST_011D6A74])
        .u32();
    while node != 0 && excess > 0 {
        let item_at = e
            .call(NODE_ITEM_006317A0, &args![DEBRIS_LIST_011D6A74, node])
            .u32();
        let item = e.mem.u32(item_at);
        if item != 0 {
            e.call(0x0068_3460, &args![item, 1u32]);
            excess = excess.wrapping_sub(1);
        }
        node = e
            .call(0x007b_5300, &args![DEBRIS_LIST_011D6A74, node])
            .u32();
    }
}

// Translated from 006834a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (thiscall, one word, `RET 4`; the result in `AL`): false for a null `this`,
/// otherwise whether the word at `+4` equals the argument.
pub fn fn_006834a0(e: &mut Engine, this: Ptr, value: u32) -> bool {
    if this.addr() == 0 {
        return false;
    }
    e.mem.u32(this.addr() + 4) == value
}

// Translated from 006834e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: an `AddHead(item)`-shaped list helper (thiscall, one word: the item's address; `RET 4`): a new
/// node from the list's slot 4, the item stored at its `+8`, the node linked in front
/// ([`fn_00683610`]).
pub fn fn_006834e0(e: &mut Engine, this: Ptr, item: Ptr) {
    let node = e.vcall(this.addr(), 4, &args![]).u32();
    let value = e.mem.u32(item.addr());
    e.mem.set_u32(node + 8, value);
    fn_00683610(e, this, node);
}

// Translated from 00683520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a `GetNext(position)`-shaped list helper (thiscall, one word: the address of a node local;
/// `RET 4`; `this` is not read): returns the address of the node's item (`node + 8`) and moves
/// the local to the node's `+4` link.
pub fn fn_00683520(e: &mut Engine, _this: Ptr, position: Ptr) -> u32 {
    let node = e.mem.u32(position.addr());
    let item = node.wrapping_add(8);
    let next = e.mem.u32(node.wrapping_add(4));
    e.mem.set_u32(position.addr(), next);
    item
}

// Translated from 00683550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a `RemoveAt(position)`-shaped list helper (thiscall, one word: the address of a node local;
/// `RET 4`). At the head: the local moves to the node's `+0` link and the head removal
/// `0062e0a0` (thiscall) gives the result; at the tail: the local becomes 0 and the tail
/// removal [`fn_00683670`] gives it; otherwise the node is unlinked (the local and the
/// neighbours' links set), the list's slot 8 frees it and the count drops. Returns the item.
pub fn fn_00683550(e: &mut Engine, this: Ptr, position: Ptr) -> u32 {
    let base = this.addr();
    let node = e.mem.u32(position.addr());
    if node == e.mem.u32(base + 4) {
        let next = e.mem.u32(node);
        e.mem.set_u32(position.addr(), next);
        return e.call(0x0062_e0a0, &args![base]).u32();
    }
    if node == e.mem.u32(base + 8) {
        e.mem.set_u32(position.addr(), 0);
        return fn_00683670(e, this);
    }
    let previous = e.mem.u32(node + 4);
    let next = e.mem.u32(node);
    e.mem.set_u32(position.addr(), next);
    if previous != 0 {
        e.mem.set_u32(previous, next);
    }
    if next != 0 {
        e.mem.set_u32(next + 4, previous);
    }
    let item = e.mem.u32(node + 8);
    e.vcall(base, 8, &args![node]);
    let count = e.mem.u32(base + 0xc).wrapping_sub(1);
    e.mem.set_u32(base + 0xc, count);
    item
}

// Translated from 00683610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: links a node in front of the list (thiscall, one word, `RET 4`): its `+4` is 0, its
/// `+0` the old head; the old head's `+4` (or the tail, for an empty list) becomes the node;
/// the node becomes the head and the count grows.
pub fn fn_00683610(e: &mut Engine, this: Ptr, node: u32) {
    let base = this.addr();
    e.mem.set_u32(node + 4, 0);
    let head = e.mem.u32(base + 4);
    e.mem.set_u32(node, head);
    if e.mem.u32(base + 4) != 0 {
        let head = e.mem.u32(base + 4);
        e.mem.set_u32(head + 4, node);
    } else {
        e.mem.set_u32(base + 8, node);
    }
    e.mem.set_u32(base + 4, node);
    let count = e.mem.u32(base + 0xc).wrapping_add(1);
    e.mem.set_u32(base + 0xc, count);
}

// Translated from 00683670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a `RemoveTail()`-shaped list helper (thiscall): the tail becomes the old tail's `+4` link;
/// its `+0` is cleared (or the head, when the list is now empty); the old tail's item is kept,
/// the node freed through the list's slot 8 and the count dropped. Returns the item.
pub fn fn_00683670(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let tail = e.mem.u32(base + 8);
    let previous = e.mem.u32(tail + 4);
    e.mem.set_u32(base + 8, previous);
    if e.mem.u32(base + 8) != 0 {
        let tail_now = e.mem.u32(base + 8);
        e.mem.set_u32(tail_now, 0);
    } else {
        e.mem.set_u32(base + 4, 0);
    }
    let item = e.mem.u32(tail + 8);
    e.vcall(base, 8, &args![tail]);
    let count = e.mem.u32(base + 0xc).wrapping_sub(1);
    e.mem.set_u32(base + 0xc, count);
    item
}

// Translated from 00683740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a forwarder (thiscall) to the list destructor [`fn_006837f0`].
pub fn fn_00683740(e: &mut Engine, this: Ptr) {
    fn_006837f0(e, this);
}

// Translated from 00683760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `BSPointerListBase<BSTempEffectDebris*>` constructor (by its vtable;
/// thiscall): the base constructor [`fn_006837b0`], then the vtable `01069958`. Returns
/// `this`.
pub fn fn_00683760(e: &mut Engine, this: Ptr) -> Ptr {
    fn_006837b0(e, this);
    e.mem.set_u32(this.addr(), VTABLE_POINTER_LIST_01069958);
    this
}

// Translated from 006837b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `BSListBase<BSTempEffectDebris*>` constructor (by its vtable; thiscall): the
/// vtable `01069968`, then 0 at `+0xc`, `+4` and `+8`. Returns `this`.
pub fn fn_006837b0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_LIST_BASE_01069968);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 8, 0);
    this
}

// Translated from 006837f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006837f0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `BSPointerListBase<BSTempEffectDebris*>` destructor (thiscall): the vtable
/// `01069958`, the list emptied (`0062e290`, thiscall), then the base destructor
/// [`fn_00683850`].
pub fn fn_006837f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_POINTER_LIST_01069958);
    e.call(0x0062_e290, &args![this]);
    fn_00683850(e, this);
}

// Translated from 00683850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `BSListBase<BSTempEffectDebris*>` destructor (thiscall): stores its vtable
/// `01069968`.
pub fn fn_00683850(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_LIST_BASE_01069968);
}

/// `MemoryManager` deallocation (`00401030`, cdecl, the block).
const DEALLOCATE: u32 = 0x0040_1030;
/// `006815c0` (unnamed in the engine map; no stack word): run on the
/// matrix local before it is filled.
const LOCAL_INIT_006815C0: u32 = 0x0068_15c0;
/// `00457990` (unnamed in the engine map; no stack word; ST0 result): a
/// value of the vector local (with its third component zeroed first).
const VECTOR_VALUE_00457990: u32 = 0x0045_7990;
/// `atan2` (`004b5550`, engine map; cdecl, two `float`s; ST0 result).
const ATAN2: u32 = 0x004b_5550;
/// `NiMatrix3::FromEulerAnglesZYX` (`00a59780`, engine map; `thiscall`,
/// `RET 0xC`: three `float`s).
const MATRIX_FROM_EULER_ANGLES_ZYX: u32 = 0x00a5_9780;

// Translated from 00682b00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00682b00 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: a `BSTempEffectDebris` constructor from a direction (by its
/// vtable `0106986c`; thiscall, eleven words, `RET 0x2c`). As
/// [`fn_00682c40`] up to the stored fields (the base constructor with the
/// cell and lifetime, the vtable, `sp3D` at `+0x18` constructed and set to
/// 0, `pFilename` at `+0x1c`, `bUseDebrisCounter` from the last byte,
/// `bForceDelete` 0). The three floats at `direction` are copied into a
/// local vector; with `z` the old third component and the third set to
/// 0.0, a local matrix (`006815c0` run on it) is built by
/// `NiMatrix3::FromEulerAnglesZYX(atan2(x, y), 0.0, -atan2(z, 00457990(vector)))`.
/// Then [`fn_00682d10`] with the words before and after the direction, the
/// matrix's address in the direction's place, the second float and the
/// byte before the last (zero-extended). Returns `this`.
pub fn fn_00682b00(
    e: &mut Engine,
    this: Ptr,
    cell: u32,
    lifetime: f32,
    filename: u32,
    for_00683190: u32,
    for_00440460: u32,
    direction: Ptr,
    for_0062b8d0: u32,
    for_0062ba80: u32,
    float_for_00440490: f32,
    from_cache: u32,
    use_debris_counter: u32,
) -> Ptr {
    let base = this.addr();
    e.call(
        0x0068_27b0,
        &args![base, cell, x87_quiet_bits(lifetime.to_bits())],
    );
    e.mem.set_u32(base, VTABLE_DEBRIS_0106986C);
    e.call(0x0063_3c90, &args![base + 0x18, 0u32]);
    e.call(POINTER_SET_0066B0D0, &args![base + 0x18, 0u32]);
    e.mem.set_u32(base + 0x1c, filename);
    e.mem.set_u8(base + 0x20, use_debris_counter as u8);
    e.mem.set_u8(base + 0x21, 0);
    // The frame from EBP-0x44: the vector (-0x44), the matrix (-0x34).
    e.with_stack(0x34, |e, frame| {
        let vector = frame.addr();
        let matrix = frame.addr() + 0x10;
        for i in 0..3 {
            let word = e.mem.u32(direction.addr() + 4 * i);
            e.mem.set_u32(vector + 4 * i, word);
        }
        e.call(LOCAL_INIT_006815C0, &args![matrix]);
        let z = x87_quiet_bits(e.mem.u32(vector + 8));
        e.mem.set_u32(vector + 8, x87_store32(0.0));
        let value = e.call(VECTOR_VALUE_00457990, &args![vector]).f64();
        let value = x87_store32(value);
        let pitch = e.call(ATAN2, &args![z, x87_quiet_bits(value)]).f64();
        let negated_pitch = x87_store32(-pitch);
        let x = x87_quiet_bits(e.mem.u32(vector));
        let y = x87_quiet_bits(e.mem.u32(vector + 4));
        let yaw = e.call(ATAN2, &args![x, y]).f64();
        let yaw = x87_store32(yaw);
        e.call(
            MATRIX_FROM_EULER_ANGLES_ZYX,
            &args![matrix, yaw, x87_store32(0.0), negated_pitch],
        );
        fn_00682d10(
            e,
            this,
            for_00683190,
            for_00440460,
            matrix,
            for_0062b8d0,
            for_0062ba80,
            x87_quiet_bits(float_for_00440490.to_bits()),
            from_cache & 0xff,
        );
    });
    this
}

// Translated from 00683460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (thiscall, `RET 4`): stores the argument's low byte in
/// `bForceDelete` (`+0x21`, Xbox PDB).
pub fn fn_00683460(e: &mut Engine, this: Ptr, force_delete: u32) {
    e.mem.set_u8(this.addr() + 0x21, force_delete as u8);
}

// Translated from 00683710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a scalar deleting destructor (thiscall, `RET 4`): the body
/// [`fn_00683740`], then frees `this` (`00401030`) when bit 0 of `flags`
/// is set. Returns `this`.
pub fn fn_00683710(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00683740(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 00683780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `BSPointerListBase<BSTempEffectDebris*>` scalar deleting
/// destructor (thiscall, `RET 4`): [`fn_006837f0`], then frees `this`
/// (`00401030`) when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00683780(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006837f0(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 00683870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSListBase<BSTempEffectDebris*>` scalar deleting destructor (engine
/// map; thiscall, `RET 4`): [`fn_00683850`], then frees `this`
/// (`00401030`) when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00683870(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00683850(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0068_2b00,
            fn_00682b00(Ptr, u32, f32, u32, u32, u32, Ptr, u32, u32, f32, u32, u32) -> Ptr
        ),
        entry!(0x0068_3460, fn_00683460(Ptr, u32)),
        entry!(0x0068_3710, fn_00683710(Ptr, u32) -> Ptr),
        entry!(0x0068_3780, fn_00683780(Ptr, u32) -> Ptr),
        entry!(0x0068_3870, fn_00683870(Ptr, u32) -> Ptr),
        entry!(0x0068_2ac0, fn_00682ac0(Ptr) -> u32),
        entry!(0x0068_3480, fn_00683480(Ptr, u32) -> bool),
        entry!(0x0068_36f0, fn_006836f0(Ptr) -> Ptr),
        entry!(0x0061_9b40, bs_temp_effect_debris_get_is_foreground_effect(Ptr) -> u8),
        entry!(0x0068_2ad0, bs_temp_effect_debris_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x0068_2c40,
            fn_00682c40(Ptr, u32, f32, u32, u32, u32, u32, u32, u32, f32, u32, u32) -> Ptr
        ),
        entry!(
            0x0068_2d10,
            fn_00682d10(Ptr, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(0x0068_2f40, bs_temp_effect_debris_destructor(Ptr)),
        entry!(0x0068_30a0, bs_temp_effect_debris_update(Ptr, f32) -> u8),
        entry!(0x0068_3190, fn_00683190(Ptr, u32)),
        entry!(0x0068_3310, fn_00683310(u32)),
        entry!(0x0068_3330, fn_00683330(u32)),
        entry!(0x0068_33b0, fn_006833b0()),
        entry!(0x0068_34a0, fn_006834a0(Ptr, u32) -> bool),
        entry!(0x0068_34e0, fn_006834e0(Ptr, Ptr)),
        entry!(0x0068_3520, fn_00683520(Ptr, Ptr) -> u32),
        entry!(0x0068_3550, fn_00683550(Ptr, Ptr) -> u32),
        entry!(0x0068_3610, fn_00683610(Ptr, u32)),
        entry!(0x0068_3670, fn_00683670(Ptr) -> u32),
        entry!(0x0068_3740, fn_00683740(Ptr)),
        entry!(0x0068_3760, fn_00683760(Ptr) -> Ptr),
        entry!(0x0068_37b0, fn_006837b0(Ptr) -> Ptr),
        entry!(0x0068_37f0, fn_006837f0(Ptr)),
        entry!(0x0068_3850, fn_00683850(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_00683480_forwards_this_and_the_argument_and_returns_the_byte() {
        let mut e = Engine::new();
        // The double sets bit 8 too: only the low byte (AL) is the result.
        e.register(MATCH_TARGET_006834A0, |_, a| {
            (0x100 | (a[1] & 1)).into_ret()
        });
        for (argument, expected) in [(0x2001u32, true), (0x2000u32, false)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0068_3480, &args![0x1000u32, argument]);
            let log = e.call_log.take().unwrap();
            let hits: Vec<_> = log
                .iter()
                .filter(|call| call.0 == MATCH_TARGET_006834A0)
                .collect();
            assert_eq!(hits.len(), 1);
            assert_eq!(hits[0].1, vec![0x1000u32, argument]);
            assert_eq!(result.bool(), expected);
        }
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00682ac0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0068_2ac0, &args![0x1000u32]).u32(), 0x011d_6a90);
        assert_eq!(e.call(0x0068_2ac0, &args![0u32]).u32(), 0x011d_6a90);
    }
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn runs_the_base_constructor_then_stores_the_vtable() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR_00683760, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0068_36f0, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&mut e, BASE_CONSTRUCTOR_00683760),
            vec![vec![this]]
        );
        assert_eq!(e.mem.u32(this), VTABLE_006836F0);
    }
}

#[cfg(test)]
mod b046_tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn stub(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, quiet);
        }
    }

    /// The debris list at its address, with a vtable whose slot 4 allocates a node and whose
    /// slot 8 frees one (a no-op double).
    fn debris_list(e: &mut Engine) {
        e.map(DEBRIS_LIST_011D6A74, 0x10);
        e.put_vtable(0x0320_0000, &[0, 0x0320_1004, 0x0320_1008]);
        e.mem.set_u32(DEBRIS_LIST_011D6A74, 0x0320_0000);
        e.register(0x0320_1004, |e, _| e.mem.alloc(0xc).into_ret());
        e.register(0x0320_1008, quiet);
    }

    /// Links nodes (items given) head first: `+0` toward the tail, `+4` toward the head.
    fn link(e: &mut Engine, list: u32, items: &[u32]) -> Vec<u32> {
        let nodes: Vec<u32> = items.iter().map(|_| e.mem.alloc(0xc)).collect();
        for (i, node) in nodes.iter().enumerate() {
            e.mem.set_u32(node + 8, items[i]);
            let next = nodes.get(i + 1).copied().unwrap_or(0);
            let previous = if i == 0 { 0 } else { nodes[i - 1] };
            e.mem.set_u32(*node, next);
            e.mem.set_u32(node + 4, previous);
        }
        e.mem.set_u32(list + 4, nodes.first().copied().unwrap_or(0));
        e.mem.set_u32(list + 8, nodes.last().copied().unwrap_or(0));
        e.mem.set_u32(list + 0xc, nodes.len() as u32);
        nodes
    }

    #[test]
    fn b046_00619b40_reads_the_foreground_byte() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x24);
        e.mem.set_u8(this + 0x22, 7);
        assert_eq!(e.call(0x0061_9b40, &args![this]).u32() & 0xff, 7);
    }

    /// What the destructor calls, with `00683480`'s list at `011c3ea4` answering `matches`.
    fn destructor_world(e: &mut Engine, matches: bool) {
        stub(
            e,
            &[
                0x0044_5300,
                0x00c6_a270,
                0x00c6_9ee0,
                0x0087_b140,
                POINTER_SET_0066B0D0,
                0x0045_cec0,
                0x0068_2860,
                0x00aa_1460,
                0x0320_10e8,
            ],
        );
        e.map(MODEL_LOADER_011C3B3C, 4);
        e.set_global(MODEL_LOADER_011C3B3C, 0x0bad_0001u32);
        e.map(0x011c_3ea4, 8);
        e.set_global(0x011c_3ea8, if matches { 1u32 } else { 0 });
        e.register(0x0045_37b0, |_, _| 0x0bad_0002u32.into_ret());
        e.register(LIST_EMPTY_0069B080, |_, _| 1u32.into_ret());
    }

    #[test]
    fn b046_00682ad0_and_00682f40_tear_the_debris_down() {
        let mut e = Engine::new();
        destructor_world(&mut e, true);
        e.register(POINTER_GET_00559450, |_, _| 0u32.into_ret());
        let this = e.mem.alloc(0x24);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0068_2ad0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_DEBRIS_0106986C);
        assert!(calls_to(&e, 0x0044_5300).is_empty());
        assert_eq!(calls_to(&e, 0x00c6_a270), vec![vec![0, 1, 1, 0]]);
        assert_eq!(calls_to(&e, 0x00c6_9ee0), vec![vec![0, 1, 0]]);
        assert!(calls_to(&e, 0x0087_b140).is_empty());
        assert_eq!(
            calls_to(&e, POINTER_SET_0066B0D0),
            vec![vec![this + 0x18, 0]]
        );
        assert_eq!(calls_to(&e, 0x0045_cec0), vec![vec![this + 0x18]]);
        assert_eq!(calls_to(&e, 0x0068_2860), vec![vec![this]]);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x24]]);
        // A model, a node with an owner, the other world branch, the debris counter.
        let mut e = Engine::new();
        destructor_world(&mut e, false);
        e.register(POINTER_GET_00559450, |_, _| 0x5000u32.into_ret());
        let owner = e.mem.alloc(4);
        let mut slots = vec![0u32; 0x3b];
        slots[0x3a] = 0x0320_10e8;
        e.put_vtable(0x0321_0000, &slots);
        e.mem.set_u32(owner, 0x0321_0000);
        e.register_double(0x0096_11e0, move |_, _| owner.into_ret());
        let this = e.mem.alloc(0x24);
        e.mem.set_u32(this + 0x1c, 0x4400);
        e.mem.set_u8(this + 0x20, 1);
        e.call_log = Some(vec![]);
        e.call(0x0068_2ad0, &args![this, 0u32]);
        assert_eq!(
            calls_to(&e, 0x0044_5300),
            vec![vec![0x0bad_0001, 0x4400, 0, 1]]
        );
        assert_eq!(calls_to(&e, 0x0320_10e8), vec![vec![owner, 0x5000]]);
        assert_eq!(calls_to(&e, 0x0087_b140), vec![vec![0x0bad_0002, 0x5000]]);
        assert!(calls_to(&e, 0x00c6_a270).is_empty());
        assert_eq!(calls_to(&e, LIST_EMPTY_0069B080).len(), 1);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
    }

    fn setup_world(e: &mut Engine) {
        stub(
            e,
            &[
                0x0068_27b0,
                0x0063_3c90,
                POINTER_SET_0066B0D0,
                0x007e_6580,
                0x0045_aba0,
                0x004a_d050,
                0x004a_d270,
                0x0044_0460,
                0x0043_fa80,
                0x0044_0490,
                0x00b5_5360,
                0x00a5_a040,
                0x0043_d410,
                0x00a5_9c60,
                0x0062_b8d0,
                0x0062_ba80,
                0x0097_3fd0,
            ],
        );
        e.map(MODEL_LOADER_011C3B3C, 4);
        e.set_global(MODEL_LOADER_011C3B3C, 0x0bad_0001u32);
        e.map(0x011d_ea10, 4);
        e.map(0x011c_3c08, 4);
        e.set_global(0x011c_3c08, 0x7f80_0002u32);
        e.register(0x0044_7080, |_, _| 0x4700u32.into_ret());
        e.register(0x00a5_d2c0, |_, _| 0x4800u32.into_ret());
        e.register(LIST_EMPTY_0069B080, |_, _| 1u32.into_ret());
    }

    #[test]
    fn b046_00682c40_constructs_and_stops_without_a_node() {
        let mut e = Engine::new();
        setup_world(&mut e);
        e.register(POINTER_GET_00559450, |_, _| 0u32.into_ret());
        let this = e.mem.alloc(0x24);
        e.mem.write(this, &[0xcc; 0x24]);
        let nan = f32::from_bits(0x7f80_0001);
        e.call_log = Some(vec![]);
        let words = args![
            this, 0xc1u32, nan, 0xf1u32, 0x11u32, 0x12u32, 0x13u32, 0x14u32, 0x15u32, 2.0f32,
            0x101u32, 0x201u32
        ];
        assert_eq!(e.call(0x0068_2c40, &words).u32(), this);
        assert_eq!(
            calls_to(&e, 0x0068_27b0),
            vec![vec![this, 0xc1, 0x7fc0_0001]]
        );
        assert_eq!(e.mem.u32(this), VTABLE_DEBRIS_0106986C);
        assert_eq!(calls_to(&e, 0x0063_3c90), vec![vec![this + 0x18, 0]]);
        assert_eq!(e.mem.u32(this + 0x1c), 0xf1);
        assert_eq!(e.mem.u8(this + 0x20), 1);
        assert_eq!(e.mem.u8(this + 0x21), 0);
        assert_eq!(calls_to(&e, 0x007e_6580), vec![vec![this, 0]]);
        assert_eq!(
            calls_to(&e, 0x0044_7080),
            vec![vec![0x0bad_0001, 0xf1, 0, 1, 0, 0, 0]]
        );
        // The cache byte is set (low byte of 0x101) and there is no cache: set to 0 twice.
        assert_eq!(
            calls_to(&e, POINTER_SET_0066B0D0),
            vec![vec![this + 0x18, 0], vec![this + 0x18, 0]]
        );
        assert!(calls_to(&e, 0x0084_e3a0).is_empty());
    }

    #[test]
    fn b046_00682d10_clones_or_caches_then_places_the_node() {
        let mut e = Engine::new();
        setup_world(&mut e);
        debris_list(&mut e);
        e.register(POINTER_GET_00559450, |_, _| 0x5000u32.into_ret());
        e.register(GETTER_0084E3A0, |_, _| 0x6000u32.into_ret());
        let target = e.mem.alloc(4);
        let mut slots = vec![0u32; 0x38];
        slots[0x37] = 0x0322_10dc;
        e.put_vtable(0x0322_0000, &slots);
        e.mem.set_u32(target, 0x0322_0000);
        e.register(0x0322_10dc, quiet);
        e.register_double(0x0054_97a0, move |_, _| target.into_ret());
        let this = e.mem.alloc(0x24);
        // Slot 0x98 (`Get3D`) answers null: `00683190` stops at once.
        let mut own = vec![0u32; 0x27];
        own[0x26] = 0x0323_1098;
        e.put_vtable(0x0323_0000, &own);
        e.mem.set_u32(this, 0x0323_0000);
        e.register(0x0323_1098, |_, _| 0u32.into_ret());
        e.mem.set_u32(this + 0x1c, 0xf1);
        e.mem.set_u8(this + 0x20, 1);
        e.call_log = Some(vec![]);
        let words = args![this, 1u32, 2u32, 3u32, 4u32, 5u32, 0x7f80_0003u32, 0x100u32];
        e.call(0x0068_2d10, &words);
        let cloning = calls_to(&e, 0x004a_d050);
        assert_eq!(cloning.len(), 1);
        assert_eq!(cloning[0][1], 0x3f80_0000);
        assert_eq!(calls_to(&e, 0x00a5_d2c0), vec![vec![0x4700, cloning[0][0]]]);
        assert_eq!(
            calls_to(&e, POINTER_SET_0066B0D0),
            vec![vec![this + 0x18, 0x4800]]
        );
        assert_eq!(calls_to(&e, 0x004a_d270), vec![vec![cloning[0][0]]]);
        assert_eq!(calls_to(&e, 0x0322_10dc), vec![vec![target, 0x5000, 1]]);
        assert_eq!(calls_to(&e, 0x0044_0460), vec![vec![0x5000, 2]]);
        assert_eq!(calls_to(&e, 0x0043_fa80), vec![vec![0x5000, 3]]);
        assert_eq!(calls_to(&e, 0x0044_0490), vec![vec![0x5000, 0x7fc0_0003]]);
        assert_eq!(calls_to(&e, 0x00b5_5360), vec![vec![0x5000, 5]]);
        assert_eq!(calls_to(&e, 0x0323_1098), vec![vec![this]]);
        let point = calls_to(&e, 0x0043_d410);
        assert_eq!(point[0][1..], [0x7fc0_0002, 1, 0]);
        assert_eq!(calls_to(&e, 0x00a5_9c60), vec![vec![0x5000, point[0][0]]]);
        assert_eq!(calls_to(&e, 0x0062_b8d0), vec![vec![0x5000, 4, 1]]);
        assert_eq!(calls_to(&e, 0x0062_ba80), vec![vec![0x5000, 5, 1]]);
        assert_eq!(calls_to(&e, 0x0097_3fd0), vec![vec![0x011e_0e80, this]]);
        // Added to the debris list as its only node.
        let head = e.mem.u32(DEBRIS_LIST_011D6A74 + 4);
        assert_eq!(e.mem.u32(head + 8), this);
        assert_eq!(e.mem.u32(DEBRIS_LIST_011D6A74 + 0xc), 1);
        // The cache path.
        e.set_global(0x011d_ea10, 0x0cac_0000u32);
        e.mem.set_u8(this + 0x20, 0);
        e.call_log = Some(vec![]);
        let words = args![this, 1u32, 2u32, 3u32, 4u32, 5u32, 0u32, 1u32];
        e.call(0x0068_2d10, &words);
        assert_eq!(
            calls_to(&e, 0x0045_aba0),
            vec![vec![0x0cac_0000, 0x4700, this + 0x18]]
        );
        assert!(calls_to(&e, 0x004a_d050).is_empty());
    }

    fn update_world(e: &mut Engine, now: f64, alive: u32) -> (u32, u32) {
        e.register_double(0x0068_28a0, move |_, _| alive.into_ret());
        e.register_double(0x0048_8d50, move |_, _| Ret {
            st0: now,
            ..Ret::default()
        });
        e.register(0x0062_1b00, |_, _| Ret {
            st0: 7.5,
            ..Ret::default()
        });
        e.map(0x0101_2060, 8);
        e.set_global(0x0101_2060, 0.0f64);
        let threshold = e.mem.alloc(4);
        e.mem.set_u32(threshold, 5.0f32.to_bits());
        e.register_double(0x0040_3e20, move |_, _| threshold.into_ret());
        e.register(0x00b6_bb30, quiet);
        let node_holder = e.mem.alloc(4);
        e.put_vtable(0x0324_0000, &[0, 0, 0, 0x0324_100c]);
        e.mem.set_u32(node_holder, 0x0324_0000);
        e.register(0x0324_100c, |_, _| 0x5555u32.into_ret());
        let this = e.mem.alloc(0x24);
        let mut own = vec![0u32; 0x27];
        own[0x26] = 0x0325_1098;
        e.put_vtable(0x0325_0000, &own);
        e.mem.set_u32(this, 0x0325_0000);
        e.register_double(0x0325_1098, move |_, _| node_holder.into_ret());
        (this, node_holder)
    }

    #[test]
    fn b046_006830a0_fades_the_node_by_the_remaining_time() {
        let mut e = Engine::new();
        let (this, _) = update_world(&mut e, 10.0, 0x101);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0068_30a0, &args![this, 0.25f32]).u32() & 0xff, 1);
        assert_eq!(
            calls_to(&e, 0x0068_28a0),
            vec![vec![this, 0.25f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, 0x00b6_bb30),
            vec![vec![0x5555, 0.5f32.to_bits()]]
        );
        // Past the threshold, before the start, NaN: no fade.
        for now in [13.0f64, 7.0, f64::NAN] {
            let (this, _) = update_world(&mut e, now, 1);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0068_30a0, &args![this, 0.25f32]).u32() & 0xff, 1);
            assert!(calls_to(&e, 0x00b6_bb30).is_empty());
        }
        // Dead, or forced: 0 and nothing else.
        let (this, _) = update_world(&mut e, 10.0, 0x100);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0068_30a0, &args![this, 0.25f32]).u32() & 0xff, 0);
        assert!(calls_to(&e, 0x0048_8d50).is_empty());
        let (this, _) = update_world(&mut e, 10.0, 1);
        e.mem.set_u8(this + 0x21, 1);
        assert_eq!(e.call(0x0068_30a0, &args![this, 0.25f32]).u32() & 0xff, 0);
    }

    fn placement_world(e: &mut Engine) -> (u32, u32, u32) {
        // `this` with slot 0x98 answering the holder; the holder's slot 0xc the list.
        let holder = e.mem.alloc(4);
        e.put_vtable(0x0326_0000, &[0, 0, 0, 0x0326_100c]);
        e.mem.set_u32(holder, 0x0326_0000);
        e.register(0x0326_100c, |_, _| 0x0000_9900u32.into_ret());
        let this = e.mem.alloc(0x24);
        let mut own = vec![0u32; 0x27];
        own[0x26] = 0x0327_1098;
        e.put_vtable(0x0327_0000, &own);
        e.mem.set_u32(this, 0x0327_0000);
        e.register_double(0x0327_1098, move |_, _| holder.into_ret());
        let receiver = e.mem.alloc(4);
        let mut slots = vec![0u32; 0x36];
        slots[0x35] = 0x0328_10d4;
        e.put_vtable(0x0328_0000, &slots);
        e.mem.set_u32(receiver, 0x0328_0000);
        e.register(0x0328_10d4, quiet);
        e.register(GETTER_0084E3A0, |_, _| 0x6000u32.into_ret());
        e.register_double(0x0045_43c0, move |_, _| receiver.into_ret());
        e.register(0x0045_bc00, |_, _| 0x8800u32.into_ret());
        e.register(0x004a_8b00, |_, _| 0u32.into_ret());
        e.register(0x0043_b480, |_, _| 2u32.into_ret());
        e.register(0x0043_b4a0, |_, a| (0x9100 + a[1]).into_ret());
        stub(e, &[0x0093_1ed0, 0x004a_3a20, 0x00c6_a3d0]);
        (this, holder, receiver)
    }

    #[test]
    fn b046_00683190_picks_the_target_and_hands_it_on() {
        let mut e = Engine::new();
        let (this, holder, receiver) = placement_world(&mut e);
        // Found through 00653270: its 0045bc00 is the target.
        e.register(0x0065_3270, |_, _| 0x8000u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0068_3190, &args![this, 0u32]);
        assert_eq!(calls_to(&e, 0x0065_3270), vec![vec![0x011f_9140, holder]]);
        assert_eq!(calls_to(&e, 0x0045_bc00), vec![vec![0x8000, 0]]);
        assert_eq!(
            calls_to(&e, 0x0328_10d4),
            vec![vec![receiver, 0x8800, holder]]
        );
        // Not found: the list's entries 0, then 1; an object whose slot 0x100 answers true.
        e.register(0x0065_3270, |_, _| 0u32.into_ret());
        let object = e.mem.alloc(4);
        let mut slots = vec![0u32; 0x41];
        slots[0x40] = 0x0329_1100;
        e.put_vtable(0x0329_0000, &slots);
        e.mem.set_u32(object, 0x0329_0000);
        e.register(0x0329_1100, |_, _| 0x101u32.into_ret());
        e.register(0x004a_3a20, |_, _| 0x7777u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0068_3190, &args![this, object]);
        assert_eq!(calls_to(&e, 0x004a_8b00), vec![vec![holder], vec![0x9100]]);
        assert_eq!(calls_to(&e, 0x00c6_a3d0), vec![vec![0x9101, 0x7777, 1, 1]]);
        assert_eq!(calls_to(&e, 0x0093_1ed0).len(), 1);
        assert_eq!(
            calls_to(&e, 0x0328_10d4),
            vec![vec![receiver, 0x9101, holder]]
        );
        // A flagged holder keeps the holder itself.
        e.register(0x004a_8b00, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0068_3190, &args![this, 0u32]);
        assert_eq!(
            calls_to(&e, 0x0328_10d4),
            vec![vec![receiver, holder, holder]]
        );
    }

    #[test]
    fn b046_00683310_006834e0_and_00683610_add_at_the_head() {
        let mut e = Engine::new();
        debris_list(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0068_3310, &args![0u32]);
        assert!(calls_to(&e, 0x0320_1004).is_empty());
        e.call(0x0068_3310, &args![0xd1u32]);
        e.call(0x0068_3310, &args![0xd2u32]);
        let list = DEBRIS_LIST_011D6A74;
        let head = e.mem.u32(list + 4);
        let tail = e.mem.u32(list + 8);
        assert_eq!(e.mem.u32(head + 8), 0xd2);
        assert_eq!(e.mem.u32(tail + 8), 0xd1);
        assert_eq!(e.mem.u32(head), tail);
        assert_eq!(e.mem.u32(tail + 4), head);
        assert_eq!(e.mem.u32(head + 4), 0);
        assert_eq!(e.mem.u32(list + 0xc), 2);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0xd3);
        e.call(0x0068_34e0, &args![list, slot]);
        assert_eq!(e.mem.u32(e.mem.u32(list + 4) + 8), 0xd3);
        let node = e.mem.alloc(0xc);
        e.call(0x0068_3610, &args![list, node]);
        assert_eq!(e.mem.u32(list + 4), node);
        assert_eq!(e.mem.u32(list + 0xc), 4);
    }

    fn walking_doubles(e: &mut Engine) {
        e.register(LIST_EMPTY_0069B080, |_, _| 0u32.into_ret());
        e.register(LIST_HEAD_0044DDC0, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(NODE_ITEM_006317A0, |_, a| (a[1] + 8).into_ret());
    }

    #[test]
    fn b046_00683330_00683520_and_00683550_remove_the_effect() {
        let mut e = Engine::new();
        debris_list(&mut e);
        walking_doubles(&mut e);
        let list = DEBRIS_LIST_011D6A74;
        let nodes = link(&mut e, list, &[1, 2, 3]);
        e.call_log = Some(vec![]);
        e.call(0x0068_3330, &args![2u32]);
        assert_eq!(e.mem.u32(nodes[0]), nodes[2]);
        assert_eq!(e.mem.u32(nodes[2] + 4), nodes[0]);
        assert_eq!(e.mem.u32(list + 0xc), 2);
        assert_eq!(calls_to(&e, 0x0320_1008), vec![vec![list, nodes[1]]]);
        // Not in the list: the walk runs out.
        e.call_log = Some(vec![]);
        e.call(0x0068_3330, &args![9u32]);
        assert!(calls_to(&e, 0x0320_1008).is_empty());
        assert_eq!(e.mem.u32(list + 0xc), 2);
        // Null or empty: nothing.
        e.call_log = Some(vec![]);
        e.call(0x0068_3330, &args![0u32]);
        assert!(calls_to(&e, LIST_EMPTY_0069B080).is_empty());
        let position = e.mem.alloc(4);
        e.mem.set_u32(position, nodes[2]);
        assert_eq!(
            e.call(0x0068_3520, &args![0u32, position]).u32(),
            nodes[2] + 8
        );
        assert_eq!(e.mem.u32(position), nodes[0]);
    }

    #[test]
    fn b046_00683550_and_00683670_remove_at_the_ends() {
        let mut e = Engine::new();
        debris_list(&mut e);
        e.register(0x0062_e0a0, |_, _| 0xaaaau32.into_ret());
        let list = DEBRIS_LIST_011D6A74;
        let nodes = link(&mut e, list, &[1, 2, 3]);
        let position = e.mem.alloc(4);
        e.mem.set_u32(position, nodes[0]);
        assert_eq!(e.call(0x0068_3550, &args![list, position]).u32(), 0xaaaa);
        assert_eq!(e.mem.u32(position), nodes[1]);
        e.mem.set_u32(position, nodes[2]);
        assert_eq!(e.call(0x0068_3550, &args![list, position]).u32(), 3);
        assert_eq!(e.mem.u32(position), 0);
        assert_eq!(e.mem.u32(list + 8), nodes[1]);
        assert_eq!(e.mem.u32(nodes[1]), 0);
        assert_eq!(e.mem.u32(list + 0xc), 2);
        // The last node: the head is cleared too.
        let single = link(&mut e, list, &[7]);
        assert_eq!(e.call(0x0068_3670, &args![list]).u32(), 7);
        assert_eq!(e.mem.u32(list + 4), 0);
        assert_eq!(e.mem.u32(list + 8), 0);
        assert_eq!(e.mem.u32(list + 0xc), 0);
        assert_ne!(single[0], 0);
    }

    #[test]
    fn b046_006833b0_trims_the_list_to_its_limit() {
        let mut e = Engine::new();
        debris_list(&mut e);
        walking_doubles(&mut e);
        let list = DEBRIS_LIST_011D6A74;
        let nodes = link(&mut e, list, &[0x40, 0x30, 0x20, 0, 0x10]);
        // The walk follows `+4` from the tail toward the head.
        e.register(0x007b_5300, |e, a| e.mem.u32(a[1] + 4).into_ret());
        e.register(GETTER_0084E3A0, |_, _| 5u32.into_ret());
        let limit = e.mem.alloc(4);
        e.mem.set_u32(limit, 3);
        e.register_double(0x0043_d4d0, move |_, _| limit.into_ret());
        e.register(0x0068_3460, quiet);
        e.call_log = Some(vec![]);
        e.call(0x0068_33b0, &args![]);
        assert_eq!(
            calls_to(&e, 0x0068_3460),
            vec![vec![0x10, 1], vec![0x20, 1]]
        );
        assert_eq!(calls_to(&e, NODE_ITEM_006317A0).len(), 3);
        assert_ne!(nodes[0], 0);
        // At the limit: nothing.
        e.mem.set_u32(limit, 5);
        e.call_log = Some(vec![]);
        e.call(0x0068_33b0, &args![]);
        assert!(calls_to(&e, LIST_HEAD_0044DDC0).is_empty());
    }

    #[test]
    fn b046_006834a0_matches_the_word_at_4() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x42);
        assert!(e.call(0x0068_34a0, &args![this, 0x42u32]).bool());
        assert!(!e.call(0x0068_34a0, &args![this, 0x43u32]).bool());
        assert!(!e.call(0x0068_34a0, &args![0u32, 0u32]).bool());
    }

    #[test]
    fn b046_list_constructors_and_destructors() {
        let mut e = Engine::new();
        e.register(0x0062_e290, quiet);
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0xcc; 0x10]);
        assert_eq!(e.call(0x0068_37b0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_LIST_BASE_01069968);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.call(0x0068_3760, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_POINTER_LIST_01069958);
        e.register(0x0062_e290, |e, a| {
            assert_eq!(e.mem.u32(a[0]), VTABLE_POINTER_LIST_01069958);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0068_3740, &args![this]);
        assert_eq!(calls_to(&e, 0x0062_e290), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), VTABLE_LIST_BASE_01069968);
        e.mem.set_u32(this, 0);
        e.call(0x0068_37f0, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_LIST_BASE_01069968);
        e.mem.set_u32(this, 0);
        e.call(0x0068_3850, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_LIST_BASE_01069968);
    }
}

#[cfg(test)]
mod tests_n073 {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Default::default()
        }
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_00682b00_builds_the_rotation_from_the_direction() {
        let mut e = Engine::new();
        for address in [
            0x0068_27b0u32,
            0x0063_3c90,
            POINTER_SET_0066B0D0,
            LOCAL_INIT_006815C0,
            MATRIX_FROM_EULER_ANGLES_ZYX,
            0x007e_6580,
            0x0044_7080,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.map(MODEL_LOADER_011C3B3C & !0xfff, 0x1000);
        e.map(0x011d_ea10 & !0xfff, 0x1000);
        e.register(POINTER_GET_00559450, |_, _| ret(0));
        // The vector value double checks the third component was zeroed.
        e.register(VECTOR_VALUE_00457990, |e, a| {
            assert_eq!(e.mem.u32(a[0] + 8), 0);
            Ret {
                st0: 5.0,
                ..Default::default()
            }
        });
        // atan2 double: x + 10 * y, to tell the calls apart.
        e.register(ATAN2, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])) + 10.0 * f64::from(f32::from_bits(a[1])),
            ..Default::default()
        });
        let this = e.mem.alloc(0x24);
        let direction = e.mem.alloc(0xc);
        e.mem.set_f32(direction, 1.0);
        e.mem.set_f32(direction + 4, 2.0);
        e.mem.set_f32(direction + 8, 3.0);
        e.call_log = Some(vec![]);
        let r = e
            .call(
                0x0068_2b00,
                &args![
                    this,
                    0xce11u32,
                    0.5f32.to_bits(),
                    0xf11eu32,
                    0x11u32,
                    0x22u32,
                    direction,
                    0x44u32,
                    0x55u32,
                    0.25f32.to_bits(),
                    0x101u32,
                    0x301u32
                ],
            )
            .u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(r, this);
        assert_eq!(e.mem.u32(this), VTABLE_DEBRIS_0106986C);
        assert_eq!(e.mem.u32(this + 0x1c), 0xf11e);
        assert_eq!(e.mem.u8(this + 0x20), 1);
        assert_eq!(e.mem.u8(this + 0x21), 0);
        assert_eq!(
            calls_to(&log, 0x0068_27b0),
            vec![vec![this, 0xce11, 0.5f32.to_bits()]]
        );
        // atan2(3, 5) = 53 and atan2(1, 2) = 21 under the double.
        assert_eq!(
            calls_to(&log, ATAN2),
            vec![
                vec![3.0f32.to_bits(), 5.0f32.to_bits()],
                vec![1.0f32.to_bits(), 2.0f32.to_bits()]
            ]
        );
        let euler = calls_to(&log, MATRIX_FROM_EULER_ANGLES_ZYX);
        assert_eq!(euler.len(), 1);
        assert_eq!(euler[0][1..], [21.0f32.to_bits(), 0, (-53.0f32).to_bits()]);
        // fn_00682d10 runs with the matrix in the direction's place.
        assert_eq!(calls_to(&log, 0x007e_6580), vec![vec![this, 0]]);
        assert_eq!(calls_to(&log, LOCAL_INIT_006815C0), vec![vec![euler[0][0]]]);
        // The direction itself is not changed.
        assert_eq!(e.mem.f32(direction + 8), 3.0);
    }

    #[test]
    fn fn_00683460_sets_force_delete() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x24);
        e.call(0x0068_3460, &args![this, 0x1201u32]);
        assert_eq!(e.mem.u8(this + 0x21), 1);
        e.call(0x0068_3460, &args![this, 0x100u32]);
        assert_eq!(e.mem.u8(this + 0x21), 0);
    }

    #[test]
    fn list_deleting_destructors_free_on_flag_one() {
        let mut e = Engine::new();
        e.register(DEALLOCATE, |_, _| Ret::default());
        e.register(0x0062_e290, |_, _| Ret::default());
        for (address, vtable) in [
            (0x0068_3710u32, VTABLE_LIST_BASE_01069968),
            (0x0068_3780, VTABLE_LIST_BASE_01069968),
            (0x0068_3870, VTABLE_LIST_BASE_01069968),
        ] {
            let this = e.mem.alloc(0x10);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![this, 0u32]).u32(), this);
            assert_eq!(e.mem.u32(this), vtable);
            assert!(calls_to(&e.call_log.take().unwrap(), DEALLOCATE).is_empty());
            e.call_log = Some(vec![]);
            e.call(address, &args![this, 1u32]);
            assert_eq!(
                calls_to(&e.call_log.take().unwrap(), DEALLOCATE),
                vec![vec![this]]
            );
        }
    }
}
