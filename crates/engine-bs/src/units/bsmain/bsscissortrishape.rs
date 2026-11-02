//! `BSMain/bsscissortrishape.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BSScissorTriShape` is an `NiTriShape` (0xD4 bytes on PC) with a
//! rectangle (`kRect`, a Windows `RECT`: left, top, right, bottom). Its
//! `RenderImmediate` switches a render state on the renderer's device and
//! hands it the rectangle around the `NiTriShape` drawing when the
//! rectangle passes the test described there.
//!
//! The C++ exception-unwinding frame of `CreateClone` is not translated;
//! it carries the `Translation incomplete` marker.

#[allow(unused_imports)]
use crate::prelude::*;

/// Address of the `BSScissorTriShape` vtable.
pub(crate) const BS_SCISSOR_TRI_SHAPE_VTABLE: u32 = 0x010c_2e7c;
/// Address of `BSScissorTriShape`'s run-time type information.
pub(crate) const BS_SCISSOR_TRI_SHAPE_RTTI: u32 = 0x0120_3348;
/// Size of a `BSScissorTriShape` on PC (the Xbox build is 0x100).
pub(crate) const BS_SCISSOR_TRI_SHAPE_SIZE: u32 = 0xd4;

/// The allocator `00aa13e0` (cdecl, size): returns the block.
const ALLOCATE: u32 = 0x00aa_13e0;
/// The sized free `00aa1460` (cdecl, block, size).
const FREE_SIZED: u32 = 0x00aa_1460;
/// The `NiTriShape` constructor without arguments that `00c5ceb0` calls
/// (`00a744a0`, thiscall).
const NI_TRI_SHAPE_CONSTRUCT_DEFAULT: u32 = 0x00a7_44a0;
/// `NiTriShape::NiTriShape` (Xbox PDB, `00a74410`): thiscall, nine words.
const NI_TRI_SHAPE_CONSTRUCT: u32 = 0x00a7_4410;
/// The `NiTriShape` destructor body the `BSScissorTriShape` one ends with
/// (`00a89760`, thiscall, no arguments).
const NI_TRI_SHAPE_DESTRUCT: u32 = 0x00a8_9760;
/// The `NiTriShape` `CopyMembers(this, target, cloneProcess)` that
/// `00c5d0a0` ends with (`00a744c0`).
const NI_TRI_SHAPE_COPY_MEMBERS: u32 = 0x00a7_44c0;
/// `NiTriShape::GetViewerStrings(this, strings)` (Xbox PDB).
const NI_TRI_SHAPE_GET_VIEWER_STRINGS: u32 = 0x00a7_4690;
/// `NiTriShape::RenderImmediate(this, renderer)` (Xbox PDB).
const NI_TRI_SHAPE_RENDER_IMMEDIATE: u32 = 0x00a7_45c0;

/// Offset in the renderer of the device object `RenderImmediate` calls.
const RENDERER_DEVICE: u32 = 0x288;
/// The device's vtable slot called with a state number and a value. The
/// device is a COM interface (`this` is pushed); in the Direct3D 9 device
/// interface this slot is `SetRenderState` (not confirmed from the exe).
const DEVICE_SLOT_SET_STATE: u32 = 0xe4;
/// The device's vtable slot called with the address of the rectangle; in
/// the Direct3D 9 device interface it is `SetScissorRect` (not confirmed
/// from the exe).
const DEVICE_SLOT_SET_RECTANGLE: u32 = 0x12c;
/// The state number passed to [`DEVICE_SLOT_SET_STATE`] (in Direct3D 9,
/// `D3DRS_SCISSORTESTENABLE`; not confirmed from the exe).
const DEVICE_STATE_0XAE: u32 = 0xae;

layout! {
    /// `BSScissorTriShape` (Xbox PDB), 0xD4 bytes on PC; `kRect` sits 0x2C
    /// lower than in the Xbox PDB (+0xF0 there).
    pub struct BSScissorTriShape: 0xd4 {
        /// `kRect.left` (Xbox PDB `kRect`, a `tagRECT`).
        0xC4 kRect_left: i32,
        /// `kRect.top`.
        0xC8 kRect_top: i32,
        /// `kRect.right`.
        0xCC kRect_right: i32,
        /// `kRect.bottom`.
        0xD0 kRect_bottom: i32,
    }
}

type Shape = Ptr<BSScissorTriShape>;

// Translated from 00c5ceb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSScissorTriShape` constructor without arguments: builds the
/// `NiTriShape` (`00a744a0`) and sets the vtable; the rectangle is left as
/// it is. Returns `this`.
pub fn fn_00c5ceb0(e: &mut Engine, this: Shape) -> Shape {
    e.call(NI_TRI_SHAPE_CONSTRUCT_DEFAULT, &args![this]);
    e.mem.set_u32(this.addr(), BS_SCISSOR_TRI_SHAPE_VTABLE);
    this
}

// Translated from 00c5cf30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScissorTriShape::BSScissorTriShape` (Xbox PDB): passes its first nine
/// words unchanged to `NiTriShape::NiTriShape` (the first, sixth and eighth
/// as 16-bit values), sets the vtable and stores the rectangle. Returns
/// `this`.
#[allow(clippy::too_many_arguments)]
pub fn bs_scissor_tri_shape_bs_scissor_tri_shape(
    e: &mut Engine,
    this: Shape,
    tri_shape_arg_1: u16,
    tri_shape_arg_2: u32,
    tri_shape_arg_3: u32,
    tri_shape_arg_4: u32,
    tri_shape_arg_5: u32,
    tri_shape_arg_6: u16,
    tri_shape_arg_7: u32,
    tri_shape_arg_8: u16,
    tri_shape_arg_9: u32,
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
) -> Shape {
    e.call(
        NI_TRI_SHAPE_CONSTRUCT,
        &args![
            this,
            tri_shape_arg_1,
            tri_shape_arg_2,
            tri_shape_arg_3,
            tri_shape_arg_4,
            tri_shape_arg_5,
            tri_shape_arg_6,
            tri_shape_arg_7,
            tri_shape_arg_8,
            tri_shape_arg_9
        ],
    );
    e.mem.set_u32(this.addr(), BS_SCISSOR_TRI_SHAPE_VTABLE);
    e.set(this, BSScissorTriShape::kRect_left, left);
    e.set(this, BSScissorTriShape::kRect_top, top);
    e.set(this, BSScissorTriShape::kRect_right, right);
    e.set(this, BSScissorTriShape::kRect_bottom, bottom);
    this
}

// Translated from 00c5cfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSScissorTriShape` destructor body: restores the vtable and runs the
/// `NiTriShape` one (`00a89760`).
pub fn fn_00c5cfb0(e: &mut Engine, this: Shape) {
    e.mem.set_u32(this.addr(), BS_SCISSOR_TRI_SHAPE_VTABLE);
    e.call(NI_TRI_SHAPE_DESTRUCT, &args![this]);
}

// Translated from 00c5cfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the rectangle (left, top, right, bottom). Ghidra's library match
/// names this body `CPane::SetBorders`; it is the four stores into `kRect`.
pub fn fn_00c5cfd0(e: &mut Engine, this: Shape, left: i32, top: i32, right: i32, bottom: i32) {
    e.set(this, BSScissorTriShape::kRect_left, left);
    e.set(this, BSScissorTriShape::kRect_top, top);
    e.set(this, BSScissorTriShape::kRect_right, right);
    e.set(this, BSScissorTriShape::kRect_bottom, bottom);
}

// Translated from 00c5d010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScissorTriShape::CreateClone` (Xbox PDB): allocates 0xD4 bytes and
/// constructs a shape in them with `00c5ceb0` (null if the allocation
/// failed), then copies this shape's members into it (`00c5d0a0`, which
/// also runs on a null result, as in the game).
// Translation incomplete: 00c5d010 - C++ exception unwinding frame is not translated.
pub fn bs_scissor_tri_shape_create_clone(e: &mut Engine, this: Shape, clone_process: u32) -> Shape {
    let block = e.call(ALLOCATE, &args![BS_SCISSOR_TRI_SHAPE_SIZE]).u32();
    let clone = if block == 0 {
        Ptr::NULL
    } else {
        fn_00c5ceb0(e, Ptr::new(block))
    };
    fn_00c5d0a0(e, this, clone, clone_process);
    clone
}

// Translated from 00c5d0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `CopyMembers` `CreateClone` calls: copies the rectangle into
/// `target`, then runs the `NiTriShape` one (`00a744c0`).
pub fn fn_00c5d0a0(e: &mut Engine, this: Shape, target: Shape, clone_process: u32) {
    for word in 0..4 {
        let value = e.mem.u32(this.addr() + 0xc4 + 4 * word);
        e.mem.set_u32(target.addr() + 0xc4 + 4 * word, value);
    }
    e.call(
        NI_TRI_SHAPE_COPY_MEMBERS,
        &args![this, target, clone_process],
    );
}

// Translated from 00c5d0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScissorTriShape::GetViewerStrings` (Xbox PDB): forwards to
/// `NiTriShape::GetViewerStrings`.
pub fn bs_scissor_tri_shape_get_viewer_strings(e: &mut Engine, this: Shape, strings: Ptr) {
    e.call(NI_TRI_SHAPE_GET_VIEWER_STRINGS, &args![this, strings]);
}

/// The device object of the renderer (re-read before every use, as the
/// game does).
fn renderer_device(e: &Engine, renderer: Ptr) -> u32 {
    e.mem.u32(renderer.addr() + RENDERER_DEVICE)
}

// Translated from 00c5d110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScissorTriShape::RenderImmediate` (Xbox PDB): draws the shape through
/// `NiTriShape::RenderImmediate`. Before it, when the rectangle is not all
/// zero and either both `left` and `right` are negative, both are above 1,
/// both `top` and `bottom` are negative, or both are above 1 (signed
/// comparisons), the renderer's device (`renderer + 0x288`) gets state
/// 0xAE set to 1 (slot 0xE4) and the rectangle's address (slot 0x12C); after
/// the drawing, state 0xAE goes back to 0.
pub fn bs_scissor_tri_shape_render_immediate(e: &mut Engine, this: Shape, renderer: Ptr) {
    let left = e.get(this, BSScissorTriShape::kRect_left);
    let top = e.get(this, BSScissorTriShape::kRect_top);
    let right = e.get(this, BSScissorTriShape::kRect_right);
    let bottom = e.get(this, BSScissorTriShape::kRect_bottom);
    let any_set = bottom != 0 || top != 0 || left != 0 || right != 0;
    let enable = any_set
        && ((left < 0 && right < 0)
            || (left > 1 && right > 1)
            || (top < 0 && bottom < 0)
            || (top > 1 && bottom > 1));
    if enable {
        let device = renderer_device(e, renderer);
        e.vcall(
            device,
            DEVICE_SLOT_SET_STATE,
            &args![DEVICE_STATE_0XAE, 1u32],
        );
        let device = renderer_device(e, renderer);
        e.vcall(
            device,
            DEVICE_SLOT_SET_RECTANGLE,
            &args![this.addr() + 0xc4],
        );
    }
    e.call(NI_TRI_SHAPE_RENDER_IMMEDIATE, &args![this, renderer]);
    if enable {
        let device = renderer_device(e, renderer);
        e.vcall(
            device,
            DEVICE_SLOT_SET_STATE,
            &args![DEVICE_STATE_0XAE, 0u32],
        );
    }
}

// Translated from 00c5d260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScissorTriShape::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor body and, when bit 0 of `flags` is set, frees the 0xD4 bytes.
/// Returns `this`.
pub fn bs_scissor_tri_shape_scalar_deleting_destructor(
    e: &mut Engine,
    this: Shape,
    flags: u32,
) -> Shape {
    fn_00c5cfb0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_SIZED, &args![this, BS_SCISSOR_TRI_SHAPE_SIZE]);
    }
    this
}

// Translated from 00c5d2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScissorTriShape::GetRTTI` (Xbox PDB): the address of the class's
/// run-time type information.
pub fn bs_scissor_tri_shape_get_rtti(_e: &mut Engine, _this: Shape) -> u32 {
    BS_SCISSOR_TRI_SHAPE_RTTI
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c5ceb0,
            fn_00c5ceb0(Ptr<BSScissorTriShape>) -> Ptr<BSScissorTriShape>
        ),
        entry!(
            0x00c5cf30,
            bs_scissor_tri_shape_bs_scissor_tri_shape(
                Ptr<BSScissorTriShape>,
                u16,
                u32,
                u32,
                u32,
                u32,
                u16,
                u32,
                u16,
                u32,
                i32,
                i32,
                i32,
                i32,
            ) -> Ptr<BSScissorTriShape>
        ),
        entry!(0x00c5cfb0, fn_00c5cfb0(Ptr<BSScissorTriShape>)),
        entry!(
            0x00c5cfd0,
            fn_00c5cfd0(Ptr<BSScissorTriShape>, i32, i32, i32, i32)
        ),
        entry!(
            0x00c5d010,
            bs_scissor_tri_shape_create_clone(
                Ptr<BSScissorTriShape>,
                u32,
            ) -> Ptr<BSScissorTriShape>
        ),
        entry!(
            0x00c5d0a0,
            fn_00c5d0a0(Ptr<BSScissorTriShape>, Ptr<BSScissorTriShape>, u32)
        ),
        entry!(
            0x00c5d0f0,
            bs_scissor_tri_shape_get_viewer_strings(Ptr<BSScissorTriShape>, Ptr)
        ),
        entry!(
            0x00c5d110,
            bs_scissor_tri_shape_render_immediate(Ptr<BSScissorTriShape>, Ptr)
        ),
        entry!(
            0x00c5d260,
            bs_scissor_tri_shape_scalar_deleting_destructor(
                Ptr<BSScissorTriShape>,
                u32,
            ) -> Ptr<BSScissorTriShape>
        ),
        entry!(
            0x00c5d2a0,
            bs_scissor_tri_shape_get_rtti(Ptr<BSScissorTriShape>) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Fake addresses for the device's vtable and the slot functions.
    const DEVICE_VTABLE: u32 = 0x7100_0000;
    const DEVICE_SET_STATE: u32 = 0x7100_1000;
    const DEVICE_SET_RECTANGLE: u32 = 0x7100_2000;

    type CallLog = Vec<(u32, Vec<u32>)>;

    fn test_engine() -> Engine {
        let mut e = Engine::new();
        for addr in [
            NI_TRI_SHAPE_CONSTRUCT_DEFAULT,
            NI_TRI_SHAPE_CONSTRUCT,
            NI_TRI_SHAPE_DESTRUCT,
            NI_TRI_SHAPE_COPY_MEMBERS,
            NI_TRI_SHAPE_GET_VIEWER_STRINGS,
            NI_TRI_SHAPE_RENDER_IMMEDIATE,
            FREE_SIZED,
            DEVICE_SET_STATE,
            DEVICE_SET_RECTANGLE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e
    }

    fn calls_to(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn set_rect(e: &mut Engine, shape: Shape, rect: [i32; 4]) {
        e.set(shape, BSScissorTriShape::kRect_left, rect[0]);
        e.set(shape, BSScissorTriShape::kRect_top, rect[1]);
        e.set(shape, BSScissorTriShape::kRect_right, rect[2]);
        e.set(shape, BSScissorTriShape::kRect_bottom, rect[3]);
    }

    #[test]
    fn default_constructor_sets_the_vtable() {
        let mut e = test_engine();
        let shape: Shape = e.new_object();
        e.call_log = Some(vec![]);
        let back = e
            .call(0x00c5_ceb0, &args![shape])
            .ptr::<BSScissorTriShape>();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, shape);
        assert_eq!(e.mem.u32(shape.addr()), BS_SCISSOR_TRI_SHAPE_VTABLE);
        assert_eq!(
            calls_to(&log, NI_TRI_SHAPE_CONSTRUCT_DEFAULT),
            vec![vec![shape.addr()]]
        );
    }

    #[test]
    fn constructor_forwards_nine_words_and_stores_the_rectangle() {
        let mut e = test_engine();
        let shape: Shape = e.new_object();
        e.call_log = Some(vec![]);
        let back = e
            .call(
                0x00c5_cf30,
                &args![
                    shape,
                    0xffff_0003u32,
                    2u32,
                    3u32,
                    4u32,
                    5u32,
                    0x1_0006u32,
                    7u32,
                    0x2_0008u32,
                    9u32,
                    -1i32,
                    2i32,
                    30i32,
                    40i32
                ],
            )
            .ptr::<BSScissorTriShape>();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, shape);
        assert_eq!(
            calls_to(&log, NI_TRI_SHAPE_CONSTRUCT),
            vec![vec![shape.addr(), 3, 2, 3, 4, 5, 6, 7, 8, 9]]
        );
        assert_eq!(e.mem.u32(shape.addr()), BS_SCISSOR_TRI_SHAPE_VTABLE);
        assert_eq!(e.get(shape, BSScissorTriShape::kRect_left), -1);
        assert_eq!(e.get(shape, BSScissorTriShape::kRect_top), 2);
        assert_eq!(e.get(shape, BSScissorTriShape::kRect_right), 30);
        assert_eq!(e.get(shape, BSScissorTriShape::kRect_bottom), 40);
    }

    #[test]
    fn destructor_body_restores_the_vtable_then_runs_the_base() {
        let mut e = test_engine();
        let shape: Shape = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x00c5_cfb0, &args![shape]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(shape.addr()), BS_SCISSOR_TRI_SHAPE_VTABLE);
        assert_eq!(
            calls_to(&log, NI_TRI_SHAPE_DESTRUCT),
            vec![vec![shape.addr()]]
        );
    }

    #[test]
    fn set_rectangle_stores_four_words() {
        let mut e = test_engine();
        let shape: Shape = e.new_object();
        e.call(0x00c5_cfd0, &args![shape, 1i32, 2i32, 3i32, 4i32]);
        assert_eq!(e.mem.i32(shape.addr() + 0xc4), 1);
        assert_eq!(e.mem.i32(shape.addr() + 0xc8), 2);
        assert_eq!(e.mem.i32(shape.addr() + 0xcc), 3);
        assert_eq!(e.mem.i32(shape.addr() + 0xd0), 4);
    }

    #[test]
    fn create_clone_constructs_and_copies_the_rectangle() {
        let mut e = test_engine();
        let shape: Shape = e.new_object();
        set_rect(&mut e, shape, [5, 6, 7, 8]);
        e.call_log = Some(vec![]);
        let clone = e
            .call(0x00c5_d010, &args![shape, 0x44u32])
            .ptr::<BSScissorTriShape>();
        let log = e.call_log.take().unwrap();
        assert!(!clone.is_null());
        assert_eq!(calls_to(&log, ALLOCATE), vec![vec![0xd4]]);
        assert_eq!(e.mem.u32(clone.addr()), BS_SCISSOR_TRI_SHAPE_VTABLE);
        assert_eq!(e.get(clone, BSScissorTriShape::kRect_left), 5);
        assert_eq!(e.get(clone, BSScissorTriShape::kRect_bottom), 8);
        assert_eq!(
            calls_to(&log, NI_TRI_SHAPE_COPY_MEMBERS),
            vec![vec![shape.addr(), clone.addr(), 0x44]]
        );
    }

    #[test]
    fn create_clone_without_memory_constructs_nothing() {
        let mut e = test_engine();
        e.register(ALLOCATE, |_, _| Ret::default());
        // `CopyMembers` writes into the null clone, as in the game: give
        // the page at 0 to the test so the stores land.
        e.map(0, 0x1000);
        let shape: Shape = e.new_object();
        e.call_log = Some(vec![]);
        let clone = e
            .call(0x00c5_d010, &args![shape, 1u32])
            .ptr::<BSScissorTriShape>();
        let log = e.call_log.take().unwrap();
        assert!(clone.is_null());
        assert!(calls_to(&log, NI_TRI_SHAPE_CONSTRUCT_DEFAULT).is_empty());
        assert_eq!(
            calls_to(&log, NI_TRI_SHAPE_COPY_MEMBERS),
            vec![vec![shape.addr(), 0, 1]]
        );
    }

    #[test]
    fn get_viewer_strings_forwards() {
        let mut e = test_engine();
        e.call_log = Some(vec![]);
        e.call(0x00c5_d0f0, &args![0x10u32, 0x20u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, NI_TRI_SHAPE_GET_VIEWER_STRINGS),
            vec![vec![0x10, 0x20]]
        );
    }

    /// Runs `RenderImmediate` with `rect` and returns the device calls.
    fn render_with(rect: [i32; 4]) -> (Shape, u32, CallLog) {
        let mut e = test_engine();
        let mut table = vec![0u32; 0x50];
        table[(DEVICE_SLOT_SET_STATE / 4) as usize] = DEVICE_SET_STATE;
        table[(DEVICE_SLOT_SET_RECTANGLE / 4) as usize] = DEVICE_SET_RECTANGLE;
        e.put_vtable(DEVICE_VTABLE, &table);
        let device = e.mem.alloc(8);
        e.mem.set_u32(device, DEVICE_VTABLE);
        let renderer = e.mem.alloc(0x290);
        e.mem.set_u32(renderer + RENDERER_DEVICE, device);
        let shape: Shape = e.new_object();
        set_rect(&mut e, shape, rect);
        e.call_log = Some(vec![]);
        e.call(0x00c5_d110, &args![shape, renderer]);
        let log = e.call_log.take().unwrap();
        let log = log
            .into_iter()
            .filter(|(a, _)| {
                [
                    DEVICE_SET_STATE,
                    DEVICE_SET_RECTANGLE,
                    NI_TRI_SHAPE_RENDER_IMMEDIATE,
                ]
                .contains(a)
            })
            .collect();
        (shape, device, log)
    }

    #[test]
    fn render_immediate_wraps_the_drawing_when_the_rectangle_qualifies() {
        for rect in [[-1, 0, -2, 0], [2, 0, 3, 0], [0, -1, 0, -1], [0, 2, 0, 5]] {
            let (shape, device, log) = render_with(rect);
            assert_eq!(log.len(), 4, "{rect:?}");
            assert_eq!(log[0], (DEVICE_SET_STATE, vec![device, 0xae, 1]));
            assert_eq!(
                log[1],
                (DEVICE_SET_RECTANGLE, vec![device, shape.addr() + 0xc4])
            );
            assert_eq!(log[2].0, NI_TRI_SHAPE_RENDER_IMMEDIATE);
            assert_eq!(log[3], (DEVICE_SET_STATE, vec![device, 0xae, 0]));
        }
    }

    #[test]
    fn render_immediate_draws_plainly_otherwise() {
        for rect in [[0, 0, 0, 0], [0, 0, 100, 100], [-1, 0, 5, 0], [1, 1, 1, 1]] {
            let (shape, _, log) = render_with(rect);
            assert_eq!(log.len(), 1, "{rect:?}");
            assert_eq!(log[0].0, NI_TRI_SHAPE_RENDER_IMMEDIATE);
            assert_eq!(log[0].1[0], shape.addr());
        }
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = test_engine();
        let shape: Shape = e.new_object();
        e.call_log = Some(vec![]);
        let back = e
            .call(0x00c5_d260, &args![shape, 0u32])
            .ptr::<BSScissorTriShape>();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, shape);
        assert_eq!(calls_to(&log, NI_TRI_SHAPE_DESTRUCT).len(), 1);
        assert!(calls_to(&log, FREE_SIZED).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x00c5_d260, &args![shape, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, FREE_SIZED), vec![vec![shape.addr(), 0xd4]]);
    }

    #[test]
    fn get_rtti_is_the_class_block() {
        let mut e = test_engine();
        assert_eq!(e.call(0x00c5_d2a0, &args![0u32]).u32(), 0x0120_3348);
    }
}
