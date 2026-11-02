//! What the world asks the viewer for `GetLineOfSight`
//! (`world::sight::Sight`): references' 3D bounds, the camera and its
//! view, and rays through the cell's collision.

use esm::FormId;

/// The camera and collision this frame.
pub struct ViewerSight<'a> {
    pub collision: &'a physics::Collider,
    /// Placed objects' world-space bounds (`scripts::ObjectBounds`).
    pub bounds: Option<&'a crate::scripts::ObjectBounds>,
    /// People in the loaded cells and where their feet are.
    pub people: &'a [(FormId, [f32; 3])],
    /// The camera: position and unit forward and up (game space), the
    /// tangent of half its vertical view angle, and width over height.
    pub eye: [f32; 3],
    pub forward: [f32; 3],
    pub up: [f32; 3],
    pub tan_half_fov: f32,
    pub aspect: f32,
}

fn sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

impl ViewerSight<'_> {
    /// Whether a point is inside the camera's view.
    fn sees_point(&self, p: [f32; 3]) -> bool {
        let v = sub(p, self.eye);
        let depth = dot(v, self.forward);
        if depth <= 0.0 {
            return false;
        }
        let right = cross(self.forward, self.up);
        let half_h = depth * self.tan_half_fov;
        dot(v, self.up).abs() <= half_h && dot(v, right).abs() <= half_h * self.aspect
    }
}

impl world::sight::Sight for ViewerSight<'_> {
    /// A placed object's rendered bounds; a person's: their collision
    /// shape (the game uses their model's bound: a stand-in).
    fn bound(&self, reference: FormId) -> Option<([f32; 3], [f32; 3])> {
        if let Some(b) = self.bounds.and_then(|b| b.0.get(&reference.0)) {
            return Some(*b);
        }
        let (_, feet) = self.people.iter().find(|(r, _)| *r == reference)?;
        let shape = physics::CharacterShape::PLAYER;
        Some((
            [feet[0] - shape.radius, feet[1] - shape.radius, feet[2]],
            [
                feet[0] + shape.radius,
                feet[1] + shape.radius,
                feet[2] + shape.height,
            ],
        ))
    }

    fn camera(&self) -> Option<[f32; 3]> {
        Some(self.eye)
    }

    /// Any corner or the middle of the box inside the view (the game tests
    /// the bound against the camera's frustum).
    fn in_view(&self, lo: [f32; 3], hi: [f32; 3]) -> bool {
        let mid = [
            (lo[0] + hi[0]) * 0.5,
            (lo[1] + hi[1]) * 0.5,
            (lo[2] + hi[2]) * 0.5,
        ];
        self.sees_point(mid)
            || (0..8).any(|i| {
                self.sees_point([
                    if i & 1 == 0 { lo[0] } else { hi[0] },
                    if i & 2 == 0 { lo[1] } else { hi[1] },
                    if i & 4 == 0 { lo[2] } else { hi[2] },
                ])
            })
    }

    fn ray(&self, from: [f32; 3], to: [f32; 3]) -> Option<f32> {
        let d = sub(to, from);
        let length = dot(d, d).sqrt();
        if length < 1e-3 {
            return None;
        }
        let dir = [d[0] / length, d[1] / length, d[2] / length];
        self.collision.raycast(from, dir, length).map(|(t, _)| t)
    }
}
