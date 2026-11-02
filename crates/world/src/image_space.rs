//! Image spaces (`IMGS`): the final color adjustment the game applies to
//! the picture in a cell: saturation, contrast, brightness and a tint (what
//! gives New Vegas its warm, washed-out look), plus HDR and bloom settings.

use esm::{FormId, FourCC, LoadOrder, LoadedPlugin, Record};

use crate::cell::{le_f32, le_u32};

const IMGS: FourCC = FourCC::new(b"IMGS");
const DNAM: FourCC = FourCC::new(b"DNAM");
const XCIM: FourCC = FourCC::new(b"XCIM");

/// Index of the first cinematic value among `DNAM`'s floats: after 14 HDR
/// values, 3 bloom, 3 "get hit" and 4 night-eye values. New Vegas's own
/// image spaces are this 132-byte layout (32 floats, then flags); the
/// longer 152-byte one adds a 15th HDR value (skin dimmer) and padding.
const CINEMATIC: usize = 24;
const CINEMATIC_LONG: usize = 25;
const LONG_SIZE: usize = 152;

/// An image space record.
#[derive(Debug, Clone, PartialEq)]
pub struct ImageSpace {
    pub form_id: FormId,
    pub editor_id: Option<String>,
    /// Size of `DNAM` in bytes.
    pub size: usize,
    /// Every float in `DNAM`, in order (the trailing flags read as floats
    /// too, so only the documented ones mean anything).
    pub values: Vec<f32>,
}

/// The cinematic part of an image space, applied in this order to each
/// pixel (in the game's gamma-encoded colors): mix toward gray by
/// `saturation`, toward `tint` times the pixel's brightness by
/// `tint_amount`, then scale by `brightness` and spread around
/// `contrast_average` by `contrast`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cinematic {
    pub saturation: f32,
    pub contrast_average: f32,
    pub contrast: f32,
    pub brightness: f32,
    /// 0..1 per channel.
    pub tint: [f32; 3],
    pub tint_amount: f32,
}

/// The HDR values the game's bloom and brightness steps use: `DNAM`'s first
/// floats, read the same way in every layout (the game's loader copies the
/// first 14 as stored and shifts only what follows). Confirmed against a
/// recording of the shader constants in Doc Mitchell's house
/// (`ShackInterior01`): the bright pass gets (`bright_clamp`,
/// `bright_scale`) = (0.9, 2.4), the blur reaches `blur_radius` = 6 texels
/// each way, and the average-brightness step gets `eye_adapt_speed` 0.3.
/// The Mojave Outpost barracks (`OfficeDefaultImageSpace`, where they
/// differ) settled the rest: the average's clamp is `upper_lum_clamp` (2),
/// the final pass's brightness limit `target_lum` (1), and self-lit
/// surfaces are multiplied by `emissive_mult` (3: the tube fixtures' glow
/// 1.0 × 1.3 arrived as 3.9).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hdr {
    pub eye_adapt_speed: f32,
    pub blur_radius: f32,
    /// Multiplies the glow of lit self-lit surfaces (not no-lighting
    /// effects such as light beams, whose color the barracks recording
    /// shows unchanged).
    pub emissive_mult: f32,
    pub target_lum: f32,
    pub upper_lum_clamp: f32,
    pub bright_scale: f32,
    pub bright_clamp: f32,
    /// Float 2 (published editors call it "blur passes"): what the game
    /// multiplies the directional light by when it lights skin (faces,
    /// bare arms, hands: meshes with shader flag 0x400) in its separate
    /// directional-light pass (`SLS1002.pso`'s `PSLightColor`). Read from
    /// three recordings, where it was exactly this value times the cell's
    /// directional colour: the Prospector Saloon (`ShackInteriorWaste01`,
    /// 2), the Mojave Outpost barracks (`OfficeDefaultImageSpace`, 2) and
    /// Doc Mitchell's house (`ShackInterior01`, 4). Other surfaces in the
    /// same frames got the plain colour. The code that sets it isn't traced.
    pub skin_directional: f32,
}

impl ImageSpace {
    /// The HDR values (see [`Hdr`]), if `DNAM` holds them.
    pub fn hdr(&self) -> Option<Hdr> {
        let v = self.values.get(0..8)?;
        let hdr = Hdr {
            eye_adapt_speed: v[0],
            blur_radius: v[1],
            emissive_mult: v[3],
            target_lum: v[4],
            upper_lum_clamp: v[5],
            bright_scale: v[6],
            bright_clamp: v[7],
            skin_directional: v[2],
        };
        let sane = [
            hdr.eye_adapt_speed,
            hdr.blur_radius,
            hdr.emissive_mult,
            hdr.target_lum,
            hdr.upper_lum_clamp,
            hdr.bright_scale,
            hdr.bright_clamp,
            hdr.skin_directional,
        ]
        .iter()
        .all(|x| x.is_finite() && (0.0..=1000.0).contains(x));
        sane.then_some(hdr)
    }

    /// Float 11, the sunlight dimmer: outdoors (with HDR on) the sun's
    /// light in lit shaders is multiplied by it after the image space
    /// modifiers apply (the game's `00b8b440` publishes it each frame;
    /// `NVDefaultExterior` has 1.1). Point lights and ambient aren't.
    pub fn sunlight_dimmer(&self) -> Option<f32> {
        self.value(11)
    }

    /// Float 8, "LUM ramp no tex": with HDR on, the sky shaders (`SKY.pso`,
    /// `SKYTEX.pso`: the dome, clouds, sun and stars) multiply their colour
    /// by it after the image space modifiers apply, as their `Params.y`.
    /// Read from the game's code (`00b8b440` copies working-copy float 8 to
    /// the global `011ad87c`, which `PrintHDRParam` calls "Lum Ramp" and
    /// `00b89d80` hands every sky shader) and confirmed by the Goodsprings
    /// recording: `NVDefaultExterior`'s 1.1 × `NVWastelandIS`'s 0.8 arrived
    /// as `Params.y` = 0.88.
    pub fn lum_ramp_no_tex(&self) -> Option<f32> {
        self.value(8)
    }

    /// One of the HDR floats (0–13, the same place in every layout), if it
    /// looks like a real value.
    fn value(&self, index: usize) -> Option<f32> {
        self.values
            .get(index)
            .copied()
            .filter(|v| v.is_finite() && (0.0..=100.0).contains(v))
    }

    pub fn load(order: &LoadOrder, form_id: FormId) -> Option<Self> {
        let rr = order.get(form_id)?;
        if rr.entry.header.kind != IMGS {
            return None;
        }
        let record = rr.record().ok()?;
        let data = &record.get(DNAM)?.data;
        Some(Self {
            form_id,
            editor_id: record.editor_id(),
            size: data.len(),
            values: (0..data.len() / 4).map(|i| le_f32(data, i * 4)).collect(),
        })
    }

    /// The cinematic values, if `DNAM` is long enough to hold them and they
    /// look like what they should be (a misread layout gives nonsense).
    pub fn cinematic(&self) -> Option<Cinematic> {
        let start = if self.size >= LONG_SIZE {
            CINEMATIC_LONG
        } else {
            CINEMATIC
        };
        let v = self.values.get(start..start + 8)?;
        let c = Cinematic {
            saturation: v[0],
            contrast_average: v[1],
            contrast: v[2],
            brightness: v[3],
            tint: [v[4], v[5], v[6]],
            tint_amount: v[7],
        };
        let within = |x: f32, max: f32| x.is_finite() && (0.0..=max).contains(&x);
        let plausible = within(c.saturation, 5.0)
            && within(c.contrast_average, 2.0)
            && within(c.contrast, 5.0)
            && within(c.brightness, 5.0)
            && c.tint.iter().all(|&t| within(t, 1.0))
            && within(c.tint_amount, 1.0);
        plausible.then_some(c)
    }
}

/// The image space a cell record names (`XCIM`).
pub(crate) fn cell_image_space(
    order: &LoadOrder,
    plugin: &LoadedPlugin,
    record: &Record,
) -> Option<ImageSpace> {
    let s = record.get(XCIM).filter(|s| s.data.len() >= 4)?;
    let id = plugin.to_global(FormId(le_u32(&s.data, 0)));
    if id.0 == 0 {
        return None;
    }
    ImageSpace::load(order, id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skin_light_comes_from_the_third_float() {
        // `ShackInterior01`'s first floats: 0.3, 6, 4, 1 …
        let space = ImageSpace {
            form_id: FormId(1),
            editor_id: None,
            size: 132,
            values: vec![0.3, 6.0, 4.0, 1.0, 1.0, 1.0, 2.4, 0.9],
        };
        assert_eq!(space.hdr().unwrap().skin_directional, 4.0);
    }
}
