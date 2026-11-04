//! `(unplaced)/run_00a981d0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiTexture::RendererData`'s vtable (its RTTI names the class).
const RENDERER_DATA_VTABLE: u32 = 0x010a_080c;
/// `NiPersistentSrcTextureRendererData`'s vtable (its RTTI names the class).
const PERSISTENT_SRC_VTABLE: u32 = 0x010a_08ac;
/// The base constructor `00a5d3a0` and destructor `00a5d3d0` (thiscall;
/// the engine map puts both in `NiMain/niobject.cpp`).
const BASE_CONSTRUCTOR: u32 = 0x00a5_d3a0;
const BASE_DESTRUCTOR: u32 = 0x00a5_d3d0;
/// `NiPixelFormat::NiPixelFormat` (engine map), `00a7cca0` (thiscall).
const PIXEL_FORMAT_CONSTRUCTOR: u32 = 0x00a7_cca0;
/// `InterlockedDecrement` (import slot; stdcall: the address).
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// Frees a block through the Ni allocator (`00aa10f0`, cdecl; null is
/// ignored).
const NI_FREE: u32 = 0x00aa_10f0;
/// Allocates through the Ni allocator (`00aa13e0`, cdecl: the size).
const NI_ALLOC: u32 = 0x00aa_13e0;

// Translated from 00a981d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `NiPersistentSrcTextureRendererData` (the class of the
/// vtable it stores last; called by `00e916e6`'s code and by
/// [`fn_00a982d0`]): the base constructor `00a5d3a0`, the
/// `NiTexture::RendererData` vtable, `NiPixelFormat`'s constructor on +0x14,
/// the bytes +0x58..+0x5A set to 0xFF and +0x5B to 0, the words
/// +0x60..+0x8C cleared, its own vtable, and the words +0x90, +0x08, +0x0C
/// and +0x10 cleared. Returns `this`.
pub fn fn_00a981d0(e: &mut Engine, this: u32) -> u32 {
    e.call(BASE_CONSTRUCTOR, &[this]);
    e.mem.set_u32(this, RENDERER_DATA_VTABLE);
    e.call(PIXEL_FORMAT_CONSTRUCTOR, &[this + 0x14]);
    e.mem.set_u8(this + 0x58, 0xff);
    e.mem.set_u8(this + 0x59, 0xff);
    e.mem.set_u8(this + 0x5a, 0xff);
    e.mem.set_u8(this + 0x5b, 0);
    for offset in (0x60..=0x8c).step_by(4) {
        e.mem.set_u32(this + offset, 0);
    }
    e.mem.set_u32(this, PERSISTENT_SRC_VTABLE);
    e.mem.set_u32(this + 0x90, 0);
    e.mem.set_u32(this + 8, 0);
    e.mem.set_u32(this + 0xc, 0);
    e.mem.set_u32(this + 0x10, 0);
    this
}

/// Releases the reference-counted object at +0x90 (`InterlockedDecrement`
/// on its +4; at 0 its vtable slot +4, `DeleteThis`).
fn release_0x90(e: &mut Engine, this: u32) -> bool {
    let object = e.mem.u32(this + 0x90);
    if object == 0 {
        return false;
    }
    if e.call(IMPORT_INTERLOCKED_DECREMENT, &[object + 4]).u32() == 0 {
        e.vcall(object, 4, &[]);
    }
    true
}

// Translated from 00a98240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `NiPersistentSrcTextureRendererData` (called by its
/// scalar deleting destructor `00a98460`'s code and by `00e91750`): its
/// vtable, then releases the object at +0x90 (and clears the word), frees
/// the buffer at +0x60 through `00aa10f0` and clears +0x60..+0x6C when it is
/// set, releases +0x90 once more (now null, so nothing happens unless it was
/// set again), and goes on to the base destructor `00a5d3d0` with the
/// `NiTexture::RendererData` vtable, whose EAX it passes on (a tail jump).
pub fn fn_00a98240(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u32(this, PERSISTENT_SRC_VTABLE);
    if release_0x90(e, this) {
        e.mem.set_u32(this + 0x90, 0);
    }
    let buffer = e.mem.u32(this + 0x60);
    if buffer != 0 {
        e.call(NI_FREE, &[buffer]);
        e.mem.set_u32(this + 0x6c, 0);
        e.mem.set_u32(this + 0x68, 0);
        e.mem.set_u32(this + 0x64, 0);
        e.mem.set_u32(this + 0x60, 0);
    }
    release_0x90(e, this);
    e.mem.set_u32(this, RENDERER_DATA_VTABLE);
    e.call(BASE_DESTRUCTOR, &[this]).u32()
}

// Translated from 00a982d0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Creates a `NiPersistentSrcTextureRendererData` (registered as a function
/// pointer by `00a98460` and `00a89f62`'s code; Ghidra made no function of
/// it): 0x94 bytes from `00aa13e0`, constructed by [`fn_00a981d0`]; 0 when
/// the allocation fails.
pub fn fn_00a982d0(e: &mut Engine) -> u32 {
    let block = e.call(NI_ALLOC, &[0x94]).u32();
    if block == 0 {
        return 0;
    }
    fn_00a981d0(e, block)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a9_81d0, fn_00a981d0(u32) -> u32),
        entry!(0x00a9_8240, fn_00a98240(u32) -> u32),
        entry!(0x00a9_82d0, fn_00a982d0() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn construct_engine() -> Engine {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            a[0].into_ret()
        });
        e.register(PIXEL_FORMAT_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0x2222);
            a[0].into_ret()
        });
        e
    }

    #[test]
    fn constructor_sets_its_defaults() {
        let mut e = construct_engine();
        let this = e.mem.alloc(0x94);
        e.mem.write(this, &[0xaa; 0x94]);
        assert_eq!(e.call(0x00a9_81d0, &[this]).u32(), this);
        assert_eq!(e.mem.u32(this), PERSISTENT_SRC_VTABLE);
        assert_eq!(e.mem.u32(this + 0x14), 0x2222);
        assert_eq!(e.mem.bytes(this + 0x58, 4), vec![0xff, 0xff, 0xff, 0]);
        for offset in [8u32, 0xc, 0x10, 0x60, 0x7c, 0x8c, 0x90] {
            assert_eq!(e.mem.u32(this + offset), 0, "+{offset:#x}");
        }
        assert_eq!(e.mem.u8(this + 0x5c), 0xaa);
        assert_eq!(e.mem.u32(this + 4), 0xaaaa_aaaa);
    }

    #[test]
    fn creation_allocates_and_constructs() {
        let mut e = construct_engine();
        e.register(NI_ALLOC, |e, a| e.mem.alloc(a[0]).into_ret());
        let made = e.call(0x00a9_82d0, &[]).u32();
        assert_ne!(made, 0);
        assert_eq!(e.mem.u32(made), PERSISTENT_SRC_VTABLE);
        let mut e = construct_engine();
        e.register(NI_ALLOC, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_82d0, &[]).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn destructor_releases_frees_and_chains() {
        const DELETE_THIS: u32 = 0x00de_ad10;
        for (count, deleted) in [(1u32, true), (3, false)] {
            let mut e = Engine::new();
            e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
                let n = e.mem.u32(a[0]) - 1;
                e.mem.set_u32(a[0], n);
                n.into_ret()
            });
            e.register(DELETE_THIS, |_, _| 0u32.into_ret());
            e.register(NI_FREE, |_, _| 0u32.into_ret());
            e.register(BASE_DESTRUCTOR, |e, a| e.mem.u32(a[0]).into_ret());
            let vtable = e.mem.alloc(8);
            e.mem.set_u32(vtable + 4, DELETE_THIS);
            let object = e.mem.alloc(8);
            e.mem.set_u32(object, vtable);
            e.mem.set_u32(object + 4, count);
            let this = e.mem.alloc(0x94);
            e.mem.set_u32(this + 0x90, object);
            e.mem.set_u32(this + 0x60, 0x5000);
            e.mem.set_u32(this + 0x6c, 9);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00a9_8240, &[this]).u32(), RENDERER_DATA_VTABLE);
            let log = e.call_log.take().unwrap();
            assert_eq!(log.contains(&(DELETE_THIS, vec![object])), deleted);
            assert!(log.contains(&(NI_FREE, vec![0x5000])));
            assert_eq!(log.last().unwrap(), &(BASE_DESTRUCTOR, vec![this]));
            assert_eq!(e.mem.u32(object + 4), count - 1);
            for offset in [0x60u32, 0x6c, 0x90] {
                assert_eq!(e.mem.u32(this + offset), 0);
            }
        }
    }
}
