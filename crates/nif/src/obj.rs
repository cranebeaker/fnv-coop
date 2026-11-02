//! Wavefront OBJ export, for checking meshes in any 3D viewer.

use std::io::{self, Write};

use crate::math::{normalize, Vec3};
use crate::scene::Mesh;

/// Gamebryo is Z-up; OBJ viewers expect Y-up. This is a proper rotation
/// (-90° about X), so triangle winding is preserved.
fn to_y_up(v: Vec3) -> Vec3 {
    [v[0], v[2], -v[1]]
}

fn object_name(mesh: &Mesh, i: usize) -> String {
    let name = if mesh.name.trim().is_empty() {
        format!("mesh{i}")
    } else {
        mesh.name.clone()
    };
    name.chars()
        .map(|c| if c.is_whitespace() { '_' } else { c })
        .collect()
}

/// Writes meshes as one OBJ file, one object per mesh, with every node
/// transform applied. Texture V coordinates are flipped (DirectX-style
/// textures have their origin at the top).
pub fn write_obj(out: &mut impl Write, meshes: &[Mesh], source: &str) -> io::Result<()> {
    writeln!(out, "# {source}")?;
    writeln!(
        out,
        "# Exported by nv-rs. Game units, converted from Z-up to Y-up."
    )?;
    let (mut v_base, mut vt_base, mut vn_base) = (1usize, 1usize, 1usize);
    for (i, mesh) in meshes.iter().enumerate() {
        writeln!(out, "o {}", object_name(mesh, i))?;
        if let Some(texture) = mesh.diffuse_texture() {
            writeln!(out, "# diffuse texture: {texture}")?;
        }
        for p in mesh.model_positions() {
            let [x, y, z] = to_y_up(p);
            writeln!(out, "v {x} {y} {z}")?;
        }
        let has_uvs = !mesh.uvs.is_empty() && mesh.uvs.len() == mesh.positions.len();
        let has_normals = !mesh.normals.is_empty() && mesh.normals.len() == mesh.positions.len();
        if has_uvs {
            for uv in &mesh.uvs {
                writeln!(out, "vt {} {}", uv[0], 1.0 - uv[1])?;
            }
        }
        if has_normals {
            for &n in &mesh.normals {
                let [x, y, z] = to_y_up(normalize(mesh.transform.apply_direction(n)));
                writeln!(out, "vn {x} {y} {z}")?;
            }
        }
        for t in &mesh.triangles {
            write!(out, "f")?;
            for &vertex in t {
                let k = usize::from(vertex);
                match (has_uvs, has_normals) {
                    (true, true) => write!(out, " {}/{}/{}", v_base + k, vt_base + k, vn_base + k)?,
                    (true, false) => write!(out, " {}/{}", v_base + k, vt_base + k)?,
                    (false, true) => write!(out, " {}//{}", v_base + k, vn_base + k)?,
                    (false, false) => write!(out, " {}", v_base + k)?,
                }
            }
            writeln!(out)?;
        }
        v_base += mesh.positions.len();
        if has_uvs {
            vt_base += mesh.uvs.len();
        }
        if has_normals {
            vn_base += mesh.normals.len();
        }
    }
    Ok(())
}
