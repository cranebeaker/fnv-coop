# Movies (Bink): M1 working evidence

The opening quest starts the game with a movie. `VCG00` stage 0 runs
`PlayBink "FNVIntro.bik" 1 1 0 1`, then `SetStage VCG00 90` (the source
marks this "TEMP: DEMO CODE"; the burial scene of stages 17 onwards, with its
`TriggerScreenSplatter` calls, is never reached in the shipped game). Seen
with `nvinspect <Data> source VCG00` on the official data. The base game's
only movie is `Data\Video\FNVIntro.bik`.

## The file

`nvinspect Data\Video\FNVIntro.bik info` on the 1.4.0.525 install:

- Bink revision `i`, 1280 x 720, 8,692 frames at 10000000/333333 frames per
  second (30.00003), 289.7 s, 336.9 MiB.
- Video flags 0 (no alpha plane, not grey). One keyframe (frame 0).
- One audio track: 48000 Hz, two channels, the DCT variant, 16-bit, 8.8 MiB.

## Video decoder: `crates/bink`

A pure Rust Bink 1 decoder (no external crates, like `crates/mp3`).
`bink::Movie` reads the header, track list and frame index;
`bink::Decoder` decodes the video data of each frame in order into Y, U and
V planes.

Its fixed tables were read from the install's own `binkw32.dll`
(SHA-256 `c6c06e2d21f2179f0b7fbe829a4b17a7b371440d2c679253ec86b64d421185bd`,
image base 0x18000000), with the address of each in
`crates/bink/src/tables.rs`:

| Table | Address | How it was confirmed |
| --- | --- | --- |
| Coefficient scan order | 0x1804ca20 | A permutation of the 64 block positions (test). |
| 16 run-fill patterns | 0x1804ca68 | Read by the plane decoder at 0x1801aac0; each a permutation (test). |
| Huffman code lengths and codes | 0x1804ce70, 0x1804cf78 | All 16 trees are complete prefix codes when read from the low bit (test). The codes follow 8 bytes of padding. |
| Block-type run lengths 4, 8, 12, 32 | 0x180487c8 | Indexed from 0x180487bc (symbol minus 12) by the block-type reader at 0x18019860. |
| Intra and inter dequantization, 16 x 64 each | 0x180489a0, 0x180499c0 | Indexed by the inverse DCTs at 0x1801cb30/0x1801ce60 (intra) and 0x1801d230 (inter). |

From the DLL's inverse DCT (0x1801cb30, and the 16x16 and add variants at
0x1801ce60 and 0x1801d230), read in Ghidra:

- Each coefficient (16-bit) is multiplied by its table entry in natural
  row-major order and shifted right by 11 as the columns are read, with
  32-bit wrap-around. A column whose entries below the first are all zero
  is filled with the first.
- The 8-point transform uses 2896, 3784, -5352 and 2217 with `>> 11`.
- Each output is `(x + 127) >> 8` kept to its low 8 bits: no clamping.
  Inter blocks add that byte to the predicted pixel, also without clamping.
- The frame function (0x1801c990) decodes the alpha plane only with video
  flag 0x100000 and the chroma planes only without 0x20000.

The rest of the bitstream (bundles, their Huffman trees, block types,
coefficient and residue coding, plane order and 32-bit plane alignment)
follows the public descriptions of the format.

### Checked against the game's library

`research/nv-oracle`'s `nv-bink` decodes the movie with the install's
`binkw32.dll` and records the SHA-256 of each frame's Y, V and U planes
(`BinkCopyToBuffer`, YV12, `BINKCOPYALL`). `nvinspect FNVIntro.bik frames OUT`
writes the same from `crates/bink`.

On 2026-10-06, on the maintainer's machine, **all 8,692 frames of
`FNVIntro.bik` were identical in all three planes**, with no decode errors.
The DLL took about 44 s for the whole movie and `crates/bink` (release
build) 39.5 s.

Both hash files are private recordings and are not committed. Rendering a
dumped frame's planes as Y, then Cr, then Cb gave the expected colours, which
confirmed the plane order of the library's YV12 output.

Only revision `i` was compared. The decoder also accepts revisions `b` to
`k` (revision `k`'s whole-plane fill and the pre-`i` colour coding are
handled as publicly described) but those paths have no evidence here.

## Audio decoder: `bink::AudioDecoder`

Bink audio, DCT variant. From the DLL (read in Ghidra):

- Raw track interface: `BinkGetTrackData` (0x18012eb0) walks the frame's
  audio packets, takes the first u32 of the track's packet as the number of
  PCM bytes to produce, and runs the block decoder (0x18018800) until it
  has them; the last block's output may be cut short.
- Set-up (0x180185d0): frame length 512, 1024 or 2048 below 22050 Hz,
  below 44100 Hz, and otherwise. Band edges in units of two coefficients:
  `max(1, f * (N / 2) / ((rate + 1) / 2))` for the band frequencies at
  0x1805e710 (0, 100, 200 ... 15500 Hz; the list starts at 0) up to the first
  that is not below half the rate, then `N / 2`. Scale `2 / sqrt(N)`.
- Block (0x18018210): two unused bits; per channel two 29-bit leading
  coefficients (5-bit exponent, 23-bit mantissa, sign), each the mantissa
  times the float at 0x1805eb78 + 4 * exponent (2^(e - 23); entries 24 and up
  are not powers of two, see `audio.rs`); one 8-bit level code per band,
  looked up in the 256 floats at 0x1805e778 (1.1652^code, not clamped); then
  the coefficients (0x18017f20): a run flag (8 coefficients, or 8 times the
  entry of the run table at 0x1805e700 picked by 4 bits), a 4-bit width,
  and either zeros or width-bit magnitudes with a sign bit, times the
  current band's level. The block ends at the next 32-bit word.
- Each channel goes through an inverse DCT with weight 1 on coefficient 0
  (Takuya Ooura's `ddct(n, 1, ...)`, single-precision x87 code).
- Output (0x18017e40): sample times the scale, rounded by `FISTP` (to
  nearest even), saturated to 16 bits, interleaved.
- Overlap (0x18018800): the first N/16 samples per channel of each block
  but the first are crossfaded with the previous block's last N/16 in
  integers, `(prev * (n - i) + cur * i)` divided by `n` as an unsigned
  number (n counts interleaved samples), and each block outputs its first
  `N - N/16` samples per channel.

`crates/bink` follows all of this except the transform, which it computes in
double precision through a complex FFT; the DLL's single-precision x87
transform gives results that depend on the thread's precision control.

### Checked against the game's library

`nv-bink ... --audio 0 --precision 24|53` records the DLL's output through
`BinkGetTrackData` with the x87 precision control set to 24 or 53 bits (the
DLL left it unchanged). `nvinspect FNVIntro.bik audio OUT` writes the Rust
decoder's.

| Compared | Samples | Identical | Differ by 1 | Larger |
| --- | --- | --- | --- | --- |
| DLL at 24 bits vs DLL at 53 bits | 27,814,372 | 99.9799% | 5,588 | 0 |
| `crates/bink` vs DLL at 24 bits | 27,814,372 | 99.9825% | 4,863 | 0 |
| `crates/bink` vs DLL at 53 bits | 27,814,372 | 99.9847% | 4,269 | 0 |

Same length (289.733 s at 48 kHz, stereo) and never more than one step
apart, which is as close as the library is to itself under the two
precision settings. Which setting the game's decoding thread has is not
established yet (Direct3D 9 lowers it to 24 bits unless the device is created
with `D3DCREATE_FPU_PRESERVE`, and Bink may decode on its own thread).
Decoding the whole track takes 0.6 s.

## How the game shows a movie

From `FalloutNV.exe` 1.4.0.525 (Ghidra; addresses in that build) and the
Xbox 360 prototype's symbols where marked (Xbox PDB).

**`PlayBink`** (opcode 0x1114, handler `005d15d0`; the script table entry
confirmed with `NvLabelCommandTables`, stride 40) takes a string and four
optional integers with no names in the PC program. Their effects, from the
handler and the movie player it calls:

| Argument | Default | Effect | Evidence |
| --- | --- | --- | --- |
| 1 | 0 | The movie can be interrupted | Passed down to the per-frame continue check (`00867440`, the player's vtable `01082564` + 0x40), which ends the movie on control 5 or 28 only when it is set (or when a player flag at +0x20 is). |
| 2 | 1 | Game sound muted | Brackets the movie with the player's +0x4/+0x8 (`008671a0`, `00867220`: the sound system's pause and resume); the handler also fades or pauses sounds around the call. |
| 3 | 1 | Game music paused | +0xc/+0x10 (`00867270`, `00867280`). |
| 4 | 1 | Letterboxed | Picks the fit below; also skips a viewport change when clear. |

The Xbox symbols name the handler's locals `iinterruptable`,
`imuteGameAudio`, `ipauseGameMusic` and `iletterBoxed` (Xbox PDB); the
order above is the PC handler's (`005accb0` fills them in that order). The
opening calls `PlayBink "FNVIntro.bik" 1 1 0 1`: skippable, game sound
muted, music not paused, letterboxed. The PC build has no
`bAllowBinkSkipping` setting (the Xbox build's `General` setting).

**Playback loop.** The command does not return until the movie ends: the
player (`00ec2320`, then `00ec0e00`) opens the file from `Data\Video\`
(`00ec0ac0`, `BinkOpen`), and loops (`00ec1060`): when `BinkWait` says the
next frame is due, `BinkDoFrame`, copy, `BinkNextFrame`; then draw and
`Present`. It logs "Bink playback: Skipped frame #%i" when Bink skips one.
The loop ends after the frame counter reaches the frame count, or when the
continue check fails. An optional timeout (the player's float at +0x28)
makes the movie interruptible after that many seconds.

**Picture.** The frame goes through `BinkCopyToBufferRect` with surface type
3 into 256x256 `X8R8G8B8` textures (`00ebf8e0`, `00ec14d0`); an edge tile is
rounded up to a power of two (`00ebfc80`) and its spare texels stay black,
so the intro's bottom row of tiles holds 208 movie rows in 256. Each frame:
`Clear` to opaque black, then each tile as a quad of pre-transformed
vertices (`00ec0280`, FVF `XYZRHW|TEX1`, texture coordinates 0 to 1) with
linear min/mag/mip filtering and clamped addressing (`00ec0460`), alpha
blending, depth test and depth writes off (`00ebff30`).

Placement (`00ec2aa0`, `00ec2b20`, `00ec2bb0`): letterboxed, the scale is
screen width / movie width, x the screen's x offset, y (screen height -
movie height * scale) / 2; otherwise the scale is screen height / movie
height, x centres it and y is 0.

**Colours.** `binkw32.dll`'s 32-bit blitter: see `crates/bink/src/color.rs`.
`Decoder::to_bgrx` matches the DLL on all 8,692 frames of the intro
(`nvinspect FNVIntro.bik frames OUT bgrx` against `nv-bink --surface 3`).

**Sound.** The game imports `BinkOpenDirectSound` and `BinkSetSoundSystem`
but not `BinkSetVolume`, so movie sound plays at Bink's default loudness.

## The viewer

`viewer/src/movie.rs` plays `PlayBink` movies: the game's clock stops and
input is withheld while the movie has the screen; tiles, filtering,
placement and colours as above, drawn by a 2D camera over everything
(order 100); sound from `bink::AudioDecoder` at full loudness; the game's
sounds and music paused when the movie asks; control 5 (E) or 28 (Esc) ends
an interruptible movie. `--no-movies` skips them; screenshots skip them
unless `--movies` is given.

Checked 2026-10-06: `--new-game --movies --screenshot movie84.png --wait
84` at 1920x1080 shows frame 2478. Compared with a model of the game's
drawing of the DLL's frame 2478 (256 tiles, bilinear in stored bytes, D3D9
pixel centres; `compare_screen.py` in the private tree): 81.5% of colour
values identical, 99.3% within 2 levels, mean difference 0.22, largest 35,
about +0.2 brighter on average. The difference comes from filtering: the
viewer interpolates between texels in linear light (sRGB textures), the
game in stored bytes, which differs most across sharp edges.

## The engine's platform layer

The translated engine reaches `binkw32.dll` through its imports, which
`crates/engine-core/src/units/platform/bink.rs` answers with `crates/bink`
(a platform layer like D3DX: written from the documented meaning of each
call and matched against the player's DLL, whose code is not read). The
game imports nine: `BinkOpen`, `BinkClose`, `BinkDoFrame`, `BinkNextFrame`,
`BinkWait`, `BinkPause`, `BinkCopyToBufferRect`, `BinkSetSoundSystem` and
`BinkOpenDirectSound` (only its address is passed on). It reads `Width`,
`Height` and `Frames` (+0, +4, +8) of the handle.

Measured on 2026-10-23 with `nv-bink --game`, `--wait` and `--sound-system`
(the 1.4.0.525 install's DLL, sound off):

- The handle starts `Width`, `Height`, `Frames`, `FrameNum` (1 after
  `BinkOpen`, back to 1 after `BinkNextFrame` on the last frame),
  `LastFrameNum` (`0xffffffff`, then the frame decoded last), `FrameRate`,
  `FrameRateDiv`, 0, the open flags, 0 and the file's size field.
- `BinkDoFrame` and every `BinkCopyToBufferRect` answered 0, also with the
  frame late. A tile below the movie's bottom gets only the movie's rows
  (the intro's last tile row: 208 of 256). Copying with or without
  `BINKCOPYALL` into tiles that keep the previous frame gave the same tiles
  for all 8,692 frames.
- **All 8,692 frames, all 15 tiles each, are identical** between the layer
  and the DLL when copied as the game copies them.
- `BinkWait` answers 0 for the first frame; the first `BinkDoFrame` starts
  the schedule (frame `n` due `floor(n * 1000 * FrameRateDiv / FrameRate)`
  ms later: 33, 66, 99, 133 ...); a frame more than two frames late starts it
  again; a pause moves it on by the paused time.
- `BinkSetSoundSystem(BinkOpenDirectSound, 0)` answered 0 in a process
  without a window. The platform has no sound output, so the layer answers
  0 and movies play silent, timed as above.

The intro's audio decoded frame by frame with `bink::AudioDecoder` has the
DLL's length for every frame; 5,908 of 8,692 frames are identical to the
DLL at 53-bit x87 precision (5,660 at 24-bit), every other sample within one
step (the totals of the audio table above).

## Not done yet

- Filtering in stored bytes (a custom sprite material) would remove the
  remaining difference above; cosmetic, deferred.
- The handler's sound fade around the call (`005d1720` with 6000, and
  `00ad7230`), and the viewport change when not letterboxed (a global at
  `011f9426`), are not reproduced.
- What sets the player's +0x20 flag (interrupt even when the script says
  not) is not traced.
- Which x87 precision the game's thread decodes audio at is not
  established (see the audio table).
- Movie sound in the engine: the platform has no sound output yet, so its
  Bink layer opens no sound system. With sound the DLL times the picture by
  the sound it plays, which is not measured.
