//! `BSMain/bspsysmultitargetemitterctlr.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c1c0e0` (named `NiPSysEmitterCtlr::LinkObject` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00C1C0E0: u32 = 0x00c1_c0e0;

// Translated from 00c5cc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysMultiTargetEmitterCtlr::LinkObject` (engine map vtable name, vtable word 010c2da4):
/// forwards `this` and the stream to `00c1c0e0` (`NiPSysEmitterCtlr::LinkObject`). Returns
/// nothing: Xbox PDB `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the
/// `NiStream` reference as an address. Parameter count from the `RET 4` of both bodies.
pub fn bs_psys_multi_target_emitter_ctlr_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00C1C0E0, &args![this, stream]);
}

// Translated from 00c5cea0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01203340 without reading the receiver. The map names this body `BSPSysMultiTargetEmitterCtlr::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c5cea0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_3340
}

layout! {
    /// `BSPSysMultiTargetEmitterCtlr` (Xbox PDB; 0x64 bytes, the size its
    /// `CreateObject` allocates): the fields its functions use. The
    /// `NiTimeController` / `NiPSysEmitterCtlr` fields sit at the Xbox PDB's
    /// offsets on PC.
    pub struct BSPSysMultiTargetEmitterCtlr: 0x64 {
        /// `m_uFlags` (Xbox PDB, `NiTimeController`).
        0x08 m_uFlags: u16,
        /// `m_fFrequency` (Xbox PDB).
        0x0C m_fFrequency: u32,
        /// `m_fPhase` (Xbox PDB).
        0x10 m_fPhase: u32,
        /// `m_fHiKeyTime` (Xbox PDB).
        0x18 m_fHiKeyTime: u32,
        /// `m_fStartTime` (Xbox PDB).
        0x1C m_fStartTime: u32,
        /// `m_fLastTime` (Xbox PDB).
        0x20 m_fLastTime: u32,
        /// `m_fWeightedLastTime` (Xbox PDB).
        0x24 m_fWeightedLastTime: u32,
        /// `m_fScaledTime` (Xbox PDB).
        0x28 m_fScaledTime: u32,
        /// `m_pkTarget` (Xbox PDB).
        0x2C m_pkTarget: u32,
        /// `m_pkModifier` (Xbox PDB, `NiPSysModifierCtlr`): the emitter.
        0x3C m_pkModifier: u32,
        /// `m_fLastScaledTime` (Xbox PDB, `NiPSysEmitterCtlr`).
        0x48 m_fLastScaledTime: u32,
        /// `m_kParticleAges` (Xbox PDB): `NiTPrimitiveSet<float>`, elements
        /// at +0x50, allocated count +0x54, used count +0x58.
        0x50 m_kParticleAges: u32,
        0x58 m_kParticleAgesUsed: u32,
        /// `pMasterParticleSystem` (Xbox PDB).
        0x5C pMasterParticleSystem: u32,
        /// `usMaxFullEmitters` (Xbox PDB).
        0x60 usMaxFullEmitters: u16,
    }
}

/// The vtable of `BSPSysMultiTargetEmitterCtlr` (RTTI
/// `.?AVBSPSysMultiTargetEmitterCtlr@@`).
const VTABLE: u32 = 0x010c_2d54;
/// The base constructor `00c1c390` and destructor `00c1c300` (`thiscall`;
/// no Xbox PDB names).
const BASE_CONSTRUCTOR_00C1C390: u32 = 0x00c1_c390;
const BASE_DESTRUCTOR_00C1C300: u32 = 0x00c1_c300;
/// `NiNew` allocation (`00aa13e0`, `cdecl`, the size).
const NI_NEW: u32 = 0x00aa_13e0;
/// `NiDelete` of a sized block (`00aa1460`, `cdecl`: block, size).
const NI_DELETE: u32 = 0x00aa_1460;
/// A block allocation (`00aa1070`, `cdecl`, the size; no Xbox PDB name).
const NI_ALLOC: u32 = 0x00aa_1070;
/// A block free (`00aa10f0`, `cdecl`, one word; no Xbox PDB name).
const NI_FREE: u32 = 0x00aa_10f0;
/// `NiPSysEmitterCtlr::LoadBinary` (`00c1bdf0`, engine map; `thiscall`,
/// one word).
const EMITTER_CTLR_LOAD_BINARY: u32 = 0x00c1_bdf0;
/// `00a30370` (`cdecl`: stream, address): loads a 16-bit value.
const STREAM_LOAD_SHORT_00A30370: u32 = 0x00a3_0370;
/// `NiStream::ResolveLinkID` (`00a64620`, engine map; `thiscall`).
const STREAM_RESOLVE_LINK_ID: u32 = 0x00a6_4620;
/// The `float` `FLT_MAX` (`7f7fffff`) at `010c2d4c`; its negation marks an
/// unset time.
const FLOAT_MAX: u32 = 0x010c_2d4c;
/// `0.0` (`double`).
const DOUBLE_ZERO: u32 = 0x0101_2060;
/// `00ec9130` (two `double`s, result in ST0): a CRT math routine (a jump
/// into `00ed60eb`; unnamed in the engine map) that wraps the scaled time by
/// the key span here.
const MATH_00EC9130: u32 = 0x00ec_9130;
/// `NiPSysEmitterCtlr::GetBirthRateInterpolator` (`00639b60`, engine map;
/// `thiscall`).
const GET_BIRTH_RATE_INTERPOLATOR: u32 = 0x0063_9b60;
/// `00c1c240` (`thiscall` on the age set, one word: the address of a
/// `float`): appends the `float`, growing the set (doubling, at least 1)
/// through [`fn_00c1beb0`] when full.
const AGES_ADD_00C1C240: u32 = 0x00c1_c240;
/// The 16-bit cap on particles emitted per target (`011afa9c`).
const MAX_PER_TARGET: u32 = 0x011a_fa9c;

// Translated from 00c5c400 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSPSysMultiTargetEmitterCtlr` constructor (`thiscall`; no Xbox PDB
/// name): the base constructor `00c1c390`, the vtable, no master system and
/// no maximum. Returns `this`.
pub fn fn_00c5c400(
    e: &mut Engine,
    this: Ptr<BSPSysMultiTargetEmitterCtlr>,
) -> Ptr<BSPSysMultiTargetEmitterCtlr> {
    e.call(BASE_CONSTRUCTOR_00C1C390, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    e.set(this, BSPSysMultiTargetEmitterCtlr::pMasterParticleSystem, 0);
    e.set(this, BSPSysMultiTargetEmitterCtlr::usMaxFullEmitters, 0);
    this
}

// Translated from 00c5c440 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSPSysMultiTargetEmitterCtlr` destructor (`thiscall`; no Xbox PDB
/// name): stores the vtable, clears the master system pointer and runs the
/// base destructor `00c1c300`.
pub fn fn_00c5c440(e: &mut Engine, this: Ptr<BSPSysMultiTargetEmitterCtlr>) {
    e.mem.set_u32(this.addr(), VTABLE);
    e.set(this, BSPSysMultiTargetEmitterCtlr::pMasterParticleSystem, 0);
    e.call(BASE_DESTRUCTOR_00C1C300, &args![this]);
}

/// `FLD dword` of a field.
fn load(e: &Engine, address: u32) -> f64 {
    x87_load32(e.mem.u32(address))
}

/// The negated `FLT_MAX` (`FLD`, `FCHS`).
fn unset_time(e: &Engine) -> f64 {
    -load(e, FLOAT_MAX)
}

// Translated from 00c5c470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysMultiTargetEmitterCtlr::Update` (Xbox PDB, matched by the engine
/// map; `thiscall`, one word: the address of the time). Without active
/// emitters in the master system only `m_fLastTime` follows the time.
/// Otherwise, with a target and an emitter, active (flag 0x8) and either a
/// new time or flag 0x80: the start and last times are set up (the negated
/// `FLT_MAX` marks them unset), `m_fWeightedLastTime` advances by the
/// elapsed time times the frequency, `m_fScaledTime` is it plus the phase,
/// wrapped by `m_fHiKeyTime` (`00ec9130`, made non-negative; 0 without a
/// span). A first update only records the scaled time (or 0). Then, when the
/// birth rate interpolator gives a rate (vtable +0x9C), the particles since
/// the last scaled time (across the wrap when the time went round) are
/// emitted ([`bs_psys_multi_target_emitter_ctlr_emit_particles`]), and the
/// last scaled time becomes the scaled time.
pub fn bs_psys_multi_target_emitter_ctlr_update(
    e: &mut Engine,
    this: Ptr<BSPSysMultiTargetEmitterCtlr>,
    time: Ptr,
) {
    type C = BSPSysMultiTargetEmitterCtlr;
    let base = this.addr();
    let time = time.addr();
    let master = e.get(this, C::pMasterParticleSystem);
    // BSMasterParticleSystem::usActiveEmitterObjCount (+0xB8).
    if e.mem.u16(master + 0xb8) == 0 {
        let now = x87_quiet_bits(e.mem.u32(time));
        e.set(this, C::m_fLastTime, now);
        return;
    }
    // FCOMPP of the time against m_fLastTime, TEST AH,5 / JP.
    if load(e, time) < load(e, base + 0x20) {
        let unset = x87_store32(unset_time(e));
        e.set(this, C::m_fLastScaledTime, unset);
    }
    if e.get(this, C::m_pkTarget) == 0 || e.get(this, C::m_pkModifier) == 0 {
        return;
    }
    if e.get(this, C::m_uFlags) & 8 == 0 {
        return;
    }
    // FUCOMPP, TEST AH,0x44 / JP: an equal time needs flag 0x80.
    if load(e, time) == load(e, base + 0x20) && e.get(this, C::m_uFlags) & 0x80 == 0 {
        return;
    }
    if load(e, base + 0x1c) == unset_time(e) {
        let now = x87_quiet_bits(e.mem.u32(time));
        e.set(this, C::m_fStartTime, now);
    }
    let elapsed = if load(e, base + 0x20) == unset_time(e) {
        x87_store32(0.0)
    } else {
        x87_store32(x87_sub(load(e, time), x87_m32(e.mem.u32(base + 0x20))))
    };
    let weighted = x87_mul(x87_load32(elapsed), x87_m32(e.mem.u32(base + 0xc)));
    let weighted = x87_add(weighted, x87_m32(e.mem.u32(base + 0x24)));
    e.set(this, C::m_fWeightedLastTime, x87_store32(weighted));
    let scaled = x87_add(load(e, base + 0x24), x87_m32(e.mem.u32(base + 0x10)));
    e.set(this, C::m_fScaledTime, x87_store32(scaled));
    let now = x87_quiet_bits(e.mem.u32(time));
    e.set(this, C::m_fLastTime, now);
    let span = x87_quiet_bits(e.get(this, C::m_fHiKeyTime));
    let zero: f64 = e.global(DOUBLE_ZERO);
    if x87_load32(span) == zero {
        e.set(this, C::m_fScaledTime, x87_store32(0.0));
    } else {
        let scaled = x87_quiet_bits(e.get(this, C::m_fScaledTime));
        let wrapped = e
            .call(MATH_00EC9130, &args![x87_load32(scaled), x87_load32(span)])
            .f64();
        let wrapped = x87_quiet_bits(x87_store32(wrapped));
        e.set(this, C::m_fScaledTime, wrapped);
        // FCOMP with 0.0, TEST AH,5 / JP: only a negative value moves up.
        if load(e, base + 0x28) < zero {
            let up = x87_add(load(e, base + 0x28), x87_m32(span));
            e.set(this, C::m_fScaledTime, x87_store32(up));
        }
    }
    if load(e, base + 0x48) == unset_time(e) {
        // FCOMP with 0.0, TEST AH,0x41 / JNZ.
        if load(e, base + 0x28) > zero {
            e.set(this, C::m_fLastScaledTime, x87_store32(0.0));
        } else {
            let scaled = x87_quiet_bits(e.get(this, C::m_fScaledTime));
            e.set(this, C::m_fLastScaledTime, scaled);
            return;
        }
    }
    let interpolator = e.call(GET_BIRTH_RATE_INTERPOLATOR, &args![this]).u32();
    let target = e.get(this, C::m_pkTarget);
    let age = x87_quiet_bits(e.mem.u32(target + 0xd4));
    let mut wrapped = false;
    // FCOMPP of m_fLastScaledTime against m_fScaledTime.
    let delta = if load(e, base + 0x48) < load(e, base + 0x28) {
        x87_store32(x87_sub(
            load(e, base + 0x28),
            x87_m32(e.mem.u32(base + 0x48)),
        ))
    } else {
        let back = x87_sub(load(e, base + 0x18), x87_m32(e.mem.u32(base + 0x48)));
        let sum = x87_store32(x87_add(back, x87_m32(e.mem.u32(base + 0x28))));
        wrapped = true;
        sum
    };
    if interpolator != 0 {
        e.with_stack(4, |e, rate| {
            let target = e.get(this, C::m_pkTarget);
            let found = e
                .vcall(interpolator, 0x9c, &args![x87_store32(0.0), target, rate])
                .bool();
            if found {
                let end = if wrapped {
                    x87_quiet_bits(e.get(this, C::m_fHiKeyTime))
                } else {
                    x87_quiet_bits(e.get(this, C::m_fScaledTime))
                };
                let now = x87_store32(x87_add(x87_load32(age), x87_m32(delta)));
                let last = x87_quiet_bits(e.get(this, C::m_fLastScaledTime));
                let span = x87_quiet_bits(e.get(this, C::m_fHiKeyTime));
                let rate = x87_quiet_bits(e.mem.u32(rate.addr()));
                bs_psys_multi_target_emitter_ctlr_emit_particles(
                    e,
                    this,
                    f32::from_bits(x87_quiet_bits(now)),
                    f32::from_bits(end),
                    f32::from_bits(last),
                    0.0,
                    f32::from_bits(span),
                    f32::from_bits(rate),
                );
            }
        });
    }
    let scaled = x87_quiet_bits(e.get(this, C::m_fScaledTime));
    e.set(this, C::m_fLastScaledTime, scaled);
}

// Translated from 00c5c790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysMultiTargetEmitterCtlr::EmitParticles` (Xbox PDB, matched by the
/// engine map; `thiscall`, six `float` words: the current age, the end,
/// last and low times, the span and the birth rate). With a master system,
/// a positive rate and `low < end`, `last < span`, `low < span`: the number
/// to emit is `rate * (min(end, span) - low)` less `rate * (last - low)`
/// (when `low <= last`), both truncated (`FISTP` toward zero) to 16 bits;
/// it is spread over the master system's active emitters (`min(count *
/// usMaxFullEmitters, count * active)` divided by the active count, capped
/// by the word at `011afa9c`; at least 1, then over all of them). Each
/// particle's age (`end - n / rate`) goes into `m_kParticleAges`
/// (`00c1c240`), then each emitter system from the master system's iterator
/// ([`fn_00c5ce40`]) that lacks bit 0 of +0x30 gets the emitter (vtable
/// +0xAC: set the system; +0x50 takes the system's `float` at +0x64) and
/// emits (vtable +0xA4 with the age, the per-emitter count and the ages);
/// the emitter's system and +0x50 value are put back at the end.
#[allow(clippy::too_many_arguments)]
pub fn bs_psys_multi_target_emitter_ctlr_emit_particles(
    e: &mut Engine,
    this: Ptr<BSPSysMultiTargetEmitterCtlr>,
    age: f32,
    end: f32,
    last: f32,
    low: f32,
    span: f32,
    rate: f32,
) {
    type C = BSPSysMultiTargetEmitterCtlr;
    let master = e.get(this, C::pMasterParticleSystem);
    if master == 0 {
        return;
    }
    let zero: f64 = e.global(DOUBLE_ZERO);
    let (end_v, last_v, low_v, span_v, rate_v) = (
        x87_load32(end.to_bits()),
        x87_load32(last.to_bits()),
        x87_load32(low.to_bits()),
        x87_load32(span.to_bits()),
        x87_load32(rate.to_bits()),
    );
    // Each FCOMP/FCOMPP with TEST AH,0x41 / JNZ (or TEST AH,5 / JP) leaves
    // only the strict ordered comparison going on.
    if !(rate_v > zero) || !(span_v > low_v) || !(low_v < end_v) || !(span_v > last_v) {
        return;
    }
    // TEST AH,1 / JNZ: `span < end` (or unordered) takes the span.
    let range = if span_v >= end_v {
        x87_store32(x87_sub(end_v, x87_m32(low.to_bits())))
    } else {
        x87_store32(x87_sub(span_v, x87_m32(low.to_bits())))
    };
    let mut done = x87_store32(0.0);
    // TEST AH,0x41 / JP: `low < last` or equal.
    if low_v <= last_v {
        done = x87_store32(x87_sub(last_v, x87_m32(low.to_bits())));
    }
    // BSMasterParticleSystem::usActiveEmitterObjCount (+0xB8).
    if e.mem.u16(master + 0xb8) == 0 {
        return;
    }
    let total = crate::x87::fistp_chop(x87_mul(rate_v, x87_m32(range))) as u16;
    let mut emitted = crate::x87::fistp_chop(x87_mul(rate_v, x87_m32(done))) as u16;
    let count = total.wrapping_sub(emitted);
    if count == 0 {
        return;
    }
    let master = e.get(this, C::pMasterParticleSystem);
    let mut emitters = e.mem.u16(master + 0xb8);
    let full = (u32::from(e.get(this, C::usMaxFullEmitters)).wrapping_mul(u32::from(count))) as u16;
    let all = i32::from(count).wrapping_mul(i32::from(emitters));
    let limit = (if i32::from(full) < all {
        u32::from(full)
    } else {
        all as u32
    }) as u16;
    let mut per_emitter = (i32::from(limit) / i32::from(emitters)) as u16;
    let cap: u16 = e.global(MAX_PER_TARGET);
    if i32::from(per_emitter) > i32::from(cap) {
        per_emitter = cap;
    }
    if per_emitter == 0 {
        emitters = limit;
        per_emitter = 1;
    }
    // FLD1; FDIVRP (DE F1): ST(1) = ST(0) / ST(1), so 1 / rate.
    let step = x87_store32(x87_div(1.0, rate_v));
    e.set(this, C::m_kParticleAgesUsed, 0);
    e.with_stack(4, |e, particle_age| {
        let mut i: u16 = 0;
        while i < per_emitter {
            emitted = emitted.wrapping_add(1);
            let back = x87_mul(f64::from(emitted), x87_m32(step));
            let value = x87_sub(x87_m32(range), back);
            e.mem.set_u32(particle_age.addr(), x87_store32(value));
            e.call(AGES_ADD_00C1C240, &args![this.addr() + 0x50, particle_age]);
            i = i.wrapping_add(1);
        }
    });
    let emitter = e.get(this, C::m_pkModifier);
    let saved_system = e.mem.u32(emitter + 0x54);
    let saved_value = x87_quiet_bits(e.mem.u32(emitter + 0x50));
    while emitters > 0 {
        let master = e.get(this, C::pMasterParticleSystem);
        let system = fn_00c5ce40(e, Ptr::new(master));
        if system != 0 && e.mem.u32(system + 0x30) & 1 == 0 {
            e.vcall(emitter, 0xac, &args![system]);
            let value = x87_quiet_bits(e.mem.u32(system + 0x64));
            e.mem.set_u32(emitter + 0x50, value);
            let ages = e.get(this, C::m_kParticleAges);
            e.vcall(
                emitter,
                0xa4,
                &args![x87_quiet_bits(age.to_bits()), u32::from(per_emitter), ages],
            );
            emitters = emitters.wrapping_sub(1);
        }
    }
    e.vcall(emitter, 0xac, &args![saved_system]);
    e.mem.set_u32(emitter + 0x50, saved_value);
}

// Translated from 00c5cbb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5cbb0 - the C++ exception unwinding frame (FS:[0], handler 00f2fd09) is not translated
/// The class's `CreateObject` (`cdecl`, no words; no Xbox PDB name): a
/// 0x64-byte block (`00aa13e0`) constructed by [`fn_00c5c400`], or null
/// when the allocation fails. The exception-unwinding frame is not
/// translated.
pub fn fn_00c5cbb0(e: &mut Engine) -> Ptr<BSPSysMultiTargetEmitterCtlr> {
    let block = e.call(NI_NEW, &args![0x64u32]).u32();
    if block != 0 {
        fn_00c5c400(e, Ptr::new(block))
    } else {
        Ptr::new(0)
    }
}

// Translated from 00c5cc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysMultiTargetEmitterCtlr::LoadBinary` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the stream):
/// `NiPSysEmitterCtlr::LoadBinary`, `usMaxFullEmitters` (at least 1) and the
/// master system's link id (`NiStream::ResolveLinkID`).
pub fn bs_psys_multi_target_emitter_ctlr_load_binary(
    e: &mut Engine,
    this: Ptr<BSPSysMultiTargetEmitterCtlr>,
    stream: Ptr,
) {
    e.call(EMITTER_CTLR_LOAD_BINARY, &args![this, stream]);
    e.call(
        STREAM_LOAD_SHORT_00A30370,
        &args![stream, this.addr() + 0x60],
    );
    if e.get(this, BSPSysMultiTargetEmitterCtlr::usMaxFullEmitters) < 1 {
        e.set(this, BSPSysMultiTargetEmitterCtlr::usMaxFullEmitters, 1);
    }
    let link = e.call(STREAM_RESOLVE_LINK_ID, &args![stream]).u32();
    e.set(
        this,
        BSPSysMultiTargetEmitterCtlr::pMasterParticleSystem,
        link,
    );
}

// Translated from 00c5ce10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysMultiTargetEmitterCtlr::_scalar_deleting_destructor_` (Xbox PDB,
/// matched by the engine map; `thiscall`, one word: flags): the destructor
/// ([`fn_00c5c440`]), then with flag bit 0 the 0x64-byte block is freed
/// (`00aa1460`). Returns `this`.
pub fn bs_psys_multi_target_emitter_ctlr_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSPSysMultiTargetEmitterCtlr>,
    flags: u32,
) -> Ptr<BSPSysMultiTargetEmitterCtlr> {
    fn_00c5c440(e, this);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x64u32]);
    }
    this
}

// Translated from 00c5ce40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The master particle system's next emitter object (`thiscall` on the
/// `BSMasterParticleSystem`; no Xbox PDB name): the item (+8) of the node at
/// `FirstEmitterObjIter` (+0xC0), which moves to the next node, back to the
/// list's head (+0xAC) at the end.
pub fn fn_00c5ce40(e: &mut Engine, master: Ptr) -> u32 {
    let iterator = master.addr() + 0xc0;
    let node = e.mem.u32(iterator);
    let item = node.wrapping_add(8);
    let next = e.mem.u32(node);
    e.mem.set_u32(iterator, next);
    let object = e.mem.u32(item);
    if e.mem.u32(iterator) == 0 {
        let head = e.mem.u32(master.addr() + 0xac);
        e.mem.set_u32(iterator, head);
    }
    object
}

// Translated from 00c1beb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resizes an `NiTSet<float, NiTMallocInterface<float>>` (`thiscall`, one
/// word: the new allocated count; the engine map's name for this body,
/// `NiTSet<..>::Add`, does not match it): unless the count is unchanged,
/// allocates `count` `float`s (none for 0; `00aa1070`), copies the used
/// ones (through the x87 stack), frees the old block (`00aa10f0`) and stores
/// the block (+0) and the count (+4).
pub fn fn_00c1beb0(e: &mut Engine, this: Ptr, count: u32) {
    let set = this.addr();
    if count == e.mem.u32(set + 4) {
        return;
    }
    let block = if count != 0 {
        let block = e.call(NI_ALLOC, &args![count.wrapping_mul(4)]).u32();
        let mut index = 0u32;
        if e.mem.u32(set + 8) > index {
            loop {
                let old = e.mem.u32(set);
                let value = x87_quiet_bits(e.mem.u32(old.wrapping_add(index.wrapping_mul(4))));
                index = index.wrapping_add(1);
                e.mem.set_u32(
                    block.wrapping_add(index.wrapping_mul(4)).wrapping_sub(4),
                    value,
                );
                if index >= e.mem.u32(set + 8) {
                    break;
                }
            }
        }
        block
    } else {
        0
    };
    let old = e.mem.u32(set);
    e.call(NI_FREE, &args![old]);
    e.mem.set_u32(set, block);
    e.mem.set_u32(set + 4, count);
}

/// `BSPSysMultiTargetEmitterCtlr` is 0x64 bytes.
const CONTROLLER_SIZE: u32 = 0x64;
/// `NiPSysEmitterCtlr`'s member copy (`00c1bd30`, `thiscall`, `(destination, cloning)`),
/// `ProcessClone` (`00c1bdc0`), `RegisterStreamables` (`00c1be10`), `SaveBinary`
/// (`00c1be40`), `IsEqual` (`00c1be60`) and `GetViewerStrings` (`00c1ce70`), the last five
/// `thiscall` with one word.
const EMITTER_CTLR_COPY_MEMBERS: u32 = 0x00c1_bd30;
const EMITTER_CTLR_PROCESS_CLONE: u32 = 0x00c1_bdc0;
const EMITTER_CTLR_REGISTER_STREAMABLES: u32 = 0x00c1_be10;
const EMITTER_CTLR_SAVE_BINARY: u32 = 0x00c1_be40;
const EMITTER_CTLR_IS_EQUAL: u32 = 0x00c1_be60;
const EMITTER_CTLR_GET_VIEWER_STRINGS: u32 = 0x00c1_ce70;
/// The cloning process's object map lookup (`00a53c30`, `thiscall` on the map, `(key,
/// &value)`); the map is the first word of the cloning process.
const CLONE_MAP_LOOKUP: u32 = 0x00a5_3c30;
/// `NiStreamSaveBinary<unsigned short>` (`00a303a0`, `cdecl`, `(stream, &value)`).
const STREAM_SAVE_SHORT: u32 = 0x00a3_03a0;
/// `NiGetViewerString` (`00a7d330`, `(name)`), its `(label, pointer)` form `00a7d5a0` and
/// its `(label, unsigned short)` form `00a7d490` (all `cdecl`), and the viewer-string
/// array's add (`0096a610`, `thiscall`, `(&string)`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_POINTER: u32 = 0x00a7_d5a0;
const NI_GET_VIEWER_STRING_SHORT: u32 = 0x00a7_d490;
const VIEWER_STRINGS_ADD: u32 = 0x0096_a610;
/// The class's `ms_RTTI` (its first word is the class name), and the labels `"Master
/// Particle System"` and `"Max Full Emitters"`.
const RTTI: u32 = 0x0120_3340;
const MASTER_LABEL: u32 = 0x010c_2e5c;
const MAX_FULL_EMITTERS_LABEL: u32 = 0x010c_2e48;

// Translated from 00c5ca90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5ca90 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2fcdf, state variable) is not translated
/// `BSPSysMultiTargetEmitterCtlr::CreateClone` (Xbox PDB, engine map): allocates 0x64
/// bytes, constructs a controller in it when that succeeds (`00c5c400`), copies the
/// members into it (`00c5cb20`; into address 0 when the allocation failed) and returns it.
pub fn bs_psys_multi_target_emitter_ctlr_create_clone(
    e: &mut Engine,
    this: Ptr<BSPSysMultiTargetEmitterCtlr>,
    cloning: u32,
) -> Ptr<BSPSysMultiTargetEmitterCtlr> {
    let block = e.call(NI_NEW, &args![CONTROLLER_SIZE]).u32();
    let clone = if block == 0 {
        Ptr::new(0)
    } else {
        fn_00c5c400(e, Ptr::new(block))
    };
    fn_00c5cb20(e, this, clone, cloning);
    clone
}

// Translated from 00c5cb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `(destination, cloning)`, the member copy
/// `CreateClone` uses): `NiPSysEmitterCtlr`'s member copy (`00c1bd30`), then
/// `usMaxFullEmitters` copied (the master system is not).
pub fn fn_00c5cb20(
    e: &mut Engine,
    this: Ptr<BSPSysMultiTargetEmitterCtlr>,
    destination: Ptr<BSPSysMultiTargetEmitterCtlr>,
    cloning: u32,
) {
    e.call(
        EMITTER_CTLR_COPY_MEMBERS,
        &args![this, destination, cloning],
    );
    let most = e.get(this, BSPSysMultiTargetEmitterCtlr::usMaxFullEmitters);
    e.set(
        destination,
        BSPSysMultiTargetEmitterCtlr::usMaxFullEmitters,
        most,
    );
}

// Translated from 00c5cb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysMultiTargetEmitterCtlr::ProcessClone` (Xbox PDB, engine map):
/// `NiPSysEmitterCtlr::ProcessClone`, then through the cloning process's object map
/// (`00a53c30`, one local starting at 0) the clone of this controller and, into the same
/// local, the clone of the master system, which the controller's clone takes as its
/// master. A master without a clone leaves the local holding the controller's clone.
pub fn bs_psys_multi_target_emitter_ctlr_process_clone(
    e: &mut Engine,
    this: Ptr<BSPSysMultiTargetEmitterCtlr>,
    cloning: u32,
) {
    e.call(EMITTER_CTLR_PROCESS_CLONE, &args![this, cloning]);
    e.with_stack(4, |e, found| {
        e.mem.set_u32(found.addr(), 0);
        let map = e.mem.u32(cloning);
        e.call(CLONE_MAP_LOOKUP, &args![map, this, found]);
        let clone = e.mem.u32(found.addr());
        let master = e.get(this, BSPSysMultiTargetEmitterCtlr::pMasterParticleSystem);
        let map = e.mem.u32(cloning);
        e.call(CLONE_MAP_LOOKUP, &args![map, master, found]);
        let master_clone = e.mem.u32(found.addr());
        e.set(
            Ptr::<BSPSysMultiTargetEmitterCtlr>::new(clone),
            BSPSysMultiTargetEmitterCtlr::pMasterParticleSystem,
            master_clone,
        );
    });
}

// Translated from 00c5ccb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysMultiTargetEmitterCtlr::RegisterStreamables` (Xbox PDB, engine map): whether
/// `NiPSysEmitterCtlr::RegisterStreamables` succeeded.
pub fn bs_psys_multi_target_emitter_ctlr_register_streamables(
    e: &mut Engine,
    this: Ptr<BSPSysMultiTargetEmitterCtlr>,
    stream: Ptr,
) -> bool {
    e.call(EMITTER_CTLR_REGISTER_STREAMABLES, &args![this, stream])
        .u8()
        != 0
}

// Translated from 00c5cce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysMultiTargetEmitterCtlr::SaveBinary` (Xbox PDB, engine map):
/// `NiPSysEmitterCtlr::SaveBinary`, `usMaxFullEmitters` (`00a303a0`), then the master
/// system's link (the stream's vtable slot 0x30).
pub fn bs_psys_multi_target_emitter_ctlr_save_binary(
    e: &mut Engine,
    this: Ptr<BSPSysMultiTargetEmitterCtlr>,
    stream: Ptr,
) {
    e.call(EMITTER_CTLR_SAVE_BINARY, &args![this, stream]);
    e.call(
        STREAM_SAVE_SHORT,
        &args![stream, this.addr().wrapping_add(0x60)],
    );
    let master = e.get(this, BSPSysMultiTargetEmitterCtlr::pMasterParticleSystem);
    e.vcall(stream.addr(), 0x30, &args![master]);
}

// Translated from 00c5cd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysMultiTargetEmitterCtlr::IsEqual` (Xbox PDB, engine map):
/// `NiPSysEmitterCtlr::IsEqual`, and when that holds, whether the master systems and
/// `usMaxFullEmitters` are equal.
pub fn bs_psys_multi_target_emitter_ctlr_is_equal(
    e: &mut Engine,
    this: Ptr<BSPSysMultiTargetEmitterCtlr>,
    other: Ptr<BSPSysMultiTargetEmitterCtlr>,
) -> bool {
    if e.call(EMITTER_CTLR_IS_EQUAL, &args![this, other]).u8() == 0 {
        return false;
    }
    e.get(this, BSPSysMultiTargetEmitterCtlr::pMasterParticleSystem)
        == e.get(other, BSPSysMultiTargetEmitterCtlr::pMasterParticleSystem)
        && e.get(this, BSPSysMultiTargetEmitterCtlr::usMaxFullEmitters)
            == e.get(other, BSPSysMultiTargetEmitterCtlr::usMaxFullEmitters)
}

// Translated from 00c5cd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysMultiTargetEmitterCtlr::GetViewerStrings` (Xbox PDB, engine map):
/// `NiPSysEmitterCtlr::GetViewerStrings`, then the class name, `"Master Particle System"`
/// with the master system and `"Max Full Emitters"` with `usMaxFullEmitters`, each added to
/// the list.
pub fn bs_psys_multi_target_emitter_ctlr_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<BSPSysMultiTargetEmitterCtlr>,
    strings: Ptr,
) {
    e.call(EMITTER_CTLR_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(0xc, |e, slots| {
        let name = e.global::<u32>(RTTI);
        let text = e.call(NI_GET_VIEWER_STRING, &args![name]).u32();
        let slot = slots.addr().wrapping_add(8);
        e.mem.set_u32(slot, text);
        e.call(VIEWER_STRINGS_ADD, &args![strings, slot]);
        let master = e.get(this, BSPSysMultiTargetEmitterCtlr::pMasterParticleSystem);
        let text = e
            .call(NI_GET_VIEWER_STRING_POINTER, &args![MASTER_LABEL, master])
            .u32();
        let slot = slots.addr().wrapping_add(4);
        e.mem.set_u32(slot, text);
        e.call(VIEWER_STRINGS_ADD, &args![strings, slot]);
        let most = u32::from(e.get(this, BSPSysMultiTargetEmitterCtlr::usMaxFullEmitters));
        let text = e
            .call(
                NI_GET_VIEWER_STRING_SHORT,
                &args![MAX_FULL_EMITTERS_LABEL, most],
            )
            .u32();
        e.mem.set_u32(slots.addr(), text);
        e.call(VIEWER_STRINGS_ADD, &args![strings, slots]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c5_ca90,
            bs_psys_multi_target_emitter_ctlr_create_clone(
                Ptr<BSPSysMultiTargetEmitterCtlr>,
                u32,
            )
                -> Ptr<BSPSysMultiTargetEmitterCtlr>
        ),
        entry!(
            0x00c5_cb20,
            fn_00c5cb20(Ptr<BSPSysMultiTargetEmitterCtlr>, Ptr<BSPSysMultiTargetEmitterCtlr>, u32)
        ),
        entry!(
            0x00c5_cb50,
            bs_psys_multi_target_emitter_ctlr_process_clone(Ptr<BSPSysMultiTargetEmitterCtlr>, u32)
        ),
        entry!(
            0x00c5_ccb0,
            bs_psys_multi_target_emitter_ctlr_register_streamables(
                Ptr<BSPSysMultiTargetEmitterCtlr>,
                Ptr,
            ) -> bool
        ),
        entry!(
            0x00c5_cce0,
            bs_psys_multi_target_emitter_ctlr_save_binary(Ptr<BSPSysMultiTargetEmitterCtlr>, Ptr)
        ),
        entry!(
            0x00c5_cd20,
            bs_psys_multi_target_emitter_ctlr_is_equal(
                Ptr<BSPSysMultiTargetEmitterCtlr>,
                Ptr<BSPSysMultiTargetEmitterCtlr>,
            ) -> bool
        ),
        entry!(
            0x00c5_cd80,
            bs_psys_multi_target_emitter_ctlr_get_viewer_strings(
                Ptr<BSPSysMultiTargetEmitterCtlr>,
                Ptr,
            )
        ),
        entry!(0x00c5_cea0, fn_00c5cea0(Ptr) -> u32),
        entry!(
            0x00c5_cc90,
            bs_psys_multi_target_emitter_ctlr_link_object(Ptr, Ptr)
        ),
        entry!(
            0x00c5_c400,
            fn_00c5c400(Ptr<BSPSysMultiTargetEmitterCtlr>) -> Ptr<BSPSysMultiTargetEmitterCtlr>
        ),
        entry!(0x00c5_c440, fn_00c5c440(Ptr<BSPSysMultiTargetEmitterCtlr>)),
        entry!(
            0x00c5_c470,
            bs_psys_multi_target_emitter_ctlr_update(Ptr<BSPSysMultiTargetEmitterCtlr>, Ptr)
        ),
        entry!(
            0x00c5_c790,
            bs_psys_multi_target_emitter_ctlr_emit_particles(
                Ptr<BSPSysMultiTargetEmitterCtlr>,
                f32,
                f32,
                f32,
                f32,
                f32,
                f32,
            )
        ),
        entry!(0x00c5_cbb0, fn_00c5cbb0() -> Ptr<BSPSysMultiTargetEmitterCtlr>),
        entry!(
            0x00c5_cc30,
            bs_psys_multi_target_emitter_ctlr_load_binary(Ptr<BSPSysMultiTargetEmitterCtlr>, Ptr)
        ),
        entry!(
            0x00c5_ce10,
            bs_psys_multi_target_emitter_ctlr_scalar_deleting_destructor(
                Ptr<BSPSysMultiTargetEmitterCtlr>,
                u32,
            ) -> Ptr<
                BSPSysMultiTargetEmitterCtlr,
            >
        ),
        entry!(0x00c5_ce40, fn_00c5ce40(Ptr) -> u32),
        entry!(0x00c1_beb0, fn_00c1beb0(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_bs_psys_multi_target_emitter_ctlr_link_object_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00C1C0E0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c5_cc90, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00C1C0E0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c5cea0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c5_cea0, &args![0x1000u32]).u32(), 0x0120_3340);
        assert_eq!(e.call(0x00c5_cea0, &args![0u32]).u32(), 0x0120_3340);
    }
}

#[cfg(test)]
mod emitter_tests {
    use super::*;

    type C = BSPSysMultiTargetEmitterCtlr;
    type CallLog = Vec<(u32, Vec<u32>)>;

    const INTERPOLATOR_RATE: u32 = 0x0001_0010;
    const EMITTER_SET_SYSTEM: u32 = 0x0001_0020;
    const EMITTER_EMIT: u32 = 0x0001_0030;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn calls(e: &mut Engine) -> CallLog {
        e.call_log.take().unwrap().into_iter().skip(1).collect()
    }

    fn bits(value: f32) -> u32 {
        value.to_bits()
    }

    fn globals(e: &mut Engine) {
        e.map(0x010c_2000, 0x1000);
        e.set_global(FLOAT_MAX, f32::MAX);
        e.map(0x0101_2000, 0x100);
        e.set_global(DOUBLE_ZERO, 0.0f64);
        e.map(0x011a_f000, 0x1000);
        e.set_global(MAX_PER_TARGET, 100u16);
    }

    #[test]
    fn constructor_and_destructor_set_the_fields() {
        let mut e = Engine::new();
        let this = e.new_object::<C>();
        e.mem.write(this.addr() + 0x5c, &[0xcd; 8]);
        e.register(BASE_CONSTRUCTOR_00C1C390, |_, _| Ret::default());
        e.register(BASE_DESTRUCTOR_00C1C300, |_, _| Ret::default());
        assert_eq!(e.call(0x00c5_c400, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(e.get(this, C::pMasterParticleSystem), 0);
        assert_eq!(e.get(this, C::usMaxFullEmitters), 0);
        assert_eq!(e.mem.u16(this.addr() + 0x62), 0xcdcd);
        e.set(this, C::pMasterParticleSystem, 5);
        e.mem.set_u32(this.addr(), 0);
        e.call_log = Some(vec![]);
        e.call(0x00c5_c440, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(e.get(this, C::pMasterParticleSystem), 0);
        assert_eq!(
            calls(&mut e),
            vec![(BASE_DESTRUCTOR_00C1C300, vec![this.addr()])]
        );
    }

    /// A controller with a master system holding one emitter object (a
    /// particle system with +0x64 = 7), a target (+0xD4 = 0.5) and an
    /// emitter whose vtable +0xAC / +0xA4 are doubles; the birth rate
    /// interpolator answers a rate of 10.
    fn controller(e: &mut Engine) -> (Ptr<C>, u32, u32) {
        globals(e);
        let this = e.new_object::<C>();
        let master = e.mem.alloc(0xe0);
        e.mem.set_u16(master + 0xb8, 1);
        let particles = e.mem.alloc(0x70);
        e.mem.set_f32(particles + 0x64, 7.0);
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(node + 8, particles);
        e.mem.set_u32(master + 0xac, node);
        e.mem.set_u32(master + 0xc0, node);
        e.set(this, C::pMasterParticleSystem, master);
        let target = e.mem.alloc(0xe0);
        e.mem.set_f32(target + 0xd4, 0.5);
        e.set(this, C::m_pkTarget, target);
        let emitter = e.mem.alloc(0x60);
        let mut slots = vec![0u32; 0xb0 / 4];
        slots[0xa4 / 4] = EMITTER_EMIT;
        slots[0xac / 4] = EMITTER_SET_SYSTEM;
        e.put_vtable(0x0002_0000, &slots);
        e.mem.set_u32(emitter, 0x0002_0000);
        e.mem.set_f32(emitter + 0x50, 3.0);
        e.mem.set_u32(emitter + 0x54, 0x4444);
        e.set(this, C::m_pkModifier, emitter);
        e.set(this, C::m_uFlags, 8);
        e.set(this, C::usMaxFullEmitters, 3);
        let interpolator = e.mem.alloc(0x10);
        let mut slots = vec![0u32; 0xa0 / 4];
        slots[0x9c / 4] = INTERPOLATOR_RATE;
        e.put_vtable(0x0002_1000, &slots);
        e.mem.set_u32(interpolator, 0x0002_1000);
        e.register_double(GET_BIRTH_RATE_INTERPOLATOR, move |_, _| word(interpolator));
        e.register(INTERPOLATOR_RATE, |e, a| {
            assert_eq!(a[1], 0);
            e.mem.set_f32(a[3], 10.0);
            word(1)
        });
        e.register(MATH_00EC9130, |_, a| {
            let x = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            let y = f64::from_bits(u64::from(a[2]) | u64::from(a[3]) << 32);
            st0(x % y)
        });
        e.register(AGES_ADD_00C1C240, |_, _| Ret::default());
        e.register(EMITTER_SET_SYSTEM, |_, _| Ret::default());
        e.register(EMITTER_EMIT, |e, a| {
            assert_eq!(e.mem.f32(a[0] + 0x50), 7.0);
            Ret::default()
        });
        (this, emitter, particles)
    }

    #[test]
    fn update_without_active_emitters_only_follows_the_time() {
        let mut e = Engine::new();
        let (this, _, _) = controller(&mut e);
        let master = e.get(this, C::pMasterParticleSystem);
        e.mem.set_u16(master + 0xb8, 0);
        let time = e.mem.alloc(4);
        e.mem.set_f32(time, 2.0);
        e.call_log = Some(vec![]);
        e.call(0x00c5_c470, &args![this, time]);
        assert_eq!(e.get(this, C::m_fLastTime), bits(2.0));
        assert!(calls(&mut e).is_empty());
    }

    #[test]
    fn update_emits_the_particles_since_the_last_scaled_time() {
        let mut e = Engine::new();
        let (this, emitter, particles) = controller(&mut e);
        let unset = bits(-f32::MAX);
        e.set(this, C::m_fStartTime, unset);
        e.set(this, C::m_fLastTime, bits(1.0));
        e.set(this, C::m_fFrequency, bits(1.0));
        e.set(this, C::m_fHiKeyTime, bits(4.0));
        e.set(this, C::m_fLastScaledTime, bits(0.25));
        let time = e.mem.alloc(4);
        e.mem.set_f32(time, 2.0);
        e.call_log = Some(vec![]);
        e.call(0x00c5_c470, &args![this, time]);
        let log = calls(&mut e);
        assert_eq!(e.get(this, C::m_fStartTime), bits(2.0));
        assert_eq!(e.get(this, C::m_fWeightedLastTime), bits(1.0));
        assert_eq!(e.get(this, C::m_fScaledTime), bits(1.0));
        assert_eq!(e.get(this, C::m_fLastTime), bits(2.0));
        assert_eq!(e.get(this, C::m_fLastScaledTime), bits(1.0));
        // rate 10: 10 particles up to the end, 2 already; 8 for the one
        // emitter, aged from the end back by n / 10.
        let ages: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == AGES_ADD_00C1C240)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(ages, vec![this.addr() + 0x50; 8]);
        assert_eq!(
            log.iter()
                .filter(|c| c.0 == EMITTER_SET_SYSTEM)
                .map(|c| c.1[1])
                .collect::<Vec<_>>(),
            vec![particles, 0x4444]
        );
        let emit: Vec<_> = log.iter().filter(|c| c.0 == EMITTER_EMIT).collect();
        assert_eq!(emit.len(), 1);
        assert_eq!(emit[0].1[..3], [emitter, bits(1.25), 8]);
        assert_eq!(e.mem.f32(emitter + 0x50), 3.0);
        assert_eq!(e.get(this, C::m_kParticleAgesUsed), 0);
    }

    #[test]
    fn emit_particles_stores_each_particle_age() {
        let mut e = Engine::new();
        let (this, _, _) = controller(&mut e);
        e.register(AGES_ADD_00C1C240, |e, a| {
            let value = e.mem.u32(a[1]);
            let used = e.mem.u32(a[0] + 8);
            e.mem.set_u32(0x0003_0000 + 4 * used, value);
            e.mem.set_u32(a[0] + 8, used + 1);
            Ret::default()
        });
        e.map(0x0003_0000, 0x100);
        e.call(
            0x00c5_c790,
            &args![this, 1.25f32, 1.0f32, 0.25f32, 0.0f32, 4.0f32, 10.0f32],
        );
        let step = f64::from(0.1f32);
        for (i, n) in (3..=10).enumerate() {
            let expected = (1.0 - f64::from(n) * step) as f32;
            assert_eq!(
                e.mem.f32(0x0003_0000 + 4 * i as u32),
                expected,
                "particle {n}"
            );
        }
        assert_eq!(e.get(this, C::m_kParticleAgesUsed), 8);
    }

    #[test]
    fn emit_particles_returns_early_on_bad_ranges() {
        let mut e = Engine::new();
        let (this, _, _) = controller(&mut e);
        for words in [
            args![this, 1.0f32, 1.0f32, 0.25f32, 0.0f32, 4.0f32, 0.0f32],
            args![this, 1.0f32, 1.0f32, 0.25f32, 1.0f32, 4.0f32, 10.0f32],
            args![this, 1.0f32, 1.0f32, 4.0f32, 0.0f32, 4.0f32, 10.0f32],
            args![this, 1.0f32, 1.0f32, 0.25f32, 0.0f32, f32::NAN, 10.0f32],
            // As many done as there are: nothing to emit.
            args![this, 1.0f32, 1.0f32, 1.0f32, 0.0f32, 4.0f32, 10.0f32],
        ] {
            e.call_log = Some(vec![]);
            e.call(0x00c5_c790, &words);
            assert!(calls(&mut e).is_empty());
        }
    }

    #[test]
    fn update_records_the_first_scaled_time_only() {
        let mut e = Engine::new();
        let (this, _, _) = controller(&mut e);
        e.set(this, C::m_fLastScaledTime, bits(-f32::MAX));
        e.set(this, C::m_fLastTime, bits(1.0));
        let time = e.mem.alloc(4);
        e.mem.set_f32(time, 2.0);
        e.call_log = Some(vec![]);
        e.call(0x00c5_c470, &args![this, time]);
        // No key span: the scaled time is 0 and becomes the last one.
        assert_eq!(e.get(this, C::m_fScaledTime), 0);
        assert_eq!(e.get(this, C::m_fLastScaledTime), 0);
        assert!(calls(&mut e).is_empty());

        // The same time again without flag 0x80 changes nothing.
        e.set(this, C::m_fScaledTime, bits(9.0));
        e.call(0x00c5_c470, &args![this, time]);
        assert_eq!(e.get(this, C::m_fScaledTime), bits(9.0));
    }

    #[test]
    fn create_object_constructs_the_controller() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x64);
        e.register_double(NI_NEW, move |_, a| {
            assert_eq!(a[0], 0x64);
            word(block)
        });
        e.register(BASE_CONSTRUCTOR_00C1C390, |_, _| Ret::default());
        assert_eq!(e.call(0x00c5_cbb0, &args![]).u32(), block);
        assert_eq!(e.mem.u32(block), VTABLE);
        e.register(NI_NEW, |_, _| word(0));
        assert_eq!(e.call(0x00c5_cbb0, &args![]).u32(), 0);
    }

    #[test]
    fn load_binary_reads_the_maximum_and_the_master_link() {
        let mut e = Engine::new();
        let this = e.new_object::<C>();
        e.register(EMITTER_CTLR_LOAD_BINARY, |_, _| Ret::default());
        e.register(STREAM_LOAD_SHORT_00A30370, |e, a| {
            e.mem.set_u16(a[1], 0);
            Ret::default()
        });
        e.register(STREAM_RESOLVE_LINK_ID, |_, _| word(0x9000));
        e.call(0x00c5_cc30, &args![this, 0x500u32]);
        assert_eq!(e.get(this, C::usMaxFullEmitters), 1);
        assert_eq!(e.get(this, C::pMasterParticleSystem), 0x9000);
        e.register(STREAM_LOAD_SHORT_00A30370, |e, a| {
            e.mem.set_u16(a[1], 4);
            Ret::default()
        });
        e.call(0x00c5_cc30, &args![this, 0x500u32]);
        assert_eq!(e.get(this, C::usMaxFullEmitters), 4);
    }

    #[test]
    fn scalar_deleting_destructor_frees_with_bit_0() {
        let mut e = Engine::new();
        let this = e.new_object::<C>();
        e.register(BASE_DESTRUCTOR_00C1C300, |_, _| Ret::default());
        e.register(NI_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c5_ce10, &args![this, 1u32]).u32(), this.addr());
        assert_eq!(
            calls(&mut e).last().unwrap(),
            &(NI_DELETE, vec![this.addr(), 0x64])
        );
        e.call_log = Some(vec![]);
        e.call(0x00c5_ce10, &args![this, 0u32]);
        assert!(calls(&mut e).iter().all(|c| c.0 != NI_DELETE));
    }

    #[test]
    fn master_iterator_wraps_to_the_head() {
        let mut e = Engine::new();
        let master = e.mem.alloc(0xe0);
        let second = e.mem.alloc(0xc);
        e.mem.set_u32(second + 8, 0x22);
        let first = e.mem.alloc(0xc);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 8, 0x11);
        e.mem.set_u32(master + 0xac, first);
        e.mem.set_u32(master + 0xc0, first);
        assert_eq!(e.call(0x00c5_ce40, &args![master]).u32(), 0x11);
        assert_eq!(e.mem.u32(master + 0xc0), second);
        assert_eq!(e.call(0x00c5_ce40, &args![master]).u32(), 0x22);
        assert_eq!(e.mem.u32(master + 0xc0), first);
    }

    #[test]
    fn float_set_resize_copies_the_used_values() {
        let mut e = Engine::new();
        let set = e.mem.alloc(0xc);
        let old = e.mem.alloc(8);
        e.mem.set_f32(old, 1.5);
        e.mem.set_u32(old + 4, 0x7f80_0001);
        e.mem.set_u32(set, old);
        e.mem.set_u32(set + 4, 2);
        e.mem.set_u32(set + 8, 2);
        e.register(NI_ALLOC, |e, a| word(e.mem.alloc(a[0])));
        e.register(NI_FREE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c1_beb0, &args![set, 4u32]);
        let log = calls(&mut e);
        let block = e.mem.u32(set);
        assert_eq!(log, vec![(NI_ALLOC, vec![16]), (NI_FREE, vec![old])]);
        assert_eq!(e.mem.f32(block), 1.5);
        assert_eq!(e.mem.u32(block + 4), 0x7fc0_0001);
        assert_eq!(e.mem.u32(set + 4), 4);
        // The same count: nothing.
        e.call_log = Some(vec![]);
        e.call(0x00c1_beb0, &args![set, 4u32]);
        assert!(calls(&mut e).is_empty());
        // Zero: no block.
        e.call(0x00c1_beb0, &args![set, 0u32]);
        assert_eq!(e.mem.u32(set), 0);
        assert_eq!(e.mem.u32(set + 4), 0);
    }
}

#[cfg(test)]
mod n068_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;
    type Controller = Ptr<BSPSysMultiTargetEmitterCtlr>;

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            eax.into_ret()
        });
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn controller(e: &mut Engine, master: u32, most: u16) -> Controller {
        let this = e.new_object::<BSPSysMultiTargetEmitterCtlr>();
        e.set(
            this,
            BSPSysMultiTargetEmitterCtlr::pMasterParticleSystem,
            master,
        );
        e.set(this, BSPSysMultiTargetEmitterCtlr::usMaxFullEmitters, most);
        this
    }

    #[test]
    fn create_clone_constructs_and_copies_the_maximum() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, BASE_CONSTRUCTOR_00C1C390, 0);
        logged(&mut e, &log, EMITTER_CTLR_COPY_MEMBERS, 0);
        let block = e.mem.alloc(0x64);
        e.mem.write(block, &[0xab; 0x64]);
        logged(&mut e, &log, NI_NEW, block);
        let this = controller(&mut e, 0x7000, 12);
        assert_eq!(e.call(0x00c5_ca90, &args![this, 0x5555u32]).u32(), block);
        assert_eq!(e.mem.u32(block), VTABLE);
        assert_eq!(e.mem.u16(block + 0x60), 12);
        assert_eq!(e.mem.u32(block + 0x5c), 0);
        assert_eq!(calls(&log, NI_NEW), vec![vec![0x64]]);
        assert_eq!(
            calls(&log, EMITTER_CTLR_COPY_MEMBERS),
            vec![vec![this.addr(), block, 0x5555]]
        );
    }

    #[test]
    fn process_clone_points_the_clone_at_the_cloned_master() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, EMITTER_CTLR_PROCESS_CLONE, 0);
        let this = controller(&mut e, 0x7000, 1);
        let clone = controller(&mut e, 0x7000, 1);
        e.register_double(CLONE_MAP_LOOKUP, move |e, a| {
            assert_eq!(a[0], 0x9900);
            if a[1] == this.addr() {
                e.mem.set_u32(a[2], clone.addr());
                return 1u32.into_ret();
            }
            if a[1] == 0x7000 {
                e.mem.set_u32(a[2], 0x7100);
                return 1u32.into_ret();
            }
            0u32.into_ret()
        });
        let cloning = e.mem.alloc(8);
        e.mem.set_u32(cloning, 0x9900);
        e.call(0x00c5_cb50, &args![this, cloning]);
        assert_eq!(e.mem.u32(clone.addr() + 0x5c), 0x7100);
        assert_eq!(
            calls(&log, EMITTER_CTLR_PROCESS_CLONE),
            vec![vec![this.addr(), cloning]]
        );
        // A master without a clone: the local still holds the controller's clone.
        e.set(
            this,
            BSPSysMultiTargetEmitterCtlr::pMasterParticleSystem,
            0x7200,
        );
        e.call(0x00c5_cb50, &args![this, cloning]);
        assert_eq!(e.mem.u32(clone.addr() + 0x5c), clone.addr());
    }

    #[test]
    fn register_streamables_answers_the_base_result() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, EMITTER_CTLR_REGISTER_STREAMABLES, 0x300);
        assert!(!e.call(0x00c5_ccb0, &args![0x1000u32, 0x2000u32]).bool());
        logged(&mut e, &log, EMITTER_CTLR_REGISTER_STREAMABLES, 1);
        assert!(e.call(0x00c5_ccb0, &args![0x1000u32, 0x2000u32]).bool());
    }

    #[test]
    fn save_binary_writes_the_maximum_and_the_master_link() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, EMITTER_CTLR_SAVE_BINARY, 0);
        logged(&mut e, &log, STREAM_SAVE_SHORT, 0);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x30, 0x0040_6500);
        logged(&mut e, &log, 0x0040_6500, 0);
        let stream = e.mem.alloc(4);
        e.mem.set_u32(stream, vtable);
        let this = controller(&mut e, 0x7000, 3);
        e.call(0x00c5_cce0, &args![this, stream]);
        assert_eq!(
            *log.borrow(),
            vec![
                (EMITTER_CTLR_SAVE_BINARY, vec![this.addr(), stream]),
                (STREAM_SAVE_SHORT, vec![stream, this.addr() + 0x60]),
                (0x0040_6500, vec![stream, 0x7000]),
            ]
        );
    }

    #[test]
    fn is_equal_compares_master_and_maximum() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, EMITTER_CTLR_IS_EQUAL, 1);
        let this = controller(&mut e, 0x7000, 3);
        let same = controller(&mut e, 0x7000, 3);
        let other_master = controller(&mut e, 0x7001, 3);
        let other_most = controller(&mut e, 0x7000, 4);
        assert!(e.call(0x00c5_cd20, &args![this, same]).bool());
        assert!(!e.call(0x00c5_cd20, &args![this, other_master]).bool());
        assert!(!e.call(0x00c5_cd20, &args![this, other_most]).bool());
        logged(&mut e, &log, EMITTER_CTLR_IS_EQUAL, 0);
        assert!(!e.call(0x00c5_cd20, &args![this, same]).bool());
    }

    #[test]
    fn viewer_strings_add_name_master_and_maximum() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, EMITTER_CTLR_GET_VIEWER_STRINGS, 0);
        e.map(RTTI, 4);
        e.set_global::<u32>(RTTI, 0x0101_0000);
        e.register(NI_GET_VIEWER_STRING, |_, a| (a[0] + 1).into_ret());
        e.register(NI_GET_VIEWER_STRING_POINTER, |_, a| {
            (a[0] + a[1]).into_ret()
        });
        e.register(NI_GET_VIEWER_STRING_SHORT, |_, a| (a[0] + a[1]).into_ret());
        let l = log.clone();
        e.register_double(VIEWER_STRINGS_ADD, move |e, a| {
            l.borrow_mut().push((0, vec![a[0], e.mem.u32(a[1])]));
            Ret::default()
        });
        let this = controller(&mut e, 0x7000, 3);
        e.call(0x00c5_cd80, &args![this, 0x6000u32]);
        assert_eq!(
            calls(&log, 0),
            vec![
                vec![0x6000, 0x0101_0001],
                vec![0x6000, MASTER_LABEL + 0x7000],
                vec![0x6000, MAX_FULL_EMITTERS_LABEL + 3],
            ]
        );
        assert_eq!(
            calls(&log, EMITTER_CTLR_GET_VIEWER_STRINGS),
            vec![vec![this.addr(), 0x6000]]
        );
    }
}
