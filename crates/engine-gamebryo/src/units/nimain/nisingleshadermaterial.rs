//! `NiMain/nisingleshadermaterial.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiSingleShaderMaterial::ms_RTTI` (the class's `NiRTTI` record).
const NI_SINGLE_SHADER_MATERIAL_RTTI: u32 = 0x011f_6048;

// Translated from 00aa06e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSingleShaderMaterial::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_single_shader_material_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SINGLE_SHADER_MATERIAL_RTTI)
}

/// `NiMaterial::NiMaterial` (`00a8f950`, `__thiscall`: the address of the name), the
/// `NiMaterial` destructor body `00a8f810` and `00a8fe20` (cdecl: the address of a name; the
/// material already made under it, or null).
const MATERIAL_CONSTRUCT: u32 = 0x00a8_f950;
const MATERIAL_DESTRUCTOR: u32 = 0x00a8_f810;
const MATERIAL_FIND: u32 = 0x00a8_fe20;
/// `NiShaderFactory::GetInstance` (`00a9fff0`) and the factory's virtuals +0x8 (three words:
/// the shader name, the implementation, a flag; the shader) and +0x24 (one word: a shader;
/// answers a byte).
const SHADER_FACTORY_GET_INSTANCE: u32 = 0x00a9_fff0;
const FACTORY_RETRIEVE_SHADER_SLOT: u32 = 0x8;
const FACTORY_IS_DEFAULT_SLOT: u32 = 0x24;
/// The shader's name address (`00a62660`, a body that answers `this + 8`; the engine map puts a
/// folded name on it), its implementation (`00cc0760`, a body that answers the word at +0xc),
/// and its virtuals +0x48 and +0x4c (no words; answer a byte) and +0x4 (`DeleteThis`).
const SHADER_NAME_ADDRESS: u32 = 0x00a6_2660;
const SHADER_IMPLEMENTATION: u32 = 0x00cc_0760;
const SHADER_IS_INITIALIZED_SLOT: u32 = 0x48;
const SHADER_INITIALIZE_SLOT: u32 = 0x4c;
const DELETE_THIS_SLOT: u32 = 0x4;
/// `00a5b690` (cdecl, a C string: its fixed-string handle), the empty handle's holder
/// (`0109b220`), the allocation `00aa13e0`, the sized delete `00aa1460` and the
/// `InterlockedIncrement`/`InterlockedDecrement` import slots.
const FIXED_STRING_FROM: u32 = 0x00a5_b690;
const FIXED_STRING_EMPTY: u32 = 0x0109_b220;
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiSingleShaderMaterial`'s vtable (`010a21dc`), size, shader name (+0x0c, a fixed-string
/// handle), cached shader (+0x10) and the byte (+0x14) that keeps it loaded.
const NI_SINGLE_SHADER_MATERIAL_VTABLE: u32 = 0x010a_21dc;
const NI_SINGLE_SHADER_MATERIAL_SIZE: u32 = 0x18;
const SHADER_NAME: u32 = 0x0c;
const CACHED_SHADER: u32 = 0x10;
const KEEP_LOADED: u32 = 0x14;

/// A fixed-string handle for a C string: the empty one for null, `00a5b690` otherwise.
fn handle_for(e: &mut Engine, name: Ptr) -> u32 {
    if name.is_null() {
        e.mem.u32(FIXED_STRING_EMPTY)
    } else {
        e.call(FIXED_STRING_FROM, &args![name]).u32()
    }
}

/// A reference given up (the object deleted through its virtual +4 at the last one).
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, DELETE_THIS_SLOT, &args![]);
    }
}

/// A fixed-string handle given up (nothing for the empty one).
fn release_handle(e: &mut Engine, handle: u32) {
    if handle != e.mem.u32(FIXED_STRING_EMPTY) {
        e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
    }
}

// Translated from 00aa06f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSingleShaderMaterial::NiSingleShaderMaterial` (Xbox PDB; `__thiscall`: the address of
/// the material name, the shader name (C string), a load byte): `NiMaterial`'s constructor
/// (`00a8f950`), the vtable `010a21dc`, the shader name's handle, no cached shader and the
/// keep byte clear. With the load byte the factory's shader of that name is retrieved (its
/// virtual +8 with -1 and 0) and its reference taken and given up again at once (deleted when
/// that was the last). Answers `this`.
pub fn ni_single_shader_material_ni_single_shader_material(
    e: &mut Engine,
    this: Ptr,
    name: Ptr,
    shader_name: Ptr,
    load: u8,
) -> Ptr {
    let base = this.addr();
    e.call(MATERIAL_CONSTRUCT, &args![this, name]);
    e.mem.set_u32(base, NI_SINGLE_SHADER_MATERIAL_VTABLE);
    let handle = handle_for(e, shader_name);
    e.mem.set_u32(base + SHADER_NAME, handle);
    e.mem.set_u32(base + CACHED_SHADER, 0);
    e.mem.set_u8(base + KEEP_LOADED, 0);
    if load != 0 {
        let factory = e.call(SHADER_FACTORY_GET_INSTANCE, &args![]).u32();
        let shader = e
            .vcall(
                factory,
                FACTORY_RETRIEVE_SHADER_SLOT,
                &args![shader_name, 0xffff_ffffu32, 0u32],
            )
            .u32();
        if shader != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![shader + 4]);
            release(e, shader);
        }
    }
    this
}

// Translated from 00aa0770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSingleShaderMaterial::IsShaderCurrent` (Xbox PDB; `__thiscall`, six words, the first the
/// shader): whether there is a shader and its name (`00a62660`) is this material's shader name
/// handle. The other five words are not read.
#[allow(clippy::too_many_arguments)]
pub fn ni_single_shader_material_is_shader_current(
    e: &mut Engine,
    this: Ptr,
    shader: Ptr,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> bool {
    if shader.is_null() {
        return false;
    }
    let name = e.call(SHADER_NAME_ADDRESS, &args![shader]).u32();
    e.mem.u32(name) == e.mem.u32(this.addr() + SHADER_NAME)
}

// Translated from 00aa07a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSingleShaderMaterial::GetCurrentShader` (Xbox PDB; `__thiscall`, five words, the fifth
/// the implementation): the cached shader when its implementation (`00cc0760`) is the one
/// asked for, or when -1 is asked for and the factory takes it as the default (its virtual
/// +0x24). Otherwise, with a factory, the shader it retrieves by name (virtual +8 with the
/// implementation and 1) when that is initialized (its virtual +0x48) or initializes (+0x4c);
/// 0 otherwise. The cache is not updated.
pub fn ni_single_shader_material_get_current_shader(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    implementation: u32,
) -> Ptr {
    let base = this.addr();
    let factory = e.call(SHADER_FACTORY_GET_INSTANCE, &args![]).u32();
    let cached = e.mem.u32(base + CACHED_SHADER);
    if cached != 0 {
        if e.call(SHADER_IMPLEMENTATION, &args![cached]).u32() == implementation {
            return Ptr::new(e.mem.u32(base + CACHED_SHADER));
        }
        if implementation == 0xffff_ffff {
            if factory == 0 {
                return Ptr::NULL;
            }
            let cached = e.mem.u32(base + CACHED_SHADER);
            if e.vcall(factory, FACTORY_IS_DEFAULT_SLOT, &args![cached])
                .bool()
            {
                return Ptr::new(e.mem.u32(base + CACHED_SHADER));
            }
        }
    }
    if factory == 0 {
        return Ptr::NULL;
    }
    let name = e.mem.u32(base + SHADER_NAME);
    let shader = e
        .vcall(
            factory,
            FACTORY_RETRIEVE_SHADER_SLOT,
            &args![name, implementation, 1u32],
        )
        .u32();
    if shader == 0 {
        return Ptr::NULL;
    }
    if e.vcall(shader, SHADER_IS_INITIALIZED_SLOT, &args![]).bool()
        || e.vcall(shader, SHADER_INITIALIZE_SLOT, &args![]).bool()
    {
        return Ptr::new(shader);
    }
    Ptr::NULL
}

// Translated from 00aa0830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSingleShaderMaterial::UnloadShaders` (Xbox PDB): unless the keep byte (+0x14) is set,
/// the cached shader's reference is given up (deleted at the last) and the cache cleared.
pub fn ni_single_shader_material_unload_shaders(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u8(base + KEEP_LOADED) != 0 {
        return;
    }
    let shader = e.mem.u32(base + CACHED_SHADER);
    if shader != 0 {
        release(e, shader);
        e.mem.set_u32(base + CACHED_SHADER, 0);
    }
}

// Translated from 00aa0870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSingleShaderMaterial::_scalar_deleting_destructor_` (Xbox PDB): the vtable, the cached
/// shader's reference given up (the field is left), the shader name's handle given up,
/// `NiMaterial`'s destructor body (`00a8f810`), then the sized delete of 0x18 bytes when bit 0
/// of `flags` is set. Answers `this`.
pub fn ni_single_shader_material_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, NI_SINGLE_SHADER_MATERIAL_VTABLE);
    let shader = e.mem.u32(base + CACHED_SHADER);
    if shader != 0 {
        release(e, shader);
    }
    let handle = e.mem.u32(base + SHADER_NAME);
    release_handle(e, handle);
    e.call(MATERIAL_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, NI_SINGLE_SHADER_MATERIAL_SIZE]);
    }
    this
}

// Translated from 00aa08d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSingleShaderMaterial::Create` (Xbox PDB; cdecl: the shader name (C string), a word whose
/// low byte is the load flag): the material already made under that name (`00a8fe20` with a
/// temporary handle, given up after) when there is one; otherwise a new material (0x18 bytes,
/// the constructor with a second temporary handle for the name, given up after, and the
/// word), or null when the allocation fails.
pub fn ni_single_shader_material_create(e: &mut Engine, shader_name: Ptr, load: u32) -> Ptr {
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let handle = handle_for(e, shader_name);
        e.mem.set_u32(slot, handle);
        let found = e.call(MATERIAL_FIND, &args![slot]).u32();
        release_handle(e, handle);
        if found != 0 {
            return Ptr::new(found);
        }
        let block = e
            .call(NI_ALLOCATE, &args![NI_SINGLE_SHADER_MATERIAL_SIZE])
            .u32();
        if block == 0 {
            return Ptr::NULL;
        }
        let handle = handle_for(e, shader_name);
        e.mem.set_u32(slot, handle);
        let material = ni_single_shader_material_ni_single_shader_material(
            e,
            Ptr::new(block),
            Ptr::new(slot),
            shader_name,
            load as u8,
        );
        let handle = e.mem.u32(slot);
        release_handle(e, handle);
        material
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa06e0, ni_single_shader_material_get_rtti() -> Ptr),
        entry!(
            0x00aa_06f0,
            ni_single_shader_material_ni_single_shader_material(Ptr, Ptr, Ptr, u8) -> Ptr
        ),
        entry!(
            0x00aa_0770,
            ni_single_shader_material_is_shader_current(Ptr, Ptr, u32, u32, u32, u32, u32) -> bool
        ),
        entry!(
            0x00aa_07a0,
            ni_single_shader_material_get_current_shader(Ptr, u32, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x00aa_0830, ni_single_shader_material_unload_shaders(Ptr)),
        entry!(
            0x00aa_0870,
            ni_single_shader_material_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00aa_08d0, ni_single_shader_material_create(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_single_shader_material_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00aa06e0, &args![]).u32(),
            NI_SINGLE_SHADER_MATERIAL_RTTI
        );
    }
}

#[cfg(test)]
mod n019_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    const RETRIEVE_DOUBLE: u32 = 0x0071_3000;
    const DEFAULT_DOUBLE: u32 = 0x0071_3010;
    const INITIALIZED_DOUBLE: u32 = 0x0071_3020;
    const INITIALIZE_DOUBLE: u32 = 0x0071_3030;
    const DELETE_DOUBLE: u32 = 0x0071_3040;

    fn counting(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            word(count)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            word(count)
        });
    }

    /// A shader (implementation word at +0xc, name handle at +8) whose virtuals are doubles. The
    /// name and implementation getters are doubles of the two small bodies.
    fn shader(e: &mut Engine, implementation: u32, name: u32) -> u32 {
        e.register(SHADER_NAME_ADDRESS, |_, a| word(a[0] + 8));
        e.register(SHADER_IMPLEMENTATION, |e, a| word(e.mem.u32(a[0] + 0xc)));
        let shader = e.mem.alloc(0x20);
        let vtable = e.mem.alloc(0x60);
        e.mem.set_u32(shader, vtable);
        e.mem.set_u32(vtable + DELETE_THIS_SLOT, DELETE_DOUBLE);
        e.mem
            .set_u32(vtable + SHADER_IS_INITIALIZED_SLOT, INITIALIZED_DOUBLE);
        e.mem
            .set_u32(vtable + SHADER_INITIALIZE_SLOT, INITIALIZE_DOUBLE);
        e.mem.set_u32(shader + 0xc, implementation);
        e.mem.set_u32(shader + 8, name);
        shader
    }

    /// The factory, its retrieval answering `retrieved`.
    fn factory(e: &mut Engine, retrieved: u32) -> u32 {
        let factory = e.mem.alloc(8);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(factory, vtable);
        e.mem
            .set_u32(vtable + FACTORY_RETRIEVE_SHADER_SLOT, RETRIEVE_DOUBLE);
        e.mem
            .set_u32(vtable + FACTORY_IS_DEFAULT_SLOT, DEFAULT_DOUBLE);
        e.register_double(SHADER_FACTORY_GET_INSTANCE, move |_, _| word(factory));
        e.register_double(RETRIEVE_DOUBLE, move |_, _| word(retrieved));
        factory
    }

    fn doubles(e: &mut Engine) {
        counting(e);
        e.map(FIXED_STRING_EMPTY, 4);
        e.register(MATERIAL_CONSTRUCT, |_, _| Ret::default());
        e.register(MATERIAL_DESTRUCTOR, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        e.register(DELETE_DOUBLE, |_, _| Ret::default());
        e.register(FIXED_STRING_FROM, |e, _| word(e.mem.alloc(0x10) + 8));
    }

    #[test]
    fn construct_with_and_without_loading() {
        let mut e = Engine::new();
        doubles(&mut e);
        let s = shader(&mut e, 0, 0);
        factory(&mut e, s);
        let material = e.mem.alloc(NI_SINGLE_SHADER_MATERIAL_SIZE);
        e.call_log = Some(vec![]);
        let made = e
            .call(0x00aa_06f0, &args![material, 0x100u32, 0x200u32, 1u32])
            .u32();
        assert_eq!(made, material);
        assert_eq!(e.mem.u32(material), NI_SINGLE_SHADER_MATERIAL_VTABLE);
        assert_eq!(
            calls_to(&e, MATERIAL_CONSTRUCT),
            vec![vec![material, 0x100]]
        );
        assert_ne!(e.mem.u32(material + SHADER_NAME), 0);
        assert_eq!(
            calls_to(&e, RETRIEVE_DOUBLE)[0][1..],
            [0x200, 0xffff_ffff, 0]
        );
        // The shader was held and let go: its last reference deleted it.
        assert_eq!(calls_to(&e, DELETE_DOUBLE), vec![vec![s]]);
        assert_eq!(e.mem.u32(material + CACHED_SHADER), 0);
        let other = e.mem.alloc(NI_SINGLE_SHADER_MATERIAL_SIZE);
        e.call(0x00aa_06f0, &args![other, 0x100u32, 0u32, 0u32]);
        assert_eq!(e.mem.u32(other + SHADER_NAME), 0);
        assert_eq!(calls_to(&e, RETRIEVE_DOUBLE).len(), 1);
    }

    #[test]
    fn is_shader_current_compares_names() {
        let mut e = Engine::new();
        let material = e.mem.alloc(NI_SINGLE_SHADER_MATERIAL_SIZE);
        e.mem.set_u32(material + SHADER_NAME, 0x4444);
        let s = shader(&mut e, 0, 0x4444);
        let current = |e: &mut Engine, shader: u32| {
            e.call(
                0x00aa_0770,
                &args![material, shader, 0u32, 0u32, 0u32, 0u32, 0u32],
            )
            .bool()
        };
        assert!(current(&mut e, s));
        assert!(!current(&mut e, 0));
        e.mem.set_u32(s + 8, 0x5555);
        assert!(!current(&mut e, s));
    }

    #[test]
    fn get_current_shader_uses_the_cache_or_the_factory() {
        let mut e = Engine::new();
        let cached = shader(&mut e, 3, 0);
        let fresh = shader(&mut e, 5, 0);
        let f = factory(&mut e, fresh);
        e.register(DEFAULT_DOUBLE, |_, _| word(1));
        e.register(INITIALIZED_DOUBLE, |_, _| word(0));
        e.register(INITIALIZE_DOUBLE, |_, _| word(1));
        let material = e.mem.alloc(NI_SINGLE_SHADER_MATERIAL_SIZE);
        e.mem.set_u32(material + SHADER_NAME, 0x4444);
        e.mem.set_u32(material + CACHED_SHADER, cached);
        let get = |e: &mut Engine, implementation: u32| {
            e.call(
                0x00aa_07a0,
                &args![material, 0u32, 0u32, 0u32, 0u32, implementation],
            )
            .u32()
        };
        e.call_log = Some(vec![]);
        assert_eq!(get(&mut e, 3), cached);
        assert_eq!(get(&mut e, 0xffff_ffff), cached);
        assert_eq!(calls_to(&e, DEFAULT_DOUBLE), vec![vec![f, cached]]);
        assert_eq!(get(&mut e, 5), fresh);
        assert_eq!(calls_to(&e, RETRIEVE_DOUBLE)[0][1..], [0x4444, 5, 1]);
        e.register(INITIALIZE_DOUBLE, |_, _| word(0));
        assert_eq!(get(&mut e, 5), 0);
        e.register(INITIALIZED_DOUBLE, |_, _| word(1));
        assert_eq!(get(&mut e, 5), fresh);
        // No factory: only an exact cached match answers.
        e.register(SHADER_FACTORY_GET_INSTANCE, |_, _| word(0));
        assert_eq!(get(&mut e, 0xffff_ffff), 0);
        assert_eq!(get(&mut e, 3), cached);
        assert_eq!(get(&mut e, 4), 0);
    }

    #[test]
    fn unload_and_destroy() {
        let mut e = Engine::new();
        doubles(&mut e);
        let s = shader(&mut e, 0, 0);
        e.mem.set_u32(s + 4, 2);
        let material = e.mem.alloc(NI_SINGLE_SHADER_MATERIAL_SIZE);
        e.mem.set_u32(material + CACHED_SHADER, s);
        e.mem.set_u8(material + KEEP_LOADED, 1);
        e.call_log = Some(vec![]);
        e.call(0x00aa_0830, &args![material]);
        assert_eq!(e.mem.u32(material + CACHED_SHADER), s);
        e.mem.set_u8(material + KEEP_LOADED, 0);
        e.call(0x00aa_0830, &args![material]);
        assert_eq!(e.mem.u32(material + CACHED_SHADER), 0);
        assert_eq!(e.mem.u32(s + 4), 1);
        e.mem.set_u32(material + CACHED_SHADER, s);
        let name = e.mem.alloc(0x10) + 8;
        e.mem.set_u32(name - 8, 1);
        e.mem.set_u32(material + SHADER_NAME, name);
        assert_eq!(e.call(0x00aa_0870, &args![material, 1u32]).u32(), material);
        assert_eq!(calls_to(&e, DELETE_DOUBLE), vec![vec![s]]);
        assert_eq!(e.mem.u32(name - 8), 0);
        assert_eq!(calls_to(&e, MATERIAL_DESTRUCTOR), vec![vec![material]]);
        assert_eq!(
            calls_to(&e, SIZED_DELETE),
            vec![vec![material, NI_SINGLE_SHADER_MATERIAL_SIZE]]
        );
    }

    #[test]
    fn create_finds_or_makes_the_material() {
        let mut e = Engine::new();
        doubles(&mut e);
        factory(&mut e, 0);
        e.register(MATERIAL_FIND, |_, _| word(0x7777));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_08d0, &args![0x200u32, 1u32]).u32(), 0x7777);
        assert_eq!(calls_to(&e, NI_ALLOCATE).len(), 0);
        e.register(MATERIAL_FIND, |_, _| word(0));
        e.register(NI_ALLOCATE, |e, a| word(e.mem.alloc(a[0])));
        let made = e.call(0x00aa_08d0, &args![0x200u32, 0x100u32]).u32();
        assert_eq!(e.mem.u32(made), NI_SINGLE_SHADER_MATERIAL_VTABLE);
        // The load byte of 0x100 is 0: nothing retrieved.
        assert_eq!(calls_to(&e, RETRIEVE_DOUBLE).len(), 0);
        // The temporary handles were given up: four made, three let go (the entry keeps one).
        assert_eq!(calls_to(&e, FIXED_STRING_FROM).len(), 4);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT).len(), 3);
        e.register(NI_ALLOCATE, |_, _| word(0));
        assert_eq!(e.call(0x00aa_08d0, &args![0u32, 0u32]).u32(), 0);
    }
}
