//! Triangles keep the Havok material they were added with (what a shot
//! striking them looks and sounds like: `world::impacts`).

use physics::Collider;

#[test]
fn triangles_keep_their_havok_material_through_extend() {
    let quad = [
        [-100.0, -100.0, 0.0],
        [100.0, -100.0, 0.0],
        [100.0, 100.0, 0.0],
        [-100.0, 100.0, 0.0],
    ];
    let wall = quad.map(|[x, y, _]| [x, 0.0, y + 100.0]);
    let mut square = Collider::new();
    square.add_with_material(&quad, &[[0, 1, 2], [0, 2, 3]], 9);
    square.add(&wall, &[[0, 1, 2], [0, 2, 3]]);
    let mut all = Collider::new();
    all.add_with_material(&quad.map(|[x, y, z]| [x, y, z - 50.0]), &[[0, 1, 2]], 5);
    all.extend(&square);
    // Down onto the floor: wood (9), found after the extend.
    let (d, t) = all
        .raycast([10.0, 10.0, 50.0], [0.0, 0.0, -1.0], 500.0)
        .unwrap();
    assert!((d - 50.0).abs() < 1e-3);
    assert_eq!(all.material(t), Some(9));
    // The wall was added without one.
    let (_, t) = all
        .raycast([10.0, -50.0, 120.0], [0.0, 1.0, 0.0], 500.0)
        .unwrap();
    assert_eq!(all.material(t), None);
    // The first collider's own triangle kept its metal (5).
    let (_, t) = all
        .raycast([90.0, -90.0, -10.0], [0.0, 0.0, -1.0], 500.0)
        .unwrap();
    assert_eq!(all.material(t), Some(5));
}
