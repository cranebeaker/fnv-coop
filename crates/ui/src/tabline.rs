//! Tab lines (`tabline.xml`): a row of tab buttons along the bottom of the
//! inventory and DATA menus, made by code from the menu's
//! `TabButtonTemplate` (`tabline_template.xml`), as FalloutNV.exe's
//! `00707be0` makes them.

use crate::names::t;
use crate::tile::{TileId, Ui};

/// Makes one tab button per label under `tabline` (`00707be0`): each from
/// the menu's `TabButtonTemplate`, its `string` the label, `listindex` its
/// number from 0 and `id` `first_id` plus that number. With W the tab
/// line's width and the buttons' widths summed, the gap between buttons
/// `_LeftLineLength` is trunc((W - Σ widths) / (count + 1)); the first
/// button's `_x` is one gap in, each next one a gap past the one before.
/// `_ButtonCount` is the count. Returns the buttons.
pub fn build(
    ui: &mut Ui,
    menu: TileId,
    tabline: TileId,
    first_id: i32,
    labels: &[&str],
) -> Vec<TileId> {
    let mut buttons = Vec::new();
    for (i, label) in labels.iter().enumerate() {
        let Some(b) = ui.instantiate(menu, tabline, "TabButtonTemplate") else {
            continue;
        };
        ui.set_string(b, t::STRING, label);
        ui.set_number(b, t::LISTINDEX, i as f32);
        ui.set_number(b, t::ID, (first_id + i as i32) as f32);
        buttons.push(b);
    }
    let widths: Vec<f32> = buttons.iter().map(|&b| ui.number(b, t::WIDTH)).collect();
    let line = ui.number(tabline, t::WIDTH);
    let gap = ((line - widths.iter().sum::<f32>()) / (buttons.len() + 1) as f32).trunc();
    let left = ui.names.lookup_or_add("_LeftLineLength").unwrap_or(0);
    ui.set_number(tabline, left, gap);
    let count = ui.names.lookup_or_add("_ButtonCount").unwrap_or(0);
    ui.set_number(tabline, count, buttons.len() as f32);
    let x_id = ui.names.lookup_or_add("_x").unwrap_or(0);
    let mut x = gap;
    for (&b, w) in buttons.iter().zip(&widths) {
        ui.set_number(b, x_id, x);
        x += w + gap;
    }
    buttons
}

/// Shows a tab as the current one (the tab line's `_CurrentTab`, which the
/// buttons' `_selected` and the pages' `visible` read).
pub fn set_current(ui: &mut Ui, tabline: TileId, tab: usize) {
    let current = ui.names.lookup_or_add("_CurrentTab").unwrap_or(0);
    ui.set_number(tabline, current, tab as f32);
}

/// The tab shown now.
pub fn current(ui: &mut Ui, tabline: TileId) -> usize {
    let current = ui.names.lookup_or_add("_CurrentTab").unwrap_or(0);
    ui.number(tabline, current).max(0.0) as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tile::{Screen, SystemColors};

    #[test]
    fn buttons_share_the_line_evenly() {
        let mut ui = Ui::new(
            Screen {
                width_px: 1920,
                height_px: 1080,
                safe_x: 15.0,
                safe_y: 15.0,
            },
            SystemColors::new(None, None),
            Box::new(|_| None),
        );
        // Buttons 100, 60 and 80 wide (their text and buffer, here fixed)
        // on an 855 line: (855 - 240) / 4 = 153.75, cut to 153.
        let m = ui
            .load_menu(
                br#"<menu name="m"><rect name="line"><width>855</width></rect>
                  <template name="TabButtonTemplate"><hotrect name="TabButton"><_x>0</_x><x><copy src="me()" trait="_x"/></x>
                    <width><copy>40</copy><add><copy src="me()" trait="listindex"/><mul>20</mul></add><add><copy src="me()" trait="listindex"/><eq>0</eq><mul>60</mul></add></width>
                    <_selected><copy src="parent()" trait="_CurrentTab"/><eq src="me()" trait="listindex"/></_selected></hotrect></template></menu>"#,
                &mut |_| None,
            )
            .unwrap();
        let line = ui.find(m, "line").unwrap();
        let tabs = build(&mut ui, m, line, 24, &["Weapons", "Apparel", "Aid"]);
        assert_eq!(tabs.len(), 3);
        let x: Vec<f32> = tabs.iter().map(|&b| ui.number(b, t::X)).collect();
        assert_eq!(
            x,
            [
                153.0,
                153.0 + 100.0 + 153.0,
                153.0 + 100.0 + 153.0 + 60.0 + 153.0
            ]
        );
        assert_eq!(ui.number(tabs[2], t::ID), 26.0);
        assert_eq!(ui.string(tabs[1], t::STRING).as_deref(), Some("Apparel"));
        set_current(&mut ui, line, 1);
        let selected = ui.names.lookup("_selected").unwrap();
        assert_eq!(ui.number(tabs[1], selected), 1.0);
        assert_eq!(ui.number(tabs[0], selected), 0.0);
        assert_eq!(current(&mut ui, line), 1);
    }
}
