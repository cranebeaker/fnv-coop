//! The platform's DirectShow filter graph, for the game's music: `CoCreateInstance` of
//! `CLSID_FilterGraph` and the interfaces `FalloutAudioMedia::ThreadProc` (`00830b30`) uses on it:
//! `IFilterGraph`/`IGraphBuilder` (`AddFilter`, `Render`), `IMediaControl` (`Run`, `Pause`,
//! `Stop`), `IMediaPosition` (`get_Duration`, `get_CurrentPosition`, `put_CurrentPosition`),
//! `IBasicAudio` (`put_Volume`) and `IMediaEvent` (`GetEvent`, `FreeEventParams`). The game adds
//! its own source filter (`CBSAReader`, over the `.mp3` in the archives) and renders its output
//! pin; Windows then builds the rest of the graph (measured: "MPEG-I Stream Splitter", "MP3
//! Decoder DMO", "Default DirectSound Device"). Here `Render` connects the game's pin to this
//! layer's own input pin through the game's own pin code, reads the whole stream through the
//! pin's `IAsyncReader` (`Length`, `SyncRead`) and its format from its first frame, and has it
//! decoded with `crates/mp3` on an OS thread of its own ([`render`], [`Decoding`]). The graph
//! keeps time on the platform clock (`Engine::clock_now_us`); with the host's output on
//! (`super::dsound::want_output`) the decoded samples play as one mixer stream that follows the
//! graph's `Run`, `Pause`, `Stop`, position and volume ([`sound_from`]).
//!
//! Measured on this machine (2026-10-24, `scripts/dshow-measure` on the player's `Data\Music`,
//! read only, the graph muted; `build43-scratch/measure/dshow-*-2026-10-24.tsv`):
//! - `get_Duration` of all 199 tracks is `floor(size * 8 * 10^7 / bitrate)` 100-ns units, the
//!   size the stream's and the bitrate its first audio frame's, bit for bit as a `double`.
//! - `AddFilter(f, NULL)` names the filter `"0001"` (the graph numbers the filters it names).
//! - `Run` from stopped answers `S_FALSE`; the first one ever queues `EC_CLOCK_CHANGED` at once,
//!   and every one from stopped an `EC_PAUSED` that comes later (the next 100 ms round);
//!   `Run` while paused or running answers `S_OK` and queues nothing. Every `Pause` (running or
//!   paused) answers `S_OK` and queues `EC_PAUSED` at once. `Stop` answers `S_OK` and keeps the
//!   position. (A graph still leaving stopped, and `Pause` from stopped: below.)
//! - The position moves at the stream's rate while running (from 36 to 58 ms after `Run` on the
//!   device: here at once) and stays while paused or stopped; at the end it is the duration
//!   and `EC_COMPLETE` (code 1, parameters 0) is queued. `put_CurrentPosition` below 0 is
//!   `E_INVALIDARG`; past the end it is the duration.
//! - `GetEvent` with nothing queued answers `E_ABORT`. `put_Volume` takes -10000..0, else
//!   `E_INVALIDARG`.
//! - The decoded samples (the MP3 Decoder DMO's output, grabbed in the graph): see
//!   [`Decoding`]: this layer's samples are Windows' for 99.9748% of all 199 tracks' samples.
//!
//! Measured on this machine (2026-10-31, `scripts/dshow-measure --states` and `--rounds` on a
//! scratch copy of `mus_BTTL_Lp_Evil1_City_Full.mp3`: 43 scenarios, each on a fresh graph,
//! muted, run 4 to 9 times; `build43-scratch/measure/dshow-states-2026-10-31`): the state
//! changes the music thread can reach. Its rounds come whenever the main loop sets its event
//! (`0082fb70`, every frame) or after 100 ms; each makes one of `Stop` (flags & 2), `Pause`
//! (flags & 4, byte `011dd312` or `00831de0`; `0083193b`, before any `Run` in the first round)
//! or `Run`, after a synch `put_CurrentPosition` (`008318b2`) and before the loop restart
//! (`Stop`, `put_CurrentPosition(0)`, `Run` on events 1 to 3):
//! - Leaving stopped (`Run` or `Pause` from stopped, on a graph that never ran or one that ran
//!   and stopped) answers `S_FALSE`; the first time the graph leaves stopped, whichever call it
//!   is, it queues `EC_CLOCK_CHANGED` at once. Its `EC_PAUSED` comes later: mostly 9.2 to 11.3
//!   ms after the call returned, at times 20 to 37 ms. The position stays where it was (0 on a
//!   new graph, or where it was put). `GetState(0)` answers `VFW_S_STATE_INTERMEDIATE` and the
//!   new state at first.
//! - It settles about a millisecond after the call returned: `Run` or `Pause` 0.5 ms after it
//!   answer `S_FALSE` and queue nothing (one `EC_PAUSED` comes in all); 1 ms after or later
//!   they answer `S_OK` (a `Pause` with its own `EC_PAUSED` at once) and the `EC_PAUSED` still
//!   to come never comes.
//! - `Stop` answers `S_OK` from every state: on a graph that never ran too, which does not
//!   choose its clock (the first `Run` after it queues `EC_CLOCK_CHANGED`); an `EC_PAUSED`
//!   still to come never comes. `put_CurrentPosition` while stopped or paused answers `S_OK`,
//!   moves the position and leaves the state and an `EC_PAUSED` to come as they are.
//! - `Run` on a graph paused at the end queues `EC_COMPLETE` at once.
//!
//! What this layer does differently, as nothing of it is visible to the game's own code but the
//! calls into its filter: Windows' splitter reads the stream as it plays (requests of its own);
//! here the whole stream is read once, at `Render`. The filter's own state methods (`Pause`,
//! `Run`, `Stop`, `SetSyncSource`) are not called: no stream runs through it here. Windows'
//! graph decodes on threads of its own while it runs; here the stream decodes on an OS thread
//! of its own from `Render` on, beside the game's threads, in the order it is played
//! ([`Decoding`]), and the mixer stream takes the samples as it reads them
//! (`super::mixer::Feed`). The game's calls answer from the stream's format and the clock,
//! never from the samples, so none of them waits for the decoding. The calls take no time here
//! (on Windows `Run` and `Pause` from stopped 10 to 19 ms, once 116 ms; `Stop` up to 36 ms;
//! `Run` from paused up to 133 ms), the `EC_PAUSED` of a transition comes 10 ms after it, and the
//! graph settles 1 ms after it. Not done here (measured 2026-10-31, `measure\dshow-states`):
//! `EC_COMPLETE` comes on Windows 35 to 208 ms after the position reached the end (24 times);
//! a `put_CurrentPosition` right after a `Run` from stopped (within 0.1 ms) holds the position
//! and that run's `EC_PAUSED` 42 to 60 ms (4 runs) or 118 to 137 ms (4 runs) after the put returned; and a
//! `put_CurrentPosition` right before a `Run` from stopped gives its `EC_PAUSED` 44 to 64 ms
//! after the `Run` returned (4 of 4). `get_event` hands out only the front event, so an `EC_PAUSED` to
//! come holds back an `EC_COMPLETE` queued behind it.

use super::mixer::Feed;
use crate::{Engine, Ptr};
use std::collections::VecDeque;
use std::sync::Arc;

/// Where the platform's DirectShow functions sit (code addresses no exe code uses).
pub const CODE_BASE: u32 = 0x7e00_0800;
const BUILDER_CODE: u32 = CODE_BASE;
const CONTROL_CODE: u32 = CODE_BASE + 0x80;
const POSITION_CODE: u32 = CODE_BASE + 0x100;
const AUDIO_CODE: u32 = CODE_BASE + 0x180;
const EVENT_CODE: u32 = CODE_BASE + 0x200;
const PIN_CODE: u32 = CODE_BASE + 0x280;

/// The interfaces of a graph object, by their offset in it.
const BUILDER: u32 = 0;
const CONTROL: u32 = 4;
const POSITION: u32 = 8;
const AUDIO: u32 = 0xc;
const EVENT: u32 = 0x10;
const GRAPH_BYTES: u32 = 0x14;

/// Result codes (winerror.h, vfwmsgs.h).
pub const S_OK: u32 = 0;
pub const S_FALSE: u32 = 1;
pub const E_NOINTERFACE: u32 = 0x8000_4002;
pub const E_NOTIMPL: u32 = 0x8000_4001;
pub const E_ABORT: u32 = 0x8000_4004;
pub const E_INVALIDARG: u32 = 0x8007_0057;
pub const VFW_E_TYPE_NOT_ACCEPTED: u32 = 0x8004_022a;
/// Event codes (evcode.h).
pub const EC_COMPLETE: u32 = 1;
pub const EC_CLOCK_CHANGED: u32 = 0x0d;
pub const EC_PAUSED: u32 = 0x0e;

fn guid(text: &str) -> [u8; 16] {
    let h: Vec<u8> = text
        .bytes()
        .filter(u8::is_ascii_hexdigit)
        .map(|c| (c as char).to_digit(16).unwrap_or(0) as u8)
        .collect();
    let b = |i: usize| h[2 * i] << 4 | h[2 * i + 1];
    [
        b(3),
        b(2),
        b(1),
        b(0),
        b(5),
        b(4),
        b(7),
        b(6),
        b(8),
        b(9),
        b(10),
        b(11),
        b(12),
        b(13),
        b(14),
        b(15),
    ]
}

/// The ids the game passes (from its `.rdata`) and those of the pin protocol.
fn clsid_filter_graph() -> [u8; 16] {
    guid("e436ebb3-524f-11ce-9f53-0020af0ba770")
}
fn iid(name: &str) -> [u8; 16] {
    guid(match name {
        "IUnknown" => "00000000-0000-0000-c000-000000000046",
        "IFilterGraph" => "56a8689f-0ad4-11ce-b03a-0020af0ba770",
        "IGraphBuilder" => "56a868a9-0ad4-11ce-b03a-0020af0ba770",
        "IMediaControl" => "56a868b1-0ad4-11ce-b03a-0020af0ba770",
        "IMediaPosition" => "56a868b2-0ad4-11ce-b03a-0020af0ba770",
        "IBasicAudio" => "56a868b3-0ad4-11ce-b03a-0020af0ba770",
        "IMediaEvent" => "56a868b6-0ad4-11ce-b03a-0020af0ba770",
        "IMediaEventEx" => "56a868c0-0ad4-11ce-b03a-0020af0ba770",
        "IPin" => "56a86891-0ad4-11ce-b03a-0020af0ba770",
        "IAsyncReader" => "56a868aa-0ad4-11ce-b03a-0020af0ba770",
        "MEDIATYPE_Stream" => "e436eb83-524f-11ce-9f53-0020af0ba770",
        "MEDIASUBTYPE_MPEG1Audio" => "e436eb87-524f-11ce-9f53-0020af0ba770",
        _ => unreachable!("an id this layer names"),
    })
}

/// Where a graph is.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Run {
    Stopped,
    Paused,
    /// Running since the clock read `start` (microseconds).
    Running {
        start: u64,
    },
}

/// A filter graph.
#[derive(Debug)]
struct Graph {
    references: u32,
    /// The game's filter (its `IBaseFilter`) and this layer's input pin.
    filter: u32,
    pin: u32,
    /// The stream's length and its duration (100 ns).
    stream_bytes: u64,
    duration: i64,
    /// The samples (interleaved 16-bit), decoded from `Render` on by the thread `decode` until
    /// it is seen to end ([`decoder_ended`]), and their format.
    samples: Arc<Feed>,
    decode: Option<std::thread::JoinHandle<()>>,
    sample_rate: u32,
    channels: u16,
    run: Run,
    /// The stream position (100 ns) where it stopped or paused, or where the run began.
    base: i64,
    /// Whether the graph ever left stopped (its first `Run` or `Pause`, which chose its clock).
    clocked: bool,
    /// Until when (clock, microseconds) the graph is still settling after it left stopped: `Run`
    /// and `Pause` then answer `S_FALSE` and leave the `EC_PAUSED` to come as it is.
    settles: u64,
    completed: bool,
    /// Queued events: code, two parameters, and the clock time (microseconds) after which it
    /// can be taken.
    events: VecDeque<(u32, u32, u32, u64)>,
    volume: i32,
}

/// This layer's input pin.
#[derive(Debug)]
struct Pin {
    references: u32,
    connected: u32,
    reader: u32,
}

/// The DirectShow objects (in `Kernel::sound`).
#[derive(Debug, Default)]
pub struct Show {
    vtables: [u32; 6],
    graphs: std::collections::BTreeMap<u32, Graph>,
    pins: std::collections::BTreeMap<u32, Pin>,
    /// The filters `AddFilter` named so far (the next name's number).
    named: u32,
    /// The tests' switch: `Render`'s decoders wait to be let start (`Feed::start`).
    #[cfg(test)]
    hold: bool,
}

type Slots<const N: usize> = [(u32, crate::AbiFn); N];

fn register_code(e: &mut Engine) {
    let builder: Slots<5> = [
        (0, crate::entry!(0, query_interface(Ptr, Ptr, Ptr) -> u32).1),
        (1, crate::entry!(0, add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, release(Ptr) -> u32).1),
        (3, crate::entry!(0, add_filter(Ptr, Ptr, Ptr) -> u32).1),
        (12, crate::entry!(0, render(Ptr, Ptr) -> u32).1),
    ];
    let control: Slots<6> = [
        (0, crate::entry!(0, query_interface(Ptr, Ptr, Ptr) -> u32).1),
        (1, crate::entry!(0, add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, release(Ptr) -> u32).1),
        (7, crate::entry!(0, control_run(Ptr) -> u32).1),
        (8, crate::entry!(0, control_pause(Ptr) -> u32).1),
        (9, crate::entry!(0, control_stop(Ptr) -> u32).1),
    ];
    let position: Slots<6> = [
        (0, crate::entry!(0, query_interface(Ptr, Ptr, Ptr) -> u32).1),
        (1, crate::entry!(0, add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, release(Ptr) -> u32).1),
        (7, crate::entry!(0, get_duration(Ptr, Ptr) -> u32).1),
        (8, crate::entry!(0, put_current_position(Ptr, f64) -> u32).1),
        (9, crate::entry!(0, get_current_position(Ptr, Ptr) -> u32).1),
    ];
    let audio: Slots<5> = [
        (0, crate::entry!(0, query_interface(Ptr, Ptr, Ptr) -> u32).1),
        (1, crate::entry!(0, add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, release(Ptr) -> u32).1),
        (7, crate::entry!(0, put_volume(Ptr, i32) -> u32).1),
        (8, crate::entry!(0, get_volume(Ptr, Ptr) -> u32).1),
    ];
    let event: Slots<5> = [
        (0, crate::entry!(0, query_interface(Ptr, Ptr, Ptr) -> u32).1),
        (1, crate::entry!(0, add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, release(Ptr) -> u32).1),
        (
            8,
            crate::entry!(0, get_event(Ptr, Ptr, Ptr, Ptr, i32) -> u32).1,
        ),
        (
            12,
            crate::entry!(0, free_event_params(Ptr, u32, u32, u32) -> u32).1,
        ),
    ];
    let pin: Slots<7> = [
        (
            0,
            crate::entry!(0, pin_query_interface(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (1, crate::entry!(0, pin_add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, pin_release(Ptr) -> u32).1),
        (
            4,
            crate::entry!(0, receive_connection(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (5, crate::entry!(0, pin_disconnect(Ptr) -> u32).1),
        (9, crate::entry!(0, query_direction(Ptr, Ptr) -> u32).1),
        (12, crate::entry!(0, enum_media_types(Ptr, Ptr) -> u32).1),
    ];
    let sets: [(&[(u32, crate::AbiFn)], u32); 6] = [
        (&builder, BUILDER_CODE),
        (&control, CONTROL_CODE),
        (&position, POSITION_CODE),
        (&audio, AUDIO_CODE),
        (&event, EVENT_CODE),
        (&pin, PIN_CODE),
    ];
    for (i, (slots, code)) in sets.into_iter().enumerate() {
        for &(slot, f) in slots {
            e.register(code + 4 * slot, f);
        }
        // Each vtable is 32 slots; a slot not provided stops the run as an open function.
        let table = e.mem.alloc(4 * 32);
        for s in 0..32 {
            e.mem.set_u32(table + 4 * s, code + 4 * s);
        }
        e.kernel.sound.show.vtables[i] = table;
    }
}

/// `CoCreateInstance(clsid, outer, context, iid, out)`: `CLSID_FilterGraph` in process
/// (`CLSCTX_INPROC_SERVER`), no outer object, asked for one of the graph's interfaces: a new
/// graph. Every other class is not provided (the DirectShow filter mapper the game's filter
/// registration would ask for, `009fba40`/`009fbab0`, is not reached by the game's run).
pub fn co_create_instance(
    e: &mut Engine,
    clsid: Ptr,
    outer: u32,
    context: u32,
    interface: Ptr,
    out: Ptr,
) -> u32 {
    let class = e.mem.bytes(clsid.addr(), 16);
    let wanted = e.mem.bytes(interface.addr(), 16);
    assert!(
        class == clsid_filter_graph() && outer == 0 && context == 1,
        "CoCreateInstance({}, outer {outer:#x}, context {context:#x}) is not provided",
        hex(&class)
    );
    if e.kernel.sound.show.vtables[0] == 0 {
        register_code(e);
    }
    let at = e.mem.alloc(GRAPH_BYTES);
    for i in 0..5 {
        let table = e.kernel.sound.show.vtables[i as usize];
        e.mem.set_u32(at + 4 * i, table);
    }
    e.kernel.sound.show.graphs.insert(
        at,
        Graph {
            references: 0,
            filter: 0,
            pin: 0,
            stream_bytes: 0,
            duration: 0,
            samples: Arc::default(),
            decode: None,
            sample_rate: 0,
            channels: 0,
            run: Run::Stopped,
            base: 0,
            clocked: false,
            settles: 0,
            completed: false,
            events: VecDeque::new(),
            volume: 0,
        },
    );
    let r = query(e, at, &wanted, out);
    assert!(
        r == S_OK,
        "CoCreateInstance(CLSID_FilterGraph) for {} is not provided",
        hex(&wanted)
    );
    r
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// The graph an interface pointer belongs to, from the vtable it holds.
fn graph_of(e: &Engine, this: Ptr) -> u32 {
    let vtable = e.mem.u32(this.addr());
    let tables = e.kernel.sound.show.vtables;
    let offset = [BUILDER, CONTROL, POSITION, AUDIO, EVENT]
        .into_iter()
        .zip(tables)
        .find(|&(_, t)| t == vtable)
        .map(|(o, _)| o)
        .expect("an interface of this layer's filter graph");
    this.addr() - offset
}

fn graph(e: &mut Engine, at: u32) -> &mut Graph {
    e.kernel
        .sound
        .show
        .graphs
        .get_mut(&at)
        .expect("a filter graph of this layer")
}

/// Hands out the graph's interface for `wanted` with a reference.
fn query(e: &mut Engine, at: u32, wanted: &[u8], out: Ptr) -> u32 {
    let offset = if wanted == iid("IUnknown")
        || wanted == iid("IFilterGraph")
        || wanted == iid("IGraphBuilder")
    {
        BUILDER
    } else if wanted == iid("IMediaControl") {
        CONTROL
    } else if wanted == iid("IMediaPosition") {
        POSITION
    } else if wanted == iid("IBasicAudio") {
        AUDIO
    } else if wanted == iid("IMediaEvent") || wanted == iid("IMediaEventEx") {
        EVENT
    } else {
        e.mem.set_u32(out.addr(), 0);
        return E_NOINTERFACE;
    };
    graph(e, at).references += 1;
    e.mem.set_u32(out.addr(), at + offset);
    S_OK
}

fn query_interface(e: &mut Engine, this: Ptr, interface: Ptr, out: Ptr) -> u32 {
    let at = graph_of(e, this);
    let wanted = e.mem.bytes(interface.addr(), 16);
    query(e, at, &wanted, out)
}

fn add_ref(e: &mut Engine, this: Ptr) -> u32 {
    let at = graph_of(e, this);
    let g = graph(e, at);
    g.references += 1;
    g.references
}

/// `Release()`: at the last reference the graph lets go of the game's filter as DirectShow
/// does: the pins disconnected (the game's pin's `Disconnect`), `JoinFilterGraph(NULL, NULL)`
/// and the reference `AddFilter` took released. A decoder still at work stops: nothing reads
/// its samples any more ([`render`]).
fn release(e: &mut Engine, this: Ptr) -> u32 {
    let at = graph_of(e, this);
    let g = graph(e, at);
    g.references -= 1;
    let left = g.references;
    if left == 0 {
        decoder_ended(e, at);
        super::dsound::stop_stream(e, at);
        let g = e.kernel.sound.show.graphs.remove(&at).expect("the graph");
        g.samples.let_go();
        if g.pin != 0 {
            let connected = e
                .kernel
                .sound
                .show
                .pins
                .get(&g.pin)
                .map_or(0, |p| p.connected);
            if connected != 0 {
                e.vcall(connected, 0x14, &[]);
            }
            pin_disconnect(e, Ptr::new(g.pin));
            pin_release(e, Ptr::new(g.pin));
        }
        if g.filter != 0 {
            e.vcall(g.filter, 0x34, &[0, 0]);
            e.vcall(g.filter, 8, &[]);
        }
        e.mem.free(at);
    }
    left
}

/// `AddFilter(filter, name)`: keeps the filter with a reference and tells it its graph
/// (`IBaseFilter::JoinFilterGraph(graph, name)`); a NULL name becomes the graph's own
/// (`"0001"`, measured). A second filter is not provided.
fn add_filter(e: &mut Engine, this: Ptr, filter: Ptr, name: Ptr) -> u32 {
    let at = graph_of(e, this);
    assert!(
        graph(e, at).filter == 0 && !filter.is_null(),
        "IFilterGraph::AddFilter of a second filter is not provided"
    );
    e.vcall(filter.addr(), 4, &[]);
    graph(e, at).filter = filter.addr();
    let named = if name.is_null() {
        e.kernel.sound.show.named += 1;
        let text: Vec<u16> = format!("{:04}", e.kernel.sound.show.named)
            .encode_utf16()
            .chain([0])
            .collect();
        let block = e.mem.alloc(2 * text.len() as u32);
        for (i, c) in text.iter().enumerate() {
            e.mem.set_u16(block + 2 * i as u32, *c);
        }
        block
    } else {
        name.addr()
    };
    let r = e.vcall(filter.addr(), 0x34, &[at + BUILDER, named]).u32();
    if name.is_null() {
        e.mem.free(named);
    }
    r
}

/// `Render(pin)`: the game's output pin connected to this layer's input pin (the pin's own
/// `Connect`, which agrees the stream's media type and hands this pin its `IAsyncReader`); the
/// whole stream read through that reader (`Length`, `SyncRead`) and its format read from its
/// first frame (`mp3::Decoder::new` decodes nothing). A stream that is not MP3 is not provided.
///
/// The samples decode on an OS thread of their own ([`Decoding`]), and `Render` answers without
/// waiting for them, as Windows' graph build does (its graph decodes while it runs). The game
/// calls it on its music thread (`FAMThread::ThreadProc` `0082f7b0` -> `00830b30`, the call at
/// `008310d2`..`008310d5`, no lock held: `00830490` released it at `00830d9d`), and what that
/// thread does after it reads only the format and the clock: the interfaces
/// (`008310ed`..`008311a0`), `get_Duration` (`008311e6`), the wait at `0083124e`, then each pass
/// `get_CurrentPosition` (`00831284`), `put_CurrentPosition` (`008318af`), `Run`/`Pause`/`Stop`
/// (`00831960`, `0083193b`, `008318ef`) and `GetEvent` (`0083198f`). Decoded inside `Render`, a
/// track stopped every game thread for its decoding time under the cooperative thread host
/// (350 to 400 ms of main's frame for a two-minute track, at each new track:
/// `build43-scratch/measure/lag1`, 2026-10-31).
fn render(e: &mut Engine, this: Ptr, out_pin: Ptr) -> u32 {
    let at = graph_of(e, this);
    let pin = e.mem.alloc(4);
    let table = e.kernel.sound.show.vtables[5];
    e.mem.set_u32(pin, table);
    e.kernel.sound.show.pins.insert(
        pin,
        Pin {
            references: 1,
            connected: 0,
            reader: 0,
        },
    );
    graph(e, at).pin = pin;
    let r = e.vcall(out_pin.addr(), 0xc, &[pin, 0]).u32();
    if (r as i32) < 0 {
        return r;
    }
    let reader = e.kernel.sound.show.pins[&pin].reader;
    assert!(
        reader != 0,
        "IGraphBuilder::Render: the pin gave no IAsyncReader; not provided"
    );
    let lengths = e.mem.alloc(16);
    let r = e.vcall(reader, 0x20, &[lengths, lengths + 8]).u32();
    assert!(
        r == S_OK,
        "IAsyncReader::Length answered {r:#x}: not provided"
    );
    let total = e.mem.u64(lengths);
    e.mem.free(lengths);
    assert!(
        total > 0 && total < 1 << 30,
        "a stream of {total} bytes is not provided"
    );
    let buffer = e.mem.alloc(total as u32);
    let r = e.vcall(reader, 0x1c, &[0, 0, total as u32, buffer]).u32();
    assert!(
        r == S_OK,
        "IAsyncReader::SyncRead answered {r:#x}: not provided"
    );
    let data = e.mem.bytes(buffer, total as u32);
    e.mem.free(buffer);
    let decoder = ::mp3::Decoder::new(data).unwrap_or_else(|err| {
        panic!("IGraphBuilder::Render of a stream mp3 cannot read ({err}): not provided")
    });
    let bitrate = u64::from(decoder.info().bitrate);
    assert!(bitrate != 0, "a free-format MP3 is not provided");
    let (rate, channels) = (decoder.sample_rate(), decoder.channels());
    // The samples, decoded beside the game while the music thread goes on from `008310d7`.
    let decoding = Decoding::new(decoder);
    let samples = Arc::new(Feed::new(decoding.frames(), 2 * usize::from(channels)));
    #[cfg(test)]
    if e.kernel.sound.show.hold {
        samples.hold();
    }
    let fed = samples.clone();
    let decode = std::thread::Builder::new()
        .name("nv-music-decode".into())
        .spawn(move || decode_stream(decoding, &fed))
        .expect("an OS thread to decode the music on");
    let g = graph(e, at);
    g.stream_bytes = total;
    g.duration = (total * 8 * 10_000_000 / bitrate) as i64;
    g.samples = samples;
    g.decode = Some(decode);
    g.sample_rate = rate;
    g.channels = channels;
    let what = format!(
        "music {at:08x}: Render, {total} bytes, {rate} Hz, {channels} ch, {:.3} s",
        g.duration as f64 / 1e7
    );
    super::dsound::note(e, &what);
    S_OK
}

/// The graph's sound on the device (`super::dsound`'s streams, keyed by the graph's address),
/// when the host turned the output on: the decoded samples from the graph's position, at its
/// volume. Windows' "Default DirectSound Device" renderer plays them through DirectSound; here
/// they go to the same mixer as one stream, held while the graph is paused or stopped.
fn sound_from(e: &mut Engine, at: u32, position: i64) {
    let g = graph(e, at);
    let frame = (position.max(0) as u128 * u128::from(g.sample_rate) / 10_000_000) as usize;
    let volume = g.volume;
    // The decoder goes there next if it has not decoded it yet ([`Decoding::step`]).
    let decoded = g.samples.decoded(frame);
    g.samples.want(frame);
    if super::dsound::has_stream(e, at) {
        super::dsound::seek_stream(e, at, frame);
        super::dsound::pause_stream(e, at, false);
    } else {
        let g = graph(e, at);
        let (samples, channels, rate) = (g.samples.clone(), g.channels, g.sample_rate);
        super::dsound::play_stream_fed(e, at, samples, channels, rate, frame);
    }
    super::dsound::stream_volume(e, at, volume);
    let not_yet = if decoded { "" } else { " (not decoded yet)" };
    super::dsound::note(
        e,
        &format!(
            "music {at:08x}: Run from {:.3} s{not_yet}",
            position as f64 / 1e7
        ),
    );
}

/// The decoder's delay: the output frames Windows' graph drops at the start of a stream
/// (measured, [`Decoding`]).
const DELAY: usize = 528;

/// A stream's decoding into the samples Windows' graph gives for it (measured against the MP3
/// Decoder DMO's output in the player's graph on all 199 tracks: same length, 99.9748% of the
/// samples the same and the rest one step apart, the decoders' own rounding): every frame
/// decoded, the Xing/Info frame too (silence: its side information is empty), the first
/// [`DELAY`] frames of output dropped (the decoder's delay), each sample scaled by 32768 and cut
/// toward zero.
///
/// It decodes the MP3 frames into a [`Feed`] one by one ([`Decoding::step`]) in the order the
/// stream is played: from the start; from where a `Run` or `put_CurrentPosition` sends the
/// stream or the mixer reads, when that is not decoded yet (`Feed::want`, `Feed::take`); then
/// on to the end, and back to what is left. A frame decoded away from the previous one is
/// decoded from a few frames before it (`mp3::Decoder::seek`), so every sample is the one the
/// whole stream decoded in order gives, bit for bit.
struct Decoding {
    decoder: ::mp3::Decoder<Vec<u8>>,
    channels: usize,
    /// Samples per channel in an MP3 frame, and the Xing/Info frame before the audio (0 or 1).
    per_frame: usize,
    lead: usize,
    /// The stream's MP3 frames, the Xing/Info frame first: those decoded, and the one the
    /// decoder gives next.
    done: Vec<bool>,
    next: usize,
}

impl Decoding {
    fn new(mut decoder: ::mp3::Decoder<Vec<u8>>) -> Decoding {
        decoder.set_gapless(false);
        let lead = usize::from(decoder.info().xing.is_some());
        let frames = lead + decoder.frame_count();
        Decoding {
            channels: usize::from(decoder.channels()),
            per_frame: decoder.info().samples_per_frame,
            lead,
            done: vec![false; frames],
            next: 0,
            decoder,
        }
    }

    /// How many frames (of all the channels' samples) the stream gives.
    fn frames(&self) -> usize {
        (self.done.len() * self.per_frame).saturating_sub(DELAY)
    }

    /// Decodes one MP3 frame into `feed`: the one holding the frame asked for
    /// (`Feed::wanted`) or the first not decoded after it, else the next one, else the first
    /// one not decoded yet. False once all are decoded, or the samples were let go.
    fn step(&mut self, feed: &Feed) -> bool {
        if feed.is_let_go() {
            return false;
        }
        let undecoded = |from: usize| (from..self.done.len()).find(|&q| !self.done[q]);
        let asked = feed.wanted().map(|at| (at + DELAY) / self.per_frame);
        let Some(q) = asked
            .and_then(undecoded)
            .or_else(|| undecoded(self.next))
            .or_else(|| undecoded(0))
        else {
            return false;
        };
        if q != self.next {
            self.decoder.seek(q.saturating_sub(self.lead));
        }
        let samples = if q < self.lead {
            vec![0.0; self.per_frame * self.channels]
        } else {
            let frame = self.decoder.next_frame();
            frame.expect("a frame counted at Render").samples
        };
        self.done[q] = true;
        self.next = q + 1;
        let start = q * self.per_frame;
        let skip = DELAY.saturating_sub(start).min(self.per_frame);
        let pcm: Vec<u8> = samples[skip * self.channels..]
            .iter()
            .map(|&x| ((x * 32768.0).trunc() as i32).clamp(-32768, 32767) as i16)
            .flat_map(i16::to_le_bytes)
            .collect();
        feed.put(start + skip - DELAY, &pcm);
        true
    }
}

/// The decoding thread ([`render`]): the stream's samples into `feed` until all are decoded or
/// the graph lets them go. However it ends (a panic too), readers stop waiting for it.
fn decode_stream(mut decoding: Decoding, feed: &Feed) {
    struct Ending<'a>(&'a Feed);
    impl Drop for Ending<'_> {
        fn drop(&mut self) {
            self.0.end();
        }
    }
    let _ending = Ending(feed);
    if feed.started() {
        while decoding.step(feed) {}
    }
}

/// The stream's decoder ([`render`]), once it has ended: one that failed passes its panic on to
/// the game's call (`std::panic::resume_unwind`), as it stopped `Render` when the stream
/// decoded inside it. Called at the calls that read the graph's run state, so the music thread's
/// next pass meets it (each pass asks `get_CurrentPosition` at `00831284` and `GetEvent` at
/// `0083198f`, after the wait at `0083124e`).
fn decoder_ended(e: &mut Engine, at: u32) {
    let g = graph(e, at);
    if g.decode.as_ref().is_some_and(|d| d.is_finished()) {
        if let Some(Err(failed)) = g.decode.take().map(|d| d.join()) {
            std::panic::resume_unwind(failed);
        }
    }
}

/// The stream position now (100 ns), queueing `EC_COMPLETE` when a run reached the end.
fn position_now(e: &mut Engine, at: u32) -> i64 {
    decoder_ended(e, at);
    super::dsound::tick(e);
    let now = e.clock_now_us();
    let g = graph(e, at);
    match g.run {
        Run::Running { start } => {
            let p = g.base + (now.saturating_sub(start) * 10) as i64;
            if p >= g.duration {
                if !g.completed {
                    g.completed = true;
                    g.events.push_back((EC_COMPLETE, 0, 0, 0));
                }
                g.duration
            } else {
                p
            }
        }
        _ => g.base,
    }
}

/// When the pause a graph goes through as it leaves stopped (`Run` or `Pause`) completes: its
/// `EC_PAUSED` comes this long after the call (measured: mostly 9.2 to 11.3 ms after the call
/// returned, at times 20 to 37 ms).
const PAUSED_AFTER_US: u64 = 10_000;
/// How long after a `Run` or `Pause` from stopped a `Run` or `Pause` still answers `S_FALSE`
/// (measured, after either: 0.5 ms after the call returned `S_FALSE` 12 times in 12, 1 ms and
/// 1.5 ms after `S_OK` 12 in 12, 2 ms after `S_OK` 11 in 12).
const SETTLES_US: u64 = 1_000;

/// Leaving stopped (`Run` or `Pause`): the first time, the graph's clock is chosen
/// (`EC_CLOCK_CHANGED` at once); the pause the graph goes through completes later
/// (`EC_PAUSED`, [`PAUSED_AFTER_US`]); until [`SETTLES_US`] after now it is still settling.
fn leave_stopped(g: &mut Graph, now: u64) {
    if !g.clocked {
        g.clocked = true;
        g.events.push_back((EC_CLOCK_CHANGED, 0, 0, 0));
    }
    g.events.push_back((EC_PAUSED, 0, 0, now + PAUSED_AFTER_US));
    g.settles = now + SETTLES_US;
}

/// The `EC_PAUSED` still to come never comes (measured: after a `Stop`, and after a `Run` or
/// `Pause` once the graph settled).
fn drop_pause_to_come(g: &mut Graph, now: u64) {
    g.events
        .retain(|&(code, _, _, after)| code != EC_PAUSED || after <= now);
}

/// `Run` or `Pause` on a graph that is not stopped: `S_FALSE` while it is still settling from
/// stopped (the `EC_PAUSED` to come stays: one in all), else `S_OK`, and an `EC_PAUSED` still to
/// come never comes (measured).
fn settle(g: &mut Graph, now: u64) -> u32 {
    if now < g.settles {
        return S_FALSE;
    }
    drop_pause_to_come(g, now);
    S_OK
}

/// `Run()`: see the module documentation.
fn control_run(e: &mut Engine, this: Ptr) -> u32 {
    let at = graph_of(e, this);
    decoder_ended(e, at);
    let now = e.clock_now_us();
    let g = graph(e, at);
    let base = g.base;
    let r = match g.run {
        Run::Running { .. } => return settle(g, now),
        Run::Paused => settle(g, now),
        Run::Stopped => {
            leave_stopped(g, now);
            S_FALSE
        }
    };
    g.run = Run::Running { start: now };
    sound_from(e, at, base);
    r
}

/// `Pause()`: see the module documentation. From stopped (the music thread's first round with
/// its pause flags, `0083193b`) `S_FALSE`, the first time `EC_CLOCK_CHANGED` at once, and
/// `EC_PAUSED` a moment later; a settled graph `S_OK` and `EC_PAUSED` at once.
fn control_pause(e: &mut Engine, this: Ptr) -> u32 {
    let at = graph_of(e, this);
    let p = position_now(e, at);
    let now = e.clock_now_us();
    let g = graph(e, at);
    g.base = p;
    let r = if g.run == Run::Stopped {
        leave_stopped(g, now);
        S_FALSE
    } else {
        let r = settle(g, now);
        if r == S_OK {
            g.events.push_back((EC_PAUSED, 0, 0, 0));
        }
        r
    };
    g.run = Run::Paused;
    super::dsound::pause_stream(e, at, true);
    r
}

/// `Stop()`: `S_OK` from every state; the position stays where it was, an `EC_PAUSED` still to
/// come never comes (measured).
fn control_stop(e: &mut Engine, this: Ptr) -> u32 {
    let at = graph_of(e, this);
    let p = position_now(e, at);
    let now = e.clock_now_us();
    let g = graph(e, at);
    g.base = p;
    g.run = Run::Stopped;
    g.settles = 0;
    drop_pause_to_come(g, now);
    super::dsound::pause_stream(e, at, true);
    S_OK
}

/// `get_Duration(out)`: the duration in seconds (`REFTIME`).
fn get_duration(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    let at = graph_of(e, this);
    let d = graph(e, at).duration;
    e.mem.set_f64(out.addr(), d as f64 / 1e7);
    S_OK
}

/// `get_CurrentPosition(out)`: the position in seconds.
fn get_current_position(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    let at = graph_of(e, this);
    let p = position_now(e, at);
    e.mem.set_f64(out.addr(), p as f64 / 1e7);
    S_OK
}

/// `put_CurrentPosition(seconds)`: below 0 `E_INVALIDARG`, past the end the end; a running
/// graph goes on from there.
fn put_current_position(e: &mut Engine, this: Ptr, seconds: f64) -> u32 {
    let at = graph_of(e, this);
    assert!(
        !seconds.is_nan(),
        "put_CurrentPosition(NaN) is not provided"
    );
    if seconds < 0.0 {
        return E_INVALIDARG;
    }
    let now = e.clock_now_us();
    let g = graph(e, at);
    let units = ((seconds * 1e7) as i64).min(g.duration);
    g.base = units;
    g.completed = false;
    let running = matches!(g.run, Run::Running { .. });
    if running {
        g.run = Run::Running { start: now };
    }
    let frame = (units.max(0) as u128 * u128::from(g.sample_rate) / 10_000_000) as usize;
    // The decoder goes there next if it has not decoded it yet ([`Decoding::step`]).
    g.samples.want(frame);
    super::dsound::seek_stream(e, at, frame);
    if running {
        // A stream that had played to its end goes on again.
        super::dsound::pause_stream(e, at, false);
    }
    S_OK
}

/// `put_Volume(v)`: -10000..0 (hundredths of a decibel), else `E_INVALIDARG`.
fn put_volume(e: &mut Engine, this: Ptr, v: i32) -> u32 {
    let at = graph_of(e, this);
    if !(-10000..=0).contains(&v) {
        return E_INVALIDARG;
    }
    graph(e, at).volume = v;
    super::dsound::stream_volume(e, at, v);
    S_OK
}

fn get_volume(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    let at = graph_of(e, this);
    let v = graph(e, at).volume;
    e.mem.set_i32(out.addr(), v);
    S_OK
}

/// `GetEvent(code, p1, p2, 0)`: the oldest event that can be taken, else `E_ABORT`. A timeout
/// other than 0 is not provided.
fn get_event(e: &mut Engine, this: Ptr, code: Ptr, p1: Ptr, p2: Ptr, timeout: i32) -> u32 {
    assert!(
        timeout == 0,
        "IMediaEvent::GetEvent with a timeout of {timeout} ms is not provided"
    );
    let at = graph_of(e, this);
    let _ = position_now(e, at);
    let now = e.clock_now_us();
    let g = graph(e, at);
    match g.events.front() {
        Some(&(c, a, b, after)) if now >= after => {
            g.events.pop_front();
            e.mem.set_u32(code.addr(), c);
            e.mem.set_u32(p1.addr(), a);
            e.mem.set_u32(p2.addr(), b);
            S_OK
        }
        _ => E_ABORT,
    }
}

/// `FreeEventParams(code, p1, p2)`: this layer's events carry nothing to free.
fn free_event_params(_e: &mut Engine, _this: Ptr, _code: u32, _p1: u32, _p2: u32) -> u32 {
    S_OK
}

fn pin(e: &mut Engine, this: Ptr) -> &mut Pin {
    e.kernel
        .sound
        .show
        .pins
        .get_mut(&this.addr())
        .expect("this layer's input pin")
}

fn pin_query_interface(e: &mut Engine, this: Ptr, interface: Ptr, out: Ptr) -> u32 {
    let wanted = e.mem.bytes(interface.addr(), 16);
    if wanted == iid("IUnknown") || wanted == iid("IPin") {
        pin(e, this).references += 1;
        e.mem.set_u32(out.addr(), this.addr());
        return S_OK;
    }
    e.mem.set_u32(out.addr(), 0);
    E_NOINTERFACE
}

fn pin_add_ref(e: &mut Engine, this: Ptr) -> u32 {
    let p = pin(e, this);
    p.references += 1;
    p.references
}

fn pin_release(e: &mut Engine, this: Ptr) -> u32 {
    let p = pin(e, this);
    p.references -= 1;
    let left = p.references;
    if left == 0 {
        e.kernel.sound.show.pins.remove(&this.addr());
        e.mem.free(this.addr());
    }
    left
}

/// `ReceiveConnection(connector, type)`: the stream type the game's pin offers
/// (`MEDIATYPE_Stream`, `MEDIASUBTYPE_MPEG1Audio`) is taken, as the splitter takes it: the
/// connector kept with a reference and asked for its `IAsyncReader` (which the game's pin
/// requires before it completes the connection). Another type is `VFW_E_TYPE_NOT_ACCEPTED`.
fn receive_connection(e: &mut Engine, this: Ptr, connector: Ptr, media_type: Ptr) -> u32 {
    let t = media_type.addr();
    if e.mem.bytes(t, 16) != iid("MEDIATYPE_Stream")
        || e.mem.bytes(t + 0x10, 16) != iid("MEDIASUBTYPE_MPEG1Audio")
    {
        return VFW_E_TYPE_NOT_ACCEPTED;
    }
    e.vcall(connector.addr(), 4, &[]);
    let id = e.mem.alloc(16 + 4);
    e.mem.write(id, &iid("IAsyncReader"));
    let r = e.vcall(connector.addr(), 0, &[id, id + 16]).u32();
    let reader = e.mem.u32(id + 16);
    e.mem.free(id);
    assert!(
        r == S_OK,
        "the game's pin has no IAsyncReader ({r:#x}): not provided"
    );
    let p = pin(e, this);
    p.connected = connector.addr();
    p.reader = reader;
    S_OK
}

/// `Disconnect()`: lets go of the connector and its reader.
fn pin_disconnect(e: &mut Engine, this: Ptr) -> u32 {
    let (connected, reader) = {
        let p = pin(e, this);
        (
            std::mem::take(&mut p.connected),
            std::mem::take(&mut p.reader),
        )
    };
    if reader != 0 {
        e.vcall(reader, 8, &[]);
    }
    if connected != 0 {
        e.vcall(connected, 8, &[]);
        S_OK
    } else {
        S_FALSE
    }
}

/// `QueryDirection(out)`: `PINDIR_INPUT` (0).
fn query_direction(e: &mut Engine, _this: Ptr, out: Ptr) -> u32 {
    e.mem.set_u32(out.addr(), 0);
    S_OK
}

/// `EnumMediaTypes(out)`: this pin proposes no types (the connecting pin's own are tried).
fn enum_media_types(_e: &mut Engine, _this: Ptr, _out: Ptr) -> u32 {
    E_NOTIMPL
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A stand-in for the game's filter and pin (the real ones are game code in the engine
    /// crates): `JoinFilterGraph` recorded; the pin's `Connect` calls `ReceiveConnection` with
    /// the stream type, as the game's `CBasePin::Connect` ends up doing; `IAsyncReader` over an
    /// MP3 made of silent frames.
    struct Fake {
        filter: u32,
        pin: u32,
        reader: u32,
    }

    /// One MPEG-1 Layer III frame header, 128 kbit/s, 44100 Hz, mono, then zeros (a silent
    /// frame: no side information, no data).
    fn mp3(frames: usize) -> Vec<u8> {
        let mut v = Vec::new();
        for _ in 0..frames {
            let mut f = vec![0u8; 417];
            f[..4].copy_from_slice(&[0xff, 0xfb, 0x90, 0xc0]);
            v.extend_from_slice(&f);
        }
        v
    }

    fn fake(e: &mut Engine, data: Vec<u8>) -> Fake {
        let base = 0x7f10_0000;
        let vt = |e: &mut Engine, code: u32| {
            let t = e.mem.alloc(4 * 16);
            for s in 0..16 {
                e.mem.set_u32(t + 4 * s, code + 4 * s);
            }
            t
        };
        let filter = e.mem.alloc(4);
        let fv = vt(e, base);
        e.mem.set_u32(filter, fv);
        e.register(base + 4, |_, _| 2u32.into_ret_());
        e.register(base + 8, |_, _| 1u32.into_ret_());
        e.register(base + 0x34, |e, a| {
            e.mem.set_u32(0x7f20_0000, a[1]);
            e.mem.set_u32(0x7f20_0004, a[2]);
            0u32.into_ret_()
        });
        let pin = e.mem.alloc(4);
        let pv = vt(e, base + 0x100);
        e.mem.set_u32(pin, pv);
        let reader = e.mem.alloc(4);
        let rv = vt(e, base + 0x200);
        e.mem.set_u32(reader, rv);
        e.mem.map(0x7f20_0000, 0x1000);
        e.mem.set_u32(0x7f20_0010, reader);
        e.mem.set_u32(0x7f20_0014, pin);
        e.register(base + 0x100 + 4, |_, _| 2u32.into_ret_());
        e.register(base + 0x100 + 8, |_, _| 1u32.into_ret_());
        e.register(base + 0x100, |e, a| {
            // QueryInterface(IID_IAsyncReader)
            let reader = e.mem.u32(0x7f20_0010);
            e.mem.set_u32(a[2], reader);
            0u32.into_ret_()
        });
        e.register(base + 0x100 + 0xc, |e, a| {
            // Connect(receiver, NULL): QueryDirection then ReceiveConnection with the type.
            let (receiver, own) = (a[1], e.mem.u32(0x7f20_0014));
            let t = e.mem.alloc(0x48);
            e.mem.write(t, &iid("MEDIATYPE_Stream"));
            e.mem.write(t + 0x10, &iid("MEDIASUBTYPE_MPEG1Audio"));
            let d = e.mem.alloc(4);
            e.vcall(receiver, 0x24, &[d]);
            assert_eq!(e.mem.u32(d), 0);
            e.vcall(receiver, 0x10, &[own, t])
        });
        e.register(base + 0x100 + 0x14, |_, _| 0u32.into_ret_());
        e.register(base + 0x200 + 8, |_, _| 1u32.into_ret_());
        let size = data.len() as u64;
        e.register_double(base + 0x200 + 0x20, move |e, a| {
            e.mem.set_u64(a[1], size);
            e.mem.set_u64(a[2], size);
            0u32.into_ret_()
        });
        e.register_double(base + 0x200 + 0x1c, move |e, a| {
            assert_eq!((a[1], a[2], a[3] as usize), (0, 0, data.len()));
            e.mem.write(a[4], &data);
            0u32.into_ret_()
        });
        Fake {
            filter,
            pin,
            reader,
        }
    }

    trait IntoRet {
        fn into_ret_(self) -> crate::Ret;
    }
    impl IntoRet for u32 {
        fn into_ret_(self) -> crate::Ret {
            crate::abi::RetVal::into_ret(self)
        }
    }

    /// The music thread's start (`00830b30`): the graph, the filter added, its pin rendered, the
    /// interfaces asked for. Answers the interfaces (graph, control, position, audio, events).
    fn start(e: &mut Engine, f: &Fake) -> [u32; 5] {
        let ids = e.mem.alloc(16 * 6 + 4 * 6);
        let names = [
            "IFilterGraph",
            "IGraphBuilder",
            "IBasicAudio",
            "IMediaControl",
            "IMediaPosition",
            "IMediaEvent",
        ];
        for (i, n) in names.iter().enumerate() {
            e.mem.write(ids + 16 * i as u32, &iid(n));
        }
        let outs = ids + 16 * 6;
        let clsid = e.mem.alloc(16);
        e.mem.write(clsid, &clsid_filter_graph());
        assert_eq!(
            co_create_instance(e, Ptr::new(clsid), 0, 1, Ptr::new(ids), Ptr::new(outs)),
            S_OK
        );
        let fg = e.mem.u32(outs);
        assert_eq!(e.vcall(fg, 0xc, &[f.filter, 0]).u32(), S_OK);
        assert_eq!(e.mem.u32(0x7f20_0000), fg, "JoinFilterGraph got the graph");
        assert_eq!(e.mem.u16(e.mem.u32(0x7f20_0004)), b'0' as u16);
        assert_eq!(e.vcall(fg, 0, &[ids + 16, outs + 4]).u32(), S_OK);
        let gb = e.mem.u32(outs + 4);
        assert_eq!(e.vcall(gb, 0x30, &[f.pin]).u32(), S_OK);
        let mut got = [fg, 0, 0, 0, 0];
        for (k, i) in [(3usize, 3u32), (2, 2), (4, 4), (5, 5)] {
            assert_eq!(e.vcall(gb, 0, &[ids + 16 * i, outs + 4 * i]).u32(), S_OK);
            got[k - 1] = e.mem.u32(outs + 4 * i);
        }
        let _ = f.reader;
        [fg, got[2], got[3], got[1], got[4]]
    }

    fn position(e: &mut Engine, p: u32) -> f64 {
        let out = e.mem.alloc(8);
        e.vcall(p, 0x24, &[out]);
        e.mem.f64(out)
    }

    /// All of a feed's samples (waiting for its decoder).
    fn samples_of(feed: &Feed) -> Vec<i16> {
        let mut bytes = vec![0; feed.frames() * feed.frame_bytes()];
        let mut at = 0;
        while at < feed.frames() {
            let got = feed.take(at, &mut bytes, usize::MAX);
            assert!(!got.is_empty(), "the decoder stopped at frame {at}");
            at = got.end;
        }
        bytes
            .chunks(2)
            .map(|b| i16::from_le_bytes([b[0], b[1]]))
            .collect()
    }

    /// The samples Windows' graph gives for a stream ([`Decoding`]), the whole stream decoded
    /// in order, as `Render` did before it decoded beside the game.
    fn windows_samples(decoder: &mut ::mp3::Decoder<&[u8]>) -> Vec<i16> {
        decoder.set_gapless(false);
        let channels = usize::from(decoder.channels());
        let mut all: Vec<f32> = Vec::new();
        if decoder.info().xing.is_some() {
            all.resize(decoder.info().samples_per_frame * channels, 0.0);
        }
        while let Some(frame) = decoder.next_frame() {
            all.extend_from_slice(&frame.samples);
        }
        let skip = (DELAY * channels).min(all.len());
        all[skip..]
            .iter()
            .map(|&x| ((x * 32768.0).trunc() as i32).clamp(-32768, 32767) as i16)
            .collect()
    }

    /// A synthetic MP3 stream (no game data): an Info frame, then `frames` MPEG-1 Layer III
    /// frames of 192 kbit/s, 48000 Hz stereo whose granules switch blocks in each channel (long,
    /// start, short, some of them mixed, stop) and whose data start up to 511 bytes back in the
    /// frames before (the bit reservoir). The spectra are quadruples of -1, 0 and 1 (table B)
    /// with no scale factors.
    fn switching_mp3(frames: usize) -> Vec<u8> {
        struct Bits(Vec<u8>, usize);
        impl Bits {
            fn put(&mut self, v: u32, n: usize) {
                for i in (0..n).rev() {
                    if self.1 % 8 == 0 {
                        self.0.push(0);
                    }
                    if (v >> i) & 1 == 1 {
                        *self.0.last_mut().expect("a byte") |= 0x80 >> (self.1 % 8);
                    }
                    self.1 += 1;
                }
            }
        }
        let mut x = 0x2545_f491u32;
        let mut rng = move |n: u32| {
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            x % n
        };
        // MPEG-1 Layer III, no checksum, 192 kbit/s, 48000 Hz, stereo: 576 bytes a frame, 32 of
        // side information, so 540 of each frame's own data.
        const HEADER: [u8; 4] = [0xff, 0xfb, 0xb4, 0x04];
        const SIZE: usize = 576;
        const OWN: usize = SIZE - 4 - 32;
        let mut info = HEADER.to_vec();
        info.resize(36, 0);
        info.extend(b"Info");
        info.resize(SIZE, 0);
        // All frames' own data one after another, and where the data so far ends in it.
        let mut run = vec![0u8; frames * OWN];
        let mut end = 0;
        let mut sides = Vec::new();
        let mut block = [0u32; 2];
        let mut kinds = std::collections::BTreeSet::new();
        for i in 0..frames {
            let begin = end.max((i * OWN).saturating_sub(511));
            let room = 8 * ((i + 1) * OWN - begin);
            let mut main = Bits(Vec::new(), 0);
            let mut granules = Vec::new();
            for _ in 0..4 {
                let ch = granules.len() % 2;
                block[ch] = match block[ch] {
                    0 | 3 => u32::from(rng(3) == 0),
                    1 => 2,
                    _ => 2 + rng(2),
                };
                let mixed = block[ch] == 2 && rng(3) == 0;
                kinds.insert((block[ch], mixed));
                let from = main.1;
                for _ in 0..rng(145) {
                    let v = rng(16);
                    if main.1 + 8 > room {
                        break;
                    }
                    main.put(v ^ 15, 4);
                    for _ in 0..v.count_ones() {
                        main.put(rng(2), 1);
                    }
                }
                granules.push((main.1 - from, 160 + rng(30), block[ch], mixed));
            }
            run[begin..begin + main.0.len()].copy_from_slice(&main.0);
            end = begin + main.0.len();
            let mut side = Bits(Vec::new(), 0);
            // main_data_begin, private bits, scfsi.
            side.put((i * OWN - begin) as u32, 9);
            side.put(0, 3 + 8);
            for (bits, gain, block_type, mixed) in granules {
                // part2_3_length, big_values, global_gain, scalefac_compress.
                side.put(bits as u32, 12);
                side.put(0, 9);
                side.put(gain, 8);
                side.put(0, 4);
                if block_type == 0 {
                    side.put(0, 1 + 15 + 4 + 3);
                } else {
                    // Window switching, the block type, mixed, two tables, subblock gains.
                    side.put(1, 1);
                    side.put(block_type, 2);
                    side.put(u32::from(mixed), 1);
                    side.put(0, 10);
                    side.put(rng(512), 9);
                }
                // preflag, scalefac_scale, count1table_select (table B).
                side.put(1, 3);
            }
            assert_eq!(side.1, 32 * 8);
            sides.push(side.0);
        }
        assert_eq!(
            kinds.into_iter().collect::<Vec<_>>(),
            [(0, false), (1, false), (2, false), (2, true), (3, false)],
            "every block type and mixed blocks"
        );
        let mut data = info;
        for (i, side) in sides.iter().enumerate() {
            data.extend(HEADER);
            data.extend(side);
            data.extend(&run[i * OWN..(i + 1) * OWN]);
        }
        data
    }

    fn events(e: &mut Engine, ev: u32) -> Vec<u32> {
        let out = e.mem.alloc(12);
        let mut got = Vec::new();
        while e.vcall(ev, 0x20, &[out, out + 4, out + 8, 0]).u32() == S_OK {
            got.push(e.mem.u32(out));
            e.vcall(ev, 0x30, &[e.mem.u32(out), 0, 0]);
        }
        got
    }

    #[test]
    fn the_music_thread_s_calls_are_answered_as_measured() {
        let mut e = Engine::new();
        let data = mp3(100);
        let size = data.len() as u64;
        let f = fake(&mut e, data);
        let [fg, control, pos, audio, ev] = start(&mut e, &f);
        // The samples, once decoded: 100 frames of 1152 less the decoder's 528, all silent.
        let samples = samples_of(&e.kernel.sound.show.graphs[&fg].samples);
        assert_eq!(samples.len(), 100 * 1152 - 528);
        assert!(samples.iter().all(|&s| s == 0));
        // Duration: size * 8 * 10^7 / 128000 in 100 ns.
        let out = e.mem.alloc(8);
        e.vcall(pos, 0x1c, &[out]);
        assert_eq!(
            e.mem.f64(out),
            (size * 8 * 10_000_000 / 128_000) as f64 / 1e7
        );
        assert_eq!(
            e.vcall(audio, 0x1c, &[(-10001i32) as u32]).u32(),
            E_INVALIDARG
        );
        assert_eq!(e.vcall(audio, 0x1c, &[(-2000i32) as u32]).u32(), S_OK);
        // The first Run: S_FALSE, EC_CLOCK_CHANGED at once, EC_PAUSED a moment later.
        assert_eq!(e.vcall(control, 0x1c, &[]).u32(), S_FALSE);
        assert_eq!(events(&mut e, ev), [EC_CLOCK_CHANGED]);
        e.clock_ms += 100;
        assert_eq!(events(&mut e, ev), [EC_PAUSED]);
        assert_eq!(e.vcall(control, 0x1c, &[]).u32(), S_OK);
        assert!((position(&mut e, pos) - 0.1).abs() < 1e-9);
        // Pause: EC_PAUSED each time; the position stays.
        assert_eq!(e.vcall(control, 0x20, &[]).u32(), S_OK);
        e.clock_ms += 500;
        assert_eq!(e.vcall(control, 0x20, &[]).u32(), S_OK);
        assert_eq!(events(&mut e, ev), [EC_PAUSED, EC_PAUSED]);
        assert!((position(&mut e, pos) - 0.1).abs() < 1e-9);
        // Run on to the end: EC_COMPLETE, the position the duration.
        assert_eq!(e.vcall(control, 0x1c, &[]).u32(), S_OK);
        e.clock_ms += 10_000;
        assert_eq!(events(&mut e, ev), [EC_COMPLETE]);
        // The loop restart: Stop, put_CurrentPosition(0), Run.
        assert_eq!(e.vcall(control, 0x24, &[]).u32(), S_OK);
        let zero = 0.0f64.to_bits();
        assert_eq!(
            e.vcall(pos, 0x20, &[zero as u32, (zero >> 32) as u32])
                .u32(),
            S_OK
        );
        assert_eq!(position(&mut e, pos), 0.0);
        let minus = (-1.0f64).to_bits();
        assert_eq!(
            e.vcall(pos, 0x20, &[minus as u32, (minus >> 32) as u32])
                .u32(),
            E_INVALIDARG
        );
        assert_eq!(e.vcall(control, 0x1c, &[]).u32(), S_FALSE);
        assert!(
            events(&mut e, ev).is_empty(),
            "EC_PAUSED comes later; no second EC_CLOCK_CHANGED"
        );
        // Released as the game does: everything, then the filter is let go.
        for i in [ev, pos, control, audio] {
            e.vcall(i, 8, &[]);
        }
        e.vcall(fg, 8, &[]);
        assert_eq!(e.vcall(fg, 8, &[]).u32(), 0);
        assert_eq!(e.mem.u32(0x7f20_0000), 0, "JoinFilterGraph(NULL)");
    }

    /// A control call (`Run` 0x1c, `Pause` 0x20, `Stop` 0x24) and `put_CurrentPosition`.
    fn call(e: &mut Engine, control: u32, slot: u32) -> u32 {
        e.vcall(control, slot, &[]).u32()
    }
    fn put(e: &mut Engine, pos: u32, seconds: f64) -> u32 {
        let b = seconds.to_bits();
        e.vcall(pos, 0x20, &[b as u32, (b >> 32) as u32]).u32()
    }
    const RUN: u32 = 0x1c;
    const PAUSE: u32 = 0x20;
    const STOP: u32 = 0x24;

    /// The music thread's first round with its pause flags (`0083193b`) pauses a graph that
    /// never ran: `S_FALSE`, `EC_CLOCK_CHANGED` at once, `EC_PAUSED` 10 ms later, the position
    /// where it was; then as a paused graph.
    #[test]
    fn a_pause_before_any_run_is_answered_as_measured() {
        let mut e = Engine::new();
        let f = fake(&mut e, mp3(100));
        let [_, control, pos, _, ev] = start(&mut e, &f);
        assert_eq!(call(&mut e, control, PAUSE), S_FALSE);
        assert_eq!(events(&mut e, ev), [EC_CLOCK_CHANGED]);
        e.clock_ms += 9;
        assert!(events(&mut e, ev).is_empty());
        e.clock_ms += 1;
        assert_eq!(events(&mut e, ev), [EC_PAUSED]);
        assert_eq!(position(&mut e, pos), 0.0);
        // Paused again a round later: S_OK and EC_PAUSED at once; the position stays.
        e.clock_ms += 90;
        assert_eq!(call(&mut e, control, PAUSE), S_OK);
        assert_eq!(events(&mut e, ev), [EC_PAUSED]);
        assert_eq!(position(&mut e, pos), 0.0);
        // Run from there: S_OK, nothing queued; the position moves.
        assert_eq!(call(&mut e, control, RUN), S_OK);
        e.clock_ms += 100;
        assert!(events(&mut e, ev).is_empty());
        assert!((position(&mut e, pos) - 0.1).abs() < 1e-9);
        // Stopped and run again: S_FALSE, the clock was chosen at the pause.
        assert_eq!(call(&mut e, control, STOP), S_OK);
        assert_eq!(call(&mut e, control, RUN), S_FALSE);
        assert!(events(&mut e, ev).is_empty());
        e.clock_ms += 10;
        assert_eq!(events(&mut e, ev), [EC_PAUSED]);
        assert!((position(&mut e, pos) - 0.11).abs() < 1e-9);
    }

    /// Rounds can come at once (the main loop sets the thread's event every frame, and on
    /// Windows `Run` and `Pause` from stopped take 11 to 18 ms): calls while the graph leaves
    /// stopped, each as a measured scenario.
    #[test]
    fn calls_while_a_graph_leaves_stopped_are_answered_as_measured() {
        let mut e = Engine::new();
        let f = fake(&mut e, mp3(100));
        let [_, c, _, _, ev] = start(&mut e, &f);
        // Pause, Pause at once: S_FALSE twice, one EC_PAUSED.
        assert_eq!(call(&mut e, c, PAUSE), S_FALSE);
        assert_eq!(call(&mut e, c, PAUSE), S_FALSE);
        e.clock_ms += 50;
        assert_eq!(events(&mut e, ev), [EC_CLOCK_CHANGED, EC_PAUSED]);
        // Pause, Run at once: S_FALSE twice, the pause's EC_PAUSED.
        assert_eq!(call(&mut e, c, STOP), S_OK);
        assert_eq!(call(&mut e, c, PAUSE), S_FALSE);
        assert_eq!(call(&mut e, c, RUN), S_FALSE);
        assert!(events(&mut e, ev).is_empty());
        e.clock_ms += 10;
        assert_eq!(events(&mut e, ev), [EC_PAUSED]);
        assert_eq!(call(&mut e, c, RUN), S_OK);
        // Run, Run at once: S_FALSE twice, one EC_PAUSED.
        assert_eq!(call(&mut e, c, STOP), S_OK);
        assert_eq!(call(&mut e, c, RUN), S_FALSE);
        assert_eq!(call(&mut e, c, RUN), S_FALSE);
        e.clock_ms += 10;
        assert_eq!(events(&mut e, ev), [EC_PAUSED]);
        // Run, Pause at once: S_FALSE twice, one EC_PAUSED.
        assert_eq!(call(&mut e, c, STOP), S_OK);
        assert_eq!(call(&mut e, c, RUN), S_FALSE);
        assert_eq!(call(&mut e, c, PAUSE), S_FALSE);
        e.clock_ms += 10;
        assert_eq!(events(&mut e, ev), [EC_PAUSED]);
        // Settled, before the EC_PAUSED: a Run answers S_OK, a Pause S_OK and EC_PAUSED at once;
        // the EC_PAUSED to come never comes (Pause then Run or Pause 2 ms later; Run then Run
        // or Pause 5 ms later; Pause and Run at once, then Pause 5 ms later).
        let settled: [(&[u32], u32, u32, &[u32]); 5] = [
            (&[PAUSE], 2, RUN, &[]),
            (&[PAUSE], 2, PAUSE, &[EC_PAUSED]),
            (&[RUN], 5, RUN, &[]),
            (&[RUN], 5, PAUSE, &[EC_PAUSED]),
            (&[PAUSE, RUN], 5, PAUSE, &[EC_PAUSED]),
        ];
        for (first, ms, then, queued) in settled {
            assert_eq!(call(&mut e, c, STOP), S_OK);
            for &slot in first {
                assert_eq!(call(&mut e, c, slot), S_FALSE);
            }
            e.clock_ms += ms;
            assert_eq!(call(&mut e, c, then), S_OK);
            assert_eq!(events(&mut e, ev), queued);
            e.clock_ms += 100;
            assert!(events(&mut e, ev).is_empty());
        }
        // Pause or Run from stopped, then Stop at once or a moment later: no EC_PAUSED comes.
        for (first, ms) in [(PAUSE, 0), (PAUSE, 2), (RUN, 0), (RUN, 5)] {
            assert_eq!(call(&mut e, c, STOP), S_OK);
            assert_eq!(call(&mut e, c, first), S_FALSE);
            e.clock_ms += ms;
            assert_eq!(call(&mut e, c, STOP), S_OK);
            e.clock_ms += 100;
            assert!(events(&mut e, ev).is_empty());
        }
    }

    /// `Stop` and `put_CurrentPosition` before any run; `Run` on a graph paused at the end.
    #[test]
    fn stop_and_seek_before_any_run_and_a_run_at_the_end() {
        let mut e = Engine::new();
        let f = fake(&mut e, mp3(100));
        let [_, c, pos, _, ev] = start(&mut e, &f);
        // Stop on a new graph: S_OK, nothing; the clock is chosen at the first Run after it.
        assert_eq!(call(&mut e, c, STOP), S_OK);
        assert!(events(&mut e, ev).is_empty());
        assert_eq!(call(&mut e, c, RUN), S_FALSE);
        assert_eq!(events(&mut e, ev), [EC_CLOCK_CHANGED]);
        e.clock_ms += 10;
        assert_eq!(events(&mut e, ev), [EC_PAUSED]);
        // Stopped, put, paused: the position where it was put, through the pause.
        assert_eq!(call(&mut e, c, STOP), S_OK);
        assert_eq!(put(&mut e, pos, 1.0), S_OK);
        assert_eq!(position(&mut e, pos), 1.0);
        assert_eq!(call(&mut e, c, PAUSE), S_FALSE);
        e.clock_ms += 10;
        assert_eq!(events(&mut e, ev), [EC_PAUSED]);
        assert_eq!(position(&mut e, pos), 1.0);
        // Put while paused: S_OK, the graph stays paused.
        assert_eq!(put(&mut e, pos, 2.0), S_OK);
        e.clock_ms += 100;
        assert_eq!(position(&mut e, pos), 2.0);
        assert!(events(&mut e, ev).is_empty());
        // Paused at the end and run: S_OK and EC_COMPLETE at once.
        assert_eq!(put(&mut e, pos, 100.0), S_OK);
        assert_eq!(call(&mut e, c, RUN), S_OK);
        assert_eq!(events(&mut e, ev), [EC_COMPLETE]);
    }

    /// `Render` (the music thread's call at `008310d2`..`008310d5`) answers with the stream
    /// read and its format known, without its samples: here its decoder is held at its start
    /// all along, and the format and duration are there, nothing decoded. Let go, the decoder
    /// stops.
    #[test]
    fn render_answers_before_the_stream_is_decoded() {
        let mut e = Engine::new();
        e.kernel.sound.show.hold = true;
        let data = mp3(2000);
        let size = data.len() as u64;
        let f = fake(&mut e, data);
        let [fg, control, pos, audio, ev] = start(&mut e, &f);
        let g = &e.kernel.sound.show.graphs[&fg];
        assert_eq!(
            (g.duration as u64, g.sample_rate, g.channels),
            (size * 8 * 10_000_000 / 128_000, 44100, 1)
        );
        let samples = g.samples.clone();
        assert_eq!(samples.frames(), 2000 * 1152 - 528);
        assert!(samples.runs().is_empty(), "nothing decoded");
        // Released as the game does: the decoder stops without having decoded anything.
        let decode = e
            .kernel
            .sound
            .show
            .graphs
            .get_mut(&fg)
            .and_then(|g| g.decode.take())
            .expect("a decoder");
        for i in [ev, pos, control, audio] {
            e.vcall(i, 8, &[]);
        }
        e.vcall(fg, 8, &[]);
        assert_eq!(e.vcall(fg, 8, &[]).u32(), 0);
        decode.join().expect("the decoder stopped");
        assert!(samples.runs().is_empty());
    }

    /// The samples decoded from wherever the stream is played first are Windows' samples bit
    /// for bit (the whole stream decoded in order): the MP3 frame holding the place asked for
    /// is decoded next, from a few frames before it, then the rest, each frame once. On a
    /// synthetic stream with block switching and the bit reservoir: places in the Info frame's
    /// silence, at and inside MP3 frames (with the decoder's 528 frames dropped at the start)
    /// and at the end, asked for at once or after a few frames decoded from the start.
    #[test]
    fn the_samples_are_the_same_from_wherever_the_stream_is_played() {
        let data = switching_mp3(24);
        let whole = windows_samples(&mut ::mp3::Decoder::new(&data[..]).expect("an MP3"));
        let frames = 25 * 1152 - DELAY;
        assert_eq!(whole.len(), 2 * frames);
        assert!(whole.iter().filter(|&&s| s != 0).count() > frames, "sound");
        let targets = [
            0,
            300,
            1152 - DELAY,
            1152 - DELAY + 1,
            2 * 1152 - DELAY - 1,
            7 * 1152 + 77,
            13 * 1152 - DELAY,
            20 * 1152 + 1000,
            frames - 1,
        ];
        for target in targets {
            for first in [0, 3] {
                let decoder = ::mp3::Decoder::new(data.clone()).expect("an MP3");
                let mut decoding = Decoding::new(decoder);
                let feed = Feed::new(decoding.frames(), 4);
                for _ in 0..first {
                    assert!(decoding.step(&feed));
                }
                feed.want(target);
                assert!(decoding.step(&feed));
                assert!(feed.decoded(target), "{target} after {first}: decoded next");
                let mut steps = first + 1;
                while decoding.step(&feed) {
                    steps += 1;
                }
                assert_eq!(steps, 25, "every MP3 frame decoded once");
                assert!(samples_of(&feed) == whole, "{target} after {first}");
            }
        }
    }

    /// The music thread's synchronised layer (`00830b30`: `put_CurrentPosition` into its deck
    /// right after `Render`, at `008318af`, then `Run` at `00831960`), heard with the output on:
    /// from that place at once with the stream's own samples, bit for bit as a voice of the
    /// whole decoded stream, though nothing was decoded when `Run` came (the decoder held until
    /// then); and the stream ends at its end.
    #[test]
    fn a_layer_run_far_into_its_stream_is_heard_from_there_at_once() {
        let mut e = Engine::new();
        super::super::dsound::hear_by_hand(&mut e, 48000);
        e.kernel.sound.show.hold = true;
        let data = switching_mp3(24);
        let whole = windows_samples(&mut ::mp3::Decoder::new(&data[..]).expect("an MP3"));
        let f = fake(&mut e, data);
        let [fg, control, pos, audio, ev] = start(&mut e, &f);
        // 0.4 s: frame 19200, in the stream's 17th MP3 frame.
        let at = 0.4f64.to_bits();
        let r = e.vcall(pos, 0x20, &[at as u32, (at >> 32) as u32]).u32();
        assert_eq!(r, S_OK);
        assert_eq!(e.vcall(control, 0x1c, &[]).u32(), S_FALSE);
        let samples = e.kernel.sound.show.graphs[&fg].samples.clone();
        assert!(samples.runs().is_empty(), "nothing decoded at Run");
        samples.start();
        let mut plain = super::super::mixer::Voice::new(
            whole.iter().flat_map(|s| s.to_le_bytes()).collect(),
            2,
            16,
        );
        (plain.rate, plain.device) = (48000, 48000);
        plain.start(19200);
        let mut heard = Vec::new();
        let mut want = Vec::new();
        for _ in 0..3 {
            heard.extend(super::super::dsound::mixed(&e, 4000));
            let mut out = vec![0f32; 2 * 4000];
            super::super::mixer::render(&mut [&mut plain], &mut out);
            want.extend(out);
        }
        let bits = |v: &[f32]| v.iter().map(|s| s.to_bits()).collect::<Vec<_>>();
        assert!(bits(&heard) == bits(&want));
        assert!(want[..2 * 480].iter().any(|&s| s != 0.0), "heard at once");
        let decode = e
            .kernel
            .sound
            .show
            .graphs
            .get_mut(&fg)
            .and_then(|g| g.decode.take())
            .expect("a decoder");
        for i in [ev, pos, control, audio] {
            e.vcall(i, 8, &[]);
        }
        e.vcall(fg, 8, &[]);
        assert_eq!(e.vcall(fg, 8, &[]).u32(), 0);
        decode.join().expect("the decoder ended");
    }

    /// With `NV_MUSIC_MP3` naming an MP3 (one of the game's tracks, read only), the same as
    /// [`the_samples_are_the_same_from_wherever_the_stream_is_played`] on it from 12 places
    /// through it: `cargo test -p engine-core --lib -- --ignored nv_music_mp3`.
    #[test]
    #[ignore]
    fn nv_music_mp3_is_the_same_from_wherever_it_is_played() {
        let Ok(path) = std::env::var("NV_MUSIC_MP3") else {
            return;
        };
        let data = std::fs::read(&path).expect("NV_MUSIC_MP3 readable");
        let whole = windows_samples(&mut ::mp3::Decoder::new(&data[..]).expect("an MP3"));
        let decoder = ::mp3::Decoder::new(data.clone()).expect("an MP3");
        let channels = usize::from(decoder.channels());
        let frames = whole.len() / channels;
        for k in 0..12 {
            let target = frames * k / 12 + 1009 * k;
            let target = target.min(frames - 1);
            let mut decoding = Decoding::new(::mp3::Decoder::new(data.clone()).expect("an MP3"));
            assert_eq!(decoding.frames(), frames);
            let feed = Feed::new(frames, 2 * channels);
            feed.want(target);
            let started = std::time::Instant::now();
            assert!(decoding.step(&feed));
            let first = started.elapsed();
            assert!(feed.decoded(target));
            while decoding.step(&feed) {}
            assert!(samples_of(&feed) == whole, "{path}: from frame {target}");
            eprintln!("{path}: from frame {target}: the first frame in {first:?}");
        }
    }

    #[test]
    #[should_panic(expected = "is not provided")]
    fn another_class_stops() {
        let mut e = Engine::new();
        let ids = e.mem.alloc(32);
        co_create_instance(
            &mut e,
            Ptr::new(ids),
            0,
            1,
            Ptr::new(ids + 16),
            Ptr::new(ids + 16),
        );
    }
}
