//! What a recording of the Direct3D 9 call stream uses (`d3d9-stats FILE`): the commands and their
//! counts, the device's present parameters, render target and depth surfaces, texture formats,
//! and each render and sampler state value with its count.
use d3d9_stream::Cmd;
use std::collections::BTreeMap;
fn main() {
    let path = std::env::args().nth(1).expect("the recording");
    let bytes = std::fs::read(path).expect("read");
    let mut rs: BTreeMap<(u32, u32), u64> = BTreeMap::new();
    let mut ss: BTreeMap<(u32, u32), u64> = BTreeMap::new();
    let mut tex: BTreeMap<(u32, u32, u32), u64> = BTreeMap::new();
    let mut names: BTreeMap<&'static str, u64> = BTreeMap::new();
    let mut presents = 0;
    for c in d3d9_stream::wire::commands(&bytes).expect("a stream") {
        let c = c.expect("a command");
        *names.entry(c.name()).or_default() += 1;
        match c {
            Cmd::CreateSurface {
                surface,
                depth,
                width,
                height,
                format,
                multisample,
                quality,
                flag,
                ..
            } => {
                println!("CreateSurface {surface:#x} depth {depth} {width}x{height} format {format} ms {multisample} q {quality} flag {flag}");
            }
            Cmd::CreateTexture {
                width,
                height,
                usage,
                format,
                pool,
                ..
            } => {
                *tex.entry((format, usage, pool)).or_default() += 1;
                if usage & 1 != 0 {
                    println!("CreateTexture RT {width}x{height} format {format} usage {usage:#x}");
                }
            }
            Cmd::SetRenderState { state, value, .. } => *rs.entry((state, value)).or_default() += 1,
            Cmd::SetSamplerState { kind, value, .. } => *ss.entry((kind, value)).or_default() += 1,
            Cmd::CreateDevice { present, .. } => {
                println!("CreateDevice present {:?}", present.to_words())
            }
            Cmd::Present { .. } => presents += 1,
            _ => {}
        }
    }
    println!("presents {presents}");
    for (k, v) in &names {
        println!("cmd {k} {v}");
    }
    for ((f, u, p), v) in &tex {
        println!(
            "texture format {f} ({:?}) usage {u:#x} pool {p}: {v}",
            f.to_le_bytes()
                .map(|b| if b.is_ascii_graphic() { b as char } else { '.' })
        );
    }
    for ((s, v), n) in &rs {
        println!("rs {s} = {v:#x}: {n}");
    }
    for ((s, v), n) in &ss {
        println!("ss {s} = {v:#x}: {n}");
    }
}
