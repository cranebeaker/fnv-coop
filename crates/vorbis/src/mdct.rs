// Follows libvorbis 1.2.0 (lib/mdct.c), Copyright (c) 2002-2004 Xiph.org Foundation, BSD
// licence: see ../LICENSE-libvorbis.
//! The inverse MDCT: libvorbis' split-radix butterflies, statement for statement, with the
//! arithmetic of the decoding thread's x87 precision (`fpu`). The trigonometric tables are
//! built as the library builds them: the angle `(pi / n) * k` at that precision, its cosine or
//! sine rounded to `f32`.

use crate::fpu::{store, Fpu};

/// `mdct_lookup` for one block size.
#[derive(Debug, Clone)]
pub struct Mdct {
    n: usize,
    log2n: u32,
    trig: Vec<f32>,
    bitrev: Vec<usize>,
    /// For measuring: the arithmetic of each stage (rotation in, the first and generic
    /// butterflies, the 32-point butterflies, the bit reversal, rotation out).
    pub(crate) stages: [Fpu; 5],
}

macro_rules! v {
    ($x:expr, $i:expr) => {
        f64::from($x[$i])
    };
}

impl Mdct {
    /// `mdct_init`.
    pub fn new(n: usize, fpu: Fpu) -> Mdct {
        let log2n = n.trailing_zeros();
        let n2 = n >> 1;
        let mut t = vec![0f32; n + n / 4];
        // The library's `M_PI` is a `float` (3.14159274...), read as such by the x87 unit.
        let pi = f64::from(std::f32::consts::PI);
        let nf = n as f64;
        let t24 = fpu.trig_24();
        let r = |x: f64| if t24 { f64::from(x as f32) } else { x };
        let a = r(pi / nf);
        let b = r(pi / (2.0 * nf));
        for i in 0..n / 4 {
            let x = r(a * (4 * i) as f64);
            t[i * 2] = x.cos() as f32;
            t[i * 2 + 1] = (-x.sin()) as f32;
            let y = r(b * (2 * i + 1) as f64);
            t[n2 + i * 2] = y.cos() as f32;
            t[n2 + i * 2 + 1] = y.sin() as f32;
        }
        for i in 0..n / 8 {
            let x = r(a * (4 * i + 2) as f64);
            t[n + i * 2] = r(x.cos() * 0.5) as f32;
            t[n + i * 2 + 1] = r(-x.sin() * 0.5) as f32;
        }
        let mask = (1i64 << (log2n - 1)) - 1;
        let msb = 1i64 << (log2n - 2);
        let mut bitrev = vec![0usize; n / 4];
        for i in 0..n / 8 {
            let mut acc = 0i64;
            let mut j = 0;
            while (msb >> j) != 0 {
                if (msb >> j) & i as i64 != 0 {
                    acc |= 1 << j;
                }
                j += 1;
            }
            bitrev[i * 2] = ((!acc & mask) - 1) as usize;
            bitrev[i * 2 + 1] = acc as usize;
        }
        Mdct {
            n,
            log2n,
            trig: t,
            bitrev,
            stages: [fpu; 5],
        }
    }

    /// `mdct_backward(init, x, x)`: the first `n / 2` values in, `n` samples out.
    pub fn backward(&self, x: &mut [f32]) {
        let f = self.stages[0];
        let n = self.n;
        let n2 = n >> 1;
        let n4 = n >> 2;
        let t = &self.trig;
        // `a * b - c * d` and `a * b + c * d` as one C expression.
        let ms =
            |a: f64, b: f32, c: f64, d: f32| f.sub(f.mul(a, f64::from(b)), f.mul(c, f64::from(d)));
        let ma =
            |a: f64, b: f32, c: f64, d: f32| f.add(f.mul(a, f64::from(b)), f.mul(c, f64::from(d)));

        // Rotate.
        let mut ix = n2 as isize - 7;
        let mut ox = n2 + n4;
        let mut ti = n4;
        loop {
            ox -= 4;
            let i = ix as usize;
            x[ox] = store(ms(-v!(x, i + 2), t[ti + 3], v!(x, i), t[ti + 2]));
            x[ox + 1] = store(ms(v!(x, i), t[ti + 3], v!(x, i + 2), t[ti + 2]));
            x[ox + 2] = store(ms(-v!(x, i + 6), t[ti + 1], v!(x, i + 4), t[ti]));
            x[ox + 3] = store(ms(v!(x, i + 4), t[ti + 1], v!(x, i + 6), t[ti]));
            ix -= 8;
            ti += 4;
            if ix < 0 {
                break;
            }
        }
        let mut ix = n2 as isize - 8;
        let mut ox = n2 + n4;
        let mut ti = n4;
        loop {
            ti -= 4;
            let i = ix as usize;
            x[ox] = store(ma(v!(x, i + 4), t[ti + 3], v!(x, i + 6), t[ti + 2]));
            x[ox + 1] = store(ms(v!(x, i + 4), t[ti + 2], v!(x, i + 6), t[ti + 3]));
            x[ox + 2] = store(ma(v!(x, i), t[ti + 1], v!(x, i + 2), t[ti]));
            x[ox + 3] = store(ms(v!(x, i), t[ti], v!(x, i + 2), t[ti + 1]));
            ix -= 8;
            ox += 4;
            if ix < 0 {
                break;
            }
        }

        self.butterflies(&mut x[n2..n], n2);
        self.bitreverse(x);

        let f = self.stages[4];
        let ms =
            |a: f64, b: f32, c: f64, d: f32| f.sub(f.mul(a, f64::from(b)), f.mul(c, f64::from(d)));
        let ma =
            |a: f64, b: f32, c: f64, d: f32| f.add(f.mul(a, f64::from(b)), f.mul(c, f64::from(d)));
        // Rotate and window.
        let mut ox1 = n2 + n4;
        let mut ox2 = n2 + n4;
        let mut ix = 0usize;
        let mut ti = n2;
        loop {
            ox1 -= 4;
            x[ox1 + 3] = store(ms(v!(x, ix), t[ti + 1], v!(x, ix + 1), t[ti]));
            x[ox2] = store(-ma(v!(x, ix), t[ti], v!(x, ix + 1), t[ti + 1]));
            x[ox1 + 2] = store(ms(v!(x, ix + 2), t[ti + 3], v!(x, ix + 3), t[ti + 2]));
            x[ox2 + 1] = store(-ma(v!(x, ix + 2), t[ti + 2], v!(x, ix + 3), t[ti + 3]));
            x[ox1 + 1] = store(ms(v!(x, ix + 4), t[ti + 5], v!(x, ix + 5), t[ti + 4]));
            x[ox2 + 2] = store(-ma(v!(x, ix + 4), t[ti + 4], v!(x, ix + 5), t[ti + 5]));
            x[ox1] = store(ms(v!(x, ix + 6), t[ti + 7], v!(x, ix + 7), t[ti + 6]));
            x[ox2 + 3] = store(-ma(v!(x, ix + 6), t[ti + 6], v!(x, ix + 7), t[ti + 7]));
            ox2 += 4;
            ix += 8;
            ti += 8;
            if ix >= ox1 {
                break;
            }
        }

        let mut ix = n2 + n4;
        let mut ox1 = n4;
        let mut ox2 = ox1;
        loop {
            ox1 -= 4;
            ix -= 4;
            x[ox1 + 3] = x[ix + 3];
            x[ox2] = -x[ox1 + 3];
            x[ox1 + 2] = x[ix + 2];
            x[ox2 + 1] = -x[ox1 + 2];
            x[ox1 + 1] = x[ix + 1];
            x[ox2 + 2] = -x[ox1 + 1];
            x[ox1] = x[ix];
            x[ox2 + 3] = -x[ox1];
            ox2 += 4;
            if ox2 >= ix {
                break;
            }
        }

        let mut ix = n2 + n4;
        let mut ox1 = n2 + n4;
        let ox2 = n2;
        loop {
            ox1 -= 4;
            x[ox1] = x[ix + 3];
            x[ox1 + 1] = x[ix + 2];
            x[ox1 + 2] = x[ix + 1];
            x[ox1 + 3] = x[ix];
            ix += 4;
            if ox1 <= ox2 {
                break;
            }
        }
    }

    fn butterflies(&self, x: &mut [f32], points: usize) {
        let f = self.stages[1];
        let t = &self.trig;
        let mut stages = self.log2n as i32 - 5;
        stages -= 1;
        if stages > 0 {
            butterfly_first(f, t, x, points);
        }
        let mut i = 1;
        loop {
            stages -= 1;
            if stages <= 0 {
                break;
            }
            for j in 0..(1usize << i) {
                let p = points >> i;
                butterfly_generic(f, t, &mut x[p * j..], p, 4 << i);
            }
            i += 1;
        }
        let mut j = 0;
        while j < points {
            butterfly_32(self.stages[2], &mut x[j..j + 32]);
            j += 32;
        }
    }

    fn bitreverse(&self, x: &mut [f32]) {
        let f = self.stages[3];
        let n = self.n;
        let half = n >> 1;
        let t = &self.trig;
        let mut bit = 0usize;
        let mut w0 = 0usize;
        let mut w1 = half;
        let mut ti = n;
        let tt = |i: usize| f64::from(t[i]);
        loop {
            for (k, wa, wb) in [(0usize, 0usize, 2usize), (2, 2, 0)] {
                let x0 = half + self.bitrev[bit + k];
                let x1 = half + self.bitrev[bit + k + 1];
                let r0 = f.local(f.sub(v!(x, x0 + 1), v!(x, x1 + 1)));
                let r1 = f.local(f.add(v!(x, x0), v!(x, x1)));
                let (ta, tb) = (tt(ti + k), tt(ti + k + 1));
                let r2 = f.local(f.add(f.mul(r1, ta), f.mul(r0, tb)));
                let r3 = f.local(f.sub(f.mul(r1, tb), f.mul(r0, ta)));
                if k == 0 {
                    w1 -= 4;
                }
                let r0 = f.local(f.mul(f.add(v!(x, x0 + 1), v!(x, x1 + 1)), 0.5));
                let r1 = f.local(f.mul(f.sub(v!(x, x0), v!(x, x1)), 0.5));
                x[w0 + wa] = store(f.add(r0, r2));
                x[w1 + wb] = store(f.sub(r0, r2));
                x[w0 + wa + 1] = store(f.add(r1, r3));
                x[w1 + wb + 1] = store(f.sub(r3, r1));
            }
            ti += 4;
            bit += 4;
            w0 += 4;
            if w0 >= w1 {
                break;
            }
        }
    }
}

#[allow(clippy::excessive_precision)]
const PI3_8: f32 = 0.38268343236508977175;
#[allow(clippy::excessive_precision, clippy::approx_constant)]
const PI2_8: f32 = 0.70710678118654752441;
#[allow(clippy::excessive_precision)]
const PI1_8: f32 = 0.92387953251128675613;

fn butterfly_8(f: Fpu, x: &mut [f32]) {
    let r0 = f.local(f.add(v!(x, 6), v!(x, 2)));
    let r1 = f.local(f.sub(v!(x, 6), v!(x, 2)));
    let r2 = f.local(f.add(v!(x, 4), v!(x, 0)));
    let r3 = f.local(f.sub(v!(x, 4), v!(x, 0)));
    x[6] = store(f.add(r0, r2));
    x[4] = store(f.sub(r0, r2));
    let r0 = f.local(f.sub(v!(x, 5), v!(x, 1)));
    let r2 = f.local(f.sub(v!(x, 7), v!(x, 3)));
    x[0] = store(f.add(r1, r0));
    x[2] = store(f.sub(r1, r0));
    let r0 = f.local(f.add(v!(x, 5), v!(x, 1)));
    let r1 = f.local(f.add(v!(x, 7), v!(x, 3)));
    x[3] = store(f.add(r2, r3));
    x[1] = store(f.sub(r2, r3));
    x[7] = store(f.add(r1, r0));
    x[5] = store(f.sub(r1, r0));
}

/// `x[a] += x[b]` with the operands as `f32`.
fn add_to(f: Fpu, x: &mut [f32], a: usize, b: usize) {
    x[a] = store(f.add(v!(x, a), v!(x, b)));
}

fn butterfly_16(f: Fpu, x: &mut [f32]) {
    let c2 = f64::from(PI2_8);
    let r0 = f.local(f.sub(v!(x, 1), v!(x, 9)));
    let r1 = f.local(f.sub(v!(x, 0), v!(x, 8)));
    add_to(f, x, 8, 0);
    add_to(f, x, 9, 1);
    x[0] = store(f.mul(f.add(r0, r1), c2));
    x[1] = store(f.mul(f.sub(r0, r1), c2));

    let r0 = f.local(f.sub(v!(x, 3), v!(x, 11)));
    let r1 = f.local(f.sub(v!(x, 10), v!(x, 2)));
    add_to(f, x, 10, 2);
    add_to(f, x, 11, 3);
    x[2] = store(r0);
    x[3] = store(r1);

    let r0 = f.local(f.sub(v!(x, 12), v!(x, 4)));
    let r1 = f.local(f.sub(v!(x, 13), v!(x, 5)));
    add_to(f, x, 12, 4);
    add_to(f, x, 13, 5);
    x[4] = store(f.mul(f.sub(r0, r1), c2));
    x[5] = store(f.mul(f.add(r0, r1), c2));

    let r0 = f.local(f.sub(v!(x, 14), v!(x, 6)));
    let r1 = f.local(f.sub(v!(x, 15), v!(x, 7)));
    add_to(f, x, 14, 6);
    add_to(f, x, 15, 7);
    x[6] = store(r0);
    x[7] = store(r1);

    butterfly_8(f, &mut x[..8]);
    butterfly_8(f, &mut x[8..16]);
}

fn butterfly_32(f: Fpu, x: &mut [f32]) {
    let (c1, c2, c3) = (f64::from(PI1_8), f64::from(PI2_8), f64::from(PI3_8));
    let ms = |a: f64, b: f64, c: f64, d: f64| f.sub(f.mul(a, b), f.mul(c, d));
    let ma = |a: f64, b: f64, c: f64, d: f64| f.add(f.mul(a, b), f.mul(c, d));

    let r0 = f.local(f.sub(v!(x, 30), v!(x, 14)));
    let r1 = f.local(f.sub(v!(x, 31), v!(x, 15)));
    add_to(f, x, 30, 14);
    add_to(f, x, 31, 15);
    x[14] = store(r0);
    x[15] = store(r1);

    let r0 = f.local(f.sub(v!(x, 28), v!(x, 12)));
    let r1 = f.local(f.sub(v!(x, 29), v!(x, 13)));
    add_to(f, x, 28, 12);
    add_to(f, x, 29, 13);
    x[12] = store(ms(r0, c1, r1, c3));
    x[13] = store(ma(r0, c3, r1, c1));

    let r0 = f.local(f.sub(v!(x, 26), v!(x, 10)));
    let r1 = f.local(f.sub(v!(x, 27), v!(x, 11)));
    add_to(f, x, 26, 10);
    add_to(f, x, 27, 11);
    x[10] = store(f.mul(f.sub(r0, r1), c2));
    x[11] = store(f.mul(f.add(r0, r1), c2));

    let r0 = f.local(f.sub(v!(x, 24), v!(x, 8)));
    let r1 = f.local(f.sub(v!(x, 25), v!(x, 9)));
    add_to(f, x, 24, 8);
    add_to(f, x, 25, 9);
    x[8] = store(ms(r0, c3, r1, c1));
    x[9] = store(ma(r1, c3, r0, c1));

    let r0 = f.local(f.sub(v!(x, 22), v!(x, 6)));
    let r1 = f.local(f.sub(v!(x, 7), v!(x, 23)));
    add_to(f, x, 22, 6);
    add_to(f, x, 23, 7);
    x[6] = store(r1);
    x[7] = store(r0);

    let r0 = f.local(f.sub(v!(x, 4), v!(x, 20)));
    let r1 = f.local(f.sub(v!(x, 5), v!(x, 21)));
    add_to(f, x, 20, 4);
    add_to(f, x, 21, 5);
    x[4] = store(ma(r1, c1, r0, c3));
    x[5] = store(ms(r1, c3, r0, c1));

    let r0 = f.local(f.sub(v!(x, 2), v!(x, 18)));
    let r1 = f.local(f.sub(v!(x, 3), v!(x, 19)));
    add_to(f, x, 18, 2);
    add_to(f, x, 19, 3);
    x[2] = store(f.mul(f.add(r1, r0), c2));
    x[3] = store(f.mul(f.sub(r1, r0), c2));

    let r0 = f.local(f.sub(v!(x, 0), v!(x, 16)));
    let r1 = f.local(f.sub(v!(x, 1), v!(x, 17)));
    add_to(f, x, 16, 0);
    add_to(f, x, 17, 1);
    x[0] = store(ma(r1, c3, r0, c1));
    x[1] = store(ms(r1, c1, r0, c3));

    butterfly_16(f, &mut x[..16]);
    butterfly_16(f, &mut x[16..32]);
}

/// One step of the first and the generic butterflies: `x1[k]`, `x1[k+1]`, `x2[k]`, `x2[k+1]`
/// with the trig pair `(t0, t1)`.
fn butterfly_step(f: Fpu, x: &mut [f32], a: usize, b: usize, t0: f32, t1: f32) {
    let (t0, t1) = (f64::from(t0), f64::from(t1));
    let r0 = f.local(f.sub(v!(x, a), v!(x, b)));
    let r1 = f.local(f.sub(v!(x, a + 1), v!(x, b + 1)));
    add_to(f, x, a, b);
    add_to(f, x, a + 1, b + 1);
    x[b] = store(f.add(f.mul(r1, t1), f.mul(r0, t0)));
    x[b + 1] = store(f.sub(f.mul(r1, t0), f.mul(r0, t1)));
}

fn butterfly_first(f: Fpu, t: &[f32], x: &mut [f32], points: usize) {
    let mut x1 = points as isize - 8;
    let mut x2 = (points >> 1) as isize - 8;
    let mut ti = 0usize;
    loop {
        let (a, b) = (x1 as usize, x2 as usize);
        for (k, s) in [(6usize, 0usize), (4, 4), (2, 8), (0, 12)] {
            butterfly_step(f, x, a + k, b + k, t[ti + s], t[ti + s + 1]);
        }
        x1 -= 8;
        x2 -= 8;
        ti += 16;
        if x2 < 0 {
            break;
        }
    }
}

fn butterfly_generic(f: Fpu, t: &[f32], x: &mut [f32], points: usize, trigint: usize) {
    let mut x1 = points as isize - 8;
    let mut x2 = (points >> 1) as isize - 8;
    let mut ti = 0usize;
    loop {
        let (a, b) = (x1 as usize, x2 as usize);
        for k in [6usize, 4, 2, 0] {
            butterfly_step(f, x, a + k, b + k, t[ti], t[ti + 1]);
            ti += trigint;
        }
        x1 -= 8;
        x2 -= 8;
        if x2 < 0 {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fpu::Precision;

    /// The transform is the inverse MDCT (`y[i] = sum X[k] cos(pi/N (i + 1/2 + N/2)(k + 1/2))`,
    /// N = n/2 coefficients), checked against the plain sum.
    #[test]
    fn backward_is_the_inverse_mdct() {
        for f in [Fpu::new(Precision::Bits24), Fpu::model(false, true, false)] {
            for n in [64usize, 256, 2048] {
                let m = Mdct::new(n, f);
                let mut x = vec![0f32; n];
                let mut s = 1u32;
                for v in x.iter_mut().take(n / 2) {
                    s = s.wrapping_mul(1_103_515_245).wrapping_add(12345);
                    *v = ((s >> 8) % 2001) as f32 / 1000.0 - 1.0;
                }
                let input: Vec<f64> = x[..n / 2].iter().map(|&v| f64::from(v)).collect();
                m.backward(&mut x);
                let half = n / 2;
                for (i, &y) in x.iter().enumerate() {
                    let mut acc = 0f64;
                    for (k, &c) in input.iter().enumerate() {
                        acc += c
                            * (std::f64::consts::PI / half as f64
                                * (i as f64 + 0.5 + half as f64 / 2.0)
                                * (k as f64 + 0.5))
                                .cos();
                    }
                    assert!(
                        (f64::from(y) - acc).abs() < 1e-3 * (half as f64).sqrt(),
                        "n {n} sample {i}: {y} against {acc}"
                    );
                }
            }
        }
    }
}
