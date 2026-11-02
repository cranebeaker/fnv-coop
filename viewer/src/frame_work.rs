//! `--fps`'s work times: how long each frame's own work takes on the main
//! thread (`First` to `Last`) and on the render thread (the render
//! schedule, extraction to cleanup), apart from any wait for the display.
//! A window behind others is held to the display's pace by Windows, so its
//! frame rate says little; its work still shows the hitches.

use std::sync::{Arc, Mutex};
use std::time::Instant;

use bevy::prelude::*;
use bevy::render::{Render, RenderApp, RenderSet};

/// Work times gathered since the last report, in milliseconds.
#[derive(Default, Clone, Copy)]
pub struct Times {
    pub sum: f64,
    pub longest: f64,
    pub frames: u32,
}

impl Times {
    fn add(&mut self, ms: f64) {
        self.sum += ms;
        self.longest = self.longest.max(ms);
        self.frames += 1;
    }

    /// The mean and the longest, and starts again.
    pub fn take(&mut self) -> (f64, f64) {
        let mean = if self.frames == 0 {
            0.0
        } else {
            self.sum / f64::from(self.frames)
        };
        let out = (mean, self.longest);
        *self = Times::default();
        out
    }
}

/// Shared by both worlds: the main thread's and the render thread's times.
#[derive(Resource, Clone, Default)]
pub struct FrameWork {
    pub main: Arc<Mutex<Times>>,
    pub render: Arc<Mutex<Times>>,
}

#[derive(Resource, Default)]
struct MainStart(Option<Instant>);

#[derive(Resource, Default)]
struct RenderStart(Option<Instant>);

pub struct FrameWorkPlugin;

impl Plugin for FrameWorkPlugin {
    fn build(&self, app: &mut App) {
        let work = FrameWork::default();
        app.insert_resource(work.clone())
            .init_resource::<MainStart>()
            .add_systems(First, |mut start: ResMut<MainStart>| {
                start.0 = Some(Instant::now());
            })
            .add_systems(Last, |start: Res<MainStart>, work: Res<FrameWork>| {
                if let (Some(t), Ok(mut m)) = (start.0, work.main.lock()) {
                    m.add(t.elapsed().as_secs_f64() * 1000.0);
                }
            });
        if let Some(render) = app.get_sub_app_mut(RenderApp) {
            render
                .insert_resource(work)
                .init_resource::<RenderStart>()
                .add_systems(
                    Render,
                    (|mut start: ResMut<RenderStart>| {
                        start.0 = Some(Instant::now());
                    })
                    .in_set(RenderSet::ExtractCommands),
                )
                .add_systems(
                    Render,
                    (|start: Res<RenderStart>, work: Res<FrameWork>| {
                        if let (Some(t), Ok(mut r)) = (start.0, work.render.lock()) {
                            r.add(t.elapsed().as_secs_f64() * 1000.0);
                        }
                    })
                    .in_set(RenderSet::Cleanup),
                );
        }
    }
}
