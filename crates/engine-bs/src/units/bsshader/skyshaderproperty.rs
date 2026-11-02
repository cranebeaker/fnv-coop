//! `BSShader/skyshaderproperty.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bsshader::run_00bae980::SkyShaderProperty;

/// Imported `InterlockedIncrement` and `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00bae930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SkyShaderProperty::SetBlendTexture` (Xbox PDB): when `texture` is not
/// the current `spBlendTexture`, releases the old one (`InterlockedDecrement`
/// on its count at +4; its vtable slot +4 when that reaches 0), stores
/// `texture` and takes a reference to it (`InterlockedIncrement`) when it is
/// not null.
pub fn sky_shader_property_set_blend_texture(
    e: &mut Engine,
    this: Ptr<SkyShaderProperty>,
    texture: u32,
) {
    let old = e.get(this, SkyShaderProperty::spBlendTexture);
    if old == texture {
        return;
    }
    if old != 0
        && e.call(IMPORT_INTERLOCKED_DECREMENT, &args![old.wrapping_add(4)])
            .u32()
            == 0
    {
        e.vcall(old, 4, &args![]);
    }
    e.set(this, SkyShaderProperty::spBlendTexture, texture);
    if texture != 0 {
        e.call(
            IMPORT_INTERLOCKED_INCREMENT,
            &args![texture.wrapping_add(4)],
        );
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00ba_e930,
        sky_shader_property_set_blend_texture(Ptr<SkyShaderProperty>, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The interlocked imports as plain counter steps.
    fn interlocked(e: &mut Engine) {
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], value);
            Ret {
                eax: value,
                ..Ret::default()
            }
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], value);
            Ret {
                eax: value,
                ..Ret::default()
            }
        });
    }

    fn texture(e: &mut Engine, count: u32) -> u32 {
        e.put_vtable(0x0109_0000, &[0x0050_0000, 0x0050_0004]);
        let texture = e.mem.alloc(8);
        e.mem.set_u32(texture, 0x0109_0000);
        e.mem.set_u32(texture + 4, count);
        texture
    }

    #[test]
    fn swaps_the_reference_and_deletes_the_last_one() {
        let mut e = Engine::new();
        interlocked(&mut e);
        e.register(0x0050_0004, |_, _| Ret::default());
        let sky: Ptr<SkyShaderProperty> = e.new_object();
        let first = texture(&mut e, 0);
        e.call(0x00ba_e930, &args![sky, first]);
        assert_eq!(e.get(sky, SkyShaderProperty::spBlendTexture), first);
        assert_eq!(e.mem.u32(first + 4), 1);
        // The same texture again changes nothing.
        e.call(0x00ba_e930, &args![sky, first]);
        assert_eq!(e.mem.u32(first + 4), 1);
        // A new one releases the old: its count reaches 0, so it is deleted.
        let second = texture(&mut e, 1);
        e.call_log = Some(vec![]);
        e.call(0x00ba_e930, &args![sky, second]);
        assert_eq!(e.mem.u32(first + 4), 0);
        assert_eq!(e.mem.u32(second + 4), 2);
        let log = e.call_log.take().unwrap();
        let deletes: Vec<_> = log.iter().filter(|c| c.0 == 0x0050_0004).collect();
        assert_eq!(deletes.len(), 1);
        assert_eq!(deletes[0].1, vec![first]);
        // Null clears it after releasing the held one (still referenced).
        e.call_log = Some(vec![]);
        e.call(0x00ba_e930, &args![sky, 0u32]);
        assert_eq!(e.get(sky, SkyShaderProperty::spBlendTexture), 0);
        assert_eq!(e.mem.u32(second + 4), 1);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != 0x0050_0004));
    }
}
