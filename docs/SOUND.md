# Sound: the platform's audio layers, measured

How the platform answers the game's sound imports, and how far each answer has
been checked against the player's own libraries. Measurements are private
recordings under `build43-scratch/measure` (never committed), made by the tools
in `scripts/`.

| Layer | Module | What it replaces | Checked against | Result |
| --- | --- | --- | --- | --- |
| Music | `engine-platform/src/dshow.rs`, `crates/mp3` | DirectShow's filter graph and MP3 decoder | `scripts/dshow-measure` | Graph answers as measured. All 199 tracks are the same length, and 99.9748% of samples are identical |
| Wave files | `engine-platform/src/mmio.rs` | `mmio*` | `scripts/mmio-measure` | Edge cases as measured |
| DirectSound | `engine-platform/src/dsound.rs` | DirectSound 8 objects, cursors | `scripts/dsound-capture` | Answers and cursors as measured |
| Mixer | `engine-platform/src/mixer.rs`, `engine-platform/src/resampler.rs`, `engine-platform/src/sound_out.rs` | DirectSound's mixer, the device | `scripts/dsmix-measure` (WASAPI loopback, `DSMIX_HOOK`, `DSMIX_SURROUND`), `scripts/dsound-oracle` (the DLLs in a test program) | 2D path, 3D stage, distortion and both resamplers bit-exact (translated from the player's DLLs); 5.1 and 7.1 devices as DirectSound places each buffer, its 2-channel stream until the first 3D buffer and the 4 ms gain ramp (measured on 8 channels) |
| Voices and sounds | `engine-platform/src/vorbisfile.rs`, `crates/vorbis` | `libvorbisfile.dll`/`libvorbis.dll` (1.2.0) | `scripts/vorbis-record` | Bit-exact at 24-bit and 53-bit x87; 64-bit not provided |
| Movie sound | `engine-platform/src/bink.rs` | Bink's DirectSound sound system | `nv-bink --sound-system`, `--wait --sound` | `BinkSetSoundSystem` answers 1 as measured; the movie track plays as a mixer voice on the clock schedule (output off by default) |

## Hearing the game (`NV_SOUND=out`)

The output is off by default and in the tests. With `NV_SOUND=out` the host has `dsound` start
it when the game makes its device (`DirectSoundCreate8` in `BSWin32Audio::Init`, `00aea230`).
From then on these go through the mixer to the default device (WASAPI shared mode):
- every secondary buffer the game plays (WAV through mmio, Vorbis voices);
- the music graph's decoded MP3, one stream that follows `Run`, `Pause`, `Stop`, the
  position and the volume;
- the movie's Bink track.

`NV_SOUND_RECORD=FILE.wav` records what goes to the device (32-bit float; on an endpoint of more
or fewer than two channels one file per stream, "More than two channels" below), and `FILE.log`
lists what started when: real ms, frames recorded, game clock ms, then the event (`Play` of a
buffer with its format, music `Render`/`Run`, the movie's track, sound file opens, `mmioOpen`
names, Ogg opens, pace skips).

**Pace.** The game reads its buffers' cursors and the music position from the platform clock,
and the output follows that clock. Within 100 ms it keeps the device's pace. When the clock
runs ahead it skips on: it mixes the frames and drops them. When the clock falls behind it
gives silence. `NV_SOUND=device` keeps the device's pace instead, for listening.

In the first play run (2026-10-27, `ng.sh`), every game thread's `Sleep` moved the platform
clock on, so it ran ahead of real time while threads slept:

| Phase | Game clock against real time |
| --- | --- |
| Start-up before the first main frame | x16 to x37 |
| Main menu and the intro movie | x1.0 |
| Loading Doc Mitchell's house | x19 |

During start-up the title music was heard only in pieces: the game finished its 125 s track
three times in 18 s of real time and started it again each time.

The thread host now keeps the clock at real time (`thread_host.rs`). A game thread's `Sleep`
waits for its wake time without moving the clock, and main's `Sleep`, or a wait while every
thread sleeps, really waits. Later runs (s11, s13) show no pace skips: the title music plays
whole, UI sounds play when the keys are pressed, and Doc Mitchell's voices start as the house
loads.

What a run plays (s11, `NV_D3D9=real`, the intro skipped with E):

| Real time | What |
| --- | --- |
| 0.4 s | output on (48000 Hz stereo, the default device) |
| 4.3 s | `Music\Special\MainTitle.mp3` (48 kHz stereo, 125.4 s, exact path) |
| 51.3-51.5 s | `ui_menu_focus.wav`, `ui_menu_ok.wav` (32 kHz mono, resampled) |
| 55.3 s | `FNVIntro.bik` track (48 kHz stereo, exact path) until the skip |
| 60.6 s | 16 Vorbis voices (32 kHz mono, 3D, resampled), 5 WAV effects (32 and 44.1 kHz) |

## Mixer (DirectSound 8 to the device)

These were measured on a device without audio enhancements (Virtual Desktop Audio, 48 kHz,
32-bit float stereo). The default device's enhancements smear the signal by
-17 dB, so it cannot be used for these measurements.

**Exact:**
- Samples become `f32` as x/32768.
- The volume and pan gain law is exact for all 429 volumes and 25 pans (see `mixer.rs`).
- A mono buffer goes to both channels.
- Voices are added in `f32`, the one started last first. This holds for 47,996 of
  48,000 samples. The other 4 are the tail of a voice started 2 frames into a device
  period.

- A 3D buffer and a buffer with the distortion: see below (translated from the DLLs).

**Resampling: DirectSound's two converters** (`engine-platform/src/resampler.rs`, translated from the
player's `dsound.dll` and `RESAMPLEDMO.DLL`, 2026-10-28). Every game buffer has
`DSBCAPS_CTRLFREQUENCY`, so every one gets a source-rate converter, and which one is the
device's state:

- **First: the Windows resampler** (`RESAMPLEDMO.DLL` ordinal 1, made by `dsound`'s APO,
  half filter length 30, default cut-off), driven as the APO drives it: each period it is handed
  what `GetRequiredInputFrames` asks for (it drops what it does not use) and makes the period's
  frames; at the device's rate it passes the samples through. Its filter is a Kaiser-windowed
  sinc (beta 10.06) cut at 0.966 of the lower rate's Nyquist frequency, in one of two forms:
  - a polyphase table, `L` rows of 68 taps at 44.1 kHz (`L`/`M` the rates over their gcd), for
    whole rates with at most 1280 phases; it is made once per converter;
  - otherwise (22071 on 48000, a rate that is not a whole number after a change) a curve of the
    filter sampled 16384 times, interpolated for every output frame at a position kept in `f64`
    (added up frame by frame, worked out again every 200 frames and at each period). The curve
    comes from `sin` (Rust's and the DLL's CRT agree on all 16385 points: the curve is the
    DLL's, compared in memory) and the window table.

  Both sum in the DLL's SSE order, which depends on where the input sits in its ring (638
  frames at 44.1 kHz). A change of 1/500 of the rate or less while it plays is its `SetRate` at
  the next period: the output and input positions are carried over in time and the filter is
  designed again (as the curve if a table was made before); after a pass-through its history is
  silence, as in the DLL. Its sinc and window are two piecewise-linear tables that are constant
  data in the DLL: the platform reads them from the player's `SysWOW64\RESAMPLEDMO.DLL` (no
  byte of them is in the repository), finding them by the masks and shifts stored beside them, as
  builds put them at different addresses (checked on 10.0.26100.1 and .9549, whose files both hold
  them, at different offsets). Where they aren't found, a buffer that needs them goes through the
  second converter below: heard, not exact (named once on stderr).
- **Second: "APL high-performance SRC"**, a 4-point Lagrange interpolator in `dsound.dll`.
  `SetFrequency` (or Doppler) moving a buffer's rate by more than 1/500 of it, or a buffer
  made while two or more exist ("More than two channels" below), turns it on for the whole
  device, for good, playing buffers included (each gets a new converter from where it is).
  From then on every buffer goes through it, also those at 48 kHz, whose samples it changes in
  the last bits. 16-bit mono uses a four-lane kernel whose factors drift between periods;
  stereo and 8-bit the plain formula.

Both work in 10 ms periods from the buffer's start. A non-looping buffer ends after the period
in which the frames handed to its converter reach its length (the first converter is handed
what its `GetRequiredInputFrames` asks, 32 or so frames beyond what it has used;
the second what its `CalcInputFrames` asks). A frequency change reaches the second converter at
the next period. The volume, pan and 3D multipliers apply to what the converter gives.

Measured with `scripts/dsmix-measure` (`DSMIX_SRC`, captured with `DSMIX_HOOK`: the tool wraps
`IAudioRenderClient` in its own process and keeps what `dsound.dll` hands the audio engine,
so any device serves; the earlier Virtual Desktop Audio captures agree), every sample up to the
voice's end and the end itself:

| Converter | Cases | Result |
| --- | --- | --- |
| First | 44100, 32000, 22050, 11025 on 48000; stereo; 8-bit; volume and pan; 44150 and 22071 set before `Play` (table, curve); changes while playing (44100 to 44130, 48000 to 48050, 22071 to 22090); lengths 22059, 22081, 22082, 22083, 22100 | bit-exact |
| Swap | 44100 playing on the first while another buffer's large change turns on the second | bit-exact |
| Second | 24000, 47000, 60000, 23456, 44101, 36000, 19000 set before `Play`; 48000 and 44100 mono, stereo, 8-bit; pan; changes while playing (44100, 30000, 23456); lengths 22052, 22059 | bit-exact |

Checked further against the DLL itself, driven by a test program as `dsound.dll` drives it
(`scratch`, private): 44100, 22071, 11025, 8000, 60000 and stereo 22050/22071, `SetRate` to
44130, 44150, 48050, 47990, 22080, 32030, 60050, 11040 and back again, and 144000 frames (the
counters' rebasing), all bit-exact.

Nothing in the resampling is approximate now.

**Moving a playing buffer** (`SetCurrentPosition`; the game does it in `00aed080`, a looping
sound started at a random byte then `Play(DSBPLAY_LOOPING)` even when it already plays, and in
`00aee580` to keep a sound in time). Measured 2026-10-31 with `scripts/dsound-capture setpos`
(the cursors: 412 calls in four runs, 272 of them on playing buffers; with `endphase`, 162 more
cases of `Play` after an end or a `Stop`) and `scripts/dsmix-measure` `DSMIX_SETPOS` with
`DSMIX_HOOK` (what is heard: 42 captures, three runs, 48000, 22050 and 44100 Hz, mono and
stereo, looping and not, two with the distortion, every sample). The cursors are in
`dsound.rs`. What is heard: what was mixed up to the write cursor plays out, then the next
period mixes from the new place through the buffer's converter as it was, history and all (a
new converter matches none of the captures at 22050 or 44100 Hz); a buffer with the distortion
has the effect started over there as at `Play` (cleared, from 32 frames before).
`mixer::Voice::move_to`; the voice moves at its next period. The moved voice keeps its place in
the sum (`DSMIX_SETPOS_ORDER`: three voices, one moved; the oldest stays last whichever moved).
Not measured: a voice the output has already played to its end (it runs up to 100 ms ahead of
the platform clock) while its buffer still plays starts again at the new place with a new
converter, in its place in the sum.

**The cursors' steps** (`dsound-capture endphase`, `freqphase` and `endstop`, 2026-10-31, and
`endstop more`, 2026-11-01): the write cursor is the frames the buffer's converter has taken.
Through the first converter that is the buffer's position, kept in hundredths of a frame (each
10 ms period adding the frequency) through `Stop`, the end, `SetCurrentPosition` and
`SetFrequency`, rounded up, and a first `Play` leads by 33 frames; at the device's own rate (the
converter passes the samples through) there is no lead. Through the second converter it is
exactly what that converter's `CalcInputFrames` (`dsound` `100636a0`) takes, in its own `f32`
arithmetic (`resampler::AplFrames`, checked against `Apl` period by period), counted from a new
converter at the buffer's first `Play` and kept by the buffer through `Stop`, the end and
`SetFrequency` (measured) and `SetCurrentPosition` (on the DLL's word: `dsound` `100580b0` only
moves the source and sets the cursor fields; no measured case moves a buffer with it on). No
rule of hundredths fits that one everywhere: a 22050 Hz 16-bit mono buffer reads 221, 442, 663
after 1, 2, 3 periods, the same buffer at 11025 Hz 111, 221, 331, 441, a 48000 Hz one 481, 961.
When the second converter turns on, every buffer playing goes on from its write cursor with a
new converter, both cursors there, the play cursor held until the write cursor is three periods
past (`dsound` `1005713d` sets both cursor fields to the write cursor, as `Play` and
`SetCurrentPosition` do; not measured). A `SetFrequency` to the frequency the buffer has
changes nothing (`loopoff_f`). (`dsound.rs`.)

The layer's replay of every measured line (`measured_replay` in `dsound.rs`, ignored; the
programs and the comparison are `tools/programs.pl` and `tools/cmp2.pl` in the private
`build43-scratch/measure/dss1-work`): the same calls after the same number of periods, the
cursors read at every period and compared with DirectSound's, the first `Play` included. Lines
that match at every read:

| Data set (`dsound-capture`) | Lines | Main (`8dda88cf`) | Now | Main, from the first call on | Now, from the first call on |
| --- | --- | --- | --- | --- | --- |
| `replays` (2026-10-30) | 4 | 4 | 4 | 4 | 4 |
| `replays`, second set | 12 | 8 | 12 | 8 | 12 |
| `endphase` | 162 | 0 | 162 | 30 | 162 |
| `endstop` | 168 | 15 | 168 | 45 | 168 |
| `endstop more` | 144 | 42 | 144 | 75 | 144 |
| `freqphase` | 156 | 12 | 114 | 30 | 120 |
| `setpos` | 412 | 24 | 408 | 32 | 408 |
| All | 1,058 | 105 | 1,012 | 224 | 1,018 |

No line matches on main and not now. ("From the first call on" leaves out the reads before the
first call after `Play`, as the earlier reviews compared.)

The lines that still differ:

- `freqphase` with the first converter at a frequency other than the format's rate (`dmo` at
  11040 or 22071 Hz: `steady`, `retune_stop`, `stop_retune` and `retune_end`, 14 cases in all
  three runs, 42 lines): there the write cursor is `RESAMPLEDMO`'s own
  `GetRequiredInputFrames`, one frame off the rule of hundredths at some periods, and a change
  of 1/500 or less reaches it a period late (its `SetRate`). Following it would need the
  converter's filter design from the player's DLL, which the cursors must not wait for; the
  layer keeps the rule of hundredths there. Deliberately not modelled.
- `setpos` `5622 1 10 ac44 302b0 loop_odd 100000` (run 2), `5622 1 10 ac44 2e0 loop_half 103300`
  (run 3), `ac44 2 10 2b110 102e0 loop_half_play 106600` and `ac44 2 10 2b110 102e0
  loop_odd_play 100000` (run 4): `SetCurrentPosition` raced DirectSound's period (three came 5
  to 24 us after a period began, one 53 us before the next): the cursors read the new place
  right after the call (in three the old ones again until the next period), then the play
  cursor ran three periods behind the write cursor, behind the new place, for two periods. A
  race inside DirectSound; the layer leaves it out (`dsound.rs`).

Not measured, and where the layer may differ:

- DirectSound caps the play cursor's lag at half the buffer (`dsound` `10056d3e`..`10056d53`,
  read by the review); the layer's three periods at the format's rate have no cap. Only a
  buffer shorter than six periods' frames is touched, and none was measured; the cap's exact
  arithmetic (bytes or frames, rounding) is not read yet.
- The cursors' counter for the second converter is kept through `Stop` and `Play`, but the
  output gives a voice a new converter at every `Play` (`mixer::Voice::start`). Whether the
  converter's own samples and fraction survive a `Stop` is not measured; a `DSMIX` capture of a
  stopped and replayed buffer at another rate would show it.
- A buffer playing when the second converter turns on (above).

**The end** (`dsound-capture endstop`, 2026-10-31): a buffer that does not loop never reads its
write cursor at its length while it plays; it ends in the period the write cursor would reach
it. Stopped in the period before, `Play` plays the rest. A looping buffer turned not looping
plays out the lap it is in.

## 3D and the distortion: translated from the player's DLLs

With the maintainer's leave (2026-10-28, "sound must match exactly"), the 32-bit `dsound.dll`
and `dsdmo.dll` the game loads (`SysWOW64`, Windows 11 10.0.26200) were read as instructions in
a private Ghidra project (not the shared servers). The platform follows their order of
operations. No bytes of either DLL are in the repository: the 3D stage's trigonometric tables
are computed from the formulas they were made with, and the distortion's one table that is not
a formula is measured from the player's own `dsdmo.dll` when the game first plays the effect.

### What the game asks for (exe)

- **Listener** (`00aea8b0`, every frame): position and orientation with `DS3D_DEFERRED`, then
  `CommitDeferredSettings`; rolloff 1 (deferred, at start-up). The listener's velocity slot
  (`00aeaa10`) is never called (all 192 callers of `BSAudio::QInstance` and the 30 users of the
  audio singleton checked), and no buffer velocity is set, so Doppler never engages.
- **3D buffers**: minimum and maximum distance both 1e9 (`00aec530`, `00af0630`, `00aefda0`),
  so DirectSound's distance factor is always 1 and the game sets the volume itself; position
  with `DS3D_DEFERRED` (`00aef090`, `00aed990`). No cones, no `SetMode`.
- **Distortion** on the sounds flagged `0x80000` (`00aec530`): gain
  `fFilterDistortionGain:Audio` (default -18.2 dB), edge 0, post-EQ centre 5000 Hz, bandwidth
  1135 Hz, pre-lowpass cutoff 5000 Hz. The I3DL2 reverb (`00af1430`) is not registered on
  Windows 11 (captured `REGDB_E_CLASSNOTREG`), so it is never heard.

### The 3D stage (`engine-platform/src/ds3d.rs`)

For a stereo device (`CMultiPan3dObject`; other layouts below, "More than two channels"):
1. **Listener frame.** The offset from the listener is turned by three angles taken from the
   listener's top and front vectors. The angles come from an arcsine table (steps of 0.01, 0.001
   above 0.9); the turns use a sine table in **whole degrees, truncated** (no interpolation). So a
   listener turned by 33.7 degrees hears the world turned by 33. `SetOrientation` stores both
   vectors made unit length and the top made square to the front.
2. **Distance.** `f = min / ((d - min) * rolloff + min)` past the minimum distance (`d` held at
   the maximum), as `trunc(602.059991328 * log2(f)) - 500` millibels: a 3D sound is always at
   least **5 dB** down. With the game's distances `f` is 1, so -500.
3. **Pan.** Speakers at -90 and +90 degrees; the share between the two nearest by the tangent
   law from the azimuth `atan2(x, z)`; each share as `trunc(301.03 * log2(share))` millibels.
   **Near zone:** when `|x|`, `|z|` and `sqrt(x^2 + z^2)` are all under **2.5 units**, a part
   `0.5 + r / 5` is panned from the azimuth and the rest from the opposite direction. The 2.5
   units are absolute: they do not scale with the minimum distance (so they apply with the
   game's 1e9 too, about 175 game units). Height counts only in the distance (step 2).
4. **Device.** Each channel is multiplied by `gain(volume + distance) * gain(pan)`, two lookups
   of the measured volume law (`mixer::gain`), multiplied as `f32` (`10057be0`).
5. **Doppler.** `f * 359660 / (v + 359660)` for the radial speed `v` (units/s times the distance
   factor, times 1000, times the Doppler factor, held to +-179830), clamped to 100..200000 Hz.
   Implemented, but the game never sets a velocity.
6. **Deferred settings.** Each setting made with `DS3D_DEFERRED` waits on its own (the DLL keeps
   a deferred copy with a flag per setting) until `CommitDeferredSettings`; `Get*` answer the
   settings in effect.

**Measured** (the captures made on the transparent device, every sample of both channels
compared after the alignment, with `ds3d.rs` and `dsfx.rs` as they are in the engine):

| Captures | What | Samples exact |
| --- | --- | --- |
| `dsmix-3d-2026-10-24` | 305 grid positions (x -4..4, z -4..4, heights 0 and 1), 13 azimuths, 10 distances 0.5 to 100 with minimum distances 1 and 3 and rolloffs 0 to 1.5, volume -500 with 3D, and the 2D volume and pan sweeps | 4,536,000 of 4,536,000 |
| `dsmix-sweep-2026-10-24` | the volume, pan, azimuth and distance sweeps (an earlier run) | 3,072,000 of 3,072,000 |
| `dsmix-vd-2026-10-24` | 11 3D cases (front, sides, behind, above, distance 10 with minimum 5, rolloff 2, maximum 4), the 4 distortion cases, the 2D cases | 1,391,946 of 1,392,000 |

The 54 others are the device-period tails every capture loses (Mixer above), at most 4 a case.
The old approximate law (`NEAR_SHARE`, "the power sums to a third") is gone: the third was the
-5 dB of the distance stage and the -3 dB of an even split.

### The distortion (`engine-platform/src/dsfx.rs`)

Per sample, in `f32`: a one-pole lowpass; a waveshaper on the integer part of the value (an
integer "float" with an exponent of 2 to 5 bits chosen by the edge); the gain; a two-state
lattice band-pass; back to the buffer's format, **rounded to nearest for mono and truncated for
stereo** (the DLL's mono and stereo loops differ), 8-bit buffers divided by 256 toward zero. The
coefficients follow `1000d8d0` with its `f64` and `f32` roundings, frequencies held to a third
of the rate. The effect runs at the buffer's format rate, before the volume, 3D and resampling.

`dsound.dll`'s chain: when a buffer starts, the effect's state is cleared and it runs from
**32 frames before the play position** (wrapping to the end of the buffer), so the start of a
sound hears its own tail.

The one table that is not a formula, the band-pass's gain correction for each edge setting
(32 values with two decimals), is measured once from the player's `dsdmo.dll` through COM (an
impulse on a float stream, `dsfx::edge_table`). The values measured in a 64-bit process match
the 32-bit DLL's table entry for entry (checked privately). The game's edge 0 uses entry 2.

**Measured:** the four `dsmix-vd-2026-10-24` distortion captures (gain 0, -18 and -40 dB, edge 0
and 50, 48 kHz mono 16-bit) are bit-exact for 23,998 of 24,000 samples each; the last two are
the device-period tail every capture loses (Mixer above). Without the 32-frame start, the first
100 or so samples differ.

### Still open

- The x87 `FYL2X` and `atan2` of the DLL are `f64` here: a truncated millibel can differ only
  when its product lies within about 1e-13 of a whole number.
- The DLL's effect runs about 100 ms ahead of the play cursor (its write-ahead, 10 ms ticks);
  here it runs just ahead of what the mixer reads. Data the game writes into a playing buffer
  with the effect inside those 100 ms is processed here and unprocessed in Windows.
- The cursors of a buffer with the effect set are not those of `dsound.rs`: the `DSMIX_SETPOS`
  captures read the write cursor 130 to 140 ms ahead of the play cursor (not three periods),
  and right after `SetCurrentPosition` 140 ms less 32 frames past the new place. Not measured
  further yet.
- Not provided (the game does not use them): cones, `SetMode`, other effects. A device layout
  DirectSound does not place 3D buffers on (below) is not provided either.
- No loopback capture yet of turned listeners, the game's 1e9 distances, or the effect on
  stereo, 8-bit and other rates (no transparent device here: the others process the signal).
  They are covered without a device instead (next section); `DSMIX_EXACT=1` in
  `scripts/dsmix-measure` plays them (cases `x_*`) when a transparent device is available.

### Without a device: the DLLs run in a test program (`scripts/dsound-oracle`)

`dsound-oracle` (32-bit, like the game) loads the player's `dsound.dll` and `dsdmo.dll` and
compares them with `ds3d.rs` and `dsfx.rs`, compiled into it as they are in the engine. Nothing
is heard: the 3D buffer plays silence, and the effect runs on test signals in memory.

- **3D:** random listeners and sources are set through the 3D interfaces (immediate), and the
  `f32` channel multipliers the DLL hands its mixer, its listener-frame position and its Doppler
  frequency are read from its software 3D object (through the analysed DLL's offsets, checked
  against its vtables first). A third of the cases use the game's settings (1e9 distances,
  camera orientations, positions to 300 units). Deferred moves are checked before and after
  `CommitDeferredSettings`.
- **Distortion:** the effect object itself (`IMediaObjectInPlace::Process`) on noise at three
  levels, PCM 8 and 16 bit, mono and stereo, 11025 to 48000 Hz, with the game's parameters, the
  extremes, every edge band and 20 random sets.

| Run (2026-10-28) | Cases | Exact |
| --- | --- | --- |
| 3D channel multipliers | 20,000 (17,772 heard, 16,830 with the channels apart, 12,560 within 2.5 units, 17,142 turned by the listener, 4,000 Doppler-shifted) | 20,000 |
| 3D listener-frame positions | 20,000 | 20,000 |
| 3D Doppler frequency | 20,000 | 20,000 |
| Deferred position (unchanged before the commit, right after) | 1,819 | 1,819 |
| Distortion | 1,060 format, rate and parameter cases, 9,540,000 samples | all |

The captures (above) show that these multipliers are what reaches the device; the oracle shows
that ours are the DLL's for settings no capture covers.

## More than two channels (5.1 and 7.1 devices)

**Why.** Windows gives a program its endpoint's shared-mode mix format, and on this laptop the
format depends on the program's name: a program named `FalloutNV.exe` gets 8 channels, 48 kHz,
32-bit float, speaker mask 0x63F (front left, right, centre, low frequency, back left, right,
side left, right); any other name gets 2 channels, mask 0x3 (measured again 2026-11-03; the
headset's per-app processing, keyed by the name, deterministic). Players whose Windows output is
5.1 or 7.1 get more than two channels under any name. The port must run as `FalloutNV.exe`
(NVIDIA's driver profile, also keyed by the name, removes the white dots), and its mixer used to
stop at sound start on anything but two channels. It now hands Windows what DirectSound hands it
(`engine-platform/src/speakers.rs`, `mixer.rs`, `ds3d.rs`, `dsound.rs`, `sound_out.rs`).

**Measured** (`scripts/dsmix-measure` mode `DSMIX_SURROUND`, 2026-11-03, private results in
`build43-scratch/measure/sound71/measure`): the tool wraps `IAudioClient` and
`IAudioRenderClient` in its own process before `DirectSoundCreate8` and keeps every packet
`dsound.dll` releases, from every stream it opens: DirectSound's own mix, before Windows' engine
and the headset (no loopback). The game's calls (flags, listener, deferred settings, one commit).
187 cases, four runs: as a copy named `FalloutNV.exe` with a 3D buffer made first (every case on
8 channels), as `FalloutNV.exe` in natural order (the 2D cases first), and under another name (2
channels) both ways.

| What | Under `FalloutNV.exe` (8 channels) | Under another name (2) |
| --- | --- | --- |
| Endpoint mix format | float32, 48000 Hz, 8 ch, mask 0x63F | 2 ch, mask 0x3 |
| `GetCaps` | the same 0x60 bytes under both names (no hardware buffers: the game asks the default 3D algorithm) | |
| `GetSpeakerConfig` | 8 (7.1 surround), at the start and the end | 0x140004 (stereo, wide) |
| Primary buffer, listener defaults | the same under both names | |
| `CreateSoundBuffer` | `DS_OK` for every case, `HRTF_LIGHT` included | the same |
| DirectSound's stream, 2D buffers only | `IsFormatSupported` and `Initialize` float32 2 ch, mask 0x3 (`S_OK`): the stereo law, bit for bit as under the other name (34 cases) | 2 ch |
| From the first 3D buffer made | the 2-channel stream closes, an 8-channel one (mask 0x63F) opens; 8 channels for good | no change |

**The device's layout** (read from `dsound.dll` `10052857`, `10053984`, and measured): on an
endpoint of three or more channels a DirectSound device mixes **two channels (front left and
right) until its first 3D buffer is made** (made, not played); that buffer **locks** it to the
endpoint's format for good. On a one- or two-channel endpoint the first buffer locks it (the
layout stays the endpoint's). At the lock DirectSound stops and disconnects every buffer, closes
its stream, gives the listener the new speakers, connects every buffer again and opens its stream
in the new layout. Measured (`k_lock_2d_then_3d`): `CreateSoundBuffer` of the 3D buffer took 59.6
ms; the 2-channel stream's last packet, about 63 ms with none, then the 8-channel stream's first
packet, 480 frames of silence, then a 2D sound that was playing goes on at exactly the frame
where it stopped (nothing skipped or repeated), now on the centre speaker, and the new 3D buffer
from its first frame. Bink's and DirectShow's own devices make only stereo 2D buffers, so on such
an endpoint they never lock and keep a 2-channel stream.

**2D buffers on 8 channels** (exact, every sample `f32(x * g)`):
- mono: **front centre only, at `gain(volume)`; the pan is not heard** (every pan from -10000 to
  10000); nothing on the other seven channels;
- stereo: left to front left, right to front right, at the stereo law's left and right gains;
- the converter (11025 to 44100 Hz), the second converter, frequency changes, 8-bit and the
  game's distortion give the same signal as on 2 channels: only where it goes differs;
- channels a voice does not reach hold `x * 0`: -0 wherever the sample is negative.

Read from the DLL (`10057d90`, default matrix `10067b0e`, `100676b6`), which gives the measured
result: a default matrix from the buffer's speakers (mono: front centre; stereo: front left and
right) to the device's, by speaker angle (front left 330, right 30, centre 0, back 225/135, side
250/110, front of centre 345/15, back centre 180 degrees; between the two nearest device speakers
by `cos`/`sin` of the fraction times pi/2; divided by the largest speaker sum and rounded to three
decimals, in `f32`); then each device row times the total left gain (left speakers), the total
right gain (right speakers) or the volume's gain (centre, low frequency, back centre), the gains
of the stereo law. So on quad (no centre) mono is front left and right with the stereo law; on
one channel stereo is halved into it.

**3D buffers on 8 channels** (exact, one `f32` multiplier per channel, no filter, no delay):
speakers at front centre 0, front right 45, side right 90, back right 135, back left -135, side
left -90, front left -45 degrees (the DLL's table for mask 0x63F, `1002a8cb`); the stereo pan
(`ds3d.rs`) over them: the two speakers either side of the azimuth by the tangent law (over pi/2,
whatever their spacing), the 2.5-unit near zone with `r/5 + 0.5` toward the sound and the rest from
the opposite direction, height only through the listener frame. Each channel is
`gain(pan) * gain(volume + distance)` (`10057be0`). At the listener (and straight above or below)
every channel, the low-frequency one too, gets one eighth (-903 mB); otherwise that channel gets
nothing. All 117 still, unturned 3D cases match this model bit for bit on 8 channels and the
stereo table on 2; `DS3DALG_HRTF_LIGHT` is placed the same; turned listeners, the converters,
Doppler and the distortion feed the same signal to both layouts. The other layouts' tables (read
only): quad -pi/4, pi/4, -3pi/4, 3pi/4; surround -pi/4, pi/4, 0, pi; 5.1 -pi/4, pi/4, 0, LFE,
-3pi/4, 3pi/4 (side 5.1: -pi/2, pi/2); 7.1 wide adds -pi/8, pi/8 to 5.1. The low-frequency
speaker has the azimuth 42, never one of the two nearest.

**The mix** (`10065690`, `10062b60`): on any layout but two channels every voice goes through the
general kernel: per device channel `acc = in[0] * m[0]` (then `+ in[1] * m[1]` for stereo), in
`f32`; the first voice of a period writes `acc`, the others add theirs, newest first.
**A gain change** (volume, pan, every 3D change) is reached at the start of the next 10 ms period
over 192 frames (4 ms at 48 kHz): frame `n` takes `f32(f32(f32(n) * step) + old)`, `step =
f32(f32(new - old) / 192)`; the first period after a buffer is connected takes its matrix as it
is. Every one of the 807 ramps of the locked run is exact; the instant change the port had fits
none of them.

**What the port does.**
- `sound_out` opens a stream in the endpoint's mix format as it is, or in another layout at its
  rate (32-bit float, `WAVE_FORMAT_EXTENSIBLE`, `IsFormatSupported` first and only `S_OK`
  opens it, as DirectSound does).
- On a two-channel endpoint (mask 0x3) one stream carries every voice through the stereo kernels,
  as before; what changed there is the second converter by buffer count (below), which DirectSound
  follows under every name.
- On any other endpoint the game's device and the platform layers' own devices (the movies, the
  music) are a stream each, as in Windows. The game's stream is 2 channels until its first 3D
  buffer; then it is closed, every voice playing goes on from where it is with a new converter,
  the 3D voices are placed over the new speakers, and the stream is opened again in the
  endpoint's layout, starting with one period of silence. The port's output thread asks for as
  many frames as the device can take, which need not end on a 10 ms period (DirectSound's packets
  always do): what a voice's converter gave and was not sent yet is heard first after the
  silence, then a new converter goes on from the frame after those it was handed, so nothing is
  skipped. The endpoint is asked again at the lock: one that no longer takes the layout (the
  default device changed since the output started) gets a stream in its own mix format and the
  game's device mixes that, as DirectSound's device takes a new default format at once
  (`10055086`, read only); one at another rate is not provided (named on stderr, no sound from
  then on, no stop). The movies' and the music's stream
  stays at 2 channels. Two-channel streams are what DirectSound gives Windows too, so what Windows
  and the headset do with them is the same by construction (not measured here, and not needed).
- A layout other than two channels uses DirectSound's general kernel with its ramp; two channels
  keep the measured stereo kernels (gains changed at once, below).
- `GetSpeakerConfig` (slot 8, which the game is not seen to call) answers from the endpoint's mix
  mask whether or not the device is locked: 8 for 0x63F and 0x140004 for 0x3 (measured), 2 for
  0x4, 3 for 0x33, 5 for 0x107, 6 for 0x3F, 7 for 0xFF, 9 for 0x60F, 0 with `DS_OK` for a mask of
  0 (the DLL's table, `10053610`), `E_FAIL` for another. `GetCaps` and the primary buffer are the
  captured table's (the same under both names).
- A recording (`NV_SOUND_RECORD=FILE.wav`) on such an endpoint writes the game's stream to
  `FILE.wav` (from the lock on `FILE.8ch.wav`, `WAVE_FORMAT_EXTENSIBLE` with the mask) and the
  movies' and the music's to `FILE.own.wav`.
- A mix format that is not 32-bit float, or a layout DirectSound does not place 3D buffers on
  (not mono, stereo, quad, surround, either 5.1 or either 7.1; in the DLL the first 3D buffer then
  fails with `E_FAIL` and later ones are silent), is not provided: no sound, named once on stderr,
  and the game goes on. This holds on two channels too: before this change any two-channel float
  endpoint played the stereo path whatever its mask. A mask of 0 (direct out, which some USB and
  virtual devices report) is silent in the original as well: there DirectSound refuses a buffer
  without the device's channel count (`DSERR_BADFORMAT`) and one with `DSBCAPS_CTRL3D` or
  `DSBCAPS_CTRLPAN` (`E_INVALIDARG`, `10056ab3`), and every buffer the game makes has one or the
  other (`00aeb270` in the exe: flags 0x40 for 2D, 0x30010 for 3D, each with 0x2a0). (The
  layer's `CreateSoundBuffer` still answers as on the measured devices there.) The movies' and
  the music's own devices there were not read. Another two-channel mask (a side or back pair) is
  not provided either, and there the port differs from the original (read only, not measured):
  the listener has no speaker table for it (`1002a8cb`), so the first secondary buffer, which
  locks the device on a two-channel endpoint, fails with `E_FAIL` as read for the larger layouts
  above; 3D buffers after it are silent, but 2D buffers after it play through the default matrix.

**Replayed through the layer** (`dsmix_surround_captures` in `dsound.rs`, ignored; the folders in
`NV_DSMIX_SURROUND`): each case made with the tool's calls, its moves (volume, pan, frequency,
position, the lock) made before the first packet released after them, every packet compared bit
for bit from the one its voice starts in (a move whose call began less than 200 us before a
packet was released raced that packet inside DirectSound; two such moves, 35 and 21 us before
it, are taken in the next packet). A lock case's 3D buffer is made at the call's start and
played when its `Play` was called, which can be a packet or more after the stream was opened
again. The stream opened at the lock starts with one silent packet in 17 of 18 captures as
`FalloutNV.exe` and with two in one (`z_lock_with2d`, whose second 8-channel packet was released
before `CreateSoundBuffer` returned, the voices not yet connected again): the layer gives one
period, and a case that differs is tried with the silent packets it has (a race inside
DirectSound). Results, 2026-11-03:

| Run | Cases replayed | Bit for bit | Only the sign of zero differs | Not replayed |
| --- | --- | --- | --- | --- |
| `FalloutNV.exe`, locked first (all on 8 channels) | 187 | **187** | 0 | 0 |
| `FalloutNV.exe`, natural order: 148 cases on 8 channels, the lock case (2 then 8), 38 on 2 | 183 | **180** (all 149 with 8 channels) | 3 (on 2 channels) | 4 (2-channel ramps) |
| Another name (2 channels) | 180 | 171 | 9 | 7 (2-channel ramps) |
| Another name, locked first | 180 | 171 | 9 | 7 |
| Later captures, both names (28 folders: the lock with one or two 2D voices playing, at 22050, 44100 and 48 kHz, with volume, pan, the distortion; ramps; the lock 11 times more as `FalloutNV.exe`) | 46 | 44 | 2 (on 2 channels) | 0 |
| All | 776 | 753 | 23 | 18 |

Every case with 8-channel packets matches every sample: 2D mono, stereo, 8-bit, every rate, the
distortion (the player's edge table), the 3D grid, HRTF, turned listeners, distances, Doppler,
two and three voices, frequency changes, the four volume and pan changes (the ramp), the three
moving sources (every ramp), and the lock (the 2-channel packets, the silent packet, the 2D voice
going on at its frame on the centre, the 3D voice from its first frame). On two channels the
only differences are signed zeros: where a gain is 0 (a pan of +-10000, a 3D sound straight to
one side, a muted one) DirectSound's stereo kernels write `x * 0` (-0 for a negative sample) and
the port's add it to +0. The stereo kernels are unchanged here (`mix_bits_are_unchanged`); the sound is the same. The
changes while playing on two channels are not replayed (the stereo kernels' ramp, below).

The captures also settle the second converter by buffer count (read in the DLL, `10052857`,
`10052af5`..`10052b7b`): a buffer made while two or more exist turns it on for the device, for
good, under both names. When the buffer does not lock the layout every listed buffer's converter
is swapped where it is (`100527c0`, `1005713d`); when it does, the flag is set before the lock
connects every buffer again. The layer follows it (`second_converter_by_count`): `w3_three_2d`
(three 2D buffers made, then played) and `z_lock_with2d` (two 2D buffers playing when the first 3D
buffer is made) match with it, and the cases after `w3_three_2d` start with it on, as the
measuring tool's device has it. The game makes many buffers, so from its third on the game's
device converts every voice with the second converter, stereo included; the cursors of buffers
playing when it turns on follow the DLL's reading (each goes on from its write cursor with a new
converter), not measured.

**Measured, the layouts above; read only, the rest.** Measured on this machine: 0x63F (8
channels) and 0x3 (2). One channel, quad, surround, both 5.1 and 7.1 wide would need a change of
Windows' speaker setup; their matrices, speaker tables and the lock rule follow the DLL's code and
are tested only against it (`speakers.rs`, `dsound.rs` tests).

**Open:**
- The ramp on two channels: DirectSound's stereo kernels ramp too (stereo buffers by two
  interleaved running sums, exact; mono buffers the same after 0 to 7 frames stepped one at a time
  until a Windows buffer address lines up, a count that changes from period to period and so
  cannot be known from the API). The two-channel path still changes gains at once; it is left
  as it is (its measured hashes) until that is settled.
- The general kernel's ramp is measured on 8 channels; on a one-channel endpoint a mono buffer
  goes through DirectSound's diagonal kernel, whose ramp is not read (the general one is used).
- What is heard during the reopen at the lock (Windows' side), and the lock with 3D voices or
  more than two 2D voices playing.
- The order of a sum of three voices on one speaker below full gain (two-voice sums are
  order-free; the three-voice case summed at unit gains).
- The cursors of buffers that play when the second converter turns on by buffer count (read
  only, as for a large frequency change).
- The sign of zero on two channels (above): the stereo kernels write, the port adds.
- A two-channel endpoint with a mask other than 0x3 or 0 (above): no sound in the port, the
  original's 2D buffers after its first (read only).
- The game itself was not run as `FalloutNV.exe`: when it makes its first 3D buffer.

## Vorbis

`scripts/vorbis-record` drives the player's `libvorbisfile.dll` the way the game
does (`00af1e00`, `00af2100`). It reads every `.ogg` in the archives (63,575 files)
and records the decoded bytes. At 24-bit and at 53-bit x87 precision, `crates/vorbis`
gives the same bytes and the same `ov_pcm_total` for all of them. It follows the public
libvorbis 1.2.0 source's order of operations (BSD, see `LEGAL.md`).

The library computes on the x87 unit, so its output depends on the decoding
thread's precision control: 62,819 of the files differ between 24 and 53 bits.
At 24 bits every operation rounds to 24 bits, which is the same as `f32` in the
source's order.

**Which thread decodes is not measured yet.** The thread is the one that runs
`AudioLoadTask::Run` (`00ae7050`), whose path to the decoder is
`00aeb270`, `00aebff0`, `00af2100`. Load tasks go to the IO manager (`01202d98`),
so they probably run on a task-manager thread. Such a thread starts at
`0x027F` (53-bit) unless something on it changes the control word.

### 53 bits: how the match was found

The recorder's `--blocks` mode calls `libvorbis.dll`'s public API
(`vorbis_synthesis`) and records every block after the inverse MDCT.
`VORBIS_PACKETS=<file>` does the same for crafted streams. A crafted stream has
a floor of 1.0 and exact spectra (impulses, random values), so its blocks are
the DLL's inverse MDCT of a known input.

Measurement alone did not find the 53-bit model. None of these came close:
rounding at each C assignment to `float`, with `float` locals rounded or kept;
angles at 24 or 53 bits; 486 per-stage combinations.

With the maintainer's leave (2026-10-24), the DLL's transform was then read as
instructions: its own Ghidra project in the agent's scratch folder, not the
shared servers.

**The 53-bit path follows the DLL's instruction order.** That order is the C
source's, statement for statement:
- every `float` local of the transform is stored to the stack, so it is rounded to
  `f32`;
- each expression is evaluated in registers at the thread's precision and rounded
  to `f32` when stored.

The one difference from the public source is the trig tables' `M_PI`: the DLL
uses `pi` rounded to `f32` (3.14159274...). At 24 bits that makes no difference,
because `pi / n` rounds to the same value. At 53 bits it changes every angle.
With that constant, the crafted blocks and all 63,575 game files match at
53 bits as well.

No bytes from the DLL are in the repository. The constant is the `f32` value of
`pi`, computed in the code.

`engine-platform/src/vorbisfile.rs` decodes at the calling thread's precision: 24 or 53
bits. 64-bit precision is not provided (not measured).

### Decoding ahead (`NV_SOUND_DECODE`)

The game decodes each voice or sound file whole on its IO thread (`00af2100`, from the
audio load task). The original's IO thread does that on its own core while the main thread
draws frames; here the game's threads take turns on the main core, so a long file's decode
landed inside one frame (in Goodsprings the two longest frames, 114 and 120 ms, were almost
all `ov_read`). In the running game (the real clock on) each opened file is now decoded ahead
on a host worker thread, and `ov_read` answers from what the worker decoded: the same bytes,
the same return values, errors and the end at the same reads (the decoder's own call
fetches a block only when none is ready, so the worker's reads of whole blocks cut the
stream where it would). A game thread whose read finds nothing decoded yet gives up its
turn (`Sleep(1)`) until a batch is ready; the main thread goes on with its frames. The game
sees only its IO task take time, as on the original. `NV_SOUND_DECODE=inline` decodes inside
the call as before; the tests and platform-clock runs always do.

Measured on the Goodsprings stutter route (one build, ahead then inline, 2026-11-07): the town's
p99 frame 27.9 ms (inline 30.5), p99.9 35.1 (56.1), frames over 33 ms 9.8 a minute (18.6), over
50 ms 1.5 a minute (4.5); the median frame (21.7 ms) and Doc's house (about 112 fps) unchanged.

## Bink with sound

The game calls `BinkSetSoundSystem(BinkOpenDirectSound, 0)` (`00ec1680`). An
earlier measurement (2026-10-23) recorded the answer as 0, and `engine-platform/src/bink.rs`
was written to match it: no sound, frames timed by the clock. That measurement
was wrong: `nv-bink` passed a garbled export name to `GetProcAddress`, so it
handed the DLL a NULL function.

Measured again on 2026-10-24 (`nv-bink --sound-system`, private recordings in
`bink-sound-2026-10-24`):

- **The answer is 1**, with or without a window.
- **That first run without a window** (`nv-bink --wait 300 --sound`): the frames came
  about 3 ms late with about ±2 ms of jitter. With `--hook-dsound` it turned out the
  library's `Play` had failed there (no window for the priority level), so that run
  did not have sound. The runs below have a window and do.

### What the library does with DirectSound (measured)

`nv-bink --wait --sound --window --hook-dsound` records every DirectSound call the
library makes while the intro plays with sound:
- **Device:** its own (`DirectSoundCreate(NULL)`), at the priority level on a window.
  Without a window the level fails and so does `Play` (`DSERR_PRIOLEVELNEEDED`).
- **Primary buffer:** given a format and played looping.
- **One secondary buffer:** 115,712 bytes of 48 kHz 16-bit stereo (0.6 s), flags `0x180e0`
  (frequency, pan and volume controls, `GETCURRENTPOSITION2`, `GLOBALFOCUS`). Frequency is
  set to 48000, pan and volume to 0.
- **Filling:** at the first `BinkDoFrame` the buffer is cleared, then filled whole.
  `Play` (looping) follows. Then, about every two frames, the part already played is
  refilled: 11,520 or 13,440 bytes, wrapping round. The play cursor is polled about every
  13 ms.
- **When `Play` comes:** 50 to 540 ms after the fill, varying from run to run.
- **Frames with sound:** frame 1 is due one frame time after the sound starts. From
  there the frames keep exactly the clock schedule they keep without sound: 600 frames,
  the same per-frame times to within 0.05 ms.

### What the platform does

`BinkSetSoundSystem` answers 1, as measured. The game calls it at start-up
(`BGSMoviePlayer`, `008670e0`) whether or not a movie plays.

A movie with a sound track has its track decoded whole at `BinkOpen` by
`crates/bink` (within one step of the DLL, docs/MOVIES.md). The track becomes one
mixer voice (`dsound::play_stream`), started at the first `BinkDoFrame`, when the
clock schedule starts. It is held while the movie is paused (`BinkPause`) and ended
at `BinkClose`. It is heard only when the host has turned the output on, which is
off by default.

This gives the same samples on the same schedule as the DLL. What is not
reproduced:
- The DLL streams the track through a 0.6 s buffer instead of one voice; only the
  samples heard matter, and they are the same.
- The DLL's 50 to 540 ms device start, which varies from run to run.

Still not provided: more than one sound track, a track the decoder cannot read, and a
rate other than the device's.
