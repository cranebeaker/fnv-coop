//! `(unplaced)/run_00b79930` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The `NiPointer` at +0x5c (`NiD3DPass::m_spVertexShader` in the Xbox PDB).
const VERTEX_SHADER: u32 = 0x5c;

// Translated from 00b79950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall`, one stack word (the object): assigns it to the
/// `NiPointer` at +0x5c (`NiD3DPass::m_spVertexShader` in the Xbox PDB; the shader passes call
/// it on their passes with a vertex shader, the name `SetVertexShader` is unconfirmed).
/// Nothing happens when that object is already there; otherwise the old one's reference is
/// given up (`InterlockedDecrement` on its +4, deleted through its vtable slot +4 at zero),
/// the new one stored and, when not null, a reference taken on it.
pub fn fn_00b79950(e: &mut Engine, this: Ptr, object: Ptr) {
    let slot = this.addr().wrapping_add(VERTEX_SHADER);
    let object = object.addr();
    let old = e.mem.u32(slot);
    if old == object {
        return;
    }
    if old != 0 {
        let left = e
            .call(IMPORT_INTERLOCKED_DECREMENT, &args![old.wrapping_add(4)])
            .u32();
        if left == 0 {
            e.vcall(old, 4, &args![]);
        }
    }
    e.mem.set_u32(slot, object);
    if object != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
    }
}

/// `00e80190` (`thiscall`, no stack words; unit `nid3dpass.cpp` in the engine map, unnamed):
/// where `00b79930` ends when the count it lowers reaches zero.
const RELEASE_TARGET_00E80190: u32 = 0x00e8_0190;

// Translated from 00b79930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a pointer slot, no stack words): when the slot
/// holds an object, lowers the object's count at +0x64 (a plain decrement) and, when it
/// reaches zero, ends in `00e80190` on the object (a tail jump).
pub fn fn_00b79930(e: &mut Engine, this: Ptr) {
    let object = e.mem.u32(this.addr());
    if object == 0 {
        return;
    }
    let count = e.mem.u32(object + 0x64).wrapping_sub(1);
    e.mem.set_u32(object + 0x64, count);
    if count == 0 {
        e.call(RELEASE_TARGET_00E80190, &args![object]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b7_9930, fn_00b79930(Ptr)),
        entry!(0x00b7_9950, fn_00b79950(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const DELETE: u32 = 0x0e60_0000;
    const VTABLE: u32 = 0x0e61_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(DELETE, |_, _| Ret::default());
        e.put_vtable(VTABLE, &[0, DELETE]);
        e.call_log = Some(vec![]);
        e
    }

    fn object(e: &mut Engine, references: u32) -> u32 {
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, VTABLE);
        e.mem.set_u32(object + 4, references);
        object
    }

    #[test]
    fn fn_00b79950_swaps_the_references() {
        let mut e = engine();
        let pass = e.mem.alloc(0x68);
        let old = object(&mut e, 1);
        let new = object(&mut e, 1);
        e.mem.set_u32(pass + VERTEX_SHADER, old);
        e.call(0x00b7_9950, &args![pass, new]);
        assert_eq!(e.mem.u32(pass + VERTEX_SHADER), new);
        assert_eq!(e.mem.u32(new + 4), 2);
        assert_eq!(e.mem.u32(old + 4), 0);
        assert_eq!(calls(&e, DELETE), vec![vec![old]]);

        // The same object again changes nothing; a still-referenced old one is not deleted.
        e.call(0x00b7_9950, &args![pass, new]);
        assert_eq!(e.mem.u32(new + 4), 2);
        e.call(0x00b7_9950, &args![pass, 0u32]);
        assert_eq!(e.mem.u32(pass + VERTEX_SHADER), 0);
        assert_eq!(e.mem.u32(new + 4), 1);
        assert_eq!(calls(&e, DELETE).len(), 1);
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_INCREMENT).len(), 1);
    }
}

#[cfg(test)]
mod batch_b011_tests {
    use super::*;

    #[test]
    fn fn_00b79930_releases_at_zero() {
        let mut e = Engine::new();
        e.register(RELEASE_TARGET_00E80190, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let slot = e.mem.alloc(4);
        // An empty slot: nothing.
        e.call(0x00b7_9930, &args![slot]);
        let object = e.mem.alloc(0x68);
        e.mem.set_u32(object + 0x64, 2);
        e.mem.set_u32(slot, object);
        e.call(0x00b7_9930, &args![slot]);
        assert_eq!(e.mem.u32(object + 0x64), 1);
        e.call(0x00b7_9930, &args![slot]);
        assert_eq!(e.mem.u32(object + 0x64), 0);
        let log = e.call_log.take().unwrap();
        let released: Vec<_> = log
            .iter()
            .filter(|c| c.0 == RELEASE_TARGET_00E80190)
            .collect();
        assert_eq!(released.len(), 1);
        assert_eq!(released[0].1, vec![object]);
    }
}
