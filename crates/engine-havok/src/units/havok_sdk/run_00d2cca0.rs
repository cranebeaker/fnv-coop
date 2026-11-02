//! `Havok SDK/run_00d2cca0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The table of 17 function words the function fills with the default handler `00d01e50`.
const HANDLER_TABLE_01268A90: u32 = 0x0126_8a90;
const DEFAULT_HANDLER: u32 = 0x00d0_1e50;

// Translated from 00d2cca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InitMTSystem`'s helper (cdecl, one word `queue`; descriptions, not confirmed): sets the 17
/// words of the table at `01268a90` to `0x00d01e50`; raises the word at `queue + 0x60` to 2 when
/// it is below (signed); calls `00d04a20` on `queue` with `(0, 0xc, 1)`; registers two sets of
/// handlers with `00d044c0` on `queue`, each a `(table, 17, function, function)` block passed by
/// value after a first word (0, then 1): `(0x00d96600, 0x00d969e0)` and `(0x00d967a0,
/// 0x00d968e0)`; and calls `00da3400` and `00da3480` with `queue`.
pub fn fn_00d2cca0(e: &mut Engine, queue: Ptr) {
    for index in 0..17 {
        e.mem
            .set_u32(HANDLER_TABLE_01268A90 + index * 4, DEFAULT_HANDLER);
    }
    if e.mem.i32(queue.addr() + 0x60) < 2 {
        e.mem.set_i32(queue.addr() + 0x60, 2);
    }
    e.call(0x00d0_4a20, &args![queue, 0u32, 0xcu32, 1u32]);
    e.call(
        0x00d0_44c0,
        &args![
            queue,
            0u32,
            HANDLER_TABLE_01268A90,
            0x11u32,
            0x00d9_6600u32,
            0x00d9_69e0u32
        ],
    );
    e.call(
        0x00d0_44c0,
        &args![
            queue,
            1u32,
            HANDLER_TABLE_01268A90,
            0x11u32,
            0x00d9_67a0u32,
            0x00d9_68e0u32
        ],
    );
    e.call(0x00da_3400, &args![queue]);
    e.call(0x00da_3480, &args![queue]);
}

/// The jump table of [`fn_00d01e50`] for job kinds 0 to 15 (`00d01f78`, read from the exe) as the
/// job function each target calls (`cdecl`: the job's `+0x1c`, the first word, the job); 0 for
/// the target `00d01f71` (answer 1).
const JOB_FUNCTIONS_00D01F78: [u32; 16] = [
    0x00d9_5cf0, // 00d01e67
    0x00d9_4df0, // 00d01e7a
    0,           // 00d01f71
    0x00d9_4c40, // 00d01e8d
    0x00d9_4a80, // 00d01ea0
    0x00d9_4340, // 00d01f25
    0x00d9_47e0, // 00d01eb3
    0x00d9_44d0, // 00d01ec6
    0x00d9_4570, // 00d01ed9
    0x00d9_4670, // 00d01eec
    0x00d9_4730, // 00d01eff
    0x00d9_43d0, // 00d01f12
    0x00d9_3ba0, // 00d01f38
    0x00d9_3380, // 00d01f4b
    0x00d9_31a0, // 00d01f5e
    0x00d9_3380, // 00d01f4b
];

// Translated from 00d01e50 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; 00d01e50 to the `RET` at 00d01f76, its jump table after it):
/// the default job handler [`fn_00d2cca0`] puts in every slot of the table at `01268a90`, `cdecl`
/// with two words (a context and the job). By the job's kind (byte `+0`, the jump table at
/// `00d01f78`) it calls that kind's job function with (the job's `+0x1c`, the context, the job)
/// and answers what it answers; kind 2 and kinds above 15 answer 1.
pub fn fn_00d01e50(e: &mut Engine, context: u32, job: Ptr) -> u32 {
    let kind = e.mem.u8(job.addr());
    if kind > 0xf {
        return 1;
    }
    let function = JOB_FUNCTIONS_00D01F78[usize::from(kind)];
    if function == 0 {
        return 1;
    }
    let word = e.mem.u32(job.addr().wrapping_add(0x1c));
    e.call(function, &args![word, context, job]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d0_1e50, fn_00d01e50(u32, Ptr) -> u32),
        entry!(0x00d2_cca0, fn_00d2cca0(Ptr)),
    ]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    #[test]
    fn fn_00d2cca0_fills_the_table_and_registers_the_handlers() {
        for (before, after) in [(0, 2), (1, 2), (2, 2), (5, 5), (-3, 2)] {
            let mut e = Engine::new();
            e.map(0x0126_8000, 0x1000);
            for address in [0x00d0_4a20, 0x00d0_44c0, 0x00da_3400, 0x00da_3480] {
                e.register(address, |_, _| Ret::default());
            }
            let queue = e.mem.alloc(0x80);
            e.mem.set_i32(queue + 0x60, before);
            e.call_log = Some(vec![]);
            e.call(0x00d2_cca0, &args![queue]);
            let log = e.call_log.take().unwrap();
            assert_eq!(e.mem.i32(queue + 0x60), after);
            for index in 0..17 {
                assert_eq!(e.mem.u32(HANDLER_TABLE_01268A90 + index * 4), 0x00d0_1e50);
            }
            assert_eq!(e.mem.u32(HANDLER_TABLE_01268A90 + 17 * 4), 0);
            assert_eq!(
                log[1..],
                [
                    (0x00d0_4a20, vec![queue, 0, 0xc, 1]),
                    (
                        0x00d0_44c0,
                        vec![queue, 0, 0x0126_8a90, 0x11, 0x00d9_6600, 0x00d9_69e0]
                    ),
                    (
                        0x00d0_44c0,
                        vec![queue, 1, 0x0126_8a90, 0x11, 0x00d9_67a0, 0x00d9_68e0]
                    ),
                    (0x00da_3400, vec![queue]),
                    (0x00da_3480, vec![queue]),
                ]
            );
        }
    }
}

#[cfg(test)]
mod gap1_tests {
    use super::*;

    #[test]
    fn fn_00d01e50_runs_the_job_function_of_the_kind() {
        let mut e = Engine::new();
        for function in JOB_FUNCTIONS_00D01F78 {
            if function != 0 {
                e.register(function, |_, a| (a[0] ^ a[1]).into_ret());
            }
        }
        e.call_log = Some(vec![]);
        let job = e.mem.alloc(0x20);
        e.mem.set_u32(job + 0x1c, 0x1c00);
        for kind in 0..=0x10u8 {
            e.mem.set_u8(job, kind);
            let answer = e.call(0x00d0_1e50, &args![0x55u32, job]).u32();
            let expected = if kind == 2 || kind > 15 {
                1
            } else {
                0x1c00 ^ 0x55
            };
            assert_eq!(answer, expected, "kind {kind}");
        }
        let called: Vec<(u32, Vec<u32>)> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != 0x00d0_1e50)
            .collect();
        let expected: Vec<(u32, Vec<u32>)> = [
            0x00d9_5cf0u32,
            0x00d9_4df0,
            0x00d9_4c40,
            0x00d9_4a80,
            0x00d9_4340,
            0x00d9_47e0,
            0x00d9_44d0,
            0x00d9_4570,
            0x00d9_4670,
            0x00d9_4730,
            0x00d9_43d0,
            0x00d9_3ba0,
            0x00d9_3380,
            0x00d9_31a0,
            0x00d9_3380,
        ]
        .iter()
        .map(|f| (*f, vec![0x1c00, 0x55, job]))
        .collect();
        assert_eq!(called, expected);
    }
}
