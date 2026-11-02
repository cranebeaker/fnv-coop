//! What a menu draws: each shown picture and text tile as the game's tile
//! refresh (`00a04640`) builds it: a rectangle in menu units, its texture
//! and texture coordinates, and the colour its tile shader multiplies the
//! texture by (`TILE1000.pso`: texture × `TintColor`; `TILE1001` adds a
//! scrolled texture and an alpha map). Blended source alpha over inverse
//! source alpha on the picture's stored values, back to front by depth
//! (read from a recording of the game's HUD).

use std::collections::BTreeMap;

use crate::atlas::Atlas;
use crate::font::Font;
use crate::names::{kind, t};
use crate::tile::{TileId, Ui};

/// What a picture or font texture needs from the game's files.
pub trait Textures {
    /// A texture's size in texels, by its path under `Data` (e.g.
    /// `textures\interface\hud\hud_tick_mark.dds`).
    fn size(&mut self, path: &str) -> Option<(u32, u32)>;
    /// An atlas, by its path under `Data` (`textures\...\x.tai`).
    fn atlas(&mut self, path: &str) -> Option<Atlas>;
}

/// One thing to draw.
#[derive(Debug, Clone, PartialEq)]
pub struct DrawItem {
    pub tile: TileId,
    /// The tile's depth (higher drawn later, on top).
    pub depth: f32,
    /// `TintColor`: red, green, blue (stored values, 0 to 1) and alpha.
    pub color: [f32; 4],
    pub kind: DrawKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DrawKind {
    /// A picture: `rect` is x, y, width, height in menu units (y down).
    Image {
        texture: String,
        rect: [f32; 4],
        /// u0, v0, u1, v1.
        uv: [f32; 4],
        /// A `tile` trait other than 0: the texture repeats across
        /// (`WRAP_S_CLAMP_T`, read at `00a054fc`); otherwise it's clamped.
        repeat_u: bool,
        /// Scrolling and an alpha map (`TILE1001`, the compass): the
        /// texture coordinates are scaled by `scroll.zw` and moved by
        /// `scroll.xy`, wrapping both ways, and the alpha is multiplied by
        /// the alpha map's red at the unscrolled coordinates.
        scroll: Option<([f32; 4], String)>,
    },
    /// Text: glyph rectangles (x, y, width, height in menu units) with
    /// their texture coordinates and which of the font's pictures.
    Text {
        font: usize,
        glyphs: Vec<([f32; 4], [[f32; 2]; 4], u32)>,
    },
    /// A `nif` tile's model piece (`Tile3D` (Xbox PDB)) laid flat on the
    /// screen: triangles of (x, y) in menu units and (u, v); no texture is
    /// white; blended `src` × source + `dst` × destination with the NIF's
    /// `NiAlphaProperty` factors (Gamebryo's numbering: 0 one, 1 zero,
    /// 2 source colour, 3 one minus it, 4 destination colour, 5 one minus
    /// it, 6 source alpha, 7 one minus it), or without blending.
    Model {
        texture: Option<String>,
        triangles: Vec<[([f32; 2], [f32; 2]); 3]>,
        /// Each corner's alpha (empty: all 1): the local map's fog of war.
        alpha: Vec<[f32; 3]>,
        blend: Option<(u8, u8)>,
    },
}

/// A triangle's corner: (x, y) in menu units, (u, v), alpha.
pub type Corner = ([f32; 2], [f32; 2], f32);

/// Triangles cut to a rectangle (x, y, width, height; Sutherland–Hodgman
/// on each edge, the corners' texture coordinates and alpha carried
/// along), fanned back into triangles.
pub fn clip_triangles(tris: &[[Corner; 3]], r: [f32; 4]) -> Vec<[Corner; 3]> {
    let lerp = |a: Corner, b: Corner, k: f32| -> Corner {
        (
            [
                a.0[0] + (b.0[0] - a.0[0]) * k,
                a.0[1] + (b.0[1] - a.0[1]) * k,
            ],
            [
                a.1[0] + (b.1[0] - a.1[0]) * k,
                a.1[1] + (b.1[1] - a.1[1]) * k,
            ],
            a.2 + (b.2 - a.2) * k,
        )
    };
    let edges: [(usize, f32, f32); 4] = [
        (0, r[0], 1.0),
        (0, r[0] + r[2], -1.0),
        (1, r[1], 1.0),
        (1, r[1] + r[3], -1.0),
    ];
    let mut out = Vec::new();
    for tri in tris {
        let mut poly: Vec<Corner> = tri.to_vec();
        for &(axis, at, sign) in &edges {
            let d = |c: &Corner| (c.0[axis] - at) * sign;
            let mut next = Vec::with_capacity(poly.len() + 2);
            for i in 0..poly.len() {
                let (a, b) = (poly[i], poly[(i + 1) % poly.len()]);
                let (da, db) = (d(&a), d(&b));
                if da >= 0.0 {
                    next.push(a);
                }
                if (da >= 0.0) != (db >= 0.0) {
                    next.push(lerp(a, b, da / (da - db)));
                }
            }
            poly = next;
            if poly.len() < 3 {
                break;
            }
        }
        for i in 1..poly.len().saturating_sub(1) {
            out.push([poly[0], poly[i], poly[i + 1]]);
        }
    }
    out
}

/// Where a texture named in a tile's `filename` is (`Data\Textures\` +
/// the name, `00a1f7d0`).
pub fn texture_path(filename: &str) -> String {
    let f = filename.trim().replace('/', "\\");
    let lower = f.to_ascii_lowercase();
    if lower.starts_with("textures\\") {
        lower
    } else {
        format!("textures\\{lower}")
    }
}

/// A tile's colour as `00a06925` works it out: alpha / 255; with a
/// brightness b >= 0, the system colour named by the nearest tile (itself
/// first) with a `systemcolor` trait or id 111 (`&noglow_branch;`) times b
/// / 255 (a direct child of a menu with id 110 takes its own colour, else
/// its menu's, else colour 1); with no such colour, or b = -1, red, green,
/// blue / 255. A tile without a brightness reads 0: black.
pub fn tile_color(ui: &mut Ui, tile: TileId) -> [f32; 4] {
    let alpha = ui.number(tile, t::ALPHA) / 255.0;
    let mut b = ui.number(tile, t::BRIGHTNESS);
    if b >= 0.0 {
        b /= 255.0;
    }
    let mut rgb = [1.0f32; 3];
    if b >= 0.0 {
        let mut index = 0;
        let mut at = Some(tile);
        while let Some(c) = at {
            if ui.has(c, t::SYSTEMCOLOR) || ui.number(c, t::ID) == 111.0 {
                index = ui.number(c, t::SYSTEMCOLOR) as i32;
                break;
            }
            let parent = ui.tiles[c].parent;
            let grand = parent.and_then(|p| ui.tiles[p].parent);
            let menu_child = grand.is_some_and(|g| ui.tiles[g].parent.is_none());
            if menu_child && ui.number(c, t::ID) == 110.0 {
                index = ui.number(c, t::SYSTEMCOLOR) as i32;
                if index == 0 {
                    index = parent.map_or(0, |p| ui.number(p, t::SYSTEMCOLOR) as i32);
                }
                if index == 0 {
                    index = 1;
                }
                break;
            }
            at = parent;
        }
        match ui.colors.get(index) {
            Some(c) => rgb = c.map(|v| v * b),
            None => b = -1.0,
        }
    }
    if b == -1.0 {
        rgb = [
            ui.number(tile, t::RED) / 255.0,
            ui.number(tile, t::GREEN) / 255.0,
            ui.number(tile, t::BLUE) / 255.0,
        ];
    }
    // A failed skill check's look (`00a06c03`): the list's highlight box
    // dark red (0.545, 0, 0) at alpha 0.2, its edges (children of a tile
    // named `lb_highlight_box`) dark red at alpha 1.
    if ui.tiles[tile].failed {
        let is_box = |ui: &Ui, t_: TileId| ui.tiles[t_].name == "lb_highlight_box";
        if is_box(ui, tile) {
            return [0.545, 0.0, 0.0, 0.2];
        }
        if ui.tiles[tile].parent.is_some_and(|p| is_box(ui, p)) {
            return [0.545, 0.0, 0.0, 1.0];
        }
    }
    [rgb[0], rgb[1], rgb[2], alpha]
}

/// An image tile's texture and coordinates (`00a04640`, flags 0x20 and
/// 0x10):
///
/// * the texture: the `texatlas`'s picture when it lists the file name,
///   else `Data\Textures\` + `filename`;
/// * the size the texture is shown at: `tile` -1 (`&scale;`): the tile's
///   height and the texture's width in proportion; else a negative zoom
///   (`&scale;` is -1): the tile's size (stretched); zoom >= 0 (0 meaning
///   100): its texel size × zoom / 100;
/// * u0 = `cropx` / shown width, v0 likewise; the coordinates run tile
///   size / shown size (beyond 1 the texture repeats or clamps);
/// * from an atlas: the coordinates map into the atlas rectangle, which
///   spans the whole tile, unless zoom > 0, when they run tile size /
///   atlas size (texels 1:1).
pub fn image_geometry(
    ui: &mut Ui,
    tile: TileId,
    textures: &mut dyn Textures,
) -> Option<(String, [f32; 4], [f32; 4])> {
    let filename = ui.string(tile, t::FILENAME).unwrap_or_default();
    if filename.trim().is_empty() {
        return None;
    }
    let (x, y) = ui.screen_position(tile);
    let w = ui.number(tile, t::WIDTH);
    let h = ui.number(tile, t::HEIGHT);
    let zoom = ui.number(tile, t::ZOOM);
    let crop_x = ui.number(tile, t::CROPX);
    let crop_y = ui.number(tile, t::CROPY);
    let atlas_path = ui.string(tile, t::TEXATLAS).unwrap_or_default();
    let mut entry = None;
    if !atlas_path.trim().is_empty() {
        let path = texture_path(&atlas_path);
        if let Some(atlas) = textures.atlas(&path) {
            if let Some(e) = atlas.get(&filename) {
                let folder = path.rsplit_once('\\').map_or("", |(f, _)| f);
                let texture = format!("{folder}\\{}", e.texture.to_ascii_lowercase());
                entry = Some((texture, e.clone()));
            }
        }
    }
    let mut texture = match &entry {
        Some((path, _)) => path.clone(),
        None => texture_path(&filename),
    };
    // A picture that can't be found draws `sMissingImage` instead
    // (`00a1f7d0`).
    if entry.is_none() && textures.size(&texture).is_none() {
        if let Some(missing) = ui.setting_text("sMissingImage") {
            let m = missing.trim().replace('/', "\\").to_ascii_lowercase();
            texture = m.strip_prefix("data\\").map_or(m.clone(), str::to_string);
        }
    }
    let (tw, th) = textures.size(&texture)?;
    let (tw, th) = (tw as f32, th as f32);
    let z = if zoom == 0.0 { 100.0 } else { zoom };
    let (shown_w, shown_h) = if ui.number(tile, t::TILE) == -1.0 {
        (tw * (h / th), h)
    } else if zoom < 0.0 {
        (w, h)
    } else {
        (z / 100.0 * tw, z / 100.0 * th)
    };
    let mut u0 = crop_x / shown_w;
    let mut v0 = crop_y / shown_h;
    let mut du = w / shown_w;
    let mut dv = h / shown_h;
    if let Some((_, e)) = &entry {
        if e.width > 0.0 && e.height > 0.0 {
            u0 = u0 * e.width + e.u;
            v0 = v0 * e.height + e.v;
            du = e.width;
            dv = e.height;
        }
        if zoom > 0.0 {
            // `filewidth` is the atlas texture's width × zoom / 100.
            du = w / (tw * zoom / 100.0) * (zoom / 100.0);
            dv = h / (th * zoom / 100.0) * (zoom / 100.0);
        }
    }
    Some((texture, [x, y, w, h], [u0, v0, u0 + du, v0 + dv]))
}

/// The picture an image tile shows: its path, and the atlas entry when it
/// comes from an atlas (the path is then the atlas page's).
fn picture_of(
    ui: &mut Ui,
    tile: TileId,
    textures: &mut dyn Textures,
) -> Option<(String, Option<crate::atlas::AtlasEntry>)> {
    let filename = ui.string(tile, t::FILENAME).unwrap_or_default();
    if filename.trim().is_empty() {
        return None;
    }
    let atlas_path = ui.string(tile, t::TEXATLAS).unwrap_or_default();
    if !atlas_path.trim().is_empty() {
        let path = texture_path(&atlas_path);
        if let Some(atlas) = textures.atlas(&path) {
            if let Some(e) = atlas.get(&filename) {
                let folder = path.rsplit_once('\\').map_or("", |(f, _)| f);
                let texture = format!("{folder}\\{}", e.texture.to_ascii_lowercase());
                return Some((texture, Some(e.clone())));
            }
        }
    }
    Some((texture_path(&filename), None))
}

/// Sets every picture tile's `filewidth` and `fileheight` from its picture
/// under `root`, as the game's tile refresh does when a picture loads
/// (`00a04640`, its flag 0x20: the `tile` trait read at `00a04e5b`, the
/// zoom at `00a04dc5`):
///
/// * `tile` -1: the picture's own size;
/// * else a zoom of 0 or more (0 meaning 100): the picture's size × zoom /
///   100;
/// * else (a negative zoom, `&scale;`) they're left as they are: the DATA
///   menu's world map gives its own 2048 × 2048 for its 1024 picture.
///
/// A picture in an atlas counts as the atlas page's size (the game divides
/// by a factor its loader returns for atlas pictures, taken as 1 here: a
/// guess). The menu files size pictures from these (`<width><copy
/// src="me()" trait="filewidth"/></width>`), so this runs before drawing.
/// Unchanged values aren't set again (setting one starts the traits'
/// working out afresh).
pub fn update_file_sizes(ui: &mut Ui, root: TileId, textures: &mut dyn Textures) {
    let mut stack = vec![root];
    while let Some(tile) = stack.pop() {
        stack.extend(ui.tiles[tile].children.iter().copied());
        if !matches!(
            ui.tiles[tile].kind,
            kind::IMAGE | kind::HOTRECT | kind::RADIAL
        ) {
            continue;
        }
        let Some((texture, _)) = picture_of(ui, tile, textures) else {
            continue;
        };
        let Some((tw, th)) = textures.size(&texture) else {
            continue;
        };
        let (tw, th) = (tw as f32, th as f32);
        let size = if ui.number(tile, t::TILE) == -1.0 {
            Some((tw, th))
        } else {
            let zoom = ui.number(tile, t::ZOOM);
            (zoom >= 0.0).then(|| {
                let z = if zoom == 0.0 { 100.0 } else { zoom };
                (z / 100.0 * tw, z / 100.0 * th)
            })
        };
        let Some((w, h)) = size else {
            continue;
        };
        if ui.number(tile, t::FILEWIDTH) != w || !ui.has(tile, t::FILEWIDTH) {
            ui.set_number(tile, t::FILEWIDTH, w);
        }
        if ui.number(tile, t::FILEHEIGHT) != h || !ui.has(tile, t::FILEHEIGHT) {
            ui.set_number(tile, t::FILEHEIGHT, h);
        }
    }
}

/// Whether a tile clips: its own `clips`, or, without one, its parent's
/// when the parent is a picture or text (`00a037e0`: setting `clips` on an
/// image, hot rectangle or text tile sets it on every child too). Tiles
/// that don't clip aren't cut, even inside a clip window.
pub fn clips(ui: &mut Ui, tile: TileId) -> bool {
    if ui.has(tile, t::CLIPS) {
        return ui.number(tile, t::CLIPS) != 0.0;
    }
    match ui.tiles[tile].parent {
        Some(p)
            if matches!(
                ui.tiles[p].kind,
                kind::IMAGE | kind::HOTRECT | kind::TEXT | kind::RADIAL
            ) =>
        {
            clips(ui, p)
        }
        _ => false,
    }
}

/// The rectangle a tile's drawing is cut to: a tile that clips
/// ([`clips`]) is cut to its nearest ancestor with `clipwindow` set (x, y,
/// width, height on the screen, menu units). The game builds such tiles'
/// shapes as clipped ones (`00a1f6e0` checks `clips`; the refresh's flag
/// 0x100 cuts them); drawing uses it on whole pixels ([`pixel_clip`]).
pub fn clip_rect(ui: &mut Ui, tile: TileId) -> Option<[f32; 4]> {
    if !clips(ui, tile) {
        return None;
    }
    let mut at = ui.tiles[tile].parent;
    while let Some(p) = at {
        if ui.number(p, t::CLIPWINDOW) != 0.0 {
            let (x, y) = ui.screen_position(p);
            let w = ui.number(p, t::WIDTH);
            let h = ui.number(p, t::HEIGHT);
            return Some([x, y, w, h]);
        }
        at = ui.tiles[p].parent;
    }
    None
}

/// The clip window a tile is drawn cut to, in menu units on whole pixels:
/// [`clip_rect`] as the game's scissor (`00a04640` refresh flag 0x80: the
/// window's rectangle in pixels, position and position + size × pixels
/// per unit; `00a07a40` applies it as a scissor to every tile below it
/// that clips, x and y at least 0, right and bottom at most the screen;
/// `00a037e0` flag 0x100 looks up the nearest window).
pub fn pixel_clip(ui: &mut Ui, tile: TileId) -> Option<[f32; 4]> {
    let [x, y, w, h] = clip_rect(ui, tile)?;
    let screen = ui.screen_size;
    let k = screen.height_px as f32 / screen.height();
    // Whole pixels, the float-to-integer conversion truncating (`00ec62c0`).
    let px = |v: f32| (v * k).trunc() / k;
    let (x0, y0) = (px(x.max(0.0)), px(y.max(0.0)));
    let (x1, y1) = (
        px((x + w).min(screen.width())),
        px((y + h).min(screen.height())),
    );
    Some([x0, y0, x1 - x0, y1 - y0])
}

/// A quad cut to a rectangle, its texture coordinates (top-left,
/// top-right, bottom-left, bottom-right) moved in proportion; `None` when
/// nothing of it is left.
pub fn clip_quad(
    rect: [f32; 4],
    uv: [[f32; 2]; 4],
    clip: [f32; 4],
) -> Option<([f32; 4], [[f32; 2]; 4])> {
    let [x, y, w, h] = rect;
    let x0 = x.max(clip[0]);
    let y0 = y.max(clip[1]);
    let x1 = (x + w).min(clip[0] + clip[2]);
    let y1 = (y + h).min(clip[1] + clip[3]);
    if x1 <= x0 || y1 <= y0 || w <= 0.0 || h <= 0.0 {
        return None;
    }
    let at = |s: f32, t_: f32| -> [f32; 2] {
        let top = [
            uv[0][0] + (uv[1][0] - uv[0][0]) * s,
            uv[0][1] + (uv[1][1] - uv[0][1]) * s,
        ];
        let bottom = [
            uv[2][0] + (uv[3][0] - uv[2][0]) * s,
            uv[2][1] + (uv[3][1] - uv[2][1]) * s,
        ];
        [
            top[0] + (bottom[0] - top[0]) * t_,
            top[1] + (bottom[1] - top[1]) * t_,
        ]
    };
    let (s0, s1) = ((x0 - x) / w, (x1 - x) / w);
    let (t0, t1) = ((y0 - y) / h, (y1 - y) / h);
    Some((
        [x0, y0, x1 - x0, y1 - y0],
        [at(s0, t0), at(s1, t0), at(s0, t1), at(s1, t1)],
    ))
}

/// Everything a menu draws, back to front: its shown image and text tiles
/// with any alpha (fully transparent ones change nothing on screen), by
/// depth, ties in tree order, each cut to its clip window
/// ([`clip_rect`]). `scroll` gives a tile its `TILE1001` scroll and alpha
/// map (set by code: the HUD's compass).
pub fn draw_list(
    ui: &mut Ui,
    menu: TileId,
    textures: &mut dyn Textures,
    scroll: &dyn Fn(TileId) -> Option<([f32; 4], String)>,
) -> Vec<DrawItem> {
    let mut order = Vec::new();
    let mut stack = vec![menu];
    while let Some(tile) = stack.pop() {
        order.push(tile);
        for &c in ui.tiles[tile].children.iter().rev() {
            stack.push(c);
        }
    }
    let mut items = Vec::new();
    for tile in order {
        let k = ui.tiles[tile].kind;
        // A radial tile (`RadialTile`, vtable `01095750`) draws as the
        // image it is.
        if !matches!(k, kind::IMAGE | kind::HOTRECT | kind::TEXT | kind::RADIAL) || !ui.shown(tile)
        {
            continue;
        }
        let color = tile_color(ui, tile);
        if color[3] <= 0.0 {
            continue;
        }
        let depth = ui.screen_depth(tile);
        let clip = pixel_clip(ui, tile);
        if k == kind::TEXT {
            let Some(layout) = ui.layout(tile) else {
                continue;
            };
            let (x, y) = ui.screen_position(tile);
            // HTML text can mix fonts: one item for each (`00a19060` makes
            // a shape for each font), in font order.
            let mut by_font: BTreeMap<usize, Vec<_>> = BTreeMap::new();
            for (i, q) in layout.quads.iter().enumerate() {
                if q.right <= q.left {
                    continue;
                }
                let rect = [x + q.left, y + q.top, q.right - q.left, q.bottom - q.top];
                let (rect, uv) = match clip {
                    Some(c) => match clip_quad(rect, q.uv, c) {
                        Some(r) => r,
                        None => continue,
                    },
                    None => (rect, q.uv),
                };
                by_font
                    .entry(layout.quad_font(i))
                    .or_default()
                    .push((rect, uv, q.texture));
            }
            for (font, glyphs) in by_font {
                items.push(DrawItem {
                    tile,
                    depth,
                    color,
                    kind: DrawKind::Text { font, glyphs },
                });
            }
            // An HTML text's pictures (`00a1a800`): the file as a
            // picture tile's, its texels one to one.
            for p in &layout.pictures {
                let texture = texture_path(&p.file);
                let Some((tw, th)) = textures.size(&texture) else {
                    continue;
                };
                let rect = [x + p.rect[0], y + p.rect[1], p.rect[2], p.rect[3]];
                let uv = [
                    0.0,
                    0.0,
                    p.rect[2] / tw.max(1) as f32,
                    p.rect[3] / th.max(1) as f32,
                ];
                let (rect, uv) = match clip {
                    Some(c) => {
                        let corners = [
                            [uv[0], uv[1]],
                            [uv[2], uv[1]],
                            [uv[0], uv[3]],
                            [uv[2], uv[3]],
                        ];
                        let Some((r, q)) = clip_quad(rect, corners, c) else {
                            continue;
                        };
                        (r, [q[0][0], q[0][1], q[3][0], q[3][1]])
                    }
                    None => (rect, uv),
                };
                items.push(DrawItem {
                    tile,
                    depth,
                    color,
                    kind: DrawKind::Image {
                        texture,
                        rect,
                        uv,
                        repeat_u: false,
                        scroll: None,
                    },
                });
            }
            continue;
        }
        let Some((texture, rect, uv)) = image_geometry(ui, tile, textures) else {
            continue;
        };
        // `rotateangle` (radians [guess: the unit and the turn's sense
        // aren't traced; the local map's arrow, 0079dbb0, fits them]) about
        // (`rotateaxisx`, `rotateaxisy`) from the tile's corner: the picture
        // as two turned triangles, cut to its clip window.
        let angle = ui.number(tile, t::ROTATEANGLE);
        if angle != 0.0 {
            let (ax, ay) = (
                rect[0] + ui.number(tile, 4091),
                rect[1] + ui.number(tile, 4092),
            );
            let (s, c) = angle.sin_cos();
            let turn = |x: f32, y: f32| {
                let (dx, dy) = (x - ax, y - ay);
                [ax + c * dx + s * dy, ay - s * dx + c * dy]
            };
            let [x0, y0, w, h] = rect;
            let corners = [
                (turn(x0, y0), [uv[0], uv[1]], 1.0),
                (turn(x0 + w, y0), [uv[2], uv[1]], 1.0),
                (turn(x0, y0 + h), [uv[0], uv[3]], 1.0),
                (turn(x0 + w, y0 + h), [uv[2], uv[3]], 1.0),
            ];
            let mut tris = vec![
                [corners[0], corners[1], corners[2]],
                [corners[1], corners[3], corners[2]],
            ];
            if let Some(c) = clip_rect(ui, tile) {
                tris = clip_triangles(&tris, c);
            }
            items.push(DrawItem {
                tile,
                depth,
                color,
                kind: DrawKind::Model {
                    texture: Some(texture),
                    triangles: tris.iter().map(|t| t.map(|c| (c.0, c.1))).collect(),
                    alpha: Vec::new(),
                    blend: None,
                },
            });
            continue;
        }
        let (rect, uv) = match clip {
            Some(c) => {
                let corners = [
                    [uv[0], uv[1]],
                    [uv[2], uv[1]],
                    [uv[0], uv[3]],
                    [uv[2], uv[3]],
                ];
                let Some((r, q)) = clip_quad(rect, corners, c) else {
                    continue;
                };
                (r, [q[0][0], q[0][1], q[3][0], q[3][1]])
            }
            None => (rect, uv),
        };
        let repeat_u = ui.number(tile, t::TILE) != 0.0;
        items.push(DrawItem {
            tile,
            depth,
            color,
            kind: DrawKind::Image {
                texture,
                rect,
                uv,
                repeat_u,
                scroll: scroll(tile),
            },
        });
    }
    items.sort_by(|a, b| {
        a.depth
            .partial_cmp(&b.depth)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    items
}

/// A font's picture paths (`textures\fonts\<name>.tex`).
pub fn font_textures(font: &Font) -> Vec<String> {
    font.textures
        .iter()
        .map(|n| Font::texture_path(&n.to_ascii_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tile::{Screen, SystemColors};

    struct Fake;
    impl Textures for Fake {
        fn size(&mut self, path: &str) -> Option<(u32, u32)> {
            match path {
                "textures\\interface\\hud\\hud_tick_mark.dds" => Some((8, 32)),
                "textures\\interface\\interfaceshared0.dds" => Some((1024, 1024)),
                _ => None,
            }
        }
        fn atlas(&mut self, path: &str) -> Option<Atlas> {
            (path == "textures\\interface\\interfaceshared.tai").then(|| {
                Atlas::parse(
                    "glow_hud_comp_direction_strip.dds\tInterfaceShared0.dds, 0, 2D, 0.000488, 0.000488, 0.000000, 0.999023, 0.061523\n\
                     glow_hud_left_seperatorglow.dds\tInterfaceShared0.dds, 0, 2D, 0.000488, 0.125488, 0.000000, 0.249023, 0.124023\n",
                )
            })
        }
    }

    fn ui() -> Ui {
        Ui::new(
            Screen {
                width_px: 1920,
                height_px: 1080,
                safe_x: 15.0,
                safe_y: 15.0,
            },
            SystemColors::new(None, None),
            Box::new(|_| None),
        )
    }

    /// The recorded frame's HUD (`frame_hud.txt`, Doc Mitchell's house):
    /// the HP meter's quad 296 x 20 with u 0..37, v 0..0.625; the compass
    /// 345 x 64 with u 0.00049..0.3374; the bracket 512 x 256 over its
    /// atlas rectangle; the meter's tint the HUD colour × 175 / 255.
    #[test]
    fn pictures_as_the_recording_drew_them() {
        let mut ui = ui();
        let m = ui
            .load_menu(
                b"<menu name=\"m\"><locus>&true;</locus><rect name=\"hp\"><id>&noglow_branch;</id><systemcolor>&hudmain;</systemcolor><locus>&true;</locus><x>40</x><y>803</y>
                  <image name=\"meter\"><filename> Interface\\HUD\\hud_tick_mark.dds </filename><tile>&true;</tile><height>20</height><width>296</width><x>20</x><y>41</y><brightness>175</brightness></image>
                  <image name=\"compass\"><filename> Interface\\HUD\\glow_hud_comp_direction_strip.dds </filename><texatlas> Interface\\InterfaceShared.tai </texatlas><width>345</width><height>64</height><zoom>100</zoom><x>20</x><y>65</y></image>
                  <image name=\"bracket\"><filename>interface\\HUD\\glow_hud_left_seperatorglow.dds</filename><texatlas> Interface\\InterfaceShared.tai </texatlas><width>512</width><height>256</height><depth>-1</depth></image>
                  </rect></menu>",
                &mut |_| None,
            )
            .unwrap();
        // Menus start hidden (the HUD's code shows its own).
        ui.set_number(m, t::VISIBLE, 1.0);
        let meter = ui.find(m, "meter").unwrap();
        let (tex, rect, uv) = image_geometry(&mut ui, meter, &mut Fake).unwrap();
        assert_eq!(tex, "textures\\interface\\hud\\hud_tick_mark.dds");
        assert_eq!(rect, [60.0, 844.0, 296.0, 20.0]);
        assert_eq!(uv, [0.0, 0.0, 37.0, 0.625]);
        let c = tile_color(&mut ui, meter);
        assert!((c[0] - 175.0 / 255.0).abs() < 1e-6);
        assert!((c[1] - 182.0 / 255.0 * 175.0 / 255.0).abs() < 1e-6);
        assert_eq!(c[3], 1.0);
        let compass = ui.find(m, "compass").unwrap();
        let (tex, rect, uv) = image_geometry(&mut ui, compass, &mut Fake).unwrap();
        assert_eq!(tex, "textures\\interface\\interfaceshared0.dds");
        assert_eq!(rect, [60.0, 868.0, 345.0, 64.0]);
        assert!((uv[0] - 0.000488).abs() < 1e-6);
        assert!((uv[2] - (0.000488 + 345.0 / 1024.0)).abs() < 1e-6);
        assert!((uv[3] - (0.000488 + 64.0 / 1024.0)).abs() < 1e-6);
        let bracket = ui.find(m, "bracket").unwrap();
        let (_, _, uv) = image_geometry(&mut ui, bracket, &mut Fake).unwrap();
        assert!((uv[2] - (0.000488 + 0.249023)).abs() < 1e-6);
        let list = draw_list(&mut ui, m, &mut Fake, &|_| None);
        // The bracket (depth -1) first.
        assert_eq!(list.len(), 3);
        assert_eq!(list[0].tile, bracket);
    }

    /// `00712450`: a fading menu's pictures at its alpha; a `disablefade`
    /// tile and what's below it only at full alpha (the interface's fade,
    /// `crate::fade`: `recursive_fade` on the tiles, `fade_items` on the
    /// pictures).
    #[test]
    fn a_fading_menu_is_drawn_at_its_alpha() {
        use crate::fade::{fade_items, recursive_fade};
        let mut ui = ui();
        let m = ui
            .load_menu(
                b"<menu name=\"m\"><image name=\"a\"><filename>Interface\\HUD\\hud_tick_mark.dds</filename><width>8</width><height>8</height><alpha>128</alpha></image>
                  <rect name=\"still\"><disablefade>&true;</disablefade>
                    <image name=\"b\"><filename>Interface\\HUD\\hud_tick_mark.dds</filename><width>8</width><height>8</height></image></rect></menu>",
                &mut |_| None,
            )
            .unwrap();
        ui.set_number(m, t::VISIBLE, 1.0);
        let list = draw_list(&mut ui, m, &mut Fake, &|_| None);
        assert_eq!(list.len(), 2);
        let a = ui.find(m, "a").unwrap();
        recursive_fade(&mut ui, m, 0.5);
        let mut half = draw_list(&mut ui, m, &mut Fake, &|_| None);
        fade_items(&mut ui, m, 0.5, &mut half);
        assert_eq!(half.len(), 1);
        assert_eq!(half[0].tile, a);
        assert!((half[0].color[3] - 0.5 * 128.0 / 255.0).abs() < 1e-6);
        recursive_fade(&mut ui, m, 1.0);
        let mut whole = draw_list(&mut ui, m, &mut Fake, &|_| None);
        fade_items(&mut ui, m, 1.0, &mut whole);
        assert_eq!(whole, list);
        let mut none = list.clone();
        fade_items(&mut ui, m, 0.0, &mut none);
        assert_eq!(none.iter().find(|i| i.tile == a).unwrap().color[3], 0.0);
    }

    #[test]
    fn picture_sizes_and_clip_windows() {
        let mut ui = ui();
        // The stats menu's sizing (zoom 150 of an 8 × 32 picture), a
        // `&scale;` picture keeping its own file size, a `tile` -1 one, and
        // a list box cutting its rows.
        let m = ui
            .load_menu(
                b"<menu name=\"m\"><image name=\"karma\"><filename>Interface\\HUD\\hud_tick_mark.dds</filename><zoom>150</zoom>
                    <width><copy src=\"me()\" trait=\"filewidth\"/></width><height><copy src=\"me()\" trait=\"fileheight\"/></height></image>
                  <image name=\"map\"><filename>Interface\\HUD\\hud_tick_mark.dds</filename><zoom>&scale;</zoom><filewidth>2048</filewidth></image>
                  <image name=\"whole\"><filename>Interface\\HUD\\hud_tick_mark.dds</filename><tile>&scale;</tile><zoom>50</zoom></image>
                  <hotrect name=\"list\"><clipwindow>&true;</clipwindow><locus>&true;</locus><x>104</x><y>104</y><width>48</width><height>16</height><alpha>0</alpha>
                    <image name=\"row\"><filename>Interface\\HUD\\hud_tick_mark.dds</filename><clips>&true;</clips><x>-8</x><y>6</y><width>32</width><height>20</height><zoom>-1</zoom></image>
                    <image name=\"free\"><filename>Interface\\HUD\\hud_tick_mark.dds</filename><x>-8</x><y>6</y><width>32</width><height>20</height><zoom>-1</zoom></image>
                  </hotrect></menu>",
                &mut |_| None,
            )
            .unwrap();
        ui.set_number(m, t::VISIBLE, 1.0);
        update_file_sizes(&mut ui, m, &mut Fake);
        let karma = ui.find(m, "karma").unwrap();
        assert_eq!(
            (ui.number(karma, t::WIDTH), ui.number(karma, t::HEIGHT)),
            (12.0, 48.0)
        );
        let map = ui.find(m, "map").unwrap();
        assert_eq!(ui.number(map, t::FILEWIDTH), 2048.0);
        let whole = ui.find(m, "whole").unwrap();
        assert_eq!(ui.number(whole, t::FILEWIDTH), 8.0);
        // The row (96..128 × 110..130) is cut to the list (104..152 ×
        // 104..120, whole pixels at 1080: the game's scissor): a quarter of
        // it from the left, half from the bottom.
        let list = draw_list(&mut ui, m, &mut Fake, &|_| None);
        let row = ui.find(m, "row").unwrap();
        let free = ui.find(m, "free").unwrap();
        let item = list.iter().find(|i| i.tile == row).unwrap();
        let DrawKind::Image { rect, uv, .. } = &item.kind else {
            panic!("not a picture");
        };
        assert_eq!(*rect, [104.0, 110.0, 24.0, 10.0]);
        assert_eq!(*uv, [0.25, 0.0, 1.0, 0.5]);
        let item = list.iter().find(|i| i.tile == free).unwrap();
        let DrawKind::Image { rect, .. } = &item.kind else {
            panic!("not a picture");
        };
        assert_eq!(*rect, [96.0, 110.0, 32.0, 20.0]);
    }

    /// `00a1f7d0`: a picture that can't be found draws `sMissingImage`.
    #[test]
    fn a_missing_picture_draws_the_missing_image() {
        let mut ui = Ui::new(
            Screen {
                width_px: 1920,
                height_px: 1080,
                safe_x: 15.0,
                safe_y: 15.0,
            },
            SystemColors::new(None, None),
            Box::new(|name| {
                (name == "sMissingImage")
                    .then(|| "Data\\Textures\\Interface\\HUD\\hud_tick_mark.dds".to_string())
            }),
        );
        let m = ui
            .load_menu(
                b"<menu name=\"m\"><image name=\"icon\"><filename>interface\\icons\\nothing.dds</filename><width>64</width><height>64</height><zoom>-1</zoom></image></menu>",
                &mut |_| None,
            )
            .unwrap();
        let icon = ui.find(m, "icon").unwrap();
        let (tex, _, _) = image_geometry(&mut ui, icon, &mut Fake).unwrap();
        assert_eq!(tex, "textures\\interface\\hud\\hud_tick_mark.dds");
    }

    /// Clip windows cut quads and their texture coordinates (`00a07a40`'s
    /// scissor), and a failed check's highlight is dark red (`00a06c03`).
    #[test]
    fn clipping_and_the_failed_check_look() {
        let uv = [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
        let (r, cuv) = clip_quad([0.0, 0.0, 100.0, 50.0], uv, [50.0, 0.0, 100.0, 25.0]).unwrap();
        assert_eq!(r, [50.0, 0.0, 50.0, 25.0]);
        assert_eq!(cuv[0], [0.5, 0.0]);
        assert_eq!(cuv[3], [1.0, 0.5]);
        assert!(clip_quad([0.0, 0.0, 10.0, 10.0], uv, [20.0, 0.0, 5.0, 5.0]).is_none());
        let mut ui = ui();
        let m = ui
            .load_menu(
                b"<menu name=\"m\"><hotrect name=\"lb_highlight_box\"><alpha>40</alpha><image name=\"top\"><alpha>255</alpha></image></hotrect></menu>",
                &mut |_| None,
            )
            .unwrap();
        let bx = ui.find(m, "lb_highlight_box").unwrap();
        let top = ui.find(m, "top").unwrap();
        ui.tiles[bx].failed = true;
        ui.tiles[top].failed = true;
        assert_eq!(tile_color(&mut ui, bx), [0.545, 0.0, 0.0, 0.2]);
        assert_eq!(tile_color(&mut ui, top), [0.545, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn colours_without_a_system_colour() {
        let mut ui = ui();
        let m = ui
            .load_menu(
                b"<menu name=\"m\"><image name=\"a\"><brightness>-1</brightness><red>255</red><green>0</green><blue>0</blue><alpha>64</alpha></image>
                  <image name=\"b\"><systemcolor>&nosystemcolor;</systemcolor><red>0</red><green>255</green></image></menu>",
                &mut |_| None,
            )
            .unwrap();
        let a = ui.find(m, "a").unwrap();
        assert_eq!(tile_color(&mut ui, a), [1.0, 0.0, 0.0, 64.0 / 255.0]);
        // Colour 0 isn't registered: the tile's own colour.
        let b = ui.find(m, "b").unwrap();
        assert_eq!(tile_color(&mut ui, b), [0.0, 1.0, 1.0, 1.0]);
    }

    /// An `ishtml` text tile (`00a21af0` → `00a17390`): its tags aren't
    /// drawn, each font is its own item, its pictures are drawn, its size
    /// and page count are the layout's.
    #[test]
    fn html_text_by_font_with_its_pictures() {
        let mut ui = crate::menus::test_support::ui();
        let m = ui
            .load_menu(
                b"<menu name=\"m\"><text name=\"t\"><x>100</x><y>50</y><ishtml>&true;</ishtml><wrapwidth>400</wrapwidth></text>
                  <text name=\"plain\"></text></menu>",
                &mut |_| None,
            )
            .unwrap();
        ui.set_number(m, t::VISIBLE, 1.0);
        let text = ui.find(m, "t").unwrap();
        let plain = ui.find(m, "plain").unwrap();
        ui.set_string(
            text,
            t::STRING,
            "<p>AB<font face=2>CD</font><img src=\"Interface\\HUD\\hud_tick_mark.dds\" width=8 height=32>E",
        );
        ui.set_string(plain, t::STRING, "<p>AB");
        let list = draw_list(&mut ui, m, &mut Fake, &|_| None);
        let fonts: Vec<(usize, usize)> = list
            .iter()
            .filter_map(|i| match &i.kind {
                DrawKind::Text { font, glyphs } if i.tile == text => Some((*font, glyphs.len())),
                _ => None,
            })
            .collect();
        // A, B and E in font 1, C and D in font 2: no tag drawn.
        assert_eq!(fonts, [(1, 3), (2, 2)]);
        let picture = list
            .iter()
            .find_map(|i| match &i.kind {
                DrawKind::Image { texture, rect, .. } if i.tile == text => {
                    Some((texture.clone(), *rect))
                }
                _ => None,
            })
            .unwrap();
        // After four letters 10 wide, standing on the line (35 down).
        assert_eq!(picture.0, "textures\\interface\\hud\\hud_tick_mark.dds");
        assert_eq!(picture.1, [140.0, 53.0, 8.0, 32.0]);
        // One line: five letters and the picture wide, the table's line
        // height (35) high; one page.
        assert_eq!(ui.number(text, t::WIDTH), 58.0);
        assert_eq!(ui.number(text, t::HEIGHT), 35.0);
        assert_eq!(ui.number(text, t::PAGECOUNT), 1.0);
        // The same text in a tile without `ishtml` shows its tags.
        let glyphs = list
            .iter()
            .filter(|i| i.tile == plain)
            .map(|i| match &i.kind {
                DrawKind::Text { glyphs, .. } => glyphs.len(),
                _ => 0,
            })
            .sum::<usize>();
        assert_eq!(glyphs, 5);
    }
}
