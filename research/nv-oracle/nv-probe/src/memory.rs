//! Reading memory safely and patching code.

use nv_win as win;
use std::ffi::c_void;

/// Copy `len` bytes from `addr` if the whole range is readable.
pub fn read_bytes(addr: u32, len: usize) -> Result<Vec<u8>, String> {
    if !win::is_readable(addr, len) {
        return Err(format!("memory at {addr:#010x} (+{len}) is not readable"));
    }
    let mut out = vec![0u8; len];
    // SAFETY: the range was just checked to be committed and readable.
    unsafe { std::ptr::copy_nonoverlapping(addr as usize as *const u8, out.as_mut_ptr(), len) };
    Ok(out)
}

pub fn read_u32(addr: u32) -> Result<u32, String> {
    let b = read_bytes(addr, 4)?;
    Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

// LOCK CMPXCHG8B needs EBX, EDX:EAX and ECX:EBX, which compiled code and
// inline assembly cannot always spare, so it lives in a tiny routine of its
// own with a plain C calling convention:
//   nv_cmpxchg8(addr, expect_lo, expect_hi, new_lo, new_hi, out: *mut [u32; 2])
// `out` receives the 8 bytes that were found at `addr`.
std::arch::global_asm!(
    ".text",
    ".globl _nv_cmpxchg8",
    "_nv_cmpxchg8:",
    "    push ebx",
    "    push edi",
    "    mov edi, [esp + 12]",
    "    mov eax, [esp + 16]",
    "    mov edx, [esp + 20]",
    "    mov ebx, [esp + 24]",
    "    mov ecx, [esp + 28]",
    "    lock cmpxchg8b [edi]",
    "    mov edi, [esp + 32]",
    "    mov [edi], eax",
    "    mov [edi + 4], edx",
    "    pop edi",
    "    pop ebx",
    "    ret",
);

extern "C" {
    fn nv_cmpxchg8(
        addr: u32,
        expect_lo: u32,
        expect_hi: u32,
        new_lo: u32,
        new_hi: u32,
        out: *mut [u32; 2],
    );
}

/// Compare 8 bytes at `addr` with `expect` and, if equal, replace them with
/// `new` in a single locked 8-byte write, so a thread executing the code
/// sees either the old or the new bytes, never a mixture. Returns the 8
/// bytes that were found.
///
/// # Safety
/// `addr..addr+8` must be committed memory that may be made writable.
unsafe fn cmpxchg8(addr: u32, expect: [u8; 8], new: [u8; 8]) -> [u8; 8] {
    let half = |b: &[u8; 8], i: usize| u32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]]);
    let mut found = [0u32; 2];
    nv_cmpxchg8(
        addr,
        half(&expect, 0),
        half(&expect, 4),
        half(&new, 0),
        half(&new, 4),
        &mut found,
    );
    let mut out = [0u8; 8];
    out[..4].copy_from_slice(&found[0].to_le_bytes());
    out[4..].copy_from_slice(&found[1].to_le_bytes());
    out
}

/// A code patch that can be undone.
#[derive(Clone, Copy, Debug)]
pub struct Patch {
    pub addr: u32,
    pub len: usize,
    pub original: [u8; 8],
    pub patched: [u8; 8],
}

fn with_writable<T>(addr: u32, len: usize, f: impl FnOnce() -> T) -> Result<T, &'static str> {
    let mut old = 0u32;
    // SAFETY: changing protection of this process's own pages.
    let ok = unsafe {
        win::VirtualProtect(
            addr as usize as *mut c_void,
            len,
            win::PAGE_EXECUTE_READWRITE,
            &mut old,
        )
    };
    if ok == 0 {
        return Err("VirtualProtect failed");
    }
    let r = f();
    let mut ignored = 0u32;
    // SAFETY: restoring the protection recorded above, then flushing the
    // instruction cache for the modified bytes.
    unsafe {
        win::VirtualProtect(addr as usize as *mut c_void, len, old, &mut ignored);
        win::FlushInstructionCache(
            win::GetCurrentProcess(),
            addr as usize as *const c_void,
            len,
        );
    }
    Ok(r)
}

/// Overwrite the first 5 bytes of a function with `jmp rel32` to `target`,
/// after checking that the bytes there are still `expected`.
///
/// When 8 bytes are readable the write is one atomic 8-byte compare and
/// exchange that keeps the three following bytes. Otherwise it falls back
/// to a plain 5-byte copy, which is only safe if no thread is running that
/// code (the caller suspends threads first for exactly this reason).
///
/// This runs while other threads are suspended, so it must not allocate (a
/// suspended thread may hold the heap lock). Errors are static strings for
/// the same reason.
pub fn write_jump(addr: u32, target: u32, expected: &[u8]) -> Result<Patch, &'static str> {
    let rel = target.wrapping_sub(addr.wrapping_add(5));
    let mut jmp = [0xe9u8, 0, 0, 0, 0];
    jmp[1..].copy_from_slice(&rel.to_le_bytes());
    let atomic = win::is_readable(addr, 8);
    let len = if atomic { 8 } else { 5 };
    if !win::is_readable(addr, len) {
        return Err("the function's first bytes are not readable");
    }
    let mut original = [0u8; 8];
    // SAFETY: the range was just checked to be readable.
    unsafe {
        std::ptr::copy_nonoverlapping(addr as usize as *const u8, original.as_mut_ptr(), len)
    };
    let compare = expected.len().min(len);
    if original[..compare] != expected[..compare] {
        return Err("code changed while the hook was being installed");
    }
    let mut patched = original;
    patched[..5].copy_from_slice(&jmp);
    with_writable(addr, len, || {
        if atomic {
            // SAFETY: the page was just made writable.
            let found = unsafe { cmpxchg8(addr, original, patched) };
            if found != original {
                return Err("code changed while the hook was being installed");
            }
        } else {
            // SAFETY: the page was just made writable.
            unsafe { std::ptr::copy_nonoverlapping(jmp.as_ptr(), addr as usize as *mut u8, 5) };
        }
        Ok(())
    })??;
    Ok(Patch {
        addr,
        len,
        original,
        patched,
    })
}

/// Undo [`write_jump`] if the jump is still in place.
pub fn restore(p: &Patch) -> Result<(), &'static str> {
    with_writable(p.addr, p.len, || {
        if p.len == 8 {
            // SAFETY: the page was just made writable.
            let found = unsafe { cmpxchg8(p.addr, p.patched, p.original) };
            if found != p.patched {
                return Err("code at the hook was changed by someone else; left alone");
            }
        } else {
            // SAFETY: the page was just made writable.
            unsafe {
                std::ptr::copy_nonoverlapping(p.original.as_ptr(), p.addr as usize as *mut u8, 5)
            };
        }
        Ok(())
    })?
}

/// Other threads of this process, suspended.
pub struct Frozen {
    threads: Vec<win::Handle>,
}

impl Frozen {
    /// Suspend every other thread. All allocation happens before the first
    /// thread is stopped, so a suspended thread holding the heap lock
    /// cannot deadlock the caller.
    pub fn freeze_others() -> Frozen {
        // SAFETY: Toolhelp snapshot of this process's threads.
        let (me, pid) = unsafe { (win::GetCurrentThreadId(), win::GetCurrentProcessId()) };
        let mut ids = Vec::new();
        // SAFETY: the snapshot handle is closed below; the entry is a valid output buffer.
        unsafe {
            let snap = win::CreateToolhelp32Snapshot(win::TH32CS_SNAPTHREAD, 0);
            if snap != win::INVALID_HANDLE_VALUE && !snap.is_null() {
                let mut e = win::ThreadEntry32 {
                    size: std::mem::size_of::<win::ThreadEntry32>() as u32,
                    usage: 0,
                    thread_id: 0,
                    owner_process_id: 0,
                    base_priority: 0,
                    delta_priority: 0,
                    flags: 0,
                };
                let mut ok = win::Thread32First(snap, &mut e);
                while ok != 0 {
                    if e.owner_process_id == pid && e.thread_id != me {
                        ids.push(e.thread_id);
                    }
                    e.size = std::mem::size_of::<win::ThreadEntry32>() as u32;
                    ok = win::Thread32Next(snap, &mut e);
                }
                win::CloseHandle(snap);
            }
        }
        let mut threads = Vec::with_capacity(ids.len());
        for id in ids {
            // SAFETY: opening and suspending threads of this process.
            unsafe {
                let h = win::OpenThread(
                    win::THREAD_SUSPEND_RESUME | win::THREAD_GET_CONTEXT | win::THREAD_SET_CONTEXT,
                    0,
                    id,
                );
                if h.is_null() {
                    continue;
                }
                if win::SuspendThread(h) == u32::MAX {
                    win::CloseHandle(h);
                } else {
                    threads.push(h);
                }
            }
        }
        Frozen { threads }
    }

    pub fn none() -> Frozen {
        Frozen {
            threads: Vec::new(),
        }
    }

    pub fn count(&self) -> usize {
        self.threads.len()
    }

    /// A suspended thread whose instruction pointer is inside the bytes
    /// that were replaced (past the first, which becomes the jump) would
    /// resume in the middle of the jump. Move it to the same place in the
    /// trampoline, whose copy of those bytes has the same layout.
    /// `ranges` is `(function address, stolen length, trampoline address)`.
    pub fn fix_instruction_pointers(&self, ranges: &[(u32, u32, u32)]) -> usize {
        let mut moved = 0;
        for &h in &self.threads {
            let mut ctx = win::Context::zeroed();
            ctx.context_flags = win::CONTEXT_CONTROL;
            // SAFETY: valid context buffer; the thread is suspended.
            unsafe {
                if win::GetThreadContext(h, &mut ctx) == 0 {
                    continue;
                }
                for &(start, len, tramp) in ranges {
                    let off = ctx.eip.wrapping_sub(start);
                    if off >= 1 && off < len {
                        ctx.eip = tramp + off;
                        if win::SetThreadContext(h, &ctx) != 0 {
                            moved += 1;
                        }
                        break;
                    }
                }
            }
        }
        moved
    }

    pub fn thaw(self) {
        for h in self.threads {
            // SAFETY: handles opened in `freeze_others`.
            unsafe {
                win::ResumeThread(h);
                win::CloseHandle(h);
            }
        }
    }
}
