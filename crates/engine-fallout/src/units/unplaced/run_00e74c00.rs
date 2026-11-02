//! `(unplaced)/run_00e74c00` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The renderer's Direct3D device (+0x288), its `TestCooperativeLevel`
/// (COM slot +0xC) and the slot +0x200 the check calls with 0 when the flag
/// at `0126f0d0` is set.
const DEVICE: u32 = 0x288;
const TEST_COOPERATIVE_LEVEL_SLOT: u32 = 0xc;
const DEVICE_SLOT_200: u32 = 0x200;
const DEVICE_SLOT_200_FLAG: u32 = 0x0126_f0d0;
/// `D3DERR_DEVICENOTRESET` and `D3DERR_DEVICELOST`.
const DEVICE_NOT_RESET: u32 = 0x8876_0869;
const DEVICE_LOST: u32 = 0x8876_0868;
/// The reset the renderer runs when the device can be reset (`00e736b0`,
/// `thiscall`, a byte; reached by a tail jump).
const RESET_DEVICE: u32 = 0x00e7_36b0;
/// The device-lost state (+0x6FC), the lost-device callbacks (+0xAF8, a
/// `cdecl (data)` function per entry answering a byte), their data (+0xB08)
/// and their count (+0xAFE, 16-bit).
const DEVICE_LOST_STATE: u32 = 0x6fc;
const LOST_CALLBACKS: u32 = 0xaf8;
const LOST_CALLBACK_DATA: u32 = 0xb08;
const LOST_CALLBACK_COUNT: u32 = 0xafe;
/// The message `00a29680` (`cdecl`, one word) is given when a callback
/// fails (the string at `010ee468`).
const REPORT: u32 = 0x00a2_9680;
const CALLBACK_FAILED_MESSAGE: u32 = 0x010e_e468;

// Translated from 00e74c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the renderer's device check): asks the device
/// `TestCooperativeLevel` (and calls its slot +0x200 with 0 when the flag at
/// `0126f0d0` is set). A device that can be reset is reset (`00e736b0`,
/// whose answer is the answer). A lost device, the first time (+0x6FC),
/// runs the lost-device callbacks in order (a failing one stops them with
/// the message `00a29680`); false either way. Any other result: true.
pub fn fn_00e74c00(e: &mut Engine, this: Ptr) -> bool {
    let device = e.mem.u32(this.addr().wrapping_add(DEVICE));
    let level = e.vcall(device, TEST_COOPERATIVE_LEVEL_SLOT, &args![]).u32();
    if e.mem.u8(DEVICE_SLOT_200_FLAG) != 0 {
        let device = e.mem.u32(this.addr().wrapping_add(DEVICE));
        e.vcall(device, DEVICE_SLOT_200, &args![0u32]);
    }
    if level == DEVICE_NOT_RESET {
        return e.call(RESET_DEVICE, &args![this]).bool();
    }
    if level != DEVICE_LOST {
        return true;
    }
    if e.mem.u8(this.addr().wrapping_add(DEVICE_LOST_STATE)) != 0 {
        return false;
    }
    let count = u32::from(e.mem.u16(this.addr().wrapping_add(LOST_CALLBACK_COUNT)));
    e.mem.set_u8(this.addr().wrapping_add(DEVICE_LOST_STATE), 1);
    for i in 0..count {
        let callbacks = e.mem.u32(this.addr().wrapping_add(LOST_CALLBACKS));
        let callback = e.mem.u32(callbacks.wrapping_add(4 * i));
        let data_array = e.mem.u32(this.addr().wrapping_add(LOST_CALLBACK_DATA));
        let data = e.mem.u32(data_array.wrapping_add(4 * i));
        if callback != 0 && !e.call(callback, &args![data]).bool() {
            e.call(REPORT, &args![CALLBACK_FAILED_MESSAGE]);
            return false;
        }
    }
    false
}

/// The geometry buffer data's two calls (`00e8eeb0`: answers a byte; `00e8ef10`: one word;
/// unit `nigeometrybufferdata.cpp`, thiscall) and the renderer's buffer update `00e723a0`
/// (unit `nixenonrenderer.cpp`; thiscall: the geometry, a word of flags).
const BUFFER_DATA_CHECK: u32 = 0x00e8_eeb0;
const BUFFER_DATA_PREPARE: u32 = 0x00e8_ef10;
const RENDERER_UPDATE_BUFFERS: u32 = 0x00e7_23a0;

// Translated from 00e74cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: prepares a geometry's buffers for drawing. With a geometry,
/// its buffer data (+0x1c) is checked (`00e8eeb0`), gets `0x01400000` at +4 and
/// `00e8ef10(1)`; then, with the 16-bit dirty flags at +0x18 of the geometry: a failed
/// check calls the buffer data's object (+8) slot +0x1c with the buffer data and 0, clears
/// +0x38 and updates with flags 0xf; a passed check asks that object's slot +0x20 and
/// updates with 0xf when it answers true, else with the dirty flags when there are any
/// (`00e723a0`). The dirty flags are then cleared.
pub fn fn_00e74cc0(e: &mut Engine, this: Ptr, geometry: Ptr) {
    let geometry = geometry.addr();
    if geometry == 0 {
        return;
    }
    let buffers = e.mem.u32(geometry + 0x1c);
    let checked = e.call(BUFFER_DATA_CHECK, &args![buffers]).u8();
    e.mem.set_u32(buffers + 4, 0x0140_0000);
    e.call(BUFFER_DATA_PREPARE, &args![buffers, 1u32]);
    let object = e.mem.u32(buffers + 8);
    let dirty = u32::from(e.mem.u16(geometry + 0x18));
    let flags = if checked == 0 {
        e.vcall(object, 0x1c, &args![buffers, 0u32]);
        e.mem.set_u32(buffers + 0x38, 0);
        Some(0xf)
    } else if e.vcall(object, 0x20, &args![]).u8() != 0 {
        Some(0xf)
    } else if dirty != 0 {
        Some(dirty)
    } else {
        None
    };
    if let Some(flags) = flags {
        e.call(RENDERER_UPDATE_BUFFERS, &args![this, geometry, flags]);
    }
    e.mem.set_u16(geometry + 0x18, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e74c00, fn_00e74c00(Ptr) -> bool),
        entry!(0x00e7_4cc0, fn_00e74cc0(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    #[test]
    fn prepares_buffers_and_updates_with_the_right_flags() {
        for (checked, answer, dirty, want) in [
            (0u32, 0u32, 3u16, Some(0xfu32)),
            (1, 1, 3, Some(0xf)),
            (1, 0, 3, Some(3)),
            (1, 0, 0, None),
        ] {
            let mut e = Engine::new();
            e.register_double(BUFFER_DATA_CHECK, move |_, _| (0x100 | checked).into_ret());
            e.register(BUFFER_DATA_PREPARE, |_, _| Ret::default());
            e.register(RENDERER_UPDATE_BUFFERS, |_, _| Ret::default());
            e.register(0x0079_001c, |_, _| Ret::default());
            e.register_double(0x0079_0020, move |_, _| (0x200 | answer).into_ret());
            let vtable = e.mem.alloc(0x24);
            e.mem.set_u32(vtable + 0x1c, 0x0079_001c);
            e.mem.set_u32(vtable + 0x20, 0x0079_0020);
            let object = e.mem.alloc(4);
            e.mem.set_u32(object, vtable);
            let buffers = e.mem.alloc(0x40);
            e.mem.set_u32(buffers + 8, object);
            e.mem.set_u32(buffers + 0x38, 9);
            let geometry = e.mem.alloc(0x20);
            e.mem.set_u32(geometry + 0x1c, buffers);
            e.mem.set_u16(geometry + 0x18, dirty);
            e.call_log = Some(vec![]);
            e.call(0x00e7_4cc0, &args![0x5000u32, geometry]);
            let log = e.call_log.take().unwrap();
            let updates: Vec<_> = log
                .iter()
                .filter(|c| c.0 == RENDERER_UPDATE_BUFFERS)
                .map(|c| c.1.clone())
                .collect();
            match want {
                Some(flags) => assert_eq!(updates, vec![vec![0x5000, geometry, flags]]),
                None => assert!(updates.is_empty()),
            }
            assert!(log.contains(&(BUFFER_DATA_PREPARE, vec![buffers, 1])));
            assert_eq!(e.mem.u32(buffers + 4), 0x0140_0000);
            assert_eq!(e.mem.u32(buffers + 0x38), if checked == 0 { 0 } else { 9 });
            assert_eq!(e.mem.u16(geometry + 0x18), 0);
        }
        // No geometry: nothing.
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x00e7_4cc0, &args![0x5000u32, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const COOPERATIVE: u32 = 0x7200_000c;
    const SLOT_200: u32 = 0x7200_0200;
    const CALLBACK: u32 = 0x7100_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn setup(level: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x204 / 4];
        slots[3] = COOPERATIVE;
        slots[0x200 / 4] = SLOT_200;
        e.put_vtable(0x7000_0000, &slots);
        e.map(0x0126_f000, 0x1000);
        e.map(0x7300_0000, 4);
        e.mem.set_u32(0x7300_0000, level);
        e.register(COOPERATIVE, |e, _| e.mem.u32(0x7300_0000).into_ret());
        e.register(SLOT_200, |_, _| Ret::default());
        e.register(RESET_DEVICE, |_, _| true.into_ret());
        e.register(REPORT, |_, _| Ret::default());
        // A callback answering its data word.
        e.register(CALLBACK, |_, a| (a[0] != 0).into_ret());
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, 0x7000_0000);
        let this = e.mem.alloc(0xb10);
        e.mem.set_u32(this + 0x288, device);
        (e, this)
    }

    #[test]
    fn device_states() {
        let (mut e, this) = setup(0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e74c00, &args![this]).bool());
        assert!(calls(&e, SLOT_200).is_empty());
        e.mem.set_u8(DEVICE_SLOT_200_FLAG, 1);
        e.mem.set_u32(0x7300_0000, DEVICE_NOT_RESET);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e74c00, &args![this]).bool());
        assert_eq!(calls(&e, SLOT_200).len(), 1);
        assert_eq!(calls(&e, RESET_DEVICE), vec![vec![this]]);
    }

    #[test]
    fn lost_device_runs_the_callbacks_once() {
        let (mut e, this) = setup(DEVICE_LOST);
        let callbacks = e.mem.alloc(12);
        let data = e.mem.alloc(12);
        e.mem.set_u32(callbacks, CALLBACK);
        e.mem.set_u32(callbacks + 8, CALLBACK);
        e.mem.set_u32(data, 1);
        e.mem.set_u32(data + 8, 1);
        e.mem.set_u32(this + 0xaf8, callbacks);
        e.mem.set_u32(this + 0xb08, data);
        e.mem.set_u16(this + 0xafe, 3);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e74c00, &args![this]).bool());
        assert_eq!(calls(&e, CALLBACK).len(), 2);
        assert!(calls(&e, REPORT).is_empty());
        assert_eq!(e.mem.u8(this + 0x6fc), 1);
        // Already lost: nothing runs.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e74c00, &args![this]).bool());
        assert!(calls(&e, CALLBACK).is_empty());
        // A failing callback reports and stops.
        e.mem.set_u8(this + 0x6fc, 0);
        e.mem.set_u32(data, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e74c00, &args![this]).bool());
        assert_eq!(calls(&e, CALLBACK).len(), 1);
        assert_eq!(calls(&e, REPORT), vec![vec![CALLBACK_FAILED_MESSAGE]]);
    }
}
