//! `NiMain/run_00a9fe90` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The Ni allocation `00aa13e0` (`cdecl`, size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The base constructor `00a8f190` (engine map unit `NiMain/ni2dbuffer.cpp`, `thiscall`, no
/// words).
const BASE_CONSTRUCT_00A8F190: u32 = 0x00a8_f190;
/// The vtable whose RTTI locator names `NiDepthStencilBuffer` (`010a2084`).
const NI_DEPTH_STENCIL_BUFFER_VTABLE: u32 = 0x010a_2084;
/// The value the format object's word at +4 must hold.
const FORMAT_KIND_15: u32 = 0xf;

// Translated from 00a9fe90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00a9fe90`): makes an `NiDepthStencilBuffer` (the vtable's
/// RTTI name). `cdecl`, five words: width, height, an unread word, a format object and a word
/// passed on. Answers 0 unless the format object's word at +4 is 0xf. Otherwise allocates
/// 0x14 bytes (`00aa13e0`), runs the base constructor `00a8f190` and sets the vtable
/// `010a2084` (a null allocation is used as it is), stores the width at +8 and the height at
/// +0xc, and calls the buffer's vtable slot +0x94 with (format object, last word). When that
/// answers false (`AL`) the buffer is deleted (slot 0 with 1) and 0 is answered; else the
/// buffer.
pub fn fn_00a9fe90(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_2: u32,
    format: Ptr,
    extra: u32,
) -> Ptr {
    if e.mem.u32(format.addr().wrapping_add(4)) != FORMAT_KIND_15 {
        return Ptr::NULL;
    }
    let block = e.call(NI_ALLOCATE, &args![0x14u32]).u32();
    let buffer = if block != 0 {
        e.call(BASE_CONSTRUCT_00A8F190, &args![block]);
        e.mem.set_u32(block, NI_DEPTH_STENCIL_BUFFER_VTABLE);
        block
    } else {
        0
    };
    let vtable = e.mem.u32(buffer);
    let slot = e.mem.u32(vtable.wrapping_add(0x94));
    e.mem.set_u32(buffer.wrapping_add(8), width);
    e.mem.set_u32(buffer.wrapping_add(0xc), height);
    if !e.call(slot, &args![buffer, format, extra]).bool() {
        e.vcall(buffer, 0, &args![1u32]);
        return Ptr::NULL;
    }
    Ptr::new(buffer)
}

/// The word at `offset` of the object at +0x10 of `buffer`, or 0 when there is none.
fn pixel_data_word(e: &Engine, buffer: u32, offset: u32) -> u32 {
    let data = e.mem.u32(buffer + 0x10);
    if data == 0 {
        0
    } else {
        e.mem.u32(data + offset)
    }
}

// Translated from 00a9ff00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00a9fe90`): a depth-stencil buffer for a buffer, `cdecl` with
/// two words: the buffer and the renderer. 0 when either is null. Otherwise asks the renderer
/// (its vtable slot +0x94) for a format object from the word at +0xc of the buffer's +0x10
/// object (0 when there is none), 0x20 and 8, and answers [`fn_00a9fe90`] with the buffer's
/// width (+8) and height (+0xc), the renderer, that format object and the word at +0x10 of the
/// buffer's +0x10 object.
pub fn fn_00a9ff00(e: &mut Engine, buffer: Ptr, renderer: Ptr) -> Ptr {
    let buffer = buffer.addr();
    let renderer = renderer.addr();
    if buffer == 0 || renderer == 0 {
        return Ptr::NULL;
    }
    let source = pixel_data_word(e, buffer, 0xc);
    let format = e.vcall(renderer, 0x94, &args![source, 0x20u32, 8u32]).u32();
    let data = pixel_data_word(e, buffer, 0x10);
    let height = e.mem.u32(buffer + 0xc);
    let width = e.mem.u32(buffer + 8);
    fn_00a9fe90(e, width, height, renderer, Ptr::new(format), data)
}

/// Import slots of `InterlockedIncrement` and `InterlockedDecrement` (`stdcall`, one word).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00a9ff60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, three words: a width, a height and a counted
/// object): a new `NiDepthStencilBuffer` (0x14 bytes from `00aa13e0`, the base constructor
/// `00a8f190`, the vtable; a failed allocation is used as null, as the exe does) with the
/// width at +8, the height at +0xc and the object assigned to +0x10 as a counted pointer
/// (the old one released through its virtual `+4` when its count at +4 reaches 0, the new
/// one counted). Answers the buffer.
pub fn fn_00a9ff60(e: &mut Engine, width: u32, height: u32, object: Ptr) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x14u32]).u32();
    let buffer = if block == 0 {
        0
    } else {
        e.call(BASE_CONSTRUCT_00A8F190, &args![block]);
        e.mem.set_u32(block, NI_DEPTH_STENCIL_BUFFER_VTABLE);
        block
    };
    let object = object.addr();
    e.mem.set_u32(buffer.wrapping_add(8), width);
    e.mem.set_u32(buffer.wrapping_add(0xc), height);
    let old = e.mem.u32(buffer.wrapping_add(0x10));
    if old != object {
        if old != 0
            && e.call(INTERLOCKED_DECREMENT, &args![old.wrapping_add(4)])
                .u32()
                == 0
        {
            e.vcall(old, 4, &args![]);
        }
        e.mem.set_u32(buffer.wrapping_add(0x10), object);
        if object != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
        }
    }
    Ptr::new(buffer)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a9_fe90,
            fn_00a9fe90(u32, u32, u32, Ptr, u32) -> Ptr
        ),
        entry!(0x00a9_ff00, fn_00a9ff00(Ptr, Ptr) -> Ptr),
        entry!(0x00a9_ff60, fn_00a9ff60(u32, u32, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The buffer's vtable at `010a2084` with slot 0 (`0f00_3000`) and slot +0x94
    /// (`0f00_3094`, answering `accepts`).
    fn rig(accepts: bool) -> Engine {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x26];
        slots[0] = 0x0f00_3000;
        slots[0x25] = 0x0f00_3094;
        e.put_vtable(NI_DEPTH_STENCIL_BUFFER_VTABLE, &slots);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BASE_CONSTRUCT_00A8F190, |e, a| {
            e.mem.set_u32(a[0], 0x1234);
            Ret::default()
        });
        e.register(0x0f00_3000, |_, _| Ret::default());
        if accepts {
            e.register(0x0f00_3094, |_, _| 1u32.into_ret());
        } else {
            // Only `AL` counts.
            e.register(0x0f00_3094, |_, _| 0x100u32.into_ret());
        }
        e
    }

    #[test]
    fn fn_00a9fe90_makes_the_buffer_when_its_set_up_succeeds() {
        let mut e = rig(true);
        let format = e.mem.alloc(8);
        e.mem.set_u32(format + 4, 0xf);
        e.call_log = Some(vec![]);
        let buffer = e
            .call(
                0x00a9_fe90,
                &args![640u32, 480u32, 0x77u32, format, 0x55u32],
            )
            .u32();
        assert_ne!(buffer, 0);
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x14]]);
        assert_eq!(calls(&e, BASE_CONSTRUCT_00A8F190), vec![vec![buffer]]);
        assert_eq!(e.mem.u32(buffer), NI_DEPTH_STENCIL_BUFFER_VTABLE);
        assert_eq!(e.mem.u32(buffer + 8), 640);
        assert_eq!(e.mem.u32(buffer + 0xc), 480);
        assert_eq!(calls(&e, 0x0f00_3094), vec![vec![buffer, format, 0x55]]);
        assert!(calls(&e, 0x0f00_3000).is_empty());
    }

    #[test]
    fn fn_00a9fe90_deletes_the_buffer_when_its_set_up_fails() {
        let mut e = rig(false);
        let format = e.mem.alloc(8);
        e.mem.set_u32(format + 4, 0xf);
        e.call_log = Some(vec![]);
        let answer = e
            .call(0x00a9_fe90, &args![1u32, 2u32, 0u32, format, 0u32])
            .u32();
        assert_eq!(answer, 0);
        let buffer = calls(&e, 0x0f00_3094)[0][0];
        assert_eq!(calls(&e, 0x0f00_3000), vec![vec![buffer, 1]]);
    }

    #[test]
    fn fn_00a9fe90_refuses_another_format_kind() {
        let mut e = rig(true);
        let format = e.mem.alloc(8);
        e.mem.set_u32(format + 4, 0xe);
        e.call_log = Some(vec![]);
        let answer = e
            .call(0x00a9_fe90, &args![1u32, 2u32, 0u32, format, 0u32])
            .u32();
        assert_eq!(answer, 0);
        assert!(calls(&e, NI_ALLOCATE).is_empty());
    }
}

#[cfg(test)]
mod buffer_target_tests {
    use super::*;

    const RENDERER_VTABLE: u32 = 0x0e00_0000;
    const FORMAT_OF: u32 = 0x0e01_0094;
    const BUFFER_SET_UP: u32 = 0x0e02_0094;

    /// The renderer's format double answers a format object whose +4 is 0xf for a source word
    /// of 3, else one whose +4 is 0.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x26];
        slots[0x25] = FORMAT_OF;
        e.put_vtable(RENDERER_VTABLE, &slots);
        let good = e.mem.alloc(8);
        e.mem.set_u32(good + 4, 0xf);
        let bad = e.mem.alloc(8);
        e.register_double(FORMAT_OF, move |_, a| {
            if a[1] == 3 { good } else { bad }.into_ret()
        });
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BASE_CONSTRUCT_00A8F190, |_, a| a[0].into_ret());
        let mut buffer_slots = vec![0u32; 0x26];
        buffer_slots[0x25] = BUFFER_SET_UP;
        e.put_vtable(NI_DEPTH_STENCIL_BUFFER_VTABLE, &buffer_slots);
        e.register(BUFFER_SET_UP, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        let renderer = e.mem.alloc(4);
        e.mem.set_u32(renderer, RENDERER_VTABLE);
        (e, renderer)
    }

    #[test]
    fn fn_00a9ff00_makes_a_buffer_of_the_source_size() {
        let (mut e, renderer) = engine();
        let source = e.mem.alloc(0x14);
        let data = e.mem.alloc(0x14);
        e.mem.set_u32(data + 0xc, 3);
        e.mem.set_u32(data + 0x10, 4);
        e.mem.set_u32(source + 8, 64);
        e.mem.set_u32(source + 0xc, 32);
        e.mem.set_u32(source + 0x10, data);
        let made = e.call(0x00a9_ff00, &args![source, renderer]).u32();
        assert_ne!(made, 0);
        assert_eq!(e.mem.u32(made + 8), 64);
        assert_eq!(e.mem.u32(made + 0xc), 32);
        let log = e.call_log.take().unwrap();
        let formats: Vec<&Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == FORMAT_OF)
            .map(|c| &c.1)
            .collect();
        assert_eq!(formats, vec![&vec![renderer, 3, 0x20, 8]]);
        let set_up: Vec<&Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == BUFFER_SET_UP)
            .map(|c| &c.1)
            .collect();
        assert_eq!(set_up[0][2], 4);
    }

    #[test]
    fn fn_00a9ff00_without_pixel_data_or_pointers() {
        let (mut e, renderer) = engine();
        let source = e.mem.alloc(0x14);
        // No pixel data: source word 0, the format refused, nothing made.
        assert_eq!(e.call(0x00a9_ff00, &args![source, renderer]).u32(), 0);
        assert_eq!(e.call(0x00a9_ff00, &args![0u32, renderer]).u32(), 0);
        assert_eq!(e.call(0x00a9_ff00, &args![source, 0u32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|c| c.0 == FORMAT_OF).count(), 1);
        assert_eq!(log.iter().filter(|c| c.0 == NI_ALLOCATE).count(), 0);
    }
}

#[cfg(test)]
mod make_tests {
    use super::*;

    #[test]
    fn makes_a_buffer_with_a_counted_object() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BASE_CONSTRUCT_00A8F190, |_, _| Ret::default());
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        let object = e.mem.alloc(8);
        e.mem.set_u32(object + 4, 2);
        e.call_log = Some(vec![]);
        let buffer = e.call(0x00a9_ff60, &args![640u32, 480u32, object]).u32();
        assert_eq!(e.mem.u32(buffer), NI_DEPTH_STENCIL_BUFFER_VTABLE);
        assert_eq!((e.mem.u32(buffer + 8), e.mem.u32(buffer + 0xc)), (640, 480));
        assert_eq!(e.mem.u32(buffer + 0x10), object);
        assert_eq!(e.mem.u32(object + 4), 3);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(BASE_CONSTRUCT_00A8F190, vec![buffer])));
        // A null object leaves the new buffer without one and counts nothing.
        let buffer = e.call(0x00a9_ff60, &args![1u32, 2u32, 0u32]).u32();
        assert_eq!(e.mem.u32(buffer + 0x10), 0);
        assert_eq!(e.mem.u32(object + 4), 3);
    }
}
