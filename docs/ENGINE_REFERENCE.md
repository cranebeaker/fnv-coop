# nv-rs: engine reference

An engine reimplementation of Fallout: New Vegas in Rust, in the spirit of
OpenMW: original engine code that loads the game's data at runtime from
the user's own install. No game files are ever committed or redistributed.
A personal project; VR is on the roadmap (see `docs/VR.md`) but there's no
headset yet. `README.md` is the full user-facing documentation and lists
what's been verified against the game.

## How the user wants this done

- **1:1 with the game, read from the game.** Everything should come from
  the game's own files and code, converted exactly: record values, mesh
  and shader settings, and the game's actual formulas (from its compiled
  shaders in `Data\Shaders`), not tuned by eye. The aim is for the user
  to need as little visual checking as possible. When something can't be
  read from the game yet, say it's a guess, mark it as one in the code,
  and list it below until it's confirmed.
- **No stand-ins, nothing new.** Only what's traced in the exe (Ghidra),
  the data or a recording gets implemented, as the game does it. A menu,
  effect or behaviour that can't be done faithfully yet is left out and
  listed, never drawn as a placeholder (the user rejected V.A.T.S. with a
  text-panel menu and normal-speed playback for this). Menus come from the
  game's own menu files through `crates/ui`.
- Explain things in plain terms. Say what changed and what's still
  uncertain.
- The game itself runs on this machine, so in-game screenshots are the
  final reference for anything that can't be read from the files.

## The user's machine

- Windows 11, PowerShell. GPU: NVIDIA RTX 5070 Ti Laptop (Vulkan).
- Project: `%USERPROFILE%\OneDrive\Desktop\nv-rs` (OneDrive itself is
  disabled; the path is just where the Desktop lives).
- Game Data folder: `C:\Games\Steam\steamapps\common\Fallout New Vegas\Data`
- The test cell for everything so far: `GSDocMitchellHouse` (Doc
  Mitchell's house, Goodsprings).

## Building and checking

```powershell
cargo test --workspace            # everything except the viewer
cargo clippy --workspace --all-targets
cargo fmt --all
cargo build --release             # nvinspect -> target\release\nvinspect.exe

cd viewer                         # the Bevy viewer: its own workspace
cargo run --release -- "C:\Games\Steam\steamapps\common\Fallout New Vegas\Data" GSDocMitchellHouse
```

Keep tests passing and clippy and rustfmt clean. The viewer prints notes
before its window opens (lights, lighting, image space values); read them.

**The user's play copy**: `%USERPROFILE%\OneDrive\Desktop\nv-rs-play`
(launchers `1 New game.cmd` … `5 Any place.cmd`, `play.cmd`, `READ ME.txt`,
`WHATS-NEW.txt`), separate from the project so builds never touch it.
After each merge batch is built and checked, publish it:
`powershell -File %USERPROFILE%\nv-re\work\publish-play.ps1 -Notes
"change", "change"` (waits in `next\` if the copy is running). The user's
F12 reports land in `nv-rs-play\reports\NNN\` (picture, spot, hour, state,
their note): reproduce each in the game (its `report.txt` has the console
lines), record or screenshot it, and compare.

`nvinspect` is the main tool for looking at real data. Useful commands
(target first, then command):

```powershell
$D = "C:\Games\Steam\steamapps\common\Fallout New Vegas\Data"
.\target\release\nvinspect $D cells
.\target\release\nvinspect $D show GSDocMitchellHouse      # any record's subrecords
.\target\release\nvinspect $D nif dlc04\clutter\lights\dlc04lightwall01on.nif
.\target\release\nvinspect $D render-cell GSDocMitchellHouse out\doc   # PNG plan + view, objects list
.\target\release\nvinspect $D walk GSDocMitchellHouse                  # walk the player through the collision
.\target\release\nvinspect $D worlds                                   # every worldspace
.\target\release\nvinspect $D land WastelandNV -18 0                   # one outdoor square: terrain, textures, weather
.\target\release\nvinspect $D grass WastelandNV -18 0                  # one square's grass: spots, blades, models, where most grows
.\target\release\nvinspect $D levels 4                                 # XP table, kill rewards, the perks offered at a level
.\target\release\nvinspect $D face DocMitchell                         # head parts' .tri morphs: which the game uses
.\target\release\nvinspect $D lip sound\voice\falloutnv.esm\maleuniquedocmitchell\vcg01_greeting_00107222_1.ogg   # a line's lip sync timing
.\target\release\nvinspect $D walk Goodsprings                         # outdoors: 3x3 squares with terrain
.\target\release\nvinspect $D collision                                # every model's Havok blocks, layers, scales; placed primitives
.\target\release\nvinspect $D scripts                                  # parse every script, compare with the compiled form
.\target\release\nvinspect $D scripts --grep "setstage vcg01"          # script lines mentioning something
.\target\release\nvinspect $D functions                                # functions the game uses, not carried out yet
.\target\release\nvinspect $D vats DocMitchell WeapNV9mmPistol 500     # V.A.T.S.: AP, cost, each part's chance
.\target\release\nvinspect $D source VCG01                             # a record's scripts + compiled form
.\target\release\nvinspect $D scripted GSDocMitchellHouse              # a cell's scripted objects and triggers
.\target\release\nvinspect $D play 40 VCG01 0                          # run quest scripts headless
.\target\release\nvinspect "$D\Shaders\shaderpackage019.sdp" dump      # every shader as text
.\target\release\nvinspect $D lockpick 001164C0 50                     # a lock: sweet spot, rings, pin health, forcing, the scene
.\target\release\nvinspect $D coverage records --official   # record/subrecord census vs nv-rs
.\target\release\nvinspect $D coverage files --official     # every file, NIF block type, format
.\target\release\nvinspect $D coverage functions --official # script functions used vs carried out
```

## Layout and conventions

- `crates/esm` plugins and load order; `bsa` archives; `nif` meshes; `dds`
  textures (plus PNG and deflate writers); `assets` file lookup across
  loose files and archives; `world` cells and placed objects; `preview`
  the CPU renderer behind `render-cell`; `cellview` turns a cell into
  renderer-ready data for the viewer; `shaders` reads `.sdp` packages and
  disassembles Direct3D 9 bytecode; `physics` the collision triangles and
  the walking capsule; `script` the scripting language; `nvinspect` the
  CLI; `testdata` builds test files (`room` an interior, `outdoors` a
  two-square worldspace with terrain, weather and a door to a shack,
  `quests` a quest with stages and scripts, a greeting, and a cell with
  a scripted person and a door that starts disabled).
- Outdoors: `world::exterior` (worldspaces, the grid of squares,
  persistent objects), `world::land` (`LAND` terrain and `LTEX`
  textures, meshes per quarter), `world::weather` (`WTHR`, `CLMT`,
  lighting a square); `cellview::Game` keeps the plugins and archives
  open and loads squares (`load_square`); the viewer streams them in
  `viewer/src/exterior.rs` and draws terrain with `terrain.rs` +
  `terrain.wgsl`. Grass: `world::grass` (records, the game's placement),
  `cellview::grass` (models, one mesh per grass and square),
  `viewer/src/grass.rs` + `grass.wgsl`. Walking and doors are in
  `viewer/src/walk.rs`.
- `mp3` the game's music: an MP3 decoder (MPEG-1/2/2.5 Layer III) with a
  WAV writer, no outside library: joint stereo, free format, checksums,
  the bit reservoir, tags skipped, LAME delay/padding trimmed. Checked
  against Windows' Fraunhofer decoder (which the game plays music through,
  via DirectShow) on all 199 tracks: every sample within one 16-bit step,
  same timing; the tables value for value against Windows' codecs. Music
  files: 48 kHz 192 kbit/s stereo (Fallout 1/2 tracks 44.1 kHz 128).
  `nvinspect "<Data>\Music" check`, `nvinspect <file.mp3> info|check|wav`.
- Faces: `nif::tri` FaceGen morph files (`.tri`); `world::face` faces that move
  (channels, morphs, the key queues, blinking), `world::lip` lip sync
  files; `viewer/src/faces.rs` moves the head parts' vertices.
- INI settings: `assets::IniSettings` (`Fallout_default.ini`, then the
  user's `Fallout.ini` and `FalloutPrefs.ini`, later winning),
  `cellview::Game::settings`.
- AI movement and talk: `world::movement` (turning, walking turns, path
  progress and arrival, `IsWithinDistance` reach, package clock, avoidance,
  process levels), `world::social` (greetings, idle chatter, conversations
  between people), `viewer/src/chatter.rs` (lines said without the menu);
  `testdata::ai` its test world.
- Combat AI: `world::combat_ai` (the rules: combat styles, noticing,
  ranged band and fire timing, melee approach and roll, losing the target),
  `viewer/src/fighting.rs` (detection runs, fights and flight, measuring
  and moving only); `testdata::fighting` its test world.
- V.A.T.S.: `world::vats` (the rules: action points, costs, chances, the
  queue), `world::vats_camera` (camera paths and shots, playback, the
  menu's eased view), `ui::vats` (the game's menu, `vats_menu.xml`),
  `nif::camera` (the shots' animated cameras), `viewer/src/vats.rs` (V:
  the menu, time stopped, playback through the camera shots; `--vats [N]`
  opens it by itself and plays N attacks), `nvinspect <Data> vats <ID>
  [WEAPON] [DISTANCE]`, `nvinspect <Data> vats-cameras [ID]`,
  `nvinspect <Data> menu vats`.
- Menus: `crates/ui` the game's menu system (`menus\*.xml` read and
  worked out as the exe does: `xml` file cleaning, includes, tokens;
  `tile` the tile tree and trait operators; `font` + `text` the `.fnt`
  fonts and layout; `draw` what's drawn; `hud` + `xp` the HUD's code;
  `anim` trait animations). `nvinspect <Data> menu hud` (or any menu file)
  prints the worked-out tree and draw list, `nvinspect <Data> font 7` a
  font. Viewer: `viewer/src/hud.rs` + `hud.wgsl` (the HUD's picture,
  laid over the scene by `grade.wgsl`'s last step); `--no-hud`.
- The game's own menus (`ui::menu`, `ui::list`, `ui::menus::*`;
  `viewer/src/game_menus/`): `menu::Interface` is the interface manager's
  part (`0070c4a0`: the tile under the pointer, mouse over/out, clicks,
  drags, the wheel, keys, `_PCButton_<key>`, the arrows/Enter as special
  codes), `menu::MenuCode` a menu class's virtual functions (SetTile,
  HandleClick, HandleMouseover…), `list::ListBox` the game's list box (the
  same exe code as the Pip-Boy's `ui::listbox`, with what these menus need:
  filter, sort, remove, the failed-check highlight; the two aren't unified
  yet). Each menu is a module following its class's code: `message` (1001),
  `dialog` (1009), `container` (1008), `quantity` (1016), `barter` (1053),
  `levelup` (1027), `traits` (1084), `chargen` (1048, tag skills),
  `textedit` (1051, the name). Their requests (moves, trades, skill
  points, tags, traits, the name, sounds) go back to the viewer, which
  carries them out in the world. The exe's own defaults for text settings
  the plugins lack: `ui::game::EXE_TEXT_SETTINGS`. Clip windows:
  `ui::draw::{clips, clip_rect, pixel_clip}` (drawing and picking alike).
  The HUD under any menu: `ui::hud::parts_for_menu` (the game's menus) and
  `mask` (V.A.T.S.), both applied by `Hud::apply_mask`. Viewer: `--open-menu
  container:REF | barter:REF | quantity:N | levelup | levelup:perks`,
  `--menu-pointer X,Y` (a 1920 × 1080 picture's pixel) for screenshots;
  scripts' menus with `--run` (`--run "ShowTraitMenu"`, `--run
  "SetTagSkills 3 1"`, `--run "ShowNameMenu"`). Findings:
  `findings\menus.md`.
- The Pip-Boy (`ui::pipboy`, `viewer/src/pipboy.rs` + `pipboy.wgsl`): the
  three menus `stats_menu.xml`, `inventory_menu.xml`, `map_menu.xml` filled
  from the game's state (`ui::pipboy::gather`), list boxes
  (`ui::listbox`) and tab lines (`ui::tabline`) as the exe builds them.
  `nvinspect <Data> pipboy <stats|items|data> [PAGE] [ITEM...]`; viewer Tab,
  `--pipboy stats:1` for screenshots.
- Lockpicking: `world::lockpick` (the rules: sweet spot, stages, forcing,
  what a pick does), `ui::lockpick` (the menu file's tiles), `cellview::
  lockpick` (the 3D scene, `nif::lights` for the model's lights),
  `viewer/src/lockpick.rs` (`--lockpick REF` tries a lock once loaded),
  `nvinspect <Data> lockpick <REF|LEVEL> [SKILL]`.
- Script functions beyond the core set: `world::script_functions` (`value`
  for reads, `change` for the rest, state in `GameState.set_by_scripts`,
  saved); `nvinspect <Data> functions` counts `scripting::HANDLED` and
  `script_functions::handled()`. The function table is at `01190910` in
  the exe (40-byte entries, handler +0x18, condition version +0x20); the
  whole table with handlers is `%USERPROFILE%\nv-re\decomp\functions\
  table.txt`, notes in `findings\functions.md`.
- Script functions, second round: `world::more_functions` (`value`,
  `change`, state in `GameState.more`, saved; `procedures` the exe's AI
  procedures, `challenges` `CHAL`, `placed` references made by `PlaceAtMe`,
  `destruction` `DEST`). What the world can't see (people's movement flags,
  last idle, procedure) the viewer reports (`more_functions::report`).
- Particles: `nif::particles` (every particle block, read as the exe's
  loaders do; sequences' particle tracks), `world::particles` (the
  simulation), `cellview::particles` (per placed object: what plays, how
  it's drawn), `viewer/src/particles.rs` + `particles.wgsl`;
  `testdata::particles` (the barracks dust as a NIF). `nvinspect <Data>
  particles [PATH [run SECONDS]]`; findings `findings\particles.md`.
- The crates use no outside libraries. Only `viewer/` uses one (Bevy),
  which is why it's a separate workspace.
- `viewer/` is pinned to **Bevy 0.16**. Check APIs against the 0.16 docs
  (docs.rs/bevy/0.16.1), not newer ones. Custom rendering there:
  `lighting.rs` + `game_lit.wgsl` (the game's lighting, as an extension of
  `StandardMaterial`), `grade.rs` + `grade.wgsl` (the image space pass).
- Tests build every input from scratch (plugins, NIFs, DDS, shader
  bytecode) in temp folders; `testdata::room` is the shared test cell.
  Add a test that fails without each fix.
- Game-specific rules live in `world` / `preview` / `cellview`, so the
  viewer stays a thin layer. Doc comments say what each rule is and how
  it was confirmed.
- Messages and docs are plain English for the user, not jargon.

## Confirmed against the game

- Rotation: XYZ order, clockwise angles: `R = Rx(-x) * Ry(-y) * Rz(-z)`
  (the bathroom mirror `RestroomMirror03` at 270,270,0 sits flush on the
  wall only this way). Heading is clockwise from north.
- The game ignores a placed model's top-node (root) transform
  (`placed_scene`); confirmed with the `NVCraftsman*` room pieces.
- Enable parents: a reference follows its parent (or the opposite) and its
  own "initially disabled" flag is then ignored.
- Arrival: the `COCMarkerHeading` marker, else the far side of a door's
  `XTEL`. Doc Mitchell's starts facing down the hallway, as in game.
- **Placed lights** (read from the game's own shader constants, see
  "Recording the game"): radius = base `LIGH` radius **+** the
  reference's `XRDS` (27.23 → 227.23 exactly); color = base color ×
  fade × the color of the light the reference's `XEMI` names (that
  light's fade not used). Doc Mitchell's hallway lights almost all point
  at `DLC01FXLightMillSmoke` (255,162,85): that's its orange.
  `world::Placement::placed_light`.
- Vertex colors are used whenever a mesh has them, lit or not, whatever
  the vertex-color flag says (lit walls with the flag off get the
  shader's vertex-color toggle on; the rug without colors gets it off).
- The directional light shines along its two angles: Doc Mitchell's
  270° up is straight down (the game sends (0,0,1) toward the light);
  the barracks' 0°/0° gives (−1,0,0) on an unrotated floor piece, so the
  around angle counts from **east** (+x), not north; positive angles turn
  it clockwise seen from above (the game's code, `00641830`).
  `world::Lighting::toward_directional`.
- An Emittance (`XEMI`) naming a region gives, every frame (`00551890`),
  the region's rolled weather's "Sunlight" color (`NAM0`, 10 colors × 6
  times; `DefaultWeather` when it has none rolled) blended for the hour as
  the sky's colours are (high noon included, the sky's climate's times),
  each channel at most 1, written to the region at +0x2C. The game sent
  (255,227,170) for `NVInteriorRegion` (`NVWastelandInterior`: the same
  by day and at high noon). `world::weather::EmittanceNow`; the viewer
  recolours these glows each game minute (`viewer/src/emittance.rs`).
- No-lighting surfaces (`NOLIGHTTEXVC`, `NOLIGHT006.vso` and kin):
  color = texture × vertex color × `MaterialColor`, where `MaterialColor`
  is white, or for meshes flagged external emittance (no-lighting ones
  too) the Emittance color × glow multiplier (beams (1, 0.89, 0.667), a
  window panel ×10). Opacity × a fade by viewing angle: a smooth
  S-curve `3t² − 2t³` between the falloff's start and stop angles (not
  linear). Fog: added effects fade to black (`1 − fog`), multiplied ones
  to white, the rest toward the fog color (the last two from `Toggles`
  values, not from a draw of each kind).
- **The game blends in stored (gamma) values**, not linear light: its
  frame buffer holds the shaders' stored outputs. The viewer now writes
  stored values and blends them, and the image space pass decodes at the
  end (`grade.wgsl` runs for every camera). Blending in linear light made
  the faint light beams about 6× too strong over dark walls.
- The game's camera clips at 5 and 5000 units (from a recorded
  projection matrix in Doc Mitchell's house; the far plane may follow the
  fog's far distance).
- Self-lit (emissive): material emissive × emissive multiplier is added to
  the light on the surface, masked by the glow map (texture set slot 2),
  then multiplies the diffuse texture. Shader flag 0x20000000 ("external
  emittance") replaces the color with the reference's `XEMI` color (a
  light's, a region's now) × the multiplier. **With no `XEMI`** the
  game's code falls back to the player's weather region (`005453b0`,
  player+0x760); with none, the mesh's own colour. In the recording that
  gave `DefaultWeather`'s day Sunlight (244,206,149) × 0.618 × 5 =
  (2.957, 2.496, 1.805) on the "on" wall lamps: `DefaultWeather`'s high
  noon is black, so its sunlight is the day's × w, w falling to 0 at noon
  and back to 1 at sunset's begin; 0.618 (rising 0.604 → 0.618 in about
  12 s) is 15:43 with `NVDefaultClimate`, the player's region having no
  weather of its own (`findings\weather.md` §6). Reproduced with
  `--weather-region VMapGoodspringsRegion --run "set GameHour to 15.72"`
  (lamps within 4% of the frame). `LoadedCell::emittance_color`.
- Alpha test + blend together: cut out, then blend (glass, fabric).
- Image space: cell `XCIM` -> `IMGS`. New Vegas's `DNAM` is 132 bytes with
  the cinematic values at float index 24: saturation, contrast average,
  contrast, brightness, tint RGB, tint amount (index 25 in the 152-byte
  variant). Doc Mitchell's uses `ShackInterior01`: saturation 0.9, tint
  0.69,0.56,0.30 at 0.5, brightness 1.1, contrast 1.2 around 0.14.
- New Vegas has **no eye adaptation** (stated by the user).
- Doc Mitchell's lighting: ambient 31,31,45, directional 32,30,21 at 35°
  around / 270° up (partly from `EnclaveLightingTemplate01`).

### From the game's shaders

The game uses **`shaderpackage013.sdp`** on this PC (`Shader Package: 13`
in `Documents\My Games\FalloutNV\RendererInfo.txt`; render path
`BSSM_SV_2_A`, HDR on, bloom lighting off). The `.sdp` reader reads it
correctly (1,007 shaders). `nvinspect <pkg> dump <folder>` writes into the
current folder by default, so give it a folder outside the project.

- Lit surfaces (`SLS2001`, `SLS2004`, `SLS2011`, `SLS2029`–`SLS2036`):
  `light = AmbientColor + EmittanceColor × GlowMap + Σ color × saturate(N·L)
  × atten`, then `max(light, 0) × texture × vertex color`. Point light
  `atten = 1 - saturate(d² / r²)` (`PSLightPosition.w` is the radius; the
  attenuation-texture variants encode the same thing). The directional
  light has no falloff. `SLS2029` does the directional light + up to 5
  point lights in one pass; `EmittanceColor.w` counts the directional
  light too, so with 5 only 4 point lights are used and the fifth
  `PSLightPosition` slot is leftover data from an earlier draw (it
  doesn't match any light in the cell; don't mistake it for a missing
  light). The vertex shader (`SLS2020`) normalizes each light's direction
  per vertex; the pixel shader renormalizes the interpolated vector.
- All of it is on stored (gamma-encoded) texture values; nothing is
  converted to linear. The viewer encodes with the exact sRGB curve,
  does the game's math, writes stored values (blending happens on them,
  as in the game), and the image space pass decodes at the very end.
- The normal is never flipped for back faces (no face register in any lit
  shader), so two-sided surfaces aren't lit from behind.
- The lit shaders don't use a material diffuse color, only the vertex
  color's RGB (`Toggles.x` switches vertex colors on). Alpha is texture
  alpha × `AmbientColor.w`. `Toggles` = (vertex colors, fog, glossiness,
  alpha test reference).
- Surfaces with many lights are drawn in several passes: a first pass
  with ambient + some lights (fog on), then light-only passes adding more
  lights (fog off), and the texture applied on top. Every light that
  reaches the surface counts, as in the viewer.
- Normal maps (all `SLS2xxx`): `n = normalize(stored × 2 − 1)` in the
  frame (U, V, vertex normal); it replaces the normal for every light.
  The meshes' stored tangent arrays are swapped relative to their names:
  the first runs along the texture's V, the second along U (measured on
  every mesh checked; `nvinspect nif` prints it per mesh). Red along U,
  green along V is the DirectX layout; drawn now in the viewer.
- Specular (separate additive pass, `SLS2047`–`SLS2056` and one-pass
  variants only in the recordings): Blinn-Phong,
  `pow(saturate(N·H), Toggles.z) × normal map alpha × light color`
  (× atten for point lights), `H = normalize(normalize(L − P) +
  normalize(eye − P))` per vertex (`SLS2048.vso`), the fade's `N·L` exact
  per pixel (the light vector is passed unnormalized), and below
  `N·L = 0.2` it's also × `saturate(N·L + 0.5)`; each pass's sum (up to
  3 lights) is saturated. Only meshes with shader flag 0x1 get it.
  **No material specular colour**: the recorded `PSLightColor` in the
  specular passes is the plain light colour, and the room pieces' NIFs
  store specular 0,0,0 (the viewer had been multiplying by it, so they
  had no highlights at all). `Toggles.z` = the NIF material's glossiness
  (10 on the room pieces). Drawn in the viewer, added before fog.
- **Per-vertex values, ported**: the lit vertex shaders (`SLS2020` and
  kin) normalize each point light's direction per vertex (attenuation is
  per pixel); specular half vectors, fog and the no-lighting fade by
  viewing angle are per vertex too. The viewer draws meshes unindexed with
  each triangle's three corners on every vertex (`lighting::
  ATTRIBUTE_CORNER_A`..`C`, `GAME_CORNERS`), works these out at the
  corners and blends them with the pixel's barycentric weights. Light
  directions are blended in world space (the game blends them in each
  corner's tangent frame; the same on flat pieces).
- Shadow maps exist: a multiplicative pass (`SLS2089.vso` + a pixel
  shader only in the recordings, `ShadowMap` s6, 9 taps) on surfaces near
  actors. Not ported (actors aren't drawn); it didn't change the measured
  regions.
- Image space (`ISHDRBLENDINSHADERCIN`), on stored values, after bloom is
  added: `lum = dot(c, 0.299/0.587/0.114)`; `c = lerp(lum, c, saturation)`;
  `c = lerp(c, lum × tint, tint amount)`; `c = contrast × (brightness × c
  - contrast avg) + contrast avg`; then the fade color. The viewer's grade
  pass already matched this.
- No tone-mapping curve: the final pass only scales the scene by
  `HDRParam.x / max(bloom alpha, HDRParam.x)` (≤ 1) and adds bloom × 0.5 /
  the same max. So nothing lifts dark corners (the CPU renderer's 0.12
  ambient floor was removed). Recorded values in Doc Mitchell's:
  `Cinematic` (0.9, 0.14, 1.2, 1.1), `Tint` (0.69, 0.561, 0.302, 0.5),
  exactly the record's values in the viewer's order; `HDRParam.x` = 1.
- **Bloom, ported** (`grade.rs` / `grade.wgsl`, from the recording and
  the shaders): the scene is drawn into a 1920×1080 FP16 target (4×
  MSAA, resolved). `ISHDRDOWN4` ×4: 480×270, 120×67, 30×16, 7×4, each
  pixel the mean of 4 bilinear taps ±1 source texel diagonally.
  `ISHDRDS4ADAPT` → 1×1: the 4 corner texels of the 7×4 (offsets ±1 in
  UV, clamped), lerped with the previous average by `1 − 0.3^TimingData.z`,
  length clamped to [0.01, `HDRParam.w` = 1]. `ISBPBLUR13` at 480×270,
  vertical: `max(c − 0.9, 0) × 2.4` per tap, 13 Gaussian taps (σ =
  radius / 2, radius 6 = the recorded weights 0.01854…0.1370), alpha =
  average's r + g + b. `ISBLUR13` horizontal, alpha from the last tap.
  `ISHDRBLENDINSHADERCIN`: Src0 = bloom (its alpha is the `a` above),
  DestBlend = scene; a third texture (the average) is bound but unused;
  last, `lerp(c, Fade.rgb, Fade.w)` (fade to a colour, 0 normally; not
  ported). The image space record's floats (132-byte `DNAM`): 0 eye adapt
  speed 0.3, 1 blur radius 6, 2 blur passes 4, 3 emissive mult 1, 4
  target lum 1, 5 upper lum clamp 1, 6 bright scale 2.4, 7 bright clamp
  0.9, 8 lum ramp no-tex, 9/10 lum ramp min/max, 11 sunlight dimmer 1.4,
  12 grass dimmer, 13 tree dimmer (then the loader's inserted 1.0).
- Fog: `FogParam` = (far, far − near, power) and `FogColor` confirmed from
  the recording (Doc Mitchell's: (5000, 4936, 0.6), (0.286, 0.267,
  0.188)).
- **Reflections (environment maps), ported** (`preview::cell::Environment`,
  `game_lit.wgsl` `reflection`): every lit mesh with shader flag 0x80 gets
  an additive pass (ONE/ONE, depth-equal, after the light passes):
  `SLS2057`, or `SLS2058` (view direction turned around) for flag
  0x200000 "window" meshes. Its vertex shader (only in the recording)
  builds the normal as `0.1 × (n.x U + n.y V) + n.z N` (the normal map
  tilts reflections a tenth as much), the view vector normalized per
  vertex, `R = 2(N·V)N − V` in world axes (`ObjToCubeSpace`), and
  `1 − fog`. Colour = cube(R) × mask × `EnvToggles.z` × `AmbientColor.w`
  (× vertex colour when the mesh has them, `EnvToggles.x`) × (1 − fog).
  Mask = slot 5's red when present (`EnvToggles.w`), else the normal map's
  alpha. `EnvToggles.z` = the shader property's environment map scale
  (0.5 on the camera). Cube map = slot 4, else **`textures\effects\
  reflection.dds` on all six faces** (a flat 64×64 file the game copies
  onto a cube at startup; byte-for-byte from the recording). Every
  reflection-flagged mesh in view got its pass (pans, frames, trim,
  windows); `nvinspect nif` now prints the reflection strength.
- **Emissive HDR multiplier** (image space float 3, `fEmissiveHDRMult`):
  multiplies lit self-lit glows. Barracks: the tubes' 1.0 × 1.3 arrived
  as 3.9 (added into `AmbientColor` for a mesh with no glow map). Not
  applied to no-lighting effects (beams stayed (1, 0.89, 0.667)).
- Bloom values, settled by the barracks (`OfficeDefaultImageSpace`,
  where they differ): the average's clamp is the upper lum clamp (2),
  the final limit the target lum (1), the bright pass (0.35, 1.5); its
  blur radius 8 was drawn with 15 taps (radius 7, σ 3.5): the game has no
  wider blur shader, so the viewer caps it at 7.

### How placed models are drawn

Read from the recordings' render states (`D3DRS_ZENABLE`,
`ZWRITEENABLE`, `ALPHABLENDENABLE`, `ALPHATESTENABLE/REF/FUNC`,
`DEPTHBIAS` at each draw, matched to meshes by vertex count with
`nvinspect <Data> meshes <CELL>`) and the game's code.
`preview::cell::{depth_of, alpha_of, is_drawn, DrawState,
placed_sequences, MeshMotion, Billboard}`, `viewer/src/lighting.rs`
(`DrawKey`, `specialize`), `game_lit.wgsl`.

- Depth: every scene draw is depth-tested (no recording had the test off;
  the first flag set's 0x80000000 "z test" isn't followed: the
  distant-object blocks lack it and are tested). Depth is written by every
  piece except decals (first set 0x04000000 / 0x08000000: all 18 decal
  draws without, though their flag is on), and by **blended** pieces only
  with the second flag set's 0x1 (the ceiling fan's blades, windows, glass
  bulbs, lamp glow cards: all written; the light beams and effects
  without it: not); distant-object blocks (second set 0x4) always write.
  Opaque pieces write whatever the flag says: the BOS water crate
  (`nv_cratebosgeneric01.nif`, second set 0x8000, no 0x1) was drawn with
  `ZWRITEENABLE` on at Goodsprings (call 152845241: scale 0.65, lying on
  its side at −68537.9, 4365.0, 3725.7 units up the view: the only
  match), and following the flag made it see-through (report 002). The
  code: `BSShaderProperty::LoadBinary` (`00ba92c0`; slot 19 of the
  vtable at `010ae0d0`, slot 20 being `LinkObject`) reads the two flag
  words as stored (0x2000 in the second also sets 0x8000) and the lit
  shader's per-draw setup (`00bc9a50`) turns `ZWRITEENABLE` off when the
  property's runtime second set lacks 0x1 (`00b97e30` → `SetRenderState(0xe)`;
  `00bc8d90` restores it), so something sets 0x1 on opaque pieces after
  loading; what, isn't traced. The fan looked see-through because its
  blended blades didn't write depth. No New Vegas file has an
  `NiZBufferProperty`.
- Decals: `D3DRS_DEPTHBIAS` = the float with bits 0xB7A7C5AC (−0.00002)
  for every decal draw, 0 otherwise. The game's projection `z = A − B/d`
  (indoors B = 5.005005: near 5, far 5000; outdoors at Goodsprings
  1.0001 / −35.38754) makes that a decal at d drawn as at d' with
  `1/d' = 1/d − bias/B`; the viewer adds `near × 0.00002 / B` to Bevy's
  reverse depth. Decals are drawn before the other blended surfaces.
- Blended surfaces are drawn after the opaque ones, farthest first by the
  centre of each **mesh's** stored bounding sphere (not the model's
  origin: the window panes and light beams in Doc Mitchell's house came
  in that order).
- Alpha: the `NiAlphaProperty`'s own comparison and threshold, exact
  (Greater, not Bevy's ≥), on the final alpha (after the fade by angle).
  No alpha property but the "dynamic alpha" shader flag (0x00080000):
  blended (SrcAlpha, InvSrcAlpha), as recorded on Goodsprings' window
  glow cards. Distant-object blocks: alpha test Greater 128 (recorded).
- Shapes without one of Bethesda's shader properties aren't drawn (bone
  boxes, the Nevada flag's eleven, a footlocker's latch helper: the Doc
  Mitchell recording draws the footlocker's box and lid, nothing else).
- Placed objects' animation (`005659f0`, `0047aec0`, `0047a490`,
  `0047aa40`): `Idle` (only if looping) and `SpecialIdle` play from the
  start (the ceiling fan, the windmill); activators, terminals,
  containers and doors with `Open` and `Close` play `Close` (the
  footlockers' lids were recorded closed); otherwise the first sequence
  is set going and the animation switched off at once: its first frame
  holds (Goodsprings' mailboxes' flags recorded down). `PlayGroup` starts
  a sequence on a placed object.
- Rotation curves (key type 4, separate X/Y/Z angle curves) with
  quadratic keys store the tangents in, then out: the fan's spin, 0 to
  −62.83 rad over 10 s, has (0, −62.83) and (−62.83, 0), an even turn a
  second only when read that way; played as cubic Hermite curves.
- Sequences also change materials: `NiAlphaController` (opacity),
  `BSMaterialEmittanceMultController`, `NiMaterialColorController`
  "SELF_ILLUM" (glow colour), matched by the shape's name. Goodsprings'
  house windows from outside (`craftsmanwindowext.nif`): by day
  `WastelandWindowScript` plays `Right` ("Turn Off"), whose keys put the
  glow card's opacity at 0: drawn invisible, as recorded (AmbientColor.w
  0); `Left` turns them on at night.
- `nvinspect <Data> meshes <CELL>` lists all of this per piece.

### Particles

Read from the exe (`findings\particles.md`, addresses there) and checked on
the game's files and the barracks and Goodsprings recordings.
- Blocks (`nif::particles`): every one of the game's 1,252 systems' blocks
  reads exactly (`nvinspect <Data> particles`: 45 block types, none left
  over). `NiPSysData` (bs 34) stores no per-particle arrays, only which
  exist and the most particles (the vertex count); sub-textures (u, w, v,
  h). The loader sets every controller's "compute scaled time" flag.
- The update (`00c1b260`): new particles join, the active modifiers run in
  the list's order, the strip updater last; world-space systems keep only
  their scale (`00c1add0`). The update controller skips a frame when the
  first looping emitter's life is shorter than the gap (`00c271c0`).
- Emission (`00c1c630`, `00c1c3c0`, `00c220c0`): the emitter controller
  walks its "emitter active" step keys between the last and this time
  (under a controller manager the playing sequence's interpolators and
  time; a new sequence only notes its time); particle n of a stretch is due
  once trunc((t − start) × rate) reaches n (chopped: the code sets the FPU
  to truncate), at most 15 a call, aged (end − start) − n/rate, on the
  system's clock (last update + the controller's step). Keyless "active"
  value 1 emits from 0 (the constant path). Each: life (U − ½) var + life
  (skipped if already older), speed likewise, declination/planar (2U − 1)
  var + base through the 512-step tables (`00a813c0`), the shape's point
  (box z, y, x; cylinder; sphere; mesh: trunc(n U) mesh, vertex / triangle
  centre / point on it with a fast square root / edge), colour, radius
  (2U − 1) var + radius, size 1, sub-texture rand() % count; random
  numbers are MSVC `rand()` (the game shares one generator: same rules,
  different numbers).
- Modifiers: age/death (age += t − last update: a new particle's age is
  counted twice on its first update, as the game does), position, rotation
  (> 10π → 0), grow/fade (≥ 1e-4), simple colour (three colours, alpha fade
  in/out), colour keys, gravity (strength × 1.6, planar/spherical, decay,
  turbulence × 500), drag (per 1/30 s), bomb, wind (strength × the wind
  vector, whose writer isn't traced: 0 here), colliders (plane and sphere
  tests over the frame, the earliest hit, bounce or slide, spawn, die),
  spawning (generation limit, chance, min + trunc((max − min) U) + ½
  rounding, a cone about the parent's direction).
- Drawing (`00e6ab40`, `00b700d0`, `00b6f65a`): quads facing the camera
  built on the CPU (corners p ∓ D, p ∓ E from the camera's right/up and the
  angle; uv (0,1) (1,1) (1,0) (0,0)), colours packed round(c × 255)
  unclamped; blended ones sorted back to front. `NOLIGHT017.vso` (atlas) /
  `NOLIGHT016.vso`, no fade by angle; `NOLIGHTTEXVC.pso` (texture × vertex
  colour × `MaterialColor`, fog toward its colour) or, for `SrcAlpha`/`One`,
  `NOLIGHTTEXVCPMA.pso` with `One`/`One` (colour × alpha × (1 − fog)).
  `MaterialColor` = the placed object's Emittance (else the player's
  region's) × the material's glow mult for shaders flagged 0x20000000,
  white otherwise; alpha = the material's; a blended system with material
  alpha 0 isn't drawn (`00e70140`). Depth tested, written only with the
  second flag set's 0x1; the alpha property's test.
- Checked: the barracks' four dust clouds were drawn with 59 particles each
  (236 vertices), sides 3.04–4.97, colour (158, 166, 171) at alpha ≤ 115:
  nv-rs gives 59, 3.0–5.0, the same colour and alpha (`nvinspect … particles
  meshes\effects\ambient\fxlightdustparticleswide02.nif run 20`). The
  Goodsprings whirlwind's debris: 192–198 particles (nv-rs 188–198),
  `MaterialColor` (1, 0.8902, 0.6667) = nv-rs's.

### Walking

- Collision is read from each model's Havok data (`nif::collision`): MOPP
  trees over packed triangle strips, `bhkNiTriStripsShape` (one model),
  boxes, spheres, capsules, convex hulls, transform and list shapes, at
  6.9991257 game units per Havok unit. `nvinspect <Data> collision`
  surveys every model the game can see (14,908; 8,346 with collision) and
  the placed primitives: every Havok block in the game's files is read
  except phantoms (`bhkSPCollisionObject`, 39: triggers, water, gas traps;
  never solid), ragdoll blends (`nif::ragdoll`), constraints and actions.
  No `bhkCompressedMeshShape`, `bhkConvexListShape`, `bhkMultiSphereShape`
  or `bhkCylinderShape` is used by the game.
- **Which layers stop people: the game's layer matrix** (`00c828f0` builds
  43 rows of 64-bit masks at `01267f20`; `00c84740` tests a pair: either
  filter's "no collision" bit 0x4000 → no; different groups → row A's bit
  B). The character controller's row (30): everything but biped (8, living
  people's bones), non-collidable (15), portal (18), small debris (19),
  projectile zone (23), shell casing (25), the dead (29), camera sphere
  (33), door detection (34), line of sight (37), path pick (38). So
  unidentified (0), weapons, projectiles, traps, large debris and other
  characters block too. Names: the game's table at `011b0810`.
  `nif::collision::layers::{NAMES, CHARACTER_COLLIDES_WITH, blocks_walking}`.
  A filter's second byte: 0x40 no collision (left out; no game model uses
  it), 0x20 scaled, 0x80 linked, low 5 bits the part. Each placed object is
  taken as its own group (the filter answers "collide" for group 0; groups
  are given out at run time, not traced).
- **Shells**: every box, hull and triangle shape in the game's models has a
  convex radius of 0.1 Havok units (the survey): their surfaces stand 0.7
  units out of the triangles. The terrain's: 0.5 (3.5 units; `00cb0820`
  wraps each height field in a triangle collection with the float at
  `01016248`). The character's shape floats 0.7 above its feet
  (`00c72410`), so on models the feet stand at the triangles, on terrain
  2.8 above them. Walls stop the player 20.25 + 0.7 units off.
  `physics::Collider::add_solid`, `physics::TERRAIN_SHELL`,
  `CharacterShape::lift`. (The proxy's 0.05 keep distance isn't kept.)
- **Scale**: a body takes its node's place and turn but not its scale; the
  shapes carry their own. Static collections show it: all 258 scaled
  pieces' packed triangles carry their node's scale already (applying both
  shrank them twice; `scolstripmaplanter.nif`'s spotlight now matches its
  mesh exactly). A placed reference's `XSCL` scales its collision (the game
  clones the bodies scaled: `00c8f2a0` scales the body and flags it
  "scaled", `00cb30d0` scales a `bhkRigidBodyT`'s offset).
- **Collision markers**: references to `CollisionMarker` (form 0x21; the game
  makes it if a plugin lacks it, `0046a370`) with a primitive (`XPRM`: half
  sizes, colour, a float, shape: 1 box, 2 sphere, 3 plane, `004a4e90`) are
  fixed bodies (`0056eab0`, from the 3D setup at `00576990`) on the `XTRI`
  layer, else transparent (3), at the reference's place and turn, scale set
  to 1 first: a box of the half sizes, a sphere of radius x, a plane as a box
  0.01 deep; box radius 0.1. The game's files place 2,757 (2,189 planes, 443
  boxes, 125 spheres), none with `XTRI`. Activators' primitives are trigger
  phantoms (`0056d7e0`) and acoustic spaces' sound volumes (`0056f140`):
  not solid. `world::Placement::primitive`.
- **Doors that swing, read from the code** (`%USERPROFILE%\nv-re\findings\
  doors.md`; `world::doors`, `preview::cell::SwingDoor`,
  `viewer/src/doors.rs`; `nvinspect <Data> doors <CELL|WORLD X Y|all>`):
  the reference's action flags (`ExtraAction` 0x0E byte +0xC, `0041b370`):
  4 open now, 8 open by default (`00561d90`; the editor writes `XACT` and an
  empty `ONAM`, 255 of each in `FalloutNV.esm`). `GetOpenState`
  (`0047b250`): 0 unless the base opens (`0047a490`: ACTI, TERM, CONT,
  DOOR), 1 open, 3 shut, 2/4 while the model's `Open`/`Close` sequence is
  animating. Activation (`005180b0` → `0047a560`): nothing while a
  sequence plays; else the flag flips, the base's `SNAM` (opening) or
  `ANAM` (closing) plays, and the new state's sequence plays from its start
  (gates 1.00 s / 0.97 s); who did it is kept (`ExtraOpenCloseActivateRef`
  0x6c). At its end (`00567050`, the sequence's clock past its last key):
  deactivated, `OnOpen`/`OnClose` (event flags 0x10000/0x20000, `0047ac70`),
  and with `bAnimateDoorPhysics` 0 (the shipped INI) the Havok bodies are
  only then re-placed (`0047ab40`): the leaf's collision (its keyframed
  parts, owned by the door in the collider, `physics::Collider::move_owner`)
  jumps to the new pose when the swing ends. Placed open: shown at `Open`'s
  end (`005659f0` → `0047aec0`). `SetOpenState` (`005ced30`) activates with
  no actor (so a lock isn't asked and is cleared, `005180b0`); `Lock`
  (`005cbf80`) shuts an open door at once; `SetDefaultOpen` (`005cebf0`).
  Text keys `Sound: X` (`004eef00`, keys in [previous, current scaled
  time)) play the named sound a frame after activation (57 of 71 door
  models have one; 30 name the record's own sound: played twice). People
  on a path (`009e20c0`, navmesh `NVDP` door portals) activate a closed
  door when they reach it and wait while it opens; open, opening or closing
  ones are walked through. `DOOR` `FNAM` (byte +0x84): 0x02 automatic
  (`BSDoorHavokController`, `fAutoDoorActivateDistance` 300; no NV door),
  0x04 hidden, 0x08 minimal use, 0x10 sliding (`00518080`: no navmesh
  obstacle, no Havok change for people). Load doors never swing (the game
  teleports after `SNAM`).
- **Clutter and people**: moving clutter blocks where it was placed (the
  contact callback `00c711d0` takes clutter, weapon, projectile and prop
  contacts as ground and only stops those lighter than `fMoveLimitMass` 95
  from carrying the character along; nothing is pushed or knocked over
  here). Characters collide (layer 30 with itself) and `00c711d0` flattens
  those contacts to walls (normal z = 0): the viewer gives the player the
  living people as upright cylinders (`physics::Person`,
  `Collider::set_people`); the dead (`DEADBIP`) don't block.
- Triangles without area are dropped (some models have them, e.g. in the
  Prospector Saloon, where they made `walk` NaN).
- Movement settings from the game's `GMST`s: walk 77 × SpeedMult/100,
  run ×4, sneak ×0.57, activation reach 150.
- **The character controller, read from the game's code**
  (`%USERPROFILE%\nv-re\findings\physics.md`): gravity (0, 0, −98.1)
  Havok = 686.61 units/s² (`00f4b550`); people all share one size, not
  scaled by height: half extents 23 × 17.5 × 64, radius 20.25, height 128
  (`00c72410`); step 31 (`00c6eb00`); steepest ground 47° (cos 0.682);
  jump `fJumpHeightMin` 64 at √(2gh) = 296.5 units/s, keeping the
  horizontal speed; fixed physics steps of `fMaxTime` 0.016 s, at most 3
  a frame. `physics::CharacterShape::PLAYER`, `physics::GRAVITY`.
- Falls (`008a62b0`): from where the feet left the ground (not the top of
  a jump) to the landing; above `fJumpFallHeightMin` 600 the damage is
  `fJumpFallHeightMult` 0.025 × (fall − 600)^1.65 (700 → 49.9); the
  player's legs each with 50 % chance lose half as much; hard/light
  landing sounds; flying creatures take none. `world::combat::land`,
  `physics::Character::fell`.
- Load doors: `XTEL` gives the far side's position and heading; the
  viewer loads the destination cell and puts the player there.
- `nvinspect <Data> walk <CELL> [SECONDS]` runs the capsule from the
  arrival point in four directions and says how far it got and where it
  climbed or dropped. Doc Mitchell's (radius 20.25): walls 88 and 95
  units to the sides. Barracks: stops at the desks and benches (a 32-unit
  bench is above a step).

### Outdoors

- A worldspace's squares are `CELL`s with `XCLC` in its child groups; its
  persistent objects (doors, markers, many people) sit in one extra cell
  flagged persistent (0x400, also `XCLC` 0,0) and are sorted into squares
  by position. `WastelandNV`: 16,396 squares; Goodsprings is -18,0.
- `LAND`: `VHGT` (start value, then a signed step per point: the first of
  each row from the row below's first point, the rest from the west; ×8
  units), `VNML` signed bytes, `VCLR`, `BTXT`/`ATXT`+`VTXT` per quarter
  (quarters 0 SW, 1 SE, 2 NW, 3 NE; `VTXT` points 0..288 in the quarter's
  17 × 17 grid). `LTEX` → `TNAM` texture set (`TX00` diffuse, `TX01`
  normal). Goodsprings: a base plus six layers per quarter.
- **Terrain, read from the game's code** (`%USERPROFILE%\nv-re\findings\
  terrain.md`): `LAND` `DATA` flags 0x1 heights and normals, 0x2 colours,
  0x4 textures (each read only with its flag; no colours → white). An
  `ATXT`'s layer number is its slot (0–5, higher clamped to 5, a later one
  in the same slot replaces it); `VTXT` opacities ≤ 0 are 0, none capped.
  Weights (`0053aeb0`): layers keep their opacities, the base gets
  `clamp(1 − Σ, 0, 1)`, and past Σ = 1 every layer is divided by Σ (base
  0): no layer covers another (`world::land::blend_weights`). Packed per
  vertex as texcoord1 (base, layers 1–3), texcoord2 (layers 4–6, 0). UVs
  `column × fLandTextureTilingMult / 4` (0.5 per point: textures repeat
  every 256 units). Each square's diagonal in a checkerboard: south-west →
  north-east where column + row is even, else south-east → north-west,
  counter-clockwise from above. Heights: the running total rounded to a
  whole number (halves to even) before × 8. `VNML` signed ÷ 127,
  normalized (zero stays zero). Tangents per quarter: east along the slope,
  made square to the normal; the other axis normal × tangent (north).
- Where a quarter names no texture: the INI's `sDefaultLandDiffuseTexture`
  / `sDefaultLandNormalTexture` (`DirtWasteland01.dds` and `_N`, under
  `textures\landscape\`). `fLandTextureTilingMult` is 2 in this install's
  `Fallout.ini`; `uGridsToLoad` 5 (the viewer keeps 5 × 5 squares).
- The terrain shaders (`SLS2092`–`SLS2147`, seven groups for one to seven
  textures; vertex `SLS2100.vso`): `light = Ambient + Sun × saturate(n·L)`
  (+ point lights in some), `color = light × Σ wᵢ × BaseMapᵢ × vertex
  colour`, the normal `normalize(Σ wᵢ (2 NormalMapᵢ − 1))` in the
  vertex's tangent frame, fog as elsewhere. The weights come per vertex
  from the CPU (texcoords 1 and 2). Ported in `viewer/src/terrain.wgsl`.
- A weather's `NAM0`: ten colours (sky upper, fog, clouds lower, ambient,
  sunlight, sun, stars, sky lower, horizon, clouds upper) × six times
  (sunrise, day, sunset, night, high noon, midnight), RGBA. `FNAM`: fog
  near/far by day and night, then power by day and night. A climate's
  `TNAM`: sunrise and sunset begin/end in 10-minute units (NV: 6–8,
  18–20). Weathers' image space modifiers: `[n] "IAD"` names time `n`'s
  `IMAD` (`NVWastelandGS`: `NVWastelandSunriseIS`, `NVWastelandIS`,
  `NVWastelandSunsetIS`, `NVWastelandNightIS`, the day's again for high
  noon; not animatable). Goodsprings' region weather is `NVWastelandGS`.
  High noon's colours are used (blended around 12:00, confirmed by the
  Goodsprings recording); midnight's never are, and the colours' "clouds
  lower/upper" aren't used (clouds take `PNAM`). Its night ambient and
  light (172,170,215), (166,191,210) are brighter than by day: the night
  modifier's blue tint (55,119,236 at 0.63) is what darkens nights.
- **The sky by the hour, read from the game's code** (`%USERPROFILE%\
  nv-re\findings\sky.md`; `world::weather::{SkySettings, SkyClock,
  time_weights, modifier_weights, daylight, stars_alpha, sun_at}`):
  - Colours (`0063b630`): the climate's sunrise starts and sunset ends
    `fDaytimeColorExtension` (0.5 h) wider (NV 5:30–8:00, 18:00–20:30);
    night → sunrise (full at the window's middle, 6:45) → day (8:00) →
    **high noon (full at a fixed 12:00)** → day (18:00) → sunset (19:15)
    → night (20:30), linear; midnight never used. Bytes / 255, no gamma.
    Ambient = ambient; the sun's light = sunlight, × the outdoor image
    space's sunlight dimmer (float 11 after modifiers; 1.1 for
    `NVDefaultExterior`) in lit shaders, not ambient, not indoors; fog
    colour unscaled with HDR. Fog distances and power blend day/night by a
    day share ramping across the widened windows.
  - Dome `BlendColor[0]` = horizon, `[1]` = lower sky, `[2]` = upper sky;
    clouds use `PNAM` colours blended the same way (NAM0's cloud colours
    are unused) and move `wind (DATA[0]/255) × fWeatherCloudSpeedMax 0.1
    × layer speed / 255` a second.
  - The sun's path (`00641830`, the climate's own times): `D0` = middle of
    sunrise − `fSunAlphaTransTime`/2, `D1` = middle of sunset + that (NV
    6:00, 20:00); `x = fSunXExtreme (800) × (1 − 2f)`; the disc and glare
    at `(x, Y, |X| − |x|)` (Y −100), half-sizes `[Weather] fSunBaseSize`
    750 and `fSunGlareSize` 800, the disc fading in over 6:00–8:00 and out
    over 18:00–20:00, the glare × the weather's `DATA[4]`/255; the light
    from `normalize(x, −Y, −Z)` = `(x, 100, 100)`: due east at 6:00, 45°
    from the north at 13:00, due west at 20:00; by night it goes back west
    to east with the night's sunlight colour (no moonlight).
  - Stars: the stars colour at alpha 1 → 0 over 5:30–6:45, 0 → 1 over
    19:15–20:30, else 0 by day, 1 by night.
  - Weather image space modifiers (`0063ef20`): the two times' modifiers
    at their first keys, averaged by share (`v × Σ s·mult + Σ s·add`);
    from 8:00 to 18:00 the day's and high noon's shares are swapped
    (the day's full at noon); a time with none gets a blank one.
  - Interiors: a positive "around" angle turns the light clockwise seen
    from above (toward the light `(−cos a cos u, sin a cos u, −sin u)`).
- **Grass, read from the game's code** (`%USERPROFILE%\nv-re\findings\
  grass.md`, with three corrections below; `world::grass`,
  `cellview::grass`, `viewer/src/grass.rs` + `grass.wgsl`; `nvinspect
  <Data> grass WastelandNV -18 0`):
  - `GRAS` `DATA` (32 bytes, over the form's defaults): density u8 (%),
    min/max slope u8 (degrees), units from water u16 at 4, water rule u32
    at 8, position range, height range, colour range, wave period f32 at
    12–24, flags u8 at 28 (0x01 lit by its normals, 0x02 uniform scaling,
    0x04 fit to slope: every New Vegas grass has 6). `LTEX` lists its
    grasses in `GNAM`s. Settings: INI `[Grass]` (`iGrassDensityEvalSize`
    2, `iMaxGrassTypesPerTexure` 2, `fTexturePctThreshold` 0 here (engine
    0.3), `iMinGrassSize` 80, `fGrassStartFadeDistance` 7000 in
    `FalloutPrefs.ini`, `fGrassFadeRange` 1000, wind 5–125).
  - Per square (`0053bc10`): spots on points 2, 6, 10, 14 of each quarter,
    each 512 units, corner = the point − 256. Entries: the base texture's
    grasses (its opacity 1 − Σ all six layers, can be negative), then each
    layer slot 0–5 more than 0.1 opaque at one of the 3 × 3 samples; at
    most `MaxTypes` + 1 per texture (the game's off-by-one) and 16 per
    spot. Each sample's density is the grass's density / 100 where the
    texture is more than the threshold opaque, else 0 (not × opacity).
  - Filling (`0057de00`): n = min(512 / position range, 512 /
    `iMinGrassSize`) = 6 candidates per side, x outer; the 3 × 3 samples
    stretched by `3i/n` rounded to even; kept if a random 0..32767 <
    trunc(d × 32768); jittered by ±position range, floored; height = the
    triangle's plane; water rule against the cell's `XCLW`; normal =
    normalize(S + F) for fit-to-slope; slope test against a 512-step cosine
    table; the normal packed into the position's fractions (0.5 + n/2, at
    most 0.97); luminance 0.31 R + 0.37 G + 0.32 B of the `VCLR`.
  - Corrections to the findings: S (the "smooth" normal, `0053caf0`) is
    one corner's (the blend's fractions come from the nearest corner and
    are always 0); a blade jittered into a neighbouring square takes that
    square's colour through `0053f570` (corners blended by position `fmod`
    128, negative west/south of the origin); a grass is filled once per
    spot even when two textures list it.
  - Per blade (`00b62de0`): `srand((floor x << 16) | (floor y & 0xFFFF))`;
    brightness `(cr × 0.5 × (rand/65534 − 1) + 1 − cr) × luminance`, ≥ 1 →
    0.99; `w = floor(hr × (2 rand/32767 − 1) × 100) + b`.
  - Which squares (`0057d0a0`): the player's and the 8 around it within
    `fGrassStartFadeDistance + fGrassFadeRange` (8000); never farther.
    400–1,000 blades a square around Goodsprings.
  - Shader (`GRASS2002.vso`, `GRASS2000TMS.pso`): N = 2 frac(I.xyz) − 1,
    T, B from it, p = B·x·s + T·y·s + N·z·s (s = 1 + 0.01 w), sway sin((I.x
    + I.y)/128 + phase) × magnitude × vertex alpha² along north; light =
    b·Ambient + b·vertex rgb·sat(N·L)·Sun·grass dimmer (b = 0.25 + 0.75
    frac w); fade 1 − sat((|MVP·I| − 7000)/1000), the 4D clip length; fog
    as elsewhere; alpha = sat(1.75 × texture alpha) × fade. One heading for
    all grass. Light (`00baac10`): the sun light's diffuse colour × the
    image space's grass dimmer (`DNAM` float 12: 1.5 in
    `NVDefaultExterior`). Wind (0, 1, 5 + wind/255 × 120, fmod(2π × wave
    period × GameHour, 2π)).
- **Trees, read from the game's code** (`%USERPROFILE%\nv-re\findings\
  trees.md`; `speedtree` crate, `world::tree`, `cellview::tree`,
  `viewer/src/trees.rs` + `tree_leaf.wgsl`, `tree_branch.wgsl`; `nvinspect
  <Data> trees <TREE|WORLD X Y>`): the exe's own SpeedTreeRT
  (`__IdvSpt_02_`) under `BSTreeManager`/`BSTreeModel`.
  - `TREE` (`0051bbc0`): `MODL` (`Trees` + it: `trees\wastelandshrub01.spt`,
    root of `Fallout - Meshes.bsa`), `ICON` (`Textures\Trees\Leaves\`), `SNAM`
    seeds (0 skipped), `CNAM` 32 bytes (curvature, min/max bud angle, branch
    and leaf dimming, shadow radius i32 (< 0 → 128), rock speed, rustle
    speed), `BNAM` billboard size. A reference's seed: `XSED` byte →
    `SNAM[b % n]` (every reference has one). Only `WastelandShrub01`
    (18,332), `EuonymusBush01` (85), `WhiteOak01` (1); no tree is turned.
  - Model (`0066ac40`, `00666940`): size × `fTreeSizeConversion` (exe 10),
    rocking groups 2 with < 4 leaf textures else 1, wind matrices (0, 4),
    `CNAM` dimmings/curvature/bud angles unless `[SpeedTree] fTreeForce…` is
    in range; `Compute(NULL, seed, 1)`. Growing reproduces the recorded
    shrub (seed 171677, 425.07 units) vertex for vertex: both leaf levels
    (39, 14 leaves) and branch level 0 (228 vertices, a 464-index strip).
  - Meshes (`00668500`, `00667470`): leaves 4 vertices at the leaf with
    `BLENDINDICES` (1 − wind weight, matrix × 4, ((k+2)&3) + 4 × (groups ×
    map + rock group) + green/255 (< 1), 1 + level × 9009); indices
    (3,1,2),(0,1,3); uv from the file's 10002 entries, v × −1 (`011f8b79`),
    odd maps mirrored, (u of k, v of 3 − k) when the leaf textures differ.
    `LeafBase` (`00b288f0`) per map and group, level 0 uploaded. Branches:
    the level's used vertices in order, strips joined by repeating the last
    and next index (`00b2ed70(0)`); bark `Textures\Trees\Branches\<file>.dds`
    + `_n`.
  - Each frame (`00669a10`): lod = clamp(1 − (fastdist(camera, tree) − near)
    / (far − near)) with near/far `fTreeNear/FarDistanceBase` 2048/16384 ×
    `fLODMultTrees` 0.5; branch level trunc((1 − lod) n); leaves cross-fade
    between levels (`00b0c8d0`, the billboard counting as one more level).
    The recorded shrubs at 6,216/6,087 units got alpha references 239/217
    exactly (fast square root). The billboard level is never drawn
    (recorded): shrubs vanish past ~6,300 units.
  - Wind (`006658b0`): W = weather wind / 255 (0 inside); four matrices
    yaw/pitch 0.61 W × fast sin/cos(20 W × rate × clock), rates (0.15,
    0.17), (0.25, 0.15), (0.19, 0.05), (0.15, 0.22); leaves' rock/rustle
    amounts W × l/2 (l = |a| + |b| of matrix 2, `fLeaf…SwayInfluence` 1),
    clocks × `fLeafRockTimeScale` 2 / `fLeafRustleTimeScale` 0.5;
    `RockParams` = (amount, `CNAM` rock speed × clock, file 21001, 0).
  - Leaves draw first after the clear with `ATOC` (NVIDIA alpha to
    coverage; with the alpha test on the driver uses coverage instead, as
    DXVK emulates): the cross-fade's alpha references then don't cut;
    alpha = 2 × texture alpha. Light = saturate(n · L) × DiffColor (sun ×
    sunlight dimmer) × the image space's tree dimmer (`DNAM` float 13) +
    ambient, × the leaf's brightness. Branches: `STB2004.vso`'s first lit
    pass (brightness as vertex colour, normal mapped).
  - Checked: the shrubs' leaves replayed from the recording (draw
    152842689) against the viewer's at the same camera: same places and
    shapes, stored colour 0.93–0.96 of the game's over the leaves (edges
    blend with the sky there, with terrain here).
- **Which weather** (`world::weather::WeatherState`, read from the code,
  `%USERPROFILE%\nv-re\findings\weather.md`): the climate's `WLST` and
  every region's `RDWT` (after an `RDAT` of type 3: override u8 at 4,
  priority u8 at 5) are rolled together: entries {weather, chance,
  global}, a global's rounded value replacing the chance (`VNight` gives
  `NVWastelandClearNight` 1 against 100 at night, 0 by day), `random %
  total`; nothing → `DefaultWeather` (0x15E). Again once `1 + 22 × (255 −
  volatility)/255` hours have passed (23 for every climate) and no fade
  runs, every frame until the result differs. The target: a script's
  override, else (outdoors) the player's weather region's roll, else the
  climate's. The player's weather region is chosen on entering a cell:
  cleared outdoors, then the cell's `XCLR` regions (reversed) in this
  worldspace whose outlines hold the player, the first kept unless a
  later one has weather data and the kept one none, or overrides when it
  doesn't, or has higher priority; interiors keep it. A new climate
  (worldspace `CNAM`; an interior "behaving like an exterior", `DATA`
  0x80, its `XCCM`) clears the weather and override and shows the next
  roll at once. A change fades linearly over `fWeatherTransMin` +
  (`Max` − min) × `DATA[3]`/255 hours (0.25 h for NV's weathers), sped ×5
  when a script's `SetWeather` comes mid-fade. `SetWeather W [1]`,
  `ForceWeather W [1]` (at once; clears the region), `ReleaseWeatherOverride`,
  `GetIsCurrentWeather`, `GetCurrentWeatherPercent`, `IsRaining` /
  `IsSnowing` (`DATA[11]` 0x04 / 0x08 with the `DATA[6]`/`[7]` thresholds).
  Saved. Vanilla NV has no rain, thunder or weather sounds.
- Sky: the code names `Meshes\Sky\Atmosphere.nif` and `Clouds.nif`.
  `SKY.vso`/`SKY.pso`: colour = `BlendColor[0] × vertex red +
  BlendColor[1] × green + BlendColor[2] × blue`, alpha = vertex alpha ×
  `BlendColor[0].w`, then × `Params.y`; drawn at the far plane. On the
  dome, red covers the bottom and the horizon, green a band just above
  it, blue the top (0.51 at the zenith).
- **Distant land, read from the game's code** (the terrain manager:
  `BGSTerrainManager.cpp`, `BGSTerrainNode.cpp` in its asserts;
  `world::lod`, `Game::lod_land`, `viewer/src/lod.rs`, `exterior.rs`): a
  quadtree described by `lodsettings\<world>.dlodsettings` (24 bytes, read
  by `006fc490`: u32 finest level 4, u32 coarsest level with models 32,
  u32 root size 128, i16 × 2 root cell −64 −64, i16 × 2 last cell 63 63,
  u32 objects level 4 for WastelandNV). Each frame (`006fdaa0`) a node is
  drawn whole unless it's above level 32 (always split), touches the
  loaded `uGridsToLoad` square around the player's cell, edges included
  (`006fede0`), or the player is nearer than its radius (half its
  diagonal) × `[TerrainManager] fSplitDistanceMult` (`006fe550`; exe
  default 0.75, `FalloutPrefs.ini` 1.5, and only 1.5 fits the recording).
  Distance = flat distance from the player's position to the node's
  square (`006fe830`). Models `meshes\landscape\lod\<w>\<w>.level<L>.x<X>
  .y<Y>.nif` with `...\Diffuse\` / `...\Normals\<w>.n.level<L>...dds`.
  At Goodsprings: 40 level-4, 30 level-8, 22 level-16 and 8 level-32
  chunks cover the whole map. A split node stays drawn until all four
  quarters are loaded; they then appear together, each starting in its
  quarter of the parent's texture and normal map and fading to its own
  over `uTerrainTextureFadeTime` 1000 ms, linearly (`006ff2c0`,
  `006ff3f0`: `LODTexParams` = (corner in the parent's texture: 0.5 across
  for the western quarters, the textures running east to west; 0.5 up for
  the northern ones), `fDetailTextureScale` 3 × level / 4 (unused by the
  shader), fade)). Level-32 chunks stay loaded while split
  (`bKeepLowDetailTerrain` 1). Geomorphing (`006feb20`): factor 1 nearer
  than the parent's radius × `fMorphEndDistanceMult` 0.65, 0 past ×
  `fMorphStartDistanceMult` 0.7, linear between (not in the INIs; the
  INI's `fBlockMorphDistanceMult` doesn't exist in the exe); the vertex
  shader draws `lerp(texcoord1, z, factor)` (`NiAdditionalGeometryData`,
  `nif::additional`). In all 2,308 distant-land models of every worldspace
  texcoord1 equals the vertex's own height, so geomorphing changes nothing
  with the game's files.
  `SLS2002.vso` + the recorded pixel shader: inside `HighDetailRange` the
  land is lowered by `GeomorphParams.y` (`fLODLandDropAmount` 230); colour
  = BaseMap (a texel in) [lerp from the parent's, while fading] × (0.55 +
  0.8 × noise at 1.75 × uv) × max(Ambient + Sun × saturate(n·L), 0) with
  `n = 2 × NormalMap − 1` in world space, fogged.
  The game's outdoor far clip plane is about 352,000 units (recorded
  projection, ±2,500; how it's chosen isn't traced): the viewer draws no
  distant land past it and keeps its sky dome just beyond.
- **Distant objects, read from the game's code** (`006fdfc0`,
  `BGSDistantObjectBlock.cpp`; `world::lod::object_blocks`,
  `Game::lod_object_block`, `viewer/src/lod_objects.rs`): one block per
  node of the objects level (4) whatever the land does: kept within
  `fBlockLoadDistance` (125000) of the player, its ordinary model
  `...\blocks\<w>.level4.x<X>.y<Y>.nif` within `fBlockLoadDistanceLow`
  (50000; exe default `fDefaultBlockLoadDistanceLow` 50000), farther its
  "high" model `...\blocks\<w>.level4.high.x<X>.y<Y>.nif` (only six in the
  Mojave, each a few tall landmarks: (−20, −28)'s is one object about
  1,000 units across standing at heights 11,000–12,300). Only
  level 4 exists; no `Stinger` or `PostApocalypse` blocks, and no distant
  trees (`...\Trees\TreeTypes.lst`, `.DTL` per level-8 node within
  `fTreeLoadDistance`: none shipped). Segments (`nif::segments`: count u32
  at the block's end, then flags u8, index-list start u32, triangle count
  u32) are one per cell, north first then east; those over loaded cells
  are hidden. `BSShaderPPLightingProperty`, second flag set 0x4, the
  `<w>.buildings.dds` atlas.


- **Recorded at Goodsprings (13:06, `NVWastelandGS`)**, every value the
  game's shaders got, matching nv-rs:
  - `AmbientColor` (0.37962, 0.45983, 0.59244) = the day's (87, 105, 138)
    and high noon's (99, 120, 154) blended 0.183 / 0.817, ÷ 255, unrounded;
    `PSLightDir` (−0.07881, 0.70491, 0.70491) = `normalize(−11.18, 100,
    100)`; `PSLightColor` = sunlight (255, 227, 170) × 1.21
    (`NVDefaultExterior`'s sunlight dimmer 1.1 × `NVWastelandIS`'s 1.1);
    `FogParam` (120000, 119990, 0.5), `FogColor` (150, 168, 190). The
    picture is cleared to the fog colour. nv-rs now passes these unrounded
    (`world::weather::SkyLight`, `WeatherMix::light_at`).
  - Sky dome `BlendColor` [0] horizon, [1] lower, [2] upper (upper
    (0.26657, 0.34251, 0.60631) = (50, 71, 135) / (72, 91, 159) at the same
    0.183 / 0.817); the sun's `BlendColor[0]` the sun colour the same way;
    every cloud layer's `BlendColor[0]` its `PNAM` colour the same way.
    **`Params.y` = 0.88 on the dome, clouds and sun**: the image space's
    float 8 ("LUM ramp no tex", 1.1) × the weather modifier's track 8
    (0.8). From the code: `00b8b440` copies working-copy float 8 to
    `011ad87c` (which `PrintHDRParam` calls "Lum Ramp"), `00b89d80` hands it
    to every sky shader when HDR is on. `world::weather::sky_brightness`.
  - Clouds: `Clouds.nif`'s four shapes are the four layers in file order
    (`CloudDome` 0, `HorizonLayerClear` 1, `HorizonLayerOvercast` 2,
    `LowerLayer` 3; drawn with 289, 84, 96, 341 vertices); `alpha.dds`
    layers are drawn too (invisible). Their vertex colours are pure red, so
    only `BlendColor[0]` counts. `TexCoordYOff` 0.232 and 0.29 on layers 0
    and 3 (speeds 52 and 65): the per-layer speeds as the code says.
    `Params.x` (texture cross-fade) 0 with no weather change.
  - The sun's disc: `SKYTEX`, added (SrcAlpha, One), × `Params.y`. No glare
    was drawn (the sun was overhead behind the camera).
  - Terrain (`SLS2124`/`2132`/`2140`/`2144`, `SLS2100.vso`): the vertex
    buffers (declaration: position, normal, colour float4, uv, tangent,
    binormal, texcoord1 float4, texcoord2 float4; 104 bytes) hold exactly
    what `world::land` builds: uv 0.5 per point, weights (base, layer 1–3)
    and (layer 4–6, 0), `VCLR` ÷ 255 with alpha 1, and **the game's tangent
    frame**: per vertex the sum of its triangles' `dP/du`, `T = normalize(S
    − (N·S) N)`, `B = normalize(N × T)` (a slope corner with normal (0.1026,
    −0.3708, 0.923) got T (0.9924, −0.0258, −0.1206), B (0.0685, 0.9284,
    0.3653)). The pixel shader turns the light into (T̂·L, B̂·L, N̂·L), each
    axis normalized on its own. `world::land::TerrainMesh::tangents`.
  - **The terrain fades into the distant land at the loaded area's edge**:
    a second pass over every loaded quarter (alpha blended, depth equal;
    shaders not in package 13): the quarter's level-4 chunk texture × (0.55
    + 0.8 × noise) × (ambient + sun × saturate(n · L)) with the chunk's
    world-space normal map, fogged, at alpha `1 − saturate((9625.6 − d) ×
    0.0003756)`, d the flat distance from the middle of the player's cell
    (`LandBlendParams.zw`): none within 6963 units (1.7 cells), full at
    9626 (2.35 cells). Coordinates: the quarter's uv / 64 + its corner in
    the chunk (`LandBlendParams.xy` = ((cell − chunk) × 2 + quarter column
    or row) / 8), then u turned around (the chunks' textures run east to
    west: their meshes' south-west corner is at uv (1, 0)), inset a texel.
    `cellview::TerrainLodBlend`, `viewer/src/terrain.wgsl`.
  - Distant land: `HighDetailRange` = the middle of the player's cell and
    10225 (2.5 cells − 15) each way; `GeomorphParams.y` 230. The 37 chunks
    drawn in the main view (levels 4, 8, 16, 32; plus a reflection pass)
    are exactly the quadtree's nodes in view (`world::lod` test).
    `GeomorphParams.x` 1 on the three nearest level-4 chunks, 0.41288 on
    (−20, 4), 0 elsewhere: the formula gives 0.41288 with the player 3.74
    units north of the camera (the first-person camera stands behind the
    player's position, which the terrain manager uses). `LODTexParams.w` 1
    everywhere (no fade running). Replayed draw by draw: only three
    level-4 chunks are visible in the final picture; every coarser chunk
    is hidden behind the hills.
  - Distant objects: exactly the ordinary blocks within 50000 of the
    player (14 drawn, one of them mostly behind the camera, by segments)
    and the "high" blocks of (−4, 12), (−8, 20), (−8, 24); every other
    ordinary block in view (from 50,824 units on) wasn't drawn.
  - Distant objects: `SLS2000`, alpha tested greater than 128.
  - Grass: `DiffuseColor` the sunlight without the dimmer, `AddlParams.x`
    1.5 (grass dimmer), `AlphaParam` (0, 0, 7000, 1000), `WindData` (0, 1,
    5 + 50/255 × 120, phase); the phases of the three Goodsprings grasses
    (periods 60, 27, 30) are `fmod(2π × period × GameHour, 2π)` at the same
    hour.
  - Fog's distance is the projected position's length with the
    projection's depth, which starts at the near plane (5 units): nv-rs
    now subtracts it everywhere (`cellview::GAME_NEAR_CLIP`, `fog_range.z`).
  - Lit statics outdoors: the sun's highlight is drawn in the same pass
    (shader not in package 13): Blinn-Phong with the half vector per
    vertex, × the normal map's alpha, the N·L fade below 0.2, × the sun's
    colour (with the dimmer), saturated. So the directional light does get
    highlights (was a guess). Self-lit glows × 1.5 (`NVDefaultExterior`'s
    emissive mult).
  - Image space outdoors (`NVDefaultExterior`, 152-byte `DNAM`, with
    `NVWastelandIS` at its first keys): bloom radius 7 (15 taps, σ 3.5),
    bright pass (0.6, 2), average clamp 1, final limit 1.4; `Cinematic`
    (1.1, 0.2, 1.1, **1.3**: brightness × track 20's 1.3); `Tint` (0.99283,
    0.6602, 0.02768, 0.39216) = the record's (251, 145, 0) at 0.33 and the
    modifier's (255, 188, 13) at 0.392 averaged by their amounts, the
    larger amount kept (was a guess). The final pass scaled the scene by
    1.4 / a with the bloom's average a ≈ 1.73 (the recorded 1 × 1 average
    (0.612, 0.624, 0.639) as an 8-bit picture).
  - Eye adaptation: the average pass (`ISHDRDS4ADAPT`) gets `TimingData.z`
    = the frame's seconds as the game counts them (0.24 here: the
    recording ran at about four frames a second) and `HDRParam.z` = the
    image space's eye adapt speed (0.9), and eases from the last frame's
    average by `1 − speed^seconds` (a tenth of the way a second at 0.9).
    `TimingData.z` is 0 whenever the console is open (menu mode): both
    indoor recordings had it open, and a later Goodsprings frame with the
    console open got 0 with the same speed 0.9. Ported
    (`grade::adapt_eyes`).
- **Textures are sampled as stored**: no `D3DSAMP_SRGBTEXTURE` in the
  whole recording (152 million calls), so the game filters stored values.
  The viewer now uploads every texture UNORM and its shaders take the
  samples as they are (vertex colours are still encoded back). With sRGB
  sampling the filtering averaged decoded light, which brightened high
  contrast textures: Goodsprings' terrain and statics 1–3%, Doc Mitchell's
  rug 1.07–1.12 → 1.00–1.03 now; the barracks' walls 1% closer.
- **Texture filtering** (the Goodsprings frame's 2,033 `SetSamplerState`
  calls): every scene texture stage gets `D3DSAMP_MAXANISOTROPY` = the
  INI's `[Display] iMaxAnisotropy` (15 in this `FalloutPrefs.ini`; 8 in
  `Fallout_default.ini`), `MINFILTER` anisotropic, `MAGFILTER` linear,
  `MIPFILTER` linear (stage 5's attenuation textures and the HUD: none),
  `MIPMAPLODBIAS` 0; the post-process and HUD stages 1. The viewer
  samples at the INI's anisotropy (`main::anisotropy_setting`), trilinear,
  no bias, as before but no longer a fixed 16 (report 004's "blurry
  textures" can't be settled this way: the game's and the viewer's
  sampling are the same; a captured frame at that spot is needed).

### Water

Read from the game's code and data (`%USERPROFILE%\nv-re\findings\water.md`
plus the implementation's own reading) and its shaders; not yet compared
with an in-game picture. `world::water`, `cellview::water`,
`viewer/src/water.rs` + `water.wgsl`. `nvinspect <Data> water WastelandNV`
lists the squares whose water stands above part of their terrain (767 in
the Mojave) and the 24 placed waters; `water WastelandNV 13 9` one square
(with a map of where the ground is under the water); `water <WATR|PWAT|
CELL>` a type's values, a placeable's placements, a cell's placed water.

- `WATR` `DNAM` (196 bytes): sun power 16, reflectivity 20, fresnel 24, fog
  near/far 32/36 (the far also scales the depth map), shallow/deep/
  reflection colours 40/44/48, wading strength 88, noise strength 96, wind
  directions 100–108 and speeds 112–120 of the three ripple layers, depth
  falloff 124/128, fog amount 132, noise tile 136, underwater fog 140–148,
  distortion 152, shininess 156, reflection multiplier 160 (max(v/10, 1)),
  light radius/brightness 164/168, layer uv scales 172–180 (max(1,
  ceil(v/100))), amplitudes 184–192. `ANAM` opacity /100, `NNAM` noise.
- A cell's water (`005471e0`): none indoors or without `DATA` 0x02; `XCLW`,
  or the worldspace's `DNAM` water height when `XCLW` is the largest float;
  type `XCWT`, else `NAM2`. Lake Mead at 2600 (`NVCleanWater`), the
  Colorado below the dam at −2300. Worldspace `NAM3` = distant water's
  type, `NAM4` its height: **water at `NAM4` (−2300 in the Mojave)
  reflects the whole scene**, other outdoor water only the sky.
- Placed water (`PWAT`): `MODL` (a quad with a `WaterShaderProperty`),
  `DNAM` flags + type: 0x1 reflects, 0x200 refracts, 0x10000000 depth,
  0x20000000 ripples from the model's UVs, 0x80000000 no fog amount.
  Goodsprings' "pond" is the windmill's cattle troughs
  (`NVCleanWater1x402`, `WATER001`).
- Shaders (decoded from package 13): `WATER000` as in the findings;
  `WATER001` (no reflection): top = lerp(waterC·N·L, ReflectionColor,
  Dx·fres), lerp(body, top, Dy), + sun; `WATER008` (inside, reflection):
  the body is pulled toward waterC by Dy before the reflection mix; the
  no-depth variants bend by lerp(4, distortion, sat(dxy/5000)) alone;
  distant water `WATER033`. The ripple passes: `ISNOISESCROLLANDBLEND`
  (layer 1 blue, 2 green, 3 red), `ISNOISENORMALMAP` (3×3 Sobel at 1/256).
- Distant water: the distant-land chunk models' second shape (no
  properties; surface at each cell's water height), drawn with the
  worldspace's `NAM3` type at `NAM4`.
- Sun for water: the weather's Sunlight of the hour (no dimmer); `L =
  normalize(s.x, 4 s.y, s.z)` for N·L; highlights pow(sat(−0.57 N.x + 0.82
  N.z), 100) + pow(sat(R·s), sun power).

### People and creatures

- The dead go limp (`nif::ragdoll`, read byte for byte from `_male\
  skeleton.nif`): 18 bones carry a `bhkBlendCollisionObject` →
  `bhkRigidBody` (frame at 52/68 = exactly the bone's in the file's pose,
  inertia rows at 116, centre 164, mass 180, damping, friction, the most
  speeds; pelvis friction 10, the rest 0.3) with a capsule; 17 joints:
  `bhkRagdollConstraint` (169 bytes: twist, plane, motor, pivot per body,
  cone, plane and twist limits; spine 18° cone, ±10°) and
  `bhkMalleableConstraint` wrapping one (type 7) or a limited hinge (type
  2: axle, two perpendiculars, pivot per body, min/max; knees −50..87°),
  then `tau` 0.9. The pivots meet in the file's pose. `nvinspect <Data>
  nif meshes\characters\_male\skeleton.nif` lists them, the file pose's
  angles against the limits, and drops it on a floor (at rest in 5.7 s).
  The death poses (`deathposes.psa`, a `bhkPoseArray`) aren't used.
- An NPC is assembled from its records (`world::actor`): race (`RACE`:
  after `NAM0` the head parts, after `NAM1` the body parts, each `MNAM`
  male / `FNAM` female, `INDX` + `MODL` + `ICON` (the part's skin
  texture); head 0, mouth 2, teeth 3–4, tongue 5, eyes 6–7; upper body 0,
  hands 1–2), clothes from `CNTO` (`ARMO`: `BMDT` slots, `MODL` male /
  `MOD3` female; the slots they cover hide the race's parts), hair
  (`HAIR`: model + `ICON` texture; `NoHat`/`Hat` versions), eyes (`EYES`
  `ICON` replaces the eye model's texture), head parts (`HDPT`), hair
  colour `HCLR`, height `NAM6`. `TPLT` + `ACBS` template flags (traits
  0x01, model 0x40, inventory 0x100) follow templates and leveled lists.
  Creatures (`CREA`): `MODL` skeleton, `NIFZ` models beside it.
  `nvinspect <Data> actor <NPC>` prints the result.
- Skinning (`nif::skin`): vertex = Σ weight × bone pose × the bone's
  skin-to-bone transform (`NiSkinData`); with the skeleton's own pose this
  gives back the model file's body within 0.1 units (`upperbody.nif`
  against `skeleton.nif`). `BSDismemberSkinInstance` partitions 100–299
  are gore caps (hidden); the body uses 0–13 and thousands.
- Animation (`nif::anim`): `.kf` sequences, keyframes or compressed
  B-splines (`NiBSplineCompTransformInterpolator`, open uniform cubic;
  -FLT_MAX defaults mean "not set"); the idle is
  `<skeleton folder>\locomotion\mtidle.kf` (people), 4 s, looping. The
  accumulation root (`Bip01`) isn't moved by the idle and sits at the
  actor's feet; `Bip01 NonAccum` carries the body's height.
- Animation groups and blending (`world::animation`, `viewer/src/
  actors.rs`; `findings\animation.md`): the exe's 245 groups (`011977d8`:
  section, kind); `Actor::PickAnimations` (`00895110`) picks the movement
  group from the mover's flags each frame and sets the **movement rate =
  speed ÷ the whole-number root speed of the Forward/FastForward file**
  (people 77 ÷ 85, running 308 ÷ 353); a group replacing a section's old
  one **cross-fades** over max(old blend-out, new blend-in) text-key
  frames ÷ 30 (`Blend:`, `BlendIn:`, `BlendOut:`; `mtforward` 6,
  `mtfastforward` 9), else `fAnimationDefaultBlend` 0.2 s, ÷
  `fAnimationMult` (`004949a0`); an empty section **blends from the
  pose** (`BlendFromPose`); a group ending or stopped eases out by its
  blend-out (`004994f0`). Gamebryo blends per bone by the controlled
  blocks' priorities (`.kf`: idles 10, walks 30, aims 25–45 arms / 0 legs,
  attacks 55): the highest wins, the next fills `1 − highEase`, shares `w
  × spinner × (highEase | 1 − highEase)` normalized (`00a37260`), ramps
  linear (`00a34ba0`). The actor moves by the root's travel (× scale),
  analytically at the group velocity × rate while the walk eases in
  (`00491180`); rate scaling only in state ANIMATING. Walk speed
  (`00647d10`) = SpeedMult/100 × `fMoveBaseSpeed` 77 × legs; run × 4.
  **Turning in place plays `TurnLeft`/`TurnRight`** (no move direction,
  flags 0x10/0x20) at the turn's scale (1.5, combat 2.5, creatures 1.25):
  the files are `characters\_male\locomotion\mtturnleft.kf` /
  `mtturnright.kf` (1 s, looping, `Blend: 3`), and the game loads a
  skeleton's `<folder>\*.KF`, for characters also `<folder>\Locomotion\
  *.KF` and (as idles) `Locomotion\Hurt\*.KF` (`00447330`,
  `ModelLoader.cpp`), so the group lookup (`00495740`) finds them (the
  findings' "no TurnLeft files, the idle" was wrong). With the weapon out
  the group is the weapon kind's (`1hpturnleft.kf`, `h2hturnleft.kf` in
  `_male\`; `1gt`/`1lm`/`1md`/`2hh` in `Locomotion\`; none for rifles,
  which fall back to `mt`): not loaded here, so no turn plays then.
  `preview::actor::turn_path`, `ActorSkeleton::turn_left`. The
  accumulation root `Bip01`, where no sequence moves it (the idle
  doesn't; walks and turns do, unturned), stays at the actor's origin
  unturned (`nif::posed`'s rule), not as `skeleton.nif` stores it (68 up,
  turned 90°): idle people had stood a quarter turn off their heading
  (Sunny Smiles faced north for her east). Animations
  stand still in menu mode (dialogue menu, viewer menus), except the
  dialogue menu's speaker, who stops walking and turns to face the player
  (inferred, see "AI and walking").
- Rigid pieces name their bone in a `NiStringExtraData` `Prn` (eyes,
  hair: `Bip01 Head`). They're modelled around the bone in the skeleton's
  upright axes, so they take the posed bone × the inverse of its bind
  rotation (checked: hair spans the skinned head's top, eyes and mustache
  sit at the face). The files get upright two ways: 36 hair models
  (`hairbun.nif`, `hairfemalea.nif`, children's) turn the mesh node and
  turn it back on the top node (upright only with it); 16 head models
  (eyes, teeth, mouth, tongue) turn only the top node (upright only
  without it); 73 turn neither. So each mesh takes whichever is upright
  (`preview::actor::upright`; the game's own rule isn't traced). Sunny
  Smiles' bun sat in front of her face before.
- Between places (`world::ai::door_toward`, `catch_up`, `moved_into`):
  a package whose target is in another interior or worldspace leads to
  the nearest load door in the person's cell whose far side (`XTEL`'s
  door) is there, else (into an interior) one of its doors' far sides
  standing where they are; through it they come out at the walked door's
  `XTEL` position and heading, as the player does. One door only. People
  out of sight (moved this game, not on screen) walk there in game-time
  steps (see "AI and walking": people out of sight). Whoever the
  state has in the player's place but placed elsewhere is spawned on their
  own (`Game::actors_scene`, lit as the place), outdoors within 2.5
  squares. Checked on Sunny Smiles: `VCG02SunnyTravelOutside` takes her
  through `0010618E` to `WastelandNV` and round the saloon to
  `VCG02SunnyOutsideMarkerREF` (her package needs `VFreeformGoodsprings.
  bMetSunny` and `VCG02` stage 10).
- Follow (1) and accompany (7) packages (`world::ai::followed`): `PTDT`
  kind 0, the reference to keep near, and its value read as the distance
  (`CheyenneAccompany`: Sunny, 128; a guess at the field from its values;
  at least 64). Followers re-path each second and stop within it; through
  doors after them.
- Sandbox packages (`world::sandbox`, `viewer/src/sitting.rs`; read:
  `009f4700`, `009f54d0`, `009f5070`, `009f43c0`, `00929fc0`, `006435c0`,
  `009f3e80`, `009f41f0`): `PKDT` u16 at 8: 0x01 no eating, 0x02 no
  sleeping, 0x04 no conversation, 0x08 no idle markers, 0x10 no furniture,
  0x20 no wandering. The area: the package radius, else
  `fSandBoxSearchRadius` 6000; strayed past radius + 150, they walk back.
  A scan (every 3–6 s when choosing) finds, in the area: food (`INGR`,
  `ALCH`), furniture with markers (a bed when sleeping is allowed, else
  sitting or other non-bed furniture), other people, idle markers with an
  idle that passes (with its parents) — each if the owner allows (the
  reference's `XOWN`, else its cell's: none, them, or their faction) and
  not marked `XIBS` (ignored by sandbox); children only things flagged for
  them; then food they carry and one wander. Choice: weight = counter of
  its activity (six counters start at 128; after each choice all +1, max
  255, and the chosen one → 1); sleeping 0, eating +20 if food was found
  and −15 if no chair was; the last target rests 30 s
  (`iSandBoxPreventRepeatedActionTime`); targets gone, elsewhere, without a
  free marker or out of the area (people: + 384) weigh 0; `rand() %
  total`. Four times of day take over (sleep 19:00–01:00 start for 6–10 h,
  meals 6–9, 11–14, 17–20 for 0.5–2 h, rolled each time): then only that
  activity counts (10 each) and lasts the window. Durations in game
  minutes: `fSandboxDurationBase` 10 × the activity's multiplier
  (furniture 3 in `FalloutNV.esm`, eating 0.75, wandering and talking 0.5,
  idle marker 1.5) × (1 + energy × 0.05 × the activity's energy multiplier:
  −0.1, −0.05, 0.2, 0.05), ±25 %; energy is `AIDT` byte 2 (energy 50: sit
  22.5, eat 6.6, wander 7.5, idle marker 16.9). Elapsed minutes are counted
  as the game does (whole hours crossed + minutes, a signed byte).
  Re-choosing waits for a sitting or idle-marker idle to end, and someone
  seated gets up first. `nvinspect <Data> ai <REF>` prints a sandbox's
  flags, area, what it finds and the durations (Chet's store: 2 chairs, 2
  idle markers, one food item, carried food, wander; the shelves' goods
  are `XIBS`).
- Terminals (`world::terminal`): `TERM` `DESC` header, `DNAM` (hacking
  difficulty u8 0–4, flags u8 with 0x02 unlocked, server type), items
  `ITXT` text, `RNAM` printed result, `ANAM` flags (0x01 the note goes to
  the Pip-Boy), `INAM` note (`NOTE`: `DATA` kind, `TNAM` text), `TNAM`
  sub-menu, result script (`SCTX`, its own variables like `ref myLink`
  allowed), `CTDA` conditions. Goodsprings' schoolhouse terminal
  (`DefaultUnlockTermDESK1Easy`) unlocks its linked safe. Hacking (read
  from the code, `findings\ui_rules.md`): Science needed by difficulty
  0/25/50/75/100 (the INI's `[Hacking] fHackingMinSkill…`, exe defaults),
  else `sHackIneligible`; the word game isn't here (20 words at exactly
  the needed skill down to 5 at 100, 4 tries, word length 2 × difficulty
  + 4 or 5): with the skill the terminal opens, stays open, and gives
  `iXPRewardHackComputer…` once.
- Locks (`world::locks`): `XLOC` level u8, key (`KEYM`) at 4; locked as
  placed; `Lock [level]`, `UnLock` (the table's name), `GetLocked`,
  `GetLockLevel`, `GetLinkedRef` (`XLKR`). Read from the code: a lock's
  difficulty is the first `iLockLevelMax…` bracket (0, 25, 50, 75, 100,
  exe defaults) its level doesn't pass, above 100 key only
  (`sImpossibleLock`); picking needs Lockpick at least the bracket's top
  (`sLockpickSkillTooLow`); the first pick of each lock gives
  `iXPRewardPickLock<bracket>`. With the skill the lockpicking menu opens
  (below); picked, it stays unlocked (`GameState::locks`, saved). Leveled
  locks (+ the area's level) aren't applied.
- **Lockpicking** (`world::lockpick`, `ui::lockpick`, `cellview::lockpick`,
  `viewer/src/lockpick.rs`; read from the code, `%USERPROFILE%\nv-re\
  findings\lockpick.md`): `LockPickMenu` (class 1014, vtable `0107439c`),
  opened by doors/containers (`005180b0`, `00516dc0` → `0078db00`) unless
  the lock is level 5 or broken (`sImpossibleLock`, padlock, no sound) or
  Lockpick < `iLockLevelMax…` (`sLockpickSkillTooLow`, padlock,
  `UIPopUpMessageGeneral`). Menu `menus\lockpick_menu.xml`: tiles by `id`
  0..15; filled with "%d" skill, "Force Lock [n%]", `sLockLevelName…`, pins
  (`_Value` 1 then the count), English titles. Sweet spot (`00791000`): t =
  (skill − req) / (2 × step to the next requirement) in 0..1; sweet =
  trunc(lerp `iSweetSpotLength` 400/200/100/50/25), concentric 200;
  centre = random × meter width (screen − 4 safe zones, 1646.67 at 16:9),
  kept sweet/2 + concentric from the ends; six rings 17–22, each
  concentric/5 wider. Max turn (`007908f0`): 90, 75 … 15 by ring; outside
  15 × (half + d − W)/(2 half − W). Mouse x × menu width / screen pixels;
  movement sound on a hundred crossed and rand(10) = 0, or starting to
  move. Pin turned (x/W × −180 + 90)° about the lock's y through (0, 0,
  0.6), node at (0, 0, −1.6). Stages (`0078eb50`, whole ms): entering
  (lock + pin `Forward`, `UILockpickingEnter` or `sOutOfLockpicks`), new
  pin, ready (Slide Left/Right/Forward/Back → turn 90°/s; springs back
  90°/s; squeaks at 30 and 60), straining at the limit (health 1 + 0.01 ×
  skill, − 1/`fPickBreakSecs` a second; ±0.25° and ±2.5 shake; pin
  `Backward` at 1 − health; `UILockpickingPickTensionLPM` faded 500 ms on
  release, 100 ms on a break), break (`Left`, one bobby pin `0000000A`
  used), a new pin after 1 s. At 90°: Locks Picked + 1, first pick
  `iXPRewardPickLock…`, the owner's stealing rules, `UILockpickingUnlock`,
  unlocked, the player uses it. Force (F, `00790330`): rand(100) < skill −
  req + 10 (0..100) turns it all the way; else the lock breaks for good
  (`sLockBroken`, `UILockpickingForceFail`; `GetIsLockBroken`, the "Ignore
  Broken Lock" perk entry allows one more). Exit E. Scene (`0078e1c0`):
  `LockInterface01.NIF` + `BobbyPin01.NIF` under a node turned X(π/2)·Z(π/2)
  at (125, 0, 0); the lock's two point lights only, radius 3 × the node's
  bound (156.7); camera at the origin, ±0.75 tan(`fDefaultFOV` × 0.15°)
  wide, `fNearDistance`; drawn after the image space pass (`00872940`).
  `nvinspect <Data> lockpick <REF|LEVEL> [SKILL]`.
- Skill checks in dialogue: a topic named like "< Speech 25 >" with two
  lines, each with its own prompt (`RNAM`): one on `GetActorValue Speech
  >= 25` run on the player ("[SUCCEEDED] ...", Trudy's also runs
  `RewardXP 25`), one on `< 25` ("[FAILED] ..."; those words are the
  lines' own text). The tag (read from the code, `007638b0`): only for a
  line with `KNAM` (the `AVIF` or `PERK` checked: Trudy's `AVSpeech`),
  its name, then from the line's first `GetActorValue` condition "[Speech
  25]" if the player has it, else "[Speech 12/25]"; none: "[Perk name]";
  two spaces before the prompt. A topic's `TDUM` prompt is said instead
  at Intelligence ≤ `iDialogueDummySpeakThisIntOrBelow` (3).
- **Levels and experience** (`world::experience`, `world::perks`; read
  from the code, `%USERPROFILE%\nv-re\findings\levelling.md`): XP to
  reach level L = 25 (L − 1)(3L + 2) (`iXPBase` 200 exe default,
  `iXPBumpBase` 150): 200, 550, 1050 …; top level `iMaxCharacterLevel`
  30. Every award: × the perks' entry point 9 (Swift Learner × 1.1),
  rounded up, capped at the top level's total; nothing at the top. The
  level-up waits until out of combat (and menus), one level at a time;
  skill points ⌊10 + 0.5 × INT⌋ (+1 on even levels with odd INT) + entry
  point 10 (Educated + 2); a perk on even levels (`iLevelsPerPerk` 2);
  the list: playable non-traits with ranks left whose `GetIsSex` passes,
  the others' requirements deciding whether they can be picked. Player
  health = base + 20 × END + 5 × (level − 1) (200 at the start); people
  base + 5 × END, no level term. Kills: the player and teammates
  (`SetPlayerTeammate`) must have done more than
  `iXPDeathRewardHealthThreshold` 40 % of the victim's health; XP by the
  victim's own level from the creature table (`iXPLevelKillCreature…` 1,
  3, 6, 9, 12 → 1, 5, 10, 25, 50) or people's (0, 1, 7, 10, 13 → 0, 10,
  20, 30, 50 exe defaults). Map markers 10, picks and hacks 20–60 by
  difficulty, `RewardXP` (always the player). HUD texts `sStatsXP` "XP
  +N", `sLevelUp` "LEVEL UP", the menu `sLevelUpTitleText`. `nvinspect
  <Data> levels [LEVEL]` prints the table, kill rewards and a level's
  perks.
- **Perk entries** (`world::perks`): `PRKE` (kind 0 quest stage, 1
  ability, 2 entry point; rank from 0; priority), then for an entry point
  `DATA` (number, function 1 set / 2 add / 3 multiply, tab count), `PRKC`
  tab + its `CTDA`s, `EPFT` 1, `EPFD` value; for an ability `DATA` the
  `SPEL`; `PRKF` ends it. A ranked perk lists its entries once per rank
  and only the held rank's apply (Toughness + 3 / + 6). The game's 74
  entry point names are `perks::ENTRY_POINT_NAMES` (its table, built at
  `00f4fa60`: 0 Calculate Weapon Damage … 56 Modify Damage Threshold
  (defender) … 73 Adjust Heavy Weapon Weight). **How the game applies an entry point** (`005e58f0`, read; `findings\
  perks.md`): the player's entry lists (player `+0x884`, one per entry
  point); nobody else holds perks (`Character` `+0x4ac` → `0050fbe0`, 0;
  teammates would use the player's teammate lists at `+0xadc`, filled by
  `AddPerk`'s teammate flag, which no script in the data uses). Each entry
  point takes parameters after the owner (`perks::tab_kinds`, the table at
  `01196e44`): weapon + target (0, 1, 2, 8, 35, 36, 58), weapon (3, 5, 7, 34,
  37–40, 43, 49, 50, 52–54, 57, 59, 60, 72), item (4, 17), attacker +
  attacker's weapon (6, 56), target (14, 21, 27), attacker (15), attacker +
  attackee (16), none (the rest); the fists (`Fists` 000001F4, confirmed:
  `004839c0(500)`) stand in for no weapon. An entry's tab count must equal
  the parameter count + 1 (`005ea060`, else refused); tab 0 is asked about
  the owner, tab n about parameter n. Functions (`01197388`): 1 set, 2 add,
  3 multiply, 4 add a random value in a range, 5 add an actor value × x, 6/7
  (negative) absolute value, 8 add a leveled list, 9 add an activate choice;
  the data uses 1–3 and 9. Applied (36): 0 weapon damage after armour and
  ammunition effects, before the 20% floor (`009b5a30`); 1 crit chance, 36 the
  target's perks on the attacker's chance, 2 crit damage (`009b7060`); 6 limb
  damage (`0089a760`); 8, 33, 34, 35, 39, 40, 53, 54, 69 V.A.T.S.; 9, 10, 11
  levels; 12 regeneration (`0088b510`) and aid items' Health; 17 barter; 22
  carry weight (`008a0c20`); 24/25 addiction / aid effect durations, hostile
  effects (`MGEF` 0x01) excepted (`00823210`); 31 detection: never running,
  not moving while sneaking (`008a0d10`); 32 broken locks; 37/38 reload and
  equip rate = 1 + (Agility − `fAgilityReloadBase` 5) × `fAgilityReloadModifier`
  0.1, then the perk (`008c17c0`, `008c1940`); 42 the whole movement speed
  (`00885bf0`); 43 attack animation rate = `DNAM` 4 speed → perk × `DNAM` 60
  attack multiplier (`00893a40`: the 9mm's 1.25); 44 a food's rads; 51 fast
  travel while over-encumbered (Inventory Weight > Carry Weight, `00954cc0`;
  `sNoFastTravelOverencumbered`); 55 items ≤ `fPackRatThreshold` 2 ×
  `fPackRatModifier` 0.5 and 73 weapons ≥ 10 (`004d0900`); 56/58 on the
  threshold after the ammunition's DT effects: − attacker's 58 + defender's
  56, ≥ 0 (`009b5a30`; the Pip-Boy's figure `00782a90` asks 56 with the
  player's own weapon); 57 the ammunition's `DAT2` case (form at 12) at its
  percentage (f32 at 16) through the perk, `rand % 100 ≤ chance`, the player
  only (`00523150`); 68 what an item loses (`00891360`, `DamageItem`): the
  weapon-part hit, and each attack's wear `fDamageToWeaponValue` 0.2 (data;
  exe 0.1) after the ammunition's wear effects (`00646260`). Aid items
  (`00815d00`): a medicine's (`ENIT` 0x04) effects × (`fMagicMedicineSkillBase`
  1 + Medicine ÷ 100 × `fMagicMedicineSkillMult` 2), a food's (0x02) × the
  Survival factor (except rads, through 44), the player's Medicine/Survival
  whoever eats. Recognised only: 28, 29, 41. Not applied (35; `nvinspect
  <Data> perks` lists the perks): unused by any perk 3, 5, 7, 13–16, 18, 19,
  21, 26, 30, 45, 49, 50, 60, 63; the mechanic missing 4 mines, 20 terminal
  lockouts, 23 addiction rolls (`00824e70`: the item's `ENIT` chance, or past
  `fAddictionUsageMonitorThreshold` 70 of accumulated use (use − 70) ÷ 100,
  through the perk, `rand % 100`; `sChemsAddicted`), 27 activate choices, 46
  Meltdown, 47 the HUD's enemy health, 48 repairing, 52 knockdowns, 59
  throwing, 61–66 unarmed specials, 67, 70, 71 highlights, 72 explosions.
  `nvinspect <Data> perks [ENTRY]` prints every entry point the perks use.
- Map markers (`world::map`): persistent references with `XMRK`, then
  `FNAM` (0x01 on the map from the start, 0x02 can travel), `FULL` name,
  `TNAM` kind, `XRDS` finding radius (Jimmy's Well 500), `XLKR` arrival.
  `WastelandNV` has 186. Found (read from the code, `00779070`) strictly
  inside the radius measured in 3D, `XRDS` else `iMapMarkerRevealDistance`
  1000; notice `sDiscoveredText` "You have discovered" + the name, 10 XP;
  a marker starting both shown and travellable gives nothing. Saved; the
  Pip-Boy's Map tab lists found and revealed ones and travels to found
  ones. Fast travel (`0093cdf0`): refused with an enemy near or from an
  interior without `DATA` flag 0x04 (most); the trip takes route length
  ÷ run speed (308) as game time, whole hours past one hour (here the
  straight line, not the pathfinding route); no random encounters.
- Waiting (`GameState::wait`, T in the viewer): the clock moves on by the
  hours chosen (1–24); refused with `sNoWaitHostilActorsNear` when an
  enemy is near (`009764a0`: alive, within `fHostileActorInterior/
  ExteriorDistance` 2000/3000, fighting the player or attacking on
  sight; here only those in the player's cell or square). Each hour heals
  Heal Rate ÷ `TimeScale` (Heal Rate, actor value 15: 0 up to Endurance
  5, then `fAVDHealRateEndurance<6..10>Bonus` 5, 5, 5, 10, 10). Sleeping:
  below. Not here: moving people along hour by hour.
- **Hardcore needs** (`world::living::needs`, read from the code:
  `00969c30`, `008c5350`, `008c5610`, `008c5890`, `008c5b10`, `00969e90`,
  `005de920`; `findings\living.md` §4): only in hardcore, every frame of
  play and before each waited hour; nothing while `DisablePlayerControls`
  has the Pip-Boy off (the clocks wait). Time is `GameDaysPassed` × 1440
  game minutes; a point of Dehydration (73) per `fHCDehydrationRate` 10
  (exe), Sleep Deprivation (75) per `fHCSleepDeprivationRate` 50, Hunger
  (74) per `fHCStarvationRate` 25 real seconds of game time (× `TimeScale`
  ÷ 60 game minutes), whole points, the clock jumping to now (fraction
  dropped); sleep deprivation's clock moves but adds nothing while
  sleeping. Clocks not saved: a 0 clock (new or loaded game) sets all three
  to now. Stages: `DEHY`/`HUNG`/`SLPD`/`RADS` `DATA` (threshold u32, spell),
  kept highest first (a later record of equal threshold after,
  `004016a0`); a value's stage is the first it reaches. On a change, for
  the player: a different stage takes the old spell off and gives the new,
  then two notices: `s<Need>Increase`/`Decrease` (by the change's sign) and
  `sRadiationSick` "You are now sick with" + the spell's name (`%s %s`) or
  `s<Need>NotSick`; the same for RadiationRads with `sRadiation…`. Stage 5
  (1000) is an ability doing 10000 Health damage. `SetHardcore 0|1` (else
  an error): off clears "always hardcore" (+0xe38, set for a new player,
  `00938180`) and sets the needs to 0 in the order 73, 75, 74.
  `nvinspect <Data> living` prints the tables.
- `GameDaysPassed` grows continuously by hours ÷ 24 (`00867a40`), not by 1
  at midnight.
- **Sleeping and waiting** (`world::living::sleep`; `005095b0`, `00969fa0`,
  `005e0090`, `007c0220`, `007c0580`, `0094df80`, `008a0960`, `0088b510`):
  a bed (`MNAM` 0x80000000) refuses, in order: owned by someone the player
  may not take from (`sNoSleepInOwnedBed`; faction members of any rank may,
  `XGLB` no effect), the place forbids waiting (interior record flag
  0x80000, outdoors the worldspace's, `005444c0`; `sNoWaitInCell`),
  trespassing (`sNoSleepTrespass`), an enemy near (`sNoSleepHostileActors
  Near`), an active detrimental effect on Health (`00822e00`;
  `sNoSleepTakingHealthDamage`). Waiting refuses: trespassing, enemies,
  no-wait place or `EnableFastTravel … 0`, Health damage (the `sNoWait…`
  texts). Chairs are never refused. Accepting the hours: hours left
  (`GetPCSleepHours`) and the sleeping flag (`IsPCSleeping`) set, "Times
  Slept" +1 for a sleep; then an hour per real second: needs (hardcore),
  the clock by 3600 ÷ `TimeScale` real seconds, Heal Rate ÷ 3600 a second
  of it (the awake rate) + perk entry 12, in hardcore while sleeping Sleep
  Deprivation − min(it, `fHCSleepRestorationMod` 60); after the last hour,
  a sleep outside hardcore restores Health and Fatigue in full and the
  seven body parts by 1000. Cancel: no restore. The game's menu (see "The
  game's menus") is drawn. `PlayerBedSCRIPT` beds: `MenuMode` notes `IsPCSleeping`,
  `GameMode` casts `WellRestedSpell` (1440 s; `ResetHealth`, `WellRested
  Perk` × 1.1 XP), in hardcore too. `SetPCSleepHours n` = hours n and the
  flag on (`005c1a00(n, 1)`). `ShowSleepWaitMenu s 1` = the wait refusals
  then the menu in sleep mode (`007054f0(1)`), `ShowSleepWaitMenu s 0` =
  the menu in mode s.
- `Activate ref 1` runs the target's `OnActivate` (fewer than 5 deep),
  which does the usual thing only through its own `Activate`; without the
  1 the usual thing at once (`005b59f0`). `VCG01`'s hardcore question
  (`VCG01CasualHardcoreMessageREF.Activate player 1`) comes up this way.
- **Pickpocketing** (`world::living::pickpocket`; `005fa330`, `0075dc80`,
  `0075e240`, `0075e0b0`, `00643400`, `008c00e0`): using a person:
  unconscious → "<name> is unconscious." (`sNoTalkUnConscious`, `%s %s`),
  fleeing → `sNoTalkFleeing`, dead → search, sneaking + not a teammate →
  pickpocket unless they caught you before (`sNoPickPocketAgain`), else
  talk. Per stack: V = value × count taken (the player's own planted
  stacks back without a roll), 1 placed; chance = trunc(sum of the six
  `fPickPocket…` terms) clamped to Min…Max = clamp(40 + 0.6 × your Sneak −
  0.6 × theirs − 0.5 × V, 5, 85); no roll if V ≤ 0; else −5 karma unless
  their record karma is evil, success below the random 0–99; the visit's
  first theft is "Pockets Picked" +1. Caught: nothing moves,
  `sPickpocketFail`, they refuse further tries (process+0x188), and unless
  they ignore crime +1 minor crime / +2 infamy for their crime-tracking
  factions and the player, and the day-of-month record shared with thefts
  (a second the same day: they attack).
- **Trespass warnings** (`world::living::trespass`; `008f5480`, `009f9250`,
  `00918120`, `008db240`, `008c0ec0`, `008c1010`, `008b8e90`, `0047d740`):
  someone detecting the trespassing player (> 0) takes a trespass package
  (type 0x17, one at a time) unless the owner is evil (a faction flagged
  0x02; a person whose factions are all evil) and they aren't; it allows 3
  warnings, 0 in an off-limits cell (record flag 0x20000). They walk to
  within 200 units, say `GuardTrespass` (000000F2) when their timer is out,
  timer += `fAITrespassWarningTimer` 10 s; more warnings given than allowed
  and the timer out → the alarm: everyone loaded who doesn't ignore crime,
  detects the player and is in one of the alarmer's factions (or the
  owner's, for `SendTrespassAlarm`, which gives the caller's own base)
  attacks; the first of them: +1 minor crime / +2 infamy for their
  crime-tracking factions, the player +1 minor crime. Warnings at 0, 10,
  20, 30 s, the alarm at 40 s. Checked in the viewer: Boone in
  `NovacBoonesRoom` says "You need to leave." four times and shoots at 40 s.
- Crosshair red (`world::living::crosshair_red`, `00579690`): not while
  dead; a living person while sneaking; else anything but creatures whose
  owner the player may not take from (beds and chairs alike). Doors:
  `None` (their rule asks `00518f00`, not traced).
- `--freeze-ai` holds everyone's AI (the console's `tai`); scripts given
  with `--run` can still place people. `--no-hud` also hides the roll-over
  line.
- Viewer testing: `--run "LINE"` runs script lines once loaded (as the
  console, after the player's position is known), `--wait SECONDS` delays
  `--screenshot`.
- Idles (`world::idles`, read from the exe: `005ff2e0`, `00600950`,
  `008dafd0`, `008dab40`): `IDLE` records form a tree (`ANAM` parent,
  previous sibling; `MODL` a `.kf` or a folder; `CTDA`s; `DATA` byte 0 the
  body section (0 base loop, 7 special, 0x14 whole body, 0x15 upper body)
  with 0x40 loose and 0x80 blocking, bytes 1–2 loop min/max, i16 at 4 the
  replay delay). The roots are the top-level, non-loose idles in the
  actor's skeleton's `IdleAnims` folder (`Characters\_Male\IdleAnims`).
  The walk: a node whose conditions fail gives nothing; one that passes
  tries its children in order, first answer wins; with none it gives itself
  if it's an animation not waiting out its replay delay, or if it's a
  blocking folder (which means "no idle" and stops the search: `Sitting`
  and `SitDown` are blocking, so seated people never get standing idles);
  no random choice beyond `GetRandomPercent`. Asked once a second
  (`fAIIdleWaitTime` 1.0) of free time within `fAIIdleAnimationDistance`
  (2000) × bound radius ÷ 64 of the player, in sit states 0/4/9 only;
  a section-0 answer becomes the base loop, others play their loop count
  (0 if a count is 0, 255 for ever, else random(min, max) − 1 more times).
  `GetSitting` 1 (loading) gives the seated loop, 2 the entry, 3 seated
  idles (relax, eat, talk), 4 the exit. `IsCurrentFurnitureObj` takes form
  lists (`0059dfe0`). `nvinspect <Data> idles [WORD]` prints the tree with
  each idle's DATA.
- Furniture (`world::furniture`, `viewer/src/sitting.rs`; read:
  `009213e0`, `00921e80`, `00904f50`, `0092a9b0`, `005686b0`, `00509920`):
  models carry `BSFurnitureMarker` (offset, heading u16 thousandths of a
  radian, marker number); the base's `MNAM` says which marker indexes may
  be used (bit i), 0x40000000 sitting, 0x80000000 bed (Doc's chair
  0x40000004: front only). Someone heading for furniture takes the nearest
  free usable marker (its world position: the furniture's transform on the
  offset; heading: the furniture's + the marker's, kept in thousandths) and
  it's theirs from then on (`IsCurrentFurnitureRef` true while they walk;
  `GetSitting` 0). They walk to it (`fAIFurnitureDestinationRadius` 5),
  turn to its heading, and within 40 units the sit procedure runs: sit
  state 0 → 1 (the seated loop from the tree at `GetSitting` 1) → 2 (the
  entry at `GetSitting` 2: `Chair_ForwardEnter.kf` from marker 14,
  `Chair_LeftEnter`/`RightEnter`/`BackEnter`, women's `ChairSkirt_…`;
  none: give up) → put exactly on the marker facing its heading → 3 while
  the entry plays → at its end heading += `fFurnitureMarkerNNHeadingDelta`
  → 4 seated (`GetSitting` 3). Getting up (`StandUp`) waits for a playing
  seated idle, then 4 → 5, heading −= HeadingDelta, the exit plays
  (`GetSitting` 4: `Chair_ForwardExit.kf`, fast exits when alerted or
  attacked, Doc's `SpecialIdle_NVDocChairFrontExit.kf` during VCG01 10–50),
  at its end heading += π and the furniture is let go. No exit: up in
  place, heading − delta + π. Beds the same through states 6–10
  (`GetSleeping` 6 → 1, 7/8 → 2, 9 → 3, 10 → 4); markers 1–9 sleep, 10–20
  and 26 sit; from 21 the entry plays and they're let go. The entry/exit
  animations don't turn `Bip01`; `Bip01 NonAccum` turns the body by exactly
  each marker's HeadingDelta, so the heading jumps line up. Someone placed
  in furniture at once (a script's `Activate`, a loaded game) sits at the
  seat: marker + (DeltaX, DeltaY, (scale − 1) × DeltaZ) turned by the
  marker's heading, heading + HeadingDelta (`00509920`; DeltaZ only for
  scales other than 1). `nvinspect <Data> sit <chair>` prints MNAM, each
  marker and its seat, the entries and exits.
- Weapons (`world::actor::Fighting`): the weapon a person fights with
  (`best_weapon`: loaded before unloaded, ranged before melee, then most
  damage — a guess) from what their record gives them (leveled lists
  without chance at level 1; a "use all" list gives everything: Sunny's
  `WithAmmoNVVarmintRifleLoot` is the rifle and its rounds), its `MODL`
  at the right hand's `Weapon` node. Put away, `<kind>holster.kf` (0.1 s)
  names the bone it hangs from (`2hrholster.kf`: `Bip01 Spine2`, on the
  back; `1hpholster.kf`: `Bip01 Pelvis`, at the hip) and the `Weapon`
  transform there (`nif::hang_weapon`: the deepest skeleton bone the file
  moves). Fighting: `<kind>aim.kf`, running `locomotion\<kind>
  fastforward.kf`, attacks `<kind><attack>.kf` laid over the aim (the
  third-person files are named as the first-person ones). Combat uses the
  same weapon (`combat::weapon_in_hand`).
- FaceGen shape: `.egm` beside a head model (`FREGM002`; vertex count,
  50 symmetric + 30 asymmetric morphs, then per morph a scale and three
  i16 offsets per vertex). Vertex += Σ control × morph, controls from the
  NPC's `FGGS`/`FGGA`; applied to the head, its parts, hair and head
  parts before skinning. `headhuman.egm` has 1,449 vertices = the
  head model's 1,211 + its `.tri`'s 238 statistical targets (the counts
  add up in every head part: eyes 49 + 196 = 245, mouth and teeth with no
  targets equal): the model's come first, the rest reshape the targets
  in file order, so a blink closes on the NPC's own lids (the reshaping is
  inferred, see "Still guesses").
- Faces that move (`nif::tri`, `world::face`, `world::lip`,
  `viewer/src/faces.rs`; from the game's code and files, findings in
  `%USERPROFILE%\nv-re\findings\lips.md`):
  - `.tri` (`FRTRI003`, the model's path with `.tri`): vertex count V,
    triangles T, quads Q, two label counts, UV count, flags (bit 0: UVs
    and texture faces follow), D differential and S statistical morphs,
    M targets, 16 reserved bytes; then V base vertices, the M targets,
    triangles, quads, UVs and texture faces, each differential morph (name
    with its length, scale, i16 × 3 per vertex) and each statistical one
    (name, K vertex indices; its targets the next K of the block). All 49
    in the archives parse; same vertex order as the model.
  - Morph names count only if they are the engine's channels
    (`00660ba0`, whole name, case ignored): 16 phonemes, 15 expressions,
    17 modifiers (the last three, head pitch/roll/yaw, aren't morphs),
    custom `VampireMorph`. Never used: `Ee` (the engine's phoneme is `Eee`),
    `Cocky`, `Disgust`, `EyeSquintLeft/Right`, `HairMorph`.
  - Applying (`00662e90`, `00662ca0`, `00662bc0`, `00662f70`): a weight
    counts only when 0 < w ≤ 1 (above 1 is skipped, not clamped: the
    `.lip`s reach 1.0485); differential `v += w × offset`; statistical
    `v[i] += w × (T − v[i])`. Opposite looks cancel (`max(0, down − up)`).
    Phonemes within `[LOD] fTalkingDistance` + 10 (1000) of the camera,
    everything else also within `fLodDistance` + 10 (500).
  - `.lip` (the voice's path with `.lip`): header 1, a size 16 too large,
    flags (bit 0 compressed); zero runs (a 0 then a u16 count); frame
    count, lead-in, per frame 16 phoneme + 17 modifier f32; 30 frames a
    second; head turns × `[Audio] fDialogueHead…Exaggeration` (2).
  - A line (`008a20d0`): lead = −offset / 30; the channels ease to 0 over
    min(0.2, lead); each frame a 1/30 s key; then 0.2 s back to rest; the
    voice starts lead + `fSpeechDelay` (0.17) later. Without a `.lip` the
    voice plays at once and the mouth stays still. Keys play from where
    the values are: `c = c(1 − t) + key × t`.
  - Blinks (`0064b630`): whenever no modifier keys wait and LookDown is
    below `fLookDownDisableBlinkingAmt` (0.25): open for a random 1.5–4 s,
    shut over `fBlinkDownTime` (0.04), open over `fBlinkUpTime` (0.14).
  - Checked in the viewer on Doc Mitchell's intro (`GSDocMitchellHouse
    --stage VCG01 0 --at 2268,2301,7327,155,0`): lips, teeth and tongue
    follow "Whoa, easy there..."; he blinks every 2.5–3 s.
- Faces (`SKIN2000.pso` and kin): colour = `(BaseMap + 2 × (FaceGenMap0 −
  0.5)) × 4 × FaceGenMap1` × vertex colour; light = ambient + light ×
  N·L + 0.5 × light × (1 − N·V)² × saturate(−L·V) (a rim term).
  `FaceGenMap0` is the NPC's own tint, shipped ready-made:
  `textures\characters\facemods\<plugin>\<form id>_0.dds` (Doc's
  `falloutnv.esm\00104c0c_0.dds`, 256², averaging 0.47: a signed tint
  around 0.5). Applied (`TextureData::plus_face_tint`, head only). `FaceGenMap1`: see
  the actors recording below. Not known: the
  tint for NPCs without a file (the game would make it from the head's
  `.egt` and `FGTS`); `headhuman_sk.dds` (flat 21,6,5) is something else.
  The rim term isn't in the viewer.
- Skin pieces (shader flag 0x400) take the race part's `ICON` texture;
  hair pieces (0x40000) are tinted (`SM3002.pso`): texture × lerp(1,
  2 × HairTint, vertex green).

- **The actors recording** (`%USERPROFILE%\nv-re\trace_actors\
  FalloutNVActors.trace`, Prospector Saloon, AI off with `tai`, HUD off).
  Frames (Present calls; the frame is the calls since the previous
  Present): Sunny 80 units `107656198`, camera 179.41,-489.03 (eye feet +
  120) heading 109.35 pitch 0; Sunny 220 units `117545072`, 47.6,-441.6
  same heading; Cheyenne front `136683359`, 48.76,-521.72, 128.36, 30
  down; Cheyenne side `146914669`, 233.01,-514.5, 218.36, 30 down; Sunny's
  face close up `159414874`, **216.0,-508.5** (from her head bone in the
  recording; not the dog-side spot) heading 90 pitch 0; first-person
  machete `166830323` at 233.01,-514.5 heading 0 (first-person pass after
  `Clear(ZBUFFER)` at 166828991). In the recording Sunny stood at
  254.89,-515.54,3457.05 facing 289.35, Cheyenne at 158.54,-608.60,3456.47
  facing 308.36: `nv-viewer … --freeze-ai --run "SunnyRef.SetAngle z
  289.35" --run "SunnyRef.MoveTo SunnyRef 23.99 -22.84 1.05" --run
  "CheyenneREF.SetAngle z 308.36" --run "CheyenneREF.MoveTo CheyenneREF
  5.18 -144.57 0.47"` (`shots\shoot.ps1`). Tools: `trace_actors\
  analyze2.ps1` (the lead's analysis plus textures per stage, states,
  bones, and the shaders the game compiles at run time); vertex buffers
  with `apitrace dump --blobs`.
- **FaceGen shape, confirmed exactly**: each FaceGen part's vertices = its
  NIF vertices + Σ (race value + NPC value) × `.egm` morph, the race's
  `FGGS`/`FGGA` for the NPC's sex (after `MNAM`/`FNAM` following the
  race's hair and eye lists). Sunny Smiles' head, hair, right eye and
  eyebrows in the game's own vertex buffers match to 0.0000 (her own values
  alone were up to 1 unit off; she looked like someone else). Hair with hat
  versions has one `.egm` per piece: `<stem><piece>.egm`
  (`hairbunnohat.egm`); without it the hair didn't follow the head.
  `world::actor::face`, `preview::actor::facegen_path`.
- **Skin texture pass, confirmed** (`SLS1005.pso`, blended DESTCOLOR ×
  ZERO over the light passes): `4 × FaceGenMap1 × (BaseMap + 2 ×
  (FaceGenMap0 − 0.5))`. `FaceGenMap0` = the face's `facemods` file (Sunny:
  256² DXT1) or, on bare arms, hands and gloves' fingers, the body tint
  `textures\characters\bodymods\<plugin>\<id>modbody<male|female>.dds` (8²
  DXT1). `FaceGenMap1` is the same 32 × 32 texture for everyone, made at
  startup: every pixel (62, 65, 62) → ×(0.973, 1.020, 0.973).
  `cellview::texture::FACEGEN_MAP1`, `world::actor::body_tint_path`.
- **Skin lighting** (shader flag 0x400; read from the passes, the point-
  light ones compiled by the game at run time): ambient (`SLS1000`), the
  directional light alone (`SLS1002`, `saturate(N·L)`) **times the image
  space's `DNAM` float 2** (2 in the saloon and the barracks, 4 in Doc's
  house: in all three recordings exactly that × the cell's colour; its
  highlight too), each point light `c × saturate(N·L) + rim × (0.5 c +
  (0.15, 0, 0)) + (0.3, 0, 0) × saturate(S(w) − S(saturate(N·L)))`, all ×
  its fade, with rim `(1 − saturate(N·V))² × saturate(−L·V)`, `S(x) = 3x² −
  2x³`, `w = saturate((N·L + 0.3) / 1.3)`, L and V per vertex. Skin always
  gets the specular pass (her arms, flags 0x82000402, did).
  `preview::cell::Shading`, `world::Hdr::skin_directional`, `game_lit.wgsl`.
- **Hair** (flag 0x40000; hair shader compiled at run time, `SM3003.vso`):
  `HairTint` = `HCLR`/255 (confirmed: (0.2588, 0.1098, 0.0588) for Sunny's
  (66, 28, 15)), mask = vertex green (the vertex shader passes the
  colour); texture = `lerp(base, layer, layer.a)` with the layer map from
  the model's texture slot 2 (`HairBun_hl.dds`); no ordinary specular
  pass; per point light `0.7 × (1 − saturate(|B·L − B·H|))³⁰ × fade ×
  colour × max(L·N_vertex, 0)`, `B = normalize(0.5 n + N_vertex)`, × mask ×
  normal map alpha; the directional light's highlight is × `ToggleNumLights.w`
  = 0 in every recorded draw.
- **Where rigid pieces hang, confirmed**: FaceGen head parts (eyes, mouth,
  teeth, tongue, hair, eyebrows) are all drawn with the head mesh's own
  skinning matrix (posed head × its skin-to-bone), whatever their files'
  top nodes say — what the upright rule gives. Other rigid models with a
  `Prn` (a creature's eyes) hang with their top node as a child of the
  bone, its transform included: Cheyenne's eyes land at posed head ×
  `eyessetblue.nif`'s top node to 0.0001 (they had stood one above the
  other at her collar). `preview::actor::rigid_transform`.
- Pieces without a shader property aren't drawn: the machete's `Weapon:0`
  (a bare bar along the spine) never is. `preview::actor::drawn`.
- **Specular needs a normal map with alpha** (all meshes): all 438
  specular passes in three frames sample DXT5 normal maps; none of the 35 lit
  draws with a DXT1 or X8R8G8B8 normal map got one (the machete's `machete_n.dds`).
  `cellview::specular_allowed`.
- **Armour addons**: an `ARMO`'s `BIPL` (form list of `ARMA`, each with
  `BMDT` slots and `MODL`/`MOD3`) is worn with it: leather armour's gloves.
  Sunny is drawn with them and without the race's hands; the gloves' bare
  fingers take the race's hand texture (the pass binds a 256² DXT1, as
  `HandFemale.dds`) and the body tint. `world::actor::Armor::addons`.
- First-person weapon placement confirmed: the machete's `Weapon` node
  relative to the camera in `1hmaim.kf` is the game's to 0.1 unit and the
  same axes.

### Dialogue

- Topics (`DIAL`) hold their lines (`INFO`) in a child group (type 7;
  `esm` records each line's topic, `LoadOrder::in_topic`). A line:
  `DATA` (flags: 0x01 goodbye), `QSTI` quest, `TRDT` + `NAM1` per
  response (emotion, value, response number, text), `CTDA` conditions,
  `TCLT` follow-up topics, `NAME` added topics, `RNAM` player prompt.
- `CTDA`: byte 0 = comparison (top 3 bits: = ≠ > ≥ < ≤) and flags (0x01
  OR with the next, 0x04 value is a global); value f32 / global; function
  u16; two parameters; run-on (0 subject, 1 target); reference.
  Function numbers = the game's script opcodes − 0x1000, read from its
  function table (`world::functions`, 640 names; the full dump with
  short names is `%USERPROFILE%\nv-re\script_functions.txt`).
- Voice files: `sound\voice\<plugin>\<voice type>\<quest EDID>_<topic EDID>_<line's
  form ID, 8 hex>_<response>.ogg`, the quest and topic whole when together at most
  25 letters, else cut to 10 and 15 (`vfreeformgoodsprings_hit_00126f2b_1.ogg`,
  `vfreeformg_greeting_00107220_1.ogg`; true of all 16,293 such files in
  `Fallout - Voices1.bsa`; `world::dialogue::voice_name_parts`).
- `nvinspect <Data> dialogue <NPC>` lists a person's lines, conditions,
  responses and follow-ups, and the greeting a new game would give.
- A line's `DATA` byte 2 flags: 0x01 goodbye, 0x04 say once (all of Doc's
  `VCG01Intro` lines; without it he repeats his first line). A line has
  two result scripts: `SCHR`/`SCTX` as it starts, then `NEXT`, then the
  one run after it's said (most of the intro's `SetStage`s are the
  second).
- `SayTo` only has someone say a line (no dialogue menu; the game and the
  intro's timers carry on); `StartConversation` opens the dialogue.
- A line is only available while its quest (`QSTI`) runs and the
  quest's own conditions (`CTDA` before its first `INDX`) pass for the
  speaker: `vDialogueEDE` runs from the start and its lines have no
  conditions of their own, but the quest's say `GetIsID` one of ED-E's
  three records. Without this Sunny Smiles greeted the player with a
  beep; with it she says "Cheyenne, stay…", as in the game.
- Dialogue packages (type 15): `PTDT` target (kind i32, form: 0x14 the
  player, distance: 256 for Sunny's greeting), `PKDD` (field of view
  f32, topic: 0xC8 = greeting). Once at the package's place, the person
  starts a conversation when the target comes within that distance.
- A topic's `DATA`: kind (0 ordinary, 1 conversation, 2 combat, 3
  persuasion, 4 detection, 5 service, 6 misc, 7 radio), flags (0x01
  rumors, 0x02 top-level); `PNAM` priority (50 default); `QSTI` the
  quests with lines under it. Sunny's questions are 0 / 0x02 at 75–100,
  follow-ups only offered after a line ("How many are there?", her
  "Goodbye.") 0 / 0. Her greeting's `TCLT` lists her top-level questions
  and "Goodbye." in exactly descending priority, and "That's all I wanted
  to know…" has no follow-ups (it leads back to the main list). The menu
  (`world::dialogue::next_choices`): a line's `TCLT` as stored; after a
  line without, the main list (`menu_topics`): the opening line's
  follow-ups + top-level topics + learned ones (`AddTopic`, a line's
  `NAME`), by priority, those the speaker answers now. `nvinspect
  dialogue` prints it: Sunny's and Trudy's match the game's.

### Message boxes and making the character

- `MESG`: `DESC` text, `FULL` title, `DNAM` flags (0x01 a message box
  that waits for a button), `ITXT` buttons, each followed by its own
  `CTDA`s. `ShowMessage` fills the text's `%.0f`/`%g` places with the
  values after the message (not in the function table's entry; the
  parser allows nine). `GetButtonPressed` gives the pressed button's
  number once, then -1. `VCG01ChooseSexMessage` ("Mister" / "Ma'am") →
  `SexChange` (parameter type 18: 0 male, 1 female).
- The opening's menus (`world::chargen`; the name, tag skills and traits are
  the game's own, `viewer/src/game_menus/`, the SPECIAL a text panel in
  `viewer/src/menus.rs`):
  `GetPlayerName` (stage 15) the name, default `sDefaultPlayerName`
  "Courier"; `ShowRaceMenu` the face (1036, still skipped);
  `ShowLoveTesterMenuParams 40` SPECIAL (40 points, each 1–10, the player
  record starts each at 5); `SetTagSkills 3 1` three tags (+
  `fAVDTagSkillBonus` 15); `ShowTraitMenu` up to `iTraitMenuMaxNumTraits`
  (2) of the perks with `DATA` trait 1, playable 1, hidden 0 (ten in the
  base game; picked traits become the player's perks).
- Names shown: the `AVIF` records' (`AV` + script name: `AVSmallGuns`
  "Guns", `AVThrowing` "Survival", `AVBigGuns` "Big Guns - OBSOLETE",
  not in the menus). New Vegas scripts write `Guns` and `Survival`
  (`script::actor_value` takes both names).
- The player's skills: the player record's `DNAM` (Melee 30, Guns 25 at
  SPECIAL 5) isn't what a new character has, so they're worked out:
  `fAVDSkill<Name>Base` (2) + 2 × attribute + ⌈Luck / 2⌉ (+ 15 tagged).
  The 2 × and the Luck term, and which attribute governs which skill, are
  the documented formula, not read from the game.

### Items, containers and trading

- Value and weight per item type (`world::items`): `MISC`/`KEYM`/`IMOD`
  `DATA` value, weight; `WEAP` `DATA` value, health, weight, damage i16,
  clip u8; `ARMO` value, health, weight; `AMMO` `DATA` speed, flags, 3
  unused, value, clip and weight in `DAT2` at 8; `ALCH` weight in `DATA`,
  value first in `ENIT`; `BOOK` flags, skill, value, weight; `CMNY`,
  `CCRD` value only. `InventoryWeight` (46) sums them.
- Equipping (`GameState::equip`): one weapon in hand; clothes take off
  what's worn on any shared `BMDT` slot. The inventory screen (I or Tab)
  equips and uses.
- Aid (`world::items::effects`, `use_item`): `EFID` magic effect, `EFIT`
  magnitude, area, duration, range, actor value i32 at 16, then that
  effect's `CTDA`s; `MGEF` `DATA` (72 bytes): flags at 0 (0x04 harmful),
  archetype at 64 (0 value modifier, 34 value and limb conditions), actor
  value at 68. The Stimpak: `RestoreHealthStimpak` 30 at once outside
  hardcore without Fast Metabolism (36 with), `RestoreHealth` 5/s for 6 s
  in hardcore. The consume sound is `ENIT`'s form at 16.
- Effects (`world::magic`, `GameState::active_effects`): `MGEF` flag 0x02
  *recover* = a modifier while it lasts (Buffout's `ChemIncSTBuffout`
  0x72: +2 Strength for 240 s; `ChemIncHealthBuffout` +60 max health);
  without it the change stays, at once or magnitude per second for the
  duration (`RestoreHealth` 0x70: Nuka-Cola 2/s for 25 s). 0x04
  detrimental lowers; the resisting value is `DATA` i32 at 16 (Damage
  Rads → 20 Rad Resistance). Archetype 1 script effects run the script at
  `DATA` 8 on the target: `ScriptEffectStart`/`Update`/`Finish`
  (`NukaColaAddCapEffectScript`: `additem caps001 1`). Rads (54) and the
  hardcore needs (73–75) count up with damage: the game's scripts read
  `player.getav RadiationRads >= 200` and cure with `player.RestoreAV
  RadiationRads 1000`. `SPIT` type 1 disease, 4 ability, 10 addiction
  last while held (`AddSpell`); `CastImmediateOnSelf` (the `Generic`
  script's `PlayerConcussed`), `Cast`, `RemoveSpell`, `Dispel`,
  `HasMagicEffect`, `IsSpellTarget`. `GetPermAV` = base + held effects;
  `GetBaseAV` without effects. Guesses: changes per second applied
  smoothly; resistance as (1 − value/100); zero-duration script effects
  run start, update and finish at once; other spells given with
  `AddSpell` cast once. Concussion (archetype 33) and limb conditions do
  nothing yet.
- **Reputation and karma** (`world::reputation`, read from the code,
  `%USERPROFILE%\nv-re\findings\reputation.md`): `REPU` `DATA` f32 = the
  most (NCR 80, Legion 100, Goodsprings 15); fame (script type 1) and
  infamy (0) from 0; `AddReputation`/`RemoveReputation` by
  `fReputationBump…` (size 1–5: 1, 2, 4, 7, 12) clamped to 0..most,
  `…Exact` by the amount, `SetReputation` unclamped and silent;
  `GetReputationPct` = value / most; levels 1 at 0.15, 2 at 0.5, 3 at the
  most; titles by infamy × 4 + fame (`sRepTitlePos<F>Neg<I>`: Neutral,
  Accepted, Liked, Idolized, Shunned, Mixed …); `GetReputationThreshold`
  per axis 0 mixed / 1 good / 2 bad as `00616a90` answers. Notices
  "<name>\nFame Gained!" (`sRepPositiveGain` …) and the new title when a
  level changes. Karma (actor value 23): `RewardKarma` (always the player)
  clamped ±1000 with `sKarma{Major,Minor}{Gained,Lost}`; bands very evil
  ≤ −750, evil ≤ −250, good ≥ 250, very good ≥ 750; the player's kills of
  people in a crime-tracking faction (`FACT` `DATA` 0x100) by the victim's
  record karma: good −50, very good −100, evil +100, very evil +2. Karmic
  titles `sKarmicTitle<Good|Neutral|Evil><NN>` by level.
- **Crime** (`world::crime`, findings §3): owner = the reference's `XOWN`,
  a door's partner's, (not people, furniture, doors, activators) its
  encounter zone's (`ECZN` `DATA` form at 0), else the cell's `XOWN` or
  zone's; the player may take what's unowned, theirs, or a faction's
  they're in (rank not checked). Taking someone else's item or from their
  container ("Steal", `sSteal`): −5 karma (`fKarmaModStealing`) unless the
  owner is evil (faction `DATA` 0x02, or record karma evil); if the victim
  (the person stolen from, else the owner or a member of the owning faction
  in the cell) detects the player (> 0): +1 minor crime and +2 infamy
  (`fReputationMinorCrimeNeg`) per crime-tracking faction, and a second
  witnessed theft the same day of the month makes the victim attack
  (`iStealWarnings` 2 warnings counted; the warning package isn't
  walked). `SetPCEnemyofFaction` / `ClearFactionPlayerEnemyFlag` /
  `GetPCEnemyofFaction`: a faction holding the player as an enemy makes its
  members enemies. `IsTrespassing` (owned, not public `DATA` 0x60, no
  `XGLB`, below the cell's `XRNK`), `GetMinor/MajorCrimeCount`. Pickpocketing and trespass warnings: see
  "Sleeping and waiting" above. Not done: stolen marks on items,
  assault and murder crimes (+30 infamy).
- Misc statistics (`world::stats`, `GetPCMiscStat`, `ModPCMiscStat`): the
  game's 43 names by number (its table at `01189280`: 0 Quests
  Completed, 1 Locations Discovered, 2 People Killed, 3 Creatures Killed,
  4 Locks Picked, 5 Computers Hacked, 12 Speech Successes, 15 Books Read,
  23 Times Slept, 31 Speech Failures, 35 Total Things Killed …); bumped
  where the code bumps them (places, locks, terminals, books) and for the
  player's kills (where the game counts kills isn't traced). Not counted
  yet: speech checks, chems, sleeping. Saved.
- Skill books (`world::items::read_book`, read from the code `00515040`,
  `0088c830`): `BOOK` `DATA` byte 1 the skill (from Barter, −1 none);
  reading one while the skill's permanent value is under 200 adds
  `fBookPerkBonus` (3; exe default 1) after perk entry point 11 (Adjust
  Book Skill Points: Comprehension + 1), for good, and uses the book up.
  The notice is the game's `sSkillIncreasedNum` ("Science increased by 3").
- Weapon condition: full (1.0) unless `SetWeaponHealthPerc` /
  `ModWeaponHealthPerc` changed it; the damage formula's condition term
  uses it; fists count as 100% for `GetWeaponHealthPerc` (a guess; the
  tutorial warns below 25). No wear from firing yet.
- `GetDetected` (a guess): fighting the target, or within
  `fSneakMaxDistance` (× `fSneakExteriorDistanceMult` outdoors); no sight
  lines, light or sneaking. `ForceActiveQuest` is kept (`active_quest`).
- E on a container opens the game's container menu (see "The game's
  menus"; closing runs the container's `OnClose` block).
- Merchants: `ShowBarterMenu` on the speaker; their goods are their own
  things and the container their reference names in `XMRC` (Trudy's
  `00104DF3`: 130 caps, food and drink), those their `AIDT` services
  cover; prices, the offer and its settling read from the barter menu's
  code (`world::barter`, see "The game's menus"; `nvinspect <Data> barter
  TrudyREF`). Caps are `Caps001` (`0000000F`).

### Fighting

- Weapons (`world::combat::Weapon`): `DATA` value, health, weight, damage
  i16, clip u8; `NAM0` the ammo: a form list (`AmmoList556mm`: five kinds,
  `LNAM`s) or one `AMMO`; `DNAM` (204 bytes, laid out as published and
  checked on the 9mm pistol, varmint rifle, machete, service rifle):
  animation type u32 at 0 (0–2 melee), reach f32 at 8 (machete 0.5),
  ammo use u8 at 14, min spread at 16, projectile at 36, projectile count
  u8 at 42, min/max range at 44/48, attack shots per second at 88, reload
  time at 92, skill i32 at 104 (41 Guns, 38 Melee); `CRDT` critical
  damage u16, multiplier f32; first `SNAM` the attack sound.
  `nvinspect <Data> show <WEAP>` prints all of it.
- **Damage, read from the code** (`%USERPROFILE%\nv-re\findings\
  hits.md`; `world::combat`): weapon damage (`00644ce0`) = ((base × S ×
  power) + A) × C × the attacker's scale; base the weapon's (fists 1); S =
  0.5 + 0.5 × skill/100 × R for guns and melee weapons, R alone for
  hand-to-hand and fists, R = 0.2 + 0.8 × arms intact (melee only); A =
  Melee Damage (actor value 17: 0.5 × STR) for melee weapons, Unarmed
  Damage (0.5 + 0.05 × Unarmed) for hand-to-hand and fists; C = 1 above 75%
  condition, else 1 − 0.67 × (0.75 − c) (the `fDamage…WeapCond…` settings
  aren't used for it); power attack × `fDamagePowerAttackBonus` 2.
  Criticals (`009b7060`): chance % = (0 + 1 × Luck) × the weapon's `CRDT`
  multiplier, per mille; the player sneaking unseen (the target's
  detection < 1): certain; adds `CRDT` damage (fists: × 1 of the hit).
  Armour (`009b5a30`): at least `fMinDamMultiplier` 0.2 of the hit stays;
  × (1 − DR/100) with DR = actor value 18 + worn armour (`ARMO` `DNAM`
  i16 at 0; how worn armour sums isn't traced), at most `fMaxArmorRating`
  85%; − DT (actor value 76 + worn armour's f32 at 4 + perks 56/58). A
  sneak attack then × 2 (guns) / × 5 (melee). Ammunition effects (`AMMO`
  `RCIL` → `AMEF` `DATA`: kind 0 damage / 1 DR / 2 DT / 3 spread / 4 wear /
  5 fatigue, operation 0 add / 1 multiply / 2 subtract, value): DR and DT
  ones on those, damage ones after the threshold (the 5.56mm hollow point:
  DT × 3, then × 1.75). Creatures bite for `CREA`
  `DATA` damage (i16 at 8: the tutorial gecko `VCG02CrGecko` 5, health 20,
  aggression 2). Not done: blocking (DT + 5 + 0.3 × skill), people's aim
  (NPCs add 15° × wobble to the cone), difficulty (Normal, × 1).
- **Body parts and limbs** (`world::body_parts`, read from the code; `findings\
  hits.md` §3–§5): `BPTD` parts (`005e4d80`/`005e4100`, read in order: `BPTN`?
  `PNAM`? `BPNN` `BPNT` `BPNI`? `BPND` then `NAM1`–`NAM5`; a part missing a
  required one stops the loading): `BPNN` the node the part starts at, `BPND`
  (84 bytes, as-is at part `+0x5c`) damage mult f32 at 0, flags u8 at 4 (0x01
  severable, 0x08 explodable, 0x40 own explode chance), part type u8 at 5,
  health % at 6, actor value i8 at 7, explode chance at 9; kept by type in 15
  slots, a later one of a type replacing it (`005e5220`). Who uses which
  (`005f0f80`, `005fa2e0`, `005e53a0`): people `DefaultBodyPartData` 0x1D
  (head ×2 at 20%, torso 60%, arms/legs 25%), the player `PlayerBodyPartData`
  0x1C (head ×1 at 75%, legs 150%, torso 255%, right arm from `Bip01 R
  Forearm`), creatures `CREA` `PNAM` else 0x1D. Where a hit lands
  (`008b3ef0`): from the struck node (the bone of the Havok body met) up its
  parents, the actor's root not counted: the first that is a part's `BPNN`
  node (looked up by name when the 3D loads, `0092b830`) gives the part, a
  node whose parent is the weapon node gives 14, none → no part (a gecko's
  pelvis: its torso starts at `Spine1`). What it does (`009b6620`): the hit's
  multiplier (`+0x5c`, from 0) = max(itself, the part's mult) for ranged hits,
  1 for melee/fists/bites; limb damage `100 × damage ÷ (health%/100 × base
  health)` (`00647a30`; base health ≤ 0 → the damage; 0% → 0), × 0.5
  `fCombatPlayerLimbDamageMult` for the player, × the weapon's `DNAM` f32 at 116
  (9mm 1, machete 2), from the damage after armour, before the multiplier;
  part 14 moves all the health damage onto the weapon. Then (`009b73d0`) if
  the multiplier > 0: a sneak attack's ×2/×5 when it's ≥ 1, then damage × it;
  no part → nothing multiplied, no sneak bonus. The part (`0089a760`): the
  target's perks' entry 6 "Adjust Limb Damage", then its actor value (25–31)
  is damaged; crippled when it goes from > 0 to ≤ 0. A person (not the
  player) crippled in the right arm (5, 6), or with a two-handed weapon
  (`00646cb0`: anim types 2, 5, 6, 8, 9 — not 7) the left (3, 4), drops the
  weapon, unless Ignore Crippled Limbs (72); a critical on part 14 drops it
  (`iWeaponCriticalHitDropChance` 100). Legs (`00647d10`): one crippled ×
  `fMoveOneCrippledLegSpeedMult` 0.85, both × `fMoveTwoCrippledLegsSpeedMult`
  0.75 (actor value 72 cancels), for the player and people. `GetHitLocation`
  (`005a3c30`: the last hit's part, -1 none), `GetKillingBlowLimb`
  (`005a3e10`, -1). "Value and parts" effects (archetype 34, the Stimpak's
  `RestoreHealthStimpak`; `0082b970`): health by the magnitude and every part
  25–31 by magnitude × `fMagicVACNoPartTargetedMult` 0.1 (a part aimed at from
  the Pip-Boy gets the full magnitude, health × `fMagicVACPartTargetedMult`
  0.1: not done). `nvinspect <Data> show <BPTD|NPC|CREA>` prints the parts;
  `nvinspect <Data> hits <ID>` maps what straight shots from in front meet
  (Doc Mitchell: head 14 cells, torso 46, arms 10/11, legs 42/45).
- Hit shapes in the viewer: the skeleton's ragdoll capsules on the posed bones
  (the same `bhkBlendCollisionObject` bodies the living carry). Shots must meet
  one; melee reaches by the bounds and lands where the view meets a capsule.
- Compressed B-spline channels (`NiBSplineCompTransformInterpolator`) whose
  handle is 0xFFFF are unused (the files write USHRT_MAX, with ±FLT_MAX ranges;
  the gecko's idle, with more than 65535 shorts, made the pelvis scale NaN and
  geckos invisible before).
- Health: full = base + `fAVDHealthEnduranceMult` 20 × END +
  `fAVDHealthLevelMult` 5 × (level − 1) for the player (200 at the start),
  `fAVDNPCHealthEnduranceMult` 5 × END for people, the record's for
  creatures;
  current = full − damage taken (`GameState::damage`). At 0: dead
  (`GameState::dead`, `GetDead`, `GetDeadCount`), `OnDeath` runs, fights
  with them end.
- `Runner::hit`: the target's `OnHit` (any or naming the attacker) and
  `OnHitWith` (any or naming the weapon) blocks run (the tutorial's
  bottles: `VCG02TargetSCRIPT` counts `OnHitWith`); people and creatures
  take damage and fight back (`GameState::combat`, `IsInCombat`,
  `StartCombat`, `StopCombat`). `EquipItem` (one weapon in hand),
  `UnequipItem`, `GetEquipped`; `AddItem` of a leveled list gives what it
  picks (`condnvvarmintrifleloot`).
- Hostility (`world::factions`): `FACT` `XNAM` (faction, modifier i32,
  reaction u32: 0 neutral, 1 enemy, 2 ally, 3 friend); people's `SNAM`s
  (from the template when `ACBS` template flags have 0x04; aggression
  from it with 0x10: Powder Gangers' 0x03BE has both); the player is in
  `PlayerFaction` (`0001B2A4`). The Powder Gangers' records leave them
  neutral to the player; the game's scripts make them enemies (`SetEnemy
  PowderGangerFactionNV PlayerFaction` in `VFreeformNCRCFScript`,
  `VES04FactReactPGDeathScript`), so `SetEnemy`/`SetAlly` (flags 1 =
  neutral / friend for that side), `AddToFaction`, `RemoveFromFaction`,
  `SetFactionRank`, `GetFactionRelation` are carried out and kept in the
  state. Attacks on sight by aggression: 0 never, 1 enemies, 2 enemies and
  neutrals (geckos), 3 anyone. `nvinspect <Data> hostile <CELL or person>`.
  Guesses: an enemy relation winning over others, the aggression rule as
  the editor words it; reputations and crimes not counted.
- The player's attacks (`viewer/src/combat.rs`, from `findings\hits.md`):
  guns reach their projectile's range (`PROJ` `DATA` f32 at 12: 10000 for
  the 9mm bullet), a shot's pellets (the weapon's count, or the ammo's
  `DAT2` count × ammo use) each carry damage ÷ count and fly within the
  weapon's min spread (degrees, after ammo spread effects) at r = U(0,
  cone), θ = U(0, 2π) off the view; melee reaches the weapon's `DNAM`
  reach × 128 (64 unarmed) between the bodies' edges. All shots are
  hitscan along those lines (the game's bullets are; other projectiles
  fly and fall at 686.6 × the `PROJ` gravity: not here).
- **Hit effects, read from the code** (`%USERPROFILE%\nv-re\findings\
  hiteffects.md`; `world::impacts`, `cellview::impacts`, `viewer/src/hiteffects.rs`;
  `nvinspect <Data> impacts <WEAPON|IPDS|IPCT|PERSON|CREATURE> [MATERIAL]`):
  - Records: `IPCT` (`0058dea0`): `MODL`, `DATA` 24 bytes (duration, orientation
    0 normal / 1 back along the shot / 2 its reflection, angle threshold, placement
    radius, sound level 0 loud 1 normal 2 silent, flags 0x01 no decal), `DODT`
    (36: min/max width, min/max height, depth, shininess, parallax scale, passes
    u8, flags u8 0x01 parallax 0x02 blend 0x04 test, colour), `DNAM` `TXST`, `SNAM`,
    `NAM1`. `IPDS` (`0058ec00`) `DATA`: 12 impacts by material (table `0118c4a0`):
    stone, dirt, grass, glass, metal, wood, organic, cloth, water, hollow metal,
    organic bug, organic glow.
  - Material of a surface (`0058e8f0`, the shape's Havok material & 0x1f,
    `00c84f10`): 0/10/19 stone, 1 cloth, 2/18 dirt, 3/24 glass, 4 grass, 6/7
    organic, 8 water, 9/12/30/31 wood, 16/17/22/23/25/28/29 hollow metal, else
    metal. Collision keeps it per triangle (`nif::CollisionPart::material`,
    `physics::Collider::material`); land without one is dirt (the game's default
    in `00457880`; the land texture's own `HNAM` material isn't looked up yet).
    People and creatures: `NAM4` (default organic, `00601570`/`005f7a40`); worn
    power armour (`BMDT` 0x20, `0092baa0`) makes body hits metal, a power armour
    helmet (head or hair slot) head hits (parts 1, 2).
  - Which set: the hit's weapon's `INAM`; a creature without one its `CNAM` (and
    its "weapon" sound, type 9); a person without one the fists' (`000001F4`).
  - A hit on someone (`0088e1e0`): set[their `NAM4`]'s `SNAM` and `NAM1`, plus
    set[metal]'s with power armour on the body, each played only if the camera is
    nearer than that sound's largest distance (the player hit: always). Blood
    (`0088e8d0`, with gore: not `bDisableAllGore`, not both `ACBS` 0x800 and
    0x1000): the attacker's *held* weapon's set at the hit material, else
    `DefaultImpactDataSet` (`00000276`); the body part's own set (`BPND` +68) for
    a wall spatter (chance `fCombatEnvironmentBloodChance` 0.75, a ray 512 units
    along the spray tipped down by 0.5; spray = normalize(2 × direction + U(−0.8,
    0.8)³)); all only within `fGunParticleCameraDistance` (data 2048) of the camera.
  - A shot on the world (`009c20e0`): the weapon's set at the surface's material:
    its two sounds (no distance test), its model (within 2048 and in view) along
    the orientation, a decal.
  - Effect models (`006890b0`, `00689310`): Z along the direction, a random turn
    of U(0, 1) *radians* about it; they last their controllers' longest (stop −
    start) (`00689840`, at least 0.1 s), else the impact's duration (world) or 1 s
    (blood). The ballistic ones are billboarded quads with transform and alpha
    controllers, 0.07–0.27 s.
  - Hurt line (`0089a760` → `009839b0`): topic `Hit` (`000000DD`) when knocked
    out, or damage ÷ health (whole number) > `fCombatSpeakHitThreshold` (data
    0.01), else chance `fCombatSpeakHitChance` (data 0.01); only when a shared
    cooldown has run out (`[Audio] fDialogHitSoundCooldownMin`–`Max`, 2–4 s,
    restarted by any hit after it ran out) and 1.5 s after any combat line.
    Death (`0089d900`): a creature's type-8 sound (`CSDT`), else topic `Death`
    (`000000EF`). Creature type 7 ("hit") sounds are never played by the engine.
    Combat topics by subtype (`0119a238`): 0 Attack, 1 PowerAttack, 2 Hit, 3 Flee,
    4 FireExplosive, 5 AvoidThreat, 6 Death, 7 CombatIdleChatter, 8 GroupStrategy.
  - The player hit (`0089a760`): image space modifier `GetHit` (`00000162`) at
    strength `fGetHitPainMult` 1.5 — only a radial blur (0.046 → 0 over 0.4 s,
    centred on the attacker); no red flash in New Vegas. Strength (`00531720`,
    `00b8cb30`): each value goes `strength` of the way to `v·mult + add`; tint and
    fade alpha × strength (≤ 1), blur × strength (≤ 7). Screen blood: max(0,
    trunc(2 + 0.1 × damage + U(−1, 1))) drops (`00647ae0`).
  - Guesses: which template flag carries `NAM4`/`CNAM`/sounds (taken as "use
    model", 0x40); worn armour of people who haven't changed (the look's first
    per slot); the health in the hurt-line share taken after the damage; people's
    hits measured at where the target stands (their shots have no point on the
    body here); the "talking/scene" checks that block hurt lines (`008a67f0`,
    `009336c0`) aren't modelled; the player says no combat lines (the code's player
    branch isn't traced).
- **The combat AI, read from the code** (`%USERPROFILE%\nv-re\findings\
  combat_ai.md`; `world::combat_ai`, `viewer/src/fighting.rs`):
  - Noticing (`008e40d0`): each actor's detection run every
    `fDetectionTimerSetting` 0.3 s in combat, else 0.3 s + random[0,
    min(frame × actors, 5 s)]; none 8192 or more from the player (`01084d28`).
    Its value for the player and every other loaded actor (`world::
    detection`; nearer than 2 units 100). When a value first rises above
    `fSneakNoticedMin` −20 (considered once per rise, `008f5480` /
    `008ff350`), a fight starts if aggression and reaction say so
    (`attacks_on_sight`); with `iCombatTargetPlayerSoftCap` 15 already on the
    player only a value above 0 joins. Allies (`008b0970`): someone seen
    (> 0) fighting X, X detected at 1 or more, Assistance (AV 57, `AIDT`
    byte 14) 1 helps allies, 2 friends and allies; frenzied never.
    Unaggressive actors run from someone who'd attack them when
    `threshold[Confidence]` (`fConfidenceCowardly` 1000, Cautious 0.375,
    Average 0.1875, Brave 0.0375, Foolhardy 0) > their strength ÷ theirs,
    strength = attack power × health ÷ (1 − min(armour/30, 0.99))
    (`008acbe0`). Being hit starts a fight at once (as before).
  - Combat styles (`CSTY`, `00505180`): `CSTD` 92 bytes (dodge 0/1, block 36,
    attack 37, recoil/unconscious/hand-to-hand attack bonuses f32 40/44/48,
    power attack u8 52 + bonuses 56/60, directions 64–68, hold timer
    72/76, flags u16 80), `CSAD` 21 floats (10/11 block skill mult/base,
    12/13 block (not) under attack, 14/15 attack skill mult/base, 16/17
    attack (not) under attack, 18 attack during block, 19/20 power attack
    fatigue base/mult), `CSSD` 64 bytes (cover radius, take cover, wait /
    wait-to-fire / fire timers, range mult min 32, restrictions u32 40,
    range mult max 44, max targeting FOV 48, combat radius 52, semi-auto
    delay mults 56/60); a shorter `CSSD` keeps the rest; form version < 12
    sets both range mults to 1. `ZNAM` on the actor (its template's with
    "use AI data"), else form `0000003D` (`DefaultCombatstyle`).
  - Ranged (`009a9180`, `009d3b80`): band min = `DNAM` min range × style
    min mult, optimal = max range × style max mult (not with "range fixed",
    flags2 0x20), absolute max = optimal × `fCombatAbsoluteMaxRangeMult`
    (data 4); with a projectile, optimal ≤ its reach × 0.85, absolute max
    ≤ its reach, min ≤ optimal − 256 (≥ 0). Projectile reach (`009a7f00`):
    hitscan (flag 0x1) or no gravity → `PROJ` range; else speed² ÷
    (gravity × 686.61), lobbers gravity 1. 9mm: 256 / 768 / 3072. First a
    0.1–0.5 s wait; out of the band 2 s (0.1 s too far beyond the absolute
    max) → a spot in the band (fast walk; run beyond the max; tried at most
    every 5 s), else a 128–256 step, else (beyond the max) run at the target
    until within 128; target unseen 0.5 s within 512 → go to it; in the
    band every 2–5 s: 75% strafe 64–192 units (128–384 in sight) at a fast
    walk, otherwise crouch/stand (not done).
  - Fire (`009d0a30`): only within the style's targeting FOV (CSSD 48), and
    within the aim arc max(`DNAM` 100, 15°) across, both ways (`009d1de0` →
    `009a6d70`: ±7.5° on vanilla guns), or nearer than the band's min.
    Semi-automatic: the next shot once the attack has ended and the delay
    started with the shot (random[`DNAM` 128 × CSSD 56, `DNAM` 132 × CSSD
    60], 0–0.3 s on vanilla guns) has run out (timer +0x38). Automatic
    (flags1 0x02 or flags2 0x200): `fAutomaticWeaponBurstFireTime` 1 s
    bursts, `…CooldownTime` 1 s pauses; within a burst shots at `DNAM` 64
    (inferred).
  - Melee (`009a69c0`, `009a64d0`, `009cc240`, `009cc510`): reach = melee
    weapon `DNAM` reach × `fCombatDistance` 128, a gun 128, a creature its
    `RNAM` (giant, type 7, × 2), a person's hands `fHandReachMult` 0.5 ×
    128, all × scale; gap = distance − both collision radii (people 20.25;
    creatures from their skeleton's `BSBound`); run while gap − reach >
    `fCombatMinEngageDistance` 64, then fast-walk (walk × 1.5, `00981570`),
    in reach when gap < reach. After each attack (and when a hold ends):
    attack = (attack% + recoil × CSTD 40 + unconscious × 44 + unarmed × 48
    + CSAD 14 × skill/100 + CSAD 15) × (CSAD 16 / 17 target attacking or
    not) × (CSAD 18 while blocking), + 100 against a target not fighting
    that isn't the player; block = (block% + CSAD 11 + CSAD 10 × 0.5) ×
    (CSAD 12 / 13) × `fBlockScoreNoShieldMult` 0.5 × (1 attacking / 0.25),
    0 without a block animation; hold = max(100 − both, 30), 0 while
    holding; roll % trunc(sum) (100 below 1): attack, block, else hold
    random(CSTD 72, 76). Power attack (`00644810`): chance + bonuses + CSAD
    19 + CSAD 20 × (1 − fatigue share) ≥ roll % 100 (0 never, ≥ 100
    always). Skill: the weapon's (`DNAM` 104), Unarmed without one, a
    creature's combat skill (`DATA` byte 1). The tutorial gecko: attack 68
    (51 with the player attacking), hold 32, holds 0.1–0.35 s.
  - Losing the target (`00987220`, `0098add0`): unseen (detection ≤ 0) for
    `fCombatDetectionLostTime` 15 s → search; unseen past
    `fCombatTargetLostRemoveTime` 30 s and never seen, or past `…Distance
    Time` 60 s and more than `…Distance` 4096 away, or dead → given up, and
    back to packages.
  - Templates: a creature or person made from a template (most spawned
    creatures: template flags 0x3FF through a leveled list) takes its
    SPECIAL, health, skills and creature damage from it with "use stats"
    (0x02) and its AI values with "use AI data" (0x10) (`Facts::
    actor_value`, `combat::creature_damage`). The Broc Flower Cave rats
    (`VCrGiantRatTier3RatCave`: combat skill 100, health 125, bite 50) had
    0 health and bit for 0 before.
  - `nvinspect <Data> nif <skeleton>` prints a skeleton's `BSBound`.
- Detection (`world::detection`, read from the code `00642ed0`): the
  game's value (sound, light in sight and the 190° cone, perception
  against sneak, each × ((maxD − d)/maxD)², maxD `fSneakMaxDistance` 2500,
  × 2 outdoors; −35 base): above 0 seen (`GetDetected`), above −20
  noticed. Checked on the findings' worked example (500 units → 38). The viewer
  supplies the line of sight (a ray through the cell's collision, 60 units
  above the feet) and how other people move; for scripts the line is taken
  as clear. The light stays 50 (a guess until light levels are worked
  out), shots' noise 0 (when the game clears it isn't traced). Faction
  reactions combine with ally beating friend beating enemy beating neutral
  (`008b8740`).

### V.A.T.S.

Read from the code (`%USERPROFILE%\nv-re\findings\vats.md`; `world::vats`,
`viewer/src/vats.rs`; `nvinspect <Data> vats DocMitchell WeapNV9mmPistol 500`).
- Action points (actor value 12): most `fAVDActionPointsBase` 65 +
  `fAVDActionPointsMult` 3 × Agility (`006439d0`: 80 at Agility 5); back at
  `fActionPointsRestoreRate` 0.06 × seconds × perk entry 39 × the most, onto
  what's been taken, only with V.A.T.S. off (`0088b660`, `0088b740`; also
  while waiting). Taken when each queued attack has played (`009c7240`), not
  when queued (the menu's own count drops at once). None needed to enter.
- Cost (`0066dce0`): `DNAM` f32 at 68 when `DNAM` second flags (u32 at 56)
  have 0x08 (every vanilla weapon: 9mm 17, varmint rifle 35, assault carbine
  20, 10mm SMG 22, minigun 30, machete 20), else the table by attack kind
  (`0066dba0`: animation 0 unarmed 22, 1 one-hand melee 15, 2 two-hand 30,
  3–4 pistol 20, 5–7 rifle 25, 8 handle 20, 9 launcher 40, 10 grenade 35, 11
  mine 10, 13 thrown, 12 none; no weapon → unarmed 22), then perk entry 40
  asked about the weapon (`Fists` 000001F4 without one): Fast Shot × 0.8
  for Guns/Energy Weapons. A gun attack the projected clip can't cover adds
  `fActionPointsReload` 10. Specials (`007eb920`): the weapon's `VATS`
  record (f32 4 skill, 8 damage ×, 12 AP) when the skill is met; Uppercut /
  Stomp (Unarmed > 50, 20 AP) and Cross (> 75, 20 AP).
- Shots (`007f5050`): automatic (`DNAM` flags 0x02): round(`DNAM` 60 × `DNAM`
  64 × 0.43 s), long bursts (second flags 0x800) to 1.75 s: carbine 5, SMG 4,
  minigun 9 + 26; others the ammo use.
- Entering (`00942800`, `007e9200`): not with V.A.T.S. on, a menu open,
  swimming, or the fighting controls off; not with a weapon in
  `VATSBannedWeaponsList` (00174256); `UIVATSEnterFail` with nobody to
  target. Targets (`007f52c0`): alive, 2–`fVATSMaxEngageDistance` 5000 away,
  in line of sight, on screen (root or a part's node) or within the sneak
  distance (2500 inside, × 2 out), base not in `BannedVATSTargets`
  (001770BC). First: score (180 − angle)/180 + 40/d, an earlier pick inside
  `iVatsTargetAngle` 15° stops score wins, a nearer one inside it takes over.
  Parts: each `BPTD` slot 0–14, aimed at `BPNT`, + the weapon held unless
  built in (`DNAM` flags 0x20); none → the whole body.
- The menu (`menus\vats_menu.xml`, `007e9200` setup, `007ec810` every
  frame; `ui::vats`, findings `vats_camera_menu.md`): its own AP bracket
  (what's left after the cost, the cost pulsing, "Low", the projected
  clip "%i/%i", CND), HP bracket with the compass, the target's health
  bracket (system colour 2 when hostile) and name; a `body_part_percent`
  label per part ("%i%%", `_LimbName`, `_MeterPercent` the part's
  condition) placed by the part's node on screen, offset and justified by
  part type, pushed apart (31 passes of 150 × 100), shown once the view
  has settled, the selected part's group pulsing, a group's later label
  hidden (a head keeps its own) with the lowest meter (`007f0ea0`); the
  queue's lines "<target>: <part>" (+ ": <special>", a "Reload" line after
  an attack with a reload); Accept/Return/special buttons. The HUD shows
  only its messages (mask 0x100; playing, 0x108: `00771700`). The exe's
  words for settings the data lacks (`sAccept` "Accept" …) are
  `ui::vats::EXE_TEXT`.
- Time: the menu is MenuMode (`0086e650` skips the clock, actors, physics):
  the world stops; the menu, its animations and its camera run on real
  time. Modes: 1 while V is held after opening, 2 once let go, 3 while
  the target is scanned, V again → 1 (`007ec810`).
- Keys (PC): A / D previous / next target in bearing order (`007f0500`,
  `00772840`; `UIVATSSelectTarget`, `UIVATSEnterFail` with one), opening
  on its torso; W / S next / previous part whose label shows (`007f3070`,
  `UIVATSSelectTargetPart`; ranged weapons, or while scanning); left button
  queues (`UIVATSMove`); B or right button takes back (`UIMenuCancel`; none
  left: closes); R / F specials; E plays (`007ebd50`: `UIVATSExit`,
  `UIVATSReady`; nothing queued: off).
- The view (`009445b0` modes 1–3, `0095de30`; `MenuView`): the player
  turned toward the target's aim (its head node within 300 units of the
  player's feet, else the parts' nodes' middle) and, in modes 2–3, the
  field of view eased to atan(r × `fDlgFocus` 3.2 / d) × lerp(`…MultNear`
  50, `…MultFar` 34, min(d / `…FarDist` 2750, 1)), at least 1.5°, the
  first-person view's at most `fDefaultFOV`; eases from a 100-entry table
  (0.02 i² / 99 mirrored) over `fVATSTargetSelectCamPanTime` (mode 1),
  `fVATSCamZoomInTime` (mode 2, new target), `fVATSLimbSelectCamPanTime`
  (a new aim height); in mode 1 the field of view goes back at 30 ÷
  `fIronSightsFOVTimeChange` 0.25 = 120° a second; settled once mode 2's
  turn and zoom arrive, when a selected part showing within 0.2 of the
  screen's top or bottom moves the aim to its height.
- Playback (`009c7240`; `world::vats_camera::Playback`): each attack picks
  its camera path (`CPTH` tree `0058aef0`, pick `0058bb50`/`0058b510`,
  conditions asked about the player and the target): every vanilla player
  attack gets `SimpleFrontHit01` (the `QuickSimpleKill…COPY0000` paths:
  the side-view ones need `GetVATSRightTargetVisible`, not carried out).
  A shot waits for its action (shoot 0, fly 1, hit 2) by the attack's
  phase (`009c95e0`: the projectile fired → 1; both there and a hit shot,
  near the target, or gone → 2; melee when the blow lands); then plays at
  least its shortest time, at most its longest (≤ 0.001: `fVATSCamera-
  MaxTime` 20), in real seconds: the world at its multiplier (0.0001 while
  the camera dollies, 0.3 for kind 0x12), the player and the target at
  theirs (`009c8cc0`/`009c8d60`: else `fVATSPlayerTimeUpdateMult` 6,
  `fVATSTargetTimeUpdateMult` 1; a shot set but not started: 1). Its camera
  model (`0058c5d0`) placed at its location node (the target's part, its
  root, the attacker's root; flag 0x01 follows it, else where it began),
  its keyed offset turned by the heading (the attacker's, or the target's
  for a bone-following shot at the target), looking at the target node
  (or the share of the way between the bounds, `009c9fe0`) with the
  world's up, × the model's X·Y·Z angles; its frustum (FOV keys v → ±v,
  ±0.75 v) widened by (w/h)/(4/3); a dolly over `fVATSCameraDollyTime`
  past `fVATSCameraDollyMin` (path zoom 0, or zoom shots with 2); within
  `fVATSCameraCutAwayDistance` of what it looks at: first person; flag 0x08
  first person; its `MNAM` image space modifier on. An attack is done
  when its shots are out, the shot's shortest time is over and its
  projectile gone (melee: its animation over); its AP taken then. Past
  `fVATSCameraMaxTime` of playback the queue is dropped. Queue empty:
  `fVATSPlaybackDelay` 0.17 of the world's time with the shot still on,
  then off (`009c6c30`: modifiers removed, first person, the kill reward).
  Between shots the game keeps its third-person camera (`0058ce50` doesn't
  switch back).
- **The shot's camera kept out of walls** (`0094a0c0`, called by
  `0058cf60` each frame; `world::vats_camera::ChaseDistance`,
  `physics::Collider::spherecast`): a sphere `fCameraCasterSize` (10) wide
  is cast from the shot's location node to where its model puts the
  camera; touching something, the camera's distance from the node becomes
  |touched point − node| − 5. The distance is the chase camera's own
  global: taken at once on a shot's first frame (`[011e07c3]`, set by
  `0058c5d0`), on the frames after pulled in at once but let out at most
  `fChase3rdPersonZUnitsPerSecond` (800) a second of the world's time and
  kept between `fVanityModeWheelMin` 30 (unless the cast took more than 2
  off) and `fChaseCameraMax` 120 (so `SimpleFrontHit01`'s 175-unit offset
  is whole on the first frame and 120 after); in playback never under
  `fZoom3rdPersonSnapDist` 50. The caster's own layer isn't traced (the
  viewer casts against everything that stops walking; the hit walk in
  `00620bc0` skips the target's body).
- **The smart camera checks** (`GetVATS<Right|Left|Back|Front>AreaFree`,
  `…TargetVisible`: `005a5460`…`005a5840` → `008bd830` / `008bdbd0`;
  `world::vats::{Side, SmartCamera, …}`): asked about the attacker with
  the target as the parameter (a null reference parameter takes the
  condition's other actor, `00681600`). From the actor's position raised
  `fVATSSmartCameraCheckHeight` (64) along the side (heading + 0, π/2,
  3π/2, π; `(sin, cos, 0)`): AreaFree = the first thing met (not the
  actor) on a 10000-unit line, as units, else the largest float;
  TargetVisible = the farthest of 5 samples `…StepDistance` (64) apart
  whose line to the target's node isn't blocked (a hit within 85% of the
  way; an actor only within 256 of the sample), 0 if the first is. The
  viewer casts them as each attack starts (collision; people as cylinders,
  the player's own body too) into `state.vats.smart_camera`. In vanilla
  the unconditional `QuickSimpleKillRight02` comes before the side-view
  paths, so a player's attack never reaches them; `PlayerDeath`'s
  `NormalDeathFront/Back` and the Stranger's `DocuCam` paths do use them.
- **The reload** (`007ec810` l.563–567, 866–877; `007f28d0`; `007efa10`;
  `world::vats::Plan::queue`): an attack whose shots are at least the
  rounds the projected clip holds (clip size > 1) costs
  `fActionPointsReload` 10 more; the projection takes its shots off and,
  run out, refills from the reserve; the "Reload" line follows that
  attack (not for a clip of one shot's worth or with nothing left). No
  reload step plays: guns hold the attack control (`00a24280(6)`), so the
  weapon reloads itself when empty, as outside V.A.T.S.
- `GetVATSValue` 8, 11, 12, 13, 14 read the precomputed hit record's
  flags 0x4, 0x10, 0x40, 0x20, 0x80 (critical, fatal, explode, dismember,
  cripple; `0058cba0`), 9/10 its critical effect, 16 a helper's attack;
  the menu is `MenuMode` 1056 (`vats::VATS_MENU`).
- Chance (`00646f70`): melee/fists 100 within `fVATSMeleeMaxDistance` 300
  of the edges; a proximity projectile (`PROJ` f32 28) 0; thrown (1 −
  max(d − 128, 0)/(2R − 128) + 0.4 × skill/100 × arm) × 100; guns: r =
  tan((min spread + wobble × 15°) × `fVATSSpreadMult` 1.1) × d, T =
  bound²/3 (× 2r/bound when 2r < bound), chance = visible × min(T/πr², 100) ×
  the part's `BPND` byte 8 (head 30, torso 60, arms 45, legs 50; the weapon:
  its `DNAM` byte 40); × 0.5 against Chameleon (or the attacker Invisible);
  at most 95, ÷ 100. Bound = the base's `OBND` diagonal (Doc 143.9; < 1 →
  128); d = centre distance − both radii. The wobble (`00646910`, aiming on,
  sneaking off while scanning): 0.1 + crippled arms + Strength/skill short
  of `DNAM` 168/200 × `fWeapStrengthReqPenalty` 0.025, × (1 − skill/200), at
  least 0.01, perk entry 34. Then (`007f1290`) perk entry 8 (weapon and
  target tabs), the larger kept: Commando / Gunslinger (`GetWeaponAnimType`
  5–6 / 4) and Sniper (`GetVATSValue 5` 25) × 1.25. Stored once. Shown
  (`007f0ea0`) max(0, min(trunc(100 × chance + 5 n), 95)), n the attacks in
  a row on the part with Concentrated Fire; parts sharing an actor value
  take a later one's higher chance; after 2 s parts never on screen take the
  average (`007f3e00`). Default part: the head within 300 units, else the
  part type last aimed at, else the torso. Doc with the 9mm, Guns 50, 500
  units: torso 95, head 52, arms 78 (the findings' worked example).
- Queue (`007ec810`): refused under 1% shown (`sVATSMessageZeroChance`),
  over the AP left (`UIVATSInsufficientAP`), no rounds (`sVATSMessageNoAmmo`).
  Hit rolled now: chance (+ 0.05 per earlier attack in a row on the part,
  Concentrated Fire) ≥ U[0,1) (`004dff20`). Melee: a random part among slots
  0–6, trunc(max(r,1) × n/100); Uppercut/Stomp the head; a melee kind
  without those parts: head, else torso, else the first.
- Playing (`009c7240`, `009c8e00`): a hit becomes a miss out of line of
  sight; guns aim at the part's node, misses `fAutoAimMissRatio` 1–1.3 ×
  bound radius / d off in a random direction, at most
  `fAutoAimMaxDegreesMiss` 3° each way (`00965620`); a hit meeting the
  target lands on the part chosen (`009b6620`); melee puts the player reach
  × `fVATSMeleeWarpDistanceMult` 0.32 (unarmed 0.27) away (`009c9280`) and
  reaches × 2. Attacks on the dead are dropped without their AP. Crits +
  `fVATSCriticalChanceBonus` 5 for queued hits (`009b7060`); the player takes
  × `fVATSPlayerDamageMult` 0.75 while V.A.T.S. is on (`009b5a30`); specials
  × their `VATS` damage, Uppercut 1.15, Cross 1.1, Stomp 2, automatic melee
  2 (`009b5170`). The pace and the camera: the Playback bullet above.
  After the last attack `fVATSPlaybackDelay` 0.17 s of the world's time,
  then off; a kill gives perk entry 35's AP back (`009c8950`).
- `GetVATSMode` = the mode (0 off, 1 menu, 2 scanned, 3 scanning, 4
  playing); `GetVATSValue` 0–7, 15, 17 about the attack composed or playing;
  `GetWeaponAnimType` = the table at `0118a838` (1 hand to hand / none, 2
  one-hand melee, 3 two-hand, 4 pistols incl. energy, 5 rifles incl. energy,
  6 automatics, 7 handles, 8 launchers, 9 grenades and thrown, 10 mines, 11
  lunchbox; `005a09b0`); `IsWeaponSkillType` on a weapon = its skill, with
  none true for Unarmed (`005a0ab0`).
- Collision with triangles of no area: skipped by the walking capsule (it
  went NaN near the Prospector Saloon's pool table).

### First person

- The game's first-person view (`world::actor::first_person_look`,
  `viewer/src/viewmodel.rs`): `Characters\_1stPerson\Skeleton.NIF` (74
  nodes: `Bip01 Looking` at eye height 118 is the pivot, `Camera1st` 3.7
  behind it, a `Weapon` node in the right hand; `nvinspect nif` lists a
  skeleton's nodes), the clothes worn (else the race's upper body), the
  race's hands as their `…1st.nif` versions (`LeftHand1st.NIF`, with the
  race's `HandMale.dds`), and the weapon's own world model (`MODL`; its
  `WNAM` first-person record names the same file) held at `Weapon` in that
  node's axes (`ActorPart::bone`).
- Poses: `<kind>aim.kf` by animation type (h2h, 1hm, 2hm, 1hp, 2hr, 2ha,
  2hh, 2hl, 1gt, 1lm, 1md); the attack by `DNAM` 41 in steps of six (26
  Left, 32 Right, 38 Attack3 … 68 Attack8, 255 default Right; melee and
  fists ship `_a`/`_b`); the reload by `DNAM` 15 as a letter (A–S, then
  W X Y Z). Checked: every name for the 9mm, varmint rifle, service
  rifle, .357, hunting shotgun exists; the 9mm's reload lasts 1.67 s, its
  record's reload time. Attacks and reloads play over the hold pose
  (`nif::posed_layers`: a bone takes the last layer that moves it).
- Placement: looking up and down turns everything under `Bip01 Looking`
  about it; the root faces the heading with `Camera1st` at the eye. Lit
  by the place's lights. `nvinspect <Data> first-person [WEAPON]`;
  `nv-viewer … --weapon ID` (screenshots then show it, drawn; `--weapon
  none`: fists drawn).
- **The first-person pass** (the actors recording, frame `166830323`):
  after the world, `Clear(D3DCLEAR_ZBUFFER)` (call 166828991), then the
  pass with its own projection (call 166828994: z row 1.000758,
  −5.003791 → near exactly 5 units, far about 6,600; cotangents 1.440737
  / 2.56131 = `fDefault1stPersonFOV` 55 as a 4:3 width at 16:9) and the
  view matrix at the origin (the skeleton placed in camera space); the
  image space passes come after it. The viewer: a second camera
  (`viewmodel::spawn_camera`, `FirstPersonCamera`), a child of the main
  one, order 1, clears depth, draws only `FIRST_PERSON_LAYER`; the main
  camera's output is skipped (`CameraOutputMode::Skip`) and its grade
  deferred (`grade::GradeDeferred`): the grade is copied to the
  first-person camera each frame (`viewmodel::copy_grade`) and runs
  there, last. So the hands never go into walls (report 005).
- **Weapon out** (`combat::ReadyKey`, `PlayerAttack::out`, `GameState::
  weapon_out`, `IsWeaponOut`): the process flag at +0x135 (`00915d40`
  reads it, `00915d60` sets it; `IsWeaponOut`'s handler `005a0550` →
  `008a16d0` → vtable +0x454). It starts 0; equipping a weapon into an
  empty hand (`0088db20`) and unequipping one (`0088d7d0`) put it away
  for the player. The Ready Item control (7; keyboard default scan 0x13
  = R, mouse Attack = button 0: the binds table `00a24b70`, `+0x1b94`;
  key states `00a24180`: 0 down, 1 just pressed, 2 just released) in the
  player's frame code `009466d0`: while down, a timer (`011e07e0`) runs;
  nothing while a weapon action is queued (`008a7570() != -1`:
  drawing or putting away still plays); holstered → `008a6840(1)`
  draws at once; out and not reloadable (the weapon's anim type 0, or no
  ammunition equipped: `00525980`) → put away at once; out and
  reloadable → put away once the timer passes a settings object at
  `011cdfcc` (its name and value aren't traced: not done here); each
  press acts once (`011e0bec`). Releasing with a gun out before that
  (`00948310` l.527) reloads. Attack just pressed with the weapon away
  (`00948310` l.282) draws it instead of attacking. In the five recorded
  holstered frames the pass after the depth clear drew nothing; with the
  machete out it drew the arms, hands and weapon: the viewer shows the
  first-person view only while the weapon is out or its drawing /
  putting away plays (`<kind>equip.kf` / `<kind>unequip.kf`,
  `world::actor::first_person_ready`; shipped for every kind).
- Guesses: energy pistols and rifles sharing 1hp/2hr; the weapon at
  `Weapon` in its axes (looks right on the 9mm, varmint rifle); the
  outfit's whole model drawn (the game may use only the arms); hands
  lit like the place (the game may light the first-person pass
  separately); the view hidden while the putting-away animation plays
  isn't recorded (shown here until it ends); the first-person pass's far
  plane (only culling here). Not yet: the reload's `…start` parts, the
  weapon's own moving parts (bolt, cylinder), the Pip-Boy glove, holding
  R to put a gun away, V.A.T.S. drawing the weapon by itself.

### Player controls and screen effects

- `DisablePlayerControls` / `EnablePlayerControls` flags, in order:
  movement, Pip-Boy, fighting, view switch, looking, roll-over text,
  sneaking (`VCG01`'s own comments: "1 1 1 1 0 0 1 ; disable everything
  but looking"). A flag of 1 turns that control off / on; the others stay.
  Left out: the editor's defaults (disable: the first four; enable: all),
  not traced. The viewer stops walking and jumping, mouse look (the
  free-flying camera F stays free), using and talking (roll-over), and
  I/J (Pip-Boy). `GetPlayerControlsDisabled`.
- Image space modifiers (`world::modifier`, `viewer/src/effects.rs`):
  `DNAM` flags (0x01 animatable), duration, then key counts per track:
  21 × (multiply, add) for the image space's values in `IMGS` order, then
  tint, blur, double vision, radial blur (…, centre 0.5 0.5 as floats),
  depth of field, fade colour, motion blur; every count matches its
  subrecord's keys. Tracks: `[n]IAD` multiply, `[n+0x40]IAD` add, keys
  (time, value) with time 0..1 over the duration; `TNAM` tint and `NAM3`
  fade colour (time, r, g, b, a); `BNAM` blur, `VNAM` double vision.
  `VCG01FadeInFromBlackISFX` (7 s): bright scale × 2 → 3 → 1, bright clamp
  × 0.5 → 0.3 → 1, blur 5 → 0, fade colour white 1 → 0 by 0.71. The fade is
  the final pass's `lerp(c, Fade.rgb, Fade.w)` (`grade.wgsl`, now drawn).
- People in a conversation turn to face the player (not when walking or
  sitting), at the walking turn speed.

### Menus and the HUD

Read from the exe (`%USERPROFILE%\nv-re\findings\ui_tiles.md`) and checked
against a recorded frame of the game's HUD (`FalloutNVDocMitchell.trace`,
call 1179629, replayed to `%USERPROFILE%\nv-re\ui_frames\
doc0001179629.png`). `crates/ui`, `viewer/src/hud.rs`.

- Units: the screen tile is 960 high and (w / h) × 960 wide (1706.67 at
  16:9); 1 unit = h / 960 pixels (1.125 at 1080p). The recorded
  `ModelViewProj` scales by 2/1706.67 and 2/960. Safe zone `[Interface]
  iSafeZoneXWide`/`YWide` (15 here).
- Files (`00a1c9b0` → `00a1ce70`): tabs and line breaks dropped, runs of
  spaces and spaces beside `<`/`>` removed, `<!-- -->` cut out whole,
  before includes are pasted (textually, `Data\Menus\Prefabs\X`,
  `00a02d40`). A commented-out `<include>` is therefore never pasted
  (`hotkeys.xml` has one).
- Names (`009ff970`), tokens (`00a01e20`, `00a0a410`), traits and their
  operator chains (`00a09410`: no reset, `copy` first; `onlyif`/
  `onlyifnot` zero the value and go on; `not` of the operand; groups),
  selectors (`me()`, `parent()`, `sibling(N)`, `child(N)`, `screen()`,
  `globals()`), templates (`00a1ddb0`), tile defaults (menus start hidden,
  locus 1, system colour 1: `00a1ef30`).
- Position = own x, y + every ancestor with `locus`; depth likewise plus
  the menu; drawn back to front by depth.
- Colour (`00a06925`): alpha / 255; brightness b ≥ 0 → the system colour
  of the nearest tile with `systemcolor` or id 111 × b / 255 (`uHUDColor`
  255,182,66; colour 2 = 255,67,42); b = −1 → red/green/blue.
- Pictures (`00a04640`): `tile` = −1: height-fitted; negative `zoom`:
  stretched; else texels × zoom / 100; `.tai` atlases map the UVs into
  their rectangle (zoom > 0: 1:1 texels); `tile` ≠ 0 repeats across
  (`00a054fc`).
- Fonts (`00a15320`): `.fnt` 14632 bytes, `.tex` RGBA; loader patches
  (space, 0xA0, glyph 0, 127); layout (`00a12fb0`): advance trunc(width +
  right kerning), breaks at spaces, extra line gap fonts 2/6 = 4, 3 = 6,
  7 = 2; justify right −line width, centre −width/2. "HP" lands at the
  recorded glyph positions (−30..−5, −15..10).
- Drawing (the recording): after the image space pass, straight onto
  the back buffer, source alpha / inverse source alpha, no depth;
  `TILE1000.pso` texture × `TintColor`; the compass `TILE1001` (uv ×
  `TexScroll.zw` + `.xy`, alpha × `hud_compass_alphamap.dds` red at the
  unscrolled uv); bilinear, no mipmaps, clamped (wrap for tiled pictures
  across, both ways for the compass). Direct3D 9 pixel centres: the
  viewer moves its quads half a pixel right and down.
- The HUD (`0076bfe0` setup, `00770430` update every frame): HitPoints at
  2 sx + 10, AP at W − 2 sx − width + 30, both at H − 2 sy − height;
  meters (`007748b0`) in steps of `_ImageWidth`, HP at least one step
  while alive, AP growing left; ammo `"%i/%i"` = clip / (count − clip)
  (`007721c0`); condition meter blinking below 25%; crosshair centred
  (−3); compass (`00779070`): scroll = heading/360 − 0.15, bearings ×
  1.45 (`00778b80`, with the game's mirroring above π), ±70 limit, icons
  fading over the last 20%, map markers within 20000, people within
  `fSneakMaxDistance` (× 2 outdoors), red when hostile; messages
  (`00775380`): default Vault Boy icons, 2 s (`010162c0`), 0.35 s fades.
- XP meter (`0077c4e0`): placed at W − 2 sx − 390, H − 2 sy − 280; awards
  summed when it's free; fades in over 2 s with the pointer at the old
  experience, moves to the new over 2 s (to the end over 1 s past a
  level; levels step on out of combat), fades out over 2 s; then "LEVEL
  UP" (0.5 s fades, 1.5 s) when a level-up waits. Level starts
  (`00648b50`): base + Σ (base + n × bump).
- Menu trait animations (`00a07c60`, `00a080d0`): straight line, value =
  from + (to − from) × min(t / length, 1).
- Checked: the viewer's "HP", "AP", meters and crosshair against the
  recorded frame at the same camera: mean difference under 1.5/255; the
  compass strip identical (the game's frame also has Doc's tick and a
  quest marker, absent in the viewer's run).

### The game's menus

Read from the exe (`%USERPROFILE%\nv-re\findings\menus.md`); each drawn
from the game's own menu file (`ui::menus`, `viewer/src/game_menus/`).

- Interface (`0070c4a0`): the pointer picks the topmost `target` tile
  (a tile without an `id` gives its nearest ancestor's); a changed tile
  gets `mouseover` 0/1, its `mouseoversound` and the menu's
  HandleUnmouseover/HandleMouseover (list boxes choose first); a click is
  release over the tile pressed: `clicksound`, `clicked`, HandleClick(id).
  Drags (`draggable`, `dragx`/`dragy`…), the wheel (−120 a notch away →
  `wheelmoved`), keys (`007154b0`: characters, then `_PCButton_<key>` on the
  menu naming a tile to click, a non-target one gives `UIMenuCancel`; else
  the arrows/Enter/Page keys as special codes 1–4, −2, 15/16 to the menu,
  then its list boxes step). Menu sounds (`00717280`): 1 `UIMenuOK`, 2/0x14
  `UIMenuCancel`, 3 `UIMenuPrevNext`, 4 `UIMenuFocus`, 0x24 `UIMenuMode`…
  Template lookup (`00a1ddb0`): an unknown name gives the file's last
  template. A picture that can't be found draws `sMissingImage`. `io()`
  (`00a08b20` case 5008) is the menu tile once it has a `class` (every menu
  file gives one). Clip windows (`00a037e0`, `00a04640`, `00a07a40`): a
  tile clips with its own `clips`, or its picture/text parent's, to the
  nearest `clipwindow` ancestor, cut on whole pixels (x, y at least 0,
  right and bottom at most the screen).
- The HUD under menus (`0070c4a0` picks a mode by the menu that opened from
  the game, `00771700` its pieces, `ui::hud::parts_for_menu`): none for the
  message box, level-up, character making, barter and most others; the
  quest reminders, messages and XP meter in dialogue (the XP meter moved up
  330, `01073488`); the XP meter over a container; quest reminders and XP
  meter for terminals and hacking; the messages for the Pip-Boy's pages,
  lockpicking and V.A.T.S. The viewer's roll-over line goes with them.
- Message box (1001, `007a8e60`, `007a92e0`): `ShowMessage` boxes with the
  file's own sizing (`_MinMenuWidth`/`_MaxMenuWidth`, measured title, text and
  buttons), centred text on one line, `&` key letters, Escape the last
  button (not for `ShowMessage`'s kind), `GetButtonPressed` once; its
  background `fMenuBackgroundOpacity` × 255, or `fPopUpBackgroundOpacity`
  over another menu.
- Dialogue (1009, `00761a20`, `00762950`, `007638b0`): the speaker's name
  and line, then the topics list (`topic_<i>`, `_line_alpha` 128 for topics
  already chosen or flagged), skill-check tags; click or Enter chooses, a
  click on the line skips it.
- Containers (1008, `0075b0d0`…): two lists filtered by the arrows/title
  (weapons, apparel, aid, misc, ammo; English titles upper-cased), "name
  (count)", the weight line "Wg a/b", names cut to fit with "...",
  `iInventoryAskQuantityAt` (5) asks how many, Take All, the items'
  sounds and the container's open/close sounds, `OnClose` on closing,
  stealing by `world::crime`. The item card (`00707e30`, the Pip-Boy's)
  isn't shown in these menus yet.
- How many (1016): the meter (most, start, a page = max(most/20, 1) rounded
  to tens above 10, a quarter), arrows ±1, the triggers ± a quarter, Page
  keys ± a page, OK gives the whole amount.
- Barter (1053, `0072d250`…, `world::barter`): the player's things (no
  caps, keys or quest items) and the vendor's goods (theirs and their
  merchant container's, those their `AIDT` services cover); picking offers
  a thing (it moves across, marked), the total and the caps flow follow;
  prices (`0072ed00`): value at condition (`fItemConditionValue…`) × the
  barter multiplier (`00649050`: `fBarterBuy/SellBase`, `…Mult`, Barter
  skill, perk entry 17), rounded halves up (`004bd510`), "--" for nothing,
  tenths below 1; Accept pays floor(total) (`0072fd10`) and counts "Barter
  Amount Traded" (stat 39); Exit with offers asks "Cancel transaction?".
  `nvinspect <Data> barter TrudyREF`.
- Level up (1027, `00784c80`…): page 0 the 13 skills (Big Guns hidden by
  its actor value flag 0x2000, `0066f260`), each with the player's
  permanent value, arrows giving and taking the level's points (a tag skill
  × `iSkillPointsTagSkillMult`), the points going into the skill at once;
  page 1 (perk levels) the perks (`world::perks::level_up_choices`; only the
  pickable ones with `[Interface] bHideUnavailablePerks`), the pickable ones
  bright and first, then by level and name, each with its picture and
  "Req: Level 6, Endurance 5…" text (`005ebac0`); Done gives the perk's next
  rank. `MUSSuccess` on showing.
- Traits (1084, `007e6990`): the level-up perk page's rules for perks
  flagged as traits (`world::perks::trait_choices`: level byte above 0, no
  hidden-flag test), "CHOOSE UP TO %d TRAITS" (`iTraitMenuMaxNumTraits`), "%d
  TRAITS LEFT", up to the most picked, Done gives them.
- Tag skills (1048 skills mode, `SetTagSkills n [p]` → `00753420(1, n, 0,
  p == 1)`): the 13 skills by name, a tag skill shown less
  `fAVDTagSkillBonus` and (with p) picked to start with, "SKILLS:  %d/%d
  Selected", "Tag %d Skill(s)", Done clears every tag slot and fills them
  with the picked skills in the list's order
  (`world::chargen::set_tag_skills`). The attributes mode (S.P.E.C.I.A.L.
  points, `SetSPECIALPoints`) isn't used by the opening and isn't here.
- The name (1051, `ShowNameMenu`/`GetPlayerName` → `007ab690` →
  `007e6320`): "Enter character name.", the name so far, a blinking cursor
  (character 127 / "|" by turns every 500 ms), the first key replacing the
  name, Backspace/Delete/arrows/Home/End, the text kept no wider than the
  text tile's `wrapwidth` − 5; OK only for a valid name (`007ab820`:
  something besides spaces, no "\" or "~", every character drawn by fonts 1
  and 7).
- Inventory weight (`004d0900`, `0048ebc0`, `world::items::weight`):
  ammunition weighs something only in hardcore mode.

- Sleep/wait (1012, `007bfc30`, `007c0220`, `007c0580`; `ui::menus::
  sleepwait`, `world::living::sleep`): T (the Rest control, 15), a bed, or
  `ShowSleepWaitMenu` open it after the refusals; the file's scrollbar gives
  1–24 hours (Page Up/Down −2/+2); Wait: `UIMenuOK`, an autosave when
  `[GamePlay] bSaveOnRest`/`bSaveOnWait` (exe 1) say so, for a sleep the
  screen fades to black over `fFadeToBlackFadeSeconds` (0.3; the fader
  manager `FaderManager.cpp`, alpha ± seconds ÷ time, back from 0.9999 on
  closing), Wait and the bar unusable, then an hour a real second; the
  hours out (or T again) click Cancel (`UIMenuCancel`). The time line
  "%s, %s, %d:%02d %s": `sDay…[round(GameDaysPassed) mod 7]`, the date
  "%02d.%02d.%02d", a 12-hour clock (12 before 1:00, −12 from 13:00),
  `sAMTime`/`sPMTime` at 12:00. `MenuMode 1012` blocks run while it's open.
  `--open-menu wait|sleep[:N]`.
- Books: New Vegas has no book menu in play (`BookMenu` 1026 is only
  reached by the debug console's menu commands, `0071b210` → `00720aa0`;
  its file names pictures the game doesn't ship); using a `BOOK` reads a
  skill book (`00515040`): "%s increased by %d" (`sSkillIncreasedNum`),
  nothing at 200 or more, `sCanNotReadBook` in combat outside the menus.
- The Vit-o-matic's `LoveTesterMenu` (1074) is a rendered 3D menu (the
  cabinet and Activate models, their camera `007945f0`, buttons as named
  nodes picked from the mouse, bulb texture swaps; findings `menus.md`
  §13): not built; the viewer's text panel stays.

### The Pip-Boy

Read from the exe (decompiled output in `%USERPROFILE%\nv-re\decomp\pipboy`).
`ui::pipboy`, `viewer/src/pipboy.rs`.

- Shown with `[Pipboy] bUsePipboyMode` 1: the menus are drawn into a picture
  (an orthographic camera over 1280 × 960 menu units, `007fba00`) that the
  `ISIFSCANBLEND` effect (`00bb89b0`) turns into the texture of
  `pipboyscreen:0` in `PipBoy3000\PipBoyArm.NIF` (the `PipBoy` apparel's
  model; its texture coordinates show 0..0.753 × 0..0.7625 of the picture;
  the cursor maps back as u × 1280, v × 960, `007f8720`). The shader:
  y = t.y + scroll (wrapping), glow = mean of 3 × 3 taps ± the blur radius
  in texels × intensity × `uPipboyColor`, + the picture; + the distort map
  (`PipboyDistortEffectMap.dds`) × 0.35 × colour while a band runs; ×
  `PipboyScanlines.dds` (`sScanlineTexture`) at t × `fScanlineFrequencyPipboy`.
  Timing (`007fb150`…`007fb530`): blur radius/intensity from `[Pipboy]`
  (3.5 / 0.25 here); × (1 + `fDefaultBurstIntensity` × (1 − t/200 ms)) on
  opening and each page change; a roll < `fVertHoldChance` rolls the picture
  (5.5 screens/s for 500 ms, then a shudder), < + `fShudderChance` shakes it
  (sin(t × 0.05) × 0.05 fading over 250 ms); pulse sin(t × `fPulseRate`)
  × 2 × `fPulseBrightenIntensity` / `fPulseRadiusIntensity`; a band every
  random(0, `iDistortMaxInterval`) ms for (`fDistortVerticalScale` + 1) ×
  `fDistortDuration`.
- The model sits on the first-person skeleton's `Bip01 L ForeTwist` (its top
  node's `Prn`), in that bone's axes (the forearm runs along x 4..17); the
  first-person `Locomotion\Male\Pipboy.kf` (female `PipboyFemale.kf`; 0.73 s,
  text keys start, Blend: 8, Hit 0.33, end) raises the left arm; drawn with
  `fPipboy1stPersonFOV` 47. `ScreenLit:8` lies just behind the screen and
  `glare:0` just in front. Lamps: `007f9070` finds `StatsGlow`, `ItemsGlow`,
  `DataGlow`; `007fa010` hides all three and lights the shown menu's.
  `PipboyLightEffect` is shown only with the light on (`007fa310`).
- Tab = control 14 "Pip-Boy" (`sUActnMenumode`; default DIK 0x0F, set in
  `00a24b70`). Held past `fPlayerPipBoyLightTimer` 0.8 s it casts or
  dispels the light (`009673d0`: `PipBoyLight` 0x147 → `PipLight` MGEF,
  archetype Light, `DATA` +24 → `PipboyLight640`: radius 768, colour 194,
  245, 209), with `UIPipBoyLightOn`/`Off`; let go sooner, it opens.
- Menu keys (`007154b0`, `0070c4a0`, `0070f6e0`): ←/→/↑/↓ give codes 4/3/1/2
  (first moving the focus along `xleft/xright/xup/xdown` tiles); Shift + ←/→
  give 0x0D/0x0E (the pad's triggers: previous/next menu, round the three:
  `007db680`, `00782190`, `00799790`); Enter = A (the chosen tile), Shift +
  Enter = X (0x0B), Alt + Enter = Y (0x0C); Page Up/Down = LB/RB
  (0x0F/0x10); other keys: the menu's `_PCButton_<LETTER>` trait names a
  tile, pressed (`HandleClick` with its `id`) when visible and `target`. The
  pad (`00710060`, `00717a40`): XInput bits 1/2/4/8 d-pad, 0x1000 A … 0x100
  LB, 0x200 RB, triggers bytes 2/3.
- STATS (`StatsMenu`, vtable `0106ffd4`; setup `007da2c0`, values `007dd090`,
  keys `007db680`, clicks `007db380`): the page is the menu's `user0`;
  headline `user8` level, `user5`/`user6` "%d/%d" HP and AP, `user7` XP
  "%d/%d" (else `sStatsXPMax`); name line "%s - %s %hd"; limb pictures
  `Interface\Stats\<part>[_broken].dds` and meters `_Value` = AV/100; the
  Vault Boy's face `face_%02d.dds` = trunc((most − health)/most × 5),
  0..4 (`007dfe40`); rads `user1` = rounded (1 between 0 and 1,
  `007ddb00`); aid buttons from default objects (`DOBJ` `DATA`, `0058db10`):
  slot 0 Stimpak (button id 10), 21 Doctor's Bag (59), 3 RadAway (11), 2
  Rad-X (31); text "(%d) %s" with the count, else the name (`007df230`);
  Status page: A presses Stimpak under CND, RadAway under RAD; Y Doctor's Bag
  / Rad-X; X Rad-X under RAD. Perks "%s" / "%s (%d)" from rank 2, sorted by
  name (`007e04e0`); 43 misc statistics; effects (`007ddf00`) a row per
  source with "%s %+d" by the `AVIF`'s `ANAM` (STR +2), joined ", ".
- ITEMS (`InventoryMenu`, vtable `010739b4`; `0077fc10`, row `00782850`,
  headline `00782a90`, keys `00782190`): tabs by the list's filter
  (`00782620`): weapons `WEAP`; apparel `ARMO`/`CLOT` except `BMDT` general
  flag 0x40 (not playable: the Pip-Boy and its glove are never listed);
  aid `ALCH`, `INGR`, `BOOK`; ammunition `AMMO` minus `RegeneratingAmmo`;
  misc the rest; keys (`KEYM`) never. Sorted by name (`007824e0`). Headline
  caps, HP "%d/%d", DT "%.1f", weight "%d/%d". Tabs wrap. The item card
  (`00707e30`, shared with containers and barter): mask bits 0x1 DR, 0x2 DPS,
  0x4 WG, 0x8 VAL, 0x10 CND, 0x20 ammo line, 0x40 effects, 0x80–0x200 mods,
  0x400 STR, 0x800 DAM, 0x1000 DT; weapons 0xc3e (ammo line "%s (%i/%i)":
  short name else name, in the clip / the rest, "--" without; DAM stays at
  the file's alpha 0; STR made alpha 255, "%d"); apparel 0x3c + DR/DT
  ("--" in DR with neither), its ammo-line card holds the weight class
  `sArmorWeightHeavy` (flag 0x80), `Medium` (0x08), else `Light`
  (`00f48ef0`…); others 0xc (+ 0x40 with an effects text). Value "--" ≤ 0,
  "%.1f" < 1, else "%.0f" (× count); weight "%.2f"; DAM "%d" (rounded,
  `004bd510` halves up) or "%.1fx%d" for several projectiles; the effects
  card at y = card height/2 − 20, doubled when CND or the ammo line shows.
  `user5` the condition meter (≤ 1), `user6` 1 for weapons.
- DATA (`MapMenu`, vtable `01074d44`; `00796b90`, keys `00799790`): tabs
  (ids from 0x20) wrap; the date line "%s, %d:%02d" (`0079aba0`) with the
  date "%02d.%02d.%02d" = month + 1, day, year % 100 (`00867970`) and the
  hour truncated (24-hour), minutes (fraction × 60) truncated; the place
  (`00578870`) the cell's name. World map: the worldspace's `ICON` and
  `MNAM` (usable size, NW cell, SE cell) — a place's share between (NW.x ×
  4096, NW.y × 4096 + 4096) and ((SE.x + 1) × 4096, SE.y × 4096)
  (`0079c380`); marker sizes linear in the magnification's share between
  `fWorldMapMinZoom` and `MaxZoom` (`0079c5a0`); markers by `TNAM` picture
  (`011a0404`), undiscovered ones `icon_map_undiscovered.dds`. On the world
  map A presses the marker under the cursor; the mouse wheel / Page Up/Down
  zoom; the right stick pans.
- Exe default texts the plugins lack (`ui::game::EXE_TEXT_SETTINGS`): STATS,
  ITEMS, DATA, LVL, CND, RAD, EFF, Wg, WG, VAL, DAM, DPS, DR, DT, STR …

### Sound

- `world::sound` (`SOUN`: `FNAM` file under `sound\` or a folder to pick
  from, `SNDD` distances ×5 / ×100, flags 0x10 loop; doors `SNAM` open,
  `ANAM` close; items `YNAM` pick-up; acoustic spaces `ASPC` four `SNAM` loops dawn, day, dusk,
  night, then crowd; cells `XCAS`). `cellview::sound::read_wav` (the
  game's WAVs are 16-bit PCM: a Vegas door 44.1 kHz stereo),
  `Game::sound_file` (folders; a missing `.wav` plays the `.ogg` of the
  same name, which the game ships for Goodsprings' interior loop
  `amb_gsinteriorloop`). Viewer: `viewer/src/sounds.rs` (`PcmSound`, a
  Bevy audio source).
- Music: see Music below.
- Guesses: which loop by hour (6–8 dawn, 8–18 day, 18–20 dusk, else
  night, from the climate's sunrise/sunset); no distance, direction or
  volume settings; sounds a folder picks from by the state's dice.

### Music

From the game's code (`%USERPROFILE%\nv-re\findings\music.md`):
`world::music` (records, `MusicDirector`, `Decks`, `CombatMusic`),
`cellview::music` (track lengths, a decoding thread per track, a headless
mix), `viewer/src/music.rs` (`MusicPlugin`), `nvinspect <Data> music
<PLACE> [X Y] [at X Y] [hour H] [seconds S] [combat S [E]] [play MUSIC]
[wav OUT]`.

- A cell's music type (`XCMO`) is read into a field nothing reads. Music
  comes from audio markers (`REFR` of `AudioMarker` `00000023` with
  `MMRK`, `CNAM` → `ALOC`, `XRDS` radius, 0 → 5000), else the region
  sound data (`REGN` type 7: `RDSI` incidental set, `RDSB` battle sets) of
  the exterior square's region with incidental music (`011dd380`) or the
  acoustic space's region (`ASPC` `RDAT`), and from `PlayMusic`.
- `MUSC`: `FNAM` a file or a folder under `Data\Music`, `ANAM` dB
  (default −6): above 0 loops, plays at −|ANAM|. A folder's tracks: never
  the same twice running (`00591a40`). `1NoMusic` (no `FNAM`) plays
  nothing.
- `MSET` `NAM1` kind: 0 battle, 1 location, 2 dungeon, 3 incidental;
  tracks `NAM2`–`NAM7` (location: day layers 1–3, night 1–3; dungeon:
  battle, explore, suspense; battle: the loop), dB `NAM8 NAM9 NAM0 ANAM
  BNAM CNAM`, boundaries `JNAM`–`ONAM` (% of the radius **squared**),
  `PNAM` enabled layers, `DNAM` least time on a layer/track (battle: the
  hold after the intro, in **ms**; incidental: least gap by day), `ENAM`
  (battle: its fades; incidental: least gap by night), `FNAM` cross-fade
  and lead before a track's end (battle: recovery; incidental: most gap by
  day), `GNAM` (most gap by night), `HNAM`/`INAM` intro/outro (incidental:
  day/night) sounds.
- `ALOC` `NAM1`: bits 0–3 the list (0 neutral `HNAM`, 1 enemy `YNAM`, 2
  ally `ZNAM`, 3 friend `XNAM`, 4 location `LNAM`), bits 4–5 0 = tracks
  start again at their end, 0x40 day/night from the climate (all 89 in
  `FalloutNV.esm`); `NAM4` delay (0 everywhere); `GNAM` battle sets; lists
  kept in reverse file order; `RNAM` a faction. Picking: the first entry,
  each later one replaces it 1 time in 4, the pick moves to the end
  (region battle lists: 1 in 3, to the front).
- The marker: the first in the list (an interior's references, last
  placed first) holding the player (2D, d² < r²), else the nearest;
  chosen again after > 1 s. Its controller plays while it says so;
  otherwise region music, and location/dungeon decks fade out over 1 s.
- The faction rule: with a member of `RNAM` aware of the player, the list
  is chosen by `RNAM`'s relation toward `PlayerFaction`, reading the
  `XNAM` **modifier** (2nd number) as 0 neutral / 1 enemy / 2 ally / 3
  friend (a slip in the game, done as it does); a hostile member makes it
  enemy.
- Day for controllers: from the middle of sunrise to the middle of sunset
  (`NVDefaultClimate` 7:00–19:00, `DefaultClimate` 7:00–18:00, which
  interiors have in a new game); for incidentals sunrise begin to sunset
  end.
- Location sets: layer = the innermost enabled one whose boundary is above
  100 × d²/r²; a new layer (or day flag) starts once the least time is up,
  **in step with the old track** (`sync`), cross-fading over `FNAM`; the
  track is asked for again `FNAM` s before its end (from the start); in a
  fight it stops answering and the controller turns to its battle list.
  Doc Mitchell's: 3high within 1443 units, 2mid 2040, 1low 2500;
  Goodsprings 6261 / 10844 / 14000.
- Dungeon sets: battle track in a fight (intro, 1 s fade), suspense while
  someone aware is hostile, else explore (outro after a fight); their
  track-end check asks a *location* deck's length, so dungeon tracks just
  loop (as the game's code does). Battle sets: intro, the loop fading in
  over `ENAM`, both decks held `DNAM` ms, asked for again 1 s before its
  end; at the fight's end the outro, a fade over `ENAM`, `FNAM` s of
  recovery. Incidental sets: the day/night sound every random[min, max) s
  when the last deck is empty.
- Two decks (`008300c0`): each request goes into the other deck, fading in
  from 0 linearly while the old one fades out over the same time; kinds
  (priorities) 2 location, 3 dungeon, 4 battle, 6 `PlayMusic`, 7 radio, 8
  menus; a request below the most recent deck's kind is refused unless
  forced. Gain = master × music volume × 10^(dB/20) (INI `[Audio]
  fDefaultMasterVolume` 1, `fDefaultMusicVolume` 0.6). Outside fades the
  volume steps 0.05 a tick; a fading-out deck ends below 0.01. A synced
  deck keeps to (now − sync) mod length (500 ms tolerance).
- `PlayMusic`: kind 6, 1 s fade, not forced; nothing else runs until it
  ends. Leaving Doc Mitchell's house plays `musSCRGoodspringsStinger`
  (−7.74 dB, once, 29.8 s), then Goodsprings' music from its start.
- Combat music (`00992d90`, every `fCombatMusicUpdateTime` 1 s): on when a
  group targeting the player saw them within `fCombatDetectionLostTime`
  (15 s) and P / E < `fCombatMusicPlayerTargetedThreatRatio` (100), or
  fighting groups within `fCombatMusicNearCombatInnerRadius` (500)
  outweigh the player; off `fCombatMusicStopTime` (3 s) after nothing
  wants it.
- Music isn't ducked in dialogue. On the player's death all decks fade
  over 1 s and `MUSDeath` plays.

### AI and walking

- `world::ai`: packages (`PACK`: `PKDT` flags u32 + type u8; `PLDT`
  kind i32, form, radius; `PSDT` month, weekday, date, hour, duration;
  `CTDA`), `current_package` (a script's, else the first of the base's
  `PKID` whose schedule and conditions pass, asked about the person),
  `destination`, `NavMesh` (`NVVX`, `NVTR` 16-byte triangles; neighbour
  `i` across the edge from vertex `i` to `i + 1`, checked on Doc
  Mitchell's house) with A* and a funnel. Viewer: `viewer/src/ai.rs`.
  `nvinspect <Data> ai <REF> [QUEST STAGE]`.
- Navmeshes join through external connections: triangle flag bits 0x1,
  0x2, 0x4 mark edges 0, 1, 2 as leading to another navmesh, the edge's
  link then counting into `NVEX` (10 bytes: 4 unknown, navmesh form ID,
  triangle). Checked on Goodsprings' square (21 + 24 + 8 flagged edges,
  53 connections, links all below 53). Outdoors the viewer joins the
  3 × 3 squares around the player. A destination in another interior or
  worldspace isn't walked to, and a path needs both ends within 128
  units of the navmesh.
- Package types (New Vegas): 0 find, 1 follow, 2 escort, 3 eat, 4 sleep,
  5 wander, 6 travel, 7 accompany, 8 use item at, 9 ambush, 10 flee, 12
  sandbox, 13 patrol, 14 guard, 15 dialogue, 16 use weapon. Doc's intro
  packages are travels gated by `GetStage VCG01`; at stage 55 he takes
  `VCG01DocMitchellTravelToPlayerAtTester` (560 units through the house).
- Walking animation: `characters\_male\locomotion\male\mtforward.kf`
  (`female\` for women; creatures `locomotion\mtforward.kf`). Its
  accumulation root `Bip01` travels 102.3 units forward (+y) in 1.2 s:
  85.3 units a second (women 85.1); running (`mtfastforward.kf`) 283.1
  in 0.8 s, 353.9 a second. `nvinspect <Data> nif <kf>` prints these.
  People move at the game's walking speed (`world::animation::
  walk_speed`: `fMoveBaseSpeed` 77 × SpeedMult ÷ 100 × legs) × their
  scale, the walk played at the rate that makes its root travel that; the
  root's translation is taken out of the pose so the body walks on the
  spot.
- **Turning, read from the code** (`findings\ai_rules.md` §1;
  `world::movement`): people turn `fCharacterDefaultTurningSpeed` 90°/s
  (`008d3290`), creatures their `CREA` `TNAM` (0 → `fCreatureDefault
  TurningSpeed` 45, `008d47f0`; the tutorial gecko 200). In place (`009e7610`,
  `009e7d70`) × `[Pathfinding] fAITurnSpeedScale` 1.5 (combat 2.5;
  creatures 1.25): 135°/s, 225°/s; ≤ 1° isn't started, the side is fixed at
  the start, the turn state lasts ≥ `fActorTurnAnimMinTime` 0.3 s; it plays
  the turn group (see "People and creatures": animation groups). Walking
  (`009e4800`): ≤ 3 × speed × dt × k toward the point 0.1 path-units ahead,
  k = angle to the look-ahead point ÷ 90° in 0.1..1 (speed > 45 on a path);
  forward × 0.05 / 0.1 / 0.2 past 2.7 / 2.0 / 1.6 rad left; the body moves
  toward the steering point whatever the facing (`009e3560`). A new path
  starts with a turn in place. Look targets (`008a3100`): the body turns in
  place when the target is > `iActorTurnDegree` × 0.8 = 80° off (8° while
  turning), standing, not fighting, seated or in dialogue/use packages.
  At the end of a travel (travel and dialogue packages' travel step,
  `008e5e90` → `008bb5c0`) they turn to an `XMarkerHeading`'s heading, or
  near the editor location to their placed heading; out of sight it's set
  (`0090ad40`). `world::ai::arrival_heading`.
- **Paths** (`009e0a00`, `009e3d50`, `009e8000`): the steering point moves
  on in 0.1 steps; arrival = 2D within the request's radius, height within
  180; radii by what's there (`00678670`: furniture 10, markers 20, a
  sleeper 90, else half the bounds' diagonal + 20; 0 → 100). A straight line
  on the navmesh is tried first (`bUseStraightLineCheckFirst`; the door
  portals it crosses are opened as on any path). Packages are
  looked at again every 20 s, on each game-hour change and when forced
  (`EvaluatePackage`, `ResetAI`, `Add/RemoveScriptPackage`, a conversation's
  end; `008da670`). Others in the way (`009e5ae0`): blocked 0.5 s → wait 5 s
  (the one walked up to, or someone walking) or a way round them (avoid
  nodes radius × 1.5, cost 2, left out 5 s / 2.5 s after the player); the
  player counts. Waiting (for others, or for a door to swing) is standing,
  not walking on the spot.
- **Dialogue packages** (`008e8600`, list 10 TRAVEL → DIALOGUE_ACTIVATE →
  WAIT → DIALOGUE): the travel once (it's not gone back to when walking up
  takes them off the place); "Say To" with the player: within `PTDT`
  distance (≤ 0 → 120, 3D) the topic's line (else `HELLO`), no menu; else
  walk up until `IsWithinDistance` (`008b2f90`: distance, 0 →
  `iAIDistanceRadiusMinLocation` 100, + own radius ≥ 32, 2D at body height
  ± 30) and talk; with `PLD2` wait for the player there first. `PKDD` FOV is
  the dialogue camera's zoom, not a gate. `StartConversation` (`005c8740` →
  `008b2170`): the speaker comes within 90 (200 seated, `world::movement::
  conversation_reach`), then the menu (the player) or a conversation (anyone
  else); a seated speaker doesn't walk. In the dialogue menu the speaker
  stops walking and turns in place to face the player (the conversation's
  rule, `008ec460`; that the player's menu runs it is inferred), the turn
  animation playing while everything else stands still.
- **People out of sight** (`009334b0`, `0096b810`/`0096b470`/`0096b050`,
  `009ea8a0`): moved every 0.3 game h in a buffered cell (`uInterior/Exterior
  Cell Buffer`, middle-low), else every game hour (low; persistent people,
  `iLowProcessingMilliseconds` 2 ms a frame), by (game hours since the last)
  × 3600 ÷ TimeScale seconds (none stored: 900 ÷ TimeScale) at
  `fMoveBaseSpeed` × SpeedMult (× `fMoveRunMult` in a fight), point to point
  along the navmesh path, through load doors. Replaces "catch up every 2 s,
  at once".
- **Greetings, chatter, conversations** (`008eeec0`, `00904800`,
  `0061b440`; `world::social`): someone who detects the player (> 0) within
  `fAIMinGreetingDistance` 150 says `HELLO` (no menu) when their greeting
  timer is out, then `fAIGreetingTimer` 20 s (also after the dialogue menu
  closes); else `IdleChatter` every `fIdleChatterCommentTimer…` 15–60 s
  (not in dialogue packages). People talk to each other within 200 when
  their timer (30–60 s) is out, 25 % a try, `fAItalktosameNPCtimer` 120 s per
  pair; not in sleep/use-item/ambush/guard/dialogue/use-weapon packages, a
  sandbox only without its "no conversation" flag, followers only with
  their target. The lines are worked out at the start: `HELLO`, a random
  follow-up each step, the line's next-speaker byte (0 other, 1 same, 2 coin
  flip), `GOODBYE` when none, ≤ 100. The starter walks within 90 (200
  seated); the other faces them; lines ≥ 2 s apart.
- **Sandboxes** (adds to the sandbox bullet): no fixed re-choice timer; eating
  (`008e3140`) takes food lying in the area (walked to, activate reach) or
  uses carried food, sits in a free chair if one is found, eats one seated
  and asks for an eating idle every 1–6 s; talking is a real conversation;
  wandering (`008ed420`): spots 32..0.75 × radius, walked to within 50,
  pause (`fAIMaxWanderTime` − energy) ÷ 2 − 10 s; the "under 60 stand" and
  "radius + 250 back" tests are only for wander packages (type 5), which now
  wander the same way at their place (`world::ai::wander_step`).

### Scripts

- Crate `script`: lexer, parser, interpreter (`interp::Host`), the
  640-function table with parameter types, and `compiled` (reads `SCDA`).
  `world::scripting`: `GameState`, `Facts` (functions that read; used by
  dialogue conditions too), `Runner` (the `Host`; functions that change
  things; `set_stage`, `update`, `run_event`, `menu_mode`), `Event`s for
  the viewer, `scripted_references` (a cell's scripted objects, trigger
  boxes). `world::quest` reads `QUST`. Viewer: `viewer/src/scripts.rs`.
- **Checked against the compiled form of every script**
  (`nvinspect <Data> scripts`): all 10,684 sources parse; 35,485 of
  35,485 expressions in the compiled order; 10,420 of 10,455 scripts the
  same statement by statement with the same argument counts (the 35 others
  have stray `endif`/`elseif`, which the outline drops).
- Compiled form: statements are u16 code + u16 length + data (a call on a
  reference is preceded by `1C 00` + `SCRO` index). Codes: 0x10 Begin,
  0x11 End, 0x15 Set, 0x16 If, 0x17 Else, 0x18 ElseIf, 0x19 EndIf, 0x1D
  ScriptName, 0x1E Return, 0x1000 + n function n. If/ElseIf/Else start
  with how many statements their branch holds. Expressions are in
  evaluation order, each token after a space: numbers and operators as
  text, `s`/`f` + u16 local, `r` + u16 reference (then a variable or
  call), `Z` + u16 a reference as a value, `G` + u16 global, `X` + u16
  code + u16 length + parameters; unary minus is `~`.
- Precedence (from the compiled conditions): unary, `* / %`, `+ -`,
  comparisons, then **`||` tighter than `&&`**, left to right. Extra
  arguments become values of their own (`GetStage Q 110 != 1` compares
  110 with 1). Unconfirmed: whether `==` binds looser than `<` (never
  mixed in the game's scripts).
- `if`/`endif` pair up by counting and `elseif`/`else` belong to the
  innermost open `if`, whatever the indentation (the Lucky 38 terminal's
  and the campfire's compiled `else` spans show it). Lines of only
  punctuation and anything after `else` on its line are ignored.
- Actor value numbers (from compiled `GetActorValue` arguments):
  Aggression 0, Strength 5 … Luck 11, Health 16, DamageResist 18,
  SpeedMult 21, … BrainCondition 31, Explosives 35, Lockpick 36, Medicine
  37, Repair 39, Science 40, Sneak 42, RadiationRads 54, Variable01 62 …
  Variable10 71. The rest follow the `AVIF` records in form-ID order
  (`script::ACTOR_VALUES`).
- `fQuestScriptDelayTime=5.0000` in `[Main]` of the user's `Fallout.ini`
  (found by its name in the exe): quest scripts run every 5 s unless the
  quest's `DATA` delay is set (`VCG01`'s is 0.01).
- `QUST`: `DATA` (flags: 0x01 start game enabled, 0x08 repeated stages;
  priority; delay f32), `INDX` stage, log entries `QSDT` (0x01 completes,
  0x02 fails) + `CTDA` + `CNAM` + result script + `NAM0`; objectives
  `QOBJ` + `NNAM` + `QSTA`. 281 quests run in a new game.
- Trigger boxes: `XPRM` on the reference (three sizes, colour, a float,
  shape 1 box / 2 sphere / 3 portal box). The sizes are **half** sizes:
  every trigger in Doc Mitchell's house then reaches exactly down to the
  floor at 7360 (the vigor tester's: 7460 ± 100).
- `VCG00` (the opening) sets `VCG01` stage 0; nothing in the scripts or
  the exe's data starts `VCG00`, so how a new game starts it is unknown.
  Its stage 0 moves the player (`player.moveto
  VCG01PlayerStartMarkerREF`), plays the movie, and runs on to stage 100
  and `VCG01` 0 by itself: the viewer's `--new-game` sets `VCG00` 0.
- Scripts write functions by short name (`MoveTo`); the runtime matches
  the table's full names (`MoveToMarker`), checked by a test against the
  table (`world::scripting` tests).
- **More functions, read from their handlers** (`world::script_functions`;
  `%USERPROFILE%\nv-re\findings\functions.md`):
  - `GetGameSetting` (`005be900`): integer and float settings' values; text
    settings and unknown names 0; values the data lacks are the exe's
    (`iFriendHitCombatAllowed` 3, `iLockLevelMax…` 0/25/50/75/100/255).
  - `GetFactionRank` (`008b8290`) gives **1 for a member, −1 otherwise**,
    never the rank (base factions with rank ≥ 0, then the reference's
    changes).
  - `GetInCell` / `GetInCellParam` (`0059ef60`) compare editor IDs over the
    asked cell's length (`strnicmp`): a prefix match.
  - Objectives (`005ec5d0`): state bit 0 shown, bit 1 done.
    `SetObjectiveDisplayed 1` shows (and starts the quest), `0` clears;
    `SetObjectiveCompleted 1` keeps whether it's shown, `0` clears both;
    `CompleteAllObjectives` (`0060c950`) completes all, hidden ones stay
    hidden; an objective the quest lacks is an error.
  - `IsChild` (`008d40e0`): the race's `DATA` flag 0x04 (u32 at 32; the
    four child races), or a race body model (parts 0–2 for the sex) with
    "child" in its path; creatures never.
  - `SetRestrained` / `GetRestrained`: life state 5 (`008ace50`).
  - `IgnoreCrime` (actor+0x144): a victim who ignores crime counts no theft
    or assault (but still fights back); such witnesses don't count
    (`008bfa40`, `008c0460`, `008c09e0`). `SetIgnoreFriendlyHits`: form
    flag 0x100000; a friend or ally with it ignores the player's hits
    entirely (`008987f0`). `GetFriendHit`: the friendly-hit count.
  - `SetOwnership` / `IsOwner` (ownership extra 0x21; default owner the
    player's base), `SetCellOwnership`, `SetCellPublicFlag` (cell flag
    0x20): the crime rules use them.
  - Doors: `GetOpenState`, `SetOpenState`, `Lock` on an open door: see
    "Doors that swing" under Walking.
  - `EnableFastTravel enable [wait=1] [keep=0]` (`005d13e0`): player+0x66d
    bit 0 (fast travel), +0x66e (waiting), bit 1 (keep). Any move of the
    player (doors, `MoveTo`, fast travel; `0093bea0`) allows fast travel
    again unless kept off.
  - `GetWeaponAnimType` (`0118a838`): h2h 1, 1hm 2, 2hm 3, pistols 4, rifles
    5, automatics 6, handles 7, launchers 8, grenades/thrown 9, mines 10,
    lunchbox mines 11; no weapon 1. `IsWeaponInList` checks the fists
    (`000001F4`) when nothing is in hand. Both read the process's weapon:
    the player's equipped one (others: the one the engine has them draw).
    `IsWeaponSkillType` is asked about a weapon (else counts as Unarmed
    45).
  - `GetCurrentAIPackage` (`005a0b60`): the package's `PKDT` type (kept as
    is, `00670fc0`) mapped to Oblivion's numbering: 0–4 as they are, wander
    13, travel 14, accompany 15, use item at 16, ambush 17, flee 18, cast
    magic 19, sandbox 36, patrol 37, guard 35, dialogue 34, use weapon 33.
  - `RemoveAllTypedItems` / `RemoveAllItems` (`004ce380`): by form type
    number (ARMO 24 … ALCH 47, the exe's types), minus a holdout list; the
    player's quest items (flag 0x400, `SetQuestObject`) stay.
  - `ResetQuest` (`0060d720`): stages, objectives and variables reset,
    completed/failed cleared, running only if it starts with the game.
  - `EnterTrigger` (`005d8cf0`): the trigger's script runs once with
    `OnTriggerEnter` and `OnTrigger` flagged (0x20000000, 0x10000000) for
    that actor (below 5 nested runs).
  - `SendAssaultAlarm` (`005da2a0`): the assault crime and the alarmed
    person turning on the player: the caller, the named person, or for
    `Player <faction>` the faction's nearest loaded member; `Player` alone
    nothing.
  - `PlayerInRegion` = `IsPlayerInRegion` (the player's region list,
    player+0x764). `IsInList`, `AddFormToFormList`, `IsInInterior`,
    `IsImageSpaceActive`, `IsKiller`, `IsPlayerTagSkill` (the class's tag
    skills), `GetHeadingAngle` (−180…180, clockwise), `GetPos`/`SetPos`,
    `GetAngle`/`SetAngle` (degrees), `GetStartingPos`/`Angle` (people: z
    only), `GetScale`, `GetIsCreature`, `GetIsCreatureType` (`CREA` `DATA`
    byte 0), `GetIsClass`, `GetActorFactionPlayerEnemy`, `Exists`,
    `RemoveNote`, `RemoveFromAllFactions`, `ResetHealth` (health by what's
    lacking, body parts by 999), `ResetInventory`, `AddItemHealthPercent`,
    `AddAchievement`, `UnlockChallenge` (flag 0x01), `SetActorFullName` (the
    message's `FULL`: message+0x18 is its `TESFullName`), `IsPS3` 0,
    `IsPC1stPerson` 1 (player+0x64A clear; this engine has only the
    first-person view).
  - Shown by the viewer (events): `Say` (a line to no one in particular),
    `PlayGroup`, `PlayIdle` (by editor ID), `SwapTextureOnRef`
    (`Textures\<name>.dds` on a node); kept: `Look` (until `StopLook`).
    `ShowWarning` shows nothing in the shipped game (`005d8bf0` formats and
    drops the text); `PurgeCellBuffers` only frees memory.
  - **Second round** (`world::more_functions`;
    `%USERPROFILE%\nv-re\findings\functions2.md`):
  - `PlaceAtMe base [count] [distance] [direction]` (`005c4b30`): new
    references (form IDs 0xFF000001 on; saved) turned as the caller, in its
    cell; the first on the caller's spot, the next eight on a ring of 100
    units (a random start between 0 and 45 degrees, 45 apart) at the
    caller's height, then round again; ring spots outside the loaded grid
    (outdoors: `uGridsToLoad` 5 around the player's square) are the caller's
    spot. Leveled actor lists: `count` picks at the player's level, all on
    one spot (the caller's, or `distance` in front/behind/left/right of its
    model; markers have none). The value is the last one made. The viewer
    draws made people and models once a second. `PlaceLeveledActorAtMe
    actor` (`005d9810`): one new actor of that base on the caller.
  - `SetGhost` / `GetIsGhost` (`008acf10`, `008ace90`): a ghost isn't hit
    (`00899cb0`, `008987f0` return at once: no damage, no reaction), shots
    and explosions pass it by (`00816f10`, `00817b90`, `00818ce0`; the
    viewer's shots too), and noticing starts no fight with or by it
    (`008ff350`, `combat_ai::starts_combat`).
  - `IsEssential` (`0087f3d0`): a teammate outside hardcore, the base's
    `ACBS` 0x02 (`SetEssential`), or the reference's own flag
    (`SetActorRefEssential`, actor +0x140 bit 31; `IsActorRefEssential`).
  - `SetCombatStyle` (extra 0x69): fights use it at once
    (`more_functions::combat_style`).
  - `SetTalkingActivatorActor` (TACT base +0x90) /
    `IsActorTalkingThroughActivator` (a talking activator saying a line
    whose speaker is the actor); `SetBroadcastState` / `GetBroadcastState`
    (the station's base flag 0x40000000, which the record's header flag
    starts).
  - `GetCurrentAIProcedure` (`005a1210`): the exe's procedures by name
    (`011a3cc0`) through its switch (travel 0, wander 7, sleep 8, eat 10,
    combat 13, dialogue 4 …); a sandbox by its state and activity (45, 21
    getting up, 0 on the way, 48 sitting, 8, 10, 7, 46 at an idle marker, 4
    talking). Reported by the viewer each frame, with `IsMoving` (1 fwd, 2
    back, 3 left, 4 right), `IsRunning` (0x200), `IsSneaking` (0x400 and not
    0x800), `IsTurning`, `IsLastIdlePlayed` (process +0x10c),
    `IsIdlePlaying`, `IsSwimming`.
  - Challenges (`005f5950`, `005f6110`): `CHAL` `DATA` (kind, threshold,
    flags 0x01 start disabled 0x02 recurring, interval, three values),
    `SNAM`/`XNAM`; count +0x6c, flags +0x70 (unlocked, completed, recurred,
    no longer recurs). `IncrementScriptedChallenge`: a notice "Name
    count\threshold" + description at multiples of the interval (100 when
    0); at the threshold its script's `ScriptEffectStart` runs on the player,
    "Challenges Completed" (stat 27) + 1, `UILevelUp`, completed or (if
    recurring) back by the threshold. Every misc stat change counts for
    kind-11 challenges. `RemoveRecurringFromChallenge`;
    `GetChallengeCompleted`: scripts completed-or-recurred (1), conditions
    the completed flag itself (0 or 2).
  - Destruction (`004781e0`, `00475b20`, `00477430`, `00477d10`): `DEST`
    health/count, `DSTD` health %, number, damage stage, flags (0x01 cap,
    0x02 disable, 0x04 destroy), self damage, explosion, debris; `DMDL`.
    `DamageObject n`: health (extra 0x56) down, stages between the old and
    new percent (rounded up) reached in order (cap stops; disable; destroy;
    no health left destroys); `GetDestructionStage` −1 untouched, else the
    stages passed; `ClearDestruction`.
  - `StopCombatAlarmOnActor` (`00972840`): everyone fighting the actor
    stops, the actor stops; for the player the factions around forgive
    crimes. `IsCombatTarget` (`008bc700`). `GetFactionRankDifference`
    (`0047d680` on both bases). `GetArmorRating` (actor value 18).
  - `GetShouldAttack` (`0059ed30`): 100 when the caller would attack the
    other on sight (`008b06d0`), else 0 (the game's scripts all test `== 0`;
    its early 0 within one combat group, `00992640`, needs combat groups).
    `GetIsAlignment` (`005a4dd0`): Karma (actor value 23) in `0047e040`'s
    band (0 good, 1 neutral, 2 evil, 3 very good, 4 very evil).
    `SetItemValue` (`005d3e30` → `0048e960`): the reference's base's value
    for every one of them, saved; barter, repair costs, pickpocketing and
    the Pip-Boy's VAL read it. `GetContainer` (`005ce5c0`): the holder of
    the item whose script runs, else 0.
  - `MenuMode n` (`0059c380`): 0 any menu, 1 a Pip-Boy menu (1002, 1003,
    1023, 1035, 1061), else that menu; open while its `MenuMode` blocks run.
  - `Autosave`, `ForceSave`, `SystemSave` (`bAllowScripted…:SaveGame`, exe
    1): the viewer writes its own format to `nv-rs-autosave.txt` /
    `-forcesave` / `-systemsave`. `SetGlobalTimeMultiplier` (`00aa4db0`):
    the viewer's virtual time speed.
  - `ScriptEffectElapsedSeconds` (`005c43a0`): the update's seconds (scripts
    count timers down by it). `SetSecuritronExpression` (extra 0x8F, kept),
    `Add`/`SetSPECIALPoints`, `AddTagSkills` (`00753420`, kept),
    `SetActorAlpha` (clamped 0–1, kept, shown as a message), `SetAlert` /
    `GetIsAlerted`, `SetCriticalStage` / `IsInCriticalStage`, `IsLimbGone`
    (extra 0x5f: nothing dismembers yet), `SetForceSneak`, `SetActorsAI`,
    `AlwaysShowActorSubtitles`, `SetInChargen` / `GetInCharGen`, `WakeUpPC`,
    `ClearOwnership`, `GetParentRef`, `GetIsObjectType` (the exe's form type
    numbers), `GetXPForNextLevel`, `GetIgnoreCrime`,
    `GetIgnoreFriendlyHits`, `IsActor`, `Sqrt`, `IsWin32` 1, `IsXBox` 0,
    `IsPlayerGrabbedRef` 0, `GetSandman` 0, `SetRumble` (XInput only),
    `SetNoAvoidance`, `SetSceneIsComplex`, `SetAllVisible`,
    `SetAllReachable`, `TrapUpdate`, `PreloadMagicEffect`.
- Saves (`world::save`): this project's own text format, one fact a line
  (`stage 00104C1C 55`, `var 00104C0F italked i 1`, `player <cell>
  <world|-> x y z heading`), form IDs as the load order numbers them (a
  save belongs to its load order). Viewer: F5 / F9, `nv-rs-quicksave.txt`
  in the folder it runs in. Not the game's `.fos`.
- Things simply true in a new game (and in this engine, with no combat
  yet), so those functions are answered: nobody is in combat or dead;
  no reputation, casino winnings or magic effects; not hardcore.

## Still guesses

These are in the viewer (`game_lit.wgsl`, `grade.wgsl`, `main.rs`) and the
CPU renderer (`preview::raster`, `preview::cell::lighting_for`). The
shaders can't settle them: they're values the engine works out on the CPU.

- Placed models (`preview::cell`, `viewer/src/main.rs`): sequences run at
  frequency 1 (not read); several sequences moving one node are laid over
  one another in file order (the game blends them by weight and
  priority); `PlayGroup` replaces whatever the object was playing and
  plays once from the start (its mixing and the flag argument aren't
  traced); TBC keys are drawn as straight lines (none found in angle
  curves); colour keys straight between keys. Billboards follow
  Gamebryo's documented `NiBillboardNode` modes (0/2 face the camera's
  plane, 3/4 face its position with up kept); modes 1/5/9 (turn about the
  up axis) stay as modelled; not traced in the exe or a recording. The 19
  shapes textured only through an `NiTexturingProperty` (rubble, a door)
  are left out with the other shader-less ones, unchecked. Where the
  outdoor near plane (35.4) comes from isn't traced. The game draws
  alpha-tested surfaces with NVIDIA's alpha to coverage (`ATOC` in the
  recordings); the viewer uses a plain test. Not drawn: texture-transform and visibility controllers,
  specular/refraction colour controllers, `BSOrderedNode` draw order. The
  CPU renderer has the depth rules but not the decal pull or sorting.
- Particles (`world::particles`, `viewer/src/particles.rs`): left out and
  noted in the place's notes: systems whose nodes a playing sequence moves
  along a path (`NiPathInterpolator`: every dust whirlwind; the mover isn't
  read) and `BSStripParticleSystem` strips (10, grenades' trails),
  multi-target emitters (`BSPSysMultiTargetEmitterCtlr`, 58 in 37 models)
  and `BSParentVelocityModifier` (2) aren't run; skinned mesh emitters;
  the wind vector (0); bound updates/culling; the threaded update. Guesses:
  the controllers' clock is the seconds since the object appeared (the
  game's is its own app time, which only shifts where looping keys fall);
  angle indices worked out in floats (Direct3D's single-precision FPU, if
  the game doesn't ask to keep it); the spawn turn's twist about the
  parent's direction (the spread is the same); impacts', projectiles' and
  actors' particle effects aren't spawned (only placed models'). Not
  compared pixel for pixel: the barracks' specks are 1–2 pixels at the
  recorded camera, and positions are random.
- Sleeping: the refusals for an alarm sounding, under water, in the air and
  irradiated (player+0x15c/+0x15d are set at actor creation and reset, so
  their meaning isn't the report's; process+0x764 not traced); the hour's
  run of every AI process as if 3600 s passed (people don't move on, their
  effects don't run) and followers told to come (`00974fc0`); the 00867a40 re-sync of `GameDaysPassed` when the hour jumps more
  than one (which globals it reads isn't certain).
- Pickpocketing: the warning after a crime (`008bf500`'s PickpocketWarning
  package, type 0x23); live grenades (only `is_live_grenade`, the item
  test, is here: placing one is an ordinary placing); the gates read but
  not traced in `005fa330` (base+0x30 vtable +0x1c, actor byte +0x104,
  `008ace90`'s process flag 0x10000000) are taken as passing; creatures
  aren't pickpocketed; the value of worn/modded items (`004bd400`);
  whether the "caught you" flag is saved by the game (kept here).
- Trespassing: which of several people noticing at once comes (here the
  first in the cell's reference order); what follows an alarm (here
  nothing more in that cell while the player stays); a warner who dies;
  "loaded" means the player's cell.
- The needs' callback runs for the player only (vtable +0x360, from the
  report).
- Outdoors (`world::land`, `world::weather`, `cellview::game`):
  - Terrain: layers whose texture can't be found aren't dropped (the
    game drops them after counting their opacity).
  - The sky by the hour, from the game's code (see "Outdoors"): what's
    still not done or not read: on a weather change the viewer blends
    every colour, the fog, the modifiers and the grass's light and wind
    (`viewer/src/weather.rs`, `world::weather::WeatherMix`), but keeps the
    first weather's cloud textures and sun (the game cross-fades the
    clouds); the weather region is looked at on entering each square (the
    game: each cell change); lightning, the glare
    shrinking as the sun is hidden (an occlusion query). The CPU renderer
    still shows 10:00. (The sky's brightness, the clouds, the sun's path and
    colour were settled by the Goodsprings recording: see "Outdoors".)
  - Water (`viewer/src/water.rs`, `water.wgsl`, `cellview::water`): not
    compared with the game yet (an apitrace recording at Lake Mead or
    Cottonwood Cove would settle the rest). The ripples' normal map is
    worked out per pixel from the noise (the game renders it into a 256²
    texture every frame: same sums and Sobel steps). The depth map is the
    scene's depth at full resolution (the game's a 512² 8-bit render),
    holding 1 where nothing is under the water (the game's pre-fill isn't
    traced). The refraction is Bevy's copy before its transmissive pass
    (no sky dome or blended surfaces in it). One mirrored camera per frame
    in the nearest reflecting water's plane; inside, everything is
    reflected (the game picks by the placed water's flags); the
    first-person model shows in full-scene reflections; the blur taken as
    the bloom's kernel at radius 5. A square's water is a flat square over
    the cell (the game cuts the land triangles). The sun's visibility its
    fade × 100 (inferred). Distant water hidden over loaded squares. Not
    done: underwater (`WATER016`), wading ripples, the point-light pass.
  - Grass (`world::grass`, `viewer/src/grass.rs`): where each blade lands
    is random in the game (one generator seeded from the clock); here the
    same generator is seeded per spot and grass, so counts and spread
    match on average, not blade for blade (brightness and size are
    exact). The multisampling pixel shader (`GRASS2000TMS`, alpha to
    coverage) is taken because `bTransparencyMultisampling` is on (not
    traced). The sun light's own dimmer taken as 1. Quarters out of view
    aren't skipped. The fade's and fog's depth is the view depth (the
    game's is less by its near plane). Not done: `BILLBOARD` grass,
    point-lit and shadowed variants, `bDoTallGrassEffect`. Not compared
    with an in-game picture yet (by the schoolhouse, `-72700,1450,<z>,315`,
    would do).
  - Trees (`viewer/src/trees.rs`): no branch was drawn in the recording, so
    the branch shader's passes beyond the first (point lights, highlights)
    aren't known and aren't drawn; leaves and branches take no point
    lights; the trees of a square are loaded once (scripts' `Enable`/
    `Disable` later aren't followed); outdoors only; the one trunk
    collision capsule (`00c81380`, `WhiteOak01`) isn't in the walking
    collision; the wind's clocks start at the viewer's start (the game's
    run since it started).
  - Distant land: the far clip plane's 352,000 is measured, not traced
    (fBlockLoadDistance × 2√2 = 353,553 fits as well); the viewer measures
    the levels from the camera's spot (the game: the player's, 3.7 units
    ahead in first person; with the free camera the viewer follows the
    camera); fades run on the viewer's clock (the game's timer); new
    quarters fade only when their parent was on screen the frame before.
    Distant water (`water.rs`) still comes from the level-4 chunks out to
    20 cells. The blend constants 9625.6 / 0.0003756 are the recorded
    shader's (with `uGridsToLoad` 5). `HighDetailRange`'s 15-unit inset is
    from the one recording.
  - Eye adaptation: what the average holds after a loading screen isn't
    known: the viewer takes the current frame's average while a place
    loads and on the frame it's entered, then eases. The frame's seconds
    are taken as 0 while the viewer's menus or dialogue menu are open (the
    game's menu mode); the game's own code for `TimingData` isn't traced.
  - Doc Mitchell's house windows from outside (`NVCraftsmanWindowExt`,
    `CraftsmanWindowExt.nif`): the game draws them with the material's alpha
    0 (`AmbientColor.w` 0, alpha blended: invisible by day); the model's
    controllers (`NiMaterialColorController`,
    `BSMaterialEmittanceMultController`, sequences `Left` / `Right`) set it.
    The viewer draws them opaque and glowing (1.20× there).

- Lockpicking (`viewer/src/lockpick.rs`): the menu's scene's ambient light
  black (its shadow scene node's own light, inferred from `00b5e0f0`), no
  fog; a light's colour = diffuse × dimmer; spheres of no size left out of
  the bound merge; the straining sound's fades linear; the HUD (and its
  messages, "You are out of lockpicks.") hidden while the menu is open, as
  for every viewer menu; the world not paused (people keep fighting, as
  for the viewer's other menus); the dice are the state's, not the game's;
  the INI controls' keyboard key read as their second byte (from the
  values, not the parser); the mouse taken raw (the cursor's acceleration,
  `007118d0`, not traced). Not done: the first-time tutorial message (help
  topic 20, `00718630`), the mouse cursor and clicking the buttons, the
  gamepad, rumble, `MenuMode` blocks for menu 1014, the key path's own
  message and sound.
- Perks: the attack animation rate leaves out
weapon mods' rate-of-fire bonus (mods aren't modelled); the equip rate is
worked out but nothing plays equip animations; `attack_wear` skips the actor
flag (`+0x100` bit 0x20) that exempts some actors, not identified; the hit's
own weapon wear (`fDamageToWeapon<kind>Mult`, the hit record's `+0x2c`)
isn't traced; items' "broken" messages and unequipping at 0 health aren't
done; the viewer's `attack_seconds` for the AI still uses 1 ÷ shots a second.
- People: the viewer plays the idle looped (GPU skinning, each actor at
  its own point in the loop; four weights per vertex); the CPU renderer
  uses the first frame. Walking is the only other animation (see AI); the first item per body slot is worn (the game picks the
  best), leveled actor lists give their first entry (the game picks at random); FaceGen skin tones only from the shipped facemods (no `FGTS` / `.egt`); faces: the `.egm`'s rows past the
  model taken as the `.tri`'s statistical targets (the counts match; the code isn't traced), normals not moved
  by face morphs, a line's head turns, eye tracking, expressions and the eyes-closed state not done (the dead
  just stop blinking), a skipped line's face easing to rest over 0.2 s, each face's blinks from its own dice
  seeded by its form ID; no hair highlight layer (`LayerMap`); the
  weapon is the best by a guessed rule, its leveled lists drawn without
  chance (combat once their inventory changes uses what's there); no
  reload or equip animations, no creature attack animations.
- Animation (`world::animation`): `BlendValues` (translations/scales averaged, rotations slerped
  in turn: Gamebryo 2's), the group's movement vector = root travel ÷
  length (read as a velocity, its writer not found), the mover still
  moves people at a constant speed rather than the root's per-frame
  travel (the files' roots move evenly within 3%), the viewer's idle
  spread (phase) at spawn, the first-person/menu blend setting at
  `011c5740` unread, the `00451530` flag (instant activation) taken as
  off.
- People: idles. The game was playing its random idles (head tilts, looks
  around; Sunny's head turns between frames); the viewer plays the
  standing `mtidle.kf`, so poses differ in every comparison.
- Skin: that the directional factor is `DNAM` float 2 (three cells agree;
  the code isn't traced); which surfaces count as skin is flag 0x400.
- Eyes: the game's eye reflection (flag 0x20000, `SLS2059`: normal halfway
  between the eye's bounding-sphere normal and the view, faded by a smooth
  step of `(S·V − 0.5)/0.7`, tinted by one point light's colour) isn't
  ported; eyes aren't fogged (`Toggles.y` 0) — not done.
- Hair: the game writes depth while blending hair (zwrite on); not done.
- Glove fingers taking the race's hand texture: matched by size only.
- Doc Mitchell's neck: no streaks in a standing pose with this work; in
  the intro pose the bandana's knot (weighted to `Bip01 Head`) stretches
  up the turned neck — whether the game does the same isn't recorded.
- Ragdolls (`physics::ragdoll`, `preview::ragdoll`): Havok isn't
  reimplemented; the bodies are stepped with extended position-based
  dynamics (10 substeps of the game's 0.016 s step). Read from the code
  (`findings\physics.md`): gravity (the world's, 686.61), and the death
  push (`008ae000`): a change of speed for every body (mass doesn't
  matter) away from a point 2048 units back along the blow, the weapon's
  kill impulse (`DNAM` f32 at 180) × 2.5 Havok/s (a tenth past its impulse
  distance at 196 for shots), `fDeathForce…` only without a weapon, × the
  body's part share (part number = the collision filter's second byte:
  head, body, spine, upper arms 1; forearms, thighs 0.75; hands, calves
  0.5; feet 0.25; checked on `skeleton.nif`). Guesses: how Havok measures
  the cone, plane, twist and hinge angles (only checked in that the
  skeleton's pose falls inside 16 of 17 limits), no bounce (restitution
  not combined with the world's), bodies not colliding with each other,
  when they come to rest (still for a second), where the blow struck (the
  chest, 90 up; the body part hit, which takes 1.5, isn't tracked). The
  neck's file pose lies 13° outside its 7° cone, so the head settles a
  little on the first step.
- Dialogue (`world::dialogue`): in conditions, functions not carried out
  give 0 (a rule of this reimplementation). OR binds before AND. A
  topic's lines are tried by quest priority, then file order; the first
  that passes (and isn't a "say once" line already said) is said. The
  main list keeping the opening line's follow-ups (so "Goodbye." stays)
  is a guess, as is listing equal priorities in file order; when nothing
  is left to say the viewer ends the conversation. No random lines,
  or the dialogue camera. A failed check's dark red
  highlight and said lines drawn at half strength aren't in the viewer.
- Levels (`world::experience`): the interface checks that also hold a
  level-up back aren't named (the viewer waits for its menus); not in
  character generation isn't checked (no XP comes then); kill XP takes
  full health with effects (the game's base health; the same unless an
  effect raises it); difficulty is Normal (×1); speech-challenge XP (the
  Fallout 3 kind) isn't given (New Vegas' checks give theirs by script).
- Scripts (`world::scripting`, `script`):
  - A branch after an `if`'s `else` never runs, and an `elseif`/`else`
    with no `if` open acts as a new `if` (an `else` as one whose
    condition failed): how the game runs these (35 shipped scripts) isn't
    traced. The compiled skip counts settle only the nesting.
  - When operands outnumber operators, the last value is the answer.
  - `SetStage`: a stage the quest lacks does nothing; a done stage does
    nothing unless repeated stages are allowed; the current stage never
    goes down; it starts the quest. Log entries' conditions are asked
    about the player.
  - A quest's script first runs as soon as it's running; quest stage
    result scripts have the quest's variables as their own; dialogue
    result scripts the speaker's.
  - The clock: `GameHour` moves at `TimeScale`; Gregorian months,
    `GameMonth` from 0; `GetDayOfWeek` Gregorian with Sunday 0.
  - `IsDLCInstalled "Tribal Pack"` = `TribalPack.esm` loaded.
  - NPC levels with "PC level mult": that many thousandths of the
    player's, at least calc min.
  - The face menu isn't drawn: `ShowRaceMenu` (number 1036 from
    `VCG01SCRIPT`'s `MenuMode 1036`) closes at once with nothing changed,
    its `MenuMode` blocks run once. The character menus and message boxes
    are drawn (see above); their `MenuMode` numbers aren't known, so no
    `MenuMode` blocks run for them, and time stops while they're open.
    Message box buttons' conditions are asked about the player, and a box
    without buttons gets "OK" (both guesses). The psych exam
    (`VCG01TestSCRIPT`) tags its picks with `SetPlayerTagSkill <skill>
    <slot 0–2>` (one combat, one "gate opener", one wild card, by its own
    comments), and the tag menu starts with them.
  - Triggers test three points of the player's or a person's body (10,
    64 and 110 above the feet); the game tests their collision shapes.
    Objects' scripts run only in the interior or outdoor square the player
    is in.
  - Furniture: E sits the player in place (scripts see it: `IsCurrentFurnitureRef`,
    `GetSitting` 3; walking 40 units away gets them up): no seat, sit
    states or animations for the player yet. For people (`world::furniture`):
    that the entry/exit animations' root travel moves the actor (inferred:
    nothing else in the procedure moves them from the marker to the seat;
    they end 3–4 units off the settings' seat), the turn toward the marker
    at 135°/s in place, no animation blending, no character-controller
    change while seated, the sandbox reserving the nearest free marker (the
    game reserves the first free one and walks to the nearest).
  - Idles and sandboxes (`world::idles`, `world::sandbox`,
    `viewer/src/sitting.rs`): `GetCurrentAIProcedure` numbers other than 4
    (dialogue) and 10 (eating) are the editor documentation's; replay delays
    counted in seconds; the bound radius taken as half the `OBND` diagonal;
    `GetRandomPercent` drawn afresh per condition; idles stopped by walking
    or a fight; upper-body idles played as whole-body; no tree asks at an
    idle marker, whose idles play back to back (its `IDLT` timer not traced);
    eating's chair search range taken as `fAIFindBedChairsDistance` 8000,
    eating where they stand without a chair (inferred), the eating idle the
    tree gives with the food; an
    "in a cell" package's centre is where they stand when it starts; the
    dialogue timer that zeroes talking, the centre reference and a target's
    marker being taken by someone else aren't modelled; faction ranks not
    checked for ownership; the scan reads only the cell (outdoors, the
    square) they're in; `XIBS` as extra data 0x80 and the "child can use"
    flag are inferred from names and data.
  - Actor values: from the record (SPECIAL, skills, health, `AIDT`'s
    aggression…mood, creatures' `DATA`, karma `ACBS` f32 at 16: Doc
    Mitchell 500), `Variable01`–`10` 0; `SetActorValue`/`ForceActorValue`
    set, `Mod`/`Damage`/`Restore` add. Action points, carry weight and
    melee damage as the settings' names say (`fAVDActionPointsBase` 65 +
    `…Mult` 3 × Agility, 150 + 10 × Strength, 0 + 0.5 × Strength; the
    shape isn't traced). Body part conditions 100, and what only effects
    raise (rads, paralysis, hardcore needs…) 0: nothing applies effects
    yet. Not done: health from Endurance and level, skills from SPECIAL,
    inventory weight, resistances.
  - Places: `MoveTo` (with its x, y, z offsets) moves the player or
    anyone in the state (`GameState::place`, `spaces`); `GetInSameCell`
    outdoors means the same grid square. `IsPlayerInRegion`: the cell's
    `XCLR` lists it, or the player stands inside one of its `RPLD`
    outlines (`world::region`; Novac's cells list only the wasteland's
    region, so map regions must be outlines; counting both is a guess).
    `GetMapMarkerVisible`: the marker's `FNAM` 0x01, or `ShowMap`.
    `GetIsCurrentPackage`: the package the state picks now.
    `GetPCMiscStat`: 0 (nothing it counts happens yet) except quests
    completed (whether hidden quests count isn't known).
  - Kept but not shown: forced weather, Bink videos (`FNVIntro.bik`),
    music, scale, unconsciousness, the player's script packages (the bed
    idles in Doc's intro).
  - `IsTalking`: the viewer's "saying a line" (the game also counts a
    flag at actor+0x7D whose setter isn't traced, and talking
    activators speaking for someone).
  - `IsChild` leaves out the game's last test (a loaded model under 110
    units tall ÷ scale).
  - `SendAssaultAlarm Player <faction>`: the member is the nearest in the
    player's cell who notices the player (above −20) within 10,000; the
    game asks for detection level ≥ 1 among loaded actors.
  - `GetCurrentAIPackage` isn't answered for someone fighting (the
    engine's combat package's number isn't traced).
  - `AddItemHealthPercent`: condition kept for weapons only (as
    `weapon_health`, per holder and weapon); armour stays whole.
  - `GetChallengeCompleted` and `GetCasinoWinningStage` are 0: nothing
    counts challenges or casino winnings here yet.
  - `SetRestrained 1` doesn't revive the dead or clear unconsciousness (the
    game's life state would); restrained people still follow packages in
    the viewer (the AI doesn't read it yet).
  - Item ownership in containers (the "ownership added" flag) and the
    "item added/removed" notices aren't modelled.
  - `Look` targets are kept but no head tracking shows them; `PlayGroup`,
    `PlayIdle`, `SwapTextureOnRef` are printed, not shown, by the viewer.
  - Second round (`world::more_functions`): made references number from
    0xFF000001 (the game's own counter isn't traced); `PlaceAtMe`'s ring
    spots aren't cut short by collision (the game casts a line to each, the
    world has no collision); a made leveled actor is picked at the player's
    level (encounter zones' levels aren't kept); `PlaceLeveledActorAtMe`'s
    level modifier isn't kept (what reads it isn't traced); a made
    explosion, projectile or debris does nothing (no explosion system).
    `DamageObject` doesn't check the 3D is loaded, isn't carried out on
    people (another path), and the model swap, explosions, debris and self
    damage per second are kept/told, not shown or applied.
    `bEssentialTakeNoDamage` (essentials going down instead of dying,
    `008a1800`) isn't applied to deaths yet. `SetActorAlpha` and
    Securitron faces aren't drawn. The viewer reports people walking forward
    only (not running) and the procedure outside sandboxes as before
    (fighting 13, walking 0, wander 7, eat 10, talk 4, sleeping 8, else
    none); the player's movement from the keys. `IsTurning`'s 1/2 as left/
    right is the editor's documentation. Autosaves write this project's own
    format to a file of their own. Challenges of kinds other than
    scripted and misc statistics (kills, hits, items, skills, damage,
    crafting) aren't fed yet; the completion sound is `UILevelUp` looked up
    by editor ID; stat changes reach challenges when scripts next run
    (the game does it at once).
  - Image space modifiers: which of tracks 14–20 is which (bloom and
    cinematic; taken in `IMGS` order), linear keys, held past the ends,
    animatable ones ending after their duration, static ones at their
    first keys, several combining (multiplies and adds in turn, tints
    averaged by amount with the largest kept, fades stacked): all
    guesses. Blur, double vision, radial blur, depth of field and motion
    blur aren't drawn. Weathers' modifiers aren't applied.
  - Doors and objects with an `OnActivate` script do their usual thing
    (open, be taken) only if the script calls `Activate` (as
    `GSDocMitchellDoorScript` does); a door set destroyed doesn't open.
  - Inventories: `CNTO` of the holder's base, copied into the state when
    first changed, leveled lists (`LVLI`) picked then for the player's
    level the way the editor documents it (`world::leveled`: chance of
    nothing, highest level reached unless "all levels", one at random
    unless "use all"; not traced in the game's code). `GetItemCount` on a
    holder not yet copied counts only its plain items. The container
    screen moves whole stacks (the game asks how many for big ones).
  - Not done: `GetDistance` between different places (what the game
    gives isn't traced), detection, magic effects, player controls
    (`DisablePlayerControls` is reported, not applied), items' own
    scripts (`OnAdd`), weight, equipping. `nvinspect <Data> play 300
    VCG00 0` lists what's left, each with its first caller.
- Music (`world::music`, `viewer/src/music.rs`): who is aware of the
  player (anyone fighting them, or alive within `fSneakMaxDistance`, × 2
  outdoors; the game's detection isn't run); the combat music's inputs
  (each fighter a group of one, "detects" = within that distance,
  strength = health, 3D distances); the acoustic space (the cell's own
  `XCAS`; placed acoustic spaces aren't read); which references form an
  exterior's marker list and their order (all of the worldspace's audio
  markers; only matters where markers overlap); the random picks use this
  project's dice, not the game's shared Mersenne twister (the rules are
  the game's); a music folder's file order (sorted by name); looping with
  no gap (the game seeks back a tick later); the sets' intro, outro and
  incidental sounds at master × music volume; a held deck (one frame after
  a battle intro) plays on in the viewer; the audio clock is the viewer's
  real time; `SetEnemy`/`SetAlly` aren't applied to the modifier the
  controllers read (not traced). Not done: the Pip-Boy radio (which stops
  all music while on), title/loading/credits music, the game's audio
  pause, `iMusicSynchOverride` (0 assumed: 500 ms).
- Body parts (`world::body_parts`, `viewer/src/combat.rs`): a melee blow whose
  view ray meets no capsule lands on the bone nearest the view that gives a
  part (the game casts 200 units from the weapon, then a line between the two
  actors); skeletons without bodies use the bounds and the nearest bone; the
  weapon node of part 14 taken as the skeleton's `Weapon` bone; base health for
  limb damage = full health without effects (`008803a0`'s derived part not
  traced in detail); a part-14 hit takes its points off the weapon's condition
  as points of its `DATA` health; a dropped weapon stays in the inventory and
  its model is hidden (the game lays it on the ground, and people pick another
  or pick it up again); the weapon drawn isn't replaced when they switch;
  `GetKillingBlowLimb` taken as the fatal hit's part (the game keeps it with
  the dismembered-limbs data). NPCs' attacks carry no part yet (no multiplier,
  no limb damage, no hit location), so only the player's hits and falls
  cripple. Not done: dismembering/exploding (`009b73d0` → `00646e50`,
  `008b43a0`), `IsLimbGone`, stagger, the concussion (`0080be40`:
  `FXCrippleHead` every `fConcussionTimer` 40 s for the player), arm wobble,
  difficulty, VATS, Pip-Boy limb healing.
- Fighting (`viewer/src/fighting.rs`, `world::combat_ai`): where a gunman
  out of its band goes (the game searches the navmesh, `009d5000`; here on
  the line to the target, halfway into the band, else just inside its
  nearer edge); which way a strafe goes (a coin toss); rays for sight from
  60 units above the feet; a creature's radius from its `BSBound` as
  people's ((half x + half y) ÷ 2); its attack reach from its template with
  "use base data" (0x80) and the default style form `0000003D` as the
  fallback (both inferred); an attack's length: a gun's 1 ÷ `DNAM` 88, a
  melee weapon's animation ÷ its attack multiplier `DNAM` 60, creatures'
  and fists' their animation (1 s without one); fire rate within a burst
  `DNAM` 64; running 4 × walking without a run animation; attack power for
  fleeing = damage per second (creatures' bites once a second);
  `fCombatMaxHoldScore` (the setting at `011cea38`, named by the findings);
  the search (to the last known spot, then a random spot within the
  smallest search radius every `fCombatSearchAreaUpdateTime` 5 s, walking);
  fleeing (`fCombatFleeNormalDistance` 2048 straight away, until the threat
  is no longer noticed); paths after a moving target made again every
  0.5 s. Not done: the aggro radius's guard package (creatures and people
  with aggression 1 that attack when you come close: `AIDT` 15/16,
  `iGuardWarnings`, `fGuardPackageAttackRadiusMult`), heard events
  (gunfire noticed at −19), crouching, dodging, cover and reloading from
  it, blocking (no block animations), suppressive fire, grenades, the
  line of fire clear of friends, the combat planner's weapon switching,
  fleeing in combat (threat ratio), followers' `FollowerSwitchAggressive`,
  the per-target 5 s bonus to the attack score, the hit landing at the
  animation's hit key, spread (every shot hits), shot noise for detection.
- HUD (`crates/ui`, `viewer/src/hud.rs`): parts not followed yet are
  hidden (sneak meter, enemy health, quest reminders and compass quest
  markers, hotkeys, region name, radiation, hardcore needs, breath,
  explosives, crippled limbs, damage threshold icons, the prompt box
  "Info", subtitles); the recorded frame drew all of them at alpha 0.
  Messages all get the neutral Vault Boy for 2 s (the game picks icons
  per kind, and a script's message box without buttons its own time).
  XP awards are read from `world`'s "XP +N" notice; "in combat" and "a
  level-up waits" for the XP meter's two player flags are inferred; the
  HUD is hidden while one of the viewer's own text panels is open (under
  the game's menus it shows what `ui::hud::parts_for_menu` leaves); a mask
  lifted shows the pieces as they were before it (the game sets every
  piece's flag each frame); the meters' and compass's first-200-ms delay
  isn't kept; the NPC tick's "hostile" is "fighting the player or
  attacking on sight".
- The game's menus (`ui::menus`, `viewer/src/game_menus/`): menus open
  and close without the fade (`menufade`/`explorefade`); the world stops
  while one is open (the viewer's rule for its menus; the opening's
  fade-in waits under the first message box); `MenuMode` numbers
  for these menus aren't passed to scripts; the controller's buttons aren't
  followed; first-time help messages (`00718630`) aren't shown. Not done:
  the item card (`BM_ItemData`/`CM_ItemData`, the Pip-Boy's `00707e30`),
  barter's price adjustment from `ShowBarterMenu` (the world's event has
  none: 0), the dialogue topic said after a trade, the dialogue camera and
  zoom, pickpocket and companion container modes, armed mines in
  containers, a perk's skill bonuses shown on the level-up page while it's
  picked (`007867a0`), the character generation menu's attributes mode,
  waiting and sleeping (1012, read in `findings\menus.md` §11; the T
  key's text panel serves), terminals and hacking (1057, 1055), books and
  notes (1026), the vigor tester (1074), the face (1036). How pixels per
  unit round for clip windows (taken as the screen's pixel height ÷ 960,
  exact at 1080).
- V.A.T.S. (`world::vats`, `world::vats_camera`, `ui::vats`,
  `viewer/src/vats.rs`): a part's visible share (the game counts pixels
  with occlusion queries; here the share of points along its capsules a
  line from the eye reaches past walls and bodies), read two frames after
  the target comes up; bounds from `OBND` (`vats_camera::bound_sphere`:
  the zoom's radius and middle, the shots' shares; the game reads the
  model's bound sphere); shots hitscan (the game's fly as missiles in
  V.A.T.S.): the projectile is there the frame it's fired, gone the next;
  an attack's first shot fired at once and the rest spaced by the weapon's
  fire rate in the player's time (the game's attack animation fires them);
  a melee blow landing as it's swung (the game asks its attack animation's
  state, not followed); the target's multiplier speeding its animations
  only (its movement and AI timers run at the world's); the player turned
  at the part at once in playback; the third-person view between shots
  isn't drawn (no player body): the player's view shows; a miss's bound
  radius half the `OBND` diagonal; the warp measured between the bodies'
  edges; a melee miss swinging as an ordinary blow. Not done: the camera
  pushed out of walls (`0094a0c0`: a `fCameraCasterSize` 10 sphere cast
  from the location node; shots can end up behind a wall or door), the
  scan highlight (`IFVATSSCAN`), the per-target light, mouse picking and
  edge panning, the damage preview, quaternion-keyed cameras' turns, the
  attacker's bone for shots that follow it (not placed), image space
  cross-fades between shots and the dolly's radial blur, Mysterious
  Stranger / Miss Fortune, Paralyzing Palm's spell, grenades and thrown
  weapons, objects as targets, `GetVATSValue` 8–14 and 16, the "Living
  Anatomy" name line, MenuMode blocks for the V.A.T.S. menu, the side-view
  paths' functions (`GetVATSRight/LeftTargetVisible`,
  `GetVATSFront/BackAreaFree`).
- AI (`world::movement`, `world::social`, `viewer/src/ai.rs`): the path
  smoothers (`0069f010`, `006ad770`) aren't traced: A* + funnel kept; the
  arrival special cases (40², 80²) and the height-tolerance field not
  traced; which callers turn before a path (here: every new path);
  conversation pauses (2/10/50 s; here 2 s and the line's own length);
  `0061b320`'s choice among valid lines (here the dialogue rules); the
  conversation package's 800/600 give-up, "don't control target movement",
  and `PTDT` ≤ 0 → 400 for the side closing in, not done; a seated
  `StartConversation` speaker waits for the player (inferred); greetings
  while walking up for a conversation are allowed (no rule traced); the
  wander spot within the ring (a random point of a random triangle); the
  middle of "near the current location" (where they stood when the package
  began); a wander that finds no spot stands and tries again each frame;
  avoidance's share for someone close behind (negative, as written); out
  of sight the middle-high level (0.15 h) isn't used (no loading cells),
  `008bc7f0(player)` not traced; head tracking turns the body only (no head
  bones); lines without a voice last 0.35 s a word (≥ 2 s); voices aren't
  placed in 3D. Only the paths to a package's place carry their doors (the
  walks of dialogue, conversations, sandboxes and follows don't open doors
  on the way). The dialogue menu's speaker turning (and animating) while
  the rest of the world stands still is inferred. Schedules: "-1 / 0 =
  any"; durations wrap past midnight.
- Walking (`physics::CharacterShape::PLAYER`): the sizes are the game's
  (see Walking), but the shape is a capsule from the feet, not the game's
  eight-sided hull with a 31.7-unit cone underneath floating 0.7 above
  the feet; the game's step test (contacts up to 4/3 of a step, cast
  ahead by radius + step ÷ tan 47°, ramps) isn't copied: steps are
  measured from the point stood on, so a ledge above a step can't be
  climbed bit by bit. No air control or creature sizes (`BSBound`).
  Falls: where the feet left the ground is the point last stood on when
  walking off an edge.
  Not done: pushing or knocking over clutter (needs its bodies simulated
  with the contact rule above), the Havok commands a person's opening of a
  door gives its bodies (`0047a560`, `bAnimateDoorPhysics` off; people wait
  for the swing instead), a swinging leaf's collision following it with
  `bAnimateDoorPhysics` on (it does, but the game's way isn't traced),
  people blocking each other's walking (only the player is blocked), the
  proxy's 0.05 keep distance; creatures' heights are 128 × scale (the game:
  their `BSBound`). 19 `bhkTransformShape`s in 12 models have matrices that
  also scale by 1/7 (exporter leftovers, e.g. `mettundoorway01.nif`); they're
  applied as stored, which puts those pieces far from their models (what
  Havok does with them isn't traced).

- Bloom's average: in both recordings `TimingData.z` was 0, so the game
  kept the previous average unchanged (no adaptation, as the user said);
  what that held isn't known. The viewer uses the current frame's
  average, which only matters when its r + g + b passes the target lum
  (bright scenes). The camera exposure (EV 5.5) is cancelled in the
  shader so 1.0 = white.
- Reflections: the view vector is per pixel (the game's is per vertex).
  A model whose slot 4 names a flat (non-cube) file is given that picture
  on all six faces, as for the default (only the default is confirmed). A
  cube map in the mask slot (the Vigor Tester glass) is read as its first
  face; what the game's card does there isn't known.
- Emittance by the hour (`viewer/src/emittance.rs`): starting indoors with
  no weather state, the viewer acts as if the player had come in through
  the place's first door to the outdoors (its square's climate and
  weather region, the regions' weathers rolled once); a new game or a
  save would bring its own. Doc Mitchell's then gets `GSWeatherRegion`
  (`NVWastelandGS`, whose high noon sunlight is the day's), so its lamps
  are brighter than in the recording, which had a region without weather.
  Placed lights tinted by a region's Emittance take its colour when the
  place loads (as the game starts), not as the hour moves; the lightning
  flash isn't added.
- Specular saturation is per light in the viewer, per pass of up to 3
  lights in the game (only differs where highlights add past 1).
- Fog and the unlit-surface rules aren't in the CPU renderer yet. Which
  surfaces the game leaves unfogged in their main pass (some draws have
  `Toggles.y` = 0) isn't worked out.
- Which lights give highlights (all, in the viewer); full mask strength
  without a normal map.
- Not drawn yet: actors. The CPU renderer has no normal maps, specular,
  reflections, image space, fog, or tint.

## Comparing with the game

- `nv-viewer <Data> <cell> --at X,Y,Z,HEADING[,PITCH] --screenshot out.png`
  renders at 1920x1080 and quits; X,Y,Z,HEADING,PITCH are the console's
  `player.getpos x/y/z`, `player.getangle z/x`. Read the PNG and sample
  pixel colors (System.Drawing in PowerShell) against the user's
  screenshot instead of asking them to look. The image space step is
  affine in the stored color, so region averages can be ungraded exactly
  and regraded with other values to fit parameters.
- **Confirmed field of view:** `fDefaultFOV=75` is the width of a *4:3*
  picture; wider screens keep its height (59.8° tall, 91.3° wide at
  16:9). Lined up against an in-game shot at a known `getpos`; the eye
  height of 120 matched too. `preview::cell::{vertical_fov,
  horizontal_fov}`.
- The game's other settings (`FalloutPrefs.ini`): 1920x1080, HDR on, 4x
  MSAA.
- `FalloutNV.esm` has a record with a wrong zlib checksum (`LAND`
  00150FC0); the game ignores it, so the loader accepts it when the size
  is right.
- Test poses (the user's screenshots):
  `GSDocMitchellHouse --at 2402.57,1592.28,7360.82,270.34,0.08` (fog
  73,68,48 from 64 to 5000 units, power 0.6), and
  `MojaveOutpostBarracks01 --at 2296.23,2682.00,9329.05,208.5,-0.09`
  (the console said 2211.50 for the heading; 208.5 was found by lining up
  the picture, so check headings that way when they look odd). Both
  places now have recorded frames with known cameras (below), which are
  better references than screenshots.

**Best reference: the recorded frame itself.** `%USERPROFILE%\nv-re\
trace\frame0010057127.png` is the game's own output (lossless), and its
exact camera is known: `GSDocMitchellHouse --at
2406.234,1617.788,7358.806,266.39,0.76` (eye 7478.8, i.e. feet + 120),
with the recording's weather state `--weather-region VMapGoodspringsRegion
--run "set GameHour to 15.72"` (the wall lamps' glow follows the hour).
Found from the recording: heading and pitch from any draw's
`ModelViewProj` w row; the eye from `EyePosition` (c16, model space) of a
mesh with no node transforms, e.g. the rug `dlc04mhallrug03` (85 verts)
placed at 2208,1602,7360 (eye (198.234, 15.788, 118.806)). Don't use the
ceiling lamp: its mesh node has an offset. Useful checks (System.Drawing
in PowerShell): both pictures cropped side by side, a per-block
viewer/game brightness ratio map, region averages.

Doc Mitchell's hallway against that frame (2026-10-01, after bloom, the
lamp colour, reflections, per-vertex values and specular without the
material colour): plaster 1.03–1.04, ceiling 1.00–1.01, wainscot,
chair rail, doorway, far end 1.00–1.04; lamp shades and their glow match.
Still off: a thin strip where the left wall meets the ceiling 1.19×, the
rug's middle 1.13× (near 1.07×). (The earlier "floorboards 0.75×,
ceiling 1.2×" came from comparing with a JPEG taken from a slightly
different spot.) Nothing like ambient occlusion is drawn by the game here: the
frame has no such pass; the darkening in corners is the meshes' vertex
colours, which the viewer applies. With reflections the frames and the
glass over the right doorway moved to the game's values.

**Barracks recording** (`%USERPROFILE%\nv-re\trace_barracks`, frame
`frame0026759178.png`, console open): `MojaveOutpostBarracks01 --at
2362.209,2647.2,9335.025,207.69,-1.90` (eye 9455.0; from the tube
fixtures' `ObjToCubeSpace` offsets, which all agree, and a
`ModelViewProj` w row). Against it (2026-10-01, all fixes): tubes, bar,
shelves, floor, walls, ceiling and windows within ±3%; the left window
area 1.07×, the far right fixture 1.13×, chairs at the back left a little
bright. (The ceiling was 0.79× before specular lost the material colour;
the floor 1.14× until the directional light pointed east.) The user's earlier screenshot of the barracks shows the tubes far
more blown out than this frame; the recording is the reference.

**Goodsprings recording** (`%USERPROFILE%\nv-re\trace_goodsprings`,
`FalloutNVGoodsprings.trace`, frame ending at call 152857171, picture
`frame0152857171.png`; first person, HUD off): `WastelandNV --at
-72151.5,639.2589,8281.634,0,0` (eye -72151.5, 639.2589, 8401.634 from the
water draw's `QPosAdjust` and the grass's `ModelViewProj`; heading and
pitch exactly 0: the view matrix is the axis swap to within 2e-7 rad).
Hour 13.0978 (13:05:52: the sun's x = −11.18 of 800; the grass's wind
phases give 13.0977), weather `NVWastelandGS` (no change running). To
reproduce: `nv-viewer <Data> WastelandNV --at
-72151.5,639.2589,8281.634,0,0 --run "set TimeScale to 0" --run "set
GameHour to 13.09783" --cloud-time 58.022 --wait 2 --screenshot out.png`.
Freezing the clock matters: otherwise the hour runs on at `TimeScale`
while the squares load and the sun moves enough to change the terrain
by 1–2%. `--cloud-time 58.022` puts the clouds where the recording had
them (`TexCoordYOff` 0.29 on layer 3). Tools there: `analyze2.ps1`
(`analyze.ps1` plus render target, textures, blend states, matrices),
`shader.ps1 <handle>` (one recorded shader), `compare.ps1` (block ratios
and named regions), `fit.ps1`, `crop.ps1`, `terrainvb.ps1` (a recorded
terrain vertex buffer). Viewer / game by region (R G B), before → after
this work: slope (terrain) 1.024 → 1.000; road (statics) 1.025 → 1.010;
road middle 1.03 → 1.003; truck 1.013 → 1.002; store 1.015 → 1.001; far
left buildings 1.03 → 1.000; far hills 1.07–1.10 → 1.00; low sky 1.13–1.21
→ 1.002; upper sky and the brightest cloud 1.004 / 1.001 (before: clouds
on the wrong shape and not lined up). Left: Doc Mitchell's house windows
1.20 (see "Still guesses"), grass and shrubs (blades placed by a random
generator, the missing `wastelandshrub01.spt`), the "discovered" notice.

## Coverage

`%USERPROFILE%\nv-re\coverage\COVERAGE.md` is the inventory of everything the game has,
counted from its own files and exe, each item marked done / partial / not started in nv-rs:
records.md, files.md, functions.md (generated by `nvinspect <Data> coverage records|files|
functions --official`), exe_systems.md (form types, classes, extra data ids, menus and their
ids, console commands, actor values, perk entry points, effect archetypes, package types,
animation groups, controls), settings.md, shaders.md, features.md (115 player-visible
features with sizes). Plan work from its "Gaps" list. When a module starts reading a record
type, subrecord, file kind or NIF block, update `crates/nvinspect/src/coverage_table.rs`
(tests fail when a form type the exe knows has no entry); refresh steps are in COVERAGE.md.
Facts found doing it: the form type table is at `01187000` (12-byte entries, 121 types);
`BSExtraData::BSExtraData` (`0040ec80`) takes the extra data type, so every `Extra*` class's
number is known (133 of them; `ExtraIgnoredBySandbox` is 0x80, as `XIBS` was guessed); the
menu ids are made in `0071e420` (37 menus; `MenuMode N` = these); Gamebryo classes use their
own NiRTTI (`00a7b610`, 495 classes); the console table is at `0118e8e0` (206 commands); the
hacking game's words come from `Data\Menus\FalloutDict.txt` (`00768070`), the death poses from
`Death.psa` (`0087e130`), LOD settings from `Data\LODSettings\%s.DLODSettings` (`006fc490`);
`lsdata\*.dat` is LIPSinc lip-sync generator data (XOR 0x85 text).

## Recording the game (apitrace)

The user installed apitrace (`%USERPROFILE%\OneDrive\Desktop\
apitrace-latest-win32`, 32-bit build). Its `d3d9.dll` placed next to
`FalloutNV.exe` records every Direct3D call while the game runs; the
recording lands on the Desktop as `FalloutNV.trace`. Recordings so far:
`FalloutNVDocMitchell.trace` (Doc Mitchell's hallway; `%USERPROFILE%\
nv-re\trace`) and `FalloutNV.trace` (Mojave Outpost barracks;
`...\trace_barracks`). Rename a recording before making the next one.
**Remove that `d3d9.dll` from the game folder when not recording**
(every launch records otherwise).

**Making a recording without the user** (done 2026-10-01 for Goodsprings;
the user allowed full control of the desktop): copy the `d3d9.dll` into the
game folder; `Start-Process steam://rungameid/22380` opens the launcher
(running `FalloutNV.exe` directly gives a Steam error); click PLAY
(physical pixels: make the process DPI-aware; the screen is 2560×1440 at
125%). The game reads the mouse through DirectInput, so drive it with keys
sent as scan codes (`keybd_event` with `KEYEVENTF_SCANCODE`; arrows need
the extended flag): Down then Enter = Continue, Enter = Yes (loads the
latest save); the console key is scan 0x29; `coc goodsprings`, `set
gamehour to 13`, `tm` (hides the HUD), then `qqq` quits. Bring the game
window to the front before every key (other windows, e.g. an agent's
viewer, take focus). Helpers: `%USERPROFILE%\nv-re\recording\drive.ps1`
(dot-source it; `Run-Console`, `Send-Text`, `Focus-Game`, `Save-Screen`).
Then remove the DLL and move the trace off the Desktop. Reference captures
of the user's reports live in `%USERPROFILE%\nv-re\references\`
(`report_00N\`: `game.png`, `viewer.png`, `compare.png`, `camera.txt`;
`README.md` lists them); `references\tools\game-driver.ps1` is the newer
driver: it clicks away the DLC "items added" boxes (`coc`/`cow` from the
main menu raises all nine over 4–9 minutes, each eating every key until OK
is clicked with the game's own cursor), reads the console's open state from
its pixels, and keeps the display awake. `scof` files are only flushed on
the next `scof`. Starting with `cow` from the main menu crashed the game
(`0x00477c6d`) at some Goodsprings spots: load a save instead. **Reading the
game's state**: the shipped game keeps `scof <file>` (SetConsoleOutputFile,
handler `005d9170`): everything the console prints then also goes to that
file (`scof 0` stops), so `player.getpos x`, `getav`, `getstage`, `sqv
<quest>` (a quest's variables) and the like can be read back without
screenshots, for comparing behaviour with nv-rs. The INI's
`sStartingWorld`/`sStartingCellX`/`Y` (code `00876dd0`) don't skip the
main menu. The game's own idle camera turns third person after about a
minute without input.

**Goodsprings recording** (`%USERPROFILE%\nv-re\trace_goodsprings\
FalloutNVGoodsprings.trace`, 4 GB, 23,571 frames): first person on the
road at 13:00, HUD hidden, the frame ending at call 152857171
(`frame0152857171.png`); later frames the idle third-person camera around
the player. Being analysed (see the outdoor notes once merged).

Turning a recording into a reference (about 10 minutes): `apitrace dump
--grep=Present --arg-names=no` for frame ends; one late frame with
`--calls=A-B --arg-names=no` → `frame.txt`; `--grep=CreatePixelShader` /
`CreateVertexShader` → `create_ps.txt` / `create_vs.txt`;
`analyze.ps1 -Dir <folder> -Sdp <package dump folder>` → `draws.txt`;
`apitrace dump-images --calls=<Present>` for the picture; the camera from
`ObjToCubeSpace` / `EyePosition` of a placed object with no node offsets
and a `ModelViewProj` w row.

- Tools and outputs live in `%USERPROFILE%\nv-re\trace`: `presents.txt`
  (frame ends), `frame.txt` (one frame's calls), `create_ps.txt` /
  `create_vs.txt` (every shader, disassembled with its constant table),
  `analyze.ps1` (per draw: game shader name, vertex/primitive counts,
  every named constant with its value) → `draws.txt`.
- `apitrace dump --calls=A-B`, `--grep=Name`; `apitrace dump-images
  --calls=N -o prefix` replays to a PNG (works on this PC). A plain
  `dump` prints argument names (`StartRegister = 2, pConstantData =
  {...}`); `frame.txt` was made without them. The recording is about
  2,250 frames; Doc Mitchell's house is drawn from frame ~1,438 on.
- Find a mesh's draws by its vertex count (from `nvinspect nif`).
- Some pixel shaders in use aren't in package 13 (e.g. a one-pass
  diffuse + specular variant); the recording has their code.

## Reading the game's program

The user chose to decompile `FalloutNV.exe` to read the engine-side
values. Everything lives outside the project and must never be
committed or shared:

- `%USERPROFILE%\nv-re\FalloutNV_unpacked.exe`: a copy with Steam's
  wrapper removed (by the user, with Steamless). The installed game is
  untouched.
- `%USERPROFILE%\nv-re\tools`: Ghidra 12.1.4 and Java 21 (Temurin),
  unzipped, nothing installed system-wide. Headless analysis is done and
  saved in `%USERPROFILE%\nv-re\ghidra` (project `FalloutNV`).
- `%USERPROFILE%\nv-re\scripts`: Ghidra scripts `FindRefs.java` (code
  using a string), `Decompile.java` (functions to C), `Symbols.java`
  (classes and vtables). Run with `analyzeHeadless.bat ... -process
  FalloutNV_unpacked.exe -noanalysis -readOnly -scriptPath ... -postScript`.
  Running them needs the user's permission in this environment (the
  auto-mode classifier blocked it twice, the second time on 2026-10-01
  while looking for the terrain weights; ask the user to allow it).
- Use it to learn facts (constants, formulas, rules) and write our own
  code; never copy decompiled code into the project.
- Extra scripts: `Disasm.java` (instructions in a range),
  `DecompileRange.java` (all functions in a range into one file),
  `FindPointers.java` (4-byte values equal to an address, e.g. table
  entries), `Dump.java` (values at an address), `Refs.java` (functions
  referring to an address); `ghidra.ps1 <Script.java> args...` runs one.
  Avoid `%`, `|` and `^` in arguments (Windows' shell mangles them).
- Console commands are a table of 48-byte entries: name, short name,
  opcode, help text, ..., handler function at +24. Find an entry with
  `FindPointers` on the name string's address.
- **Found so far** (facts, by function address):
  - The image space manager is the object at `[011f91ac]`. Its working
    copy of the settings is 0x98 bytes at manager+0x14, copied from the
    cell's record (manager+0xac) or a forced one (manager+0xb0).
  - Image space record loader (`0052b440`): a DNAM shorter than 152 bytes
    is read as: first 14 floats as stored, float 14 set to 1.0, the rest
    one slot later. So the cinematic values in a 132-byte record are at
    file float 24.., in memory 25.. (what `world::image_space` reads).
  - Per frame (`00b8b9a0` reset, `00b8d020` finish), modifiers' tints are
    averaged weighted by their amounts; with no modifier, the shader
    gets the record's tint color and amount unchanged. **No doubling**
    (the earlier ×2 fit is removed).
  - The HDR + cinematic shader is set up in `00b90200`: source
    `imagespace\2x\p\HDRBlur.p.hlsl` with defines CINEMATIC, SHBLEND,
    TONEMAP (precompiled as `ISHDRBLENDINSHADERCIN`). Its constants are
    set by shared base-class code not traced yet.
  - `PrintHDRParam` (`005c9a00`) prints the HDR settings; sunlight,
    tree and grass dimmers are globals at `011f9190`/`011f918c`/`011f9188`.
  - Shader constants reach Direct3D through generic Gamebryo wrappers
    (`00e89910`, `00e899a0`, `00e89a40` call the device's
    SetPixelShaderConstantF, vtable +0x1B4) fed from constant-table
    entries; which game value feeds which register isn't traced yet.
  - `sisme 0`/`sisme 1` (SetImageSpaceModifiersEnable, handler
    `005d78c0`) switches image space modifiers off/on in game.
  - Emittance: `ExtraEmittanceSource` is extra data type 0x67 (form at
    +0xC; getter `00421d20`, reference wrapper `00569580`). `005453b0`
    applies it to a reference's 3D: the reference's own source, else
    `[011dea3c]+0x760`; `[011dea3c]` is the player (`PlayerCharacter::
    LoadGame` `00956f70` fills it) and +0x760 is a `TESRegion` (the
    player's current region; cleared by forced weathers `0063d0e0`, the
    console weather command `005b7590`, and on moving into an exterior
    cell). The source's vfunc +0xC0 gives the colour (`TESRegion`: +0x2C;
    vtable `0102397c`); `00b55480` stores its pointer at shader property
    +0x34 for properties with flag 0x20000000. Lights (form type 0x1E)
    take another path (`00844700`).
  - Sky: `006335d0` sets the shaders' sun colour to sky colour (+0x48) ×
    sunlight dimmer / 3.75 (`[0104ebc0]` is the double 3.75).
  - Collision filter: `00c84740` (is collision enabled, filter infos A, B);
    layer matrix at `01267f20` (43 × u64), part matrix `01268078`, built by
    `00c828f0`; layer names `011b0810`; filter description `00c849b0`
    ("COLFILTER", "-LAYER", "-NOCOL" ...).
  - Placed primitives: `004a4e90` (type 1 box, 2 sphere, 3 plane),
    `00576990` dispatch by base (PWAT, `CollisionMarker` `[011ca240]`,
    ASPC, ACTI), `0056eab0` collision marker body, `0056d7e0` trigger
    phantom; `ExtraPrimitive` 0x6B, `ExtraCollisionData` (`XTRI`) 0x72,
    `ExtraAction` (`XACT`) 0x0E.
  - Scaled clones: `00c8f2a0` (rigid body), `00cb30d0` (`bhkRigidBodyT`),
    `00cca6f0` builds `hkScaledMoppBvTreeShape`.
  - Terrain height field: `00cb0820` (triangle radius 0.5 at `01016248`).
  - Terrain manager (`006fc490` manager, `006fd210` node constructor,
    `006fdaa0` per-frame update, `006fe550` split test, `006fede0` loaded
    area test, `006fe830` box distance, `006feb20` geomorph factor,
    `006ff2c0`/`006ff3f0`/`006ff380` the parent-texture fade, `006fdfc0`
    object blocks, `006f6d10` block file choice, `006fe330` tree blocks).
    Settings (objects at): `fSplitDistanceMult` `011d86e0`,
    `fMorphStartDistanceMult` `011d8754`, `fMorphEndDistanceMult`
    `011d86d4`, `fBlockLoadDistance` `011d877c`, `fBlockLoadDistanceLow`
    `011d8724`, `fTreeLoadDistance` `011d8788`, `fDetailTextureScale`
    `011d86f8`, `uTerrainTextureFadeTime` `011d8740`,
    `bKeepLowDetailTerrain` `011d8760`; `uGridsToLoad` `011c63cc`; the
    loaded grid's middle cell is TES+0x24/+0x28 (`[011dea10]`).
  - Character contacts: `00c711d0` (per layer: clutter/props as ground,
    characters as walls, `fMoveLimitMass` at `011b0128`).
  - Extra scripts: `FindPush.java <hex>` (every `PUSH constant` and the
    call after it, e.g. extra data type IDs), `FindInstr.java <regex>
    [start end]` (instructions matching a pattern, optionally in an
    address range).

## Next step

Interiors match the recordings closely and can be walked; the outdoors
load and stream around the player (terrain, objects, weather light, doors
both ways); people idle, walk, sit, follow, sandbox and go between places
through doors as their packages say, carry and fight with their weapons,
and fight by faction; dialogue, the game's scripts, effects and spells run
(`nv-viewer <Data> --new-game` plays the opening through Doc Mitchell's
intro: the sex box, name, SPECIAL, the psych exam's tags, traits).
`nvinspect <Data> play SECONDS QUEST STAGE` answers message boxes with
their first button. Levels, experience, perks, falls, the character
controller, the death push, skill-check tags, waiting and fast-travel
rules are read from the game's code.

Working method now: research agents read the exe with Ghidra (each with
its own project copy `%USERPROFILE%\nv-re\ghidra_N`, set by
`$env:NVRE_GHIDRA`) and write reports to `%USERPROFILE%\nv-re\findings\`;
implementation agents work in copies under `%USERPROFILE%\nv-re\work\
<topic>\nv-rs` (with a pristine `base` beside it; `work\changed.ps1 -Dir
<folder>` lists what changed) and write `NOTES.md`; changes are merged
into the project from there (`work\merge.ps1 -Dir <folder>`, which backs
up every file it touches to `work\merge-backups\`). Before writing NOTES an
agent runs `work\sync.ps1 -Dir <folder>`: it brings the project's changes
since the snapshot into the work copy (merging where both changed a file)
and resets `base`, so overlaps are settled by the agent who knows the work,
not at merge time. An agent's shell starts in the real project folder:
every command must use absolute paths into its work copy. When a merge
goes badly, restore the backed-up files, delete its new files, check
`work\guard.ps1` says nothing changed, and have an agent integrate the
work into a fresh copy of the project instead.

Next:

0. Scripts: more functions (`nvinspect <Data> play 300 VCG00 0` lists
   the ones scripts wait on; with no player position `GetInCell` and
   `GetDistance` can't be answered there); fighting's next steps
   (third-person reload animations, limbs coming off, NPCs' hits finding a
   body part, V.A.T.S.'s own menu (`VATS_menu.xml`), camera shots and the kill cam,
   thrown weapons, an in-game check of its chances; the combat AI's leftovers:
   the aggro radius's guard package, heard events, cover, dodging,
   blocking, grenades, suppressive fire); the player sitting; sleeping in
   beds (full heal); the AI's leftovers (`world::movement`: the path
   smoothers, the conversation package's give-up, head tracking of head
   bones, weapon-kind turn animations); a recording of two NPCs talking,
   someone greeting the player, Sunny walking up (turn rates, pauses).
1. Outdoors, still missing: the sun's glare, underwater, distant water at
   the quadtree's levels. A recording from high ground (or Lake Mead)
   would show the coarser distant-land levels and a fade in progress.
2. People: FaceGen skin tones, more animations (reloads, getting in and
   out of chairs), creatures' attack animations; eye and head tracking,
   expressions and a line's head turns; the ragdoll's limits
   against Havok's (a recording of a death would show the bodies' frames).
3. (Done: lights across square borders, `cellview::lights_reaching`;
   each square's materials get every loaded light reaching its ground
   area, nearest first, 64 at most. People brought in later keep the
   place's lights as they were.)
4. (Done: the character controller's values, `findings\physics.md`.)
   Left: its hull shape and step test, creature sizes.
5. Small rendering leftovers: Doc Mitchell's corner strip and rug, the
   barracks' far fixture, actor shadows once actors exist.
