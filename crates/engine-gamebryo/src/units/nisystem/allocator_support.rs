//! PC allocator startup helpers used before constructing form registries.
//! These unnamed address ranges do not establish original source-unit names.

use crate::prelude::*;

// Translated from 009fd0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the pointer word at the receiver; no stack arguments.
pub fn held_allocator(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr()))
}

// Translated from 00aa21d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the supplied allocator pointer and clears only byte +4. RET 4.
pub fn construct_holder(e: &mut Engine, this: Ptr, allocator: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), allocator.addr());
    e.mem.set_u8(this.addr() + 4, 0);
    this
}

// Translated from 00aa2230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Installs the PC vtable for `BSNiAllocator` (Xbox PDB).
pub fn construct_allocator(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x010a_2528);
    this
}

// Translated from 00aa2020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates a global pointer holder, resolves the previously constructed
/// allocator, then tail-calls its Initialize slot (+0x1c). The caller must
/// already have initialized the holder at global 011f6118.
pub fn initialize_allocator(e: &mut Engine) {
    let holder = e.call(0x00aa_3e40, &args![0x011f_6238u32, 4u32]).u32();
    let source = e.mem.u32(0x011f_6118);
    e.mem.set_u32(0x011f_6080, holder);
    let allocator = e.call(0x009f_d0f0, &args![source]).u32();
    let holder = e.mem.u32(0x011f_6080);
    e.mem.set_u32(holder, allocator);
    let holder = e.mem.u32(0x011f_6080);
    let allocator = e.mem.u32(holder);
    e.vcall(allocator, 0x1c, &[]);
}

// Translated from 00a29680 (decompiled, FalloutNV.exe 1.4.0.525)
/// A bare RET, folded across void call sites with and without a receiver.
/// It touches no memory and consumes no stack words; return registers are
/// unspecified for these void calls, so accept their differing argument lists.
pub fn empty_void_call(_e: &mut Engine, _args: &[u32]) -> Ret {
    Ret::default()
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009fd0f0, held_allocator(Ptr) -> Ptr),
        entry!(0x00aa21d0, construct_holder(Ptr, Ptr) -> Ptr),
        entry!(0x00aa2230, construct_allocator(Ptr) -> Ptr),
        entry!(0x00aa2020, initialize_allocator()),
        (0x00a29680, empty_void_call),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors_and_getter_preserve_guard_bytes() {
        // Generated native nv-call vectors, FalloutNV.exe 1.4.0.525 SHA256
        // 19406942e48724d797300c4ea6be9ac69a32f670b8c35db09279a2258422739f,
        // Ryzen AI 9 HX 370, FCW027F/MXCSR1F80: ctor, holder, getter agree.
        let mut e = Engine::new();
        let allocator = e.mem.alloc(8);
        e.mem.write(allocator, &[0xaa; 8]);
        assert_eq!(e.call(0x00aa2230, &[allocator]).u32(), allocator);
        assert_eq!(e.mem.u32(allocator), 0x010a2528);
        assert_eq!(e.mem.bytes(allocator + 4, 4), [0xaa; 4]);
        let holder = e.mem.alloc(8);
        e.mem.write(holder, &[0xaa; 8]);
        assert_eq!(e.call(0x00aa21d0, &[holder, allocator]).u32(), holder);
        assert_eq!(e.call(0x009fd0f0, &[holder]).u32(), allocator);
        assert_eq!(e.mem.bytes(holder + 4, 4), [0, 0xaa, 0xaa, 0xaa]);
        e.mem.write(holder, &[0xaa; 8]);
        e.call(0x00aa21d0, &[holder, 0x12345678]);
        assert_eq!(e.call(0x009fd0f0, &[holder]).u32(), 0x12345678);
        assert_eq!(
            e.mem.bytes(holder, 8),
            [0x78, 0x56, 0x34, 0x12, 0, 0xaa, 0xaa, 0xaa]
        );
    }

    #[test]
    fn initializer_reloads_global_holder_after_getter_callback() {
        let mut e = Engine::new();
        e.map(0x011f6000, 0x1000);
        let allocator = e.mem.alloc(4);
        let replacement = e.mem.alloc(4);
        e.put_vtable(0x03000000, &[0, 0, 0, 0, 0, 0, 0, 0x00a29680]);
        e.mem.set_u32(allocator, 0x03000000);
        e.mem.set_u32(0x011f6118, 0x1234);
        e.register_double(0x009fd0f0, move |e, args| {
            assert_eq!(args, &[0x1234]);
            e.mem.set_u32(0x011f6080, replacement);
            allocator.into_ret()
        });
        e.call(0x00aa2020, &[]);
        assert_eq!(e.mem.u32(replacement), allocator);
    }

    #[test]
    fn folded_void_call_accepts_both_receiver_shapes_without_stores() {
        let mut e = Engine::new();
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, 0x12345678);
        e.call(0x00a29680, &[]);
        e.call(0x00a29680, &[object]);
        assert_eq!(e.mem.u32(object), 0x12345678);
    }
}
