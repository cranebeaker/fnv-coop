# ADR-0007: The game's renderer is translated and runs on our own Direct3D 9 layer

- **Status:** Accepted by the maintainer, 2026-10-17.
- **Date:** 2026-10-17

## Context

The renderer is the largest piece left after the game logic. The ledger
counts about 2,100 `platform` functions in it: Gamebryo's Direct3D 9
renderer, the shader managers and passes, render targets and the image-space
effects.

The game's renderer decides what is drawn, in what order, with which
shader, constants and render states, and hands each decision to Direct3D 9.
The look itself comes from the game's compiled shaders in
`Data\Shaders\shaderpackageNNN.sdp` (one package per class of graphics
card), which `crates/shaders` already reads and disassembles.

Three ways were weighed:

1. **Keep the Bevy renderer** and redraw the game's look from traced data.
   Lighting, fog, water and post effects would be rebuilt by eye. It cannot
   match exactly, and it is invented behaviour (ADR-0001, ADR-0003).
2. **Translate the renderer and implement Direct3D 9 underneath it** in our
   own layer on a modern graphics API.
3. **Write Vulkan directly.** Replacing each Direct3D 9 call inside the
   translated renderer with Vulkan code is option 2 spread over 2,100
   functions instead of one layer; a new hand-written Vulkan renderer is
   option 1 again.

## Decision

Option 2.

- The renderer's functions are translated like the rest of the exe, with
  addresses marked. Their Direct3D 9 calls stay calls through the device's
  interface, as in the exe.
- A Rust crate implements the part of Direct3D 9 the game uses (device,
  resources, render states, shader constants, draws) on wgpu, which runs on
  Vulkan, Metal and Direct3D 12. It reports device capabilities so the
  game's own code picks its shader package.
- A converter turns the game's Direct3D 9 shader bytecode into wgpu shaders
  at load time, read from the player's install. No shader bytes are
  committed.
- The Bevy renderer stays as the development viewer until the new path
  matches, then retires.

## Consequences

- The game's own shaders run with the game's own inputs and order, so the
  output should match the original up to what already varies between
  graphics cards (filtering, gamma, multisampling).
- The 2,100 renderer functions move from `platform` to `open` in the
  ledger as their units are scaffolded, the same way SpeedTree and the
  logic units moved (#213).
- New work beyond translation: the Direct3D 9 layer and the shader
  converter. Checked by rendering one scene and comparing screenshots
  against the real game, then widening.
- One platform for all targets (Windows, Linux and Steam Deck, macOS, ARM).
- The x87 precision question (HANDOFF, open questions) must be settled
  first. Direct3D 9 drops the creating thread to 24-bit precision unless
  the device is created with `D3DCREATE_FPU_PRESERVE`, and that changes the
  renderer's matrix maths. Read the exe's `CreateDevice` flags.
