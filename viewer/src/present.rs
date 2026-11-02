//! How frames reach the window: mailbox where the window's surface takes it.
//!
//! With Bevy's default (`AutoVsync`, first-in first-out) each frame waits
//! its turn for the display: no tearing, but a window that isn't in front
//! (another window over it, or the viewer started from a terminal) is held
//! to 60 frames a second however fast the frames are made. Mailbox shows
//! the newest finished frame at each refresh instead: still no tearing,
//! and the frame rate isn't held to the display's. The surface's own modes
//! are asked first (Bevy doesn't fall back from a mode the surface lacks);
//! without mailbox the default stays.

#[cfg(not(target_os = "macos"))]
use std::sync::atomic::{AtomicU8, Ordering};
#[cfg(not(target_os = "macos"))]
use std::sync::Arc;

use bevy::prelude::*;
#[cfg(not(target_os = "macos"))]
use bevy::render::renderer::{RenderAdapter, RenderInstance};
#[cfg(not(target_os = "macos"))]
use bevy::render::view::window::ExtractedWindows;
#[cfg(not(target_os = "macos"))]
use bevy::render::{Render, RenderApp, RenderSet};
#[cfg(not(target_os = "macos"))]
use bevy::window::{PresentMode, PrimaryWindow};

/// What the render world found about the surface: 0 not yet, 1 mailbox
/// there, 2 not.
#[derive(Resource, Clone, Default)]
#[cfg(not(target_os = "macos"))]
struct Mailbox(Arc<AtomicU8>);

pub struct PresentPlugin;

impl Plugin for PresentPlugin {
    /// Metal surface capability queries must run on the UI thread. This
    /// optimization probes from Bevy's render thread, so macOS keeps Bevy's
    /// default present mode instead.
    #[cfg(target_os = "macos")]
    fn build(&self, _app: &mut App) {}

    #[cfg(not(target_os = "macos"))]
    fn build(&self, app: &mut App) {
        let found = Mailbox::default();
        app.insert_resource(found.clone()).add_systems(
            Update,
            use_mailbox.in_set(crate::frame_order::ViewerSet::AfterFrame),
        );
        if let Some(render) = app.get_sub_app_mut(RenderApp) {
            render
                .insert_resource(found)
                .add_systems(Render, look_at_surface.in_set(RenderSet::Cleanup));
        }
    }
}

/// Once the window is up: whether a surface on it takes mailbox (one
/// made for the question, as Bevy makes its own, and dropped).
#[cfg(not(target_os = "macos"))]
fn look_at_surface(
    found: Res<Mailbox>,
    windows: Res<ExtractedWindows>,
    instance: Res<RenderInstance>,
    adapter: Res<RenderAdapter>,
) {
    if found.0.load(Ordering::Relaxed) != 0 {
        return;
    }
    let Some(window) = windows.primary.and_then(|e| windows.windows.get(&e)) else {
        return;
    };
    let target = wgpu::SurfaceTargetUnsafe::RawHandle {
        raw_display_handle: window.handle.get_display_handle(),
        raw_window_handle: window.handle.get_window_handle(),
    };
    // SAFETY: the extracted window's handles are valid to make surfaces on
    // (Bevy makes its own from them the same way).
    let Ok(surface) = (unsafe { instance.create_surface_unsafe(target) }) else {
        found.0.store(2, Ordering::Relaxed);
        return;
    };
    let has = surface
        .get_capabilities(&adapter)
        .present_modes
        .contains(&wgpu::PresentMode::Mailbox);
    found.0.store(if has { 1 } else { 2 }, Ordering::Relaxed);
}

/// The window presents by mailbox once its surface is known to take it.
#[cfg(not(target_os = "macos"))]
fn use_mailbox(
    found: Res<Mailbox>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut done: Local<bool>,
) {
    if *done {
        return;
    }
    match found.0.load(Ordering::Relaxed) {
        0 => {}
        1 => {
            if let Ok(mut window) = windows.single_mut() {
                window.present_mode = PresentMode::Mailbox;
            }
            *done = true;
        }
        _ => {
            println!("  the window can't present by mailbox: frames wait for the display");
            *done = true;
        }
    }
}
