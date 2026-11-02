//! Generated machine code: the per-hook entry stub, return stub and
//! trampoline, all placed in one executable block.
//!
//! Frame layout after the stubs save state (`ebx` points at it):
//!
//! ```text
//! ebx+0   edi esi ebp (esp slot, unused) ebx edx ecx eax   (pushad order,
//!         lowest address first)
//! ebx+32  eflags
//! ebx+36  return address of the hooked function (entry stub)
//!         or a slot for the original return address (return stub)
//! ebx+40  first stack argument ...
//! ```
//!
//! Below the frame the stubs align the stack to 16 bytes and save the
//! FPU/SSE state with FXSAVE, reset the FPU (FNINIT) and MXCSR so the Rust
//! handler runs in a known state, call the handler as
//! `handler(hook_id, frame, fxsave_image)`, and then restore everything.

use nv_win as win;
use std::ffi::c_void;

/// Register frame saved by the stubs.
#[repr(C)]
pub struct SavedRegs {
    pub edi: u32,
    pub esi: u32,
    pub ebp: u32,
    pub esp_unused: u32,
    pub ebx: u32,
    pub edx: u32,
    pub ecx: u32,
    pub eax: u32,
    pub eflags: u32,
    /// Return address (entry stub) or the slot the return stub fills in.
    pub ret_addr: u32,
}

pub const FRAME_SIZE: u32 = 36;
const _: () = assert!(std::mem::size_of::<SavedRegs>() == 40);
const _: () = assert!(std::mem::offset_of!(SavedRegs, ret_addr) == FRAME_SIZE as usize);

/// Offsets inside an FXSAVE image (32-bit layout).
pub mod fx {
    pub const FSW: usize = 2;
    pub const FTW: usize = 4;
    pub const ST0: usize = 32;
    pub const XMM0: usize = 160;
    pub const SIZE: usize = 512;
}

/// One block of executable memory that stubs are carved out of.
pub struct Region {
    base: u32,
    used: u32,
    size: u32,
}

/// Bytes in the region before the first stub: a constant MXCSR value.
const HEADER: u32 = 16;

impl Region {
    pub fn new(size: u32) -> Result<Region, String> {
        // SAFETY: fresh allocation anywhere in the address space.
        let p = unsafe {
            win::VirtualAlloc(
                std::ptr::null_mut(),
                size as usize,
                win::MEM_RESERVE | win::MEM_COMMIT,
                win::PAGE_EXECUTE_READWRITE,
            )
        };
        if p.is_null() {
            return Err("cannot allocate the stub region".into());
        }
        let base = p as usize as u32;
        // SAFETY: the header is inside the new allocation.
        unsafe { std::ptr::write(base as usize as *mut u32, 0x1f80) };
        Ok(Region {
            base,
            used: HEADER,
            size,
        })
    }

    pub fn mxcsr_const(&self) -> u32 {
        self.base
    }

    /// Reserve `len` bytes, 16-byte aligned. Returns the address.
    pub fn alloc(&mut self, len: usize) -> Result<u32, String> {
        let start = (self.used + 15) & !15;
        let end = start as u64 + len as u64;
        if end > u64::from(self.size) {
            return Err("stub region is full".into());
        }
        self.used = end as u32;
        Ok(self.base + start)
    }

    /// Copy code into previously allocated space.
    pub fn write(&self, addr: u32, code: &[u8]) {
        debug_assert!(
            addr >= self.base
                && u64::from(addr) + code.len() as u64
                    <= u64::from(self.base) + u64::from(self.size)
        );
        // SAFETY: the target lies inside this region (checked above in debug builds
        // and guaranteed by `alloc` in all builds).
        unsafe {
            std::ptr::copy_nonoverlapping(code.as_ptr(), addr as usize as *mut u8, code.len())
        };
        // SAFETY: flushing the range just written.
        unsafe {
            win::FlushInstructionCache(
                win::GetCurrentProcess(),
                addr as usize as *const c_void,
                code.len(),
            )
        };
    }
}

/// Common middle part: save state, call `handler`, restore state.
fn call_handler(c: &mut Vec<u8>, id: u32, handler: u32, mxcsr_addr: u32) {
    c.extend([0x9c, 0x60, 0x89, 0xe3]); // pushfd; pushad; mov ebx, esp
    c.extend([0x81, 0xec, 0x10, 0x02, 0x00, 0x00]); // sub esp, 0x210
    c.extend([0x83, 0xe4, 0xf0]); // and esp, -16
    c.extend([0x0f, 0xae, 0x04, 0x24]); // fxsave [esp]
    c.push(0xfc); // cld
    c.extend([0xdb, 0xe3]); // fninit
    c.extend([0x0f, 0xae, 0x15]); // ldmxcsr [mxcsr_addr]
    c.extend(mxcsr_addr.to_le_bytes());
    c.extend([0x89, 0xe0]); // mov eax, esp        (fxsave image)
    c.extend([0x83, 0xec, 0x04]); // sub esp, 4     (keep 16-byte alignment at the call)
    c.push(0x50); // push eax
    c.push(0x53); // push ebx                      (frame)
    c.push(0x68); // push imm32                    (hook id)
    c.extend(id.to_le_bytes());
    c.push(0xb8); // mov eax, handler
    c.extend(handler.to_le_bytes());
    c.extend([0xff, 0xd0]); // call eax
    c.extend([0x83, 0xc4, 0x10]); // add esp, 16
    c.extend([0x0f, 0xae, 0x0c, 0x24]); // fxrstor [esp]
    c.extend([0x89, 0xdc]); // mov esp, ebx
    c.extend([0x61, 0x9d]); // popad; popfd
}

/// Entry stub: runs in place of the function's first instructions, then
/// continues in the trampoline.
pub fn entry_stub(at: u32, id: u32, handler: u32, mxcsr_addr: u32, trampoline: u32) -> Vec<u8> {
    let mut c = Vec::with_capacity(80);
    call_handler(&mut c, id, handler, mxcsr_addr);
    c.push(0xe9); // jmp trampoline
    let rel = trampoline.wrapping_sub(at.wrapping_add(c.len() as u32 + 4));
    c.extend(rel.to_le_bytes());
    c
}

/// Return stub: reached when the hooked function returns. It pushes an
/// empty slot, lets the handler fill in the original return address, and
/// returns through it, so the stack ends up as the caller expects for both
/// caller-cleaned and callee-cleaned conventions.
pub fn return_stub(id: u32, handler: u32, mxcsr_addr: u32) -> Vec<u8> {
    let mut c = Vec::with_capacity(80);
    c.extend([0x6a, 0x00]); // push 0
    call_handler(&mut c, id, handler, mxcsr_addr);
    c.push(0xc3); // ret
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entry_stub_ends_with_a_jump_to_the_trampoline() {
        let code = entry_stub(0x1000_0100, 7, 0x1234_5678, 0x1000_0000, 0x1000_0200);
        let n = code.len();
        assert_eq!(code[n - 5], 0xe9);
        let rel = u32::from_le_bytes(code[n - 4..].try_into().unwrap());
        assert_eq!(
            0x1000_0100u32.wrapping_add(n as u32).wrapping_add(rel),
            0x1000_0200
        );
        assert!(code.windows(5).any(|w| w == [0x68, 7, 0, 0, 0]));
        assert!(code.windows(5).any(|w| w == [0xb8, 0x78, 0x56, 0x34, 0x12]));
    }

    #[test]
    fn return_stub_starts_with_the_slot_and_ends_with_ret() {
        let code = return_stub(3, 0x1111_2222, 0x1000_0000);
        assert_eq!(&code[..2], &[0x6a, 0x00]);
        assert_eq!(*code.last().unwrap(), 0xc3);
    }
}
