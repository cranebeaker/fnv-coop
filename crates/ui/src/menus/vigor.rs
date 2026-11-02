//! LoveTesterMenu (1074): its 3D button callbacks and page state.
//! The XML supplies input hotrects; the machine's models supply the picture.
//! No fallback picture is generated here. See `docs/VIGOR.md`.
//!
//! The caller supplies the seven integer actor values read through the
//! player's AV owner +0x1c, applies requests immediately, then reads values
//! again before another click. Values are not held until Done: 007957c0,
//! 007958e0, 00795a00 and 00795b30 call 0093a7c0 on every adjustment.

use crate::menu::MenuCode;
use crate::tile::{TileId, Ui};

pub const FILE: &str = "menus\\chargen\\love_tester_menu.xml";
pub const CLASS: i32 = 1074;
const KEY_PLUS: u32 = b'+' as u32;
const KEY_EQUALS: u32 = b'=' as u32;
const KEY_MINUS: u32 = b'-' as u32;
const KEY_UNDERSCORE: u32 = b'_' as u32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Request {
    Sound(&'static str),
    SetActorValue {
        av: u16,
        value: i32,
    },
    /// Play the new page's actual model sequence, with its clock reset.
    Page {
        page: u8,
        forward: bool,
    },
    Close,
}

/// The fields initialized by 00720550. Scene setup calls next once
/// (007928e0 -> 00795c60), starting page1's Forward sequence.
#[derive(Debug, Clone)]
pub struct LoveTester {
    pub page: u8,
    pub selection: u8,
    pub ready: bool,
    pub budget: i32,
}

impl LoveTester {
    pub fn new(budget: i32) -> Self {
        Self {
            page: 0,
            selection: 1,
            ready: true,
            budget,
        }
    }

    /// 00794d30. The index-page selection is enabled only under the
    /// original 004b71d0 guard. Its caller supplies that input-mode flag;
    /// this layer does not infer it from cursor visibility.
    pub fn actor_value(&self, index_input: bool) -> Option<u16> {
        let page = if self.page == 8 && index_input {
            self.selection
        } else {
            self.page
        };
        (1..=7).contains(&page).then_some(u16::from(page) + 4)
    }

    /// 00791ab0: no arbitrary delay between pages. Use the actual
    /// sequence's clock and end key (or no sequence if lookup failed).
    pub fn update_ready(&mut self, sequence: Option<(f32, f32)>) {
        self.ready = self.page == 0 || sequence.map_or(true, |(time, end)| time >= end);
    }

    /// 007949f0, called by the ready-guarded buttons 00795c60/00795ca0.
    pub fn navigate(&mut self, forward: bool, values: &[i32; 7]) -> Vec<Request> {
        if !self.ready {
            return Vec::new();
        }
        if forward && self.page >= 8 {
            return self.done(values);
        }
        if !forward && self.page == 0 {
            return Vec::new();
        }
        self.page = if forward {
            self.page + 1
        } else {
            self.page - 1
        };
        // The menu's update, not this callback, sets ready from sequence
        // progress. Preserve that boundary for multiple input callbacks.
        vec![
            Request::Sound("OBJBookSpecialPageTurn"),
            Request::Page {
                page: self.page,
                forward,
            },
        ]
    }

    /// AllDone's handler (00795ce0), also the final next-page attempt.
    pub fn done(&self, values: &[i32; 7]) -> Vec<Request> {
        if values.iter().sum::<i32>() == self.budget {
            vec![
                Request::Sound("OBJBookSpecialNumberFinished"),
                Request::Close,
            ]
        } else {
            vec![Request::Sound("UIActivateNothing")]
        }
    }

    /// 00791e40 tile callbacks. For ids2/3 the caller compares the old
    /// value with the live value after applying SetActorValue and plays
    /// the corresponding number sound; unlike a model click this does
    /// not sound merely because a change was attempted.
    pub fn tile_click(&mut self, id: i32, values: &[i32; 7], controller: bool) -> Vec<Request> {
        match id {
            0 | 1 => self.navigate(id == 0, values),
            2 | 3 => self
                .actor_value(controller)
                .map(|av| {
                    self.adjust(av, id == 2, values)
                        .into_iter()
                        .filter(|r| !matches!(r, Request::Sound(_)))
                        .collect()
                })
                .unwrap_or_default(),
            4 => self.done(values),
            6 | 7 if self.page == 8 => {
                let old = self.selection;
                self.selection = if id == 6 {
                    self.selection.saturating_sub(1).max(1)
                } else {
                    (self.selection + 1).min(7)
                };
                if old != self.selection {
                    vec![Request::Sound("OBJBookSpecialFocus")]
                } else {
                    Vec::new()
                }
            }
            _ => Vec::new(),
        }
    }

    /// Direct 3D button handlers. The + sound checks the attribute limit
    /// BEFORE the budget check: an exhausted budget can still make the
    /// sound (00795a00/00795b30). This differs from tile clicks00791e40,
    /// which compare the values after changing them.
    pub fn adjust(&self, av: u16, increase: bool, values: &[i32; 7]) -> Vec<Request> {
        let Some(index) = av.checked_sub(5).map(usize::from).filter(|&i| i < 7) else {
            return Vec::new();
        };
        let old = values[index];
        let mut out = Vec::new();
        if increase {
            if old < 10 {
                out.push(Request::Sound("OBJBookSpecialNumber"));
            }
            if values.iter().sum::<i32>() < self.budget {
                out.push(Request::SetActorValue {
                    av,
                    value: old.saturating_add(1).min(10),
                });
            }
        } else {
            if old > 1 {
                out.push(Request::Sound("OBJBookSpecialNumberDown"));
            }
            out.push(Request::SetActorValue {
                av,
                value: old.saturating_sub(1).max(1),
            });
        }
        out
    }

    /// A picked button node from the two menu models (007928e0).
    /// Picking and visibility belong to the rendered scene, not guessed
    /// rectangles. Unknown names produce no action.
    pub fn model_button(
        &mut self,
        name: &str,
        values: &[i32; 7],
        index_input: bool,
    ) -> Vec<Request> {
        match name {
            "P1_RT_Btn:0" | "LookInside_Btn:0" => self.navigate(true, values),
            "P1_LT_Btn:0" => self.navigate(false, values),
            "AllDone_Btn:0" => self.done(values),
            "P1_Increase_Btn:0" | "P1_Decrease_Btn:0" => self
                .actor_value(index_input)
                .map(|av| self.adjust(av, name == "P1_Increase_Btn:0", values))
                .unwrap_or_default(),
            _ => {
                let Some(tail) = name.strip_prefix("Index_") else {
                    return Vec::new();
                };
                const ATTRIBUTES: [&str; 7] = [
                    "Strength",
                    "Perception",
                    "Endurance",
                    "Charisma",
                    "Intelligence",
                    "Agility",
                    "Luck",
                ];
                for (i, attribute) in ATTRIBUTES.iter().enumerate() {
                    if let Some(button) = tail.strip_prefix(attribute) {
                        let increase = match button {
                            "Increase_Btn:0" => true,
                            "Decrease_Btn:0" => false,
                            _ => return Vec::new(),
                        };
                        return self.adjust(5 + i as u16, increase, values);
                    }
                }
                Vec::new()
            }
        }
    }
}

/// Input deferred to the viewer, which owns live actor-value reads and
/// applies menu callbacks one at a time before refreshing its snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputIntent {
    /// Dispatch the LoveTesterMenu click callback (`00791e40`).
    TileClick(i32),
    /// Ask the 3D scene picker to resolve the picked model button (`id == 5`).
    ModelPick,
}

/// XML and keyboard front-end for [`LoveTester`]. `tiles` are the eight
/// `SetTile` slots (`00791750`); the machine picture and picking stay in viewer.
///
/// Drain [`take_inputs`](Self::take_inputs), apply each intent serially, then
/// supply fresh SPECIAL values before processing another input. Menu clicks
/// only queue work and never read or write game state.
#[derive(Debug, Clone)]
pub struct VigorMenu {
    pub menu: TileId,
    /// `SetTile` ids 0..7: next, previous, increase, decrease, done,
    /// model picking, previous SPECIAL, next SPECIAL.
    pub tiles: [Option<TileId>; 8],
    pub closed: bool,
    pub model: LoveTester,
    /// Inputs for the viewer to process serially.
    pub intents: Vec<InputIntent>,
}

impl VigorMenu {
    pub fn new(menu: TileId, budget: i32) -> Self {
        Self {
            menu,
            tiles: [None; 8],
            closed: false,
            model: LoveTester::new(budget),
            intents: Vec::new(),
        }
    }

    /// Remove queued work for the viewer to apply in order.
    pub fn take_inputs(&mut self) -> Vec<InputIntent> {
        std::mem::take(&mut self.intents)
    }

    /// Called by the viewer after the game accepts the Done action.
    pub fn mark_closed(&mut self) {
        self.closed = true;
    }

    /// Handle a key using the current SPECIAL snapshot and input-mode guard.
    /// Mapping and gates follow `007927a0`: Left/Right navigate backward/
    /// forward on every page, Up/`+`/`=` increase, and Down/`-`/`_` decrease.
    /// Index selection is through tile ids 6/7. The caller applies the intent
    /// before reusing its value snapshot.
    pub fn keyboard(&mut self, code: u32, values: &[i32; 7], index_input: bool) -> bool {
        if self.closed {
            return false;
        }

        let click = match code {
            crate::menu::key::LEFT => Some(1),
            crate::menu::key::RIGHT => Some(0),
            crate::menu::key::UP | KEY_PLUS | KEY_EQUALS => {
                let Some(av) = self.model.actor_value(index_input) else {
                    return false;
                };
                let index = usize::from(av - 5);
                (values.iter().sum::<i32>() < self.model.budget && values[index] < 10).then_some(2)
            }
            crate::menu::key::DOWN | KEY_MINUS | KEY_UNDERSCORE => {
                let Some(av) = self.model.actor_value(index_input) else {
                    return false;
                };
                (values[usize::from(av - 5)] > 1).then_some(3)
            }
            _ => return false,
        };
        let Some(id) = click else {
            return false;
        };
        self.intents.push(InputIntent::TileClick(id));
        true
    }
}

impl MenuCode for VigorMenu {
    fn class(&self) -> i32 {
        CLASS
    }

    fn set_tile(&mut self, id: i32, tile: TileId) {
        if (0..8).contains(&id) {
            self.tiles[id as usize] = Some(tile);
        }
    }

    /// `00791e40` entry point. A model hit is resolved after the viewer drains
    /// the queued intent.
    fn click(&mut self, _ui: &mut Ui, id: i32, _tile: Option<TileId>, _now: f64) {
        if self.closed || !(0..8).contains(&id) {
            return;
        }
        self.intents.push(if id == 5 {
            InputIntent::ModelPick
        } else {
            InputIntent::TileClick(id)
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_navigation_waits_for_the_real_animation_and_does_not_wrap() {
        let mut menu = LoveTester::new(40);
        let values = [5; 7];
        assert!(menu.navigate(false, &values).is_empty());
        assert_eq!(
            menu.navigate(true, &values)[1],
            Request::Page {
                page: 1,
                forward: true
            }
        );
        menu.update_ready(Some((0.3, 1.3)));
        assert!(menu.navigate(true, &values).is_empty());
        menu.update_ready(Some((1.3, 1.3)));
        assert_eq!(
            menu.navigate(false, &values)[1],
            Request::Page {
                page: 0,
                forward: false
            }
        );
        menu.page = 8;
        assert_eq!(
            menu.navigate(true, &values),
            vec![Request::Sound("UIActivateNothing")]
        );
        assert_eq!(menu.page, 8);
    }

    #[test]
    fn increase_sound_is_independent_of_remaining_points_but_limits_are_kept() {
        let menu = LoveTester::new(35);
        assert_eq!(
            menu.adjust(5, true, &[5; 7]),
            vec![Request::Sound("OBJBookSpecialNumber")]
        );
        let menu = LoveTester::new(40);
        let mut values = [1; 7];
        values[0] = 10;
        assert_eq!(
            menu.adjust(5, true, &values),
            vec![Request::SetActorValue { av: 5, value: 10 }]
        );
        assert_eq!(
            menu.adjust(6, false, &values),
            vec![Request::SetActorValue { av: 6, value: 1 }]
        );
        assert!(menu.adjust(12, true, &values).is_empty());
    }

    #[test]
    fn index_buttons_address_their_attribute_without_changing_the_selection() {
        let mut menu = LoveTester::new(40);
        menu.page = 8;
        assert_eq!(menu.actor_value(false), None);
        assert_eq!(menu.actor_value(true), Some(5));
        assert_eq!(
            menu.model_button("Index_LuckIncrease_Btn:0", &[5; 7], false),
            vec![
                Request::Sound("OBJBookSpecialNumber"),
                Request::SetActorValue { av: 11, value: 6 },
            ]
        );
        assert_eq!(menu.selection, 1);
    }

    #[test]
    fn done_requires_exact_spending_including_the_last_next_button() {
        let mut menu = LoveTester::new(40);
        menu.page = 8;
        let spent = [10, 5, 5, 5, 5, 5, 5];
        assert_eq!(
            menu.navigate(true, &spent),
            vec![
                Request::Sound("OBJBookSpecialNumberFinished"),
                Request::Close
            ]
        );
        assert_eq!(
            menu.done(&[10; 7]),
            vec![Request::Sound("UIActivateNothing")]
        );
    }

    #[test]
    fn vigor_menu_records_eight_xml_tiles_and_queues_model_pick() {
        let mut ui = crate::menus::test_support::ui();
        let mut menu = VigorMenu::new(0, 40);
        let tiles = (0..8)
            .map(|id| format!("<hotrect name=\"Vigor{id}\"><id>{id}</id></hotrect>"))
            .collect::<String>();
        let xml = format!("<menu name=\"LoveTesterMenu\">{tiles}</menu>");
        menu.menu = crate::menus::test_support::load(&mut ui, &xml, &mut menu);
        assert!(menu.tiles.iter().all(Option::is_some));
        assert_eq!(menu.class(), CLASS);

        for id in 0..8 {
            menu.click(&mut ui, id, menu.tiles[id as usize], 0.0);
        }
        assert_eq!(
            menu.take_inputs(),
            vec![
                InputIntent::TileClick(0),
                InputIntent::TileClick(1),
                InputIntent::TileClick(2),
                InputIntent::TileClick(3),
                InputIntent::TileClick(4),
                InputIntent::ModelPick,
                InputIntent::TileClick(6),
                InputIntent::TileClick(7),
            ]
        );
    }

    #[test]
    fn keyboard_keys_queue_only_when_original_value_gates_allow_them() {
        let mut menu = VigorMenu::new(0, 40);
        menu.model.page = 1;
        let values = [5; 7];
        assert!(menu.keyboard(crate::menu::key::UP, &values, false));
        assert_eq!(menu.take_inputs(), vec![InputIntent::TileClick(2)]);
        assert!(menu.keyboard(b'+' as u32, &values, false));
        assert_eq!(menu.take_inputs(), vec![InputIntent::TileClick(2)]);
        assert!(menu.keyboard(b'=' as u32, &values, false));
        assert_eq!(menu.take_inputs(), vec![InputIntent::TileClick(2)]);
        let full_budget = [10, 5, 5, 5, 5, 5, 5];
        assert!(!menu.keyboard(crate::menu::key::UP, &full_budget, false));
        assert!(!menu.keyboard(b'+' as u32, &full_budget, false));
        assert!(menu.take_inputs().is_empty());

        let maxed_attribute = [10, 1, 1, 1, 1, 1, 1];
        assert!(!menu.keyboard(b'=' as u32, &maxed_attribute, false));
        assert!(menu.take_inputs().is_empty());

        assert!(menu.keyboard(crate::menu::key::DOWN, &values, false));
        assert_eq!(menu.take_inputs(), vec![InputIntent::TileClick(3)]);
        assert!(menu.keyboard(b'-' as u32, &values, false));
        assert_eq!(menu.take_inputs(), vec![InputIntent::TileClick(3)]);
        assert!(menu.keyboard(b'_' as u32, &values, false));
        assert_eq!(menu.take_inputs(), vec![InputIntent::TileClick(3)]);
        let minimum = [1; 7];
        assert!(!menu.keyboard(crate::menu::key::DOWN, &minimum, false));
        assert!(menu.take_inputs().is_empty());
    }

    #[test]
    fn keyboard_left_right_always_navigate_and_index_tiles_stay_distinct() {
        let values = [5; 7];
        let mut menu = VigorMenu::new(0, 40);
        menu.model.page = 1;
        assert!(menu.keyboard(crate::menu::key::LEFT, &values, false));
        assert_eq!(menu.take_inputs(), vec![InputIntent::TileClick(1)]);
        assert!(menu.keyboard(crate::menu::key::RIGHT, &values, false));
        assert_eq!(menu.take_inputs(), vec![InputIntent::TileClick(0)]);

        menu.model.page = 8;
        assert!(!menu.keyboard(b'+' as u32, &values, false));
        assert!(menu.take_inputs().is_empty());
        assert!(menu.keyboard(b'+' as u32, &values, true));
        assert_eq!(menu.take_inputs(), vec![InputIntent::TileClick(2)]);
        assert!(menu.keyboard(crate::menu::key::LEFT, &values, false));
        assert_eq!(menu.take_inputs(), vec![InputIntent::TileClick(1)]);
        assert!(menu.keyboard(crate::menu::key::RIGHT, &values, false));
        assert_eq!(menu.take_inputs(), vec![InputIntent::TileClick(0)]);

        menu.mark_closed();
        assert!(!menu.keyboard(crate::menu::key::LEFT, &values, true));
        assert!(menu.take_inputs().is_empty());
    }
}
