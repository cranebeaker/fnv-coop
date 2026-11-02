//! `fallout/magic/magiclight.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `MagicLight` (Xbox PDB), 0x1C bytes; the PC code uses the PDB's
    /// offsets.
    pub struct MagicLight: 0x1c {
        /// `pParent` (Xbox PDB): `NiPointer<NiNode>`.
        0x00 pParent: u32,
        /// `pLightObject` (Xbox PDB): `TESObjectLIGH*`.
        0x04 pLightObject: u32,
        /// `LightData` (Xbox PDB): a `REFR_LIGHT`, its light pointer first.
        0x08 LightData: u32,
        /// `LightData`'s second word (a `float` the code sets to 1.0).
        0x0C fLightDataFade: u32,
        /// `fFadeTimer` (Xbox PDB).
        0x10 fFadeTimer: u32,
        /// `fFadeTargetTime` (Xbox PDB).
        0x14 fFadeTargetTime: u32,
        /// `bFadeIn` (Xbox PDB).
        0x18 bFadeIn: u8,
    }
}

/// `LightData`'s offset.
const LIGHT_DATA: u32 = 0x08;
/// The `NiPointer` constructor `00633c90`, assignment `0066b0d0` (both
/// `thiscall`, one word), get `00559450` and destructor `0045cec0`.
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
const POINTER_GET: u32 = 0x0055_9450;
const POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// The `REFR_LIGHT` constructor `005726c0` and destructor `00418f40`
/// (`thiscall`).
const LIGHT_DATA_CONSTRUCT: u32 = 0x0057_26c0;
const LIGHT_DATA_DESTRUCT: u32 = 0x0041_8f40;
/// `004839c0` (`cdecl`, one word: the form of an id), `__RTDynamicCast`
/// (`00ec43fb`) and the type descriptors `01183028` (`TESForm`) and
/// `01183938` (`TESObjectLIGH`).
const LOOKUP_FORM_004839C0: u32 = 0x0048_39c0;
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_LIGH: u32 = 0x0118_3938;
/// The allocator `00aa13e0` (`cdecl`, the size) and the point light
/// constructor `00a7d670` (`thiscall`).
const ALLOCATE: u32 = 0x00aa_13e0;
const POINT_LIGHT_CONSTRUCT: u32 = 0x00a7_d670;
/// `005e3fc0` (`thiscall` on the light form: its colour word, red in the
/// low byte) and `006d2c40` (`thiscall` on the light form: a `float` in
/// `ST0`).
const LIGHT_COLOUR_005E3FC0: u32 = 0x005e_3fc0;
const LIGHT_VALUE_006D2C40: u32 = 0x006d_2c40;
/// `00416870` (`thiscall`, three floats: fills a point), `0080ed20` and
/// `00439180` (`thiscall`, one `float`: divide the point by it / scale it).
const SET_POINT_00416870: u32 = 0x0041_6870;
const POINT_DIVIDE_0080ED20: u32 = 0x0080_ed20;
const POINT_SCALE_00439180: u32 = 0x0043_9180;
/// `004bc2e0`, `0050dd50` (`thiscall` on the light, one word: a point) and
/// `0050dd20` (`thiscall` on the light, one `float`).
const LIGHT_SET_COLOUR_004BC2E0: u32 = 0x004b_c2e0;
const LIGHT_SET_POINT_0050DD50: u32 = 0x0050_dd50;
const LIGHT_SET_VALUE_0050DD20: u32 = 0x0050_dd20;
/// The floats 255.0 (`01023cd8`) and 500.0 (`01013d84`), the `double`s
/// 0.0 (`01012060`), 1.0 (`01012070`) and 255.0 (`0101e568`).
const COLOUR_SCALE: u32 = 0x0102_3cd8;
const LIGHT_POINT_X: u32 = 0x0101_3d84;
const ZERO_DOUBLE: u32 = 0x0101_2060;
const ONE_DOUBLE: u32 = 0x0101_2070;
const COLOUR_SCALE_DOUBLE: u32 = 0x0101_e568;
/// `00450b80` (`cdecl`, one word: the shadow scene node),
/// `ShadowSceneNode::AddLight` (engine map, `00b5c940`, `thiscall`, two
/// words) and `ShadowSceneNode::RemoveLight` (`00b5eed0`, one word).
const SHADOW_SCENE_NODE: u32 = 0x0045_0b80;
const ADD_LIGHT: u32 = 0x00b5_c940;
const REMOVE_LIGHT: u32 = 0x00b5_eed0;
/// `0050de20` (`thiscall` on the light form, two words).
const LIGHT_UPDATE_0050DE20: u32 = 0x0050_de20;

/// Sets `point` to the light form's colour (the colour word's low, middle
/// and high bytes as floats, `FILD`/`FSTP`).
fn colour_point(e: &mut Engine, light: u32, point: u32) {
    let colour = e.call(LIGHT_COLOUR_005E3FC0, &args![light]).u32();
    let blue = x87_store32(f64::from((colour >> 16) & 0xff));
    let green = x87_store32(f64::from(((colour & 0xffff) >> 8) & 0xff));
    let red = x87_store32(f64::from(colour & 0xff));
    e.call(SET_POINT_00416870, &args![point, red, green, blue]);
}

/// The light (`LightData`'s pointer).
fn light(e: &mut Engine, this: Ptr<MagicLight>) -> u32 {
    e.call(POINTER_GET, &args![this.addr().wrapping_add(LIGHT_DATA)])
        .u32()
}

// Translated from 0081bf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicLight::MagicLight` (engine map; `thiscall`, `RET 8`: the light
/// form's id word and the parent node): constructs `pParent` (null) and
/// `LightData`, clears `pLightObject` and the light, sets the fade word
/// 1.0, `bFadeIn` false and both fade times 0.0, then `Init` (`0081c030`).
/// Returns `this`.
// Translation incomplete: 0081bf80 - the FS:[0] exception frame (unwinding the members on a throw) is not translated
pub fn magic_light_magic_light(
    e: &mut Engine,
    this: Ptr<MagicLight>,
    form: u32,
    parent: u32,
) -> Ptr<MagicLight> {
    let base = this.addr();
    e.call(POINTER_CONSTRUCT, &args![base, 0u32]);
    e.call(LIGHT_DATA_CONSTRUCT, &args![base.wrapping_add(LIGHT_DATA)]);
    e.set(this, MagicLight::pLightObject, 0);
    e.call(POINTER_ASSIGN, &args![base.wrapping_add(LIGHT_DATA), 0u32]);
    e.set(this, MagicLight::fLightDataFade, 1.0f32.to_bits());
    e.set(this, MagicLight::bFadeIn, 0);
    e.set(this, MagicLight::fFadeTimer, 0);
    e.set(this, MagicLight::fFadeTargetTime, 0);
    magic_light_init(e, this, form, parent);
    this
}

// Translated from 0081c030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicLight::Init` (engine map; `thiscall`, `RET 8`: the light form's id
/// word and the parent node): `pLightObject` is the form (`004839c0`) cast
/// to `TESObjectLIGH`; with one, a new point light (0xFC bytes, `00a7d670`)
/// becomes `LightData`'s light with the fade word 1.0, its colour the
/// form's colour divided by 255 (`0080ed20`), its point (500, 0, 0)
/// (`0050dd50`), its value the form's `006d2c40` (`0050dd20`); it is added
/// to the shadow scene node when there is one (`AddLight` with 1), the
/// parent is stored and its slot +0xDC attaches the light (with 1).
// Translation incomplete: 0081c030 - the FS:[0] exception frame (freeing the light when its constructor throws) is not translated
pub fn magic_light_init(e: &mut Engine, this: Ptr<MagicLight>, form: u32, parent: u32) {
    let base = this.addr();
    let found = e.call(LOOKUP_FORM_004839C0, &args![form]).u32();
    let object = e
        .call(
            DYNAMIC_CAST,
            &args![found, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_LIGH, 0u32],
        )
        .u32();
    e.set(this, MagicLight::pLightObject, object);
    if e.get(this, MagicLight::pLightObject) == 0 {
        return;
    }
    let block = e.call(ALLOCATE, &args![0xfcu32]).u32();
    let created = if block != 0 {
        e.call(POINT_LIGHT_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.call(
        POINTER_ASSIGN,
        &args![base.wrapping_add(LIGHT_DATA), created],
    );
    e.set(this, MagicLight::fLightDataFade, 1.0f32.to_bits());
    e.with_stack(0x18, |e, locals| {
        // `[EBP-0x1c]` (the colour) and `[EBP-0x28]` (the point).
        let colour = locals.addr().wrapping_add(0xc);
        let point = locals.addr();
        let object = e.get(this, MagicLight::pLightObject);
        colour_point(e, object, colour);
        let scale = x87_quiet_bits(e.global::<u32>(COLOUR_SCALE));
        e.call(POINT_DIVIDE_0080ED20, &args![colour, scale]);
        let light_object = light(e, this);
        e.call(LIGHT_SET_COLOUR_004BC2E0, &args![light_object, colour]);
        let x = x87_quiet_bits(e.global::<u32>(LIGHT_POINT_X));
        e.call(SET_POINT_00416870, &args![point, x, 0u32, 0u32]);
        let light_object = light(e, this);
        e.call(LIGHT_SET_POINT_0050DD50, &args![light_object, point]);
    });
    let object = e.get(this, MagicLight::pLightObject);
    let value = e.call(LIGHT_VALUE_006D2C40, &args![object]).st0;
    let value = x87_store32(value);
    let light_object = light(e, this);
    e.call(LIGHT_SET_VALUE_0050DD20, &args![light_object, value]);
    let shadow = e.call(SHADOW_SCENE_NODE, &args![0u32]).u32();
    if shadow != 0 {
        let light_object = light(e, this);
        e.call(ADD_LIGHT, &args![shadow, light_object, 1u32]);
    }
    e.call(POINTER_ASSIGN, &args![base, parent]);
    let node = e.call(POINTER_GET, &args![base]).u32();
    let light_object = light(e, this);
    e.vcall(node, 0xdc, &args![light_object, 1u32]);
}

// Translated from 0081c240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicLight`'s destructor (`thiscall`): detaches the light from the
/// parent (its slot +0xE8) when there is one, clears the parent, removes
/// the light from the shadow scene node when there is one, clears the
/// light and `pLightObject`, and destroys `LightData` and `pParent`.
// Translation incomplete: 0081c240 - the FS:[0] exception frame (unwinding the members on a throw) is not translated
pub fn fn_0081c240(e: &mut Engine, this: Ptr<MagicLight>) {
    let base = this.addr();
    if e.call(POINTER_GET, &args![base]).u32() != 0 {
        let node = e.call(POINTER_GET, &args![base]).u32();
        let light_object = light(e, this);
        e.vcall(node, 0xe8, &args![light_object]);
    }
    e.call(POINTER_ASSIGN, &args![base, 0u32]);
    let shadow = e.call(SHADOW_SCENE_NODE, &args![0u32]).u32();
    if shadow != 0 {
        let light_object = light(e, this);
        e.call(REMOVE_LIGHT, &args![shadow, light_object]);
    }
    e.call(POINTER_ASSIGN, &args![base.wrapping_add(LIGHT_DATA), 0u32]);
    e.set(this, MagicLight::pLightObject, 0);
    e.call(LIGHT_DATA_DESTRUCT, &args![base.wrapping_add(LIGHT_DATA)]);
    e.call(POINTER_DESTRUCT, &args![base]);
}

// Translated from 0081c320 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `MagicLight` update (`thiscall`, `RET 4`: the elapsed seconds;
/// unnamed in the engine map). With a light form: while
/// `fFadeTargetTime` is above 0, the fade timer advances, the light's
/// colour becomes the form's colour scaled by the fade (`timer / target`
/// fading in, `1 - timer / target` fading out, ended at 1 or 0, which
/// clears the target time) divided by 255; then `0050de20` on the form
/// with `LightData` and 0.
pub fn fn_0081c320(e: &mut Engine, this: Ptr<MagicLight>, seconds: f32) {
    let base = this.addr();
    if e.get(this, MagicLight::pLightObject) == 0 {
        return;
    }
    let target = e.get(this, MagicLight::fFadeTargetTime);
    if x87_load32(target) > e.global::<f64>(ZERO_DOUBLE) {
        let timer = e.get(this, MagicLight::fFadeTimer);
        let timer = x87_store32(x87_add(x87_load32(timer), x87_m32(seconds.to_bits())));
        e.set(this, MagicLight::fFadeTimer, timer);
        e.with_stack(0xc, |e, colour| {
            let colour = colour.addr();
            let object = e.get(this, MagicLight::pLightObject);
            colour_point(e, object, colour);
            let timer = x87_load32(e.get(this, MagicLight::fFadeTimer));
            let target = x87_m32(e.get(this, MagicLight::fFadeTargetTime));
            let mut fade = if e.get(this, MagicLight::bFadeIn) != 0 {
                x87_store32(x87_div(timer, target))
            } else {
                x87_store32(x87_sub(1.0, x87_div(timer, target)))
            };
            if x87_load32(fade) > e.global::<f64>(ONE_DOUBLE) {
                e.set(this, MagicLight::fFadeTargetTime, 0);
                fade = 1.0f32.to_bits();
            } else if x87_load32(fade) < e.global::<f64>(ZERO_DOUBLE) {
                e.set(this, MagicLight::fFadeTargetTime, 0);
                fade = 0;
            }
            let scale = x87_store32(x87_div(
                x87_load32(fade),
                e.global::<f64>(COLOUR_SCALE_DOUBLE),
            ));
            e.call(POINT_SCALE_00439180, &args![colour, x87_quiet_bits(scale)]);
            let light_object = light(e, this);
            e.call(LIGHT_SET_COLOUR_004BC2E0, &args![light_object, colour]);
        });
    }
    let object = e.get(this, MagicLight::pLightObject);
    e.call(
        LIGHT_UPDATE_0050DE20,
        &args![object, base.wrapping_add(LIGHT_DATA), 0u32],
    );
}

// Translated from 0081c470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicLight::Fade` (engine map; `thiscall`, `RET 8`: fade in, the target
/// time): sets `bFadeIn` and `fFadeTargetTime` (through `FLD`/`FSTP`) and
/// restarts the fade timer at 0.0.
pub fn magic_light_fade(e: &mut Engine, this: Ptr<MagicLight>, fade_in: u8, time: f32) {
    e.set(this, MagicLight::bFadeIn, fade_in);
    e.set(
        this,
        MagicLight::fFadeTargetTime,
        x87_quiet_bits(time.to_bits()),
    );
    e.set(this, MagicLight::fFadeTimer, 0);
}

// Translated from 0081c4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicLight::Transfer` (engine map; `thiscall`, `RET 4`: a reference):
/// when the reference has 3D (its slot +0x1D0), moves the light there
/// (`0081c4e0`).
pub fn magic_light_transfer(e: &mut Engine, this: Ptr<MagicLight>, reference: u32) {
    if e.vcall(reference, 0x1d0, &[]).u32() != 0 {
        let node = e.vcall(reference, 0x1d0, &[]).u32();
        fn_0081c4e0(e, this, node);
    }
}

// Translated from 0081c4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`: a node): with a light form, stores the
/// node as `pParent` and attaches the light to it through its slot +0xDC
/// (with 1).
pub fn fn_0081c4e0(e: &mut Engine, this: Ptr<MagicLight>, node: u32) {
    if e.get(this, MagicLight::pLightObject) == 0 {
        return;
    }
    let base = this.addr();
    e.call(POINTER_ASSIGN, &args![base, node]);
    let parent = e.call(POINTER_GET, &args![base]).u32();
    let light_object = light(e, this);
    e.vcall(parent, 0xdc, &args![light_object, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0081_bf80,
            magic_light_magic_light(Ptr<MagicLight>, u32, u32) -> Ptr<MagicLight>
        ),
        entry!(0x0081_c030, magic_light_init(Ptr<MagicLight>, u32, u32)),
        entry!(0x0081_c240, fn_0081c240(Ptr<MagicLight>)),
        entry!(0x0081_c320, fn_0081c320(Ptr<MagicLight>, f32)),
        entry!(0x0081_c470, magic_light_fade(Ptr<MagicLight>, u8, f32)),
        entry!(0x0081_c4a0, magic_light_transfer(Ptr<MagicLight>, u32)),
        entry!(0x0081_c4e0, fn_0081c4e0(Ptr<MagicLight>, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn zero(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let r = e.call(addr, words);
        (r, e.call_log.take().unwrap())
    }

    /// Pointers that hold their object in their first word; a node whose
    /// slots +0xDC and +0xE8 are `0x0300_00dc` / `0x0300_00e8`.
    fn rig(e: &mut Engine) -> u32 {
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        zero(
            e,
            &[
                POINTER_CONSTRUCT,
                POINTER_DESTRUCT,
                LIGHT_DATA_CONSTRUCT,
                LIGHT_DATA_DESTRUCT,
                POINT_DIVIDE_0080ED20,
                POINT_SCALE_00439180,
                LIGHT_SET_COLOUR_004BC2E0,
                LIGHT_SET_POINT_0050DD50,
                LIGHT_SET_VALUE_0050DD20,
                ADD_LIGHT,
                REMOVE_LIGHT,
                LIGHT_UPDATE_0050DE20,
                0x0300_00dc,
                0x0300_00e8,
            ],
        );
        e.register(SET_POINT_00416870, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            ret(a[0])
        });
        e.register(LIGHT_COLOUR_005E3FC0, |_, _| ret(0x00c0_8040));
        e.register(LIGHT_VALUE_006D2C40, |_, _| Ret {
            st0: 300.0,
            ..Ret::default()
        });
        e.register(SHADOW_SCENE_NODE, |_, _| ret(0x5e5e));
        for (addr, bits) in [
            (COLOUR_SCALE, 255.0f32.to_bits()),
            (LIGHT_POINT_X, 500.0f32.to_bits()),
        ] {
            e.map(addr, 4);
            e.set_global::<u32>(addr, bits);
        }
        for (addr, value) in [
            (ZERO_DOUBLE, 0.0f64),
            (ONE_DOUBLE, 1.0),
            (COLOUR_SCALE_DOUBLE, 255.0),
        ] {
            e.map(addr, 8);
            e.set_global::<f64>(addr, value);
        }
        let node = e.mem.alloc(4);
        let vtable = e.mem.alloc(0xec);
        e.mem.set_u32(vtable + 0xdc, 0x0300_00dc);
        e.mem.set_u32(vtable + 0xe8, 0x0300_00e8);
        e.mem.set_u32(node, vtable);
        node
    }

    #[test]
    fn constructor_initialises_a_light() {
        let mut e = Engine::new();
        let node = rig(&mut e);
        e.register(LOOKUP_FORM_004839C0, |_, a| ret(a[0] + 1));
        e.register(DYNAMIC_CAST, |_, a| ret(a[0] + 1));
        e.register(ALLOCATE, |_, _| ret(0x9900));
        e.register(POINT_LIGHT_CONSTRUCT, |_, a| ret(a[0] + 4));
        let this = e.new_object::<MagicLight>();
        e.mem.write(this.addr(), &[0xffu8; 0x1c]);
        let (r, log) = run(&mut e, 0x0081_bf80, &args![this, 0x40u32, node]);
        assert_eq!(r.u32(), this.addr());
        assert_eq!(e.get(this, MagicLight::pLightObject), 0x42);
        assert_eq!(e.get(this, MagicLight::LightData), 0x9904);
        assert_eq!(e.get(this, MagicLight::pParent), node);
        assert_eq!(e.get(this, MagicLight::fLightDataFade), 1.0f32.to_bits());
        assert_eq!(e.get(this, MagicLight::bFadeIn), 0);
        assert_eq!(e.get(this, MagicLight::fFadeTimer), 0);
        assert_eq!(e.get(this, MagicLight::fFadeTargetTime), 0);
        assert_eq!(
            calls(&log, DYNAMIC_CAST),
            vec![vec![0x41, 0, TYPE_TES_FORM, TYPE_TES_OBJECT_LIGH, 0]]
        );
        assert_eq!(calls(&log, ALLOCATE), vec![vec![0xfc]]);
        let colour = calls(&log, SET_POINT_00416870)[0].clone();
        assert_eq!(
            colour[1..],
            [64.0f32.to_bits(), 128.0f32.to_bits(), 192.0f32.to_bits()]
        );
        assert_eq!(
            calls(&log, POINT_DIVIDE_0080ED20),
            vec![vec![colour[0], 255.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&log, LIGHT_SET_COLOUR_004BC2E0),
            vec![vec![0x9904, colour[0]]]
        );
        let point = calls(&log, SET_POINT_00416870)[1].clone();
        assert_eq!(point[1..], [500.0f32.to_bits(), 0, 0]);
        assert_eq!(
            calls(&log, LIGHT_SET_VALUE_0050DD20),
            vec![vec![0x9904, 300.0f32.to_bits()]]
        );
        assert_eq!(calls(&log, ADD_LIGHT), vec![vec![0x5e5e, 0x9904, 1]]);
        assert_eq!(calls(&log, 0x0300_00dc), vec![vec![node, 0x9904, 1]]);
        // Not a light: only the cast.
        e.register(DYNAMIC_CAST, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x0081_c030, &args![this, 0x40u32, node]);
        assert_eq!(e.get(this, MagicLight::pLightObject), 0);
        assert!(calls(&log, ALLOCATE).is_empty());
    }

    #[test]
    fn destructor_detaches_and_removes_the_light() {
        let mut e = Engine::new();
        let node = rig(&mut e);
        let this = e.new_object::<MagicLight>();
        e.set(this, MagicLight::pParent, node);
        e.set(this, MagicLight::LightData, 0x9904);
        e.set(this, MagicLight::pLightObject, 0x42);
        let (_, log) = run(&mut e, 0x0081_c240, &args![this]);
        assert_eq!(calls(&log, 0x0300_00e8), vec![vec![node, 0x9904]]);
        assert_eq!(calls(&log, REMOVE_LIGHT), vec![vec![0x5e5e, 0x9904]]);
        assert_eq!(e.get(this, MagicLight::pParent), 0);
        assert_eq!(e.get(this, MagicLight::LightData), 0);
        assert_eq!(e.get(this, MagicLight::pLightObject), 0);
        assert_eq!(
            calls(&log, LIGHT_DATA_DESTRUCT),
            vec![vec![this.addr() + 8]]
        );
        assert_eq!(calls(&log, POINTER_DESTRUCT), vec![vec![this.addr()]]);
        // No parent, no shadow scene node.
        e.register(SHADOW_SCENE_NODE, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x0081_c240, &args![this]);
        assert!(calls(&log, 0x0300_00e8).is_empty());
        assert!(calls(&log, REMOVE_LIGHT).is_empty());
    }

    #[test]
    fn update_fades_the_colour() {
        let mut e = Engine::new();
        rig(&mut e);
        let this = e.new_object::<MagicLight>();
        e.set(this, MagicLight::pLightObject, 0x42);
        e.set(this, MagicLight::LightData, 0x9904);
        e.set(this, MagicLight::fFadeTargetTime, 2.0f32.to_bits());
        e.set(this, MagicLight::fFadeTimer, 0.5f32.to_bits());
        e.set(this, MagicLight::bFadeIn, 1);
        let (_, log) = run(&mut e, 0x0081_c320, &args![this, 0.5f32.to_bits()]);
        assert_eq!(e.get(this, MagicLight::fFadeTimer), 1.0f32.to_bits());
        let scale = calls(&log, POINT_SCALE_00439180);
        let expected = x87_store32(0.5 / 255.0);
        assert_eq!(scale[0][1], expected);
        assert_eq!(
            calls(&log, LIGHT_UPDATE_0050DE20),
            vec![vec![0x42, this.addr() + 8, 0]]
        );
        // Fading out past the end: 0, the target cleared.
        e.set(this, MagicLight::bFadeIn, 0);
        e.set(this, MagicLight::fFadeTimer, 2.5f32.to_bits());
        let (_, log) = run(&mut e, 0x0081_c320, &args![this, 0.5f32.to_bits()]);
        assert_eq!(calls(&log, POINT_SCALE_00439180)[0][1], 0);
        assert_eq!(e.get(this, MagicLight::fFadeTargetTime), 0);
        // No target: only the update.
        let (_, log) = run(&mut e, 0x0081_c320, &args![this, 0.5f32.to_bits()]);
        assert!(calls(&log, POINT_SCALE_00439180).is_empty());
        assert_eq!(calls(&log, LIGHT_UPDATE_0050DE20).len(), 1);
        // Fading in past the end: 1.
        e.set(this, MagicLight::bFadeIn, 1);
        e.set(this, MagicLight::fFadeTargetTime, 1.0f32.to_bits());
        let (_, log) = run(&mut e, 0x0081_c320, &args![this, 0.5f32.to_bits()]);
        assert_eq!(
            calls(&log, POINT_SCALE_00439180)[0][1],
            x87_store32(1.0 / 255.0)
        );
        // Without a light form nothing at all.
        e.set(this, MagicLight::pLightObject, 0);
        let (_, log) = run(&mut e, 0x0081_c320, &args![this, 0.5f32.to_bits()]);
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn fade_transfer_and_attach() {
        let mut e = Engine::new();
        let node = rig(&mut e);
        let this = e.new_object::<MagicLight>();
        e.set(this, MagicLight::fFadeTimer, 7);
        e.call(0x0081_c470, &args![this, 0x101u32, 0x7f80_0001u32]);
        assert_eq!(e.get(this, MagicLight::bFadeIn), 1);
        assert_eq!(e.get(this, MagicLight::fFadeTargetTime), 0x7fc0_0001);
        assert_eq!(e.get(this, MagicLight::fFadeTimer), 0);
        // No light form: nothing attached.
        let reference = e.mem.alloc(4);
        let vtable = e.mem.alloc(0x1d4);
        e.mem.set_u32(vtable + 0x1d0, 0x0300_01d0);
        e.mem.set_u32(reference, vtable);
        e.register_double(0x0300_01d0, move |_, _| ret(node));
        let (_, log) = run(&mut e, 0x0081_c4a0, &args![this, reference]);
        assert_eq!(calls(&log, 0x0300_01d0).len(), 2);
        assert!(calls(&log, 0x0300_00dc).is_empty());
        e.set(this, MagicLight::pLightObject, 0x42);
        e.set(this, MagicLight::LightData, 0x9904);
        let (_, log) = run(&mut e, 0x0081_c4a0, &args![this, reference]);
        assert_eq!(e.get(this, MagicLight::pParent), node);
        assert_eq!(calls(&log, 0x0300_00dc), vec![vec![node, 0x9904, 1]]);
        e.register(0x0300_01d0, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x0081_c4a0, &args![this, reference]);
        assert_eq!(calls(&log, 0x0300_01d0).len(), 1);
    }
}
