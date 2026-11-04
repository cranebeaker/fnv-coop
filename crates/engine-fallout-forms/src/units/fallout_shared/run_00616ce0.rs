//! `fallout shared/run_00616ce0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Unnamed `TESResponse` (laid out in `tesresponse.rs`) helpers: the
//! response's constructor (`00616ce0`, as `GetResponseList` and the
//! response list copy use it), its data's constructor and its destructor.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared::tesresponse::TesResponse;

/// `004037b0` (`ECX` = string): the string object's constructor.
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
/// `004037f0` (`ECX` = string; characters, a word; `RET 8`): sets the
/// string.
const STRING_SET: u32 = 0x0040_37f0;
/// `004037d0` (`ECX` = string): the string object's destructor.
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// The emotion value the data constructor stores (0x32).
const DEFAULT_EMOTION_VALUE: u32 = 0x32;

// Translated from 00616ce0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00616ce0 - the exception-unwinding frame (FS:[0]) that destroys the text if a later step throws is not translated.
/// The response's (`ECX`) constructor: the data (`00616d70`), the text
/// (+0x18: `004037b0`, then `004037f0` with 0, 0), and the idles and next
/// (+0x20, +0x24, +0x28) zeroed. Returns the response. The engine map
/// gives it no name; `GetResponseList` (`0061e780`) and `0061cb60` call it
/// on a new 0x2c-byte block. The exception-unwinding frame is not
/// translated.
pub fn fn_00616ce0(e: &mut Engine, this: Ptr<TesResponse>) -> Ptr<TesResponse> {
    fn_00616d70(e, this);
    let text = this.addr() + 0x18;
    e.call(STRING_CONSTRUCT, &args![text]);
    e.call(STRING_SET, &args![text, 0u32, 0u32]);
    e.set(this, TesResponse::pSpeakerIdle, 0);
    e.set(this, TesResponse::pListenerIdle, 0);
    e.set(this, TesResponse::pNext, 0);
    this
}

// Translated from 00616d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The response data's (`ECX`, `RESPONSE_DATA`) constructor: emotion 0,
/// emotion value 0x32, topic 0, response number 0, sound 0, and the byte
/// at +0x14 (`bUseEmotion`) 1. Returns `ECX`. The engine map gives it no
/// name.
pub fn fn_00616d70(e: &mut Engine, this: Ptr<TesResponse>) -> Ptr<TesResponse> {
    e.set(this, TesResponse::eEmotion, 0);
    e.set(this, TesResponse::iEmotionValue, DEFAULT_EMOTION_VALUE);
    e.set(this, TesResponse::pConversationTopic, 0);
    e.set(this, TesResponse::ucResponseID, 0);
    e.set(this, TesResponse::pVoiceSound, 0);
    e.set(this, TesResponse::bUseEmotion, 1);
    this
}

// Translated from 00616dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The response's (`ECX`) destructor: destroys the text (+0x18,
/// `004037d0`). The engine map gives it no name.
pub fn fn_00616dc0(e: &mut Engine, this: Ptr<TesResponse>) {
    e.call(STRING_DESTRUCT, &args![this.addr() + 0x18]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00616ce0,
            fn_00616ce0(Ptr<TesResponse>) -> Ptr<TesResponse>
        ),
        entry!(
            0x00616d70,
            fn_00616d70(Ptr<TesResponse>) -> Ptr<TesResponse>
        ),
        entry!(0x00616dc0, fn_00616dc0(Ptr<TesResponse>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A response block filled with 0xaa bytes.
    fn block(e: &mut Engine) -> u32 {
        let p = e.mem.alloc(0x2c);
        for i in 0..0x2c {
            e.mem.set_u8(p + i, 0xaa);
        }
        p
    }

    #[test]
    fn the_constructor_sets_every_field() {
        let mut e = Engine::new();
        e.register(STRING_CONSTRUCT, |e, a| {
            e.mem.set_u64(a[0], 0);
            ret(a[0])
        });
        e.register(STRING_SET, |_, _| ret(0));
        e.call_log = Some(vec![]);
        let p = block(&mut e);
        assert_eq!(e.call(0x0061_6ce0, &args![p]).u32(), p);
        assert_eq!(e.mem.u32(p), 0);
        assert_eq!(e.mem.u32(p + 4), 0x32);
        assert_eq!(e.mem.u32(p + 8), 0);
        assert_eq!(e.mem.u8(p + 0xc), 0);
        // The padding after the response number is left alone.
        assert_eq!(e.mem.u8(p + 0xd), 0xaa);
        assert_eq!(e.mem.u32(p + 0x10), 0);
        assert_eq!(e.mem.u8(p + 0x14), 1);
        assert_eq!(e.mem.u8(p + 0x15), 0xaa);
        assert_eq!(e.mem.u32(p + 0x20), 0);
        assert_eq!(e.mem.u32(p + 0x24), 0);
        assert_eq!(e.mem.u32(p + 0x28), 0);
        assert_eq!(calls(&e, STRING_CONSTRUCT), vec![vec![p + 0x18]]);
        assert_eq!(calls(&e, STRING_SET), vec![vec![p + 0x18, 0, 0]]);
    }

    #[test]
    fn the_data_constructor_returns_its_object() {
        let mut e = Engine::new();
        let p = block(&mut e);
        assert_eq!(e.call(0x0061_6d70, &args![p]).u32(), p);
        assert_eq!(e.mem.u32(p + 4), 0x32);
        assert_eq!(e.mem.u32(p + 0x18), 0xaaaa_aaaa);
    }

    #[test]
    fn the_destructor_destroys_the_text() {
        let mut e = Engine::new();
        e.register(STRING_DESTRUCT, |_, _| ret(0));
        e.call_log = Some(vec![]);
        let p = block(&mut e);
        e.call(0x0061_6dc0, &args![p]);
        assert_eq!(calls(&e, STRING_DESTRUCT), vec![vec![p + 0x18]]);
    }
}
