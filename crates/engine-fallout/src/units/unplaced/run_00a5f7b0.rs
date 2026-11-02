//! `(unplaced)/run_00a5f7b0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiTexture::ms_kTextureListCriticalSection` (Xbox PDB name; the section the code enters
/// at `011f4580`), `NiTexture::ms_pkHead` (`011f4500`) and `NiTexture::ms_pkTail`
/// (`011f4504`): the list of every texture.
const TEXTURE_LIST_CRITICAL_SECTION: u32 = 0x011f_4580;
const TEXTURE_LIST_HEAD: u32 = 0x011f_4500;
const TEXTURE_LIST_TAIL: u32 = 0x011f_4504;
/// `NiTexture::m_pkPrev` (Xbox PDB) +0x28 and `m_pkNext` +0x2c.
const TEXTURE_PREVIOUS: u32 = 0x28;
const TEXTURE_NEXT: u32 = 0x2c;
/// `EnterCriticalSection` and `LeaveCriticalSection` import slots (stdcall: the section).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;

// Translated from 00a5f7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`__thiscall` on an `NiTexture`, no words; called by the `NiSourceTexture`
/// constructor, `00a7fb80` and `00ee8240`): appends the texture to the list of all textures
/// inside the list's critical section. It becomes the head when the list has none, links
/// after the old tail (or has no previous one) and becomes the tail, with no next one.
pub fn fn_00a5f7b0(e: &mut Engine, this: Ptr) {
    let texture = this.addr();
    e.call(
        ENTER_CRITICAL_SECTION,
        &args![TEXTURE_LIST_CRITICAL_SECTION],
    );
    if e.mem.u32(TEXTURE_LIST_HEAD) == 0 {
        e.mem.set_u32(TEXTURE_LIST_HEAD, texture);
    }
    let tail = e.mem.u32(TEXTURE_LIST_TAIL);
    if tail != 0 {
        e.mem.set_u32(tail + TEXTURE_NEXT, texture);
        let tail = e.mem.u32(TEXTURE_LIST_TAIL);
        e.mem.set_u32(texture + TEXTURE_PREVIOUS, tail);
    } else {
        e.mem.set_u32(texture + TEXTURE_PREVIOUS, 0);
    }
    e.mem.set_u32(TEXTURE_LIST_TAIL, texture);
    e.mem.set_u32(texture + TEXTURE_NEXT, 0);
    e.call(
        LEAVE_CRITICAL_SECTION,
        &args![TEXTURE_LIST_CRITICAL_SECTION],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a5f7b0, fn_00a5f7b0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_00a5f7b0_appends_to_the_texture_list() {
        let mut e = Engine::new();
        e.register(ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        e.map(TEXTURE_LIST_HEAD, 8);
        let a = e.mem.alloc(0x30);
        let b = e.mem.alloc(0x30);
        e.mem.set_u32(a + TEXTURE_PREVIOUS, 0x99);
        e.mem.set_u32(a + TEXTURE_NEXT, 0x99);
        e.call_log = Some(vec![]);
        e.call(0x00a5f7b0, &args![a]);
        assert_eq!(e.mem.u32(TEXTURE_LIST_HEAD), a);
        assert_eq!(e.mem.u32(TEXTURE_LIST_TAIL), a);
        assert_eq!(
            (e.mem.u32(a + TEXTURE_PREVIOUS), e.mem.u32(a + TEXTURE_NEXT)),
            (0, 0)
        );
        e.call(0x00a5f7b0, &args![b]);
        assert_eq!(e.mem.u32(TEXTURE_LIST_HEAD), a);
        assert_eq!(e.mem.u32(TEXTURE_LIST_TAIL), b);
        assert_eq!(e.mem.u32(a + TEXTURE_NEXT), b);
        assert_eq!(
            (e.mem.u32(b + TEXTURE_PREVIOUS), e.mem.u32(b + TEXTURE_NEXT)),
            (a, 0)
        );
        assert_eq!(
            calls_to(&e, ENTER_CRITICAL_SECTION),
            vec![vec![TEXTURE_LIST_CRITICAL_SECTION]; 2]
        );
        assert_eq!(
            calls_to(&e, LEAVE_CRITICAL_SECTION),
            vec![vec![TEXTURE_LIST_CRITICAL_SECTION]; 2]
        );
    }
}
