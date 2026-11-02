//! `d3dx-measure check FILE`: reads a measurement file (no DLL involved) and counts, for each
//! way of ordering a 4x4 product's multiplies and adds, how many results differ bit for bit
//! from the measured ones. The arithmetic is IEEE single precision with the SSE rules for NaNs
//! spelled out (an operation with a NaN operand answers the first NaN operand, quieted; an
//! invalid operation answers the default NaN ffc00000), so the answer does not depend on how
//! the compiler orders commutative operations.

fn quiet(x: u32) -> u32 {
    x | 0x0040_0000
}

fn is_nan(x: u32) -> bool {
    x & 0x7fff_ffff > 0x7f80_0000
}

/// `x op y` with SSE's NaN choice.
fn op(x: u32, y: u32, f: fn(f32, f32) -> f32) -> u32 {
    if is_nan(x) {
        return quiet(x);
    }
    if is_nan(y) {
        return quiet(y);
    }
    let r = f(f32::from_bits(x), f32::from_bits(y));
    if r.is_nan() {
        0xffc0_0000
    } else {
        r.to_bits()
    }
}

fn mul(x: u32, y: u32) -> u32 {
    op(x, y, |a, b| a * b)
}

fn add(x: u32, y: u32) -> u32 {
    op(x, y, |a, b| a + b)
}

/// One way to work out `out[i][j]` from the products `p[k] = a[i][k] * b[k][j]`.
#[derive(Clone, Debug)]
enum Order {
    /// Multiply operands b first; accumulate in the permutation's order; `acc + p` or `p + acc`.
    Sequential { b_first: bool, perm: [usize; 4], acc_first: bool },
    /// `(p[w] + p[x]) + (p[y] + p[z])` with each add's operands in the order given.
    Pairs { b_first: bool, pairs: [usize; 4], swap: [bool; 3] },
    /// In double precision: the products exact (`round_products` false) or rounded to single,
    /// summed in the permutation's order in double, rounded to single once at the end.
    Wide { b_first: bool, perm: [usize; 4], round_products: bool },
    /// The first product rounded, then each further one fused into the sum (`fma`).
    Fused { b_first: bool, perm: [usize; 4] },
}

fn is_nan64(x: u64) -> bool {
    x & 0x7fff_ffff_ffff_ffff > 0x7ff0_0000_0000_0000
}

/// Single to double (`cvtss2sd`: a NaN keeps its payload, quieted).
fn widen(x: u32) -> u64 {
    if is_nan(x) {
        let sign = u64::from(x >> 31) << 63;
        return sign | 0x7ff8_0000_0000_0000 | (u64::from(x & 0x007f_ffff) << 29);
    }
    f64::from(f32::from_bits(x)).to_bits()
}

/// Double to single (`cvtsd2ss`: a NaN keeps the top of its payload, quieted).
fn narrow(x: u64) -> u32 {
    if is_nan64(x) {
        let sign = ((x >> 63) as u32) << 31;
        return sign | 0x7fc0_0000 | ((x >> 29) as u32 & 0x007f_ffff);
    }
    (f64::from_bits(x) as f32).to_bits()
}

fn op64(x: u64, y: u64, f: fn(f64, f64) -> f64) -> u64 {
    if is_nan64(x) {
        return x | 0x0008_0000_0000_0000;
    }
    if is_nan64(y) {
        return y | 0x0008_0000_0000_0000;
    }
    let r = f(f64::from_bits(x), f64::from_bits(y));
    if r.is_nan() {
        0xfff8_0000_0000_0000
    } else {
        r.to_bits()
    }
}

fn permutations() -> Vec<[usize; 4]> {
    let mut out = Vec::new();
    for a in 0..4 {
        for b in 0..4 {
            for c in 0..4 {
                for d in 0..4 {
                    let p = [a, b, c, d];
                    let mut seen = [false; 4];
                    p.iter().for_each(|&i| seen[i] = true);
                    if seen.iter().all(|&s| s) {
                        out.push(p);
                    }
                }
            }
        }
    }
    out
}

fn orders() -> Vec<Order> {
    let mut out = Vec::new();
    for b_first in [false, true] {
        for perm in permutations() {
            for acc_first in [true, false] {
                out.push(Order::Sequential { b_first, perm, acc_first });
            }
            for round_products in [false, true] {
                out.push(Order::Wide { b_first, perm, round_products });
            }
            out.push(Order::Fused { b_first, perm });
        }
        for pairs in [[0, 1, 2, 3], [0, 2, 1, 3], [0, 3, 1, 2]] {
            for s in 0..8 {
                out.push(Order::Pairs {
                    b_first,
                    pairs,
                    swap: [s & 1 != 0, s & 2 != 0, s & 4 != 0],
                });
            }
        }
    }
    out
}

fn product(a: &[u32; 16], b: &[u32; 16], order: &Order) -> [u32; 16] {
    let mut out = [0u32; 16];
    for i in 0..4 {
        for j in 0..4 {
            let b_first = match order {
                Order::Sequential { b_first, .. }
                | Order::Pairs { b_first, .. }
                | Order::Wide { b_first, .. }
                | Order::Fused { b_first, .. } => *b_first,
            };
            let p: Vec<u32> = (0..4)
                .map(|k| {
                    if b_first {
                        mul(b[4 * k + j], a[4 * i + k])
                    } else {
                        mul(a[4 * i + k], b[4 * k + j])
                    }
                })
                .collect();
            out[4 * i + j] = match order {
                Order::Sequential { perm, acc_first, .. } => {
                    let mut acc = p[perm[0]];
                    for &k in &perm[1..] {
                        acc = if *acc_first { add(acc, p[k]) } else { add(p[k], acc) };
                    }
                    acc
                }
                Order::Pairs { pairs, swap, .. } => {
                    let pair = |x: u32, y: u32, s: bool| if s { add(y, x) } else { add(x, y) };
                    let l = pair(p[pairs[0]], p[pairs[1]], swap[0]);
                    let r = pair(p[pairs[2]], p[pairs[3]], swap[1]);
                    pair(l, r, swap[2])
                }
                Order::Fused { perm, .. } => {
                    let x = |k: usize| f32::from_bits(a[4 * i + k]);
                    let y = |k: usize| f32::from_bits(b[4 * k + j]);
                    let mut acc = p[perm[0]];
                    for &k in &perm[1..] {
                        acc = x(k).mul_add(y(k), f32::from_bits(acc)).to_bits();
                    }
                    acc
                }
                Order::Wide { perm, round_products, .. } => {
                    let q: Vec<u64> = (0..4)
                        .map(|k| {
                            if *round_products {
                                widen(p[k])
                            } else {
                                let (x, y) = if b_first {
                                    (b[4 * k + j], a[4 * i + k])
                                } else {
                                    (a[4 * i + k], b[4 * k + j])
                                };
                                op64(widen(x), widen(y), |u, v| u * v)
                            }
                        })
                        .collect();
                    let mut acc = q[perm[0]];
                    for &k in &perm[1..] {
                        acc = op64(acc, q[k], |u, v| u + v);
                    }
                    narrow(acc)
                }
            };
        }
    }
    out
}

fn parse16(text: &str) -> [u32; 16] {
    let mut m = [0u32; 16];
    for (i, w) in text.split(' ').enumerate() {
        m[i] = u32::from_str_radix(w, 16).unwrap();
    }
    m
}

/// Whether a word is a NaN, an infinity or a denormal (or zero's other sign).
fn special(x: u32) -> bool {
    let e = x & 0x7f80_0000;
    e == 0x7f80_0000 || (e == 0 && x & 0x007f_ffff != 0)
}

type Case = (String, String, [u32; 16], [u32; 16], [u32; 16], bool);

fn ranked(cases: &[&Case]) -> Vec<(usize, usize, Order)> {
    let mut results: Vec<(usize, usize, Order)> = Vec::new();
    for order in orders() {
        let mut bad_cases = 0;
        let mut bad_words = 0;
        for (_, _, a, b, out, _) in cases {
            let r = product(a, b, &order);
            // Only words whose inputs and result are plain numbers: the NaN rules apart.
            let plain = |k: usize| {
                let (i, j) = (k / 4, k % 4);
                !(0..4).any(|q| special(a[4 * i + q]) || special(b[4 * q + j])) && !special(out[k])
            };
            let n = (0..16).filter(|&k| plain(k) && r[k] != out[k]).count();
            if n > 0 {
                bad_cases += 1;
                bad_words += n;
            }
        }
        results.push((bad_cases, bad_words, order));
    }
    results.sort_by_key(|r| (r.0, r.1));
    results
}

pub fn check(path: &str) {
    let text = std::fs::read_to_string(path).unwrap();
    let mut cases: Vec<Case> = Vec::new();
    for line in text.lines() {
        let f: Vec<&str> = line.split('\t').collect();
        if f[0] == "MatrixMultiply" {
            cases.push((f[1].to_string(), f[2].to_string(), parse16(f[3]), parse16(f[4]), parse16(f[5]), f[6] == "1"));
        }
    }
    println!("{} cases; returned the out pointer in all: {}", cases.len(), cases.iter().all(|c| c.5));
    let mut keys: Vec<(String, String)> = cases.iter().map(|c| (c.0.clone(), c.1.clone())).collect();
    keys.sort();
    keys.dedup();
    for (alias, cw) in keys {
        let subset: Vec<&Case> = cases.iter().filter(|c| c.0 == alias && c.1 == cw).collect();
        let results = ranked(&subset);
        println!("== {alias} {cw}: {} cases", subset.len());
        for (bad_cases, bad_words, order) in results.iter().take(4) {
            println!("  {bad_cases} cases ({bad_words} words) differ: {order:?}");
        }
        let best = &results[0].2;
        let (mut plain, mut odd) = (0, 0);
        let mut shown = 0;
        for (_, _, a, b, out, _) in &subset {
            let r = product(a, b, best);
            for k in 0..16 {
                if r[k] == out[k] {
                    continue;
                }
                let i = k / 4;
                let j = k % 4;
                let row: Vec<u32> = (0..4).map(|q| a[4 * i + q]).collect();
                let col: Vec<u32> = (0..4).map(|q| b[4 * q + j]).collect();
                let is_odd = row.iter().chain(col.iter()).any(|&x| special(x)) || special(out[k]) || special(r[k]);
                if is_odd {
                    odd += 1;
                } else {
                    plain += 1;
                }
                if !is_odd && shown < 12 {
                    shown += 1;
                    let ex: f64 = (0..4).map(|q| f64::from(f32::from_bits(row[q])) * f64::from(f32::from_bits(col[q]))).sum();
                    let pr: Vec<String> = (0..4).map(|q| format!("{:08x}", mul(row[q], col[q]))).collect();
                    println!("      exact {ex:e} -> {:08x}; measured {:e} model {:e}; f32 products {}", (ex as f32).to_bits(), f32::from_bits(out[k]), f32::from_bits(r[k]), pr.join(" "));
                    let h = |v: &[u32]| v.iter().map(|x| format!("{x:08x}")).collect::<Vec<_>>().join(" ");
                    println!("    [{i}][{j}] a row {} b col {} measured {:08x} model {:08x}", h(&row), h(&col), out[k], r[k]);
                }
            }
        }
        println!("  best's differing words: {plain} with plain values, {odd} with NaN/inf/denormal");
    }
}

/// For each output position, the orders that match best there (plain words only, all cases).
pub fn positions(path: &str) {
    let text = std::fs::read_to_string(path).unwrap();
    let mut cases: Vec<Case> = Vec::new();
    for line in text.lines() {
        let f: Vec<&str> = line.split('\t').collect();
        if f[0] == "MatrixMultiply" && f[1] == "none" {
            cases.push((f[1].to_string(), f[2].to_string(), parse16(f[3]), parse16(f[4]), parse16(f[5]), f[6] == "1"));
        }
    }
    let all = orders();
    for cw in ["027f", "007f"] {
        let subset: Vec<&Case> = cases.iter().filter(|c| c.1 == cw).collect();
        // bad[order][position]
        let mut bad = vec![[0usize; 16]; all.len()];
        let mut total = [0usize; 16];
        for (_, _, a, b, out, _) in &subset {
            let plain = |k: usize| {
                let (i, j) = (k / 4, k % 4);
                !(0..4).any(|q| special(a[4 * i + q]) || special(b[4 * q + j])) && !special(out[k])
            };
            for k in 0..16 {
                if plain(k) {
                    total[k] += 1;
                }
            }
            for (o, order) in all.iter().enumerate() {
                let r = product(a, b, order);
                for k in 0..16 {
                    if plain(k) && r[k] != out[k] {
                        bad[o][k] += 1;
                    }
                }
            }
        }
        println!("== {cw}");
        for k in 0..16 {
            let mut idx: Vec<usize> = (0..all.len()).collect();
            idx.sort_by_key(|&o| bad[o][k]);
            let best: Vec<String> = idx.iter().take(3).map(|&o| format!("{} {:?}", bad[o][k], all[o])).collect();
            println!("[{}][{}] of {}: {}", k / 4, k % 4, total[k], best.join(" | "));
        }
    }
}

/// Rows 0 and 3 as `(p0 + p1) + (p2 + p3)`, rows 1 and 2 as `((p0 + p1) + p2) + p3` (what the
/// plain values show); the operand orders of each multiply and add are the parameters. Counts
/// the differing words over all cases, NaNs included.
fn rows_model(a: &[u32; 16], b: &[u32; 16], params: u32) -> [u32; 16] {
    let bit = |n: u32| params & (1 << n) != 0;
    let mut out = [0u32; 16];
    for i in 0..4 {
        let pair_row = i == 0 || i == 3;
        let b_first = if pair_row { bit(0) } else { bit(1) };
        let add2 = |x: u32, y: u32, swap: bool| if swap { add(y, x) } else { add(x, y) };
        for j in 0..4 {
            let p: Vec<u32> = (0..4)
                .map(|k| if b_first { mul(b[4 * k + j], a[4 * i + k]) } else { mul(a[4 * i + k], b[4 * k + j]) })
                .collect();
            out[4 * i + j] = if pair_row {
                let l = add2(p[0], p[1], bit(2));
                let r = add2(p[2], p[3], bit(3));
                add2(l, r, bit(4))
            } else {
                let s = add2(p[0], p[1], bit(5));
                let s = add2(s, p[2], bit(6));
                add2(s, p[3], bit(7))
            };
        }
    }
    out
}

pub fn rows(path: &str) {
    let text = std::fs::read_to_string(path).unwrap();
    let mut cases: Vec<Case> = Vec::new();
    for line in text.lines() {
        let f: Vec<&str> = line.split('\t').collect();
        if f[0] == "MatrixMultiply" {
            cases.push((f[1].to_string(), f[2].to_string(), parse16(f[3]), parse16(f[4]), parse16(f[5]), f[6] == "1"));
        }
    }
    let mut results: Vec<(usize, u32)> = (0..256u32)
        .map(|params| {
            let bad = cases
                .iter()
                .map(|(_, _, a, b, out, _)| {
                    let r = rows_model(a, b, params);
                    (0..16).filter(|&k| r[k] != out[k]).count()
                })
                .sum();
            (bad, params)
        })
        .collect();
    results.sort();
    for (bad, params) in results.iter().take(6) {
        println!("{bad} words differ with params {params:08b} (bit0 pair rows b*a, bit1 seq rows b*a, bits 2-4 pair adds swapped, bits 5-7 seq adds swapped)");
    }
    let best = results[0].1;
    let mut shown = 0;
    let mut by: std::collections::BTreeMap<(String, String), (usize, usize)> = Default::default();
    for (alias, cw, a, b, out, _) in &cases {
        let r = rows_model(a, b, best);
        let e = by.entry((alias.clone(), cw.clone())).or_default();
        e.0 += 1;
        if r != *out {
            e.1 += 1;
            for k in 0..16 {
                if r[k] != out[k] && shown < 20 {
                    shown += 1;
                    let (i, j) = (k / 4, k % 4);
                    let h = |v: Vec<u32>| v.iter().map(|x| format!("{x:08x}")).collect::<Vec<_>>().join(" ");
                    println!("  {alias} {cw} [{i}][{j}] a row {} b col {} measured {:08x} model {:08x}",
                        h((0..4).map(|q| a[4 * i + q]).collect()), h((0..4).map(|q| b[4 * q + j]).collect()), out[k], r[k]);
                }
            }
        }
    }
    for ((alias, cw), (n, bad)) in by {
        println!("{alias} {cw}: {bad} of {n} cases differ");
    }
}

/// One output word with the row's tree (rows 0 and 3 pairs, 1 and 2 sequential) and `params`:
/// bits 0-3 product k as `b * a`, bits 4-6 the three adds' operands swapped.
fn word_model(a: &[u32; 16], b: &[u32; 16], i: usize, j: usize, params: u32) -> u32 {
    let bit = |n: u32| params & (1 << n) != 0;
    let p: Vec<u32> = (0..4)
        .map(|k| if bit(k as u32) { mul(b[4 * k + j], a[4 * i + k]) } else { mul(a[4 * i + k], b[4 * k + j]) })
        .collect();
    let add2 = |x: u32, y: u32, swap: bool| if swap { add(y, x) } else { add(x, y) };
    if i == 0 || i == 3 {
        let l = add2(p[0], p[1], bit(4));
        let r = add2(p[2], p[3], bit(5));
        add2(l, r, bit(6))
    } else {
        let s = add2(p[0], p[1], bit(4));
        let s = add2(s, p[2], bit(5));
        add2(s, p[3], bit(6))
    }
}

/// For each output position, the operand orders (`word_model`'s params) that match best over
/// all cases, NaNs included, and the words still differing.
pub fn nan_positions(path: &str) {
    let text = std::fs::read_to_string(path).unwrap();
    let mut cases: Vec<Case> = Vec::new();
    for line in text.lines() {
        let f: Vec<&str> = line.split('\t').collect();
        if f[0] == "MatrixMultiply" {
            cases.push((f[1].to_string(), f[2].to_string(), parse16(f[3]), parse16(f[4]), parse16(f[5]), f[6] == "1"));
        }
    }
    for k in 0..16 {
        let (i, j) = (k / 4, k % 4);
        let mut results: Vec<(usize, u32)> = (0..128u32)
            .map(|params| {
                let bad = cases.iter().filter(|(_, _, a, b, out, _)| word_model(a, b, i, j, params) != out[k]).count();
                (bad, params)
            })
            .collect();
        results.sort();
        let best: Vec<String> = results.iter().take(4).map(|(bad, p)| format!("{bad} with {p:07b}")).collect();
        println!("[{i}][{j}]: {}", best.join(", "));
        let mut shown = 0;
        for (alias, cw, a, b, out, _) in &cases {
            let r = word_model(a, b, i, j, results[0].1);
            if r != out[k] && shown < 3 {
                shown += 1;
                let h = |v: Vec<u32>| v.iter().map(|x| format!("{x:08x}")).collect::<Vec<_>>().join(" ");
                println!("    {alias} {cw} a row {} b col {} measured {:08x} model {r:08x}",
                    h((0..4).map(|q| a[4 * i + q]).collect()), h((0..4).map(|q| b[4 * q + j]).collect()), out[k]);
            }
        }
    }
}
