//! The trees' wind, as the game moves it every frame (`BSTreeManager`'s
//! update `006658b0`, the matrices `00bb2fc0`, the leaves' rock and rustle
//! `00bb1390`; the shader constants are set in `00bb1f60`). Four wind
//! matrices sway every branch and leaf vertex by its weight; the leaves
//! also rock and rustle about their own axes in the vertex shader.

/// The game's fast sine and cosine (`0063c600`, `0057e960`): tables of 512
/// floats filled at start (`00a813c0`: the angle starting at 0 and growing
/// by 2π/512 as a float), read at `trunc(x × 512 / 2π) & 511`.
pub struct FastTrig {
    sin: [f32; 512],
    cos: [f32; 512],
}

/// 2π as the game stores it (a float, `011ab3fc`).
const TWO_PI: f32 = 6.283_185_5;

impl FastTrig {
    pub fn new() -> FastTrig {
        let mut sin = [0.0; 512];
        let mut cos = [0.0; 512];
        let step = TWO_PI as f64 * (1.0 / 512.0);
        let mut angle = 0.0f32;
        for i in 0..512 {
            sin[i] = (angle as f64).sin() as f32;
            cos[i] = (angle as f64).cos() as f32;
            angle = (angle as f64 + step) as f32;
        }
        FastTrig { sin, cos }
    }

    fn index(x: f32) -> usize {
        let v = (512.0 / TWO_PI as f64) as f32 * x;
        ((v as i64) & 0x1ff) as usize
    }

    pub fn sin(&self, x: f32) -> f32 {
        self.sin[Self::index(x)]
    }

    pub fn cos(&self, x: f32) -> f32 {
        self.cos[Self::index(x)]
    }
}

impl Default for FastTrig {
    fn default() -> Self {
        FastTrig::new()
    }
}

/// The game settings the wind reads (`fLeafRock…`, `fLeafRustle…`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindSettings {
    /// `fLeafRockAmountSwayInfluence` (`011d5dc4`).
    pub rock_amount_sway: f32,
    /// `fLeafRustleAmountSwayInfluence` (`011d5d3c`).
    pub rustle_amount_sway: f32,
    /// `fLeafRockSpeedSwayInfluence` (`011d5c68`).
    pub rock_speed_sway: f32,
    /// `fLeafRustleSpeedSwayInfluence` (`011d5c74`).
    pub rustle_speed_sway: f32,
    /// `fLeafRockTimeScale` (`011d5cdc`).
    pub rock_time_scale: f32,
    /// `fLeafRustleTimeScale` (`011d5c5c`).
    pub rustle_time_scale: f32,
}

/// Each matrix's two rates (`0119b8cc`: sine, cosine pairs).
const RATES: [[f32; 2]; 4] = [[0.15, 0.17], [0.25, 0.15], [0.19, 0.05], [0.15, 0.22]];

/// The wind's state from frame to frame.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Wind {
    /// `011d5dd8`: each matrix's clock.
    clocks: [f32; 4],
    /// `011d5de8`: last frame's strength (`None` before the first frame).
    previous: Option<f32>,
    /// `01200608`, `0120060c`: the leaves' rock and rustle clocks.
    rock_time: f32,
    rustle_time: f32,
}

/// What the shaders get for a frame.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindFrame {
    /// `WindMatrices`: four 4 × 4 matrices, rows as the shader reads them.
    pub matrices: [[[f32; 4]; 4]; 4],
    /// The leaves' rock and rustle amounts (`RockParams.x`, `RustleParams.x`).
    pub rock_amount: f32,
    pub rustle_amount: f32,
    /// The rock and rustle clocks (times each tree's speeds for `.y`).
    pub rock_time: f32,
    pub rustle_time: f32,
}

impl Wind {
    /// One frame (`006658b0`): `strength` is the weather's wind (`DATA[0]`
    /// / 255; 0 indoors), `seconds` the frame's. For each matrix i: its
    /// clock += seconds, then × last strength / strength (when it isn't 0);
    /// a = sin(20 w × rate₀ × clock), b = cos(20 w × rate₁ × clock) by the
    /// fast tables; the matrix turns by yaw 0.61 w a and pitch 0.61 w b
    /// (`D3DXMatrixRotationYawPitchRoll`, transposed). With s = |a| + |b|
    /// of the third matrix, the leaves' amounts are w × ((1 − i) + i s / 2)
    /// for the amount influences i, and their clocks advance by time scale
    /// × seconds × ((1 − i) + i s / 2) for the speed influences.
    pub fn update(
        &mut self,
        strength: f32,
        seconds: f32,
        settings: &WindSettings,
        trig: &FastTrig,
    ) -> WindFrame {
        let previous = *self.previous.get_or_insert(strength);
        let amplitude = strength * 0.61;
        let mut sway = 0.0f32;
        let mut matrices = [[[0.0; 4]; 4]; 4];
        for (i, rate) in RATES.iter().enumerate() {
            self.clocks[i] += seconds;
            if strength != 0.0 {
                self.clocks[i] = self.clocks[i] * previous / strength;
            }
            let speed = strength * 20.0;
            let a = trig.sin(speed * rate[0] * self.clocks[i]);
            let b = trig.cos(speed * rate[1] * self.clocks[i]);
            if i == 2 {
                sway = b.abs() + (a.abs() + sway);
            }
            matrices[i] = yaw_pitch(amplitude * a, amplitude * b);
        }
        let mix =
            |influence: f32| ((1.0 - influence) as f64 + (influence * sway) as f64 / 2.0) as f32;
        self.rock_time += settings.rock_time_scale * seconds * mix(settings.rock_speed_sway);
        self.rustle_time += settings.rustle_time_scale * seconds * mix(settings.rustle_speed_sway);
        self.previous = Some(strength);
        WindFrame {
            matrices,
            rock_amount: strength * mix(settings.rock_amount_sway),
            rustle_amount: strength * mix(settings.rustle_amount_sway),
            rock_time: self.rock_time,
            rustle_time: self.rustle_time,
        }
    }
}

/// `D3DXMatrixRotationYawPitchRoll(yaw, pitch, 0)` transposed: rows
/// (cos y, sin p sin y, cos p sin y), (0, cos p, −sin p), (−sin y, sin p cos
/// y, cos p cos y), (0, 0, 0, 1).
fn yaw_pitch(yaw: f32, pitch: f32) -> [[f32; 4]; 4] {
    let (sy, cy) = yaw.sin_cos();
    let (sp, cp) = pitch.sin_cos();
    [
        [cy, sp * sy, cp * sy, 0.0],
        [0.0, cp, -sp, 0.0],
        [-sy, sp * cy, cp * cy, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settings() -> WindSettings {
        WindSettings {
            rock_amount_sway: 1.0,
            rustle_amount_sway: 1.0,
            rock_speed_sway: 1.0,
            rustle_speed_sway: 1.0,
            rock_time_scale: 2.0,
            rustle_time_scale: 0.5,
        }
    }

    #[test]
    fn the_tables_wrap() {
        let t = FastTrig::new();
        assert_eq!(t.sin(0.0), 0.0);
        assert_eq!(t.cos(0.0), 1.0);
        // A quarter turn lands on entry 128.
        assert!((t.sin(std::f32::consts::FRAC_PI_2 + 1e-4) - 1.0).abs() < 1e-4);
        // Negative angles wrap round the table.
        assert!((t.sin(-std::f32::consts::FRAC_PI_2 - 1e-3) + 1.0).abs() < 1e-3);
    }

    /// The recorded first matrix (Goodsprings, wind 50/255): yaw and pitch
    /// of a few hundredths of a radian in the D3DX layout.
    #[test]
    fn matrices_have_the_d3dx_layout() {
        let m = yaw_pitch(0.06149, 0.07813);
        assert!((m[0][0] - 0.99811).abs() < 1e-4);
        assert!((m[0][1] - 0.0048).abs() < 1e-4);
        assert!((m[0][2] - 0.06126).abs() < 1e-4);
        assert!((m[1][2] + 0.07805).abs() < 1e-4);
        assert!((m[2][0] + 0.06145).abs() < 1e-4);
    }

    #[test]
    fn rock_runs_four_times_as_fast_as_rustle() {
        let trig = FastTrig::new();
        let mut wind = Wind::default();
        let mut frame = None;
        for _ in 0..100 {
            frame = Some(wind.update(50.0 / 255.0, 0.05, &settings(), &trig));
        }
        let f = frame.unwrap();
        assert!((f.rock_time / f.rustle_time - 4.0).abs() < 1e-3);
        assert_eq!(f.rock_amount, f.rustle_amount);
        // No wind: no sway, matrices plain.
        let still = Wind::default().update(0.0, 0.05, &settings(), &trig);
        assert_eq!(still.matrices[0][0], [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(still.rock_amount, 0.0);
    }
}
