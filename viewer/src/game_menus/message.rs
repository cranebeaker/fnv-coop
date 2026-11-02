//! Message boxes (`ui::menus::message`): a script's `ShowMessage` of a
//! message box (`world::scripting`'s `Event::Message` with buttons, queued
//! as `menus::Menu::Message`) shown in the game's message menu; the button
//! pressed goes back as what `GetButtonPressed` gives.

use cellview::Game;
use esm::FormId;
use ui::menus::message::{MessageBox, MessageMenu, FILE};

use super::{OpenMenu, Screen};
use crate::menus::Menu;

/// Whether this module shows a request.
pub fn takes(menu: &Menu) -> bool {
    matches!(menu, Menu::Message { .. } | Menu::Popup { .. })
}

/// The owner of the game's own boxes (`Menu::Popup`): their answer is
/// taken and dropped (the reputation box's callback `00615720` only reads
/// it), so no script's `GetButtonPressed` sees it.
pub const POPUP_OWNER: u32 = 0x0061_5720;

/// The menus' background alpha (`_background_fill_alpha` of the globals:
/// `fMenuBackgroundOpacity` × 255), which a box keeps (`007a8850`).
pub fn background_alpha(screen: &mut Screen) -> f32 {
    let Some(globals) = screen.ui.globals else {
        return ui::game::MENU_BACKGROUND_OPACITY * 255.0;
    };
    match screen.ui.names.lookup("_background_fill_alpha") {
        Some(id) => screen.ui.number(globals, id),
        None => ui::game::MENU_BACKGROUND_OPACITY * 255.0,
    }
}

/// A message box request becomes a box in the message menu (opened if it
/// isn't).
pub fn open(screen: &mut Screen, game: &Game, request: Menu) {
    if let Menu::Popup {
        title,
        text,
        icon,
        sound,
    } = request
    {
        // `00703f10`: kind 0x17, the one button `sOk`.
        let ok = screen
            .ui
            .setting_text("sOk")
            .unwrap_or_else(|| ui::menus::message::DEFAULT_BUTTON.to_string());
        let alpha = background_alpha(screen);
        let mut b = MessageBox::new(&text, title.as_deref(), &[&ok], 0x17, alpha);
        b.icon = icon;
        b.sound = sound;
        b.owner = Some(POPUP_OWNER);
        show(screen, game, b);
        return;
    }
    let Menu::Message {
        title,
        text,
        buttons,
    } = request
    else {
        return;
    };
    // The world lists the buttons whose conditions pass with their
    // numbers; the game keeps the others as blanks (`005b4630`).
    let count = buttons.iter().map(|(i, _)| i + 1).max().unwrap_or(0);
    let mut slots: Vec<Option<String>> = vec![None; count];
    for (i, label) in buttons {
        slots[i] = Some(label);
    }
    let alpha = background_alpha(screen);
    let b = MessageBox::script(&text, title.as_deref(), &slots, alpha);
    show(screen, game, b);
}

/// Puts a box in the message menu, opening it if it isn't (`007a8e60`).
pub fn show(screen: &mut Screen, game: &Game, b: MessageBox) {
    if let Some(m) = screen.message() {
        m.push(b);
        return;
    }
    // Another menu open: the box's background is the pop-up's
    // (`fPopUpBackgroundOpacity`, `007a8ba0`).
    let popup = (!screen.open.is_empty()).then(|| {
        game.settings
            .float("Interface", "fPopUpBackgroundOpacity")
            .unwrap_or(ui::game::POPUP_BACKGROUND_OPACITY)
    });
    let mut menu = MessageMenu::new(0);
    match screen.load(game, FILE, &mut menu) {
        Ok(tile) => {
            menu.menu = tile;
            menu.push(b);
            menu.open(&mut screen.ui, popup);
            screen.open.push(OpenMenu::Message(menu));
        }
        Err(e) => println!("The message box can't be shown: {e}"),
    }
}

/// After the player's input: the button pressed is what `GetButtonPressed`
/// gives next; the menu's sounds play.
pub fn after(
    open: &mut [OpenMenu],
    state: &mut world::scripting::GameState,
    sounds: &mut crate::sounds::SoundRequests,
    order: &esm::LoadOrder,
) {
    for m in open.iter_mut() {
        let OpenMenu::Message(m) = m else {
            continue;
        };
        m.take_pressed_for(Some(POPUP_OWNER));
        if let Some(n) = m.take_pressed() {
            println!("Message box: button {n}.");
            state.button = Some(n);
        }
        for name in m.sounds.drain(..) {
            if let Some(id) = order.form_by_editor_id(&name) {
                sounds.0.push(FormId(id.0));
            }
        }
    }
}
