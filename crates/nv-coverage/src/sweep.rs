//! Cell and world load sweep: loads every interior cell and every exterior
//! grid square, and counts what fails.
//!
//! Two modes. `records` needs only the ESMs: it runs the world crate's cell
//! loader (placement, arrivals, left-out references). `full` needs the whole
//! `Data` folder and runs the viewer's own loader (`cellview::Game`: models,
//! textures, collision, terrain, doors), the same code the viewer uses.

use std::collections::{BTreeMap, BTreeSet};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::Path;

use esm::{FormId, FourCC, LoadOrder};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Records,
    Full,
}

#[derive(Debug, Default, Clone)]
pub struct Counts {
    pub tried: usize,
    pub loaded: usize,
    pub failed: usize,
}

#[derive(Debug, Default)]
pub struct SweepReport {
    pub full: bool,
    pub interiors: Counts,
    /// Worldspace editor id → counts.
    pub exteriors: BTreeMap<String, Counts>,
    /// Failure kind → (cells affected, examples).
    pub failures: BTreeMap<String, (usize, Vec<String>)>,
    /// Asset problem kind (missing model, missing texture, ...) → path → cells affected.
    pub assets: BTreeMap<String, BTreeMap<String, usize>>,
    /// Distinct model paths the loaded cells name (records mode: what to check on full data).
    pub model_paths: BTreeSet<String>,
    pub skipped: Option<String>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Limits {
    pub max_interiors: Option<usize>,
    /// Every Nth exterior square.
    pub stride: usize,
    pub max_squares_per_world: Option<usize>,
}

impl SweepReport {
    fn fail(&mut self, kind: &str, example: String) {
        let e = self.failures.entry(kind.to_string()).or_default();
        e.0 += 1;
        if e.1.len() < 5 {
            e.1.push(example);
        }
    }

    fn note(&mut self, line: &str, cell: &str) {
        let Some((kind, rest)) = classify(line) else {
            return;
        };
        let path = rest
            .split(" (x")
            .next()
            .unwrap_or(rest)
            .split(": ")
            .next()
            .unwrap_or(rest)
            .to_string();
        *self
            .assets
            .entry(kind.to_string())
            .or_default()
            .entry(path)
            .or_default() += 1;
        let _ = cell;
    }
}

/// A note line from the loader that reports a problem: its kind and detail.
pub fn classify(line: &str) -> Option<(&'static str, &str)> {
    for (prefix, kind) in [
        ("missing model: ", "missing model"),
        ("missing texture: ", "missing texture"),
        (
            "unreadable model: ",
            "unreadable model (unsupported NIF or damaged)",
        ),
        ("unreadable collision: ", "unreadable collision"),
        ("unreadable texture: ", "unreadable texture"),
        ("placed water ", "water"),
    ] {
        if let Some(rest) = line.strip_prefix(prefix) {
            return Some((kind, rest));
        }
    }
    None
}

fn panic_text(p: Box<dyn std::any::Any + Send>) -> String {
    p.downcast_ref::<String>()
        .cloned()
        .or_else(|| p.downcast_ref::<&str>().map(|s| s.to_string()))
        .unwrap_or_else(|| "panic".into())
}

fn name_of(order: &LoadOrder, id: FormId) -> String {
    order
        .get(id)
        .and_then(|r| r.editor_id().ok().flatten())
        .unwrap_or_else(|| id.to_string())
}

/// Exterior squares by worldspace.
pub fn squares(order: &LoadOrder) -> BTreeMap<FormId, Vec<(i32, i32)>> {
    let xclc = FourCC::new(b"XCLC");
    let mut map: BTreeMap<FormId, Vec<(i32, i32)>> = BTreeMap::new();
    for rr in order.records_of_type(FourCC::new(b"CELL")) {
        let Some(w) = order.world_of(&rr) else {
            continue;
        };
        if rr.entry.header.is_deleted() {
            continue;
        }
        if let Ok(Some(d)) = rr.subrecord(xclc) {
            if d.len() >= 8 {
                let x = i32::from_le_bytes([d[0], d[1], d[2], d[3]]);
                let y = i32::from_le_bytes([d[4], d[5], d[6], d[7]]);
                map.entry(w).or_default().push((x, y));
            }
        }
    }
    for v in map.values_mut() {
        v.sort_unstable();
    }
    map
}

pub fn run(dir: &Path, order: &LoadOrder, mode: Mode, limits: Limits) -> SweepReport {
    let mut rep = SweepReport {
        full: mode == Mode::Full,
        ..SweepReport::default()
    };
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let game = if mode == Mode::Full {
        match cellview::Game::open(
            dir,
            &cellview::Options {
                official: true,
                ..Default::default()
            },
        ) {
            Ok(g) => Some(g),
            Err(e) => {
                rep.skipped = Some(format!("cannot open the full game data: {e}"));
                std::panic::set_hook(hook);
                return rep;
            }
        }
    } else {
        None
    };
    let interiors = world::interior_cells(order).unwrap_or_default();
    for c in interiors
        .iter()
        .take(limits.max_interiors.unwrap_or(usize::MAX))
    {
        rep.interiors.tried += 1;
        let label = c.editor_id.clone().unwrap_or_else(|| c.form_id.to_string());
        let out = catch_unwind(AssertUnwindSafe(|| {
            load_one(order, game.as_ref(), c.form_id)
        }));
        record(&mut rep, true, "", &label, out);
    }
    let stride = limits.stride.max(1);
    for (w, sq) in squares(order) {
        let wname = name_of(order, w);
        let grid = match world::WorldGrid::load(order, w) {
            Ok(g) => g,
            Err(e) => {
                rep.fail("worldspace failed to load", format!("{wname}: {e}"));
                continue;
            }
        };
        let list: Vec<(i32, i32)> = sq
            .into_iter()
            .step_by(stride)
            .take(limits.max_squares_per_world.unwrap_or(usize::MAX))
            .collect();
        for s in list {
            rep.exteriors.entry(wname.clone()).or_default().tried += 1;
            let label = format!("{wname} ({},{})", s.0, s.1);
            let out = catch_unwind(AssertUnwindSafe(|| {
                load_square(order, game.as_ref(), &grid, s)
            }));
            record(&mut rep, false, &wname, &label, out);
        }
    }
    std::panic::set_hook(hook);
    rep
}

type Outcome = Result<(Vec<String>, Vec<String>), String>;

fn load_one(order: &LoadOrder, game: Option<&cellview::Game>, id: FormId) -> Outcome {
    match game {
        Some(g) => g
            .load_cell(id)
            .map(|s| (s.notes, Vec::new()))
            .map_err(|e| e.to_string()),
        None => world::load_cell(order, id)
            .map(|c| summarise(&c))
            .map_err(|e| e.to_string()),
    }
}

fn load_square(
    order: &LoadOrder,
    game: Option<&cellview::Game>,
    grid: &world::WorldGrid,
    s: (i32, i32),
) -> Outcome {
    match game {
        Some(g) => match g.load_square(grid, s) {
            Ok(Some(v)) => Ok((v.notes, Vec::new())),
            Ok(None) => Err("no cell at the square".into()),
            Err(e) => Err(e.to_string()),
        },
        None => match grid.load_square(order, s) {
            Ok(Some(c)) => Ok(summarise(&c)),
            Ok(None) => Err("no cell at the square".into()),
            Err(e) => Err(e.to_string()),
        },
    }
}

/// Records mode: the loader's own report as (problem lines, model paths).
fn summarise(c: &world::LoadedCell) -> (Vec<String>, Vec<String>) {
    let mut problems = Vec::new();
    for (reason, n) in &c.skipped {
        problems.push(format!("left out: {reason} (x{n})"));
    }
    let models = c.objects.iter().filter_map(|o| o.model.clone()).collect();
    (problems, models)
}

fn record(
    rep: &mut SweepReport,
    interior: bool,
    world: &str,
    label: &str,
    out: Result<Outcome, Box<dyn std::any::Any + Send>>,
) {
    fn counts<'a>(rep: &'a mut SweepReport, interior: bool, world: &str) -> &'a mut Counts {
        if interior {
            &mut rep.interiors
        } else {
            rep.exteriors.entry(world.to_string()).or_default()
        }
    }
    match out {
        Err(p) => {
            counts(rep, interior, world).failed += 1;
            rep.fail("panic", format!("{label}: {}", panic_text(p)));
        }
        Ok(Err(e)) => {
            counts(rep, interior, world).failed += 1;
            rep.fail(
                &format!("load error: {}", e.chars().take(80).collect::<String>()),
                label.to_string(),
            );
        }
        Ok(Ok((notes, models))) => {
            counts(rep, interior, world).loaded += 1;
            for m in models {
                rep.model_paths.insert(m);
            }
            for n in &notes {
                if let Some(rest) = n.strip_prefix("left out: ") {
                    let key = rest.split(" (x").next().unwrap_or(rest).to_string();
                    rep.fail_soft("left out", key, label);
                } else {
                    rep.note(n, label);
                }
            }
        }
    }
}

impl SweepReport {
    /// A problem that does not fail the load: counted per reason, with examples.
    fn fail_soft(&mut self, kind: &str, key: String, label: &str) {
        let e = self.failures.entry(format!("{kind}: {key}")).or_default();
        e.0 += 1;
        if e.1.len() < 3 {
            e.1.push(label.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_notes() {
        assert_eq!(
            classify("missing model: meshes\\a.nif (x3)").unwrap().0,
            "missing model"
        );
        assert!(classify("collision: 4 triangles").is_none());
    }
}
