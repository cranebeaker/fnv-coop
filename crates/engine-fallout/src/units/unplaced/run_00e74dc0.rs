//! `(unplaced)/run_00e74dc0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The renderer's fields these functions use: the Direct3D device (+0x288),
/// the device-lost state (+0x6FC), the resource array object (+0x888, with
/// its size at vtable +0xA4 and element at +0xC8; +0x88C cleared with it),
/// the list of resources to recreate (+0x900 head, +0x904 tail, +0x908
/// count; nodes are next, previous, element) and the object at +0x8C8.
const DEVICE: u32 = 0x288;
const DEVICE_LOST_STATE: u32 = 0x6fc;
const RESOURCES: u32 = 0x888;
const RESOURCES_SECOND: u32 = 0x88c;
const RECREATE_LIST: u32 = 0x900;
const RECREATE_TAIL: u32 = 0x904;
const RECREATE_COUNT: u32 = 0x908;
const OBJECT_8C8: u32 = 0x8c8;
/// `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiDX92DBufferData`'s `NiRTTI` record (its name from the static
/// initializer at `00fc7320`).
const BUFFER_DATA_RTTI: u32 = 0x0126_f59c;
/// The list's add (`00a5a310`, `thiscall (list, &element)`) and node free
/// (`00e6e830`, `thiscall (list, node)`).
const LIST_ADD: u32 = 0x00a5_a310;
const LIST_FREE_NODE: u32 = 0x00e6_e830;
/// The device check (`00e74c00`), the renderer step `00e74120` and the call
/// `00e8b560` on the object at +0x8C8 (all `thiscall`).
const DEVICE_CHECK: u32 = 0x00e7_4c00;
const RENDERER_00E74120: u32 = 0x00e7_4120;
const OBJECT_00E8B560: u32 = 0x00e8_b560;
/// The node pool these lists free into: its lock (`0040fbf0`, `thiscall
/// (lock, 0)` on `011c5f80`), its lock count (`011c5f84`, the lock's owner
/// word `011c5f80` cleared when it reaches 0) and its free list (`011c5f58`).
const POOL_LOCK: u32 = 0x011c_5f80;
const POOL_LOCK_COUNT: u32 = 0x011c_5f84;
const POOL_FREE_LIST: u32 = 0x011c_5f58;
const LOCK: u32 = 0x0040_fbf0;

fn field(this: Ptr, offset: u32) -> u32 {
    this.addr().wrapping_add(offset)
}

fn increment(e: &mut Engine, object: u32) {
    e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
}

/// Releases one reference (deleting through vtable +4 at 0).
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

/// Unlinks the head node of the recreate list (the new head's previous
/// link, or the tail, cleared) and answers it.
fn pop_head(e: &mut Engine, this: Ptr) -> u32 {
    let node = e.mem.u32(field(this, RECREATE_LIST));
    let next = e.mem.u32(node);
    e.mem.set_u32(field(this, RECREATE_LIST), next);
    if next != 0 {
        e.mem.set_u32(next + 4, 0);
    } else {
        e.mem.set_u32(field(this, RECREATE_TAIL), 0);
    }
    node
}

// Translated from 00e74dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: unless the device is lost (+0x6FC), walks the
/// resource array (+0x888): each element is cast to `NiDX92DBufferData`
/// (its RTTI, vtable +0xC, or a base at +4; a null element is used as it
/// is); when its vtable +0x64 answers non-zero the device's slot +0x88 runs
/// with its words at +0x18 and +0x14; when its vtable +0x4C answers true
/// and it is not yet in the recreate list (+0x900), it is added
/// (`00a5a310`, counted). Then the array and the word after it are
/// cleared. Answers true.
pub fn fn_00e74dc0(e: &mut Engine, this: Ptr) -> bool {
    if e.mem.u8(field(this, DEVICE_LOST_STATE)) != 0 {
        return true;
    }
    let mut index = 0u32;
    let array = e.mem.u32(field(this, RESOURCES));
    if e.vcall(array, 0xa4, &args![]).u32() != 0 {
        loop {
            let array = e.mem.u32(field(this, RESOURCES));
            let element = e.vcall(array, 0xc8, &args![index]).u32();
            let resource = if element == 0 {
                0
            } else {
                let mut rtti = e.vcall(element, 0xc, &args![]).u32();
                let mut kind = false;
                while rtti != 0 {
                    if rtti == BUFFER_DATA_RTTI {
                        kind = true;
                        break;
                    }
                    rtti = e.mem.u32(rtti + 4);
                }
                if kind {
                    element
                } else {
                    0
                }
            };
            if e.vcall(resource, 0x64, &args![]).u32() != 0 {
                let device = e.mem.u32(field(this, DEVICE));
                let a = e.mem.u32(resource + 0x18);
                let b = e.mem.u32(resource + 0x14);
                e.vcall(device, 0x88, &args![a, 0u32, b, 0u32, 0u32]);
            }
            if e.vcall(resource, 0x4c, &args![]).bool() {
                increment(e, resource);
                let mut node = e.mem.u32(field(this, RECREATE_LIST));
                let mut found = 0u32;
                while node != 0 {
                    let next = e.mem.u32(node);
                    if resource == e.mem.u32(node + 8) {
                        found = node;
                        break;
                    }
                    node = next;
                }
                release(e, resource);
                if found == 0 {
                    e.with_stack(4, |e, slot| {
                        e.mem.set_u32(slot.addr(), resource);
                        increment(e, resource);
                        e.call(LIST_ADD, &args![field(this, RECREATE_LIST), slot]);
                        release(e, resource);
                    });
                }
            }
            index = index.wrapping_add(1);
            let array = e.mem.u32(field(this, RESOURCES));
            if index >= e.vcall(array, 0xa4, &args![]).u32() {
                break;
            }
        }
    }
    e.mem.set_u32(field(this, RESOURCES), 0);
    e.mem.set_u32(field(this, RESOURCES_SECOND), 0);
    true
}

// Translated from 00e74f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: true when the device check (`00e74c00`)
/// fails. Otherwise `00e74120`, `00e8b560` on the object at +0x8C8, and the
/// device's slot +0xA4: false when it answers a negative result; else the
/// recreate list is emptied (each node unlinked and freed, `00e6e830`, its
/// element held over the free) and the answer is true.
pub fn fn_00e74f40(e: &mut Engine, this: Ptr) -> bool {
    if !e.call(DEVICE_CHECK, &args![this]).bool() {
        return true;
    }
    e.call(RENDERER_00E74120, &args![this]);
    let object = e.mem.u32(field(this, OBJECT_8C8));
    e.call(OBJECT_00E8B560, &args![object]);
    let device = e.mem.u32(field(this, DEVICE));
    if (e.vcall(device, 0xa4, &args![]).u32() as i32) < 0 {
        return false;
    }
    while e.mem.u32(field(this, RECREATE_COUNT)) != 0 {
        let node = pop_head(e, this);
        let element = e.mem.u32(node + 8);
        if element != 0 {
            increment(e, element);
        }
        e.call(LIST_FREE_NODE, &args![field(this, RECREATE_LIST), node]);
        let count = e.mem.u32(field(this, RECREATE_COUNT)).wrapping_sub(1);
        e.mem.set_u32(field(this, RECREATE_COUNT), count);
        if element != 0 {
            release(e, element);
        }
    }
    true
}

// Translated from 00e75000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: unless the device is lost (+0x6FC), empties
/// the recreate list: each node is unlinked, its element held and the
/// node's reference dropped (cleared), the node returned to the pool's free
/// list under the pool lock (`0040fbf0`; the lock word cleared when its
/// count reaches 0), and the element's vtable +0x50 run before it is
/// released. Answers true.
pub fn fn_00e75000(e: &mut Engine, this: Ptr) -> bool {
    if e.mem.u8(field(this, DEVICE_LOST_STATE)) != 0 {
        return true;
    }
    while e.mem.u32(field(this, RECREATE_COUNT)) != 0 {
        let node = pop_head(e, this);
        let element = e.mem.u32(node + 8);
        if element != 0 {
            increment(e, element);
        }
        let held = e.mem.u32(node + 8);
        if held != 0 {
            release(e, held);
            e.mem.set_u32(node + 8, 0);
        }
        e.call(LOCK, &args![POOL_LOCK, 0u32]);
        e.mem.set_u32(node + 4, 0);
        let free = e.mem.u32(POOL_FREE_LIST);
        e.mem.set_u32(node, free);
        let count = e.mem.u32(POOL_LOCK_COUNT).wrapping_sub(1);
        e.mem.set_u32(POOL_LOCK_COUNT, count);
        e.mem.set_u32(POOL_FREE_LIST, node);
        if count == 0 {
            e.mem.set_u32(POOL_LOCK, 0);
        }
        let remaining = e.mem.u32(field(this, RECREATE_COUNT)).wrapping_sub(1);
        e.mem.set_u32(field(this, RECREATE_COUNT), remaining);
        e.vcall(element, 0x50, &args![]);
        release(e, element);
    }
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e74dc0, fn_00e74dc0(Ptr) -> bool),
        entry!(0x00e74f40, fn_00e74f40(Ptr) -> bool),
        entry!(0x00e75000, fn_00e75000(Ptr) -> bool),
        entry!(0x00e7_50e0, fn_00e750e0(Ptr, Ptr)),
        entry!(0x00e7_52a0, fn_00e752a0(Ptr, u32) -> u32),
    ]
}

/// The `MemoryManager` singleton and its allocation (`00aa3e40`, thiscall: size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
/// The object allocator `00aa13e0` (cdecl: size).
const OBJECT_ALLOCATE: u32 = 0x00aa_13e0;
/// `NiPropertyState::NiPropertyState` (engine map, `00a89ab0`).
const PROPERTY_STATE_CTOR: u32 = 0x00a8_9ab0;
/// The three property constructors (`00a5f1e0`, `004391c0` (unit
/// `multiboundmarkerdata.cpp`), `00a5f4d0`; thiscall) and the property-state setter
/// `00a59f50` (unit `niavobject.cpp`; thiscall on the state: the property).
const PROPERTY_CTOR_00A5F1E0: u32 = 0x00a5_f1e0;
const PROPERTY_CTOR_004391C0: u32 = 0x0043_91c0;
const PROPERTY_CTOR_00A5F4D0: u32 = 0x00a5_f4d0;
const PROPERTY_STATE_SET: u32 = 0x00a5_9f50;
/// The renderer's own state (+0x8a8), the object and word it calls with a geometry (+0x8ac,
/// vtable +0x10; +0x8b0), and the current property state and effect words (+0xc, +0x10).
const OWN_PROPERTY_STATE: u32 = 0x8a8;
const BATCH_OBJECT: u32 = 0x8ac;
const BATCH_WORD: u32 = 0x8b0;
/// `00e74cc0` (the geometry buffer preparation) and `00e715a0` (unit NiXenonRenderer;
/// thiscall, six words).
const PREPARE_GEOMETRY: u32 = 0x00e7_4cc0;
const RENDERER_DRAW_00E715A0: u32 = 0x00e7_15a0;
/// The identity rotation copied into the local transform (`011a9448`, nine floats), the
/// translation words (`011f426c`..`011f4274`) and the default property restored after
/// (`011f49ac`).
const IDENTITY_ROTATION: u32 = 0x011a_9448;
const ZERO_TRANSLATION: u32 = 0x011f_426c;
const DEFAULT_PROPERTY: u32 = 0x011f_49ac;
/// `NiXenonRenderer::AddToBatch` (engine map, `00e74a40`; thiscall, one word).
const ADD_TO_BATCH: u32 = 0x00e7_4a40;

// Translated from 00e750e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a vtable word of the renderer): draws one geometry with the
/// renderer's own property state. Nothing while the device is lost (+0x6fc) or without a
/// geometry or its word at +0x10. The current state and effects (+0xc, +0x10) are saved.
/// On first use the own state (+0x8a8) is made: an `NiPropertyState` (0x1c bytes from the
/// memory manager, null when that fails) given three new properties (0x1c bytes each from
/// `00aa13e0`, built by `00a5f1e0`, `004391c0`, `00a5f4d0`; their flag words at +0x18 set to
/// `(f & 0xffd7) | 0x10`, `(f & 0xfeed) | 0xed`, `f & 0xfffc`) through `00a59f50`. The
/// geometry's property (+0x14) is set on it, it becomes the current state (effects 0), the
/// object at +0x8ac gets slot +0x10 with the word at +0x8b0 and the geometry, the buffers
/// are prepared (`00e74cc0`), and `00e715a0` draws with an identity transform (rotation
/// from `011a9448`, translation from `011f426c`, scale 1), a 0x10-byte local and the
/// geometry's word at +0x1c. The default property (`011f49ac`) is set back and the saved
/// state restored.
pub fn fn_00e750e0(e: &mut Engine, this: Ptr, geometry: Ptr) {
    let renderer = this.addr();
    if e.mem.u8(renderer + DEVICE_LOST_STATE) != 0 {
        return;
    }
    let geometry = geometry.addr();
    if geometry == 0 || e.mem.u32(geometry + 0x10) == 0 {
        return;
    }
    let saved_state = e.mem.u32(renderer + 0xc);
    let saved_effects = e.mem.u32(renderer + 0x10);
    if e.mem.u32(renderer + OWN_PROPERTY_STATE) == 0 {
        let block = e
            .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, 0x1cu32])
            .u32();
        let state = if block != 0 {
            e.call(PROPERTY_STATE_CTOR, &args![block]).u32()
        } else {
            0
        };
        e.mem.set_u32(renderer + OWN_PROPERTY_STATE, state);
        for (ctor, mask, set) in [
            (PROPERTY_CTOR_00A5F1E0, 0xffd7u16, 0x10u16),
            (PROPERTY_CTOR_004391C0, 0xfeed, 0xed),
            (PROPERTY_CTOR_00A5F4D0, 0xfffc, 0),
        ] {
            let block = e.call(OBJECT_ALLOCATE, &args![0x1cu32]).u32();
            let property = if block != 0 {
                e.call(ctor, &args![block]).u32()
            } else {
                0
            };
            let flags = e.mem.u16(property.wrapping_add(0x18));
            e.mem
                .set_u16(property.wrapping_add(0x18), (flags & mask) | set);
            let state = e.mem.u32(renderer + OWN_PROPERTY_STATE);
            e.call(PROPERTY_STATE_SET, &args![state, property]);
        }
    }
    let state = e.mem.u32(renderer + OWN_PROPERTY_STATE);
    let property = e.mem.u32(geometry + 0x14);
    e.call(PROPERTY_STATE_SET, &args![state, property]);
    let state = e.mem.u32(renderer + OWN_PROPERTY_STATE);
    let word = e.mem.u32(renderer + BATCH_WORD);
    e.mem.set_u32(renderer + 0xc, state);
    let object = e.mem.u32(renderer + BATCH_OBJECT);
    e.mem.set_u32(renderer + 0x10, 0);
    e.vcall(object, 0x10, &args![word, geometry]);
    e.call(PREPARE_GEOMETRY, &args![renderer, geometry]);
    e.with_stack(0x50, |e, frame| {
        // The exe's frame: a 0x10-byte local at +0xc, the transform (rotation, translation,
        // scale) at +0x1c.
        let local = frame.addr() + 0xc;
        let transform = frame.addr() + 0x1c;
        let rotation = e.mem.bytes(IDENTITY_ROTATION, 0x24);
        e.mem.write(transform, &rotation);
        e.mem.set_f32(transform + 0x30, 1.0);
        let x = e.mem.u32(ZERO_TRANSLATION);
        e.mem.set_u32(transform + 0x24, x);
        let y = e.mem.u32(ZERO_TRANSLATION + 4);
        e.mem.set_u32(transform + 0x28, y);
        let z = e.mem.u32(ZERO_TRANSLATION + 8);
        e.mem.set_u32(transform + 0x2c, z);
        let extra = e.mem.u32(geometry + 0x1c);
        e.call(
            RENDERER_DRAW_00E715A0,
            &args![renderer, 0u32, 0u32, 0u32, transform, local, extra],
        );
    });
    let default = e.mem.u32(DEFAULT_PROPERTY);
    let state = e.mem.u32(renderer + OWN_PROPERTY_STATE);
    e.call(PROPERTY_STATE_SET, &args![state, default]);
    e.mem.set_u32(renderer + 0xc, saved_state);
    e.mem.set_u32(renderer + 0x10, saved_effects);
}

// Translated from 00e752a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (two vtable words): a jump to `NiXenonRenderer::AddToBatch`
/// (`00e74a40`, `RET 4`) with `this` and the word, passing on its `EAX`.
pub fn fn_00e752a0(e: &mut Engine, this: Ptr, object: u32) -> u32 {
    e.call(ADD_TO_BATCH, &args![this, object]).u32()
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    #[test]
    fn add_to_batch_thunk_passes_its_words_and_answer() {
        let mut e = Engine::new();
        e.register(ADD_TO_BATCH, |_, a| (a[0] + a[1]).into_ret());
        assert_eq!(e.call(0x00e7_52a0, &args![0x10u32, 0x5u32]).u32(), 0x15);
    }

    #[test]
    fn draw_with_own_state_builds_it_once_and_restores() {
        let mut e = Engine::new();
        e.map(IDENTITY_ROTATION, 0x24);
        e.mem.set_f32(IDENTITY_ROTATION, 1.0);
        e.map(ZERO_TRANSLATION, 0xc);
        e.mem.set_f32(ZERO_TRANSLATION + 4, 7.0);
        e.map(DEFAULT_PROPERTY, 4);
        e.mem.set_u32(DEFAULT_PROPERTY, 0xd00d);
        let state_block = e.mem.alloc(0x1c);
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |_, _| state_block.into_ret());
        e.register(PROPERTY_STATE_CTOR, |_, a| a[0].into_ret());
        e.register(OBJECT_ALLOCATE, |e, _| {
            let p = e.mem.alloc(0x1c);
            e.mem.set_u16(p + 0x18, 0xffff);
            p.into_ret()
        });
        for ctor in [
            PROPERTY_CTOR_00A5F1E0,
            PROPERTY_CTOR_004391C0,
            PROPERTY_CTOR_00A5F4D0,
        ] {
            e.register(ctor, |_, a| a[0].into_ret());
        }
        e.register(PROPERTY_STATE_SET, |_, _| Ret::default());
        e.register(PREPARE_GEOMETRY, |_, _| Ret::default());
        e.register(RENDERER_DRAW_00E715A0, |e, a| {
            assert_eq!(e.mem.f32(a[4]), 1.0);
            assert_eq!(e.mem.f32(a[4] + 0x28), 7.0);
            assert_eq!(e.mem.f32(a[4] + 0x30), 1.0);
            assert_eq!(a[5] + 0x10, a[4]);
            Ret::default()
        });
        e.register(0x0079_0010, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x14);
        e.mem.set_u32(vtable + 0x10, 0x0079_0010);
        let batch = e.mem.alloc(4);
        e.mem.set_u32(batch, vtable);
        let renderer = e.mem.alloc(0x900);
        e.mem.set_u32(renderer + BATCH_OBJECT, batch);
        e.mem.set_u32(renderer + BATCH_WORD, 0x77);
        e.mem.set_u32(renderer + 0xc, 0x1111);
        e.mem.set_u32(renderer + 0x10, 0x2222);
        let geometry = e.mem.alloc(0x20);
        e.mem.set_u32(geometry + 0x10, 1);
        e.mem.set_u32(geometry + 0x14, 0xbeef);
        e.mem.set_u32(geometry + 0x1c, 0x99);
        e.call_log = Some(vec![]);
        e.call(0x00e7_50e0, &args![renderer, geometry]);
        let log = e.call_log.take().unwrap();
        let sets: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == PROPERTY_STATE_SET)
            .map(|c| c.1[1])
            .collect();
        assert_eq!(sets.len(), 5);
        assert_eq!(&sets[3..], &[0xbeef, 0xd00d]);
        let flags: Vec<u16> = sets[..3].iter().map(|p| e.mem.u16(p + 0x18)).collect();
        assert_eq!(flags, vec![0xffd7, 0xfeed, 0xfffc]);
        assert!(log.contains(&(0x0079_0010, vec![batch, 0x77, geometry])));
        assert!(log.contains(&(PREPARE_GEOMETRY, vec![renderer, geometry])));
        let draw: Vec<_> = log
            .iter()
            .filter(|c| c.0 == RENDERER_DRAW_00E715A0)
            .collect();
        assert_eq!(&draw[0].1[..4], &[renderer, 0, 0, 0]);
        assert_eq!(draw[0].1[6], 0x99);
        assert_eq!(e.mem.u32(renderer + OWN_PROPERTY_STATE), state_block);
        assert_eq!(e.mem.u32(renderer + 0xc), 0x1111);
        assert_eq!(e.mem.u32(renderer + 0x10), 0x2222);
        // A second draw keeps the state; a lost device or an empty geometry does nothing.
        e.call_log = Some(vec![]);
        e.call(0x00e7_50e0, &args![renderer, geometry]);
        assert_eq!(
            e.call_log
                .take()
                .unwrap()
                .iter()
                .filter(|c| c.0 == PROPERTY_STATE_SET)
                .count(),
            2
        );
        e.mem.set_u32(geometry + 0x10, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e7_50e0, &args![renderer, geometry]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
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

    const VTABLE: u32 = 0x7000_0000;
    const DELETE: u32 = 0x7200_0004;
    const RTTI_SLOT: u32 = 0x7200_000c;
    const SLOT_4C: u32 = 0x7200_004c;
    const SLOT_50: u32 = 0x7200_0050;
    const SLOT_64: u32 = 0x7200_0064;
    const SLOT_88: u32 = 0x7200_0088;
    const SLOT_A4: u32 = 0x7200_00a4;
    const SLOT_C8: u32 = 0x7200_00c8;

    /// Objects share one vtable; each keeps its answers in its own words:
    /// +0x20 the RTTI, +0x24 the +0x64 answer, +0x28 the +0x4C answer;
    /// the array object keeps its count at +0x30 and its elements from
    /// +0x34; the device answers its +0x2C word for slot +0xA4.
    fn engine() -> Engine {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0xcc / 4];
        slots[1] = DELETE;
        slots[3] = RTTI_SLOT;
        slots[0x4c / 4] = SLOT_4C;
        slots[0x50 / 4] = SLOT_50;
        slots[0x64 / 4] = SLOT_64;
        slots[0x88 / 4] = SLOT_88;
        slots[0xa4 / 4] = SLOT_A4;
        slots[0xc8 / 4] = SLOT_C8;
        e.put_vtable(VTABLE, &slots);
        e.register(DELETE, |_, _| Ret::default());
        e.register(RTTI_SLOT, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(SLOT_64, |e, a| e.mem.u32(a[0] + 0x24).into_ret());
        e.register(SLOT_4C, |e, a| e.mem.u32(a[0] + 0x28).into_ret());
        e.register(SLOT_50, |_, _| Ret::default());
        e.register(SLOT_88, |_, _| Ret::default());
        e.register(SLOT_A4, |e, a| e.mem.u32(a[0] + 0x2c).into_ret());
        e.register(SLOT_C8, |e, a| e.mem.u32(a[0] + 0x34 + 4 * a[1]).into_ret());
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let n = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], n);
            n.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let n = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], n);
            n.into_ret()
        });
        e.map(0x011c_5000, 0x1000);
        e
    }

    fn object(e: &mut Engine) -> u32 {
        let o = e.mem.alloc(0x60);
        e.mem.set_u32(o, VTABLE);
        e.mem.set_u32(o + 4, 1);
        o
    }

    #[test]
    fn collects_the_buffers_to_recreate() {
        let mut e = engine();
        e.register(LIST_ADD, |e, a| {
            // Push a node (next, previous, element) at the head.
            let node = e.mem.alloc(0xc);
            e.mem.set_u32(node, e.mem.u32(a[0]));
            e.mem.set_u32(node + 8, e.mem.u32(a[1]));
            e.mem.set_u32(a[0], node);
            Ret::default()
        });
        let base = e.mem.alloc(8);
        e.mem.set_u32(base + 4, BUFFER_DATA_RTTI);
        let buffer = object(&mut e);
        e.mem.set_u32(buffer + 0x20, base);
        e.mem.set_u32(buffer + 0x24, 1);
        e.mem.set_u32(buffer + 0x28, 1);
        e.mem.set_u32(buffer + 0x14, 0x14);
        e.mem.set_u32(buffer + 0x18, 0x18);
        let array = object(&mut e);
        e.mem.set_u32(array + 0x2c, 2);
        // The same buffer twice: added once.
        e.mem.set_u32(array + 0x34, buffer);
        e.mem.set_u32(array + 0x38, buffer);
        let device = object(&mut e);
        let this = e.mem.alloc(0x910);
        e.mem.set_u32(this + 0x288, device);
        e.mem.set_u32(this + 0x888, array);
        e.mem.set_u32(this + 0x88c, 5);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e74dc0, &args![this]).bool());
        assert_eq!(calls(&e, LIST_ADD).len(), 1);
        assert_eq!(
            calls(&e, SLOT_88),
            vec![vec![device, 0x18, 0, 0x14, 0, 0]; 2]
        );
        let node = e.mem.u32(this + 0x900);
        assert_eq!(e.mem.u32(node + 8), buffer);
        assert_eq!(e.mem.u32(buffer + 4), 1);
        assert_eq!((e.mem.u32(this + 0x888), e.mem.u32(this + 0x88c)), (0, 0));
        // A lost device: nothing.
        e.mem.set_u8(this + 0x6fc, 1);
        e.mem.set_u32(this + 0x888, array);
        assert!(e.call(0x00e74dc0, &args![this]).bool());
        assert_eq!(e.mem.u32(this + 0x888), array);
    }

    fn with_list(e: &mut Engine, this: u32, elements: &[u32]) {
        let mut next = 0u32;
        for element in elements.iter().rev() {
            let node = e.mem.alloc(0xc);
            e.mem.set_u32(node, next);
            e.mem.set_u32(node + 8, *element);
            if next != 0 {
                e.mem.set_u32(next + 4, node);
            }
            next = node;
        }
        e.mem.set_u32(this + 0x900, next);
        e.mem.set_u32(this + 0x908, elements.len() as u32);
    }

    #[test]
    fn device_reset_empties_the_list() {
        let mut e = engine();
        e.register(RENDERER_00E74120, |_, _| Ret::default());
        e.register(OBJECT_00E8B560, |_, _| Ret::default());
        e.register(LIST_FREE_NODE, |_, _| Ret::default());
        e.map(0x7300_0000, 4);
        e.register(DEVICE_CHECK, |e, _| e.mem.u32(0x7300_0000).into_ret());
        let device = object(&mut e);
        let this = e.mem.alloc(0x910);
        e.mem.set_u32(this + 0x288, device);
        let a = object(&mut e);
        with_list(&mut e, this, &[a, 0]);
        // The check fails: true, nothing else.
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e74f40, &args![this]).bool());
        assert!(calls(&e, RENDERER_00E74120).is_empty());
        // A negative device answer: false.
        e.mem.set_u32(0x7300_0000, 1);
        e.mem.set_u32(device + 0x2c, 0x8000_0000);
        assert!(!e.call(0x00e74f40, &args![this]).bool());
        assert_eq!(e.mem.u32(this + 0x908), 2);
        e.mem.set_u32(device + 0x2c, 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e74f40, &args![this]).bool());
        assert_eq!(calls(&e, LIST_FREE_NODE).len(), 2);
        assert_eq!(e.mem.u32(this + 0x908), 0);
        assert_eq!(e.mem.u32(a + 4), 1);
    }

    #[test]
    fn recreate_runs_each_element_and_frees_the_nodes() {
        let mut e = engine();
        e.register(LOCK, |e, _| {
            let n = e.mem.u32(POOL_LOCK_COUNT) + 1;
            e.mem.set_u32(POOL_LOCK_COUNT, n);
            e.mem.set_u32(POOL_LOCK, 1);
            Ret::default()
        });
        let this = e.mem.alloc(0x910);
        let a = object(&mut e);
        let b = object(&mut e);
        e.mem.set_u32(a + 4, 2);
        with_list(&mut e, this, &[a, b]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e75000, &args![this]).bool());
        assert_eq!(calls(&e, SLOT_50), vec![vec![a], vec![b]]);
        // `a` loses the list's reference, `b` its only one (deleted).
        assert_eq!(e.mem.u32(a + 4), 1);
        assert_eq!(calls(&e, DELETE), vec![vec![b]]);
        assert_eq!(e.mem.u32(this + 0x908), 0);
        assert_eq!(e.mem.u32(this + 0x900), 0);
        assert_eq!(e.mem.u32(POOL_LOCK), 0);
        assert_ne!(e.mem.u32(POOL_FREE_LIST), 0);
    }
}
