# nv-rs

The first building block of a from-scratch Rust engine for **Fallout: New Vegas**,
in the style of [OpenMW](https://openmw.org/) and
[iw4L](https://github.com/vladtrc/iw4L): the engine code is original, and the
game itself (world, characters, quests, dialogue, art) is read at runtime from
the data files of a copy you own. No game data lives in this repository.

"nv-rs" is a working name. Rename it whenever you like.

## What's here

| Crate | What it does |
| --- | --- |
| `crates/esm` | Reads plugin files (`.esm` / `.esp`) and combines them into a load order. No external dependencies. |
| `crates/bsa` | Reads BSA archives, where the game keeps its meshes, textures and sounds. |
| `crates/nif` | Reads NIF meshes and turns them into drawable geometry. |
| `crates/dds` | Reads DDS textures, decodes them, and writes PNG. |
| `crates/assets` | Finds any game file across loose files and the archives the game loads. |
| `crates/world` | Reads cells and the objects placed in them: position, rotation, scale, lights, doors. |
| `crates/preview` | Draws a cell into an image on the CPU (no GPU or window), to check placement against the game. |
| `crates/cellview` | Turns a cell into plain meshes, textures, materials and lights, ready for a real-time renderer. |
| `crates/shaders` | Reads the game's compiled shader packages (`Data\Shaders\*.sdp`) and disassembles their Direct3D 9 shaders, so the renderer can port the game's exact lighting and color math. |
| `crates/mp3` | Decodes the game's music: MP3 (MPEG-1, 2 and 2.5 Layer III), with gapless trimming and a WAV writer. |
| `crates/ui` | The game's menu system: its menu XML, fonts and textures worked out the way the game's code does, and the HUD's layout and updates. |
| `crates/physics` | Collision: the models' solid shapes as triangles, ray tests, a walking capsule for the player (the game's controller sizes, step, slope and gravity) that falls, slides along walls and climbs low steps, and ragdolls. |
| `crates/nvinspect` | Command-line tool built on all of them, for browsing game data. |
| `crates/testdata` | Builds small NIF, DDS and plugin files for the tests. |
| `viewer` | The real-time viewer: walk around a cell in a window (uses Bevy). |

None of the crates use outside libraries. The viewer uses the Bevy engine,
so it's a separate project in `viewer/`: building the rest never downloads it.

The `esm` crate handles:

- the `TES4` file header: format version, author, description, master list
- the full group tree, including nested cell, worldspace and exterior-block groups
- 24-byte record headers, with an index by record type and by form ID
- subrecords, including `XXXX`-prefixed subrecords over 64 KiB
- zlib-compressed records, through a built-in DEFLATE decoder with checksum verification
- Windows-1252 text, so names like *O’Malley* come through correctly
- typed decoding of weapon stats (damage, clip size, value, weight, condition)
- **load order:** picks the active plugins (`plugins.txt`, `.nam` files,
  masters of active plugins), orders them the way the game does
  (master-flagged files, then the rest, each by file date, masters moved
  before the master files needing them), renumbers form IDs to their
  load-order index and resolves overrides, keeping every version of each
  record; [MODS.md](MODS.md) has the traced rules

The `bsa` crate handles:

- version 104 archives (Fallout 3 / New Vegas) and version 103 (Oblivion)
- folder and file tables, per-file compression, and names embedded in file data
- lookup by path, ignoring case and slash direction
- reading file data from disk only when asked, so the multi-gigabyte texture
  archives don't need to fit in memory
- a check of every stored name hash against the names read

The `nif` crate handles:

- Gamebryo version 20.2.0.7 files with Bethesda version 34 (Fallout 3 / New
  Vegas), using the block size table to skip block types it doesn't decode
  (collision, animation, particles), so unknown blocks never derail a file
- the node tree, including fade, multi-bound, switch and LOD nodes, with
  transforms composed from the root down
- `NiTriShape` and `NiTriStrips` geometry: positions, normals, texture
  coordinates, vertex colors and triangles (strips converted to triangles)
- shader properties and texture sets (diffuse, normal map, glow...),
  single-texture shaders (no-lighting, sky, tile, tall grass), the older
  `NiTexturingProperty` style used by hair and some ported assets,
  materials, alpha blending and testing, and double-sided rendering
- properties inherited from parent nodes, as the engine applies them
- hidden objects, editor markers and collision meshes left out, as in game
- skinning (which bones move which vertices, and the dismemberment
  partitions that hold the gore caps), skeletons, and animations: `.kf`
  sequences as keyframes or the compressed B-splines most character
  animations use
- collision: the Havok shapes the game walks on (triangle meshes inside
  MOPP trees, triangle strips, boxes, spheres, capsules, convex hulls, and
  the transforms and lists that combine them), in game units, with each
  body's layer, whether the model's animation moves it (door leaves) and
  how far its surface stands out (Havok's convex radius); which layers stop
  people is the game's own layer matrix
- per-file detection of two layout details that the format description is
  vague about, by checking which variant fits the recorded block sizes
- export to Wavefront `.obj`, so meshes can be checked in any 3D viewer

The `dds` crate handles:

- DXT1, DXT3 and DXT5 (BC1-BC3), including DXT1's transparent mode
- ATI1 and ATI2 (BC4/BC5), with normal maps' Z rebuilt for viewing
- uncompressed layouts: A8R8G8B8, X8R8G8B8, R8G8B8, R5G6B5, A1R5G5B5,
  A4R4G4B4, L8, A8L8, A8, and DX10 headers for the same formats
- mip chains (including files missing their smallest levels) and cube maps
- raw level data for GPU upload, and CPU decoding to RGBA
- a PNG writer with its own deflate compressor, so any texture (or render)
  can be looked at

Its decoders were checked against Pillow and match it pixel for pixel on
DXT1, DXT3, DXT5, BC5 and uncompressed files.

The `assets` crate finds files the way the game does:

- the archives in the game's own list (`SArchiveList`, read from
  `Fallout.ini` in `Documents\My Games\FalloutNV`, or else the install's
  `Fallout_default.ini`), then the game's patch archive `Update.bsa` (which
  those ini files don't list, but whose meshes records in FalloutNV.esm
  use), then any archive whose name starts with an active plugin's
  (`DeadMoney - Main.bsa` for `DeadMoney.esm`), in load order
- which archive's copy of a file wins follows the game's archive list (mod
  and DLC archives before the base game's; [MODS.md](MODS.md)), and loose
  files in `Data` replace archived ones when `bInvalidateOlderFiles` is on
  (the exe's default), with `ArchiveInvalidation.txt` applied
- paths are matched ignoring case and slash direction, and texture and model
  references are normalized the way meshes and records write them

The `world` crate reads a cell the way the game loads it:

- finds cells by editor ID, form ID or display name
- reads the cell's flags, grid position and lighting, including lighting
  templates and the per-field inheritance flags that pick values from them
- collects every placed object (the winning version, so a mod that moves a
  chair moves it here too), with its base object's model, and for lights,
  their radius and color
- leaves out what the game doesn't show when the cell first loads: deleted
  and initially disabled references, and those whose *enable parent* says so
  (followed up the chain, including "opposite of parent")
- separates editor markers, people and creatures from objects to draw
- static collections come with their pieces, for when the combined model is
  missing; armor on the ground uses its ground model
- finds where the player arrives: the cell's `coc` marker, and the far side
  of each load door
- turns a placement into a transform, with the rotation convention as a
  choice (see [How placement works](#how-placement-works))
- people and creatures: what each is made of, from its race (head, body,
  hands and their skin textures), the clothes in its inventory (which hide
  the body parts they cover), hair and its colour, eyes and head parts
  (beards, eyebrows), following templates (`nvinspect <Data> actor
  DocMitchell` lists them)
- the outdoors: worldspaces and their grid of cells (with the objects the
  game keeps loaded everywhere sorted into the square they stand in),
  terrain (heights, normals, vertex colours, and the textures painted on
  each quarter of a cell) turned into meshes, and each square's light from
  its weather (sun, ambient and fog colours by day)

The `preview` crate is a small software renderer: textured triangles with a
depth buffer, per-pixel lighting from the cell's ambient and directional light
and its light sources, alpha testing and blending as the meshes specify
(including material opacity and the angle-based fade of effect shaders), decals
drawn over what they lie on, back-face culling that works out each mesh's
winding from its normals, near-plane and ceiling clipping, and supersampling.
It exists to check placement before the real-time renderer depends on it.

## Running it on your game

1. Install Rust from <https://rustup.rs>.
2. Build:

   ```sh
   cargo build --release
   ```

   The program ends up at `target/release/nvinspect` (`nvinspect.exe` on
   Windows); the examples below just call it `nvinspect`.

3. Give `nvinspect` a target: a single plugin file, the whole `Data` folder,
   a `.bsa` archive, a `.nif` mesh or a `.dds` texture. On Steam for Windows the `Data` folder is usually
   `C:\Program Files (x86)\Steam\steamapps\common\Fallout New Vegas\Data`.

### The whole game: the Data folder

Pointing at the `Data` folder loads FalloutNV.esm, the DLC and your active
mods together, in load order. Form IDs are shown with their load-order index,
as the in-game console shows them. When the winning version of a record comes
from a DLC or mod, that file's name is shown next to it.

```sh
# The load order: each file's position, new records and overrides
nvinspect "<Data folder>"

# Every weapon in the game, DLC included, with the stats that win
nvinspect "<Data folder>" weapons

# One record, plus every file that changes it
nvinspect "<Data folder>" show 0000000F
```

The active list comes from `%LOCALAPPDATA%\FalloutNV\plugins.txt`. Use
`--plugins FILE` to read a different list, or `--official` for just the base
game and official DLC. Which plugins load and in what order follows the game's
rules (file dates, not the list's order; see [MODS.md](MODS.md)). To see
which script extender (NVSE) functions a mod's scripts call, run
`nvinspect "<Data folder>/SomeMod.esp" coverage nvse`. **Mod Organizer 2 users:** MO2 keeps mods outside the
`Data` folder, so only the files physically in `Data` are found.

### One plugin

```sh
nvinspect "<Data folder>/FalloutNV.esm"                 # header and record counts
nvinspect "<Data folder>/FalloutNV.esm" types           # every record type
nvinspect "<Data folder>/FalloutNV.esm" list NPC_ --grep ranger
nvinspect "<Data folder>/FalloutNV.esm" list CELL --limit 50
nvinspect "<Data folder>/DeadMoney.esm" weapons
```

### Archives

```sh
# Format, flags, file counts, and the name-hash check
nvinspect "<Data folder>/Fallout - Meshes.bsa"

# Every file, or just the ones matching a word
nvinspect "<Data folder>/Fallout - Meshes.bsa" files --grep 10mm

# Save one file to the current folder (or to a path you give)
nvinspect "<Data folder>/Fallout - Meshes.bsa" extract <path from files>
```

`extract` never overwrites an existing file unless you add `--force`, and it
never modifies the archive.

### Meshes

Meshes can be read straight from an archive or from a loose `.nif` file.

```sh
# Check every mesh in an archive: how many read cleanly, and why any failed
nvinspect "<Data folder>/Fallout - Meshes.bsa" check-nifs

# Describe one mesh: its parts, triangle counts and textures
nvinspect "<Data folder>/Fallout - Meshes.bsa" nif <path from files>

# Convert a mesh to .obj, then open it in Blender or Windows 3D Viewer
nvinspect "<Data folder>/Fallout - Meshes.bsa" obj <path from files>

# The same for a loose file, plus a list of every block in it
nvinspect some_mesh.nif
nvinspect some_mesh.nif blocks
nvinspect some_mesh.nif obj
```

The `.obj` files are converted to the Y-up orientation most viewers expect.
They carry the geometry only, with each part's texture path noted in a
comment.

### Textures

```sh
# Decode every texture in an archive and report formats and problems
nvinspect "<Data folder>/Fallout - Textures.bsa" check-textures

# Describe one texture, or convert it to .png
nvinspect "<Data folder>/Fallout - Textures.bsa" dds <path from files>
nvinspect "<Data folder>/Fallout - Textures.bsa" png <path from files>

# The same for a loose file
nvinspect some_texture.dds
nvinspect some_texture.dds png
```

Cube maps are written with their six faces side by side.

### Finding files

```sh
# Where the game loads a file from, and which copies it overrides
nvinspect "<Data folder>" find textures\weapons\1handpistol\10mmpistol.dds

# Check that every model named by a record and every texture named by a
# mesh can actually be found
nvinspect "<Data folder>" check-assets

# One model, wherever the game loads it from: how each of its meshes is
# drawn (shader, blending, material) and whether each texture is found
nvinspect "<Data folder>" nif clutter\museum\tornpaintingsm01.nif
```

`check-assets` also shows which ini file the archive list came from, the
archives the game loads in order, and any in the folder it doesn't load. A
missing file that exists in one of those unloaded archives is marked as such,
and for objects that only appear by being placed in the world (statics,
doors, furniture and the like) it says how many times they're placed, which
separates real gaps from unused leftovers.
Only meshes that records actually use are checked for textures, since the
archives also carry leftover meshes (some from Fallout 3) whose textures were
never shipped; `--all-meshes` checks everything. `--ini FILE` reads the
archive list from a specific file, and `--limit N` changes how many missing
files are listed (20 by default).

### Cells

```sh
# Every interior cell, with how many objects are placed in it
nvinspect "<Data folder>" cells --grep mitchell

# Draw one: a floor plan and the view from where the player arrives
nvinspect "<Data folder>" render-cell GSDocMitchellHouse

# Walk the player through its collision from where they arrive
nvinspect "<Data folder>" walk GSDocMitchellHouse
```

`walk` runs the player's capsule forward, right, back and left from the
arrival point for 2 seconds each (or the seconds you give after the cell)
at running speed, and says how far it got each way and where it stepped up
or dropped down. It's a quick check of the collision without opening the
viewer. Given an outdoor cell (`walk Goodsprings`), it walks on the
terrain of that cell and the eight around it, from the middle.

```sh
# Every worldspace, and one square of one: its terrain, the textures
# painted on each quarter, and the weather that lights it
nvinspect "<Data folder>" worlds
nvinspect "<Data folder>" land WastelandNV -18 0
```

`render-cell` accepts an editor ID, a form ID or the cell's name, and writes
two images next to where you run it (or to the prefix you give as a second
argument):

- **`<name>_plan.png`**: the cell from straight above with everything more
  than 160 units above the main floor cut away. North is up. White lines are
  where walls and objects cross the cut; yellow marks arrival points (with a
  line showing which way the player faces), cyan the camera of the view
  image, orange the lights, pink the people. The bar at the bottom left is
  256 units, about 3.7 m.
- **`<name>_view.png`**: what you'd see standing where the player arrives,
  eye height 120 units, 75° field of view.
- **`<name>_objects.txt`**: every placed object with its model, position,
  rotation, scale, enable parent and the box it occupies, including where
  it lands on the plan image in pixels, any texture that couldn't be found,
  then the markers, people, and each reference that was left out and why. Handy for working out what a
  shape in the picture is.

Light effects (sunbeams through windows, lamp glows, haze) are left out of
the plan, since from above they'd cover what's under them. The view draws
them faintly, using each effect's opacity and how it fades with viewing
angle.

It also prints the cell's lighting, what was left out and why, any missing
models or textures (drawn grey), the floor heights it found, the arrival
points, and the objects tilted on more than one axis. Options move the
camera (`--arrival N`, `--from X,Y,Z`, `--heading`, `--pitch`, `--fov`),
change the cut (`--cut Z`), the lighting (`--brightness`, `--fullbright`) and
the image size (`--size 1920x1080`, `--plan-size`). People and creatures
aren't drawn yet, and outdoor cells render without terrain.

### Scripts

The `script` crate reads the game's scripting language, and `world::
scripting` runs it. Every script record, and every dialogue line's and quest
stage's result script, keeps its source text next to the compiled form the
game actually runs; the parser reads the source and was checked against the
compiled form of every script in the game.

```sh
# Parse every script source, and compare with the game's compiled form
nvinspect "<Data folder>" scripts

# One record's scripts, numbered, with the compiled form statement by
# statement
nvinspect "<Data folder>" source VCG01

# Every script line mentioning something, with its compiled form
nvinspect "<Data folder>" scripts --grep "setstage vcg01"

# The scripted objects in a cell, their blocks and trigger boxes
nvinspect "<Data folder>" scripted GSDocMitchellHouse

# A new game's quest scripts run for 40 game seconds, after setting a
# stage: what happened, and the functions they needed that aren't done yet
nvinspect "<Data folder>" play 40 VCG01 0
```

All 10,684 script sources parse; all 35,485 conditions and `set`
expressions come out in the same order as the game's compiled form; and
10,420 of the 10,455 scripts that have both match it statement by
statement, with every function given the same number of arguments. The
other 35 are scripts with a stray `endif` or `elseif`, which the game's
compiler accepts. Things learned from the compiled form along the way:

- `||` binds tighter than `&&` (`a && b || c` is `a && (b || c)`);
  arithmetic before comparisons, as usual; unary minus is stored as `~`.
- A function given too many arguments (`GetStage VMQYesMan01a 110 != 1`)
  compiles the extra one as a value of its own.
- `if` and `endif` pair up by counting, and every `elseif` / `else`
  belongs to the innermost open `if`, however it's indented.
- Lines that are only dashes or equals signs, and anything after `else`
  on its line, are ignored.
- Actor value numbers: Strength 5 to Luck 11, Health 16, Science 40 and
  so on.

The runtime keeps a game's state (quest stages, objectives, script
variables, globals, items given and taken, what's enabled, owners, door
states, form lists and more) and carries out 228 of the 390 functions the
game's scripts and dialogue conditions use, each the way the game's own
handler does (`nvinspect "<Data folder>" functions` lists the rest, most
used first). When a script needs the value of a function that isn't done
yet, it stops there for that run instead of carrying on with a made-up
value, so quests never advance on a guess; `play` lists those functions by
how often they were needed.

`nvinspect "<Data folder>" functions` counts every function the game's
scripts and conditions use and lists those not carried out yet, most used
first.

### Menus

```powershell
# The HUD as the game sets it up: every tile's place, size, colour and
# text, and what's drawn (texture, rectangle, coordinates, colour)
nvinspect "<Data folder>" menu hud
# Any menu file the same way
nvinspect "<Data folder>" menu message_menu.xml
# A font: line height, pictures, every glyph's size and kerning
nvinspect "<Data folder>" font 7
```

### Collision

```powershell
# Every model's collision, surveyed
nvinspect "<Data folder>" collision
```

`collision` reads every model the game can see and counts its Havok blocks
(and says how nv-rs handles each), the layers bodies sit on and which stop
people, how bodies move, the arrangements that matter (collision below the
top node, scaled pieces, offsets), models whose collision reaches far past
what they draw, and the references placed with a primitive (collision
markers, triggers, sound volumes) by base, shape and layer. `walk` now also
says what's solid in the place: doors that open, collision markers, people,
moving clutter, and the biggest things drawn without collision.
### V.A.T.S.

```sh
# What V.A.T.S. offers a new character against someone with a weapon:
# action points, the shot's cost, and each part's chance at some distances
nvinspect "<Data folder>" vats DocMitchell WeapNV9mmPistol 500
```

```sh
# The camera paths an attack tries, in order, with their shots
nvinspect "<Data folder>" vats-cameras
# The V.A.T.S. menu worked out for a sample target
nvinspect "<Data folder>" menu vats
# A camera shot's model: where it moves, how it turns, its field of view
nvinspect "<Data folder>" nif vatscameras\simplefronthit01.nif
```

### Music

```sh
# What plays at a place: its audio markers, the controller and sets, the
# region music, and the music manager run for a while (by night, with a
# fight from 10 s to 25 s, a script's PlayMusic first, mixed to a .wav)
nvinspect "<Data folder>" music GSDocMitchellHouse
nvinspect "<Data folder>" music WastelandNV -18 0 hour 23
nvinspect "<Data folder>" music Goodsprings seconds 40 combat 10 25 wav fight.wav
nvinspect "<Data folder>" music WastelandNV -18 0 play musSCRGoodspringsStinger
```

### Coverage

- `nvinspect <Data> coverage records|files|functions`: what the game has against what nv-rs
  covers, as Markdown. `records` counts every record type and subrecord in each plugin and
  marks which nv-rs reads; `files` reads every archive and loose file and counts file kinds,
  model block types (and which nv-rs decodes), texture and sound formats and menu files;
  `functions` lists the script functions with how often the game uses each and whether nv-rs
  carries it out.

### How placement works

Every placed object stores a position and three angles. The heading turns
clockwise seen from above (0 is north, 90 east). The order the engine
combines the three angles in isn't documented, but it matters for objects
tilted on more than one axis, and the game's own data settles it: the
bathroom mirror in Doc Mitchell's house is stored at 270°, 270°, 0°, and only
the X·Y·Z order puts it flush on the wall (the other would leave it floating
flat in mid-air). `--variants` still renders both orders side by side.

The engine also replaces the transform stored on a model's top node with the
placement's own, so a model whose file carries a rotation there looks turned
in a model viewer but not in the game. `render-cell` places models the way
the game does, lists the models this affects, and `--keep-root-transforms`
shows them the viewer's way instead. (In Doc Mitchell's house, the
entrance-room piece is one: applying its top-node rotation put a wall across
the hallway.)

### The game's shaders

The lighting and color formulas live in the game's compiled shaders, not
in its data files. `nvinspect` reads the shader packages in
`Data\Shaders` and turns each Direct3D 9 shader into readable
instructions, labelled with the names the game gives its inputs
(`AmbientColor`, `LightData`, ...), which is what the viewer's lighting is
ported from. Which package the game uses is recorded under
`Shader Package` in `Documents\My Games\FalloutNV\RendererInfo.txt`. `dump`
writes into the current folder unless given another:

```sh
nvinspect "<Data folder>\Shaders\shaderpackage019.sdp"                 # list them
nvinspect "<Data folder>\Shaders\shaderpackage019.sdp" shader SLS2001.pso
nvinspect "<Data folder>\Shaders\shaderpackage019.sdp" dump            # all, as text files
```

Run `nvinspect --help` for everything.

## The real-time viewer

`viewer/` opens a window inside a cell that you can walk or fly around. It
draws the same scene as `render-cell`, with the game's full-size textures
(sent to the graphics card still compressed, with all their mip levels),
starting where the player arrives.

You start on foot: the player is a capsule the size the game's own
character controller gives people (radius 20.25, 128 tall, eyes at 120)
that walks into walls and furniture, climbs steps up to 31 units and
slopes up to 47°, falls under the game's gravity (686.6 units a second
squared) and jumps 64 units, using the game's movement settings (walking
speed 77 units a second, running four times that, sneaking 0.57 times).
Falls of more than 600 units hurt as the game's do (0.025 × (fall −
600)^1.65: a 1000-unit drop takes 491). Look at a
load door within reach and press E to go through it, into the next
interior or out into the world. F switches between walking and flying.

Everything the game makes solid is: the models' collision on the layers the
game's own collision filter lets a character run into, the invisible boxes
and planes level designers placed with collision markers, and the people
around you (living ones; you can't stand on them). Clutter stays where it
was placed: you bump into it but can't push it yet.

Doors and gates that aren't load doors swing open and shut the way the
game's own code does it (read from `FalloutNV.exe`,
`nvinspect "<Data folder>" doors <CELL|WORLD X Y|all>` prints what each
door has): E on one plays its model's `Open` or `Close` animation from the
start (Goodsprings' picket gates take a second), with the door record's
opening or closing sound, and while it's still swinging another press does
nothing. A door placed open starts open; scripts' `SetOpenState` and
`Activate` work the same way (a script's `SetOpenState` also clears a lock,
as in the game), `Lock` shuts an open door at once, `GetOpenState` answers
1 open, 2 opening, 3 shut, 4 closing, and a door's `OnOpen` / `OnClose`
script blocks run when its swing ends. The leaf's collision is where the
leaf is: with the game's shipped setting (`bAnimateDoorPhysics=0`) it
jumps to the new place when the swing ends; with the setting on it follows
the swing. People walking a path through a closed door open it when they
reach it and wait for it to open, as the game's path following does.
Sounds named in the animation's text keys play too, as the game plays
them.

Outdoors (`nv-viewer "<Data folder>" Goodsprings`, a worldspace with
`--at`, or through a door), the cells around the player load in the
background as they walk: the player's and two more on every side, as the
game's `uGridsToLoad` setting says. The terrain is drawn the way the
game's landscape shaders draw it, blending up to seven textures by how
much each is painted at each point, with their normal maps and the
terrain's vertex colours. The light comes from the cell's weather and
follows the game's clock: its sunrise, day, sunset and night colours
blend as the hours pass (the climate says when the sun rises and sets),
the sun moves across the sky, and each time of day's image space
modifier plays over the picture (the day's yellow Mojave tint, the
night's dark blue). T waits 1 to 24 hours (not with enemies about: the
game's own "You cannot wait when enemies are nearby."), and
`--run "set GameHour to 22"` starts at night. The
sky is the game's sky dome coloured by the weather, with the climate's
sun and the weather's cloud layers drifting over it (the game's cloud
model and textures, drawn as its cloud shaders draw them), and past the
loaded cells the
game's distant land covers the whole map at the game's own levels of
detail: chunks of 4 cells near the player, 8, 16 and 32 farther out,
chosen and swapped by the game's own rules (read from its program), each
new piece fading in from the coarser one's texture over a second as the
game does, sunk under the loaded cells as the game does. At night the
climate's stars come out. Far-off buildings, rocks and trees come from the
game's own merged distant-object blocks, out to the game's distances:
everything within about a dozen cells, and past that, out to about 30
cells, only the few tall landmarks the game keeps a separate model for;
each block's part over a fully loaded cell hidden so
the real objects stand there.

**Trees and shrubs.** The game's trees aren't models: each is a small
SpeedTree recipe (`.spt`) that the game grows into branches and leaves when
it loads, from a seed. nv-rs reads the recipe and grows it the same way —
the Goodsprings shrub comes out vertex for vertex as the game's own
(checked against a recording of the game's graphics calls). Each tree shows
the detail level the game picks for its distance (the leaves fading between
levels, everything gone past about 6,000 units, as in the game), sways in
the weather's wind with the leaves rocking and rustling, and is lit like the
rest of the outdoors. `nvinspect <Data> trees WastelandShrub01` shows a tree
grown and where its levels change; `nvinspect <Data> trees WastelandNV -19 1`
lists the trees on a square.

Grass grows where the game grows it: each land texture lists its grasses,
and the game's own rules (read from its code) scatter them over the
player's cell and the eight around it, wherever those textures are
painted, within each grass's slope and water limits, each blade sized,
tinted and turned to the slope as the game does. It's drawn with the
game's grass shader: lit by the sun and the image space's grass dimmer,
swaying in the weather's wind, fading out from 7000 units (the "Grass
Fade" slider's value in `FalloutPrefs.ini`). Where each blade lands is
random in the game, so the patches match, not each blade.
`nvinspect "<Data folder>" grass WastelandNV -18 0` shows the grass the
game grows on one square.

Water is drawn with the game's own water shaders: each square's water and
placed pools and troughs, lakes and the river, inside and out. Ripples
come from the water type's noise texture scrolled three ways, the water
bends what's under it, darkens with depth (Lake Mead's shallows show the
lake bed, its middle doesn't), reflects the sky (and, on the Colorado
below the dam and inside, everything around it) through a mirrored second
camera, and catches the sun. Past the loaded cells the game's distant
water covers the lake. `nvinspect "<Data folder>" water WastelandNV` lists
where water stands (`water WastelandNV 13 9` one square, with a map;
`water NVCleanWater` a water type's values). Still missing outdoors: the
view from under water.

Lit surfaces are lit the way the game lights them, not with Bevy's
physically based lighting. The formula is ported from the game's own
lit-surface shaders (`SLS2029` and its relatives in the shader package the
game picks on this PC, `shaderpackage013.sdp`): the cell's ambient light,
the glow color times the glow map, the directional light and every point
light are added up, each light times how directly the surface faces it and
each point light also times `1 - (distance / radius)^2`; the sum multiplies
the texture and vertex colors. All of it works on colors as stored, never
converted to linear light, and the back of a two-sided surface is lit only
by what's in front of it. `render-cell` uses the same formula. Each placed
light reaches its base light's radius plus the placed light's own Radius,
and its colour is the base light's times the colour of the light its
Emittance setting names, if any. Both were read from the values the game
sends its shaders (recorded with apitrace). Doc Mitchell's hallway gets its
orange that way. Vertex colours are used whenever a mesh has them, and a
cell's directional light shines along its two angles.

```sh
cd viewer
cargo run --release -- "<Data folder>" GSDocMitchellHouse
```

The first build downloads and compiles Bevy, which takes several minutes;
later builds are quick. Controls: hold the right mouse button and move to
look, the left one to attack (R draws your weapon, puts fists and melee
weapons away, and reloads a gun; see "Fighting"), W A S D to move (running; hold
Shift to walk, Ctrl or C to sneak), Space to jump, E to use things and open
load doors, F to fly instead (then either mouse button looks, Space and Ctrl
go up and down, Shift faster, the mouse wheel changes speed), `[` and `]`
for darker and brighter, G to switch the image space's colour step off and
on, Home to go back to the start, Esc to quit. `--official`, `--plugins`, `--ini`
and `--keep-root-transforms` work as in `nvinspect`, and `--brightness F`
scales every light. To compare with the game, stand somewhere in game,
note what the console gives for `player.getpos x`/`y`/`z` and
`player.getangle z`/`x`, and pass them as `--at X,Y,Z,HEADING,PITCH`;
`--screenshot FILE` saves that view at 1920x1080 and quits. The field of
view matches the game's: its 75° setting is the width of a 4:3 picture,
and wider screens keep that height (91° across at 16:9).
`nv-viewer --help` lists everything.

The game's HUD is drawn over the picture from the game's own menu file,
fonts and textures, laid out and updated as its code does: health and
action points with their meters, the compass turning with you (map
markers and nearby people on it, red when hostile), the crosshair, the
weapon's ammunition and condition, the message corner (top left) for
discoveries and notices, and the XP meter with "LEVEL UP". It is drawn
the way the game draws it, over the finished picture's stored values, so
it lines up pixel for pixel with a recorded frame of the game.
`--no-hud` leaves it out.

Self-lit surfaces (lamps that are on, glowing windows, screens) light
themselves the way the game does: their glow color is added to the light
falling on their texture, masked by their glow map when they have one.
Parts marked for *external emittance* take that color from the placed
object's Emittance setting: a light's colour, or a region's, which is its
weather's sunlight at that hour and changes through the day as the
game's does. When it names nothing, the game uses the weather region the
player is in (the wall lamps in Doc Mitchell's house). Starting indoors,
the viewer acts as if you had just come in from outside through the
place's door; `--weather-region` sets that region instead, as a saved game
would (`--weather-region VMapGoodspringsRegion --run "set GameHour to
15.72"` gives the lamps the glow the game showed in a recording). Glass and other surfaces that both cut out
and blend their transparency keep their soft edges and stay see-through.

Bright surfaces bloom the way the game's HDR passes do it (ported from
its shaders and a recording of them): the picture is shrunk 4× four times,
whatever is brighter than the image space's bright clamp (0.9 in Doc
Mitchell's house) is scaled by its bright scale (2.4), blurred 6 pixels
each way at a quarter size, and added at half strength. Then the cell's
*image space* is applied to the finished picture: the saturation, tint,
brightness and contrast that give New Vegas its warm, washed-out look. G
switches that colour step off and on (bloom stays). The viewer prints the
values it read (`image space ...`); `render-cell` prints them too but
doesn't apply them.

Shiny surfaces (pots, picture frames, window glass, light fittings)
reflect a cube map the way the game's reflection pass does: the model's
own, or the game's default (`textures\effects\reflection.dds` copied onto
all six faces), masked and scaled as the model says. Glowing surfaces are
multiplied by the image space's emissive multiplier (3 in the Mojave
Outpost barracks, which is what makes its tube lights so bright).

People and creatures stand where the game places them, put together from
their records and posed by the first frame of their idle animation: the
body skinned to the skeleton, clothes over it, the race's skin and head
textures, eyes, hair in the NPC's hair colour, beards and eyebrows hung
from the head. In the viewer they play their idle animation (skinned on
the graphics card), and they follow their AI packages the way the game's
code does: the first package whose conditions and schedule fit (or one a
script gave them) sends them somewhere, and they walk there along the
cell's navmesh (outdoors the joined navmeshes of the squares around you)
at the game's walking speed (77 units a second for people), the walk
animation played at the rate that matches it. They turn at the game's own
rates (people 135° a second on the spot, faster in a fight; creatures by
their record), playing the game's turning animation, turn first before
setting off, and round corners smoothly, slowing on sharp turns. They
look at their packages again every 20 seconds and whenever the game hour
changes, step aside or wait for each other and for you, and at a heading
marker they turn to face the way the level designer set. Someone whose
package is to talk to you (Sunny Smiles in the Prospector Saloon) walks
up and starts the conversation; when you talk to someone they stop and
turn to face you. A script's "start a conversation" makes the person come
up to you first. People notice you nearby and greet you ("Howdy."),
mutter idle remarks now and then, and strike up conversations with each
other, their lines worked out from the game's dialogue, spoken with their
voices.
People who use a chair walk to the marker its record allows, turn, and sit
down with the game's own entry animation (which carries them into the seat),
playing the seated loop and, now and then, the seated idles the game's idle
tree gives them (relaxing, eating); getting up plays the exit (Doc Mitchell
has his own). Standing people ask the same tree once a second for idles.
People on sandbox packages choose what to do the game's way — a chair, a
bed at night, an idle marker (Chet sweeps his store), a wander, a meal at
mealtimes — for as long as the game would; they also eat — picking food
up, finding a chair and eating it — and wander around their area. People
whose package sends them to another place walk to the load door that leads
there and go through it, coming out where the game puts the player on the
far side; people out of sight carry on at walking pace in the game's steps
(every 18 game minutes nearby, every hour farther away), through doors,
instead of jumping to where they're going, and whoever arrives where you
are appears (Sunny Smiles leaves the saloon for her tutorial and walks
round to the back, whether you leave before or after her). Followers keep
near whom they follow (her dog Cheyenne trails her). Going somewhere,
sitting, following, sandboxing, talking and fighting (see below) are done
so far (one door at a time). `nvinspect "<Data folder>" sit
DocMitchellChairREF` shows how a chair is used; `nvinspect "<Data folder>"
idles chair` prints the idle tree's chair branches with each idle's loops
and replay delay; `nvinspect "<Data folder>" ai GSChetRef` shows a
sandbox's area and what it finds.
`nvinspect "<Data folder>" ai DocMitchellREF VCG01 55` shows a person's
packages, the one they'd follow and the path. Faces take each NPC's own FaceGen shape
(its face controls applied through the game's `.egm` morphs) and, where
the game ships one, their own skin tint (`textures\characters\facemods\`),
laid on the way the game's skin shaders do it.

Faces move as in the game: everyone blinks every 1.5 to 4 seconds, and a
spoken line with the game's lip sync file (the `.lip` beside each voice
file) moves the speaker's lips, teeth, tongue, brows and eyelids frame by
frame, the voice starting a moment later (the line's lead-in plus the
game's `fSpeechDelay`) so the two line up. A line without a lip sync file
leaves the mouth still, as in the game. `nvinspect "<Data folder>" face
DocMitchell` lists the morphs each head part has and which of them the
game actually uses (every head's `Ee`, for one, is named so the game never
finds it); `nvinspect "<Data folder>" lip <voice file>` shows a line's
frames and when its voice starts.

Look at someone within reach and press E to talk: it opens the game's own
dialogue menu, drawn from its menu file: the speaker's name and line at the
top (the first greeting the game's conditions allow, in their recorded
voice, with lip sync), then the topics you can choose, the ones already
asked shown dimmer; skill checks are tagged as the game tags them
("[Speech 25]", "[Speech 12/25]"). Click a topic (or use the arrow keys and
Enter); a click on the line skips it. After an answer with no
follow-ups, the person's main list comes back (their greeting's choices
and every top-level topic they answer, highest priority first: Sunny
Smiles offers "Doc Mitchell said you could teach me to survive in the
desert.", the road to Primm, work, "What do you do around here?", the
areas around Goodsprings and "Goodbye.", as in the game). `--talk` starts
talking to the nearest person once the place has loaded.
`nvinspect "<Data folder>" dialogue SunnyREF` prints the same menu.

Message boxes from scripts ("Mister" / "Ma'am") are the game's message
box: its sizing, its buttons, the keys marked in the buttons' text.

The game's scripts run (see [Scripts](#scripts)): quest scripts as time
passes, each dialogue line's result scripts, scripted objects in the cell
(their `GameMode` blocks every frame, trigger boxes as the player walks
through them, and E on a machine or switch runs its `OnActivate`). What
they do shows up: messages, journal entries and objectives as notices at
the top right, people saying lines (`SayTo`) or starting conversations,
objects appearing and disappearing, the player moved elsewhere
(`MoveTo`), doors that won't open until a script allows it. Places load
the way scripts left them.

Sound: doors make their opening sounds, items their pick-up sounds,
scripts' sounds play, and an
interior plays its ambient loop (its acoustic space's, for the time of
day: Doc Mitchell's house hums with Goodsprings' interior loop). The
game's WAV files are decoded by this project's own code and OGG files
by Bevy; a record naming a `.wav` that the game ships as `.ogg` plays
the `.ogg`, as in the game. Not yet: sounds placed in the world (they
play at full volume wherever they are) and footsteps. The `mp3` crate
decodes the game's MP3s: all 199 tracks match Windows' own decoder to
within one step of 16-bit output.
`nvinspect "<Data folder>/Music" check` decodes every track;
`nvinspect "<Data folder>/Music/SCR/mus_SCR_DocMitchell.mp3" wav doc.wav 10`
saves a track's first 10 seconds as a `.wav`.

Music plays as the game chooses it: the audio marker nearest the player
names a controller, which picks a set; a location set's layers change
with how near the marker you stand (cross-fading, the new layer taking up
where the old one was), dungeon sets go from exploring to suspense to
battle, and a fight switches to a battle set with its intro and outro.
Away from every marker, the region plays its short phrases. Scripts'
`PlayMusic` (the stinger as you first leave Doc Mitchell's) holds the rest
until it ends. The MP3s are decoded by this project's own decoder on a
thread of their own, at the game's volumes (`fDefaultMusicVolume`). Doc
Mitchell's house plays the Corporate Ruins "3high" layer; Goodsprings by
day the Desert Exploration "3high". The viewer prints what starts.

I (or Tab, when not talking) opens your own screen, in the Pip-Boy's
spirit: a stats page (level, health, action points, carry weight, caps,
S.P.E.C.I.A.L., skills and tags) and your weapons, apparel, aid, misc
items and ammunition with value and weight; Enter equips a weapon (one
in hand) or clothes (taking off whatever is on the same body slots),
takes them off again, or uses an aid item, whose effects work as their
records say: a Stimpak's 30 health at once (outside hardcore mode),
Nuka-Cola's 2 health a second for 25 seconds and the bottle cap its
script effect hands you, Buffout's extra Strength, Endurance and health
for four minutes, the radiation in dirty water (less with Rad
Resistance) and RadAway taking it away. The stats page lists your rads
and what's working on you. Scripts cast spells the same way (a
concussion, abilities), and doctors cure radiation with the game's own
dialogue scripts.

E on a computer terminal shows its screen as the game writes it: the
header, its menu items (those whose conditions pass), sub-menus, notes to
read, and the result scripts they run (Goodsprings' schoolhouse terminal
unlocks its safe). Locked doors and containers open with their key. Without it, if your Lockpick skill
reaches the lock's bracket (0, 25, 50, 75 or 100), E opens the game's own lockpicking
screen: its menu file, its lock, screwdriver and bobby pin models lit by the lights
in the lock's model, seen through the game's camera. Move the mouse to place the pin,
hold W, A, S or D (the game's movement controls) to turn the lock; it turns further
the nearer you are to a hidden sweet spot (wider for easy locks and higher skill), and
straining short of the full turn wears the pin until it breaks and costs a bobby pin.
F forces the lock (the chance is shown; failing breaks it for good, and then only the
key opens it), E leaves. Every number, sound and timing is the game's, read from its
code. A picked lock gives experience the first time and counts as stealing if it's
someone else's. Too little skill gets the game's "You need a lockpick skill of 50 to
pick this lock."; locks that need a key say so. Locked terminals open through the
game's hacking screen (see `docs/HACKING.md`).

`nvinspect <Data> lockpick <door or container, or a lock level> [skill]` prints what
a lock does: the sweet spot and its rings at your screen size, how long a pin lasts,
the chance to force it, and the menu's models, lights and camera. The viewer's
`--lockpick REF` opens a lock's menu once loaded (for testing).

Skill checks in conversations are tagged the way the game's dialogue
menu does it: "[Speech 25]  You should help me take down the powder
gang." when your Speech is high enough, "[Speech 12/25]  ..." when it
isn't (and the person answers accordingly, as the game's lines are
written).

Experience and levels follow the game's code: each level takes 25 × (L −
1) × (3L + 2) experience (200, 550, 1050 …, up to level 30); quest
scripts, found places (10), first picks of locks and hacked terminals (20
to 60 by difficulty) and kills give it (by the victim's level from the
game's tables, when you and your companions did more than 40% of the
damage). Out of combat levelling up opens the game's level-up screen:
"WELCOME TO LEVEL 2", the skills with the arrows to give the level's
points (10 + half your Intelligence, as the game counts them; tag skills
as the game counts them), the pointer over a skill showing its picture
and description; on even levels Continue leads to the perks, the ones you
can take first and bright, the rest dimmed, each with its picture and
requirements ("Req: Level 2, Guns 30, Agility 5"). Points given count at
once, so a perk unlocked by them becomes available on the same level, as
in the game. `--open-menu levelup` raises the player a level for testing.
Swift Learner and Educated work through the perks' own entry points, and
health grows by 5 a level.
`nvinspect "<Data folder>" levels 4` prints the table, the kill rewards
and the perks offered at level 4.

Placed models are now drawn the way the game sets up its graphics card,
read from recordings of the game: see-through pieces that the game marks
as solid in depth (the ceiling fan's blades, windows, glass) hide what's
behind them instead of letting it show through; solid pieces always do,
whatever their file says (the big wooden water crates by the Prospector
Saloon were see-through: their file leaves the "write depth" mark off,
but the game's recording shows it writing depth all the same); decals
(shadows under picture frames, grime) sit just in front of their surface
without flickering; see-through pieces are drawn back to front piece by
piece; cut-outs use the game's exact alpha test. Textures are filtered as
the game's settings say (`iMaxAnisotropy` in your `FalloutPrefs.ini`, 15
here), which is what the game's recording sets on every texture. Models play their own
animations the way the game starts them: the ceiling fan turns (one turn
a second), Goodsprings' windmill spins, footlockers show closed and
mailboxes keep their flags down, and glow cards turn to face you. Scripts
can play a model's animation (`PlayGroup`): Goodsprings' house windows
glow only at night, as in the game. `nvinspect <Data> meshes <CELL>`
lists every piece of every model in a place and how the game draws it.

Perks now work the way the game applies them: each perk entry is checked
against what it's about — the weapon you hold (Cowboy's revolvers and
lever-actions, Piercing Strike's blades and fists), who you're hitting
(Lady Killer, Entomologist, Hunter) or who's hitting you (Stonewall, Hit
the Deck) — and changes the game's numbers where the game changes them:
damage after armour (Cowboy, Bloody Mess, the challenge perks), critical
chance and damage (Laser Commander, Better Criticals, Ninja; Dream Crusher
on whoever shoots at you), damage threshold (Toughness, Piercing Strike,
Shotgun Surgeon), limb damage (Adamantium Skeleton), what food and
medicine do (Better Healing, Lead Belly, Chemist, Day Tripper — and, as
the game does it, your Medicine and Survival skills scale what stimpaks
and food restore), noticing (Silent Running), movement (Travel Light),
reloading (Rapid Reload, and Agility: reloads take longer below 5 and
less above), attack speed (Fast Shot, Slayer; attack animations now play
at each weapon's own rate), carrying (Pack Rat; fast travel is refused
while over-encumbered unless you have Long Haul), cases and cells coming
back from shots (Hand Loader, Vigilant Recycler), and weapon and armour
wear (armour wears as its damage threshold soaks up hits, and gives less
DT below half condition; "Your armor condition is dangerously low." below
25%) (Built to Destroy; weapons now wear a little with every attack, as in the
game). `nvinspect "<Data folder>" perks` lists every perk's entries and
conditions. Perks whose mechanic isn't here yet (mines, terminal lockouts,
addiction, Mister Sandman and Cannibal, Meltdown, repairing, knockdowns,
throwing, unarmed specials) do nothing yet.

**Particles.** Dust, smoke, sparks, steam and the like in placed models now
move and draw as the game's code runs them: each system's emitter, the
forces on the particles (gravity, drag, bombs), colours and fading,
spinning, bouncing off colliders and spawning, worked out from the game's
own program and drawn with its own particle shaders. The dust in the
Mojave Outpost barracks settles at the same 59 particles per cloud as in
a recording of the game, with the same sizes and colours. Not yet: the dust
whirlwinds outside Goodsprings (they travel along a path that isn't read
yet, so they're left out rather than shown standing still), grenade
trails, and the particles of explosions, impacts and creatures.

T opens the game's own wait menu and E on a bed its sleep menu (sleep_wait_menu.xml): an hour passes each second, a sleep goes over a black screen, and the game autosaves as its settings say.

Walking near a place on the world map finds it ("You have discovered
Goodsprings", within the distance its map marker gives, measured as the
game does). The Pip-Boy (I) has a Map tab listing the places found (and
ones scripts reveal); Enter on one you've been to fast travels there, to
the spot the game uses, and the trip's time passes (the distance at
running speed, as game time). As in the game you can't fast travel or
wait with enemies near, or fast travel out of most interiors.

Hardcore mode works as in the game when a script turns it on (the
game's own question at the end of Doc Mitchell's intro, or
`--run "SetHardcore 1"`): every 10 real seconds of game time you grow a
point thirstier, every 25 hungrier and every 50 more tired (not while
scripts have your Pip-Boy off), and at 200, 400, 600, 800 and 1000 points
the game's sicknesses set in ("Your dehydration level has increased." /
"You are now sick with Minor Dehydration"), lowering your S.P.E.C.I.A.L.;
at 1000 you die. Drinking and eating bring them down; turning hardcore off
clears them. Radiation sickness works the same way with the game's five
stages.

Beds answer as the game's do: "You cannot sleep in an owned bed.", not
while trespassing, with enemies near or while something is hurting you.
The sleep and wait menus themselves aren't drawn yet, but their rules are
in place: an hour a real second, healing as you wait, a full heal after a
sleep (outside hardcore), sleep lowering hardcore's tiredness, and the
safehouse beds making you Well Rested. T refuses to wait the same way.

Sneaking up to someone and pressing E is a pickpocket attempt (the menu
isn't drawn yet): the chances are the game's (40 + 0.6 × your Sneak − 0.6
× theirs − half the value, between 5 and 85%), each try costs karma unless
they're evil, and being caught is a crime they remember.

Walk into someone's home uninvited and, once they see you, they come over:
"You need to leave." every ten seconds, four times, then they and their
friends attack. `nvinspect "<Data folder>" living NovacBoonesRoom` shows
what a new character meets in a place: whether it's trespassing, who sees
you, what each bed says, and every pocket's chances.

E also takes an item lying around (it goes into your inventory and
disappears from the world) and sits you on furniture. E on a container (or
a body) opens the game's container screen: your things on the left, the
container's on the right, the arrows (or the wheel over a title) filtering
by kind; click a line to move it across (more than five asks "How many?"
with the game's slider), Take All (A), Exit (E), as the screen's own hints
say. The weight you carry and can carry is shown as the game shows it;
the item and container sounds play; taking from someone else's container
is stealing, as in the game. `--open-menu container:REF` opens one for
testing.

Merchants trade when their dialogue says so (Trudy's "Show me what you
have for sale.") through the game's barter screen: your things and their
goods (only what their services cover), each priced by the game's own
formula (the item's condition, your Barter skill, perks); clicking a line
offers it, the running total and the caps that will change hands show at
the bottom, Accept settles the trade, Exit with an offer asks "Cancel
transaction?". `nvinspect "<Data folder>" barter TrudyREF` lists a
merchant's goods and prices.

Merchants who repair (Mick in Freeside, Samuel at the 188, Old Lady
Gibson, Raul, Calamity, Major Knight, Dale Barton, Sato) do it through the
game's repair services screen when their dialogue says so: your damaged
weapons and armour with what mending each costs, the condition and damage
(or DT/DR) now and after, Repair All; how far they mend and what they ask
come from their Repair skill by the game's own formulas (see
`docs/REPAIR.md`). `--open-menu repair:REF` opens a vendor's for testing.

Companions trade things with you when their dialogue says so ("Let's
trade equipment."): the game's container screen on their things, without
Take All; they refuse what they've no room for ("Cass can't carry any
more.") and say their trading lines (see `docs/COMPANIONS.md`).
`--open-menu teammate:REF` opens one for testing.
Inventories start
as the records list them, with leveled lists picked for your level; I
shows what you carry (E on a skill book reads it: +3 to its skill, +4
with Comprehension, and the book is used up, as in the game) and J your
objectives. F5 saves the game (quest
stages, script variables, what you carry, what's been taken or
switched, where people and you are) to `nv-rs-quicksave.txt` in the
folder the viewer runs in, and F9 loads it back; it's this project's own
text format, not the game's saves. `--stage QUEST STAGE` sets a quest's stage once the place has
loaded, as a script would. `--new-game` starts the game: it sets the
opening quest (`VCG00`) to its first stage. That stage requests the opening
movie (playback is not implemented), moves you to Doc Mitchell's house and
starts his intro through the quest scripts:

```sh
cargo run --release -- "<Data folder>" --new-game
```

The opening's player packages now drive the actual wakeup, situp, bedsit and
standup camera tracks while holstered. Script/save/report coordinates stay
with the physical player. Package actions can be inspected with
`nvinspect "<Data folder>" ai VCG01PlayerSection1`. The opening remains
incomplete: movie playback, the face menu and restoring active
animations from a save are unfinished. NPC scripts now play unconditional
special-idle KF files on loaded actors, including Doc's mirror gesture;
other idle groups and conditional trees are not connected yet. See
[opening foundations](OPENING.md) for tested scope and remaining work.

Doc Mitchell wakes you ("You're awake. How about that."), and the intro
quest's own scripts carry it on line by line: your name (stage 15), the
"Mister / Ma'am" box (stage 17), the face menu (not drawn yet: it closes at once, as if
accepted), then "walk to the Vit-o-matic Vigor Tester", where Doc walks
over to meet you and the objective completes when you walk into the
tester's trigger box; E on the tester opens the S.P.E.C.I.A.L. menu
(40 points, each 1 to 10). From stage 79 (`--stage VCG01 79`) he
introduces the psych exam, asks you to sit on his couch and walks to his
chair; then come the tag skills (three) and the traits (up to two, from
the game's ten). Scripts lock and unlock the player's controls as in the
game (in bed you can look around but not get up), the screen effects
they apply play (waking up: a white, glowing haze that clears over a
few seconds; blur and double vision aren't drawn), and whoever talks to
you turns to face you.

In Doc Mitchell's intro the name, the tag skills and the traits are the
game's own menus: "Enter character name." with a blinking cursor (OK only
for a name the game would accept), "SKILLS:  2/3 Selected" with the psych
exam's picks already tagged, and "CHOOSE UP TO 2 TRAITS (OR NONE)" with
each trait's picture and description. The vigor tester now uses the original
3D cabinet, animated pages, numbers, bulbs and button geometry through the
game's menu XML. Left/Right turns pages, Up/Down changes the current value,
and A finishes only when all points are assigned (Q for French). Mouse
buttons are picked from the original model triangles. Allocation/closing
has an automated regression and a live PC input check (mouse increase/page
turn, keyboard allocation, rejected early Done and closing at the budget).
The Strength page has a checked render; the complete opening route and
original-game visual comparison remain unverified. Controller input is not connected. The face menu is still
missing. See [Vit-o-matic evidence](VIGOR.md).

People and creatures, checked against a recording of the game in the Prospector Saloon:

- People now look like themselves: faces take their race's shape as well
  as their own (Sunny Smiles was a different woman before), their skin
  tint (face and body) and the skin colour the game multiplies everything
  by; skin is lit the way the game lights it (a soft rim and a red glow
  where light wraps round), hair has its colour layer and its own sheen,
  clothes bring their gloves.
- Creatures' eyes sit in their heads (Cheyenne's had floated at her
  collar).
- The machete looks like the game's: no stray bar down the blade, no
  highlight the game doesn't draw (surfaces whose normal map has no
  highlight mask get none, everywhere).
- `--freeze-ai` keeps people still, like the console's `tai`.
- Still different: the game plays random idles (head tilts, looking
  around); the viewer only the standing one.

People's animations now change the way the game
changes them: a walk fades in over a fifth of a second (the files say how
long), fades out the same way, and runs at the pace that makes the walk
cover the ground the person actually moves (77 units a second for people,
not the animation's own 85); the arms follow the aim or an attack while
the legs keep walking, because each bone takes the animation with the
highest priority. Nobody jumps between poses any more, and people stand
still while you're in a menu or talking to them.

Fighting, first version: the weapon in hand is the one scripts or you
equipped (Sunny's tutorial equips the varmint rifle), else the first one
you carry, else your fists. The left mouse button attacks along your
view, at the weapon's own rate, using its ammunition (any kind it takes,
from a form list like `AmmoList556mm`) a clip at a time (R reloads). A
hit's damage follows the game's settings: the weapon's damage × (0.5 +
0.5 × your skill / 100) × its condition factor, less the target's damage
threshold but never under 20% of the hit. Hit objects run their
`OnHitWith` scripts (the tutorial's bottles count this way), killed
people and creatures run `OnDeath` (its geckos) and go limp as the
game's dead do: the skeleton's own ragdoll (18 capsule-shaped bodies with
the game's masses, friction and joint limits) falls from the pose they
died in, thrown as the game throws the dead (every body by the weapon's
kill impulse, hands and feet less), and comes to rest on whatever is under
it (E on a body searches it, as a container); whoever is
hurt fights back. People and creatures notice you (and each
other) the way the game works it out: every third of a second in a
fight, every few seconds otherwise, by distance, light, noise and whether
they can see you; once they notice someone their nature and faction say
to attack (very aggressive creatures like geckos and the giant rats of
Broc Flower Cave, people whose faction is your enemy, which the game's own
scripts decide: they make the Powder Gangers enemies with `SetEnemy`, and
those changes are kept), they come for you; friends and allies join in,
and the timid run from those much stronger. Gunmen keep to their weapon's
range (a 9mm between 256 and 768 units), stepping sideways every few
seconds, and fire at the game's pace (a varmint rifle about every 0.8 s;
automatic weapons in one-second bursts); fighters with melee weapons,
fists or teeth run in, then pace their attacks as their combat style
says, sometimes waiting a moment between blows. Lose them for 15 seconds
and they search for you; after half a minute to a minute they give up and
go back to what they were doing. `--run "SunnyREF.StartCombat player"`
in the Prospector Saloon shows Sunny backing off to her rifle's range and
Cheyenne joining in.
`nvinspect "<Data folder>" hostile PowderGangerGunCM2` shows how someone
reacts to you. The health line at the bottom shows your health and
ammunition. In first person you see your hands and the weapon as the game
draws them: its first-person skeleton, the race's first-person hands and
the weapon's own model, in the game's hold pose for that kind of weapon,
with its own attack and reload animations, at the game's first-person
field of view (55), drawn after everything else over a cleared depth
buffer as the game draws them (so your hands never sink into a wall you
stand against). As in the game you start with your weapon (or fists)
away and nothing in view: R or an attack draws it, with the game's
drawing animation; R puts fists and melee weapons away again (with the
putting-away animation), and with a gun a tap of R reloads it (the game
puts a gun away when R is held for a while; how long is a setting whose
name hasn't been found in the game's code, so that isn't done yet).
Changing weapons starts with the new one away, as the game does.
`--weapon WeapNVVarmintRifle` starts with a weapon in hand and drawn
(`--weapon none`: fists drawn). People carry their weapons as the game draws them: put away on the
back or at the hip (where the game's holster animation for that kind of
weapon hangs it), held ready with the game's aim pose once they fight,
and swung or fired with the weapon's own attack animation (Sunny Smiles'
varmint rifle, a Powder Ganger's .22 pistol). `nvinspect "<Data folder>"
actor SunnyREF` lists a person's weapon and those animations. `--run
"SunnyREF.StartCombat player"` runs a line of the game's script language
once the place has loaded, as the game's console would (any number of
`--run`s), and `--wait 20` lets the game run 20 seconds before a
`--screenshot`.

Shots land where they hit: on the head, torso, an arm or a leg, found as the
game finds them, from the capsule-shaped bodies the skeleton carries on its
bones (the same ones that go limp when someone dies) and the body part data
the game gives people and creatures (`nvinspect "<Data folder>" hits
DocMitchell` draws what each shot from in front would hit). A shot to a
person's head does double damage; every hit also wears down that part, by its
share of the body's health, and a part worn to nothing is crippled: crippled
legs slow you (to 0.85 with one, 0.75 with both) and people alike, and a
person whose gun arm is crippled drops their weapon. Scripts see all of it
(`GetActorValue LeftMobilityCondition`, `GetHitLocation`). The health line
lists your crippled limbs, and a Stimpak mends each a little (a tenth of what
it heals). A sneak attack's bonus only counts on a hit that lands on a part,
as in the game. Not yet: limbs coming off, aiming spread, bullets in flight,
death animations, VATS, cover, dodging and blocking.

**Hits** now sound like the game's: each weapon's impact data says what a hit on
stone, dirt, wood, metal, glass, flesh and so on plays, and the viewer plays the
right one for what was struck — walls by the material in their collision, people
and creatures by their own material (power armour rings as metal). Bitten by a
rat you hear its bite; shot, people cry out ("Ahh!") as the game decides —
usually, but not more often than every 2–4 seconds — and the dying say their
death line or, for creatures, make their death cry. `nvinspect <Data> impacts
WeapNV9mmPistol` lists what a weapon's hits play on every material;
`nvinspect <Data> impacts GSSunnySmiles` what hitting someone sounds like and
says. The effect puffs, blood sprays, bullet holes and blood on walls and on the
screen aren't drawn yet: the game's own decal and particle systems will be needed
for those.

V.A.T.S. (V): the world stops and the game's own V.A.T.S. menu comes up,
read from its menu file and laid out as its code lays it out: action
points with what an attack costs pulsing, hit points with the compass, the
target's health and name, and a label by each body part with its chance
and condition. Hold V to stay on choosing the target; let go and the view
turns and zooms onto them as the game's camera does, the parts are
scanned and the labels appear. A and D change the target, W and S the
part, the left mouse button queues an attack (a 9mm costs 17 of a new
character's 80; a reload adds 10), B or the right button takes it back,
R and F the special attacks, and E plays the queue (B or E with nothing
queued leaves). Playing, each attack picks the game's camera for it: the
shot fires from your view, and as it lands the camera cuts to the game's
camera beside the target in slow motion (the world at a quarter speed),
for at least a second and a half, then the next attack, and back to your
view at the end. The chances, costs and rolls are the game's (heads are
harder than torsos; misses land just beside the part). While V.A.T.S. is
on you take a quarter less damage and its hits are likelier to be
critical. Action points are spent as each attack plays and come back at
6% a second once V.A.T.S. is over. The camera is kept out of walls as the game keeps it, and a gun that empties its clip reloads itself between shots, the menu charging that reload on the shot that empties it, as the game's does. Not yet: the scan's highlight, the mouse.
`--vats` opens it by itself three seconds after loading and `--vats 3`
also queues and plays three attacks, for testing with `--screenshot`.

The Pip-Boy:

Press Tab to raise the Pip-Boy 3000 on your left arm, as in the game: its
three screens — STATS, ITEMS and DATA — are the game's own menus, filled
from your character, drawn onto the Pip-Boy's screen with the game's
scanlines, glow and flicker. Hold Tab for a moment instead to switch the
Pip-Boy light on or off.

While it's up: the arrow keys move up and down lists and left and right
between pages and tabs; Shift with left or right changes between STATS,
ITEMS and DATA; Enter equips, takes off or uses the chosen item, or makes
the chosen quest the active one; on the Status page Enter uses a Stimpak
(RadAway under RAD). On STATS the letter keys press the buttons that show
them (S Stimpak, E Doctor's Bag, A RadAway, X Rad-X, R reputations on the
General page); R on ITEMS repairs the chosen weapon with another like it
(or one on its repair list) as the game's repair screen does, E going back;
the Mod button doesn't work yet. Tab puts it away.

STATS shows your level, health, action points, experience, limbs, rads and
effects, your S.P.E.C.I.A.L. and skills with their pictures and
descriptions, perks, and the general statistics or reputations. ITEMS lists
weapons, apparel, aid, misc and ammunition the way the game sorts them, with
each item's card (weight, value, condition, strength needed, ammunition, DT
or DR, weight class). DATA shows the date and time, the world map with the
places you've found, your quests with their objectives, notes and radio
stations.

Not there yet: the mouse (so travelling from the map), the local map,
modding, the DPS figure and effect descriptions on item cards,
and the light actually lighting the room.

For screenshots: `--pipboy stats`, `--pipboy items:1` (the tab from 0),
`--pipboy data:2`. `nvinspect <Data> pipboy stats 1` prints a new
character's S.P.E.C.I.A.L. page as the game would build it.

Menus that wait for an answer (message boxes and those character menus)
come up in the middle of the screen, and the game waits while they're
open: number keys pick a button; arrows move and change values, Space
tags a skill or picks a trait, Enter accepts; typing edits the name.
Skills follow your S.P.E.C.I.A.L. and tags (2 + 2 × the attribute + half
your Luck, rounded up, + 15 tagged; the 2 and 15 are the game's own
settings).

The viewer is written against Bevy 0.16 and pinned to it, since Bevy's API
changes between releases. It doesn't draw shadows yet.

## Tests

```sh
cargo test
```

The tests build small plugins and archives byte by byte: nested groups,
compressed records and files, oversized subrecords, load orders with
overrides and renumbering, and truncated or corrupt input. The zlib decoder is
checked against vectors made with Python's `zlib`. The cell tests (and
those for the viewer's loading code in `cellview`) build a room from scratch
(a plugin, meshes and textures in a temporary Data folder), load it and
check the rendered pixels: that a heading turns objects the
right way, that walls cull from behind, that a glow map lights only what it
should, and that missing files are reported.
Nothing from the game is needed to run them.

## Verified against the real game

**Outdoors at Goodsprings, against a recording of the real game.** A
frame of the game in Goodsprings at 13:06 was recorded (every graphics
call) and nv-rs was set up at the same spot and hour. Every value the
game sent its shaders for the sun, the ambient light, the fog, the sky,
the clouds, the terrain and the final colour grading now matches, and
the pictures agree within 1% over the ground, the buildings, the far
hills and the sky (before: the sky 13–21% too bright, the ground 2–3%,
the far hills 7–10%). What changed: the sky and clouds are dimmed by the
image space's "lum ramp" value as the game does; each cloud layer is
drawn on its own dome; the terrain's normal maps use the game's own
frame; the loaded terrain fades into the distant land toward the edge of
the loaded area, as the game draws it; the light is passed unrounded;
and textures are filtered on their stored values, as the game's are
(this also fixed Doc Mitchell's rug, which was 10% too bright). To see
the same picture: `nv-viewer <Data> WastelandNV --at
-72151.5,639.2589,8281.634,0,0 --run "set TimeScale to 0" --run "set
GameHour to 13.09783" --cloud-time 58.022`. The house's windows are invisible by day,
as in the game. The distant land and distant objects the game drew are the
ones nv-rs draws (checked chunk by chunk against the recording), and the
far hills match within 1%. The picture's slow "eye adaptation" (the
brightness the bloom works from catching up about a tenth of the way each
second) is drawn too; it holds still while a menu is open, as in the game.
Still different: the grass's exact placement (random in the game).

- Script functions, second round: 71 more of the game's script functions
  are carried out as the game's own code does them, among them the most
  used ones that weren't: placing new objects and people (`PlaceAtMe`, the
  game's first spot and ring of eight around the caller), ghosts that can't
  be hit, essential people, challenges (counting, the "3\5" notices,
  completion with the challenge's reward script, recurring ones),
  destructible objects breaking in stages, the AI procedure numbers the
  game's idle and script conditions ask about (now read from the game:
  e.g. a sandboxing person eating is 10, talking 4, sitting 48), combat
  styles set by scripts, talking activators and radio stations, menus open,
  and scripted autosaves. Altogether nv-rs now carries out 325 of the
  game's 622 named functions, 311 of the 410 the game's data uses: 98.7% of
  all their uses.
- `FalloutNV.esm` parses completely: all 465,016 records and 77,000 groups,
  matching the file header's own count.
- Weapon stats decode correctly: damage, clip size, value and weight match
  known weapons.
- Load order and archives work on a real install; the archive tested reports
  all 20,665 name hashes matching.
- `render-cell GSDocMitchellHouse`: the floor plan matches the house as
  remembered, furniture and textures included, and the object list's
  tilted objects settle the rotation order (see
  [How placement works](#how-placement-works)).
- Ignoring the transform on a placed model's top node matches the game: in
  Doc Mitchell's house it opens the entrance onto the hallway (the view from
  the front door now looks down it, as in game), sits the kitchen cabinet
  doors flush on their cabinets and hangs a wall clock flat on its wall.
- No-lighting surfaces use their vertex colors even when the mesh's
  vertex-color flag is off: the soft shadow behind picture frames is made of
  nothing else (found with `nif clutter\museum\tornpaintingsm01.nif`).

- `check-nifs` reads 14,880 of the 14,881 meshes in `Fallout - Meshes.bsa`
  (48,996 visible parts, 29 million triangles). The one failure is an
  invisible trigger box saved in an older format.
- Exported meshes look right in a 3D viewer (checked with the 10mm SMG).
- `check-textures` decodes all 22,054 textures in `Fallout - Textures.bsa`
  and `Fallout - Textures2.bsa`, and converted PNGs look right.

- All 48,006 shader blocks in the game's meshes are read at exactly the size
  their type should have. The ~3,100 visible parts without a texture in their
  mesh file are by design: self-lit colored parts (neon, glowing windows),
  distant-terrain meshes whose textures follow a naming scheme, skin and
  body parts textured by race, water, and the sky.
- `check-assets` on an unmodded install, with `Update.bsa` loaded as the
  patch archive: 12,453 of 12,503 models found, and 7,506 of 7,556 textures
  used by those models. 46 of the 50 missing models belong to objects never
  placed in the world (editor markers, test content, cut objects).
- The few real gaps are in the game's own data, and renderers need to
  tolerate them: the Helios One solar reflector model (placed 12 times), a
  boardwalk barricade's normal map, the Yangtze memorial's environment mask,
  Papa Khan's armor ground textures, and a Ranger helmet that names its
  environment map `chromedull_e.nif` instead of `.dds` (`render-cell` tries
  the `.dds` name).

- Outdoors: `WastelandNV` has 16,396 cells; Goodsprings (-18,0) loads with
  its terrain (heights 8112 to 8776, a base texture and six painted layers
  in each quarter), its objects and the persistent ones standing in it,
  lit by its region's weather `NVWastelandGS`, and the player can walk
  over it.

- Scripts: every script source in the game parses, and matches the
  game's compiled form (see [Scripts](#scripts)). Doc Mitchell's intro
  (`VCG01`) runs from its own scripts: his lines come in the game's order,
  each line's result script setting the next stage.

Still to confirm:

- **Exterior block labels** (`GroupKind` in `crates/esm/src/record.rs`):
  assumed to store the Y coordinate before X. Nothing reads them yet (cells
  are found by their own `XCLC` grid position).

## Roadmap

See [docs/MILESTONES.md](MILESTONES.md) for current priorities, acceptance
gates and the next session action. Existing systems need integrated gameplay
verification before their milestones can be marked complete.

## VR

VR is planned as a built-in mode of the engine, not a separate mod: the same
build runs on a flat screen or a headset. Supporting it well means a few rules
have to hold from the first line of rendering code, such as keeping the camera
separate from the player and routing all controls through named actions.
[`docs/VR.md`](VR.md) lists those rules, the New Vegas features that need
VR-specific designs (VATS, dialogue, iron sights) and the VR milestones.

## Legal

This project contains no game data and is not affiliated with or endorsed by
Bethesda Softworks, ZeniMax Media or Obsidian Entertainment. Fallout and
Fallout: New Vegas are their trademarks. The tools read files from an
installation you own and never modify them.

Licensed under the GNU GPL v3 or later (see LEGAL.md).
