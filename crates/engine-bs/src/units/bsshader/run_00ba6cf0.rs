//! `BSShader/run_00ba6cf0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two `ParticleShaderProperty` methods (its vtable is `010b8250`, whose `GetRTTI` answers
//! the record `01200434` the static initializer `00fba130` names `"ParticleShaderProperty"`).
//! The fields at their PC offsets (the Xbox PDB's less 8, the base `BSShaderProperty` being
//! 0x60 bytes on PC): `pInstanceData` (+0x60, an array of 0x30-byte
//! `ParticleShader::ParticleData`: `fXPos` +0, `fBirthTime` +0xc), `fTimer` (+0x104) and
//! `pMyGeometry` (+0x12c).

#[allow(unused_imports)]
use crate::prelude::*;

/// The particle count (`00ba52d0`, `cdecl`, no arguments: the word at `012003ec`, set on first
/// use from the shader level).
const PARTICLE_COUNT_00BA52D0: u32 = 0x00ba_52d0;
/// The `double` 3.4028234663852886e38 (`FLT_MAX`, `010231b0`) that marks an unused particle's
/// `fXPos`.
const UNUSED_MARK_010231B0: u32 = 0x0102_31b0;

// Translated from 00ba6ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ghidra made no function here: `ParticleShaderProperty`'s vtable slot +0x9c (`thiscall`, one
/// word): stores the word in `pMyGeometry` (`+0x12c`) and answers true (`MOV AL,1`).
pub fn fn_00ba6ce0(e: &mut Engine, this: Ptr, geometry: u32) -> bool {
    e.mem.set_u32(this.addr() + 0x12c, geometry);
    true
}

/// `fBirthTime += delta` for the particle at `particle` unless its `fXPos` equals the unused
/// mark (an ordered equal: `FUCOMP`, `TEST AH,0x44`, `JNP`); `delta_first` says whether the
/// code adds as `FLD delta; FADD [birth]` (else `FLD [birth]; FADD ST0,ST2`).
fn shift_birth_time(e: &mut Engine, particle: u32, delta: f64, mark: f64, delta_first: bool) {
    let x = x87_load32(e.mem.u32(particle));
    if x == mark {
        return;
    }
    let birth = e.mem.u32(particle.wrapping_add(0xc));
    let sum = if delta_first {
        x87_add(delta, x87_m32(birth))
    } else {
        x87_add(x87_load32(birth), delta)
    };
    e.mem.set_u32(particle.wrapping_add(0xc), x87_store32(sum));
}

// Translated from 00ba6cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a `ParticleShaderProperty`, one float: the new
/// time; `ResetAlphaTimer` `008217cf` calls it): moves every used particle's birth time by the
/// change of time. `delta = time - fTimer` (stored as a `float`); for each of the
/// `00ba52d0` particles of `pInstanceData` whose `fXPos` is not `FLT_MAX` (a NaN counts as
/// not equal) `fBirthTime` gets `delta` added; the code takes the particles four at a time
/// while at least four are left (`FLD delta; FADD`), then one at a time (`FLD; FADD ST0,ST2`).
/// Then `fTimer` becomes the time (`FLD`/`FSTP`).
pub fn fn_00ba6cf0(e: &mut Engine, this: Ptr, time: f32) {
    let base = this.addr();
    let delta = x87_sub(x87_load32(time.to_bits()), x87_m32(e.mem.u32(base + 0x104)));
    let delta = x87_load32(x87_store32(delta));
    let count = e.call(PARTICLE_COUNT_00BA52D0, &args![]).i32();
    let mark = e.mem.f64(UNUSED_MARK_010231B0);
    let mut done = 0i32;
    if count >= 4 {
        let groups = ((count as u32).wrapping_sub(4) >> 2).wrapping_add(1);
        done = groups.wrapping_mul(4) as i32;
        let mut offset = 0u32;
        for _ in 0..groups {
            for particle in 0..4u32 {
                let data = e.mem.u32(base + 0x60);
                let at = data.wrapping_add(offset).wrapping_add(particle * 0x30);
                shift_birth_time(e, at, delta, mark, true);
            }
            offset = offset.wrapping_add(0xc0);
        }
    }
    if done < count {
        let mut offset = (done as u32).wrapping_mul(0x30);
        for _ in 0..count.wrapping_sub(done) {
            let data = e.mem.u32(base + 0x60);
            shift_birth_time(e, data.wrapping_add(offset), delta, mark, false);
            offset = offset.wrapping_add(0x30);
        }
    }
    e.mem.set_u32(base + 0x104, x87_quiet_bits(time.to_bits()));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ba_6ce0, fn_00ba6ce0(Ptr, u32) -> bool),
        entry!(0x00ba_6cf0, fn_00ba6cf0(Ptr, f32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00ba6ce0_keeps_the_geometry() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x154);
        assert_eq!(e.call(0x00ba_6ce0, &args![this, 0x1234u32]).u32(), 1);
        assert_eq!(e.mem.u32(this + 0x12c), 0x1234);
    }

    #[test]
    fn fn_00ba6cf0_moves_the_used_particles_birth_times() {
        let mut e = Engine::new();
        e.map(0x0102_3000, 0x1000);
        e.mem.set_f64(UNUSED_MARK_010231B0, f64::from(f32::MAX));
        // Six particles: four in a group, then two one at a time.
        e.register(PARTICLE_COUNT_00BA52D0, |_, _| 6u32.into_ret());
        let this = e.mem.alloc(0x154);
        let data = e.mem.alloc(6 * 0x30);
        e.mem.set_u32(this + 0x60, data);
        e.mem.set_f32(this + 0x104, 10.0);
        for i in 0..6 {
            e.mem.set_f32(data + 0x30 * i + 0xc, i as f32);
        }
        // Particles 1 and 5 are unused; particle 3's position is a NaN (it counts as used).
        e.mem.set_f32(data + 0x30, f32::MAX);
        e.mem.set_f32(data + 0x30 * 5, f32::MAX);
        e.mem.set_u32(data + 0x30 * 3, 0x7fc0_0000);
        e.call(0x00ba_6cf0, &args![this, 12.5f32]);
        let births: Vec<f32> = (0..6).map(|i| e.mem.f32(data + 0x30 * i + 0xc)).collect();
        assert_eq!(births, vec![2.5, 1.0, 4.5, 5.5, 6.5, 5.0]);
        assert_eq!(e.mem.f32(this + 0x104), 12.5);
    }
}
