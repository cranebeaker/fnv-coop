//! `BSShader/bsrenderedtexture.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiRTTI` the object at `+0x30` is compared with (`012708e4`; its slot +8, `GetRTTI`,
/// answers it).
const RTTI_012708E4: u32 = 0x0127_08e4;
/// The error print `00a29680` (`cdecl`, one word: the message).
const PRINT_ERROR_00A29680: u32 = 0x00a2_9680;
/// `"NiRenderer::BeginUsingRenderTargetGroup> Failed - NULL pointer for render target. ..."`.
const NULL_TARGET_MESSAGE: u32 = 0x010a_e590;
/// `EnterCriticalSection` (KERNEL32 import slot).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
/// `LeaveCriticalSection` (KERNEL32 import slot).
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;

// Translated from 00b6b260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed by the body (the engine map's `BSRenderedTexture::StopOffscreen` comes from the
/// call graph only). `thiscall`, no stack words: 0 for a null `this`; when the object at
/// `+0x30` exists and its `GetRTTI` (slot +8) answers `012708e4`, the word at
/// `this + 8 + 4 * [object + 0x48]`; otherwise the word at `+8`.
pub fn fn_00b6b260(e: &mut Engine, this: Ptr) -> u32 {
    if this.is_null() {
        return 0;
    }
    let object = e.mem.u32(this.addr().wrapping_add(0x30));
    if object != 0 && e.vcall(object, 8, &args![]).u32() == RTTI_012708E4 {
        let object = e.mem.u32(this.addr().wrapping_add(0x30));
        let index = e.mem.u32(object.wrapping_add(0x48));
        return e.mem.u32(
            this.addr()
                .wrapping_add(index.wrapping_mul(4))
                .wrapping_add(8),
        );
    }
    e.mem.u32(this.addr().wrapping_add(8))
}

// Translated from 00b6b2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRenderer::BeginUsingRenderTargetGroup` (named by its own error message; the engine map
/// has `NiXenonRenderer::BeginUsingRenderTargetGroup`). `thiscall`, two stack words (the
/// target group, a clear mode). Answers 0 unless the frame state at `+0x200` is 1 or 2 and
/// the "using a target group" byte at `+0x208` is 0; a null group prints the error
/// (`00a29680`) and answers 0. Otherwise enters the critical section at `+0x80`, calls the
/// renderer's slot `+0x194` with the two words; on a non-zero answer sets `+0x208` to 1 and
/// keeps the section entered, else leaves it. Answers the slot's low byte.
pub fn ni_renderer_begin_using_render_target_group(
    e: &mut Engine,
    this: Ptr,
    group: u32,
    clear_mode: u32,
) -> u8 {
    let state = e.mem.u32(this.addr().wrapping_add(0x200));
    if (state != 1 && state != 2) || e.mem.u8(this.addr().wrapping_add(0x208)) != 0 {
        return 0;
    }
    if group == 0 {
        e.call(PRINT_ERROR_00A29680, &args![NULL_TARGET_MESSAGE]);
        return 0;
    }
    let section = this.addr().wrapping_add(0x80);
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    let result = e.vcall(this.addr(), 0x194, &args![group, clear_mode]).u32() as u8;
    if result != 0 {
        e.mem.set_u8(this.addr().wrapping_add(0x208), 1);
        return result;
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b6_b260, fn_00b6b260(Ptr) -> u32),
        entry!(
            0x00b6_b2a0,
            ni_renderer_begin_using_render_target_group(Ptr, u32, u32) -> u8
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_00b6b260_picks_the_indexed_word_for_the_matching_type() {
        let mut e = Engine::new();
        // GetRTTI answers the word at +4 of the object.
        e.put_vtable(0x0f00_1000, &[0, 0, 0x0f00_0000]);
        e.register(0x0f00_0000, |e, a| e.mem.u32(a[0] + 4).into_ret());
        assert_eq!(e.call(0x00b6_b260, &args![0u32]).u32(), 0);
        let this = e.mem.alloc(0x40);
        for i in 0..8 {
            e.mem.set_u32(this + 8 + 4 * i, 0x100 + i);
        }
        assert_eq!(e.call(0x00b6_b260, &args![this]).u32(), 0x100);
        let object = e.mem.alloc(0x50);
        e.mem.set_u32(object, 0x0f00_1000);
        e.mem.set_u32(object + 0x48, 3);
        e.mem.set_u32(this + 0x30, object);
        assert_eq!(e.call(0x00b6_b260, &args![this]).u32(), 0x100);
        e.mem.set_u32(object + 4, RTTI_012708E4);
        assert_eq!(e.call(0x00b6_b260, &args![this]).u32(), 0x103);
    }

    fn renderer(e: &mut Engine, answer: u32) -> u32 {
        let this = e.mem.alloc(0x210);
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(vtable + 0x194, 0x0f00_0010);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 4, answer);
        e.register(0x0f00_0010, |e, a| e.mem.u32(a[0] + 4).into_ret());
        for addr in [
            PRINT_ERROR_00A29680,
            ENTER_CRITICAL_SECTION,
            LEAVE_CRITICAL_SECTION,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        this
    }

    #[test]
    fn begin_using_render_target_group_refuses_the_wrong_state_and_a_null_group() {
        let mut e = Engine::new();
        let this = renderer(&mut e, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b6_b2a0, &args![this, 5u32, 0u32]).u32() as u8, 0);
        e.mem.set_u32(this + 0x200, 2);
        e.mem.set_u8(this + 0x208, 1);
        assert_eq!(e.call(0x00b6_b2a0, &args![this, 5u32, 0u32]).u32() as u8, 0);
        e.mem.set_u8(this + 0x208, 0);
        assert_eq!(e.call(0x00b6_b2a0, &args![this, 0u32, 0u32]).u32() as u8, 0);
        assert_eq!(
            calls(&e, PRINT_ERROR_00A29680),
            vec![vec![NULL_TARGET_MESSAGE]]
        );
        assert!(calls(&e, ENTER_CRITICAL_SECTION).is_empty());
    }

    #[test]
    fn begin_using_render_target_group_keeps_the_section_on_success() {
        let mut e = Engine::new();
        let this = renderer(&mut e, 0x301);
        e.mem.set_u32(this + 0x200, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b6_b2a0, &args![this, 5u32, 7u32]).u32() as u8, 1);
        assert_eq!(e.mem.u8(this + 0x208), 1);
        assert_eq!(calls(&e, 0x0f00_0010), vec![vec![this, 5, 7]]);
        assert_eq!(calls(&e, ENTER_CRITICAL_SECTION), vec![vec![this + 0x80]]);
        assert!(calls(&e, LEAVE_CRITICAL_SECTION).is_empty());
    }

    #[test]
    fn begin_using_render_target_group_leaves_the_section_on_failure() {
        let mut e = Engine::new();
        let this = renderer(&mut e, 0x300);
        e.mem.set_u32(this + 0x200, 2);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b6_b2a0, &args![this, 5u32, 7u32]).u32() as u8, 0);
        assert_eq!(e.mem.u8(this + 0x208), 0);
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION), vec![vec![this + 0x80]]);
    }
}
