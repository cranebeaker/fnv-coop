//! What the Pip-Boy asks with the game's own menus over it: message boxes
//! with a callback (the fast-travel question, the custom map marker's) and
//! "how many?" (dropping). The game opens these as ordinary menus on the
//! main screen over the Pip-Boy's (they aren't among the menus drawn on
//! its screen: only those whose file's `id` is `&pipboymenu;` are); each
//! answer goes back to the Pip-Boy's code by its callback.

use bevy::prelude::*;
use cellview::Game;
use ui::menus::message::MessageBox;

use super::{OpenMenu, Screen};

/// The Pip-Boy's questions waiting to be shown, and the answers.
#[derive(Resource, Default)]
pub struct PipboyAsks {
    pub asks: Vec<Ask>,
    pub answers: Vec<Answer>,
}

/// A question.
#[derive(Debug, Clone, PartialEq)]
pub enum Ask {
    /// A message box (`00703e80`): its text and buttons, the number the
    /// first button answers with (the rest count on from it); `owner` says
    /// which callback hears the button pressed.
    Box {
        owner: u32,
        text: String,
        buttons: Vec<String>,
        first_number: i32,
    },
    /// "How many?" (`007aba00`): up to `most`, starting at all of it.
    HowMany { owner: u32, most: i32 },
}

/// An answer: the button's number, or the amount (0: cancelled).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Answer {
    pub owner: u32,
    pub value: i32,
}

/// The callbacks' numbers (the boxes' `owner`s), out of the way of the
/// other menus' own.
pub const TRAVEL: u32 = 0x5049_0001;
pub const SET_MARKER: u32 = 0x5049_0002;
pub const DROP: u32 = 0x5049_0003;
const BOX_OWNERS: [u32; 2] = [TRAVEL, SET_MARKER];

/// The kind the Pip-Boy's boxes have (`00796fd0` passes 0x17).
const KIND: i32 = 0x17;

/// Opens the questions asked.
pub fn open(screen: &mut Screen, game: &Game, asks: &mut PipboyAsks) {
    for ask in asks.asks.drain(..) {
        match ask {
            Ask::Box {
                owner,
                text,
                buttons,
                first_number,
            } => {
                let alpha = super::message::background_alpha(screen);
                let labels: Vec<&str> = buttons.iter().map(String::as_str).collect();
                let mut b = MessageBox::new(&text, None, &labels, KIND, alpha);
                b.owner = Some(owner);
                b.first_number = first_number;
                super::message::show(screen, game, b);
            }
            Ask::HowMany { owner, most } => {
                super::container::open_quantity(screen, game, most);
                screen.quantity_owner = Some(owner);
            }
        }
    }
}

/// The answers given this frame.
pub fn after(screen: &mut Screen, asks: &mut PipboyAsks) {
    let mut below_other = false;
    let quantity_owner = screen.quantity_owner;
    for m in screen.open.iter_mut() {
        match m {
            OpenMenu::Message(msg) => {
                for owner in BOX_OWNERS {
                    if let Some(n) = msg.take_pressed_for(Some(owner)) {
                        asks.answers.push(Answer { owner, value: n });
                    }
                }
            }
            OpenMenu::Container(_) | OpenMenu::Barter(_) => below_other = true,
            OpenMenu::Quantity(q) if !below_other => {
                if let (Some(owner), Some(n)) = (quantity_owner, q.answer) {
                    q.answer = None;
                    asks.answers.push(Answer { owner, value: n });
                    screen.quantity_owner = None;
                }
            }
            _ => {}
        }
    }
}
