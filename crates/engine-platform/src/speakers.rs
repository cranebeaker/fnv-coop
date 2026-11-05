//! The channel layout a DirectSound device mixes in, and where each kind of buffer goes on it, as
//! the player's 32-bit `dsound.dll` does it (`SysWOW64`, 10.0.26100.9549, read in a private
//! analysis with the maintainer's leave, 2026-11-02; addresses are that DLL's, image base
//! `0x10000000`), checked against what that DLL hands the Windows audio engine on this machine's
//! 8-channel layout (`scripts/dsmix-measure` `DSMIX_SURROUND`, docs/SOUND.md). No bytes of the
//! DLL are here: its speaker tables are written from the angles they hold.
//!
//! - **The layout** ([`Layout`]): the endpoint's shared-mode mix format gives the channel count
//!   and the speaker mask (`100548f0`: a `WAVEFORMATEXTENSIBLE`'s own mask, else front centre
//!   for one channel and front left and right otherwise). A device does not take it at once
//!   ([`DeviceLayout`]): on an endpoint that mixes three or more channels it mixes two (front left
//!   and right) until its first 3D buffer is made, which locks it to the mix format for good
//!   (`10052857`, `10053984`); on a one- or two-channel endpoint its first buffer locks it.
//! - **2D buffers** ([`matrix_2d`]): the DLL's default matrix from the buffer's speakers (mono:
//!   front centre; stereo: front left and right) to the device's (`10067b0e`), by speaker angle,
//!   rounded to three decimals; then each device channel's row times the total left gain, the
//!   total right gain or the volume's gain by its speaker (`10057d90`). So on a layout with a
//!   front centre a mono buffer is heard there only, at its volume, its pan not heard; a stereo
//!   buffer goes to front left and right at the left and right gains, as on two channels.
//! - **3D buffers** ([`azimuths_3d`]): the listener's speakers for the device's mask (`1002a8cb`);
//!   `super::ds3d` pans over them.
//!
//! Measured on this machine: the 8-channel layout 0x63F (as `FalloutNV.exe`) and two channels
//! (0x3), every placement above bit for bit (`DSMIX_SURROUND`, 2026-11-03, docs/SOUND.md). One
//! channel, quad, surround, both 5.1 and 7.1 wide follow the DLL's code only (this machine
//! cannot give them without a change of Windows' speaker setup).

/// A device's channels and speaker mask (`dwChannelMask`: one bit per speaker, channels in the
/// order of their bits).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Layout {
    pub channels: u16,
    pub mask: u32,
}

/// Speaker bits (`SPEAKER_*`).
pub const FRONT_LEFT: u32 = 0x1;
pub const FRONT_RIGHT: u32 = 0x2;
pub const FRONT_CENTER: u32 = 0x4;

/// The most channels a device of this layer mixes (the DLL's largest 3D layout).
pub const MAX_CHANNELS: usize = 8;

/// A buffer's place on the device: for each device channel (rows) the multiplier of each of the
/// buffer's channels (one or two columns).
pub type Matrix = [[f32; 2]; MAX_CHANNELS];

impl Default for Layout {
    fn default() -> Layout {
        Layout::STEREO
    }
}

impl Layout {
    /// Two channels, front left and right: every device on a two-channel endpoint, and a device
    /// on a larger one until its layout is locked.
    pub const STEREO: Layout = Layout {
        channels: 2,
        mask: FRONT_LEFT | FRONT_RIGHT,
    };
    /// One channel, front centre.
    pub const MONO: Layout = Layout {
        channels: 1,
        mask: FRONT_CENTER,
    };

    /// The layout of a mix format with `channels` and, for a `WAVEFORMATEXTENSIBLE`, its
    /// `extensible_mask` (`100548f0`: its own mask; else front centre for one channel, front left
    /// and right otherwise).
    pub fn of_mix(channels: u16, extensible_mask: Option<u32>) -> Layout {
        let mask = extensible_mask.unwrap_or(if channels == 1 {
            FRONT_CENTER
        } else {
            FRONT_LEFT | FRONT_RIGHT
        });
        Layout { channels, mask }
    }

    /// Whether this layer mixes for it: one of the speaker layouts the DLL places 3D buffers on
    /// ([`azimuths_3d`]), with one channel per speaker bit. Another layout makes the listener's
    /// `SetSpeakerConfig` fail in the DLL (`1002a8cb` answers `E_FAIL`), which this layer does not
    /// provide.
    pub fn provided(self) -> bool {
        (self == Layout::MONO || azimuths_3d(self.mask).is_some())
            && self.mask.count_ones() == u32::from(self.channels)
    }

    /// The speaker bit of each channel, in channel order.
    pub fn speakers(self) -> impl Iterator<Item = u32> {
        let mask = self.mask;
        (0..32).map(|b| 1u32 << b).filter(move |b| mask & b != 0)
    }

    /// The channel that carries `speaker`, if the layout has it.
    pub fn channel_of(self, speaker: u32) -> Option<usize> {
        self.speakers().position(|s| s == speaker)
    }
}

/// `IDirectSound8::GetSpeakerConfig`'s answer for a device on an endpoint whose mix format has
/// `mask` (`10053610`: from the mix format, whether or not the device is locked): measured
/// `DSSPEAKER_7POINT1_SURROUND` (8) for 0x63F (a program named `FalloutNV.exe` on this machine)
/// and `DSSPEAKER_COMBINED(DSSPEAKER_STEREO, DSSPEAKER_GEOMETRY_WIDE)` (0x140004) for 0x3 (any
/// other name), the rest as the DLL has them: a mask of 0 (direct out) answers `DS_OK` with 0;
/// `None` (`E_FAIL`, nothing written but 0) for a mask it does not know.
pub fn speaker_config(mask: u32) -> Option<u32> {
    Some(match mask {
        0x0 => 0,
        0x3 => 0x14_0004,
        0x4 => 2,
        0x33 => 3,
        0x3f => 6,
        0xff => 7,
        0x107 => 5,
        0x60f => 9,
        0x63f => 8,
        _ => return None,
    })
}

/// The speaker azimuths (radians, the `f64` of an `f32`) the listener pans 3D buffers over for a
/// device mask (`1002a8cb`, the tables at `10007610`..`10007730`), in channel order. The
/// low-frequency speaker has 42, which is never one of the two nearest. `None` for a mask the
/// DLL refuses (`E_FAIL`); front centre alone (one channel) has no table (pan 0, `1002a4c0`).
pub fn azimuths_3d(mask: u32) -> Option<&'static [f64]> {
    // The DLL's values: the nearest `f32` of each angle, widened.
    const Q: f64 = std::f32::consts::FRAC_PI_4 as f64;
    const H: f64 = std::f32::consts::FRAC_PI_2 as f64;
    const E: f64 = std::f32::consts::FRAC_PI_8 as f64;
    const T: f64 = (3.0 * std::f64::consts::FRAC_PI_4) as f32 as f64;
    const P: f64 = std::f32::consts::PI as f64;
    const LFE: f64 = 42.0;
    Some(match mask {
        0x3 => &[-H, H],
        0x33 => &[-Q, Q, -T, T],
        0x107 => &[-Q, Q, 0.0, P],
        0x3f => &[-Q, Q, 0.0, LFE, -T, T],
        0x60f => &[-Q, Q, 0.0, LFE, -H, H],
        0xff => &[-Q, Q, 0.0, LFE, -T, T, -E, E],
        0x63f => &[-Q, Q, 0.0, LFE, -T, T, -H, H],
        _ => return None,
    })
}

/// The speaker angles (degrees) of the default matrix (`10067b0e`), for the first 11 speaker
/// bits: front left, right, centre, low frequency (none: -360), back left, right, front left and
/// right of centre, back centre, side left, right.
const ANGLES: [f32; 11] = [
    330.0, 30.0, 0.0, -360.0, 225.0, 135.0, 345.0, 15.0, 180.0, 250.0, 110.0,
];

/// Why the default matrix cannot be made (the DLL's errors).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatrixError {
    /// `E_INVALIDARG`: more speakers in a mask than channels, or two speakers at one angle.
    InvalidArg,
    /// `E_NOTIMPL`: a speaker without an angle (other than low frequency) in one layout only.
    NotImpl,
}

/// The DLL's default matrix (`10067b0e`, `100675d2`, `100676b6`) from `inputs` channels with
/// speakers `in_mask` to the device `out`, as `m[device channel][input channel]`.
///
/// Each layout's speakers get their angles (only the first 11 bits have one; channels past the
/// mask's bits and the low-frequency speaker have none, -360). If one layout has back speakers
/// without side ones and the other side ones without back ones, the side ones take the back
/// angles. Both lists are sorted by angle (a bubble sort, so equal angles keep their order).
/// Each input with an angle goes between the two device speakers around it by the sine law:
/// with `e` its angle past the lower one and `span` the angle between them (both taken modulo
/// 360 in `f32`), `a = e / span * 3.1415926535 * 0.5`, the lower speaker gets `cos a` and the
/// upper `sin a` (as `f32`, at least 0; both 1 on a one-channel device), or 1 when a single
/// device speaker has an angle. The low-frequency speaker in both layouts is 1 on its own; one
/// the device lacks is shared out. Then every entry is divided by the largest device speaker's
/// sum and rounded to three decimals, all in `f32`.
pub fn default_matrix(inputs: u16, in_mask: u32, out: Layout) -> Result<Matrix, MatrixError> {
    let (n_in, n_out) = (usize::from(inputs), usize::from(out.channels));
    assert!(
        n_in <= 2 && n_out <= MAX_CHANNELS,
        "a matrix of {n_in} by {n_out} is not provided"
    );
    let mut angles = ANGLES;
    // `100675d2`: back speakers in one layout, side ones in the other.
    for (back, side) in [(0x10u32, 0x200u32), (0x20, 0x400)] {
        let pair = back | side;
        if (in_mask & pair == back && out.mask & pair == side)
            || (out.mask & pair == back && in_mask & pair == side)
        {
            angles[side.trailing_zeros() as usize] = angles[back.trailing_zeros() as usize];
        }
    }
    let place = |mask: u32, n: usize| -> Result<Vec<f32>, MatrixError> {
        let mut list = Vec::with_capacity(n);
        for (i, &a) in angles.iter().enumerate() {
            if mask & (1 << i) != 0 {
                if list.len() == n {
                    return Err(MatrixError::InvalidArg);
                }
                list.push(a);
            }
        }
        list.resize(n, -360.0);
        Ok(list)
    };
    let in_angles = place(in_mask, n_in)?;
    let out_angles = place(out.mask, n_out)?;
    let mut m = positioned(&in_angles, &out_angles)?;
    // `10067b0e`: the speakers without an angle.
    let (mut input, mut row) = (0usize, 0usize);
    for (i, &a) in angles.iter().enumerate() {
        let bit = 1u32 << i;
        let (has_in, has_out) = (in_mask & bit != 0, out.mask & bit != 0);
        if a < 0.0 {
            if has_in && has_out {
                m[row][input] = 1.0;
            } else if (has_in || has_out) && bit != 0x8 {
                return Err(MatrixError::NotImpl);
            } else if has_in {
                // An input low-frequency channel the device lacks.
                let f = 1.0f32 / (n_in as f32 - 1.0);
                let outs = n_out as f32;
                for r in m.iter_mut().take(n_out) {
                    for (k, v) in r.iter_mut().enumerate().take(n_in) {
                        let x = if k == input { f } else { outs * *v };
                        *v = x / (outs + f);
                    }
                }
            }
        }
        if has_in {
            input += 1;
        }
        if has_out {
            row += 1;
        }
    }
    Ok(m)
}

/// `100676b6`: the inputs with an angle placed between the device's speakers, then the whole
/// matrix scaled and rounded.
fn positioned(in_angles: &[f32], out_angles: &[f32]) -> Result<Matrix, MatrixError> {
    let (n_in, n_out) = (in_angles.len(), out_angles.len());
    // `10067569`: bubble sort, ascending, moving an entry on only when the next is smaller.
    let sorted = |angles: &[f32]| {
        let mut a = angles.to_vec();
        let mut index: Vec<usize> = (0..a.len()).collect();
        loop {
            let mut swapped = false;
            for k in 0..a.len().saturating_sub(1) {
                if a[k + 1] < a[k] {
                    a.swap(k, k + 1);
                    index.swap(k, k + 1);
                    swapped = true;
                }
            }
            if !swapped {
                return (a, index);
            }
        }
    };
    let (ia, ii) = sorted(in_angles);
    let (oa, oi) = sorted(out_angles);
    let first_in = ia.iter().position(|&a| a >= 0.0).unwrap_or(n_in);
    let first_out = oa.iter().position(|&a| a >= 0.0).unwrap_or(n_out);
    let count = n_out - first_out;
    let mut m: Matrix = [[0.0; 2]; MAX_CHANNELS];
    if count == 0 {
        return Ok(m);
    }
    let mut sums = [0f32; MAX_CHANNELS];
    let turn = |x: f32| x - ((f64::from(x / 360.0)).floor() as f32) * 360.0;
    for (&angle, &input) in ia.iter().zip(ii.iter()).skip(first_in) {
        let mut j = first_out;
        while j < n_out && oa[j] < angle {
            j += 1;
        }
        let hi = (j - first_out) % count + first_out;
        let lo = (count - 1 + (j - first_out)) % count + first_out;
        let span = turn(oa[hi] - oa[lo]);
        let e = angle - oa[lo];
        if lo == hi {
            m[oi[lo]][input] = 1.0;
            sums[oi[lo]] += 1.0;
            continue;
        }
        if span == 0.0 {
            return Err(MatrixError::InvalidArg);
        }
        let a = f64::from(turn(e) / span) * PI_10 * 0.5;
        let (mut low, mut high) = ((a.cos() as f32).max(0.0), (a.sin() as f32).max(0.0));
        if n_out == 1 {
            (low, high) = (1.0, 1.0);
        }
        m[oi[lo]][input] = low;
        m[oi[hi]][input] = high;
        sums[oi[lo]] += low;
        sums[oi[hi]] += high;
    }
    let mut most = 0f32;
    for &s in sums.iter().take(n_out) {
        if s > most {
            most = s;
        }
    }
    if most != 0.0 {
        for r in m.iter_mut().take(n_out) {
            for v in r.iter_mut().take(n_in) {
                *v = ((*v / most) * 1000.0 + 0.5) as i32 as f32 / 1000.0;
            }
        }
    }
    Ok(m)
}

/// The DLL's pi, cut to ten decimals (`100112f0`).
#[allow(clippy::approx_constant)]
const PI_10: f64 = 3.141_592_653_5;

/// Which of a buffer's gains a device speaker's row is multiplied by (`10057d90`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Side {
    Left,
    Right,
    Volume,
    One,
}

fn side(speaker: u32) -> Side {
    match speaker {
        0x1 | 0x10 | 0x40 | 0x200 | 0x1000 | 0x8000 => Side::Left,
        0x2 | 0x20 | 0x80 | 0x400 | 0x4000 | 0x2_0000 => Side::Right,
        0x4 | 0x8 | 0x100 | 0x800 | 0x2000 | 0x1_0000 => Side::Volume,
        _ => Side::One,
    }
}

/// A 2D buffer's matrix on the device (`10057d90`): the default matrix for its channels (`mono`
/// or `stereo`, made once per layout with [`default_matrix`]) with each device channel's row
/// multiplied, in `f32`, by the buffer's total left gain, total right gain or volume's gain by
/// that channel's speaker (`super::mixer::channel_gains`, `super::mixer::gain`), or by 1.
pub fn matrix_2d(default: &Matrix, out: Layout, volume: i32, pan: i32) -> Matrix {
    let (left, right) = super::mixer::channel_gains(volume, pan);
    let level = super::mixer::gain(volume.max(-10000));
    let mut m = *default;
    for (row, speaker) in m.iter_mut().zip(out.speakers()) {
        let g = match side(speaker) {
            Side::Left => left,
            Side::Right => right,
            Side::Volume => level,
            Side::One => 1.0,
        };
        for v in row.iter_mut() {
            *v = g * *v;
        }
    }
    m
}

/// How a DirectSound device's layout follows the buffers made on it (`10052857`, called as a
/// PCM buffer is made, before it joins the device's list; `10053984` locks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceLayout {
    /// The endpoint's mix format.
    pub mix: Layout,
    /// What the device mixes now (none before its first buffer).
    pub now: Option<Layout>,
    /// Locked to the mix format, for good.
    pub locked: bool,
}

impl DeviceLayout {
    pub fn new(mix: Layout) -> DeviceLayout {
        DeviceLayout {
            mix,
            now: None,
            locked: false,
        }
    }

    /// A PCM buffer in `buffer`'s layout is made (`three_d`: `DSBCAPS_CTRL3D`); answers whether
    /// this locked the device (`10052857`):
    ///
    /// | The device | The endpoint | The buffer | Then |
    /// | --- | --- | --- | --- |
    /// | no buffer yet | 1 or 2 channels | 1 or 2 channels, or 3D | locked |
    /// | no buffer yet | 1 or 2 channels | more, 2D | the buffer's layout, not locked |
    /// | no buffer yet | 3 or more | 1 or 2 channels, 2D | two channels, not locked |
    /// | no buffer yet | 3 or more | more, or 3D | locked |
    /// | not locked | 1 or 2 channels | 1 or 2 channels, or 3D | locked |
    /// | not locked | 3 or more | more, or 3D | locked |
    /// | otherwise | | | no change |
    ///
    /// Measured as `FalloutNV.exe` on the 8-channel endpoint (2026-11-03, `DSMIX_SURROUND`): with
    /// 2D mono and stereo buffers only DirectSound asks Windows for a two-channel stream (front
    /// left and right, `IsFormatSupported` `S_OK`) and mixes two channels; the first 3D buffer,
    /// made, not played, gives an 8-channel stream. (The game's buffers are mono or stereo; the
    /// rows of larger buffers are the DLL's only.)
    pub fn made(&mut self, buffer: Layout, three_d: bool) -> bool {
        let small = buffer.channels <= 2;
        let lock = match (self.now, self.locked, self.mix.channels <= 2) {
            (_, true, _) => false,
            (None, _, true) if !small && !three_d => {
                self.now = Some(buffer);
                false
            }
            (None, _, false) if small && !three_d => {
                self.now = Some(Layout::STEREO);
                false
            }
            (None, _, _) => true,
            (Some(_), false, true) => small || three_d,
            (Some(_), false, false) => !small || three_d,
        };
        if lock {
            self.lock();
        }
        lock
    }

    /// Locked to the endpoint's mix format, for good (`10053984`; also when the endpoint does not
    /// take the two-channel stream, `10051b41`, read only).
    pub fn lock(&mut self) {
        self.now = Some(self.mix);
        self.locked = true;
    }

    /// What the device mixes: two channels until a buffer locks it (or the mix format's own
    /// on a one- or two-channel endpoint).
    pub fn current(&self) -> Layout {
        self.now.unwrap_or(if self.mix.channels <= 2 {
            self.mix
        } else {
            Layout::STEREO
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LAYOUTS: [u32; 7] = [0x3, 0x33, 0x107, 0x3f, 0x60f, 0xff, 0x63f];

    fn layout(mask: u32) -> Layout {
        Layout {
            channels: mask.count_ones() as u16,
            mask,
        }
    }

    /// Mono and stereo buffers on every layout the DLL places 3D buffers on: mono goes to front
    /// centre where there is one (else front left and right in full), stereo to front left and
    /// right; on one channel stereo is halved into it.
    #[test]
    fn mono_and_stereo_default_matrices() {
        for mask in LAYOUTS {
            let out = layout(mask);
            let mono = default_matrix(1, FRONT_CENTER, out).unwrap();
            let stereo = default_matrix(2, FRONT_LEFT | FRONT_RIGHT, out).unwrap();
            for (o, speaker) in out.speakers().enumerate() {
                let centre = mask & FRONT_CENTER != 0;
                let want_mono = match speaker {
                    0x4 => 1.0,
                    0x1 | 0x2 if !centre => 1.0,
                    _ => 0.0,
                };
                assert_eq!(mono[o][0], want_mono, "mono on {mask:#x}, channel {o}");
                let want = match speaker {
                    0x1 => [1.0, 0.0],
                    0x2 => [0.0, 1.0],
                    _ => [0.0, 0.0],
                };
                assert_eq!(stereo[o], want, "stereo on {mask:#x}, channel {o}");
            }
        }
        let mono = default_matrix(1, FRONT_CENTER, Layout::MONO).unwrap();
        assert_eq!(mono[0][0], 1.0);
        let stereo = default_matrix(2, FRONT_LEFT | FRONT_RIGHT, Layout::MONO).unwrap();
        assert_eq!(stereo[0], [0.5, 0.5]);
    }

    /// On two channels the 2D matrix is the stereo law the mixer has (`mixer::channel_gains`).
    #[test]
    fn the_two_channel_matrix_is_the_stereo_law() {
        let d = default_matrix(1, FRONT_CENTER, Layout::STEREO).unwrap();
        for (volume, pan) in [(0, 0), (-600, 0), (0, -1000), (-300, -700), (-100, 2500)] {
            let m = matrix_2d(&d, Layout::STEREO, volume, pan);
            let (l, r) = super::super::mixer::channel_gains(volume, pan);
            assert_eq!((m[0][0], m[1][0]), (l, r));
        }
    }

    /// On 7.1 a mono 2D buffer is heard at its volume from the centre only, whatever its pan.
    #[test]
    fn a_mono_2d_buffer_on_7_1_is_centre_only() {
        let out = layout(0x63f);
        let d = default_matrix(1, FRONT_CENTER, out).unwrap();
        let m = matrix_2d(&d, out, -600, -3000);
        let g = super::super::mixer::gain(-600);
        for (o, row) in m.iter().enumerate().take(8) {
            assert_eq!(row[0], if o == 2 { g } else { 0.0 }, "channel {o}");
        }
    }

    #[test]
    fn the_3d_tables_hold_the_dll_angles() {
        let t = azimuths_3d(0x63f).unwrap();
        let bits: Vec<u64> = t.iter().map(|a| a.to_bits()).collect();
        assert_eq!(
            bits,
            [
                0xbfe9_21fb_6000_0000,
                0x3fe9_21fb_6000_0000,
                0,
                0x4045_0000_0000_0000,
                0xc002_d97c_8000_0000,
                0x4002_d97c_8000_0000,
                0xbff9_21fb_6000_0000,
                0x3ff9_21fb_6000_0000,
            ]
        );
        assert_eq!(
            azimuths_3d(0xff).unwrap()[7].to_bits(),
            0x3fd9_21fb_6000_0000
        );
        assert_eq!(
            azimuths_3d(0x107).unwrap()[3].to_bits(),
            0x4009_21fb_6000_0000
        );
        for mask in LAYOUTS {
            assert_eq!(azimuths_3d(mask).unwrap().len(), mask.count_ones() as usize);
            assert!(layout(mask).provided());
        }
        assert!(Layout::MONO.provided());
        assert!(!layout(0x7).provided());
        assert!(!Layout {
            channels: 8,
            mask: 0x3f
        }
        .provided());
    }

    /// Measured (2026-11-03): an 8-channel endpoint (0x63F) answers 8, a two-channel one 0x140004.
    /// Read (`10053610`): a mask of 0 (direct out) answers `DS_OK` with 0; one it does not know
    /// `E_FAIL`.
    #[test]
    fn speaker_configs() {
        assert_eq!(speaker_config(0x63f), Some(8));
        assert_eq!(speaker_config(0x3), Some(0x14_0004));
        assert_eq!(speaker_config(0x0), Some(0));
        assert_eq!(speaker_config(0x7), None);
    }

    /// The lock: two channels after 2D buffers on an 8-channel endpoint, its own format from the
    /// first 3D buffer on; on a two-channel endpoint the first buffer locks.
    #[test]
    fn the_device_locks_at_its_first_3d_buffer() {
        let mono = Layout::of_mix(1, None);
        let stereo = Layout::STEREO;
        let mut d = DeviceLayout::new(layout(0x63f));
        assert_eq!(d.current(), Layout::STEREO);
        assert!(!d.made(mono, false));
        assert!(!d.made(stereo, false));
        assert_eq!(d.current(), Layout::STEREO);
        assert!(d.made(mono, true));
        assert_eq!(d.current(), layout(0x63f));
        assert!(!d.made(mono, true));
        assert!(!d.made(mono, false));
        // A 3D buffer first locks at once.
        let mut d = DeviceLayout::new(layout(0x3f));
        assert!(d.made(mono, true));
        assert_eq!(d.current(), layout(0x3f));
        // On one or two channels the first buffer locks (the layout stays the endpoint's).
        let mut d = DeviceLayout::new(Layout::STEREO);
        assert!(d.made(mono, false));
        assert_eq!(d.current(), Layout::STEREO);
        let mut d = DeviceLayout::new(Layout::MONO);
        assert_eq!(d.current(), Layout::MONO);
        assert!(d.made(stereo, false));
        assert_eq!(d.current(), Layout::MONO);
        // The DLL's rows for buffers of more than two channels (the game makes none).
        let mut d = DeviceLayout::new(Layout::STEREO);
        assert!(!d.made(layout(0x3f), false));
        assert_eq!(d.current(), layout(0x3f));
        assert!(!d.made(layout(0x3f), false));
        assert!(d.made(mono, false));
        assert_eq!(d.current(), Layout::STEREO);
        let mut d = DeviceLayout::new(layout(0x63f));
        assert!(!d.made(stereo, false));
        assert!(d.made(layout(0x3f), false));
        assert_eq!(d.current(), layout(0x63f));
    }
}
