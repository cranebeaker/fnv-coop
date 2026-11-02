//! Test builds only: the host heap allocations each thread makes, counted by a global allocator
//! around the system's, for the `#[ignore]`d measurements of what allocates per operation on the
//! paths a game frame runs (`cargo test --release -p engine-core --lib allocations_per -- --ignored
//! --nocapture`). The count is the running thread's own, so tests running beside each other do
//! not mix theirs.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

struct Counting;

thread_local! {
    /// Allocations and reallocations made on this thread (a constant initializer and no
    /// destructor: reading it never allocates itself).
    static MADE: Cell<u64> = const { Cell::new(0) };
}

#[inline]
fn made_one() {
    // A thread being torn down has no counter any more; its last allocations go uncounted.
    let _ = MADE.try_with(|n| n.set(n.get() + 1));
}

// SAFETY: every call goes to the system allocator with the arguments it was given.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        made_one();
        System.alloc(layout)
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        made_one();
        System.alloc_zeroed(layout)
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        made_one();
        System.realloc(ptr, layout, new_size)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

/// The allocations (reallocations included) the running thread has made so far.
pub fn made() -> u64 {
    MADE.try_with(Cell::get).unwrap_or(0)
}

/// What one run of an operation cost: host heap allocations and nanoseconds.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Per {
    pub allocations: f64,
    pub ns: f64,
}

/// What `f` costs on the running thread, per run, over `runs` runs (after one run to warm up: a
/// table's first insert, a buffer's first growth).
pub fn per_run(runs: u32, mut f: impl FnMut()) -> Per {
    f();
    let before = made();
    let t = std::time::Instant::now();
    for _ in 0..runs {
        f();
    }
    let ns = t.elapsed().as_nanos() as f64 / f64::from(runs);
    Per {
        allocations: (made() - before) as f64 / f64::from(runs),
        ns,
    }
}

/// Prints one row of a measurement table: what was run, its allocations and time per run.
pub fn row(what: &str, per: Per) {
    println!("alloc {:8.2} {:12.1} ns  {what}", per.allocations, per.ns);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The counter sees this thread's allocations and nothing else's.
    #[test]
    fn the_count_is_the_threads_own() {
        let per = per_run(10, || {
            std::hint::black_box(vec![1u8; 16]);
        });
        assert_eq!(per.allocations, 1.0);
        // Spawning allocates here (a few), the thread's thousand vectors are its own.
        let before = made();
        std::thread::spawn(|| {
            for _ in 0..1000 {
                std::hint::black_box(vec![1u8; 16]);
            }
        })
        .join()
        .unwrap();
        assert!(made() - before < 100, "{}", made() - before);
    }
}
