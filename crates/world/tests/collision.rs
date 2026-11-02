//! What placed references carry for collision: collision markers'
//! primitives (`XPRM`, `XTRI`) and doors placed open (`XACT`).

use esm::{ActivePlugins, FormId, LoadOrder};
use std::path::{Path, PathBuf};

use testdata::{f32s, group, placed, record, stat, sub, zstr};

/// A Data folder in the temp directory, removed afterwards.
struct TempData(PathBuf);

impl Drop for TempData {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

impl TempData {
    fn write(&self, relative: &str, bytes: &[u8]) {
        let path = self.0.join(relative);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, bytes).unwrap();
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

fn plugin(tag: &str) -> TempData {
    let dir = std::env::temp_dir().join(format!("nv-rs-col-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let data = TempData(dir);
    // The engine's collision marker (0x21: no model), a door with one.
    let mut marker = sub(b"EDID", &zstr("CollisionMarker"));
    marker.extend(sub(b"OBND", &[0; 12]));
    let mut statics = record(b"STAT", 0x21, &marker);
    statics.extend(stat(0x800, "Floor", "Test\\Floor.nif"));
    let mut door = sub(b"EDID", &zstr("TestDoor"));
    door.extend(sub(b"MODL", &zstr("Test\\Door.nif")));
    let doors = record(b"DOOR", 0x830, &door);
    // XPRM: half sizes, colour, a float, the shape.
    let primitive = |half: [f32; 3], shape: u32| {
        let mut d = f32s(&half);
        d.extend(f32s(&[1.0, 0.0, 0.0, 0.0]));
        d.extend(shape.to_le_bytes());
        sub(b"XPRM", &d)
    };
    let mut refs = placed(0x901, 0x800, [0.0; 3], [0.0; 3], &[]);
    refs.extend(placed(
        0x902,
        0x21,
        [100.0, 0.0, 0.0],
        [0.0, 0.0, 90.0],
        &primitive([200.0, 1.0, 150.0], 3),
    ));
    let mut layered = primitive([30.0, 40.0, 50.0], 1);
    layered.extend(sub(b"XTRI", &15u32.to_le_bytes()));
    refs.extend(placed(0x903, 0x21, [0.0; 3], [0.0; 3], &layered));
    refs.extend(placed(
        0x904,
        0x830,
        [0.0, 50.0, 0.0],
        [0.0; 3],
        &sub(b"XACT", &0x08u32.to_le_bytes()),
    ));
    refs.extend(placed(0x905, 0x830, [0.0, -50.0, 0.0], [0.0; 3], &[]));
    let mut cell = sub(b"EDID", &zstr("CollisionRoom"));
    cell.extend(sub(b"DATA", &[1]));
    let mut contents = record(b"CELL", 0x900, &cell);
    contents.extend(group(
        0x900u32.to_le_bytes(),
        6,
        &group(0x900u32.to_le_bytes(), 9, &refs),
    ));
    let cells = group(*b"CELL", 0, &group([0; 4], 2, &group([0; 4], 3, &contents)));
    let mut hedr = 1.34f32.to_le_bytes().to_vec();
    hedr.extend([0; 8]);
    let mut esm = record(b"TES4", 0, &sub(b"HEDR", &hedr));
    esm.extend(group(*b"STAT", 0, &statics));
    esm.extend(group(*b"DOOR", 0, &doors));
    esm.extend(cells);
    data.write("FalloutNV.esm", &esm);
    data
}

#[test]
fn reads_collision_markers_and_doors_placed_open() {
    let data = plugin("markers");
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    let cell = world::load_cell(&order, FormId(0x900)).unwrap();
    // Collision markers are editor markers (not drawn), with their
    // primitives: a plane 200 wide and 150 tall, and a box on the layer its
    // `XTRI` names.
    let marker = |id: u32| {
        cell.markers
            .iter()
            .find(|m| m.form_id == FormId(id))
            .unwrap_or_else(|| panic!("{id:X} among {:?}", cell.markers))
    };
    let plane = marker(0x902);
    assert_eq!(plane.base, world::COLLISION_MARKER);
    assert_eq!(
        plane.primitive,
        Some(world::Primitive {
            half: [200.0, 1.0, 150.0],
            shape: 3,
            layer: None
        })
    );
    assert_eq!(marker(0x903).primitive.unwrap().layer, Some(15));
    // Doors: one placed open (`XACT` 0x08), one closed.
    let door = |id: u32| {
        cell.objects
            .iter()
            .find(|o| o.form_id == FormId(id))
            .unwrap()
    };
    assert!(door(0x904).open_by_default);
    assert!(!door(0x905).open_by_default);
    assert_eq!(door(0x904).primitive, None);
}
