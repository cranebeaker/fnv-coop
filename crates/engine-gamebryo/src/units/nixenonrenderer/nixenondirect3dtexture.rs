//! `NiXenonRenderer/nixenondirect3dtexture.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Import slots of `EnterCriticalSection` and `LeaveCriticalSection`.
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// The critical section guarding the texture list (`011f4580`).
const TEXTURE_LIST_LOCK: u32 = 0x011f_4580;
/// The texture list's two end pointers (`011f4500`, `011f4504`).
const LIST_HEAD: u32 = 0x011f_4500;
const LIST_TAIL: u32 = 0x011f_4504;

// Translated from 00a5f800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexture::~NiTexture` (engine map; `ECX`, no words): unlinks the texture
/// from the global texture list under its critical section: a head
/// (`011f4500`) on this texture moves to its `+0x2C` link, a tail
/// (`011f4504`) to its `+0x28` link, and the neighbours (`+0x28`'s `+0x2C`,
/// `+0x2C`'s `+0x28`) are joined.
pub fn ni_texture_destructor(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.call(ENTER_CRITICAL_SECTION, &args![TEXTURE_LIST_LOCK]);
    if e.global::<u32>(LIST_HEAD) == t {
        let next = e.mem.u32(t + 0x2c);
        e.set_global(LIST_HEAD, next);
    }
    if e.global::<u32>(LIST_TAIL) == t {
        let previous = e.mem.u32(t + 0x28);
        e.set_global(LIST_TAIL, previous);
    }
    let previous = e.mem.u32(t + 0x28);
    if previous != 0 {
        let next = e.mem.u32(t + 0x2c);
        e.mem.set_u32(previous + 0x2c, next);
    }
    let next = e.mem.u32(t + 0x2c);
    if next != 0 {
        let previous = e.mem.u32(t + 0x28);
        e.mem.set_u32(next + 0x28, previous);
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![TEXTURE_LIST_LOCK]);
}

/// `NiTexture`'s RTTI record (`011f450c`).
const TEXTURE_RTTI: u32 = 0x011f_450c;
/// The vtable the deleting destructor installs (`0109b944`, the one whose slot 0 is this
/// deleting destructor).
const TEXTURE_VTABLE: u32 = 0x0109_b944;
/// The destructor body run after this unit's (`00a5c240`, unit `niobjectnet.cpp` in the
/// engine map; `thiscall`, no stack words).
const BASE_DESTRUCTOR_00A5C240: u32 = 0x00a5_c240;
/// `00aa1460` (`cdecl`, two words: block, size): the Ni sized delete.
const NI_SIZED_DELETE: u32 = 0x00aa_1460;

// Translated from 00a5fc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexture::GetRTTI` (Xbox PDB, engine map): answers the RTTI record `011f450c`.
pub fn ni_texture_get_rtti(_e: &mut Engine) -> u32 {
    TEXTURE_RTTI
}

// Translated from 00a60120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexture::'scalar deleting destructor'` (Xbox PDB, engine map), `thiscall` with one stack
/// word (the flags): installs the vtable `0109b944`, deletes the renderer data at `+0x24`
/// when set (its virtual slot 0 with 1), unlinks the texture (`00a5f800`), runs the base
/// destructor `00a5c240`, and when bit 0 of the flags is set frees the object with size 0x30
/// (`00aa1460`). Answers `this`.
pub fn ni_texture_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let t = this.addr();
    let renderer_data = e.mem.u32(t + 0x24);
    e.mem.set_u32(t, TEXTURE_VTABLE);
    if renderer_data != 0 {
        e.vcall(renderer_data, 0, &[1]);
    }
    ni_texture_destructor(e, this);
    e.call(BASE_DESTRUCTOR_00A5C240, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, 0x30u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a5_f800, ni_texture_destructor(Ptr)),
        entry!(0x00a5_fc00, ni_texture_get_rtti() -> u32),
        entry!(
            0x00a6_0120,
            ni_texture_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn destructor_unlinks_the_texture_under_the_lock() {
        let mut e = Engine::new();
        e.register(ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        e.map(LIST_HEAD, 8);
        let [a, b, c] = [e.mem.alloc(0x30), e.mem.alloc(0x30), e.mem.alloc(0x30)];
        e.mem.set_u32(a + 0x2c, b);
        e.mem.set_u32(b + 0x28, a);
        e.mem.set_u32(b + 0x2c, c);
        e.mem.set_u32(c + 0x28, b);
        e.set_global(LIST_HEAD, a);
        e.set_global(LIST_TAIL, c);
        e.call_log = Some(vec![]);
        // The middle one: its neighbours are joined.
        e.call(0x00a5_f800, &args![b]);
        assert_eq!(e.mem.u32(a + 0x2c), c);
        assert_eq!(e.mem.u32(c + 0x28), a);
        // The head and the tail move.
        e.call(0x00a5_f800, &args![a]);
        assert_eq!(e.global::<u32>(LIST_HEAD), c);
        assert_eq!(e.mem.u32(c + 0x28), 0);
        e.call(0x00a5_f800, &args![c]);
        assert_eq!(e.global::<u32>(LIST_HEAD), 0);
        assert_eq!(e.global::<u32>(LIST_TAIL), 0);
        let log = e.call_log.take().unwrap();
        let locks = log.iter().filter(|c| c.0 == ENTER_CRITICAL_SECTION).count();
        assert_eq!(locks, 3);
        assert!(log.contains(&(LEAVE_CRITICAL_SECTION, vec![TEXTURE_LIST_LOCK])));
    }

    #[test]
    fn get_rtti_answers_the_record() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a5_fc00, &[]).u32(), 0x011f_450c);
    }

    #[test]
    fn deleting_destructor_deletes_the_renderer_data_and_unlinks() {
        let mut e = Engine::new();
        e.register(ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(BASE_DESTRUCTOR_00A5C240, |_, _| Ret::default());
        e.register(NI_SIZED_DELETE, |_, _| Ret::default());
        e.register(0x0066_0000, |_, _| Ret::default());
        e.map(LIST_HEAD, 8);
        let vtable = e.mem.alloc(4);
        e.mem.set_u32(vtable, 0x0066_0000);
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, vtable);
        let texture = e.mem.alloc(0x30);
        e.mem.set_u32(texture + 0x24, data);
        e.set_global(LIST_HEAD, texture);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a6_0120, &args![texture, 1u32]).u32(), texture);
        assert_eq!(e.mem.u32(texture), TEXTURE_VTABLE);
        assert_eq!(e.global::<u32>(LIST_HEAD), 0);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().skip(1).map(|c| c.0).collect();
        assert_eq!(
            order,
            vec![
                0x0066_0000,
                ENTER_CRITICAL_SECTION,
                LEAVE_CRITICAL_SECTION,
                BASE_DESTRUCTOR_00A5C240,
                NI_SIZED_DELETE
            ]
        );
        assert_eq!(log[1].1, vec![data, 1]);
        assert_eq!(log[5].1, vec![texture, 0x30]);
        // No renderer data and no free.
        e.mem.set_u32(texture + 0x24, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a6_0120, &args![texture, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(!log
            .iter()
            .any(|c| c.0 == 0x0066_0000 || c.0 == NI_SIZED_DELETE));
    }
}
