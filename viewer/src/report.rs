//! F12 (a tool for comparing with the game, not one of the game's keys):
//! saves a report of what's on screen into `reports\NNN\` in the folder
//! the viewer runs in: the picture, where the player stands and looks
//! (as the game's console gives them), the hour, the lines that put the
//! real game and the viewer in the same spot, and the whole state as F5
//! saves it. The person playing writes what looks or behaves differently
//! in its `note.txt`.

use std::path::{Path, PathBuf};

use bevy::prelude::*;
use bevy::render::view::screenshot::{save_to_disk, Screenshot};

use crate::dialogue::DialogueState;
use crate::scripts::Notices;
use crate::walk::{game_point, Player};
use crate::{FlyCamera, GameFiles};

/// Where reports go, under the folder the viewer runs in.
const REPORTS: &str = "reports";

/// The next free `reports\NNN` folder.
fn next_folder(root: &Path) -> PathBuf {
    let used = std::fs::read_dir(root)
        .map(|dir| {
            dir.filter_map(|e| e.ok())
                .filter_map(|e| e.file_name().to_str()?.parse::<u32>().ok())
                .max()
                .unwrap_or(0)
        })
        .unwrap_or(0);
    root.join(format!("{:03}", used + 1))
}

/// Where the player is, as the game's console gives it: the feet, the
/// heading clockwise from north and the pitch positive looking down, in
/// degrees.
struct Spot {
    feet: [f32; 3],
    heading: f32,
    pitch: f32,
}

fn spot(camera: &Transform, player: &Player, body_angles: Option<(f32, f32)>) -> Spot {
    let eye = game_point(camera.translation);
    let f = camera.forward().as_vec3();
    Spot {
        feet: player.position_for_view(eye),
        heading: body_angles
            .map_or_else(|| f.x.atan2(-f.z), |a| a.0)
            .to_degrees()
            .rem_euclid(360.0),
        pitch: body_angles
            .map_or_else(|| -f.y.clamp(-1.0, 1.0).asin(), |a| a.1)
            .to_degrees(),
    }
}

/// The report's text: the place, the spot, the hour, and how to get
/// there in the game and in the viewer.
fn describe(place: &str, world: bool, s: &Spot, hour: Option<f32>) -> String {
    let [x, y, z] = s.feet;
    let mut out = String::new();
    out.push_str(&format!("Place: {place}\n"));
    out.push_str(&format!(
        "Feet: {x:.2}, {y:.2}, {z:.2}; heading {:.2}; pitch {:.2}\n",
        s.heading, s.pitch
    ));
    if let Some(h) = hour {
        out.push_str(&format!("Game hour: {h:.4}\n"));
    }
    out.push_str("\nIn the game's console:\n");
    if world {
        let (cx, cy) = ((x / 4096.0).floor(), (y / 4096.0).floor());
        out.push_str(&format!("cow {place} {cx} {cy}\n"));
    } else {
        out.push_str(&format!("coc {place}\n"));
    }
    out.push_str(&format!(
        "player.setpos x {x:.2}\nplayer.setpos y {y:.2}\nplayer.setpos z {z:.2}\n\
         player.setangle z {:.2}\nplayer.setangle x {:.2}\n",
        s.heading, s.pitch
    ));
    if let Some(h) = hour {
        out.push_str(&format!("set gamehour to {h:.4}\n"));
    }
    out.push_str("\nIn the viewer:\n");
    out.push_str(&format!(
        "nv-viewer <Data folder> {place} --at {x:.2},{y:.2},{z:.2},{:.2},{:.2}",
        s.heading, s.pitch
    ));
    if let Some(h) = hour {
        out.push_str(&format!(" --run \"set GameHour to {h:.4}\""));
    }
    out.push_str(
        "\n(state.txt is the whole state as F5 saves it: copy it to \
         nv-rs-quicksave.txt and press F9 to load it.)\n",
    );
    out
}

#[allow(clippy::too_many_arguments)] // Bevy system resources are separate parameters.
pub fn report_key(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    game: Res<GameFiles>,
    state: Res<DialogueState>,
    player: Res<Player>,
    mut notices: ResMut<Notices>,
    cameras: Query<(&Transform, &FlyCamera)>,
    // The game's own code: how many quests its interpreter ran.
    game_code: Option<NonSend<host::Host>>,
) {
    if !keys.just_pressed(KeyCode::F12) {
        return;
    }
    let Ok((camera, input)) = cameras.single() else {
        return;
    };
    let order = &game.0.order;
    let state = &state.0;
    let s = spot(
        camera,
        &player,
        player.walking.then_some((-input.yaw, -input.pitch)),
    );
    let name_of = |form| {
        order
            .get(form)
            .and_then(|r| r.editor_id().ok().flatten())
            .unwrap_or_else(|| format!("{form:?}"))
    };
    let (place, world) = match (state.player_world, state.player_cell) {
        (Some(w), _) => (name_of(w), true),
        (None, Some(c)) => (name_of(c), false),
        (None, None) => ("(unknown)".to_string(), false),
    };
    let hour = state.global(order, "GameHour");
    let folder = next_folder(Path::new(REPORTS));
    let saved = state.player_cell.map(|cell| world::save::PlayerPlace {
        cell,
        world: state.player_world,
        position: s.feet,
        heading: s.heading.to_radians(),
    });
    let written = std::fs::create_dir_all(&folder)
        .and_then(|()| {
            let mut report = describe(&place, world, &s, hour);
            if let Some(game_code) = &game_code {
                report.push_str(&format!("\n{}\n", game_code.quests.summary()));
            }
            std::fs::write(folder.join("report.txt"), report)
        })
        .and_then(|()| std::fs::write(folder.join("state.txt"), world::save::save(state, saved)))
        .and_then(|()| {
            std::fs::write(
                folder.join("note.txt"),
                "Write here what looks or behaves differently from the game:\n\n",
            )
        });
    let text = match written {
        Ok(()) => {
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(folder.join("picture.png")));
            format!("Report saved in {}.", folder.display())
        }
        Err(e) => format!("Couldn't save the report: {e}"),
    };
    println!("{text}");
    notices.push(text, time.elapsed_secs());
}

#[cfg(test)]
mod tests {
    use super::*;
    use cellview::space;

    #[test]
    fn spot_reads_like_the_console() {
        // Looking north and a little down from 120 units above the feet.
        let eye = Vec3::new(1.0, 2.0, -3.0) * space::METERS_PER_UNIT;
        let mut camera = Transform::from_translation(eye);
        camera.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, -0.1, 0.0);
        let player = Player::new(false);
        let s = spot(&camera, &player, None);
        assert!((s.feet[0] - 1.0).abs() < 1e-3);
        assert!((s.feet[1] - 3.0).abs() < 1e-3);
        assert!((s.feet[2] - (2.0 - cellview::EYE_HEIGHT)).abs() < 1e-3);
        assert!(s.heading.abs() < 1e-3 || (s.heading - 360.0).abs() < 1e-3);
        assert!((s.pitch - 0.1f32.to_degrees()).abs() < 1e-3);
    }

    #[test]
    fn walking_report_feet_ignore_displaced_camera() {
        let mut player = Player::new(true);
        player.arrive([10.0, 20.0, 30.0]);
        let camera =
            Transform::from_translation(Vec3::new(1.0, 20.0, -3.0) * space::METERS_PER_UNIT);

        let s = spot(&camera, &player, Some((0.7, -0.2)));
        assert!((s.heading - 0.7f32.to_degrees()).abs() < 1e-3);
        assert!((s.pitch + 0.2f32.to_degrees()).abs() < 1e-3);

        assert_eq!(s.feet, [10.0, 20.0, 30.0]);
    }

    #[test]
    fn free_camera_report_keeps_eye_height_offset() {
        let player = Player::new(false);
        let camera =
            Transform::from_translation(Vec3::new(1.0, 120.0, -3.0) * space::METERS_PER_UNIT);

        let s = spot(&camera, &player, None);

        assert!((s.feet[0] - 1.0).abs() < 1e-3);
        assert!((s.feet[1] - 3.0).abs() < 1e-3);
        assert!((s.feet[2] - (120.0 - cellview::EYE_HEIGHT)).abs() < 1e-3);
    }

    #[test]
    fn report_gives_the_console_lines() {
        let s = Spot {
            feet: [-72151.5, 639.26, 8281.63],
            heading: 90.0,
            pitch: 0.0,
        };
        let text = describe("WastelandNV", true, &s, Some(13.0978));
        assert!(text.contains("cow WastelandNV -18 0"));
        assert!(text.contains("player.setangle z 90.00"));
        assert!(text.contains("set gamehour to 13.0978"));
    }

    #[test]
    fn folders_count_up() {
        let root = std::env::temp_dir().join(format!("nv-rs-reports-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        assert_eq!(next_folder(&root), root.join("001"));
        std::fs::create_dir_all(root.join("001")).unwrap();
        std::fs::create_dir_all(root.join("007")).unwrap();
        assert_eq!(next_folder(&root), root.join("008"));
        let _ = std::fs::remove_dir_all(&root);
    }
}
