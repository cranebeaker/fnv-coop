//! `play [SECONDS]`: starts a new game's state and lets time pass, running
//! every running quest's script as the game would, then says what
//! happened (stages set, messages, objectives) and which functions the
//! scripts needed that aren't carried out yet.

use std::io::Write;
use std::time::Instant;

use esm::LoadOrder;
use world::chargen::CharacterMenu;
use world::scripting::{Event, GameState, Runner, ScriptCache};

use crate::records::describe_id;
use crate::CliError;

/// `scripted <CELL>`: the objects in a cell with scripts, and the script
/// blocks each has (triggers with their volumes).
pub fn scripted(out: &mut impl Write, order: &LoadOrder, cell: &str) -> Result<(), CliError> {
    let found = world::find_cells(order, cell).map_err(|e| CliError::NotFound(e.to_string()))?;
    let Some(&cell) = found.first() else {
        return Err(CliError::NotFound(format!("no cell matches '{cell}'")));
    };
    let scripts = ScriptCache::default();
    let refs = world::scripting::interactive_references(order, cell);
    let items = refs.iter().filter(|r| r.is_item()).count();
    let containers = refs.iter().filter(|r| r.is_container()).count();
    writeln!(
        out,
        "{} objects to use in {} ({items} items lying around, {containers} containers):",
        refs.len(),
        describe_id(order, cell)
    )?;
    let navmeshes: Vec<String> = order
        .in_cell(cell)
        .iter()
        .filter(|r| r.entry.header.kind == esm::FourCC::new(b"NAVM"))
        .map(|r| r.form_id.to_string())
        .collect();
    writeln!(out, "  navmeshes: {}", navmeshes.join(", "))?;
    // How a navmesh marks its edges to other navmeshes: triangles' flag
    // bits against the external connection count.
    for r in order.in_cell(cell) {
        if r.entry.header.kind != esm::FourCC::new(b"NAVM") {
            continue;
        }
        let Ok(record) = r.record() else { continue };
        let (Some(tr), ex) = (
            record.get(esm::FourCC::new(b"NVTR")),
            record.get(esm::FourCC::new(b"NVEX")),
        ) else {
            continue;
        };
        let triangles = tr.data.len() / 16;
        let mut by_bit = [0usize; 16];
        let mut beyond = 0usize;
        let mut flagged_values = Vec::new();
        for c in tr.data.chunks_exact(16) {
            let flags = u16::from_le_bytes([c[12], c[13]]);
            for (b, n) in by_bit.iter_mut().enumerate() {
                if flags & (1 << b) != 0 {
                    *n += 1;
                }
            }
            for e in 0..3 {
                let n = u16::from_le_bytes([c[6 + e * 2], c[7 + e * 2]]);
                if n != 0xFFFF && usize::from(n) >= triangles {
                    beyond += 1;
                }
                if flags & (1 << e) != 0 {
                    flagged_values.push(n);
                }
            }
        }
        writeln!(
            out,
            "  {}: {triangles} triangles, {} external connections; triangles per flag bit {:?}; \
             edge links past the last triangle: {beyond}; links on edges flagged 0/1/2 (first 12): {:?}",
            r.form_id,
            ex.map_or(0, |e| e.data.len() / 10),
            by_bit,
            &flagged_values[..flagged_values.len().min(12)]
        )?;
    }
    for r in refs.iter().filter(|r| r.script.is_some()) {
        let script = r.script.unwrap();
        let blocks: Vec<String> = scripts
            .script(order, script)
            .map(|s| {
                s.blocks
                    .iter()
                    .map(|b| {
                        let args: Vec<String> = b
                            .args
                            .iter()
                            .map(|a| match a {
                                script::Arg::Word(w) => w.clone(),
                                script::Arg::Number(n) => n.to_string(),
                                script::Arg::Str(s) => format!("{s:?}"),
                            })
                            .collect();
                        format!("{} {}", b.kind, args.join(" ")).trim().to_string()
                    })
                    .collect()
            })
            .unwrap_or_else(|| vec!["(script doesn't parse)".into()]);
        writeln!(
            out,
            "  {} ({}) at {:.0},{:.0},{:.0}: {}",
            r.reference,
            describe_id(order, r.base),
            r.position[0],
            r.position[1],
            r.position[2],
            describe_id(order, script)
        )?;
        writeln!(out, "      blocks: {}", blocks.join(", "))?;
        if let Some(t) = r.trigger {
            writeln!(
                out,
                "      trigger: shape {}, {:.0} x {:.0} x {:.0} (half sizes), turned {:.0},{:.0},{:.0} degrees",
                t.shape,
                t.half[0],
                t.half[1],
                t.half[2],
                r.rotation[0].to_degrees(),
                r.rotation[1].to_degrees(),
                r.rotation[2].to_degrees()
            )?;
        }
    }
    Ok(())
}

/// `ai <REF|PACK> [QUEST STAGE]`: a package's actions, or a person's packages, which one they'd
/// follow in a new game (after setting a stage), where it leads, and the
/// navmesh path there.
pub fn ai(
    out: &mut impl Write,
    order: &LoadOrder,
    target: &str,
    stage: Option<(&str, u16)>,
) -> Result<(), CliError> {
    use world::ai::{current_package, destination, packages_of, NavMesh, Package};
    let reference = esm::FormId::parse_hex(target)
        .filter(|id| order.get(*id).is_some())
        .or_else(|| order.form_by_editor_id(target))
        .ok_or_else(|| CliError::NotFound(format!("no reference '{target}'")))?;
    if let Some(pkg) = Package::load(order, reference) {
        writeln!(
            out,
            "{}: type {}, flags {:#010x}",
            describe_id(order, reference),
            pkg.kind,
            pkg.flags
        )?;
        package_actions(out, order, &pkg)?;
        return Ok(());
    }
    let base = world::scripting::base_of(order, reference)
        .ok_or_else(|| CliError::NotFound(format!("{target} isn't a placed object")))?;
    let scripts = ScriptCache::default();
    let mut state = GameState::new(order);
    if let Some((quest, n)) = stage {
        let q = order
            .form_by_editor_id(quest)
            .ok_or_else(|| CliError::NotFound(format!("no quest '{quest}'")))?;
        Runner::new(order, &scripts, &mut state).set_stage(q, n);
        // Let the quest scripts settle.
        let mut runner = Runner::new(order, &scripts, &mut state);
        for _ in 0..30 {
            runner.update(0.1);
        }
    }
    writeln!(
        out,
        "{} ({}), packages in order:",
        describe_id(order, reference),
        describe_id(order, base)
    )?;
    for p in packages_of(order, base) {
        let Some(pkg) = Package::load(order, p) else {
            writeln!(out, "  {} (not a package)", describe_id(order, p))?;
            continue;
        };
        let facts = world::scripting::Facts {
            order,
            state: &state,
            speaker: None,
        };
        let passes = facts.conditions_pass(&pkg.conditions, reference, world::dialogue::PLAYER_REF);
        writeln!(
            out,
            "  {} type {}, {} conditions{}, location {:?}",
            describe_id(order, p),
            pkg.kind,
            pkg.conditions.len(),
            if passes { " (pass)" } else { "" },
            pkg.location
        )?;
    }
    let Some(pkg) = current_package(order, &state, reference) else {
        writeln!(out, "Follows no package now.")?;
        return Ok(());
    };
    writeln!(out, "Follows now: {}", describe_id(order, pkg.form_id))?;
    package_actions(out, order, &pkg)?;
    if pkg.kind == world::ai::kinds::SANDBOX {
        sandbox(out, order, &state, reference, &pkg)?;
    }
    let Some((to, radius)) = destination(order, &state, reference, &pkg) else {
        // Somewhere else: through a door.
        let way = world::ai::target_place(order, &state, reference, &pkg)
            .and_then(|(space, _)| world::ai::door_toward(order, &state, reference, space));
        match way {
            Some(d) => writeln!(
                out,
                "It leads to {}: through {} at {:.0},{:.0},{:.0}, coming out at {:.0},{:.0},{:.0}",
                describe_id(order, d.to_space),
                describe_id(order, d.door),
                d.at[0],
                d.at[1],
                d.at[2],
                d.to[0],
                d.to[1],
                d.to[2]
            )?,
            None => writeln!(out, "It doesn't lead anywhere this works out.")?,
        }
        return Ok(());
    };
    let from = world::scripting::whereabouts(order, reference)
        .ok_or_else(|| CliError::NotFound("no position".into()))?;
    writeln!(
        out,
        "From {:.0},{:.0},{:.0} to {:.0},{:.0},{:.0} (within {radius})",
        from.position[0], from.position[1], from.position[2], to[0], to[1], to[2]
    )?;
    let mesh = NavMesh::load(order, from.cell);
    writeln!(
        out,
        "Navmesh: {} vertices, {} triangles",
        mesh.vertices.len(),
        mesh.triangles.len()
    )?;
    match mesh.path(from.position, to) {
        Some(points) => {
            let length: f32 = points
                .windows(2)
                .map(|w| {
                    (0..3)
                        .map(|i| (w[1][i] - w[0][i]).powi(2))
                        .sum::<f32>()
                        .sqrt()
                })
                .sum();
            writeln!(out, "Path ({length:.0} units):")?;
            for p in points {
                writeln!(out, "  {:.0},{:.0},{:.0}", p[0], p[1], p[2])?;
            }
        }
        None => writeln!(out, "No path on the navmesh.")?,
    }
    Ok(())
}

/// Report stored choreography without claiming that the viewer plays it.
fn package_actions(
    out: &mut impl Write,
    order: &LoadOrder,
    pkg: &world::ai::Package,
) -> Result<(), CliError> {
    let idles = world::idles::IdleTree::load(order);
    for (name, action) in [
        ("Begin", &pkg.actions.begin),
        ("End", &pkg.actions.end),
        ("Change", &pkg.actions.change),
    ] {
        writeln!(out, "{name} action:")?;
        if let Some(id) = action.idle {
            writeln!(out, "  idle: {}", describe_id(order, id))?;
            if let Some(idle) = idles.get(id) {
                writeln!(
                    out,
                    "  animation: {} (group {}, loop counts {:?})",
                    idle.model,
                    idle.group(),
                    idle.loops()
                )?;
            }
        } else {
            writeln!(out, "  idle: none")?;
        }
        if let Some(id) = action.topic {
            writeln!(out, "  topic: {}", describe_id(order, id))?;
        }
        writeln!(out, "  compiled script: {} bytes", action.bytecode().len())?;
        if let Some(source) = action.source().filter(|s| !s.trim().is_empty()) {
            for line in source.lines() {
                writeln!(out, "    {line}")?;
            }
        }
    }
    writeln!(
        out,
        "Stored actions only; package action playback is not implemented."
    )?;
    Ok(())
}

/// A sandbox package (`world::sandbox`): its flags, area, the person's
/// energy, what a scan of their cell finds (furniture counted as having
/// markers: the models aren't read here), and how long each activity
/// lasts for them (the middle of its range).
fn sandbox(
    out: &mut impl Write,
    order: &LoadOrder,
    state: &GameState,
    who: esm::FormId,
    pkg: &world::ai::Package,
) -> Result<(), CliError> {
    use world::sandbox::{self as sb, activities};
    let settings = sb::Settings::read(order);
    let flags = sb::package_flags(order, pkg.form_id);
    let energy = sb::energy(order, who);
    let Some((_, cell, here, _)) = state.place(order, who) else {
        return Ok(());
    };
    let center = world::ai::destination(order, state, who, pkg)
        .filter(|_| pkg.location.is_some_and(|l| matches!(l.kind, 0 | 3 | 6)))
        .map_or(here, |(c, _)| c);
    let mut b = sb::Sandbox::new(
        pkg.form_id,
        center,
        pkg.location.map_or(0, |l| l.radius),
        flags,
        energy,
        &settings,
    );
    let names = [
        (sb::flags::NO_EATING, "no eating"),
        (sb::flags::NO_SLEEPING, "no sleeping"),
        (sb::flags::NO_CONVERSATION, "no conversation"),
        (sb::flags::NO_IDLE_MARKERS, "no idle markers"),
        (sb::flags::NO_FURNITURE, "no furniture"),
        (sb::flags::NO_WANDERING, "no wandering"),
    ];
    let said: Vec<&str> = names
        .iter()
        .filter(|(f, _)| flags & f != 0)
        .map(|(_, n)| *n)
        .collect();
    writeln!(
        out,
        "Sandbox: flags {flags:04X} ({}), within {:.0} of {:.0},{:.0},{:.0}, energy {energy}",
        if said.is_empty() {
            "everything allowed".to_string()
        } else {
            said.join(", ")
        },
        b.radius,
        center[0],
        center[1],
        center[2]
    )?;
    let nearby: Vec<sb::Nearby> = order
        .references_in_cell(cell)
        .into_iter()
        .filter(|rr| !rr.entry.header.is_deleted() && !sb::ignored(order, rr.form_id))
        .filter_map(|rr| {
            let r = rr.form_id;
            let kind = sb::kind_of(order, r)?;
            let (_, _, position, _) = state.place(order, r)?;
            Some(sb::Nearby {
                reference: r,
                kind,
                position,
                allowed: sb::may_use(order, state, who, r),
                usable: true,
            })
        })
        .collect();
    let carries_food = state.inventory(order, who).iter().any(|(item, _)| {
        order
            .get(*item)
            .is_some_and(|r| matches!(r.entry.header.kind.as_bytes(), b"INGR" | b"ALCH"))
    });
    b.scan(who, &nearby, carries_food);
    writeln!(out, "  finds {} things to do:", b.candidates.len())?;
    for c in &b.candidates {
        writeln!(
            out,
            "    {}{}",
            activities::name(c.activity),
            c.target
                .map(|t| format!(" {}", describe_id(order, t)))
                .unwrap_or_default()
        )?;
    }
    let owned = nearby.iter().filter(|n| !n.allowed).count();
    if owned > 0 {
        writeln!(out, "  ({owned} nearby owned by others, left out)")?;
    }
    write!(out, "  each lasts (game minutes):")?;
    for a in 0..6u8 {
        write!(
            out,
            " {} {:.1}",
            activities::name(a),
            sb::duration(&settings, a, energy, 0.5)
        )?;
    }
    writeln!(
        out,
        "; wander pause {:.0} s",
        sb::wander_pause(&settings, energy)
    )?;
    Ok(())
}

/// `hostile`: the people and creatures in a cell (or an outdoor square,
/// with its persistent ones), how aggressive they are, how their
/// factions react to the player in a new game, and whether they attack on
/// sight.
pub fn hostile(out: &mut impl Write, order: &LoadOrder, cell: &str) -> Result<(), CliError> {
    let state = GameState::new(order);
    // A person or creature (placed or not) on their own.
    if let Ok(rr) = crate::records::find_record(order, cell) {
        let kind = rr.entry.header.kind;
        if [b"NPC_", b"CREA", b"ACHR", b"ACRE"].contains(&kind.as_bytes()) {
            let who = rr.form_id;
            let player = world::dialogue::PLAYER_REF;
            let names: Vec<String> = world::factions::factions_of(order, &state, who)
                .into_iter()
                .map(|f| describe_id(order, f))
                .collect();
            writeln!(
                out,
                "{}: aggression {}, {:?} to the player{}\n  factions: {}",
                describe_id(order, who),
                world::factions::aggression(order, who),
                world::factions::reaction(order, &state, who, player),
                if world::factions::attacks_on_sight(order, &state, who, player) {
                    ", attacks on sight"
                } else {
                    ""
                },
                names.join(", ")
            )?;
            return Ok(());
        }
    }
    let id = world::find_cells(order, cell)
        .map_err(crate::render_cmd::world_error)?
        .into_iter()
        .next()
        .ok_or_else(|| CliError::NotFound(format!("no cell '{cell}'")))?;
    let loaded = world::load_cell(order, id).map_err(crate::render_cmd::world_error)?;
    let player = world::dialogue::PLAYER_REF;
    for a in &loaded.actors {
        let who = a.form_id;
        let reaction = world::factions::reaction(order, &state, who, player);
        writeln!(
            out,
            "  {:<50} aggression {}  {:?}{}",
            describe_id(order, who),
            world::factions::aggression(order, who),
            reaction,
            if world::factions::attacks_on_sight(order, &state, who, player) {
                "  ATTACKS ON SIGHT"
            } else {
                ""
            }
        )?;
    }
    writeln!(out, "{} people and creatures", loaded.actors.len())?;
    Ok(())
}

/// `sit`: how people sit in a piece of furniture (a `FURN` or a placed
/// one): its markers, each marker's exit and travel, the seat and the
/// seated loop.
pub fn sit(
    out: &mut impl Write,
    order: &LoadOrder,
    assets: &assets::Assets,
    target: &str,
) -> Result<(), CliError> {
    let found = crate::records::find_record(order, target)?;
    let base = world::scripting::base_of(order, found.form_id).unwrap_or(found.form_id);
    let tree = world::idles::IdleTree::load(order);
    for (who, female) in [("a man", false), ("a woman", true)] {
        let Some(s) = preview::furniture::sitting(assets, order, &tree, base, female) else {
            writeln!(
                out,
                "{} has no furniture markers the idle tree knows",
                describe_id(order, base)
            )?;
            return Ok(());
        };
        if !female {
            writeln!(
                out,
                "MNAM {:08X}: {}",
                s.flags,
                if s.flags & world::furniture::BED != 0 {
                    "a bed"
                } else if s.flags & world::furniture::SIT_FURNITURE != 0 {
                    "for sitting"
                } else {
                    "other furniture"
                }
            )?;
            for (i, m) in s.markers.iter().enumerate() {
                let usable = s.flags & (1 << i.min(31)) != 0;
                // The seat the game's settings give from this marker.
                let settings = world::furniture::MarkerSettings::read(order, m.marker);
                let placed =
                    world::furniture::place_markers(std::slice::from_ref(m), [0.0; 3], 0.0, 1.0)[0];
                let (seat, heading) = world::furniture::seat(&placed, &settings, 1.0);
                writeln!(
                    out,
                    "marker {i}: number {} at ({:.1}, {:.1}, {:.1}) heading {:.3}{}; seat ({:.1}, {:.1}, {:.1}) heading {:.3}",
                    m.marker,
                    m.offset[0],
                    m.offset[1],
                    m.offset[2],
                    m.heading,
                    if usable { "" } else { " (not usable)" },
                    seat[0],
                    seat[1],
                    seat[2],
                    heading
                )?;
            }
            for (m, entry, t) in &s.entries {
                writeln!(
                    out,
                    "  entry {m}: {entry}, travel ({:.1}, {:.1})",
                    t[0], t[1]
                )?;
            }
            for (m, exit, t) in &s.exits {
                writeln!(out, "  exit {m}: {exit}, travel ({:.1}, {:.1})", t[0], t[1])?;
            }
        }
        writeln!(
            out,
            "{who} sits at ({:.1}, {:.1}, {:.1}) heading {:.3}, playing {}",
            s.seat[0],
            s.seat[1],
            s.seat[2],
            s.heading,
            s.idle.as_deref().unwrap_or("nothing")
        )?;
    }
    Ok(())
}

/// `hits`: where shots from in front land on a person or creature: their
/// body part data, and a map of what each straight shot meets — their
/// skeleton's bodies (the game's hit shapes, posed in the first frame of
/// their idle) and the part each body's bone gives (`world::body_parts`).
pub fn hits(
    out: &mut impl Write,
    order: &LoadOrder,
    assets: &assets::Assets,
    target: &str,
) -> Result<(), CliError> {
    use world::body_parts::{part, BodyPartData};
    let found = crate::records::find_record(order, target)?;
    let base = world::scripting::base_of(order, found.form_id).unwrap_or(found.form_id);
    let look = world::actor_look(order, base)
        .ok_or_else(|| CliError::NotFound(format!("{target} isn't a person or creature")))?;
    let data = BodyPartData::of(order, found.form_id)
        .ok_or_else(|| CliError::NotFound(format!("{target} has no body part data")))?;
    let read = |path: &str| {
        assets
            .read(&assets::mesh_path(path))
            .ok()
            .flatten()
            .and_then(|b| nif::Nif::parse(b).ok())
    };
    let skeleton = read(&look.skeleton)
        .ok_or_else(|| CliError::NotFound(format!("the skeleton {}", look.skeleton)))?;
    let bones = skeleton.skeleton()?;
    let idle = read(&look.idle)
        .and_then(|n| n.sequences().ok())
        .and_then(|s| s.into_iter().next());
    let pose = nif::posed(
        &bones,
        idle.as_ref(),
        idle.as_ref().map_or(0.0, |s| s.start),
    );
    writeln!(
        out,
        "{} uses {} ({}), skeleton {}, posed by {}",
        describe_id(order, found.form_id),
        describe_id(order, data.form_id),
        data.model.as_deref().unwrap_or("no skeleton named"),
        look.skeleton,
        if idle.is_some() {
            &look.idle
        } else {
            "nothing"
        }
    )?;
    let letter = |p: u8| match p {
        part::TORSO => 'T',
        part::HEAD | part::HEAD2 => 'H',
        part::LEFT_ARM | part::LEFT_ARM2 => 'a',
        part::RIGHT_ARM | part::RIGHT_ARM2 => 'b',
        part::LEFT_LEG..=part::LEFT_LEG3 => 'l',
        part::RIGHT_LEG..=part::RIGHT_LEG3 => 'r',
        part::BRAIN => 'B',
        _ => 'W',
    };
    for p in data.parts.iter().flatten() {
        writeln!(
            out,
            "  {} {:<14} from {:<18} damage ×{}, {}% of health",
            letter(p.part_type),
            p.name,
            p.node,
            p.damage_mult,
            p.health_percent
        )?;
    }
    let Some(ragdoll) = skeleton.ragdoll()? else {
        writeln!(
            out,
            "The skeleton carries no bodies: shots are met by the bounds."
        )?;
        return Ok(());
    };
    let rig = preview::ragdoll::RagdollRig::new(&bones, ragdoll);
    let placement = nif::Transform {
        scale: look.scale,
        ..nif::Transform::IDENTITY
    };
    // The bodies' extent, for the map.
    let (mut lo, mut hi) = ([f32::MAX; 3], [f32::MIN; 3]);
    for (b, offset) in rig.ragdoll.bodies.iter().zip(&rig.offsets) {
        let Some((a, c, r)) = b.capsule else { continue };
        let frame = placement.then_child(&pose[b.bone]).then_child(offset);
        for p in [frame.apply_point(a), frame.apply_point(c)] {
            for k in 0..3 {
                lo[k] = lo[k].min(p[k] - r);
                hi[k] = hi[k].max(p[k] + r);
            }
        }
    }
    writeln!(
        out,
        "Shots from in front (they face north), one every 4 units, top {:.0} to bottom {:.0}; \
         a letter per part (legend above), '.' a body with no part, ' ' a miss:",
        hi[2], lo[2]
    )?;
    let mut counts = std::collections::BTreeMap::new();
    let mut z = hi[2];
    while z >= lo[2] {
        let mut row = String::new();
        // The shooter's left (east) to right.
        let mut x = hi[0];
        while x >= lo[0] {
            let origin = [x, hi[1] + 500.0, z];
            let c = match rig.ray_hit(&pose, &placement, origin, [0.0, -1.0, 0.0]) {
                Some((_, bone)) => match data.part_of_bone(&bones, bone) {
                    Some(p) => {
                        *counts.entry(letter(p)).or_insert(0usize) += 1;
                        letter(p)
                    }
                    None => '.',
                },
                None => ' ',
            };
            row.push(c);
            x -= 4.0;
        }
        writeln!(out, "  {:>4.0} |{row}|", z)?;
        z -= 4.0;
    }
    writeln!(out, "Shots per part: {counts:?}")?;
    Ok(())
}

/// `first-person`: what the player's first-person view is built from with
/// a weapon in hand (or fists), and what of it loads.
pub fn first_person(
    out: &mut impl Write,
    order: &LoadOrder,
    assets: &assets::Assets,
    weapon: Option<&str>,
) -> Result<(), CliError> {
    let loaded = match weapon {
        Some(w) => {
            let id = crate::records::find_record(order, w)?.form_id;
            world::combat::Weapon::load(order, id)
        }
        None => None,
    };
    let weapon = loaded.as_ref().and_then(|w| {
        let model = order
            .get(w.form_id)?
            .record()
            .ok()?
            .get(esm::FourCC::new(b"MODL"))?
            .zstring();
        Some((model, w.animation))
    });
    // The attack and reload it plays, and whether the game has them.
    let animation = loaded.as_ref().map(|w| w.animation);
    let mut animations = vec![world::actor::first_person_attack(
        animation,
        loaded.as_ref().map_or(255, |w| w.attack_animation),
    )];
    if let Some(w) = &loaded {
        animations.push(world::actor::first_person_reload(
            animation,
            w.reload_animation,
        ));
    }
    for a in &animations {
        let path = assets::mesh_path(a);
        let seq = assets
            .read(&path)
            .ok()
            .flatten()
            .and_then(|b| nif::Nif::parse(b).ok())
            .and_then(|n| n.sequences().ok())
            .and_then(|s| s.into_iter().next());
        match seq {
            Some(s) => writeln!(
                out,
                "Animation {path}: {:.2} s, {} bones",
                s.stop - s.start,
                s.tracks.len()
            )?,
            None => writeln!(out, "Animation {path}: not found")?,
        }
    }
    let Some(look) = world::actor::first_person_look(order, false, &[], weapon) else {
        writeln!(out, "The player record or race couldn't be read.")?;
        return Ok(());
    };
    writeln!(out, "Skeleton: {}\nPose: {}", look.skeleton, look.idle)?;
    for p in &look.parts {
        writeln!(
            out,
            "  part {}{}{}",
            p.model,
            p.bone
                .as_ref()
                .map(|b| format!(" (at {b})"))
                .unwrap_or_default(),
            p.skin_texture
                .as_ref()
                .map(|t| format!(", skin {t}"))
                .unwrap_or_default()
        )?;
    }
    let scene = preview::cell::actor_scene(assets, &look);
    let r = &scene.report;
    writeln!(
        out,
        "Built: {} model(s), {} actor(s) drawn; missing models {:?}, unreadable {:?}, missing animations {:?}",
        scene.models.len(),
        r.actors_drawn,
        r.missing_models.keys().collect::<Vec<_>>(),
        r.unreadable_models.keys().collect::<Vec<_>>(),
        r.missing_animations
    )?;
    for m in &scene.models {
        writeln!(out, "  {} pieces", m.meshes.len())?;
        for mesh in &m.meshes {
            writeln!(
                out,
                "    {} ({} triangles, {}), texture {} ({})",
                mesh.name,
                mesh.triangles.len(),
                if mesh.rig.is_some() {
                    "rigged"
                } else {
                    "not rigged"
                },
                mesh.texture_path.as_deref().unwrap_or("none"),
                if mesh.texture.is_some() {
                    "found"
                } else {
                    "not found"
                }
            )?;
        }
    }
    Ok(())
}

/// `levels [LEVEL]`: the experience each level takes, what killing
/// creatures and people of each level gives, and the perks a new
/// character (Intelligence 5) would be offered at a level (2 by default),
/// with its skill points.
pub fn levels(out: &mut impl Write, order: &LoadOrder, level: Option<u16>) -> Result<(), CliError> {
    use world::experience as xp;
    let max = xp::max_level(order);
    writeln!(out, "Experience to reach each level (top level {max}):")?;
    for l in 2..=max {
        write!(out, "  {l:>2}: {:>6}", xp::xp_for_level(order, l))?;
        if (l - 1) % 5 == 0 || l == max {
            writeln!(out)?;
        }
    }
    writeln!(out, "\nKill rewards by the victim's level:")?;
    writeln!(out, "  level  creature  person")?;
    for l in [0, 1, 2, 3, 5, 7, 9, 10, 12, 13, 20] {
        writeln!(
            out,
            "  {l:>5}  {:>8}  {:>6}",
            xp::kill_xp(order, true, l),
            xp::kill_xp(order, false, l)
        )?;
    }
    let level = level.unwrap_or(2).clamp(2, max);
    let mut state = GameState::new(order);
    state.player_level = level - 1;
    state.actor_values.insert(
        (world::dialogue::PLAYER_REF, xp::XP),
        xp::xp_for_level(order, level),
    );
    state.level_up_pending = true;
    let up = xp::level_up(order, &mut state);
    writeln!(
        out,
        "\nLevel {}: {} skill points (Intelligence 5){}",
        up.level,
        up.skill_points,
        if up.perk { ", a perk:" } else { ", no perk." }
    )?;
    if up.perk {
        for c in world::perks::level_up_choices(order, &state) {
            let note = if c.available {
                String::new()
            } else {
                format!("  (from level {}, or its conditions fail)", c.min_level)
            };
            writeln!(
                out,
                "  {:<28} {} rank{}{note}",
                c.name,
                c.ranks,
                if c.ranks == 1 { "" } else { "s" }
            )?;
        }
    }
    Ok(())
}

/// `perks [ENTRY]`: every perk entry point the loaded perks use (or just
/// one), with each perk's entries: rank, function (set / add / multiply …),
/// value, and conditions by tab (0 the perk's holder, then what the entry
/// point is about: the weapon, the target, …).
pub fn perks(out: &mut impl Write, order: &LoadOrder, entry: Option<u8>) -> Result<(), CliError> {
    use world::perks::{entry_points, ENTRY_POINT_NAMES};
    const FUNCTIONS: [&str; 10] = [
        "none",
        "set",
        "add",
        "multiply",
        "add range",
        "add actor value mult",
        "absolute value",
        "negative absolute value",
        "add leveled list",
        "add activate choice",
    ];
    let mut by_entry: Vec<Vec<(String, world::perks::EntryPoint)>> =
        (0..ENTRY_POINT_NAMES.len()).map(|_| Vec::new()).collect();
    let mut perks: Vec<_> = order
        .records_of_type(esm::FourCC::new(b"PERK"))
        .filter(|rr| !rr.entry.header.is_deleted())
        .collect();
    perks.sort_by_key(|rr| rr.form_id.0);
    for rr in perks {
        let name = rr
            .record()
            .ok()
            .and_then(|r| r.editor_id())
            .unwrap_or_else(|| rr.form_id.to_string());
        for e in entry_points(order, rr.form_id) {
            if let Some(list) = by_entry.get_mut(usize::from(e.entry)) {
                list.push((name.clone(), e));
            }
        }
    }
    let used = by_entry.iter().filter(|l| !l.is_empty()).count();
    if entry.is_none() {
        writeln!(
            out,
            "{used} of {} entry points are used by the loaded perks:",
            ENTRY_POINT_NAMES.len()
        )?;
    }
    for (n, list) in by_entry.iter().enumerate() {
        if entry.is_some_and(|e| usize::from(e) != n) || (entry.is_none() && list.is_empty()) {
            continue;
        }
        let mut names: Vec<&str> = list.iter().map(|(name, _)| name.as_str()).collect();
        names.dedup();
        writeln!(
            out,
            "\n{n:>2} {} ({} perk{}):",
            ENTRY_POINT_NAMES[n],
            names.len(),
            if names.len() == 1 { "" } else { "s" }
        )?;
        for (name, e) in list {
            let value = match e.value {
                Some(v) => format!("{v}"),
                None => "-".to_string(),
            };
            writeln!(
                out,
                "  {name} rank {}: {} {value}",
                e.rank + 1,
                FUNCTIONS
                    .get(usize::from(e.function))
                    .copied()
                    .unwrap_or("?")
            )?;
            for (tab, c) in &e.conditions {
                writeln!(
                    out,
                    "      tab {tab}: {}({:08X}, {:08X}) {:?} {}{}{}",
                    c.function_name(),
                    c.param_forms[0].0,
                    c.params[1],
                    c.comparison,
                    c.value,
                    match c.global {
                        Some(g) => format!(" (global {})", describe_id(order, g)),
                        None => String::new(),
                    },
                    if c.or { " OR" } else { "" }
                )?;
            }
        }
    }
    Ok(())
}

/// `barter`: what a merchant sells (their own things and their merchant
/// container's, stocked as a new game would, those their services cover)
/// and the prices a new character sees in the barter menu
/// (`world::barter`).
pub fn barter(out: &mut impl Write, order: &LoadOrder, target: &str) -> Result<(), CliError> {
    let merchant = crate::records::find_record(order, target)?.form_id;
    let goods = world::barter::merchant_container(order, merchant)
        .ok_or_else(|| CliError::NotFound(format!("{target} has no merchant container (XMRC)")))?;
    let mut state = GameState::new(order);
    for h in world::barter::vendor_holders(order, merchant) {
        state.stock(order, h);
    }
    let caps = world::barter::caps(order);
    let facts = world::scripting::Facts {
        order,
        state: &state,
        speaker: None,
    };
    let skill = facts
        .current_actor_value(world::dialogue::PLAYER_REF, 32)
        .unwrap_or(0.0);
    let m = world::barter::multipliers(order, &state);
    let services = world::barter::services(order, merchant);
    writeln!(
        out,
        "{} sells from {} and their own things: {} caps; services {services:#x}; a new character's Barter is {skill} (buying x{:.3}, selling x{:.3})",
        describe_id(order, merchant),
        describe_id(order, goods),
        world::barter::vendor_caps(order, &state, merchant),
        m.0,
        m.1
    )?;
    // The menu's number: whole, or tenths below 1, "--" for nothing.
    let shown = |p: f32| {
        if p < 0.0 {
            "--".to_string()
        } else if p <= 0.0 || p >= 1.0 {
            format!("{p:.0}")
        } else {
            format!("{p:.1}")
        }
    };
    for holder in world::barter::vendor_holders(order, merchant) {
        for (item, n) in state.inventory(order, holder) {
            if item == caps || !world::barter::deals_in(order, services, item) {
                continue;
            }
            let Some(info) = world::items::item_info(order, item) else {
                continue;
            };
            let worth = world::barter::item_value(order, &state, holder, item);
            writeln!(
                out,
                "  {:<40} x{n:<4} worth {:>6}  buy {:>5}  sell {:>5}  ({})",
                info.name,
                shown(worth),
                shown(world::barter::price(
                    order, &state, item, worth, false, m, 0
                )),
                shown(world::barter::price(order, &state, item, worth, true, m, 0)),
                describe_id(order, holder)
            )?;
        }
    }
    Ok(())
}

pub fn play(
    out: &mut impl Write,
    order: &LoadOrder,
    seconds: f32,
    stage: Option<(&str, u16)>,
    show: usize,
    character: Option<&std::path::Path>,
    cell: Option<&str>,
) -> Result<(), CliError> {
    let started = Instant::now();
    let scripts = ScriptCache::default();
    let mut state = GameState::new(order);
    if let Some(cell) = cell {
        let id = order
            .form_by_editor_id(cell)
            .ok_or_else(|| CliError::NotFound(format!("no cell '{cell}'")))?;
        state.player_cell = Some(id);
        state.player_world = None;
        writeln!(out, "The player is in {cell}.")?;
    }
    if let Some(path) = character {
        let text = std::fs::read_to_string(path)
            .map_err(|e| CliError::Usage(format!("can't read {}: {e}", path.display())))?;
        let problems = world::character::apply(order, &scripts, &mut state, &text);
        writeln!(
            out,
            "Character {}: {} problem(s)",
            path.display(),
            problems.len()
        )?;
        for p in problems {
            writeln!(out, "  line {}: {} ({})", p.line, p.text, p.why)?;
        }
        // What it set up happened before; only what follows is news.
        state.events.clear();
    }
    writeln!(
        out,
        "A new game: {} quests running, {} globals",
        state.running.len(),
        state.globals.len()
    )?;
    // Finds editor IDs once, so the time below is the scripts'.
    order.form_by_editor_id("player");
    let indexed = started.elapsed();
    let step = 1.0 / 30.0;
    let steps = (seconds / step).ceil() as usize;
    let mut runner = Runner::new(order, &scripts, &mut state);
    if let Some((quest, n)) = stage {
        let id = order
            .form_by_editor_id(quest)
            .ok_or_else(|| CliError::NotFound(format!("no quest '{quest}'")))?;
        let set = runner.set_stage(id, n);
        writeln!(
            out,
            "SetStage {quest} {n}: {}",
            if set { "set" } else { "not set" }
        )?;
    }
    let ran = Instant::now();
    let mut seen = 0;
    for _ in 0..steps {
        runner.update(step);
        // What waits for the player is answered as by a player who takes
        // the first choice: a message box's first button, the first tag
        // skills, everything else as it is; menus close at once.
        while seen < runner.state.events.len() {
            match runner.state.events[seen].clone() {
                Event::Message { buttons, .. } if !buttons.is_empty() => {
                    runner.state.button = Some(buttons[0].0 as i32);
                }
                Event::CharacterMenu(CharacterMenu::TagSkills { count, .. }) => {
                    runner.state.tag_skills = world::chargen::SHOWN_SKILLS
                        .iter()
                        .take(count as usize)
                        .copied()
                        .collect();
                }
                Event::Menu(m) => runner.menu_mode(m),
                _ => {}
            }
            seen += 1;
        }
    }
    let ran = ran.elapsed();
    writeln!(
        out,
        "{seconds} game seconds in {:.2} s ({:.2} s reading the files first)",
        ran.as_secs_f32(),
        indexed.as_secs_f32()
    )?;

    let mut stages: Vec<_> = state.stages_done.iter().collect();
    stages.sort();
    writeln!(out, "\nStages set ({}):", stages.len())?;
    for (quest, stage) in stages.iter().take(show) {
        writeln!(out, "  {} stage {stage}", describe_id(order, *quest))?;
    }
    let mut lines: Vec<String> = Vec::new();
    for e in &state.events {
        lines.push(match e {
            Event::Message {
                title,
                text,
                buttons,
                ..
            } => format!(
                "message: {}{}{}",
                title.as_ref().map(|t| format!("{t}: ")).unwrap_or_default(),
                text.replace(['\r', '\n'], " "),
                if buttons.is_empty() {
                    String::new()
                } else {
                    let b: Vec<&str> = buttons.iter().map(|(_, l)| l.as_str()).collect();
                    format!(" [{}]", b.join(" | "))
                }
            ),
            Event::Popup { title, text, .. } => format!(
                "box: {}{}",
                title.as_ref().map(|t| format!("{t}: ")).unwrap_or_default(),
                text.replace(['\r', '\n'], " ")
            ),
            Event::CharacterMenu(m) => format!("character menu opened: {m:?}"),
            Event::Barter(m) => format!("trading with {}", describe_id(order, *m)),
            Event::KnockedOut { who } => format!("{} is down", describe_id(order, *who)),
            Event::GotUp { who } => format!("{} gets up", describe_id(order, *who)),
            Event::Flees { who, to } => match to {
                Some(to) => format!(
                    "{} flees to {}",
                    describe_id(order, *who),
                    describe_id(order, *to)
                ),
                None => format!("{} flees", describe_id(order, *who)),
            },
            Event::RecipeMenu { actor, category } => format!(
                "crafting ({}) for {}",
                describe_id(order, *category),
                describe_id(order, *actor)
            ),
            Event::TeammateContainer(m) => {
                format!("trading things with {}", describe_id(order, *m))
            }
            Event::BackUp(m) => format!("{} steps back from the player", describe_id(order, *m)),
            Event::Caravan {
                npc,
                deck,
                difficulty,
                share,
            } => format!(
                "Caravan against {} ({}, difficulty {difficulty}, betting {share})",
                describe_id(order, *npc),
                describe_id(order, *deck)
            ),
            Event::RepairServices(m) => {
                format!("{}'s repair services", describe_id(order, *m))
            }
            Event::TutorialMenu(m) => format!("the tutorial menu: {}", describe_id(order, *m)),
            Event::Casino {
                game,
                casino,
                min_bet,
                max_bet,
                min_winnings,
            } => format!(
                "{game:?} at {} (bets {min_bet} to {max_bet}, least winnings {min_winnings})",
                describe_id(order, *casino)
            ),
            Event::Died { who, by } => format!(
                "{} killed by {}",
                describe_id(order, *who),
                describe_id(order, *by)
            ),
            Event::Talk {
                speaker,
                to,
                topic,
                conversation,
            } => format!(
                "{} {} {}{}",
                describe_id(order, *speaker),
                if *conversation {
                    "starts a conversation with"
                } else {
                    "says a line to"
                },
                if to.0 == 0 {
                    "no one in particular".to_string()
                } else {
                    describe_id(order, *to)
                },
                topic
                    .map(|t| format!(" about {}", describe_id(order, t)))
                    .unwrap_or_default()
            ),
            Event::PlayGroup { who, group, flags } => format!(
                "{} plays the animation group {group} (flags {flags})",
                describe_id(order, *who)
            ),
            Event::PackageAction { who, package, kind } => format!(
                "{} requests package {} {kind:?} action (dispatch pending)",
                describe_id(order, *who),
                describe_id(order, *package)
            ),
            Event::PlayIdle { who, idle } => format!(
                "{} plays the idle {}",
                describe_id(order, *who),
                describe_id(order, *idle)
            ),
            Event::SwapTexture {
                what,
                node,
                texture,
            } => format!(
                "{}'s {node} takes the texture {texture}",
                describe_id(order, *what)
            ),
            Event::SleepWaitMenu { sleep } => {
                format!("the {} menu opens", if *sleep { "sleep" } else { "wait" })
            }
            Event::More(shown) => world::more_functions::describe(order, &state, shown),
            Event::TerminalBack => "the terminal menu goes back a screen".to_string(),
            Event::Enable(r, on) => format!(
                "{} {}",
                describe_id(order, *r),
                if *on { "enabled" } else { "disabled" }
            ),
            Event::PlayerControls(on) => {
                format!("player controls {}", if *on { "on" } else { "off" })
            }
            Event::MoveTo { what, to } => format!(
                "{} moved to {}",
                describe_id(order, *what),
                describe_id(order, *to)
            ),
            Event::Sound(s) => format!("sound {}", describe_id(order, *s)),
            Event::Menu(m) => format!("menu {m} opened"),
            Event::ImageSpace(m, on) => format!(
                "image space modifier {} {}",
                describe_id(order, *m),
                if *on { "applied" } else { "removed" }
            ),
            Event::Video(v) => format!("video {}", v.file),
            Event::Music(m) => format!("music {}", describe_id(order, *m)),
            Event::Weather(w) => match w {
                Some(w) => format!("weather forced to {}", describe_id(order, *w)),
                None => "weather released".to_string(),
            },
            Event::Activate { what, .. } => format!("{} activated", describe_id(order, *what)),
            Event::Journal { quest, text } => format!(
                "journal, {}: {}",
                describe_id(order, *quest),
                text.replace(['\r', '\n'], " ")
            ),
            Event::Objective {
                quest,
                text,
                completed,
            } => format!(
                "objective {}, {}: {text}",
                if *completed { "completed" } else { "shown" },
                describe_id(order, *quest)
            ),
            Event::QuestText(t) => match t {
                world::quest_text::QuestText::Quest { quest, update } => {
                    format!("quest {update:?}: {}", describe_id(order, *quest))
                }
                world::quest_text::QuestText::Custom(c) => {
                    format!("quest text: {} / {}", c.title, c.subtitle)
                }
            },
        });
    }
    writeln!(out, "\nWhat happened ({} events):", lines.len())?;
    for l in lines.iter().take(show) {
        writeln!(out, "  {l}")?;
    }
    let mut unhandled: Vec<_> = state.unhandled.iter().collect();
    unhandled.sort_by_key(|(_, n)| std::cmp::Reverse(**n));
    writeln!(
        out,
        "\nFunctions not carried out yet ({}), by how often scripts called them:",
        unhandled.len()
    )?;
    for (name, n) in unhandled.iter().take(show) {
        writeln!(out, "  {n:>7}  {name}")?;
        if let Some(first) = state.unhandled_first.get(*name) {
            writeln!(out, "           first {first}")?;
        }
    }
    if !state.broken.is_empty() {
        writeln!(out, "\nScripts that didn't parse ({}):", state.broken.len())?;
        for (owner, e) in state.broken.iter().take(show) {
            writeln!(out, "  {}: {e}", describe_id(order, *owner))?;
        }
    }
    Ok(())
}

/// `recipes <CATEGORY> [ITEM:N ...] [AV=VALUE ...]`: what the crafting menu
/// of a category shows a new character (`world::crafting`), after giving
/// items and setting actor values (skills by number, 32 Barter to 45
/// Unarmed).
pub fn recipes(
    out: &mut impl Write,
    order: &LoadOrder,
    category: &str,
    extra: &[String],
) -> Result<(), CliError> {
    use world::crafting;
    use world::dialogue::PLAYER_REF;
    let cat = crate::records::find_record(order, category)?.form_id;
    let mut state = GameState::new(order);
    state.stock(order, PLAYER_REF);
    for e in extra {
        if let Some((av, v)) = e.split_once('=') {
            let av: u16 = av
                .parse()
                .map_err(|_| CliError::Usage(format!("'{av}' isn't an actor value number")))?;
            let v: f64 = v
                .parse()
                .map_err(|_| CliError::Usage(format!("'{v}' isn't a number")))?;
            state.actor_values.insert((PLAYER_REF, av), v);
        } else {
            let (item, n) = e.split_once(':').unwrap_or((e.as_str(), "1"));
            let id = crate::records::find_record(order, item)?.form_id;
            let n: i32 = n
                .parse()
                .map_err(|_| CliError::Usage(format!("'{n}' isn't a count")))?;
            *state.items.entry((PLAYER_REF, id)).or_insert(0) += n;
        }
    }
    let list = crafting::listing(order, &state, PLAYER_REF, cat, None);
    writeln!(
        out,
        "{} ({}): {} recipes listed, {} can be made; filter: {}",
        crafting::category_name(order, cat),
        describe_id(order, cat),
        list.lines.len(),
        list.makeable,
        list.subcategories
            .iter()
            .map(|s| crafting::category_name(order, *s))
            .collect::<Vec<_>>()
            .join(", ")
    )?;
    for line in &list.lines {
        let r = crafting::Recipe::load(order, line.recipe)
            .ok_or_else(|| CliError::NotFound(format!("recipe {} vanished", line.recipe)))?;
        let skill = match r.skill {
            Some(s) => format!(
                "{} {}",
                world::chargen::actor_value_name(order, s),
                r.skill_level
            ),
            None => "no skill".into(),
        };
        let parts = |cs: &[crafting::Component]| {
            cs.iter()
                .map(|c| format!("{} x{}", describe_id(order, c.item), c.count))
                .collect::<Vec<_>>()
                .join(" + ")
        };
        writeln!(
            out,
            "  {:<32} {:>3}  [{}] {} -> {}{}",
            line.text,
            line.makeable,
            skill,
            parts(&r.ingredients),
            parts(&r.outputs),
            if r.conditions.is_empty() {
                String::new()
            } else {
                format!("  ({} conditions)", r.conditions.len())
            }
        )?;
    }
    Ok(())
}
