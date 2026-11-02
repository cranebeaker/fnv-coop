//! `Havok SDK/hkawaveletcompressedanimationclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00dfb7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00df_b720`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dfb7b0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00df_b720, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkStringPtr` destructor (`00cb0c00`, thiscall on the pointer word).
const STRING_PTR_DESTRUCT: u32 = 0x00cb_0c00;
/// `hkReferencedObject::removeReference` (`00c905b0`, thiscall).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// The text builder of the log messages: constructor `00cb1600` (thiscall, `(buffer, size)`),
/// `operator<<(const char*)` `00cc01c0` (`(text)`) and the destructor `00cc00e0`.
const TEXT_BUILDER_CONSTRUCT: u32 = 0x00cb_1600;
const TEXT_BUILDER_APPEND: u32 = 0x00cc_01c0;
const TEXT_BUILDER_DESTRUCT: u32 = 0x00cc_00e0;
/// The report sink (slot `0xc`: `(level, id, text, file, line)`) in the global at `01268360`, and
/// the source file name of `hkaAnimation.inl` the two "not implemented" messages carry.
const REPORT_SINK: u32 = 0x0126_8360;
const ANIMATION_INL_FILE: u32 = 0x010d_db80;
/// "samplePartialPose not implemented for this type of animation" and "getDataChunks not
/// implemented for this type of animation".
const TEXT_SAMPLE_PARTIAL_POSE: u32 = 0x010d_dbd8;
const TEXT_GET_DATA_CHUNKS: u32 = 0x010d_dc18;
/// The vtable `~hkaAnimation` leaves in the object (the base class, shared with the other
/// destructors of the exe).
const BASE_VTABLE: u32 = 0x0102_e388;
/// Bits 0..29 of an array's capacity word, and bit 31: the buffer is not owned.
const CAPACITY_MASK: u32 = 0x3fff_ffff;
const NOT_OWNED: u32 = 0x8000_0000;

/// The allocator at `+0x10` of the memory router.
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

// Translated from 00dfb5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaAnnotationTrack::~hkaAnnotationTrack` (Xbox PDB), `this` = the track: the name string at
/// `+0`, the annotations (an `hkArray`: data `+4`, size `+8`, capacity word `+0xC`) of 8 bytes
/// each, a time and a string at `+4`. Destroys each annotation's string (`00cb0c00`), empties the
/// array and gives its buffer back through the router's `+0x10` allocator when it owns it
/// (`capacity * 8` bytes), then ends with a jump to the string destructor (`00cb0c00`) on the
/// name; its `EAX` is passed through.
pub fn hka_annotation_track_destructor(e: &mut Engine, this: Ptr) -> u32 {
    let this = this.addr();
    let allocator = router_allocator(e);
    let count = e.mem.i32(this + 8);
    let data = e.mem.u32(this + 4);
    if count > 0 {
        let mut string = data.wrapping_add(4);
        for _ in 0..count {
            e.call(STRING_PTR_DESTRUCT, &args![string]);
            string = string.wrapping_add(8);
        }
    }
    let capacity = e.mem.u32(this + 0xc);
    e.mem.set_u32(this + 8, 0);
    if capacity as i32 >= 0 {
        let bytes = (capacity & CAPACITY_MASK).wrapping_mul(8);
        let data = e.mem.u32(this + 4);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(this + 4, 0);
    e.mem.set_u32(this + 0xc, NOT_OWNED);
    e.call(STRING_PTR_DESTRUCT, &args![this]).u32()
}

/// Reports `text` through the sink as the exe does for a "not implemented" message: a 512-byte
/// text builder (12 bytes then the buffer), the text appended, the sink's slot `0xc` called with
/// `(3, id, buffer, hkaAnimation.inl, line)`. A non-zero answer is an `INT3` at `break_address` in
/// the exe (a panic here); otherwise the builder is destroyed.
fn report_not_implemented(e: &mut Engine, text: u32, id: u32, line: u32, break_address: &str) {
    e.with_stack(0x20c, |e, builder| {
        let builder = builder.addr();
        let buffer = builder + 0xc;
        e.call(TEXT_BUILDER_CONSTRUCT, &args![builder, buffer, 0x200u32]);
        e.call(TEXT_BUILDER_APPEND, &args![builder, text]);
        let sink = e.mem.u32(REPORT_SINK);
        let answer = e
            .vcall(
                sink,
                0xc,
                &args![3u32, id, buffer, ANIMATION_INL_FILE, line],
            )
            .u32();
        if answer != 0 {
            panic!("INT3 at {break_address}: the report sink asked for a break");
        }
        e.call(TEXT_BUILDER_DESTRUCT, &args![builder]);
    });
}

// Translated from 00dfb610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaAnimation::samplePartialTracks` (Xbox PDB), the base class' version: `this` and the six
/// stack words (`RET 0x18`) are not read. Reports "samplePartialPose not implemented for this
/// type of animation" (level 3, id `0x54e32123`, line `0x19` of `hkaAnimation.inl`); a non-zero
/// answer of the sink is an `INT3` in the exe (at `00dfb65b`) and panics here.
pub fn hka_animation_sample_partial_tracks(
    e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) {
    report_not_implemented(e, TEXT_SAMPLE_PARTIAL_POSE, 0x54e3_2123, 0x19, "00dfb65b");
}

// Translated from 00dfb670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaAnimation::getDataChunks` (Xbox PDB), the base class' version: `this` and the four stack
/// words (`RET 0x10`) are not read. Reports "getDataChunks not implemented for this type of
/// animation" (level 3, id `0x54e32124`, line `0x25` of `hkaAnimation.inl`); a non-zero answer of
/// the sink is an `INT3` in the exe (at `00dfb6bb`) and panics here.
pub fn hka_animation_get_data_chunks(
    e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
) {
    report_not_implemented(e, TEXT_GET_DATA_CHUNKS, 0x54e3_2124, 0x25, "00dfb6bb");
}

// Translated from 00dfb6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaAnimation::getExtractedMotionReferenceFrame` (Xbox PDB), stack `(time, out)`, `RET 8`:
/// forwards to virtual slot `+0xc` of the object at `+0x18` of the animation, with that object as
/// `this` and the arguments `(time, out)`; the time goes through the x87 stack, so a signalling
/// NaN is quieted. The callee's `EAX` is passed through (the decompiler shows `void`).
pub fn hka_animation_get_extracted_motion_reference_frame(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    out: u32,
) -> u32 {
    let frame = e.mem.u32(this.addr() + 0x18);
    e.vcall(frame, 0xc, &args![x87_quiet(time), out]).u32()
}

// Translated from 00dfb6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, stack `(time, delta, third, fourth)`, `RET 0x10`: like the one above
/// but forwards to virtual slot `+0x10` of the object at `+0x18` with `(time, delta, third,
/// fourth)`; the first two words go through the x87 stack (a signalling NaN is quieted), the other
/// two as they are. The callee's `EAX` is passed through.
pub fn fn_00dfb6f0(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    delta: f32,
    third: u32,
    fourth: u32,
) -> u32 {
    let frame = e.mem.u32(this.addr() + 0x18);
    e.vcall(
        frame,
        0x10,
        &args![x87_quiet(time), x87_quiet(delta), third, fourth],
    )
    .u32()
}

// Translated from 00dfb720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaAnimation::~hkaAnimation` (Xbox PDB), `this` = the animation: destroys each annotation
/// track of the array at `+0x1C` (size `+0x20`, capacity word `+0x24`; 0x10 bytes each) with
/// `~hkaAnnotationTrack`, empties the array and gives its buffer back through the router's
/// `+0x10` allocator when it owns it (`capacity * 16` bytes). Then, when the reference at `+0x18`
/// is not null, lets go of it (`removeReference`), clears it, and leaves the base vtable
/// `0102e388` in the object.
pub fn hka_animation_destructor(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    let allocator = router_allocator(e);
    let count = e.mem.i32(this + 0x20);
    let mut track = e.mem.u32(this + 0x1c);
    if count > 0 {
        for _ in 0..count {
            hka_annotation_track_destructor(e, Ptr::new(track));
            track = track.wrapping_add(0x10);
        }
    }
    let capacity = e.mem.u32(this + 0x24);
    e.mem.set_u32(this + 0x20, 0);
    if capacity as i32 >= 0 {
        let bytes = (capacity & CAPACITY_MASK) << 4;
        let data = e.mem.u32(this + 0x1c);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(this + 0x1c, 0);
    e.mem.set_u32(this + 0x24, NOT_OWNED);
    let reference = e.mem.u32(this + 0x18);
    if reference != 0 {
        e.call(REMOVE_REFERENCE, &args![reference]);
    }
    e.mem.set_u32(this + 0x18, 0);
    e.mem.set_u32(this, BASE_VTABLE);
}

// Translated from 00dfb7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaWaveletCompressedAnimation::_scalar_deleting_destructor_` (Xbox PDB), stack `(flags)`,
/// `RET 4`: empties the array at `+0x64` (size `+0x68`, capacity word `+0x6C`, bytes) giving its
/// buffer back through the router's `+0x10` allocator when it owns it (`capacity & mask`
/// bytes), then runs `~hkaAnimation` on the object and, when bit 0 of `flags` is set, frees the
/// object through the router's `+0x10` allocator (slot 8, with the 16-bit size at `+4`).
/// Returns `this`.
pub fn hka_wavelet_compressed_animation_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    let object = this.addr();
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(object + 0x6c);
    e.mem.set_u32(object + 0x68, 0);
    if capacity as i32 >= 0 {
        let data = e.mem.u32(object + 0x64);
        e.vcall(allocator, 0x10, &args![data, capacity & CAPACITY_MASK]);
    }
    e.mem.set_u32(object + 0x64, 0);
    e.mem.set_u32(object + 0x6c, NOT_OWNED);
    hka_animation_destructor(e, this);
    if flags & 1 != 0 {
        let allocator = router_allocator(e);
        let size = u32::from(e.mem.u16(object + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_b5a0, hka_annotation_track_destructor(Ptr) -> u32),
        entry!(
            0x00df_b610,
            hka_animation_sample_partial_tracks(Ptr, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00df_b670,
            hka_animation_get_data_chunks(Ptr, u32, u32, u32, u32)
        ),
        entry!(
            0x00df_b6d0,
            hka_animation_get_extracted_motion_reference_frame(Ptr, f32, u32) -> u32
        ),
        entry!(0x00df_b6f0, fn_00dfb6f0(Ptr, f32, f32, u32, u32) -> u32),
        entry!(0x00df_b720, hka_animation_destructor(Ptr)),
        entry!(0x00df_b7b0, deleting_destructor_00dfb7b0(Ptr, u8) -> Ptr),
        entry!(
            0x00df_b7e0,
            hka_wavelet_compressed_animation_scalar_deleting_destructor(Ptr, u8) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00dfb7b0_frees_only_when_bit_zero_is_set() {
        check(0x00df_b7b0, 0x00df_b720);
    }

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Fake vtable slots of the allocator: free (`+0x10`), free of an object (`+8`) and the
    /// report sink's `+0xc`, the reference frame's `+0xc` and `+0x10`.
    const ALLOCATOR_FREE: u32 = 0x0900_0110;
    const ALLOCATOR_FREE_OBJECT: u32 = 0x0900_0108;
    const SINK_REPORT: u32 = 0x0900_020c;
    const FRAME_SLOT_C: u32 = 0x0900_030c;
    const FRAME_SLOT_10: u32 = 0x0900_0310;

    fn record(e: &mut Engine, log: &Log, address: u32, answer: u32) {
        let log = Rc::clone(log);
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
    }

    fn calls_of(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// A memory router whose `+0x10` allocator frees through `ALLOCATOR_FREE` (slot `0x10`) and
    /// `ALLOCATOR_FREE_OBJECT` (slot 8); returns the allocator.
    fn install_allocator(e: &mut Engine, log: &Log) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, ALLOCATOR_FREE_OBJECT);
        e.mem.set_u32(vtable + 0x10, ALLOCATOR_FREE);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        record(e, log, MEMORY_ROUTER, router);
        record(e, log, ALLOCATOR_FREE, 0);
        record(e, log, ALLOCATOR_FREE_OBJECT, 0);
        allocator
    }

    /// A report sink object in the global at `01268360`, answering `answer` to its slot `0xc`.
    fn install_sink(e: &mut Engine, log: &Log, answer: u32) {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, SINK_REPORT);
        let sink = e.mem.alloc(8);
        e.mem.set_u32(sink, vtable);
        e.mem.set_u32(REPORT_SINK, sink);
        record(e, log, SINK_REPORT, answer);
        record(e, log, TEXT_BUILDER_CONSTRUCT, 0);
        record(e, log, TEXT_BUILDER_APPEND, 0);
        record(e, log, TEXT_BUILDER_DESTRUCT, 0);
    }

    /// An annotation track: `count` annotations in a buffer of `capacity_word`.
    fn track(e: &mut Engine, count: u32, capacity_word: u32) -> (u32, u32) {
        let this = e.mem.alloc(0x10);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(this + 4, data);
        e.mem.set_u32(this + 8, count);
        e.mem.set_u32(this + 0xc, capacity_word);
        (this, data)
    }

    #[test]
    fn an_annotation_track_destroys_its_strings_frees_its_buffer_and_ends_with_the_name() {
        let (mut e, log) = (engine(), Rc::new(RefCell::new(Vec::new())));
        let allocator = install_allocator(&mut e, &log);
        record(&mut e, &log, STRING_PTR_DESTRUCT, 0x77);
        let (this, data) = track(&mut e, 2, 5);
        let got = e.call(0x00df_b5a0, &args![this]).u32();
        assert_eq!(got, 0x77);
        assert_eq!(
            calls_of(&log, STRING_PTR_DESTRUCT),
            vec![vec![data + 4], vec![data + 0xc], vec![this]]
        );
        assert_eq!(
            calls_of(&log, ALLOCATOR_FREE),
            vec![vec![allocator, data, 40]]
        );
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0x8000_0000);
    }

    #[test]
    fn an_annotation_track_with_a_borrowed_buffer_frees_nothing() {
        let (mut e, log) = (engine(), Rc::new(RefCell::new(Vec::new())));
        install_allocator(&mut e, &log);
        record(&mut e, &log, STRING_PTR_DESTRUCT, 0);
        let (this, _) = track(&mut e, 0, 0x8000_0004);
        e.call(0x00df_b5a0, &args![this]);
        assert_eq!(calls_of(&log, STRING_PTR_DESTRUCT), vec![vec![this]]);
        assert!(calls_of(&log, ALLOCATOR_FREE).is_empty());
        assert_eq!(e.mem.u32(this + 0xc), 0x8000_0000);
    }

    /// The text builder is used around the report of a stub; returns the log and the builder
    /// address the construction saw.
    fn run_stub(address: u32, args: &[u32], answer: u32) -> (Log, u32) {
        let (mut e, log) = (engine(), Rc::new(RefCell::new(Vec::new())));
        install_sink(&mut e, &log, answer);
        e.call(address, args);
        let builder = calls_of(&log, TEXT_BUILDER_CONSTRUCT)[0][0];
        (log, builder)
    }

    #[test]
    fn sample_partial_tracks_reports_that_it_is_not_implemented() {
        let (log, builder) = run_stub(0x00df_b610, &[1, 2, 3, 4, 5, 6, 7], 0);
        assert_eq!(
            calls_of(&log, TEXT_BUILDER_CONSTRUCT),
            vec![vec![builder, builder + 0xc, 0x200]]
        );
        assert_eq!(
            calls_of(&log, TEXT_BUILDER_APPEND),
            vec![vec![builder, TEXT_SAMPLE_PARTIAL_POSE]]
        );
        let reports = calls_of(&log, SINK_REPORT);
        assert_eq!(reports.len(), 1);
        assert_eq!(
            reports[0][1..],
            [3, 0x54e3_2123, builder + 0xc, ANIMATION_INL_FILE, 0x19]
        );
        assert_eq!(calls_of(&log, TEXT_BUILDER_DESTRUCT), vec![vec![builder]]);
    }

    #[test]
    fn get_data_chunks_reports_that_it_is_not_implemented() {
        let (log, builder) = run_stub(0x00df_b670, &[1, 2, 3, 4, 5], 0);
        assert_eq!(
            calls_of(&log, TEXT_BUILDER_APPEND),
            vec![vec![builder, TEXT_GET_DATA_CHUNKS]]
        );
        let reports = calls_of(&log, SINK_REPORT);
        assert_eq!(
            reports[0][1..],
            [3, 0x54e3_2124, builder + 0xc, ANIMATION_INL_FILE, 0x25]
        );
        assert_eq!(calls_of(&log, TEXT_BUILDER_DESTRUCT), vec![vec![builder]]);
    }

    #[test]
    #[should_panic(expected = "INT3 at 00dfb65b")]
    fn sample_partial_tracks_stops_where_the_exe_executes_int3() {
        run_stub(0x00df_b610, &[1, 2, 3, 4, 5, 6, 7], 1);
    }

    #[test]
    #[should_panic(expected = "INT3 at 00dfb6bb")]
    fn get_data_chunks_stops_where_the_exe_executes_int3() {
        run_stub(0x00df_b670, &[1, 2, 3, 4, 5], 1);
    }

    /// An animation whose reference frame (at `+0x18`) answers `answer` to slots `+0xc` and `+0x10`.
    fn animation_with_frame(e: &mut Engine, log: &Log, answer: u32) -> (u32, u32) {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, FRAME_SLOT_C);
        e.mem.set_u32(vtable + 0x10, FRAME_SLOT_10);
        let frame = e.mem.alloc(8);
        e.mem.set_u32(frame, vtable);
        let animation = e.mem.alloc(0x80);
        e.mem.set_u32(animation + 0x18, frame);
        record(e, log, FRAME_SLOT_C, answer);
        record(e, log, FRAME_SLOT_10, answer);
        (animation, frame)
    }

    #[test]
    fn the_extracted_motion_reference_frame_is_forwarded_with_a_quieted_time() {
        let (mut e, log) = (engine(), Rc::new(RefCell::new(Vec::new())));
        let (animation, frame) = animation_with_frame(&mut e, &log, 0x1357);
        let got = e
            .call(
                0x00df_b6d0,
                &args![animation, f32::from_bits(0x7f80_0001), 0x2000u32],
            )
            .u32();
        assert_eq!(got, 0x1357);
        assert_eq!(
            calls_of(&log, FRAME_SLOT_C),
            vec![vec![frame, 0x7fc0_0001, 0x2000]]
        );
        let got = e
            .call(0x00df_b6d0, &args![animation, 1.5f32, 0x2004u32])
            .u32();
        assert_eq!(got, 0x1357);
        assert_eq!(
            calls_of(&log, FRAME_SLOT_C)[1],
            vec![frame, 1.5f32.to_bits(), 0x2004]
        );
    }

    #[test]
    fn the_delta_reference_frame_is_forwarded_with_two_quieted_floats() {
        let (mut e, log) = (engine(), Rc::new(RefCell::new(Vec::new())));
        let (animation, frame) = animation_with_frame(&mut e, &log, 0x2468);
        let got = e
            .call(
                0x00df_b6f0,
                &args![
                    animation,
                    f32::from_bits(0xff80_0001),
                    0.25f32,
                    0x11u32,
                    0x22u32
                ],
            )
            .u32();
        assert_eq!(got, 0x2468);
        assert_eq!(
            calls_of(&log, FRAME_SLOT_10),
            vec![vec![frame, 0xffc0_0001, 0.25f32.to_bits(), 0x11, 0x22]]
        );
        e.call(
            0x00df_b6f0,
            &args![
                animation,
                2.0f32,
                f32::from_bits(0x7f80_0002),
                0x33u32,
                0x44u32
            ],
        );
        assert_eq!(
            calls_of(&log, FRAME_SLOT_10)[1],
            vec![frame, 2.0f32.to_bits(), 0x7fc0_0002, 0x33, 0x44]
        );
    }

    /// An animation with `tracks` annotation tracks in a buffer of `capacity_word`, and the
    /// reference at `+0x18` set to `reference`.
    fn animation(e: &mut Engine, tracks: u32, capacity_word: u32, reference: u32) -> (u32, u32) {
        let this = e.mem.alloc(0x80);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x1c, data);
        e.mem.set_u32(this + 0x20, tracks);
        e.mem.set_u32(this + 0x24, capacity_word);
        e.mem.set_u32(this + 0x18, reference);
        (this, data)
    }

    #[test]
    fn an_animation_destroys_its_tracks_frees_the_array_and_lets_go_of_the_reference() {
        let (mut e, log) = (engine(), Rc::new(RefCell::new(Vec::new())));
        let allocator = install_allocator(&mut e, &log);
        record(&mut e, &log, STRING_PTR_DESTRUCT, 0);
        record(&mut e, &log, REMOVE_REFERENCE, 0);
        let (this, data) = animation(&mut e, 2, 3, 0x5555);
        // The tracks hold nothing and own no buffer.
        e.mem.set_u32(data + 0xc, 0x8000_0000);
        e.mem.set_u32(data + 0x1c, 0x8000_0000);
        e.call(0x00df_b720, &args![this]);
        // Each track destroys its name (at the track's `+0`): the two tracks at +0 and +0x10.
        assert_eq!(
            calls_of(&log, STRING_PTR_DESTRUCT),
            vec![vec![data], vec![data + 0x10]]
        );
        assert_eq!(
            calls_of(&log, ALLOCATOR_FREE),
            vec![vec![allocator, data, 3 * 16]]
        );
        assert_eq!(calls_of(&log, REMOVE_REFERENCE), vec![vec![0x5555]]);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(e.mem.u32(this + 0x24), 0x8000_0000);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }

    #[test]
    fn an_animation_without_a_reference_or_an_owned_array_only_resets_itself() {
        let (mut e, log) = (engine(), Rc::new(RefCell::new(Vec::new())));
        install_allocator(&mut e, &log);
        record(&mut e, &log, REMOVE_REFERENCE, 0);
        let (this, _) = animation(&mut e, 0, 0x8000_0003, 0);
        e.call(0x00df_b720, &args![this]);
        assert!(calls_of(&log, REMOVE_REFERENCE).is_empty());
        assert!(calls_of(&log, ALLOCATOR_FREE).is_empty());
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x24), 0x8000_0000);
    }

    #[test]
    fn the_wavelet_scalar_deleting_destructor_frees_its_array_then_the_object_by_flag() {
        let (mut e, log) = (engine(), Rc::new(RefCell::new(Vec::new())));
        let allocator = install_allocator(&mut e, &log);
        record(&mut e, &log, STRING_PTR_DESTRUCT, 0);
        record(&mut e, &log, REMOVE_REFERENCE, 0);
        let (this, _) = animation(&mut e, 0, 0x8000_0000, 0);
        let buffer = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x64, buffer);
        e.mem.set_u32(this + 0x68, 7);
        e.mem.set_u32(this + 0x6c, 0x4000_0010);
        e.mem.set_u16(this + 4, 0x70);
        let got = e.call(0x00df_b7e0, &args![this, 0xfeu8]).u32();
        assert_eq!(got, this);
        // The array's buffer goes back with `capacity & mask` bytes; the object is not freed.
        assert_eq!(
            calls_of(&log, ALLOCATOR_FREE),
            vec![vec![allocator, buffer, 0x10]]
        );
        assert!(calls_of(&log, ALLOCATOR_FREE_OBJECT).is_empty());
        assert_eq!(e.mem.u32(this + 0x64), 0);
        assert_eq!(e.mem.u32(this + 0x68), 0);
        assert_eq!(e.mem.u32(this + 0x6c), 0x8000_0000);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        // With bit 0 set the object goes back too, with its 16-bit size.
        let got = e.call(0x00df_b7e0, &args![this, 1u8]).u32();
        assert_eq!(got, this);
        assert_eq!(
            calls_of(&log, ALLOCATOR_FREE_OBJECT),
            vec![vec![allocator, this, 0x70]]
        );
    }

    #[test]
    fn the_wavelet_scalar_deleting_destructor_does_not_free_a_borrowed_array() {
        let (mut e, log) = (engine(), Rc::new(RefCell::new(Vec::new())));
        install_allocator(&mut e, &log);
        record(&mut e, &log, REMOVE_REFERENCE, 0);
        let (this, _) = animation(&mut e, 0, 0x8000_0000, 0);
        e.mem.set_u32(this + 0x6c, 0x8000_0010);
        e.call(0x00df_b7e0, &args![this, 0u8]);
        assert!(calls_of(&log, ALLOCATOR_FREE).is_empty());
        assert!(calls_of(&log, ALLOCATOR_FREE_OBJECT).is_empty());
    }
}
