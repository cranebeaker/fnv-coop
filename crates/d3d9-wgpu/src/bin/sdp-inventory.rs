//! What the game's shader packages hold: each package's shader models and the instructions they
//! use, with their counts (`sdp-inventory <Data/Shaders folder> [package number]`). Reads the
//! player's own files; prints only names and counts.

use d3d9_wgpu::sm::{self, opcode_name, package::Package};
use std::collections::BTreeMap;

fn main() {
    let mut args = std::env::args().skip(1);
    let dir = std::path::PathBuf::from(args.next().expect("the Data/Shaders folder"));
    let only = args.next();
    let mut names: Vec<_> = std::fs::read_dir(&dir)
        .expect("the folder")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x.eq_ignore_ascii_case("sdp")))
        .collect();
    names.sort();
    for path in names {
        let file = path.file_name().unwrap().to_string_lossy().into_owned();
        if only.as_ref().is_some_and(|o| !file.contains(o.as_str())) {
            continue;
        }
        let bytes = std::fs::read(&path).expect("the package");
        let package = match Package::parse(&bytes) {
            Ok(p) => p,
            Err(e) => {
                println!("{file}\terror\t{e}");
                continue;
            }
        };
        let mut profiles: BTreeMap<String, u32> = BTreeMap::new();
        let mut ops: BTreeMap<String, u32> = BTreeMap::new();
        let mut errors = 0;
        for s in &package.shaders {
            let p = match sm::tokens(&s.bytecode).and_then(|t| sm::parse(&t)) {
                Ok(p) => p,
                Err(e) => {
                    errors += 1;
                    eprintln!("{file}\t{}\t{e}", s.name);
                    continue;
                }
            };
            *profiles.entry(p.profile()).or_default() += 1;
            for ins in &p.ins {
                let mut name = opcode_name(ins.op).unwrap_or("?").to_string();
                if ins.op == sm::op::TEX {
                    name = ["texld", "texldp", "texldb"]
                        .get(ins.controls as usize)
                        .unwrap_or(&"texld?")
                        .to_string();
                }
                if matches!(ins.op, sm::op::IFC | sm::op::BREAKC | sm::op::SETP) {
                    name.push_str("_c");
                }
                let stage = match p.stage {
                    sm::Stage::Vertex => "vs",
                    sm::Stage::Pixel => "ps",
                };
                *ops.entry(format!("{stage}:{name}")).or_default() += 1;
                if ins.predicate.is_some() {
                    *ops.entry(format!("{stage}:(predicated)")).or_default() += 1;
                }
                for src in &ins.src {
                    if src.rel.is_some() {
                        *ops.entry(format!("{stage}:(relative source)")).or_default() += 1;
                    }
                    if src.modifier != 0 {
                        *ops.entry(format!("{stage}:(source modifier {})", src.modifier))
                            .or_default() += 1;
                    }
                    *ops.entry(format!("{stage}:(reg type {})", src.reg.ty))
                        .or_default() += 1;
                }
                if let Some(d) = ins.dst {
                    if d.sat {
                        *ops.entry(format!("{stage}:(_sat)")).or_default() += 1;
                    }
                    if d.shift != 0 {
                        *ops.entry(format!("{stage}:(shift)")).or_default() += 1;
                    }
                    if d.rel.is_some() {
                        *ops.entry(format!("{stage}:(relative destination)"))
                            .or_default() += 1;
                    }
                    if ins.op != sm::op::DCL {
                        *ops.entry(format!("{stage}:(dst type {})", d.reg.ty))
                            .or_default() += 1;
                    }
                }
                if let Some(decl) = ins.decl {
                    let d = ins.dst.unwrap();
                    *ops.entry(format!(
                        "{stage}:(dcl type {} usage {} index {} tex {})",
                        d.reg.ty, decl.usage, decl.index, decl.texture
                    ))
                    .or_default() += 1;
                }
            }
        }
        println!(
            "{file}\tshaders\t{}\tunreadable\t{errors}",
            package.shaders.len()
        );
        for (k, v) in &profiles {
            println!("{file}\tprofile\t{k}\t{v}");
        }
        for (k, v) in &ops {
            println!("{file}\top\t{k}\t{v}");
        }
    }
}
