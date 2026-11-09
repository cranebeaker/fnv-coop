//! A constant map's upload of its entries for a draw (`NiD3DShaderConstantMap::SetShaderConstants`),
//! what each shader's geometry set-up runs on its pixel and vertex maps (vtable +0x78), about 1,500
//! times a frame in the crowded street. The entries' own set-ups (`SetDefinedConstant` and the other
//! kinds, the map's vtable +0x8c to +0xa4) stay calls through the map's vtable, the uploads calls on the
//! device: the order of every call is the exe's.

use engine_core::ptr::Ptr;
use engine_core::{args, Engine};

/// The entry's kind (`m_uiFlags`, +0x14, its top four bits).
const KIND_MASK: u32 = 0xf000_0000;
const KIND_CONSTANT: u32 = 0x1000_0000;
const KIND_PREDEFINED: u32 = 0x2000_0000;
const KIND_ATTRIBUTE: u32 = 0x3000_0000;
const KIND_GLOBAL: u32 = 0x4000_0000;
const KIND_OPERATOR: u32 = 0x5000_0000;
const KIND_OBJECT: u32 = 0x6000_0000;
/// The device's `SetVertexShaderConstantF` (+0x178) and `SetPixelShaderConstantF` (+0x1b4).
const SLOT_VERTEX_CONSTANTS: u32 = 0x178;
const SLOT_PIXEL_CONSTANTS: u32 = 0x1b4;

/// Whether an entry is to be set: present, enabled (`m_bEnabled`, +8), with a register (+0x1c not -1)
/// or a variable name (+0x24).
fn settable(e: &Engine, entry: u32) -> bool {
    entry != 0
        && e.mem.u8(entry.wrapping_add(8)) != 0
        && (e.mem.u32(entry.wrapping_add(0x1c)) != 0xffff_ffff
            || e.mem.u32(entry.wrapping_add(0x24)) != 0)
}

// Typed from 00e826d0
/// `NiD3DShaderConstantMap::SetShaderConstants` (Xbox PDB), `thiscall` with eleven stack words:
/// the shader program (kept in `m_pkLastShaderProgram`, +0x30) and ten words passed on. Every
/// enabled entry (`m_bEnabled`, +8) with a register (+0x1c not -1) or a variable name (+0x24)
/// is set. First the predefined ones (kind 0x2) of `m_aspDefinedEntries` through
/// `SetDefinedConstant` (vtable +0x8c: program, entry, the ten words). Then by kind those of
/// `m_aspEntries`: constants (0x1) with extra word 1 or 2 go to the device (+0x38) as vertex
/// (slot +0x178) or pixel (slot +0x1b4) float constants (register, data, register count; a
/// negative result fails), other constants through `SetConstantConstant` (+0x90: program,
/// entry, the tenth word); attributes (0x3, +0x94: program, entry, the ten words, 0), globals
/// (0x4, +0x98: program, entry, the tenth word), operators (0x5, +0x9c: program, entry, the
/// second, third and sixth to tenth words) and objects (0x6, +0xa4: program, entry, the ten
/// words); predefined ones are skipped, any other kind fails. A failure is a non-zero result.
/// Answers whether any failed.
#[allow(clippy::too_many_arguments)]
pub fn set_shader_constants(
    e: &mut Engine,
    this: Ptr,
    program: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
    word_5: u32,
    word_6: u32,
    word_7: u32,
    word_8: u32,
    word_9: u32,
    word_10: u32,
    word_11: u32,
) -> bool {
    let base = this.addr();
    // m_pkLastShaderProgram.
    e.mem.set_u32(base.wrapping_add(0x30), program);
    let mut failed = false;
    // The predefined entries (m_aspDefinedEntries: elements +0x20, size +0x26), through
    // SetDefinedConstant.
    let size = u32::from(e.mem.u16(base.wrapping_add(0x26)));
    let mut index = 0u32;
    while index < size {
        let elements = e.mem.u32(base.wrapping_add(0x20));
        let entry = e.mem.u32(elements.wrapping_add(index.wrapping_mul(4)));
        index += 1;
        if !settable(e, entry) || e.mem.u32(entry.wrapping_add(0x14)) & KIND_MASK != KIND_PREDEFINED
        {
            continue;
        }
        let words = args![
            program, entry, word_2, word_3, word_4, word_5, word_6, word_7, word_8, word_9, word_10
        ];
        if e.vcall(base, 0x8c, &words).u32() != 0 {
            failed = true;
        }
    }
    // The other entries (m_aspEntries: elements +0x10, size +0x16), by kind.
    let size = u32::from(e.mem.u16(base.wrapping_add(0x16)));
    let mut index = 0u32;
    while index < size {
        let elements = e.mem.u32(base.wrapping_add(0x10));
        let entry = e.mem.u32(elements.wrapping_add(index.wrapping_mul(4)));
        index += 1;
        if !settable(e, entry) {
            continue;
        }
        let result = match e.mem.u32(entry.wrapping_add(0x14)) & KIND_MASK {
            KIND_PREDEFINED => continue,
            KIND_CONSTANT => match e.mem.u32(entry.wrapping_add(0x18)) {
                program_type @ (1 | 2) => {
                    let slot = if program_type == 1 {
                        SLOT_VERTEX_CONSTANTS
                    } else {
                        SLOT_PIXEL_CONSTANTS
                    };
                    let register = e.mem.u32(entry.wrapping_add(0x1c));
                    let data = e.mem.u32(entry.wrapping_add(0x30));
                    let count = e.mem.u32(entry.wrapping_add(0x20));
                    let device = e.mem.u32(base.wrapping_add(0x38));
                    u32::from(e.vcall(device, slot, &args![register, data, count]).i32() < 0)
                }
                _ => e.vcall(base, 0x90, &args![program, entry, word_10]).u32(),
            },
            KIND_ATTRIBUTE => {
                let words = args![
                    program, entry, word_2, word_3, word_4, word_5, word_6, word_7, word_8, word_9,
                    word_10, word_11, 0u32
                ];
                e.vcall(base, 0x94, &words).u32()
            }
            KIND_GLOBAL => e.vcall(base, 0x98, &args![program, entry, word_10]).u32(),
            KIND_OPERATOR => {
                let words =
                    args![program, entry, word_2, word_3, word_6, word_7, word_8, word_9, word_10];
                e.vcall(base, 0x9c, &words).u32()
            }
            KIND_OBJECT => {
                let words = args![
                    program, entry, word_2, word_3, word_4, word_5, word_6, word_7, word_8, word_9,
                    word_10
                ];
                e.vcall(base, 0xa4, &words).u32()
            }
            _ => 1,
        };
        if result != 0 {
            failed = true;
        }
    }
    failed
}
