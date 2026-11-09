//! The stream against a backend that logs: the threaded runner carries out the same commands in
//! the same order as the direct one, a sync command sees every command before it, at most one
//! frame is in flight, a backend's panic stops the recording side, the queue stays bounded, files
//! replay, and a tee feeds both backends.

use crate::cmd::NAMES;
use crate::*;
use std::collections::HashMap;
use std::io::Write;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// TEST DATA: a backend that logs each command as it prints, keeps the levels it is given,
/// counts its draws and presents, and can be slow or refuse a command.
#[derive(Default)]
struct Log {
    lines: Arc<Mutex<Vec<String>>>,
    levels: HashMap<Handle, Vec<u8>>,
    draws: u32,
    presents: Arc<AtomicU64>,
    slow_present: Option<Duration>,
    slow_upload: Option<Duration>,
    refuse: Option<&'static str>,
    /// Added to its query answers (a shadow that answers otherwise).
    offset: u32,
}

impl Log {
    fn lines(&self) -> Arc<Mutex<Vec<String>>> {
        Arc::clone(&self.lines)
    }
}

impl Backend for Log {
    fn execute(&mut self, cmd: Cmd<Data<'_>>) -> Reply {
        self.lines.lock().unwrap().push(format!("{cmd:?}"));
        if self.refuse == Some(cmd.name()) {
            panic!("the test backend refuses {}", cmd.name());
        }
        match cmd {
            Cmd::CreateDevice { .. } => Reply::Device {
                created: true,
                native: None,
            },
            Cmd::UploadSurface { surface, data, .. } => {
                if let Some(d) = self.slow_upload {
                    std::thread::sleep(d);
                }
                self.levels.insert(surface, data.bytes().into_owned());
                Reply::None
            }
            Cmd::DrawIndexedPrimitive { .. } => {
                self.draws += 1;
                Reply::None
            }
            Cmd::Present { .. } => {
                if let Some(d) = self.slow_present {
                    std::thread::sleep(d);
                }
                self.presents.fetch_add(1, Ordering::SeqCst);
                Reply::None
            }
            Cmd::ReadSurface { surface, size } => Reply::Bytes(
                self.levels
                    .get(&surface)
                    .filter(|b| b.len() == size as usize)
                    .cloned(),
            ),
            Cmd::GetRenderTargetData { pitch, .. } => Reply::Rows(Some(Rows {
                row: pitch,
                bytes: vec![self.draws as u8; pitch as usize],
            })),
            Cmd::QueryGetData { .. } => Reply::Query(Some((0, self.draws + self.offset))),
            _ => Reply::None,
        }
    }
}

/// A small random number source (xorshift), so a run is the same every time.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 16) as u32
    }
}

/// TEST DATA: `frames` frames shaped like the game's (constants, textures, states, buffers,
/// draws; now and then an upload, a large upload, a read-back, a query, a creation and a
/// destruction), submitted to `stream`; the replies as they print.
fn play(stream: &mut Stream, frames: u32) -> Vec<String> {
    let mut rng = Rng(0x2545_f491_4f6c_dd1d);
    let mut replies = Vec::new();
    let present: Vec<u32> = (0..14).collect();
    replies.push(format!(
        "{:?}",
        stream.submit(Cmd::CreateDevice {
            device: 0x1000,
            focus: 0x10,
            window: 0x20,
            behavior: 0x54,
            present: Data::from_words(&present),
            back_buffers: Data::from_words(&[0x2000]),
            depth_stencil: 0x3000,
        })
    ));
    for frame in 0..frames {
        stream.submit(Cmd::BeginScene {
            device: 0x1000,
            model: 0,
        });
        let draws = 40 + rng.next() % 40;
        for _ in 0..draws {
            let constants: Vec<u32> = (0..4 * (1 + rng.next() % 8)).map(|_| rng.next()).collect();
            let kind = (rng.next() % 6) as u8;
            // Booleans are one word a register.
            let per = if kind % 3 == 2 { 1 } else { 4 };
            stream.submit(Cmd::SetConstants {
                device: 0x1000,
                kind,
                start: rng.next() % 200,
                count: constants.len() as u32 / per,
                data: Data::from_words(&constants),
            });
            stream.submit(Cmd::SetTexture {
                device: 0x1000,
                sampler: rng.next() % 8,
                texture: 0x4000 + 8 * (rng.next() % 64),
            });
            stream.submit(Cmd::SetSamplerState {
                device: 0x1000,
                sampler: rng.next() % 8,
                kind: 1 + rng.next() % 13,
                value: rng.next(),
            });
            stream.submit(Cmd::SetRenderState {
                device: 0x1000,
                state: rng.next() % 210,
                value: rng.next(),
            });
            let matrix: Vec<u32> = (0..16).map(|_| rng.next()).collect();
            stream.submit(Cmd::SetTransform {
                device: 0x1000,
                state: 256,
                matrix: Data::from_words(&matrix),
            });
            stream.submit(Cmd::SetStreamSource {
                device: 0x1000,
                stream: 0,
                buffer: 0x5000 + 8 * (rng.next() % 32),
                offset: 0,
                stride: 32,
            });
            stream.submit(Cmd::SetIndices {
                device: 0x1000,
                buffer: 0x6000 + 8 * (rng.next() % 32),
            });
            stream.submit(Cmd::DrawIndexedPrimitive {
                device: 0x1000,
                kind: 4,
                base_vertex: 0,
                min_index: 0,
                vertices: rng.next() % 1000,
                start_index: 0,
                count: rng.next() % 500,
            });
            match rng.next() % 64 {
                0 => {
                    let seed = rng.next() as u8;
                    let fill = move |offset: u32, into: &mut [u8]| {
                        for (i, b) in into.iter_mut().enumerate() {
                            *b = (offset as usize + i) as u8 ^ seed;
                        }
                    };
                    stream.submit(Cmd::UploadBuffer {
                        buffer: 0x5000,
                        offset: 16,
                        flags: 0x2000,
                        data: Data::Read {
                            len: 1 + rng.next() % 3000,
                            read: &fill,
                        },
                    });
                }
                1 => {
                    let size = 64 * 64 * 4 + rng.next() % 200_000;
                    let level: Vec<u8> = (0..size).map(|i| (i ^ frame) as u8).collect();
                    let surface = 0x7000 + 8 * (rng.next() % 4);
                    stream.submit(Cmd::UploadSurface {
                        surface,
                        row: size,
                        rows: 1,
                        data: Data::Bytes(&level),
                    });
                    replies.push(format!(
                        "{:?}",
                        stream.submit(Cmd::ReadSurface { surface, size })
                    ));
                }
                2 => {
                    stream.submit(Cmd::QueryIssue {
                        query: 0x8000,
                        flags: 1,
                    });
                    replies.push(format!(
                        "{:?}",
                        stream.submit(Cmd::QueryGetData {
                            query: 0x8000,
                            flags: 1,
                        })
                    ));
                }
                3 => {
                    let tokens: Vec<u32> = (0..rng.next() % 300).map(|_| rng.next()).collect();
                    stream.submit(Cmd::CreateShader {
                        device: 0x1000,
                        shader: 0x9000,
                        pixel: true,
                        tokens: Data::from_words(&tokens),
                    });
                    stream.submit(Cmd::Destroy { object: 0x9000 });
                }
                4 => {
                    replies.push(format!(
                        "{:?}",
                        stream.submit(Cmd::GetRenderTargetData {
                            device: 0x1000,
                            source: 0x2000,
                            destination: 0x7000,
                            pitch: 16,
                        })
                    ));
                }
                _ => {}
            }
        }
        stream.submit(Cmd::EndScene {
            device: 0x1000,
            model: 0,
        });
        stream.submit(Cmd::Present {
            device: 0x1000,
            source: None,
            dest: None,
            window: 0,
            dirty: false,
            model: 0,
            frame: 0,
        });
    }
    replies
}

/// The log and the replies of [`play`] through `make`'s stream.
fn run(make: impl FnOnce(Box<dyn Backend>) -> Stream, frames: u32) -> (Vec<String>, Vec<String>) {
    let log = Log::default();
    let lines = log.lines();
    let mut stream = make(Box::new(log));
    let replies = play(&mut stream, frames);
    drop(stream);
    let lines = lines.lock().unwrap().clone();
    (lines, replies)
}

#[test]
fn threaded_carries_out_the_same_commands_in_the_same_order_as_direct() {
    let (direct, direct_replies) = run(Stream::direct, 60);
    assert!(direct.len() > 20_000, "{} commands", direct.len());
    assert!(direct_replies.len() > 10);
    let tiny = Settings {
        chunk_commands: 3,
        chunk_bytes: 300,
        ..Settings::default()
    };
    let no_frame_in_flight = Settings {
        frames_in_flight: 0,
        check: true,
        ..Settings::default()
    };
    for settings in [Settings::default(), tiny, no_frame_in_flight] {
        let (threaded, replies) = run(|b| Stream::threaded(b, settings.clone()), 60);
        assert_eq!(threaded.len(), direct.len(), "{settings:?}");
        if let Some(i) = (0..direct.len()).find(|&i| threaded[i] != direct[i]) {
            panic!(
                "{settings:?}: command {i} differs:\n  direct   {}\n  threaded {}",
                direct[i], threaded[i]
            );
        }
        assert_eq!(replies, direct_replies, "{settings:?}");
    }
}

#[test]
fn presents_are_numbered_from_one() {
    let (lines, _) = run(Stream::direct, 3);
    let frames: Vec<&String> = lines.iter().filter(|l| l.starts_with("Present")).collect();
    assert_eq!(frames.len(), 3);
    for (i, l) in frames.iter().enumerate() {
        assert!(l.contains(&format!("frame: {}", i + 1)), "{l}");
    }
}

#[test]
fn a_sync_command_sees_every_command_before_it() {
    let log = Log {
        slow_upload: Some(Duration::from_millis(2)),
        ..Log::default()
    };
    let mut stream = Stream::threaded(Box::new(log), Settings::default());
    let mut last = Vec::new();
    for i in 0..20u8 {
        last = vec![i; 4096];
        stream.submit(Cmd::UploadSurface {
            surface: 0x7000,
            row: 64,
            rows: 64,
            data: Data::Bytes(&last),
        });
    }
    match stream.submit(Cmd::ReadSurface {
        surface: 0x7000,
        size: 4096,
    }) {
        Reply::Bytes(Some(b)) => assert_eq!(b, last),
        other => panic!("{other:?}"),
    }
}

/// Presents left to carry out each time `Present` returns, with a backend whose presents take
/// 20 ms.
fn ahead(frames_in_flight: u64) -> Vec<u64> {
    let log = Log {
        slow_present: Some(Duration::from_millis(20)),
        ..Log::default()
    };
    let done = Arc::clone(&log.presents);
    let settings = Settings {
        frames_in_flight,
        ..Settings::default()
    };
    let mut stream = Stream::threaded(Box::new(log), settings);
    (0..6)
        .map(|_| {
            stream.submit(Cmd::Present {
                device: 1,
                source: None,
                dest: None,
                window: 0,
                dirty: false,
                model: 0,
                frame: 0,
            });
            u64::from(stream.presents()) - done.load(Ordering::SeqCst)
        })
        .collect()
}

#[test]
fn at_most_one_frame_is_in_flight() {
    let one = ahead(1);
    assert!(one.iter().all(|&n| n <= 1), "{one:?}");
    assert!(
        one.contains(&1),
        "the game goes on while a present is carried out: {one:?}"
    );
    let none = ahead(0);
    assert!(none.iter().all(|&n| n == 0), "{none:?}");
}

#[test]
fn a_backend_panic_stops_the_recording_side_with_its_message() {
    let log = Log {
        refuse: Some("SetFvf"),
        ..Log::default()
    };
    let mut stream = Stream::threaded(Box::new(log), Settings::default());
    stream.submit(Cmd::SetFvf { device: 1, fvf: 2 });
    let stopped = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        stream.submit(Cmd::QueryGetData { query: 3, flags: 1 });
    }));
    let message = crate::runner::panic_message(&*stopped.expect_err("the recording side stops"));
    assert!(
        message.contains("render thread stopped") && message.contains("refuses SetFvf"),
        "{message}"
    );
    // And it ends without waiting for the stopped render thread.
    drop(stream);
}

#[test]
fn the_queue_stays_bounded_while_the_render_thread_is_behind() {
    let log = Log {
        slow_upload: Some(Duration::from_millis(1)),
        ..Log::default()
    };
    let settings = Settings {
        max_queued_bytes: 1 << 20,
        max_queued_chunks: 8,
        chunk_bytes: 64 << 10,
        ..Settings::default()
    };
    let mut stream = Stream::threaded(Box::new(log), settings);
    let level = vec![7u8; 100_000];
    let mut most = 0;
    for _ in 0..100 {
        stream.submit(Cmd::UploadSurface {
            surface: 1,
            row: 1000,
            rows: 100,
            data: Data::Bytes(&level),
        });
        let (bytes, chunks) = stream.queued();
        most = most.max(bytes);
        assert!(chunks <= 9, "{chunks} chunks waiting");
    }
    assert!(most <= (1 << 20) + 2 * 100_000, "{most} bytes waiting");
    stream.finish();
    assert_eq!(stream.queued(), (0, 0));
}

#[test]
fn dropping_the_stream_carries_out_what_was_recorded() {
    let log = Log::default();
    let lines = log.lines();
    let mut stream = Stream::threaded(Box::new(log), Settings::default());
    for i in 0..10_000 {
        stream.submit(Cmd::SetRenderState {
            device: 1,
            state: 7,
            value: i,
        });
    }
    drop(stream);
    assert_eq!(lines.lock().unwrap().len(), 10_000);
}

/// TEST DATA: payloads of the sizes the calls give them, filled with distinct words.
struct Payloads {
    present: [u32; 14],
    handles: [u32; 3],
    rect: [u32; 4],
    other_rect: [u32; 4],
    matrix: [u32; 16],
    ramp: Vec<u8>,
    registers: [u32; 12],
    vertices: [u32; 9],
    material: [u32; 17],
    light: [u32; 26],
    cube: [u32; 6],
    tokens: [u32; 5],
    elements: [u32; 4],
    level: [u32; 2],
}

impl Payloads {
    fn new() -> Payloads {
        fn words<const N: usize>(from: u32) -> [u32; N] {
            std::array::from_fn(|i| from.wrapping_add(i as u32).wrapping_mul(0x0101_0101))
        }
        Payloads {
            present: words(1),
            handles: words(20),
            rect: words(30),
            other_rect: words(40),
            matrix: words(50),
            ramp: (0..0x600u32).map(|i| (i * 3) as u8).collect(),
            registers: words(70),
            vertices: words(100),
            material: words(110),
            light: words(130),
            cube: words(160),
            tokens: words(170),
            elements: words(180),
            level: words(190),
        }
    }
}

/// One of every command, in the list's order; `big` the payload of the buffer's upload.
fn every_command<'a>(p: &'a Payloads, big: Data<'a>) -> Vec<Cmd<Data<'a>>> {
    let w = |words: &'a [u32]| Data::from_words(words);
    let d = 0x1000;
    vec![
        Cmd::CreateDevice {
            device: d,
            focus: -2,
            window: 0x1234_5678_9abc,
            behavior: 0x54,
            present: w(&p.present),
            back_buffers: w(&p.handles),
            depth_stencil: 3,
        },
        Cmd::Reset {
            device: d,
            window: 9,
            present: w(&p.present),
            back_buffers: w(&p.handles[..1]),
            depth_stencil: 0,
        },
        Cmd::Present {
            device: d,
            source: Some(w(&p.rect)),
            dest: None,
            window: 5,
            dirty: true,
            model: 1,
            frame: 1, // as the stream numbers its first present
        },
        Cmd::BeginScene {
            device: d,
            model: 0,
        },
        Cmd::EndScene {
            device: d,
            model: 0x8876_086c,
        },
        Cmd::Clear {
            device: d,
            count: 1,
            rects: Some(w(&p.other_rect)),
            flags: 7,
            colour: 0xff00_00ff,
            z: 1f32.to_bits(),
            stencil: 3,
            model: 0,
        },
        Cmd::SetRenderTarget {
            device: d,
            index: 1,
            surface: 4,
        },
        Cmd::SetDepthStencilSurface {
            device: d,
            surface: 0,
        },
        Cmd::SetViewport {
            device: d,
            viewport: [1, 2, 3, 4, 5, 6],
        },
        Cmd::SetTransform {
            device: d,
            state: 256,
            matrix: w(&p.matrix),
        },
        Cmd::SetScissorRect {
            device: d,
            rect: [-1, 2, -3, 4],
        },
        Cmd::SetClipPlane {
            device: d,
            index: 2,
            plane: [9, 8, 7, 6],
        },
        Cmd::SetGammaRamp {
            device: d,
            swap_chain: 0,
            flags: 1,
            ramp: Data::Bytes(&p.ramp),
        },
        Cmd::SetRenderState {
            device: d,
            state: 7,
            value: 1,
        },
        Cmd::SetTextureStageState {
            device: d,
            stage: 1,
            kind: 2,
            value: 3,
        },
        Cmd::SetSamplerState {
            device: d,
            sampler: 257,
            kind: 5,
            value: 2,
        },
        Cmd::SetTexture {
            device: d,
            sampler: 3,
            texture: 0x4000,
        },
        Cmd::SetVertexDeclaration {
            device: d,
            declaration: 0x4100,
        },
        Cmd::SetFvf {
            device: d,
            fvf: 0x152,
        },
        Cmd::SetShader {
            device: d,
            pixel: true,
            shader: 0x4200,
        },
        Cmd::SetConstants {
            device: d,
            kind: 3,
            start: 2,
            count: 3,
            data: w(&p.registers),
        },
        Cmd::SetStreamSource {
            device: d,
            stream: 1,
            buffer: 0x4300,
            offset: 16,
            stride: 32,
        },
        Cmd::SetIndices {
            device: d,
            buffer: 0x4400,
        },
        Cmd::DrawPrimitive {
            device: d,
            kind: 5,
            start: 1,
            count: 2,
        },
        Cmd::DrawIndexedPrimitive {
            device: d,
            kind: 4,
            base_vertex: -7,
            min_index: 1,
            vertices: 2,
            start_index: 3,
            count: 4,
        },
        Cmd::DrawPrimitiveUp {
            device: d,
            kind: 4,
            count: 1,
            stride: 12,
            vertices: w(&p.vertices),
        },
        Cmd::SetMaterial {
            device: d,
            material: w(&p.material),
        },
        Cmd::SetLight {
            device: d,
            index: 1,
            light: w(&p.light),
        },
        Cmd::LightEnable {
            device: d,
            index: 1,
            enable: 1,
        },
        Cmd::SetSoftwareVertexProcessing {
            device: d,
            software: 0,
        },
        Cmd::UpdateTexture {
            device: d,
            source: 1,
            destination: 2,
        },
        Cmd::StretchRect {
            device: d,
            source: 1,
            source_rect: None,
            destination: 2,
            destination_rect: Some(w(&p.rect)),
            filter: 2,
        },
        Cmd::ColorFill {
            device: d,
            surface: 1,
            rect: Some(w(&p.other_rect)),
            colour: 5,
        },
        Cmd::CreateTexture {
            device: d,
            texture: 0x4000,
            levels: w(&p.handles),
            width: 256,
            height: 128,
            usage: 0,
            format: 21,
            pool: 1,
        },
        Cmd::CreateCubeTexture {
            device: d,
            texture: 0x4500,
            surfaces: w(&p.cube),
            edge: 64,
            usage: 1,
            format: 22,
            pool: 0,
        },
        Cmd::CreateSurface {
            device: d,
            surface: 0x4600,
            depth: true,
            width: 1,
            height: 2,
            format: 75,
            multisample: 0,
            quality: 0,
            flag: 1,
        },
        Cmd::CreateOffscreenSurface {
            device: d,
            surface: 0x4700,
            width: 3,
            height: 4,
            format: 21,
            pool: 2,
        },
        Cmd::CreateBuffer {
            device: d,
            buffer: 0x4300,
            index: false,
            length: 4096,
            usage: 0x208,
            format: 0x152,
            pool: 0,
        },
        Cmd::CreateShader {
            device: d,
            shader: 0x4200,
            pixel: false,
            tokens: w(&p.tokens),
        },
        Cmd::CreateDeclaration {
            device: d,
            declaration: 0x4100,
            elements: w(&p.elements),
        },
        Cmd::CreateQuery {
            device: d,
            query: 0x4800,
            kind: 9,
        },
        Cmd::Destroy { object: 0x4800 },
        Cmd::UploadSurface {
            surface: 0x4600,
            row: 4,
            rows: 2,
            data: w(&p.level),
        },
        Cmd::UploadBuffer {
            buffer: 0x4300,
            offset: 8,
            flags: 0x1000,
            data: big,
        },
        Cmd::SetLod {
            texture: 0x4000,
            lod: 2,
            model: 1,
        },
        Cmd::QueryIssue {
            query: 0x4800,
            flags: 2,
        },
        Cmd::ReadSurface {
            surface: 0x4600,
            size: 8,
        },
        Cmd::GetRenderTargetData {
            device: d,
            source: 1,
            destination: 2,
            pitch: 0,
        },
        Cmd::QueryGetData {
            query: 0x4800,
            flags: 0,
        },
    ]
}

#[test]
fn every_command_is_listed_once_and_well_formed() {
    let p = Payloads::new();
    let cmds = every_command(&p, Data::Bytes(&[1, 2, 3]));
    let names: Vec<&str> = cmds.iter().map(|c| c.name()).collect();
    assert_eq!(names, NAMES);
    for c in &cmds {
        assert_eq!(c.check(), Ok(()), "{}", c.name());
    }
    let sync: Vec<&str> = cmds
        .iter()
        .filter(|c| c.is_sync())
        .map(|c| c.name())
        .collect();
    assert_eq!(
        sync,
        [
            "CreateDevice",
            "ReadSurface",
            "GetRenderTargetData",
            "QueryGetData"
        ]
    );
}

/// A payload of another size than its call gives it is refused: by the check, on submitting in a
/// debug build, and on reading a file.
#[test]
fn a_payload_of_the_wrong_size_is_refused() {
    let short = [0u32; 15];
    let bad = Cmd::SetTransform {
        device: 1,
        state: 256,
        matrix: Data::from_words(&short),
    };
    assert!(bad
        .check()
        .unwrap_err()
        .contains("matrix is 60 bytes, not 64"));
    let booleans = [1u32; 3];
    let ok = Cmd::SetConstants {
        device: 1,
        kind: 2,
        start: 0,
        count: 3,
        data: Data::from_words(&booleans),
    };
    assert_eq!(ok.check(), Ok(()));
    let mut bytes = wire::header().to_vec();
    wire::encode(&bad, &mut bytes);
    assert!(matches!(
        wire::commands(&bytes).unwrap().next(),
        Some(Err(wire::WireError::Payload { .. }))
    ));
    if cfg!(debug_assertions) {
        let refused = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            Stream::direct(Box::new(Log::default())).submit(bad);
        }));
        assert!(refused.is_err());
    }
}

/// A file being written that the test reads afterwards.
#[derive(Clone, Default)]
struct File(Arc<Mutex<Vec<u8>>>);

impl Write for File {
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(b);
        Ok(b.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[test]
fn every_command_reads_back_as_written_and_replays() {
    let p = Payloads::new();
    let fill = |offset: u32, into: &mut [u8]| {
        for (i, b) in into.iter_mut().enumerate() {
            *b = (offset as usize + i) as u8;
        }
    };
    let big = Data::Read {
        len: 70_000,
        read: &fill,
    };
    let cmds = every_command(&p, big);
    let log = Log::default();
    let lines = log.lines();
    let mut stream = Stream::direct(Box::new(log));
    let file = File::default();
    stream.record_to(Box::new(file.clone())).unwrap();
    for c in &cmds {
        stream.submit(*c);
    }
    drop(stream);
    let bytes = file.0.lock().unwrap().clone();
    let read: Vec<String> = wire::commands(&bytes)
        .unwrap()
        .map(|c| format!("{:?}", c.unwrap()))
        .collect();
    let written = lines.lock().unwrap().clone();
    assert_eq!(read.len(), NAMES.len());
    same(&read, &written, "read from the file");
    // Replayed into another backend: the same commands.
    let mut again = Log::default();
    let replayed = again.lines();
    assert_eq!(
        wire::replay(&bytes, &mut again).unwrap(),
        NAMES.len() as u64
    );
    same(&replayed.lock().unwrap(), &written, "replayed");
    // And through a chunk (the render thread's way).
    let mut c = crate::chunk::Chunk::default();
    for cmd in &cmds {
        c.push(*cmd);
    }
    let chunked: Vec<String> = (0..c.len()).map(|i| format!("{:?}", c.get(i))).collect();
    same(&chunked, &written, "through a chunk");
}

#[test]
fn a_damaged_file_is_refused() {
    assert_eq!(
        wire::commands(b"not a stream").err(),
        Some(wire::WireError::NotAStream)
    );
    let mut bytes = wire::header().to_vec();
    wire::encode(&Cmd::SetFvf { device: 1, fvf: 2 }, &mut bytes);
    let whole = bytes.clone();
    bytes.truncate(bytes.len() - 1);
    let mut cmds = wire::commands(&bytes).unwrap();
    assert!(matches!(
        cmds.next(),
        Some(Err(wire::WireError::Truncated { .. }))
    ));
    assert!(cmds.next().is_none());
    let mut unknown = whole.clone();
    unknown[16 + 4] = 0xff;
    assert!(matches!(
        wire::commands(&unknown).unwrap().next(),
        Some(Err(wire::WireError::UnknownCommand { tag: 0xff, .. }))
    ));
    assert_eq!(wire::commands(&whole).unwrap().count(), 1);
    // Written with another command list: refused before any command is read.
    let mut other = whole.clone();
    other[8] ^= 1;
    assert!(matches!(
        wire::commands(&other).err(),
        Some(wire::WireError::OtherList { .. })
    ));
}

/// The list's hash covers each command's place and fields: two commands of the same layout
/// (SetVertexDeclaration and SetFvf) swapped would give another hash.
#[test]
fn the_list_hash_covers_names_order_and_fields() {
    let list = crate::cmd::LIST;
    let a = list.find("SetVertexDeclaration").unwrap();
    let b = list.find("SetFvf").unwrap();
    assert!(a < b);
    let mut swapped = list.to_string();
    swapped.replace_range(b..b + "SetFvf".len(), "SetVertexDeclaration");
    swapped.replace_range(a..a + "SetVertexDeclaration".len(), "SetFvf");
    assert_ne!(
        wire::hash(wire::HASH_START, swapped.as_bytes()),
        wire::LIST_HASH
    );
    assert_eq!(
        wire::hash(wire::HASH_START, list.as_bytes()),
        wire::LIST_HASH
    );
    assert!(
        list.contains("SetConstants") && list.contains("optional"),
        "{list}"
    );
}

#[test]
fn a_tee_feeds_both_and_the_primary_answers() {
    let primary = Log::default();
    let shadow = Log {
        offset: 1000,
        ..Log::default()
    };
    let (a, b) = (primary.lines(), shadow.lines());
    let tee = Tee::new(Box::new(primary), Box::new(shadow));
    let mut stream = Stream::threaded(Box::new(tee), Settings::default());
    let replies = play(&mut stream, 5);
    drop(stream);
    let (_, alone) = run(Stream::direct, 5);
    assert_eq!(replies, alone, "the primary's answers");
    assert_eq!(*a.lock().unwrap(), *b.lock().unwrap());
    assert!(a.lock().unwrap().iter().any(|l| l.contains("frame: 5")));
}

#[test]
fn a_shadow_that_panics_is_left_out() {
    let shadow = Log {
        refuse: Some("SetFvf"),
        ..Log::default()
    };
    let shadow_lines = shadow.lines();
    let primary = Log::default();
    let lines = primary.lines();
    let mut tee = Tee::new(Box::new(primary), Box::new(shadow));
    for fvf in 0..3 {
        tee.execute(Cmd::SetFvf { device: 1, fvf });
    }
    assert!(!tee.shadow_running());
    assert_eq!(lines.lock().unwrap().len(), 3);
    assert_eq!(shadow_lines.lock().unwrap().len(), 1);
}

/// Two logs are the same (the first difference printed, not the whole logs).
fn same(a: &[String], b: &[String], what: &str) {
    if let Some(i) = (0..a.len().max(b.len())).find(|&i| a.get(i) != b.get(i)) {
        let short =
            |s: Option<&String>| s.map_or("(none)".to_string(), |s| s.chars().take(300).collect());
        panic!(
            "{what}: line {i} differs:\n  {}\n  {}",
            short(a.get(i)),
            short(b.get(i))
        );
    }
}

/// The recording side's cost a command, against a backend that does nothing: carried out at once
/// (direct) and recorded for the render thread (threaded), over frames shaped like the town's
/// (about 7,000 commands: 640 draws of 11 calls, constants of 4 registers).
/// `cargo test --release -p d3d9-stream cost_of_recording -- --ignored --nocapture`.
#[test]
#[ignore]
fn cost_of_recording() {
    struct Nothing;
    impl Backend for Nothing {
        fn execute(&mut self, cmd: Cmd<Data<'_>>) -> Reply {
            std::hint::black_box(&cmd);
            Reply::None
        }
    }
    let registers: Vec<u32> = (0..16).collect();
    let frame = |stream: &mut Stream| {
        for i in 0..640u32 {
            for kind in [0u8, 0, 0, 3, 3] {
                stream.submit(Cmd::SetConstants {
                    device: 1,
                    kind,
                    start: i % 200,
                    count: 4,
                    data: Data::from_words(&registers),
                });
            }
            stream.submit(Cmd::SetSamplerState {
                device: 1,
                sampler: i % 8,
                kind: 5,
                value: i,
            });
            stream.submit(Cmd::SetTexture {
                device: 1,
                sampler: i % 8,
                texture: 0x4000 + i,
            });
            stream.submit(Cmd::SetRenderState {
                device: 1,
                state: 7,
                value: i,
            });
            stream.submit(Cmd::SetStreamSource {
                device: 1,
                stream: 0,
                buffer: 0x5000 + i,
                offset: 0,
                stride: 32,
            });
            stream.submit(Cmd::SetIndices {
                device: 1,
                buffer: 0x6000 + i,
            });
            stream.submit(Cmd::DrawIndexedPrimitive {
                device: 1,
                kind: 4,
                base_vertex: 0,
                min_index: 0,
                vertices: 100,
                start_index: 0,
                count: 50,
            });
        }
        stream.submit(Cmd::Present {
            device: 1,
            source: None,
            dest: None,
            window: 0,
            dirty: false,
            model: 0,
            frame: 0,
        });
    };
    let frames = 300;
    let commands = f64::from(frames * (640 * 11 + 1));
    for threaded in [false, true] {
        let mut stream = if threaded {
            Stream::threaded(Box::new(Nothing), Settings::default())
        } else {
            Stream::direct(Box::new(Nothing))
        };
        frame(&mut stream);
        let t = std::time::Instant::now();
        for _ in 0..frames {
            frame(&mut stream);
        }
        let ns = t.elapsed().as_nanos() as f64 / commands;
        eprintln!(
            "{}: {ns:.1} ns a command, {:.3} ms a frame of 7,041",
            if threaded { "threaded" } else { "direct" },
            ns * 7041.0 / 1e6
        );
    }
}

/// [`cost_of_recording`]'s chunk part alone: the commands pushed into a chunk and the chunk
/// emptied every 1,024, on one thread.
#[test]
#[ignore]
fn cost_of_pushing() {
    let registers: Vec<u32> = (0..16).collect();
    let mut c = crate::chunk::Chunk::default();
    let rounds = 300 * 7041 / 11;
    let t = std::time::Instant::now();
    for i in 0..rounds as u32 {
        for kind in [0u8, 0, 0, 3, 3] {
            c.push(Cmd::SetConstants {
                device: 1,
                kind,
                start: i % 200,
                count: 4,
                data: Data::from_words(&registers),
            });
        }
        for _ in 0..6 {
            c.push(Cmd::SetRenderState {
                device: 1,
                state: 7,
                value: i,
            });
        }
        if c.len() >= 1024 {
            std::hint::black_box(&c);
            c.clear();
        }
    }
    let ns = t.elapsed().as_nanos() as f64 / (rounds * 11) as f64;
    eprintln!("pushing: {ns:.1} ns a command");
}

/// The stream can live in state the game's threads share (behind their lock), and a reply can
/// cross from the render thread.
#[test]
fn the_stream_and_its_replies_move_between_threads() {
    fn moves<T: Send>() {}
    moves::<Stream>();
    moves::<Reply>();
    moves::<Tee>();
}

/// A backend that refuses every command and panics again when dropped.
struct Bomb;

impl Backend for Bomb {
    fn execute(&mut self, cmd: Cmd<Data<'_>>) -> Reply {
        panic!("the bomb refuses {}", cmd.name());
    }
}

impl Drop for Bomb {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            panic!("the bomb goes off in its drop");
        }
    }
}

#[test]
fn a_shadow_that_panics_in_its_drop_too_is_left_out() {
    let primary = Log::default();
    let lines = primary.lines();
    let mut tee = Tee::new(Box::new(primary), Box::new(Bomb));
    for fvf in 0..3 {
        tee.execute(Cmd::SetFvf { device: 1, fvf });
    }
    assert!(!tee.shadow_running());
    assert_eq!(lines.lock().unwrap().len(), 3);
}

/// A render thread stopped by its backend stops the recording side at its next command, not only
/// at the next wait.
#[test]
fn the_next_command_after_a_backend_panic_stops() {
    let log = Log {
        refuse: Some("SetFvf"),
        ..Log::default()
    };
    let mut stream = Stream::threaded(Box::new(log), Settings::default());
    stream.submit(Cmd::SetFvf { device: 1, fvf: 2 });
    stream.flush();
    let stopped = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        for value in 0..10_000 {
            stream.submit(Cmd::SetRenderState {
                device: 1,
                state: 7,
                value,
            });
            std::thread::sleep(Duration::from_millis(1));
        }
    }));
    let message = crate::runner::panic_message(&*stopped.expect_err("the recording side stops"));
    assert!(message.contains("refuses SetFvf"), "{message}");
}

/// A stopped render thread stops a caller that only hands over (`flush`), with nothing recorded
/// since: a poll that never records would otherwise spin for ever.
#[test]
fn a_flush_after_the_render_thread_stopped_stops_too() {
    let log = Log {
        refuse: Some("SetFvf"),
        ..Log::default()
    };
    let mut stream = Stream::threaded(Box::new(log), Settings::default());
    stream.submit(Cmd::SetFvf { device: 1, fvf: 2 });
    stream.flush();
    let stopped = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        for _ in 0..10_000 {
            stream.flush();
            std::thread::sleep(Duration::from_millis(1));
        }
    }));
    let message = crate::runner::panic_message(&*stopped.expect_err("the flush stops"));
    assert!(message.contains("refuses SetFvf"), "{message}");
}
