# ADR-0008: On Windows, the Direct3D 9 layer is forwarded to the real d3d9.dll

- **Status:** Accepted by the maintainer, 2026-10-27.
- **Date:** 2026-10-27
- **Extends:** [ADR-0007](0007-renderer-translated-on-own-d3d9-layer.md) (the wgpu layer
  stays the plan for every platform; this is how the game's own rendering is seen now).

## Context

The game's renderer runs translated on the platform's own Direct3D 9 layer
(`crates/engine-core/src/units/platform/d3d9*.rs`). That layer is a model: it
answers every call as Direct3D 9 does on the player's machine (the machine
table of `scripts/d3d9-capture`), keeps the resources in engine memory, and
draws nothing. The game's main menu, intro movie and world were only visible
through the Bevy viewer's own redraw.

The wgpu layer of ADR-0007 and its shader converter are a large piece of
work. Until it exists, nothing shows whether the translated renderer draws
the right thing.

## Decision

On Windows, with `NV_D3D9=real`, the platform layer also makes every call
that changes what is drawn on the real `d3d9.dll` of the player's system, in
a real window. The game's code keeps calling the modelled objects; the model
keeps answering. Forwarding to the real DLL is the platform itself: it is
what `FalloutNV.exe` calls. Nothing about the game is invented.

Design (`units/platform/d3d9real.rs`):

1. **The window.** When the game creates its top-level window
   (`CreateWindowExA`), a real Win32 window of the same title, styles and
   rectangle is made, and `SetWindowPos` moves it with the modelled one. It
   runs on a thread of its own that only creates it and pumps its messages
   (not pumped from the frame driver, as first planned: the game's threads
   are OS threads of the thread host, so a window owned by one of them would
   only get messages while that thread runs). It is shown without taking the
   focus; a click gives it the focus like any window. While it has the
   focus its keyboard (scan codes as DirectInput key codes, extended keys
   with 0x80) and mouse (motion between its mouse messages, buttons, wheel)
   are an input source of the game's DirectInput devices, merged at each
   frame with the viewer's window and the scripted keys (a key is down when
   any source holds it). Closing it does not end the run. The modelled window
   records are unchanged.
2. **The device.** `Direct3DCreate9` makes the real `IDirect3D9`;
   `CreateDevice` makes the real device on the real window with the present
   parameters as the model filled them in and the game's behaviour flags
   plus `D3DCREATE_FPU_PRESERVE`. The game's x87 precision switch is already
   modelled from the game's own flags
   (`units::platform::direct3d_device_created`); the real runtime must not
   change the host thread's floating-point state, which is not the game's.
   `D3DCREATE_MULTITHREADED` is passed only when the game asks for it: the
   thread host runs one game thread at a time, so the real device is never
   used by two threads at once.
3. **Resources.** Each modelled object holds its real counterpart (a map
   from the modelled address to the real object; destroying the modelled
   object releases the real one). Textures, cube textures, render targets,
   depth surfaces, vertex and index buffers, shaders (the bytecode from
   engine memory) and vertex declarations are made with the same arguments.
   Contents are copied from engine memory when the game unlocks: a surface's
   rows (texture levels included; the D3DX texture loader fills textures
   through the same `LockRect`/`UnlockRect`), a buffer's locked range (with
   `D3DLOCK_DISCARD`/`NOOVERWRITE` passed on for dynamic buffers).
4. **State and draws.** Render, texture stage and sampler states, textures,
   streams, indices, declarations, FVF, shaders and their constants,
   transforms, viewport, scissor, clip planes, render targets, depth
   surfaces, materials and lights, `Clear`, `BeginScene`/`EndScene`,
   `DrawPrimitive`, `DrawIndexedPrimitive`, `DrawPrimitiveUP` (the vertices
   copied from engine memory), `UpdateTexture`, `StretchRect` and `Present`
   are forwarded. A slot the model does not provide yet still stops the run.
5. **Answers.** The game always gets the model's answer (measured against
   the same runtime). Where the real runtime answers differently the call is
   printed once (`NV_D3D9=real: <call>: the model answers .., the real
   runtime ..`), a list of what to measure next.
6. **D3DX.** `D3DXCompileShader` and `D3DXGetShaderConstantTable` call the
   player's own `d3dx9_38.dll` (loaded from the system, never copied into the
   repository) with the source from engine memory, the results handed back
   as modelled `ID3DXBuffer` / `ID3DXConstantTable` objects in engine memory
   (the table's reading slots ask the real table and copy names, defaults and
   descriptions into engine memory; the game's `ID3DXInclude` is called back
   for each `#include`). `D3DXAssembleShader` likewise; the shader profile
   functions give the DLL a stand-in device whose `GetDeviceCaps` asks the
   game's device. These are Windows only and do not depend on
   `NV_D3D9=real`.
7. **The switch.** `NV_D3D9=real` turns the forwarding on; unset (tests,
   other platforms) the layer is exactly the model. The viewer's Bevy window
   stays for now.
8. **Lifetimes and bindings (found 2026-10-27, play run p169).** The game
   creates a pure device (behaviour flags `0x54`: hardware vertex
   processing, `D3DCREATE_PUREDEVICE`, `D3DCREATE_MULTITHREADED`). A pure
   device's runtime takes the fast paths (`CD3DBase::SetStreamSource_FP`) and
   keeps no reference of its own to what is bound, and it does not check what
   it draws with. Three rules follow, all in `d3d9real.rs` and the model:
   - **The model's references are the program's.** Binding a buffer, shader,
     declaration or texture does not change what `AddRef`/`Release` answer; a
     `Release` to 0 while bound answers 0 and the object lives on until it is
     unbound (`d3d9-capture device refs`, measured on a hardware device). The
     model keeps the bindings in a count of their own (`bound`) and destroys an
     object when neither the program nor the device holds it.
   - **A real object outlives the call that unbinds it.** The model lets go of
     the old object inside the call that replaces the binding, before the same
     call reaches the real device. Released then, the real object was freed
     while the pure device still had it bound, and the real `SetStreamSource`
     read freed memory (p169's access violation). A forgotten real object is
     now kept until the frame's `Present`.
   - **The real device never draws with a binding the model does not have.** A
     call the model refuses is not forwarded (the model's state does not
     change, so the real device's must not either). A binding the model takes
     whose object has no real counterpart sets the real binding to null, and
     the real draws are skipped (printed) until it is set again.

## Consequences

- The game's own main menu, movies and world can be seen drawn by the game's
  own renderer through the real driver, and compared with the original
  game's screenshots, before the wgpu layer exists.
- Windows only. The wgpu layer (ADR-0007) remains the way to every other
  platform, and the forwarding becomes the reference it is checked against.
- A real-runtime answer that differs from the model's is a measurement gap
  in the machine table, found by running, not a change of behaviour.
- Calls the model does not provide yet still stop the run as before; each is
  added to the model and forwarded together.
