//! Day and night outdoors: the light follows the game's clock (`GameHour`,
//! which runs at `TimeScale`). The weather's colours for the hour
//! (`world::weather::time_weights`), the fog between its day and night
//! distances, and the sun's place (`world::weather::sun_at`) go into the
//! light every outdoor surface shares (`crate::shared_light`: the lit
//! surfaces, the terrain, the distant land), the sky dome, the clouds and
//! the sun, a game minute at a time. Lit surfaces are switched to the
//! shared light once (all of them on the first frame outdoors, then each
//! as it appears); the materials aren't touched as the hour moves.

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use esm::FormId;

use crate::dialogue::DialogueState;
use crate::exterior::Exterior;
use crate::lighting::GameLitMaterial;
use crate::shared_light::{MenuLit, SharedLight, SharedLightNow};
use crate::{light_fields, CloudScroll, GameFiles, SkyEntity};

/// The sky dome's own vertex colours: how much of each of its three colours
/// each vertex takes (`cellview::game::sky_color`).
#[derive(Component)]
pub struct SkyWeights(pub Vec<[f32; 4]>);

/// One of the sun's squares: its half-size in sky units, and for the
/// glare the weather's glare strength.
#[derive(Component)]
pub struct SunDisk {
    pub half_size: f32,
    pub glare: Option<f32>,
}

/// The stars: each vertex's fade at the horizon.
#[derive(Component)]
pub struct Stars {
    pub fade: Vec<f32>,
}

/// The weathers the light last followed (current, fading out), the hour
/// last applied, and whether every lit surface outdoors takes the shared
/// light.
/// Lit materials made again in place (a lone actor lit again for another
/// place, `crate::LitPieces::relight`): taken up as newly put on screen
/// ones are, as they would be had the actor been rebuilt (here, and by the
/// Pip-Boy light, `crate::pipboy`).
#[derive(Resource, Default)]
pub struct RemadeLit {
    /// Not yet taken up here.
    pub ids: Vec<AssetId<GameLitMaterial>>,
    /// How many times some were made again.
    pub times: u64,
}

#[derive(Resource, Default)]
pub struct Daylight {
    shown: Option<(Option<FormId>, Option<FormId>)>,
    applied: Option<f32>,
    switched: bool,
}

/// One game minute, in hours: how far the clock moves before the light is
/// worked out again.
const MINUTE: f32 = 1.0 / 60.0;

/// Lit surfaces outdoors (not the sky, not a 3D menu's pieces).
type Outdoors = (Without<SkyEntity>, Without<MenuLit>);
/// Those just put on screen.
type NewLit = (
    Added<MeshMaterial3d<GameLitMaterial>>,
    Without<SkyEntity>,
    Without<MenuLit>,
);
/// A sky dome or stars just put on screen.
type NewSky = Or<(Added<SkyWeights>, Added<Stars>)>;

#[derive(SystemParam)]
pub struct Lit<'w, 's> {
    lit: ResMut<'w, Assets<GameLitMaterial>>,
    shared: ResMut<'w, SharedLightNow>,
    meshes: ResMut<'w, Assets<Mesh>>,
    all_lit: Query<'w, 's, &'static MeshMaterial3d<GameLitMaterial>, Outdoors>,
    new_lit: Query<'w, 's, &'static MeshMaterial3d<GameLitMaterial>, NewLit>,
    remade: ResMut<'w, RemadeLit>,
    domes: Query<'w, 's, (&'static Mesh3d, &'static SkyWeights)>,
    new_domes: Query<'w, 's, (), NewSky>,
    stars: Query<'w, 's, (&'static Mesh3d, &'static Stars)>,
    clouds: Query<'w, 's, (&'static Mesh3d, &'static CloudScroll)>,
    suns: Query<'w, 's, (&'static Mesh3d, &'static SunDisk, &'static mut Visibility)>,
}

/// Outdoors, every frame: once the clock has moved a game minute, the
/// light of the hour; lit surfaces not yet on the shared light switched to
/// it.
#[allow(clippy::too_many_arguments)]
pub fn follow_the_clock(
    game: Res<GameFiles>,
    state: Res<DialogueState>,
    exterior: Option<ResMut<Exterior>>,
    settings: Res<crate::Settings>,
    mut daylight: ResMut<Daylight>,
    mut clear: ResMut<ClearColor>,
    mut place: ResMut<crate::viewmodel::PlaceLighting>,
    mut weathers: ResMut<crate::weather::Weathers>,
    mut lit: Lit,
) {
    let daylight = &mut *daylight;
    // Like the `Added` filter, seen once.
    let remade = std::mem::take(&mut lit.remade.ids);
    let Some(mut exterior) = exterior else {
        daylight.applied = None;
        daylight.switched = false;
        return;
    };
    let order = &game.0.order;
    // The game state's weather, mid-fade or not (`crate::weather`).
    let shown = (
        state.0.weather.current.or(exterior.weather),
        state.0.weather.previous,
    );
    if daylight.shown != Some(shown) {
        daylight.shown = Some(shown);
        daylight.applied = None;
    }
    let Some(weather) = weathers.mix(order, &state.0, exterior.weather) else {
        return;
    };
    let weather = &weather;
    let hour = state
        .0
        .global(order, "GameHour")
        .unwrap_or(world::weather::DEFAULT_HOUR);
    let all = daylight
        .applied
        .is_none_or(|h| (h - hour).abs() >= MINUTE || lit.new_domes.iter().next().is_some());
    let newcomers = !lit.new_lit.is_empty() || !remade.is_empty() || !daylight.switched;
    if !all && !newcomers {
        return;
    }
    let clock = world::weather::SkyClock::new(
        exterior.grid.climate.as_ref(),
        world::weather::SkySettings::load(order),
    );
    // The colours blended in floats and the sun's exact direction, as the
    // game sends them (`world::weather::SkyLight`).
    let light = weather.light_at(&clock, hour);
    let (ambient, mut directional, fog) = cellview::exterior_light(&light);
    // Outdoors the sun's light (not the ambient) is × the image space's
    // sunlight dimmer after the weather's modifiers (`00b70820`,
    // `00b8b440`; Goodsprings by day 1.1 × 1.1 = 1.21, recorded); the sky's
    // shaders take its "LUM ramp no tex" the same way (0.88, recorded).
    let modifier = weather.modifier_at(order, &clock, hour);
    let image_space = exterior.grid.image_space.as_ref();
    let dimmer = world::weather::sunlight_dimmer(image_space, modifier.as_ref());
    let sky_brightness = world::weather::sky_brightness(image_space, modifier.as_ref());
    if let Some(d) = directional.as_mut() {
        d.color = d.color.map(|c| c * dimmer);
    }
    let fields = light_fields(
        ambient,
        directional.as_ref(),
        fog.as_ref(),
        settings.brightness,
    );

    // The shared light (the terrain and the distant land always read it).
    let now = SharedLight {
        ambient: fields.ambient,
        directional_color: fields.directional_color,
        directional_direction: fields.directional_direction,
        fog_color: fields.fog_color,
        fog_range: fields.fog_range,
    };
    if lit.shared.0 != now {
        lit.shared.0 = now;
    }
    // Lit surfaces onto it: every one the first time outdoors, else the
    // new ones (their own copy kept up to date with it too).
    let lit_handles: Vec<AssetId<GameLitMaterial>> = if daylight.switched {
        lit.new_lit.iter().map(|m| m.0.id()).chain(remade).collect()
    } else {
        lit.all_lit.iter().map(|m| m.0.id()).collect()
    };
    daylight.switched = true;
    for id in lit_handles {
        if let Some(m) = lit.lit.get_mut(id) {
            fields.apply(&mut m.extension.lighting);
            m.extension.lighting.scale.z = 1.0;
        }
    }
    if let Some(p) = exterior.lod_params.as_mut() {
        p.ambient = fields.ambient;
        p.sun_color = fields.directional_color;
        p.sun_direction = fields.directional_direction;
        p.fog_color = fields.fog_color;
        p.fog_range = fields.fog_range;
    }
    // What's lit by the place later (people coming in) takes it too; the
    // first-person view's materials were among those above.
    place.relight(|l| fields.apply(l));
    if !all {
        return;
    }
    daylight.applied = Some(hour);

    // The sky: the dome's colours, the clouds', the stars', the sun's place
    // and colour, each × the sky's brightness (`Params.y` of the sky
    // shaders).
    let sky = weather.sky_at(&clock, hour).map(|c| c.map(|v| v / 255.0));
    // Where nothing is drawn: the game clears the picture to the fog's
    // colour (recorded at Goodsprings: (150, 168, 190), `NVWastelandGS`'s
    // fog by day), as a stored value like the rest.
    let [r, g, b] = light.fog_color;
    clear.0 = Color::linear_rgb(r, g, b);
    let bright = |c: f32| crate::srgb_decode(c * sky_brightness);
    for (mesh, weights) in &lit.domes {
        if let Some(m) = lit.meshes.get_mut(&mesh.0) {
            let colors: Vec<[f32; 4]> = weights
                .0
                .iter()
                .map(|&w| {
                    let c = cellview::game::sky_color(w, sky);
                    [bright(c[0]), bright(c[1]), bright(c[2]), c[3]]
                })
                .collect();
            m.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        }
    }
    for (mesh, cloud) in &lit.clouds {
        let c = weather
            .cloud_color_at(cloud.layer, &clock, hour)
            .map(|v| bright(v / 255.0));
        if let Some(m) = lit.meshes.get_mut(&mesh.0) {
            let colors: Vec<[f32; 4]> = cloud
                .alphas
                .iter()
                .map(|&a| [c[0], c[1], c[2], a])
                .collect();
            m.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        }
    }
    // The stars: the weather's star colour at the hour's star alpha
    // (`00640020`), times each vertex's fade below the horizon.
    let star_alpha = world::weather::stars_alpha(&clock, hour);
    let star = weather
        .color_at(world::weather::SkyColor::Stars, &clock, hour)
        .map(|v| bright(v / 255.0));
    for (mesh, stars) in &lit.stars {
        if let Some(m) = lit.meshes.get_mut(&mesh.0) {
            let colors: Vec<[f32; 4]> = stars
                .fade
                .iter()
                .map(|&a| [star[0], star[1], star[2], a * star_alpha])
                .collect();
            m.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        }
    }
    // The sun's disc and glare where the game's path puts them, coloured by
    // the weather's sun colour, at the sun's visibility (the glare also × the
    // weather's glare strength).
    let now = world::weather::sun_at(&clock, hour);
    let distance = now.disc.iter().map(|c| c * c).sum::<f32>().sqrt().max(1.0);
    let sun = weather
        .color_at(world::weather::SkyColor::Sun, &clock, hour)
        .map(|v| bright(v / 255.0));
    for (mesh, disk, mut visibility) in &mut lit.suns {
        let alpha = now.visibility * disk.glare.unwrap_or(1.0);
        *visibility = if alpha > 0.0 {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if let Some(m) = lit.meshes.get_mut(&mesh.0) {
            // Kept the size it has as seen from the eye.
            let size = 2.0 * disk.half_size * cellview::game::SUN_DISTANCE / distance;
            m.insert_attribute(Mesh::ATTRIBUTE_POSITION, crate::sun_square(now.disc, size));
            m.insert_attribute(
                Mesh::ATTRIBUTE_COLOR,
                vec![[sun[0], sun[1], sun[2], alpha]; 4],
            );
        }
    }
}
