//! `NiParticle/niparticleinfo.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiParticleInfo` (Xbox PDB, 0x1c bytes): `m_kVelocity` +0, `m_fAge` +0xc, `m_fLifeSpan`
/// +0x10, `m_fLastUpdate` +0x14, `m_usGeneration` +0x18, `m_usCode` +0x1a.
const AGE: u32 = 0xc;
const LIFE_SPAN: u32 = 0x10;
const LAST_UPDATE: u32 = 0x14;
const GENERATION: u32 = 0x18;
const CODE: u32 = 0x1a;
/// `NiPoint3::LoadBinary` (`00a59be0`, `thiscall`, one word: the stream).
const POINT_LOAD_BINARY_00A59BE0: u32 = 0x00a5_9be0;
/// The stream's file version (+0xd8; files before `0a030005` carry an extra point) and its
/// input binary stream (+0x24c) with its read function (word +8, `cdecl`: the object, the
/// buffer, the count, the address of the component size and the component count); the bytes
/// read are added to the object's word +4.
const STREAM_VERSION: u32 = 0xd8;
const EXTRA_POINT_VERSION: u32 = 0x0a03_0005;
const STREAM_INPUT_BINARY_STREAM: u32 = 0x24c;

// Translated from 00c31e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleInfo::IsEqual` (engine map, `thiscall`, one word: the other info): the six
/// floats (velocity, age, lifespan, last update) equal (`FUCOMPP`, unordered counts as
/// different), then the same generation and code.
pub fn ni_particle_info_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let (mine, theirs) = (this.addr(), other.addr());
    for offset in [0u32, 4, 8, AGE, LIFE_SPAN, LAST_UPDATE] {
        let left = x87_load32(e.mem.u32(mine + offset));
        let right = x87_load32(e.mem.u32(theirs + offset));
        if right != left {
            return false;
        }
    }
    e.mem.u16(mine + GENERATION) == e.mem.u16(theirs + GENERATION)
        && e.mem.u16(mine + CODE) == e.mem.u16(theirs + CODE)
}

/// Reads `count` bytes into `field` from the stream's input binary stream (the component
/// size through `size_slot`).
fn read_bytes(e: &mut Engine, stream: u32, field: u32, count: u32, size_slot: u32) {
    let binary = e.mem.u32(stream + STREAM_INPUT_BINARY_STREAM);
    let read = e.mem.u32(binary + 8);
    e.mem.set_u32(size_slot, count);
    let moved = e
        .call(read, &args![binary, field, count, size_slot, 1u32])
        .u32();
    let total = e.mem.u32(binary + 4).wrapping_add(moved);
    e.mem.set_u32(binary + 4, total);
}

// Translated from 00c31f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleInfo::LoadBinary` (`thiscall`, one word: the stream): the velocity
/// (`NiPoint3::LoadBinary`), for files before version `0a030005` one more point read into a
/// stack temporary and dropped, then the age, lifespan and last update (4 bytes each), the
/// generation and the code (2 bytes each) from the stream's input binary stream (the
/// component size kept in the argument's stack word).
pub fn ni_particle_info_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let (base, stream) = (this.addr(), stream.addr());
    e.call(POINT_LOAD_BINARY_00A59BE0, &args![this, stream]);
    e.with_stack(0x10, |e, frame| {
        // The 12-byte local point and the argument's stack word.
        let (point, argument) = (frame.addr(), frame.addr() + 0xc);
        e.mem.set_u32(argument, stream);
        if e.mem.u32(stream + STREAM_VERSION) < EXTRA_POINT_VERSION {
            e.call(POINT_LOAD_BINARY_00A59BE0, &args![point, stream]);
        }
        for (offset, count) in [
            (AGE, 4u32),
            (LIFE_SPAN, 4),
            (LAST_UPDATE, 4),
            (GENERATION, 2),
            (CODE, 2),
        ] {
            read_bytes(e, stream, base + offset, count, argument);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c3_1e90, ni_particle_info_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00c3_1f10, ni_particle_info_load_binary(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod n063_tests {
    use super::*;

    fn info(e: &mut Engine) -> u32 {
        let info = e.mem.alloc(0x1c);
        for word in 0..6 {
            e.mem.set_f32(info + 4 * word, word as f32);
        }
        e.mem.set_u16(info + GENERATION, 3);
        e.mem.set_u16(info + CODE, 4);
        info
    }

    #[test]
    fn is_equal_compares_every_field() {
        let mut e = Engine::new();
        let first = info(&mut e);
        let second = info(&mut e);
        assert!(e.call(0x00c3_1e90, &args![first, second]).bool());
        for (offset, value) in [
            (LIFE_SPAN, 0x4100_0000u32),
            (CODE, 9),
            (GENERATION, 9),
            (4, 0x7fc0_0000),
        ] {
            let copy = info(&mut e);
            e.mem.set_u32(copy + offset, value);
            if offset == CODE || offset == GENERATION {
                e.mem.set_u16(copy + offset, value as u16);
            }
            assert!(
                !e.call(0x00c3_1e90, &args![first, copy]).bool(),
                "offset {offset:#x}"
            );
        }
        // A NaN never compares equal, even with itself.
        e.mem.set_u32(first + AGE, 0x7fc0_0000);
        assert!(!e.call(0x00c3_1e90, &args![first, first]).bool());
    }

    #[test]
    fn load_binary_reads_the_fields_and_an_old_extra_point() {
        let mut e = Engine::new();
        e.register(POINT_LOAD_BINARY_00A59BE0, |_, _| Ret::default());
        e.register(0x0e60_0000, |e, a| {
            assert_eq!(e.mem.u32(a[3]), a[2]);
            for byte in 0..a[2] {
                e.mem.set_u8(a[1] + byte, 0x40 + a[2] as u8);
            }
            a[2].into_ret()
        });
        let binary = e.mem.alloc(0x10);
        e.mem.set_u32(binary + 8, 0x0e60_0000);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + STREAM_INPUT_BINARY_STREAM, binary);
        e.mem.set_u32(stream + STREAM_VERSION, EXTRA_POINT_VERSION);
        let this = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        e.call(0x00c3_1f10, &args![this, stream]);
        let loads: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == POINT_LOAD_BINARY_00A59BE0)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(loads, vec![vec![this, stream]]);
        assert_eq!(e.mem.u32(this + AGE), 0x4444_4444);
        assert_eq!(e.mem.u16(this + CODE), 0x4242);
        assert_eq!(e.mem.u32(binary + 4), 16);
        // An older file also reads the extra point into the stack.
        e.mem
            .set_u32(stream + STREAM_VERSION, EXTRA_POINT_VERSION - 1);
        e.call_log = Some(vec![]);
        e.call(0x00c3_1f10, &args![this, stream]);
        let loads = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == POINT_LOAD_BINARY_00A59BE0)
            .count();
        assert_eq!(loads, 2);
        assert_eq!(e.mem.u32(binary + 4), 32);
    }
}
