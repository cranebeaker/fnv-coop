//! `d3d9-capture device`: how the retail Direct3D 9 runtime answers the device calls the game
//! makes, measured on a windowed HAL device with hardware vertex processing on a hidden window
//! (never shown). These answers are the runtime's general behaviour (they do not describe the
//! player's hardware); the platform's layer is written from them. Each line: the call, the case,
//! the HRESULT (hex), and what was read back afterwards where the case says so.
//!
//! `cargo run --release --manifest-path scripts/d3d9-capture/Cargo.toml -- device` (64-bit, as
//! the rest of the tool; the game runs the 32-bit runtime).
//!
//! Modes (a call that faults the runtime is run alone, so the line before it says it was
//! tried): `device` (the draws from memory and the scene checks), `device <case>` (one risky
//! draw: `null-data`, `type-0`, `type-7`, `type-9`, `count-0`, `null-declaration`), `device
//! outside` and `device outside-present` (what makes a draw from memory outside a scene fail),
//! `device fixed` (materials, lights, software vertex processing on a hardware and a mixed
//! device), `device fixed null-material|null-light|get-light`, and `device fixed
//! "texture:<case>"` (one UpdateTexture or StretchRect case, named as in `textures`).
use std::ffi::c_void;
use std::io::Write;

/// `IDirect3DDevice9` slots used here (d3d9.h order).
const RELEASE: usize = 2;
const BEGIN_SCENE: usize = 41;
const END_SCENE: usize = 42;
const CREATE_VERTEX_BUFFER: usize = 26;
const CREATE_INDEX_BUFFER: usize = 27;
const DRAW_INDEXED_PRIMITIVE: usize = 82;
const SET_INDICES: usize = 104;
const DRAW_PRIMITIVE: usize = 81;
const DRAW_PRIMITIVE_UP: usize = 83;
const SET_FVF: usize = 89;
const SET_VERTEX_DECLARATION: usize = 87;
const SET_STREAM_SOURCE: usize = 100;
const GET_STREAM_SOURCE: usize = 101;
const SET_RENDER_STATE: usize = 57;
const SET_TEXTURE: usize = 65;
const PRESENT: usize = 17;

type P = *mut c_void;

#[repr(C)]
#[derive(Clone, Copy)]
struct PresentParameters {
    width: u32,
    height: u32,
    format: u32,
    count: u32,
    multisample: u32,
    quality: u32,
    swap_effect: u32,
    window: P,
    windowed: i32,
    auto_depth: i32,
    depth_format: u32,
    flags: u32,
    refresh: u32,
    interval: u32,
}

unsafe fn slot(object: P, index: usize) -> usize {
    *(*(object as *const *const usize)).add(index)
}

/// Calls slot `index` of `object` with `args` (all words; at most 8).
unsafe fn call(object: P, index: usize, args: &[usize]) -> u32 {
    let f = slot(object, index);
    match args.len() {
        0 => std::mem::transmute::<usize, unsafe extern "system" fn(P) -> u32>(f)(object),
        1 => std::mem::transmute::<usize, unsafe extern "system" fn(P, usize) -> u32>(f)(object, args[0]),
        2 => std::mem::transmute::<usize, unsafe extern "system" fn(P, usize, usize) -> u32>(f)(object, args[0], args[1]),
        3 => std::mem::transmute::<usize, unsafe extern "system" fn(P, usize, usize, usize) -> u32>(f)(object, args[0], args[1], args[2]),
        4 => std::mem::transmute::<usize, unsafe extern "system" fn(P, usize, usize, usize, usize) -> u32>(f)(object, args[0], args[1], args[2], args[3]),
        5 => std::mem::transmute::<usize, unsafe extern "system" fn(P, usize, usize, usize, usize, usize) -> u32>(f)(object, args[0], args[1], args[2], args[3], args[4]),
        6 => std::mem::transmute::<usize, unsafe extern "system" fn(P, usize, usize, usize, usize, usize, usize) -> u32>(f)(object, args[0], args[1], args[2], args[3], args[4], args[5]),
        7 => std::mem::transmute::<usize, unsafe extern "system" fn(P, usize, usize, usize, usize, usize, usize, usize) -> u32>(f)(object, args[0], args[1], args[2], args[3], args[4], args[5], args[6]),
        8 => std::mem::transmute::<usize, unsafe extern "system" fn(P, usize, usize, usize, usize, usize, usize, usize, usize) -> u32>(f)(object, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7]),
        n => panic!("{n} arguments"),
    }
}

/// The stream 0 binding read back: buffer, offset, stride.
unsafe fn stream0(device: P) -> (usize, u32, u32) {
    let (mut buffer, mut offset, mut stride) = (0usize, 0xaaaa_aaaau32, 0xaaaa_aaaau32);
    call(device, GET_STREAM_SOURCE, &[0, &mut buffer as *mut usize as usize, &mut offset as *mut u32 as usize, &mut stride as *mut u32 as usize]);
    if buffer != 0 {
        // GetStreamSource adds a reference.
        call(buffer as P, RELEASE, &[]);
    }
    (buffer, offset, stride)
}

pub unsafe fn measure(o: &mut impl Write) {
    let instance = crate::GetModuleHandleA(std::ptr::null());
    let name = b"nv-rs d3d9-capture device\0";
    let class = crate::WndClassA {
        style: 3,
        procedure: crate::DefWindowProcA,
        class_extra: 0,
        window_extra: 0,
        instance,
        icon: std::ptr::null_mut(),
        cursor: std::ptr::null_mut(),
        background: std::ptr::null_mut(),
        menu_name: std::ptr::null(),
        class_name: name.as_ptr(),
    };
    crate::RegisterClassA(&class);
    // WS_OVERLAPPED without WS_VISIBLE: the window is never shown.
    let window = crate::CreateWindowExA(0, name.as_ptr(), name.as_ptr(), 0, 0, 0, 320, 240,
        std::ptr::null_mut(), std::ptr::null_mut(), instance, std::ptr::null_mut());
    assert!(!window.is_null());
    let d3d = crate::Direct3DCreate9(32) as P;
    assert!(!d3d.is_null());
    // D3DPRESENT_PARAMETERS: 320x240, D3DFMT_UNKNOWN back buffer, one, discard, windowed,
    // automatic D24S8 depth.
    let mut pp = PresentParameters {
        width: 320,
        height: 240,
        format: 0,
        count: 1,
        multisample: 0,
        quality: 0,
        swap_effect: 1,
        window,
        windowed: 1,
        auto_depth: 1,
        depth_format: 75,
        flags: 0,
        refresh: 0,
        interval: 0,
    };
    let mut device: P = std::ptr::null_mut();
    // IDirect3D9::CreateDevice (slot 16): adapter 0, HAL, window, hardware vertex processing.
    let hr = call(d3d, 16, &[0, 1, window as usize, 0x40, &mut pp as *mut PresentParameters as usize, &mut device as *mut P as usize]);
    writeln!(o, "CreateDevice\thal windowed hardware\t{hr:x}").unwrap(); o.flush().unwrap();
    assert!(!device.is_null(), "no device");

    // The vertices the movie player draws: XYZRHW | TEX1 (FVF 0x104), 0x18 bytes each.
    let vertices = [0f32; 6 * 6];
    let data = vertices.as_ptr() as usize;
    let up = |device: P, kind: usize, count: usize, data: usize, stride: usize| call(device, DRAW_PRIMITIVE_UP, &[kind, count, data, stride]);

    // `device outside`: which condition made the first draws from memory fail: each step on
    // the fresh device, outside any scene, reported in turn.
    if std::env::args().nth(2).as_deref() == Some("outside") {
        call(device, SET_FVF, &[0x104]);
        let mut buffer: P = std::ptr::null_mut();
        call(device, CREATE_VERTEX_BUFFER, &[0x18 * 6, 8, 0x104, 0, &mut buffer as *mut P as usize, 0]);
        call(device, SET_STREAM_SOURCE, &[0, buffer as usize, 0x18, 0x18]);
        let hr = up(device, 5, 2, data, 0x18);
        let after = stream0(device);
        writeln!(o, "DrawPrimitiveUP\tfresh, fvf, a vertex buffer on stream 0, never in a scene\t{hr:x}\tafter: buffer {} offset {:x} stride {:x}", after.0 != 0, after.1, after.2).unwrap(); o.flush().unwrap();
        call(device, SET_STREAM_SOURCE, &[0, 0, 0, 0]);
        writeln!(o, "DrawPrimitiveUP\tfresh, fvf, no stream 0, never in a scene\t{:x}", up(device, 5, 2, data, 0x18)).unwrap(); o.flush().unwrap();
        call(device, BEGIN_SCENE, &[]);
        call(device, END_SCENE, &[]);
        writeln!(o, "DrawPrimitiveUP\tfvf, no stream 0, after a scene\t{:x}", up(device, 5, 2, data, 0x18)).unwrap(); o.flush().unwrap();
        call(device, BEGIN_SCENE, &[]);
        writeln!(o, "DrawPrimitiveUP\tfvf, no stream 0, in a scene\t{:x}", up(device, 5, 2, data, 0x18)).unwrap(); o.flush().unwrap();
        call(device, END_SCENE, &[]);
        writeln!(o, "DrawPrimitiveUP\tfvf, after a draw in a scene\t{:x}", up(device, 5, 2, data, 0x18)).unwrap(); o.flush().unwrap();
        call(device, PRESENT, &[0, 0, 0, 0]);
        writeln!(o, "DrawPrimitiveUP\tfvf, after a draw in a scene and a Present\t{:x}", up(device, 5, 2, data, 0x18)).unwrap(); o.flush().unwrap();
        call(device, BEGIN_SCENE, &[]);
        call(device, END_SCENE, &[]);
        call(device, PRESENT, &[0, 0, 0, 0]);
        writeln!(o, "DrawPrimitiveUP\tfvf, after an empty scene and a Present\t{:x}", up(device, 5, 2, data, 0x18)).unwrap(); o.flush().unwrap();
        // A draw from a vertex buffer on a fresh device's history is the other check: a second
        // device below.
        o.flush().unwrap();
        return;
    }
    if std::env::args().nth(2).as_deref() == Some("outside-present") {
        call(device, SET_FVF, &[0x104]);
        writeln!(o, "Present\tfresh\t{:x}", call(device, PRESENT, &[0, 0, 0, 0])).unwrap(); o.flush().unwrap();
        writeln!(o, "DrawPrimitiveUP\tfvf, no stream 0, after a Present, never in a scene\t{:x}", up(device, 5, 2, data, 0x18)).unwrap(); o.flush().unwrap();
        o.flush().unwrap();
        return;
    }
    // `device queries`: the machine table's lines of which query types the device takes
    // (`CreateQuery` with a null out), `CreateQuery<TAB>type<TAB>hr`, to add to the table.
    if std::env::args().nth(2).as_deref() == Some("queries") {
        for kind in 0..=20usize {
            let hr = call(device, CREATE_QUERY, &[kind, 0]);
            writeln!(o, "CreateQuery\t{kind:x}\t{hr:x}").unwrap();
        }
        call(device, RELEASE, &[]);
        return;
    }
    if std::env::args().nth(2).as_deref() == Some("refs") {
        refs(o, device);
        return;
    }
    if std::env::args().nth(2).as_deref() == Some("reset") {
        reset(o, device, &pp);
        return;
    }
    if std::env::args().nth(2).as_deref() == Some("extra") {
        extra(o, device);
        call(device, RELEASE, &[]);
        return;
    }
    if std::env::args().nth(2).as_deref() == Some("fixed") {
        fixed_function(o, device);
        if let Some(case) = std::env::args().nth(3).filter(|a| a.starts_with("texture:")) {
            textures(o, device, &case[8..]);
            return;
        }
        call(device, RELEASE, &[]);
        // The same calls on a device with mixed vertex processing (0x80).
        let mut mixed: P = std::ptr::null_mut();
        let mut pp2 = PresentParameters { ..pp };
        let hr = call(d3d, 16, &[0, 1, window as usize, 0x80, &mut pp2 as *mut PresentParameters as usize, &mut mixed as *mut P as usize]);
        writeln!(o, "CreateDevice\thal windowed mixed\t{hr:x}").unwrap(); o.flush().unwrap();
        if !mixed.is_null() {
            software_vertex_processing(o, mixed, "mixed");
            call(mixed, RELEASE, &[]);
        }
        call(d3d, RELEASE, &[]);
        return;
    }
    // Without an FVF or a declaration.
    let hr = up(device, 5, 2, data, 0x18);
    writeln!(o, "DrawPrimitiveUP\tno fvf, outside a scene\t{hr:x}").unwrap(); o.flush().unwrap();
    call(device, SET_FVF, &[0x104]);
    let hr = up(device, 5, 2, data, 0x18);
    writeln!(o, "DrawPrimitiveUP\toutside a scene\t{hr:x}").unwrap(); o.flush().unwrap();
    call(device, BEGIN_SCENE, &[]);
    for (case, kind, count, d, stride) in [
        ("strip 2", 5usize, 2usize, data, 0x18usize),
        ("list 6 (movie quads)", 2, 6, data, 0x18),
        ("type 1 points", 1, 2, data, 0x18),
        ("type 3 lines", 3, 1, data, 0x18),
        ("stride 0", 5, 2, data, 0),
        ("stride 4", 5, 2, data, 4),
    ] {
        let hr = up(device, kind, count, d, stride);
        writeln!(o, "DrawPrimitiveUP\tin a scene, {case}\t{hr:x}").unwrap(); o.flush().unwrap();
    }
    // Stream 0 after a draw from memory: with a vertex buffer bound before.
    let mut buffer: P = std::ptr::null_mut();
    let hr = call(device, CREATE_VERTEX_BUFFER, &[0x18 * 6, 8, 0x104, 0, &mut buffer as *mut P as usize, 0]);
    writeln!(o, "CreateVertexBuffer\t0x90 bytes, write-only, default pool\t{hr:x}").unwrap(); o.flush().unwrap();
    call(device, SET_STREAM_SOURCE, &[0, buffer as usize, 0x18, 0x18]);
    let before = stream0(device);
    writeln!(o, "GetStreamSource\tafter SetStreamSource\tbuffer {} offset {:x} stride {:x}", before.0 != 0, before.1, before.2).unwrap(); o.flush().unwrap();
    let hr = up(device, 5, 2, data, 0x18);
    let after = stream0(device);
    writeln!(o, "DrawPrimitiveUP\tin a scene, a vertex buffer on stream 0\t{hr:x}\tafter: buffer {} offset {:x} stride {:x}", after.0 != 0, after.1, after.2).unwrap(); o.flush().unwrap();

    // The cases that may fault inside the runtime, one per run (`device <case>`): the line
    // before the call says it was tried; EndScene and Present follow, each reported.
    let risky = std::env::args().nth(2).unwrap_or_default();
    if !risky.is_empty() {
        writeln!(o, "DrawPrimitiveUP\tin a scene, {risky}\ttrying").unwrap(); o.flush().unwrap();
        call(device, SET_STREAM_SOURCE, &[0, buffer as usize, 0x18, 0x18]);
        let (kind, count, d) = match risky.as_str() {
            "null-data" => (5, 2, 0),
            "type-0" => (0, 2, data),
            "type-7" => (7, 2, data),
            "type-9" => (9, 2, data),
            "count-0" => (5, 0, data),
            _ => {
                call(device, SET_VERTEX_DECLARATION, &[0]);
                (5, 2, data)
            }
        };
        let hr = up(device, kind, count, d, 0x18);
        let after = stream0(device);
        writeln!(o, "DrawPrimitiveUP\tin a scene, {risky}\t{hr:x}\tafter: buffer {} offset {:x} stride {:x}", after.0 != 0, after.1, after.2).unwrap(); o.flush().unwrap();
        let hr = call(device, END_SCENE, &[]);
        writeln!(o, "EndScene\tafter {risky}\t{hr:x}").unwrap(); o.flush().unwrap();
        let hr = call(device, PRESENT, &[0, 0, 0, 0]);
        writeln!(o, "Present\tafter {risky}\t{hr:x}").unwrap(); o.flush().unwrap();
        return;
    }
    writeln!(o, "EndScene\tafter the draws\t{:x}", call(device, END_SCENE, &[])).unwrap(); o.flush().unwrap();
    // A draw from a vertex buffer outside a scene.
    call(device, SET_STREAM_SOURCE, &[0, buffer as usize, 0, 0x18]);
    let hr = call(device, DRAW_PRIMITIVE, &[5, 0, 2]);
    writeln!(o, "DrawPrimitive\toutside a scene\t{hr:x}").unwrap(); o.flush().unwrap();
    // An indexed draw outside a scene (16-bit indices, write-only, default pool).
    let mut indices: P = std::ptr::null_mut();
    let hr = call(device, CREATE_INDEX_BUFFER, &[12, 8, 101, 0, &mut indices as *mut P as usize, 0]);
    writeln!(o, "CreateIndexBuffer\t12 bytes, 16-bit, write-only, default pool\t{hr:x}").unwrap(); o.flush().unwrap();
    call(device, SET_INDICES, &[indices as usize]);
    let hr = call(device, DRAW_INDEXED_PRIMITIVE, &[4, 0, 0, 4, 0, 2]);
    writeln!(o, "DrawIndexedPrimitive\toutside a scene\t{hr:x}").unwrap(); o.flush().unwrap();
    call(device, SET_INDICES, &[0]);
    call(indices, RELEASE, &[]);
    // Stream 0 after a draw from memory that fails: outside a scene, and with stride 0.
    call(device, SET_STREAM_SOURCE, &[0, buffer as usize, 0x18, 0x18]);
    let hr = up(device, 5, 2, data, 0x18);
    let after = stream0(device);
    writeln!(o, "DrawPrimitiveUP\toutside a scene, a vertex buffer on stream 0\t{hr:x}\tafter: buffer {} offset {:x} stride {:x}", after.0 != 0, after.1, after.2).unwrap(); o.flush().unwrap();
    call(device, BEGIN_SCENE, &[]);
    call(device, SET_STREAM_SOURCE, &[0, buffer as usize, 0x18, 0x18]);
    let hr = up(device, 5, 2, data, 0);
    let after = stream0(device);
    writeln!(o, "DrawPrimitiveUP\tin a scene, stride 0, a vertex buffer on stream 0\t{hr:x}\tafter: buffer {} offset {:x} stride {:x}", after.0 != 0, after.1, after.2).unwrap(); o.flush().unwrap();
    let hr = call(device, BEGIN_SCENE, &[]);
    writeln!(o, "BeginScene\tinside a scene\t{hr:x}").unwrap(); o.flush().unwrap();
    call(device, END_SCENE, &[]);
    let hr = call(device, END_SCENE, &[]);
    writeln!(o, "EndScene\toutside a scene\t{hr:x}").unwrap(); o.flush().unwrap();
    // State calls outside a scene.
    let hr = call(device, SET_RENDER_STATE, &[7, 0]);
    writeln!(o, "SetRenderState\toutside a scene\t{hr:x}").unwrap(); o.flush().unwrap();
    let hr = call(device, SET_TEXTURE, &[0, 0]);
    writeln!(o, "SetTexture\toutside a scene, null\t{hr:x}").unwrap(); o.flush().unwrap();
    let hr = call(device, PRESENT, &[0, 0, 0, 0]);
    writeln!(o, "Present\toutside a scene\t{hr:x}").unwrap(); o.flush().unwrap();
    call(device, SET_STREAM_SOURCE, &[0, 0, 0, 0]);
    call(buffer, RELEASE, &[]);
    let refs = call(device, RELEASE, &[]);
    writeln!(o, "Release\tdevice\t{refs:x}").unwrap(); o.flush().unwrap();
    call(d3d, RELEASE, &[]);
    crate::DestroyWindow(window);
    crate::UnregisterClassA(name.as_ptr(), instance);
}

const SET_MATERIAL: usize = 49;
const SET_LIGHT: usize = 51;
const GET_LIGHT: usize = 52;
const LIGHT_ENABLE: usize = 53;
const GET_LIGHT_ENABLE: usize = 54;
const SET_SOFTWARE_VERTEX_PROCESSING: usize = 77;
const GET_SOFTWARE_VERTEX_PROCESSING: usize = 78;
const CREATE_TEXTURE: usize = 23;
const UPDATE_TEXTURE: usize = 31;
const STRETCH_RECT: usize = 34;
const CREATE_RENDER_TARGET: usize = 28;
const CREATE_OFFSCREEN_PLAIN_SURFACE: usize = 36;
const TEXTURE_GET_SURFACE_LEVEL: usize = 18;

fn words(w: &[u32]) -> String {
    w.iter().map(|x| format!("{x:08x}")).collect::<Vec<_>>().join(" ")
}

/// A `D3DLIGHT9` (26 words): type, diffuse, specular, ambient (4 floats each), position,
/// direction (3 each), range, falloff, attenuation 0..2, theta, phi.
fn light(kind: u32) -> [u32; 26] {
    let f = |v: f32| v.to_bits();
    let mut l = [0u32; 26];
    l[0] = kind;
    for w in l.iter_mut().take(13).skip(1) {
        *w = f(1.0);
    }
    l[18] = f(1.0); // direction (0, 0, 1)
    l[19] = f(100.0); // range
    l[20] = f(1.0); // falloff
    l[21] = f(1.0); // attenuation 0
    l[24] = f(0.5); // theta
    l[25] = f(1.0); // phi
    l
}

unsafe fn fixed_function(o: &mut impl Write, device: P) {
    let mut material = [0u32; 17];
    for w in material.iter_mut().take(16) {
        *w = 1f32.to_bits();
    }
    let null = std::env::args().nth(3).unwrap_or_default();
    if null == "null-material" || null == "null-light" {
        writeln!(o, "{null}\ttrying").unwrap();
        o.flush().unwrap();
        let hr = if null == "null-material" { call(device, SET_MATERIAL, &[0]) } else { call(device, SET_LIGHT, &[0, 0]) };
        writeln!(o, "{null}\t{hr:x}").unwrap(); o.flush().unwrap();
        return;
    }
    for (case, ptr) in [("a material", material.as_mut_ptr() as usize)] {
        writeln!(o, "SetMaterial\t{case}\t{:x}", call(device, SET_MATERIAL, &[ptr])).unwrap(); o.flush().unwrap();
    }
    let f = |v: f32| v.to_bits();
    let mut cases: Vec<(&str, u32, [u32; 26])> = vec![
        ("point", 0, light(1)),
        ("spot", 1, light(2)),
        ("directional", 2, light(3)),
        ("type 0", 3, light(0)),
        ("type 4", 3, light(4)),
        ("index 9", 9, light(1)),
    ];
    let edit = |kind: u32, at: usize, value: u32| {
        let mut l = light(kind);
        l[at] = value;
        l
    };
    cases.push(("point, attenuation all 0", 4, edit(1, 21, 0)));
    cases.push(("point, range -1", 4, edit(1, 19, f(-1.0))));
    cases.push(("point, range 0", 4, edit(1, 19, 0)));
    cases.push(("point, range 1e30", 4, edit(1, 19, f(1e30))));
    cases.push(("directional, direction 0", 4, edit(3, 18, 0)));
    cases.push(("spot, theta above phi", 4, edit(2, 24, f(2.0))));
    cases.push(("spot, phi above pi", 4, edit(2, 25, f(4.0))));
    cases.push(("point, attenuation 0 negative", 4, edit(1, 21, f(-1.0))));
    cases.push(("point, NaN diffuse", 4, edit(1, 1, 0x7fc0_0000)));
    for (case, index, l) in &cases {
        let hr = call(device, SET_LIGHT, &[*index as usize, l.as_ptr() as usize]);
        writeln!(o, "SetLight\t{case} at {index}\t{hr:x}").unwrap(); o.flush().unwrap();
    }
    let mut back = [0xaaaa_aaaau32; 26];
    if std::env::args().nth(3).as_deref() == Some("get-light") {
        writeln!(o, "GetLight\tat 0 (a point light set)\t{:x}\t{}", call(device, GET_LIGHT, &[0, back.as_mut_ptr() as usize]), words(&back)).unwrap(); o.flush().unwrap();
        writeln!(o, "GetLight\tat 7 never set\t{:x}", call(device, GET_LIGHT, &[7, back.as_mut_ptr() as usize])).unwrap(); o.flush().unwrap();
        return;
    }
    for (case, index, enable) in [
        ("set light 0, TRUE", 0usize, 1usize),
        ("set light 0, FALSE", 0, 0),
        ("set light 0, 2", 0, 2),
        ("never set 7, TRUE", 7, 1),
        ("never set 8, FALSE", 8, 0),
        ("index 9 set, TRUE", 9, 1),
    ] {
        let hr = call(device, LIGHT_ENABLE, &[index, enable]);
        let mut on = 0xaaaa_aaaau32;
        let hr2 = call(device, GET_LIGHT_ENABLE, &[index, &mut on as *mut u32 as usize]);
        writeln!(o, "LightEnable\t{case}\t{hr:x}\tGetLightEnable {hr2:x} {on:x}").unwrap(); o.flush().unwrap();
    }
    let hr = call(device, GET_LIGHT, &[7, back.as_mut_ptr() as usize]);
    writeln!(o, "GetLight\tat 7 after LightEnable\t{hr:x}\t{}", words(&back)).unwrap(); o.flush().unwrap();
    for index in 10..20usize {
        let hr = call(device, LIGHT_ENABLE, &[index, 1]);
        writeln!(o, "LightEnable\tTRUE, light {index}\t{hr:x}").unwrap(); o.flush().unwrap();
    }
    software_vertex_processing(o, device, "hardware");
}

unsafe fn software_vertex_processing(o: &mut impl Write, device: P, kind: &str) {
    for value in [1usize, 0, 2] {
        let hr = call(device, SET_SOFTWARE_VERTEX_PROCESSING, &[value]);
        let now = call(device, GET_SOFTWARE_VERTEX_PROCESSING, &[]);
        writeln!(o, "SetSoftwareVertexProcessing\t{kind} device, {value}\t{hr:x}\tGet {now:x}").unwrap(); o.flush().unwrap();
    }
}

unsafe fn textures(o: &mut impl Write, device: P, only: &str) {
    let texture = |w: usize, h: usize, levels: usize, usage: usize, format: usize, pool: usize| {
        let mut t: P = std::ptr::null_mut();
        call(device, CREATE_TEXTURE, &[w, h, levels, usage, format, pool, &mut t as *mut P as usize, 0]);
        t
    };
    // UpdateTexture: system memory to default pool, A8R8G8B8 (21), 64x64.
    let sys = texture(64, 64, 0, 0, 21, 2);
    let sys1 = texture(64, 64, 1, 0, 21, 2);
    let sys_small = texture(32, 32, 0, 0, 21, 2);
    let sys_x8 = texture(64, 64, 0, 0, 22, 2);
    let def = texture(64, 64, 0, 0, 21, 0);
    let def1 = texture(64, 64, 1, 0, 21, 0);
    let dynamic = texture(64, 64, 1, 0x200, 21, 0);
    let managed = texture(64, 64, 0, 0, 21, 1);
    for (case, src, dst) in [
        ("sysmem full chain to default full chain", sys, def),
        ("sysmem 1 level to default 1 level", sys1, def1),
        ("sysmem 1 level to dynamic default 1 level", sys1, dynamic),
        ("sysmem full chain to default 1 level", sys, def1),
        ("sysmem 1 level to default full chain", sys1, def),
        ("sysmem 32x32 to default 64x64", sys_small, def),
        ("sysmem X8R8G8B8 to default A8R8G8B8", sys_x8, def),
        ("default to default", def, def1),
        ("sysmem to managed", sys, managed),
        ("sysmem to sysmem", sys, sys1),
        ("null source", std::ptr::null_mut(), def),
    ] {
        if case != only {
            continue;
        }
        writeln!(o, "UpdateTexture\t{case}\ttrying").unwrap(); o.flush().unwrap();
        writeln!(o, "UpdateTexture\t{case}\t{:x}", call(device, UPDATE_TEXTURE, &[src as usize, dst as usize])).unwrap(); o.flush().unwrap();
    }
    let level = |t: P| {
        let mut s: P = std::ptr::null_mut();
        call(t, TEXTURE_GET_SURFACE_LEVEL, &[0, &mut s as *mut P as usize]);
        s
    };
    let target = |w: usize, h: usize, format: usize| {
        let mut s: P = std::ptr::null_mut();
        call(device, CREATE_RENDER_TARGET, &[w, h, format, 0, 0, 0, &mut s as *mut P as usize, 0]);
        s
    };
    let plain = |pool: usize| {
        let mut s: P = std::ptr::null_mut();
        call(device, CREATE_OFFSCREEN_PLAIN_SURFACE, &[64, 64, 21, pool, &mut s as *mut P as usize, 0]);
        s
    };
    let rt_a = target(64, 64, 21);
    let rt_b = target(64, 64, 21);
    let rt_big = target(128, 128, 21);
    let rt_x8 = target(64, 64, 22);
    let rt_tex_level = level(texture(64, 64, 1, 1, 21, 0));
    let def_level = level(def1);
    let sys_level = level(sys1);
    let plain_default = plain(0);
    let plain_sys = plain(2);
    let rect = [0i32, 0, 32, 32];
    let r = rect.as_ptr() as usize;
    for (case, src, src_rect, dst, dst_rect, filter) in [
        ("render target to render target", rt_a, 0, rt_b, 0, 0usize),
        ("render target to render target, linear", rt_a, 0, rt_b, 0, 2),
        ("render target to render target, point", rt_a, 0, rt_b, 0, 1),
        ("render target to a bigger render target", rt_a, 0, rt_big, 0, 0),
        ("render target to render target, rects", rt_a, r, rt_b, r, 0),
        ("render target to X8R8G8B8 render target", rt_a, 0, rt_x8, 0, 0),
        ("render target to render-target texture level", rt_a, 0, rt_tex_level, 0, 0),
        ("render-target texture level to render target", rt_tex_level, 0, rt_a, 0, 0),
        ("default plain surface to render target", plain_default, 0, rt_a, 0, 0),
        ("render target to default plain surface", rt_a, 0, plain_default, 0, 0),
        ("default texture level to render target", def_level, 0, rt_a, 0, 0),
        ("render target to default texture level", rt_a, 0, def_level, 0, 0),
        ("sysmem plain surface to render target", plain_sys, 0, rt_a, 0, 0),
        ("sysmem texture level to render target", sys_level, 0, rt_a, 0, 0),
        ("render target to itself", rt_a, 0, rt_a, 0, 0),
        ("render target to null", rt_a, 0, std::ptr::null_mut(), 0, 0),
    ] {
        if case != only {
            continue;
        }
        writeln!(o, "StretchRect\t{case}\ttrying").unwrap(); o.flush().unwrap();
        let hr = call(device, STRETCH_RECT, &[src as usize, src_rect, dst as usize, dst_rect, filter]);
        writeln!(o, "StretchRect\t{case}\t{hr:x}").unwrap(); o.flush().unwrap();
    }
    if only != "in a scene" {
        return;
    }
    call(device, BEGIN_SCENE, &[]);
    let hr = call(device, STRETCH_RECT, &[rt_a as usize, 0, rt_b as usize, 0, 0]);
    writeln!(o, "StretchRect\tin a scene, render target to render target\t{hr:x}").unwrap(); o.flush().unwrap();
    let hr = call(device, UPDATE_TEXTURE, &[sys1 as usize, def1 as usize]);
    writeln!(o, "UpdateTexture\tin a scene, sysmem 1 level to default 1 level\t{hr:x}").unwrap(); o.flush().unwrap();
    call(device, END_SCENE, &[]);
}

/// `IDirect3DDevice9` slots of `device extra`, and `IDirect3DQuery9`'s / `IDirect3DSurface9`'s.
const TEST_COOPERATIVE_LEVEL: usize = 3;
const GET_BACK_BUFFER: usize = 18;
const GET_RENDER_TARGET_DATA: usize = 32;
const COLOR_FILL: usize = 35;
const CREATE_QUERY: usize = 118;
const QUERY_GET_TYPE: usize = 4;
const QUERY_GET_DATA_SIZE: usize = 5;
const QUERY_ISSUE: usize = 6;
const QUERY_GET_DATA: usize = 7;
const SURFACE_LOCK_RECT: usize = 13;
const SURFACE_UNLOCK_RECT: usize = 14;

/// `device extra`: queries (support by type, event and occlusion life cycle), offscreen plain
/// surfaces, `GetRenderTargetData`, `ColorFill`, `TestCooperativeLevel`.
unsafe fn extra(o: &mut impl Write, device: P) {
    macro_rules! line {
        ($($t:tt)*) => {{ writeln!(o, $($t)*).unwrap(); o.flush().unwrap(); }};
    }
    line!("TestCooperativeLevel\twindowed, fresh\t{:x}", call(device, TEST_COOPERATIVE_LEVEL, &[]));
    for kind in 0..=20usize {
        line!("CreateQuery\ttype {kind:x}, null out\t{:x}", call(device, CREATE_QUERY, &[kind, 0]));
    }
    // An event query (8) and an occlusion query (9) through their life cycle.
    for kind in [8usize, 9] {
        let mut q: P = std::ptr::null_mut();
        let hr = call(device, CREATE_QUERY, &[kind, &mut q as *mut P as usize]);
        line!("CreateQuery\ttype {kind:x}\t{hr:x}");
        if q.is_null() {
            continue;
        }
        line!("Query::GetType\ttype {kind:x}\t{:x}", call(q, QUERY_GET_TYPE, &[]));
        line!("Query::GetDataSize\ttype {kind:x}\t{:x}", call(q, QUERY_GET_DATA_SIZE, &[]));
        let mut value = 0xaaaa_aaaau32;
        let hr = call(q, QUERY_GET_DATA, &[&mut value as *mut u32 as usize, 4, 1]);
        line!("Query::GetData\ttype {kind:x}, never issued, flush\t{hr:x}\t{value:x}");
        line!("Query::Issue\ttype {kind:x}, END first\t{:x}", call(q, QUERY_ISSUE, &[1]));
        line!("Query::Issue\ttype {kind:x}, BEGIN\t{:x}", call(q, QUERY_ISSUE, &[2]));
        line!("Query::Issue\ttype {kind:x}, BEGIN twice\t{:x}", call(q, QUERY_ISSUE, &[2]));
        let mut value = 0xaaaa_aaaau32;
        let hr = call(q, QUERY_GET_DATA, &[&mut value as *mut u32 as usize, 4, 1]);
        line!("Query::GetData\ttype {kind:x}, after BEGIN, flush\t{hr:x}\t{value:x}");
        line!("Query::Issue\ttype {kind:x}, END\t{:x}", call(q, QUERY_ISSUE, &[1]));
        let (mut first, mut polls) = (u32::MAX, 0u32);
        let mut value = 0xaaaa_aaaau32;
        let hr = loop {
            let hr = call(q, QUERY_GET_DATA, &[&mut value as *mut u32 as usize, 4, 1]);
            if first == u32::MAX {
                first = hr;
            }
            polls += 1;
            if hr != 1 || polls > 1_000_000 {
                break hr;
            }
        };
        line!("Query::GetData\ttype {kind:x}, after END, flush, polled\t{hr:x}\t{value:x}\tfirst {first:x} polls {polls}");
        line!("Query::GetData\ttype {kind:x}, done, null data size 0\t{:x}", call(q, QUERY_GET_DATA, &[0, 0, 0]));
        let mut big = [0xaaaa_aaaau32; 2];
        let hr = call(q, QUERY_GET_DATA, &[big.as_mut_ptr() as usize, 8, 0]);
        line!("Query::GetData\ttype {kind:x}, done, size 8\t{hr:x}\t{:x} {:x}", big[0], big[1]);
        let mut small = 0xaaaa_aaaau32;
        let hr = call(q, QUERY_GET_DATA, &[&mut small as *mut u32 as usize, 2, 0]);
        line!("Query::GetData\ttype {kind:x}, done, size 2\t{hr:x}\t{small:x}");
        line!("Query::Release\ttype {kind:x}\t{:x}", call(q, RELEASE, &[]));
    }
    // Offscreen plain surfaces, X8R8G8B8 (22), by pool.
    for pool in 0..=3usize {
        let mut s: P = std::ptr::null_mut();
        let hr = call(device, CREATE_OFFSCREEN_PLAIN_SURFACE, &[64, 64, 22, pool, &mut s as *mut P as usize, 0]);
        line!("CreateOffscreenPlainSurface\t64x64 X8R8G8B8 pool {pool}\t{hr:x}");
        if !s.is_null() {
            let hr = call(device, COLOR_FILL, &[s as usize, 0, 0xff11_2233]);
            let mut locked = [0usize; 2];
            let lr = call(s, SURFACE_LOCK_RECT, &[locked.as_mut_ptr() as usize, 0, 0]);
            let first = if lr == 0 { *(locked[1] as *const u32) } else { 0 };
            if lr == 0 {
                call(s, SURFACE_UNLOCK_RECT, &[]);
            }
            line!("ColorFill\toffscreen pool {pool}, whole, ff112233\t{hr:x}\tlock {lr:x} first {first:x}");
            let rect = [8i32, 8, 16, 16];
            let hr = call(device, COLOR_FILL, &[s as usize, rect.as_ptr() as usize, 0x0044_5566]);
            line!("ColorFill\toffscreen pool {pool}, 8,8-16,16\t{hr:x}");
            let bad = [8i32, 8, 100, 16];
            let hr = call(device, COLOR_FILL, &[s as usize, bad.as_ptr() as usize, 0]);
            line!("ColorFill\toffscreen pool {pool}, rect past the edge\t{hr:x}");
            call(s, RELEASE, &[]);
        }
    }
    let mut zero: P = std::ptr::null_mut();
    let hr = call(device, CREATE_OFFSCREEN_PLAIN_SURFACE, &[0, 64, 22, 2, &mut zero as *mut P as usize, 0]);
    line!("CreateOffscreenPlainSurface\t0x64 sysmem\t{hr:x}");
    // GetRenderTargetData from the back buffer (320x240, the display format).
    let mut back: P = std::ptr::null_mut();
    call(device, GET_BACK_BUFFER, &[0, 0, 0, &mut back as *mut P as usize]);
    let mut desc = [0u32; 8];
    call(back, 12, &[desc.as_mut_ptr() as usize]);
    line!("GetBackBuffer\tdesc\t{}", words(&desc));
    let format = desc[0] as usize;
    let surface = |w: usize, h: usize, f: usize, pool: usize| {
        let mut s: P = std::ptr::null_mut();
        call(device, CREATE_OFFSCREEN_PLAIN_SURFACE, &[w, h, f, pool, &mut s as *mut P as usize, 0]);
        s
    };
    let cases: [(&str, usize, usize, usize, usize); 5] = [
        ("same size and format, sysmem", 320, 240, format, 2),
        ("same size and format, default pool", 320, 240, format, 0),
        ("smaller, sysmem", 160, 120, format, 2),
        ("another format (A8R8G8B8 21 / X8R8G8B8 22), sysmem", 320, 240, if format == 22 { 21 } else { 22 }, 2),
        ("same size and format, scratch", 320, 240, format, 3),
    ];
    call(device, 43, &[0, 0, 1, 0xff12_3456, 0x3f80_0000, 0]);
    for (case, w, h, f, pool) in cases {
        let s = surface(w, h, f, pool);
        if s.is_null() {
            line!("GetRenderTargetData\t{case}\tno surface");
            continue;
        }
        let hr = call(device, GET_RENDER_TARGET_DATA, &[back as usize, s as usize]);
        let mut locked = [0usize; 2];
        let lr = call(s, SURFACE_LOCK_RECT, &[locked.as_mut_ptr() as usize, 0, 0x10]);
        let first = if lr == 0 { *(locked[1] as *const u32) } else { 0 };
        if lr == 0 {
            call(s, SURFACE_UNLOCK_RECT, &[]);
        }
        line!("GetRenderTargetData\t{case}\t{hr:x}\tlock {lr:x} first {first:x}");
        call(s, RELEASE, &[]);
    }
    let s = surface(320, 240, format, 2);
    call(device, BEGIN_SCENE, &[]);
    line!("GetRenderTargetData\tin a scene\t{:x}", call(device, GET_RENDER_TARGET_DATA, &[back as usize, s as usize]));
    line!("ColorFill\tback buffer, in a scene\t{:x}", call(device, COLOR_FILL, &[back as usize, 0, 0]));
    call(device, END_SCENE, &[]);
    line!("ColorFill\tback buffer\t{:x}", call(device, COLOR_FILL, &[back as usize, 0, 0]));
    line!("GetRenderTargetData\tsysmem source\t{:x}", call(device, GET_RENDER_TARGET_DATA, &[s as usize, s as usize]));
    call(s, RELEASE, &[]);
    call(back, RELEASE, &[]);
}

/// `device reset`: `Reset` on a windowed device: with a default-pool resource alive, with only
/// managed ones, in a scene, with a back buffer reference held, at another size; the device's
/// states and bindings before and after, and what `TestCooperativeLevel` answers around it.
unsafe fn reset(o: &mut impl Write, device: P, pp: &PresentParameters) {
    const RESET: usize = 16;
    const GET_RENDER_TARGET: usize = 38;
    const GET_DEPTH_STENCIL: usize = 40;
    const SET_VIEWPORT: usize = 47;
    const GET_VIEWPORT: usize = 48;
    const GET_RENDER_STATE: usize = 58;
    const GET_TEXTURE: usize = 64;
    const SET_SAMPLER_STATE: usize = 69;
    const GET_SAMPLER_STATE: usize = 68;
    const SET_TRANSFORM: usize = 44;
    const GET_TRANSFORM: usize = 45;
    macro_rules! line {
        ($($t:tt)*) => {{ writeln!(o, $($t)*).unwrap(); o.flush().unwrap(); }};
    }
    let state = |device: P, rs: usize| {
        let mut v = 0xaaaa_aaaau32;
        call(device, GET_RENDER_STATE, &[rs, &mut v as *mut u32 as usize]);
        v
    };
    let object = |device: P, slot: usize, index: Option<usize>| {
        let mut p: usize = 0;
        let hr = match index {
            Some(i) => call(device, slot, &[i, &mut p as *mut usize as usize]),
            None => call(device, slot, &[&mut p as *mut usize as usize]),
        };
        if p != 0 {
            call(p as P, RELEASE, &[]);
        }
        (hr, p)
    };
    let report = |o: &mut dyn Write, device: P, when: &str| {
        let mut vp = [0u32; 6];
        call(device, GET_VIEWPORT, &[vp.as_mut_ptr() as usize]);
        let mut sampler = 0xaaaa_aaaau32;
        call(device, GET_SAMPLER_STATE, &[0, 5, &mut sampler as *mut u32 as usize]);
        let mut world = [0u32; 16];
        call(device, GET_TRANSFORM, &[256, world.as_mut_ptr() as usize]);
        let (_, rt) = object(device, GET_RENDER_TARGET, Some(0));
        let (_, ds) = object(device, GET_DEPTH_STENCIL, None);
        let (_, tex) = object(device, GET_TEXTURE, Some(0));
        let (_, back) = {
            let mut p: usize = 0;
            let hr = call(device, GET_BACK_BUFFER, &[0, 0, 0, &mut p as *mut usize as usize]);
            let mut desc = [0u32; 8];
            call(p as P, 12, &[desc.as_mut_ptr() as usize]);
            call(p as P, RELEASE, &[]);
            writeln!(o, "GetBackBuffer\t{when}\t{hr:x}\t{}", words(&desc)).unwrap();
            (hr, p)
        };
        writeln!(
            o,
            "State\t{when}\tzenable {:x} cullmode {:x} lighting {:x} magfilter {sampler:x} world[0] {:x} world[12] {:x} viewport {} rt=back {} depth {} texture0 {}",
            state(device, 7),
            state(device, 22),
            state(device, 137),
            world[0],
            world[12],
            words(&vp),
            rt == back,
            ds != 0,
            tex != 0
        )
        .unwrap();
        o.flush().unwrap();
    };
    report(o, device, "fresh");
    // Change states and bindings.
    call(device, SET_RENDER_STATE, &[7, 0]);
    call(device, SET_RENDER_STATE, &[22, 1]);
    call(device, SET_RENDER_STATE, &[137, 0]);
    call(device, SET_SAMPLER_STATE, &[0, 5, 1]);
    let mut m = [0u32; 16];
    m[0] = 0x4000_0000;
    m[12] = 0x4040_0000;
    call(device, SET_TRANSFORM, &[256, m.as_ptr() as usize]);
    let vp = [10u32, 10, 100, 100, 0, 0x3f80_0000];
    call(device, SET_VIEWPORT, &[vp.as_ptr() as usize]);
    let mut managed: P = std::ptr::null_mut();
    line!("CreateTexture\tmanaged\t{:x}", call(device, CREATE_TEXTURE, &[64, 64, 1, 0, 21, 1, &mut managed as *mut P as usize, 0]));
    call(device, SET_TEXTURE, &[0, managed as usize]);
    report(o, device, "changed");
    let mut default: P = std::ptr::null_mut();
    line!("CreateTexture\tdefault pool\t{:x}", call(device, CREATE_TEXTURE, &[64, 64, 1, 1, 21, 0, &mut default as *mut P as usize, 0]));
    let mut p = *pp;
    line!("Reset\twith a default-pool texture alive\t{:x}", call(device, RESET, &[&mut p as *mut PresentParameters as usize]));
    line!("TestCooperativeLevel\tafter the failed Reset\t{:x}", call(device, TEST_COOPERATIVE_LEVEL, &[]));
    line!("BeginScene\tafter the failed Reset\t{:x}", call(device, BEGIN_SCENE, &[]));
    line!("EndScene\tafter the failed Reset\t{:x}", call(device, END_SCENE, &[]));
    line!("Present\tafter the failed Reset\t{:x}", call(device, PRESENT, &[0, 0, 0, 0]));
    line!("Release\tthe default-pool texture\t{:x}", call(default, RELEASE, &[]));
    line!("TestCooperativeLevel\tafter its release\t{:x}", call(device, TEST_COOPERATIVE_LEVEL, &[]));
    // A back buffer reference held across Reset.
    let mut held: usize = 0;
    call(device, GET_BACK_BUFFER, &[0, 0, 0, &mut held as *mut usize as usize]);
    let mut p = *pp;
    line!("Reset\twith a back buffer reference held\t{:x}", call(device, RESET, &[&mut p as *mut PresentParameters as usize]));
    // The reference held is not released: after this Reset it no longer points at a live
    // object (releasing it faulted, 2026-10-27).
    let _ = held;
    let mut p = *pp;
    line!("Reset\tonly managed resources\t{:x}\tcount {:x} format {:x}", call(device, RESET, &[&mut p as *mut PresentParameters as usize]), p.count, p.format);
    line!("TestCooperativeLevel\tafter Reset\t{:x}", call(device, TEST_COOPERATIVE_LEVEL, &[]));
    report(o, device, "after Reset");
    call(device, BEGIN_SCENE, &[]);
    let mut p = *pp;
    line!("Reset\tin a scene\t{:x}", call(device, RESET, &[&mut p as *mut PresentParameters as usize]));
    line!("EndScene\tafter Reset in a scene\t{:x}", call(device, END_SCENE, &[]));
    let mut p = PresentParameters { width: 640, height: 480, ..*pp };
    line!("Reset\t640x480\t{:x}", call(device, RESET, &[&mut p as *mut PresentParameters as usize]));
    report(o, device, "after Reset 640x480");
    let mut p = PresentParameters { width: 0, height: 0, ..*pp };
    line!("Reset\t0x0 (the window's client area)\t{:x}\twidth {} height {}", call(device, RESET, &[&mut p as *mut PresentParameters as usize]), p.width, p.height);
    let mut p = PresentParameters { count: 4, ..*pp };
    line!("Reset\tfour back buffers\t{:x}\tcount {}", call(device, RESET, &[&mut p as *mut PresentParameters as usize]), p.count);
    line!("TestCooperativeLevel\tafter that\t{:x}", call(device, TEST_COOPERATIVE_LEVEL, &[]));
    call(managed, RELEASE, &[]);
    line!("Release\tdevice\t{:x}", call(device, RELEASE, &[]));
}

/// `device refs`: the reference counts `AddRef`/`Release` answer for buffers and textures bound
/// to the device (`SetIndices`, `SetStreamSource`, `SetTexture`) and read back (`GetIndices`,
/// `GetStreamSource`, `GetTexture`), and whether a buffer released to 0 while bound can still be
/// read back.
unsafe fn refs(o: &mut impl Write, device: P) {
    const ADD_REF: usize = 1;
    const GET_INDICES: usize = 105;
    const GET_TEXTURE: usize = 64;
    macro_rules! line {
        ($($t:tt)*) => {{ writeln!(o, $($t)*).unwrap(); o.flush().unwrap(); }};
    }
    let count = |object: P| {
        let up = call(object, ADD_REF, &[]);
        let down = call(object, RELEASE, &[]);
        (up, down)
    };
    let mut ib: P = std::ptr::null_mut();
    line!("CreateIndexBuffer\t12 bytes\t{:x}", call(device, CREATE_INDEX_BUFFER, &[12, 8, 101, 0, &mut ib as *mut P as usize, 0]));
    line!("IndexBuffer\tfresh: AddRef, Release\t{:x?}", count(ib));
    line!("SetIndices\t\t{:x}", call(device, SET_INDICES, &[ib as usize]));
    line!("IndexBuffer\tbound: AddRef, Release\t{:x?}", count(ib));
    let mut back: usize = 0;
    line!("GetIndices\t\t{:x}\tsame {}", call(device, GET_INDICES, &[&mut back as *mut usize as usize]), back == ib as usize);
    line!("IndexBuffer\tafter GetIndices: AddRef, Release\t{:x?}", count(ib));
    line!("Release\tthe GetIndices reference\t{:x}", call(back as P, RELEASE, &[]));
    line!("Release\tthe creation reference, still bound\t{:x}", call(ib, RELEASE, &[]));
    let mut again: usize = 0;
    line!("GetIndices\tafter the last Release, still bound\t{:x}\tsame {}", call(device, GET_INDICES, &[&mut again as *mut usize as usize]), again == ib as usize);
    if again != 0 {
        line!("Release\tthat reference\t{:x}", call(again as P, RELEASE, &[]));
    }
    line!("SetIndices\tnull (unbinds)\t{:x}", call(device, SET_INDICES, &[0]));

    let mut vb: P = std::ptr::null_mut();
    line!("CreateVertexBuffer\t96 bytes\t{:x}", call(device, CREATE_VERTEX_BUFFER, &[96, 8, 0, 0, &mut vb as *mut P as usize, 0]));
    line!("SetStreamSource\t0\t{:x}", call(device, SET_STREAM_SOURCE, &[0, vb as usize, 0, 24]));
    line!("VertexBuffer\tbound: AddRef, Release\t{:x?}", count(vb));
    line!("Release\tthe creation reference, still bound\t{:x}", call(vb, RELEASE, &[]));
    line!("SetStreamSource\tnull\t{:x}", call(device, SET_STREAM_SOURCE, &[0, 0, 0, 0]));

    let mut tex: P = std::ptr::null_mut();
    line!("CreateTexture\t64x64 managed\t{:x}", call(device, CREATE_TEXTURE, &[64, 64, 1, 0, 21, 1, &mut tex as *mut P as usize, 0]));
    line!("SetTexture\t0\t{:x}", call(device, SET_TEXTURE, &[0, tex as usize]));
    line!("Texture\tbound: AddRef, Release\t{:x?}", count(tex));
    let mut t: usize = 0;
    line!("GetTexture\t0\t{:x}\tsame {}", call(device, GET_TEXTURE, &[0, &mut t as *mut usize as usize]), t == tex as usize);
    line!("Texture\tafter GetTexture: AddRef, Release\t{:x?}", count(tex));
    if t != 0 {
        call(t as P, RELEASE, &[]);
    }
    line!("Release\tthe creation reference, still bound\t{:x}", call(tex, RELEASE, &[]));
    line!("SetTexture\tnull\t{:x}", call(device, SET_TEXTURE, &[0, 0]));
    line!("Release\tdevice\t{:x}", call(device, RELEASE, &[]));
}
