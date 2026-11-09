//! When a recording's shaders are first drawn with (`d3d9-first-use FILE [Data/Shaders folder]
//! [FROM]`): for each `Present` from `FROM` on (0 by default) at which a vertex or pixel shader, or
//! a pair of them, is drawn with for the first time in the run, the present, the shaders and the
//! present each was created at, named from the game's shader packages when their folder is given
//! (the player's own files; only names are printed). A driver compiles a Direct3D 9 shader when it
//! is first drawn with, so these are the frames that wait for its compiler.

use d3d9_stream::Cmd;
use d3d9_wgpu::sm::package::Package;
use std::collections::{HashMap, HashSet};

fn main() {
    let mut args = std::env::args().skip(1);
    let path = args.next().expect("the recording");
    let shaders_dir = args.next().filter(|a| a != "-");
    let from: u64 = args
        .next()
        .map_or(0, |a| a.parse().expect("FROM: a present"));
    let mut names: HashMap<Vec<u8>, String> = HashMap::new();
    if let Some(dir) = shaders_dir {
        let mut files: Vec<_> = std::fs::read_dir(&dir)
            .expect("the Shaders folder")
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.extension().is_some_and(|x| x.eq_ignore_ascii_case("sdp")))
            .collect();
        files.sort();
        for file in files {
            let stem = file.file_stem().unwrap().to_string_lossy().into_owned();
            let bytes = std::fs::read(&file).expect("a package");
            if let Ok(package) = Package::parse(&bytes) {
                for s in package.shaders {
                    names
                        .entry(s.bytecode.clone())
                        .or_insert_with(|| format!("{stem}/{}", s.name));
                }
            }
        }
    }
    let bytes = std::fs::read(&path).expect("read");
    // Each shader: (pixel, created at present, name).
    let mut made: HashMap<u32, (bool, u64, String)> = HashMap::new();
    let (mut vs, mut ps) = (0u32, 0u32);
    let mut seen: HashSet<u32> = HashSet::new();
    let mut pairs: HashSet<(u32, u32)> = HashSet::new();
    let mut presents = 0u64;
    let mut created_late = 0u64;
    let mut new_here: Vec<String> = Vec::new();
    let mut new_pairs_here = 0;
    let mut first_uses = 0u64;
    for c in d3d9_stream::wire::commands(&bytes).expect("a stream") {
        match c.expect("a command") {
            Cmd::CreateShader {
                shader,
                pixel,
                tokens,
                ..
            } => {
                let name = names
                    .get(tokens.bytes().as_ref())
                    .cloned()
                    .unwrap_or_else(|| format!("({} bytes)", tokens.len()));
                made.insert(shader, (pixel, presents, name));
                seen.remove(&shader);
                if presents >= from {
                    created_late += 1;
                }
            }
            Cmd::SetShader { pixel, shader, .. } => {
                if pixel {
                    ps = shader;
                } else {
                    vs = shader;
                }
            }
            Cmd::DrawPrimitive { .. }
            | Cmd::DrawIndexedPrimitive { .. }
            | Cmd::DrawPrimitiveUp { .. } => {
                for s in [vs, ps] {
                    if s != 0 && seen.insert(s) {
                        first_uses += 1;
                        if presents >= from {
                            let (pixel, at, name) =
                                made.get(&s).cloned().unwrap_or((false, 0, "?".into()));
                            new_here.push(format!(
                                "{} {name} (made at {at})",
                                if pixel { "ps" } else { "vs" }
                            ));
                        }
                    }
                }
                if pairs.insert((vs, ps)) && presents >= from {
                    new_pairs_here += 1;
                }
            }
            Cmd::Present { .. } => {
                if !new_here.is_empty() || new_pairs_here > 0 {
                    println!(
                        "present {presents}: {} new shaders, {new_pairs_here} new pairs",
                        new_here.len()
                    );
                    for s in &new_here {
                        println!("  {s}");
                    }
                }
                new_here.clear();
                new_pairs_here = 0;
                presents += 1;
            }
            _ => {}
        }
    }
    println!(
        "{presents} presents; {} shaders made ({created_late} from present {from} on), {} drawn with ({first_uses} first uses), {} pairs",
        made.len(),
        seen.len(),
        pairs.len()
    );
}
