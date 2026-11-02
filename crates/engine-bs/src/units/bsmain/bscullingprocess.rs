//! `BSMain/bscullingprocess.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BSCullingProcess` (0xC8 bytes on PC: `BSSceneGraph`'s constructor
//! allocates 0xC8 before calling the constructor `004a0eb0`) is an
//! `NiCullingProcess` with a culling mode, a small stack of saved modes,
//! an optional `BSCompoundFrustum` and the accumulator that geometry is
//! registered with. The Xbox build keeps SIMD plane copies in the base
//! class, so the PC offsets of the `BSCullingProcess` fields are the Xbox
//! PDB's minus 0x90.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiAVObject;

layout! {
    /// `BSCullingProcess` (Xbox PDB), the fields this unit uses, PC offsets.
    pub struct BSCullingProcess: 0xc8 {
        /// `NiCullingProcess::m_pkVisibleSet` (Xbox PDB): the third argument
        /// of `Process`, swapped in for one call.
        0x08 m_pkVisibleSet: u32,
        /// `NiCullingProcess::m_pkCamera` (Xbox PDB): `Process`'s camera.
        0x0C m_pkCamera: u32,
        /// `NiCullingProcess::m_kPlanes.m_uiActivePlanes` (Xbox PDB,
        /// `NiFrustumPlanes` +0x60): non-zero while planes are active.
        0x8C m_uiActivePlanes: u32,
        /// `kCullMode` (Xbox PDB): 1 and 2 skip the culling test (1 appends
        /// everything, 2 nothing); 3 ignores the compound frustum.
        0x90 kCullMode: u32,
        /// `eTypeStackA` (Xbox PDB): ten saved modes.
        0x94 eTypeStackA: u32,
        /// `iCTStackIndex` (Xbox PDB): the number of saved modes.
        0xBC iCTStackIndex: u32,
        /// `pCompoundFrustum` (Xbox PDB): `BSCompoundFrustum*`.
        0xC0 pCompoundFrustum: Ptr,
        /// `spAccumulator` (Xbox PDB): `NiPointer<NiAccumulator>`.
        0xC4 spAccumulator: u32,
    }
}

/// `NiCullingProcess::DoCulling` (engine map, `00a694e0`): `(this, object)`.
const DO_CULLING: u32 = 0x00a6_94e0;
/// `BSCompoundFrustum::GetActivePlaneState` / `Process` /
/// `SetActivePlaneState` (engine map).
const COMPOUND_GET_ACTIVE_PLANE_STATE: u32 = 0x00c4_9050;
const COMPOUND_PROCESS: u32 = 0x00c4_91a0;
const COMPOUND_SET_ACTIVE_PLANE_STATE: u32 = 0x00c4_9100;
/// `BSCompoundFrustum::iFreeOp` (Xbox PDB): its operations in use.
const COMPOUND_FREE_OP: u32 = 0x28;
/// `00a694a0(this, &frustum)`: sets the process's frustum.
const SET_FRUSTUM: u32 = 0x00a6_94a0;
/// `00c46270(this, &source)`: copies the `NiPointer` at `source` into the
/// one at `this` (here the accumulator into a temporary).
const NI_POINTER_COPY: u32 = 0x00c4_6270;
/// `0066b0d0(this, value)`: assigns a pointer to the `NiPointer` at `this`.
const NI_POINTER_ASSIGN_VALUE: u32 = 0x0066_b0d0;
/// The global whose object's +8 word is the fallback accumulator.
const DEFAULT_ACCUMULATOR_HOLDER: u32 = 0x011f_4748;
/// `NiAVObject::Cull` (engine map, `00a59e00`): `(this, process)`.
const AV_OBJECT_CULL: u32 = 0x00a5_9e00;
/// `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `00a29680` (`cdecl`, a format and its arguments): the debug print.
const DEBUG_PRINT: u32 = 0x00a2_9680;
/// `"NiCullingProcess::Append> Geometry failed RegisterObject, not
/// rendering: %s\n"`.
const APPEND_FAILED_FORMAT: u32 = 0x0109_c2e0;
/// `""`.
const EMPTY_TEXT: u32 = 0x0101_f7a8;

/// `NiAVObject::m_uFlags` bits tested by `Process`.
const FLAG_BIT_12: u32 = 0x1000;
const FLAG_BIT_20: u32 = 0x10_0000;
const FLAG_BIT_11: u32 = 0x800;

/// Appends `object` through its virtual slot +0xD4 (with the process).
fn append(e: &mut Engine, this: Ptr<BSCullingProcess>, object: Ptr<NiAVObject>) {
    e.vcall(object.addr(), 0xd4, &args![this]);
}

// Translated from 00c4ee90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSCullingProcess::Process` (Xbox PDB), for one object. Mode 1 appends
/// it, mode 2 does nothing. Otherwise, with flag 0x1000 set the object is
/// appended unless flag 0x100000 is set too. Without it: when there is no
/// compound frustum in use (none, `iFreeOp` 0, or mode 3), it is culled
/// (`DoCulling`) when flag 0x800 is clear and planes are active, appended
/// otherwise; with the compound frustum, flag 0x800 appends it, and
/// otherwise the compound frustum decides (appending on a true result),
/// its active-plane state saved and restored around the test.
pub fn bs_culling_process_process(
    e: &mut Engine,
    this: Ptr<BSCullingProcess>,
    object: Ptr<NiAVObject>,
) {
    let mode = e.get(this, BSCullingProcess::kCullMode);
    if mode == 2 || mode == 1 {
        if e.get(this, BSCullingProcess::kCullMode) == 1 {
            append(e, this, object);
        }
        return;
    }
    let flags = e.get(object, NiAVObject::m_uFlags);
    if flags & FLAG_BIT_12 != 0 {
        if e.get(object, NiAVObject::m_uFlags) & FLAG_BIT_20 == 0 {
            append(e, this, object);
        }
        return;
    }
    let frustum_in_use = |e: &mut Engine| {
        let frustum = e.get(this, BSCullingProcess::pCompoundFrustum);
        !frustum.is_null() && e.mem.u32(frustum.addr() + COMPOUND_FREE_OP) != 0
    };
    if !frustum_in_use(e) || e.get(this, BSCullingProcess::kCullMode) == 3 {
        if e.get(object, NiAVObject::m_uFlags) & FLAG_BIT_11 == 0
            && e.get(this, BSCullingProcess::m_uiActivePlanes) != 0
        {
            e.call(DO_CULLING, &args![this, object]);
        } else {
            append(e, this, object);
        }
        return;
    }
    if e.get(object, NiAVObject::m_uFlags) & FLAG_BIT_11 != 0
        || (!frustum_in_use(e) && e.get(this, BSCullingProcess::m_uiActivePlanes) == 0)
    {
        append(e, this, object);
        return;
    }
    let frustum = e.get(this, BSCullingProcess::pCompoundFrustum);
    let state = e
        .call(COMPOUND_GET_ACTIVE_PLANE_STATE, &args![frustum])
        .u32();
    let frustum = e.get(this, BSCullingProcess::pCompoundFrustum);
    if e.call(COMPOUND_PROCESS, &args![frustum, object]).bool() {
        append(e, this, object);
    }
    let frustum = e.get(this, BSCullingProcess::pCompoundFrustum);
    e.call(COMPOUND_SET_ACTIVE_PLANE_STATE, &args![frustum, state]);
}

// Translated from 00c4f070 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c4f070 - FS:[0] exception frame (unwinding of the NiPointer temporary) not translated
/// `BSCullingProcess::Process_ov2` (Xbox PDB): culls `scene` seen from
/// `camera` when both are non-null. Stores the camera, sets the frustum
/// from the camera's (+0xDC), and swaps in `visible_set` for the call when
/// it is non-null. Without a visible set, the accumulator (copied into an
/// `NiPointer` temporary, or the object at `011f4748`'s +8 word when there
/// is none) gets the camera through its virtual slot +0x8C. Then
/// `NiAVObject::Cull` runs on the scene, the old visible set comes back and
/// the temporary is released.
pub fn bs_culling_process_process_ov2(
    e: &mut Engine,
    this: Ptr<BSCullingProcess>,
    camera: u32,
    scene: u32,
    visible_set: u32,
) {
    if camera == 0 || scene == 0 {
        return;
    }
    e.set(this, BSCullingProcess::m_pkCamera, camera);
    let camera_frustum = e.get(this, BSCullingProcess::m_pkCamera).wrapping_add(0xdc);
    e.call(SET_FRUSTUM, &args![this, camera_frustum]);
    let mut saved = 0;
    if visible_set != 0 {
        saved = e.get(this, BSCullingProcess::m_pkVisibleSet);
        e.set(this, BSCullingProcess::m_pkVisibleSet, visible_set);
    }
    e.with_stack(4, |e, accumulator| {
        // The temporary starts null; the exe still tests it.
        let initial = e.mem.u32(accumulator.addr());
        if initial != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![initial.wrapping_add(4)]);
        }
        if e.get(this, BSCullingProcess::m_pkVisibleSet) == 0 {
            let own = this.addr() + BSCullingProcess::spAccumulator.off;
            e.call(NI_POINTER_COPY, &args![accumulator, own]);
            if e.mem.u32(accumulator.addr()) == 0 {
                let holder: u32 = e.global(DEFAULT_ACCUMULATOR_HOLDER);
                let fallback = e.mem.u32(holder.wrapping_add(8));
                e.call(NI_POINTER_ASSIGN_VALUE, &args![accumulator, fallback]);
            }
            let current = e.mem.u32(accumulator.addr());
            if current != 0 {
                e.vcall(current, 0x8c, &args![camera]);
            }
        }
        e.call(AV_OBJECT_CULL, &args![scene, this]);
        if visible_set != 0 {
            e.set(this, BSCullingProcess::m_pkVisibleSet, saved);
        }
        let current = e.mem.u32(accumulator.addr());
        if current != 0 {
            let count = e
                .call(INTERLOCKED_DECREMENT, &args![current.wrapping_add(4)])
                .u32();
            if count == 0 {
                e.vcall(current, 4, &[]);
            }
        }
    });
}

// Translated from 00c4f1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSCullingProcess::AppendVirtual` (Xbox PDB): registers `geometry` with
/// the accumulator (virtual slot +0x98, a `bool`); when there is no
/// accumulator or it refuses, prints "Geometry failed RegisterObject" with
/// the geometry's name (+8, `""` when null).
pub fn bs_culling_process_append_virtual(
    e: &mut Engine,
    this: Ptr<BSCullingProcess>,
    geometry: u32,
) {
    let mut registered = false;
    let accumulator = e.get(this, BSCullingProcess::spAccumulator);
    if accumulator != 0 {
        let accumulator = e.get(this, BSCullingProcess::spAccumulator);
        if e.vcall(accumulator, 0x98, &args![geometry]).bool() {
            registered = true;
        }
    }
    if !registered {
        let name = e.mem.u32(geometry.wrapping_add(8));
        let name = if name != 0 { name } else { EMPTY_TEXT };
        e.call(DEBUG_PRINT, &args![APPEND_FAILED_FORMAT, name]);
    }
}

// Translated from 00c4f270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Pushes the current culling mode on the mode stack and makes `mode`
/// current.
pub fn fn_00c4f270(e: &mut Engine, this: Ptr<BSCullingProcess>, mode: u32) {
    let current = e.get(this, BSCullingProcess::kCullMode);
    let index = e.get(this, BSCullingProcess::iCTStackIndex);
    let slot = this
        .addr()
        .wrapping_add(index.wrapping_mul(4))
        .wrapping_add(BSCullingProcess::eTypeStackA.off);
    e.mem.set_u32(slot, current);
    let index = e.get(this, BSCullingProcess::iCTStackIndex);
    e.set(this, BSCullingProcess::iCTStackIndex, index.wrapping_add(1));
    e.set(this, BSCullingProcess::kCullMode, mode);
}

// Translated from 00c4f2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Pops the last saved culling mode back into the current one.
pub fn fn_00c4f2d0(e: &mut Engine, this: Ptr<BSCullingProcess>) {
    let index = e.get(this, BSCullingProcess::iCTStackIndex);
    e.set(this, BSCullingProcess::iCTStackIndex, index.wrapping_sub(1));
    let index = e.get(this, BSCullingProcess::iCTStackIndex);
    let slot = this
        .addr()
        .wrapping_add(index.wrapping_mul(4))
        .wrapping_add(BSCullingProcess::eTypeStackA.off);
    let mode = e.mem.u32(slot);
    e.set(this, BSCullingProcess::kCullMode, mode);
}

// Translated from 00c4f320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSCullingProcess::AddShared` (Xbox PDB; `bool` there): hands `object` to
/// the accumulator's virtual slot +0x9C and answers its value (the exe ends on
/// that call and leaves its `EAX`; the callers test `AL`).
pub fn bs_culling_process_add_shared(
    e: &mut Engine,
    this: Ptr<BSCullingProcess>,
    object: u32,
) -> bool {
    let accumulator = e.get(this, BSCullingProcess::spAccumulator);
    e.vcall(accumulator, 0x9c, &args![object]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c4ee90,
            bs_culling_process_process(Ptr<BSCullingProcess>, Ptr<NiAVObject>)
        ),
        entry!(
            0x00c4f070,
            bs_culling_process_process_ov2(Ptr<BSCullingProcess>, u32, u32, u32)
        ),
        entry!(
            0x00c4f1d0,
            bs_culling_process_append_virtual(Ptr<BSCullingProcess>, u32)
        ),
        entry!(0x00c4f270, fn_00c4f270(Ptr<BSCullingProcess>, u32)),
        entry!(0x00c4f2d0, fn_00c4f2d0(Ptr<BSCullingProcess>)),
        entry!(
            0x00c4f320,
            bs_culling_process_add_shared(Ptr<BSCullingProcess>, u32) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    const APPEND: u32 = 0x0050_00d4;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn calls_to(log: &Log, addr: u32) -> usize {
        log.iter().filter(|(a, _)| *a == addr).count()
    }

    /// A process and an object whose slot +0xD4 is the append double.
    fn setup() -> (Engine, Ptr<BSCullingProcess>, Ptr<NiAVObject>) {
        let mut e = Engine::new();
        let mut table = vec![0u32; 0x36];
        table[0xd4 / 4] = APPEND;
        e.put_vtable(0x0090_0000, &table);
        e.register(APPEND, |_, _| Ret::default());
        e.register(DO_CULLING, |_, _| Ret::default());
        e.register(COMPOUND_GET_ACTIVE_PLANE_STATE, |_, _| ret(0x5a));
        e.register(COMPOUND_SET_ACTIVE_PLANE_STATE, |_, _| Ret::default());
        let this: Ptr<BSCullingProcess> = e.new_object();
        let object = Ptr::new(e.mem.alloc(0x40));
        e.mem.set_u32(object.addr(), 0x0090_0000);
        (e, this, object)
    }

    /// Runs `Process` and returns the calls it made (without the top one).
    fn run(e: &mut Engine, this: Ptr<BSCullingProcess>, object: Ptr<NiAVObject>) -> Log {
        e.call_log = Some(vec![]);
        e.call(0x00c4_ee90, &args![this, object]);
        let mut log = e.call_log.take().unwrap();
        log.remove(0);
        log
    }

    #[test]
    fn process_modes_one_and_two() {
        let (mut e, this, object) = setup();
        e.set(this, BSCullingProcess::kCullMode, 1);
        let log = run(&mut e, this, object);
        assert_eq!(log, vec![(APPEND, vec![object.addr(), this.addr()])]);
        e.set(this, BSCullingProcess::kCullMode, 2);
        assert!(run(&mut e, this, object).is_empty());
    }

    #[test]
    fn process_flag_0x1000_appends_unless_0x100000() {
        let (mut e, this, object) = setup();
        e.set(object, NiAVObject::m_uFlags, 0x1000);
        assert_eq!(calls_to(&run(&mut e, this, object), APPEND), 1);
        e.set(object, NiAVObject::m_uFlags, 0x10_1000);
        assert!(run(&mut e, this, object).is_empty());
    }

    #[test]
    fn process_without_compound_frustum_culls_or_appends() {
        let (mut e, this, object) = setup();
        // Planes active, flag 0x800 clear: DoCulling.
        e.set(this, BSCullingProcess::m_uiActivePlanes, 6);
        let log = run(&mut e, this, object);
        assert_eq!(log, vec![(DO_CULLING, vec![this.addr(), object.addr()])]);
        // Flag 0x800: append.
        e.set(object, NiAVObject::m_uFlags, 0x800);
        assert_eq!(calls_to(&run(&mut e, this, object), APPEND), 1);
        // No active planes: append.
        e.set(object, NiAVObject::m_uFlags, 0);
        e.set(this, BSCullingProcess::m_uiActivePlanes, 0);
        assert_eq!(calls_to(&run(&mut e, this, object), APPEND), 1);
        // A compound frustum with no operations counts as none; so does mode 3.
        let frustum = Ptr::new(e.mem.alloc(0xa4));
        e.set(this, BSCullingProcess::pCompoundFrustum, frustum);
        e.set(this, BSCullingProcess::m_uiActivePlanes, 6);
        assert_eq!(calls_to(&run(&mut e, this, object), DO_CULLING), 1);
        e.mem.set_u32(frustum.addr() + COMPOUND_FREE_OP, 2);
        e.set(this, BSCullingProcess::kCullMode, 3);
        assert_eq!(calls_to(&run(&mut e, this, object), DO_CULLING), 1);
    }

    #[test]
    fn process_with_compound_frustum_asks_it() {
        let (mut e, this, object) = setup();
        let frustum = Ptr::new(e.mem.alloc(0xa4));
        e.mem.set_u32(frustum.addr() + COMPOUND_FREE_OP, 1);
        e.set(this, BSCullingProcess::pCompoundFrustum, frustum);
        // Visible: appended, the plane state restored.
        e.register(COMPOUND_PROCESS, |_, _| ret(0xffff_ff01));
        let log = run(&mut e, this, object);
        let f = frustum.addr();
        assert_eq!(
            log,
            vec![
                (COMPOUND_GET_ACTIVE_PLANE_STATE, vec![f]),
                (COMPOUND_PROCESS, vec![f, object.addr()]),
                (APPEND, vec![object.addr(), this.addr()]),
                (COMPOUND_SET_ACTIVE_PLANE_STATE, vec![f, 0x5a]),
            ]
        );
        // Not visible (only the low byte counts).
        e.register(COMPOUND_PROCESS, |_, _| ret(0x100));
        assert_eq!(calls_to(&run(&mut e, this, object), APPEND), 0);
        // Flag 0x800: appended without asking.
        e.set(object, NiAVObject::m_uFlags, 0x800);
        let log = run(&mut e, this, object);
        assert_eq!(log, vec![(APPEND, vec![object.addr(), this.addr()])]);
    }

    /// Doubles for `Process_ov2`: the frustum setter, the culling call, the
    /// `NiPointer` copy and assignment, reference counting, and an
    /// accumulator whose slot +0x8C records the camera.
    fn ov2_engine() -> (Engine, Ptr<BSCullingProcess>, u32) {
        let mut e = Engine::new();
        e.register(SET_FRUSTUM, |_, _| Ret::default());
        e.register(AV_OBJECT_CULL, |_, _| Ret::default());
        e.register(NI_POINTER_COPY, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            if value != 0 {
                let n = e.mem.u32(value + 4) + 1;
                e.mem.set_u32(value + 4, n);
            }
            Ret::default()
        });
        e.register(NI_POINTER_ASSIGN_VALUE, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            if a[1] != 0 {
                let n = e.mem.u32(a[1] + 4) + 1;
                e.mem.set_u32(a[1] + 4, n);
            }
            Ret::default()
        });
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let n = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], n);
            ret(n)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let n = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], n);
            ret(n)
        });
        let mut table = vec![0u32; 0x24];
        table[0x8c / 4] = 0x0050_008c;
        table[1] = 0x0050_0004;
        e.put_vtable(0x0090_0000, &table);
        e.register(0x0050_008c, |_, _| Ret::default());
        e.register(0x0050_0004, |_, _| Ret::default());
        let accumulator = e.mem.alloc(0x10);
        e.mem.set_u32(accumulator, 0x0090_0000);
        e.mem.set_u32(accumulator + 4, 1);
        let this: Ptr<BSCullingProcess> = e.new_object();
        (e, this, accumulator)
    }

    #[test]
    fn process_ov2_with_own_accumulator() {
        let (mut e, this, accumulator) = ov2_engine();
        e.set(this, BSCullingProcess::spAccumulator, accumulator);
        e.call_log = Some(vec![]);
        e.call(0x00c4_f070, &args![this, 0x7000u32, 0x8000u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(this, BSCullingProcess::m_pkCamera), 0x7000);
        assert_eq!(log[1], (SET_FRUSTUM, vec![this.addr(), 0x70dc]));
        assert!(log.contains(&(0x0050_008c, vec![accumulator, 0x7000])));
        assert!(log.contains(&(AV_OBJECT_CULL, vec![0x8000, this.addr()])));
        assert_eq!(calls_to(&log, NI_POINTER_ASSIGN_VALUE), 0);
        // The temporary's reference is gone again.
        assert_eq!(e.mem.u32(accumulator + 4), 1);
    }

    #[test]
    fn process_ov2_falls_back_to_the_global_accumulator() {
        let (mut e, this, accumulator) = ov2_engine();
        e.map(0x011f_4000, 0x1000);
        let holder = e.mem.alloc(0x10);
        e.mem.set_u32(holder + 8, accumulator);
        e.set_global(DEFAULT_ACCUMULATOR_HOLDER, holder);
        e.call_log = Some(vec![]);
        e.call(0x00c4_f070, &args![this, 0x7000u32, 0x8000u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(NI_POINTER_ASSIGN_VALUE, vec![log[2].1[0], accumulator])));
        assert!(log.contains(&(0x0050_008c, vec![accumulator, 0x7000])));
        assert_eq!(e.mem.u32(accumulator + 4), 1);
    }

    #[test]
    fn process_ov2_with_a_visible_set_swaps_it_in_for_the_call() {
        let (mut e, this, accumulator) = ov2_engine();
        e.set(this, BSCullingProcess::spAccumulator, accumulator);
        e.set(this, BSCullingProcess::m_pkVisibleSet, 0x1234);
        e.register(AV_OBJECT_CULL, |e, a| {
            assert_eq!(e.mem.u32(a[1] + 8), 0x4321);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c4_f070, &args![this, 0x7000u32, 0x8000u32, 0x4321u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(this, BSCullingProcess::m_pkVisibleSet), 0x1234);
        assert_eq!(calls_to(&log, NI_POINTER_COPY), 0);
        assert_eq!(calls_to(&log, 0x0050_008c), 0);
        // Null camera or scene: nothing.
        e.call_log = Some(vec![]);
        e.call(0x00c4_f070, &args![this, 0u32, 0x8000u32, 0u32]);
        e.call(0x00c4_f070, &args![this, 0x7000u32, 0u32, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn append_virtual_reports_refused_geometry() {
        let mut e = Engine::new();
        let mut table = vec![0u32; 0x28];
        table[0x98 / 4] = 0x0050_0098;
        e.put_vtable(0x0090_0000, &table);
        e.register(DEBUG_PRINT, |_, _| Ret::default());
        let this: Ptr<BSCullingProcess> = e.new_object();
        let geometry = e.mem.alloc(0x10);
        e.mem.set_u32(geometry + 8, 0x4444);
        // No accumulator.
        e.call_log = Some(vec![]);
        e.call(0x00c4_f1d0, &args![this, geometry]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (DEBUG_PRINT, vec![APPEND_FAILED_FORMAT, 0x4444]));
        // Accepted (low byte 1): no print.
        let accumulator = e.mem.alloc(8);
        e.mem.set_u32(accumulator, 0x0090_0000);
        e.set(this, BSCullingProcess::spAccumulator, accumulator);
        e.register(0x0050_0098, |_, _| ret(0x201));
        e.call_log = Some(vec![]);
        e.call(0x00c4_f1d0, &args![this, geometry]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (0x0050_0098, vec![accumulator, geometry]));
        assert_eq!(calls_to(&log, DEBUG_PRINT), 0);
        // Refused, unnamed geometry: the empty text.
        e.register(0x0050_0098, |_, _| ret(0x200));
        e.mem.set_u32(geometry + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c4_f1d0, &args![this, geometry]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[2],
            (DEBUG_PRINT, vec![APPEND_FAILED_FORMAT, EMPTY_TEXT])
        );
    }

    #[test]
    fn mode_stack_push_and_pop() {
        let mut e = Engine::new();
        let this: Ptr<BSCullingProcess> = e.new_object();
        e.set(this, BSCullingProcess::kCullMode, 4);
        e.call(0x00c4_f270, &args![this, 1u32]);
        e.call(0x00c4_f270, &args![this, 2u32]);
        assert_eq!(e.get(this, BSCullingProcess::kCullMode), 2);
        assert_eq!(e.get(this, BSCullingProcess::iCTStackIndex), 2);
        assert_eq!(e.mem.u32(this.addr() + 0x94), 4);
        assert_eq!(e.mem.u32(this.addr() + 0x98), 1);
        e.call(0x00c4_f2d0, &args![this]);
        assert_eq!(e.get(this, BSCullingProcess::kCullMode), 1);
        e.call(0x00c4_f2d0, &args![this]);
        assert_eq!(e.get(this, BSCullingProcess::kCullMode), 4);
        assert_eq!(e.get(this, BSCullingProcess::iCTStackIndex), 0);
    }

    #[test]
    fn add_shared_forwards_to_the_accumulator() {
        let mut e = Engine::new();
        let mut table = vec![0u32; 0x28];
        table[0x9c / 4] = 0x0050_009c;
        e.put_vtable(0x0090_0000, &table);
        e.register(0x0050_009c, |_, _| Ret::default());
        let this: Ptr<BSCullingProcess> = e.new_object();
        let accumulator = e.mem.alloc(8);
        e.mem.set_u32(accumulator, 0x0090_0000);
        e.set(this, BSCullingProcess::spAccumulator, accumulator);
        e.call_log = Some(vec![]);
        e.call(0x00c4_f320, &args![this, 0x55u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (0x0050_009c, vec![accumulator, 0x55]));
    }

    #[test]
    fn add_shared_answers_the_accumulators_value() {
        let mut e = Engine::new();
        let mut table = vec![0u32; 0x28];
        table[0x9c / 4] = 0x0050_009c;
        e.put_vtable(0x0090_0000, &table);
        e.register(0x0050_009c, |_, _| true.into_ret());
        let this: Ptr<BSCullingProcess> = e.new_object();
        let accumulator = e.mem.alloc(8);
        e.mem.set_u32(accumulator, 0x0090_0000);
        e.set(this, BSCullingProcess::spAccumulator, accumulator);
        assert!(e.call(0x00c4_f320, &args![this, 0x55u32]).bool());
    }
}
