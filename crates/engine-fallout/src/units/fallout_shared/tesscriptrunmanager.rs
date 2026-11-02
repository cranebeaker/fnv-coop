//! `fallout shared/tesscriptrunmanager.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `ScriptRunner` (Xbox PDB), size and offsets checked against PC
    /// constructor 005e0c60 and busy predicate 005e2850.
    pub struct ScriptRunner: 0xa4 {
        0x14 m_scriptRunning: Ptr,
    }
    /// `ScriptRunManager` (Xbox PDB), unchanged eight-byte PC layout.
    pub struct ScriptRunManager: 8 {
        0 m_pScriptRunner: Ptr<ScriptRunner>,
        4 pCurrentRunner: Ptr<ScriptRunner>,
    }
}

const MANAGER: u32 = 0x011c_aed0;
const INITIALIZED: u32 = 0x011c_aed8;
/// The run manager's entry count, raised by `InterlockedIncrement`
/// (`0040b460`) on entry and lowered (`004019a0`) on exit.
const RUN_DEPTH: u32 = 0x011c_aedc;
/// `QueryPerformanceFrequency` result cached by the runner (two words).
const PROFILER_FREQUENCY: u32 = 0x011c_aec8;
/// Import slots of `QueryPerformanceFrequency` and `QueryPerformanceCounter`.
const QUERY_PERFORMANCE_FREQUENCY: u32 = 0x00fd_f0a4;
const QUERY_PERFORMANCE_COUNTER: u32 = 0x00fd_f0a0;
/// `SCRIPTS: Nested call to ScriptRunner::Run. Script '%s' attempting to
/// execute script '%s'.`
const NESTED_RUN_MESSAGE: u32 = 0x0103_d498;

// Translated from 005e0c60 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// `ScriptRunner::ScriptRunner` (Xbox PDB). Preserves the two padding bytes.
pub fn runner_construct(e: &mut Engine, this: Ptr<ScriptRunner>) -> Ptr<ScriptRunner> {
    for offset in [0x20, 0x4c, 0x18, 0x1c] {
        e.mem.set_u32(this.addr() + offset, 0);
    }
    for offset in [0x24, 0x50, 0x78] {
        e.call(0x0040_3d30, &args![this.addr() + offset, 0u32, 0x28u32]);
    }
    for offset in [0, 4, 8, 0xc, 0x10, 0x14] {
        e.mem.set_u32(this.addr() + offset, 0);
    }
    e.mem.set_u8(this.addr() + 0xa0, 0);
    e.mem.set_u8(this.addr() + 0xa1, 0);
    this
}

// Translated from 005e0d20 (PC disassembly, FalloutNV.exe 1.4.0.525).
// Translation incomplete: 005e0d20 - compiler SEH exception cleanup and stack cookie.
/// `ScriptRunner::Run` (unnamed on PC; the runner's main line loop). Runs
/// `script`'s compiled lines through `ParseLine` (`005e13a0`) and
/// `EvaluateLine` (`005e1550`, execute mode) until the data ends, a parse
/// fails (the script's data size is then cleared), a line fails, or a
/// `return` (opcode 0x1e) is reached. The runner remembers the container
/// (+0), the reference's form (+4, `007af430`) and the locals (+8; a fresh
/// `005abf60` set, deleted afterwards, when none is given). A magic-effect
/// script stores `effect_start`, `effect_finish` and `elapsed` in the
/// locals' 8-byte effect block (allocated when missing). Unless
/// `keep_actions` is set or a line failed, the locals' action list is
/// processed (`005a8ea0`). With the interface's debug text visible
/// (`00702fc0`) the run time is added to the script's profiler timer
/// (`005e1140`). Answers the runner's "references changed" byte (+0xa1),
/// cleared on entry; a busy runner logs and runs nothing.
#[allow(clippy::too_many_arguments)] // PC receiver plus eight stack words.
pub fn runner_run(
    e: &mut Engine,
    this: Ptr<ScriptRunner>,
    script: Ptr,
    reference: Ptr,
    locals: Ptr,
    container: Ptr,
    keep_actions: bool,
    effect_start: u8,
    effect_finish: u8,
    elapsed: f32,
) -> bool {
    let profile = e.call(0x0070_2fc0, &args![]).bool();
    e.with_stack(8, |e, start| {
        if profile {
            if e.mem.u32(PROFILER_FREQUENCY) | e.mem.u32(PROFILER_FREQUENCY + 4) == 0 {
                e.call(QUERY_PERFORMANCE_FREQUENCY, &args![PROFILER_FREQUENCY]);
            }
            e.call(QUERY_PERFORMANCE_COUNTER, &args![start]);
        }
        e.mem.set_u8(this.addr() + 0xa1, 0);
        if script.is_null() || !e.call(0x0040_13e0, &args![script]).bool() {
            return e.mem.u8(this.addr() + 0xa1) != 0;
        }
        if !e.get(this, ScriptRunner::m_scriptRunning).is_null() {
            let requested_name = e.vcall(script.addr(), 0x130, &args![]).u32();
            let running = e.get(this, ScriptRunner::m_scriptRunning);
            let running_name = e.vcall(running.addr(), 0x130, &args![]).u32();
            e.call(
                0x005b_5e40,
                &args![NESTED_RUN_MESSAGE, running_name, requested_name],
            );
            return e.mem.u8(this.addr() + 0xa1) != 0;
        }
        e.set(this, ScriptRunner::m_scriptRunning, script);
        // CExpression (Xbox PDB), 0x710 bytes at EBP-0x75c.
        e.with_stack(0x710, |e, expression| {
            e.call(0x0059_3490, &args![expression]);
            let mut created_locals = 0u32;
            e.mem.set_u32(this.addr() + 0x24, 0);
            e.mem.set_u32(this.addr() + 0x20, 0);
            e.with_stack(20, |e, work| {
                let opcode = work;
                let cursor = work.byte_add(4);
                let length = work.byte_add(8);
                let target = work.byte_add(12);
                let skip = work.byte_add(16);
                e.mem.set_u32(opcode.addr(), 0xffff);
                e.mem.set_u32(cursor.addr(), 0);
                e.mem.set_u32(this.addr(), container.addr());
                let form = if reference.is_null() {
                    0
                } else {
                    e.call(0x007a_f430, &args![reference]).u32()
                };
                e.mem.set_u32(this.addr() + 4, form);
                if locals.is_null() {
                    created_locals = e.call(0x005a_bf60, &args![script]).u32();
                    e.mem.set_u32(this.addr() + 8, created_locals);
                } else {
                    e.mem.set_u32(this.addr() + 8, locals.addr());
                }
                let header = e.call(0x0050_0940, &args![script]).u32();
                if e.mem.u8(header + 0x11) != 0 {
                    // ScriptLocals::m_pScriptEffectData (Xbox PDB) +0x10.
                    let runner_locals = e.mem.u32(this.addr() + 8);
                    if e.mem.u32(runner_locals + 0x10) == 0 {
                        let block = e.call(0x0040_1000, &args![8u32]).u32();
                        let runner_locals = e.mem.u32(this.addr() + 8);
                        e.mem.set_u32(runner_locals + 0x10, block);
                    }
                    let effect = e.mem.u32(e.mem.u32(this.addr() + 8) + 0x10);
                    e.mem.set_u8(effect, effect_start);
                    let effect = e.mem.u32(e.mem.u32(this.addr() + 8) + 0x10);
                    e.mem.set_u8(effect + 1, effect_finish);
                    let effect = e.mem.u32(e.mem.u32(this.addr() + 8) + 0x10);
                    e.mem.set_f32(effect + 4, x87_quiet(elapsed));
                }
                e.mem.set_u32(length.addr(), 0);
                e.mem.set_u32(target.addr(), 0);
                e.mem.set_u32(skip.addr(), 0);
                let mut failed = false;
                let mut line = 0u32;
                loop {
                    let header = e.call(0x0050_0940, &args![script]).u32();
                    if e.mem.u32(cursor.addr()) >= e.mem.u32(header + 8) {
                        break;
                    }
                    line = line.wrapping_add(1);
                    e.mem.set_u32(this.addr() + 0x10, 0);
                    e.mem.set_u32(length.addr(), 0);
                    let skipping = e.mem.u32(skip.addr()) != 0;
                    if !e
                        .call(
                            0x005e_13a0,
                            &args![this, script, opcode, cursor, length, target, skipping],
                        )
                        .bool()
                    {
                        let header = e.call(0x0050_0940, &args![script]).u32();
                        e.mem.set_u32(header + 8, 0);
                        break;
                    }
                    let remaining = e.mem.u32(skip.addr());
                    if remaining != 0 {
                        e.mem.set_u32(skip.addr(), remaining.wrapping_sub(1));
                    } else {
                        if e.mem.u32(opcode.addr()) == 0x1e {
                            break;
                        }
                        if e.mem.u32(target.addr()) == 0 {
                            e.mem.set_u32(target.addr(), reference.addr());
                        }
                        let command = e.mem.u32(opcode.addr());
                        let target_ref = e.mem.u32(target.addr());
                        let position = e.mem.u32(cursor.addr());
                        if !e
                            .call(
                                0x005e_1550,
                                &args![
                                    this, script, command, target_ref, length, position, skip,
                                    line, 0u32, 0u32
                                ],
                            )
                            .bool()
                        {
                            failed = true;
                            break;
                        }
                    }
                    let next = e
                        .mem
                        .u32(cursor.addr())
                        .wrapping_add(e.mem.u32(length.addr()));
                    e.mem.set_u32(cursor.addr(), next);
                }
                if !keep_actions && !failed {
                    let runner_locals = e.mem.u32(this.addr() + 8);
                    e.call(0x005a_8ea0, &args![runner_locals]);
                }
            });
            if created_locals != 0 {
                e.call(0x0041_af70, &args![created_locals, 1u32]);
            }
            e.set(this, ScriptRunner::m_scriptRunning, Ptr::NULL);
            if profile {
                e.with_stack(8, |e, now| {
                    e.call(QUERY_PERFORMANCE_COUNTER, &args![now]);
                    let ticks =
                        (e.mem.u64(now.addr()) as i64).wrapping_sub(e.mem.u64(start.addr()) as i64);
                    let frequency = e.mem.u64(PROFILER_FREQUENCY) as i64;
                    let seconds = (ticks as f64 / frequency as f64) as f32;
                    e.call(0x005e_1140, &args![script, seconds]);
                });
            }
            let changed = e.mem.u8(this.addr() + 0xa1);
            e.call(0x0048_3710, &args![expression]);
            changed != 0
        })
    })
}

// Translated from 005e1140 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Adds `seconds` to a script's profiler timer (`Script::fProfilerTimer`
/// (Xbox PDB), +0x34): x87 `FLD`/`FADD`/`FSTP dword`.
pub fn fn_005e1140(e: &mut Engine, this: Ptr, seconds: f32) {
    let timer = e.mem.f32(this.addr() + 0x34) as f64;
    e.mem
        .set_f32(this.addr() + 0x34, (timer + seconds as f64) as f32);
}

// Translated from 005e23f0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// The thread's console-echo byte (TLS +0x268): set while a console command
/// runs, so script messages go to the console instead of the log.
pub fn fn_005e23f0(e: &mut Engine) -> bool {
    let tls = e.tls();
    e.mem.u8(tls + 0x268) != 0
}

// Translated from 005e2410 (PC disassembly, FalloutNV.exe 1.4.0.525).
// Translation incomplete: 005e2410 - stack cookie.
/// The runner's error report (`cdecl`, variable arguments after the format):
/// formats into a 0x108-byte buffer (`00407000`, `vsprintf`), prints it on
/// the console (`00703c00`) while a console command runs (`005e23f0`) or
/// logs it (`005b5e40`) otherwise, clears the script's data size and sets
/// the runner's error byte (+0xa0).
pub fn runner_report_error(
    e: &mut Engine,
    this: Ptr<ScriptRunner>,
    script: Ptr,
    format: u32,
    variable_arguments: &[u32],
) {
    let list = e.mem.alloc(4 * variable_arguments.len().max(1) as u32);
    for (i, word) in variable_arguments.iter().enumerate() {
        e.mem.set_u32(list + 4 * i as u32, *word);
    }
    e.with_stack(0x108, |e, buffer| {
        e.call(0x0040_7000, &args![buffer, format, list]);
        if fn_005e23f0(e) {
            let text = e.call(0x0046_4f30, &args![buffer, 0u32]).u32();
            e.call(0x0070_3c00, &args![text]);
        } else {
            let text = e.call(0x0046_4f30, &args![buffer, 0u32]).u32();
            e.call(0x005b_5e40, &args![text]);
        }
    });
    e.mem.free(list);
    let header = e.call(0x0050_0940, &args![script]).u32();
    e.mem.set_u32(header + 8, 0);
    e.mem.set_u8(this.addr() + 0xa0, 1);
}

/// The uniform form of [`runner_report_error`] (variable arguments).
fn runner_report_error_entry(e: &mut Engine, a: &[u32]) -> Ret {
    runner_report_error(e, Ptr::new(a[0]), Ptr::new(a[1]), a[2], &a[3..]);
    Ret::default()
}

// Translated from 005e24b0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Stores an error code in the runner (+0x18); the second word (the script
/// the caller passes) is not read. `RET 8`.
pub fn runner_set_error_code(
    e: &mut Engine,
    this: Ptr<ScriptRunner>,
    _unused_script: u32,
    code: u32,
) {
    e.mem.set_u32(this.addr() + 0x18, code);
}

// Translated from 005e2550 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Frees the manager's cached runner (`00401030`, plain free: no
/// destructor runs) and clears both runner pointers.
pub fn manager_clear(e: &mut Engine, this: Ptr<ScriptRunManager>) {
    let runner = e.get(this, ScriptRunManager::m_pScriptRunner);
    if !runner.is_null() {
        e.call(0x0040_1030, &args![runner]);
        e.set(this, ScriptRunManager::m_pScriptRunner, Ptr::NULL);
    }
    e.set(this, ScriptRunManager::pCurrentRunner, Ptr::NULL);
}

// Translated from 005e2590 (PC disassembly, FalloutNV.exe 1.4.0.525).
// Translation incomplete: 005e2590 - compiler SEH exception cleanup.
/// `ScriptRunManager::Run` (Xbox PDB). The outermost run (entry count 1
/// after `0040b460`) uses the cached runner, created on first use, and
/// makes it current for the call; a nested run builds a temporary runner,
/// makes it current, restores the previous current runner afterwards and
/// frees the temporary (`00401030`). Both pass the eight words through to
/// `ScriptRunner::Run` (`005e0d20`); the float goes through `FLD`/`FSTP`.
#[allow(clippy::too_many_arguments)] // PC receiver plus eight stack words.
pub fn run(
    e: &mut Engine,
    this: Ptr<ScriptRunManager>,
    script: Ptr,
    reference: Ptr,
    locals: Ptr,
    container: Ptr,
    keep_actions: u8,
    effect_start: u8,
    effect_finish: u8,
    elapsed: f32,
) -> bool {
    let words = |runner: Ptr<ScriptRunner>| {
        args![
            runner,
            script,
            reference,
            locals,
            container,
            keep_actions as u32,
            effect_start as u32,
            effect_finish as u32,
            x87_quiet(elapsed)
        ]
    };
    let result = if e.call(0x0040_b460, &args![RUN_DEPTH]).u32() == 1 {
        if e.get(this, ScriptRunManager::m_pScriptRunner).is_null() {
            let block = e.call(0x0040_1000, &args![0xa4u32]).u32();
            let runner = if block == 0 {
                Ptr::NULL
            } else {
                e.call(0x005e_0c60, &args![block]).ptr::<ScriptRunner>()
            };
            e.set(this, ScriptRunManager::m_pScriptRunner, runner);
        }
        let cached = e.get(this, ScriptRunManager::m_pScriptRunner);
        e.set(this, ScriptRunManager::pCurrentRunner, cached);
        let runner = e.get(this, ScriptRunManager::m_pScriptRunner);
        let result = e.call(0x005e_0d20, &words(runner)).bool();
        e.set(this, ScriptRunManager::pCurrentRunner, Ptr::NULL);
        result
    } else {
        let block = e.call(0x0040_1000, &args![0xa4u32]).u32();
        let temporary = if block == 0 {
            Ptr::NULL
        } else {
            e.call(0x005e_0c60, &args![block]).ptr::<ScriptRunner>()
        };
        let previous = e.get(this, ScriptRunManager::pCurrentRunner);
        e.set(this, ScriptRunManager::pCurrentRunner, temporary);
        let result = e.call(0x005e_0d20, &words(temporary)).bool();
        e.set(this, ScriptRunManager::pCurrentRunner, previous);
        e.call(0x0040_1030, &args![temporary]);
        result
    };
    e.call(0x0040_19a0, &args![RUN_DEPTH]);
    result
}

// Translated from 005e1160 (PC disassembly, FalloutNV.exe 1.4.0.525).
// Translation incomplete: 005e1160 - compiler SEH exception cleanup.
/// Builds a script's action list through the real parser and EvaluateLine
/// entry. Parser failure clears the script's data size; evaluator failure
/// merely ends this pass. Both normal exits return true after clearing busy.
pub fn runner_init_action_list(
    e: &mut Engine,
    this: Ptr<ScriptRunner>,
    script: Ptr,
    reference: Ptr,
    locals: Ptr,
) -> bool {
    if script.is_null() || !e.call(0x0040_13e0, &args![script]).bool() {
        return false;
    }
    if !e.get(this, ScriptRunner::m_scriptRunning).is_null() {
        let requested_name = e.vcall(script.addr(), 0x130, &args![]).u32();
        let running = e.get(this, ScriptRunner::m_scriptRunning);
        let running_name = e.vcall(running.addr(), 0x130, &args![]).u32();
        e.call(
            0x005b_5e40,
            &args![0x0103_d498u32, running_name, requested_name],
        );
        return false;
    }
    e.set(this, ScriptRunner::m_scriptRunning, script);
    // CExpression (Xbox PDB) is 0x710 bytes, matching EBP-0x73c..EBP-0x2c.
    // Its constructors initialize stack indices, not the embedded buffers.
    e.with_stack(0x710, |e, expression| {
        e.call(0x0059_3490, &args![expression]);
        let object = if reference.is_null() {
            0
        } else {
            e.call(0x007a_f430, &args![reference]).u32()
        };
        e.mem.set_u32(this.addr() + 4, object);
        e.mem.set_u32(this.addr() + 8, locals.addr());
        e.with_stack(20, |e, work| {
            let opcode = work;
            let cursor = work.byte_add(4);
            let length = work.byte_add(8);
            let target = work.byte_add(12);
            let skip = work.byte_add(16);
            e.mem.set_u32(opcode.addr(), 0xffff);
            for offset in [4, 8, 12, 16] {
                e.mem.set_u32(work.addr() + offset, 0);
            }
            let mut line = 0u32;
            loop {
                let header = e.call(0x0050_0940, &args![script]).u32();
                if e.mem.u32(cursor.addr()) >= e.mem.u32(header + 8) {
                    break;
                }
                line = line.wrapping_add(1);
                e.mem.set_u32(this.addr() + 0x10, 0);
                e.mem.set_u32(length.addr(), 0);
                let skipping = e.mem.u32(skip.addr()) != 0;
                if !e
                    .call(
                        0x005e_13a0,
                        &args![this, script, opcode, cursor, length, target, skipping],
                    )
                    .bool()
                {
                    let header = e.call(0x0050_0940, &args![script]).u32();
                    e.mem.set_u32(header + 8, 0);
                    break;
                }
                let remaining = e.mem.u32(skip.addr());
                if remaining != 0 {
                    e.mem.set_u32(skip.addr(), remaining.wrapping_sub(1));
                } else {
                    if e.mem.u32(target.addr()) == 0 {
                        e.mem.set_u32(target.addr(), reference.addr());
                    }
                    let command = e.mem.u32(opcode.addr());
                    let target_ref = e.mem.u32(target.addr());
                    let position = e.mem.u32(cursor.addr());
                    if !e
                        .call(
                            0x005e_1550,
                            &args![
                                this, script, command, target_ref, length, position, skip, line,
                                0u32, 1u32
                            ],
                        )
                        .bool()
                    {
                        break;
                    }
                }
                let next = e
                    .mem
                    .u32(cursor.addr())
                    .wrapping_add(e.mem.u32(length.addr()));
                e.mem.set_u32(cursor.addr(), next);
            }
        });
        e.set(this, ScriptRunner::m_scriptRunning, Ptr::NULL);
        e.call(0x0048_3710, &args![expression]);
    });
    true
}

fn script_word(e: &mut Engine, script: Ptr, cursor: Ptr) -> i32 {
    let data = e.call(0x0067_1d10, &args![script]).u32();
    e.mem.i16(data.wrapping_add(e.mem.u32(cursor.addr()))) as i32
}

fn advance_word(e: &mut Engine, cursor: Ptr) {
    e.mem
        .set_u32(cursor.addr(), e.mem.u32(cursor.addr()).wrapping_add(2));
}

// Translated from 005e13a0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Reads signed opcode/length words, optionally resolving a reference prefix.
/// A skipped prefix leaves the reference output unchanged; ordinary commands
/// clear it. The PC body does not add bounds checks to the bytecode reads.
#[allow(clippy::too_many_arguments)] // PC receiver plus six stack arguments.
pub fn runner_parse_line(
    e: &mut Engine,
    this: Ptr<ScriptRunner>,
    script: Ptr,
    opcode: Ptr,
    cursor: Ptr,
    length: Ptr,
    reference: Ptr,
    skip_reference: bool,
) -> bool {
    e.mem.set_u32(length.addr(), 0);
    let command = script_word(e, script, cursor);
    e.mem.set_i32(opcode.addr(), command);
    advance_word(e, cursor);
    if e.mem.u32(opcode.addr()) == 0x1c {
        let index = script_word(e, script, cursor);
        advance_word(e, cursor);
        let mut variable = false;
        if !skip_reference {
            let locals = e.mem.u32(this.addr() + 8);
            let found = e.call(0x005a_c4f0, &args![script, index, locals]).u32();
            if found != 0 {
                variable = e.mem.u32(found + 0xc) != 0;
                let object = e.mem.u32(found + 8);
                if object != 0 {
                    let target = e
                        .call(
                            0x00ec_43fb,
                            &args![object, 0u32, 0x0118_3028u32, 0x0118_41ccu32, 0u32],
                        )
                        .u32();
                    e.mem.set_u32(reference.addr(), target);
                }
            }
        }
        let command = script_word(e, script, cursor);
        e.mem.set_i32(opcode.addr(), command);
        advance_word(e, cursor);
        if !skip_reference && e.mem.u32(reference.addr()) == 0 && !variable {
            let command = e.mem.u32(opcode.addr());
            let descriptor = e.call(0x005b_1120, &args![command]).u32();
            let script_name = e.vcall(script.addr(), 0x130, &args![]).u32();
            if descriptor != 0 {
                let command_name = e.mem.u32(descriptor);
                e.call(
                    0x005e_2410,
                    &args![this, script, 0x0103_d538u32, command_name, script_name],
                );
            } else {
                e.call(
                    0x005e_2410,
                    &args![this, script, 0x0103_d4f4u32, script_name],
                );
            }
            return false;
        }
    } else {
        e.mem.set_u32(reference.addr(), 0);
    }
    let size = script_word(e, script, cursor);
    e.mem.set_i32(length.addr(), size);
    advance_word(e, cursor);
    true
}

// Translated from 005e1550 (PC disassembly, FalloutNV.exe 1.4.0.525).
// Translation incomplete: 005e1550 - compiler SEH exception cleanup and stack cookie.
/// `ScriptRunner::EvaluateLine` (Xbox PDB): one compiled line at `position`
/// (after its opcode and length words). `BEGIN` (0x10) runs the block
/// type's handler and adds the block's span to `*length` when the block
/// does not apply; with `action_list` every other line is accepted
/// unread. Otherwise: `set` (0x15), `if`/`elseif` (0x16/0x18, the runner's
/// if-stack at +0x20/+0x24: bit 1 the level is open, bit 2 a branch was
/// taken; a false condition adds the jump count to `*skip`), `else`
/// (0x17), `endif` (0x19), `end`, a reference prefix and `scriptname`
/// (0x11, 0x1c, 0x1d: nothing), and every other opcode as a command
/// through its definition (`ScriptCompiler::GetFunctionDef`). `parse_only`
/// reads parameters and expressions without effects. False when the line
/// failed (logged).
#[allow(clippy::too_many_arguments)] // Exact PC receiver and nine stack words.
pub fn runner_evaluate_line(
    e: &mut Engine,
    this: Ptr<ScriptRunner>,
    script: Ptr,
    command: i32,
    reference: Ptr,
    length: Ptr,
    position: u32,
    skip: Ptr,
    line: u32,
    parse_only: bool,
    action_list: bool,
) -> bool {
    let data = e.call(0x0067_1d10, &args![script]).u32();
    if command != 0x10 {
        if action_list {
            return true;
        }
        let line_context = Line {
            this,
            script,
            reference,
            data,
            position,
            skip,
            length,
            line,
            parse_only,
        };
        return match command.wrapping_sub(0x10) as u32 {
            0x01 | 0x0c | 0x0d => true,
            0x05 => evaluate_set(e, &line_context),
            0x06 => {
                let depth = e.mem.u32(this.addr() + 0x20);
                if e.mem.u32(if_level(this, depth)) != 0 {
                    e.mem.set_u32(this.addr() + 0x20, depth.wrapping_add(1));
                }
                let depth = e.mem.u32(this.addr() + 0x20);
                e.mem.set_u32(if_level(this, depth), 1);
                evaluate_condition(e, &line_context)
            }
            0x08 => evaluate_condition(e, &line_context),
            0x07 => {
                let count = e.mem.i16(data.wrapping_add(position)) as i32;
                let depth = e.mem.u32(this.addr() + 0x20);
                let level = e.mem.u32(if_level(this, depth));
                if level & 2 == 0 {
                    e.mem.set_u32(if_level(this, depth), level | 2);
                } else if !parse_only {
                    let skipped = e.mem.u32(skip.addr()).wrapping_add(count as u32);
                    e.mem.set_u32(skip.addr(), skipped);
                }
                true
            }
            0x09 => {
                let depth = e.mem.u32(this.addr() + 0x20);
                e.mem.set_u32(if_level(this, depth), 0);
                let depth = e.mem.u32(this.addr() + 0x20);
                if depth != 0 {
                    e.mem.set_u32(this.addr() + 0x20, depth.wrapping_sub(1));
                }
                true
            }
            0x0e if parse_only => true,
            _ => evaluate_command(e, &line_context, command),
        };
    }
    e.with_stack(12, |e, work| {
        let cursor = work;
        let result = work.byte_add(4);
        let kind = e.mem.i16(data.wrapping_add(position)) as i32;
        e.mem.set_u32(cursor.addr(), position.wrapping_add(2));
        if !(0..0x26).contains(&kind) {
            let form_id = e.call(0x0084_e3a0, &args![script]).u32();
            let name = e.vcall(script.addr(), 0x130, &args![]).u32();
            e.call(0x005b_5e40, &args![0x0103_d8d0u32, line, name, form_id]);
            return false;
        }
        let span = e.mem.u32(data.wrapping_add(e.mem.u32(cursor.addr())));
        e.mem.set_u32(cursor.addr(), position.wrapping_add(6));
        let entry = 0x0118_e300 + kind as u32 * 0x28;
        let parameters = e.mem.u32(entry + 4);
        let handler = e.mem.u32(entry + 8);
        if parse_only {
            if parameters != 0 && e.mem.u32(length.addr()) != 0 {
                let locals = e.mem.u32(this.addr() + 8);
                let container = e.mem.u32(this.addr());
                e.call(
                    0x005a_ccb0,
                    &args![parameters, data, cursor, reference, container, script, locals],
                );
            }
            return true;
        }
        if e.mem.u8(entry) != 0 && reference.is_null() {
            let name = e.vcall(script.addr(), 0x130, &args![]).u32();
            e.call(0x005b_5e40, &args![0x0103_d890u32, name, line]);
            return false;
        }
        let succeeded = if handler == 0 {
            false
        } else {
            let locals = e.mem.u32(this.addr() + 8);
            let container = e.mem.u32(this.addr());
            e.call(
                handler,
                &args![parameters, data, reference, container, script, locals, result, cursor],
            )
            .bool()
        };
        if !succeeded {
            // The PC pushes the cursor and line before calling the name getter.
            let failed_at = e.mem.u32(cursor.addr());
            let name = e.vcall(script.addr(), 0x130, &args![]).u32();
            e.call(0x005b_5e40, &args![0x0103_d848u32, name, line, failed_at]);
            return false;
        }
        if e.mem.f64(result.addr()) == 0.0 || action_list {
            e.mem
                .set_u32(length.addr(), e.mem.u32(length.addr()).wrapping_add(span));
        }
        true
    })
}

/// The words of one `EvaluateLine` call that its branches share.
struct Line {
    this: Ptr<ScriptRunner>,
    script: Ptr,
    reference: Ptr,
    data: u32,
    position: u32,
    skip: Ptr,
    length: Ptr,
    line: u32,
    parse_only: bool,
}

/// The address of if-stack level `depth` (runner +0x24, one word each).
fn if_level(this: Ptr<ScriptRunner>, depth: u32) -> u32 {
    this.addr()
        .wrapping_add(0x24)
        .wrapping_add(depth.wrapping_mul(4))
}

/// `CExpression` error names, one pointer per code.
const EXPRESSION_ERROR_NAMES: u32 = 0x0118_c594;
/// The double the `if` condition is compared with (0.0 in the exe).
const CONDITION_FALSE: u32 = 0x0101_2060;

/// Logs `format` with the line, the script's name (`+0x130`) and its form
/// id (`0084e3a0`, asked first), the order of the `SET` and command errors.
fn log_line_error(e: &mut Engine, at: &Line, format: u32) {
    let form_id = e.call(0x0084_e3a0, &args![at.script]).u32();
    let name = e.vcall(at.script.addr(), 0x130, &args![]).u32();
    e.call(0x005b_5e40, &args![format, at.line, name, form_id]);
}

/// Evaluates `size` expression bytes at `cursor` with a fresh
/// `CExpression` on the stack; `f` sees the expression and the value.
fn with_expression<R>(
    e: &mut Engine,
    at: &Line,
    cursor: u32,
    size: i16,
    parse: bool,
    f: impl FnOnce(&mut Engine, Ptr, f64) -> R,
) -> R {
    e.with_stack(0x710, |e, expression| {
        e.call(0x0059_3490, &args![expression]);
        let container = e.mem.u32(at.this.addr());
        let locals = e.mem.u32(at.this.addr() + 8);
        let value = e
            .call(
                0x0059_3db0,
                &args![
                    expression,
                    at.data.wrapping_add(cursor),
                    at.reference,
                    container,
                    at.script,
                    locals,
                    size as i32,
                    parse
                ],
            )
            .f64();
        let result = f(e, expression, value);
        e.call(0x0048_3710, &args![expression]);
        result
    })
}

/// `set` (0x15) of [`runner_evaluate_line`]: the variable kind (`G` a
/// global; `r` and an index first name the referenced object whose
/// variable it is), the variable index (not for `G`), the expression
/// length and bytes. A reference's variables are its script variables
/// (`005673e0`), a quest's its locals (`0059e300`). Floats (`f`) take the
/// value, integers (`l`, `s`) its `_ftol2` truncation; a global takes it as
/// a float (`0046dce0`). A set variable on a reference, or on the line's
/// reference, marks it changed (`+0x48` with 0x80000000) unless `004077c0`
/// or its process (`008d6f30`, `00450ff0`) says otherwise. While a console
/// command runs, the new value is echoed.
fn evaluate_set(e: &mut Engine, at: &Line) -> bool {
    let data = at.data;
    let mut cursor = at.position;
    let mut kind = e.mem.u8(data.wrapping_add(cursor));
    cursor = cursor.wrapping_add(1);
    let mut variable = 0i16;
    let mut target_locals = e.mem.u32(at.this.addr() + 8);
    let mut object = 0u32;
    if kind == b'G' || kind == b'r' {
        let index = e.mem.i16(data.wrapping_add(cursor));
        cursor = cursor.wrapping_add(2);
        if kind == b'r' {
            kind = e.mem.u8(data.wrapping_add(cursor));
            cursor = cursor.wrapping_add(1);
        }
        let locals = e.mem.u32(at.this.addr() + 8);
        object = e
            .call(0x005a_c4f0, &args![at.script, index as i32, locals])
            .u32();
        if !at.parse_only {
            if kind == b'r' || object == 0 || e.mem.u32(object + 8) == 0 {
                log_line_error(e, at, 0x0103_d7b8);
                return false;
            }
            let form = e.mem.u32(object + 8);
            let as_reference = e
                .call(
                    0x00ec_43fb,
                    &args![form, 0u32, 0x0118_3028u32, 0x0118_41ccu32, 0u32],
                )
                .u32();
            if as_reference != 0 {
                target_locals = e.call(0x0056_73e0, &args![as_reference]).u32();
            } else {
                let form = e.mem.u32(object + 8);
                let as_quest = e
                    .call(
                        0x00ec_43fb,
                        &args![form, 0u32, 0x0118_3028u32, 0x0118_6500u32, 0u32],
                    )
                    .u32();
                if as_quest != 0 {
                    target_locals = e.call(0x0059_e300, &args![as_quest]).u32();
                }
            }
            if target_locals == 0 {
                log_line_error(e, at, 0x0103_d7f8);
                return false;
            }
        }
    }
    if kind != b'G' {
        variable = e.mem.i16(data.wrapping_add(cursor));
        cursor = cursor.wrapping_add(2);
    }
    let size = e.mem.i16(data.wrapping_add(cursor));
    cursor = cursor.wrapping_add(2);
    with_expression(
        e,
        at,
        cursor,
        size,
        at.parse_only,
        |e, expression, value| {
            if e.call(0x0055_9450, &args![expression]).u32() != 0 {
                let code = e.call(0x0055_9450, &args![expression]).u32();
                let error_name = e
                    .mem
                    .u32(EXPRESSION_ERROR_NAMES.wrapping_add(code.wrapping_mul(4)));
                let name = e.vcall(at.script.addr(), 0x130, &args![]).u32();
                e.call(
                    0x005b_5e40,
                    &args![0x0103_d770u32, name, at.line, error_name],
                );
                let header = e.call(0x0050_0940, &args![at.script]).u32();
                e.mem.set_u32(header + 8, 0);
                return false;
            }
            if at.parse_only {
                return true;
            }
            if kind == b'G' {
                if object == 0 || e.mem.u32(object + 8) == 0 {
                    log_line_error(e, at, 0x0103_d718);
                    return false;
                }
                // FSTP dword, then FLD dword for the float argument.
                let single = value as f32;
                let global = e.mem.u32(object + 8);
                e.call(0x0046_dce0, &args![global, single]);
                if fn_005e23f0(e) {
                    e.call(0x0070_3c00, &args![0x0103_d758u32, single as f64]);
                }
                return true;
            }
            if target_locals == 0 {
                log_line_error(e, at, 0x0103_d6a8);
                return false;
            }
            let index = variable as i32;
            match kind {
                b'f' => {
                    let stored = x87_reload_f64(value);
                    e.call(0x005a_9290, &args![target_locals, index, stored]);
                    if fn_005e23f0(e) {
                        let master = e.mem.u32(target_locals);
                        let name = e.call(0x005a_bed0, &args![master, index]).u32();
                        e.call(0x0070_3c00, &args![0x0103_d6f4u32, name, stored]);
                    }
                }
                b'l' | b's' => {
                    let truncated = e.call(0x00ec_62c0, &args![value]).u32() as i32;
                    e.call(0x005a_9290, &args![target_locals, index, truncated as f64]);
                    if fn_005e23f0(e) {
                        let truncated = e.call(0x00ec_62c0, &args![value]).u32();
                        let master = e.mem.u32(target_locals);
                        let name = e.call(0x005a_bed0, &args![master, index]).u32();
                        e.call(0x0070_3c00, &args![0x0103_d704u32, name, truncated]);
                    }
                }
                _ => {}
            }
            let has_form = object != 0 && e.mem.u32(object + 8) != 0;
            if has_form || !at.reference.is_null() {
                let mut target = 0u32;
                if object != 0 {
                    let form = e.mem.u32(object + 8);
                    if form != 0 && e.vcall(form, 0xf0, &args![]).bool() {
                        target = e.mem.u32(object + 8);
                    }
                } else {
                    target = at.reference.addr();
                }
                if target != 0 && !e.call(0x0040_77c0, &args![target]).bool() {
                    let process = e.call(0x008d_6f30, &args![target]).u32();
                    let keep = process != 0 && {
                        let process = e.call(0x008d_6f30, &args![target]).u32();
                        e.call(0x0045_0ff0, &args![process]).bool()
                    };
                    if !keep {
                        e.vcall(target, 0x48, &args![0x8000_0000u32]);
                    }
                }
            }
            true
        },
    )
}

/// x87 `FLD`/`FSTP qword`: a signaling NaN comes out quiet.
fn x87_reload_f64(value: f64) -> f64 {
    let bits = value.to_bits();
    if value.is_nan() && bits & 0x0008_0000_0000_0000 == 0 {
        f64::from_bits(bits | 0x0008_0000_0000_0000)
    } else {
        value
    }
}

/// The condition half of `if` and `elseif` (0x16/0x18) of
/// [`runner_evaluate_line`]: the jump count and expression length words,
/// then the expression. When no branch of this level was taken yet, a
/// condition other than 0.0 takes this one (bit 2); otherwise the jump
/// count is added to `*skip`. `parse_only` reads the expression and
/// marks the level taken. An expression error reports, stores error code
/// 6 (`005e24b0`) and the expression's code (+0x1c), clears the script's
/// data size and fails.
fn evaluate_condition(e: &mut Engine, at: &Line) -> bool {
    let data = at.data;
    let count = e.mem.i16(data.wrapping_add(at.position));
    let size = e.mem.i16(data.wrapping_add(at.position).wrapping_add(2));
    let cursor = at.position.wrapping_add(4);
    let this = at.this;
    let level = |e: &Engine| if_level(this, e.mem.u32(this.addr() + 0x20));
    let add_skip = |e: &mut Engine| {
        let skipped = e.mem.u32(at.skip.addr()).wrapping_add(count as i32 as u32);
        e.mem.set_u32(at.skip.addr(), skipped);
    };
    let parse = at.parse_only;
    e.with_stack(0x710, |e, expression| {
        e.call(0x0059_3490, &args![expression]);
        let container = e.mem.u32(this.addr());
        let locals = e.mem.u32(this.addr() + 8);
        let words = args![
            expression,
            data.wrapping_add(cursor),
            at.reference,
            container,
            at.script,
            locals,
            size as i32,
            parse
        ];
        if parse {
            e.call(0x0059_3db0, &words);
            let slot = level(e);
            e.mem.set_u32(slot, e.mem.u32(slot) | 2);
        } else if e.mem.u32(level(&*e)) & 2 == 0 {
            let value = e.call(0x0059_3db0, &words).f64();
            if value == e.mem.f64(CONDITION_FALSE) {
                add_skip(e);
            } else {
                let slot = level(e);
                e.mem.set_u32(slot, e.mem.u32(slot) | 2);
            }
        } else {
            add_skip(e);
        }
        let succeeded = if e.call(0x0055_9450, &args![expression]).u32() != 0 {
            e.call(0x005e_24b0, &args![this, at.script, 6u32]);
            let name = e.vcall(at.script.addr(), 0x130, &args![]).u32();
            e.call(0x005b_5e40, &args![0x0103_d664u32, name, at.line]);
            let header = e.call(0x0050_0940, &args![at.script]).u32();
            e.mem.set_u32(header + 8, 0);
            let code = e.call(0x0055_9450, &args![expression]).u32();
            e.mem.set_u32(this.addr() + 0x1c, code);
            false
        } else {
            true
        };
        e.call(0x0048_3710, &args![expression]);
        succeeded
    })
}

/// Formats a message into a temporary `BSString` (`004037b0`, `00406f60`)
/// and prints it on the console while a console command runs, or logs it
/// (`00464f30` of its text), then frees the string (`004037d0`).
fn report_through_string(e: &mut Engine, words: &[u32]) {
    e.with_stack(8, |e, string| {
        e.call(0x0040_37b0, &args![string]);
        let mut all = vec![string.addr()];
        all.extend_from_slice(words);
        e.call(0x0040_6f60, &all);
        let printer = if fn_005e23f0(e) {
            0x0070_3c00
        } else {
            0x005b_5e40
        };
        let characters = e.call(0x0055_9450, &args![string]).u32();
        let text = e.call(0x0046_4f30, &args![characters, 0u32]).u32();
        e.call(printer, &args![text]);
        e.call(0x0040_37d0, &args![string]);
    });
}

/// A command line of [`runner_evaluate_line`]: its definition
/// (`005b1120`; unknown is reported), the needs-a-reference byte (+0x10),
/// the parameters (+0x14). `parse_only` parses the parameters
/// (`005accb0`) when the line has any, and for opcode 0x1059 the counted
/// operands after them (`005ac7a0`, parse mode) and four more bytes.
/// Otherwise the handler (+0x18) runs with the eight command words; the
/// "references changed" byte (+0x25) is copied to the runner first. A
/// missing or failing handler is reported.
fn evaluate_command(e: &mut Engine, at: &Line, command: i32) -> bool {
    let definition = e.call(0x005b_1120, &args![command]).u32();
    if definition == 0 {
        let name = e.vcall(at.script.addr(), 0x130, &args![]).u32();
        report_through_string(e, &[0x0103_d618, command as u32, name]);
        return false;
    }
    let parameters = e.mem.u32(definition + 0x14);
    if e.mem.u8(definition + 0x10) != 0 && at.reference.is_null() && !at.parse_only {
        let command_name = e.mem.u32(definition);
        let name = e.vcall(at.script.addr(), 0x130, &args![]).u32();
        report_through_string(e, &[0x0103_d5d0, name, at.line, command_name]);
        return false;
    }
    let data = at.data;
    e.with_stack(16, |e, work| {
        let cursor = work;
        let result = work.byte_add(8);
        e.mem.set_u32(cursor.addr(), at.position);
        let container = e.mem.u32(at.this.addr());
        let locals = e.mem.u32(at.this.addr() + 8);
        if at.parse_only {
            if parameters != 0 && e.mem.u32(at.length.addr()) != 0 {
                e.call(
                    0x005a_ccb0,
                    &args![
                        parameters,
                        data,
                        cursor,
                        at.reference,
                        container,
                        at.script,
                        locals
                    ],
                );
                if command == 0x1059 {
                    let at_count = e.mem.u32(cursor.addr());
                    let count = e.mem.i16(data.wrapping_add(at_count));
                    e.mem.set_u32(cursor.addr(), at_count.wrapping_add(2));
                    let mut operand = 0i16;
                    while operand < count {
                        let container = e.mem.u32(at.this.addr());
                        let locals = e.mem.u32(at.this.addr() + 8);
                        e.call(
                            0x005a_c7a0,
                            &args![
                                result,
                                data,
                                cursor,
                                at.reference,
                                container,
                                at.script,
                                locals,
                                1u32
                            ],
                        );
                        operand = operand.wrapping_add(1);
                    }
                    let after = e.mem.u32(cursor.addr()).wrapping_add(4);
                    e.mem.set_u32(cursor.addr(), after);
                }
            }
            return true;
        }
        let handler = e.mem.u32(definition + 0x18);
        if e.mem.u8(definition + 0x25) != 0 {
            e.mem.set_u8(at.this.addr() + 0xa1, 1);
        }
        let succeeded = handler != 0
            && e.call(
                handler,
                &args![
                    parameters,
                    data,
                    at.reference,
                    container,
                    at.script,
                    locals,
                    result,
                    cursor
                ],
            )
            .bool();
        if !succeeded {
            let form_id = e.call(0x0084_e3a0, &args![at.script]).u32();
            let name = e.vcall(at.script.addr(), 0x130, &args![]).u32();
            let command_name = e.mem.u32(definition);
            e.call(
                0x005b_5e40,
                &args![0x0103_d588u32, command_name, at.line, name, form_id],
            );
            return false;
        }
        true
    })
}

// Translated from 005e24d0 (PC disassembly, FalloutNV.exe 1.4.0.525).
// Translation incomplete: 005e24d0 - compiler SEH exception cleanup.
/// `ScriptRunManager::Instance` (Xbox PDB). Sets the guard before construction.
pub fn instance(e: &mut Engine) -> Ptr<ScriptRunManager> {
    if e.mem.u32(INITIALIZED) & 1 == 0 {
        e.mem.set_u32(INITIALIZED, e.mem.u32(INITIALIZED) | 1);
        e.call(0x0096_a2d0, &args![MANAGER]);
        e.call(0x00ec_658f, &args![0x00fc_b1f0u32]);
    }
    Ptr::new(MANAGER)
}

// Translated from 005e2730 (PC disassembly, FalloutNV.exe 1.4.0.525).
// Translation incomplete: 005e2730 - compiler SEH exception cleanup.
/// `ScriptRunManager::InitActionList` (Xbox PDB): uses the cached runner,
/// or a temporary runner if the cached one is busy.
pub fn init_action_list(
    e: &mut Engine,
    this: Ptr<ScriptRunManager>,
    script: Ptr,
    reference: Ptr,
    locals: Ptr,
) -> bool {
    if e.get(this, ScriptRunManager::m_pScriptRunner).is_null() {
        let block = e.call(0x0040_1000, &args![0xa4u32]).u32();
        let runner = if block == 0 {
            Ptr::NULL
        } else {
            e.call(0x005e_0c60, &args![block]).ptr::<ScriptRunner>()
        };
        e.set(this, ScriptRunManager::m_pScriptRunner, runner);
    }
    let runner = e.get(this, ScriptRunManager::m_pScriptRunner);
    if e.call(0x005e_2850, &args![runner]).bool() {
        let block = e.call(0x0040_1000, &args![0xa4u32]).u32();
        let temporary = if block == 0 {
            0
        } else {
            e.call(0x005e_0c60, &args![block]).u32()
        };
        e.call(0x005e_1160, &args![temporary, script, reference, locals]);
        e.call(0x0040_1030, &args![temporary]);
    } else {
        let runner = e.get(this, ScriptRunManager::m_pScriptRunner);
        e.call(0x005e_1160, &args![runner, script, reference, locals]);
    }
    true
}

// Translated from 005e2850 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Tests the full script pointer, returning the result in AL.
pub fn runner_is_busy(e: &mut Engine, this: Ptr<ScriptRunner>) -> bool {
    !e.get(this, ScriptRunner::m_scriptRunning).is_null()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x005e0c60,
            runner_construct(Ptr<ScriptRunner>) -> Ptr<ScriptRunner>
        ),
        entry!(0x005e24d0, instance() -> Ptr<ScriptRunManager>),
        entry!(
            0x005e1160,
            runner_init_action_list(Ptr<ScriptRunner>, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x005e13a0,
            runner_parse_line(Ptr<ScriptRunner>, Ptr, Ptr, Ptr, Ptr, Ptr, bool) -> bool
        ),
        entry!(
            0x005e1550,
            runner_evaluate_line(
                Ptr<ScriptRunner>,
                Ptr,
                i32,
                Ptr,
                Ptr,
                u32,
                Ptr,
                u32,
                bool,
                bool,
            ) -> bool
        ),
        entry!(
            0x005e2730,
            init_action_list(Ptr<ScriptRunManager>, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x005e2850, runner_is_busy(Ptr<ScriptRunner>) -> bool),
        entry!(
            0x005e0d20,
            runner_run(Ptr<ScriptRunner>, Ptr, Ptr, Ptr, Ptr, bool, u8, u8, f32) -> bool
        ),
        entry!(0x005e1140, fn_005e1140(Ptr, f32)),
        entry!(0x005e23f0, fn_005e23f0() -> bool),
        (0x005e2410, runner_report_error_entry as AbiFn),
        entry!(
            0x005e24b0,
            runner_set_error_code(Ptr<ScriptRunner>, u32, u32)
        ),
        entry!(0x005e2550, manager_clear(Ptr<ScriptRunManager>)),
        entry!(
            0x005e2590,
            run(Ptr<ScriptRunManager>, Ptr, Ptr, Ptr, Ptr, u8, u8, u8, f32) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parser_fixture(e: &mut Engine, bytes: &[u8]) -> (Ptr<ScriptRunner>, Ptr, Ptr) {
        let runner = e.new_object::<ScriptRunner>();
        let script = Ptr::new(e.mem.alloc(0x54));
        let code = e.mem.alloc(bytes.len() as u32);
        e.mem.write(code, bytes);
        e.register_double(0x00671d10, move |_, a| {
            assert_eq!(a, [script.addr()]);
            Ret {
                eax: code,
                ..Ret::default()
            }
        });
        let work = Ptr::new(e.mem.alloc(16));
        e.mem.set_u32(work.addr(), 0x12345678);
        e.mem.set_u32(work.addr() + 8, 0x87654321);
        e.mem.set_u32(work.addr() + 12, 0xdeadbeef);
        (runner, script, work)
    }

    fn parse(
        e: &mut Engine,
        runner: Ptr<ScriptRunner>,
        script: Ptr,
        work: Ptr,
        skip: bool,
    ) -> bool {
        runner_parse_line(
            e,
            runner,
            script,
            work,
            work.byte_add(4),
            work.byte_add(8),
            work.byte_add(12),
            skip,
        )
    }

    /// A script whose compiled bytes the real parser reads, with its data
    /// size in a header that `00500940` returns, and an `IsCompiled`-style
    /// check (`004013e0`) that accepts it.
    fn run_fixture(e: &mut Engine, bytes: &[u8], effect: bool) -> (Ptr<ScriptRunner>, Ptr, u32) {
        let (runner, script, _) = parser_fixture(e, bytes);
        let header = e.mem.alloc(0x14);
        e.mem.set_u32(header + 8, bytes.len() as u32);
        e.mem.set_u8(header + 0x11, u8::from(effect));
        e.register_double(0x00500940, move |_, a| {
            assert_eq!(a, [script.addr()]);
            header.into_ret()
        });
        e.register_double(0x004013e0, |_, _| 1u32.into_ret());
        e.register_double(0x00702fc0, |_, _| 0u32.into_ret());
        e.register_double(0x00593490, |_, a| a[0].into_ret());
        e.register_double(0x00483710, |_, _| Ret::default());
        e.register_double(0x007af430, |_, a| (a[0] + 0x100).into_ret());
        e.register_double(0x005a8ea0, |_, _| Ret::default());
        (runner, script, header)
    }

    #[test]
    fn run_executes_lines_honors_skip_and_stops_at_return() {
        let mut e = Engine::new();
        let bytes = [
            0x11, 0, 0, 0, 0x16, 0, 2, 0, 0xaa, 0xbb, 0x19, 0, 0, 0, 0x1e, 0, 0, 0, 0x11, 0, 0, 0,
        ];
        let (runner, script, _) = run_fixture(&mut e, &bytes, false);
        let lines = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let log = lines.clone();
        e.register_double(0x005e1550, move |e, a| {
            assert_eq!(a.len(), 10);
            assert_eq!(&a[..2], &[runner.addr(), script.addr()]);
            assert_eq!(a[3], 0x3333, "target defaults to the reference");
            assert_eq!(&a[8..], &[0, 0]);
            assert_eq!(e.mem.u32(runner.addr()), 0x4444);
            assert_eq!(e.mem.u32(runner.addr() + 4), 0x3433);
            assert_eq!(e.mem.u32(runner.addr() + 8), 0x5555);
            assert_eq!(e.get(runner, ScriptRunner::m_scriptRunning), script);
            log.borrow_mut().push((a[2], a[5], a[7], e.mem.u32(a[4])));
            if a[2] == 0x16 {
                e.mem.set_u32(a[6], 1);
                e.mem.set_u8(runner.addr() + 0xa1, 1);
            }
            1u32.into_ret()
        });
        e.call_log = Some(Vec::new());
        assert!(e
            .call(
                0x005e0d20,
                &args![runner, script, 0x3333u32, 0x5555u32, 0x4444u32, false, 0u8, 0u8, 0.0f32]
            )
            .bool());
        assert_eq!(*lines.borrow(), [(0x11, 4, 1, 0), (0x16, 8, 2, 2)]);
        assert!(e.get(runner, ScriptRunner::m_scriptRunning).is_null());
        let calls: Vec<u32> = e.call_log.take().unwrap().iter().map(|c| c.0).collect();
        // Locals were given: none are created or deleted; the action list runs.
        assert!(!calls.contains(&0x005abf60) && !calls.contains(&0x0041af70));
        assert_eq!(calls.iter().filter(|a| **a == 0x005a8ea0).count(), 1);
    }

    #[test]
    fn run_creates_and_deletes_missing_locals_and_fills_effect_block() {
        for (keep_actions, fail) in [(false, false), (true, false), (false, true)] {
            let mut e = Engine::new();
            let (runner, script, header) = run_fixture(&mut e, &[0x11, 0, 0, 0], true);
            let locals = e.mem.alloc(0x14);
            let block = e.mem.alloc(8);
            e.register_double(0x005abf60, move |_, a| {
                assert_eq!(a, [script.addr()]);
                locals.into_ret()
            });
            e.register_double(0x00401000, move |_, a| {
                assert_eq!(a, [8]);
                block.into_ret()
            });
            e.register_double(0x005e1550, move |_, _| u32::from(!fail).into_ret());
            e.register_double(0x005a8ea0, move |_, a| {
                assert_eq!(a, [locals]);
                Ret::default()
            });
            e.register_double(0x0041af70, move |_, a| {
                assert_eq!(a, [locals, 1]);
                Ret::default()
            });
            e.call_log = Some(Vec::new());
            // A signaling NaN elapsed time is stored quiet (FLD/FSTP dword).
            assert!(!e
                .call(
                    0x005e0d20,
                    &args![
                        runner,
                        script,
                        0u32,
                        0u32,
                        0x4444u32,
                        keep_actions,
                        7u8,
                        9u8,
                        f32::from_bits(0x7f80_0001)
                    ]
                )
                .bool());
            assert_eq!(e.mem.u32(locals + 0x10), block);
            assert_eq!(e.mem.bytes(block, 2), [7, 9]);
            assert_eq!(e.mem.u32(block + 4), 0x7fc0_0001);
            assert_eq!(e.mem.u32(runner.addr() + 4), 0);
            assert_eq!(e.mem.u32(header + 8), 4);
            let calls: Vec<u32> = e.call_log.take().unwrap().iter().map(|c| c.0).collect();
            let actions = calls.iter().position(|a| *a == 0x005a8ea0);
            let delete = calls.iter().position(|a| *a == 0x0041af70).unwrap();
            assert_eq!(actions.is_some(), !keep_actions && !fail);
            if let Some(actions) = actions {
                assert!(actions < delete);
            }
        }
    }

    #[test]
    fn run_parse_failure_clears_data_size_and_skips_nothing_else() {
        let mut e = Engine::new();
        let (runner, script, header) = run_fixture(&mut e, &[0x11, 0, 0, 0], false);
        e.register_double(0x005e13a0, |_, _| 0u32.into_ret());
        e.call_log = Some(Vec::new());
        assert!(!e
            .call(
                0x005e0d20,
                &args![runner, script, 0u32, 0x5555u32, 0u32, false, 0u8, 0u8, 0.0f32]
            )
            .bool());
        assert_eq!(e.mem.u32(header + 8), 0);
        let calls: Vec<u32> = e.call_log.take().unwrap().iter().map(|c| c.0).collect();
        assert!(calls.contains(&0x005a8ea0));
        assert!(!calls.contains(&0x005e1550));
    }

    #[test]
    fn run_rejects_null_invalid_and_busy_without_starting() {
        let mut e = Engine::new();
        let (runner, script, _) = run_fixture(&mut e, &[0x11, 0, 0, 0], false);
        e.mem.set_u8(runner.addr() + 0xa1, 1);
        let words = |script: u32| args![runner, script, 0u32, 0u32, 0u32, false, 0u8, 0u8, 0.0f32];
        assert!(!e.call(0x005e0d20, &words(0)).bool());
        assert_eq!(e.mem.u8(runner.addr() + 0xa1), 0);
        e.register_double(0x004013e0, |_, _| 0x100u32.into_ret());
        e.call_log = Some(Vec::new());
        assert!(!e.call(0x005e0d20, &words(script.addr())).bool());
        assert_eq!(e.call_log.take().unwrap().len(), 3);
        e.register_double(0x004013e0, |_, _| 1u32.into_ret());
        let running = e.mem.alloc(4);
        let table = e.mem.alloc(0x134);
        e.mem.set_u32(script.addr(), table);
        e.mem.set_u32(running, table);
        e.mem.set_u32(table + 0x130, 0x12340000);
        e.register_double(0x12340000, move |_, a| {
            if a[0] == script.addr() {
                0x2222u32
            } else {
                0x3333
            }
            .into_ret()
        });
        e.register_double(0x005b5e40, |_, a| {
            assert_eq!(a, [0x0103d498, 0x3333, 0x2222]);
            Ret::default()
        });
        e.set(runner, ScriptRunner::m_scriptRunning, Ptr::new(running));
        assert!(!e.call(0x005e0d20, &words(script.addr())).bool());
        assert_eq!(e.get(runner, ScriptRunner::m_scriptRunning).addr(), running);
    }

    #[test]
    fn run_profiles_with_cached_frequency_when_debug_text_is_visible() {
        for cached in [0u64, 1_000] {
            let mut e = Engine::new();
            let (runner, script, _) = run_fixture(&mut e, &[], false);
            e.register_double(0x00702fc0, |_, _| 1u32.into_ret());
            e.map(PROFILER_FREQUENCY, 8);
            e.mem.set_u64(PROFILER_FREQUENCY, cached);
            e.register_double(QUERY_PERFORMANCE_FREQUENCY, |e, a| {
                assert_eq!(a, [PROFILER_FREQUENCY]);
                e.mem.set_u64(a[0], 4_000);
                1u32.into_ret()
            });
            let mut tick = 0u64;
            e.register_double(QUERY_PERFORMANCE_COUNTER, move |e, a| {
                tick += 3_000;
                e.mem.set_u64(a[0], tick);
                1u32.into_ret()
            });
            e.register_double(0x005e1140, move |_, a| {
                assert_eq!(a[0], script.addr());
                let expected = if cached == 0 { 0.75f32 } else { 3.0 };
                assert_eq!(f32::from_bits(a[1]), expected);
                Ret::default()
            });
            e.call_log = Some(Vec::new());
            e.call(
                0x005e0d20,
                &args![runner, script, 0u32, 0x5555u32, 0u32, false, 0u8, 0u8, 0.0f32],
            );
            let calls: Vec<u32> = e.call_log.take().unwrap().iter().map(|c| c.0).collect();
            let count = |a| calls.iter().filter(|c| **c == a).count();
            assert_eq!(count(QUERY_PERFORMANCE_FREQUENCY), usize::from(cached == 0));
            assert_eq!(count(QUERY_PERFORMANCE_COUNTER), 2);
            assert_eq!(count(0x005e1140), 1);
        }
    }

    #[test]
    fn profiler_timer_adds_in_single_precision() {
        let mut e = Engine::new();
        let script = e.mem.alloc(0x38);
        e.mem.set_f32(script + 0x34, 1.0);
        e.call(0x005e1140, &args![script, 0.25f32]);
        assert_eq!(e.mem.f32(script + 0x34), 1.25);
        e.mem.set_f32(script + 0x34, 16_777_216.0);
        e.call(0x005e1140, &args![script, 1.0f32]);
        assert_eq!(e.mem.f32(script + 0x34), 16_777_216.0);
    }

    #[test]
    fn console_echo_flag_is_the_tls_byte() {
        let mut e = Engine::new();
        assert!(!e.call(0x005e23f0, &[]).bool());
        let tls = e.tls();
        e.mem.set_u8(tls + 0x268, 2);
        assert!(e.call(0x005e23f0, &[]).bool());
    }

    #[test]
    fn error_report_formats_routes_and_marks_the_runner() {
        for echo in [false, true] {
            let mut e = Engine::new();
            // The block check tracks `with_stack` frames, so the buffer's size can be read.
            e.mem.enable_block_check(false, false);
            let runner = e.new_object::<ScriptRunner>();
            let header = e.mem.alloc(0x14);
            e.mem.set_u32(header + 8, 40);
            e.register_double(0x00500940, move |_, a| {
                assert_eq!(a, [0x2222]);
                header.into_ret()
            });
            let tls = e.tls();
            e.mem.set_u8(tls + 0x268, u8::from(echo));
            let buffer = std::rc::Rc::new(std::cell::Cell::new(0));
            let seen = buffer.clone();
            e.register_double(0x00407000, move |e, a| {
                assert_eq!(a[1], 0x0103d538);
                assert_eq!((e.mem.u32(a[2]), e.mem.u32(a[2] + 4)), (7, 8));
                // The message buffer is the 0x108-byte stack local.
                assert_eq!(e.mem.block_size(a[0]), Some(0x108));
                seen.set(a[0]);
                Ret::default()
            });
            let text = buffer.clone();
            e.register_double(0x00464f30, move |_, a| {
                assert_eq!(a, [text.get(), 0]);
                0x9999u32.into_ret()
            });
            let printer = if echo { 0x00703c00 } else { 0x005b5e40 };
            e.register_double(printer, |_, a| {
                assert_eq!(a, [0x9999]);
                Ret::default()
            });
            e.register_double(if echo { 0x005b5e40 } else { 0x00703c00 }, |_, _| {
                panic!("wrong printer")
            });
            e.call(0x005e2410, &[runner.addr(), 0x2222, 0x0103d538, 7, 8]);
            assert_eq!(e.mem.u32(header + 8), 0);
            assert_eq!(e.mem.u8(runner.addr() + 0xa0), 1);
        }
    }

    #[test]
    fn error_code_setter_ignores_its_first_argument() {
        let mut e = Engine::new();
        let runner = e.new_object::<ScriptRunner>();
        e.call(0x005e24b0, &[runner.addr(), 0x1234, 6]);
        assert_eq!(e.mem.u32(runner.addr() + 0x18), 6);
        assert_eq!(e.mem.u32(runner.addr() + 0x14), 0);
    }

    #[test]
    fn manager_clear_frees_only_a_cached_runner() {
        for cached in [0u32, 0x4444] {
            let mut e = Engine::new();
            let manager = e.new_object::<ScriptRunManager>();
            e.mem.set_u32(manager.addr(), cached);
            e.mem.set_u32(manager.addr() + 4, 0x5555);
            e.register_double(0x00401030, move |_, a| {
                assert_eq!(a, [cached]);
                Ret::default()
            });
            e.call_log = Some(Vec::new());
            e.call(0x005e2550, &[manager.addr()]);
            assert_eq!(e.mem.bytes(manager.addr(), 8), [0; 8]);
            assert_eq!(
                e.call_log.take().unwrap().len(),
                if cached == 0 { 1 } else { 2 }
            );
        }
    }

    fn manager_fixture(e: &mut Engine, depth: i32) -> Ptr<ScriptRunManager> {
        e.map(RUN_DEPTH, 4);
        e.mem.set_i32(RUN_DEPTH, depth);
        // InterlockedIncrement/Decrement through their import slots need
        // the exe; these doubles do what the translations do with it.
        for (address, step) in [(0x0040b460u32, 1i32), (0x004019a0, -1)] {
            e.register_double(address, move |e, a| {
                let value = e.mem.i32(a[0]).wrapping_add(step);
                e.mem.set_i32(a[0], value);
                value.into_ret()
            });
        }
        e.new_object::<ScriptRunManager>()
    }

    #[test]
    fn manager_run_uses_cached_runner_outside_nested_runs() {
        let mut e = Engine::new();
        let manager = manager_fixture(&mut e, 0);
        let runner = e.new_object::<ScriptRunner>();
        e.register_double(0x00401000, move |_, a| {
            assert_eq!(a, [0xa4]);
            runner.into_ret()
        });
        e.register_double(0x005e0d20, move |e, a| {
            assert_eq!(e.mem.i32(RUN_DEPTH), 1);
            assert_eq!(e.get(manager, ScriptRunManager::pCurrentRunner), runner);
            assert_eq!(a, [runner.addr(), 1, 2, 3, 4, 5, 6, 7, 0x7fc0_0002]);
            1u32.into_ret()
        });
        e.call_log = Some(Vec::new());
        let words = [manager.addr(), 1, 2, 3, 4, 0x105, 6, 0x307, 0x7f80_0002];
        assert!(e.call(0x005e2590, &words).bool());
        assert!(e.call(0x005e2590, &words).bool());
        let calls: Vec<u32> = e.call_log.take().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(calls.iter().filter(|a| **a == 0x00401000).count(), 1);
        assert_eq!(e.get(manager, ScriptRunManager::m_pScriptRunner), runner);
        assert!(e.get(manager, ScriptRunManager::pCurrentRunner).is_null());
        assert_eq!(e.mem.i32(RUN_DEPTH), 0);
    }

    #[test]
    fn manager_run_nested_uses_temporary_and_restores_current() {
        let mut e = Engine::new();
        let manager = manager_fixture(&mut e, 1);
        let cached = e.new_object::<ScriptRunner>();
        let temporary = e.new_object::<ScriptRunner>();
        e.set(manager, ScriptRunManager::m_pScriptRunner, cached);
        e.set(manager, ScriptRunManager::pCurrentRunner, cached);
        e.register_double(0x00401000, move |_, _| temporary.into_ret());
        e.register_double(0x005e0d20, move |e, a| {
            assert_eq!(a[0], temporary.addr());
            assert_eq!(e.get(manager, ScriptRunManager::pCurrentRunner), temporary);
            0u32.into_ret()
        });
        e.register_double(0x00401030, move |e, a| {
            assert_eq!(a, [temporary.addr()]);
            assert_eq!(e.get(manager, ScriptRunManager::pCurrentRunner), cached);
            Ret::default()
        });
        assert!(!e
            .call(0x005e2590, &[manager.addr(), 1, 2, 3, 4, 0, 0, 0, 0])
            .bool());
        assert_eq!(e.get(manager, ScriptRunManager::m_pScriptRunner), cached);
        assert_eq!(e.mem.i32(RUN_DEPTH), 1);
    }

    /// A runner (container +0 = 0x4444, locals +8 = 0x5555) and a script
    /// whose compiled bytes are `bytes`, named 0x6666 through slot 0x130,
    /// form id 0x8888; the expression evaluator answers `values` in turn
    /// (an `Err(code)` sets that expression error and answers 0).
    fn line_fixture(
        e: &mut Engine,
        bytes: &[u8],
        values: Vec<Result<f64, u32>>,
    ) -> (Ptr<ScriptRunner>, Ptr, Ptr, Ptr) {
        let (runner, script, _) = parser_fixture(e, bytes);
        e.mem.set_u32(runner.addr(), 0x4444);
        e.mem.set_u32(runner.addr() + 8, 0x5555);
        let table = e.mem.alloc(0x134);
        e.mem.set_u32(script.addr(), table);
        e.mem.set_u32(table + 0x130, 0x12340000);
        e.register_double(0x12340000, |_, _| 0x6666u32.into_ret());
        e.register_double(0x0084e3a0, |_, _| 0x8888u32.into_ret());
        let header = e.mem.alloc(0x14);
        e.mem.set_u32(header + 8, 99);
        e.register_double(0x00500940, move |_, _| header.into_ret());
        e.map(CONDITION_FALSE, 8);
        let mut values = values.into_iter();
        e.register_double(0x00593db0, move |e, a| {
            assert_eq!(a.len(), 8);
            assert_eq!(&a[3..6], &[0x4444, script.addr(), 0x5555]);
            match values.next().expect("unexpected expression") {
                Ok(value) => Ret {
                    st0: value,
                    ..Ret::default()
                },
                Err(code) => {
                    e.mem.set_u32(a[0], code);
                    Ret::default()
                }
            }
        });
        let length = Ptr::new(e.mem.alloc(4));
        let skip = Ptr::new(e.mem.alloc(4));
        (runner, script, length, skip)
    }

    fn evaluate(
        e: &mut Engine,
        (runner, script, length, skip): (Ptr<ScriptRunner>, Ptr, Ptr, Ptr),
        command: i32,
        reference: u32,
        parse: bool,
    ) -> bool {
        runner_evaluate_line(
            e,
            runner,
            script,
            command,
            Ptr::new(reference),
            length,
            0,
            skip,
            5,
            parse,
            false,
        )
    }

    #[test]
    fn set_writes_float_and_truncated_integer_variables() {
        for (kind, value, stored) in [(b'f', 2.75, 2.75), (b's', 2.75, 2.0), (b'l', -2.75, -2.0)] {
            let mut e = Engine::new();
            let bytes = [kind, 3, 0, 4, 0, b' ', b'n', 0, 0];
            let fixture = line_fixture(&mut e, &bytes, vec![Ok(value)]);
            e.register_double(0x005a9290, move |_, a| {
                assert_eq!(a[..2], [0x5555, 3]);
                assert_eq!(f64::from_bits(a[2] as u64 | (a[3] as u64) << 32), stored);
                Ret::default()
            });
            e.call_log = Some(Vec::new());
            assert!(evaluate(&mut e, fixture, 0x15, 0, false));
            let calls: Vec<u32> = e.call_log.take().unwrap().iter().map(|c| c.0).collect();
            assert!(calls.contains(&0x005a9290));
            // No reference and no object: nothing is marked changed.
            assert!(!calls.contains(&0x004077c0));
        }
    }

    #[test]
    fn set_on_a_reference_variable_uses_its_locals_and_marks_it_changed() {
        for process_keeps in [false, true] {
            let mut e = Engine::new();
            let bytes = [b'r', 2, 0, b'f', 7, 0, 1, 0, b'1'];
            let fixture = line_fixture(&mut e, &bytes, vec![Ok(1.5)]);
            let object = e.mem.alloc(16);
            let form = e.mem.alloc(4);
            let table = e.mem.alloc(0xf4);
            e.mem.set_u32(form, table);
            e.mem.set_u32(table + 0x48, 0x12350000);
            e.mem.set_u32(table + 0xf0, 0x12350001);
            e.mem.set_u32(object + 8, form);
            e.register_double(0x005ac4f0, move |_, a| {
                assert_eq!(a, [fixture.1.addr(), 2, 0x5555]);
                object.into_ret()
            });
            e.register_double(0x00ec43fb, move |_, a| {
                assert_eq!(a, [form, 0, 0x01183028, 0x011841cc, 0]);
                form.into_ret()
            });
            e.register_double(0x005673e0, move |_, a| {
                assert_eq!(a, [form]);
                0x7777u32.into_ret()
            });
            e.register_double(0x005a9290, |_, a| {
                assert_eq!(a[..2], [0x7777, 7]);
                Ret::default()
            });
            e.register_double(0x12350001, |_, _| 1u32.into_ret());
            e.register_double(0x004077c0, |_, _| 0u32.into_ret());
            e.register_double(0x008d6f30, |_, _| 0x9999u32.into_ret());
            e.register_double(0x00450ff0, move |_, a| {
                assert_eq!(a, [0x9999]);
                u32::from(process_keeps).into_ret()
            });
            let marked = std::rc::Rc::new(std::cell::Cell::new(false));
            let mark = marked.clone();
            e.register_double(0x12350000, move |_, a| {
                assert_eq!(a, [form, 0x8000_0000]);
                mark.set(true);
                Ret::default()
            });
            assert!(evaluate(&mut e, fixture, 0x15, 0x3333, false));
            assert_eq!(marked.get(), !process_keeps);
        }
    }

    #[test]
    fn set_global_stores_a_single_precision_value() {
        let mut e = Engine::new();
        let bytes = [b'G', 1, 0, 1, 0, b'1'];
        let fixture = line_fixture(&mut e, &bytes, vec![Ok(0.1)]);
        let object = e.mem.alloc(16);
        e.mem.set_u32(object + 8, 0x2468);
        e.register_double(0x005ac4f0, move |_, _| object.into_ret());
        e.register_double(0x00ec43fb, |_, _| 0u32.into_ret());
        e.register_double(0x0046dce0, |_, a| {
            assert_eq!(a, [0x2468, 0.1f32.to_bits()]);
            Ret::default()
        });
        e.call_log = Some(Vec::new());
        assert!(evaluate(&mut e, fixture, 0x15, 0, false));
        let calls: Vec<u32> = e.call_log.take().unwrap().iter().map(|c| c.0).collect();
        assert!(calls.contains(&0x0046dce0));
        assert!(!calls.contains(&0x005a9290));
    }

    #[test]
    fn set_failures_report_and_stop() {
        // A missing reference object.
        let mut e = Engine::new();
        let fixture = line_fixture(&mut e, &[b'r', 2, 0, b'f', 7, 0, 1, 0, b'1'], vec![]);
        e.register_double(0x005ac4f0, |_, _| 0u32.into_ret());
        e.register_double(0x005b5e40, |_, a| {
            assert_eq!(a, [0x0103d7b8, 5, 0x6666, 0x8888]);
            Ret::default()
        });
        assert!(!evaluate(&mut e, fixture, 0x15, 0, false));
        // In parse mode the object is not checked and nothing is set.
        let mut e = Engine::new();
        let fixture = line_fixture(&mut e, &[b'r', 2, 0, b'f', 7, 0, 1, 0, b'1'], vec![Ok(3.0)]);
        e.register_double(0x005ac4f0, |_, _| 0u32.into_ret());
        assert!(evaluate(&mut e, fixture, 0x15, 0, true));
        // An expression error clears the script's data size.
        let mut e = Engine::new();
        let fixture = line_fixture(&mut e, &[b'f', 1, 0, 1, 0, b'1'], vec![Err(4)]);
        e.map(EXPRESSION_ERROR_NAMES, 0x18);
        e.mem.set_u32(EXPRESSION_ERROR_NAMES + 16, 0xabcd);
        e.register_double(0x005b5e40, |_, a| {
            assert_eq!(a, [0x0103d770, 0x6666, 5, 0xabcd]);
            Ret::default()
        });
        let header = e.call(0x00500940, &[fixture.1.addr()]).u32();
        assert!(!evaluate(&mut e, fixture, 0x15, 0, false));
        assert_eq!(e.mem.u32(header + 8), 0);
    }

    #[test]
    fn if_elseif_else_endif_track_taken_branches_and_skip_lines() {
        let mut e = Engine::new();
        // Each condition line: jump count 3, expression length 1.
        let fixture = line_fixture(&mut e, &[3, 0, 1, 0, b'1'], vec![Ok(0.0), Ok(2.0)]);
        let (runner, _, _, skip) = fixture;
        let level = |e: &Engine, depth: u32| e.mem.u32(runner.addr() + 0x24 + 4 * depth);
        // if 0: not taken, three lines skipped.
        assert!(evaluate(&mut e, fixture, 0x16, 0x3333, false));
        assert_eq!((e.mem.u32(runner.addr() + 0x20), level(&e, 0)), (0, 1));
        assert_eq!(e.mem.u32(skip.addr()), 3);
        // elseif 2: taken.
        assert!(evaluate(&mut e, fixture, 0x18, 0x3333, false));
        assert_eq!(level(&e, 0), 3);
        assert_eq!(e.mem.u32(skip.addr()), 3);
        // A later elseif is skipped without evaluating, else too.
        assert!(evaluate(&mut e, fixture, 0x18, 0x3333, false));
        assert_eq!(e.mem.u32(skip.addr()), 6);
        assert!(evaluate(&mut e, fixture, 0x17, 0x3333, false));
        assert_eq!(e.mem.u32(skip.addr()), 9);
        // A nested if opens level 1 because level 0 is in use.
        e.register_double(0x00593db0, |_, _| Ret {
            st0: -1.0,
            ..Ret::default()
        });
        assert!(evaluate(&mut e, fixture, 0x16, 0x3333, false));
        assert_eq!((e.mem.u32(runner.addr() + 0x20), level(&e, 1)), (1, 3));
        assert!(evaluate(&mut e, fixture, 0x19, 0x3333, false));
        assert_eq!((e.mem.u32(runner.addr() + 0x20), level(&e, 1)), (0, 0));
        assert!(evaluate(&mut e, fixture, 0x19, 0x3333, false));
        assert_eq!((e.mem.u32(runner.addr() + 0x20), level(&e, 0)), (0, 0));
        // else on a level without a taken branch takes it.
        assert!(evaluate(&mut e, fixture, 0x17, 0x3333, false));
        assert_eq!(level(&e, 0), 2);
        assert_eq!(e.mem.u32(skip.addr()), 9);
    }

    #[test]
    fn if_parse_mode_marks_taken_and_expression_errors_fail() {
        let mut e = Engine::new();
        let fixture = line_fixture(&mut e, &[3, 0, 1, 0, b'1'], vec![Ok(0.0)]);
        assert!(evaluate(&mut e, fixture, 0x16, 0x3333, true));
        assert_eq!(e.mem.u32(fixture.0.addr() + 0x24), 3);
        assert_eq!(e.mem.u32(fixture.3.addr()), 0);

        let mut e = Engine::new();
        let fixture = line_fixture(&mut e, &[3, 0, 1, 0, b'1'], vec![Err(5)]);
        e.register_double(0x005b5e40, |_, a| {
            assert_eq!(a, [0x0103d664, 0x6666, 5]);
            Ret::default()
        });
        let header = e.call(0x00500940, &[fixture.1.addr()]).u32();
        assert!(!evaluate(&mut e, fixture, 0x16, 0x3333, false));
        assert_eq!(e.mem.u32(fixture.0.addr() + 0x18), 6);
        assert_eq!(e.mem.u32(fixture.0.addr() + 0x1c), 5);
        assert_eq!(e.mem.u32(header + 8), 0);
        // The error leaves the line skipped (the value read was 0).
        assert_eq!(e.mem.u32(fixture.3.addr()), 3);
    }

    /// A command definition at its table address with the given handler.
    fn define(e: &mut Engine, opcode: u32, needs_reference: bool, changes: bool, handler: u32) {
        let entry = 0x0119_0910 + (opcode - 0x1000) * 0x28;
        e.map(entry, 0x28);
        e.mem.set_u32(entry, 0xc0de);
        e.mem.set_u8(entry + 0x10, u8::from(needs_reference));
        e.mem.set_u32(entry + 0x14, 0x7777);
        e.mem.set_u32(entry + 0x18, handler);
        e.mem.set_u8(entry + 0x25, u8::from(changes));
    }

    #[test]
    fn command_runs_its_handler_with_the_eight_command_words() {
        for (succeeds, changes) in [(true, false), (true, true), (false, false)] {
            let mut e = Engine::new();
            let fixture = line_fixture(&mut e, &[1, 2, 3, 4], vec![]);
            define(&mut e, 0x1021, false, changes, 0x12360000);
            let (runner, script, _, _) = fixture;
            e.register_double(0x12360000, move |e, a| {
                assert_eq!(a.len(), 8);
                assert_eq!(
                    [a[0], a[2], a[3], a[4], a[5]],
                    [0x7777, 0x3333, 0x4444, script.addr(), 0x5555]
                );
                assert_eq!(e.mem.u32(a[7]), 0);
                assert_eq!(e.mem.u8(runner.addr() + 0xa1), u8::from(changes));
                u32::from(succeeds).into_ret()
            });
            e.register_double(0x005b5e40, |_, a| {
                assert_eq!(a, [0x0103d588, 0xc0de, 5, 0x6666, 0x8888]);
                Ret::default()
            });
            assert_eq!(evaluate(&mut e, fixture, 0x1021, 0x3333, false), succeeds);
        }
    }

    #[test]
    fn command_reports_unknown_opcodes_and_missing_references() {
        for (opcode, reference, format) in [
            (0x0012, 0x3333, 0x0103d618u32),
            (0x001e, 0x3333, 0x0103d618),
            (0x1021, 0, 0x0103d5d0),
        ] {
            let mut e = Engine::new();
            let fixture = line_fixture(&mut e, &[1, 2, 3, 4], vec![]);
            define(&mut e, 0x1021, true, false, 0x12360000);
            e.register_double(0x004037b0, |e, a| {
                e.mem.set_u32(a[0], 0x5151);
                Ret::default()
            });
            e.register_double(0x00406f60, move |_, a| {
                let expected: &[u32] = if format == 0x0103d618 {
                    &[format, opcode, 0x6666]
                } else {
                    &[format, 0x6666, 5, 0xc0de]
                };
                assert_eq!(&a[1..], expected);
                Ret::default()
            });
            e.register_double(0x00464f30, |_, a| {
                assert_eq!(a, [0x5151, 0]);
                0x5252u32.into_ret()
            });
            e.register_double(0x005b5e40, |_, a| {
                assert_eq!(a, [0x5252]);
                Ret::default()
            });
            e.register_double(0x004037d0, |_, _| Ret::default());
            assert!(!evaluate(&mut e, fixture, opcode as i32, reference, false));
        }
    }

    #[test]
    fn command_parse_mode_reads_parameters_and_counted_operands() {
        for (opcode, length) in [(0x1021u32, 4u32), (0x1059, 4), (0x1059, 0)] {
            let mut e = Engine::new();
            // Parameters (doubled: they take two bytes), then for 0x1059 a
            // count of 2 and two operands (doubled: two bytes each).
            let fixture = line_fixture(&mut e, &[9, 9, 2, 0, 1, 1, 1, 1], vec![]);
            define(&mut e, opcode, true, false, 0);
            e.mem.set_u32(fixture.2.addr(), length);
            e.register_double(0x005accb0, move |e, a| {
                assert_eq!(a.len(), 7);
                assert_eq!(a[0], 0x7777);
                assert_eq!([a[3], a[4], a[6]], [0, 0x4444, 0x5555]);
                e.mem.set_u32(a[2], e.mem.u32(a[2]) + 2);
                Ret::default()
            });
            let operands = std::rc::Rc::new(std::cell::Cell::new(0));
            let count = operands.clone();
            e.register_double(0x005ac7a0, move |e, a| {
                assert_eq!(a[7], 1);
                e.mem.set_u32(a[2], e.mem.u32(a[2]) + 2);
                count.set(count.get() + 1);
                Ret::default()
            });
            e.call_log = Some(Vec::new());
            assert!(evaluate(&mut e, fixture, opcode as i32, 0, true));
            let parsed = e
                .call_log
                .take()
                .unwrap()
                .iter()
                .filter(|c| c.0 == 0x005accb0)
                .count();
            assert_eq!(parsed, usize::from(length != 0));
            assert_eq!(
                operands.get(),
                if opcode == 0x1059 && length != 0 {
                    2
                } else {
                    0
                }
            );
        }
    }

    #[test]
    fn structural_lines_need_no_bytes() {
        for command in [0x11, 0x1c, 0x1d] {
            let mut e = Engine::new();
            let fixture = line_fixture(&mut e, &[], vec![]);
            e.call_log = Some(Vec::new());
            assert!(evaluate(&mut e, fixture, command, 0, false));
            assert_eq!(e.call_log.take().unwrap().len(), 1);
        }
        let mut e = Engine::new();
        let fixture = line_fixture(&mut e, &[], vec![]);
        assert!(evaluate(&mut e, fixture, 0x1e, 0, true));
    }

    /// The interpreter's trunk (S0 of the script track): every function on
    /// the path from `Script::Run` to a command handler, with whether this
    /// crate translates it completely or only up to compiler-generated
    /// exception/cookie code. An entry here that is not registered fails.
    const INTERPRETER_TRUNK: &[(u32, &str, &str)] = &[
        (0x005ac1e0, "Script::Run", "complete"),
        (0x005e24d0, "ScriptRunManager::Instance", "partial: SEH"),
        (0x005e2590, "ScriptRunManager::Run", "partial: SEH"),
        (0x005e0c60, "ScriptRunner::ScriptRunner", "complete"),
        (
            0x005e0d20,
            "ScriptRunner::Run (main line loop)",
            "partial: SEH, cookie",
        ),
        (0x005e13a0, "ScriptRunner::ParseLine", "complete"),
        (0x005e1550, "ScriptRunner::EvaluateLine", "partial"),
        (0x005e1140, "Script profiler timer add", "complete"),
        (0x005e23f0, "console echo flag", "complete"),
        (0x005e2410, "runner error report", "partial: cookie"),
        (0x005e24b0, "runner error code", "complete"),
        (0x005e2550, "run manager clear", "complete"),
        (0x005b1120, "ScriptCompiler::GetFunctionDef", "complete"),
        (0x005accb0, "Script::ParseParameters", "complete"),
        (0x005ac4f0, "Script::GetReferencedObject", "complete"),
        (
            0x005ac7a0,
            "script operand / function in expression",
            "complete",
        ),
        (0x00593490, "CExpression::CExpression", "complete"),
        (0x0060d4d0, "quest script run", "complete"),
        (0x0060d700, "TESQuest::GetCurrentStage", "complete"),
        (0x0084d030, "elapsed-time getter", "complete"),
        (0x00792760, "quest delay getter", "complete"),
        (0x00526ac0, "global value getter", "complete"),
    ];

    #[test]
    fn interpreter_trunk_is_registered() {
        let e = Engine::new();
        let missing: Vec<String> = INTERPRETER_TRUNK
            .iter()
            .filter(|(address, _, _)| !e.is_translated(*address))
            .map(|(address, name, _)| format!("{address:08x} {name}"))
            .collect();
        assert!(
            missing.is_empty(),
            "untranslated trunk functions: {missing:?}"
        );
    }

    #[test]
    fn evaluator_action_mode_skips_non_begin_opcodes_after_fetching_data() {
        for command in [0x11i32, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1d, 0x1059, -1] {
            let mut e = Engine::new();
            e.register_double(0x00671d10, |_, a| {
                assert_eq!(a, [0x2222]);
                Ret::default()
            });
            assert!(e
                .call(
                    0x005e1550,
                    &args![
                        0x1111u32,
                        0x2222u32,
                        command,
                        0u32,
                        0u32,
                        0xfffffff0u32,
                        0u32,
                        7u32,
                        false,
                        true
                    ]
                )
                .bool());
        }
    }

    fn begin_fixture(e: &mut Engine, kind: i16, span: u32) -> (Ptr<ScriptRunner>, Ptr, Ptr) {
        let mut bytes = Vec::from(kind.to_le_bytes());
        bytes.extend(span.to_le_bytes());
        let (runner, script, work) = parser_fixture(e, &bytes);
        e.mem.set_u32(runner.addr(), 0x4444);
        e.mem.set_u32(runner.addr() + 8, 0x5555);
        e.mem.set_u32(work.addr(), 11);
        let table = e.mem.alloc(0x134);
        e.mem.set_u32(script.addr(), table);
        e.mem.set_u32(table + 0x130, 0x12340000);
        e.register_double(0x12340000, |_, _| Ret {
            eax: 0x6666,
            ..Ret::default()
        });
        if (0..0x26).contains(&kind) {
            let entry = 0x0118e300 + kind as u32 * 0x28;
            e.map(entry, 12);
            e.mem.set_u8(entry, 0);
            e.mem.set_u32(entry + 4, 0x7777);
            e.mem.set_u32(entry + 8, 0x12340001);
        }
        (runner, script, work)
    }

    #[test]
    fn evaluator_begin_uses_eight_callback_arguments_and_live_length() {
        for result in [0.0, -0.0, 1.0, -1.0, f64::NAN] {
            for action_list in [false, true] {
                let mut e = Engine::new();
                let (runner, script, length) = begin_fixture(&mut e, 3, 173);
                e.register_double(0x12340001, move |e, a| {
                    assert_eq!(a.len(), 8);
                    assert_eq!(a[0], 0x7777);
                    assert_eq!(&a[2..6], &[0x3333, 0x4444, script.addr(), 0x5555]);
                    assert_eq!(e.mem.i16(a[1]), 3);
                    assert_eq!(e.mem.u32(a[7]), 6);
                    e.mem.set_f64(a[6], result);
                    e.mem.set_u32(length.addr(), u32::MAX - 1);
                    Ret {
                        eax: 1,
                        ..Ret::default()
                    }
                });
                assert!(runner_evaluate_line(
                    &mut e,
                    runner,
                    script,
                    16,
                    Ptr::new(0x3333),
                    length,
                    0,
                    Ptr::NULL,
                    2,
                    false,
                    action_list
                ));
                assert_eq!(
                    e.mem.u32(length.addr()),
                    if result == 0.0 || action_list {
                        171
                    } else {
                        u32::MAX - 1
                    }
                );
            }
        }
    }

    #[test]
    fn evaluator_begin_invalid_kind_and_required_reference_have_distinct_errors() {
        for kind in [-1, 38, 3] {
            let mut e = Engine::new();
            let (runner, script, length) = begin_fixture(&mut e, kind, 173);
            e.register_double(0x0084e3a0, move |_, a| {
                assert_eq!(a, [script.addr()]);
                Ret {
                    eax: 0x8888,
                    ..Ret::default()
                }
            });
            if kind == 3 {
                e.mem.set_u8(0x0118e300 + 3 * 0x28, 1);
            }
            e.register_double(0x005b5e40, move |_, a| {
                if kind == 3 {
                    assert_eq!(a, [0x0103d890, 0x6666, 2]);
                } else {
                    assert_eq!(a, [0x0103d8d0, 2, 0x6666, 0x8888]);
                }
                Ret::default()
            });
            assert!(!runner_evaluate_line(
                &mut e,
                runner,
                script,
                16,
                Ptr::NULL,
                length,
                0,
                Ptr::NULL,
                2,
                false,
                true
            ));
            assert_eq!(e.mem.u32(length.addr()), 11);
        }
    }

    #[test]
    fn evaluator_begin_failure_reports_cursor_before_script_name_callback() {
        for handler_present in [false, true] {
            let mut e = Engine::new();
            let (runner, script, length) = begin_fixture(&mut e, 3, 173);
            let cursor_slot = std::rc::Rc::new(std::cell::Cell::new(0));
            let slot = cursor_slot.clone();
            if handler_present {
                e.register_double(0x12340001, move |e, a| {
                    e.mem.set_u32(a[7], 19);
                    slot.set(a[7]);
                    Ret::default()
                });
            } else {
                e.mem.set_u32(0x0118e300 + 3 * 0x28 + 8, 0);
            }
            e.register_double(0x12340000, move |e, _| {
                if cursor_slot.get() != 0 {
                    e.mem.set_u32(cursor_slot.get(), 99);
                }
                Ret {
                    eax: 0x6666,
                    ..Ret::default()
                }
            });
            e.register_double(0x005b5e40, move |_, a| {
                assert_eq!(
                    a,
                    [0x0103d848, 0x6666, 2, if handler_present { 19 } else { 6 }]
                );
                Ret::default()
            });
            assert!(!runner_evaluate_line(
                &mut e,
                runner,
                script,
                16,
                Ptr::NULL,
                length,
                0,
                Ptr::NULL,
                2,
                false,
                true
            ));
        }
    }

    #[test]
    fn evaluator_begin_parse_mode_ignores_result_and_requires_parameters_and_length() {
        for (parameters, bytes) in [(0, 11), (0x7777, 0), (0x7777, 11)] {
            let mut e = Engine::new();
            let (runner, script, length) = begin_fixture(&mut e, 3, 173);
            e.mem.set_u8(0x0118e300 + 3 * 0x28, 1);
            e.mem.set_u32(0x0118e300 + 3 * 0x28 + 4, parameters);
            e.mem.set_u32(length.addr(), bytes);
            let calls = std::rc::Rc::new(std::cell::Cell::new(0));
            let count = calls.clone();
            e.register_double(0x005accb0, move |e, a| {
                assert_eq!(a.len(), 7);
                assert_eq!(a[0], parameters);
                assert_eq!(e.mem.u32(a[2]), 6);
                assert_eq!(&a[3..], &[0, 0x4444, script.addr(), 0x5555]);
                count.set(count.get() + 1);
                Ret::default()
            });
            assert!(runner_evaluate_line(
                &mut e,
                runner,
                script,
                16,
                Ptr::NULL,
                length,
                0,
                Ptr::NULL,
                2,
                true,
                true
            ));
            assert_eq!(calls.get(), i32::from(parameters != 0 && bytes != 0));
            assert_eq!(e.mem.u32(length.addr()), bytes);
        }
    }

    #[test]
    fn action_list_rejects_null_invalid_and_busy_scripts_before_construction() {
        let mut e = Engine::new();
        e.call_log = Some(Vec::new());
        let runner = e.new_object::<ScriptRunner>();
        let script = Ptr::new(e.mem.alloc(0x54));
        assert!(!runner_init_action_list(
            &mut e,
            runner,
            Ptr::NULL,
            Ptr::NULL,
            Ptr::NULL
        ));
        assert!(e.call_log.as_ref().unwrap().is_empty());
        e.register_double(0x004013e0, |_, _| Ret::default());
        assert!(!runner_init_action_list(
            &mut e,
            runner,
            script,
            Ptr::NULL,
            Ptr::NULL
        ));
        assert!(e.get(runner, ScriptRunner::m_scriptRunning).is_null());
        e.register_double(0x004013e0, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        let replacement = e.mem.alloc(4);
        let table = e.mem.alloc(0x134);
        e.mem.set_u32(script.addr(), table);
        e.mem.set_u32(replacement, table);
        e.mem.set_u32(table + 0x130, 0x12340000);
        e.set(runner, ScriptRunner::m_scriptRunning, Ptr::new(0x1111));
        e.register_double(0x12340000, move |e, a| {
            if a[0] == script.addr() {
                e.set(runner, ScriptRunner::m_scriptRunning, Ptr::new(replacement));
                Ret {
                    eax: 0x2222,
                    ..Ret::default()
                }
            } else {
                assert_eq!(a, [replacement]);
                Ret {
                    eax: 0x3333,
                    ..Ret::default()
                }
            }
        });
        e.register_double(0x005b5e40, |_, a| {
            assert_eq!(a, [0x0103d498, 0x3333, 0x2222]);
            Ret::default()
        });
        assert!(!runner_init_action_list(
            &mut e,
            runner,
            script,
            Ptr::NULL,
            Ptr::NULL
        ));
        assert_eq!(
            e.get(runner, ScriptRunner::m_scriptRunning).addr(),
            replacement
        );
    }

    #[test]
    fn action_list_empty_script_matches_native_mutations_and_preserves_context() {
        // Native 005e1160: AL=1, RET 12; only +4/+8 change in these
        // generated runner/script buffers. Null and disabled return AL=0.
        for locals in [0u32, 0x12345678] {
            let mut e = Engine::new();
            let block = e.mem.alloc(0x100);
            let mut expected = vec![0xa5; 0x100];
            expected[0x14..0x18].fill(0);
            expected[0xc4..0xc8].fill(0);
            expected[0xac..0xb0].copy_from_slice(&8u32.to_le_bytes());
            e.mem.write(block, &expected);
            expected[4..8].fill(0);
            expected[8..12].copy_from_slice(&locals.to_le_bytes());
            e.register_double(0x004013e0, |e, a| Ret {
                eax: u32::from(e.mem.u32(a[0] + 8) & 8 != 0),
                ..Ret::default()
            });
            e.register_double(0x00500940, |_, a| Ret {
                eax: a[0] + 0x18,
                ..Ret::default()
            });
            e.register_double(0x00593490, |_, a| Ret {
                eax: a[0],
                ..Ret::default()
            });
            e.register_double(0x00483710, |_, _| Ret::default());
            assert!(e.call(0x005e1160, &[block, block + 0xa4, 0, locals]).bool());
            assert_eq!(e.mem.bytes(block, 0x100), expected);
        }
    }

    #[test]
    fn action_list_parses_three_lines_and_honors_evaluator_skip_and_length() {
        let mut e = Engine::new();
        // First length is replaced by EvaluateLine; second is skipped without
        // resolving its reference prefix; third clears that stale reference.
        let (runner, script, _) = parser_fixture(
            &mut e,
            &[
                16, 0, 99, 0, 0xaa, 0xbb, 28, 0, 7, 0, 17, 0, 0, 0, 18, 0, 0, 0,
            ],
        );
        let header = e.mem.alloc(12);
        e.mem.set_u32(header + 8, 18);
        e.register_double(0x004013e0, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x00593490, move |e, a| {
            assert_eq!(e.get(runner, ScriptRunner::m_scriptRunning), script);
            e.mem.set_u32(a[0] + 0x70c, 0x12345678);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register_double(0x007af430, |_, a| {
            assert_eq!(a, [0x4444]);
            Ret {
                eax: 0x5555,
                ..Ret::default()
            }
        });
        e.register_double(0x00500940, move |_, a| {
            assert_eq!(a, [script.addr()]);
            Ret {
                eax: header,
                ..Ret::default()
            }
        });
        let evaluations = std::rc::Rc::new(std::cell::Cell::new(0));
        let count = evaluations.clone();
        e.register_double(0x005e1550, move |e, a| {
            assert_eq!(a.len(), 10);
            assert_eq!(&a[..2], &[runner.addr(), script.addr()]);
            assert_eq!(a[3], 0x4444);
            assert_eq!(&a[8..], &[0, 1]);
            assert_eq!(e.mem.u32(runner.addr() + 4), 0x5555);
            assert_eq!(e.mem.u32(runner.addr() + 8), 0x6666);
            assert_eq!(e.mem.u32(runner.addr() + 0x10), 0);
            if count.get() == 0 {
                assert_eq!((a[2], a[5], a[7]), (16, 4, 1));
                assert_eq!(e.mem.u32(a[4]), 99);
                e.mem.set_u32(a[4], 2);
                e.mem.set_u32(a[6], 1);
                e.mem.set_u32(runner.addr() + 0x10, 0x7777);
            } else {
                assert_eq!((a[2], a[5], a[7]), (18, 18, 3));
                assert_eq!(e.mem.u32(a[6]), 0);
            }
            count.set(count.get() + 1);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x00483710, move |e, a| {
            assert!(e.get(runner, ScriptRunner::m_scriptRunning).is_null());
            assert_eq!(e.mem.u32(a[0] + 0x70c), 0x12345678);
            Ret::default()
        });
        assert!(e
            .call(0x005e1160, &args![runner, script, 0x4444u32, 0x6666u32])
            .bool());
        assert_eq!(evaluations.get(), 2);
        assert_eq!(e.mem.u32(header + 8), 18);
    }

    #[test]
    fn action_list_uses_real_parser_evaluator_and_expression_for_two_event_blocks() {
        let mut e = Engine::new();
        let bytes = [
            29, 0, 0, 0, 16, 0, 6, 0, 3, 0, 4, 0, 0, 0, 17, 0, 0, 0, 16, 0, 6, 0, 6, 0, 0, 0, 0, 0,
        ];
        let (runner, script, _) = parser_fixture(&mut e, &bytes);
        let header = e.mem.alloc(12);
        e.mem.set_u32(header + 8, bytes.len() as u32);
        e.register_double(0x004013e0, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x007af430, |_, a| Ret {
            eax: a[0] + 4,
            ..Ret::default()
        });
        e.register_double(0x00500940, move |_, _| Ret {
            eax: header,
            ..Ret::default()
        });
        e.register_double(0x00483710, move |e, a| {
            assert!(e.get(runner, ScriptRunner::m_scriptRunning).is_null());
            assert_eq!(e.mem.u32(a[0]), 0);
            assert_eq!(e.mem.u32(a[0] + 0x504), u32::MAX);
            assert_eq!(e.mem.u32(a[0] + 0x708), u32::MAX);
            Ret::default()
        });
        let hits = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        for (kind, cursor) in [(3u32, 14u32), (6, 28)] {
            let entry = 0x0118e300 + kind * 0x28;
            e.map(entry, 12);
            let handler = 0x12340000 + kind;
            e.mem.set_u8(entry, 1);
            e.mem.set_u32(entry + 4, 0x7777);
            e.mem.set_u32(entry + 8, handler);
            let log = hits.clone();
            e.register_double(handler, move |e, a| {
                assert_eq!(&a[2..6], &[0x3333, 0, script.addr(), 0x5555]);
                assert_eq!(e.mem.u32(a[7]), cursor);
                e.mem.set_f64(a[6], 1.0);
                log.borrow_mut().push(kind);
                Ret {
                    eax: 1,
                    ..Ret::default()
                }
            });
        }
        assert!(e
            .call(0x005e1160, &args![runner, script, 0x3333u32, 0x5555u32])
            .bool());
        assert_eq!(*hits.borrow(), [3, 6]);
        assert_eq!(e.mem.u32(header + 8), 28);
    }

    #[test]
    fn action_list_failures_return_true_but_only_parse_failure_clears_fresh_header() {
        for parse_success in [false, true] {
            let mut e = Engine::new();
            let runner = e.new_object::<ScriptRunner>();
            let script = Ptr::new(e.mem.alloc(0x54));
            let original = e.mem.alloc(12);
            let replacement = e.mem.alloc(12);
            e.mem.set_u32(original + 8, 4);
            e.mem.set_u32(replacement + 8, 9);
            e.register_double(0x004013e0, |_, _| Ret {
                eax: 1,
                ..Ret::default()
            });
            e.register_double(0x00593490, |_, _| Ret::default());
            let mut reads = 0;
            e.register_double(0x00500940, move |_, _| {
                reads += 1;
                Ret {
                    eax: if reads == 1 { original } else { replacement },
                    ..Ret::default()
                }
            });
            e.register_double(0x005e13a0, move |e, a| {
                assert_eq!(a[6], 0);
                assert_eq!(e.mem.u32(a[2]), 0xffff);
                assert_eq!(e.mem.u32(a[3]), 0);
                e.mem.set_u32(a[3], 4);
                Ret {
                    eax: u32::from(parse_success),
                    ..Ret::default()
                }
            });
            e.register_double(0x005e1550, |_, a| {
                assert_eq!((a[3], a[5], a[7]), (0, 4, 1));
                Ret::default()
            });
            e.register_double(0x00483710, move |e, _| {
                assert!(e.get(runner, ScriptRunner::m_scriptRunning).is_null());
                Ret::default()
            });
            assert!(runner_init_action_list(
                &mut e,
                runner,
                script,
                Ptr::NULL,
                Ptr::NULL
            ));
            assert_eq!(e.mem.u32(original + 8), 4);
            assert_eq!(
                e.mem.u32(replacement + 8),
                if parse_success { 9 } else { 0 }
            );
            assert_eq!(e.mem.u32(runner.addr() + 4), 0);
        }
    }

    #[test]
    fn parser_matches_six_native_signed_word_and_prefix_vectors() {
        // Generated nv-call cases, PC 005e13a0, exe SHA-256
        // 19406942e48724d797300c4ea6be9ac69a32f670b8c35db09279a2258422739f.
        type ParserCase<'a> = (&'a [u8], bool, i32, u32, i32, u32);
        let cases: &[ParserCase<'_>] = &[
            (&[0, 0, 0, 0], false, 0, 4, 0, 0),
            (&[16, 0, 10, 0], false, 16, 4, 10, 0),
            (&[255, 255, 1, 0], false, -1, 4, 1, 0),
            (&[16, 0, 255, 255], false, 16, 4, -1, 0),
            (&[16, 0, 0, 128], true, 16, 4, -32768, 0),
            (&[28, 0, 7, 0, 16, 0, 255, 255], true, 16, 8, -1, 0xdeadbeef),
        ];
        for &(bytes, skip, opcode, cursor, length, reference) in cases {
            let mut e = Engine::new();
            let (runner, script, work) = parser_fixture(&mut e, bytes);
            assert!(e
                .call(
                    0x005e13a0,
                    &args![
                        runner,
                        script,
                        work,
                        work.byte_add(4),
                        work.byte_add(8),
                        work.byte_add(12),
                        skip
                    ]
                )
                .bool());
            assert_eq!(e.mem.i32(work.addr()), opcode);
            assert_eq!(e.mem.u32(work.addr() + 4), cursor);
            assert_eq!(e.mem.i32(work.addr() + 8), length);
            assert_eq!(e.mem.u32(work.addr() + 12), reference);
        }
    }

    #[test]
    fn parser_reloads_code_and_resolves_signed_reference_index() {
        let mut e = Engine::new();
        let (runner, script, work) = parser_fixture(&mut e, &[28, 0, 255, 255, 16, 0, 8, 0]);
        e.mem.set_u32(runner.addr() + 8, 0x7000);
        let mut blocks = Vec::new();
        for bytes in [
            [28, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 255, 255, 0, 0, 0, 0],
            [0, 0, 0, 0, 17, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 255, 255],
        ] {
            let block = e.mem.alloc(8);
            e.mem.write(block, &bytes);
            blocks.push(block);
        }
        let mut read = 0;
        e.register_double(0x00671d10, move |_, _| {
            let result = blocks[read];
            read += 1;
            Ret {
                eax: result,
                ..Ret::default()
            }
        });
        let found = e.mem.alloc(16);
        e.mem.set_u32(found + 8, 0x8000);
        e.register_double(0x005ac4f0, move |_, a| {
            assert_eq!(a, [script.addr(), u32::MAX, 0x7000]);
            Ret {
                eax: found,
                ..Ret::default()
            }
        });
        e.register_double(0x00ec43fb, |_, a| {
            assert_eq!(a, [0x8000, 0, 0x01183028, 0x011841cc, 0]);
            Ret {
                eax: 0x9000,
                ..Ret::default()
            }
        });
        assert!(parse(&mut e, runner, script, work, false));
        assert_eq!(e.mem.i32(work.addr()), 17);
        assert_eq!(e.mem.i32(work.addr() + 8), -1);
        assert_eq!(e.mem.u32(work.addr() + 12), 0x9000);
    }

    #[test]
    fn parser_null_variable_reference_is_allowed_but_missing_reference_logs() {
        for (variable, descriptor_present) in [(true, false), (false, false), (false, true)] {
            let mut e = Engine::new();
            let (runner, script, work) = parser_fixture(&mut e, &[28, 0, 1, 0, 16, 0, 9, 0]);
            e.mem.set_u32(work.addr() + 12, 0);
            let found = e.mem.alloc(16);
            e.mem.set_u32(found + 12, u32::from(variable));
            e.register_double(0x005ac4f0, move |_, _| Ret {
                eax: found,
                ..Ret::default()
            });
            let descriptor = e.mem.alloc(4);
            e.mem.set_u32(descriptor, 0x1111);
            e.register_double(0x005b1120, move |_, a| {
                assert_eq!(a, [16]);
                Ret {
                    eax: if descriptor_present { descriptor } else { 0 },
                    ..Ret::default()
                }
            });
            let vtable = e.mem.alloc(0x134);
            e.mem.set_u32(script.addr(), vtable);
            e.mem.set_u32(vtable + 0x130, 0x12340000);
            e.register_double(0x12340000, move |e, _| {
                e.mem.set_u32(descriptor, 0x2222);
                Ret {
                    eax: 0x3333,
                    ..Ret::default()
                }
            });
            e.register_double(0x005e2410, move |_, a| {
                let mut expected = vec![runner.addr(), script.addr()];
                if descriptor_present {
                    expected.extend([0x0103d538, 0x2222, 0x3333]);
                } else {
                    expected.extend([0x0103d4f4, 0x3333]);
                }
                assert_eq!(a, expected);
                Ret::default()
            });
            assert_eq!(parse(&mut e, runner, script, work, false), variable);
            assert_eq!(e.mem.u32(work.addr() + 4), if variable { 8 } else { 6 });
            assert_eq!(e.mem.u32(work.addr() + 8), if variable { 9 } else { 0 });
        }
    }

    #[test]
    fn parser_preserves_store_order_when_opcode_aliases_cursor() {
        let mut e = Engine::new();
        let (runner, script, work) = parser_fixture(&mut e, &[4, 0, 99, 0, 99, 0, 7, 0]);
        e.mem.set_u32(work.addr(), 0);
        assert!(runner_parse_line(
            &mut e,
            runner,
            script,
            work,
            work,
            work.byte_add(8),
            work.byte_add(12),
            false
        ));
        assert_eq!(e.mem.u32(work.addr()), 8);
        assert_eq!(e.mem.u32(work.addr() + 8), 7);
    }

    #[test]
    fn constructor_clears_fields_and_keeps_padding_and_guards() {
        // Three guarded nv-call vectors agree with PC 005e0c60, unpacked
        // exe SHA-256 19406942e48724d797300c4ea6be9ac69a32f670b8c35db09279a2258422739f.
        for fill in [0xa5, 0xcc, 0xff] {
            let mut e = Engine::new();
            let block = e.mem.alloc(0xac);
            e.mem.write(block, &[fill; 0xac]);
            let this = block + 4;
            assert_eq!(e.call(0x005e0c60, &[this]).u32(), this);
            assert_eq!(e.mem.bytes(block, 4), vec![fill; 4]);
            assert_eq!(e.mem.bytes(this, 0xa2), vec![0; 0xa2]);
            assert_eq!(e.mem.bytes(this + 0xa2, 6), vec![fill; 6]);
        }
    }

    #[test]
    fn constructor_preserves_native_call_and_store_order() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xa4);
        e.mem.write(this, &[0xcc; 0xa4]);
        let mut step = 0;
        e.register_double(0x00403d30, move |e, a| {
            assert_eq!(a, &[this + [0x24, 0x50, 0x78][step], 0, 0x28]);
            assert_eq!(e.mem.u32(this + 0x20), 0);
            assert_eq!(e.mem.u32(this + 0x4c), 0);
            assert_eq!(e.mem.u32(this + 0x18), 0);
            assert_eq!(e.mem.u32(this + 0x1c), 0);
            assert_eq!(e.mem.u32(this), 0xcccccccc);
            step += 1;
            e.mem.write(a[0], &[step as u8; 0x28]);
            if step == 3 {
                e.mem.set_u32(this + 0x20, 7);
            }
            Ret::default()
        });
        e.call(0x005e0c60, &[this]);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 0x20), 7);
        for (offset, fill) in [(0x24, 1), (0x50, 2), (0x78, 3)] {
            assert_eq!(e.mem.bytes(this + offset, 0x28), vec![fill; 0x28]);
        }
    }

    #[test]
    fn singleton_guard_precedes_constructor_and_preserves_other_bits() {
        let mut e = Engine::new();
        e.map(MANAGER, 12);
        e.mem.set_u32(INITIALIZED, 0xa4);
        e.register_double(0x0096a2d0, |e, a| {
            assert_eq!(a, &[MANAGER]);
            assert_eq!(e.mem.u32(INITIALIZED), 0xa5);
            assert_eq!(e.call(0x005e24d0, &[]).u32(), MANAGER);
            e.mem.set_u32(MANAGER, 0x1234);
            Ret::default()
        });
        e.register_double(0x00ec658f, |e, a| {
            assert_eq!(a, &[0x00fcb1f0]);
            assert_eq!(e.mem.u32(MANAGER), 0x1234);
            Ret::default()
        });
        assert_eq!(e.call(0x005e24d0, &[]).u32(), MANAGER);
        assert_eq!(e.call(0x005e24d0, &[]).u32(), MANAGER);
    }

    #[test]
    fn busy_predicate_tests_full_pointer() {
        // Native 005e2850 returns AL=0,1,1 for these pointers, with no
        // memory writes. Upper EAX is incidental and not part of bool ABI.
        let mut e = Engine::new();
        let this = e.new_object::<ScriptRunner>();
        for pointer in [0, 0x100, u32::MAX] {
            e.set(this, ScriptRunner::m_scriptRunning, Ptr::new(pointer));
            assert_eq!(e.call(0x005e2850, &[this.addr()]).bool(), pointer != 0);
        }
    }

    #[test]
    fn cached_runner_with_null_script_matches_native_manager_result() {
        // Native 005e2730 with a cached idle runner and three null arguments
        // returns AL=1, pops twelve bytes and leaves the objects untouched.
        // The interpreter's null-script early return is the only double.
        let mut e = Engine::new();
        let manager = e.new_object::<ScriptRunManager>();
        let runner = e.new_object::<ScriptRunner>();
        e.set(manager, ScriptRunManager::m_pScriptRunner, runner);
        e.mem.set_u32(manager.addr() + 4, 0xa5a5a5a5);
        let before = e.mem.bytes(manager.addr(), 8);
        e.register_double(0x005e1160, move |_, a| {
            assert_eq!(a, &[runner.addr(), 0, 0, 0]);
            0u32.into_ret()
        });
        assert!(e.call(0x005e2730, &[manager.addr(), 0, 0, 0]).bool());
        assert_eq!(e.mem.bytes(manager.addr(), 8), before);
        assert_eq!(e.mem.bytes(runner.addr(), 0xa4), vec![0; 0xa4]);
    }

    #[test]
    fn idle_cached_runner_is_reloaded_after_busy_callback() {
        let mut e = Engine::new();
        let manager = e.new_object::<ScriptRunManager>();
        let first = e.new_object::<ScriptRunner>();
        let second = e.new_object::<ScriptRunner>();
        e.set(manager, ScriptRunManager::m_pScriptRunner, first);
        e.register_double(0x005e2850, move |e, a| {
            assert_eq!(a, &[first.addr()]);
            e.set(manager, ScriptRunManager::m_pScriptRunner, second);
            0u32.into_ret()
        });
        e.register_double(0x005e1160, move |_, a| {
            assert_eq!(a, &[second.addr(), 0x10, 0x20, 0x30]);
            0u32.into_ret()
        });
        assert!(e
            .call(0x005e2730, &[manager.addr(), 0x10, 0x20, 0x30])
            .bool());
    }

    #[test]
    fn busy_runner_uses_and_frees_temporary_without_replacing_cache() {
        let mut e = Engine::new();
        let manager = e.new_object::<ScriptRunManager>();
        let cached = e.new_object::<ScriptRunner>();
        let temporary = e.new_object::<ScriptRunner>();
        e.set(manager, ScriptRunManager::m_pScriptRunner, cached);
        e.set(cached, ScriptRunner::m_scriptRunning, Ptr::new(0x100));
        e.register_double(0x00401000, move |_, a| {
            assert_eq!(a, &[0xa4]);
            temporary.into_ret()
        });
        e.register_double(0x005e1160, move |e, a| {
            assert_eq!(a, &[temporary.addr(), 1, 2, 3]);
            assert_eq!(e.mem.u32(temporary.addr() + 0x14), 0);
            assert_eq!(e.get(manager, ScriptRunManager::m_pScriptRunner), cached);
            e.mem.set_u32(temporary.addr(), 0x1234);
            0u32.into_ret()
        });
        e.register_double(0x00401030, move |e, a| {
            assert_eq!(a, &[temporary.addr()]);
            assert_eq!(e.mem.u32(temporary.addr()), 0x1234);
            Ret::default()
        });
        assert!(e.call(0x005e2730, &[manager.addr(), 1, 2, 3]).bool());
    }

    #[test]
    fn empty_cache_allocates_and_keeps_constructor_return() {
        let mut e = Engine::new();
        let manager = e.new_object::<ScriptRunManager>();
        let allocated = e.new_object::<ScriptRunner>();
        let returned = e.new_object::<ScriptRunner>();
        e.register_double(0x00401000, move |_, a| {
            assert_eq!(a, &[0xa4]);
            allocated.into_ret()
        });
        e.register_double(0x005e0c60, move |_, a| {
            assert_eq!(a, &[allocated.addr()]);
            returned.into_ret()
        });
        e.register_double(0x005e1160, move |e, a| {
            assert_eq!(a, &[returned.addr(), 1, 2, 3]);
            assert_eq!(e.get(manager, ScriptRunManager::m_pScriptRunner), returned);
            Ret::default()
        });
        assert!(e.call(0x005e2730, &[manager.addr(), 1, 2, 3]).bool());
    }
}
