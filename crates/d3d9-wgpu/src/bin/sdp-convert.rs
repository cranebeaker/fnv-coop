//! Converts every shader of the game's packages to WGSL and checks it
//! (`sdp-convert <Data/Shaders folder> [package] [shader name to print | --gpu]`): with naga's
//! validator, and with `--gpu` also by making a render pipeline of each on this machine's GPU
//! (wgpu, `NV_WGPU_BACKEND`), which runs the backend's own compiler (SPIR-V for Vulkan, HLSL and
//! DXC/FXC for Direct3D 12). Reads the player's own files; prints only names, counts and errors
//! (and, when asked, one shader's listing and WGSL).

use d3d9_wgpu::sm::{self, package::Package, wgsl, Stage};

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

fn block_on<F: std::future::Future>(f: F) -> F::Output {
    struct Noop;
    impl std::task::Wake for Noop {
        fn wake(self: std::sync::Arc<Self>) {}
    }
    let waker = std::task::Waker::from(std::sync::Arc::new(Noop));
    let mut cx = std::task::Context::from_waker(&waker);
    let mut f = std::pin::pin!(f);
    loop {
        if let std::task::Poll::Ready(v) = f.as_mut().poll(&mut cx) {
            return v;
        }
        std::thread::yield_now();
    }
}

/// A render pipeline of the converted shader with a stand-in for the other stage; the error, if
/// wgpu or the backend's compiler refuses it.
fn on_gpu(device: &wgpu::Device, p: &sm::Program, source: &str) -> Option<String> {
    let link = default_link(p);
    let sig = wgsl::signature(p);
    let partner = match p.stage {
        Stage::Vertex => {
            "@fragment fn main() -> @location(0) vec4<f32> { return vec4<f32>(0.0); }".to_string()
        }
        Stage::Pixel => {
            let mut s = String::from("struct O {\n    @builtin(position) p: vec4<f32>,\n");
            for l in 0..link.varyings.len() {
                let interp = if link.centroid.get(l).copied().unwrap_or(false) {
                    " @interpolate(perspective, centroid)"
                } else {
                    ""
                };
                s.push_str(&format!("    @location({l}){interp} l{l}: vec4<f32>,\n"));
            }
            s.push_str("}\n@vertex fn main() -> O { var o: O; return o; }\n");
            s
        }
    };
    let scope = device.push_error_scope(wgpu::ErrorFilter::Validation);
    let own = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(source.into()),
    });
    let other = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(partner.into()),
    });
    let (vm, fm) = match p.stage {
        Stage::Vertex => (&own, &other),
        Stage::Pixel => (&other, &own),
    };
    let attributes: Vec<wgpu::VertexAttribute> = match p.stage {
        Stage::Vertex => sig
            .inputs
            .iter()
            .map(|(r, _)| wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x4,
                offset: 0,
                shader_location: r.n,
            })
            .collect(),
        Stage::Pixel => Vec::new(),
    };
    let buffers = [Some(wgpu::VertexBufferLayout {
        array_stride: 16,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &attributes,
    })];
    let targets: Vec<Option<wgpu::ColorTargetState>> = match p.stage {
        Stage::Vertex => vec![Some(wgpu::TextureFormat::Rgba8Unorm.into())],
        Stage::Pixel => {
            let n = (0..4)
                .rev()
                .find(|k| sig.colours & (1 << k) != 0)
                .map_or(1, |k| k + 1);
            (0..n)
                .map(|_| Some(wgpu::TextureFormat::Rgba8Unorm.into()))
                .collect()
        }
    };
    let _ = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: None,
        vertex: wgpu::VertexState {
            module: vm,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            buffers: if attributes.is_empty() { &[] } else { &buffers },
        },
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: sig.depth.then(|| wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(true),
            depth_compare: Some(wgpu::CompareFunction::Always),
            stencil: Default::default(),
            bias: Default::default(),
        }),
        multisample: Default::default(),
        fragment: Some(wgpu::FragmentState {
            module: fm,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            targets: &targets,
        }),
        multiview_mask: None,
        cache: None,
    });
    block_on(scope.pop()).map(|e| e.to_string())
}

fn main() {
    let mut args = std::env::args().skip(1);
    let dir = std::path::PathBuf::from(args.next().expect("the Data/Shaders folder"));
    let only = args.next().unwrap_or_default();
    let show = args.next();
    let gpu = show.as_deref() == Some("--gpu");
    let device = gpu.then(|| {
        let backends = match std::env::var("NV_WGPU_BACKEND").as_deref() {
            Ok("dx12") => wgpu::Backends::DX12,
            Ok("vulkan") => wgpu::Backends::VULKAN,
            _ => wgpu::Backends::PRIMARY,
        };
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends,
            ..wgpu::InstanceDescriptor::new_without_display_handle()
        });
        let adapter = block_on(instance.request_adapter(&Default::default())).expect("an adapter");
        println!("on {:?}", adapter.get_info());
        let (device, _queue) = block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: adapter.features() & wgpu::Features::FLOAT32_FILTERABLE,
            required_limits: adapter.limits(),
            memory_hints: Default::default(),
            experimental_features: Default::default(),
            trace: Default::default(),
        }))
        .expect("a device");
        device
    });
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
        if let (Some(name), false) = (&show, gpu) {
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
        if let Some(device) = &device {
            let mut refused = 0;
            for s in &package.shaders {
                let Ok(p) = sm::tokens(&s.bytecode).and_then(|t| sm::parse(&t)) else {
                    continue;
                };
                let Ok(source) = wgsl::convert(&p, &default_link(&p)) else {
                    continue;
                };
                if let Some(e) = on_gpu(device, &p, &source) {
                    refused += 1;
                    if refused <= 10 {
                        println!(
                            "  {}: {}",
                            s.name,
                            e.lines().take(8).collect::<Vec<_>>().join(" | ")
                        );
                    }
                }
            }
            println!("{file}\t{refused} refused by the GPU's compiler");
        }
    }
}
