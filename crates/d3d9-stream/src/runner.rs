//! The runner: where the backend carries the commands out. [`Stream::direct`] on the calling
//! thread, at once; [`Stream::threaded`] on a render thread of its own, at most a frame behind.

use crate::chunk::Chunk;
use crate::cmd::{Backend, Cmd, Data, Reply};
use crate::wire;
use std::collections::VecDeque;
use std::io::Write;
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};

/// How the threaded runner hands commands over.
#[derive(Debug, Clone)]
pub struct Settings {
    /// Presents the render thread may still have to carry out when `Present` returns: 1 (the
    /// default) lets the game start its next frame while the render thread finishes this one; 0
    /// returns only once this Present is carried out (`NV_RENDER_THREAD_FRAMES`).
    pub frames_in_flight: u64,
    /// A chunk is handed over at this many commands (`NV_RENDER_THREAD_CHUNK`) ...
    pub chunk_commands: usize,
    /// ... or this many bytes, at every Present and before every sync command.
    pub chunk_bytes: usize,
    /// Past this many bytes or chunks waiting, the recording thread waits for the render thread
    /// to carry out half of them (a loading burst of uploads stays bounded).
    pub max_queued_bytes: usize,
    pub max_queued_chunks: usize,
    /// `NV_D3D9_STREAM_CHECK=1`: the commands are hashed as recorded and as carried out, and a
    /// difference stops the run.
    pub check: bool,
    /// `NV_RENDER_THREAD_STATS=1`: a line a second on the hand-overs and the waits.
    pub stats: bool,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            frames_in_flight: 1,
            chunk_commands: 1024,
            chunk_bytes: 256 << 10,
            max_queued_bytes: 64 << 20,
            max_queued_chunks: 64,
            check: false,
            stats: false,
        }
    }
}

impl Settings {
    /// The defaults, with the settings the environment gives.
    pub fn from_env() -> Settings {
        let number = |name: &str| std::env::var(name).ok()?.trim().parse::<u64>().ok();
        let mut s = Settings::default();
        if let Some(n) = number("NV_RENDER_THREAD_FRAMES") {
            s.frames_in_flight = n;
        }
        if let Some(n) = number("NV_RENDER_THREAD_CHUNK") {
            s.chunk_commands = (n as usize).max(1);
        }
        s.check = on("NV_D3D9_STREAM_CHECK");
        s.stats = on("NV_RENDER_THREAD_STATS");
        s
    }
}

fn on(name: &str) -> bool {
    std::env::var(name).is_ok_and(|v| v.trim() == "1")
}

/// The commands' way to a backend.
pub struct Stream {
    runner: Runner,
    presents: u32,
    record: Option<Recording>,
}

// One stream a run, made once and never moved on the way of a call: kept unboxed, so each
// command reaches the chunk without another pointer to follow.
#[allow(clippy::large_enum_variant)]
enum Runner {
    Direct(Box<dyn Backend>),
    Threaded(Threaded),
}

/// `NV_D3D9_RECORD`: every command written as [`crate::wire`] encodes it.
struct Recording {
    out: Box<dyn Write + Send>,
    buf: Vec<u8>,
    /// [`Stream::record_repeatable`]: the window handles written as 0, the query reads left out.
    repeatable: bool,
}

impl std::fmt::Debug for Stream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stream")
            .field("threaded", &self.is_threaded())
            .field("presents", &self.presents)
            .field("recording", &self.record.is_some())
            .finish()
    }
}

impl Stream {
    /// Commands carried out on the calling thread as they come: the calls the backend makes are
    /// the ones a direct call would make, in the same place.
    pub fn direct(backend: Box<dyn Backend>) -> Stream {
        Stream {
            runner: Runner::Direct(backend),
            presents: 0,
            record: None,
        }
    }

    /// Commands carried out on a render thread (named `nv-render`), which owns `backend`.
    pub fn threaded(backend: Box<dyn Backend>, settings: Settings) -> Stream {
        Stream {
            runner: Runner::Threaded(Threaded::start(backend, settings)),
            presents: 0,
            record: None,
        }
    }

    pub fn is_threaded(&self) -> bool {
        matches!(self.runner, Runner::Threaded(_))
    }

    /// The presents submitted so far (the last [`Cmd::Present`]'s `frame`).
    pub fn presents(&self) -> u32 {
        self.presents
    }

    /// From now on every command is also written to `out` ([`crate::wire`]'s file: [`wire::MAGIC`]
    /// first). A write that fails is printed and ends the recording.
    pub fn record_to(&mut self, mut out: Box<dyn Write + Send>) -> std::io::Result<()> {
        out.write_all(&wire::header())?;
        self.record = Some(Recording {
            out,
            buf: Vec::new(),
            repeatable: false,
        });
        Ok(())
    }

    /// [`Stream::record_to`] for `NV_REPEATABLE` (a test mode): what differs between two runs of
    /// the same exe with the same input is left out, so their files compare byte for byte: the
    /// real windows' handles ([`Cmd::CreateDevice`], [`Cmd::Reset`], [`Cmd::Present`]) are
    /// written as 0, and [`Cmd::QueryGetData`] (a read, which draws nothing, made as often as the
    /// GPU takes) is not written.
    pub fn record_repeatable(&mut self, out: Box<dyn Write + Send>) -> std::io::Result<()> {
        self.record_to(out)?;
        if let Some(r) = &mut self.record {
            r.repeatable = true;
        }
        Ok(())
    }

    /// Writes out what is recorded so far (`NV_REPEATABLE`'s end, before the process ends).
    pub fn finish_recording(&mut self) {
        if let Some(r) = &mut self.record {
            if let Err(error) = r.out.flush() {
                eprintln!("NV_D3D9_RECORD: the command file could not be written ({error})");
            }
        }
    }

    /// Carries out `cmd` or queues it. A sync command ([`Cmd::is_sync`]) returns its reply once
    /// every command before it is carried out; a [`Cmd::Present`] gets its `frame` number here and
    /// returns once no more than [`Settings::frames_in_flight`] presents are left to carry out;
    /// any other returns [`Reply::None`] at once.
    pub fn submit(&mut self, mut cmd: Cmd<Data<'_>>) -> Reply {
        #[cfg(debug_assertions)]
        if let Err(why) = cmd.check() {
            panic!("a malformed command: {why}");
        }
        if let Cmd::Present { frame, .. } = &mut cmd {
            self.presents = self.presents.wrapping_add(1);
            *frame = self.presents;
        }
        if let Some(r) = &mut self.record {
            r.buf.clear();
            let written = if r.repeatable {
                repeatable_form(cmd)
            } else {
                Some(cmd)
            };
            if let Some(written) = &written {
                wire::encode(written, &mut r.buf);
            }
            if let Err(error) = r.out.write_all(&r.buf) {
                eprintln!("NV_D3D9_RECORD: the command file could not be written ({error}); the recording stops");
                self.record = None;
            }
        }
        match &mut self.runner {
            Runner::Direct(backend) => backend.execute(cmd),
            Runner::Threaded(t) => t.submit(cmd),
        }
    }

    /// Counts an event of the layer above for `NV_RENDER_THREAD_STATS` (an answer it gave without
    /// waiting for the render thread, say): printed a frame, by name. Nothing on the calling thread
    /// or without the stats.
    pub fn note(&mut self, what: &'static str) {
        if let Runner::Threaded(t) = &mut self.runner {
            t.stats.note(what);
        }
    }

    /// Hands what is recorded to the render thread now, without waiting.
    pub fn flush(&mut self) {
        if let Runner::Threaded(t) = &mut self.runner {
            t.hand_over();
        }
    }

    /// The bytes and chunks handed over and not yet carried out (tests).
    #[cfg(test)]
    pub(crate) fn queued(&self) -> (usize, usize) {
        match &self.runner {
            Runner::Direct(_) => (0, 0),
            Runner::Threaded(t) => {
                let q = lock(&t.shared.queue);
                (q.queued_bytes, q.chunks.len())
            }
        }
    }

    /// Returns once every command submitted is carried out.
    pub fn finish(&mut self) {
        if let Runner::Threaded(t) = &mut self.runner {
            t.hand_over();
            drop(t.wait(|q| q.done == q.handed));
        }
    }
}

/// What [`Stream::record_repeatable`] writes for `cmd`: the window handles as 0, a query read not
/// at all.
fn repeatable_form(cmd: Cmd<Data<'_>>) -> Option<Cmd<Data<'_>>> {
    Some(match cmd {
        Cmd::CreateDevice {
            device,
            behavior,
            present,
            back_buffers,
            depth_stencil,
            ..
        } => Cmd::CreateDevice {
            device,
            focus: 0,
            window: 0,
            behavior,
            present,
            back_buffers,
            depth_stencil,
        },
        Cmd::Reset {
            device,
            present,
            back_buffers,
            depth_stencil,
            ..
        } => Cmd::Reset {
            device,
            window: 0,
            present,
            back_buffers,
            depth_stencil,
        },
        Cmd::Present {
            device,
            source,
            dest,
            dirty,
            model,
            frame,
            ..
        } => Cmd::Present {
            device,
            source,
            dest,
            window: 0,
            dirty,
            model,
            frame,
        },
        Cmd::QueryGetData { .. } => return None,
        other => other,
    })
}

impl Drop for Stream {
    fn drop(&mut self) {
        if let Some(r) = &mut self.record {
            let _ = r.out.flush();
        }
    }
}

/// The recording side of the threaded runner.
struct Threaded {
    shared: Arc<Shared>,
    chunk: Chunk,
    settings: Settings,
    presents: u64,
    /// `check`: the hash of every command recorded.
    hash: u64,
    encoded: Vec<u8>,
    thread: Option<std::thread::JoinHandle<()>>,
    stats: Stats,
}

/// What both sides share.
struct Shared {
    /// The render thread stopped (`Queue::dead` says why): read at every command, so the
    /// recording side stops at its next call.
    dead: std::sync::atomic::AtomicBool,
    queue: Mutex<Queue>,
    /// The render thread waits on it for chunks.
    work: Condvar,
    /// The recording thread waits on it for the render thread's progress.
    progress: Condvar,
}

#[derive(Default)]
struct Queue {
    chunks: VecDeque<Chunk>,
    /// Emptied chunks to reuse.
    free: Vec<Chunk>,
    queued_bytes: usize,
    /// Chunks handed over, and carried out.
    handed: u64,
    done: u64,
    presents_done: u64,
    /// The last sync command's reply, until taken.
    reply: Option<Reply>,
    /// The render thread's time carrying commands out.
    busy: Duration,
    closing: bool,
    /// Why the render thread stopped (its backend panicked).
    dead: Option<String>,
    renderer_sleeps: bool,
    producer_waits: bool,
}

/// Emptied chunks kept for reuse.
const FREE_CHUNKS: usize = 4;

fn lock(m: &Mutex<Queue>) -> MutexGuard<'_, Queue> {
    m.lock().unwrap_or_else(PoisonError::into_inner)
}

impl Threaded {
    fn start(backend: Box<dyn Backend>, settings: Settings) -> Threaded {
        let shared = Arc::new(Shared {
            dead: std::sync::atomic::AtomicBool::new(false),
            queue: Mutex::new(Queue::default()),
            work: Condvar::new(),
            progress: Condvar::new(),
        });
        let theirs = Arc::clone(&shared);
        let check = settings.check;
        let thread = std::thread::Builder::new()
            .name("nv-render".into())
            .spawn(move || render(&theirs, backend, check))
            .expect("the render thread starts");
        Threaded {
            shared,
            chunk: Chunk::default(),
            stats: Stats {
                on: settings.stats,
                ..Stats::default()
            },
            settings,
            presents: 0,
            hash: wire::HASH_START,
            encoded: Vec::new(),
            thread: Some(thread),
        }
    }

    fn submit(&mut self, cmd: Cmd<Data<'_>>) -> Reply {
        if self.shared.dead.load(std::sync::atomic::Ordering::Relaxed) {
            stopped(&lock(&self.shared.queue));
        }
        if self.settings.check {
            self.encoded.clear();
            wire::encode(&cmd, &mut self.encoded);
            self.hash = wire::hash(self.hash, &self.encoded);
        }
        self.stats.commands += 1;
        let sync = cmd.is_sync();
        let present = matches!(cmd, Cmd::Present { .. });
        let name = cmd.name();
        self.chunk.push(cmd);
        if sync {
            self.chunk.reply_wanted = true;
            self.hand_over();
            let t = Instant::now();
            let reply = self
                .wait(|q| q.reply.is_some())
                .reply
                .take()
                .expect("the reply waited for");
            self.stats.reply(name, t.elapsed());
            reply
        } else if present {
            self.presents += 1;
            self.hand_over();
            let need = self.presents.saturating_sub(self.settings.frames_in_flight);
            let t = Instant::now();
            drop(self.wait(|q| q.presents_done >= need));
            self.stats.present.add(t.elapsed());
            self.stats.frame(&self.shared);
            Reply::None
        } else {
            if self.chunk.len() >= self.settings.chunk_commands
                || self.chunk.bytes >= self.settings.chunk_bytes
            {
                self.hand_over();
            }
            Reply::None
        }
    }

    /// The chunk being recorded goes to the render thread (waking it if it sleeps); past the
    /// queue's limit, waits until the render thread has carried out half of it.
    fn hand_over(&mut self) {
        // A stopped render thread stops the caller here even with nothing to hand over (a caller
        // polling for a result the render thread will never give would spin otherwise).
        if self.shared.dead.load(std::sync::atomic::Ordering::Relaxed) {
            stopped(&lock(&self.shared.queue));
        }
        if self.chunk.is_empty() {
            return;
        }
        if self.settings.check {
            self.chunk.check = Some(self.hash);
        }
        let (wake, full) = {
            let mut q = lock(&self.shared.queue);
            stopped(&q);
            let next = q.free.pop().unwrap_or_default();
            let chunk = std::mem::replace(&mut self.chunk, next);
            q.queued_bytes += chunk.bytes;
            q.chunks.push_back(chunk);
            q.handed += 1;
            (
                q.renderer_sleeps,
                q.queued_bytes > self.settings.max_queued_bytes
                    || q.chunks.len() > self.settings.max_queued_chunks,
            )
        };
        self.stats.chunks += 1;
        if wake {
            self.shared.work.notify_one();
        }
        if full {
            let (bytes, chunks) = (
                self.settings.max_queued_bytes / 2,
                self.settings.max_queued_chunks / 2,
            );
            let t = Instant::now();
            drop(self.wait(|q| q.queued_bytes <= bytes && q.chunks.len() <= chunks));
            self.stats.full.add(t.elapsed());
        }
    }

    /// The queue, once `ready` holds for it (panics when the render thread stopped).
    fn wait(&self, mut ready: impl FnMut(&Queue) -> bool) -> MutexGuard<'_, Queue> {
        let mut q = lock(&self.shared.queue);
        loop {
            stopped(&q);
            if ready(&q) {
                q.producer_waits = false;
                return q;
            }
            q.producer_waits = true;
            q = self
                .shared
                .progress
                .wait(q)
                .unwrap_or_else(PoisonError::into_inner);
        }
    }
}

/// Stops the recording thread with the render thread's reason when it stopped.
fn stopped(q: &Queue) {
    if let Some(why) = &q.dead {
        panic!("the render thread stopped: {why}");
    }
}

impl Drop for Threaded {
    /// What is recorded is carried out, then the render thread ends.
    fn drop(&mut self) {
        {
            let mut q = lock(&self.shared.queue);
            if q.dead.is_none() && !self.chunk.is_empty() {
                if self.settings.check {
                    self.chunk.check = Some(self.hash);
                }
                let chunk = std::mem::take(&mut self.chunk);
                q.queued_bytes += chunk.bytes;
                q.chunks.push_back(chunk);
                q.handed += 1;
            }
            q.closing = true;
        }
        self.shared.work.notify_one();
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

/// The render thread: carries the chunks out in order until the stream closes. A panic of the
/// backend is kept as the reason the recording side stops with.
fn render(shared: &Shared, mut backend: Box<dyn Backend>, check: bool) {
    let mut hash = wire::HASH_START;
    let mut encoded = Vec::new();
    loop {
        let mut chunk = {
            let mut q = lock(&shared.queue);
            loop {
                if let Some(chunk) = q.chunks.pop_front() {
                    break chunk;
                }
                if q.closing {
                    return;
                }
                q.renderer_sleeps = true;
                q = shared.work.wait(q).unwrap_or_else(PoisonError::into_inner);
                q.renderer_sleeps = false;
            }
        };
        let started = Instant::now();
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut last = Reply::None;
            for i in 0..chunk.len() {
                let cmd = chunk.get(i);
                if check {
                    encoded.clear();
                    wire::encode(&cmd, &mut encoded);
                    hash = wire::hash(hash, &encoded);
                }
                last = backend.execute(cmd);
            }
            if chunk.check.is_some_and(|recorded| recorded != hash) {
                panic!(
                    "NV_D3D9_STREAM_CHECK: the commands carried out differ from the commands recorded"
                );
            }
            last
        }));
        let busy = started.elapsed();
        let mut q = lock(&shared.queue);
        q.busy += busy;
        match outcome {
            Ok(reply) => {
                q.queued_bytes -= chunk.bytes;
                q.presents_done += chunk.presents;
                if chunk.reply_wanted {
                    q.reply = Some(reply);
                }
                q.done += 1;
                chunk.clear();
                if q.free.len() < FREE_CHUNKS {
                    q.free.push(chunk);
                }
            }
            Err(panic) => {
                q.dead = Some(panic_message(&*panic));
                q.chunks.clear();
                shared
                    .dead
                    .store(true, std::sync::atomic::Ordering::Relaxed);
                drop(q);
                shared.progress.notify_all();
                return;
            }
        }
        let wake = q.producer_waits;
        drop(q);
        if wake {
            shared.progress.notify_all();
        }
    }
}

/// A panic's message.
pub(crate) fn panic_message(panic: &(dyn std::any::Any + Send)) -> String {
    if let Some(s) = panic.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = panic.downcast_ref::<String>() {
        s.clone()
    } else {
        "a panic without a message".to_string()
    }
}

/// `NV_RENDER_THREAD_STATS`: counted on the recording side, printed once a second.
#[derive(Default)]
struct Stats {
    on: bool,
    since: Option<Instant>,
    busy_before: Duration,
    frames: u64,
    commands: u64,
    chunks: u64,
    present: Waits,
    /// The waits for replies, by command.
    replies: Vec<(&'static str, Waits)>,
    /// The layer's events ([`Stream::note`]), by name.
    events: Vec<(&'static str, u64)>,
    full: Waits,
}

/// Time spent waiting, and how many waits.
#[derive(Default)]
struct Waits {
    count: u64,
    time: Duration,
}

impl Waits {
    fn add(&mut self, t: Duration) {
        self.count += 1;
        self.time += t;
    }

    fn per_frame(&self, frames: f64) -> String {
        format!(
            "{:.3} ms ({:.2})",
            self.time.as_secs_f64() * 1e3 / frames,
            self.count as f64 / frames
        )
    }
}

impl Stats {
    fn note(&mut self, what: &'static str) {
        if !self.on {
            return;
        }
        match self.events.iter_mut().find(|(n, _)| *n == what) {
            Some((_, count)) => *count += 1,
            None => self.events.push((what, 1)),
        }
    }

    fn reply(&mut self, name: &'static str, t: Duration) {
        if !self.on {
            return;
        }
        match self.replies.iter_mut().find(|(n, _)| *n == name) {
            Some((_, w)) => w.add(t),
            None => {
                let mut w = Waits::default();
                w.add(t);
                self.replies.push((name, w));
            }
        }
    }

    fn frame(&mut self, shared: &Shared) {
        if !self.on {
            return;
        }
        self.frames += 1;
        let now = Instant::now();
        let since = *self.since.get_or_insert(now);
        let elapsed = now - since;
        if elapsed < Duration::from_secs(1) {
            return;
        }
        let busy = lock(&shared.queue).busy;
        let frames = self.frames as f64;
        let replies = if self.replies.is_empty() {
            "none".to_string()
        } else {
            self.replies
                .iter()
                .map(|(n, w)| format!("{n} {}", w.per_frame(frames)))
                .collect::<Vec<_>>()
                .join(", ")
        };
        let events: String = self
            .events
            .iter()
            .map(|(n, count)| format!("; {n} {:.2} a frame", *count as f64 / frames))
            .collect();
        eprintln!(
            "NV_RENDER_THREAD: {:.1} frames/s; a frame: {:.0} commands in {:.1} chunks, render \
             thread busy {:.3} ms; the recording thread waited a frame: at Present {}, on a full \
             queue {}, for replies: {replies}{events}",
            frames / elapsed.as_secs_f64(),
            self.commands as f64 / frames,
            self.chunks as f64 / frames,
            (busy - self.busy_before).as_secs_f64() * 1e3 / frames,
            self.present.per_frame(frames),
            self.full.per_frame(frames),
        );
        *self = Stats {
            on: true,
            since: Some(now),
            busy_before: busy,
            ..Stats::default()
        };
    }
}
