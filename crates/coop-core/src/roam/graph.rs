//! Which cells a peer has loaded around its own cell.
//!
//! The engine loads one scene around one player: an interior is that cell
//! (plus whatever a door shows), an exterior is a square of grid cells
//! around the player's. The host cannot ask a guest what it loaded, so both
//! sides derive the set from the same table: exterior cells are registered
//! with their worldspace and grid coordinates, interior links are
//! registered explicitly. Nothing here depends on the game crates.

use std::collections::{BTreeMap, BTreeSet};

use crate::ids::CellId;

/// Grid cells loaded on each side of the player's (the game's `uGridsToLoad`
/// is 5: two on every side).
pub const GRID_RADIUS: i32 = 2;

/// Where an exterior cell sits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GridPos {
    /// Worldspace form id.
    pub world: u32,
    /// Grid x.
    pub x: i32,
    /// Grid y.
    pub y: i32,
}

/// The cell table both sides share.
#[derive(Clone, Debug)]
pub struct CellGraph {
    grid: BTreeMap<CellId, GridPos>,
    by_pos: BTreeMap<GridPos, CellId>,
    links: BTreeMap<CellId, BTreeSet<CellId>>,
    /// Grid cells on each side that count as loaded.
    pub radius: i32,
}

impl Default for CellGraph {
    fn default() -> Self {
        CellGraph { grid: BTreeMap::new(), by_pos: BTreeMap::new(), links: BTreeMap::new(), radius: GRID_RADIUS }
    }
}

impl CellGraph {
    /// Empty table with the default radius.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers an exterior grid cell.
    pub fn register_exterior(&mut self, cell: CellId, world: u32, x: i32, y: i32) {
        let p = GridPos { world, x, y };
        self.grid.insert(cell, p);
        self.by_pos.insert(p, cell);
    }

    /// Declares two cells visible to each other (an interior seen through a
    /// door, a load door pair). Symmetric.
    pub fn link(&mut self, a: CellId, b: CellId) {
        self.links.entry(a).or_default().insert(b);
        self.links.entry(b).or_default().insert(a);
    }

    /// Grid position of an exterior cell.
    pub fn grid_of(&self, cell: CellId) -> Option<GridPos> {
        self.grid.get(&cell).copied()
    }

    /// True for a registered exterior cell.
    pub fn is_exterior(&self, cell: CellId) -> bool {
        self.grid.contains_key(&cell)
    }

    /// True when both cells are exterior cells of the same worldspace (their
    /// coordinates share one frame, so distances across them mean something).
    pub fn same_space(&self, a: CellId, b: CellId) -> bool {
        a == b || matches!((self.grid_of(a), self.grid_of(b)), (Some(p), Some(q)) if p.world == q.world)
    }

    /// Every cell a peer standing in `home` has loaded: the cell itself, the
    /// registered grid square around it, and the explicitly linked cells.
    pub fn loaded_set(&self, home: CellId) -> BTreeSet<CellId> {
        let mut s = BTreeSet::new();
        s.insert(home);
        if let Some(g) = self.grid.get(&home) {
            for dx in -self.radius..=self.radius {
                for dy in -self.radius..=self.radius {
                    if let Some(c) = self.by_pos.get(&GridPos { world: g.world, x: g.x + dx, y: g.y + dy }) {
                        s.insert(*c);
                    }
                }
            }
        }
        if let Some(l) = self.links.get(&home) {
            s.extend(l.iter().copied());
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn world() -> CellGraph {
        let mut g = CellGraph::new();
        for x in 0..10 {
            for y in 0..10 {
                g.register_exterior(CellId(1000 + (x * 10 + y) as u32), 0x3C, x, y);
            }
        }
        g
    }

    #[test]
    fn exterior_loads_a_square() {
        let g = world();
        let s = g.loaded_set(CellId(1000 + 55));
        assert_eq!(s.len(), 25);
        assert!(s.contains(&CellId(1000 + 33)) && s.contains(&CellId(1000 + 77)));
        assert!(!s.contains(&CellId(1000 + 88)));
        let corner = g.loaded_set(CellId(1000));
        assert_eq!(corner.len(), 9, "clipped at the map edge");
    }

    #[test]
    fn interior_loads_itself_and_links() {
        let mut g = world();
        g.link(CellId(5), CellId(1000 + 55));
        assert_eq!(g.loaded_set(CellId(5)).len(), 2);
        assert!(g.loaded_set(CellId(1000 + 55)).contains(&CellId(5)));
        assert_eq!(g.loaded_set(CellId(77)).len(), 1, "unknown cells load alone");
        assert!(g.same_space(CellId(1000), CellId(1099)));
        assert!(!g.same_space(CellId(5), CellId(1000)));
    }
}
