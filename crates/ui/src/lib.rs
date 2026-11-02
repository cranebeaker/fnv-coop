//! The game's menus, read from its own files and worked out the way
//! FalloutNV.exe works them out: its menu XML (`menus\*.xml` in `Fallout -
//! Misc.bsa`) read into "tiles", each tile's "traits" (position, size,
//! picture, text, colour...) evaluated from the operator chains the files
//! give them, its fonts (`.fnt` and `.tex`) and its text layout, and the
//! HUD (`hud_main_menu.xml`) laid out and fed as the game's code does.
//!
//! What each rule is and where in the exe it was read is in the doc
//! comments; `%USERPROFILE%\nv-re\findings\ui_tiles.md` has the findings
//! with addresses. The menus work in their own units: 960 high, as wide as
//! the screen's shape makes it (1706.67 at 16:9), drawn at screen height /
//! 960 pixels per unit.

pub mod anim;
pub mod atlas;
pub mod compass;
pub mod controls;
pub mod draw;
pub mod fade;
pub mod font;
pub mod game;
pub mod html;
pub mod hud;
pub mod list;
pub mod listbox;
pub mod lockpick;
pub mod menu;
pub mod menus;
pub mod names;
pub mod pipboy;
pub mod quest_text;
pub mod tabline;
pub mod text;
pub mod tile;
pub mod vats;
pub mod xml;
pub mod xp;

pub use atlas::Atlas;
pub use draw::{draw_list, DrawItem, DrawKind};
pub use font::Font;
pub use hud::{Hud, HudInput};
pub use names::Names;
pub use tile::{Screen, SystemColors, TileId, Ui};
pub use xp::Experience;
