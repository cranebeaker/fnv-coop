//! `NiXenonRenderer/nid3dshader.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::{release_fixed_string, GET_FIXED_STRING};

/// The five words `fn_00e81100` stores at `0126f738`.
const SHADER_FUNCTIONS: [u32; 5] = [
    0x00e8_0860,
    0x00e8_0a50,
    0x00e8_0c50,
    0x00e8_0de0,
    0x00e8_0f70,
];

// Translated from 00e81100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShader::_SDMInit` (Xbox PDB; run, by a jump, at the end of the start-up
/// `00e91dc0`; no arguments): sets the static fixed string at `0126f750` from the literal
/// at `010ef37c` when it does not hold the literal's address already (releasing the old
/// handle), then stores five function addresses at `0126f738` to `0126f748`.
pub fn ni_d3d_shader_sdm_init(e: &mut Engine) {
    let old = e.mem.u32(0x0126_f750);
    if old != 0x010e_f37c {
        let handle = e.call(GET_FIXED_STRING, &args![0x010e_f37cu32]).u32();
        e.mem.set_u32(0x0126_f750, handle);
        release_fixed_string(e, old);
    }
    for (index, function) in SHADER_FUNCTIONS.into_iter().enumerate() {
        e.mem.set_u32(0x0126_f738 + 4 * index as u32, function);
    }
}

/// `NiD3DShader`'s RTTI record (`0126f754`).
const SHADER_RTTI_0126F754: u32 = 0x0126_f754;

// Translated from 00e7c680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShader::GetRTTI` (slot +8 of the `NiD3DShader` vtable `010eeda4`; unnamed in the
/// engine map), `thiscall`, `this` not read: the RTTI record at `0126f754`.
pub fn ni_d3d_shader_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    SHADER_RTTI_0126F754
}

/// The sized free `00aa1460` (`cdecl`: block, size).
const FREE_SIZED_00AA1460: u32 = 0x00aa_1460;

// Translated from 00e7c690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShader::scalar deleting destructor` (slot 0 of the vtable `010eeda4`; unnamed in the
/// engine map), `thiscall` with the flags: the destructor ([`fn_00e806c0`]), then with bit 0
/// the 0x3c bytes freed (`00aa1460`). Answers `this`.
pub fn ni_d3d_shader_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00e806c0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_SIZED_00AA1460, &args![this, 0x3cu32]);
    }
    this
}

/// The shift that places the shader constant's second word (`01101294`, a word whose low byte
/// is the count).
const ENTRY_SHIFT_01101294: u32 = 0x0110_1294;

// Translated from 00e80640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSCMExtraData::AddEntry` (engine map), `thiscall` with five words: a key, a second word,
/// the list (0 or 1), a value and a byte. Appends an 8-byte entry to the list's array (`this`
/// +0x24 + 4 * list, its count at +0x18 + 4 * list): the key ORed with the second word (0xff
/// when the byte is set) shifted left by `01101294`'s low byte (as `SHL` masks it, the low five
/// bits), then the value; the count goes up by one.
pub fn ni_scm_extra_data_add_entry(
    e: &mut Engine,
    this: Ptr,
    key: u32,
    second: u32,
    list: u32,
    value: u32,
    byte: u8,
) {
    let base = this.addr();
    let high = if byte == 0 { second } else { 0xff };
    let shift = e.mem.u32(ENTRY_SHIFT_01101294) & 0x1f;
    let count_at = base.wrapping_add(list.wrapping_mul(4)).wrapping_add(0x18);
    let array_at = base.wrapping_add(list.wrapping_mul(4)).wrapping_add(0x24);
    let array = e.mem.u32(array_at);
    let count = e.mem.u32(count_at);
    let word = (high << shift) | key;
    e.mem
        .set_u32(array.wrapping_add(count.wrapping_mul(8)), word);
    let count = e.mem.u32(count_at);
    let array = e.mem.u32(array_at);
    e.mem.set_u32(
        array.wrapping_add(count.wrapping_mul(8)).wrapping_add(4),
        value,
    );
    // `INC dword ptr`: the count read again.
    let count = e.mem.u32(count_at);
    e.mem.set_u32(count_at, count.wrapping_add(1));
}

/// The texture stage state setter `00e88fc0` (unit `run_00e88fc0`; `thiscall` on the stage's
/// object at +0x10, three words: the state, its value and the caller's fourth word) and the
/// value `00e80a50` takes from `0126f0e4`.
const SET_STAGE_STATE_00E88FC0: u32 = 0x00e8_8fc0;
const TRANSFORM_FLAGS_0126F0E4: u32 = 0x0126_f0e4;

/// A product term: the float at `loaded` (`FLD dword`) times the float memory operand at
/// `operand` (`FMUL dword`).
fn product(e: &Engine, (loaded, operand): (u32, u32)) -> f64 {
    x87_mul(x87_load32(e.mem.u32(loaded)), x87_m32(e.mem.u32(operand)))
}

/// One element as the code sums it: the first two products (`FADDP`), then the third.
fn sum3(e: &Engine, terms: [(u32, u32); 3]) -> f64 {
    x87_add(
        x87_add(product(e, terms[0]), product(e, terms[1])),
        product(e, terms[2]),
    )
}

/// The destination offset and the three products (loaded, operand) of one element.
type Element = (u32, [(u32, u32); 3]);

/// The rotation part shared by `00e80860`, `00e80c50`, `00e80de0` and `00e80f70`: the 3x3 at
/// `rotation` times the one at `matrices` +0xa40 (rows of 16 bytes), in the code's operand
/// order.
fn rotation_elements(r: u32, m: u32) -> [Element; 9] {
    [
        (
            0x1c,
            [(r + 4, m + 0xa44), (m + 0xa40, r), (m + 0xa48, r + 8)],
        ),
        (
            0x2c,
            [(r, m + 0xa50), (r + 4, m + 0xa54), (r + 8, m + 0xa58)],
        ),
        (
            0x3c,
            [(r, m + 0xa60), (r + 4, m + 0xa64), (r + 8, m + 0xa68)],
        ),
        (
            0x20,
            [
                (r + 0x10, m + 0xa44),
                (m + 0xa40, r + 0xc),
                (m + 0xa48, r + 0x14),
            ],
        ),
        (
            0x30,
            [
                (r + 0x10, m + 0xa54),
                (r + 0xc, m + 0xa50),
                (m + 0xa58, r + 0x14),
            ],
        ),
        (
            0x40,
            [
                (r + 0x10, m + 0xa64),
                (r + 0xc, m + 0xa60),
                (m + 0xa68, r + 0x14),
            ],
        ),
        (
            0x24,
            [
                (m + 0xa40, r + 0x18),
                (r + 0x1c, m + 0xa44),
                (r + 0x20, m + 0xa48),
            ],
        ),
        (
            0x34,
            [
                (m + 0xa54, r + 0x1c),
                (r + 0x18, m + 0xa50),
                (r + 0x20, m + 0xa58),
            ],
        ),
        (
            0x44,
            [
                (m + 0xa64, r + 0x1c),
                (r + 0x18, m + 0xa60),
                (r + 0x20, m + 0xa68),
            ],
        ),
    ]
}

/// Stores each element's sum as a float at `stage` + its offset, in order.
fn store_elements(e: &mut Engine, stage: u32, elements: &[Element]) {
    for (offset, terms) in elements {
        let value = x87_store32(sum3(e, *terms));
        e.mem.set_u32(stage.wrapping_add(*offset), value);
    }
}

/// Stores the translated row: each element's sum plus the float at `translation` + 4 * i
/// (`FADD dword`).
fn store_translated(e: &mut Engine, stage: u32, translation: u32, elements: &[Element; 3]) {
    for (i, (offset, terms)) in elements.iter().enumerate() {
        let added = x87_m32(e.mem.u32(translation.wrapping_add(4 * i as u32)));
        let value = x87_store32(x87_add(sum3(e, *terms), added));
        e.mem.set_u32(stage.wrapping_add(*offset), value);
    }
}

/// The `FLDZ` and four stores ending all five: +0x58, +0x48, +0x38, +0x28 cleared.
fn clear_column(e: &mut Engine, stage: u32) {
    for offset in [0x58u32, 0x48, 0x38, 0x28] {
        e.mem.set_u32(stage.wrapping_add(offset), 0);
    }
}

/// The two stage states the five set (`00e88fc0` on the stage's +0x10 with the fourth word):
/// state 0xb with `first`, then (after `flag` goes to the stage's byte +0x5e) state 0x18 with
/// `second`.
fn set_states(e: &mut Engine, stage: u32, extra: u32, first: u32, flag: u8, second: u32) {
    let target = e.mem.u32(stage.wrapping_add(0x10));
    e.call(
        SET_STAGE_STATE_00E88FC0,
        &args![target, 0xbu32, first, extra],
    );
    let target = e.mem.u32(stage.wrapping_add(0x10));
    e.mem.set_u8(stage.wrapping_add(0x5e), flag);
    e.call(
        SET_STAGE_STATE_00E88FC0,
        &args![target, 0x18u32, second, extra],
    );
}

// Translated from 00e80860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshader.cpp`; the first of the five functions
/// `_SDMInit` stores at `0126f738`), `cdecl` with five words: a 3x3 float matrix, a float
/// translation, a stage, a word passed on and an object holding a 3x3 matrix at +0xa40 and a
/// translation row at +0xa70 (rows of 16 bytes). Sets the stage states 0xb to 0x20000 and 0x18
/// to 2 (the stage's byte +0x5e set to 1 between them); then the stage's 4x4 at +0x1c gets the
/// product of the two matrices (x87, in the code's operand order), its row +0x4c the matrix
/// times the object's +0xa70 row plus the translation, and +0x28, +0x38, +0x48, +0x58 0.
pub fn fn_00e80860(
    e: &mut Engine,
    rotation: u32,
    translation: u32,
    stage: u32,
    extra: u32,
    matrices: u32,
) {
    set_states(e, stage, extra, 0x20000, 1, 2);
    let (r, m) = (rotation, matrices);
    store_elements(e, stage, &rotation_elements(r, m));
    let translated = [
        (
            0x4c,
            [(r, m + 0xa70), (r + 4, m + 0xa74), (r + 8, m + 0xa78)],
        ),
        (
            0x50,
            [
                (r + 0x10, m + 0xa74),
                (m + 0xa70, r + 0xc),
                (r + 0x14, m + 0xa78),
            ],
        ),
        (
            0x54,
            [
                (m + 0xa70, r + 0x18),
                (r + 0x1c, m + 0xa74),
                (r + 0x20, m + 0xa78),
            ],
        ),
    ];
    store_translated(e, stage, translation, &translated);
    clear_column(e, stage);
}

// Translated from 00e80a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshader.cpp`; the second of the five), `cdecl` with
/// the same five words as [`fn_00e80860`]: stage state 0xb to 0x20000, the byte +0x5e set unless
/// the word at `0126f0e4` is 0 or 0x100, and state 0x18 to that word; then the same product,
/// translated row and cleared column, each element summed in this function's own operand order.
pub fn fn_00e80a50(
    e: &mut Engine,
    rotation: u32,
    translation: u32,
    stage: u32,
    extra: u32,
    matrices: u32,
) {
    let target = e.mem.u32(stage.wrapping_add(0x10));
    e.call(
        SET_STAGE_STATE_00E88FC0,
        &args![target, 0xbu32, 0x20000u32, extra],
    );
    let flags = e.mem.u32(TRANSFORM_FLAGS_0126F0E4);
    let flag = u8::from(flags != 0 && flags != 0x100);
    e.mem.set_u8(stage.wrapping_add(0x5e), flag);
    let target = e.mem.u32(stage.wrapping_add(0x10));
    e.call(
        SET_STAGE_STATE_00E88FC0,
        &args![target, 0x18u32, flags, extra],
    );
    let (r, m) = (rotation, matrices);
    let elements = [
        (
            0x1c,
            [(m + 0xa40, r), (m + 0xa44, r + 4), (m + 0xa48, r + 8)],
        ),
        (
            0x2c,
            [(r + 4, m + 0xa54), (r, m + 0xa50), (r + 8, m + 0xa58)],
        ),
        (
            0x3c,
            [(r + 4, m + 0xa64), (r, m + 0xa60), (r + 8, m + 0xa68)],
        ),
        (
            0x20,
            [
                (m + 0xa40, r + 0xc),
                (m + 0xa44, r + 0x10),
                (r + 0x14, m + 0xa48),
            ],
        ),
        (
            0x30,
            [
                (r + 0x10, m + 0xa54),
                (r + 0xc, m + 0xa50),
                (r + 0x14, m + 0xa58),
            ],
        ),
        (
            0x40,
            [
                (r + 0x10, m + 0xa64),
                (r + 0xc, m + 0xa60),
                (r + 0x14, m + 0xa68),
            ],
        ),
        (
            0x24,
            [
                (m + 0xa40, r + 0x18),
                (m + 0xa44, r + 0x1c),
                (r + 0x20, m + 0xa48),
            ],
        ),
        (
            0x34,
            [
                (r + 0x18, m + 0xa50),
                (m + 0xa54, r + 0x1c),
                (r + 0x20, m + 0xa58),
            ],
        ),
        (
            0x44,
            [
                (r + 0x18, m + 0xa60),
                (m + 0xa64, r + 0x1c),
                (r + 0x20, m + 0xa68),
            ],
        ),
    ];
    store_elements(e, stage, &elements);
    let translated = [
        (
            0x4c,
            [(r, m + 0xa70), (m + 0xa74, r + 4), (m + 0xa78, r + 8)],
        ),
        (
            0x50,
            [
                (r + 0xc, m + 0xa70),
                (m + 0xa74, r + 0x10),
                (r + 0x14, m + 0xa78),
            ],
        ),
        (
            0x54,
            [
                (m + 0xa74, r + 0x1c),
                (r + 0x18, m + 0xa70),
                (m + 0xa78, r + 0x20),
            ],
        ),
    ];
    store_translated(e, stage, translation, &translated);
    clear_column(e, stage);
}

/// The body of `00e80c50`, `00e80de0` and `00e80f70` (they differ in the two state values):
/// the states, the product of the two matrices, the translation copied into the row at +0x4c
/// (`FLD`/`FSTP`, a signalling NaN quieted) and the cleared column.
fn rotation_with_translation(
    e: &mut Engine,
    rotation: u32,
    translation: u32,
    stage: u32,
    extra: u32,
    matrices: u32,
    first: u32,
    second: u32,
) {
    set_states(e, stage, extra, first, 1, second);
    store_elements(e, stage, &rotation_elements(rotation, matrices));
    for i in 0..3u32 {
        let value = x87_quiet_bits(e.mem.u32(translation.wrapping_add(4 * i)));
        e.mem.set_u32(stage.wrapping_add(0x4c + 4 * i), value);
    }
    clear_column(e, stage);
}

// Translated from 00e80c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshader.cpp`; the third of the five), `cdecl` with the
/// same five words as [`fn_00e80860`]: stage states 0xb to 0x30000 and 0x18 to 2 (the byte +0x5e
/// set to 1 between them), the product of the two matrices into the stage's 4x4 at +0x1c (as
/// `00e80860` sums it), the translation copied into its row +0x4c and +0x28 to +0x58 cleared.
pub fn fn_00e80c50(
    e: &mut Engine,
    rotation: u32,
    translation: u32,
    stage: u32,
    extra: u32,
    matrices: u32,
) {
    rotation_with_translation(e, rotation, translation, stage, extra, matrices, 0x30000, 2);
}

// Translated from 00e80de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshader.cpp`; the fourth of the five): as
/// [`fn_00e80c50`] with the state 0x18 set to 3.
pub fn fn_00e80de0(
    e: &mut Engine,
    rotation: u32,
    translation: u32,
    stage: u32,
    extra: u32,
    matrices: u32,
) {
    rotation_with_translation(e, rotation, translation, stage, extra, matrices, 0x30000, 3);
}

// Translated from 00e80f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshader.cpp`; the fifth of the five): as
/// [`fn_00e80c50`] with the state 0xb set to 0x10000 and 0x18 to 3.
pub fn fn_00e80f70(
    e: &mut Engine,
    rotation: u32,
    translation: u32,
    stage: u32,
    extra: u32,
    matrices: u32,
) {
    rotation_with_translation(e, rotation, translation, stage, extra, matrices, 0x10000, 3);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e7_c680, ni_d3d_shader_get_rtti(Ptr) -> u32),
        entry!(
            0x00e7_c690,
            ni_d3d_shader_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00e8_0640,
            ni_scm_extra_data_add_entry(Ptr, u32, u32, u32, u32, u8)
        ),
        entry!(0x00e8_0860, fn_00e80860(u32, u32, u32, u32, u32)),
        entry!(0x00e8_0a50, fn_00e80a50(u32, u32, u32, u32, u32)),
        entry!(0x00e8_0c50, fn_00e80c50(u32, u32, u32, u32, u32)),
        entry!(0x00e8_0de0, fn_00e80de0(u32, u32, u32, u32, u32)),
        entry!(0x00e8_0f70, fn_00e80f70(u32, u32, u32, u32, u32)),
        entry!(0x00e8_1100, ni_d3d_shader_sdm_init()),
        entry!(0x00aa_1ac0, fn_00aa1ac0(Ptr) -> u32),
        entry!(0x00e8_0690, fn_00e80690()),
        entry!(0x00e8_06c0, fn_00e806c0(Ptr)),
        entry!(
            0x00e8_07d0,
            fn_00e807d0(Ptr, u32, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
    ]
}

// Translated from 00aa1ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Named `NiD3DShader::GetShaderDecl` in the engine map, `thiscall`, no stack words: answers
/// the word at +0x20. The linker shares this body with other getters of a word at +0x20 (the
/// archive code calls it as a file's buffer getter), so the name is not the only user.
pub fn fn_00aa1ac0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(0x20))
}

// Translated from 00e80690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshader.cpp`), no arguments: the shut-down of the
/// static fixed string `_SDMInit` sets. When `0126f750` holds a handle it gets the handle for
/// a null string (`00a5b690` with 0) and the old one is released (its count at -8 down,
/// unless it is the empty string's handle).
pub fn fn_00e80690(e: &mut Engine) {
    let old = e.mem.u32(0x0126_f750);
    if old == 0 {
        return;
    }
    let handle = e.call(GET_FIXED_STRING, &args![0u32]).u32();
    e.mem.set_u32(0x0126_f750, handle);
    release_fixed_string(e, old);
}

/// What the destructor `00e806c0` uses: the shader's table (`010eeda4`), the empty fixed
/// string (`0109b220`), the shader factory getter `00e91f10` (no arguments), the program's name
/// getter (`00cc0760`, engine map `NiD3DShaderProgram::GetName`; `thiscall`, no stack words),
/// the renderer's release of the shader `00e75310` (unit `nixenonrenderer.cpp`; `thiscall`, the
/// shader), the render-state block release `00e7f6b0` (unit `run_00e7ef80`; `cdecl`), the
/// current shader global (`0126f74c`), the interlocked decrement import and the
/// `NiD3DShaderInterface` destructor `00e81a50`.
const SHADER_VTABLE_010EEDA4: u32 = 0x010e_eda4;
const EMPTY_FIXED_STRING_0109B220: u32 = 0x0109_b220;
const SHADER_FACTORY_00E91F10: u32 = 0x00e9_1f10;
const GET_NAME_00CC0760: u32 = 0x00cc_0760;
const RENDERER_RELEASE_SHADER_00E75310: u32 = 0x00e7_5310;
const RELEASE_STATE_BLOCK_00E7F6B0: u32 = 0x00e7_f6b0;
const CURRENT_SHADER_0126F74C: u32 = 0x0126_f74c;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const SHADER_INTERFACE_DESTRUCT_00E81A50: u32 = 0x00e8_1a50;

// Translated from 00e806c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshader.cpp`): the `NiD3DShader` destructor,
/// `thiscall`, no stack words. Stores the class's table. With a name (+8) that is not the empty
/// string and whose length word (-4) is not 0: the shader factory (`00e91f10`) looks it up
/// (slot 0xc: the name and the name `00cc0760` answers for the shader) and removes it (slot
/// 0x1c with the same) when it finds this shader; with the byte +0x20 set the same with -1.
/// With a renderer (+0x18) it releases the shader (`00e75310`). Then the byte +0x10 is
/// cleared, the render-state block (+0x2c) released (`00e7f6b0`), +0x38 and the current shader
/// global `0126f74c` cleared, the objects at +0x34, +0x30 and +0x28 released (their count at +4
/// down, deleted through their slot 4 at 0), and the `NiD3DShaderInterface` destructor runs
/// (a tail jump).
pub fn fn_00e806c0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, SHADER_VTABLE_010EEDA4);
    let empty = e.mem.u32(EMPTY_FIXED_STRING_0109B220);
    // The name is compared with the empty string twice, read again for the second time.
    let named = e.mem.u32(base.wrapping_add(8)) != empty && {
        let name = e.mem.u32(base.wrapping_add(8));
        name != empty && e.mem.u32(name.wrapping_sub(4)) != 0
    };
    if named {
        let factory = e.call(SHADER_FACTORY_00E91F10, &args![]).u32();
        let name = e.mem.u32(base.wrapping_add(8));
        let other = e.call(GET_NAME_00CC0760, &args![base]).u32();
        if e.vcall(factory, 0xc, &args![name, other]).u32() == base {
            let name = e.mem.u32(base.wrapping_add(8));
            let other = e.call(GET_NAME_00CC0760, &args![base]).u32();
            e.vcall(factory, 0x1c, &args![name, other]);
        }
        if e.mem.u8(base.wrapping_add(0x20)) != 0 {
            let name = e.mem.u32(base.wrapping_add(8));
            if e.vcall(factory, 0xc, &args![name, 0xffff_ffffu32]).u32() == base {
                let name = e.mem.u32(base.wrapping_add(8));
                e.vcall(factory, 0x1c, &args![name, 0xffff_ffffu32]);
            }
        }
    }
    let renderer = e.mem.u32(base.wrapping_add(0x18));
    if renderer != 0 {
        e.call(RENDERER_RELEASE_SHADER_00E75310, &args![renderer, base]);
    }
    let block = e.mem.u32(base.wrapping_add(0x2c));
    e.mem.set_u8(base.wrapping_add(0x10), 0);
    e.call(RELEASE_STATE_BLOCK_00E7F6B0, &args![block]);
    e.mem.set_u32(base.wrapping_add(0x38), 0);
    e.mem.set_u32(CURRENT_SHADER_0126F74C, 0);
    for offset in [0x34u32, 0x30, 0x28] {
        let object = e.mem.u32(base.wrapping_add(offset));
        if object != 0
            && e.call(IMPORT_INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
                .u32()
                == 0
        {
            e.vcall(object, 4, &args![]);
        }
    }
    e.call(SHADER_INTERFACE_DESTRUCT_00E81A50, &args![base]);
}

/// The renderer's (+0x18) calls `00e807d0` makes: `00e6fe30` (unnamed; `thiscall`, five
/// words), `00e6fda0` (unnamed; `thiscall`, three words) and `00e6fbb0` (unnamed; `thiscall`,
/// two words).
const RENDERER_00E6FE30: u32 = 0x00e6_fe30;
const RENDERER_00E6FDA0: u32 = 0x00e6_fda0;
const RENDERER_00E6FBB0: u32 = 0x00e6_fbb0;

// Translated from 00e807d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshader.cpp`): a shader's set-up of the skinning
/// state, `thiscall` with eight words (only the second, third and seventh are read). With a
/// skin partition (the third): unless +0x38 is set, the renderer's (+0x18) `00e6fe30` with the
/// second and seventh words, 0, 4 and 0; then the object at +0x1c gets the partition's 16-bit
/// +0x24 (its slot 0x2c) and the renderer's `00e6fda0` runs with the second word, the partition
/// and the seventh. Without one: when +0x38 is clear or the renderer's byte +0x209 is set, the
/// object at +0x1c gets 0 (slot 0x2c) and the renderer's `00e6fbb0` runs with the seventh word
/// and 1. Answers 0.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e807d0(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    second: u32,
    partition: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
    seventh: u32,
    _unused_8: u32,
) -> u32 {
    let base = this.addr();
    if partition != 0 {
        if e.mem.u32(base.wrapping_add(0x38)) == 0 {
            let renderer = e.mem.u32(base.wrapping_add(0x18));
            e.call(
                RENDERER_00E6FE30,
                &args![renderer, second, seventh, 0u32, 4u32, 0u32],
            );
        }
        let target = e.mem.u32(base.wrapping_add(0x1c));
        let bones = u32::from(e.mem.u16(partition.wrapping_add(0x24)));
        e.vcall(target, 0x2c, &args![bones]);
        let renderer = e.mem.u32(base.wrapping_add(0x18));
        e.call(
            RENDERER_00E6FDA0,
            &args![renderer, second, partition, seventh],
        );
        return 0;
    }
    let renderer = e.mem.u32(base.wrapping_add(0x18));
    if e.mem.u32(base.wrapping_add(0x38)) == 0 || e.mem.u8(renderer.wrapping_add(0x209)) != 0 {
        let target = e.mem.u32(base.wrapping_add(0x1c));
        e.vcall(target, 0x2c, &args![0u32]);
        let renderer = e.mem.u32(base.wrapping_add(0x18));
        e.call(RENDERER_00E6FBB0, &args![renderer, seventh, 1u32]);
    }
    0
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn sets_the_string_once_and_the_five_functions() {
        let mut e = engine_with_counting();
        e.map(0x0126_f000, 0x1000);
        e.register(GET_FIXED_STRING, |_, a| (a[0] + 0x1000).into_ret());
        let empty = e.mem.u32(EMPTY_FIXED_STRING);
        e.mem.set_u32(0x0126_f750, empty);
        e.call_log = Some(vec![]);
        e.call(0x00e8_1100, &args![]);
        assert_eq!(e.mem.u32(0x0126_f750), 0x010e_f37c + 0x1000);
        assert_eq!(calls_to(&e, GET_FIXED_STRING), vec![vec![0x010e_f37c]]);
        for (index, function) in SHADER_FUNCTIONS.into_iter().enumerate() {
            assert_eq!(e.mem.u32(0x0126_f738 + 4 * index as u32), function);
        }
        // The word already equals the literal's address: no lookup.
        e.mem.set_u32(0x0126_f750, 0x010e_f37c);
        e.call(0x00e8_1100, &args![]);
        assert_eq!(calls_to(&e, GET_FIXED_STRING).len(), 1);
    }
}

#[cfg(test)]
mod b052_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    const LOOK_UP: u32 = 0x0eb0_000c;
    const REMOVE: u32 = 0x0eb0_001c;
    const SET_BONES: u32 = 0x0eb0_002c;
    const DELETE: u32 = 0x0eb0_0004;
    const TABLE: u32 = 0x0eb1_0000;

    #[test]
    fn fn_00aa1ac0_reads_the_word() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x24);
        e.mem.set_u32(this + 0x20, 0x55);
        assert_eq!(e.call(0x00aa_1ac0, &args![this]).u32(), 0x55);
    }

    #[test]
    fn fn_00e80690_releases_the_static_string() {
        let mut e = engine_with_counting();
        e.map(0x0126_f000, 0x1000);
        e.register(GET_FIXED_STRING, |_, _| 0x7770u32.into_ret());
        let handle = e.mem.alloc(0x10) + 8;
        e.mem.set_u32(0x0126_f750, handle);
        e.call_log = Some(vec![]);
        e.call(0x00e8_0690, &args![]);
        assert_eq!(e.mem.u32(0x0126_f750), 0x7770);
        assert_eq!(calls_to(&e, GET_FIXED_STRING), vec![vec![0]]);
        // Nothing held: nothing done.
        e.mem.set_u32(0x0126_f750, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e8_0690, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_00e806c0_unregisters_and_releases() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.map(0x0109_b000, 0x1000);
        e.mem.set_u32(EMPTY_FIXED_STRING_0109B220, 0x0eee_0000);
        let mut slots = vec![0u32; 8];
        slots[1] = DELETE;
        slots[3] = LOOK_UP;
        slots[7] = REMOVE;
        e.put_vtable(TABLE, &slots);
        let this = e.mem.alloc(0x3c);
        let factory = e.mem.alloc(4);
        e.mem.set_u32(factory, TABLE);
        e.register_double(SHADER_FACTORY_00E91F10, move |_, _| factory.into_ret());
        e.register_double(LOOK_UP, move |_, _| this.into_ret());
        e.register(GET_NAME_00CC0760, |_, _| 0x4e00u32.into_ret());
        e.register(IMPORT_INTERLOCKED_DECREMENT, |_, _| 0u32.into_ret());
        for no_op in [
            REMOVE,
            DELETE,
            RENDERER_RELEASE_SHADER_00E75310,
            RELEASE_STATE_BLOCK_00E7F6B0,
            SHADER_INTERFACE_DESTRUCT_00E81A50,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        let name = e.mem.alloc(0x10) + 8;
        e.mem.set_u32(name - 4, 3);
        e.mem.set_u32(this + 8, name);
        e.mem.set_u8(this + 0x20, 1);
        e.mem.set_u32(this + 0x18, 0x1800);
        e.mem.set_u32(this + 0x2c, 0x2c00);
        e.mem.set_u32(this + 0x38, 9);
        e.mem.set_u32(CURRENT_SHADER_0126F74C, 9);
        let objects: Vec<u32> = (0..3)
            .map(|_| {
                let object = e.mem.alloc(8);
                e.mem.set_u32(object, TABLE);
                object
            })
            .collect();
        for (offset, object) in [0x34u32, 0x30, 0x28].iter().zip(&objects) {
            e.mem.set_u32(this + offset, *object);
        }
        e.call_log = Some(vec![]);
        e.call(0x00e8_06c0, &args![this]);
        assert_eq!(
            calls(&e, LOOK_UP),
            vec![
                vec![factory, name, 0x4e00],
                vec![factory, name, 0xffff_ffff]
            ]
        );
        assert_eq!(
            calls(&e, REMOVE),
            vec![
                vec![factory, name, 0x4e00],
                vec![factory, name, 0xffff_ffff]
            ]
        );
        assert_eq!(
            calls(&e, RENDERER_RELEASE_SHADER_00E75310),
            vec![vec![0x1800, this]]
        );
        assert_eq!(calls(&e, RELEASE_STATE_BLOCK_00E7F6B0), vec![vec![0x2c00]]);
        assert_eq!(
            calls(&e, DELETE),
            vec![vec![objects[0]], vec![objects[1]], vec![objects[2]]]
        );
        assert_eq!(
            calls(&e, SHADER_INTERFACE_DESTRUCT_00E81A50),
            vec![vec![this]]
        );
        assert_eq!(e.mem.u32(this), SHADER_VTABLE_010EEDA4);
        assert_eq!(e.mem.u32(this + 0x38), 0);
        assert_eq!(e.mem.u32(CURRENT_SHADER_0126F74C), 0);
        // The empty name: no lookup; no renderer: no release.
        e.mem.set_u32(this + 8, 0x0eee_0000);
        e.mem.set_u32(this + 0x18, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e8_06c0, &args![this]);
        assert!(calls(&e, LOOK_UP).is_empty());
        assert!(calls(&e, RENDERER_RELEASE_SHADER_00E75310).is_empty());
    }

    #[test]
    fn fn_00e807d0_sets_up_skinning() {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0xc];
        slots[0xb] = SET_BONES;
        e.put_vtable(TABLE, &slots);
        for no_op in [
            SET_BONES,
            RENDERER_00E6FE30,
            RENDERER_00E6FDA0,
            RENDERER_00E6FBB0,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        let target = e.mem.alloc(4);
        e.mem.set_u32(target, TABLE);
        let renderer = e.mem.alloc(0x210);
        let this = e.mem.alloc(0x3c);
        e.mem.set_u32(this + 0x18, renderer);
        e.mem.set_u32(this + 0x1c, target);
        let partition = e.mem.alloc(0x28);
        e.mem.set_u16(partition + 0x24, 4);
        let run = |e: &mut Engine, partition: u32| {
            e.call(
                0x00e8_07d0,
                &args![this, 1u32, 2u32, partition, 4u32, 5u32, 6u32, 7u32, 8u32],
            )
            .u32()
        };
        e.call_log = Some(vec![]);
        assert_eq!(run(&mut e, partition), 0);
        assert_eq!(
            calls(&e, RENDERER_00E6FE30),
            vec![vec![renderer, 2, 7, 0, 4, 0]]
        );
        assert_eq!(calls(&e, SET_BONES), vec![vec![target, 4]]);
        assert_eq!(
            calls(&e, RENDERER_00E6FDA0),
            vec![vec![renderer, 2, partition, 7]]
        );
        // +0x38 set: no 00e6fe30.
        e.mem.set_u32(this + 0x38, 1);
        e.call_log = Some(vec![]);
        run(&mut e, partition);
        assert!(calls(&e, RENDERER_00E6FE30).is_empty());
        // No partition, +0x38 set and the renderer's byte clear: nothing.
        e.call_log = Some(vec![]);
        run(&mut e, 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.mem.set_u8(renderer + 0x209, 1);
        e.call_log = Some(vec![]);
        run(&mut e, 0);
        assert_eq!(calls(&e, SET_BONES), vec![vec![target, 0]]);
        assert_eq!(calls(&e, RENDERER_00E6FBB0), vec![vec![renderer, 7, 1]]);
    }
}

#[cfg(test)]
mod n008_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn get_rtti_answers_the_record() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e7_c680, &args![0u32]).u32(), 0x0126_f754);
    }

    #[test]
    fn scalar_deleting_destructor_runs_the_destructor_then_frees() {
        for (flags, frees) in [(1u32, true), (0, false)] {
            let mut e = Engine::new();
            e.map(0x0126_f000, 0x1000);
            e.map(0x0109_b000, 0x1000);
            for callee in [
                RELEASE_STATE_BLOCK_00E7F6B0,
                SHADER_INTERFACE_DESTRUCT_00E81A50,
                FREE_SIZED_00AA1460,
            ] {
                e.register(callee, |_, _| Ret::default());
            }
            // No name, renderer or objects: the destructor only stores and releases.
            let this = e.mem.alloc(0x3c);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00e7_c690, &args![this, flags]).u32(), this);
            assert_eq!(e.mem.u32(this), SHADER_VTABLE_010EEDA4);
            assert_eq!(
                calls(&e, SHADER_INTERFACE_DESTRUCT_00E81A50),
                vec![vec![this]]
            );
            let freed = if frees {
                vec![vec![this, 0x3c]]
            } else {
                vec![]
            };
            assert_eq!(calls(&e, FREE_SIZED_00AA1460), freed);
        }
    }

    #[test]
    fn add_entry_appends_to_the_chosen_list() {
        let mut e = Engine::new();
        e.map(0x0110_1000, 0x1000);
        e.mem.set_u32(ENTRY_SHIFT_01101294, 0x0000_0128);
        let this = e.mem.alloc(0x2c);
        let array = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x28, array);
        e.mem.set_u32(this + 0x1c, 1);
        e.call(
            0x00e8_0640,
            &args![this, 0x12u32, 0x34u32, 1u32, 0x99u32, 0u32],
        );
        // The shift is 0x28 & 0x1f = 8.
        assert_eq!(e.mem.u32(array + 8), 0x3412);
        assert_eq!(e.mem.u32(array + 12), 0x99);
        assert_eq!(e.mem.u32(this + 0x1c), 2);
        // The byte set: 0xff in place of the second word.
        e.call(
            0x00e8_0640,
            &args![this, 0x12u32, 0x34u32, 1u32, 0x77u32, 1u32],
        );
        assert_eq!(e.mem.u32(array + 16), 0xff12);
        assert_eq!(e.mem.u32(array + 20), 0x77);
        assert_eq!(e.mem.u32(this + 0x1c), 3);
    }

    /// A stage (its object +0x10 doubled), a 3x3 rotation, a translation and the matrix object;
    /// answers the engine, the four addresses.
    fn transform_rig() -> (Engine, u32, u32, u32, u32) {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.register(SET_STAGE_STATE_00E88FC0, |_, _| Ret::default());
        let stage = e.mem.alloc(0x60);
        e.mem.set_u32(stage + 0x10, 0x0e00_0010);
        for offset in [0x28u32, 0x38, 0x48, 0x58] {
            e.mem.set_u32(stage + offset, 0x4000_0000);
        }
        let rotation = e.mem.alloc(0x24);
        for i in 0..9u32 {
            e.mem.set_u32(rotation + 4 * i, (i as f32 + 1.0).to_bits());
        }
        let translation = e.mem.alloc(0xc);
        for i in 0..3u32 {
            e.mem
                .set_u32(translation + 4 * i, (10.0 * (i as f32 + 1.0)).to_bits());
        }
        let matrices = e.mem.alloc(0xa80);
        for row in 0..4u32 {
            for column in 0..3u32 {
                let value = (row * 3 + column) as f32 * 0.5 + 0.25;
                e.mem
                    .set_u32(matrices + 0xa40 + 16 * row + 4 * column, value.to_bits());
            }
        }
        e.call_log = Some(vec![]);
        (e, stage, rotation, translation, matrices)
    }

    fn float(e: &Engine, at: u32) -> f32 {
        f32::from_bits(e.mem.u32(at))
    }

    /// The stage's 3x3 at +0x1c, the products of rotation rows (stride 0xc) and matrix rows
    /// (stride 0x10), checked against plain arithmetic (every value is exact here).
    fn check_product(e: &Engine, stage: u32, rotation: u32, matrices: u32) {
        let at = |base: u32, index: u32| float(e, base + 4 * index);
        for row in 0..3u32 {
            for column in 0..3u32 {
                let mut sum = 0.0f32;
                for k in 0..3u32 {
                    sum += at(rotation, 3 * row + k) * at(matrices + 0xa40 + 16 * column, k);
                }
                assert_eq!(float(e, stage + 0x1c + 4 * row + 16 * column), sum);
            }
        }
    }

    #[test]
    fn fn_00e80860_multiplies_and_translates() {
        let (mut e, stage, rotation, translation, matrices) = transform_rig();
        e.call(
            0x00e8_0860,
            &args![rotation, translation, stage, 0x77u32, matrices],
        );
        assert_eq!(
            calls(&e, SET_STAGE_STATE_00E88FC0),
            vec![
                vec![0x0e00_0010, 0xb, 0x20000, 0x77],
                vec![0x0e00_0010, 0x18, 2, 0x77]
            ]
        );
        assert_eq!(e.mem.u8(stage + 0x5e), 1);
        check_product(&e, stage, rotation, matrices);
        for row in 0..3u32 {
            let mut sum = 0.0f32;
            for k in 0..3u32 {
                sum +=
                    float(&e, rotation + 4 * (3 * row + k)) * float(&e, matrices + 0xa70 + 4 * k);
            }
            sum += float(&e, translation + 4 * row);
            assert_eq!(float(&e, stage + 0x4c + 4 * row), sum);
        }
        for offset in [0x28u32, 0x38, 0x48, 0x58] {
            assert_eq!(e.mem.u32(stage + offset), 0);
        }
        // A signalling NaN loaded (FLD) is quieted and then wins over a smaller quiet operand:
        // the first product of +0x1c loads the rotation's second float.
        e.mem.set_u32(rotation + 4, 0x7f80_7000);
        e.mem.set_u32(matrices + 0xa44, 0x7fc0_0001);
        e.call(
            0x00e8_0860,
            &args![rotation, translation, stage, 0x77u32, matrices],
        );
        assert_eq!(e.mem.u32(stage + 0x1c), 0x7fc0_7000);
    }

    #[test]
    fn fn_00e80a50_takes_the_flag_from_the_global() {
        for (flags, flag) in [(0u32, 0u8), (0x100, 0), (3, 1)] {
            let (mut e, stage, rotation, translation, matrices) = transform_rig();
            e.mem.set_u32(TRANSFORM_FLAGS_0126F0E4, flags);
            e.call(
                0x00e8_0a50,
                &args![rotation, translation, stage, 0x77u32, matrices],
            );
            assert_eq!(e.mem.u8(stage + 0x5e), flag);
            assert_eq!(
                calls(&e, SET_STAGE_STATE_00E88FC0)[1],
                vec![0x0e00_0010, 0x18, flags, 0x77]
            );
            check_product(&e, stage, rotation, matrices);
            assert_eq!(e.mem.u32(stage + 0x28), 0);
        }
        // Here the first product of +0x1c loads the matrix float: the rotation's signalling
        // NaN stays the operand and loses to the quiet one.
        let (mut e, stage, rotation, translation, matrices) = transform_rig();
        e.mem.set_u32(rotation, 0x7f80_7000);
        e.mem.set_u32(matrices + 0xa40, 0x7fc0_0001);
        e.call(
            0x00e8_0a50,
            &args![rotation, translation, stage, 0x77u32, matrices],
        );
        assert_eq!(e.mem.u32(stage + 0x1c), 0x7fc0_0001);
    }

    #[test]
    fn rotation_with_copied_translation_variants() {
        for (addr, first, second) in [
            (0x00e8_0c50u32, 0x30000u32, 2u32),
            (0x00e8_0de0, 0x30000, 3),
            (0x00e8_0f70, 0x10000, 3),
        ] {
            let (mut e, stage, rotation, translation, matrices) = transform_rig();
            e.mem.set_u32(translation + 4, 0x7f80_0001);
            e.call(addr, &args![rotation, translation, stage, 5u32, matrices]);
            assert_eq!(
                calls(&e, SET_STAGE_STATE_00E88FC0),
                vec![
                    vec![0x0e00_0010, 0xb, first, 5],
                    vec![0x0e00_0010, 0x18, second, 5]
                ]
            );
            check_product(&e, stage, rotation, matrices);
            assert_eq!(float(&e, stage + 0x4c), 10.0);
            // The copy goes through FLD/FSTP: the signalling NaN comes out quiet.
            assert_eq!(e.mem.u32(stage + 0x50), 0x7fc0_0001);
            assert_eq!(float(&e, stage + 0x54), 30.0);
            assert_eq!(e.mem.u32(stage + 0x58), 0);
        }
    }
}
