//! Texture atlases (`.tai`): many small interface pictures packed into one
//! texture. Each line names a picture (by its file name alone) and where it
//! sits in the atlas: `name<TAB>atlas file, index, type, u, v, depth, width,
//! height` (texture coordinates, 0 to 1). A tile with a `texatlas` takes its
//! picture from the atlas this way (`00a06dd0` in FalloutNV.exe looks a
//! picture up by name).

use std::collections::HashMap;

/// One picture's place in an atlas.
#[derive(Debug, Clone, PartialEq)]
pub struct AtlasEntry {
    /// The atlas texture, as written (e.g. `InterfaceShared0.dds`), in the
    /// same folder as the `.tai`.
    pub texture: String,
    pub u: f32,
    pub v: f32,
    pub width: f32,
    pub height: f32,
}

/// A parsed `.tai` file: pictures by lower-case file name.
#[derive(Debug, Clone, Default)]
pub struct Atlas {
    entries: HashMap<String, AtlasEntry>,
}

impl Atlas {
    pub fn parse(text: &str) -> Atlas {
        let mut entries = HashMap::new();
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((name, rest)) = line.split_once(['\t', ' ']) else {
                continue;
            };
            let fields: Vec<&str> = rest.split(',').map(str::trim).collect();
            if fields.len() < 8 {
                continue;
            }
            let num = |i: usize| fields[i].parse::<f32>().ok();
            let (Some(u), Some(v), Some(width), Some(height)) = (num(3), num(4), num(6), num(7))
            else {
                continue;
            };
            entries.insert(
                name.trim().to_ascii_lowercase(),
                AtlasEntry {
                    texture: fields[0].to_string(),
                    u,
                    v,
                    width,
                    height,
                },
            );
        }
        Atlas { entries }
    }

    /// Where a picture is, by its path or file name (only the file name
    /// counts, case ignored).
    pub fn get(&self, path: &str) -> Option<&AtlasEntry> {
        let name = path.rsplit(['\\', '/']).next().unwrap_or(path).trim();
        self.entries.get(&name.to_ascii_lowercase())
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entries_by_file_name() {
        let atlas = Atlas::parse(
            "# comment\n\nglow_crosshair.dds\t\tInterfaceShared0.dds, 0, 2D, 0.812988, 0.750488, 0.000000, 0.061523, 0.061523\n",
        );
        assert_eq!(atlas.len(), 1);
        let e = atlas.get("Interface\\HUD\\Glow_Crosshair.dds").unwrap();
        assert_eq!(e.texture, "InterfaceShared0.dds");
        assert_eq!(
            (e.u, e.v, e.width, e.height),
            (0.812988, 0.750488, 0.061523, 0.061523)
        );
        assert!(atlas.get("missing.dds").is_none());
    }
}
