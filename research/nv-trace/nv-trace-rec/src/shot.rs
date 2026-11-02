//! Screenshots of the game window's client area (full or half size, 24-bit BMP) into the run
//! folder (`shots/`). Only the game process's own window, through PrintWindow: never a copy of
//! the screen, which would show any window lying over it (a port run's window has the same
//! title).

use crate::sys::Handle;
use std::ffi::c_void;
use std::sync::OnceLock;

#[repr(C)]
struct Rect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

#[repr(C)]
struct BitmapInfoHeader {
    size: u32,
    width: i32,
    height: i32,
    planes: u16,
    bit_count: u16,
    compression: u32,
    size_image: u32,
    x_ppm: i32,
    y_ppm: i32,
    clr_used: u32,
    clr_important: u32,
}

#[link(name = "user32")]
extern "system" {
    fn GetClientRect(w: Handle, r: *mut Rect) -> i32;
    fn GetDC(w: Handle) -> Handle;
    fn ReleaseDC(w: Handle, dc: Handle) -> i32;
    fn PrintWindow(w: Handle, dc: Handle, flags: u32) -> i32;
}

#[link(name = "gdi32")]
extern "system" {
    fn CreateCompatibleDC(dc: Handle) -> Handle;
    fn CreateCompatibleBitmap(dc: Handle, w: i32, h: i32) -> Handle;
    fn SelectObject(dc: Handle, o: Handle) -> Handle;
    fn GetDIBits(
        dc: Handle,
        bm: Handle,
        start: u32,
        lines: u32,
        bits: *mut c_void,
        info: *mut BitmapInfoHeader,
        usage: u32,
    ) -> i32;
    fn DeleteObject(o: Handle) -> i32;
    fn DeleteDC(dc: Handle) -> i32;
}

static FULL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Screenshots at the window's full size (else half).
pub fn set_full(full: bool) {
    FULL.store(full, std::sync::atomic::Ordering::Relaxed);
}

static DIR: OnceLock<std::path::PathBuf> = OnceLock::new();

pub fn set_dir(dir: std::path::PathBuf) {
    let _ = DIR.set(dir);
}

/// Saves the client area of `window` as `shots/<name>.bmp`; returns whether it worked.
pub fn take(window: Handle, name: &str) -> bool {
    let Some(dir) = DIR.get() else { return false };
    // SAFETY: GDI calls on handles made here and released before returning.
    unsafe {
        let mut r = Rect {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        if window.is_null() || GetClientRect(window, &mut r) == 0 {
            return false;
        }
        let screen = GetDC(std::ptr::null_mut());
        // The window's own content at its own (logical) size: PrintWindow with PW_CLIENTONLY |
        // PW_RENDERFULLCONTENT gives the game's frame as it drew it, whatever covers the
        // window. `window` is the game process's own window (`input::game_window`).
        let (lw, lh) = (r.right - r.left, r.bottom - r.top);
        let mem = CreateCompatibleDC(screen);
        let (w, h) = (lw, lh);
        let bmp = CreateCompatibleBitmap(screen, w.max(1), h.max(1));
        let old = SelectObject(mem, bmp);
        let printed = lw > 0 && lh > 0 && PrintWindow(window, mem, 3) != 0;
        // No copy of the screen as a fallback: whatever lies over the window would be captured
        // (another program's window with the same title, say). Only the window's own content.
        if !printed {
            SelectObject(mem, old);
            DeleteObject(bmp);
            DeleteDC(mem);
            ReleaseDC(std::ptr::null_mut(), screen);
            return false;
        }
        SelectObject(mem, old);
        let mut info = BitmapInfoHeader {
            size: 40,
            width: w,
            height: h,
            planes: 1,
            bit_count: 32,
            compression: 0,
            size_image: 0,
            x_ppm: 0,
            y_ppm: 0,
            clr_used: 0,
            clr_important: 0,
        };
        let mut pixels = vec![0u8; (w * h * 4) as usize];
        let lines = GetDIBits(
            mem,
            bmp,
            0,
            h as u32,
            pixels.as_mut_ptr() as *mut c_void,
            &mut info,
            0,
        );
        DeleteObject(bmp);
        DeleteDC(mem);
        ReleaseDC(std::ptr::null_mut(), screen);
        if lines == 0 {
            return false;
        }
        // PrintWindow sometimes hands back an empty (black) frame for a Direct3D window: not
        // saved, the caller tries again on a later frame.
        // (Almost) all black: a cursor or a stray pixel may be drawn into an empty frame.
        let lit = pixels
            .chunks_exact(4)
            .filter(|p| u32::from(p[0]) + u32::from(p[1]) + u32::from(p[2]) > 24)
            .count();
        if lit * 1000 < pixels.len() / 4 {
            return false;
        }
        // Full or half size, 24-bit, rows bottom-up (as GetDIBits gave them), padded to 4 bytes.
        let step = if FULL.load(std::sync::atomic::Ordering::Relaxed) {
            1
        } else {
            2
        };
        let (hw, hh) = (w / step, h / step);
        let row = ((hw * 3 + 3) & !3) as usize;
        let mut out = Vec::with_capacity(54 + row * hh as usize);
        let file_size = 54 + row * hh as usize;
        out.extend_from_slice(b"BM");
        out.extend_from_slice(&(file_size as u32).to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&54u32.to_le_bytes());
        out.extend_from_slice(&40u32.to_le_bytes());
        out.extend_from_slice(&hw.to_le_bytes());
        out.extend_from_slice(&hh.to_le_bytes());
        out.extend_from_slice(&1u16.to_le_bytes());
        out.extend_from_slice(&24u16.to_le_bytes());
        out.extend_from_slice(&[0u8; 24]);
        for y in 0..hh {
            let start = out.len();
            for x in 0..hw {
                let at = (((step * y) * w + step * x) * 4) as usize;
                out.extend_from_slice(&pixels[at..at + 3]);
            }
            out.resize(start + row, 0);
        }
        let _ = std::fs::create_dir_all(dir.join("shots"));
        std::fs::write(dir.join("shots").join(format!("{name}.bmp")), out).is_ok()
    }
}
