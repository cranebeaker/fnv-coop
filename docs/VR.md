# VR support

VR is a built-in mode of the engine rather than a mod bolted on afterwards.
One build runs both ways: on a flat screen or, with a setting or `--vr` flag,
on a headset. The two modes share the world, simulation, AI and content. Only
the camera rig, input bindings and UI placement differ.

Retrofitting VR onto an engine that assumed one flat camera is what makes VR
mods for the original game so hard. The rules below keep that from happening
here.

## Design rules

These apply from the first rendering commit, long before any headset code
exists.

1. **The camera is not the player.** The player entity (body, collision
   capsule, position) is separate from the view rig. On a flat screen the rig
   is one camera; in VR it is a play-space origin with a tracked head and two
   eye views under it. Gameplay code never reads the camera.

2. **Aim is its own input.** Firing, activating objects and line-of-sight
   checks take an *aim ray*, never "the camera's forward vector". On a flat
   screen the aim ray follows the mouse look. In VR it comes from the
   weapon-hand controller, so you can look one way and shoot another.

3. **Controls go through actions.** Gameplay listens for actions such as
   `Move`, `Turn`, `Jump`, `Activate`, `Fire`, `Reload`, `OpenMenu` and
   `QuickSlot(n)`, never for specific keys. Keyboard and mouse, gamepads and
   VR controllers are just different bindings. OpenXR is built around action
   sets, so this maps onto it directly.

4. **The renderer draws any number of views.** Nothing may assume one camera
   per frame: culling, shadows and post-processing run per view. Screen-space
   effects (bloom, ambient occlusion, motion blur) need a VR-safe version or an
   off switch.

5. **UI can live in the world.** The HUD and menus render either as a flat
   overlay or on a panel placed in 3D: at a fixed distance, or on the wrist.
   No UI element may assume a single fixed screen rectangle.

6. **Rendering runs at headset rate; simulation runs on its own fixed tick.**
   At 90 Hz, two views must be drawn in 11.1 ms. Gameplay stays on a fixed
   timestep (as iw4L does), so a faster display doesn't change how the game
   plays. Streaming, decompression and asset loading stay off the render
   thread, because a hitch that is annoying on a monitor causes nausea in a
   headset.

7. **Scale is measured, not assumed.** VR only feels right if a door is
   exactly door-sized. The conversion from game units to meters gets
   measured from real content (door frames, the player's collision height)
   once meshes load, and is kept in one constant.

8. **Comfort options exist from day one.** These are snap or smooth turning,
   teleport or smooth movement, an optional vignette while moving, and a
   seated or standing mode with height calibration.

## New Vegas features that need VR-specific designs

- **VATS** pauses the game to pick body parts from a menu. In VR it becomes a
  slow-motion mode where you aim yourself; existing VR mods for the original
  game recommend the same approach.
- **Dialogue** normally zooms the camera onto the speaker's face. In VR your
  head stays free, and the NPC turns to face you instead.
- **Iron sights** become physical: you raise the weapon to your eye.
- **Two-handed weapons** use the off hand on the foregrip for stability.
- **Third-person view** is disabled in VR, except for a detached camera in
  menus.
- **The wrist computer** becomes a real object on your left arm that you raise
  to read.

## Milestones

1. **Viewer:** stand inside the first rendered interior cell with head
   tracking.
2. **Movement:** thumbstick movement, snap and smooth turning, collision with
   the cell.
3. **Hands:** controller models, pointing, opening doors and containers.
4. **Weapons:** aiming from the controller, firing, a reload gesture.
5. **Menus:** the wrist menu and the in-world HUD.
6. **Comfort:** settings, calibration and polish.

## Technology

- OpenXR through [bevy_oxr](https://github.com/awtterpip/bevy_oxr) (the
  `bevy_mod_openxr` crate), licensed MIT or Apache-2.0, both compatible with this project's GPL.
  When Bevy is added, pin its version to one that bevy_oxr supports.
- Runtimes: SteamVR on Windows, Monado on Linux.
- **Developing without a headset:** a debug mode renders both eye views side
  by side on the monitor and moves simulated controllers with the mouse. It
  runs the VR code paths on every build, so they don't break silently between
  headset sessions.
