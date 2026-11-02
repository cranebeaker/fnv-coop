//! `NiCollision/nitrigon.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Unnamed in the engine map: normalizes the three floats at ECX in place.
const UNITIZE: u32 = 0x004a_0c10;

// Translated from 00eb3830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTrigon::GetNormal` (Xbox PDB): writes the normalized cross product of the two edges
/// (`m_kEdge0` at +0xc, `m_kEdge1` at +0x18) to `out` and answers `out`. Each component is the
/// difference of two products (`FLD`, `FMUL dword`, `FSUBP`) stored as a float, the y and z
/// first; `004a0c10` normalizes the result.
pub fn ni_trigon_get_normal(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let t = this.addr();
    let product = |e: &Engine, a: u32, b: u32| {
        x87_mul(
            x87_load32(e.mem.u32(t.wrapping_add(a))),
            x87_m32(e.mem.u32(t.wrapping_add(b))),
        )
    };
    let y = x87_store32(x87_sub(product(e, 0x18, 0x14), product(e, 0x20, 0xc)));
    let z = x87_store32(x87_sub(product(e, 0xc, 0x1c), product(e, 0x18, 0x10)));
    let x = x87_store32(x87_sub(product(e, 0x20, 0x10), product(e, 0x14, 0x1c)));
    let o = out.addr();
    e.mem.set_u32(o, x);
    e.mem.set_u32(o.wrapping_add(4), x87_quiet_bits(y));
    e.mem.set_u32(o.wrapping_add(8), x87_quiet_bits(z));
    e.call(UNITIZE, &args![out]);
    out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00eb_3830, ni_trigon_get_normal(Ptr, Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crosses_the_edges_and_normalizes() {
        let mut e = Engine::new();
        e.register(UNITIZE, |e, a| {
            let z = e.mem.f32(a[0] + 8);
            e.mem.set_f32(a[0] + 8, z / 4.0);
            Ret::default()
        });
        let trigon = e.mem.alloc(0x24);
        for (i, v) in [0.0f32, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 2.0, 0.0]
            .iter()
            .enumerate()
        {
            e.mem.set_f32(trigon + 4 * i as u32, *v);
        }
        let out = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00eb_3830, &args![trigon, out]).u32(), out);
        assert_eq!(e.call_log.as_ref().unwrap()[1], (UNITIZE, vec![out]));
        assert_eq!(e.mem.f32(out), 0.0);
        assert_eq!(e.mem.f32(out + 4), 0.0);
        assert_eq!(e.mem.f32(out + 8), 1.0);
    }
}
