//! `BSMain/run_00c3dbf0` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Fields as the code uses them (the class is not named in the code): the
/// task queue +0x64, the mode +0x68 (6 picks the first time budget and
/// skips the priority rewrite), the callback run after each task +0x70 and
/// the callback run first +0x74. A task's 64-bit key is at +0x10.
const QUEUE: u32 = 0x64;
const MODE: u32 = 0x68;
const AFTER_TASK: u32 = 0x70;
const BEFORE_RUN: u32 = 0x74;
const TASK_KEY: u32 = 0x10;
/// The mode with its own budget.
const MODE_SIX: u32 = 6;

/// Globals: the "running" byte, the two time budgets (the code tests the
/// constant addresses `01202da8` and `01202db4`, which are never null, and
/// then reads the words 4 bytes after them), the fallback word the
/// never-taken branches clear and read, the timer scale, and the task
/// sequence counter.
const RUNNING: u32 = 0x0120_2dd8;
const BUDGET_SIX_TEST: u32 = 0x0120_2da8;
const BUDGET_SIX: u32 = 0x0120_2dac;
const BUDGET_OTHER_TEST: u32 = 0x0120_2db4;
const BUDGET_OTHER: u32 = 0x0120_2db8;
const BUDGET_FALLBACK: u32 = 0x0120_2800;
const TIMER_SCALE: u32 = 0x011a_c39c;
const TASK_SEQUENCE: u32 = 0x0120_2d9c;

/// `BSPrecisionTimer::GetTimer` (`00aa4d80`, the counter in `EDX:EAX`) and
/// `00ec62f6` (the `ST0` value as a 64-bit integer in `EDX:EAX`).
const GET_TIMER: u32 = 0x00aa_4d80;
const FLOAT_TO_INT64: u32 = 0x00ec_62f6;
/// `00c3e420` (`thiscall` on the queue, `&task`): takes the next task.
const QUEUE_NEXT: u32 = 0x00c3_e420;
/// `0044dd60` (`thiscall` on a task): its release.
const TASK_RELEASE: u32 = 0x0044_dd60;
/// `00c3fb50` (`thiscall`, `(task)`): queues a task; the byte says whether
/// it was taken.
const QUEUE_TASK: u32 = 0x00c3_fb50;
/// `__allshr` (`00ec77e0`) and `__allshl` (`00ec7810`).
const ALLSHR: u32 = 0x00ec_77e0;
const ALLSHL: u32 = 0x00ec_7810;
/// `InterlockedIncrement` (import slot).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// A task's virtual slots: `0x14` runs it, `0x18` prepares it.
const SLOT_TASK_RUN: u32 = 0x14;
const SLOT_TASK_PREPARE: u32 = 0x18;

// Translated from 00c3dbf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c3dbf0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2dd89, state variable) is not translated
/// Unnamed in the engine map (unit `run_00c3dbf0`): runs queued tasks for
/// a time budget. When already running (the byte at `01202dd8`) it returns
/// true at once. Otherwise it marks itself running, takes the budget (the
/// word at `01202dac` in mode 6, at `01202db8` otherwise) times the float
/// at `011ac39c` as a 64-bit tick limit, runs the first callback (+0x74;
/// its byte becomes the result, true without one), then takes tasks from
/// the queue (`00c3e420`): each runs (slot `0x14`), then the after-task
/// callback (+0x70); once the ticks since the start pass the limit the
/// task is released (`0044dd60`) and the loop ends, otherwise the next
/// task is taken. Clears the running byte and returns the result.
///
/// The compiler kept two tests of constant non-null addresses (the budget
/// words' neighbours) whose other branches clear and read `01202800`;
/// they are translated but never taken. So are the count of a task
/// pointer known to be null and the release of the pointer at the end,
/// which is null there. The callbacks are called with `ECX` holding their
/// own address, passed here as their first word.
pub fn fn_00c3dbf0(e: &mut Engine, this: Ptr) -> bool {
    let this = this.addr();
    if e.mem.u8(RUNNING) != 0 {
        return true;
    }
    let mut result = 1u8;
    e.mem.set_u8(RUNNING, 1);
    let budget_word = if e.mem.u32(this.wrapping_add(MODE)) == MODE_SIX {
        if BUDGET_SIX_TEST != 0 {
            BUDGET_SIX
        } else {
            e.mem.set_u32(BUDGET_FALLBACK, 0);
            BUDGET_FALLBACK
        }
    } else if BUDGET_OTHER_TEST != 0 {
        BUDGET_OTHER
    } else {
        e.mem.set_u32(BUDGET_FALLBACK, 0);
        BUDGET_FALLBACK
    };
    let budget = e.mem.u32(budget_word);
    let start = e.call(GET_TIMER, &args![]).u64();
    let ticks = x87_mul(f64::from(budget), x87_m32(e.mem.u32(TIMER_SCALE)));
    let limit = e.call(FLOAT_TO_INT64, &args![ticks]).u64();
    let before = e.mem.u32(this.wrapping_add(BEFORE_RUN));
    if before != 0 {
        result = e.call(before, &args![before]).u8();
    }
    e.with_stack(4, |e, task| {
        let task = task.addr();
        if e.mem.u32(task) != 0 {
            let held = e.mem.u32(task);
            e.call(INTERLOCKED_INCREMENT, &args![held.wrapping_add(8)]);
        }
        let queue = e.mem.u32(this.wrapping_add(QUEUE));
        e.call(QUEUE_NEXT, &args![queue, task]);
        while e.mem.u32(task) != 0 {
            let current = e.mem.u32(task);
            e.vcall(current, SLOT_TASK_RUN, &args![]);
            let after = e.mem.u32(this.wrapping_add(AFTER_TASK));
            if after != 0 {
                e.call(after, &args![after]);
            }
            let elapsed = e.call(GET_TIMER, &args![]).u64().wrapping_sub(start);
            if (elapsed as i64) > (limit as i64) {
                if e.mem.u32(task) != 0 {
                    let held = e.mem.u32(task);
                    if held != 0 {
                        e.call(TASK_RELEASE, &args![held]);
                    }
                    e.mem.set_u32(task, 0);
                    let held = e.mem.u32(task);
                    if held != 0 {
                        e.call(INTERLOCKED_INCREMENT, &args![held.wrapping_add(8)]);
                    }
                }
            } else {
                let queue = e.mem.u32(this.wrapping_add(QUEUE));
                e.call(QUEUE_NEXT, &args![queue, task]);
            }
        }
        e.mem.set_u8(RUNNING, 0);
        let held = e.mem.u32(task);
        if held != 0 {
            e.call(TASK_RELEASE, &args![held]);
        }
    });
    result != 0
}

// Translated from 00c3de80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `run_00c3dbf0`): queues `task`. Unless
/// the mode is 6, a task whose priority byte (bits 16-23 of its key,
/// `__allshr`) is not above the mode has that byte cleared (plus
/// `__allshl(0, 16)`). The task's slot `0x18` runs; then `00c3fb50` offers
/// it until it is taken, each refusal giving the key's low 16 bits a new
/// number from the counter at `01202d9c` (`InterlockedIncrement`). Returns
/// true.
pub fn fn_00c3de80(e: &mut Engine, this: Ptr, task: Ptr) -> bool {
    let this = this.addr();
    let task = task.addr();
    let key_at = task.wrapping_add(TASK_KEY);
    let mode = e.mem.u32(this.wrapping_add(MODE));
    if mode != MODE_SIX {
        let key = e.mem.u64(key_at);
        let priority = e.call(ALLSHR, &args![key, 0x10u8]).u8();
        if i32::from(priority) <= e.mem.u32(this.wrapping_add(MODE)) as i32 {
            let key = e.mem.u64(key_at);
            let cleared = key & 0xffff_ffff_ff00_ffff;
            let added = e.call(ALLSHL, &args![0u64, 0x10u8]).u64();
            e.mem.set_u64(key_at, cleared.wrapping_add(added));
        }
    }
    e.vcall(task, SLOT_TASK_PREPARE, &args![]);
    while e.call(QUEUE_TASK, &args![this, task]).u8() == 0 {
        let number = e.call(INTERLOCKED_INCREMENT, &args![TASK_SEQUENCE]).u32() as u16;
        let key = e.mem.u64(key_at);
        let renumbered = (key & 0xffff_ffff_ffff_0000).wrapping_add(u64::from(number));
        e.mem.set_u64(key_at, renumbered);
    }
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c3_dbf0, fn_00c3dbf0(Ptr) -> bool),
        entry!(0x00c3_de80, fn_00c3de80(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const RUN: u32 = 0x00ee_0100;
    const PREPARE: u32 = 0x00ee_0200;
    const BEFORE: u32 = 0x00ee_0300;
    const AFTER: u32 = 0x00ee_0400;

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            eax.into_ret()
        });
    }

    fn task(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + SLOT_TASK_RUN, RUN);
        e.mem.set_u32(vtable + SLOT_TASK_PREPARE, PREPARE);
        let task = e.mem.alloc(0x20);
        e.mem.set_u32(task, vtable);
        task
    }

    /// A runner over `tasks` queued tasks whose timer advances by `step`
    /// ticks per read, with budget words 10 (mode 6) and 20 and scale 2.0.
    fn runner(tasks: u32, step: u64) -> (Engine, Log, u32, Vec<u32>) {
        let mut e = Engine::new();
        let log = Log::default();
        e.map(RUNNING, 4);
        e.map(BUDGET_SIX_TEST, 0x20);
        e.mem.set_u32(BUDGET_SIX, 10);
        e.mem.set_u32(BUDGET_OTHER, 20);
        e.map(TIMER_SCALE, 4);
        e.mem.set_f32(TIMER_SCALE, 2.0);
        let mut now = 0u64;
        e.register_double(GET_TIMER, move |_, _| {
            let value = now;
            now += step;
            value.into_ret()
        });
        let l = log.clone();
        e.register_double(FLOAT_TO_INT64, move |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            l.borrow_mut().push((FLOAT_TO_INT64, vec![value as u32]));
            (value as i64 as u64).into_ret()
        });
        let queued: Vec<u32> = (0..tasks).map(|_| task(&mut e)).collect();
        let mut next = queued.clone().into_iter();
        let l = log.clone();
        e.register_double(QUEUE_NEXT, move |e, a| {
            l.borrow_mut().push((QUEUE_NEXT, vec![a[0]]));
            e.mem.set_u32(a[1], next.next().unwrap_or(0));
            Ret::default()
        });
        logged(&mut e, &log, RUN, 0);
        logged(&mut e, &log, AFTER, 0);
        logged(&mut e, &log, TASK_RELEASE, 0);
        let this = e.mem.alloc(0x78);
        e.mem.set_u32(this + QUEUE, 0x4400);
        e.mem.set_u32(this + AFTER_TASK, AFTER);
        (e, log, this, queued)
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_00c3dbf0_runs_every_task_within_the_budget() {
        let (mut e, log, this, tasks) = runner(3, 1);
        e.mem.set_u32(this + MODE, 6);
        logged(&mut e, &log, BEFORE, 0);
        e.mem.set_u32(this + BEFORE_RUN, BEFORE);
        assert!(!e.call(0x00c3_dbf0, &args![this]).bool());
        assert_eq!(calls(&log, FLOAT_TO_INT64), [vec![20]]);
        assert_eq!(
            calls(&log, RUN),
            tasks.iter().map(|t| vec![*t]).collect::<Vec<_>>()
        );
        assert_eq!(calls(&log, AFTER).len(), 3);
        assert_eq!(calls(&log, QUEUE_NEXT).len(), 4);
        assert!(calls(&log, TASK_RELEASE).is_empty());
        assert_eq!(e.mem.u8(RUNNING), 0);
    }

    #[test]
    fn fn_00c3dbf0_stops_when_the_budget_is_spent() {
        // Budget 20 * 2.0 = 40 ticks; each timer read moves 30.
        let (mut e, log, this, tasks) = runner(3, 30);
        assert!(e.call(0x00c3_dbf0, &args![this]).bool());
        assert_eq!(calls(&log, FLOAT_TO_INT64), [vec![40]]);
        assert_eq!(calls(&log, RUN), [vec![tasks[0]], vec![tasks[1]]]);
        assert_eq!(calls(&log, TASK_RELEASE), [vec![tasks[1]]]);
        // Already running: nothing at all.
        e.mem.set_u8(RUNNING, 1);
        log.borrow_mut().clear();
        assert!(e.call(0x00c3_dbf0, &args![this]).bool());
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn fn_00c3de80_resets_the_priority_and_renumbers_until_taken() {
        let mut e = Engine::new();
        let log = Log::default();
        e.map(TASK_SEQUENCE, 4);
        e.mem.set_u32(TASK_SEQUENCE, 0x1_0005);
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], value);
            value.into_ret()
        });
        e.register(ALLSHR, |_, a| {
            let value = u64::from(a[0]) | u64::from(a[1]) << 32;
            (value >> a[2]).into_ret()
        });
        e.register(ALLSHL, |_, a| {
            let value = u64::from(a[0]) | u64::from(a[1]) << 32;
            (value << a[2]).into_ret()
        });
        logged(&mut e, &log, PREPARE, 0);
        let mut answers = [0u32, 1].into_iter();
        let l = log.clone();
        e.register_double(QUEUE_TASK, move |_, a| {
            l.borrow_mut().push((QUEUE_TASK, a.to_vec()));
            answers.next().unwrap().into_ret()
        });
        let task = task(&mut e);
        let this = e.mem.alloc(0x78);
        e.mem.set_u32(this + MODE, 4);
        e.mem.set_u64(task + TASK_KEY, 0x0000_0001_0003_1234);
        assert!(e.call(0x00c3_de80, &args![this, task]).bool());
        assert_eq!(e.mem.u64(task + TASK_KEY), 0x0000_0001_0000_0006);
        assert_eq!(calls(&log, PREPARE), [vec![task]]);
        assert_eq!(
            calls(&log, QUEUE_TASK),
            [vec![this, task], vec![this, task]]
        );
        // A priority above the mode, or mode 6, keeps it.
        e.mem.set_u64(task + TASK_KEY, 0x0000_0000_0005_0000);
        let mut answers = [1u32].into_iter();
        e.register_double(QUEUE_TASK, move |_, _| answers.next().unwrap().into_ret());
        e.call(0x00c3_de80, &args![this, task]);
        assert_eq!(e.mem.u64(task + TASK_KEY), 0x0000_0000_0005_0000);
    }
}
