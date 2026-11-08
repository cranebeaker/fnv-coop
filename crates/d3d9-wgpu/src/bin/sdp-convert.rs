//! Converts every shader of the game's packages to WGSL and checks it with naga's validator
//! (`sdp-convert <Data/Shaders folder> [package] [shader name to print]`). Reads the player's own
//! files; prints only names, counts and errors (and, when asked, one shader's listing and WGSL).

use d3d9_wgpu::sm::{self, package::Package, wgsl, Stage};

/// Each vertex input four floats, the varyings the shader itself declares.
fn default_link(p: &sm::Program) -> wgsl::Link {
    let sig = wgsl::signature(p);
    wgsl::Link {
        inputs: vec![wgsl::Input::Float(4); 16],
        centroid: if p.stage == Stage::Pixel {
            sig.centroid.clone()
        } else {
            Vec::new()
        },
        varyings: match p.stage {
            Stage::Vertex => sig
                .outputs
                .iter()
                .map(|(_, s)| *s)
                .filter(|s| !(s.usage == 0 && s.index == 0))
                .collect(),
            Stage::Pixel => sig.inputs.iter().map(|(_, s)| *s).collect(),
        },
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let dir = std::path::PathBuf::from(args.next().expect("the Data/Shaders folder"));
    let only = args.next().unwrap_or_default();
    let show = args.next();
    let mut names: Vec<_> = std::fs::read_dir(&dir)
        .expect("the folder")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x.eq_ignore_ascii_case("sdp")))
        .collect();
    names.sort();
    for path in names {
        let file = path.file_name().unwrap().to_string_lossy().into_owned();
        if !file.contains(only.as_str()) {
            continue;
        }
        let package =
            Package::parse(&std::fs::read(&path).expect("the package")).expect("a package");
        if let Some(name) = &show {
            let s = package.get(name).expect("the shader");
            let p = sm::parse(&sm::tokens(&s.bytecode).unwrap()).unwrap();
            println!("{}", sm::disasm::listing(&p));
            match wgsl::convert(&p, &default_link(&p)) {
                Ok(w) => println!("{w}"),
                Err(e) => println!("error: {e}"),
            }
            continue;
        }
        let failures = sm::check_package(&package);
        println!(
            "{file}\t{} shaders\t{} not converted",
            package.shaders.len(),
            failures.len()
        );
        for (name, why) in failures.iter().take(20) {
            println!(
                "  {name}: {}",
                why.lines().take(6).collect::<Vec<_>>().join(" | ")
            );
        }
    }
}
