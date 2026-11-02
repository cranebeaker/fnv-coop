//! `NiXenonRenderer/nid3dtexturestage.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The texture stage pool (`0126f6cc`: element array at +0, count at +8, growth size at
/// +0x10), its grow `00e7eb50` (unit `nid3dtexturestage.cpp`, unnamed; `thiscall` on the pool,
/// one word: the growth size) and the stage delete `00e7ecd0` (unnamed; `thiscall`, no stack
/// words).
const STAGE_POOL_0126F6CC: u32 = 0x0126_f6cc;
const POOL_GROW_00E7EB50: u32 = 0x00e7_eb50;
const STAGE_DELETE_00E7ECD0: u32 = 0x00e7_ecd0;

/// A stage's reference count (+0x60) down by one; the stage deleted (`00e7ecd0`) at 0.
fn stage_release(e: &mut Engine, stage: u32) {
    let left = e.mem.u32(stage.wrapping_add(0x60)).wrapping_sub(1);
    e.mem.set_u32(stage.wrapping_add(0x60), left);
    if left == 0 {
        e.call(STAGE_DELETE_00E7ECD0, &args![stage]);
    }
}

// Translated from 00e7edf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DTextureStage::CreateNewStage` (engine map), `cdecl` with one word: the address of a
/// stage pointer. When the stage pool (`0126f6cc`) is empty it grows it (`00e7eb50` with its
/// growth size at +0x10) and doubles that size; then takes the pool's first element (the last
/// one moves into its place, the count at +8 goes down), holds it (count at +0x60 up when not
/// null), sets its byte +0x5f (without a null check) and assigns it to the pointer (the old
/// stage's count dropped, deleted through `00e7ecd0` at 0; the new one held); finally the
/// first hold is dropped (`00e7ecd0` when the count reaches 0, a tail jump).
pub fn ni_d3d_texture_stage_create_new_stage(e: &mut Engine, slot: Ptr) {
    let pool = e.global::<u32>(STAGE_POOL_0126F6CC);
    if e.mem.u32(pool.wrapping_add(8)) == 0 {
        let growth = e.mem.u32(pool.wrapping_add(0x10));
        e.call(POOL_GROW_00E7EB50, &args![pool, growth]);
        let growth = e.mem.u32(pool.wrapping_add(0x10));
        e.mem
            .set_u32(pool.wrapping_add(0x10), growth.wrapping_add(growth));
    }
    let elements = e.mem.u32(pool);
    let stage = e.mem.u32(elements);
    let count = e.mem.u32(pool.wrapping_add(8)).wrapping_sub(1);
    e.mem.set_u32(pool.wrapping_add(8), count);
    let last = e.mem.u32(elements.wrapping_add(count.wrapping_mul(4)));
    e.mem.set_u32(elements, last);
    if stage != 0 {
        let held = e.mem.u32(stage + 0x60).wrapping_add(1);
        e.mem.set_u32(stage + 0x60, held);
    }
    e.mem.set_u8(stage.wrapping_add(0x5f), 1);
    let old = e.mem.u32(slot.addr());
    if old != stage {
        if old != 0 {
            stage_release(e, old);
        }
        e.mem.set_u32(slot.addr(), stage);
        let held = e.mem.u32(stage.wrapping_add(0x60)).wrapping_add(1);
        e.mem.set_u32(stage.wrapping_add(0x60), held);
    }
    stage_release(e, stage);
}

use crate::units::nixenonrenderer::run_00e7ef80::{new_pool_block, pool_add_block};

/// The stage array maker `00e7e880` (unnamed; `cdecl`, the count: that many 0x68-byte stages).
const NEW_STAGES_00E7E880: u32 = 0x00e7_e880;

// Translated from 00e7eb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dtexturestage.cpp`): the texture stage pool's grow,
/// `thiscall` on the pool with one stack word (the growth size): a new pool block (0xc bytes
/// from the memory manager: +4 the growth size, +8 cleared, +0 the stages `00e7e880` makes, or 0
/// for a growth of 0) whose 0x68-byte stages are appended to the pool's set (grown through
/// `00a311e0` when full) and the block linked in front of the pool's chain at +0xc, as the
/// render-state pools' grows do (`00e7f160`).
pub fn fn_00e7eb50(e: &mut Engine, this: Ptr, growth: u32) {
    let block = new_pool_block(e, growth, NEW_STAGES_00E7E880);
    pool_add_block(e, this.addr(), growth, block, 0x68);
}

/// The sampler-state setter `00e890c0` (unit `run_00e88fc0`; `thiscall` on the stage's state
/// block, four words: state, value and two bytes) and the filter table it reads (`0126f050`,
/// 12 bytes per filter mode: the words for sampler states 6, 5 and 7).
const SET_SAMPLER_STATE_00E890C0: u32 = 0x00e8_90c0;
const FILTER_TABLE_0126F050: u32 = 0x0126_f050;

// Translated from 00e7def0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DTextureStage::SetFilterMode` (engine map), `thiscall` with the filter mode: the
/// state block at +0x10 gets sampler states 5, 6 and 7 (`00e890c0` with 0 and 0) from the
/// mode's row of the table at `0126f050` (its words +4, +0 and +8, each read again before its
/// call), then the mode is stored at +0. The row's three words also go to the function's own
/// stack slots, which nothing reads.
pub fn ni_d3d_texture_stage_set_filter_mode(e: &mut Engine, this: Ptr, mode: u32) {
    let base = this.addr();
    let row = FILTER_TABLE_0126F050.wrapping_add(mode.wrapping_mul(0xc));
    for (state, offset) in [(5u32, 4u32), (6, 0), (7, 8)] {
        let value = e.mem.u32(row.wrapping_add(offset));
        let block = e.mem.u32(base.wrapping_add(0x10));
        e.call(
            SET_SAMPLER_STATE_00E890C0,
            &args![block, state, value, 0u32, 0u32],
        );
    }
    e.mem.set_u32(base, mode);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e7_edf0, ni_d3d_texture_stage_create_new_stage(Ptr)),
        entry!(0x00e7_eb50, fn_00e7eb50(Ptr, u32)),
        entry!(0x00e7_def0, ni_d3d_texture_stage_set_filter_mode(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A pool holding `stages` stages (count 0 each) in an 8-element array, growth size 4.
    fn engine(stages: u32) -> (Engine, u32, Vec<u32>) {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        let pool = e.mem.alloc(0x18);
        let elements = e.mem.alloc(0x20);
        let made: Vec<u32> = (0..stages).map(|_| e.mem.alloc(0x70)).collect();
        for (index, stage) in made.iter().enumerate() {
            e.mem.set_u32(elements + 4 * index as u32, *stage);
        }
        e.mem.set_u32(pool, elements);
        e.mem.set_u32(pool + 8, stages);
        e.mem.set_u32(pool + 0x10, 4);
        e.set_global(STAGE_POOL_0126F6CC, pool);
        // The grow double puts one new stage in the pool.
        e.register(POOL_GROW_00E7EB50, |e, a| {
            let stage = e.mem.alloc(0x70);
            let elements = e.mem.u32(a[0]);
            e.mem.set_u32(elements, stage);
            e.mem.set_u32(a[0] + 8, 1);
            Ret::default()
        });
        e.register(STAGE_DELETE_00E7ECD0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        (e, pool, made)
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn create_new_stage_takes_the_first_stage_of_the_pool() {
        let (mut e, pool, made) = engine(3);
        let old = e.mem.alloc(0x70);
        e.mem.set_u32(old + 0x60, 1);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, old);
        e.call(0x00e7_edf0, &args![slot]);
        assert_eq!(e.mem.u32(slot), made[0]);
        assert_eq!(e.mem.u32(made[0] + 0x60), 1);
        assert_eq!(e.mem.u8(made[0] + 0x5f), 1);
        assert_eq!(e.mem.u32(pool + 8), 2);
        let elements = e.mem.u32(pool);
        assert_eq!(e.mem.u32(elements), made[2]);
        assert_eq!(e.mem.u32(old + 0x60), 0);
        assert_eq!(calls(&e, STAGE_DELETE_00E7ECD0), vec![vec![old]]);
        assert!(calls(&e, POOL_GROW_00E7EB50).is_empty());
    }

    #[test]
    fn create_new_stage_grows_an_empty_pool_and_drops_an_unkept_stage() {
        let (mut e, pool, _) = engine(0);
        let slot = e.mem.alloc(4);
        e.call(0x00e7_edf0, &args![slot]);
        assert_eq!(calls(&e, POOL_GROW_00E7EB50), vec![vec![pool, 4]]);
        assert_eq!(e.mem.u32(pool + 0x10), 8);
        let stage = e.mem.u32(slot);
        assert_eq!(e.mem.u32(stage + 0x60), 1);
        // The pointer already holds the stage it gets: the only hold is dropped.
        let (mut e, _, made) = engine(1);
        e.mem.set_u32(made[0] + 0x60, 0);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, made[0]);
        e.call(0x00e7_edf0, &args![slot]);
        assert_eq!(e.mem.u32(made[0] + 0x60), 0);
        assert_eq!(calls(&e, STAGE_DELETE_00E7ECD0), vec![vec![made[0]]]);
    }
}

#[cfg(test)]
mod pool_grow_tests {
    use super::*;

    #[test]
    fn fn_00e7eb50_appends_the_new_stages() {
        let mut e = Engine::new();
        e.register(0x00aa_3e40, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(NEW_STAGES_00E7E880, |e, a| {
            e.mem.alloc(0x68 * a[0]).into_ret()
        });
        e.register(0x00a3_11e0, |e, a| {
            let new = e.mem.alloc(4 * a[1]);
            e.mem.set_u32(a[0], new);
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        let pool = e.mem.alloc(0x18);
        e.call(0x00e7_eb50, &args![pool, 1u32]);
        let block = e.mem.u32(pool + 0xc);
        let stages = e.mem.u32(block);
        assert_eq!(e.mem.u32(block + 4), 1);
        assert_eq!(e.mem.u32(pool + 8), 1);
        let elements = e.mem.u32(pool);
        assert_eq!(e.mem.u32(elements), stages);
    }
}

#[cfg(test)]
mod b052_tests {
    use super::*;

    #[test]
    fn set_filter_mode_sets_three_sampler_states() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.register(SET_SAMPLER_STATE_00E890C0, |_, _| Ret::default());
        // Row 2: words 0x60, 0x50, 0x70 at +0, +4, +8.
        let row = FILTER_TABLE_0126F050 + 2 * 0xc;
        e.mem.set_u32(row, 0x60);
        e.mem.set_u32(row + 4, 0x50);
        e.mem.set_u32(row + 8, 0x70);
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0x10, 0x7700);
        e.call_log = Some(vec![]);
        e.call(0x00e7_def0, &args![this, 2u32]);
        let log = e.call_log.take().unwrap();
        let sets: Vec<Vec<u32>> = log
            .iter()
            .filter(|(a, _)| *a == SET_SAMPLER_STATE_00E890C0)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(
            sets,
            vec![
                vec![0x7700, 5, 0x50, 0, 0],
                vec![0x7700, 6, 0x60, 0, 0],
                vec![0x7700, 7, 0x70, 0, 0]
            ]
        );
        assert_eq!(e.mem.u32(this), 2);
    }
}
