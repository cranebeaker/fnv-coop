# Architecture decision records

Each decision behind nv-rs's method and layout gets its own numbered file
with **Status**, **Date**, **Context**, **Decision** and **Consequences**.
A later decision that changes an earlier one says so and links back.

| ADR | Title | Status |
| --- | --- | --- |
| [0001](0001-decompilation-informed-reimplementation.md) | Reimplement from the game's own program, data and recordings, checked by oracles | Accepted |
| [0002](0002-xbox-prototype-symbols.md) | Full use of the Xbox 360 prototype symbols | Accepted |
| [0003](0003-decompiled-code-in-the-project.md) | Decompiled code may be translated into the project, marked | Accepted |
| [0004](0004-mudcrab-style-layout.md) | Mudcrab-style repository layout | Rejected 2026-10-06 |
| [0005](0005-research-tools-in-repository.md) | Research tools live in `research/` | Accepted |
| [0006](0006-engine-crate-memory-model.md) | Bulk translation goes into an engine crate on a model of the game's memory | Accepted (lead, 2026-10-09; for maintainer review) |
| [0007](0007-renderer-translated-on-own-d3d9-layer.md) | The game's renderer is translated and runs on our own Direct3D 9 layer | Accepted (maintainer, 2026-10-17) |
| [0008](0008-d3d9-layer-forwarded-to-the-real-runtime.md) | On Windows, the Direct3D 9 layer is forwarded to the real d3d9.dll (`NV_D3D9=real`) | Accepted (maintainer, 2026-10-27) |
