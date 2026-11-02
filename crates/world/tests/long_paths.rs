//! The long way between navmeshes (`world::ai::navinfo`) on generated
//! records (`testdata::long_paths`): five squares in a row, a navmesh in
//! each, the navmesh info map, a walker in square 0 sent to square 4.

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::long_paths::ids::*;
use world::ai::{self, navinfo};
use world::scripting::{Event, GameState, PackageActionKind};

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::long_paths::long_paths(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

fn middle(i: i32) -> [f32; 3] {
    [i as f32 * 4096.0 + 2048.0, 2048.0, 0.0]
}

fn distance(a: [f32; 3], b: [f32; 3]) -> f32 {
    (0..3).map(|i| (a[i] - b[i]).powi(2)).sum::<f32>().sqrt()
}

#[test]
fn every_plugins_version_of_the_navmesh_info_map_is_read() {
    let (_data, order) = order("long-map");
    // DeadMoney.esm's version (no entries) wins the record; the master's
    // entries still count.
    let map = navinfo::NavInfoMap::load(&order);
    assert_eq!(map.infos.len(), SQUARES as usize);
    let i = map.of_navmesh(FormId(NAVMESH + 2)).unwrap();
    let info = &map.infos[i];
    assert_eq!(info.space, FormId(WORLD));
    // The grid word reads y, then x.
    assert_eq!(info.square, Some((2, 0)));
    assert_eq!(info.position, middle(2));
    assert_eq!(info.links, [FormId(NAVMESH + 1), FormId(NAVMESH + 3)]);
    assert!(info.cheap_links.is_empty() && info.doors.is_empty());
    assert_eq!(map.in_place(FormId(WORLD), Some((2, 0))), [i]);
    assert!(map.in_place(FormId(WORLD), Some((2, 1))).is_empty());
}

#[test]
fn the_long_way_passes_each_navmesh_and_its_attached_run_is_walked_in_detail() {
    let (_data, order) = order("long-route");
    let mut infos = navinfo::NavInfos::default();
    let world = FormId(WORLD);
    let (from, to) = ([1000.0, 1000.0, 0.0], [19000.0, 3000.0, 0.0]);
    let nodes = infos.virtual_path(&order, world, from, to).unwrap();
    let positions: Vec<[f32; 3]> = nodes.iter().map(|n| n.position).collect();
    assert_eq!(positions, [from, middle(1), middle(2), middle(3), to]);
    let squares: Vec<_> = nodes.iter().map(|n| n.square).collect();
    assert_eq!(
        squares,
        [0, 1, 2, 3, 4].map(|x| Some((x, 0))),
        "each node in its navmesh's square"
    );
    // On one navmesh: just the two ends.
    let near = infos
        .virtual_path(&order, world, from, [3000.0, 3000.0, 0.0])
        .unwrap();
    assert_eq!(near.len(), 2);
    // Squares 0 and 1 attached: the detailed path runs to node 1 only.
    let attached = [(0, 0), (1, 0)];
    let in_attached = |n: &navinfo::VirtualNode| n.square.is_some_and(|s| attached.contains(&s));
    assert_eq!(navinfo::attached_run(&nodes, 0, in_attached), Some(1));
    assert_eq!(navinfo::attached_run(&nodes, 0, |_| true), Some(4));
    assert_eq!(navinfo::attached_run(&nodes, 0, |_| false), None);
    let mesh = ai::NavMesh::load_cells(&order, &[FormId(CELL), FormId(CELL + 1)]);
    assert!(
        mesh.path(from, to).is_none(),
        "the goal's navmesh isn't loaded"
    );
    let end = mesh.closest_point(nodes[1].position).unwrap();
    assert!(distance(end, middle(1)) < 1e-3, "{end:?}");
    let path = mesh.path(from, end).unwrap();
    assert!(distance(*path.last().unwrap(), end) < 1e-3);
    assert_eq!(mesh.navmesh_at(from), Some(FormId(NAVMESH)));
    assert_eq!(mesh.navmesh_at(end), Some(FormId(NAVMESH + 1)));
}

#[test]
fn out_of_sight_they_walk_the_long_way_node_to_node() {
    let (_data, order) = order("long-offstage");
    let mut state = GameState::new(&order);
    let me = FormId(WALKER_REF);
    let mut navs = ai::NavCache::default();
    // 2000 units of the first leg, toward square 1's middle.
    assert_eq!(
        ai::move_offstage(&order, &mut state, me, 2000.0, &mut navs),
        ai::Offstage::Moved
    );
    let start = [1000.0, 1000.0, 0.0];
    let leg = distance(start, middle(1));
    let expected = [0, 1, 2].map(|k| start[k] + (middle(1)[k] - start[k]) * 2000.0 / leg);
    let at = state.place(&order, me).unwrap().2;
    assert!(distance(at, expected) < 0.5, "{at:?} vs {expected:?}");
    // 5000 more: about 3250 to square 1's middle, then on toward square
    // 2's.
    ai::move_offstage(&order, &mut state, me, 5000.0, &mut navs);
    let at = state.place(&order, me).unwrap().2;
    let past = 5000.0 - (leg - 2000.0);
    assert!(
        distance(at, [middle(1)[0] + past, 2048.0, 0.0]) < 0.5,
        "{at:?}"
    );
    // The rest: there, within the marker's 20, and the travel's end action
    // asked for once.
    assert_eq!(
        ai::move_offstage(&order, &mut state, me, 20_000.0, &mut navs),
        ai::Offstage::Arrived
    );
    let at = state.place(&order, me).unwrap().2;
    assert!(distance(at, [19000.0, 3000.0, 0.0]) <= 20.0, "{at:?}");
    let ends = state
        .events
        .iter()
        .filter(|e| {
            matches!(
                e,
                Event::PackageAction {
                    kind: PackageActionKind::End,
                    ..
                }
            )
        })
        .count();
    assert_eq!(ends, 1);
}
