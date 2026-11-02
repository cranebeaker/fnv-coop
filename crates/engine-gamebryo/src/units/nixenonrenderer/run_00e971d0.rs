//! `NiXenonRenderer/run_00e971d0` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiD3DHLSLVertexShader` table (`010f119c`, by its RTTI), the free `00aa10f0` (`cdecl`,
/// a block) and the base destructor body `00e89b70` (`nid3dvertexshader.cpp`; `thiscall`).
const HLSL_VERTEX_SHADER_VTABLE: u32 = 0x010f_119c;
const FREE_00AA10F0: u32 = 0x00aa_10f0;
const BASE_DESTRUCT_00E89B70: u32 = 0x00e8_9b70;

// Translated from 00e971d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiD3DHLSLVertexShader` destructor body (the RTTI of its
/// table), `thiscall`: its table, the blocks at +0x3c and +0x40 freed (`00aa10f0`), the object
/// at +0x44 released (slot 8) and cleared when set, then (a tail jump) the base destructor body
/// (`00e89b70`).
pub fn fn_00e971d0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let first = e.mem.u32(base.wrapping_add(0x3c));
    e.mem.set_u32(base, HLSL_VERTEX_SHADER_VTABLE);
    e.call(FREE_00AA10F0, &args![first]);
    let second = e.mem.u32(base.wrapping_add(0x40));
    e.call(FREE_00AA10F0, &args![second]);
    let held = e.mem.u32(base.wrapping_add(0x44));
    if held != 0 {
        e.vcall(held, 8, &args![]);
        e.mem.set_u32(base.wrapping_add(0x44), 0);
    }
    e.call(BASE_DESTRUCT_00E89B70, &args![base]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e9_71d0, fn_00e971d0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00e971d0_frees_and_releases_then_runs_the_base() {
        for held in [false, true] {
            let mut e = Engine::new();
            e.put_vtable(0x6000, &[0, 0, 0x6008]);
            for addr in [0x6008, FREE_00AA10F0, BASE_DESTRUCT_00E89B70] {
                e.register(addr, |_, _| Ret::default());
            }
            let this = e.mem.alloc(0x48);
            e.mem.set_u32(this + 0x3c, 0x0e01_0000);
            e.mem.set_u32(this + 0x40, 0x0e02_0000);
            let object = e.mem.alloc(4);
            e.mem.set_u32(object, 0x6000);
            if held {
                e.mem.set_u32(this + 0x44, object);
            }
            e.call_log = Some(vec![]);
            e.call(0x00e9_71d0, &args![this]);
            let log = e.call_log.take().unwrap();
            let order: Vec<(u32, Vec<u32>)> = log[1..].to_vec();
            let mut expected = vec![
                (FREE_00AA10F0, vec![0x0e01_0000]),
                (FREE_00AA10F0, vec![0x0e02_0000]),
            ];
            if held {
                expected.push((0x6008, vec![object]));
            }
            expected.push((BASE_DESTRUCT_00E89B70, vec![this]));
            assert_eq!(order, expected);
            assert_eq!(e.mem.u32(this), HLSL_VERTEX_SHADER_VTABLE);
            assert_eq!(e.mem.u32(this + 0x44), 0);
        }
    }
}
